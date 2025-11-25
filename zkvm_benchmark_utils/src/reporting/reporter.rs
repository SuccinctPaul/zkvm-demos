//! Benchmark reporter - generates various output formats
//!
//! Currently optimized for CSV reporting.

use chrono;
use csv::Writer;
use log::{info, warn};
use std::fs;
use std::path::Path;

use crate::core::error::Result;
use crate::core::metrics::{ProgramName, ProofMode, ZkVmName};
use crate::execution::types::ExecutionResult;

/// Report format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportFormat {
    Csv,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "csv" => Some(Self::Csv),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Csv]
    }
}

pub struct BenchmarkReporter {
    results: Vec<ExecutionResult>,
}

impl BenchmarkReporter {
    pub fn new(results: Vec<ExecutionResult>) -> Self {
        Self { results }
    }

    /// Generate reports with specified formats (or all if None)
    pub fn generate(&self, output_dir: &Path, formats: Option<Vec<ReportFormat>>) -> Result<()> {
        // Ensure output directory exists
        fs::create_dir_all(output_dir)?;

        // Check if we have any results
        if self.results.is_empty() {
            warn!("No results to generate reports for");
            return Ok(());
        }

        let formats = formats.unwrap_or_else(ReportFormat::all);

        // Generate timestamp for file naming: YYYYMMDD-HHMMSS
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();

        // Extract zkvm and mode info from first result for filename
        // Format: Timestamp_zkvm_provemode_programName_param_report
        let (zkvm_name, mode, param) = if let Some(first) = self.results.first() {
            (
                first.test_run.zkvm_name.to_string(),
                first.test_run.mode.to_string(),
                first.test_run.scale.to_string(),
            )
        } else {
            (
                "unknown".to_string(),
                "unknown".to_string(),
                "0".to_string(),
            )
        };

        // TODO here should obtain from the metrics.
        let program_name = "fibonacci";

        let base_filename = format!(
            "{}_{}_{}_{}_{}_report",
            timestamp, zkvm_name, mode, program_name, param
        );

        info!(
            "Generating {} reports for {} results in {:?}",
            formats.len(),
            self.results.len(),
            output_dir
        );

        for format in formats {
            match format {
                ReportFormat::Csv => {
                    let filename = format!("{}.csv", base_filename);
                    let path = output_dir.join(&filename);
                    self.generate_csv(&path)?;
                    info!("Generated CSV: {}", path.display());
                }
            }
        }

        Ok(())
    }

    /// Generate all reports (convenience method)
    pub fn generate_all(&self, output_dir: &Path) -> Result<()> {
        self.generate(output_dir, None)
    }

    /// Generate CSV comparison table
    pub fn generate_csv<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut writer = Writer::from_path(path)?;

        // Always use default headers
        let headers = Self::default_headers();
        let header_len = headers.len();

        // Write header
        writer.write_record(&headers)?;

        // 1. Group results by (zkvm, program, scale) to aggregate metrics
        // Key: (zkvm, program, scale)
        // Value: Merged Metrics
        use std::collections::BTreeMap;
        let mut grouped_metrics: BTreeMap<
            (ZkVmName, ProgramName, u32),
            crate::core::metrics::UnifiedMetrics,
        > = BTreeMap::new();
        let mut grouped_success: BTreeMap<(ZkVmName, ProgramName, u32), bool> = BTreeMap::new();

        for result in &self.results {
            if let Some(metrics) = &result.metrics {
                let program = metrics.metadata.program_name.clone();
                let zkvm = result.test_run.zkvm_name.clone();
                let scale = result.test_run.scale;
                let key = (zkvm.clone(), program.clone(), scale);

                // Initialize entry if not exists
                let entry = grouped_metrics.entry(key.clone()).or_insert_with(|| {
                    let mut m = crate::core::metrics::UnifiedMetrics::new(program, zkvm);
                    m.metadata.scale = Some(scale);
                    m.metadata.mode = metrics.metadata.mode.clone(); // Preserve mode from first result
                    m.metadata.zkvm_version = metrics.metadata.zkvm_version.clone(); // Preserve version
                    m
                });

                // Merge Logic
                let mode = &result.test_run.mode;

                // 1. Execution Metrics (Prefer Core for cycles, but update if missing)
                if entry.execution.total_cycles.is_none() {
                    entry.execution = metrics.execution.clone();
                } else if matches!(mode, ProofMode::Core) {
                    // Core usually has the most accurate execution stats
                    entry.execution = metrics.execution.clone();
                }

                // 2. VM Circuit Metrics (From Core)
                if matches!(mode, ProofMode::Core) {
                    entry.vm_circuit = metrics.vm_circuit.clone();
                }

                // 3. Recursion Metrics (From Compressed)
                if matches!(mode, ProofMode::Compressed) {
                    entry.aggressive = metrics.aggressive.clone();
                }

                // 4. Snark Metrics (From Groth16/Plonk)
                if matches!(mode, ProofMode::Groth16 | ProofMode::Plonk) {
                    entry.snark = metrics.snark.clone();
                    // Verification usually comes with Snark
                    entry.verification = metrics.verification.clone();
                }

                // 5. Custom Metrics (Union)
                for (k, v) in &metrics.custom_metrics {
                    entry.custom_metrics.insert(k.clone(), v.clone());
                }

                // 6. Resources (Prefer Groth16, otherwise Max)
                if let Some(stats) = &result.resource_stats {
                    if matches!(mode, ProofMode::Groth16 | ProofMode::Plonk) {
                        // User requested to prioritize Groth16 data
                        entry.resources.peak_memory_mb = Some(stats.peak_memory_mb);
                        entry.resources.avg_cpu_usage_percent = Some(stats.avg_cpu_percent);
                    } else if entry.resources.peak_memory_mb.is_none() {
                        // Fill if empty
                        entry.resources.peak_memory_mb = Some(stats.peak_memory_mb);
                        entry.resources.avg_cpu_usage_percent = Some(stats.avg_cpu_percent);
                    } else {
                        // If we don't have groth16 yet, take max
                        let current = entry.resources.peak_memory_mb.unwrap_or(0.0);
                        if stats.peak_memory_mb > current {
                            entry.resources.peak_memory_mb = Some(stats.peak_memory_mb);
                            entry.resources.avg_cpu_usage_percent = Some(stats.avg_cpu_percent);
                        }
                    }
                }

                // 7. Success (Logical AND)
                let success_entry = grouped_success.entry(key).or_insert(true);
                *success_entry = *success_entry && result.success;
            }
        }

        // 2. Write rows from aggregated metrics
        for ((zkvm, program, scale), mut metrics) in grouped_metrics {
            let success = grouped_success
                .get(&(zkvm.clone(), program.clone(), scale))
                .unwrap_or(&false);

            // Ensure derived metrics (like KHZ) are up-to-date after aggregation
            metrics.update_derived_metrics();

            // Set success in summary
            metrics.summary.success = *success;

            let row = Self::get_default_metric_values(&metrics);

            // Double check row length
            if row.len() != header_len {
                warn!(
                    "Row length mismatch in aggregated CSV! Header: {}, Row: {}",
                    header_len,
                    row.len()
                );
            }

            writer.write_record(&row)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn default_headers() -> Vec<String> {
        vec![
            // Metadata
            "zkvm_Name".to_string(),
            "mode".to_string(),
            "scale".to_string(),
            "program_name".to_string(),
            "zkvm_Version".to_string(),
            // Execution Phase
            "execution_time_s".to_string(),
            "total_cycles".to_string(),
            "instruction_count".to_string(),
            "total_syscall_cycles".to_string(),
            "touched_memory_addresses".to_string(),
            // VM Prove Phase
            "vm_chunk_count".to_string(),
            "vm_chunk_size_rows".to_string(),
            "vm_prove_time_s".to_string(),
            "vm_prove_khz".to_string(),
            "vm_proof_size_bytes".to_string(),
            // Recursive Prove Phase
            "recursion_layers".to_string(),
            // Aggressive Prove Phase
            "aggressive_prove_time_s".to_string(),
            "aggressive_proof_size_bytes".to_string(),
            // Groth16/SNARK Phase
            "snark_constraints".to_string(),
            "snark_prove_time_s".to_string(),
            "snark_proof_size_bytes".to_string(),
            // Verification Phase
            "verification_time_s".to_string(),
            // Summary
            "total_time_s".to_string(),
            // Resources & Status
            "peak_memory_MB".to_string(),
            "avg_cpu_percent".to_string(),
            "success".to_string(),
        ]
    }

    fn get_default_metric_values(metrics: &crate::core::metrics::UnifiedMetrics) -> Vec<String> {
        vec![
            // Metadata
            metrics.metadata.zkvm_name.to_string(),
            metrics
                .metadata
                .mode
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| ProofMode::Core.to_string()),
            metrics
                .metadata
                .scale
                .map(|s| s.to_string())
                .unwrap_or_default(),
            metrics.metadata.program_name.to_string(),
            metrics
                .metadata
                .zkvm_version
                .clone()
                .unwrap_or_else(|| "N/A".to_string()),
            // Execution Phase
            metrics
                .execution
                .duration_s
                .map(|v| format!("{:.6}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .execution
                .total_cycles
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .execution
                .instruction_count
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .execution
                .total_syscall_cycles
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .execution
                .touched_memory_addresses
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            // VM Prove Phase
            metrics
                .vm_circuit
                .chunk_count
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .vm_circuit
                .chunk_size_rows
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .vm_circuit
                .duration_s
                .map(|v| format!("{:.3}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .vm_circuit
                .proving_khz
                .map(|v| format!("{:.3}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .vm_circuit
                .proof_size_bytes
                .map(|b| b.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            // Recursive Prove Phase
            metrics
                .aggressive
                .recursion_layers
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            // Aggressive Prove Phase
            metrics
                .aggressive
                .duration_s
                .map(|v| format!("{:.3}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .aggressive
                .proof_size_bytes
                .map(|b| b.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            // SNARK/Groth16 Phase
            metrics
                .snark
                .constraint_count
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .snark
                .proof_gen_time_s
                .map(|v| format!("{:.3}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .snark
                .proof_size_bytes
                .map(|v| v.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            // Verification Phase
            metrics
                .verification
                .duration_s
                .map(|v| format!("{:.6}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            // Summary
            format!("{:.3}", metrics.summary.total_time_s),
            // Resources & Status
            metrics
                .resources
                .peak_memory_mb
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .resources
                .avg_cpu_usage_percent
                .map(|v| format!("{:.1}", v))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics.summary.success.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reporter_creation() {
        let results = vec![];
        let reporter = BenchmarkReporter::new(results);
        assert_eq!(reporter.results.len(), 0);
    }
}
