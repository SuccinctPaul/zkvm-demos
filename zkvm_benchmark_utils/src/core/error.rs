//! Error types for the benchmark framework

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BenchmarkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Missing metric: {0}")]
    MissingMetric(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

pub type Result<T> = std::result::Result<T, BenchmarkError>;

impl From<serde_json::Error> for BenchmarkError {
    fn from(err: serde_json::Error) -> Self {
        BenchmarkError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for BenchmarkError {
    fn from(err: toml::de::Error) -> Self {
        BenchmarkError::Config(err.to_string())
    }
}

impl From<csv::Error> for BenchmarkError {
    fn from(err: csv::Error) -> Self {
        BenchmarkError::Serialization(err.to_string())
    }
}
