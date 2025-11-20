use nexus_sdk::nexus_sdk_macros::profile;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::time::Instant;
use common::load_program_input;

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

    // Pack inputs into a single u64
    let input_packed = (input.program.id() as u64) << 32 | (input.n as u64);

    print!("1. Compiling guest program...");
    let compile_start = Instant::now();
    let mut prover_compiler = Compiler::<CargoPackager>::new(GUEST_PACKAGE);
    let prover: Stwo<Local> = Stwo::compile(&mut prover_compiler).unwrap();
    let compile_duration = compile_start.elapsed();
    println!("====== Compile Cost: {}s", compile_duration.as_secs_f64());

    let elf = prover.elf.clone(); // save elf for use with verification
    println!("ELF: instructions num: {:?}", elf.instructions.len());

    println!("Proving execution of vm...");
    let now = std::time::Instant::now();
    
    // Prove with packed input
    let (view, proof) = prover
        .prove_with_input::<(), u64>(&(), &input_packed)
        .expect("failed to prove program");
        
    println!(
        "Prove cost: {:?} s, proof size: {:?} Bytes",
        std::time::Instant::now().duration_since(now).as_secs_f64(),
        proof.size_estimate()
    );

    println!("\n3. Execution Logs:");
    println!("-------------------");
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
    println!("-------------------");

    print!("Verifying execution...");

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

    println!("  Succeeded!");
    
    println!("\n============ Summary ============");
    println!("Program: {}", input.program.as_str());
    println!("Input: n = {}", input.n);
    println!("Proof size: {} bytes", proof.size_estimate());
    println!("=================================\n");
}
