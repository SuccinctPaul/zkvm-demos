use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use zkvm_programs::load_program_input;

#[derive(Parser)]
#[command(name = "zkwasm-host")]
#[command(about = "zkWasm host program for Multi-Program Demo")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the guest WASM program
    Build,
    
    /// Setup the zkWasm circuit
    Setup {
        /// Size of the circuit (k parameter)
        #[arg(short, long, default_value = "18")]
        k: u32,
    },
    
    /// Generate a proof
    Prove {
        /// Enable mock test before proving
        #[arg(short, long)]
        mock: bool,
    },
    
    /// Verify a proof
    Verify,
    
    /// Run all steps: build, setup, prove, and verify
    Run {
        /// Size of the circuit (k parameter)
        #[arg(short, long, default_value = "18")]
        k: u32,
    },
}

/// Timing results for each phase
#[derive(Default)]
struct BenchmarkTimings {
    build_time_s: f64,
    setup_time_s: f64,
    prove_time_s: f64,
    verify_time_s: f64,
    total_time_s: f64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load input from environment variables (standard way)
    let input = load_program_input();
    
    match cli.command {
        Commands::Build => build_wasm()?,
        Commands::Setup { k } => setup_circuit(k)?,
        Commands::Prove { mock } => prove(input.program.id(), input.n, mock)?,
        Commands::Verify => verify()?,
        Commands::Run { k } => {
            println!("Running complete zkWasm demo...\n");
            println!("Program: {} (ID={})", input.program.name(), input.program.id());
            println!("Input N: {}", input.n);
            
            let mut timings = BenchmarkTimings::default();
            let total_start = std::time::Instant::now();
            
            // Build phase
            let build_start = std::time::Instant::now();
            build_wasm()?;
            timings.build_time_s = build_start.elapsed().as_secs_f64();
            
            // Setup phase
            let setup_start = std::time::Instant::now();
            setup_circuit(k)?;
            timings.setup_time_s = setup_start.elapsed().as_secs_f64();
            
            // Prove phase
            let prove_start = std::time::Instant::now();
            prove(input.program.id(), input.n, false)?;
            timings.prove_time_s = prove_start.elapsed().as_secs_f64();
            
            // Verify phase
            let verify_start = std::time::Instant::now();
            verify()?;
            timings.verify_time_s = verify_start.elapsed().as_secs_f64();
            
            timings.total_time_s = total_start.elapsed().as_secs_f64();
            
            // Get proof size if available
            let proof_size = get_proof_size();
            
            // Output BENCHMARK metrics in standard format
            println!("\n--- BENCHMARK METRICS ---");
            println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
            println!("BENCHMARK: zkvm_name=zkwasm");
            println!("BENCHMARK: zkvm_version=v0.1.0");
            println!("BENCHMARK: proof_mode=core");
            println!("BENCHMARK: circuit_k={}", k);
            
            // Timing metrics
            println!("BENCHMARK: build_time_s={:.6}", timings.build_time_s);
            println!("BENCHMARK: setup_time_s={:.6}", timings.setup_time_s);
            println!("BENCHMARK: prove_time_s={:.6}", timings.prove_time_s);
            println!("BENCHMARK: verify_time_s={:.6}", timings.verify_time_s);
            println!("BENCHMARK: total_time_s={:.6}", timings.total_time_s);
            
            // Proof size
            if let Some(size) = proof_size {
                println!("BENCHMARK: proof_size_bytes={}", size);
            }
            
            println!("BENCHMARK: success_status=success");
            println!("--- END BENCHMARK METRICS ---\n");
            
            println!("✅ Complete! Proof generated and verified successfully.");
        }
    }
    
    Ok(())
}

/// Get the proof file size in bytes
fn get_proof_size() -> Option<u64> {
    // Try to find the transcript file (proof file)
    let proof_paths = [
        "output/output.0.transcript.data",
        "output/proof.bin",
    ];
    
    for path in &proof_paths {
        if let Ok(metadata) = fs::metadata(path) {
            return Some(metadata.len());
        }
    }
    
    // Try to sum all transcript files
    if let Ok(entries) = fs::read_dir("output") {
        let mut total_size = 0u64;
        let mut found = false;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.contains("transcript") || name.ends_with(".proof") {
                    if let Ok(metadata) = fs::metadata(&path) {
                        total_size += metadata.len();
                        found = true;
                    }
                }
            }
        }
        if found {
            return Some(total_size);
        }
    }
    
    None
}

fn build_wasm() -> Result<()> {
    println!("📦 Building WASM guest program...");
    
    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target", "wasm32-unknown-unknown",
            "--manifest-path", "zkwasm-guest/Cargo.toml"
        ])
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to build WASM guest program");
    }
    
    // Create output directory
    fs::create_dir_all("output")?;
    
    // Copy WASM file to output directory
    let wasm_path = "target/wasm32-unknown-unknown/release/zkwasm_guest.wasm";
    fs::copy(wasm_path, "output/guest.wasm")?;
    
    println!("✅ WASM built successfully: output/guest.wasm\n");
    Ok(())
}

fn setup_circuit(k: u32) -> Result<()> {
    println!("⚙️  Setting up zkWasm circuit (k={})...", k);
    
    // Check if zkwasm-cli is installed
    let zkwasm_cli = find_zkwasm_cli()?;
    
    let params_dir = "params";
    fs::create_dir_all(params_dir)?;
    
    let status = Command::new(&zkwasm_cli)
        .args([
            "--params", params_dir,
            "output",
            "setup",
            "-k", &k.to_string(),
            "--wasm", "output/guest.wasm",
        ])
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to setup circuit");
    }
    
    println!("✅ Circuit setup complete\n");
    Ok(())
}

fn prove(program_id: u32, n: u32, mock: bool) -> Result<()> {
    println!("🔐 Generating proof for ProgramID={} Input={}...", program_id, n);
    
    let zkwasm_cli = find_zkwasm_cli()?;
    
    // Pass inputs as separate --public arguments or space separated?
    // zkWasm CLI usually takes --public for each input
    let mut args = vec![
        "--params".to_string(),
        "params".to_string(),
        "output".to_string(),
        "prove".to_string(),
        "--wasm".to_string(),
        "output/guest.wasm".to_string(),
        "--output".to_string(),
        "output".to_string(),
        "--public".to_string(),
        format!("{}:i64", program_id),
        "--public".to_string(),
        format!("{}:i64", n),
    ];
    
    if mock {
        args.push("--mock".to_string());
    }
    
    let status = Command::new(&zkwasm_cli)
        .args(&args)
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to generate proof");
    }
    
    println!("✅ Proof generated successfully\n");
    Ok(())
}

fn verify() -> Result<()> {
    println!("🔍 Verifying proof...");
    
    let zkwasm_cli = find_zkwasm_cli()?;
    
    let status = Command::new(&zkwasm_cli)
        .args([
            "--params", "params",
            "output",
            "verify",
            "--output", "output",
        ])
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Failed to verify proof");
    }
    
    println!("✅ Proof verified successfully\n");
    Ok(())
}

fn find_zkwasm_cli() -> Result<PathBuf> {
    // Try to find zkwasm-cli in PATH or common locations
    // Note: The CLI binary is named 'zkwasm-cli' (not 'delphinus-cli')
    for cli_name in ["zkwasm-cli", "delphinus-cli"] {
        if let Ok(output) = Command::new("which").arg(cli_name).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Ok(PathBuf::from(path));
                }
            }
        }
    }
    
    // Check common installation paths
    let common_paths = vec![
        // Standard installation location
        format!("{}/.zkwasm/zkwasm/target/release/zkwasm-cli", std::env::var("HOME").unwrap_or_default()),
        format!("{}/.local/bin/zkwasm-cli", std::env::var("HOME").unwrap_or_default()),
        // Legacy name locations
        format!("{}/.zkwasm/zkwasm/target/release/delphinus-cli", std::env::var("HOME").unwrap_or_default()),
        format!("{}/.local/bin/delphinus-cli", std::env::var("HOME").unwrap_or_default()),
        // Local directory
        "zkwasm-cli".to_string(),
        "./zkwasm-cli".to_string(),
        "delphinus-cli".to_string(),
        "./delphinus-cli".to_string(),
    ];
    
    for path in common_paths {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() {
            // Verify it's executable
            if Command::new(&path).arg("--help").output().is_ok() {
                return Ok(path_buf);
            }
        }
    }
    
    anyhow::bail!(
        "zkWasm CLI (zkwasm-cli) not found. Please install it first:\n\
         git clone --recurse-submodules https://github.com/DelphinusLab/zkwasm\n\
         cd zkwasm\n\
         cargo build --release\n\
         Then add target/release/zkwasm-cli to your PATH or copy it to this directory."
    )
}
