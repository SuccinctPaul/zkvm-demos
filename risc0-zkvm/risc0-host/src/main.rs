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

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let proof_mode = std::env::var("RISC0_PROOF_MODE").unwrap_or_else(|_| "groth16".to_string());
    println!("Selected proof mode: {}", proof_mode);

    let opts = match proof_mode.as_str() {
        "core" => ProverOpts::default(),
        "compressed" => ProverOpts::succinct(),
        "groth16" => ProverOpts::groth16(),
        _ => panic!("Unknown proof mode: {}", proof_mode),
    };

    // Split Execution and Proving to capture granular metrics
    println!("Starting execution...");
    let exec_start = Instant::now();
    let exec = risc0_zkvm::default_executor();
    let session = exec.execute(env, METHODS_ELF).unwrap();
    let exec_duration = exec_start.elapsed();
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());

    // Calculate cycles from session
    // segments contains the execution trace info
    // Note: In Risc0 1.0+, session.user_cycles etc might be accessible via session.get_exit_code() or similar
    // For now we rely on guest printing "cycle tracker" or the internal logger.
    // We can also print segment count here.
    // session.segments is private/internal usually, but let's see if we can get info.
    // Actually, just proceeding to prove.

    println!("Starting proving...");
    let prove_start = Instant::now();
    let prove_info = default_prover()
        .prove_with_opts(env, METHODS_ELF, &opts)
        .unwrap();
    // prover.prove_session produces the receipt
    let ctx = risc0_zkvm::VerifierContext::default();
    let prove_info = prover.prove_session(&ctx, &session, &opts).unwrap();
    let prove_duration = prove_start.elapsed();

    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());

    println!(
        "proof mode: {:?}, proof size: {:?} Bytes, proof time total cost(s): {:?}",
        proof_mode,
        prove_info.receipt.seal_size(),
        prove_duration.as_secs_f64()
    );

    // extract the receipt.
    let receipt = prove_info.receipt;

    // For example:
    // let _output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt.verify(METHODS_ID).unwrap();
}
