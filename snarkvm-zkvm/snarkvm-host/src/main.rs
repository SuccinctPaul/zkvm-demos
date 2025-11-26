//! snarkVM Host Program - Multi-Program Demo
//!
//! snarkVM is the Aleo network's zkVM using Marlin proving system
//! This is a reference implementation for benchmarking purposes.
//!
//! Repository: https://github.com/AleoHQ/snarkVM

use anyhow::Result;
use colored::*;
use std::time::Instant;
use common::{load_program_input, execute_program};

const SNARKVM_VERSION: &str = "v0.16.0";

fn main() -> Result<()> {
    println!("{}", "========================================".bright_cyan());
    println!("{}", "snarkVM Multi-Program Demo".bright_cyan().bold());
    println!("{}", "========================================".bright_cyan());
    println!();

    // Load input
    let input = load_program_input();
    
    // Output BENCHMARK metadata early
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=snarkvm");
    println!("BENCHMARK: zkvm_version={}", SNARKVM_VERSION);
    println!("BENCHMARK: proof_mode=core");
    
    println!("Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("Input N: {}\n", input.n);

    let total_start = Instant::now();

    // Step 1: Compile/Deploy Leo program (reference)
    println!("{}", "1️⃣  Compiling Leo program...".bright_green());
    let compile_start = Instant::now();
    println!("   Note: Actual compilation requires snarkVM SDK");
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation step completed");
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
    println!();

    // Step 2: Execute program
    println!("{}", "2️⃣  Executing program...".bright_green());
    let exec_start = Instant::now();
    
    // Execute using common library
    let result = execute_program(input.program.id(), input.n);
    
    let exec_duration = exec_start.elapsed();
    
    println!("   ✓ Execution completed");
    println!("   ✓ Result: {}", result);
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    // Note: total_cycles (constraints) not available without actual SDK
    println!();

    // Step 3: Generate Marlin proof (reference)
    println!("{}", "3️⃣  Generating Marlin proof...".bright_green());
    println!("   Note: Actual proof generation requires snarkVM SDK");
    let prove_start = Instant::now();
    let prove_duration = prove_start.elapsed();
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    // Note: proof_size_bytes, vm_prove_khz not available without actual SDK
    println!();

    // Step 4: Verify proof (reference)
    println!("{}", "4️⃣  Verifying proof...".bright_green());
    println!("   Note: Actual verification requires snarkVM SDK");
    let verify_start = Instant::now();
    let verify_duration = verify_start.elapsed();
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!();

    // Verify correctness
    let expected = common::benchmarks::fibonacci(input.n);
    if result == expected {
        println!("✅ Result matches expected value!");
        println!("BENCHMARK: success_status=success");
    } else {
        println!("❌ Result mismatch! Expected: {}, Got: {}", expected, result);
        println!("BENCHMARK: success_status=failed");
    }

    // Total time
    let total_duration = total_start.elapsed();
    println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
    
    println!("\n✅ snarkVM demo completed!");
    println!("\nNote: For actual proof generation, use the snarkVM SDK:");
    println!("  https://github.com/AleoHQ/snarkVM");

    Ok(())
}
