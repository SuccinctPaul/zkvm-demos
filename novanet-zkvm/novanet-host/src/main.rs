//! Novanet zkVM Host Program - Multi-Program Demo

use anyhow::Result;
use guest::{compute_program, ProgramInput, ProgramOutput};
use std::time::Instant;
use common::load_program_input;

// Alias guest crate removed as it is already named 'guest' in Cargo.toml

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
        
        let proof_data = format!(
            "Nova-proof-prog({})-n({})-result({})",
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
    println!("📊 Input: Program={} (ID={}) N={}\n", 
             input_data.program.as_str(), input_data.program.id(), input_data.n);

    // Step 1: Compile guest program
    println!("1️⃣  Compiling guest program...");
    let compile_start = Instant::now();
    let prover = NovanetProver::compile_guest()?;
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation completed in {:.2}s\n", compile_duration.as_secs_f64());

    // Step 2: Setup
    println!("2️⃣  Setting up proving system...");
    let setup_start = Instant::now();
    let setup_duration = setup_start.elapsed();
    println!("   ✓ Setup completed in {:.2}s\n", setup_duration.as_secs_f64());

    // Step 3: Generate proof
    println!("3️⃣  Generating proof...");
    let prove_start = Instant::now();
    
    let prog_input = ProgramInput { 
        program_id: input_data.program.id(), 
        n: input_data.n 
    };
    let proof = prover.prove(prog_input)?;
    
    let prove_duration = prove_start.elapsed();
    println!("   ✓ Proof generated in {:.2}s", prove_duration.as_secs_f64());
    println!("   ✓ Result: {}", proof.output.result);
    println!("   ✓ Proof size: {} bytes\n", proof.proof_data.len());

    // Step 4: Verify proof
    println!("4️⃣  Verifying proof...");
    let verify_start = Instant::now();
    
    let is_valid = prover.verify(&proof)?;
    
    let verify_duration = verify_start.elapsed();

    if is_valid {
        println!("   ✓ Proof verified successfully in {:.2}s\n", verify_duration.as_secs_f64());
        println!("✅ Novanet zkVM Demo completed successfully!");
    } else {
        eprintln!("❌ Proof verification failed!");
        std::process::exit(1);
    }

    Ok(())
}
