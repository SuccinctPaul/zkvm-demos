//! Output Formatters for Log Analysis

pub mod csv;
pub mod json;
pub mod markdown;
pub mod text;

use crate::analyzers::log_analyzer::GenericMetrics;
use anyhow::Result;

/// Formatter trait for different output formats
pub trait Formatter {
    /// Format metrics into string output
    fn format(&self, metrics: &GenericMetrics) -> Result<String>;

    /// Get format name
    fn name(&self) -> &str;
}

/// Get formatter by name
pub fn get_formatter(name: &str) -> Option<Box<dyn Formatter>> {
    match name {
        "markdown" | "md" => Some(Box::new(markdown::MarkdownFormatter)),
        "csv" => Some(Box::new(csv::CsvFormatter)),
        "json" => Some(Box::new(json::JsonFormatter)),
        "text" | "txt" => Some(Box::new(text::TextFormatter)),
        _ => None,
    }
}

/// Format BenchmarkMetrics in a simple key-value format
pub fn format_benchmark_metrics_simple(
    metrics: &crate::metrics::BenchmarkMetrics,
    format: &str,
) -> Result<String> {
    match format {
        "json" => serde_json::to_string_pretty(metrics)
            .map_err(|e| anyhow::anyhow!("JSON serialization failed: {}", e)),
        "csv" => format_benchmark_metrics_csv(metrics),
        "text" => format_benchmark_metrics_text(metrics),
        _ => Err(anyhow::anyhow!("Unknown format: {}", format)),
    }
}

fn format_benchmark_metrics_csv(metrics: &crate::metrics::BenchmarkMetrics) -> Result<String> {
    let mut output = String::from("metric,value\n");

    // Metadata
    output.push_str(&format!("zkvm_name,{}\n", metrics.metadata.zkvm_name));
    output.push_str(&format!("program_name,{}\n", metrics.metadata.program_name));
    if let Some(ref v) = metrics.metadata.zkvm_version {
        output.push_str(&format!("zkvm_version,{}\n", v));
    }

    // Hardware info
    if let Some(ref hw) = metrics.metadata.hardware {
        output.push_str(&format!("cpu_brand,{}\n", hw.cpu_brand));
        output.push_str(&format!("cpu_cores,{}\n", hw.cpu_cores));
        output.push_str(&format!("cpu_physical_cores,{}\n", hw.cpu_physical_cores));
        if let Some(freq) = hw.cpu_frequency_mhz {
            output.push_str(&format!("cpu_frequency_mhz,{}\n", freq));
        }
        output.push_str(&format!("total_memory_mb,{}\n", hw.total_memory_mb));
        output.push_str(&format!("os_name,{}\n", hw.os_name));
        output.push_str(&format!("os_version,{}\n", hw.os_version));
        output.push_str(&format!("arch,{}\n", hw.arch));
    }

    // Execution phase
    if let Some(ref exec) = metrics.execution_phase {
        if let Some(v) = exec.total_cycles {
            output.push_str(&format!("total_cycles,{}\n", v));
        }
        if let Some(v) = exec.total_instruction_count {
            output.push_str(&format!("total_instruction_count,{}\n", v));
        }
        if let Some(v) = exec.execution_time_s {
            output.push_str(&format!("execution_time_s,{}\n", v));
        }
        if let Some(v) = exec.syscall_cycles {
            output.push_str(&format!("syscall_cycles,{}\n", v));
        }
        if let Some(v) = exec.syscall_count {
            output.push_str(&format!("syscall_count,{}\n", v));
        }
        if let Some(v) = exec.segments {
            output.push_str(&format!("segments,{}\n", v));
        }
    }

    // Proving phase
    if let Some(ref prov) = metrics.proving_phase {
        output.push_str(&format!("total_prove_time_s,{}\n", prov.total_prove_time_s));
        output.push_str(&format!(
            "final_proof_size_bytes,{}\n",
            prov.proof_size_evolution.final_proof_size_bytes
        ));

        if let Some(ref stage1) = prov.stage_1_vm_prove {
            output.push_str(&format!("vm_prove_time_s,{}\n", stage1.vm_prove_time_s));
            if let Some(v) = stage1.vm_core_proof_size_kb {
                output.push_str(&format!("vm_core_proof_size_kb,{}\n", v));
            }
        }

        if let Some(ref stage2) = prov.stage_2_recursive {
            output.push_str(&format!(
                "recursive_prove_time_s,{}\n",
                stage2.recursive_prove_time_s
            ));
            if let Some(v) = stage2.compressed_proof_size_kb {
                output.push_str(&format!("compressed_proof_size_kb,{}\n", v));
            }
        }

        if let Some(ref stage4) = prov.stage_4_snark {
            output.push_str(&format!(
                "snark_prove_time_s,{}\n",
                stage4.snark_prove_time_s
            ));
            if let Some(v) = stage4.groth16_proof_size_bytes {
                output.push_str(&format!("groth16_proof_size_bytes,{}\n", v));
            }
        }

        if let Some(v) = prov.security_bits {
            output.push_str(&format!("security_bits,{}\n", v));
        }
    }

    // Verification phase
    if let Some(ref verif) = metrics.verification_phase {
        output.push_str(&format!(
            "verification_time_s,{}\n",
            verif.verification_time_s
        ));
    }

    // Resources
    if let Some(ref res) = metrics.resources {
        if let Some(v) = res.peak_memory_mb {
            output.push_str(&format!("peak_memory_mb,{}\n", v));
        }
    }

    // Summary
    output.push_str(&format!(
        "success_status,{:?}\n",
        metrics.summary.success_status
    ));

    Ok(output)
}

fn format_benchmark_metrics_text(metrics: &crate::metrics::BenchmarkMetrics) -> Result<String> {
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

    // Hardware info
    if let Some(ref hw) = metrics.metadata.hardware {
        output.push_str("Hardware:\n");
        output.push_str(&format!(
            "  CPU: {} ({} cores, {} physical)\n",
            hw.cpu_brand, hw.cpu_cores, hw.cpu_physical_cores
        ));
        if let Some(freq) = hw.cpu_frequency_mhz {
            output.push_str(&format!("  CPU Frequency: {} MHz\n", freq));
        }
        output.push_str(&format!(
            "  Memory: {:.1} GB\n",
            hw.total_memory_mb as f64 / 1024.0
        ));
        output.push_str(&format!(
            "  OS: {} {} ({})\n",
            hw.os_name, hw.os_version, hw.arch
        ));
        output.push_str(&format!("  Hostname: {}\n", hw.hostname));
        output.push_str("\n");
    }

    // P0 Metrics
    output.push_str("P0 Metrics (Required):\n");
    if let Some(ref exec) = metrics.execution_phase {
        if let Some(v) = exec.total_cycles {
            output.push_str(&format!("  total_cycles: {}\n", v));
        }
        if let Some(v) = exec.total_instruction_count {
            output.push_str(&format!("  total_instruction_count: {}\n", v));
        }
    }

    if let Some(ref prov) = metrics.proving_phase {
        output.push_str(&format!(
            "  total_prove_time_s: {:.3}s\n",
            prov.total_prove_time_s
        ));
        output.push_str(&format!(
            "  final_proof_size_bytes: {}\n",
            prov.proof_size_evolution.final_proof_size_bytes
        ));
    }

    if let Some(ref verif) = metrics.verification_phase {
        output.push_str(&format!(
            "  verification_time_s: {:.3}s\n",
            verif.verification_time_s
        ));
    }

    output.push_str(&format!(
        "  success_status: {:?}\n",
        metrics.summary.success_status
    ));
    output.push_str("\n");

    // P1 Metrics
    let mut has_p1 = false;
    let mut p1_output = String::from("P1 Metrics (High Priority):\n");

    if let Some(ref exec) = metrics.execution_phase {
        if let Some(v) = exec.execution_time_s {
            p1_output.push_str(&format!("  execution_time_s: {:.3}s\n", v));
            has_p1 = true;
        }
    }

    if let Some(ref prov) = metrics.proving_phase {
        if let Some(ref stage1) = prov.stage_1_vm_prove {
            p1_output.push_str(&format!(
                "  vm_prove_time_s: {:.3}s\n",
                stage1.vm_prove_time_s
            ));
            if let Some(v) = stage1.vm_core_proof_size_kb {
                p1_output.push_str(&format!("  vm_core_proof_size_kb: {:.3} KB\n", v));
            }
            has_p1 = true;
        }

        if let Some(ref stage2) = prov.stage_2_recursive {
            p1_output.push_str(&format!(
                "  recursive_prove_time_s: {:.3}s\n",
                stage2.recursive_prove_time_s
            ));
            if let Some(v) = stage2.compressed_proof_size_kb {
                p1_output.push_str(&format!("  compressed_proof_size_kb: {:.3} KB\n", v));
            }
            has_p1 = true;
        }

        if let Some(ref stage4) = prov.stage_4_snark {
            p1_output.push_str(&format!(
                "  snark_prove_time_s: {:.3}s\n",
                stage4.snark_prove_time_s
            ));
            if let Some(v) = stage4.groth16_proof_size_bytes {
                p1_output.push_str(&format!("  groth16_proof_size_bytes: {}\n", v));
            }
            has_p1 = true;
        }
    }

    if has_p1 {
        output.push_str(&p1_output);
        output.push_str("\n");
    }

    // P2+ Metrics
    let mut has_p2 = false;
    let mut p2_output = String::from("P2+ Metrics (Optional):\n");

    if let Some(ref exec) = metrics.execution_phase {
        if let Some(v) = exec.syscall_cycles {
            p2_output.push_str(&format!("  syscall_cycles: {}\n", v));
            has_p2 = true;
        }
        if let Some(v) = exec.syscall_count {
            p2_output.push_str(&format!("  syscall_count: {}\n", v));
            has_p2 = true;
        }
        if let Some(v) = exec.segments {
            p2_output.push_str(&format!("  segments: {}\n", v));
            has_p2 = true;
        }
    }

    if let Some(ref res) = metrics.resources {
        if let Some(v) = res.peak_memory_mb {
            p2_output.push_str(&format!("  peak_memory_mb: {:.1} MB\n", v));
            has_p2 = true;
        }
    }

    if let Some(ref prov) = metrics.proving_phase {
        if let Some(v) = prov.security_bits {
            p2_output.push_str(&format!("  security_bits: {}\n", v));
            has_p2 = true;
        }
    }

    if has_p2 {
        output.push_str(&p2_output);
        output.push_str("\n");
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_formatter() {
        assert!(get_formatter("markdown").is_some());
        assert!(get_formatter("csv").is_some());
        assert!(get_formatter("json").is_some());
        assert!(get_formatter("text").is_some());
        assert!(get_formatter("unknown").is_none());
    }

    #[test]
    fn test_formatter_names() {
        assert!(get_formatter("md").is_some());
        assert!(get_formatter("txt").is_some());
    }
}
