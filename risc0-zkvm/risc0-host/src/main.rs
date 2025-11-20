use std::time::Instant;
use methods::{METHODS_ELF, METHODS_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use common::load_program_input;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    dotenv::dotenv().ok();

    // Load program input from environment
    let input = load_program_input();
    
    println!("╔════════════════════════════════════════╗");
    println!("║       Risc0 Multi-Program Demo        ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);
    println!();

    // Create executor environment with inputs
    let env = ExecutorEnv::builder()
        .write(&input.program.id())
        .unwrap()
        .write(&input.n)
        .unwrap()
        .build()
        .unwrap();

    // Use Groth16 prover for smallest proof size
    let opts = ProverOpts::groth16();
    
    println!("🔧 Setting up proving environment...");
    println!("🔐 Generating proof...");
    
    let start = Instant::now();
    let prove_info = default_prover()
        .prove_with_opts(env, METHODS_ELF, &opts)
        .unwrap();

    let duration = Instant::now().duration_since(start).as_secs_f64();

    println!();
    println!("✅ Successfully generated proof!");
    println!("📊 Proof Information:");
    println!("─────────────────────────────────────");
    println!("🔒 Mode: {:?}", opts.receipt_kind);
    println!("📦 Size: {} bytes", prove_info.receipt.seal_size());
    println!("⏱️  Time: {:.2}s", duration);
    
    // Verify the receipt
    println!();
    println!("🔍 Verifying proof...");
    prove_info.receipt.verify(METHODS_ID).unwrap();
    
    // Read output
    let output: u32 = prove_info.receipt.journal.decode().unwrap();
    println!("📤 Output: {}", output);
    
    println!();
    println!("✨ Successfully verified proof!");
    println!("╚════════════════════════════════════════╝");
}
