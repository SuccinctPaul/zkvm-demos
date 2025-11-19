//! zkVM Benchmark CLI

use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

use std::collections::HashMap;
use zkvm_benchmark_utils::{
    format_benchmark_metrics_simple, BenchmarkConfig, BenchmarkExecutor, BenchmarkReporter,
    LogParser, ReportFormat, ZkVmConfig,
};

#[derive(Parser)]
#[command(name = "zkvm-benchmark")]
#[command(about = "A unified benchmark framework for zkVM comparison", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run benchmarks
    Run {
        /// Output directory
        #[arg(short, long, default_value = "benchmark-results")]
        output: Option<PathBuf>,

        /// zkVMs to run (comma-separated). If not specified, runs all enabled zkVMs from configs/
        #[arg(long)]
        zkvms: Option<String>,

        /// Filter scales to run (comma-separated). Actual is the inputs of Fib(n)
        #[arg(long)]
        scales: Option<String>,

        /// Report formats to generate (csv,json,markdown,console). Default: all
        #[arg(long)]
        report_formats: Option<String>,
    },

    /// Analyze log file and extract metrics
    AnalyzeLog {
        /// Path to log file
        #[arg(short, long)]
        log_file: PathBuf,

        /// Path to pattern config (TOML)
        #[arg(short, long)]
        patterns: Option<PathBuf>,

        /// Output format: json, markdown, csv, or text
        #[arg(short = 'f', long, default_value = "markdown")]
        format: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Generate example configuration file
    GenerateConfig {
        /// Output path for config file
        #[arg(short, long, default_value = "benchmark-config.toml")]
        output: PathBuf,
    },

    /// Generate report from existing results
    Report {
        /// Directory containing parsed metrics
        #[arg(short, long, default_value = "benchmark-results/parsed-metrics")]
        metrics_dir: PathBuf,

        /// Output directory for reports
        #[arg(short, long, default_value = "benchmark-results/reports")]
        output: PathBuf,

        /// Report formats to generate (csv,json,markdown,console). Default: all
        #[arg(long)]
        formats: Option<String>,
    },

    /// Extract metrics from zkVM log using config patterns
    Extract {
        /// Path to log file
        #[arg(short, long)]
        log_file: PathBuf,

        /// zkVM name in config (e.g., sp1, risc0)
        #[arg(short, long)]
        zkvm: String,

        /// Path to configuration file
        #[arg(short, long, default_value = "benchmark-config.toml")]
        config: PathBuf,

        /// Output format: json, csv, or text
        #[arg(short = 'f', long, default_value = "json")]
        format: String,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            output,
            zkvms,
            scales,
            report_formats,
        } => {
            run_benchmarks(output, zkvms, scales, report_formats).await?;
        }
        Commands::AnalyzeLog {
            log_file,
            patterns,
            format,
            output,
        } => {
            analyze_log(log_file, patterns, format, output)?;
        }
        Commands::GenerateConfig { output } => {
            generate_config(output)?;
        }
        Commands::Report {
            metrics_dir,
            output,
            formats,
        } => {
            generate_report(metrics_dir, output, formats)?;
        }

        Commands::Extract {
            log_file,
            zkvm,
            config,
            format,
            output,
        } => {
            extract_metrics(log_file, zkvm, config, format, output)?;
        }
    }

    Ok(())
}

async fn run_benchmarks(
    output: Option<PathBuf>,
    zkvms_filter: Option<String>,
    scales_filter: Option<String>,
    report_formats: Option<String>,
) -> anyhow::Result<()> {
    // Determine which zkVMs to run
    let zkvm_names: Vec<String> = if let Some(zkvms) = zkvms_filter {
        zkvms.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        // Scan configs/ directory for all enabled zkVMs
        load_enabled_zkvms()?
    };

    if zkvm_names.is_empty() {
        anyhow::bail!("No zkVMs specified and no enabled zkVMs found in configs/");
    }

    info!("Running benchmarks for zkVMs: {}", zkvm_names.join(", "));

    // Load zkVM configurations
    let mut zkvms = HashMap::new();
    let mut test_scales = vec![10]; // default

    for zkvm_name in &zkvm_names {
        let mut zkvm_config = ZkVmConfig::from_name(zkvm_name)?;

        // Apply scale filter if specified
        if let Some(ref scales_str) = scales_filter {
            let selected: Vec<u32> = scales_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            zkvm_config.test_scales = Some(selected.clone());
            test_scales = selected;
        } else if let Some(ref config_scales) = zkvm_config.test_scales {
            test_scales = config_scales.clone();
        }

        zkvm_config.enabled = true; // Force enabled since we're explicitly running it
        zkvms.insert(zkvm_name.clone(), zkvm_config);
    }

    // Build BenchmarkConfig
    let output_dir = output
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "benchmark-results".to_string());

    let config = BenchmarkConfig {
        test_scales,
        zkvms,
        output_dir,
        timeout_seconds: Some(3600),
        repeat_count: Some(1),
    };

    let executor = BenchmarkExecutor::new(config.clone())?;
    let results = executor.run_all().await?;

    let total = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let failed = total - successful;

    info!(
        "Completed: {} total, {} success, {} failed",
        total, successful, failed
    );
    info!("Results: {}", executor.output_dir().display());

    // Parse report formats
    let formats = if let Some(formats_str) = report_formats {
        let parsed: Vec<ReportFormat> = formats_str
            .split(',')
            .filter_map(|s| ReportFormat::from_str(s.trim()))
            .collect();
        if parsed.is_empty() {
            None // Fall back to all formats if parsing failed
        } else {
            Some(parsed)
        }
    } else {
        None // Generate all formats
    };

    // Try to get reporting config from one of the enabled zkVMs
    // Ideally this should be a global config, but for now we use the first one found
    let reporting_config = config.zkvms.values().next().map(|c| c.reporting.clone());

    let reporter = BenchmarkReporter::new(results, reporting_config);
    // Generate reports in the reports/ subdirectory
    let reports_dir = executor.output_dir().join("reports");
    reporter.generate(&reports_dir, formats)?;

    // In CI mode, exit with non-zero if any benchmark failed
    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Load all enabled zkVMs from configs/ directory
fn load_enabled_zkvms() -> anyhow::Result<Vec<String>> {
    use std::fs;

    let configs_dir = "configs";
    if !std::path::Path::new(configs_dir).exists() {
        anyhow::bail!("configs/ directory not found");
    }

    let mut enabled_zkvms = Vec::new();

    for entry in fs::read_dir(configs_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let file_name = path.file_stem().and_then(|s| s.to_str());

            if let Some(name) = file_name {
                // Skip template
                if name == "template" {
                    continue;
                }

                // Try to load config and check if enabled
                if let Ok(config) = ZkVmConfig::from_file(&path) {
                    if config.enabled {
                        enabled_zkvms.push(name.to_string());
                    }
                }
            }
        }
    }

    Ok(enabled_zkvms)
}

fn generate_config(output_path: PathBuf) -> anyhow::Result<()> {
    let config = BenchmarkConfig::example();
    config.save_to_file(&output_path)?;
    info!("Config generated: {:?}", output_path);
    Ok(())
}

fn analyze_log(
    log_file: PathBuf,
    patterns: Option<PathBuf>,
    format: String,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    use std::fs;
    use zkvm_benchmark_utils::core::config::ParsedMetrics;
    use zkvm_benchmark_utils::{BenchmarkReporter, ExecutionResult, LogParser, TestRun};

    // Load patterns
    let metrics_config = if let Some(pattern_file) = patterns {
        let content = fs::read_to_string(&pattern_file)?;
        // Try to parse as ParsedMetrics directly first
        if let Ok(m) = toml::from_str::<ParsedMetrics>(&content) {
            m
        } else {
            // Try to parse as ZkVmConfig (e.g. sp1.toml) and extract parsed_metrics
            match toml::from_str::<ZkVmConfig>(&content) {
                Ok(config) => config.parsed_metrics,
                Err(_) => {
                    // Try to parse as full BenchmarkConfig and take the first zkVM's metrics?
                    // Or just fail with a helpful message.
                    // Let's try to support a simple file with [parsed_metrics] section too
                    #[derive(serde::Deserialize)]
                    struct ConfigWrapper {
                        parsed_metrics: ParsedMetrics,
                    }
                    if let Ok(wrapper) = toml::from_str::<ConfigWrapper>(&content) {
                        wrapper.parsed_metrics
                    } else {
                        anyhow::bail!("Failed to parse patterns file. Expected ParsedMetrics, ZkVmConfig, or a file with [parsed_metrics] section.");
                    }
                }
            }
        }
    } else {
        // Default generic patterns for fallback
        let mut p = HashMap::new();
        p.insert(
            "total_cycles".to_string(),
            r"BENCHMARK: total_cycles=(\d+)".to_string(),
        );
        p.insert(
            "execution_time_s".to_string(),
            r"BENCHMARK: execution_time_s=([\d.]+)".to_string(),
        );
        p.insert(
            "total_prove_time_s".to_string(),
            r"BENCHMARK: total_prove_time_s=([\d.]+)".to_string(),
        );
        ParsedMetrics { patterns: p }
    };

    // Create parser
    let parser = LogParser::new(&metrics_config)?;
    let log_content = fs::read_to_string(&log_file)?;

    let program_name = log_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Try to extract zkvm name from log content or filename, otherwise "unknown"
    let zkvm_name = if log_content.contains("sp1") {
        "sp1"
    } else if log_content.contains("risc0") {
        "risc0"
    } else {
        "unknown"
    };

    let metrics = parser.parse(&log_content, zkvm_name, &program_name, None)?;

    // If format is markdown or we want to use BenchmarkReporter capabilities
    if format == "markdown" || format == "md" {
        // Wrap in ExecutionResult for BenchmarkReporter
        let result = ExecutionResult {
            test_run: TestRun {
                zkvm_name: metrics.metadata.zkvm_name.clone(),
                mode: "unknown".to_string(), // Could try to infer
                scale: 0,
                repeat: 1,
            },
            metrics: Some(metrics),
            log_content: String::new(),
            success: true,
            error: None,
            resource_stats: None,
        };

        let reporter = BenchmarkReporter::new(vec![result], None);

        if let Some(output_file) = output {
            reporter.generate_markdown(&output_file)?;
            info!("Saved: {:?}", output_file);
        } else {
            // Printing markdown to stdout is a bit weird with BenchmarkReporter which writes to file
            // We can use a temp file or just print a summary table
            reporter.print_summary_table();
        }
    } else {
        // Use simple formatter for json, csv, text
        let output_str = format_benchmark_metrics_simple(&metrics, &format)?;

        // Write output
        if let Some(output_file) = output {
            fs::write(&output_file, output_str)?;
            info!("Saved: {:?}", output_file);
        } else {
            println!("{}", output_str);
        }
    }

    Ok(())
}
fn generate_report(
    metrics_dir: PathBuf,
    output_dir: PathBuf,
    formats: Option<String>,
) -> anyhow::Result<()> {
    // Read all JSON files from metrics directory
    use regex::Regex;
    use std::fs;
    use zkvm_benchmark_utils::{BenchmarkMetrics, ExecutionResult, TestRun};

    let mut results = Vec::new();

    // Regex for new filename format: {zkvm}-{mode}-{timestamp}-scale{scale}
    // Example: sp1-groth16-20251119-204348-scale10
    let re_new = Regex::new(r"^([^-]+)-([^-]+)-(\d{8}-\d{6})-scale(\d+)$").unwrap();

    for entry in fs::read_dir(&metrics_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path)?;
            if let Ok(metrics) = serde_json::from_str::<BenchmarkMetrics>(&content) {
                // Extract info from filename
                let filename = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let test_run = if let Some(caps) = re_new.captures(filename) {
                    // New format
                    Some(TestRun {
                        zkvm_name: caps[1].to_string(),
                        mode: caps[2].to_string(),
                        scale: caps[4].parse().unwrap_or(0),
                        repeat: 1, // Default to 1 as it's not in filename
                    })
                } else {
                    // Try old format: sp1_fib10_groth16_1.json
                    let parts: Vec<&str> = filename.split('_').collect();
                    if parts.len() >= 4 {
                        let scale_str = parts[1].trim_start_matches("fib");
                        Some(TestRun {
                            zkvm_name: parts[0].to_string(),
                            mode: parts[2].to_string(),
                            scale: scale_str.parse().unwrap_or(0),
                            repeat: parts.get(3).unwrap_or(&"1").parse().unwrap_or(1),
                        })
                    } else {
                        None
                    }
                };

                if let Some(test_run) = test_run {
                    results.push(ExecutionResult {
                        test_run,
                        metrics: Some(metrics),
                        log_content: String::new(),
                        success: true,
                        error: None,
                        resource_stats: None, // Not available when loading from existing metrics
                    });
                }
            }
        }
    }

    if results.is_empty() {
        anyhow::bail!("No metrics found in {:?}", metrics_dir);
    }

    // Parse report formats
    let report_formats = if let Some(formats_str) = formats {
        let parsed: Vec<ReportFormat> = formats_str
            .split(',')
            .filter_map(|s| ReportFormat::from_str(s.trim()))
            .collect();
        if parsed.is_empty() {
            None
        } else {
            Some(parsed)
        }
    } else {
        None
    };

    fs::create_dir_all(&output_dir)?;
    let reporter = BenchmarkReporter::new(results, None);
    reporter.generate(&output_dir, report_formats)?;

    info!("Reports: {:?}", output_dir);
    Ok(())
}

fn extract_metrics(
    log_file: PathBuf,
    zkvm_name: String,
    config_path: PathBuf,
    format: String,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    use std::fs;

    // Load config
    let config = BenchmarkConfig::from_file(&config_path)?;

    // Get zkVM config
    let zkvm_config = config
        .zkvms
        .get(&zkvm_name)
        .ok_or_else(|| anyhow::anyhow!("zkVM '{}' not found in config", zkvm_name))?;

    // Load log file
    let log_content = fs::read_to_string(&log_file)?;

    // Create parser and extract metrics
    let parser = LogParser::new(&zkvm_config.parsed_metrics)?;
    let program_name = log_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    let metrics = parser.parse(&log_content, &zkvm_name, program_name, Some(&zkvm_config.metric_mapping))?;

    // Format output using module formatter
    let output_str = format_benchmark_metrics_simple(&metrics, &format)?;

    // Write output
    if let Some(output_file) = output {
        fs::write(&output_file, output_str)?;
        info!("Saved: {}", output_file.display());
    } else {
        println!("{}", output_str);
    }

    Ok(())
}
