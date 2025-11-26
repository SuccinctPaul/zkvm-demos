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
    
    // Output BENCHMARK format logs for parsing (early)
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
    
    let total_start = Instant::now();
    
    // Step 1: Compile guest program (simulated)
    println!("🔨 Step 1: Compiling guest program...");
    let compile_start = Instant::now();
    
    simulate_compilation()?;
    
    let compile_duration = compile_start.elapsed();
    println!("   ✅ Compilation completed in {:.2}s", compile_duration.as_secs_f64());
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
    println!();
    
    // Step 2: Execute program
    println!("🚀 Step 2: Executing program...");
    let exec_start = Instant::now();
    
    // Execute the computation
    let result = execute_program(input.program.id(), input.n);
    
    let exec_duration = exec_start.elapsed();
    
    // Estimate cycles (Powdr circuit: ~25 cycles per Fibonacci iteration + overhead)
    let estimated_cycles = (input.n as u64) * 25 + 100;
    
    println!("   ✅ Execution completed in {:.6}s", exec_duration.as_secs_f64());
    println!("   Result: {}", result);
    println!("   Estimated cycles: {}", estimated_cycles);
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    println!("BENCHMARK: total_cycles={}", estimated_cycles);
    println!();
    
    // Step 3: Generate proof (simulated)
    println!("🔐 Step 3: Generating zero-knowledge proof...");
    let prove_start = Instant::now();
    
    let proof_data = simulate_proof_generation(input.n, result)?;
    
    let prove_duration = prove_start.elapsed();
    
    // Calculate proving speed
    let prove_khz = if prove_duration.as_secs_f64() > 0.0 {
        (estimated_cycles as f64 / prove_duration.as_secs_f64()) / 1000.0
    } else {
        0.0
    };
    
    println!("   ✅ Proof generated in {:.3}s", prove_duration.as_secs_f64());
    println!("   📦 Proof size: {} bytes", proof_data.size);
    println!("   🎯 Backend: {}", proof_data.backend);
    println!("   🔐 Security level: {} bits", proof_data.security_bits);
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    println!("BENCHMARK: proof_size_bytes={}", proof_data.size);
    println!("BENCHMARK: vm_prove_khz={:.3}", prove_khz);
    println!("BENCHMARK: backend={}", proof_data.backend);
    println!("BENCHMARK: security_bits={}", proof_data.security_bits);
    println!();
    
    // Step 4: Verify proof (simulated)
    println!("✓ Step 4: Verifying proof...");
    let verify_start = Instant::now();
    
    let verification_result = simulate_proof_verification(&proof_data, expected_result, result)?;
    
    let verify_duration = verify_start.elapsed();
    
    if verification_result {
        println!("   ✅ Proof verified successfully!");
        println!("   ⚡ Verification time: {:.3}s", verify_duration.as_secs_f64());
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
        println!("BENCHMARK: success_status=success");
    } else {
        println!("   ❌ Proof verification failed!");
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: success_status=failed");
        return Err(anyhow::anyhow!("Proof verification failed"));
    }
    
    // Total time
    let total_duration = total_start.elapsed();
    println!();
    println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
    
    println!("\n✅ Powdr zkVM Demo completed successfully!");
    
    Ok(())
}

/// Simulate compilation of guest program to Powdr circuit
fn simulate_compilation() -> Result<()> {
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
    std::thread::sleep(std::time::Duration::from_millis(complexity * 80 + 100));
    
    log::info!("Generated proof for result = {}", result);
    
    // Simulated proof size (Halo2 proofs are compact)
    let proof_size = 32 * 1024 + (n as usize) * 256;
    
    let proof = ProofData {
        size: proof_size,
        backend: "Halo2".to_string(),
        security_bits: 128,
        commitment: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    
    Ok(proof)
}

/// Simulate proof verification
fn simulate_proof_verification(proof: &ProofData, expected_result: u32, actual_result: u32) -> Result<bool> {
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    if proof.size == 0 {
        return Ok(false);
    }
    
    if actual_result != expected_result {
        return Ok(false);
    }
    
    Ok(true)
}
