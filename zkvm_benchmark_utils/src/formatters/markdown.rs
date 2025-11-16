//! Markdown Formatter

use super::Formatter;
use crate::analyzers::log_analyzer::GenericMetrics;
use anyhow::Result;

pub struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, metrics: &GenericMetrics) -> Result<String> {
        let mut output = String::new();
        output.push_str("# zkVM Benchmark Analysis\n\n");

        // Metadata
        if metrics.program_name.is_some() || metrics.zkvm_name.is_some() {
            output.push_str("## Metadata\n\n");
            if let Some(v) = &metrics.program_name {
                output.push_str(&format!("- **Program**: {}\n", v));
            }
            if let Some(v) = &metrics.zkvm_name {
                output.push_str(&format!("- **zkVM**: {}\n", v));
            }
            if let Some(v) = &metrics.zkvm_version {
                output.push_str(&format!("- **Version**: {}\n", v));
            }
            output.push_str("\n");
        }

        // Execution Phase
        output.push_str("## Execution Phase\n\n");
        output.push_str("| Metric | Value |\n");
        output.push_str("|--------|-------|\n");
        if let Some(v) = metrics.total_cycles {
            output.push_str(&format!("| Total Cycles | {} |\n", v));
        }
        if let Some(v) = metrics.total_instruction_count {
            output.push_str(&format!("| Instructions | {} |\n", v));
        }
        if let Some(v) = metrics.execution_time_s {
            output.push_str(&format!("| Execution Time | {:.3}s |\n", v));
        }
        output.push_str("\n");

        // Proving Phase
        if metrics.total_prove_time_s.is_some() {
            output.push_str("## Proving Phase\n\n");
            if let Some(total) = metrics.total_prove_time_s {
                output.push_str(&format!("**Total Prove Time**: {:.3}s\n\n", total));
            }
        }

        // Verification
        if metrics.verification_time_ms.is_some() {
            output.push_str("## Verification Phase\n\n");
            if let Some(v) = metrics.verification_time_ms {
                output.push_str(&format!("- **Verify Time**: {:.3}ms\n", v));
            }
            output.push_str("\n");
        }

        // Summary
        output.push_str("## Summary\n\n");
        if let Some(v) = metrics.total_time_s {
            output.push_str(&format!("- **Total Time**: {:.3}s\n", v));
        }
        if let Some(v) = metrics.throughput_khz {
            output.push_str(&format!("- **Throughput**: {:.3} kHz\n", v));
        }

        Ok(output)
    }

    fn name(&self) -> &str {
        "markdown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_formatter() {
        let formatter = MarkdownFormatter;
        let metrics = GenericMetrics {
            zkvm_name: Some("test".to_string()),
            program_name: Some("fibonacci".to_string()),
            total_cycles: Some(1000),
            execution_time_s: Some(1.5),
            ..Default::default()
        };

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("# zkVM Benchmark Analysis"));
        assert!(result.contains("## Metadata"));
        assert!(result.contains("fibonacci"));
        assert!(result.contains("test"));
    }

    #[test]
    fn test_markdown_tables() {
        let formatter = MarkdownFormatter;
        let metrics = GenericMetrics {
            total_cycles: Some(1000),
            ..Default::default()
        };

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("| Metric | Value |"));
        assert!(result.contains("|--------|-------|"));
    }

    #[test]
    fn test_formatter_name() {
        let formatter = MarkdownFormatter;
        assert_eq!(formatter.name(), "markdown");
    }
}
