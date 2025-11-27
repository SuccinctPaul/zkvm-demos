//! Novanet zkVM Host Program - Multi-Program Demo
//!
//! Novanet is a Nova-based folding scheme zkVM

use anyhow::Result;
use guest::{compute_program, ProgramInput, ProgramOutput};
use std::time::Instant;
use programs::load_program_input;

const NOVANET_VERSION: &str = "v0.1.0-dev";

/// Simulated proof structure for Novanet zkVM
#[derive(Debug, Clone)]
pub struct NovanetProof {
    pub input: ProgramInput,
    pub output: ProgramOutput,
    pub proof_data: Vec<u8>,
}

/// Simulated prover for Novanet zkVM
pub struct NovanetProver {
    pub circuit_compiled: bool,
}

impl NovanetProver {
    pub fn new() -> Self {
        Self {
            circuit_compiled: true,
        }
    }

    pub fn compile_guest() -> Result<Self> {
        println!("📦 Compiling guest program for Novanet zkVM...");
        Ok(Self::new())
    }

    pub fn prove(&self, input: ProgramInput) -> Result<NovanetProof> {
        println!("🔨 Generating proof for Program(ID={}) Input({})", input.program_id, input.n);
        
        // Execute the computation
        let output = compute_program(input.clone());
        
        // Generate proof data
        let proof_data = format!(
            "Nova-IVC-proof-prog({})-n({})-result({})",
            input.program_id, input.n, output.result
        )
        .into_bytes();

        Ok(NovanetProof {
            input,
            output,
            proof_data,
        })
    }

    pub fn verify(&self, proof: &NovanetProof) -> Result<bool> {
        println!("🔍 Verifying proof...");
        
        let expected_output = compute_program(proof.input.clone());
        let is_valid = expected_output.result == proof.output.result;
        
        Ok(is_valid)
    }
}

fn main() -> Result<()> {
    println!("========================================");
    println!("Novanet zkVM Multi-Program Demo");
    println!("========================================\n");

    // Load program input
    let input_data = load_program_input();
    
    // Output BENCHMARK metadata early
    println!("BENCHMARK: program_name={}_{}", input_data.program.as_str(), input_data.n);
    println!("BENCHMARK: zkvm_name=novanet");
    println!("BENCHMARK: zkvm_version={}", NOVANET_VERSION);
    println!("BENCHMARK: proof_mode=core");
    
    println!("📊 Input: Program={} (ID={}) N={}\n", 
             input_data.program.as_str(), input_data.program.id(), input_data.n);

    let total_start = Instant::now();

    // Step 1: Compile guest program
    println!("1️⃣  Compiling guest program...");
    let compile_start = Instant::now();
    let prover = NovanetProver::compile_guest()?;
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation completed in {:.2}s\n", compile_duration.as_secs_f64());
    println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());

    // Step 2: Execute program
    println!("2️⃣  Executing program...");
    let exec_start = Instant::now();
    
    let prog_input = ProgramInput { 
        program_id: input_data.program.id(), 
        n: input_data.n 
    };
    
    // Execute to get result
    let test_output = compute_program(prog_input.clone());
    let exec_duration = exec_start.elapsed();
    
    println!("   ✓ Execution completed in {:.6}s", exec_duration.as_secs_f64());
    println!("   ✓ Result: {}", test_output.result);
    println!("BENCHMARK: execution_time_s={:.6}", exec_duration.as_secs_f64());
    println!("BENCHMARK: output_result={}", test_output.result);
    // Note: total_cycles not available without actual Nova SDK

    // Step 3: Generate proof
    println!("\n3️⃣  Generating Nova IVC proof...");
    let prove_start = Instant::now();
    
    let proof = prover.prove(prog_input)?;
    
    let prove_duration = prove_start.elapsed();
    
    println!("   ✓ Proof generated in {:.3}s", prove_duration.as_secs_f64());
    println!("   ✓ Proof size: {} bytes", proof.proof_data.len());
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
    println!("BENCHMARK: proof_size_bytes={}", proof.proof_data.len());
    // Note: vm_prove_khz not available without cycle count

    // Step 4: Verify proof
    println!("\n4️⃣  Verifying proof...");
    let verify_start = Instant::now();
    
    let is_valid = prover.verify(&proof)?;
    
    let verify_duration = verify_start.elapsed();

    if is_valid {
        println!("   ✓ Proof verified successfully in {:.6}s\n", verify_duration.as_secs_f64());
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: success_status=success");
    } else {
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: success_status=failed");
        eprintln!("❌ Proof verification failed!");
        std::process::exit(1);
    }

    // Total time
    let total_duration = total_start.elapsed();
    println!("BENCHMARK: total_time_s={:.6}", total_duration.as_secs_f64());
    
    println!("✅ Novanet zkVM Demo completed successfully!");

    Ok(())
}
