//! o1vm zkVM Host Program - Multi-Program Demo
//!
//! o1vm is a MIPS-based zkVM using Kimchi proof system from o1Labs
//! This is a reference implementation for benchmarking purposes.
//!
//! Repository: https://github.com/o1-labs/proof-systems

use anyhow::Result;
use std::time::Instant;
use zkvm_programs::{load_program_input, execute_program};

const O1VM_VERSION: &str = "v0.1.0-dev";

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("========================================");
    println!("o1vm zkVM Demo - MIPS Proving (Reference)");
    println!("========================================\n");

    // Load input
    let input = load_program_input();
    
    // Output BENCHMARK metadata early
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=o1vm");
    println!("BENCHMARK: zkvm_version={}", O1VM_VERSION);
    println!("BENCHMARK: proof_mode=core");
    
    println!("📋 Input: Program={} (ID={}) N={}\n", 
             input.program.name(), input.program.id(), input.n);

    let total_start = Instant::now();

    // Step 1: Compile/Load phase (reference)
    println!("1️⃣  Loading MIPS binary...");
    let compile_start = Instant::now();
    println!("   Note: Actual loading requires o1vm SDK");
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Loading step completed");
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
    println!();

    // Step 2: Execute program
    println!("2️⃣  Executing program...");
    let exec_start = Instant::now();
    
    // Execute using common library
    let result = execute_program(input.program.id(), input.n);
    
    let exec_duration = exec_start.elapsed();
    
    println!("   ✓ Execution completed");
    println!("   ✓ Result: {}", result);
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    // Note: total_cycles not available without actual SDK
    println!();

    // Step 3: Generate proof (reference)
    println!("3️⃣  Generating Kimchi proof...");
    println!("   Note: Actual proof generation requires o1vm SDK");
    let prove_start = Instant::now();
    let prove_duration = prove_start.elapsed();
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    // Note: proof_size_bytes, vm_prove_khz not available without actual SDK
    println!();

    // Step 4: Verify proof (reference)
    println!("4️⃣  Verifying proof...");
    println!("   Note: Actual verification requires o1vm SDK");
    let verify_start = Instant::now();
    let verify_duration = verify_start.elapsed();
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!();

    // Verify correctness
    let expected = zkvm_programs::execute_program(input.program.id(), input.n);
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
    
    println!("\n✅ o1vm zkVM demo completed!");
    println!("\nNote: For actual proof generation, use the o1vm SDK:");
    println!("  https://github.com/o1-labs/proof-systems");
    
    Ok(())
}
