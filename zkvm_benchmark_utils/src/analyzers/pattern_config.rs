//! Pattern configuration for log parsing

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Pattern configuration loaded from TOML
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatternConfig {
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub execution: HashMap<String, String>,
    #[serde(default)]
    pub trace: HashMap<String, String>,
    #[serde(default)]
    pub proving: HashMap<String, String>,
    #[serde(default)]
    pub verification: HashMap<String, String>,
    #[serde(default)]
    pub resources: HashMap<String, String>,
}

impl PatternConfig {
    /// Load pattern config from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: PatternConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get default SP1 patterns
    pub fn sp1_default() -> Self {
        Self {
            metadata: [
                ("zkvm_name".to_string(), r"zkVM:\s*(.+)".to_string()),
                ("program_name".to_string(), r"Program:\s*(.+)".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),

            execution: [
                (
                    "total_cycles".to_string(),
                    r"(?:total_cycles|Total Cycles):\s*(\d+)".to_string(),
                ),
                (
                    "execution_time_s".to_string(),
                    r"execution_time:\s*([\d.]+)s".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            proving: [
                (
                    "stage1_vm_prove_time_s".to_string(),
                    r"Stage 1.*?:\s*([\d.]+)s".to_string(),
                ),
                (
                    "stage2_recursive_time_s".to_string(),
                    r"Stage 2.*?:\s*([\d.]+)s".to_string(),
                ),
                (
                    "stage3_compress_time_s".to_string(),
                    r"Stage 3.*?:\s*([\d.]+)s".to_string(),
                ),
                (
                    "stage4_wrap_time_s".to_string(),
                    r"Stage 4.*?:\s*([\d.]+)s".to_string(),
                ),
                (
                    "stage5_groth16_time_s".to_string(),
                    r"Stage 5.*?:\s*([\d.]+)s".to_string(),
                ),
                (
                    "total_prove_time_s".to_string(),
                    r"(?:total_prove_time|Total Prove Time):\s*([\d.]+)s".to_string(),
                ),
                (
                    "groth16_proof_size_bytes".to_string(),
                    r"proof_size:\s*(\d+)\s*bytes".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            verification: [(
                "verification_time_ms".to_string(),
                r"verification_time:\s*([\d.]+)ms".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),

            ..Default::default()
        }
    }

    /// Get default RISC0 patterns
    pub fn risc0_default() -> Self {
        Self {
            metadata: [("zkvm_name".to_string(), r"RISC0".to_string())]
                .iter()
                .cloned()
                .collect(),

            execution: [(
                "total_cycles".to_string(),
                r"user_cycles:\s*(\d+)".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),

            proving: [(
                "total_prove_time_s".to_string(),
                r"prove.*?took\s*([\d.]+)s".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),

            ..Default::default()
        }
    }

    /// Get generic/fallback patterns using BENCHMARK: prefix
    pub fn generic_default() -> Self {
        Self {
            metadata: [
                (
                    "program_name".to_string(),
                    r"BENCHMARK: program_name=(.+)".to_string(),
                ),
                (
                    "zkvm_name".to_string(),
                    r"BENCHMARK: zkvm_name=(.+)".to_string(),
                ),
                (
                    "zkvm_version".to_string(),
                    r"BENCHMARK: zkvm_version=(.+)".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            execution: [
                (
                    "total_cycles".to_string(),
                    r"BENCHMARK: total_cycles=(\d+)".to_string(),
                ),
                (
                    "total_instruction_count".to_string(),
                    r"BENCHMARK: total_instruction_count=(\d+)".to_string(),
                ),
                (
                    "execution_time_s".to_string(),
                    r"BENCHMARK: execution_time_s=([\d.]+)".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            trace: [
                (
                    "trace_generation_time_s".to_string(),
                    r"BENCHMARK: trace_generation_time_s=([\d.]+)".to_string(),
                ),
                (
                    "trace_rows".to_string(),
                    r"BENCHMARK: trace_rows=(\d+)".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            proving: [
                (
                    "stage1_vm_prove_time_s".to_string(),
                    r"BENCHMARK: stage1_vm_prove_time_s=([\d.]+)".to_string(),
                ),
                (
                    "stage2_recursive_time_s".to_string(),
                    r"BENCHMARK: stage2_recursive_time_s=([\d.]+)".to_string(),
                ),
                (
                    "stage3_compress_time_s".to_string(),
                    r"BENCHMARK: stage3_compress_time_s=([\d.]+)".to_string(),
                ),
                (
                    "stage4_wrap_time_s".to_string(),
                    r"BENCHMARK: stage4_wrap_time_s=([\d.]+)".to_string(),
                ),
                (
                    "stage5_groth16_time_s".to_string(),
                    r"BENCHMARK: stage5_groth16_time_s=([\d.]+)".to_string(),
                ),
                (
                    "total_prove_time_s".to_string(),
                    r"BENCHMARK: total_prove_time_s=([\d.]+)".to_string(),
                ),
                (
                    "vm_core_proof_size_kb".to_string(),
                    r"BENCHMARK: vm_core_proof_size_kb=([\d.]+)".to_string(),
                ),
                (
                    "compressed_proof_size_kb".to_string(),
                    r"BENCHMARK: compressed_proof_size_kb=([\d.]+)".to_string(),
                ),
                (
                    "groth16_proof_size_bytes".to_string(),
                    r"BENCHMARK: groth16_proof_size_bytes=(\d+)".to_string(),
                ),
                (
                    "total_constraints".to_string(),
                    r"BENCHMARK: total_constraints=(\d+)".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            verification: [
                (
                    "verification_time_s".to_string(),
                    r"BENCHMARK: verification_time_s=([\d.]+)".to_string(),
                ),
                (
                    "verification_time_ms".to_string(),
                    r"BENCHMARK: verification_time_ms=([\d.]+)".to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect(),

            resources: [(
                "peak_memory_mb".to_string(),
                r"BENCHMARK: peak_memory_mb=(\d+)".to_string(),
            )]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_patterns() {
        let config = PatternConfig::generic_default();
        assert!(!config.metadata.is_empty());
        assert!(!config.execution.is_empty());
        assert!(!config.proving.is_empty());
    }

    #[test]
    fn test_sp1_patterns() {
        let config = PatternConfig::sp1_default();
        assert!(config.metadata.contains_key("zkvm_name"));
        assert!(config.execution.contains_key("total_cycles"));
    }

    #[test]
    fn test_risc0_patterns() {
        let config = PatternConfig::risc0_default();
        assert!(config.execution.contains_key("total_cycles"));
    }

    #[test]
    fn test_serde() {
        let config = PatternConfig::generic_default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: PatternConfig = toml::from_str(&serialized).unwrap();
        assert_eq!(config.metadata.len(), deserialized.metadata.len());
    }
}
