use openvm_sdk::{config::ProverConfig, Prover, StdIn};
use std::time::Instant;
use common::load_program_input;

// Include the guest program ELF binary
openvm_sdk::include_guest!();

fn main() -> anyhow::Result<()> {
    // Setup environment
    dotenv::dotenv().ok();
    env_logger::init();

    // Load program input from environment
    let input = load_program_input();
    println!("╔════════════════════════════════════════╗");
    println!("║       OpenVM Multi-Program Demo       ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);

    println!("\n1. Initializing OpenVM prover...");
    let init_start = Instant::now();
    
    // Create prover configuration
    let config = ProverConfig::default();
    
    println!("   Initialization completed in {:.2}s", init_start.elapsed().as_secs_f64());

    println!("\n2. Loading guest program...");
    let load_start = Instant::now();
    
    // Load the guest program ELF
    let elf = GUEST_ELF;
    
    println!("   Guest program loaded in {:.2}s", load_start.elapsed().as_secs_f64());
    println!("   ELF size: {} bytes", elf.len());

    println!("\n3. Executing program in zkVM...");
    let exec_start = Instant::now();
    
    // Create input for the guest program
    let mut stdin = StdIn::default();
    stdin.write(&input.program.id());
    stdin.write(&input.n);
    
    // Execute the program
    let prover = Prover::new(&config)?;
    let (output, execution_report) = prover.execute(elf, stdin.clone())?;
    
    println!("   Execution completed in {:.2}s", exec_start.elapsed().as_secs_f64());
    println!("   Cycle count: {}", execution_report.total_cycles());
    
    // Read the result from output
    let result: u32 = output.read();
    println!("   Result: {}", result);

    println!("\n4. Generating zero-knowledge proof...");
    let prove_start = Instant::now();
    
    // Create stdin again for proving (clone wasn't sufficient for StdIn reuse in some versions, safe to recreate)
    let mut stdin_prove = StdIn::default();
    stdin_prove.write(&input.program.id());
    stdin_prove.write(&input.n);
    
    // Generate proof
    let proof = prover.prove(elf, stdin_prove)?;
    
    let prove_duration = prove_start.elapsed();
    println!("   Proof generation completed in {:.2}s", prove_duration.as_secs_f64());
    
    // Get proof size
    let proof_bytes = bincode::serialize(&proof)?;
    println!("   Proof size: {} bytes", proof_bytes.len());

    println!("\n5. Verifying proof...");
    let verify_start = Instant::now();
    
    // Verify the proof
    prover.verify(&proof)?;
    
    println!("   Verification completed in {:.2}s", verify_start.elapsed().as_secs_f64());
    println!("   ✓ Proof verified successfully!");

    println!("\n============ Summary ============");
    println!("Program: {}", input.program.as_str());
    println!("Input N: {}", input.n);
    println!("Output: {}", result);
    println!("Total cycles: {}", execution_report.total_cycles());
    println!("Proof size: {} bytes", proof_bytes.len());
    println!("Prove time: {:.2}s", prove_duration.as_secs_f64());
    println!("=================================\n");

    Ok(())
}
