//! Simple key-value formatter for BenchmarkMetrics
//!
//! Provides basic formatting for single-run metrics, useful for quick analysis.

use crate::core::metrics::BenchmarkMetrics;
use anyhow::Result;
use serde_json;

/// Format BenchmarkMetrics in a simple key-value format
pub fn format_benchmark_metrics_simple(metrics: &BenchmarkMetrics, format: &str) -> Result<String> {
    match format {
        "json" => serde_json::to_string_pretty(metrics)
            .map_err(|e| anyhow::anyhow!("JSON serialization failed: {}", e)),
        "csv" => format_benchmark_metrics_csv(metrics),
        "text" => format_benchmark_metrics_text(metrics),
        _ => Err(anyhow::anyhow!("Unknown format: {}", format)),
    }
}

fn format_benchmark_metrics_csv(metrics: &BenchmarkMetrics) -> Result<String> {
    let mut output = String::from("metric,value\n");

    // Metadata
    output.push_str(&format!("zkvm_name,{}\n", metrics.metadata.zkvm_name));
    output.push_str(&format!("program_name,{}\n", metrics.metadata.program_name));
    if let Some(ref v) = metrics.metadata.zkvm_version {
        output.push_str(&format!("zkvm_version,{}\n", v));
    }

    // Custom/Dynamic metrics (P0 priority for raw values)
    for (key, value) in &metrics.custom_metrics {
        output.push_str(&format!("{},{}\n", key, value));
    }

    // Summary
    output.push_str(&format!(
        "success_status,{:?}\n",
        metrics.summary.success_status
    ));

    Ok(output)
}

fn format_benchmark_metrics_text(metrics: &BenchmarkMetrics) -> Result<String> {
    let mut output = String::from("=== Extracted Metrics ===\n\n");

    // Metadata
    output.push_str("Metadata:\n");
    output.push_str(&format!("  zkVM: {}\n", metrics.metadata.zkvm_name));
    output.push_str(&format!("  Program: {}\n", metrics.metadata.program_name));
    if let Some(ref v) = metrics.metadata.zkvm_version {
        output.push_str(&format!("  Version: {}\n", v));
    }
    output.push_str(&format!("  Timestamp: {}\n", metrics.metadata.timestamp));
    output.push_str("\n");

    // Custom/Dynamic Metrics
    output.push_str("Metrics:\n");

    // Sort keys for deterministic output
    let mut keys: Vec<_> = metrics.custom_metrics.keys().collect();
    keys.sort();

    for key in keys {
        if let Some(value) = metrics.custom_metrics.get(key) {
            output.push_str(&format!("  {}: {}\n", key, value));
        }
    }

    output.push_str("\n");
    output.push_str(&format!(
        "  success_status: {:?}\n",
        metrics.summary.success_status
    ));
    output.push_str("\n");

    Ok(output)
}
