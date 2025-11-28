//! Benchmark executor - runs zkVM benchmarks and collects metrics
//!
//! Runs benchmarks sequentially for maximum accuracy.

use crate::analysis::log_parser::LogParser;
use crate::core::config::{BenchmarkConfig, ZkVmConfig};
use crate::core::error::{BenchmarkError, Result};
use crate::core::metrics::{ProgramName, ProofMode, ZkVmName};
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
use log::{error, info, warn};
use regex::Regex;
use tokio::process::Command;
use tokio::time::timeout;

/// Remove ANSI escape sequences from a string
/// This cleans up terminal color codes and formatting from log output
fn strip_ansi_codes(text: &str) -> String {
    // Match ANSI escape sequences like \x1b[0m, \x1b[34m, etc.
    // Pattern: ESC [ (any number of digits/semicolons) followed by a letter
    let re = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    re.replace_all(text, "").to_string()
}

/// Check if log content contains failure patterns (panic, crash, etc.)
fn detect_failure_in_log(log_content: &str) -> Option<String> {
    // Common failure patterns in zkVM benchmark logs
    let failure_patterns = [
        ("panicked at", "Program panicked"),
        ("thread 'main' panicked", "Main thread panicked"),
        ("Verification failed", "Verification failed"),
        ("Error:", "Error encountered"),
        ("FATAL ERROR", "Fatal error"),
        ("could not open elf file", "ELF file not found"),
        ("failed to compile", "Compilation failed"),
        ("stack overflow", "Stack overflow"),
    ];

    for (pattern, description) in failure_patterns {
        if log_content.contains(pattern) {
            return Some(description.to_string());
        }
    }

    // Also check for exit status pattern in stderr
    if log_content.contains("exit code: 1") || log_content.contains("exited with status: 1") {
        return Some("Non-zero exit code".to_string());
    }

    None
}

/// Benchmark Executor
pub struct BenchmarkExecutor {
    config: Arc<BenchmarkConfig>,
    output_dir: PathBuf,
    command_parser: Arc<CommandParser>,
}

impl BenchmarkExecutor {
    /// Create a new benchmark executor
    pub fn new(config: BenchmarkConfig) -> Result<Self> {
        let output_dir = PathBuf::from(&config.output_dir);

        // Create output directories
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(output_dir.join("raw-logs"))?;
        fs::create_dir_all(output_dir.join("parsed-metrics"))?;
        fs::create_dir_all(output_dir.join("reports"))?;

        Ok(Self {
            config: Arc::new(config),
            output_dir,
            command_parser: Arc::new(CommandParser::new()),
        })
    }

    /// Run all benchmarks
    pub async fn run_all(&self) -> Result<Vec<ExecutionResult>> {
        info!("🚀 Starting benchmark execution");
        info!("📊 Output directory: {}", self.output_dir.display());
        info!("⚙️  Execution Strategy: Sequential (for accurate benchmarking)");

        let enabled_zkvms = self.config.enabled_zkvms();

        if enabled_zkvms.is_empty() {
            return Err(BenchmarkError::Config("No enabled zkVMs found".to_string()));
        }

        info!(
            "📋 Enabled zkVMs: {}",
            enabled_zkvms
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Build all test tasks
        let test_tasks = self.build_test_tasks(&enabled_zkvms);
        info!("📝 Prepared {} test tasks", test_tasks.len());

        // Execute all tasks sequentially
        let mut results = Vec::new();

        for (zkvm_name, zkvm_config, test_run) in test_tasks {
            info!(
                "▶ Running: {} mode={} scale={} repeat={}/{}",
                test_run.zkvm_name,
                test_run.mode,
                test_run.scale,
                test_run.repeat,
                self.config.repeat_count.unwrap_or(1)
            );

            let result = self
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
                    results.push(exec_result);
                }
                Err(e) => {
                    error!(
                        "✗ Error: {} {} scale={}: {}",
                        zkvm_name, test_run.mode, test_run.scale, e
                    );
                    results.push(ExecutionResult {
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
    /// Uses only the highest priority proof mode available for each zkVM
    /// Priority: Groth16 = Plonk > Compressed > Core
    fn build_test_tasks(
        &self,
        enabled_zkvms: &[(&String, &ZkVmConfig)],
    ) -> Vec<(String, ZkVmConfig, TestRun)> {
        let mut tasks = Vec::new();

        for (zkvm_name, zkvm_config) in enabled_zkvms {
            // Parse zkvm_name string to ZkVmName enum
            let zkvm_name_enum = zkvm_name
                .parse::<ZkVmName>()
                .expect("Unknown zkVM name in configuration");

            // Get programs from config (with fallback to default)
            let programs = zkvm_config.get_programs();
            let repeat_count = self.config.repeat_count.unwrap_or(1);

            // Use only the highest priority proof mode
            // Priority: Groth16 > Plonk > Compressed > Core
            let highest_mode = ProofMode::highest_from(&zkvm_config.prove_modes);
            info!(
                "  📌 {} using highest proof mode: {} (from {:?})",
                zkvm_name, highest_mode, zkvm_config.prove_modes
            );

            for program_config in &programs {
                // Parse program name
                let program_name = program_config
                    .name
                    .parse::<ProgramName>()
                    .unwrap_or_else(|_| ProgramName::Custom(program_config.name.clone()));

                for scale in &program_config.scales {
                    for repeat in 1..=repeat_count {
                        let test_run = TestRun {
                            zkvm_name: zkvm_name_enum.clone(),
                            program_name: program_name.clone(),
                            mode: highest_mode.clone(),
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

    /// Run a single test task
    async fn run_single_task(
        &self,
        zkvm_name: &str, // Keep string arg for now as it's key in map, or change to ZkVmName? Logic uses it for config lookup? No, config is passed.
        zkvm_config: &ZkVmConfig,
        test_run: &TestRun,
    ) -> Result<ExecutionResult> {
        info!("  📍 Working directory: {}", zkvm_config.working_dir);

        // Prepare env vars
        let mut env_vars = zkvm_config.env_vars.clone().unwrap_or_default();

        // Add program-specific env vars
        let programs = zkvm_config.get_programs();
        if let Some(program_config) = programs.iter().find(|p| {
            p.name
                .parse::<ProgramName>()
                .map(|pn| pn == test_run.program_name)
                .unwrap_or(false)
        }) {
            if let Some(ref prog_env_vars) = program_config.env_vars {
                env_vars.extend(prog_env_vars.clone());
            }
        }

        // Add program ID and input parameter
        env_vars.insert(
            "PROGRAM_ID".to_string(),
            test_run.program_name.program_id().to_string(),
        );
        // For backward compatibility, also set FIBONACCI_N if it's Fibonacci
        match test_run.program_name {
            ProgramName::Fibonacci => {
                env_vars.insert("FIBONACCI_N".to_string(), test_run.scale.to_string());
            }
            _ => {
                // Generic parameter name for other programs
                env_vars.insert("PROGRAM_N".to_string(), test_run.scale.to_string());
            }
        }

        env_vars.insert(
            format!("{}_PROOF_MODE", zkvm_name.to_uppercase()),
            test_run.mode.to_string(),
        );
        env_vars
            .entry("RUST_LOG".to_string())
            .or_insert("debug".to_string());

        let work_dir = Path::new(&zkvm_config.working_dir);

        // 1. Build zkVM
        if let Err(e) = self
            .build_zkvm(zkvm_name, zkvm_config, work_dir, &env_vars)
            .await
        {
            error!("  ❌ Build {zkvm_name} failed: {}", e);
            return Ok(ExecutionResult {
                test_run: test_run.clone(),
                metrics: None,
                log_content: format!("Build failed: {}", e),
                success: false,
                error: Some(format!("Build failed: {}", e)),
                resource_stats: None,
            });
        }

        // 2. Execute zkVM
        let execution_result = self
            .execute_zkvm(zkvm_name, zkvm_config, work_dir, &env_vars, test_run)
            .await;

        let (log_content, resource_stats) = match execution_result {
            Ok(res) => res,
            Err(e) => {
                error!("  ❌ Execution failed: {}", e);
                return Ok(ExecutionResult {
                    test_run: test_run.clone(),
                    metrics: None,
                    log_content: String::new(),
                    success: false,
                    error: Some(format!("Execution failed: {}", e)),
                    resource_stats: None,
                });
            }
        };

        // 3. Process and Parse Log
        info!("  💾 Saving raw log...");
        // Save raw log (strip ANSI escape codes for clean text output)
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
        // Format: Timestamp_zkvm_provemode_programName_param
        let program_name = test_run.program_name.to_string();
        let param = test_run.scale.to_string();

        let log_filename = format!(
            "{}_{}_{}_{}_{}.log",
            timestamp, zkvm_name, test_run.mode, program_name, param
        );
        let log_path = self.output_dir.join("raw-logs").join(&log_filename);
        let cleaned_log = strip_ansi_codes(&log_content);
        fs::write(&log_path, &cleaned_log)?;
        info!("  📄 Raw log saved: {}", log_path.display());

        let metrics_result = self.process_log(
            &cleaned_log,
            &test_run.zkvm_name,
            zkvm_config,
            &test_run.mode,
            &test_run.program_name,
            &param,
            resource_stats.as_ref(),
        );

        // Save metrics to file
        if let Ok(Some(ref m)) = metrics_result {
            let metrics_filename = format!(
                "{}_{}_{}_{}_{}.json",
                timestamp, zkvm_name, test_run.mode, program_name, param
            );
            let metrics_path = self
                .output_dir
                .join("parsed-metrics")
                .join(&metrics_filename);
            if let Ok(metrics_json) = serde_json::to_string_pretty(m) {
                let _ = fs::write(&metrics_path, metrics_json);
                info!("  💾 Metrics saved: {}", metrics_path.display());
            }
        }

        let (metrics, error) = match metrics_result {
            Ok(m) => (m, None),
            Err(e) => (None, Some(e.to_string())),
        };

        // Check for failure patterns in the log content
        let log_failure = detect_failure_in_log(&cleaned_log);

        // Determine success: must have metrics AND no failure patterns in log
        let success = if let Some(failure_reason) = &log_failure {
            warn!("  ⚠️  Detected failure in log: {}", failure_reason);
            false
        } else {
            metrics.is_some()
        };

        // If we detected a failure, ensure error is set
        let error = error.or_else(|| log_failure);

        Ok(ExecutionResult {
            test_run: test_run.clone(),
            metrics,
            log_content,
            success,
            error,
            resource_stats,
        })
    }

    /// Step 1: Build zkVM project
    async fn build_zkvm(
        &self,
        zkvm_name: &str,
        zkvm_config: &ZkVmConfig,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
    ) -> Result<()> {
        if let Some(build_cmd) = &zkvm_config.build_command {
            info!("  🔨 Building {zkvm_name}: {}", build_cmd);
            self.run_command(build_cmd, work_dir, env_vars).await?;
            info!("  ✅ Build {zkvm_name} completed");
        }
        Ok(())
    }

    /// Step 2: Execute zkVM benchmark
    async fn execute_zkvm(
        &self,
        zkvm_name: &str,
        zkvm_config: &ZkVmConfig,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
        test_run: &TestRun,
    ) -> Result<(String, Option<ResourceStats>)> {
        // Use program-specific timeout if available, otherwise zkVM timeout, otherwise config timeout
        let timeout_secs = {
            let programs = zkvm_config.get_programs();
            if let Some(program_config) = programs.iter().find(|p| {
                p.name
                    .parse::<ProgramName>()
                    .map(|pn| pn == test_run.program_name)
                    .unwrap_or(false)
            }) {
                program_config
                    .timeout_seconds
                    .or(zkvm_config.timeout_seconds)
                    .or(self.config.timeout_seconds)
                    .unwrap_or(3600)
            } else {
                zkvm_config
                    .timeout_seconds
                    .or(self.config.timeout_seconds)
                    .unwrap_or(3600)
            }
        };
        info!("  ⏱️  Executing timeout: {} seconds", timeout_secs);
        info!("  ▶️  Executing {zkvm_name}: {}", zkvm_config.run_command);

        let output_result = timeout(
            Duration::from_secs(timeout_secs),
            self.run_command_with_monitoring(
                &zkvm_config.run_command,
                work_dir,
                env_vars,
                true, // Always enable resource monitoring in sequential mode
            ),
        )
        .await;

        match output_result {
            Ok(Ok((output, stats))) => {
                info!("  ✅ Execution completed");
                if let Some(ref stats) = stats {
                    info!(
                        "  📊 Resource stats: peak_memory={:.1}MB, avg_cpu={:.1}%",
                        stats.peak_memory_mb, stats.avg_cpu_percent
                    );
                }
                Ok((output, stats))
            }
            Ok(Err(e)) => Err(BenchmarkError::Execution(format!(
                "Execution failed: {}",
                e
            ))),
            Err(_) => Err(BenchmarkError::Execution(format!(
                "Timeout after {} seconds",
                timeout_secs
            ))),
        }
    }

    /// Step 3: Process and Parse Log
    fn process_log(
        &self,
        log_content: &str,
        zkvm_name: &ZkVmName,
        zkvm_config: &ZkVmConfig,
        mode: &ProofMode,
        program_name: &ProgramName,
        param: &str,
        resource_stats: Option<&ResourceStats>,
    ) -> Result<Option<crate::core::metrics::UnifiedMetrics>> {
        info!("  🔍 Parsing metrics...");
        let full_program_name = format!("{}_{}", program_name, param);

        // Parse metrics
        let parser = LogParser::new(&zkvm_config.parsed_metrics)?;

        // Pass metric mapping to parser
        match parser.parse(
            log_content,
            &zkvm_name.to_string(),
            &full_program_name,
            Some(&zkvm_config.metric_mapping),
        ) {
            Ok(mut m) => {
                info!("  ✅ Metrics parsed successfully");

                // Set metadata
                m.metadata.zkvm_name = zkvm_name.clone();
                m.metadata.zkvm_version = zkvm_config.version.clone();
                m.metadata.program_name = program_name.clone();
                m.metadata.mode = Some(mode.clone());
                if let Ok(s) = param.parse::<u32>() {
                    m.metadata.scale = Some(s);
                }

                // Merge resource stats into metrics if available
                if let Some(stats) = resource_stats {
                    m.resources.peak_memory_mb = Some(stats.peak_memory_mb);
                    m.resources.avg_cpu_usage_percent = Some(stats.avg_cpu_percent);
                }

                Ok(Some(m))
            }
            Err(e) => {
                warn!("  ⚠️  Failed to parse metrics: {}", e);
                // We don't return error here to allow flow to continue, just return None
                Ok(None)
            }
        }
    }

    /// Run command helper
    async fn run_command(
        &self,
        command: &str,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
    ) -> Result<String> {
        let (output, _) = self
            .run_command_with_monitoring(command, work_dir, env_vars, false)
            .await?;

        // Check if command succeeded - run_command_with_monitoring returns output even on failure
        // We need to check the output for error patterns or rely on the caller to check
        // For now, we return the output and let the caller decide
        Ok(output)
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

                let output_res = child.wait_with_output().await;

                // Always stop monitoring immediately
                let _ = stop_tx.send(true);

                // Wait for monitor with timeout to prevent hanging
                let stats = match tokio::time::timeout(Duration::from_secs(2), monitor_handle).await
                {
                    Ok(Ok(s)) => Some(s),
                    Ok(Err(e)) => {
                        warn!("Monitor task error: {}", e);
                        None
                    }
                    Err(_) => {
                        warn!("Monitor task timeout, continuing without stats");
                        None
                    }
                };

                match output_res {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        let combined = format!("{}\n{}", stdout, stderr);

                        if !output.status.success() {
                            warn!("Command exited with status: {}", output.status);
                        }

                        Ok((combined, stats))
                    }
                    Err(e) => Err(BenchmarkError::Execution(format!("Failed to wait: {}", e))),
                }
            } else {
                warn!("  ⚠️  Process ID not available, resource monitoring disabled");
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
                let exit_code = output.status.code().unwrap_or(-1);
                return Err(BenchmarkError::Execution(format!(
                    "Command exited with status {}: {}",
                    exit_code,
                    if combined.len() > 500 {
                        format!("{}...", &combined[..500])
                    } else {
                        combined.clone()
                    }
                )));
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
    use crate::core::config::{ParsedMetrics, ZkVmConfig};
    use crate::core::metrics::{ProgramName, ProofMode, ZkVmName};
    use std::collections::HashMap;

    #[test]
    fn test_process_log_parsing() {
        // Mock configuration
        let mut patterns = HashMap::new();
        patterns.insert(
            "total_cycles".to_string(),
            r"(?:BENCHMARK: |execution report \(totals\): )total_cycles=(\d+)".to_string(),
        );
        patterns.insert(
            "execution_time_s".to_string(),
            r"BENCHMARK: execution_time_s=([\d.]+)".to_string(),
        );
        patterns.insert(
            "total_prove_time_s".to_string(),
            r"BENCHMARK: total_prove_time_s=([\d.]+)".to_string(),
        );

        let parsed_metrics = ParsedMetrics { patterns };

        // Create a default ZkVmConfig (manually since it might not derive Default)
        let zkvm_config = ZkVmConfig {
            name: Some(ZkVmName::Sp1),
            version: None,
            enabled: true,
            default_mode: ProofMode::Groth16,
            prove_modes: vec![ProofMode::Groth16],
            programs: None,
            test_scales: None,
            working_dir: ".".to_string(),
            build_command: None,
            run_command: "echo test".to_string(),
            timeout_seconds: None,
            repeat_count: None,
            env_vars: None,
            parsed_metrics,
            metric_mapping: HashMap::new(),
            stage_merge: None,
            proof_size_config: None,
        };

        // Use sample log content (excerpt from 20251122-154058_sp1_groth16_fibonacci_20.log)
        let log_content = r#"
2025-11-22T07:38:50.169554Z  INFO execute: close time.busy=5.29ms time.idle=1.33µs

--- Execution Phase ---
BENCHMARK: total_cycles=20
BENCHMARK: total_instruction_count=203259
BENCHMARK: execution_time_s=0.005315

--- Proving Phase (mode: Groth16) ---
Running full Groth16 pipeline with detailed timing...
BENCHMARK: total_prove_time_s=128.159632
BENCHMARK: groth16_proof_size_bytes=260
BENCHMARK: final_proof_size_bytes=260
        "#;

        // Setup Executor with a temporary output directory
        let temp_dir = std::env::temp_dir().join("zkvm_test_executor");
        let config = BenchmarkConfig {
            test_scales: vec![10],
            zkvms: HashMap::new(),
            output_dir: temp_dir.to_string_lossy().to_string(),
            timeout_seconds: Some(10),
            repeat_count: Some(1),
        };
        let executor = BenchmarkExecutor::new(config).expect("Failed to create executor");

        let result = executor
            .process_log(
                log_content,
                &ZkVmName::Sp1,
                &zkvm_config,
                &ProofMode::Groth16,
                &ProgramName::Fibonacci,
                "20",
                None,
            )
            .expect("Failed to process log");

        assert!(result.is_some(), "Metrics should be parsed");
        let metrics = result.unwrap();

        assert_eq!(metrics.metadata.zkvm_name, ZkVmName::Sp1);
        assert_eq!(metrics.metadata.program_name, ProgramName::Fibonacci);
        assert_eq!(metrics.metadata.mode, Some(ProofMode::Groth16));
        assert_eq!(metrics.metadata.scale, Some(20));

        // Check values in custom_metrics
        assert_eq!(
            metrics.custom_metrics.get("total_cycles"),
            Some(&"20".to_string())
        );
        assert_eq!(
            metrics.custom_metrics.get("execution_time_s"),
            Some(&"0.005315".to_string())
        );
        assert_eq!(
            metrics.custom_metrics.get("total_prove_time_s"),
            Some(&"128.159632".to_string())
        );

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
