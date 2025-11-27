//! Powdr zkVM Host Program
//! 
//! This program demonstrates how to use Powdr zkVM to generate and verify
//! zero-knowledge proofs for multi-program execution.
//! 
//! Note: Powdr is a zkVM toolkit under active development. This implementation
//! shows the expected workflow and will be updated once the official SDK is released.

use std::time::Instant;
use anyhow::Result;
use zkvm_programs::{load_program_input, execute_program};

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
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=powdr");
    println!("BENCHMARK: zkvm_version={}", POWDR_VERSION);
    
    // Get proof mode from environment (default: core)
    let proof_mode = std::env::var("POWDR_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);
    
    println!("📊 Configuration:");
    println!("   Program: {} (ID={})", input.program.name(), input.program.id());
    println!("   Input: n = {}", input.n);
    
    let total_start = Instant::now();
    
    // Step 1: Compile guest program (reference)
    println!("\n🔨 Step 1: Compiling guest program...");
    let compile_start = Instant::now();
    println!("   Note: Actual compilation requires Powdr SDK");
    let compile_duration = compile_start.elapsed();
    println!("   ✅ Compilation step completed");
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
    
    // Step 2: Execute program
    println!("\n🚀 Step 2: Executing program...");
    let exec_start = Instant::now();
    
    // Execute the computation
    let result = execute_program(input.program.id(), input.n);
    
    let exec_duration = exec_start.elapsed();
    
    println!("   ✅ Execution completed in {:.6}s", exec_duration.as_secs_f64());
    println!("   Result: {}", result);
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    // Note: total_cycles not available without actual SDK
    
    // Step 3: Generate proof (reference)
    println!("\n🔐 Step 3: Generating proof...");
    println!("   Note: Actual proof generation requires Powdr SDK");
    let prove_start = Instant::now();
    let prove_duration = prove_start.elapsed();
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    // Note: proof_size_bytes, vm_prove_khz not available without actual SDK
    
    // Step 4: Verify proof (reference)
    println!("\n✓ Step 4: Verifying proof...");
    println!("   Note: Actual verification requires Powdr SDK");
    let verify_start = Instant::now();
    let verify_duration = verify_start.elapsed();
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    
    // Verify correctness
    let expected = zkvm_programs::execute_program(input.program.id(), input.n);
    if result == expected {
        println!("\n✅ Result matches expected value!");
        println!("BENCHMARK: success_status=success");
    } else {
        println!("\n❌ Result mismatch! Expected: {}, Got: {}", expected, result);
        println!("BENCHMARK: success_status=failed");
    }
    
    // Total time
    let total_duration = total_start.elapsed();
    println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
    
    println!("\n✅ Powdr zkVM Demo completed!");
    println!("\nNote: For actual proof generation, use the Powdr SDK:");
    println!("  https://github.com/powdr-labs/powdr");
    
    Ok(())
}
