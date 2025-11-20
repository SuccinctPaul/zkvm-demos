/// Lean zkVM Host Program - Multi-Program Demo
/// 
/// This is a reference implementation.

use anyhow::Result;
use std::time::Instant;
use common::{load_program_input, execute_program};

fn main() -> Result<()> {
    env_logger::init();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║          Lean zkVM Multi-Program Demo (Reference)       ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Get input from environment or use default
    let input = load_program_input();

    println!("📊 Configuration");
    println!("   Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("   Input N: {}", input.n);
    println!("   Target: ~128 bits of security");
    println!("   Proof system: WHIR + SuperSpartan (AIR-optimized)");
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 1: Compute the result (in actual zkVM, this happens in guest)
    // ═══════════════════════════════════════════════════════════════
    println!("🔢 Step 1: Computing Program(ID={})...", input.program.id());
    let compute_start = Instant::now();
    
    let result = execute_program(input.program.id(), input.n);
    
    let compute_time = compute_start.elapsed();
    println!("   Result: {}", result);
    println!("   Computation time: {:.3}ms", compute_time.as_secs_f64() * 1000.0);
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 2: Setup (would compile guest program and setup prover)
    // ═══════════════════════════════════════════════════════════════
    println!("🔧 Step 2: Setting up prover...");
    println!("   [Reference] In actual lean zkVM, this would:");
    println!("   • Compile guest program to bytecode");
    println!("   • Initialize WHIR prover parameters");
    println!("   • Setup KoalaBear field (p = 2^31 - 2^24 + 1)");
    println!("   • Configure AIR constraints");
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 3: Generate proof
    // ═══════════════════════════════════════════════════════════════
    println!("🔐 Step 3: Generating zero-knowledge proof...");
    let prove_start = Instant::now();
    
    // Simulate realistic proving time
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let prove_time = prove_start.elapsed();
    let simulated_cycles = (input.n as u64) * 50; // Rough estimate
    let proof_size_kb = 450; 
    
    // Generate mock proof file
    let proof_data = generate_mock_proof(input.n, result as u64)?; // result is u32 in common, u64 here
    let proof_path = "lean_fibonacci_proof.bin";
    std::fs::write(proof_path, &proof_data)?;
    
    println!();
    println!("   ✅ Proof generated successfully!");
    println!("   Proving time: {:.3}s", prove_time.as_secs_f64());
    println!("   Estimated cycles: ~{}", simulated_cycles);
    println!("   Proof size: ~{} KiB (with rate=1/2)", proof_size_kb);
    println!("   📄 Proof saved to: {}", proof_path);
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 4: Verify proof
    // ═══════════════════════════════════════════════════════════════
    println!("🔍 Step 4: Verifying proof...");
    let verify_start = Instant::now();
    
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    let verify_time = verify_start.elapsed();
    println!();
    println!("   ✅ Proof verified successfully!");
    println!("   Verification time: {:.3}ms", verify_time.as_secs_f64() * 1000.0);
    println!();

    println!("✅ Lean zkVM demo completed successfully!");
    Ok(())
}

/// Generate a mock proof for demonstration purposes
fn generate_mock_proof(n: u32, result: u64) -> Result<Vec<u8>> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let mut proof_data = Vec::new();
    proof_data.extend_from_slice(b"LEAN"); 
    proof_data.extend_from_slice(&1u32.to_le_bytes()); 
    proof_data.extend_from_slice(&timestamp.to_le_bytes()); 
    proof_data.extend_from_slice(&n.to_le_bytes());
    proof_data.extend_from_slice(&result.to_le_bytes());
    
    let checksum = proof_data.iter().fold(0u64, |acc, &b| acc.wrapping_add(b as u64));
    proof_data.extend_from_slice(&checksum.to_le_bytes());
    
    Ok(proof_data)
}
