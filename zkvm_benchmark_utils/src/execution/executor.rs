//! Benchmark executor - runs zkVM benchmarks and collects metrics
//!
//! Supports both sequential and parallel execution strategies.

use crate::analysis::log_parser::LogParser;
use crate::core::config::{BenchmarkConfig, ZkVmConfig};
use crate::core::error::{BenchmarkError, Result};
use crate::execution::command_parser::CommandParser;
use crate::execution::resource_monitor::{monitor_process_async, ResourceStats};
use crate::execution::types::{ExecutionResult, TestRun};

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use chrono;
use futures::stream::{self, StreamExt};
use log::{error, info, warn};
use regex::Regex;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// Parallel execution configuration
#[derive(Debug, Clone)]
pub struct ParallelExecutionConfig {
    /// Max concurrent zkVMs
    pub max_concurrent_zkvms: usize,
    /// Max concurrent tests per zkVM
    pub max_concurrent_tests_per_zkvm: usize,
    /// Allow parallel across zkVMs
    pub parallel_across_zkvms: bool,
    /// Enable resource monitoring
    pub enable_resource_monitoring: bool,
}

impl ParallelExecutionConfig {
    /// Default configuration (Sequential execution by default for accurate benchmarking)
    pub fn default() -> Self {
        Self::conservative()
    }

    /// Automatic parallel configuration (auto-adjusted based on CPU cores)
    /// Use this when throughput is more important than individual benchmark accuracy.
    pub fn auto_parallel() -> Self {
        let cpu_count = num_cpus::get();

        Self {
            // Limit concurrency to avoid resource contention
            max_concurrent_zkvms: (cpu_count / 2).max(1).min(4),
            max_concurrent_tests_per_zkvm: 2,
            parallel_across_zkvms: true,
            enable_resource_monitoring: true,
        }
    }

    /// Conservative configuration (sequential execution)
    pub fn conservative() -> Self {
        Self {
            max_concurrent_zkvms: 1,
            max_concurrent_tests_per_zkvm: 1,
            parallel_across_zkvms: false,
            enable_resource_monitoring: true,
        }
    }

    /// Aggressive configuration (for high-performance servers)
    pub fn aggressive() -> Self {
        let cpu_count = num_cpus::get();

        Self {
            max_concurrent_zkvms: cpu_count.min(8),
            max_concurrent_tests_per_zkvm: 4,
            parallel_across_zkvms: true,
            enable_resource_monitoring: true,
        }
    }
}

/// Remove ANSI escape sequences from a string
/// This cleans up terminal color codes and formatting from log output
fn strip_ansi_codes(text: &str) -> String {
    // Match ANSI escape sequences like \x1b[0m, \x1b[34m, etc.
    // Pattern: ESC [ (any number of digits/semicolons) followed by a letter
    let re = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    re.replace_all(text, "").to_string()
}

/// Benchmark Executor
pub struct BenchmarkExecutor {
    config: Arc<BenchmarkConfig>,
    output_dir: PathBuf,
    exec_config: ParallelExecutionConfig,
    command_parser: Arc<CommandParser>,
}

impl BenchmarkExecutor {
    /// Create a new benchmark executor with default parallelism
    pub fn new(config: BenchmarkConfig) -> Result<Self> {
        Self::with_parallelism(config, ParallelExecutionConfig::default())
    }

    /// Create a new benchmark executor with custom parallelism
    pub fn with_parallelism(
        config: BenchmarkConfig,
        exec_config: ParallelExecutionConfig,
    ) -> Result<Self> {
        let output_dir = PathBuf::from(&config.output_dir);

        // Create output directories
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(output_dir.join("raw-logs"))?;
        fs::create_dir_all(output_dir.join("parsed-metrics"))?;
        fs::create_dir_all(output_dir.join("reports"))?;

        Ok(Self {
            config: Arc::new(config),
            output_dir,
            exec_config,
            command_parser: Arc::new(CommandParser::new()),
        })
    }

    /// Run all benchmarks
    pub async fn run_all(&self) -> Result<Vec<ExecutionResult>> {
        info!("Starting benchmark execution");
        info!(
            "Parallelism: {} concurrent zkVMs, {} tests per zkVM",
            self.exec_config.max_concurrent_zkvms, self.exec_config.max_concurrent_tests_per_zkvm
        );

        let enabled_zkvms = self.config.enabled_zkvms();

        if enabled_zkvms.is_empty() {
            return Err(BenchmarkError::Config("No enabled zkVMs found".to_string()));
        }

        // Build all test tasks
        let test_tasks = self.build_test_tasks(&enabled_zkvms);
        let total_tasks = test_tasks.len();

        info!("Prepared {} test tasks", total_tasks);

        // Create semaphore to control concurrency
        let semaphore = Arc::new(Semaphore::new(self.exec_config.max_concurrent_zkvms));

        // Execute all tasks
        let results: Vec<ExecutionResult> = stream::iter(test_tasks)
            .map(|(zkvm_name, zkvm_config, test_run)| {
                let sem = Arc::clone(&semaphore);
                let executor = self.clone_for_task();

                async move {
                    // Acquire permit
                    let _permit = sem.acquire().await.unwrap();

                    info!(
                        "▶ Running: {} mode={} scale={} repeat={}/{}",
                        test_run.zkvm_name,
                        test_run.mode,
                        test_run.scale,
                        test_run.repeat,
                        executor.config.repeat_count.unwrap_or(1)
                    );

                    let result = executor
                        .run_single_task(&zkvm_name, &zkvm_config, &test_run)
                        .await;

                    match result {
                        Ok(exec_result) => {
                            if exec_result.success {
                                info!(
                                    "✓ Completed: {} {} scale={}",
                                    zkvm_name, test_run.mode, test_run.scale
                                );
                            } else {
                                warn!(
                                    "✗ Failed: {} {} scale={}: {:?}",
                                    zkvm_name, test_run.mode, test_run.scale, exec_result.error
                                );
                            }
                            exec_result
                        }
                        Err(e) => {
                            error!(
                                "✗ Error: {} {} scale={}: {}",
                                zkvm_name, test_run.mode, test_run.scale, e
                            );
                            ExecutionResult {
                                test_run,
                                metrics: None,
                                log_content: String::new(),
                                success: false,
                                error: Some(e.to_string()),
                                resource_stats: None,
                            }
                        }
                    }
                }
            })
            .buffer_unordered(self.exec_config.max_concurrent_zkvms)
            .collect()
            .await;

        info!("✅ Completed {} test runs", results.len());

        // Stats
        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.len() - successful;

        if failed > 0 {
            warn!("⚠ {} tests failed out of {}", failed, results.len());
        }

        Ok(results)
    }

    /// Build all test tasks
    fn build_test_tasks(
        &self,
        enabled_zkvms: &[(&String, &ZkVmConfig)],
    ) -> Vec<(String, ZkVmConfig, TestRun)> {
        let mut tasks = Vec::new();

        for (zkvm_name, zkvm_config) in enabled_zkvms {
            for scale in &self.config.test_scales {
                let repeat_count = self.config.repeat_count.unwrap_or(1);

                for repeat in 1..=repeat_count {
                    for mode in &zkvm_config.test_modes {
                        let test_run = TestRun {
                            zkvm_name: (*zkvm_name).clone(),
                            mode: mode.clone(),
                            scale: *scale,
                            repeat,
                        };

                        tasks.push(((*zkvm_name).clone(), (*zkvm_config).clone(), test_run));
                    }
                }
            }
        }

        tasks
    }

    /// Clone for task execution
    fn clone_for_task(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            output_dir: self.output_dir.clone(),
            exec_config: self.exec_config.clone(),
            command_parser: Arc::clone(&self.command_parser),
        }
    }

    /// Run a single test task
    async fn run_single_task(
        &self,
        zkvm_name: &str,
        zkvm_config: &ZkVmConfig,
        test_run: &TestRun,
    ) -> Result<ExecutionResult> {
        // Prepare env vars
        let mut env_vars = zkvm_config.env_vars.clone().unwrap_or_default();
        env_vars.insert("FIBONACCI_N".to_string(), test_run.scale.to_string());
        env_vars.insert(
            format!("{}_PROOF_MODE", zkvm_name.to_uppercase()),
            test_run.mode.clone(),
        );
        env_vars
            .entry("RUST_LOG".to_string())
            .or_insert("debug".to_string());

        let work_dir = Path::new(&zkvm_config.working_dir);

        // Run build command if specified
        if let Some(build_cmd) = &zkvm_config.build_command {
            if let Err(e) = self.run_command(build_cmd, work_dir, &env_vars).await {
                warn!("Build warning: {}", e);
            }
        }

        // Execute benchmark
        let timeout_secs = self.config.timeout_seconds.unwrap_or(3600);

        let output_result = timeout(
            Duration::from_secs(timeout_secs),
            self.run_command_with_monitoring(
                &zkvm_config.run_command,
                work_dir,
                &env_vars,
                self.exec_config.enable_resource_monitoring,
            ),
        )
        .await;

        let (log_content, resource_stats) = match output_result {
            Ok(Ok((output, stats))) => (output, stats),
            Ok(Err(e)) => {
                return Ok(ExecutionResult {
                    test_run: test_run.clone(),
                    metrics: None,
                    log_content: String::new(),
                    success: false,
                    error: Some(format!("Execution failed: {}", e)),
                    resource_stats: None,
                });
            }
            Err(_) => {
                return Ok(ExecutionResult {
                    test_run: test_run.clone(),
                    metrics: None,
                    log_content: String::new(),
                    success: false,
                    error: Some(format!("Timeout after {} seconds", timeout_secs)),
                    resource_stats: None,
                });
            }
        };

        // Save raw log (strip ANSI escape codes for clean text output)
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
        let log_filename = format!(
            "{}-{}-{}-scale{}.log",
            zkvm_name, test_run.mode, timestamp, test_run.scale
        );
        let log_path = self.output_dir.join("raw-logs").join(&log_filename);
        let cleaned_log = strip_ansi_codes(&log_content);
        fs::write(&log_path, &cleaned_log)?;

        // Parse metrics
        let parser = LogParser::new(&zkvm_config.parsed_metrics)?;
        let program_name = format!("fibonacci_{}", test_run.scale);

        // Pass metric mapping to parser
        let metrics = match parser.parse(
            &log_content, 
            zkvm_name, 
            &program_name, 
            Some(&zkvm_config.metric_mapping)
        ) {
            Ok(m) => {
                let metrics_filename = format!(
                    "{}-{}-{}-{}.json",
                    zkvm_name, test_run.mode, timestamp, program_name
                );
                let metrics_path = self
                    .output_dir
                    .join("parsed-metrics")
                    .join(&metrics_filename);
                let metrics_json = serde_json::to_string_pretty(&m)?;
                fs::write(&metrics_path, metrics_json)?;

                Some(m)
            }
            Err(e) => {
                warn!("Failed to parse metrics: {}", e);
                None
            }
        };

        let success = metrics.is_some();

        Ok(ExecutionResult {
            test_run: test_run.clone(),
            metrics,
            log_content,
            success,
            error: None,
            resource_stats,
        })
    }

    /// Run command helper
    async fn run_command(
        &self,
        command: &str,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
    ) -> Result<String> {
        self.run_command_with_monitoring(command, work_dir, env_vars, false)
            .await
            .map(|(output, _)| output)
    }

    /// Run command with monitoring helper
    async fn run_command_with_monitoring(
        &self,
        command: &str,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
        enable_monitoring: bool,
    ) -> Result<(String, Option<ResourceStats>)> {
        // Use command parser
        let parsed = self.command_parser.parse(command)?;

        let mut cmd = Command::new(&parsed.program);
        cmd.current_dir(work_dir);
        cmd.args(&parsed.args);
        cmd.envs(env_vars);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| BenchmarkError::Execution(format!("Failed to spawn: {}", e)))?;

        if enable_monitoring {
            if let Some(child_id) = child.id() {
                let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);

                let monitor_handle =
                    tokio::spawn(
                        async move { monitor_process_async(child_id, 100, stop_rx).await },
                    );

                let output = child
                    .wait_with_output()
                    .await
                    .map_err(|e| BenchmarkError::Execution(format!("Failed to wait: {}", e)))?;

                let _ = stop_tx.send(true);

                let stats = monitor_handle.await.map_err(|e| {
                    BenchmarkError::Execution(format!("Monitor task failed: {}", e))
                })?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let combined = format!("{}\n{}", stdout, stderr);

                if !output.status.success() {
                    warn!("Command exited with status: {}", output.status);
                }

                Ok((combined, Some(stats)))
            } else {
                let output = child
                    .wait_with_output()
                    .await
                    .map_err(|e| BenchmarkError::Execution(format!("Failed to wait: {}", e)))?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let combined = format!("{}\n{}", stdout, stderr);

                if !output.status.success() {
                    warn!("Command exited with status: {}", output.status);
                }

                Ok((combined, None))
            }
        } else {
            let output = child
                .wait_with_output()
                .await
                .map_err(|e| BenchmarkError::Execution(format!("Failed to wait: {}", e)))?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}\n{}", stdout, stderr);

            if !output.status.success() {
                warn!("Command exited with status: {}", output.status);
            }

            Ok((combined, None))
        }
    }

    /// Get output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_config_default() {
        let config = ParallelExecutionConfig::default();
        assert_eq!(config.max_concurrent_zkvms, 1);
        assert_eq!(config.max_concurrent_tests_per_zkvm, 1);
    }

    #[test]
    fn test_parallel_config_auto() {
        let config = ParallelExecutionConfig::auto_parallel();
        assert!(config.max_concurrent_zkvms > 0);
        assert!(config.max_concurrent_tests_per_zkvm > 0);
    }

    #[test]
    fn test_parallel_config_conservative() {
        let config = ParallelExecutionConfig::conservative();
        assert_eq!(config.max_concurrent_zkvms, 1);
        assert_eq!(config.max_concurrent_tests_per_zkvm, 1);
        assert!(!config.parallel_across_zkvms);
    }

    #[test]
    fn test_parallel_config_aggressive() {
        let config = ParallelExecutionConfig::aggressive();
        assert!(config.max_concurrent_zkvms > 1);
        assert!(config.max_concurrent_tests_per_zkvm > 1);
        assert!(config.parallel_across_zkvms);
    }
}
