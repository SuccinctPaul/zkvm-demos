use anyhow::Result;
use miden_assembly::Assembler;
use miden_vm::{
    AdviceInputs, DefaultHost, MemAdviceProvider, ProgramInfo, ProvingOptions, StackInputs,
};
use std::time::Instant;

fn main() -> Result<()> {
    println!("====================================");
    println!("   Miden zkVM Fibonacci Demo");
    println!("====================================\n");

    // Load Fibonacci input
    let fib_n = common::load_fib_n();
    println!("Computing Fibonacci number for n = {}\n", fib_n);

    // Step 1: Load and compile Miden Assembly program
    println!("1. Loading Miden Assembly program...");
    let source_file = std::env::current_dir()?
        .parent()
        .unwrap()
        .join("programs/fib_working.masm");
    
    let source = std::fs::read_to_string(&source_file)
        .map_err(|e| anyhow::anyhow!("Failed to read assembly file: {}. File: {:?}", e, source_file))?;
    
    println!("   Source file: {:?}", source_file);
    
    // Compile the program
    println!("\n2. Compiling Miden Assembly...");
    let compile_start = Instant::now();
    let assembler = Assembler::default();
    
    let program = assembler
        .assemble_program(&source)
        .map_err(|e| anyhow::anyhow!("Failed to compile program: {}", e))?;
    
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation successful");
    println!("   Compile time: {:.3}s", compile_duration.as_secs_f64());
    println!("   Program hash: {}", program.hash());

    // Step 3: Prepare inputs
    println!("\n3. Preparing inputs...");
    
    // Empty stack inputs - Miden requires exactly 16 elements or less
    let stack_inputs = StackInputs::default();
    
    // Empty advice provider for now
    let advice_provider = MemAdviceProvider::default();
    
    println!("   ✓ Inputs prepared for n = {}", fib_n);

    // Step 4: Execute and prove
    println!("\n4. Executing program and generating proof...");
    let prove_start = Instant::now();
    
    let host = DefaultHost::new(advice_provider);
    let options = ProvingOptions::default();
    
    let (mut stack_outputs, proof) = miden_vm::prove(
        &program,
        stack_inputs.clone(),
        host,
        options,
    ).map_err(|e| anyhow::anyhow!("Failed to prove program: {}", e))?;
    
    let prove_duration = prove_start.elapsed();
    
    // Get result from stack (get the top element, index 0)
    let stack = stack_outputs.stack_mut();
    let result_felt = stack[0];
    let result = result_felt.as_int();
    
    println!("   ✓ Proof generated successfully");
    println!("   Execution time: {:.3}s", prove_duration.as_secs_f64());
    println!("   Result: fib({}) = {}", fib_n, result);
    println!("   Proof size: {} bytes", proof.to_bytes().len());

    // Step 5: Verify the proof
    println!("\n5. Verifying proof...");
    let verify_start = Instant::now();
    
    let program_info = ProgramInfo::from(program);
    
    miden_vm::verify(
        program_info,
        stack_inputs,
        stack_outputs.clone(),
        proof,
    ).map_err(|e| anyhow::anyhow!("Failed to verify proof: {}", e))?;
    
    let verify_duration = verify_start.elapsed();
    
    println!("   ✓ Proof verified successfully");
    println!("   Verification time: {:.3}s", verify_duration.as_secs_f64());

    // Verify correctness
    println!("\n6. Verifying correctness...");
    let expected = compute_fibonacci(fib_n);
    println!("   Expected: fib({}) = {}", fib_n, expected);
    println!("   Computed: fib({}) = {}", fib_n, result);
    
    if result == expected {
        println!("   ✓ Result matches expected value!");
    } else {
        println!("   ✗ Result does NOT match expected value!");
    }

    println!("\n====================================");
    println!("   Demo completed successfully!");
    println!("====================================\n");

    Ok(())
}

/// Compute Fibonacci number for verification
fn compute_fibonacci(n: u32) -> u64 {
    if n < 2 {
        1
    } else {
        let mut prev = 1u64;
        let mut curr = 1u64;
        for _ in 1..n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        curr
    }
}
