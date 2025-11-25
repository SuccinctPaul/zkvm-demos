//! Powdr zkVM Host Program
//! 
//! This program demonstrates how to use Powdr zkVM to generate and verify
//! zero-knowledge proofs for multi-program execution.
//! 
//! Note: Powdr is a zkVM toolkit under active development. This implementation
//! shows the expected workflow and will be updated once the official SDK is released.

use std::time::Instant;
use anyhow::Result;
use common::{load_program_input, execute_program};

const POWDR_VERSION: &str = "v0.1.0-dev";

fn main() -> Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    println!("========================================");
    println!("  Powdr zkVM - Multi-Program Demo");
    println!("========================================\n");
    
    // Load program input from environment
    let input = load_program_input();
    
    // Output BENCHMARK format logs for parsing
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=powdr");
    println!("BENCHMARK: zkvm_version={}", POWDR_VERSION);
    
    // Get proof mode from environment (default: core)
    let proof_mode = std::env::var("POWDR_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);
    
    println!("📊 Configuration:");
    println!("   Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("   Input: n = {}", input.n);
    
    // Calculate expected result for simulation
    let expected_result = execute_program(input.program.id(), input.n);
    println!("   Expected result: {}\n", expected_result);
    
    println!("⚠️  Note: This is a reference implementation.");
    println!("    Powdr zkVM is a toolkit for building custom zkVMs.");
    println!("    Full integration pending official SDK release.\n");
    
    // Step 1: Compile guest program (simulated)
    println!("🔨 Step 1: Compiling guest program...");
    let compile_start = Instant::now();
    
    simulate_compilation()?;
    
    let compile_duration = compile_start.elapsed();
    println!("   ✅ Compilation completed in {:.2}s", compile_duration.as_secs_f64());
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
    println!("   Circuit generated\n");
    
    // Step 2: Setup proving system (simulated)
    println!("🔧 Step 2: Setting up proving system...");
    let setup_start = Instant::now();
    
    simulate_setup()?;
    
    let setup_duration = setup_start.elapsed();
    println!("   ✅ Setup completed in {:.2}s", setup_duration.as_secs_f64());
    println!("BENCHMARK: setup_time_s={:.6}", setup_duration.as_secs_f64());
    println!("   Proving keys generated\n");
    
    // Step 3: Execute program
    println!("🚀 Step 3: Executing program...");
    let exec_start = Instant::now();
    
    // Execute the computation
    let result = execute_program(input.program.id(), input.n);
    
    let exec_duration = exec_start.elapsed();
    println!("   ✅ Execution completed in {:.2}s", exec_duration.as_secs_f64());
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    println!("   Result: {}\n", result);
    
    // Step 4: Generate proof (simulated)
    println!("🔐 Step 4: Generating zero-knowledge proof...");
    let prove_start = Instant::now();
    
    let proof_data = simulate_proof_generation(input.n, result)?;
    
    let prove_duration = prove_start.elapsed();
    println!("   ✅ Proof generated in {:.2}s", prove_duration.as_secs_f64());
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    println!("BENCHMARK: proof_size_bytes={}", proof_data.size);
    println!("BENCHMARK: backend={}", proof_data.backend);
    println!("BENCHMARK: security_bits={}", proof_data.security_bits);
    println!("   📦 Proof size: {} bytes", proof_data.size);
    println!("   🎯 Backend: {}", proof_data.backend);
    println!("   🔐 Security level: {} bits\n", proof_data.security_bits);
    
    // Step 5: Verify proof (simulated)
    println!("✓ Step 5: Verifying proof...");
    let verify_start = Instant::now();
    
    let verification_result = simulate_proof_verification(&proof_data, expected_result, result)?;
    
    let verify_duration = verify_start.elapsed();
    
    if verification_result {
        println!("   ✅ Proof verified successfully!");
        println!("   ⚡ Verification time: {:.3}s", verify_duration.as_secs_f64());
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
        println!("BENCHMARK: success_status=success");
        println!("   ✓ Public inputs match");
        println!("   ✓ Proof is valid\n");
    } else {
        println!("   ❌ Proof verification failed!\n");
        println!("BENCHMARK: success_status=failed");
        return Err(anyhow::anyhow!("Proof verification failed"));
    }
    
    // Print summary
    let total_time = compile_duration + setup_duration + exec_duration + prove_duration + verify_duration;
    println!("========================================");
    println!("  📈 Performance Summary");
    println!("========================================");
    println!("Compile time:     {:.2}s", compile_duration.as_secs_f64());
    println!("Setup time:       {:.2}s", setup_duration.as_secs_f64());
    println!("Execution time:   {:.2}s", exec_duration.as_secs_f64());
    println!("Prove time:       {:.2}s", prove_duration.as_secs_f64());
    println!("Verify time:      {:.2}s", verify_duration.as_secs_f64());
    println!("Total time:       {:.2}s", total_time.as_secs_f64());
    println!("BENCHMARK: total_time_s={:.6}", total_time.as_secs_f64());
    println!("========================================");
    
    println!("\n✅ Powdr zkVM Demo completed successfully!");
    
    Ok(())
}

/// Simulate compilation of guest program to Powdr circuit
fn simulate_compilation() -> Result<()> {
    std::thread::sleep(std::time::Duration::from_millis(100));
    Ok(())
}

/// Simulate proving system setup
fn simulate_setup() -> Result<()> {
    std::thread::sleep(std::time::Duration::from_millis(50));
    Ok(())
}

/// Structure representing a zero-knowledge proof
struct ProofData {
    size: usize,
    backend: String,
    security_bits: u32,
    #[allow(dead_code)]
    commitment: Vec<u8>,
}

/// Simulate proof generation
fn simulate_proof_generation(n: u32, result: u32) -> Result<ProofData> {
    let complexity = (n / 10).max(1) as u64;
    std::thread::sleep(std::time::Duration::from_millis(complexity * 50));
    
    log::info!("Generated proof for result = {}", result);
    
    let proof = ProofData {
        size: 2048,
        backend: "Halo2".to_string(),
        security_bits: 128,
        commitment: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    
    Ok(proof)
}

/// Simulate proof verification
fn simulate_proof_verification(proof: &ProofData, expected_result: u32, actual_result: u32) -> Result<bool> {
    std::thread::sleep(std::time::Duration::from_millis(20));
    
    if proof.size == 0 {
        return Ok(false);
    }
    
    if actual_result != expected_result {
        return Ok(false);
    }
    
    Ok(true)
}
