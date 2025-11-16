//! 并行执行模块 - Phase 2 优化
//!
//! 提供高性能的并行 benchmark 执行能力

use crate::command_parser::CommandParser;
use crate::config::{BenchmarkConfig, ZkVmConfig};
use crate::error::{BenchmarkError, Result};
use crate::executor::{ExecutionResult, TestRun};
use crate::log_parser::LogParser;
use crate::resource_monitor::{monitor_process_async, ResourceStats};

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;

use chrono;
use futures::stream::{self, StreamExt};
use log::{error, info, warn};
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

/// 并行执行配置
#[derive(Debug, Clone)]
pub struct ParallelExecutionConfig {
    /// 最大并发 zkVM 数量
    pub max_concurrent_zkvms: usize,
    /// 单个 zkVM 的最大并发测试数
    pub max_concurrent_tests_per_zkvm: usize,
    /// 是否允许跨 zkVM 并行
    pub parallel_across_zkvms: bool,
    /// 是否启用资源监控
    pub enable_resource_monitoring: bool,
}

impl ParallelExecutionConfig {
    /// 创建默认配置（根据 CPU 核心数自动调整）
    pub fn default() -> Self {
        let cpu_count = num_cpus::get();

        Self {
            // 避免资源争用，限制并发数
            max_concurrent_zkvms: (cpu_count / 2).max(1).min(4),
            max_concurrent_tests_per_zkvm: 2,
            parallel_across_zkvms: true,
            enable_resource_monitoring: true,
        }
    }

    /// 创建保守配置（适合资源受限环境）
    pub fn conservative() -> Self {
        Self {
            max_concurrent_zkvms: 1,
            max_concurrent_tests_per_zkvm: 1,
            parallel_across_zkvms: false,
            enable_resource_monitoring: true,
        }
    }

    /// 创建激进配置（适合高性能服务器）
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

/// 并行 Benchmark 执行器
pub struct ParallelBenchmarkExecutor {
    config: Arc<BenchmarkConfig>,
    output_dir: PathBuf,
    exec_config: ParallelExecutionConfig,
    command_parser: Arc<CommandParser>,
}

impl ParallelBenchmarkExecutor {
    /// 创建新的并行执行器
    pub fn new(config: BenchmarkConfig, exec_config: ParallelExecutionConfig) -> Result<Self> {
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

    /// 使用默认并行配置创建
    pub fn with_default_parallelism(config: BenchmarkConfig) -> Result<Self> {
        Self::new(config, ParallelExecutionConfig::default())
    }

    /// 并行执行所有 benchmarks
    pub async fn run_all_parallel(&self) -> Result<Vec<ExecutionResult>> {
        info!("Starting parallel benchmark execution");
        info!(
            "Parallelism: {} concurrent zkVMs, {} tests per zkVM",
            self.exec_config.max_concurrent_zkvms, self.exec_config.max_concurrent_tests_per_zkvm
        );

        let enabled_zkvms = self.config.enabled_zkvms();

        if enabled_zkvms.is_empty() {
            return Err(BenchmarkError::Config("No enabled zkVMs found".to_string()));
        }

        // 构建所有测试任务
        let test_tasks = self.build_test_tasks(&enabled_zkvms);
        let total_tasks = test_tasks.len();

        info!("Prepared {} test tasks", total_tasks);

        // 创建信号量控制并发
        let semaphore = Arc::new(Semaphore::new(self.exec_config.max_concurrent_zkvms));

        // 并行执行所有任务
        let results: Vec<ExecutionResult> = stream::iter(test_tasks)
            .map(|(zkvm_name, zkvm_config, test_run)| {
                let sem = Arc::clone(&semaphore);
                let executor = self.clone_for_task();

                async move {
                    // 获取并发许可
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

        // 统计成功/失败
        let successful = results.iter().filter(|r| r.success).count();
        let failed = results.len() - successful;

        if failed > 0 {
            warn!("⚠ {} tests failed out of {}", failed, results.len());
        }

        Ok(results)
    }

    /// 构建所有测试任务
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

    /// 克隆用于任务执行
    fn clone_for_task(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            output_dir: self.output_dir.clone(),
            exec_config: self.exec_config.clone(),
            command_parser: Arc::clone(&self.command_parser),
        }
    }

    /// 执行单个测试任务
    async fn run_single_task(
        &self,
        zkvm_name: &str,
        zkvm_config: &ZkVmConfig,
        test_run: &TestRun,
    ) -> Result<ExecutionResult> {
        // 准备环境变量
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

        // 运行构建命令（如果指定）
        if let Some(build_cmd) = &zkvm_config.build_command {
            if let Err(e) = self.run_command(build_cmd, work_dir, &env_vars).await {
                warn!("Build warning: {}", e);
            }
        }

        // 执行 benchmark
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

        // 保存原始日志
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
        let log_filename = format!(
            "{}-{}-{}-scale{}.log",
            zkvm_name, test_run.mode, timestamp, test_run.scale
        );
        let log_path = self.output_dir.join("raw-logs").join(&log_filename);
        fs::write(&log_path, &log_content)?;

        // 解析指标
        let parser = LogParser::new(&zkvm_config.log_patterns)?;
        let program_name = format!("fibonacci_{}", test_run.scale);

        let metrics = match parser.parse(&log_content, zkvm_name, &program_name) {
            Ok(m) => {
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

    /// 运行命令
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

    /// 运行命令（带资源监控）
    async fn run_command_with_monitoring(
        &self,
        command: &str,
        work_dir: &Path,
        env_vars: &HashMap<String, String>,
        enable_monitoring: bool,
    ) -> Result<(String, Option<ResourceStats>)> {
        // 使用优化的命令解析器
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

    /// 获取输出目录
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
