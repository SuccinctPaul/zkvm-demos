//! ZisK zkVM Host Program
//! 
//! This program manages the proving and verification of the guest program
//! executing inside the ZisK zero-knowledge virtual machine.

use anyhow::Result;
use std::time::Instant;
use zisk_sdk::{Prover, ProverConfig, Stdin, Verifier};

// Include the compiled guest program
zisk_sdk::include_guest!();

fn main() -> Result<()> {
    // Setup logging and environment
    env_logger::init();
    dotenv::dotenv().ok();

    println!("\n=== ZisK zkVM Fibonacci Demo ===\n");

    // Load the Fibonacci input number from environment
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);

    // Step 1: Initialize the prover
    println!("\n1. Initializing ZisK prover...");
    let init_start = Instant::now();
    
    let config = ProverConfig::default();
    let prover = Prover::new(config)?;
    
    println!("   Initialization completed in {:.2}s", init_start.elapsed().as_secs_f64());

    // Step 2: Prepare input for the guest program
    println!("\n2. Preparing input...");
    let mut stdin = Stdin::new();
    stdin.write(&fib_n);
    
    println!("   Input prepared: n = {}", fib_n);

    // Step 3: Execute the program (optional, for getting the result)
    println!("\n3. Executing program in zkVM...");
    let exec_start = Instant::now();
    
    let elf = GUEST_ELF;
    let (output, execution_report) = prover.execute(elf, stdin.clone())?;
    
    println!("   Execution completed in {:.2}s", exec_start.elapsed().as_secs_f64());
    println!("   Cycle count: {}", execution_report.total_cycles());
    
    // Read the result from output
    let result: u32 = output.read();
    println!("   Fibonacci({}) = {}", fib_n, result);

    // Step 4: Generate the zero-knowledge proof
    println!("\n4. Generating zero-knowledge proof...");
    println!("   (This may take a while depending on the computation size)");
    let prove_start = Instant::now();
    
    // Prepare input again for proving
    let mut stdin_prove = Stdin::new();
    stdin_prove.write(&fib_n);
    
    let proof = prover.prove(elf, stdin_prove)?;
    
    let prove_duration = prove_start.elapsed();
    println!("   Proof generation completed in {:.2}s", prove_duration.as_secs_f64());
    
    // Get proof size
    let proof_bytes = serde_json::to_vec(&proof)?;
    println!("   Proof size: {} bytes ({:.2} KB)", proof_bytes.len(), proof_bytes.len() as f64 / 1024.0);

    // Step 5: Verify the proof
    println!("\n5. Verifying proof...");
    let verify_start = Instant::now();
    
    let verifier = Verifier::new();
    verifier.verify(&proof)?;
    
    println!("   Verification completed in {:.2}s", verify_start.elapsed().as_secs_f64());
    println!("   ✓ Proof verified successfully!");

    // Summary
    println!("\n============ Summary ============");
    println!("Input: n = {}", fib_n);
    println!("Output: fibonacci({}) = {}", fib_n, result);
    println!("Total cycles: {}", execution_report.total_cycles());
    println!("Proof size: {} bytes ({:.2} KB)", proof_bytes.len(), proof_bytes.len() as f64 / 1024.0);
    println!("Prove time: {:.2}s", prove_duration.as_secs_f64());
    println!("Verify time: {:.2}s", verify_start.elapsed().as_secs_f64());
    println!("=================================\n");

    Ok(())
}

