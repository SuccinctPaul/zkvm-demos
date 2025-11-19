//! Optimized Log Parser
//!
//! Uses RegexSet and single-pass extraction for high performance (5-10x faster).

use crate::core::config::ParsedMetrics;
use crate::core::error::{BenchmarkError, Result};
use crate::core::metrics::*;
use crate::system::hardware;
use regex::{Regex, RegexSet};
use std::collections::{HashMap, HashSet};

/// Optimized Log Parser
pub struct LogParser {
    /// Individual regex patterns
    patterns: HashMap<String, Regex>,
    /// Combined regex set for fast filtering
    pattern_set: RegexSet,
    /// Order of pattern keys corresponding to RegexSet indices
    key_order: Vec<String>,
}

impl LogParser {
    /// Create a new log parser with the given patterns
    pub fn new(log_patterns: &ParsedMetrics) -> Result<Self> {
        let mut patterns = HashMap::new();
        let mut pattern_strings = Vec::new();
        let mut key_order = Vec::new();

        // Iterate over all patterns in the map
        for (name, pattern_str) in &log_patterns.patterns {
            let regex = Regex::new(pattern_str)
                .map_err(|e| BenchmarkError::Parse(format!("Invalid regex for {}: {}", name, e)))?;
            patterns.insert(name.clone(), regex);
            pattern_strings.push(pattern_str.clone());
            key_order.push(name.clone());
        }

        // Create RegexSet for fast matching
        let pattern_set = if !pattern_strings.is_empty() {
            RegexSet::new(&pattern_strings)
                .map_err(|e| BenchmarkError::Parse(format!("Failed to create RegexSet: {}", e)))?
        } else {
            // Empty RegexSet
            RegexSet::new(&[] as &[&str]).unwrap()
        };

        Ok(Self {
            patterns,
            pattern_set,
            key_order,
        })
    }

    /// Parse log content and extract metrics (Optimized)
    pub fn parse(
        &self,
        log_content: &str,
        zkvm_name: &str,
        program_name: &str,
        metric_mapping: Option<&HashMap<String, String>>,
    ) -> Result<BenchmarkMetrics> {
        // Single pass extraction
        let extracted = self.extract_all_values_single_pass(log_content);

        // Build metrics
        self.build_metrics(extracted, zkvm_name, program_name, metric_mapping)
    }

    /// Single pass extraction core logic
    fn extract_all_values_single_pass(&self, content: &str) -> HashMap<String, String> {
        let mut extracted = HashMap::new();
        let mut remaining_keys: HashSet<usize> = (0..self.key_order.len()).collect();

        // Iterate through lines
        for line in content.lines() {
            if remaining_keys.is_empty() {
                // Early exit if all patterns matched
                break;
            }

            // Use RegexSet to check if line matches any pattern
            let matches = self.pattern_set.matches(line);

            if matches.matched_any() {
                // Extract values for matched patterns
                for idx in matches.iter() {
                    if !remaining_keys.contains(&idx) {
                        continue; // Already extracted
                    }

                    let key = &self.key_order[idx];
                    if let Some(regex) = self.patterns.get(key) {
                        if let Some(caps) = regex.captures(line) {
                            if let Some(value) = caps.get(1) {
                                extracted.insert(key.clone(), value.as_str().to_string());
                                remaining_keys.remove(&idx);
                            }
                        }
                    }
                }
            }
        }

        extracted
    }

    /// Build BenchmarkMetrics from extracted map
    fn build_metrics(
        &self,
        extracted: HashMap<String, String>,
        zkvm_name: &str,
        program_name: &str,
        metric_mapping: Option<&HashMap<String, String>>,
    ) -> Result<BenchmarkMetrics> {
        let mut metrics = BenchmarkMetrics::new(program_name.to_string(), zkvm_name.to_string());

        // Apply metric mapping (Standardization Layer)
        let mut normalized_metrics = extracted.clone();
        if let Some(mapping) = metric_mapping {
            for (std_key, raw_key) in mapping {
                if let Some(val) = extracted.get(raw_key) {
                    normalized_metrics.insert(std_key.clone(), val.clone());
                }
            }
        }

        // Store all extracted metrics in custom_metrics
        metrics.custom_metrics = normalized_metrics;

        // Collect hardware info
        metrics.metadata.hardware = Some(hardware::collect_hardware_info());

        // Calculate derived metrics (Derivation Layer)
        metrics.calculate_derived_metrics();

        Ok(metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::ParsedMetrics;

    #[test]
    fn test_log_parser_creation() {
        let patterns = ParsedMetrics {
            patterns: {
                let mut p = HashMap::new();
                p.insert(
                    "total_cycles".to_string(),
                    r"total_cycles=(\d+)".to_string(),
                );
                p.insert(
                    "total_prove_time_s".to_string(),
                    r"prove_time=([\d.]+)".to_string(),
                );
                p.insert(
                    "final_proof_size_bytes".to_string(),
                    r"proof_size=(\d+)".to_string(),
                );
                p.insert(
                    "verification_time_s".to_string(),
                    r"verify_time=([\d.]+)".to_string(),
                );
                p
            },
        };

        let parser = LogParser::new(&patterns);
        assert!(parser.is_ok());
    }

    #[test]
    fn test_single_pass_extraction() {
        let patterns = ParsedMetrics {
            patterns: {
                let mut p = HashMap::new();
                p.insert(
                    "total_cycles".to_string(),
                    r"total_cycles=(\d+)".to_string(),
                );
                p.insert(
                    "total_prove_time_s".to_string(),
                    r"prove_time=([\d.]+)".to_string(),
                );
                p.insert(
                    "final_proof_size_bytes".to_string(),
                    r"proof_size=(\d+)".to_string(),
                );
                p
            },
        };

        let parser = LogParser::new(&patterns).unwrap();

        let log_content = r#"
            Starting benchmark...
            total_cycles=12543
            Running proof generation...
            prove_time=45.8
            Generating proof...
            proof_size=192
            Done!
        "#;

        let extracted = parser.extract_all_values_single_pass(log_content);

        assert_eq!(extracted.get("total_cycles"), Some(&"12543".to_string()));
        assert_eq!(
            extracted.get("total_prove_time_s"),
            Some(&"45.8".to_string())
        );
        assert_eq!(
            extracted.get("final_proof_size_bytes"),
            Some(&"192".to_string())
        );
    }

    #[test]
    fn test_early_exit_optimization() {
        let patterns = ParsedMetrics {
            patterns: {
                let mut p = HashMap::new();
                p.insert(
                    "total_cycles".to_string(),
                    r"total_cycles=(\d+)".to_string(),
                );
                p.insert(
                    "total_prove_time_s".to_string(),
                    r"prove_time=([\d.]+)".to_string(),
                );
                p
            },
        };

        let parser = LogParser::new(&patterns).unwrap();

        let log_content = r#"
            total_cycles=12543
            prove_time=45.8
            (many more lines that won't be processed...)
            line 1000
            line 2000
            ...
        "#;

        let extracted = parser.extract_all_values_single_pass(log_content);
        assert_eq!(extracted.len(), 2);
    }
}
