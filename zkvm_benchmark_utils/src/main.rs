//! zkVM Benchmark CLI

use clap::{Parser, Subcommand};
use log::{error, info, warn};
use std::path::PathBuf;
use std::time::Duration;
use std::fs;

use std::collections::HashMap;
use zkvm_benchmark_utils::{
    BenchmarkConfig, BenchmarkExecutor, BenchmarkReporter, ReportFormat, ZkVmConfig,
    ExecutionResult, TestRun, UnifiedMetrics,
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
    /// Run benchmarks (Full Pipeline: Execute -> Parse -> Report)
    Benchmark {
        /// Output directory (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// zkVMs to run (comma-separated). If not specified, runs all enabled zkVMs from configs/
        #[arg(long)]
        zkvms: Option<String>,

        /// Test scales/parameters (comma-separated, e.g., "10,20,100"). Overrides config values
        #[arg(long)]
        scales: Option<String>,

        /// Report formats to generate (csv,json,markdown,console). Default: all
        #[arg(long)]
        report_formats: Option<String>,
    },

    /// Legacy Run command (Alias for Benchmark)
    Run {
        /// Output directory (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// zkVMs to run (comma-separated). If not specified, runs all enabled zkVMs from configs/
        #[arg(long)]
        zkvms: Option<String>,

        /// Test scales/parameters (comma-separated, e.g., "10,20,100"). Overrides config values
        #[arg(long)]
        scales: Option<String>,

        /// Report formats to generate (csv,json,markdown,console). Default: all
        #[arg(long)]
        report_formats: Option<String>,
    },

    /// Phase 1: Execute benchmarks (Executes zkVMs and saves raw logs)
    Execute {
        /// Output directory (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// zkVMs to run (comma-separated)
        #[arg(long)]
        zkvms: Option<String>,

        /// Test scales/parameters (comma-separated)
        #[arg(long)]
        scales: Option<String>,
    },

    /// Phase 2: Parse raw logs (Parses logs from raw-logs directory -> output/parsed-metrics)
    Parse {
        /// Output directory for parsed-metrics (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Directory containing raw logs (optional, defaults to output/raw-logs)
        #[arg(long)]
        raw_logs_dir: Option<PathBuf>,
    },

    /// Phase 3: Generate reports (Generates reports from parsed-metrics directory -> output/reports)
    Report {
        /// Output directory for reports (overrides config)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Directory containing parsed metrics (optional, defaults to output/parsed-metrics)
        #[arg(long)]
        parsed_metrics_dir: Option<PathBuf>,

        /// Report formats to generate (csv,json,markdown,console). Default: all
        #[arg(long)]
        report_formats: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Benchmark {
            output,
            zkvms,
            scales,
            report_formats,
        } => {
            run_benchmarks(output, zkvms, scales, report_formats).await?;
        }
        Commands::Run {
            output,
            zkvms,
            scales,
            report_formats,
        } => {
             // Legacy support
            run_benchmarks(output, zkvms, scales, report_formats).await?;
        }
        Commands::Execute {
            output,
            zkvms,
            scales,
        } => {
            execute_benchmarks(output, zkvms, scales).await?;
        }
        Commands::Parse { output, raw_logs_dir } => {
            parse_logs(output, raw_logs_dir).await?;
        }
        Commands::Report {
            output,
            parsed_metrics_dir,
            report_formats,
        } => {
            generate_reports_only(output, parsed_metrics_dir, report_formats).await?;
        }
    }

    Ok(())
}

fn setup_benchmark_config(
    output: Option<PathBuf>,
    zkvms_filter: Option<String>,
    scales_filter: Option<String>,
) -> anyhow::Result<BenchmarkConfig> {
    // Determine which zkVMs to run
    let zkvm_names: Vec<String> = if let Some(zkvms) = zkvms_filter {
        info!("📋 Using specified zkVMs: {}", zkvms);
        zkvms.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        info!("🔍 Scanning configs/ for enabled zkVMs...");
        load_enabled_zkvms()?
    };

    if zkvm_names.is_empty() {
        anyhow::bail!("No zkVMs specified and no enabled zkVMs found in configs/");
    }

    info!("📦 Selected zkVMs: {}", zkvm_names.join(", "));

    // Parse scale filter if specified
    let scale_values: Option<Vec<u32>> = scales_filter.map(|s| {
        s.split(',')
            .filter_map(|x| x.trim().parse::<u32>().ok())
            .collect()
    });

    // Load zkVM configurations
    info!("📖 Loading zkVM configurations...");
    let mut zkvms = HashMap::new();
    let mut test_scales = vec![10]; // default

    for zkvm_name in &zkvm_names {
        info!("  📄 Loading config for: {}", zkvm_name);
        let mut zkvm_config = ZkVmConfig::from_name(zkvm_name)?;

        if let Some(ref scales) = scale_values {
            let mut programs = zkvm_config.get_programs();
            for prog in &mut programs {
                prog.scales = scales.clone();
            }
            zkvm_config.programs = Some(programs);
        }

        if let Some(ref config_scales) = zkvm_config.test_scales {
            test_scales = config_scales.clone();
        }

        info!(
            "  ✅ Config loaded: modes={:?}, programs={:?}",
            zkvm_config.prove_modes,
            zkvm_config
                .get_programs()
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.scales))
                .collect::<Vec<_>>()
        );
        zkvms.insert(zkvm_name.clone(), zkvm_config);
    }

    let output_dir = if let Some(p) = output {
        p.to_string_lossy().to_string()
    } else {
         // Load default from a config file if we had a global one, but we don't.
         // However, BenchmarkConfig has a default output_dir, but we are constructing it here.
         // Let's use the same default as BenchmarkConfig::default().
         "benchmark-results".to_string()
    };

    info!("📁 Output directory: {}", output_dir);
    info!("🔢 Test scales: {:?}", test_scales);

    Ok(BenchmarkConfig {
        test_scales,
        zkvms,
        output_dir,
        timeout_seconds: Some(3600),
        repeat_count: Some(1),
    })
}

async fn run_benchmarks(
    output: Option<PathBuf>,
    zkvms_filter: Option<String>,
    scales_filter: Option<String>,
    report_formats: Option<String>,
) -> anyhow::Result<()> {
    info!("═══════════════════════════════════════════════════════════");
    info!("🚀 zkVM Benchmark Framework - Starting Execution");
    info!("═══════════════════════════════════════════════════════════");

    let config = setup_benchmark_config(output, zkvms_filter, scales_filter)?;

    info!("═══════════════════════════════════════════════════════════");
    info!("⚙️  Initializing Benchmark Executor");
    info!("═══════════════════════════════════════════════════════════");
    let executor = BenchmarkExecutor::new(config.clone())?;

    info!("═══════════════════════════════════════════════════════════");
    info!("🏃 Starting Benchmark Execution (max 2 hours)");
    info!("═══════════════════════════════════════════════════════════");

    let start_time = std::time::Instant::now();
    let results = tokio::time::timeout(
        Duration::from_secs(7200),
        executor.run_all(),
    )
    .await;

    let results = match results {
        Ok(Ok(r)) => {
            let elapsed = start_time.elapsed();
            info!("⏱️  Total execution time: {:.2} seconds", elapsed.as_secs_f64());
            r
        }
        Ok(Err(e)) => {
            error!("❌ Benchmark execution failed: {}", e);
            std::process::exit(1);
        }
        Err(_) => {
            error!("⏰ Benchmark execution timed out after 2 hours");
            std::process::exit(1);
        }
    };

    let total = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let failed = total - successful;

    let paths = config.get_paths();

    info!("═══════════════════════════════════════════════════════════");
    info!("📊 Execution Summary");
    info!("═══════════════════════════════════════════════════════════");
    info!("  Total runs: {}", total);
    info!("  ✅ Successful: {}", successful);
    if failed > 0 {
        warn!("  ❌ Failed: {}", failed);
    }
    info!("  📁 Results directory: {}", executor.output_dir().display());

    info!("═══════════════════════════════════════════════════════════");
    info!("📝 Generating Reports");
    info!("═══════════════════════════════════════════════════════════");

    let formats = if let Some(formats_str) = report_formats {
        info!("📋 Requested formats: {}", formats_str);
        let parsed: Vec<ReportFormat> = formats_str
            .split(',')
            .filter_map(|s| ReportFormat::from_str(s.trim()))
            .collect();
        if parsed.is_empty() {
            warn!("⚠️  No valid formats parsed, using all formats");
            None
        } else {
            info!("✅ Parsed {} format(s)", parsed.len());
            Some(parsed)
        }
    } else {
        info!("📋 No format specified, generating all formats");
        None
    };

    let reporter = BenchmarkReporter::new(results);
    let reports_dir = paths.reports;
    info!("📁 Reports directory: {}", reports_dir.display());

    match reporter.generate(&reports_dir, formats) {
        Ok(_) => {
            info!("✅ Reports generated successfully");
            info!("═══════════════════════════════════════════════════════════");
        }
        Err(e) => {
            error!("❌ Failed to generate reports: {}", e);
            error!("═══════════════════════════════════════════════════════════");
        }
    }

    if failed > 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}

async fn execute_benchmarks(
    output: Option<PathBuf>,
    zkvms_filter: Option<String>,
    scales_filter: Option<String>,
) -> anyhow::Result<()> {
    info!("═══════════════════════════════════════════════════════════");
    info!("🚀 zkVM Benchmark - Phase 1: Execution Only");
    info!("═══════════════════════════════════════════════════════════");

    let config = setup_benchmark_config(output, zkvms_filter, scales_filter)?;
    let paths = config.get_paths();
    let executor = BenchmarkExecutor::new(config)?;

    let start_time = std::time::Instant::now();
    let results = tokio::time::timeout(
        Duration::from_secs(7200),
        executor.execute_all_only(),
    )
    .await;

    match results {
        Ok(Ok(r)) => {
            info!("✅ Execution completed. Logs saved to: {}", paths.raw_logs.display());
             info!("⏱️  Total time: {:.2} seconds", start_time.elapsed().as_secs_f64());
             info!("Total tasks executed: {}", r.len());
        }
        Ok(Err(e)) => {
            error!("❌ Execution failed: {}", e);
            std::process::exit(1);
        }
        Err(_) => {
            error!("⏰ Execution timed out");
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn parse_logs(output: Option<PathBuf>, raw_logs_dir: Option<PathBuf>) -> anyhow::Result<()> {
     info!("═══════════════════════════════════════════════════════════");
    info!("🚀 zkVM Benchmark - Phase 2: Log Parsing");
    info!("═══════════════════════════════════════════════════════════");

    // We need to load config to get regex patterns.
    // We assume the configs in 'configs/' are valid.
    // We don't filter zkvms here, we just load all enabled ones to get their regexes.
    // If a log belongs to a disabled zkvm, we might miss it if we filter.
    // So we load all enabled zkvms.
    
    let config = setup_benchmark_config(output, None, None)?;
    let paths = config.get_paths();
    let executor = BenchmarkExecutor::new(config)?;

    match executor.parse_all_logs(raw_logs_dir) {
        Ok(results) => {
             info!("✅ Parsing completed. Metrics saved to: {}", paths.parsed_metrics.display());
             info!("Parsed {} logs", results.len());
        }
        Err(e) => {
             error!("❌ Parsing failed: {}", e);
             std::process::exit(1);
        }
    }

    Ok(())
}

async fn generate_reports_only(
    output: Option<PathBuf>,
    parsed_metrics_dir: Option<PathBuf>,
    report_formats: Option<String>,
) -> anyhow::Result<()> {
    info!("═══════════════════════════════════════════════════════════");
    info!("🚀 zkVM Benchmark - Phase 3: Report Generation");
    info!("═══════════════════════════════════════════════════════════");

    let output_dir = output
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "benchmark-results".to_string());
    
    // We construct a config just to get the paths logic, but passing empty filters
    let config = BenchmarkConfig {
        test_scales: vec![],
        zkvms: HashMap::new(),
        output_dir: output_dir.clone(),
        timeout_seconds: None,
        repeat_count: None,
    };
    let paths = config.get_paths();
    
    let metrics_dir = if let Some(dir) = parsed_metrics_dir {
        dir
    } else {
        paths.parsed_metrics.clone()
    };

    if !metrics_dir.exists() {
        anyhow::bail!("parsed-metrics directory not found: {}", metrics_dir.display());
    }

    let mut results = Vec::new();

    info!("📖 Reading metrics from: {}", metrics_dir.display());

    for entry in fs::read_dir(metrics_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
             let content = fs::read_to_string(&path)?;
             match serde_json::from_str::<UnifiedMetrics>(&content) {
                 Ok(metrics) => {
                     // Reconstruct ExecutionResult
                     // We need zkvm_name enum etc.
                     // metrics.metadata has it.
                     
                     let success = metrics.summary.success; // Default success from summary
                     
                     results.push(ExecutionResult {
                         test_run: TestRun {
                             zkvm_name: metrics.metadata.zkvm_name.clone(),
                             program_name: metrics.metadata.program_name.clone(),
                             mode: metrics.metadata.mode.clone().unwrap_or(zkvm_benchmark_utils::core::metrics::ProofMode::Groth16),
                             scale: metrics.metadata.scale.unwrap_or(0),
                             repeat: 1,
                         },
                         metrics: Some(metrics),
                         log_content: String::new(),
                         success,
                         error: None,
                         resource_stats: None, // We don't need resource stats for reporting as they are already in metrics
                     });
                 }
                 Err(e) => {
                     warn!("Failed to parse metrics file {}: {}", path.display(), e);
                 }
             }
        }
    }

    info!("📊 Loaded {} results", results.len());

    let formats = if let Some(formats_str) = report_formats {
        info!("📋 Requested formats: {}", formats_str);
        let parsed: Vec<ReportFormat> = formats_str
            .split(',')
            .filter_map(|s| ReportFormat::from_str(s.trim()))
            .collect();
        if parsed.is_empty() {
            warn!("⚠️  No valid formats parsed, using all formats");
            None
        } else {
            Some(parsed)
        }
    } else {
        None
    };

    let reporter = BenchmarkReporter::new(results);
    let reports_dir = paths.reports;
    
    reporter.generate(&reports_dir, formats)?;
    
    info!("✅ Reports generated in: {}", reports_dir.display());

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
