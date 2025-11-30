//! Configuration management for the benchmark framework

use crate::core::error::{BenchmarkError, Result};
use crate::core::metrics::{ProofMode, ZkVmName};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Helper struct to manage output paths
pub struct BenchmarkPaths {
    pub root: PathBuf,
    pub raw_logs: PathBuf,
    pub parsed_metrics: PathBuf,
    pub reports: PathBuf,
}

impl Default for BenchmarkPaths {
    fn default() -> Self {
        Self::new("benchmark-results")
    }
}

impl BenchmarkPaths {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let root = root.as_ref().to_path_buf();
        Self {
            raw_logs: root.join("raw-logs"),
            parsed_metrics: root.join("parsed-metrics"),
            reports: root.join("reports"),
            root,
        }
    }
}

/// Main benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    // TODO: here should be input of the program.
    pub test_scales: Vec<u32>,
    pub zkvms: HashMap<String, ZkVmConfig>,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    pub timeout_seconds: Option<u64>,
    // TOOD: remove
    pub repeat_count: Option<u32>,
}

fn default_output_dir() -> String {
    "benchmark-results".to_string()
}

/// Configuration for a single zkVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkVmConfig {
    pub name: Option<ZkVmName>,
    pub version: Option<String>,
    pub enabled: bool,
    pub prove_modes: Vec<ProofMode>,
    /// Programs to test. If None or empty, defaults to [Fibonacci]
    pub programs: Option<Vec<ProgramConfig>>,
    /// Deprecated: Use programs[].scales instead. Kept for backward compatibility.
    pub test_scales: Option<Vec<u32>>,
    pub working_dir: String,
    pub build_command: Option<String>,
    pub run_command: String,
    pub timeout_seconds: Option<u64>,
    pub repeat_count: Option<u32>,
    pub env_vars: Option<HashMap<String, String>>,
    #[serde(rename = "parsed_metrics", default)]
    pub parsed_metrics: ParsedMetrics,
    #[serde(rename = "metric_mapping", default)]
    pub metric_mapping: HashMap<String, String>,
    pub stage_merge: Option<StageMerge>,
    pub proof_size_config: Option<ProofSizeConfig>,
}

/// Configuration for a single program
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramConfig {
    /// Program name (e.g., "fibonacci", "hash", "sum")
    pub name: String,
    /// Test scales/parameters for this program
    pub scales: Vec<u32>,
    /// Program-specific environment variables
    #[serde(default)]
    pub env_vars: Option<HashMap<String, String>>,
    /// Program-specific timeout (overrides zkVM timeout)
    pub timeout_seconds: Option<u64>,
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

        if self.prove_modes.is_empty() {
            return Err(BenchmarkError::Config(
                "prove_modes cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Get test scales (use config value or default)
    /// Deprecated: Use get_programs() instead
    pub fn get_test_scales(&self, default_scales: Option<&[u32]>) -> Vec<u32> {
        if let Some(scales) = &self.test_scales {
            scales.clone()
        } else if let Some(default) = default_scales {
            default.to_vec()
        } else {
            vec![10] // fallback default
        }
    }

    /// Get program configurations, with fallback to default Fibonacci program
    pub fn get_programs(&self) -> Vec<ProgramConfig> {
        if let Some(ref programs) = self.programs {
            if programs.is_empty() {
                // Default to Fibonacci if programs list is empty
                vec![ProgramConfig {
                    name: "fibonacci".to_string(),
                    scales: self.get_test_scales(None),
                    env_vars: None,
                    timeout_seconds: None,
                }]
            } else {
                programs.clone()
            }
        } else if let Some(scales) = &self.test_scales {
            // Backward compatibility: use test_scales for Fibonacci
            vec![ProgramConfig {
                name: "fibonacci".to_string(),
                scales: scales.clone(),
                env_vars: None,
                timeout_seconds: None,
            }]
        } else {
            // Default fallback
            vec![ProgramConfig {
                name: "fibonacci".to_string(),
                scales: vec![10],
                env_vars: None,
                timeout_seconds: None,
            }]
        }
    }
}

/// Parsed metrics configuration (regex patterns)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsedMetrics {
    #[serde(flatten)]
    pub patterns: HashMap<String, String>,
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

                if config.prove_modes.is_empty() {
                    return Err(BenchmarkError::Config(format!(
                        "zkVM '{}': prove_modes cannot be empty",
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

    /// Get default paths
    pub fn default_paths() -> BenchmarkPaths {
        BenchmarkPaths::default()
    }

    /// Get paths based on output_dir
    pub fn get_paths(&self) -> BenchmarkPaths {
        BenchmarkPaths::new(&self.output_dir)
    }

    /// Get default configuration
    pub fn default() -> Self {
        Self {
            test_scales: vec![10, 20],
            zkvms: HashMap::new(),
            output_dir: "benchmark-results".to_string(),
            timeout_seconds: Some(3600), // 1 hour default
            repeat_count: Some(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a minimal example configuration
    pub fn example() -> BenchmarkConfig {
        let mut zkvms = HashMap::new();

        // SP1 example configuration
        zkvms.insert(
            "sp1".to_string(),
            ZkVmConfig {
                name: Some(ZkVmName::Sp1),
                version: Some("v4.0.0".to_string()),
                enabled: true,
                prove_modes: vec![ProofMode::Core, ProofMode::Compressed, ProofMode::Groth16],
                programs: Some(vec![ProgramConfig {
                    name: "fibonacci".to_string(),
                    scales: vec![10, 20],
                    env_vars: None,
                    timeout_seconds: None,
                }]),
                test_scales: Some(vec![10, 20]),
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

        BenchmarkConfig {
            test_scales: vec![10, 20],
            zkvms,
            output_dir: "benchmark-results".to_string(),
            timeout_seconds: Some(3600),
            repeat_count: Some(1),
        }
    }

    #[test]
    fn test_example_config() {
        let config = example();
        assert!(!config.test_scales.is_empty());
        assert!(!config.zkvms.is_empty());

        let sp1_config = config.zkvms.get("sp1").unwrap();
        assert!(sp1_config.enabled);
        assert_eq!(sp1_config.prove_modes.len(), 3);
    }

    #[test]
    fn test_config_validation() {
        let config = example();
        assert!(config.validate().is_ok());

        let mut bad_config = config.clone();
        bad_config.test_scales.clear();
        assert!(bad_config.validate().is_err());
    }
}
