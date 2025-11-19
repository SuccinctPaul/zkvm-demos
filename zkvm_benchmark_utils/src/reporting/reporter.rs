//! Benchmark reporter - generates various output formats

use chrono;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use csv::Writer;
use log::info;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::core::config::ReportingConfig;
use crate::core::error::Result;
use crate::execution::types::ExecutionResult;
use crate::reporting::statistics::{calculate_statistics, Statistics};

/// Report format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportFormat {
    Csv,
    Json,
    Markdown,
    Console,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "markdown" | "md" => Some(Self::Markdown),
            "console" | "table" => Some(Self::Console),
            _ => None,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Csv, Self::Json, Self::Markdown, Self::Console]
    }
}

pub struct BenchmarkReporter {
    results: Vec<ExecutionResult>,
    config: Option<ReportingConfig>,
}

impl BenchmarkReporter {
    pub fn new(results: Vec<ExecutionResult>, config: Option<ReportingConfig>) -> Self {
        Self { results, config }
    }

    /// Group results by zkvm, mode, and scale (ignoring repeat count)
    fn group_results(&self) -> HashMap<(String, String, u32), Vec<&ExecutionResult>> {
        let mut grouped: HashMap<(String, String, u32), Vec<&ExecutionResult>> = HashMap::new();

        for result in &self.results {
            let key = (
                result.test_run.zkvm_name.clone(),
                result.test_run.mode.clone(),
                result.test_run.scale,
            );
            grouped.entry(key).or_insert_with(Vec::new).push(result);
        }

        grouped
    }

    /// Calculate statistics for prove time from multiple runs
    fn calculate_prove_time_stats(&self, results: &[&ExecutionResult]) -> Option<Statistics> {
        let values: Vec<f64> = results
            .iter()
            .filter_map(|r| {
                r.metrics
                    .as_ref()
                    .and_then(|m| m.proving_phase.as_ref())
                    .map(|p| p.total_prove_time_s)
            })
            .collect();

        if values.len() > 1 {
            calculate_statistics(values, 2.0) // 2.0 is Z-score threshold for outliers
        } else {
            None
        }
    }

    /// Generate reports with specified formats (or all if None)
    pub fn generate(&self, output_dir: &Path, formats: Option<Vec<ReportFormat>>) -> Result<()> {
        let formats = formats.unwrap_or_else(ReportFormat::all);

        // Generate timestamp for file naming: YYYYMMDD-HHMMSS
        let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();

        // Extract zkvm and mode info from first result for filename
        let (zkvm_name, mode) = if let Some(first) = self.results.first() {
            (
                first.test_run.zkvm_name.as_str(),
                first.test_run.mode.as_str(),
            )
        } else {
            ("unknown", "unknown")
        };

        for format in formats {
            match format {
                ReportFormat::Csv => {
                    let filename = format!("{}-{}-{}.csv", zkvm_name, mode, timestamp);
                    let path = output_dir.join(&filename);
                    self.generate_csv(&path)?;
                    info!("Generated CSV: {}", path.display());
                }
                ReportFormat::Json => {
                    let filename = format!("{}-{}-{}.json", zkvm_name, mode, timestamp);
                    let path = output_dir.join(&filename);
                    self.generate_json_summary(&path)?;
                    info!("Generated JSON: {}", path.display());
                }
                ReportFormat::Markdown => {
                    let filename = format!("{}-{}-{}.md", zkvm_name, mode, timestamp);
                    let path = output_dir.join(&filename);
                    self.generate_markdown(&path)?;
                    info!("Generated Markdown: {}", path.display());
                }
                ReportFormat::Console => {
                    self.print_summary_table();
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

        let headers = if let Some(config) = &self.config {
            if !config.metrics.is_empty() {
                let mut h = vec![
                    "zkvm_name".to_string(),
                    "mode".to_string(),
                    "scale".to_string(),
                ];
                h.extend(config.metrics.clone());
                h.push("success".to_string());
                h
            } else {
                Self::default_headers()
            }
        } else {
            Self::default_headers()
        };

        // Write header
        writer.write_record(&headers)?;

        // Write data rows
        for result in &self.results {
            let mut row = vec![
                result.test_run.zkvm_name.clone(),
                result.test_run.mode.clone(),
                result.test_run.scale.to_string(),
            ];

            if let Some(metrics) = &result.metrics {
                // If config exists, use it to fetch metrics dynamically
                if let Some(config) = &self.config {
                    if !config.metrics.is_empty() {
                        for metric_key in &config.metrics {
                            row.push(metrics.get_value(metric_key));
                        }
                    } else {
                        row.extend(Self::get_default_metric_values(result, metrics));
                    }
                } else {
                    row.extend(Self::get_default_metric_values(result, metrics));
                }
            } else {
                // Fill with N/A if metrics missing (except first 3 columns)
                for _ in 3..headers.len() - 1 {
                    row.push("N/A".to_string());
                }
            }
            row.push(result.success.to_string());

            writer.write_record(&row)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn default_headers() -> Vec<String> {
        vec![
            "zkvm_name".to_string(),
            "mode".to_string(),
            "scale".to_string(),
            "total_cycles".to_string(),
            "total_prove_time_s".to_string(),
            "vm_core_proof_size_kb".to_string(),
            "compressed_proof_size_kb".to_string(),
            "groth16_proof_size_bytes".to_string(),
            "verification_time_s".to_string(),
            "khz".to_string(),
            "total_time_s".to_string(),
            "peak_memory_mb".to_string(),
            "avg_cpu_percent".to_string(),
            "disk_read_mb".to_string(),
            "disk_write_mb".to_string(),
            "success".to_string(),
        ]
    }

    fn get_default_metric_values(
        result: &ExecutionResult,
        metrics: &crate::core::metrics::BenchmarkMetrics,
    ) -> Vec<String> {
        vec![
            metrics
                .execution_phase
                .as_ref()
                .and_then(|e| e.total_cycles)
                .map(|c| c.to_string())
                .unwrap_or_default(),
            metrics
                .proving_phase
                .as_ref()
                .map(|p| format!("{:.3}", p.total_prove_time_s))
                .unwrap_or_default(),
            metrics
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_1_proof_size_kb)
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_2_proof_size_kb)
                .map(|s| format!("{:.2}", s))
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_4_proof_size_bytes)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            metrics
                .verification_phase
                .as_ref()
                .map(|v| format!("{:.6}", v.verification_time_s))
                .unwrap_or_default(),
            metrics
                .proving_phase
                .as_ref()
                .and_then(|p| p.performance_metrics.khz)
                .map(|k| format!("{:.3}", k))
                .unwrap_or_default(),
            format!("{:.3}", metrics.summary.total_time_s),
            result
                .resource_stats
                .as_ref()
                .map(|r| format!("{:.1}", r.peak_memory_mb))
                .unwrap_or_default(),
            result
                .resource_stats
                .as_ref()
                .map(|r| format!("{:.1}", r.avg_cpu_percent))
                .unwrap_or_default(),
            result
                .resource_stats
                .as_ref()
                .map(|r| format!("{:.2}", r.total_disk_read_mb))
                .unwrap_or_default(),
            result
                .resource_stats
                .as_ref()
                .map(|r| format!("{:.2}", r.total_disk_write_mb))
                .unwrap_or_default(),
        ]
    }

    /// Generate JSON summary
    pub fn generate_json_summary<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let summary = self.create_summary();
        let json = serde_json::to_string_pretty(&summary)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Generate Markdown report
    pub fn generate_markdown<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut md = String::new();

        md.push_str("# zkVM Benchmark Report\n\n");
        md.push_str(&format!(
            "Generated: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Add hardware information if available
        if let Some(result) = self.results.first() {
            if let Some(metrics) = &result.metrics {
                if let Some(hw) = &metrics.metadata.hardware {
                    md.push_str("## Hardware Environment\n\n");
                    md.push_str(&format!(
                        "- **CPU**: {} ({} cores, {} physical)\n",
                        hw.cpu_brand, hw.cpu_cores, hw.cpu_physical_cores
                    ));
                    if let Some(freq) = hw.cpu_frequency_mhz {
                        md.push_str(&format!("- **CPU Frequency**: {} MHz\n", freq));
                    }
                    md.push_str(&format!(
                        "- **Memory**: {:.1} GB total, {:.1} GB available\n",
                        hw.total_memory_mb as f64 / 1024.0,
                        hw.available_memory_mb as f64 / 1024.0
                    ));
                    md.push_str(&format!(
                        "- **OS**: {} {} (Kernel: {})\n",
                        hw.os_name, hw.os_version, hw.kernel_version
                    ));
                    md.push_str(&format!("- **Architecture**: {}\n", hw.arch));
                    md.push_str(&format!("- **Hostname**: {}\n\n", hw.hostname));
                }
            }
        }

        // Summary statistics
        md.push_str("## Summary\n\n");
        md.push_str(&format!("- Total test runs: {}\n", self.results.len()));
        md.push_str(&format!(
            "- Successful runs: {}\n",
            self.results.iter().filter(|r| r.success).count()
        ));
        md.push_str(&format!(
            "- Failed runs: {}\n\n",
            self.results.iter().filter(|r| !r.success).count()
        ));

        // Group by zkVM
        let mut by_zkvm: HashMap<String, Vec<&ExecutionResult>> = HashMap::new();
        for result in &self.results {
            by_zkvm
                .entry(result.test_run.zkvm_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Create comparison table
        md.push_str("## Performance Comparison\n\n");
        md.push_str("| zkVM | Mode | Scale | Cycles | Prove Time (s) | VM Proof (KB) | Compressed (KB) | Groth16 (B) | Verify (ms) | kHz |\n");
        md.push_str("|------|------|-------|--------|----------------|---------------|-----------------|-------------|-------------|-----|\n");

        for result in &self.results {
            if let Some(metrics) = &result.metrics {
                md.push_str(&format!(
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                    metrics.metadata.zkvm_name,
                    result.test_run.mode,
                    result.test_run.scale,
                    metrics
                        .execution_phase
                        .as_ref()
                        .and_then(|e| e.total_cycles)
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .proving_phase
                        .as_ref()
                        .map(|p| format!("{:.2}", p.total_prove_time_s))
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .proving_phase
                        .as_ref()
                        .and_then(|p| p.stage_1_vm_prove.as_ref())
                        .and_then(|s| s.vm_core_proof_size_kb)
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .proving_phase
                        .as_ref()
                        .and_then(|p| p.stage_2_recursive.as_ref())
                        .and_then(|s| s.compressed_proof_size_kb)
                        .map(|s| format!("{:.2}", s))
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .proving_phase
                        .as_ref()
                        .and_then(|p| p.stage_4_snark.as_ref())
                        .and_then(|s| s.groth16_proof_size_bytes)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .verification_phase
                        .as_ref()
                        .map(|v| format!("{:.3}", v.verification_time_ms))
                        .unwrap_or_else(|| "N/A".to_string()),
                    metrics
                        .proving_phase
                        .as_ref()
                        .and_then(|p| p.performance_metrics.khz)
                        .map(|k| format!("{:.3}", k))
                        .unwrap_or_else(|| "N/A".to_string()),
                ));
            }
        }

        md.push_str("\n## Detailed Results by zkVM\n\n");

        // Get grouped results for statistics
        let grouped = self.group_results();

        for (zkvm_name, results) in by_zkvm.iter() {
            md.push_str(&format!("### {}\n\n", zkvm_name));

            for result in results {
                // Check if this test has multiple runs for statistics
                let key = (
                    result.test_run.zkvm_name.clone(),
                    result.test_run.mode.clone(),
                    result.test_run.scale,
                );
                let group = grouped.get(&key);
                let has_multi_runs = group.map(|g| g.len() > 1).unwrap_or(false);
                let is_first_in_group = group
                    .map(|g| {
                        g.first()
                            .map(|r| r.test_run.repeat == result.test_run.repeat)
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);
                if let Some(metrics) = &result.metrics {
                    // Show header with repeat info if multiple runs
                    if has_multi_runs {
                        md.push_str(&format!(
                            "#### {} - Scale: {} (Run {}/{})\n\n",
                            result.test_run.mode,
                            result.test_run.scale,
                            result.test_run.repeat,
                            group.unwrap().len()
                        ));

                        // Show statistics summary only for the first run in the group
                        if is_first_in_group {
                            if let Some(stats) = self.calculate_prove_time_stats(group.unwrap()) {
                                md.push_str("**📊 Statistical Summary (across all runs):**\n");
                                md.push_str(&format!("- Runs: {}\n", stats.count));
                                md.push_str(&format!(
                                    "- Mean: {:.3}s ± {:.3}s (stddev)\n",
                                    stats.mean, stats.stddev
                                ));
                                md.push_str(&format!("- Median: {:.3}s\n", stats.median));
                                md.push_str(&format!(
                                    "- Min: {:.3}s, Max: {:.3}s\n",
                                    stats.min, stats.max
                                ));
                                md.push_str(&format!(
                                    "- 95% CI: [{:.3}s, {:.3}s]\n",
                                    stats.ci_95_lower, stats.ci_95_upper
                                ));
                                md.push_str(&format!(
                                    "- P50: {:.3}s, P90: {:.3}s, P95: {:.3}s, P99: {:.3}s\n",
                                    stats.p50, stats.p90, stats.p95, stats.p99
                                ));
                                if !stats.outliers.is_empty() {
                                    md.push_str(&format!(
                                        "- ⚠️ Outliers detected: {} runs\n",
                                        stats.outliers.len()
                                    ));
                                }
                                md.push_str("\n");
                            }
                        }
                    } else {
                        md.push_str(&format!(
                            "#### {} - Scale: {}\n\n",
                            result.test_run.mode, result.test_run.scale
                        ));
                    }

                    if let Some(exec) = &metrics.execution_phase {
                        md.push_str("**Execution:**\n");
                        if let Some(cycles) = exec.total_cycles {
                            md.push_str(&format!("- Total cycles: {}\n", cycles));
                        }
                        if let Some(time) = exec.execution_time_s {
                            md.push_str(&format!("- Execution time: {:.3}s\n", time));
                        }
                        md.push_str("\n");
                    }

                    if let Some(proving) = &metrics.proving_phase {
                        md.push_str("**Proving:**\n");
                        md.push_str(&format!(
                            "- Total prove time: {:.3}s\n",
                            proving.total_prove_time_s
                        ));

                        if let Some(stage1) = &proving.stage_1_vm_prove {
                            md.push_str(&format!(
                                "- VM prove time: {:.3}s\n",
                                stage1.vm_prove_time_s
                            ));
                            if let Some(size) = stage1.vm_core_proof_size_kb {
                                md.push_str(&format!("- VM proof size: {:.2} KB\n", size));
                            }
                        }

                        if let Some(stage2) = &proving.stage_2_recursive {
                            md.push_str(&format!(
                                "- Recursive time: {:.3}s\n",
                                stage2.recursive_prove_time_s
                            ));
                            if let Some(size) = stage2.compressed_proof_size_kb {
                                md.push_str(&format!("- Compressed size: {:.2} KB\n", size));
                            }
                        }

                        if let Some(stage4) = &proving.stage_4_snark {
                            md.push_str(&format!(
                                "- SNARK time: {:.3}s\n",
                                stage4.snark_prove_time_s
                            ));
                            if let Some(size) = stage4.groth16_proof_size_bytes {
                                md.push_str(&format!("- Groth16 size: {} bytes\n", size));
                            }
                        }

                        md.push_str("\n");
                    }

                    if let Some(verify) = &metrics.verification_phase {
                        md.push_str("**Verification:**\n");
                        md.push_str(&format!(
                            "- Verification time: {:.6}s ({:.3}ms)\n\n",
                            verify.verification_time_s, verify.verification_time_ms
                        ));
                    }

                    // Add resource usage if available
                    if let Some(ref resource_stats) = result.resource_stats {
                        md.push_str("**Resource Usage:**\n");
                        md.push_str(&format!(
                            "- Peak Memory: {:.1} MB\n",
                            resource_stats.peak_memory_mb
                        ));
                        md.push_str(&format!(
                            "- Avg Memory: {:.1} MB\n",
                            resource_stats.avg_memory_mb
                        ));
                        md.push_str(&format!(
                            "- Peak CPU: {:.1}%\n",
                            resource_stats.peak_cpu_percent
                        ));
                        md.push_str(&format!(
                            "- Avg CPU: {:.1}%\n",
                            resource_stats.avg_cpu_percent
                        ));
                        md.push_str(&format!(
                            "- Disk Read: {:.2} MB\n",
                            resource_stats.total_disk_read_mb
                        ));
                        md.push_str(&format!(
                            "- Disk Write: {:.2} MB\n",
                            resource_stats.total_disk_write_mb
                        ));
                        md.push_str(&format!(
                            "- Monitoring Duration: {:.1}s\n",
                            resource_stats.duration_s
                        ));
                        md.push_str(&format!(
                            "- Samples Collected: {}\n\n",
                            resource_stats.samples.len()
                        ));
                    }
                }
            }
        }

        fs::write(path, md)?;
        Ok(())
    }

    /// Print summary table to console
    pub fn print_summary_table(&self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        // Header
        table.set_header(vec![
            Cell::new("zkVM").add_attribute(Attribute::Bold),
            Cell::new("Mode").add_attribute(Attribute::Bold),
            Cell::new("Scale").add_attribute(Attribute::Bold),
            Cell::new("Prove (s)").add_attribute(Attribute::Bold),
            Cell::new("VM Proof").add_attribute(Attribute::Bold),
            Cell::new("Final Proof").add_attribute(Attribute::Bold),
            Cell::new("kHz").add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
        ]);

        // Data rows
        for result in &self.results {
            if let Some(metrics) = &result.metrics {
                let status_cell = if result.success {
                    Cell::new("✓").fg(Color::Green)
                } else {
                    Cell::new("✗").fg(Color::Red)
                };

                table.add_row(vec![
                    Cell::new(&metrics.metadata.zkvm_name),
                    Cell::new(&result.test_run.mode),
                    Cell::new(result.test_run.scale),
                    Cell::new(
                        metrics
                            .proving_phase
                            .as_ref()
                            .map(|p| format!("{:.2}", p.total_prove_time_s))
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    Cell::new(
                        metrics
                            .proving_phase
                            .as_ref()
                            .and_then(|p| p.stage_1_vm_prove.as_ref())
                            .and_then(|s| s.vm_core_proof_size_kb)
                            .map(|s| format!("{:.1}KB", s))
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    Cell::new(
                        metrics
                            .proving_phase
                            .as_ref()
                            .map(|p| format!("{}B", p.proof_size_evolution.final_proof_size_bytes))
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    Cell::new(
                        metrics
                            .proving_phase
                            .as_ref()
                            .and_then(|p| p.performance_metrics.khz)
                            .map(|k| format!("{:.2}", k))
                            .unwrap_or_else(|| "N/A".to_string()),
                    ),
                    status_cell,
                ]);
            } else {
                table.add_row(vec![
                    Cell::new(&result.test_run.zkvm_name),
                    Cell::new(&result.test_run.mode),
                    Cell::new(result.test_run.scale),
                    Cell::new("N/A"),
                    Cell::new("N/A"),
                    Cell::new("N/A"),
                    Cell::new("N/A"),
                    Cell::new("✗").fg(Color::Red),
                ]);
            }
        }

        println!("\n{}", table);
    }

    fn create_summary(&self) -> serde_json::Value {
        use serde_json::json;

        let total = self.results.len();
        let successful = self.results.iter().filter(|r| r.success).count();
        let failed = total - successful;

        let metrics_list: Vec<_> = self
            .results
            .iter()
            .filter_map(|r| r.metrics.as_ref())
            .collect();

        json!({
            "summary": {
                "total_runs": total,
                "successful": successful,
                "failed": failed,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            },
            "metrics": metrics_list,
        })
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
