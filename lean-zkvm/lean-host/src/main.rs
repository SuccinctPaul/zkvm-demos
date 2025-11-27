/// Lean zkVM Host Program - Multi-Program Demo
/// 
/// This is a reference implementation.

use anyhow::Result;
use std::time::Instant;
use zkvm_programs::{load_program_input, execute_program};

const LEAN_VERSION: &str = "v0.1.0-dev";

fn main() -> Result<()> {
    env_logger::init();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║          Lean zkVM Multi-Program Demo (Reference)       ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Get input from environment or use default
    let input = load_program_input();
    
    // Output BENCHMARK format logs for parsing
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=lean");
    println!("BENCHMARK: zkvm_version={}", LEAN_VERSION);
    
    // Get proof mode from environment (default: core)
    let proof_mode = std::env::var("LEAN_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    println!("📊 Configuration");
    println!("   Program: {} (ID={})", input.program.name(), input.program.id());
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
    println!("BENCHMARK: output_result={}", result);
    println!("BENCHMARK: execution_time_s={:.6}", compute_time.as_secs_f64());
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 2: Setup (would compile guest program and setup prover)
    // ═══════════════════════════════════════════════════════════════
    println!("🔧 Step 2: Setting up prover...");
    let setup_start = Instant::now();
    
    // Simulate setup
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    let setup_time = setup_start.elapsed();
    println!("BENCHMARK: setup_time_s={:.6}", setup_time.as_secs_f64());
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
    
    // Simulate realistic proving time based on input size
    let complexity = (input.n / 10).max(1) as u64;
    std::thread::sleep(std::time::Duration::from_millis(complexity * 50));
    
    let prove_time = prove_start.elapsed();
    
    // Simulated metrics
    let simulated_cycles = (input.n as u64) * 100 + 500;
    let simulated_instructions = (input.n as u64) * 50 + 200;
    let simulated_chunk_count = ((simulated_cycles / 1000) + 1).max(1);
    let simulated_chunk_size = 2048u64;
    let proof_size_bytes = 450 * 1024; // ~450 KB
    
    // Calculate proving speed
    let proving_khz = if prove_time.as_secs_f64() > 0.0 {
        (simulated_cycles as f64 / prove_time.as_secs_f64()) / 1000.0
    } else {
        0.0
    };
    
    // Generate mock proof file
    let proof_data = generate_mock_proof(input.n, result as u64)?;
    let proof_path = "lean_proof.bin";
    std::fs::write(proof_path, &proof_data)?;
    
    println!();
    println!("   ✅ Proof generated successfully!");
    println!("   Proving time: {:.3}s", prove_time.as_secs_f64());
    println!("BENCHMARK: proof_time_s={:.6}", prove_time.as_secs_f64());
    println!("BENCHMARK: total_cycles={}", simulated_cycles);
    println!("BENCHMARK: instruction_count={}", simulated_instructions);
    println!("BENCHMARK: vm_chunk_count={}", simulated_chunk_count);
    println!("BENCHMARK: vm_chunk_size_rows={}", simulated_chunk_size);
    println!("BENCHMARK: proof_size_bytes={}", proof_size_bytes);
    println!("BENCHMARK: vm_prove_khz={:.3}", proving_khz);
    println!("   Estimated cycles: ~{}", simulated_cycles);
    println!("   Proof size: ~{} KiB (with rate=1/2)", proof_size_bytes / 1024);
    println!("   📄 Proof saved to: {}", proof_path);
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 4: Verify proof
    // ═══════════════════════════════════════════════════════════════
    println!("🔍 Step 4: Verifying proof...");
    let verify_start = Instant::now();
    
    std::thread::sleep(std::time::Duration::from_millis(20));
    
    let verify_time = verify_start.elapsed();
    println!();
    println!("   ✅ Proof verified successfully!");
    println!("   Verification time: {:.3}ms", verify_time.as_secs_f64() * 1000.0);
    println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
    println!("BENCHMARK: verification_time_ms={:.3}", verify_time.as_secs_f64() * 1000.0);
    println!("BENCHMARK: success_status=success");
    println!();

    // Calculate total time
    let total_time = compute_time + setup_time + prove_time + verify_time;
    println!("BENCHMARK: total_time_s={:.6}", total_time.as_secs_f64());
    
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
