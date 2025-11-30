use crate::core::metrics::{ProgramName, ProofMode, UnifiedMetrics, ZkVmName};
use crate::execution::resource_monitor::ResourceStats;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TestRun {
    pub zkvm_name: ZkVmName,
    pub program_name: ProgramName,
    pub mode: ProofMode,
    pub scale: u32,
    pub repeat: u32,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub test_run: TestRun,
    pub metrics: Option<UnifiedMetrics>,
    pub log_content: String,
    pub success: bool,
    pub error: Option<String>,
    pub resource_stats: Option<ResourceStats>,
}

#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    pub test_run: TestRun,
    pub log_path: PathBuf,
    pub resource_path: Option<PathBuf>,
}
