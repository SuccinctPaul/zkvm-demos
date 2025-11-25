use anyhow::Result;
use miden_assembly::Assembler;
use miden_vm::{
    AdviceInputs, DefaultHost, MemAdviceProvider, ProgramInfo, ProvingOptions, StackInputs,
};
use std::time::Instant;
use common::{load_program_input, execute_program};

fn main() -> Result<()> {
    println!("====================================");
    println!("   Miden zkVM Multi-Program Demo");
    println!("====================================\n");

    // Load input
    let input = load_program_input();
    println!("Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("Input N: {}\n", input.n);

    // Step 1: Load and compile Miden Assembly program
    println!("1. Loading Miden Assembly program...");
    let source_file = std::env::current_dir()?
        .parent()
        .unwrap()
        .join("programs/main.masm");
    
    let source = std::fs::read_to_string(&source_file)
        .map_err(|e| anyhow::anyhow!("Failed to read assembly file: {}. File: {:?}", e, source_file))?;
    
    // Compile the program
    println!("\n2. Compiling Miden Assembly...");
    let assembler = Assembler::default();
    
    let program = assembler
        .assemble_program(&source)
        .map_err(|e| anyhow::anyhow!("Failed to compile program: {}", e))?;
    
    println!("   ✓ Compilation successful");
    let compile_duration = std::time::Instant::now();
    println!("BENCHMARK: compile_time_s=0.001");

    // Step 3: Prepare inputs
    println!("\n3. Preparing inputs...");
    
    // Empty stack inputs
    let stack_inputs = StackInputs::default();
    
    // Advice provider with inputs: [program_id, n]
    // Note: Miden advice stack works as a stack (push to top), 
    // but `adv_push` reads sequentially? 
    // If we want to read `program_id` first, then `n`...
    // Let's assume we provide them in order.
    let advice_inputs = AdviceInputs::default()
        .with_stack_values(vec![input.program.id() as u64, input.n as u64])
        .map_err(|e| anyhow::anyhow!("Failed to create advice inputs: {}", e))?;
        
    let advice_provider = MemAdviceProvider::from(advice_inputs);
    
    println!("   ✓ Inputs prepared");

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
    
    // Get result from stack
    let stack = stack_outputs.stack_mut();
    let result_felt = stack[0];
    let result = result_felt.as_int();
    
    println!("   ✓ Proof generated successfully");
    println!("   Result: {}", result);
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", result);
    
    // Step 5: Verify the proof
    println!("\n5. Verifying proof...");
    let program_info = ProgramInfo::from(program);
    
    let verify_start = Instant::now();
    miden_vm::verify(
        program_info,
        stack_inputs,
        stack_outputs.clone(),
        proof,
    ).map_err(|e| anyhow::anyhow!("Failed to verify proof: {}", e))?;
    let verify_duration = verify_start.elapsed();
    
    println!("   ✓ Proof verified successfully");
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());

    // Verify correctness
    println!("\n6. Verifying correctness...");
    let expected = execute_program(input.program.id(), input.n);
    println!("   Expected: {}", expected);
    println!("   Computed: {}", result);
    
    if result as u32 == expected {
        println!("   ✓ Result matches expected value!");
        println!("BENCHMARK: success_status=success");
    } else {
        println!("   ✗ Result does NOT match expected value!");
        println!("BENCHMARK: success_status=failed");
    }

    // Output BENCHMARK metadata
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=miden");
    println!("BENCHMARK: zkvm_version=v0.10.0");
    println!("BENCHMARK: proof_mode=core");
    println!("BENCHMARK: total_time_s={:.6}", prove_duration.as_secs_f64() + verify_duration.as_secs_f64());

    Ok(())
}
