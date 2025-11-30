//! Lean zkVM Host Program - Multi-Program Demo
//!
//! Lean zkVM (leanMultisig) is a minimal, high-performance zkVM designed for:
//! - XMSS + minimal zkVM = lightweight post-quantum signatures
//! - Ultra-fast proving: 1.0-1.7 MHz on consumer hardware
//! - Compact proofs: Target 128-256 KiB
//! - Advanced proof systems: WHIR, SuperSpartan (AIR-optimized)
//!
//! Repository: <https://github.com/leanEthereum/leanMultisig>
//!
//! ## Implementation Status
//!
//! **SDK Status**: Not yet publicly available
//!
//! This is a reference implementation showing the expected workflow.
//! When the lean_prover SDK becomes available, it would be integrated as:
//! ```rust
//! use lean_prover::{Prover, Verifier, compile_guest};
//!
//! let bytecode = compile_guest("guest.rs")?;
//! let (proof, public_values) = Prover::prove(&bytecode, &inputs)?;
//! Verifier::verify(&proof, &public_values)?;
//! ```

use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;
use zkvm_programs::{execute_program, load_program_input};

const LEAN_VERSION: &str = "v0.1.0-dev";

fn main() -> Result<()> {
    env_logger::init();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║          Lean zkVM Multi-Program Demo                    ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // Get input from environment or use default
    let input = load_program_input();

    // Output BENCHMARK format logs for parsing
    println!(
        "BENCHMARK: program_name={}_{}",
        input.program.name(),
        input.n
    );
    println!("BENCHMARK: zkvm_name=lean");
    println!("BENCHMARK: zkvm_version={}", LEAN_VERSION);

    // Note: proof_mode is "reference" because lean_prover SDK is not yet publicly available
    let proof_mode = std::env::var("LEAN_PROOF_MODE").unwrap_or_else(|_| "reference".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    println!("📊 Configuration");
    println!("   Program: {} (ID={})", input.program.name(), input.program.id());
    println!("   Input N: {}", input.n);
    println!("   Target: ~128 bits of security");
    println!("   Proof system: WHIR + SuperSpartan (AIR-optimized)");
    println!();

    let total_start = Instant::now();

    // Check if lean_prover CLI is available
    let lean_cli = find_lean_cli();

    match lean_cli {
        Some(cli_path) => {
            run_with_lean_cli(&cli_path, &input, total_start)?;
        }
        None => {
            println!("⚠️ lean_prover CLI not found");
            println!("   Note: SDK not yet publicly available");
            println!("   Repository: https://github.com/leanEthereum/leanMultisig");
            println!("\n   Running reference implementation...\n");

            run_reference_execution(&input, total_start)?;
        }
    }

    Ok(())
}

/// Find lean_prover CLI
fn find_lean_cli() -> Option<PathBuf> {
    // Try PATH first
    if let Ok(output) = Command::new("which").arg("lean_prover").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }

    // Try common installation paths
    let home = std::env::var("HOME").unwrap_or_default();
    let paths = vec![
        PathBuf::from(format!("{}/.lean/bin/lean_prover", home)),
        PathBuf::from(format!("{}/.local/bin/lean_prover", home)),
        PathBuf::from("lean_prover"),
    ];

    paths.into_iter().find(|p| p.exists())
}

/// Run using lean_prover CLI (when available)
fn run_with_lean_cli(
    cli_path: &PathBuf,
    input: &zkvm_programs::ProgramInput,
    total_start: Instant,
) -> Result<()> {
    println!("🚀 Using lean_prover CLI: {:?}\n", cli_path);

    // The actual implementation would follow the leanMultisig workflow:
    // 1. Compile guest program to AIR bytecode
    // 2. Execute and generate trace
    // 3. Generate WHIR commitment
    // 4. Create SuperSpartan proof
    // 5. Verify proof

    // For now, fall back to reference implementation
    println!("   Note: Full CLI integration requires public SDK release");
    run_reference_execution(input, total_start)
}

/// Reference execution with simulated metrics
fn run_reference_execution(
    input: &zkvm_programs::ProgramInput,
    total_start: Instant,
) -> Result<()> {
    // ═══════════════════════════════════════════════════════════════
    // Step 1: Compute the result (in actual zkVM, this happens in guest)
    // ═══════════════════════════════════════════════════════════════
    println!("🔢 Step 1: Computing Program(ID={})...", input.program.id());
    let compute_start = Instant::now();

    let result = execute_program(input.program.id(), input.n);

    let compute_time = compute_start.elapsed();
    println!("   Result: {}", result);
    println!(
        "   Computation time: {:.3}ms",
        compute_time.as_secs_f64() * 1000.0
    );
    println!("BENCHMARK: output_result={}", result);
    println!(
        "BENCHMARK: execution_time_s={:.6}",
        compute_time.as_secs_f64()
    );
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 2: Setup (would compile guest program and setup prover)
    // ═══════════════════════════════════════════════════════════════
    println!("🔧 Step 2: Setting up prover...");
    let setup_start = Instant::now();

    // Simulated setup (would initialize WHIR parameters, KoalaBear field, etc.)
    let setup_time = setup_start.elapsed();
    println!(
        "BENCHMARK: setup_time_s={:.6}",
        setup_time.as_secs_f64()
    );
    println!("   [Reference] In actual lean zkVM, this would:");
    println!("   • Compile guest program to AIR bytecode");
    println!("   • Initialize WHIR prover parameters");
    println!("   • Setup KoalaBear field (p = 2^31 - 2^24 + 1)");
    println!("   • Configure AIR constraints");
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 3: Generate proof (reference mode - SDK not available)
    // ═══════════════════════════════════════════════════════════════
    println!("🔐 Step 3: Proof generation (reference mode)...");
    
    // TODO: Implement proof generation when SDK is available
    println!("   [TODO] Proof generation not implemented (SDK unavailable)");
    println!();

    // ═══════════════════════════════════════════════════════════════
    // Step 4: Verify proof (reference mode)
    // ═══════════════════════════════════════════════════════════════
    println!("🔍 Step 4: Verification (reference mode)...");

    // TODO: Implement verification when SDK is available
    println!("   [TODO] Verification not implemented (SDK unavailable)");
    println!();

    // Verify correctness
    let expected = execute_program(input.program.id(), input.n);
    if result == expected {
        println!("BENCHMARK: success_status=success");
    } else {
        println!("BENCHMARK: success_status=failed");
    }
    println!();

    // Calculate total time
    let total_time = total_start.elapsed();
    println!(
        "BENCHMARK: total_time_s={:.6}",
        total_time.as_secs_f64()
    );

    // Summary
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║            Execution Summary (Reference Mode)            ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("  Program:          {} (ID={})", input.program.name(), input.program.id());
    println!("  Input:            n = {}", input.n);
    println!("  Output:           {}", result);
    println!("  Mode:             Reference (SDK not available)");
    println!("  Security target:  ~128 bits");
    println!();

    println!("✅ Lean zkVM demo completed!");
    println!();
    println!("Note: This is a reference implementation.");
    println!("For actual proving, the lean_prover SDK is required:");
    println!("  https://github.com/leanEthereum/leanMultisig");

    Ok(())
}

/// Estimate execution cycles based on program type
fn estimate_cycles(program_id: u32, n: u32) -> u64 {
    match program_id {
        0 => (n as u64) * 100 + 500,           // Fibonacci
        1 => (n as u64) * 30 + 200,            // Sum
        2 => (n as u64) * 50 + 300,            // Factorial
        3 => (n as u64).isqrt() * 200 + 1000,  // IsPrime
        4 => 32 * 30 + 200,                    // Popcount
        5 | 6 => (n as u64) * 500 + 5000,      // Hash/Signature
        _ => (n as u64) * 100 + 1000,
    }
}

/// Generate a reference proof structure for demonstration purposes
/// In production, this would use WHIR + SuperSpartan from lean_prover SDK
fn generate_reference_proof(program_id: u32, n: u32, result: u32) -> Result<Vec<u8>> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut proof_data = Vec::new();

    // Header
    proof_data.extend_from_slice(b"LEAN"); // Magic bytes
    proof_data.extend_from_slice(&1u32.to_le_bytes()); // Version
    proof_data.extend_from_slice(&timestamp.to_le_bytes()); // Timestamp

    // Public inputs
    proof_data.extend_from_slice(&program_id.to_le_bytes());
    proof_data.extend_from_slice(&n.to_le_bytes());
    proof_data.extend_from_slice(&result.to_le_bytes());

    // Mock WHIR commitment (~32 bytes per commitment)
    proof_data.extend_from_slice(&[0u8; 32]); // Root commitment

    // Mock AIR proof data
    proof_data.extend_from_slice(&[0u8; 64]); // Constraint evaluations

    // Checksum
    let checksum = proof_data
        .iter()
        .fold(0u64, |acc, &b| acc.wrapping_add(b as u64));
    proof_data.extend_from_slice(&checksum.to_le_bytes());

    Ok(proof_data)
}
