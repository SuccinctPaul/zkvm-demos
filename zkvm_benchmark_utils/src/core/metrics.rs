//! Benchmark metrics definitions
//!
//! Defines the complete metric structure for zkVM benchmarking,
//! following the multi-stage execution model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

// 1. Standard Key Constants (Standard Metric Protocol)
pub mod std_keys {
    pub const TOTAL_CYCLES: &str = "total_cycles";
    pub const PROVE_TIME: &str = "total_prove_time_s";
    pub const EXEC_TIME: &str = "execution_time_s";
    pub const VERIFY_TIME: &str = "verification_time_s";
    pub const TOTAL_TIME: &str = "total_time_s";
    pub const PROOF_SIZE: &str = "final_proof_size_bytes";
    pub const PEAK_RAM: &str = "peak_memory_mb";
    pub const KHZ: &str = "khz";
    pub const THROUGHPUT: &str = "execution_throughput";
    pub const CYCLES_PER_SECOND: &str = "cycles_per_second";
    pub const COMPRESSION_RATIO: &str = "compression_ratio";
    pub const TRACE_SIZE_MB: &str = "trace_size_mb";
}

/// Complete benchmark metrics for a single zkVM run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    /// Metadata
    pub metadata: Metadata,

    /// Custom/Dynamic metrics parsed from logs
    #[serde(default)]
    pub custom_metrics: HashMap<String, String>,

    /// Execution phase metrics
    pub execution_phase: Option<ExecutionPhase>,

    /// Trace generation phase
    pub trace_generation: Option<TraceGeneration>,

    /// Proving phase (multi-stage)
    pub proving_phase: Option<ProvingPhase>,

    /// Verification phase
    pub verification_phase: Option<VerificationPhase>,

    /// Resource usage
    pub resources: Option<ResourceMetrics>,

    /// End-to-end summary
    pub summary: Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub program_name: String,
    pub zkvm_name: String,
    pub zkvm_version: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub platform: Option<String>,
    pub hardware: Option<HardwareInfo>,
}

/// Hardware information for benchmark reproducibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    // CPU information
    pub cpu_brand: String,
    pub cpu_cores: u32,
    pub cpu_physical_cores: u32,
    pub cpu_frequency_mhz: Option<u64>,

    // Memory information
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,

    // OS information
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,

    // Architecture
    pub arch: String,
}

/// Execution Phase: Program running in zkVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    // Computation metrics
    pub total_cycles: Option<u64>,
    pub total_instruction_count: Option<u64>,
    pub user_cycles: Option<u64>,
    pub syscall_cycles: Option<u64>,
    pub syscall_count: Option<u64>,
    pub memory_accesses: Option<u64>,
    pub touched_memory_addresses: Option<u64>,

    // Performance metrics
    pub execution_time_s: Option<f64>,
    pub execution_throughput: Option<f64>,

    // Segmentation (if applicable)
    pub segments: Option<u64>,
    pub segment_size: Option<u64>,
    pub max_segment_cycles: Option<u64>,
}

/// Trace Generation Phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceGeneration {
    pub trace_generation_time_s: Option<f64>,
    pub trace_rows: Option<u64>,
    pub trace_columns: Option<u64>,
    pub trace_cells: Option<u64>,
    pub trace_commitment_time_s: Option<f64>,
}

/// Proving Phase: Multi-stage proof generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvingPhase {
    pub proof_mode: ProofMode,
    pub security_bits: Option<u32>,
    pub fri_queries: Option<u32>,
    pub blowup_factor: Option<u32>,
    pub recursion_enabled: Option<bool>,

    // Overall timing
    pub total_prove_time_s: f64,
    pub setup_time_s: Option<f64>,

    // Stage 0: Execute (if measured separately)
    pub stage_0_execute: Option<Stage0Execute>,

    // Stage 1: VM Prove
    pub stage_1_vm_prove: Option<Stage1VmProve>,

    // Stage 2: Recursive Prove
    pub stage_2_recursive: Option<Stage2Recursive>,

    // Stage 3: Aggressive Compression
    pub stage_3_aggressive: Option<Stage3Aggressive>,

    // Stage 4: SNARK Wrapping
    pub stage_4_snark: Option<Stage4Snark>,

    // Time breakdown percentages
    pub time_breakdown_percent: Option<TimeBreakdown>,

    // Proof size evolution
    pub proof_size_evolution: ProofSizeEvolution,

    // Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofMode {
    Core,
    Compressed,
    Groth16,
    Plonk,
    Recursive,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage0Execute {
    pub execute_time_s: f64,
    pub execute_cycles: u64,
    pub trace_generation_time_s: Option<f64>,
    pub trace_size_mb: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage1VmProve {
    pub vm_prove_time_s: f64,
    pub vm_prove_segments: Option<u64>,
    pub vm_prove_cycles_per_segment: Option<u64>,
    pub vm_core_proof_size_kb: Option<f64>,
    pub vm_prove_memory_mb: Option<f64>,
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage2Recursive {
    pub recursive_prove_time_s: f64,
    pub recursion_layers: Option<u32>,
    pub recursive_input_size_kb: Option<f64>,
    pub recursive_output_size_kb: Option<f64>,
    pub recursive_proof_count: Option<u32>,
    pub compressed_proof_size_kb: Option<f64>,
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage3Aggressive {
    pub aggressive_prove_time_s: f64,
    pub compression_ratio: Option<f64>,
    pub aggressive_proof_size_kb: Option<f64>,
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage4Snark {
    pub snark_prove_time_s: f64,
    pub snark_setup_time_s: Option<f64>,
    pub snark_witness_time_s: Option<f64>,
    pub snark_proof_time_s: Option<f64>,
    pub groth16_proof_size_bytes: Option<u64>,
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBreakdown {
    pub execute_time_percent: Option<f32>,
    pub vm_prove_time_percent: Option<f32>,
    pub recursive_time_percent: Option<f32>,
    pub aggressive_time_percent: Option<f32>,
    pub snark_time_percent: Option<f32>,
}

/// Proof size evolution across stages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSizeEvolution {
    pub stage_0_proof_size_mb: Option<f64>,
    pub stage_1_proof_size_kb: Option<f64>,
    pub stage_2_proof_size_kb: Option<f64>,
    pub stage_3_proof_size_kb: Option<f64>,
    pub stage_4_proof_size_bytes: Option<u64>,
    pub final_proof_size_bytes: u64,
    pub total_compression_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub proving_throughput_kcycles_per_sec: Option<f64>,
    pub khz: Option<f64>,
    pub cycles_per_constraint: Option<f64>,
    pub proof_efficiency_score: Option<f64>,
}

/// Verification Phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationPhase {
    pub verification_time_s: f64,
    pub verification_time_ms: f64,
    pub on_chain_gas_estimate: Option<u64>,
}

/// Resource usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub peak_memory_mb: Option<f64>,
    pub execution_memory_mb: Option<f64>,
    pub proving_memory_mb: Option<f64>,
    pub avg_cpu_percent: Option<f32>,
    pub peak_cpu_percent: Option<f32>,
    pub cpu_cores_used: Option<u32>,
}

/// End-to-end summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub total_time_s: f64,
    pub e2e_throughput: Option<f64>,
    pub proof_compression_ratio: Option<f64>,
    pub success_status: SuccessStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SuccessStatus {
    Success,
    Failed,
    Timeout,
}

/// Data source annotation for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    Instrumented,
    LogParsed,
    Estimated,
    MeasuredCoreMode,
    MeasuredCompressedMode,
    MeasuredGroth16Mode,
}

impl BenchmarkMetrics {
    /// Create a new metrics instance with minimal required fields
    pub fn new(program_name: String, zkvm_name: String) -> Self {
        Self {
            metadata: Metadata {
                program_name,
                zkvm_name,
                zkvm_version: None,
                timestamp: Utc::now(),
                platform: None,
                hardware: None,
            },
            custom_metrics: HashMap::new(),
            execution_phase: None,
            trace_generation: None,
            proving_phase: None,
            verification_phase: None,
            resources: None,
            summary: Summary {
                total_time_s: 0.0,
                e2e_throughput: None,
                proof_compression_ratio: None,
                success_status: SuccessStatus::Success,
                error_message: None,
            },
        }
    }

    /// Calculate derived metrics based on existing data
    /// 
    /// This function serves as the "Derivation Engine" in the ETL pipeline.
    /// It uses standardized keys from `custom_metrics` to compute derived values.
    pub fn calculate_derived_metrics(&mut self) {
        use std_keys::*;

        // Helper closure to safely parse data
        let get_f64 = |key: &str| -> Option<f64> {
            self.custom_metrics.get(key).and_then(|v| v.parse::<f64>().ok())
        };
        let get_u64 = |key: &str| -> Option<u64> {
            self.custom_metrics.get(key).and_then(|v| v.parse::<u64>().ok())
        };
        let set_val = |key: &str, val: String| {
            self.custom_metrics.insert(key.to_string(), val);
        };

        // 1. Calculate Performance Metrics (KHz / Throughput)
        // Logic: Derive if cycles and prove time are available
        if let (Some(cycles), Some(time)) = (get_u64(TOTAL_CYCLES), get_f64(PROVE_TIME)) {
            if time > 0.0 {
                let khz = (cycles as f64) / (time * 1000.0);
                // Only calculate if not present (respect raw data priority)
                if !self.custom_metrics.contains_key(KHZ) {
                    set_val(KHZ, format!("{:.3}", khz));
                }
                
                // Cycles per second (Hz)
                let hz = (cycles as f64) / time;
                 if !self.custom_metrics.contains_key(CYCLES_PER_SECOND) {
                    set_val(CYCLES_PER_SECOND, format!("{:.0}", hz));
                }

                // Throughput (usually same as Hz for proving throughput)
                if !self.custom_metrics.contains_key(THROUGHPUT) {
                    set_val(THROUGHPUT, format!("{:.0}", hz));
                }
            }
        }

        // 2. Calculate Total Time
        // Logic: Sum execution, proving, and verification times if total is missing
        if !self.custom_metrics.contains_key(TOTAL_TIME) {
            let exec = get_f64(EXEC_TIME).unwrap_or(0.0);
            let prove = get_f64(PROVE_TIME).unwrap_or(0.0);
            let verify = get_f64(VERIFY_TIME).unwrap_or(0.0);
            let total = exec + prove + verify;
            if total > 0.0 {
                set_val(TOTAL_TIME, format!("{:.3}", total));
                self.summary.total_time_s = total; // Also update summary struct
            }
        }

        // 3. Calculate Compression Ratio
        if let (Some(trace_mb), Some(final_bytes)) = (get_f64(TRACE_SIZE_MB), get_f64(PROOF_SIZE)) {
            if final_bytes > 0.0 {
                let trace_bytes = trace_mb * 1024.0 * 1024.0;
                let ratio = trace_bytes / final_bytes;
                set_val(COMPRESSION_RATIO, format!("{:.2}", ratio));
            }
        }
    }

    /// Get a metric value by key string (for dynamic reporting)
    pub fn get_value(&self, key: &str) -> String {
        // First check custom metrics (highest priority for raw values)
        if let Some(val) = self.custom_metrics.get(key) {
            return val.clone();
        }

        match key {
            // Execution
            "total_cycles" => self
                .execution_phase
                .as_ref()
                .and_then(|e| e.total_cycles)
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "total_instruction_count" => self
                .execution_phase
                .as_ref()
                .and_then(|e| e.total_instruction_count)
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "execution_time_s" => self
                .execution_phase
                .as_ref()
                .and_then(|e| e.execution_time_s)
                .map(|v| format!("{:.4}", v))
                .unwrap_or("N/A".to_string()),

            // Proving
            "total_prove_time_s" => self
                .proving_phase
                .as_ref()
                .map(|p| format!("{:.3}", p.total_prove_time_s))
                .unwrap_or("N/A".to_string()),
            "vm_core_proof_size_kb" => self
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_1_proof_size_kb)
                .map(|v| format!("{:.2}", v))
                .unwrap_or("N/A".to_string()),
            "compressed_proof_size_kb" => self
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_2_proof_size_kb)
                .map(|v| format!("{:.2}", v))
                .unwrap_or("N/A".to_string()),
            "groth16_proof_size_bytes" => self
                .proving_phase
                .as_ref()
                .and_then(|p| p.proof_size_evolution.stage_4_proof_size_bytes)
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "khz" => self
                .proving_phase
                .as_ref()
                .and_then(|p| p.performance_metrics.khz)
                .map(|v| format!("{:.3}", v))
                .unwrap_or("N/A".to_string()),

            // Verification
            "verification_time_s" => self
                .verification_phase
                .as_ref()
                .map(|v| format!("{:.6}", v.verification_time_s))
                .unwrap_or("N/A".to_string()),

            // Summary
            "total_time_s" => format!("{:.3}", self.summary.total_time_s),
            "success_status" => format!("{:?}", self.summary.success_status),

            // Metadata
            "zkvm_name" => self.metadata.zkvm_name.clone(),
            "program_name" => self.metadata.program_name.clone(),

            _ => "N/A".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = BenchmarkMetrics::new("fibonacci_10".to_string(), "SP1".to_string());

        assert_eq!(metrics.metadata.program_name, "fibonacci_10");
        assert_eq!(metrics.metadata.zkvm_name, "SP1");
    }
}
