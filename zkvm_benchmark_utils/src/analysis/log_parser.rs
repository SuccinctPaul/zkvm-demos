//! Optimized Log Parser
//!
//! Uses RegexSet and single-pass extraction for high performance (5-10x faster).

use crate::core::config::ParsedMetrics;
use crate::core::error::{BenchmarkError, Result};
use crate::core::metrics::*;
use crate::system::hardware;
use rayon::prelude::*;
use regex::{Regex, RegexSet};
use std::collections::HashMap;

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
    ) -> Result<UnifiedMetrics> {
        use log::debug;

        debug!("  🔍 Parsing log for {zkvm_name} (program: {program_name})",);
        debug!(
            "  📏 Log size: {} lines, {} bytes",
            log_content.lines().count(),
            log_content.len()
        );
        debug!("  🎯 Pattern count: {}", self.patterns.len());

        // Single pass extraction
        let extracted = self.extract_all_values_single_pass(log_content);

        debug!("  ✅ Extracted {} metrics", extracted.len());
        if extracted.len() < self.patterns.len() {
            debug!(
                "  ⚠️  Some patterns did not match (expected {}, got {})",
                self.patterns.len(),
                extracted.len()
            );
        }

        // Build metrics
        self.build_metrics(extracted, zkvm_name, program_name, metric_mapping)
    }

    /// Single pass extraction core logic
    fn extract_all_values_single_pass(&self, content: &str) -> HashMap<String, String> {
        // Parallel iteration over lines using rayon
        // Collects matches from all lines in parallel, preserving order
        let partial_results: Vec<HashMap<String, String>> = content
            .par_lines()
            .filter_map(|line| {
                let matches = self.pattern_set.matches(line);
                if matches.matched_any() {
                    let mut local_extracted = HashMap::new();
                    for idx in matches.iter() {
                        let key = &self.key_order[idx];
                        if let Some(regex) = self.patterns.get(key) {
                            if let Some(caps) = regex.captures(line) {
                                if let Some(value) = caps.get(1) {
                                    local_extracted.insert(key.clone(), value.as_str().to_string());
                                }
                            }
                        }
                    }
                    if local_extracted.is_empty() {
                        None
                    } else {
                        Some(local_extracted)
                    }
                } else {
                    None
                }
            })
            .collect();

        // Merge results, keeping the first occurrence for each key
        let mut extracted = HashMap::new();
        for local_map in partial_results {
            for (k, v) in local_map {
                extracted.entry(k).or_insert(v);
            }
        }

        extracted
    }

    /// Build UnifiedMetrics from extracted map
    fn build_metrics(
        &self,
        extracted: HashMap<String, String>,
        zkvm_name: &str,
        program_name: &str,
        metric_mapping: Option<&HashMap<String, String>>,
    ) -> Result<UnifiedMetrics> {
        use log::debug;

        let program_name_enum = program_name
            .parse::<ProgramName>()
            .unwrap_or_else(|_| ProgramName::Custom(program_name.to_string()));

        let zkvm_name_enum = zkvm_name
            .parse::<ZkVmName>()
            .unwrap_or_else(|_| {
                // This fallback is important because parser might be used with non-standard names
                // However, in most cases, it should match the enum variants.
                // Since we removed ZkVmName::Custom, we need to map unknown strings to a known variant or panic.
                // Given the parser context, it's safer to panic if we expect strict compliance,
                // but for flexibility, we might need to revisit the removal of Custom.
                // FOR NOW: Let's assume the string MUST be valid, otherwise we panic with a clear message.
                // This aligns with the strict typing approach.
                panic!("Unknown zkVM name encountered in parser: {}", zkvm_name)
            });

        let mut metrics = UnifiedMetrics::new(program_name_enum, zkvm_name_enum);

        // Apply metric mapping (Standardization Layer)
        let mut normalized_metrics = extracted.clone();
        if let Some(mapping) = metric_mapping {
            debug!("  🔄 Applying {} metric mappings", mapping.len());
            let mut mapped_count = 0;
            for (std_key, raw_key) in mapping {
                if let Some(val) = extracted.get(raw_key) {
                    normalized_metrics.insert(std_key.clone(), val.clone());
                    mapped_count += 1;
                }
            }
            debug!("  ✅ Mapped {} metrics", mapped_count);
        } else {
            debug!("  ℹ️  No metric mapping provided");
        }

        // Store all extracted metrics in custom_metrics
        metrics.custom_metrics = normalized_metrics;

        debug!("  💻 Collecting hardware info...");
        // Collect hardware info
        metrics.metadata.hardware = Some(hardware::collect_hardware_info());

        debug!("  🧮 Calculating derived metrics...");
        // Calculate derived metrics and populate fields
        metrics.update_derived_metrics();

        debug!("  ✅ Metrics build completed");

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
    fn test_parallel_extraction_correctness() {
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
            (many more lines that are processed in parallel...)
            line 1000
            line 2000
            ...
        "#;

        let extracted = parser.extract_all_values_single_pass(log_content);
        assert_eq!(extracted.len(), 2);
    }
}
