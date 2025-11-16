//! Log Analyzer Plugin - Parses zkVM benchmark logs

use super::pattern_config::PatternConfig;
use crate::plugin::*;
use anyhow::{anyhow, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generic metrics structure from log analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GenericMetrics {
    // Metadata
    pub program_name: Option<String>,
    pub zkvm_name: Option<String>,
    pub zkvm_version: Option<String>,

    // Execution Phase
    pub total_cycles: Option<u64>,
    pub total_instruction_count: Option<u64>,
    pub execution_time_s: Option<f64>,

    // Trace Generation
    pub trace_generation_time_s: Option<f64>,
    pub trace_rows: Option<u64>,
    pub trace_columns: Option<u64>,

    // Proving Phase - Stage breakdown
    pub stage1_vm_prove_time_s: Option<f64>,
    pub stage2_recursive_time_s: Option<f64>,
    pub stage3_compress_time_s: Option<f64>,
    pub stage4_wrap_time_s: Option<f64>,
    pub stage5_groth16_time_s: Option<f64>,
    pub total_prove_time_s: Option<f64>,

    // Proof sizes
    pub vm_core_proof_size_kb: Option<f64>,
    pub compressed_proof_size_kb: Option<f64>,
    pub groth16_proof_size_bytes: Option<u64>,
    pub final_proof_size_bytes: Option<u64>,

    // Constraints
    pub total_constraints: Option<u64>,
    pub air_constraints: Option<u64>,

    // Verification
    pub verification_time_s: Option<f64>,
    pub verification_time_ms: Option<f64>,
    pub on_chain_gas_estimate: Option<u64>,

    // Resources
    pub peak_memory_mb: Option<u64>,
    pub avg_cpu_percent: Option<f64>,
    pub disk_io_mb: Option<u64>,

    // Summary
    pub total_time_s: Option<f64>,
    pub throughput_khz: Option<f64>,
    pub success_status: Option<String>,

    // Raw extracted values (for debugging)
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub raw_values: HashMap<String, String>,
}

/// Log analyzer plugin implementation
pub struct LogAnalyzer {
    enabled: bool,
    patterns: PatternConfig,
}

impl LogAnalyzer {
    /// Create a new log analyzer with default patterns
    pub fn new() -> Self {
        Self {
            enabled: true,
            patterns: PatternConfig::generic_default(),
        }
    }

    /// Create a new log analyzer with custom patterns
    pub fn with_patterns(patterns: PatternConfig) -> Self {
        Self {
            enabled: true,
            patterns,
        }
    }

    /// Parse log content into metrics
    pub fn parse_log(&self, content: &str) -> Result<GenericMetrics> {
        let mut metrics = GenericMetrics::default();

        // Parse metadata
        for (key, pattern_str) in &self.patterns.metadata {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("metadata.{}", key), value.clone());
                match key.as_str() {
                    "program_name" => metrics.program_name = Some(value),
                    "zkvm_name" => metrics.zkvm_name = Some(value),
                    "zkvm_version" => metrics.zkvm_version = Some(value),
                    _ => {}
                }
            }
        }

        // Parse execution phase
        for (key, pattern_str) in &self.patterns.execution {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("execution.{}", key), value.clone());
                match key.as_str() {
                    "total_cycles" => metrics.total_cycles = value.parse().ok(),
                    "total_instruction_count" => {
                        metrics.total_instruction_count = value.parse().ok()
                    }
                    "execution_time_s" => metrics.execution_time_s = value.parse().ok(),
                    _ => {}
                }
            }
        }

        // Parse trace generation
        for (key, pattern_str) in &self.patterns.trace {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("trace.{}", key), value.clone());
                match key.as_str() {
                    "trace_generation_time_s" => {
                        metrics.trace_generation_time_s = value.parse().ok()
                    }
                    "trace_rows" => metrics.trace_rows = value.parse().ok(),
                    "trace_columns" => metrics.trace_columns = value.parse().ok(),
                    _ => {}
                }
            }
        }

        // Parse proving phase
        for (key, pattern_str) in &self.patterns.proving {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("proving.{}", key), value.clone());
                match key.as_str() {
                    "stage1_vm_prove_time_s" => metrics.stage1_vm_prove_time_s = value.parse().ok(),
                    "stage2_recursive_time_s" => {
                        metrics.stage2_recursive_time_s = value.parse().ok()
                    }
                    "stage3_compress_time_s" => metrics.stage3_compress_time_s = value.parse().ok(),
                    "stage4_wrap_time_s" => metrics.stage4_wrap_time_s = value.parse().ok(),
                    "stage5_groth16_time_s" => metrics.stage5_groth16_time_s = value.parse().ok(),
                    "total_prove_time_s" => metrics.total_prove_time_s = value.parse().ok(),
                    "vm_core_proof_size_kb" => metrics.vm_core_proof_size_kb = value.parse().ok(),
                    "compressed_proof_size_kb" => {
                        metrics.compressed_proof_size_kb = value.parse().ok()
                    }
                    "groth16_proof_size_bytes" => {
                        metrics.groth16_proof_size_bytes = value.parse().ok()
                    }
                    "final_proof_size_bytes" => metrics.final_proof_size_bytes = value.parse().ok(),
                    "total_constraints" => metrics.total_constraints = value.parse().ok(),
                    "air_constraints" => metrics.air_constraints = value.parse().ok(),
                    _ => {}
                }
            }
        }

        // Parse verification phase
        for (key, pattern_str) in &self.patterns.verification {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("verification.{}", key), value.clone());
                match key.as_str() {
                    "verification_time_s" => metrics.verification_time_s = value.parse().ok(),
                    "verification_time_ms" => metrics.verification_time_ms = value.parse().ok(),
                    "on_chain_gas_estimate" => metrics.on_chain_gas_estimate = value.parse().ok(),
                    _ => {}
                }
            }
        }

        // Parse resources
        for (key, pattern_str) in &self.patterns.resources {
            if let Some(value) = extract_value(content, pattern_str) {
                metrics
                    .raw_values
                    .insert(format!("resources.{}", key), value.clone());
                match key.as_str() {
                    "peak_memory_mb" => metrics.peak_memory_mb = value.parse().ok(),
                    "avg_cpu_percent" => metrics.avg_cpu_percent = value.parse().ok(),
                    "disk_io_mb" => metrics.disk_io_mb = value.parse().ok(),
                    _ => {}
                }
            }
        }

        // Calculate derived metrics
        calculate_derived_metrics(&mut metrics);

        Ok(metrics)
    }
}

impl Default for LogAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for LogAnalyzer {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "log_analyzer".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal log analyzer for zkVM benchmarks".to_string(),
            author: Some("zkVM Benchmark Framework".to_string()),
            capabilities: vec![
                "parse_logs".to_string(),
                "extract_metrics".to_string(),
                "custom_patterns".to_string(),
            ],
        }
    }

    fn initialize(&mut self, config: PluginConfig) -> Result<()> {
        self.enabled = config.enabled;

        // Load custom patterns if specified
        if let Some(pattern_file) = config.options.get("patterns") {
            self.patterns = PatternConfig::from_file(pattern_file)?;
        }

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl AnalyzerPlugin for LogAnalyzer {
    fn analyze(&self, input: &AnalyzerInput) -> Result<AnalyzerOutput> {
        if !self.enabled {
            return Err(anyhow!("LogAnalyzer plugin is disabled"));
        }

        let content = match &input.source {
            DataSource::File(path) => std::fs::read_to_string(path)?,
            DataSource::String(s) => s.clone(),
            DataSource::Bytes(b) => String::from_utf8(b.clone())?,
        };

        let metrics = self.parse_log(&content)?;
        let data = serde_json::to_value(&metrics)?;

        Ok(AnalyzerOutput {
            success: true,
            data,
            format: "json".to_string(),
            metadata: HashMap::new(),
        })
    }

    fn supported_formats(&self) -> Vec<String> {
        vec!["log".to_string(), "txt".to_string()]
    }

    fn can_handle(&self, input: &AnalyzerInput) -> bool {
        if let Some(format) = &input.format {
            self.supported_formats().contains(format)
        } else {
            true // Can try to analyze any text content
        }
    }
}

/// Extract value from content using regex pattern
fn extract_value(content: &str, pattern_str: &str) -> Option<String> {
    let re = Regex::new(pattern_str).ok()?;
    re.captures(content)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

/// Calculate derived metrics from parsed values
fn calculate_derived_metrics(metrics: &mut GenericMetrics) {
    // Calculate total_time_s if not present
    if metrics.total_time_s.is_none() {
        let mut total = 0.0;
        if let Some(t) = metrics.execution_time_s {
            total += t;
        }
        if let Some(t) = metrics.trace_generation_time_s {
            total += t;
        }
        if let Some(t) = metrics.total_prove_time_s {
            total += t;
        }
        if let Some(t) = metrics.verification_time_s {
            total += t;
        }
        if total > 0.0 {
            metrics.total_time_s = Some(total);
        }
    }

    // Calculate throughput (kHz = cycles / time_ms)
    if let (Some(cycles), Some(time_s)) = (metrics.total_cycles, metrics.total_time_s) {
        if time_s > 0.0 {
            metrics.throughput_khz = Some((cycles as f64) / (time_s * 1000.0));
        }
    }

    // Convert verification time if only one is present
    if metrics.verification_time_s.is_none() && metrics.verification_time_ms.is_some() {
        metrics.verification_time_s = metrics.verification_time_ms.map(|ms| ms / 1000.0);
    }
    if metrics.verification_time_ms.is_none() && metrics.verification_time_s.is_some() {
        metrics.verification_time_ms = metrics.verification_time_s.map(|s| s * 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_analyzer_creation() {
        let analyzer = LogAnalyzer::new();
        assert!(analyzer.is_enabled());
        assert_eq!(analyzer.name(), "log_analyzer");
    }

    #[test]
    fn test_parse_simple_log() {
        let analyzer = LogAnalyzer::new();
        let log_content = r#"
BENCHMARK: zkvm_name=sp1
BENCHMARK: program_name=fibonacci
BENCHMARK: total_cycles=1000
BENCHMARK: execution_time_s=1.5
BENCHMARK: total_prove_time_s=10.5
        "#;

        let metrics = analyzer.parse_log(log_content).unwrap();

        assert_eq!(metrics.zkvm_name, Some("sp1".to_string()));
        assert_eq!(metrics.program_name, Some("fibonacci".to_string()));
        assert_eq!(metrics.total_cycles, Some(1000));
        assert_eq!(metrics.execution_time_s, Some(1.5));
        assert_eq!(metrics.total_prove_time_s, Some(10.5));
    }

    #[test]
    fn test_calculate_derived_metrics() {
        let mut metrics = GenericMetrics {
            total_cycles: Some(10000),
            execution_time_s: Some(1.0),
            total_prove_time_s: Some(9.0),
            ..Default::default()
        };

        calculate_derived_metrics(&mut metrics);

        assert_eq!(metrics.total_time_s, Some(10.0));
        assert!(metrics.throughput_khz.is_some());
    }

    #[test]
    fn test_verification_time_conversion() {
        let mut metrics = GenericMetrics {
            verification_time_ms: Some(500.0),
            ..Default::default()
        };

        calculate_derived_metrics(&mut metrics);

        assert_eq!(metrics.verification_time_s, Some(0.5));
    }

    #[test]
    fn test_analyzer_plugin_trait() {
        let analyzer = LogAnalyzer::new();
        let input = AnalyzerInput {
            source: DataSource::String("BENCHMARK: zkvm_name=test".to_string()),
            format: Some("log".to_string()),
            options: HashMap::new(),
        };

        assert!(analyzer.can_handle(&input));
        let result = analyzer.analyze(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_supported_formats() {
        let analyzer = LogAnalyzer::new();
        let formats = analyzer.supported_formats();

        assert!(formats.contains(&"log".to_string()));
        assert!(formats.contains(&"txt".to_string()));
    }

    #[test]
    fn test_extract_value() {
        let content = "BENCHMARK: total_cycles=42";
        let pattern = r"BENCHMARK: total_cycles=(\d+)";

        let value = extract_value(content, pattern);
        assert_eq!(value, Some("42".to_string()));
    }

    #[test]
    fn test_extract_value_not_found() {
        let content = "Some random text";
        let pattern = r"BENCHMARK: total_cycles=(\d+)";

        let value = extract_value(content, pattern);
        assert_eq!(value, None);
    }

    #[test]
    fn test_parse_empty_log() {
        let analyzer = LogAnalyzer::new();
        let metrics = analyzer.parse_log("").unwrap();

        assert!(metrics.zkvm_name.is_none());
        assert!(metrics.total_cycles.is_none());
    }

    #[test]
    fn test_plugin_initialize() {
        let mut analyzer = LogAnalyzer::new();
        let config = PluginConfig {
            enabled: false,
            options: HashMap::new(),
        };

        analyzer.initialize(config).unwrap();
        assert!(!analyzer.is_enabled());
    }

    #[test]
    fn test_raw_values_tracking() {
        let analyzer = LogAnalyzer::new();
        let log_content = "BENCHMARK: zkvm_name=test\nBENCHMARK: total_cycles=100";

        let metrics = analyzer.parse_log(log_content).unwrap();

        assert!(!metrics.raw_values.is_empty());
        assert!(metrics.raw_values.contains_key("metadata.zkvm_name"));
    }
}
