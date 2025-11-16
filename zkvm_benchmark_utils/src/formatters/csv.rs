//! CSV Formatter

use super::Formatter;
use crate::analyzers::log_analyzer::GenericMetrics;
use anyhow::Result;

pub struct CsvFormatter;

impl Formatter for CsvFormatter {
    fn format(&self, metrics: &GenericMetrics) -> Result<String> {
        let mut output = String::new();

        // Header
        output.push_str("metric,value\n");

        // Add all metrics
        if let Some(v) = &metrics.program_name {
            output.push_str(&format!("program_name,{}\n", v));
        }
        if let Some(v) = &metrics.zkvm_name {
            output.push_str(&format!("zkvm_name,{}\n", v));
        }
        if let Some(v) = metrics.total_cycles {
            output.push_str(&format!("total_cycles,{}\n", v));
        }
        if let Some(v) = metrics.execution_time_s {
            output.push_str(&format!("execution_time_s,{}\n", v));
        }
        if let Some(v) = metrics.total_prove_time_s {
            output.push_str(&format!("total_prove_time_s,{}\n", v));
        }
        if let Some(v) = metrics.vm_core_proof_size_kb {
            output.push_str(&format!("vm_core_proof_size_kb,{}\n", v));
        }
        if let Some(v) = metrics.groth16_proof_size_bytes {
            output.push_str(&format!("groth16_proof_size_bytes,{}\n", v));
        }
        if let Some(v) = metrics.verification_time_ms {
            output.push_str(&format!("verification_time_ms,{}\n", v));
        }
        if let Some(v) = metrics.total_time_s {
            output.push_str(&format!("total_time_s,{}\n", v));
        }
        if let Some(v) = metrics.throughput_khz {
            output.push_str(&format!("throughput_khz,{}\n", v));
        }

        Ok(output)
    }

    fn name(&self) -> &str {
        "csv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_formatter() {
        let formatter = CsvFormatter;
        let metrics = GenericMetrics {
            zkvm_name: Some("test".to_string()),
            total_cycles: Some(1000),
            ..Default::default()
        };

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("metric,value"));
        assert!(result.contains("zkvm_name,test"));
        assert!(result.contains("total_cycles,1000"));
    }

    #[test]
    fn test_csv_header() {
        let formatter = CsvFormatter;
        let metrics = GenericMetrics::default();

        let result = formatter.format(&metrics).unwrap();
        assert!(result.starts_with("metric,value\n"));
    }

    #[test]
    fn test_formatter_name() {
        let formatter = CsvFormatter;
        assert_eq!(formatter.name(), "csv");
    }
}
