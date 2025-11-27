use anyhow::Result;
use miden_assembly::Assembler;
use miden_vm::{
    DefaultHost, MemAdviceProvider, ProgramInfo, ProvingOptions, StackInputs, AdviceInputs,
};
use miden_processor::ExecutionOptions;
use std::time::Instant;
use zkvm_programs::{load_program_input, execute_program};

const MIDEN_VERSION: &str = "v0.10.0";

fn main() -> Result<()> {
    println!("====================================");
    println!("   Miden zkVM Multi-Program Demo");
    println!("====================================\n");

    // Load input
    let input = load_program_input();
    
    // Output BENCHMARK metadata early
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=miden");
    println!("BENCHMARK: zkvm_version={}", MIDEN_VERSION);
    println!("BENCHMARK: proof_mode=core");
    
    println!("Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("Input N: {}\n", input.n);

    // Track total time
    let total_start = Instant::now();

    // Step 1: Load and compile Miden Assembly program
    println!("1. Loading Miden Assembly program...");
    let compile_start = Instant::now();
    
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
    
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation successful");
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());

    // Step 3: Prepare inputs using STACK inputs (not advice)
    println!("\n3. Preparing inputs...");
    
    // Use stack inputs directly - Miden expects inputs in reverse order on stack
    let stack_values: Vec<u64> = vec![input.n as u64, input.program.id() as u64];
    let stack_inputs = StackInputs::try_from_ints(stack_values)
        .map_err(|e| anyhow::anyhow!("Failed to create stack inputs: {}", e))?;
    
    // Empty advice provider
    let advice_provider = MemAdviceProvider::from(AdviceInputs::default());
    
    println!("   ✓ Inputs prepared (on stack)");

    // Step 4a: First execute to get trace info (cycles)
    println!("\n4. Executing program...");
    let exec_start = Instant::now();
    
    let host_for_exec = DefaultHost::new(advice_provider.clone());
    let exec_options = ExecutionOptions::default();
    
    let execution_result = miden_vm::execute(
        &program,
        stack_inputs.clone(),
        host_for_exec,
        exec_options,
    );
    
    let exec_duration = exec_start.elapsed();
    
    match &execution_result {
        Ok(trace) => {
            // Get trace length (represents execution cycles)
            let trace_len = trace.get_trace_len();
            println!("   ✓ Execution completed");
            println!("   Trace length (cycles): {}", trace_len);
            println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
            println!("BENCHMARK: total_cycles={}", trace_len);
        }
        Err(e) => {
            println!("   ✗ Execution failed: {}", e);
            println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
            println!("BENCHMARK: success_status=failed");
            println!("BENCHMARK: error_message={}", e.to_string().replace('\n', " "));
            
            let total_duration = total_start.elapsed();
            println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
            
            return Err(anyhow::anyhow!("Failed to execute program: {}", e));
        }
    }

    // Step 4b: Now prove
    println!("\n5. Generating proof...");
    let prove_start = Instant::now();
    
    let host = DefaultHost::new(advice_provider);
    let options = ProvingOptions::default();
    
    let prove_result = miden_vm::prove(
        &program,
        stack_inputs.clone(),
        host,
        options,
    );
    
    let prove_duration = prove_start.elapsed();
    
    match prove_result {
        Ok((mut stack_outputs, proof)) => {
            println!("   ✓ Proof generated successfully");
            println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
            
            // Get proof size
            let proof_bytes = proof.to_bytes();
            let proof_size = proof_bytes.len();
            println!("BENCHMARK: proof_size_bytes={}", proof_size);
            
            // Calculate proving speed (kHz)
            if let Ok(trace) = &execution_result {
                let trace_len = trace.get_trace_len() as f64;
                let prove_khz = trace_len / (prove_duration.as_secs_f64() * 1000.0);
                println!("BENCHMARK: vm_prove_khz={:.3}", prove_khz);
            }
            
            // Get result from stack
            let stack = stack_outputs.stack_mut();
            let result_felt = stack[0];
            let result = result_felt.as_int();
            
            println!("   Result: {}", result);
            println!("BENCHMARK: output_result={}", result);
            
            // Step 6: Verify the proof
            println!("\n6. Verifying proof...");
            let program_info = ProgramInfo::from(program);
            
            let verify_start = Instant::now();
            let verify_result = miden_vm::verify(
                program_info,
                stack_inputs,
                stack_outputs.clone(),
                proof,
            );
            let verify_duration = verify_start.elapsed();
            
            match verify_result {
                Ok(_) => {
                    println!("   ✓ Proof verified successfully");
                    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
                    println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
                    
                    // Verify correctness
                    println!("\n7. Verifying correctness...");
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
                }
                Err(e) => {
                    println!("   ✗ Verification failed: {}", e);
                    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
                    println!("BENCHMARK: success_status=failed");
                }
            }
            
            // Calculate total time
            let total_duration = total_start.elapsed();
            println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
        }
        Err(e) => {
            println!("   ✗ Proof generation failed: {}", e);
            println!("BENCHMARK: proof_time_s=0.0");
            println!("BENCHMARK: success_status=failed");
            println!("BENCHMARK: error_message={}", e.to_string().replace('\n', " "));
            
            let total_duration = total_start.elapsed();
            println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
            
            return Err(anyhow::anyhow!("Failed to prove program: {}", e));
        }
    }

    Ok(())
}
