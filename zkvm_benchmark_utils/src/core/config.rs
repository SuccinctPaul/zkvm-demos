//! Configuration management for the benchmark framework

use crate::core::error::{BenchmarkError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Main benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    // TODO: here should be input of the program. Tmp only use Fibonacci, so test scales are Fibonacci numbers.
    pub test_scales: Vec<u32>,
    pub zkvms: HashMap<String, ZkVmConfig>,
    pub output_dir: String,
    pub timeout_seconds: Option<u64>,
    pub repeat_count: Option<u32>,
}

/// Configuration for a single zkVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVmConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub enabled: bool,
    pub default_mode: String,
    pub test_modes: Vec<String>,
    pub test_scales: Option<Vec<u32>>,
    pub working_dir: String,
    pub build_command: Option<String>,
    pub run_command: String,
    pub timeout_seconds: Option<u64>,
    pub repeat_count: Option<u32>,
    pub env_vars: Option<HashMap<String, String>>,
    #[serde(rename = "parsed_metrics")]
    pub parsed_metrics: ParsedMetrics,
    #[serde(rename = "metric_mapping", default)]
    pub metric_mapping: HashMap<String, String>,
    #[serde(rename = "reporting")]
    pub reporting: ReportingConfig,
    pub stage_merge: Option<StageMerge>,
    pub proof_size_config: Option<ProofSizeConfig>,
}

impl ZkVmConfig {
    /// Load zkVM configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path).map_err(|e| {
            BenchmarkError::Config(format!("Failed to read config {:?}: {}", path.as_ref(), e))
        })?;

        let config: ZkVmConfig = toml::from_str(&content)?;
        config.validate()?;

        Ok(config)
    }

    /// Load zkVM configuration by name from configs/ directory
    pub fn from_name(zkvm_name: &str) -> Result<Self> {
        let config_path = format!("configs/{}.toml", zkvm_name);
        Self::from_file(&config_path)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.working_dir.is_empty() {
            return Err(BenchmarkError::Config(
                "working_dir cannot be empty".to_string(),
            ));
        }

        if self.test_modes.is_empty() {
            return Err(BenchmarkError::Config(
                "test_modes cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Get test scales (use config value or default)
    pub fn get_test_scales(&self, default_scales: Option<&[u32]>) -> Vec<u32> {
        if let Some(scales) = &self.test_scales {
            scales.clone()
        } else if let Some(default) = default_scales {
            default.to_vec()
        } else {
            vec![10] // fallback default
        }
    }
}

/// Parsed metrics configuration (regex patterns)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsedMetrics {
    #[serde(flatten)]
    pub patterns: HashMap<String, String>,
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingConfig {
    pub metrics: Vec<String>,
    pub output_formats: Option<Vec<String>>, // csv, json, etc.
}

/// Stage merge configuration for multi-mode runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageMerge {
    pub stage_1_time: String, // e.g., "core.total_time"
    pub stage_2_time: String, // e.g., "compressed.total_time - core.total_time"
    pub stage_34_time: Option<String>,
}

/// Proof size measurement configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSizeConfig {
    pub vm_core_proof_size_kb: String,
    pub compressed_proof_size_kb: String,
    pub groth16_proof_size_bytes: String,
}

impl BenchmarkConfig {
    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| BenchmarkError::Config(format!("Failed to read config: {}", e)))?;

        let config: BenchmarkConfig = toml::from_str(&content)?;
        config.validate()?;

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.test_scales.is_empty() {
            return Err(BenchmarkError::Config(
                "test_scales cannot be empty".to_string(),
            ));
        }

        if self.zkvms.is_empty() {
            return Err(BenchmarkError::Config("zkvms cannot be empty".to_string()));
        }

        // Validate each zkVM config
        for (name, config) in &self.zkvms {
            if config.enabled {
                if config.run_command.is_empty() {
                    return Err(BenchmarkError::Config(format!(
                        "zkVM '{}': run_command is required",
                        name
                    )));
                }

                if config.test_modes.is_empty() {
                    return Err(BenchmarkError::Config(format!(
                        "zkVM '{}': test_modes cannot be empty",
                        name
                    )));
                }
            }
        }

        Ok(())
    }

    /// Get list of enabled zkVMs
    pub fn enabled_zkvms(&self) -> Vec<(&String, &ZkVmConfig)> {
        self.zkvms
            .iter()
            .filter(|(_, config)| config.enabled)
            .collect()
    }

    /// Get default configuration
    pub fn default() -> Self {
        Self {
            test_scales: vec![10, 100, 1000, 10000],
            zkvms: HashMap::new(),
            output_dir: "benchmark-results".to_string(),
            timeout_seconds: Some(3600), // 1 hour default
            repeat_count: Some(1),
        }
    }

    /// Create a minimal example configuration
    pub fn example() -> Self {
        let mut zkvms = HashMap::new();

        // SP1 example configuration
        zkvms.insert(
            "sp1".to_string(),
            ZkVmConfig {
                name: Some("sp1".to_string()),
                version: Some("v4.0.0".to_string()),
                enabled: true,
                default_mode: "groth16".to_string(),
                test_modes: vec![
                    "core".to_string(),
                    "compressed".to_string(),
                    "groth16".to_string(),
                ],
                test_scales: Some(vec![10, 100, 1000]),
                working_dir: "sp1-zkvm/sp1-host".to_string(),
                build_command: Some("cargo build --release".to_string()),
                run_command: "cargo run --release -- --prove".to_string(),
                timeout_seconds: Some(3600),
                repeat_count: Some(1),
                env_vars: Some({
                    let mut env = HashMap::new();
                    env.insert("RUST_LOG".to_string(), "debug".to_string());
                    env
                }),
                parsed_metrics: ParsedMetrics {
                    patterns: {
                        let mut p = HashMap::new();
                        p.insert(
                            "total_cycles".to_string(),
                            r"BENCHMARK: total_cycles=(\d+)".to_string(),
                        );
                        p.insert(
                            "total_instruction_count".to_string(),
                            r"Number of instructions: (\d+)".to_string(),
                        );
                        // ... (truncated for brevity in example, real usage keeps all)
                        p
                    },
                },
                metric_mapping: HashMap::new(), // Default empty mapping
                reporting: ReportingConfig {
                    metrics: vec![
                        "total_cycles".to_string(),
                        "total_prove_time_s".to_string(),
                        "vm_core_proof_size_kb".to_string(),
                        "compressed_proof_size_kb".to_string(),
                        "groth16_proof_size_bytes".to_string(),
                    ],
                    output_formats: Some(vec![
                        "csv".to_string(),
                        "json".to_string(),
                        "console".to_string(),
                    ]),
                },
                stage_merge: Some(StageMerge {
                    stage_1_time: "core.total_time".to_string(),
                    stage_2_time: "compressed.total_time - core.total_time".to_string(),
                    stage_34_time: Some("groth16.total_time - compressed.total_time".to_string()),
                }),
                proof_size_config: Some(ProofSizeConfig {
                    vm_core_proof_size_kb: "core.proof_size".to_string(),
                    compressed_proof_size_kb: "compressed.proof_size".to_string(),
                    groth16_proof_size_bytes: "groth16.proof_size".to_string(),
                }),
            },
        );

        Self {
            test_scales: vec![10, 100, 1000],
            zkvms,
            output_dir: "benchmark-results".to_string(),
            timeout_seconds: Some(3600),
            repeat_count: Some(1),
        }
    }

    /// Save configuration to TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| BenchmarkError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(path, content)
            .map_err(|e| BenchmarkError::Config(format!("Failed to write config: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_config() {
        let config = BenchmarkConfig::example();
        assert!(!config.test_scales.is_empty());
        assert!(!config.zkvms.is_empty());

        let sp1_config = config.zkvms.get("sp1").unwrap();
        assert!(sp1_config.enabled);
        assert_eq!(sp1_config.test_modes.len(), 3);
    }

    #[test]
    fn test_config_validation() {
        let config = BenchmarkConfig::example();
        assert!(config.validate().is_ok());

        let mut bad_config = config.clone();
        bad_config.test_scales.clear();
        assert!(bad_config.validate().is_err());
    }
}
