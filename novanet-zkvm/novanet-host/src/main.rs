//! Novanet zkVM Host Program
//!
//! This host program orchestrates the proving and verification
//! of Fibonacci computation using Novanet zkVM.

use anyhow::Result;
use guest::{compute_fibonacci, FibInput, FibOutput};
use std::time::Instant;

/// Simulated proof structure for Novanet zkVM
/// In a real implementation, this would use Novanet's proof system
#[derive(Debug, Clone)]
pub struct NovanetProof {
    pub input: FibInput,
    pub output: FibOutput,
    pub proof_data: Vec<u8>,
}

/// Simulated prover for Novanet zkVM
/// In a real implementation, this would use Nova proof system
pub struct NovanetProver {
    pub circuit_compiled: bool,
}

impl NovanetProver {
    pub fn new() -> Self {
        Self {
            circuit_compiled: true,
        }
    }

    /// Compile the guest program into a circuit
    pub fn compile_guest() -> Result<Self> {
        println!("📦 Compiling guest program for Novanet zkVM...");
        // In real implementation: compile guest code to Novanet circuit
        Ok(Self::new())
    }

    /// Generate a proof for the computation
    pub fn prove(&self, input: FibInput) -> Result<NovanetProof> {
        println!("🔨 Generating proof for fibonacci({})...", input.n);
        
        // Execute the computation
        let output = compute_fibonacci(input.clone());
        
        // In real implementation: generate Nova proof
        // This would use Nova's IVC (Incrementally Verifiable Computation)
        let proof_data = format!(
            "Nova-proof-fib({})-result({})",
            input.n, output.result
        )
        .into_bytes();

        Ok(NovanetProof {
            input,
            output,
            proof_data,
        })
    }

    /// Verify a proof
    pub fn verify(&self, proof: &NovanetProof) -> Result<bool> {
        println!("🔍 Verifying proof...");
        
        // In real implementation: verify Nova proof
        // This would check the Nova proof's correctness
        
        // For simulation, we recompute and check
        let expected_output = compute_fibonacci(proof.input.clone());
        let is_valid = expected_output.result == proof.output.result;
        
        Ok(is_valid)
    }
}

fn main() -> Result<()> {
    println!("========================================");
    println!("Novanet zkVM Demo - Fibonacci Computation");
    println!("========================================\n");
    println!("⚠️  NOTE: This is a demonstration implementation.");
    println!("   A production Novanet zkVM would use the Nova");
    println!("   proof system for recursive SNARKs.\n");

    // Load fibonacci number from environment
    let fib_n = common::load_fib_n();
    println!("📊 Computing fibonacci({})...\n", fib_n);

    // Step 1: Compile guest program
    println!("1️⃣  Compiling guest program...");
    let compile_start = Instant::now();
    let prover = NovanetProver::compile_guest()?;
    let compile_duration = compile_start.elapsed();
    println!(
        "   ✓ Compilation completed in {:.2}s\n",
        compile_duration.as_secs_f64()
    );

    // Step 2: Setup (in Nova, this would setup the circuit)
    println!("2️⃣  Setting up proving system...");
    let setup_start = Instant::now();
    // In real Nova implementation: setup would create public parameters
    let setup_duration = setup_start.elapsed();
    println!(
        "   ✓ Setup completed in {:.2}s\n",
        setup_duration.as_secs_f64()
    );

    // Step 3: Generate proof
    println!("3️⃣  Generating proof...");
    let prove_start = Instant::now();
    
    let input = FibInput { n: fib_n };
    let proof = prover.prove(input)?;
    
    let prove_duration = prove_start.elapsed();
    println!(
        "   ✓ Proof generated in {:.2}s",
        prove_duration.as_secs_f64()
    );
    println!("   ✓ Result: fibonacci({}) = {}", fib_n, proof.output.result);
    println!(
        "   ✓ Proof size: {} bytes\n",
        proof.proof_data.len()
    );

    // Step 4: Verify proof
    println!("4️⃣  Verifying proof...");
    let verify_start = Instant::now();
    
    let is_valid = prover.verify(&proof)?;
    
    let verify_duration = verify_start.elapsed();

    if is_valid {
        println!(
            "   ✓ Proof verified successfully in {:.2}s\n",
            verify_duration.as_secs_f64()
        );

        println!("========================================");
        println!("📈 Performance Summary");
        println!("========================================");
        println!(
            "Compile time:     {:.2}s",
            compile_duration.as_secs_f64()
        );
        println!("Setup time:       {:.2}s", setup_duration.as_secs_f64());
        println!("Prove time:       {:.2}s", prove_duration.as_secs_f64());
        println!("Verify time:      {:.2}s", verify_duration.as_secs_f64());
        println!(
            "Total time:       {:.2}s",
            (compile_duration + setup_duration + prove_duration + verify_duration)
                .as_secs_f64()
        );
        println!("========================================");
        println!("✅ Novanet zkVM Demo completed successfully!");
        println!("\n💡 About Novanet zkVM:");
        println!("   - Based on Nova proof system (recursive SNARKs)");
        println!("   - Supports incrementally verifiable computation (IVC)");
        println!("   - Efficient for iterative computations");
        println!("\n💡 Tip: Try running with different FIBONACCI_N values!");
        println!("   Example: FIBONACCI_N=15 cargo run --release");
    } else {
        eprintln!("❌ Proof verification failed!");
        eprintln!("This should not happen with a correctly generated proof.");
        std::process::exit(1);
    }

    Ok(())
}

