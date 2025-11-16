//! Log parser for extracting metrics from zkVM output

use crate::config::LogPatterns;
use crate::error::{BenchmarkError, Result};
use crate::hardware;
use crate::metrics::*;
use regex::Regex;
use std::collections::HashMap;

pub struct LogParser {
    patterns: HashMap<String, Regex>,
}

impl LogParser {
    /// Create a new log parser with the given patterns
    pub fn new(log_patterns: &LogPatterns) -> Result<Self> {
        let mut patterns = HashMap::new();

        // Compile all regex patterns
        macro_rules! add_pattern {
            ($field:expr, $name:expr) => {
                if let Some(pattern_str) = $field {
                    let regex = Regex::new(pattern_str).map_err(|e| {
                        BenchmarkError::Parse(format!("Invalid regex for {}: {}", $name, e))
                    })?;
                    patterns.insert($name.to_string(), regex);
                }
            };
        }

        add_pattern!(&log_patterns.total_cycles, "total_cycles");
        add_pattern!(
            &log_patterns.total_instruction_count,
            "total_instruction_count"
        );
        add_pattern!(&log_patterns.total_prove_time_s, "total_prove_time_s");
        add_pattern!(
            &log_patterns.final_proof_size_bytes,
            "final_proof_size_bytes"
        );
        add_pattern!(&log_patterns.verification_time_s, "verification_time_s");
        add_pattern!(&log_patterns.success_status, "success_status");
        add_pattern!(&log_patterns.execution_time_s, "execution_time_s");
        add_pattern!(&log_patterns.vm_prove_time_s, "vm_prove_time_s");
        add_pattern!(
            &log_patterns.recursive_prove_time_s,
            "recursive_prove_time_s"
        );
        add_pattern!(&log_patterns.snark_prove_time_s, "snark_prove_time_s");
        add_pattern!(&log_patterns.vm_core_proof_size_kb, "vm_core_proof_size_kb");
        add_pattern!(
            &log_patterns.compressed_proof_size_kb,
            "compressed_proof_size_kb"
        );
        add_pattern!(
            &log_patterns.groth16_proof_size_bytes,
            "groth16_proof_size_bytes"
        );
        add_pattern!(&log_patterns.syscall_cycles, "syscall_cycles");
        add_pattern!(&log_patterns.syscall_count, "syscall_count");
        add_pattern!(&log_patterns.segments, "segments");
        add_pattern!(&log_patterns.peak_memory_mb, "peak_memory_mb");
        add_pattern!(&log_patterns.security_bits, "security_bits");

        Ok(Self { patterns })
    }

    /// Parse log content and extract metrics
    pub fn parse(
        &self,
        log_content: &str,
        zkvm_name: &str,
        program_name: &str,
    ) -> Result<BenchmarkMetrics> {
        let mut metrics = BenchmarkMetrics::new(program_name.to_string(), zkvm_name.to_string());

        // Collect hardware information
        metrics.metadata.hardware = Some(hardware::collect_hardware_info());

        // Extract zkvm_version if available
        if let Some(version) =
            self.extract_string_direct(log_content, "BENCHMARK: zkvm_version=([\\w.-]+)")
        {
            metrics.metadata.zkvm_version = Some(version);
        }

        // Parse execution phase
        let execution_phase = ExecutionPhase {
            total_cycles: self.extract_u64(log_content, "total_cycles"),
            total_instruction_count: self.extract_u64(log_content, "total_instruction_count"),
            user_cycles: None,
            syscall_cycles: self.extract_u64(log_content, "syscall_cycles"),
            syscall_count: self.extract_u64(log_content, "syscall_count"),
            memory_accesses: None,
            touched_memory_addresses: None,
            execution_time_s: self.extract_f64(log_content, "execution_time_s"),
            execution_throughput: None, // Calculated later
            segments: self.extract_u64(log_content, "segments"),
            segment_size: None,
            max_segment_cycles: None,
        };

        metrics.execution_phase = Some(execution_phase);

        // Parse proving phase
        let total_prove_time = self
            .extract_f64(log_content, "total_prove_time_s")
            .ok_or_else(|| BenchmarkError::MissingMetric("total_prove_time_s".to_string()))?;

        let final_proof_size = self
            .extract_u64(log_content, "final_proof_size_bytes")
            .ok_or_else(|| BenchmarkError::MissingMetric("final_proof_size_bytes".to_string()))?;

        let stage_1_vm_prove = self
            .extract_f64(log_content, "vm_prove_time_s")
            .map(|time| Stage1VmProve {
                vm_prove_time_s: time,
                vm_prove_segments: self.extract_u64(log_content, "segments"),
                vm_prove_cycles_per_segment: None,
                vm_core_proof_size_kb: self.extract_f64(log_content, "vm_core_proof_size_kb"),
                vm_prove_memory_mb: self.extract_f64(log_content, "peak_memory_mb"),
                data_source: Some(DataSource::LogParsed),
            });

        let stage_2_recursive =
            self.extract_f64(log_content, "recursive_prove_time_s")
                .map(|time| Stage2Recursive {
                    recursive_prove_time_s: time,
                    recursion_layers: None,
                    recursive_input_size_kb: None,
                    recursive_output_size_kb: None,
                    recursive_proof_count: None,
                    compressed_proof_size_kb: self
                        .extract_f64(log_content, "compressed_proof_size_kb"),
                    data_source: Some(DataSource::LogParsed),
                });

        let stage_4_snark = self
            .extract_f64(log_content, "snark_prove_time_s")
            .map(|time| Stage4Snark {
                snark_prove_time_s: time,
                snark_setup_time_s: None,
                snark_witness_time_s: None,
                snark_proof_time_s: None,
                groth16_proof_size_bytes: self.extract_u64(log_content, "groth16_proof_size_bytes"),
                data_source: Some(DataSource::LogParsed),
            });

        let proving_phase = ProvingPhase {
            proof_mode: ProofMode::Groth16, // Default, could be parsed
            security_bits: self.extract_u32(log_content, "security_bits"),
            fri_queries: None,
            blowup_factor: None,
            recursion_enabled: None,
            total_prove_time_s: total_prove_time,
            setup_time_s: None,
            stage_0_execute: None,
            stage_1_vm_prove,
            stage_2_recursive,
            stage_3_aggressive: None,
            stage_4_snark,
            time_breakdown_percent: None,
            proof_size_evolution: ProofSizeEvolution {
                stage_0_proof_size_mb: None,
                stage_1_proof_size_kb: self.extract_f64(log_content, "vm_core_proof_size_kb"),
                stage_2_proof_size_kb: self.extract_f64(log_content, "compressed_proof_size_kb"),
                stage_3_proof_size_kb: None,
                stage_4_proof_size_bytes: self.extract_u64(log_content, "groth16_proof_size_bytes"),
                final_proof_size_bytes: final_proof_size,
                total_compression_ratio: None,
            },
            performance_metrics: PerformanceMetrics {
                proving_throughput_kcycles_per_sec: None,
                khz: None,
                cycles_per_constraint: None,
                proof_efficiency_score: None,
            },
        };

        metrics.proving_phase = Some(proving_phase);

        // Parse verification phase
        if let Some(verify_time) = self.extract_f64(log_content, "verification_time_s") {
            let on_chain_gas =
                self.extract_u64_direct(log_content, "BENCHMARK: on_chain_gas_estimate=(\\d+)");
            metrics.verification_phase = Some(VerificationPhase {
                verification_time_s: verify_time,
                verification_time_ms: verify_time * 1000.0,
                on_chain_gas_estimate: on_chain_gas,
            });
        }

        // Parse resource metrics
        if let Some(peak_mem) = self.extract_f64(log_content, "peak_memory_mb") {
            metrics.resources = Some(ResourceMetrics {
                peak_memory_mb: Some(peak_mem),
                execution_memory_mb: None,
                proving_memory_mb: None,
                avg_cpu_percent: None,
                peak_cpu_percent: None,
                cpu_cores_used: None,
            });
        }

        // Update summary
        let verify_time = metrics
            .verification_phase
            .as_ref()
            .map(|v| v.verification_time_s)
            .unwrap_or(0.0);

        metrics.summary.total_time_s = metrics
            .execution_phase
            .as_ref()
            .and_then(|e| e.execution_time_s)
            .unwrap_or(0.0)
            + total_prove_time
            + verify_time;

        // Determine success status
        metrics.summary.success_status = self
            .extract_string(log_content, "success_status")
            .and_then(|s| match s.as_str() {
                "success" => Some(SuccessStatus::Success),
                "failed" => Some(SuccessStatus::Failed),
                "timeout" => Some(SuccessStatus::Timeout),
                _ => None,
            })
            .unwrap_or(SuccessStatus::Success);

        // Calculate derived metrics
        metrics.calculate_derived_metrics();

        Ok(metrics)
    }

    /// Extract u64 value from log
    fn extract_u64(&self, content: &str, key: &str) -> Option<u64> {
        self.patterns
            .get(key)?
            .captures(content)?
            .get(1)?
            .as_str()
            .parse()
            .ok()
    }

    /// Extract f64 value from log
    fn extract_f64(&self, content: &str, key: &str) -> Option<f64> {
        self.patterns
            .get(key)?
            .captures(content)?
            .get(1)?
            .as_str()
            .parse()
            .ok()
    }

    /// Extract u32 value from log
    fn extract_u32(&self, content: &str, key: &str) -> Option<u32> {
        self.patterns
            .get(key)?
            .captures(content)?
            .get(1)?
            .as_str()
            .parse()
            .ok()
    }

    /// Extract string value from log
    fn extract_string(&self, content: &str, key: &str) -> Option<String> {
        Some(
            self.patterns
                .get(key)?
                .captures(content)?
                .get(1)?
                .as_str()
                .to_string(),
        )
    }

    /// Extract string value directly using a pattern string
    fn extract_string_direct(&self, content: &str, pattern: &str) -> Option<String> {
        let regex = Regex::new(pattern).ok()?;
        Some(regex.captures(content)?.get(1)?.as_str().to_string())
    }

    /// Extract u64 value directly using a pattern string
    fn extract_u64_direct(&self, content: &str, pattern: &str) -> Option<u64> {
        let regex = Regex::new(pattern).ok()?;
        regex.captures(content)?.get(1)?.as_str().parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_parsing() {
        let log_patterns = LogPatterns {
            total_cycles: Some(r"total_cycles=(\d+)".to_string()),
            total_prove_time_s: Some(r"prove_time=([\d.]+)".to_string()),
            final_proof_size_bytes: Some(r"proof_size=(\d+)".to_string()),
            verification_time_s: Some(r"verify_time=([\d.]+)".to_string()),
            success_status: Some(r"status=(\w+)".to_string()),
            ..Default::default()
        };

        let parser = LogParser::new(&log_patterns).unwrap();

        let log_content = r#"
            total_cycles=12543
            prove_time=45.8
            proof_size=192
            verify_time=0.003
            status=success
        "#;

        let metrics = parser.parse(log_content, "TestVM", "fibonacci_10").unwrap();

        assert_eq!(metrics.metadata.zkvm_name, "TestVM");
        assert_eq!(metrics.summary.success_status, SuccessStatus::Success);
    }
}

impl Default for LogPatterns {
    fn default() -> Self {
        Self {
            total_cycles: None,
            total_instruction_count: None,
            total_prove_time_s: None,
            final_proof_size_bytes: None,
            verification_time_s: None,
            success_status: None,
            execution_time_s: None,
            vm_prove_time_s: None,
            recursive_prove_time_s: None,
            snark_prove_time_s: None,
            vm_core_proof_size_kb: None,
            compressed_proof_size_kb: None,
            groth16_proof_size_bytes: None,
            syscall_cycles: None,
            syscall_count: None,
            segments: None,
            peak_memory_mb: None,
            security_bits: None,
        }
    }
}
