//! Plain Text Formatter

use super::Formatter;
use crate::analyzers::log_analyzer::GenericMetrics;
use anyhow::Result;

pub struct TextFormatter;

impl Formatter for TextFormatter {
    fn format(&self, metrics: &GenericMetrics) -> Result<String> {
        let mut output = String::new();
        output.push_str("=== zkVM Benchmark Analysis ===\n\n");

        if let Some(v) = &metrics.zkvm_name {
            output.push_str(&format!("zkVM: {}\n", v));
        }
        if let Some(v) = &metrics.program_name {
            output.push_str(&format!("Program: {}\n", v));
        }
        output.push_str("\n");

        if let Some(v) = metrics.total_cycles {
            output.push_str(&format!("Total Cycles: {}\n", v));
        }
        if let Some(v) = metrics.execution_time_s {
            output.push_str(&format!("Execution Time: {:.3}s\n", v));
        }
        if let Some(v) = metrics.total_prove_time_s {
            output.push_str(&format!("Prove Time: {:.3}s\n", v));
        }
        if let Some(v) = metrics.groth16_proof_size_bytes {
            output.push_str(&format!("Proof Size: {} bytes\n", v));
        }
        if let Some(v) = metrics.verification_time_ms {
            output.push_str(&format!("Verify Time: {:.3}ms\n", v));
        }
        if let Some(v) = metrics.total_time_s {
            output.push_str(&format!("\nTotal Time: {:.3}s\n", v));
        }
        if let Some(v) = metrics.throughput_khz {
            output.push_str(&format!("Throughput: {:.3} kHz\n", v));
        }

        Ok(output)
    }

    fn name(&self) -> &str {
        "text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_formatter() {
        let formatter = TextFormatter;
        let metrics = GenericMetrics {
            zkvm_name: Some("test".to_string()),
            program_name: Some("fibonacci".to_string()),
            total_cycles: Some(1000),
            ..Default::default()
        };

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("=== zkVM Benchmark Analysis ==="));
        assert!(result.contains("zkVM: test"));
        assert!(result.contains("Program: fibonacci"));
    }

    #[test]
    fn test_text_empty_metrics() {
        let formatter = TextFormatter;
        let metrics = GenericMetrics::default();

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("=== zkVM Benchmark Analysis ==="));
    }

    #[test]
    fn test_formatter_name() {
        let formatter = TextFormatter;
        assert_eq!(formatter.name(), "text");
    }
}
