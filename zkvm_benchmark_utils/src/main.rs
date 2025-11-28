//! zkVM Benchmark CLI

use clap::{Parser, Subcommand};
use log::{error, info, warn};
use std::path::PathBuf;
use std::time::Duration;

use std::collections::HashMap;
use zkvm_benchmark_utils::{
    BenchmarkConfig, BenchmarkExecutor, BenchmarkReporter, ProgramConfig, ReportFormat, ZkVmConfig,
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

        /// Programs to run (comma-separated, e.g., "fibonacci,hash,sum"). If not specified, uses programs from config
        #[arg(long)]
        programs: Option<String>,

        /// Test scales/parameters (comma-separated, e.g., "10,20,100"). Overrides config values
        #[arg(long)]
        scales: Option<String>,

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
        Commands::Run {
            output,
            zkvms,
            programs,
            scales,
            report_formats,
        } => {
            run_benchmarks(output, zkvms, programs, scales, report_formats).await?;
        }
    }

    Ok(())
}

async fn run_benchmarks(
    output: Option<PathBuf>,
    zkvms_filter: Option<String>,
    programs_filter: Option<String>,
    scales_filter: Option<String>,
    report_formats: Option<String>,
) -> anyhow::Result<()> {
    info!("═══════════════════════════════════════════════════════════");
    info!("🚀 zkVM Benchmark Framework - Starting Execution");
    info!("═══════════════════════════════════════════════════════════");

    // Determine which zkVMs to run
    let zkvm_names: Vec<String> = if let Some(zkvms) = zkvms_filter {
        info!("📋 Using specified zkVMs: {}", zkvms);
        zkvms.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        info!("🔍 Scanning configs/ for enabled zkVMs...");
        // Scan configs/ directory for all enabled zkVMs
        load_enabled_zkvms()?
    };

    if zkvm_names.is_empty() {
        anyhow::bail!("No zkVMs specified and no enabled zkVMs found in configs/");
    }

    info!("📦 Selected zkVMs: {}", zkvm_names.join(", "));

    // Parse program filter if specified
    let program_names: Option<Vec<String>> =
        programs_filter.map(|p| p.split(',').map(|s| s.trim().to_string()).collect());

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

        // Apply program filter if specified
        if let Some(ref prog_names) = program_names {
            let mut filtered_programs = Vec::new();
            for prog_name in prog_names {
                // Find existing program config or create new one
                let existing_prog = zkvm_config
                    .get_programs()
                    .into_iter()
                    .find(|p| p.name == *prog_name);

                if let Some(mut prog_config) = existing_prog {
                    // Apply scale filter if specified
                    if let Some(ref scales) = scale_values {
                        prog_config.scales = scales.clone();
                    }
                    filtered_programs.push(prog_config);
                } else {
                    // Create new program config
                    let scales = scale_values
                        .clone()
                        .unwrap_or_else(|| zkvm_config.get_test_scales(None));
                    filtered_programs.push(ProgramConfig {
                        name: prog_name.clone(),
                        scales,
                        env_vars: None,
                        timeout_seconds: None,
                    });
                }
            }
            zkvm_config.programs = Some(filtered_programs);
        } else if let Some(ref scales) = scale_values {
            // Apply scale filter to all programs if no program filter specified
            let mut programs = zkvm_config.get_programs();
            for prog in &mut programs {
                prog.scales = scales.clone();
            }
            zkvm_config.programs = Some(programs);
        }

        // Apply scale filter if specified (for backward compatibility with test_scales)
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

    // Build BenchmarkConfig
    let output_dir = output
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "benchmark-results".to_string());

    info!("📁 Output directory: {}", output_dir);
    info!("🔢 Test scales: {:?}", test_scales);

    let config = BenchmarkConfig {
        test_scales,
        zkvms,
        output_dir,
        timeout_seconds: Some(3600),
        repeat_count: Some(1),
    };

    info!("═══════════════════════════════════════════════════════════");
    info!("⚙️  Initializing Benchmark Executor");
    info!("═══════════════════════════════════════════════════════════");
    let executor = BenchmarkExecutor::new(config.clone())?;

    info!("═══════════════════════════════════════════════════════════");
    info!("🏃 Starting Benchmark Execution (max 2 hours)");
    info!("═══════════════════════════════════════════════════════════");

    // Run benchmarks with timeout to prevent hanging
    let start_time = std::time::Instant::now();
    let results = tokio::time::timeout(
        Duration::from_secs(7200), // 2 hours max
        executor.run_all(),
    )
    .await;

    let results = match results {
        Ok(Ok(r)) => {
            let elapsed = start_time.elapsed();
            info!(
                "⏱️  Total execution time: {:.2} seconds",
                elapsed.as_secs_f64()
            );
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

    info!("═══════════════════════════════════════════════════════════");
    info!("📊 Execution Summary");
    info!("═══════════════════════════════════════════════════════════");
    info!("  Total runs: {}", total);
    info!("  ✅ Successful: {}", successful);
    if failed > 0 {
        warn!("  ❌ Failed: {}", failed);
    }
    info!(
        "  📁 Results directory: {}",
        executor.output_dir().display()
    );

    info!("═══════════════════════════════════════════════════════════");
    info!("📝 Generating Reports");
    info!("═══════════════════════════════════════════════════════════");

    // Parse report formats
    let formats = if let Some(formats_str) = report_formats {
        info!("📋 Requested formats: {}", formats_str);
        let parsed: Vec<ReportFormat> = formats_str
            .split(',')
            .filter_map(|s| ReportFormat::from_str(s.trim()))
            .collect();
        if parsed.is_empty() {
            warn!("⚠️  No valid formats parsed, using all formats");
            None // Fall back to all formats if parsing failed
        } else {
            info!("✅ Parsed {} format(s)", parsed.len());
            Some(parsed)
        }
    } else {
        info!("📋 No format specified, generating all formats");
        None // Generate all formats
    };

    let reporter = BenchmarkReporter::new(results);
    // Generate reports in the reports/ subdirectory
    let reports_dir = executor.output_dir().join("reports");
    info!("📁 Reports directory: {}", reports_dir.display());

    match reporter.generate(&reports_dir, formats) {
        Ok(_) => {
            info!("✅ Reports generated successfully");
            info!("═══════════════════════════════════════════════════════════");
        }
        Err(e) => {
            error!("❌ Failed to generate reports: {}", e);
            error!("═══════════════════════════════════════════════════════════");
            // Continue to exit even if report generation fails
        }
    }

    // In CI mode, exit with non-zero if any benchmark failed
    if failed > 0 {
        std::process::exit(1);
    }

    // Force exit to cleanup any lingering background tasks/threads
    std::process::exit(0);
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
