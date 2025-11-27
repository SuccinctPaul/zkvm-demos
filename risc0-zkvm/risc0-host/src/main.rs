use std::time::Instant;
use methods::{METHODS_ELF, METHODS_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts};
use zkvm_programs::load_program_input;

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
    println!("📋 Program: {} (ID={})", input.program.name(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);
    println!();

    println!("\n========== BENCHMARK START ==========");
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=risc0");

    // Proof mode from environment
    let proof_mode = std::env::var("RISC0_PROOF_MODE").unwrap_or_else(|_| "groth16".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    let opts = match proof_mode.as_str() {
        "core" => ProverOpts::default(),
        "compressed" => ProverOpts::succinct(),
        "groth16" => ProverOpts::groth16(),
        _ => panic!("Unknown proof mode: {}", proof_mode),
    };

    // Create executor environment with inputs
    let env = ExecutorEnv::builder()
        .write(&input.program.id())
        .unwrap()
        .write(&input.n)
        .unwrap()
        .build()
        .unwrap();

    // ========== Proving Phase (includes execution) ==========
    println!("\n--- Proving Phase ---");
    println!("Starting proving (mode: {})...", proof_mode);
    let prove_start = Instant::now();
    let prover = default_prover();
    let prove_info = prover.prove_with_opts(env, METHODS_ELF, &opts).unwrap();
    let prove_duration = prove_start.elapsed();

    // Extract receipt and stats
    let receipt = prove_info.receipt;
    let stats = prove_info.stats;

    println!("BENCHMARK: total_cycles={}", stats.total_cycles);
    println!("BENCHMARK: user_cycles={}", stats.user_cycles);
    println!("BENCHMARK: segment_count={}", stats.segments);
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());

    // Get proof size from seal
    let seal_size = receipt.seal_size();
    println!("BENCHMARK: proof_size_bytes={}", seal_size);

    println!(
        "\nproof mode: {}, proof size: {} Bytes, proof time: {:.3}s",
        proof_mode,
        seal_size,
        prove_duration.as_secs_f64()
    );

    // ========== Verification Phase ==========
    println!("\n--- Verification Phase ---");
    let verify_start = Instant::now();
    receipt.verify(METHODS_ID).expect("Verification failed");
    let verify_duration = verify_start.elapsed();

    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
    println!("BENCHMARK: success_status=success");

    println!("\n========== BENCHMARK END ==========");
    println!("✅ Proof generated and verified successfully!");
}
