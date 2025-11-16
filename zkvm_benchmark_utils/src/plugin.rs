//! Plugin System for zkVM Benchmark Framework
//!
//! Provides a trait-based plugin architecture for extending functionality

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub capabilities: Vec<String>,
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    #[serde(default)]
    pub options: HashMap<String, String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            options: HashMap::new(),
        }
    }
}

/// Base trait for all plugins
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Initialize the plugin with configuration
    fn initialize(&mut self, config: PluginConfig) -> Result<()>;

    /// Check if plugin is enabled
    fn is_enabled(&self) -> bool;

    /// Get plugin name
    fn name(&self) -> String {
        self.metadata().name
    }
}

/// Analyzer plugin trait for parsing and analyzing data
pub trait AnalyzerPlugin: Plugin {
    /// Analyze input data and return structured output
    fn analyze(&self, input: &AnalyzerInput) -> Result<AnalyzerOutput>;

    /// Get supported input formats
    fn supported_formats(&self) -> Vec<String>;

    /// Validate if this analyzer can handle the input
    fn can_handle(&self, input: &AnalyzerInput) -> bool;
}

/// Input for analyzer plugins
#[derive(Debug, Clone)]
pub struct AnalyzerInput {
    pub source: DataSource,
    pub format: Option<String>,
    pub options: HashMap<String, String>,
}

/// Data source types
#[derive(Debug, Clone)]
pub enum DataSource {
    File(PathBuf),
    String(String),
    Bytes(Vec<u8>),
}

/// Output from analyzer plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerOutput {
    pub success: bool,
    pub data: serde_json::Value,
    pub format: String,
    pub metadata: HashMap<String, String>,
}

/// Reporter plugin trait for generating reports
pub trait ReporterPlugin: Plugin {
    /// Generate report from data
    fn generate_report(&self, input: &ReporterInput) -> Result<ReporterOutput>;

    /// Get supported output formats
    fn supported_formats(&self) -> Vec<String>;
}

/// Input for reporter plugins
#[derive(Debug, Clone)]
pub struct ReporterInput {
    pub data: serde_json::Value,
    pub format: String,
    pub options: HashMap<String, String>,
}

/// Output from reporter plugins
#[derive(Debug, Clone)]
pub struct ReporterOutput {
    pub content: String,
    pub format: String,
    pub output_path: Option<PathBuf>,
}

/// Executor plugin trait for running benchmarks
pub trait ExecutorPlugin: Plugin {
    /// Execute a benchmark task
    fn execute(&self, input: &ExecutorInput) -> Result<ExecutorOutput>;

    /// Get supported zkVM types
    fn supported_zkvms(&self) -> Vec<String>;
}

/// Input for executor plugins
#[derive(Debug, Clone)]
pub struct ExecutorInput {
    pub zkvm: String,
    pub mode: String,
    pub scale: u32,
    pub options: HashMap<String, String>,
}

/// Output from executor plugins
#[derive(Debug, Clone)]
pub struct ExecutorOutput {
    pub success: bool,
    pub log_content: String,
    pub metrics: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Transformer plugin trait for data transformation
pub trait TransformerPlugin: Plugin {
    /// Transform data from one format to another
    fn transform(&self, input: &TransformerInput) -> Result<TransformerOutput>;

    /// Get supported transformations (from -> to)
    fn supported_transformations(&self) -> Vec<(String, String)>;
}

/// Input for transformer plugins
#[derive(Debug, Clone)]
pub struct TransformerInput {
    pub data: serde_json::Value,
    pub from_format: String,
    pub to_format: String,
    pub options: HashMap<String, String>,
}

/// Output from transformer plugins
#[derive(Debug, Clone)]
pub struct TransformerOutput {
    pub data: serde_json::Value,
    pub format: String,
}

/// Plugin types enum for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PluginType {
    Analyzer,
    Reporter,
    Executor,
    Transformer,
    Custom,
}

/// Plugin registration information
pub struct PluginRegistration {
    pub plugin_type: PluginType,
    pub metadata: PluginMetadata,
}
