//! Benchmark metrics definitions
//!
//! Defines the complete metric structure for zkVM benchmarking,
//! following the multi-stage execution model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Complete benchmark metrics for a single zkVM run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    /// Metadata
    pub metadata: Metadata,

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
    pub fn calculate_derived_metrics(&mut self) {
        // Calculate throughput
        if let Some(exec) = &self.execution_phase {
            if let (Some(cycles), Some(time)) = (exec.total_cycles, exec.execution_time_s) {
                if time > 0.0 {
                    self.execution_phase.as_mut().unwrap().execution_throughput =
                        Some(cycles as f64 / time);
                }
            }
        }

        // Calculate proving efficiency metrics
        if let Some(proving) = &mut self.proving_phase {
            if let Some(exec) = &self.execution_phase {
                if let Some(cycles) = exec.total_cycles {
                    let prove_time = proving.total_prove_time_s;
                    if prove_time > 0.0 {
                        proving.performance_metrics.khz =
                            Some(cycles as f64 / (prove_time * 1000.0));
                        proving
                            .performance_metrics
                            .proving_throughput_kcycles_per_sec =
                            Some(cycles as f64 / (prove_time * 1000.0));
                    }
                }
            }

            // Calculate compression ratio
            if let (Some(trace_size), final_size) = (
                proving.proof_size_evolution.stage_0_proof_size_mb,
                proving.proof_size_evolution.final_proof_size_bytes,
            ) {
                if final_size > 0 {
                    let trace_bytes = trace_size * 1024.0 * 1024.0;
                    proving.proof_size_evolution.total_compression_ratio =
                        Some(trace_bytes / final_size as f64);
                }
            }
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
