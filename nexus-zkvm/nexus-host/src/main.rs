use nexus_sdk::nexus_sdk_macros::profile;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::time::Instant;
use zkvm_programs::load_program_input;

const GUEST_PACKAGE: &str = "nexus-guest";

// Build with `cargo build --release` to build in release mode.
#[profile]
fn main() {
    // Load program input from environment
    let input = load_program_input();
    println!("╔════════════════════════════════════════╗");
    println!("║       Nexus Multi-Program Demo        ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);

    // Benchmark header
    println!("\n========== BENCHMARK START ==========");
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=nexus");
    println!("BENCHMARK: zkvm_version=v3.0.0");
    
    // Nexus only supports core/stwo proving mode
    let proof_mode = std::env::var("NEXUS_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    // Pack inputs into a single u64
    let input_packed = (input.program.id() as u64) << 32 | (input.n as u64);

    println!("\n--- Compilation Phase ---");
    let compile_start = Instant::now();
    let mut prover_compiler = Compiler::<CargoPackager>::new(GUEST_PACKAGE);
    let prover: Stwo<Local> = Stwo::compile(&mut prover_compiler).unwrap();
    let compile_duration = compile_start.elapsed();
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());

    let elf = prover.elf.clone(); // save elf for use with verification
    let instruction_count = elf.instructions.len();
    println!("BENCHMARK: instruction_count={}", instruction_count);
    println!("ELF: instructions num: {:?}", instruction_count);

    println!("\n--- Proving Phase ---");
    let prove_start = Instant::now();
    
    // Prove with packed input
    let (view, proof) = prover
        .prove_with_input::<(), u64>(&(), &input_packed)
        .expect("failed to prove program");
        
    let prove_duration = prove_start.elapsed();
    let proof_size = proof.size_estimate();
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    println!("BENCHMARK: proof_size_bytes={}", proof_size);
    println!(
        "Prove cost: {:?} s, proof size: {:?} Bytes",
        prove_duration.as_secs_f64(),
        proof_size
    );

    println!("\n--- Execution Logs ---");
    match view.logs() {
        Ok(logs) => {
            for log in logs {
                println!("{}", log);
                // Try to parse result from logs since we printed it
                if log.contains("Result:") {
                    println!("Found Result in logs: {}", log.trim());
                }
            }
        },
        Err(e) => eprintln!("Error: Failed to retrieve debug logs - {}", e),
    }

    println!("\n--- Verification Phase ---");
    let verify_start = Instant::now();

    #[rustfmt::skip]
    proof
        .verify_expected::<u64, ()>(
            &input_packed,  // public input
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &(),  // no public output
            &elf, // expected elf (program binary)
            &[],  // no associated data,
        )
        .expect("failed to verify proof");

    let verify_duration = verify_start.elapsed();
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
    println!("BENCHMARK: success_status=success");
    println!("Verification Succeeded!");
    
    println!("\n========== BENCHMARK END ==========");
    println!("\n============ Summary ============");
    println!("Program: {}", input.program.as_str());
    println!("Input: n = {}", input.n);
    println!("Proof size: {} bytes", proof_size);
    println!("=================================\n");
}
