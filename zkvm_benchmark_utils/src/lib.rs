//! zkVM Benchmark Framework
//!
//! A unified benchmark framework for comparing multiple zkVM implementations.
//!
//! ## Architecture
//!
//! The framework is built on a pluggable module system:
//! - **Plugin System**: Core trait-based architecture for extensibility
//! - **Analyzers**: Parse and extract metrics from logs
//! - **Formatters**: Format output in different formats (JSON, Markdown, CSV, Text)
//! - **Executors**: Run benchmarks on different zkVMs
//! - **Reporters**: Generate comprehensive reports

pub mod command_parser;
pub mod config;
pub mod config_builder;
pub mod error;
pub mod executor;
pub mod hardware;
pub mod log_parser;
pub mod metrics;
pub mod proof_size;
pub mod reporter;
pub mod resource_monitor;
pub mod statistics;

// Phase 2 优化模块
pub mod optimized_log_parser;
pub mod parallel_executor;
pub mod progress_tracker;

// Plugin system
pub mod analyzers;
pub mod formatters;
pub mod plugin;
pub mod plugin_registry;

pub use command_parser::{CommandParser, ParsedCommand};
pub use config::{BenchmarkConfig, ZkVmConfig};
pub use config_builder::{BenchmarkConfigBuilder, ZkVmConfigBuilder};
pub use error::{BenchmarkError, Result};
pub use executor::BenchmarkExecutor;
pub use metrics::{BenchmarkMetrics, DataSource, ProofMode};
pub use reporter::{BenchmarkReporter, ReportFormat};
pub use resource_monitor::{ResourceMonitor, ResourceSample, ResourceStats};
pub use statistics::{calculate_statistics, detect_regression, RegressionAnalysis, Statistics};

// Phase 2 exports
pub use optimized_log_parser::OptimizedLogParser;
pub use parallel_executor::{ParallelBenchmarkExecutor, ParallelExecutionConfig};
pub use progress_tracker::{ProgressStats, ProgressTracker, SimpleProgressTracker};

// Plugin system exports
pub use analyzers::{LogAnalyzer, PatternConfig};
pub use formatters::{format_benchmark_metrics_simple, Formatter};
pub use hardware::{collect_hardware_info, format_hardware_info};
pub use log_parser::LogParser;
pub use plugin::{
    AnalyzerPlugin, ExecutorPlugin, Plugin, PluginType, ReporterPlugin, TransformerPlugin,
};
pub use plugin_registry::{global_registry, PluginRegistry};
