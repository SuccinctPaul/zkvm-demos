//! Benchmark executor - runs zkVM benchmarks and collects metrics

use chrono;
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

use crate::config::{BenchmarkConfig, ZkVmConfig};
use crate::error::{BenchmarkError, Result};
use crate::log_parser::LogParser;
use crate::metrics::BenchmarkMetrics;
use crate::resource_monitor::{monitor_process_async, ResourceStats};

pub struct BenchmarkExecutor {
    config: BenchmarkConfig,
    output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TestRun {
    pub zkvm_name: String,
    pub mode: String,
    pub scale: u32,
    pub repeat: u32,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub test_run: TestRun,
    pub metrics: Option<BenchmarkMetrics>,
    pub log_content: String,
    pub success: bool,
    pub error: Option<String>,
    pub resource_stats: Option<ResourceStats>,
}

impl BenchmarkExecutor {
    pub fn new(config: BenchmarkConfig) -> Result<Self> {
        let output_dir = PathBuf::from(&config.output_dir);

        // Create output directories
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(output_dir.join("raw-logs"))?;
        fs::create_dir_all(output_dir.join("parsed-metrics"))?;
        fs::create_dir_all(output_dir.join("reports"))?;

        Ok(Self { config, output_dir })
    }

    /// Run all benchmarks
    pub async fn run_all(&self) -> Result<Vec<ExecutionResult>> {
        info!("Starting benchmark execution");

        let mut all_results = Vec::new();
        let enabled_zkvms = self.config.enabled_zkvms();

        info!("Enabled zkVMs: {}", enabled_zkvms.len());

        for (zkvm_name, zkvm_config) in enabled_zkvms {
            info!("Running benchmarks for zkVM: {}", zkvm_name);

            for scale in &self.config.test_scales {
                let repeat_count = self.config.repeat_count.unwrap_or(1);

                for repeat in 1..=repeat_count {
                    for mode in &zkvm_config.test_modes {
                        let test_run = TestRun {
                            zkvm_name: zkvm_name.clone(),
                            mode: mode.clone(),
                            scale: *scale,
                            repeat,
                        };

                        info!(
                            "Running: {} mode={} scale={} repeat={}/{}",
                            zkvm_name, mode, scale, repeat, repeat_count
                        );

                        let result = self.run_single(zkvm_name, zkvm_config, &test_run).await;

                        match result {
                            Ok(exec_result) => {
                                if exec_result.success {
                                    info!("✓ Completed successfully");
                                } else {
                                    warn!("✗ Failed: {:?}", exec_result.error);
                                }
                                all_results.push(exec_result);
                            }
                            Err(e) => {
                                error!("✗ Error: {}", e);
                                all_results.push(ExecutionResult {
                                    test_run,
                                    metrics: None,
                                    log_content: String::new(),
                                    success: false,
                                    error: Some(e.to_string()),
                                    resource_stats: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        info!("Completed {} test runs", all_results.len());
        Ok(all_results)
    }

    /// Run a single benchmark
    async fn run_single(
        &self,
        zkvm_name: &str,
        zkvm_config: &ZkVmConfig,
        test_run: &TestRun,
    ) -> Result<ExecutionResult> {
        // Prepare environment variables
        let mut env_vars = zkvm_config.env_vars.clone().unwrap_or_default();
        env_vars.insert("FIBONACCI_N".to_string(), test_run.scale.to_string());
        env_vars.insert(
            format!("{}_PROOF_MODE", zkvm_name.to_uppercase()),
            test_run.mode.clone(),
        );

        // Set RUST_LOG to debug for capturing detailed metrics from zkVM logs
        env_vars
            .entry("RUST_LOG".to_string())
            .or_insert("debug".to_string());

        // Log environment variables for debugging
        info!("Environment variables:");
        for (key, value) in &env_vars {
            info!("  {}={}", key, value);
        }

        // Build working directory path
        let work_dir = Path::new(&zkvm_config.working_dir);

        // Run build command if specified
        if let Some(build_cmd) = &zkvm_config.build_command {
            info!("Building: {}", build_cmd);
            let build_result = self.run_command(build_cmd, work_dir, &env_vars).await;
            if let Err(e) = build_result {
                warn!("Build warning: {}", e);
            }
        }

        // Run the benchmark with resource monitoring enabled
        info!(
            "Executing: {} in directory: {:?}",
            zkvm_config.run_command, work_dir
        );
        let timeout_secs = self.config.timeout_seconds.unwrap_or(3600);

        let output_result = timeout(
            Duration::from_secs(timeout_secs),
            self.run_command_with_monitoring(&zkvm_config.run_command, work_dir, &env_vars, true),
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

        // Save raw log with improved naming: zkvm-mode-timestamp-scaleN.log
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
        let log_filename = format!(
            "{}-{}-{}-scale{}.log",
            zkvm_name, test_run.mode, timestamp, test_run.scale
        );
        let log_path = self.output_dir.join("raw-logs").join(&log_filename);
        fs::write(&log_path, &log_content)?;
        info!("Saved log: {}", log_path.display());

        // Parse metrics
        let parser = LogParser::new(&zkvm_config.log_patterns)?;
        let program_name = format!("fibonacci_{}", test_run.scale);

        let metrics = match parser.parse(&log_content, zkvm_name, &program_name) {
            Ok(m) => {
                // Save parsed metrics with improved naming: zkvm-mode-timestamp-scaleN.json
                let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
                let metrics_filename = format!(
                    "{}-{}-{}-scale{}.json",
                    zkvm_name, test_run.mode, timestamp, test_run.scale
                );
                let metrics_path = self
                    .output_dir
                    .join("parsed-metrics")
                    .join(&metrics_filename);
                let metrics_json = serde_json::to_string_pretty(&m)?;
                fs::write(&metrics_path, metrics_json)?;
                info!("Saved metrics: {}", metrics_path.display());

                Some(m)
            }
            Err(e) => {
                warn!("Failed to parse metrics: {}", e);
                None
            }
        };

        let success = metrics.is_some();

        // Log resource stats if available
        if let Some(ref stats) = resource_stats {
            info!(
                "Resource usage: Peak Memory={:.1}MB, Avg CPU={:.1}%, Duration={:.1}s",
                stats.peak_memory_mb, stats.avg_cpu_percent, stats.duration_s
            );
        }

        Ok(ExecutionResult {
            test_run: test_run.clone(),
            metrics,
            log_content,
            success,
            error: None,
            resource_stats,
        })
    }

    /// Run a command and capture output
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

    /// Run a command with optional resource monitoring
    async fn run_command_with_monitoring(
        &self,
        command: &str,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
        enable_monitoring: bool,
    ) -> Result<(String, Option<ResourceStats>)> {
        // Parse command (simple splitting by spaces, could be improved)
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Err(BenchmarkError::Execution("Empty command".to_string()));
        }

        let mut cmd = Command::new(parts[0]);
        cmd.current_dir(work_dir);
        cmd.args(&parts[1..]);
        cmd.envs(env_vars);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .map_err(|e| BenchmarkError::Execution(format!("Failed to spawn: {}", e)))?;

        // Start resource monitoring if enabled
        if enable_monitoring {
            if let Some(child_id) = child.id() {
                let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);

                let monitor_handle =
                    tokio::spawn(
                        async move { monitor_process_async(child_id, 100, stop_rx).await },
                    );

                // Wait for process to complete
                let output = child
                    .wait_with_output()
                    .await
                    .map_err(|e| BenchmarkError::Execution(format!("Failed to wait: {}", e)))?;

                // Stop monitoring
                let _ = stop_tx.send(true);

                // Get monitoring results
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
                // Fallback: no monitoring available
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
            // No monitoring
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

    #[tokio::test]
    async fn test_executor_creation() {
        let config = BenchmarkConfig::example();
        let executor = BenchmarkExecutor::new(config);
        assert!(executor.is_ok());
    }
}
