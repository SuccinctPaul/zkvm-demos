//! Unified metrics definitions for zkVM benchmarking
//!
//! Defines the standard metric structure that all zkVM benchmark results
//! must be mapped to. This enables cross-vm comparison and standardized reporting.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 1. Standard Key Constants (Standard Metric Protocol)
pub mod std_keys {
    // Execution
    pub const TOTAL_CYCLES: &str = "total_cycles";
    pub const INSTRUCTION_COUNT: &str = "instruction_count";
    pub const EXEC_TIME: &str = "execution_time_s";
    pub const SYSCALL_CYCLES: &str = "total_syscall_cycles";
    pub const TOUCHED_MEMORY: &str = "touched_memory_addresses";

    // VM Circuit
    pub const VM_CHUNK_COUNT: &str = "vm_chunk_count";
    pub const VM_CHUNK_SIZE_ROWS: &str = "vm_chunk_size_rows";
    pub const VM_PROVE_TIME: &str = "vm_prove_time_s";
    pub const VM_PROOF_SIZE: &str = "vm_prove_proof_size_bytes";

    // Recursion
    pub const RECURSION_LAYERS: &str = "recursion_layers";
    // pub const RECURSION_NODE_COUNT: &str = "recursion_node_count";
    // pub const RECURSION_TIME: &str = "recursion_time_s";
    pub const AGGRESIVE_PROVE_TIME_S: &str = "aggressive_prove_time_s";
    pub const AGGRESIVE_PROVE_SIZE_BYTES: &str = "aggressive_proof_size_bytes";

    // SNARK
    pub const SNARK_SETUP_TIME: &str = "snark_setup_time_s";
    pub const SNARK_WITNESS_TIME: &str = "snark_witness_time_s";
    pub const SNARK_PROVE_TIME: &str = "snark_proof_time_s";
    pub const SNARK_PROOF_SIZE: &str = "snark_proof_size_bytes";
    pub const SNARK_TYPE: &str = "snark_type";
    pub const SNARK_CONSTRAINTS: &str = "snark_constraint_count";

    // Verification
    pub const VERIFICATION_TIME: &str = "verification_time_s";
    pub const VERIFICATION_GAS: &str = "on_chain_gas_estimate";
    pub const SUCCESS_STATUS: &str = "success_status";

    // General
    pub const TOTAL_TIME: &str = "total_time_s";
    pub const TOTAL_PROVE_TIME: &str = "total_prove_time_s";
    pub const PEAK_RAM: &str = "peak_memory_mb";
    pub const VM_PROVE_KHZ: &str = "vm_prove_khz";
}

// Keep for backward compatibility or explicit usage if needed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    Instrumented,
    LogParsed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ProofMode {
    Core,
    Compressed,
    Groth16,
    Plonk,
}

impl std::fmt::Display for ProofMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofMode::Core => write!(f, "core"),
            ProofMode::Compressed => write!(f, "compressed"),
            ProofMode::Groth16 => write!(f, "groth16"),
            ProofMode::Plonk => write!(f, "plonk"),
        }
    }
}

impl std::str::FromStr for ProofMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "core" | "fast" => Ok(ProofMode::Core),  // "fast" maps to Core for Pico compatibility
            "compressed" | "aggressive" | "shrink" => Ok(ProofMode::Compressed),
            "groth16" => Ok(ProofMode::Groth16),
            "plonk" => Ok(ProofMode::Plonk),
            _ => Err(format!("Unknown proof mode: {}", s)),
        }
    }
}

impl ProofMode {
    /// Get the priority of the proof mode (higher is more advanced)
    /// Priority: Core(0) < Compressed(1) < Plonk(2) < Groth16(3)
    pub fn priority(&self) -> u8 {
        match self {
            ProofMode::Core => 0,
            ProofMode::Compressed => 1,
            ProofMode::Plonk => 2,
            ProofMode::Groth16 => 2,
        }
    }

    /// Get the highest priority mode from a list of modes
    /// Returns Core if the list is empty
    pub fn highest_from(modes: &[ProofMode]) -> ProofMode {
        modes
            .iter()
            .max_by_key(|m| m.priority())
            .cloned()
            .unwrap_or(ProofMode::Core)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ZkVmName {
    Airbender,
    Cairo,
    CairoM,
    Ceno,
    Jolt,
    Lean,
    Miden,
    Nexus,
    Novanet,
    O1vm,
    OpenVm,
    Pico,
    Powdr,
    Risc0,
    SnarkVm,
    Sp1,
    Valida,
    Zisk,
    Zkm,
    ZkWasm,
}

impl std::fmt::Display for ZkVmName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZkVmName::Airbender => write!(f, "airbender"),
            ZkVmName::Cairo => write!(f, "cairo"),
            ZkVmName::CairoM => write!(f, "cairo-m"),
            ZkVmName::Ceno => write!(f, "ceno"),
            ZkVmName::Jolt => write!(f, "jolt"),
            ZkVmName::Lean => write!(f, "lean"),
            ZkVmName::Miden => write!(f, "miden"),
            ZkVmName::Nexus => write!(f, "nexus"),
            ZkVmName::Novanet => write!(f, "novanet"),
            ZkVmName::O1vm => write!(f, "o1vm"),
            ZkVmName::OpenVm => write!(f, "openvm"),
            ZkVmName::Pico => write!(f, "pico"),
            ZkVmName::Powdr => write!(f, "powdr"),
            ZkVmName::Risc0 => write!(f, "risc0"),
            ZkVmName::SnarkVm => write!(f, "snarkvm"),
            ZkVmName::Sp1 => write!(f, "sp1"),
            ZkVmName::Valida => write!(f, "valida"),
            ZkVmName::Zisk => write!(f, "zisk"),
            ZkVmName::Zkm => write!(f, "zkm"),
            ZkVmName::ZkWasm => write!(f, "zkwasm"),
        }
    }
}

impl std::str::FromStr for ZkVmName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "airbender" => Ok(ZkVmName::Airbender),
            "cairo" => Ok(ZkVmName::Cairo),
            "cairo-m" | "cairom" | "cairo_m" => Ok(ZkVmName::CairoM),
            "ceno" => Ok(ZkVmName::Ceno),
            "jolt" => Ok(ZkVmName::Jolt),
            "lean" => Ok(ZkVmName::Lean),
            "miden" => Ok(ZkVmName::Miden),
            "nexus" => Ok(ZkVmName::Nexus),
            "novanet" => Ok(ZkVmName::Novanet),
            "o1vm" => Ok(ZkVmName::O1vm),
            "openvm" | "open_vm" => Ok(ZkVmName::OpenVm),
            "pico" => Ok(ZkVmName::Pico),
            "powdr" => Ok(ZkVmName::Powdr),
            "risc0" => Ok(ZkVmName::Risc0),
            "snarkvm" | "snark_vm" => Ok(ZkVmName::SnarkVm),
            "sp1" => Ok(ZkVmName::Sp1),
            "valida" => Ok(ZkVmName::Valida),
            "zisk" => Ok(ZkVmName::Zisk),
            "zkm" => Ok(ZkVmName::Zkm),
            "zkwasm" | "zk_wasm" => Ok(ZkVmName::ZkWasm),
            _ => Err(format!("Unknown zkVM name: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ProgramName {
    Fibonacci,
    Sum,
    Factorial,
    IsPrime,
    PopCount,
    Hash,
    Sha256, // Alias for Hash
    Signature,
    #[serde(untagged)]
    Custom(String),
}

impl std::fmt::Display for ProgramName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramName::Fibonacci => write!(f, "fibonacci"),
            ProgramName::Sum => write!(f, "sum"),
            ProgramName::Factorial => write!(f, "factorial"),
            ProgramName::IsPrime => write!(f, "isprime"),
            ProgramName::PopCount => write!(f, "popcount"),
            ProgramName::Hash => write!(f, "hash"),
            ProgramName::Sha256 => write!(f, "sha256"),
            ProgramName::Signature => write!(f, "signature"),
            ProgramName::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::str::FromStr for ProgramName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "fibonacci" | "fib" => Ok(ProgramName::Fibonacci),
            "sum" => Ok(ProgramName::Sum),
            "factorial" | "fact" => Ok(ProgramName::Factorial),
            "isprime" | "prime" => Ok(ProgramName::IsPrime),
            "popcount" | "bitcount" => Ok(ProgramName::PopCount),
            "hash" | "sha256" => Ok(ProgramName::Hash),
            "signature" | "sig" | "ecdsa" => Ok(ProgramName::Signature),
            _ => Ok(ProgramName::Custom(s.to_string())),
        }
    }
}

impl ProgramName {
    /// Get the program ID as used in the common library
    pub fn program_id(&self) -> u32 {
        match self {
            ProgramName::Fibonacci => 0,
            ProgramName::Sum => 1,
            ProgramName::Factorial => 2,
            ProgramName::IsPrime => 3,
            ProgramName::PopCount => 4,
            ProgramName::Hash | ProgramName::Sha256 => 5,
            ProgramName::Signature => 6,
            ProgramName::Custom(_) => 0, // Default to Fibonacci for custom programs
        }
    }
}

/// Unified metrics for a single zkVM run.
/// This struct is the "lingua franca" of the benchmark framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMetrics {
    /// Metadata about the run
    pub metadata: Metadata,

    /// Raw metrics parsed directly from logs (for extensibility)
    #[serde(default)]
    pub custom_metrics: HashMap<String, String>,

    /// Phase 1: Execution (Guest Program Running)
    pub execution: ExecutionMetrics,

    /// Phase 2: VM Circuit Proving (Trace generation & Base proving)
    pub vm_circuit: VmCircuitMetrics,

    /// Phase 3: Recursive Proving (Aggregation & Compression)
    pub aggressive: AggressiveMetrics,

    /// Phase 4: Final SNARK Proving (Groth16/Plonk wrapping)
    pub snark: SnarkMetrics,

    /// Verification Phase
    pub verification: VerificationMetrics,

    /// Resource Usage (Memory, CPU)
    pub resources: ResourceMetrics,

    /// End-to-end summary
    pub summary: Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub zkvm_name: ZkVmName,
    pub zkvm_version: Option<String>,
    pub program_name: ProgramName,
    pub mode: Option<ProofMode>,
    pub scale: Option<u32>,
    pub timestamp: DateTime<Utc>,
    pub platform: Option<String>,
    pub hardware: Option<HardwareInfo>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            zkvm_name: ZkVmName::Sp1, // Default to Sp1 as fallback
            zkvm_version: None,
            program_name: ProgramName::Fibonacci, // Default to Fibonacci
            mode: None,
            scale: None,
            timestamp: Utc::now(),
            platform: None,
            hardware: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Execution Phase: The "VM" part.
/// Metrics related to running the guest program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionMetrics {
    /// Total cycles consumed by the execution
    pub total_cycles: Option<u64>,
    /// Total number of instructions executed
    pub instruction_count: Option<u64>,
    /// Time taken for execution (witness generation)
    pub duration_s: Option<f64>,
    /// Cycles per second during execution
    pub cycles_per_sec: Option<f64>,
    /// Total syscall cycles
    pub total_syscall_cycles: Option<u64>,
    /// Number of touched memory addresses
    pub touched_memory_addresses: Option<u64>,
}

/// VM Circuit Phase: The "Trace" part.
/// Metrics related to generating the initial execution trace and base proofs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VmCircuitMetrics {
    /// Number of chunks/shards/segments the execution was split into.
    /// This represents parallelism potential.
    pub chunk_count: Option<u64>,

    /// Size of each chunk (e.g., trace rows per shard).
    /// Represents the "granularity" of the proof.
    pub chunk_size_rows: Option<u64>,

    /// Size of the core proof in bytes
    pub proof_size_bytes: Option<u64>,

    /// Time taken for this specific phase
    pub duration_s: Option<f64>,

    /// Throughput for this phase (cycles / duration)
    pub proving_khz: Option<f64>,
}

/// Aggressive Phase: The "Compression" part.
/// Metrics related to aggregating multiple chunk proofs into fewer proofs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AggressiveMetrics {
    /// Number of recursion layers (depth of the tree).
    pub recursion_layers: Option<u32>,

    /// Size of the recursive/compressed proof in bytes
    pub proof_size_bytes: Option<u64>,

    /// Time taken for recursion/aggregation
    pub duration_s: Option<f64>,

    /// Compression ratio achieved in this phase
    pub compression_ratio: Option<f64>,
}

/// SNARK Phase: The "Wrapping" part.
/// Metrics related to generating the final constant-size proof (e.g., Groth16).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnarkMetrics {
    /// The protocol used (e.g., "Groth16", "Plonk")
    pub protocol: Option<String>,

    /// Setup time (CRS generation, loading keys)
    pub setup_time_s: Option<f64>,

    /// Witness generation time for the SNARK circuit
    pub witness_gen_time_s: Option<f64>,

    /// Core proof generation time
    pub proof_gen_time_s: Option<f64>,

    /// Final proof size in bytes (on-chain size)
    pub proof_size_bytes: Option<u64>,

    /// Number of constraints in the SNARK circuit
    pub constraint_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerificationMetrics {
    pub duration_s: Option<f64>,
    pub gas_cost: Option<u64>,
    pub result: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceMetrics {
    pub peak_memory_mb: Option<f64>,
    pub avg_cpu_usage_percent: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Summary {
    pub total_time_s: f64,
    pub end_to_end_khz: Option<f64>,
    pub success: bool,
}

impl UnifiedMetrics {
    pub fn new(program_name: ProgramName, zkvm_name: ZkVmName) -> Self {
        Self {
            metadata: Metadata {
                program_name,
                zkvm_name,
                timestamp: Utc::now(),
                ..Default::default()
            },
            custom_metrics: HashMap::new(),
            execution: Default::default(),
            vm_circuit: Default::default(),
            aggressive: Default::default(),
            snark: Default::default(),
            verification: Default::default(),
            resources: Default::default(),
            summary: Default::default(),
        }
    }

    /// Populate struct fields from custom_metrics map using standard keys
    /// Also checks for raw keys (with "raw_" prefix) as fallback
    pub fn populate_from_custom_metrics(&mut self) {
        let get_u64 = |key: &str| -> Option<u64> {
            // Try standard key first, then raw key
            self.custom_metrics
                .get(key)
                .or_else(|| self.custom_metrics.get(&format!("raw_{}", key)))
                .and_then(|v| v.parse().ok())
        };
        let get_u32 = |key: &str| -> Option<u32> {
            self.custom_metrics
                .get(key)
                .or_else(|| self.custom_metrics.get(&format!("raw_{}", key)))
                .and_then(|v| v.parse().ok())
        };
        let get_f64 = |key: &str| -> Option<f64> {
            self.custom_metrics
                .get(key)
                .or_else(|| self.custom_metrics.get(&format!("raw_{}", key)))
                .and_then(|v| v.parse().ok())
        };
        let get_bool = |key: &str| -> Option<bool> {
            self.custom_metrics
                .get(key)
                .or_else(|| self.custom_metrics.get(&format!("raw_{}", key)))
                .and_then(|v| match v.to_lowercase().as_str() {
                    "true" | "success" | "1" => Some(true),
                    "false" | "failed" | "0" => Some(false),
                    _ => None,
                })
        };
        let get_string = |key: &str| -> Option<String> {
            self.custom_metrics
                .get(key)
                .or_else(|| self.custom_metrics.get(&format!("raw_{}", key)))
                .cloned()
        };

        // Helper to handle unit conversion (ms -> s) automatically
        // If key_s (e.g. "execution_time_s") is not found, tries key_ms (e.g. "execution_time_ms") and divides by 1000.
        let get_time_s = |key_s: &str, key_ms: &str| -> Option<f64> {
            get_f64(key_s).or_else(|| get_f64(key_ms).map(|v| v / 1000.0))
        };

        // Execution
        if let Some(v) = get_u64(std_keys::TOTAL_CYCLES) {
            self.execution.total_cycles = Some(v);
        }
        if let Some(v) = get_u64(std_keys::INSTRUCTION_COUNT) {
            self.execution.instruction_count = Some(v);
        }
        // Support execution_time_ms as fallback
        if let Some(v) = get_time_s(std_keys::EXEC_TIME, "execution_time_ms") {
            self.execution.duration_s = Some(v);
        }
        if let Some(v) = get_u64(std_keys::SYSCALL_CYCLES) {
            self.execution.total_syscall_cycles = Some(v);
        }
        if let Some(v) = get_u64(std_keys::TOUCHED_MEMORY) {
            self.execution.touched_memory_addresses = Some(v);
        }

        // VM Circuit
        if let Some(v) = get_u64(std_keys::VM_CHUNK_COUNT) {
            self.vm_circuit.chunk_count = Some(v);
        }
        if let Some(v) = get_u64(std_keys::VM_CHUNK_SIZE_ROWS) {
            self.vm_circuit.chunk_size_rows = Some(v);
        }
        if let Some(v) = get_u64(std_keys::VM_PROOF_SIZE) {
            self.vm_circuit.proof_size_bytes = Some(v);
        }
        if let Some(v) = get_f64(std_keys::VM_PROVE_TIME) {
            self.vm_circuit.duration_s = Some(v);
        }

        // Aggressive
        if let Some(v) = get_u32(std_keys::RECURSION_LAYERS) {
            self.aggressive.recursion_layers = Some(v);
        }
        if let Some(v) = get_f64(std_keys::AGGRESIVE_PROVE_TIME_S) {
            self.aggressive.duration_s = Some(v);
        }
        if let Some(v) = get_u64(std_keys::AGGRESIVE_PROVE_SIZE_BYTES) {
            self.aggressive.proof_size_bytes = Some(v);
        }

        // SNARK
        if let Some(v) = get_f64(std_keys::SNARK_SETUP_TIME) {
            self.snark.setup_time_s = Some(v);
        }
        if let Some(v) = get_f64(std_keys::SNARK_WITNESS_TIME) {
            self.snark.witness_gen_time_s = Some(v);
        }
        if let Some(v) = get_f64(std_keys::SNARK_PROVE_TIME) {
            self.snark.proof_gen_time_s = Some(v);
        }
        if let Some(v) = get_u64(std_keys::SNARK_PROOF_SIZE) {
            self.snark.proof_size_bytes = Some(v);
        }
        if let Some(v) = get_string(std_keys::SNARK_TYPE) {
            self.snark.protocol = Some(v);
        }
        if let Some(v) = get_u64(std_keys::SNARK_CONSTRAINTS) {
            self.snark.constraint_count = Some(v);
        }

        // Verification & Summary (Success Status)
        if let Some(v) = get_bool(std_keys::SUCCESS_STATUS) {
            self.verification.result = v;
            self.summary.success = v;
        }

        // Verification
        if let Some(v) = get_f64(std_keys::VERIFICATION_TIME) {
            self.verification.duration_s = Some(v);
        }
        if let Some(v) = get_u64(std_keys::VERIFICATION_GAS) {
            self.verification.gas_cost = Some(v);
        }

        // Summary
        if let Some(v) = get_f64(std_keys::TOTAL_TIME) {
            self.summary.total_time_s = v;
        } else if let Some(v) = get_f64(std_keys::TOTAL_PROVE_TIME) {
            // Fallback: use prove time as total time if total_time not available
            self.summary.total_time_s = v;
        }

        // Resources
        if let Some(v) = get_f64(std_keys::PEAK_RAM) {
            self.resources.peak_memory_mb = Some(v);
        }

        // KHZ (already calculated or provided)
        if let Some(v) = get_f64(std_keys::VM_PROVE_KHZ) {
            self.vm_circuit.proving_khz = Some(v);
        }
    }

    /// Update derived metrics based on raw values
    /// Note: This calls populate_from_custom_metrics() first, which may overwrite
    /// manually set struct fields. Use calculate_derived_only() after merging.
    pub fn update_derived_metrics(&mut self) {
        // Ensure fields are populated first
        self.populate_from_custom_metrics();
        // Then calculate derived values
        self.calculate_derived_only();
    }

    /// Calculate derived metrics (KHZ, throughput) without re-populating from custom_metrics.
    /// Use this after merging multiple metrics to avoid overwriting correctly merged fields.
    pub fn calculate_derived_only(&mut self) {
        // Calculate total_time_s if not set
        if self.summary.total_time_s == 0.0 {
            let exec_time = self.execution.duration_s.unwrap_or(0.0);
            let vm_time = self.vm_circuit.duration_s.unwrap_or(0.0);
            let rec_time = self.aggressive.duration_s.unwrap_or(0.0);
            let snark_time = self.snark.proof_gen_time_s.unwrap_or(0.0);
            let verify_time = self.verification.duration_s.unwrap_or(0.0);

            // Try to get total_prove_time_s from custom_metrics
            let prove_time: f64 = self
                .custom_metrics
                .get("total_prove_time_s")
                .or_else(|| self.custom_metrics.get("raw_total_prove_time_s"))
                .and_then(|v| v.parse().ok())
                .unwrap_or(vm_time + rec_time + snark_time);

            self.summary.total_time_s = exec_time + prove_time + verify_time;
        }

        // 1. Calculate KHZ (Execution Speed vs Proving Time)
        // If not already provided
        if self.vm_circuit.proving_khz.is_none() {
            // Try to calculate from total_prove_time_s
            if let Some(prove_time_str) = self
                .custom_metrics
                .get("total_prove_time_s")
                .or_else(|| self.custom_metrics.get("raw_total_prove_time_s"))
            {
                if let (Some(cycles), Ok(prove_time)) =
                    (self.execution.total_cycles, prove_time_str.parse::<f64>())
                {
                    if prove_time > 0.0 {
                        self.vm_circuit.proving_khz = Some((cycles as f64) / (prove_time * 1000.0));
                    }
                }
            } else if let (Some(cycles), Some(time)) =
                (self.execution.total_cycles, self.vm_circuit.duration_s)
            {
                if time > 0.0 {
                    self.vm_circuit.proving_khz = Some((cycles as f64) / (time * 1000.0));
                }
            }
        }

        // 2. Calculate End-to-End Throughput
        if let Some(cycles) = self.execution.total_cycles {
            if self.summary.total_time_s > 0.0 {
                self.summary.end_to_end_khz =
                    Some((cycles as f64) / (self.summary.total_time_s * 1000.0));
            }
        }
    }

    /// Helper to access flattened string values for reporting
    pub fn get_value(&self, key: &str) -> String {
        // Check custom metrics first
        if let Some(val) = self.custom_metrics.get(key) {
            return val.clone();
        }

        match key {
            "total_cycles" => self
                .execution
                .total_cycles
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "execution_time_s" => self
                .execution
                .duration_s
                .map(|v| format!("{:.3}", v))
                .unwrap_or("N/A".to_string()),
            "vm_chunk_count" => self
                .vm_circuit
                .chunk_count
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "aggressive_layers" => self
                .aggressive
                .recursion_layers
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "proof_size_bytes" => self
                .snark
                .proof_size_bytes
                .map(|v| v.to_string())
                .unwrap_or("N/A".to_string()),
            "total_time_s" => format!("{:.3}", self.summary.total_time_s),
            _ => "N/A".to_string(),
        }
    }
}
