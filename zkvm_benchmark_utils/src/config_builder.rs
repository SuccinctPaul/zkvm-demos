///! Builder 模式用于配置构建
///!
///! 提供类型安全、易用的配置构建接口
use crate::config::{BenchmarkConfig, ZkVmConfig};
use crate::error::Result;
use std::collections::HashMap;
use std::path::Path;

/// BenchmarkConfig 的构建器
#[derive(Debug, Clone)]
pub struct BenchmarkConfigBuilder {
    test_scales: Vec<u32>,
    zkvms: HashMap<String, ZkVmConfig>,
    output_dir: String,
    timeout_seconds: Option<u64>,
    repeat_count: Option<u32>,
}

impl BenchmarkConfigBuilder {
    /// 创建新的构建器，使用合理的默认值
    pub fn new() -> Self {
        Self {
            test_scales: vec![10, 100, 1000],
            zkvms: HashMap::new(),
            output_dir: "benchmark-results".to_string(),
            timeout_seconds: Some(3600), // 1 hour
            repeat_count: Some(1),
        }
    }

    /// 设置测试规模
    pub fn test_scales(mut self, scales: Vec<u32>) -> Self {
        self.test_scales = scales;
        self
    }

    /// 添加单个测试规模
    pub fn add_test_scale(mut self, scale: u32) -> Self {
        self.test_scales.push(scale);
        self
    }

    /// 添加 zkVM 配置
    pub fn add_zkvm(mut self, name: impl Into<String>, config: ZkVmConfig) -> Self {
        self.zkvms.insert(name.into(), config);
        self
    }

    /// 从文件加载 zkVM 配置并添加
    pub fn add_zkvm_from_file(
        mut self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> Result<Self> {
        let config = ZkVmConfig::from_file(path)?;
        self.zkvms.insert(name.into(), config);
        Ok(self)
    }

    /// 从目录加载所有 zkVM 配置
    pub fn add_zkvms_from_dir(mut self, dir: impl AsRef<Path>) -> Result<Self> {
        let dir = dir.as_ref();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    // Skip template files
                    if name == "template" {
                        continue;
                    }

                    match ZkVmConfig::from_file(&path) {
                        Ok(config) if config.enabled => {
                            self.zkvms.insert(name.to_string(), config);
                        }
                        Ok(_) => {
                            // Config exists but not enabled, skip
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load config {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(self)
    }

    /// 设置输出目录
    pub fn output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    /// 设置超时时间（秒）
    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// 移除超时限制
    pub fn no_timeout(mut self) -> Self {
        self.timeout_seconds = None;
        self
    }

    /// 设置重复执行次数
    pub fn repeat_count(mut self, count: u32) -> Self {
        self.repeat_count = Some(count);
        self
    }

    /// 构建配置并验证
    pub fn build(self) -> Result<BenchmarkConfig> {
        let config = BenchmarkConfig {
            test_scales: self.test_scales,
            zkvms: self.zkvms,
            output_dir: self.output_dir,
            timeout_seconds: self.timeout_seconds,
            repeat_count: self.repeat_count,
        };

        // 验证配置
        config.validate()?;

        Ok(config)
    }

    /// 构建配置但不验证（用于测试）
    pub fn build_unchecked(self) -> BenchmarkConfig {
        BenchmarkConfig {
            test_scales: self.test_scales,
            zkvms: self.zkvms,
            output_dir: self.output_dir,
            timeout_seconds: self.timeout_seconds,
            repeat_count: self.repeat_count,
        }
    }
}

impl Default for BenchmarkConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ZkVmConfig 的构建器
#[derive(Debug, Clone)]
pub struct ZkVmConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    enabled: bool,
    default_mode: String,
    test_modes: Vec<String>,
    test_scales: Option<Vec<u32>>,
    working_dir: String,
    build_command: Option<String>,
    run_command: String,
    timeout_seconds: Option<u64>,
    repeat_count: Option<u32>,
    env_vars: Option<HashMap<String, String>>,
    log_patterns: crate::config::LogPatterns,
    stage_merge: Option<crate::config::StageMerge>,
    proof_size_config: Option<crate::config::ProofSizeConfig>,
}

impl ZkVmConfigBuilder {
    /// 创建新的构建器
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            version: None,
            enabled: true,
            default_mode: "groth16".to_string(),
            test_modes: vec![
                "core".to_string(),
                "compressed".to_string(),
                "groth16".to_string(),
            ],
            test_scales: None,
            working_dir: String::new(),
            build_command: None,
            run_command: String::new(),
            timeout_seconds: None,
            repeat_count: None,
            env_vars: None,
            log_patterns: crate::config::LogPatterns::default(),
            stage_merge: None,
            proof_size_config: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn default_mode(mut self, mode: impl Into<String>) -> Self {
        self.default_mode = mode.into();
        self
    }

    pub fn test_modes(mut self, modes: Vec<String>) -> Self {
        self.test_modes = modes;
        self
    }

    pub fn add_test_mode(mut self, mode: impl Into<String>) -> Self {
        self.test_modes.push(mode.into());
        self
    }

    pub fn test_scales(mut self, scales: Vec<u32>) -> Self {
        self.test_scales = Some(scales);
        self
    }

    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = dir.into();
        self
    }

    pub fn build_command(mut self, cmd: impl Into<String>) -> Self {
        self.build_command = Some(cmd.into());
        self
    }

    pub fn run_command(mut self, cmd: impl Into<String>) -> Self {
        self.run_command = cmd.into();
        self
    }

    pub fn timeout_seconds(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    pub fn repeat_count(mut self, count: u32) -> Self {
        self.repeat_count = Some(count);
        self
    }

    pub fn add_env_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn env_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.env_vars = Some(vars);
        self
    }

    pub fn log_patterns(mut self, patterns: crate::config::LogPatterns) -> Self {
        self.log_patterns = patterns;
        self
    }

    /// 构建并验证
    pub fn build(self) -> Result<ZkVmConfig> {
        let config = ZkVmConfig {
            name: self.name,
            version: self.version,
            enabled: self.enabled,
            default_mode: self.default_mode,
            test_modes: self.test_modes,
            test_scales: self.test_scales,
            working_dir: self.working_dir,
            build_command: self.build_command,
            run_command: self.run_command,
            timeout_seconds: self.timeout_seconds,
            repeat_count: self.repeat_count,
            env_vars: self.env_vars,
            log_patterns: self.log_patterns,
            stage_merge: self.stage_merge,
            proof_size_config: self.proof_size_config,
        };

        config.validate()?;
        Ok(config)
    }
}

// 为 BenchmarkConfig 添加扩展方法
impl BenchmarkConfig {
    /// 创建构建器
    pub fn builder() -> BenchmarkConfigBuilder {
        BenchmarkConfigBuilder::new()
    }
}

// 为 ZkVmConfig 添加扩展方法
impl ZkVmConfig {
    /// 创建构建器
    pub fn builder(name: impl Into<String>) -> ZkVmConfigBuilder {
        ZkVmConfigBuilder::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config_builder() {
        let config = BenchmarkConfigBuilder::new()
            .test_scales(vec![10, 100])
            .output_dir("test-results")
            .timeout_seconds(1800)
            .repeat_count(3)
            .build_unchecked();

        assert_eq!(config.test_scales, vec![10, 100]);
        assert_eq!(config.output_dir, "test-results");
        assert_eq!(config.timeout_seconds, Some(1800));
        assert_eq!(config.repeat_count, Some(3));
    }

    #[test]
    fn test_zkvm_config_builder() {
        let config = ZkVmConfigBuilder::new("test_zkvm")
            .version("v1.0.0")
            .working_dir("./test")
            .run_command("cargo run")
            .add_test_mode("test_mode")
            .add_env_var("RUST_LOG", "debug")
            .build();

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.name, Some("test_zkvm".to_string()));
        assert_eq!(config.version, Some("v1.0.0".to_string()));
    }

    #[test]
    fn test_builder_fluent_api() {
        let result = BenchmarkConfigBuilder::new()
            .test_scales(vec![10])
            .add_test_scale(100)
            .add_test_scale(1000)
            .output_dir("my-results")
            .timeout_seconds(3600)
            .repeat_count(5)
            .build_unchecked();

        assert_eq!(result.test_scales, vec![10, 100, 1000]);
        assert_eq!(result.repeat_count, Some(5));
    }
}
