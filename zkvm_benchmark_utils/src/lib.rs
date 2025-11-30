//! zkVM Benchmark Framework
//!
//! A unified benchmark framework for comparing multiple zkVM implementations.
//!
//! ## Architecture
//!
//! The framework is built on a pluggable module system:
//! - **Core**: Configuration, Errors, Metrics
//! - **Execution**: Executor, Command Parsing, Resource Monitoring
//! - **Analysis**: Log Parsing, Analyzers
//! - **Reporting**: Reporters, Formatters, Statistics
//! - **System**: Hardware

pub mod analysis;
pub mod core;
pub mod execution;
pub mod reporting;
pub mod system;

// Re-exports for convenience
pub use core::config::{BenchmarkConfig, ParsedMetrics, ProgramConfig, ZkVmConfig};
pub use core::error::{BenchmarkError, Result};
// Re-export UnifiedMetrics instead of BenchmarkMetrics
pub use core::metrics::{ProgramName, UnifiedMetrics};

pub use execution::command_parser::{CommandParser, ParsedCommand};
pub use execution::executor::BenchmarkExecutor;
pub use execution::resource_monitor::{ResourceMonitor, ResourceSample, ResourceStats};
pub use execution::types::{ExecutionResult, TestRun};

pub use analysis::log_parser::LogParser;

pub use reporting::reporter::{BenchmarkReporter, ReportFormat};
pub use reporting::simple_formatter::format_benchmark_metrics_simple;
pub use reporting::statistics::{
    calculate_statistics, detect_regression, RegressionAnalysis, Statistics,
};

pub use system::hardware::{collect_hardware_info, format_hardware_info};
