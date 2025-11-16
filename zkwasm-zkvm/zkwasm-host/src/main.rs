use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;
use std::fs;

#[derive(Parser)]
#[command(name = "zkwasm-host")]
#[command(about = "zkWasm host program for proving Fibonacci computation")]
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
        /// Input value for Fibonacci (n)
        #[arg(short, long, default_value = "10")]
        n: u64,
        
        /// Enable mock test before proving
        #[arg(short, long)]
        mock: bool,
    },
    
    /// Verify a proof
    Verify,
    
    /// Run all steps: build, setup, prove, and verify
    Run {
        /// Input value for Fibonacci (n)
        #[arg(short, long, default_value = "10")]
        n: u64,
        
        /// Size of the circuit (k parameter)
        #[arg(short, long, default_value = "18")]
        k: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Build => build_wasm()?,
        Commands::Setup { k } => setup_circuit(k)?,
        Commands::Prove { n, mock } => prove(n, mock)?,
        Commands::Verify => verify()?,
        Commands::Run { n, k } => {
            println!("Running complete zkWasm demo...\n");
            build_wasm()?;
            setup_circuit(k)?;
            prove(n, false)?;
            verify()?;
            println!("\n✅ Complete! Proof generated and verified successfully.");
        }
    }
    
    Ok(())
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
            "fib-demo",
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

fn prove(n: u64, mock: bool) -> Result<()> {
    println!("🔐 Generating proof for Fibonacci({})...", n);
    
    let zkwasm_cli = find_zkwasm_cli()?;
    
    let mut args = vec![
        "--params".to_string(),
        "params".to_string(),
        "fib-demo".to_string(),
        "prove".to_string(),
        "--wasm".to_string(),
        "output/guest.wasm".to_string(),
        "--output".to_string(),
        "output".to_string(),
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
            "fib-demo",
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
    if let Ok(output) = Command::new("which").arg("delphinus-cli").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
    }
    
    // Check common installation paths
    let common_paths = vec![
        "delphinus-cli",
        "./delphinus-cli",
        "../delphinus-cli",
    ];
    
    for path in common_paths {
        if Command::new(path).arg("--help").output().is_ok() {
            return Ok(PathBuf::from(path));
        }
    }
    
    anyhow::bail!(
        "zkWasm CLI (delphinus-cli) not found. Please install it first:\n\
         git clone --recurse-submodules https://github.com/DelphinusLab/zkwasm\n\
         cd zkwasm\n\
         cargo build --release\n\
         Then add target/release/delphinus-cli to your PATH or copy it to this directory."
    )
}

