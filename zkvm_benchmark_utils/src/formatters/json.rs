//! JSON Formatter

use super::Formatter;
use crate::analyzers::log_analyzer::GenericMetrics;
use anyhow::Result;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, metrics: &GenericMetrics) -> Result<String> {
        Ok(serde_json::to_string_pretty(metrics)?)
    }

    fn name(&self) -> &str {
        "json"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter;
        let metrics = GenericMetrics {
            zkvm_name: Some("test".to_string()),
            total_cycles: Some(1000),
            ..Default::default()
        };

        let result = formatter.format(&metrics).unwrap();
        assert!(result.contains("\"zkvm_name\""));
        assert!(result.contains("test"));
        assert!(result.contains("\"total_cycles\""));
    }

    #[test]
    fn test_formatter_name() {
        let formatter = JsonFormatter;
        assert_eq!(formatter.name(), "json");
    }
}
