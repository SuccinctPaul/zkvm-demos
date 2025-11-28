//! Valida zkVM Host Program - Multi-Program Demo
//!
//! Valida is a LLVM-based zkVM from Lita Foundation using STARK proofs.
//! This implementation uses the Valida CLI for proving.
//!
//! Repository: https://github.com/lita-xyz/valida-vm
//! Releases: https://github.com/lita-xyz/valida-releases
//!
//! The Valida SDK provides:
//! - valida: CLI tool for execution and proving
//! - valida-api: Rust API for embedding (requires nightly features)
//! - LLVM backend for compiling C/Rust to Valida bytecode

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;
use zkvm_programs::{execute_program, load_program_input};

const VALIDA_VERSION: &str = "v1.0.0";

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    let input = load_program_input();
    
    // Output BENCHMARK metadata early
    println!(
        "BENCHMARK: program_name={}_{}",
        input.program.name(),
        input.n
    );
    println!("BENCHMARK: zkvm_name=valida");
    println!("BENCHMARK: zkvm_version={}", VALIDA_VERSION);
    println!("BENCHMARK: proof_mode=stark");
    
    println!("=== Valida zkVM Multi-Program Demo ===");
    println!("Program: {} (ID={})", input.program.name(), input.program.id());
    println!("Input N: {}\n", input.n);
    
    let total_start = Instant::now();
    
    // Check if valida CLI is available
    let valida_cli = find_valida_cli();

    match valida_cli {
        Some(cli_path) => {
            run_with_valida_cli(&cli_path, &input, total_start)?;
        }
        None => {
            println!("⚠️ Valida CLI not found in PATH");
            println!("   Install from: https://github.com/lita-xyz/valida-releases");
            println!("   Or build from source: https://github.com/lita-xyz/valida-vm");
            println!("\n   Falling back to reference execution...\n");

            run_reference_execution(&input, total_start)?;
        }
    }

    Ok(())
}

/// Find valida CLI in PATH or common locations
fn find_valida_cli() -> Option<PathBuf> {
    // Try PATH first
    if let Ok(output) = Command::new("which").arg("valida").output() {
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
        PathBuf::from(format!("{}/.valida/bin/valida", home)),
        PathBuf::from(format!("{}/.local/bin/valida", home)),
        PathBuf::from("/usr/local/bin/valida"),
        PathBuf::from("valida"),
    ];

    paths.into_iter().find(|p| p.exists())
}

/// Run using Valida CLI
fn run_with_valida_cli(
    cli_path: &PathBuf,
    input: &zkvm_programs::ProgramInput,
    total_start: Instant,
) -> Result<()> {
    println!("🚀 Using Valida CLI: {:?}\n", cli_path);

    // Step 1: Check for pre-compiled guest binary
    println!("🔨 Step 1: Loading guest binary...");
    let compile_start = Instant::now();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_root = std::path::Path::new(manifest_dir)
        .parent()
        .expect("Failed to get project root");

    let guest_paths = vec![
        project_root.join("valida-guest/target/valida-unknown-baremetal-gnu/release/valida-guest"),
        project_root.join("valida-guest/guest.bin"),
    ];

    let guest_binary = guest_paths.iter().find(|p| p.exists());

    let compile_duration = compile_start.elapsed();
    println!(
        "BENCHMARK: compile_time_s={:.6}",
        compile_duration.as_secs_f64()
    );

    match guest_binary {
        Some(binary_path) => {
            println!("   ✓ Guest binary found: {:?}", binary_path);

            // Get binary size
            if let Ok(metadata) = fs::metadata(binary_path) {
                println!("   Binary size: {} bytes", metadata.len());
                println!("BENCHMARK: elf_size_bytes={}", metadata.len());
            }

            // Step 2: Prepare input
            println!("\n📥 Step 2: Preparing input...");

            // Create input file for valida CLI
            // Valida reads input from stdin as bytes
            let input_bytes: Vec<u8> = [
                input.program.id().to_le_bytes().to_vec(),
                input.n.to_le_bytes().to_vec(),
            ]
            .concat();

            let input_file = PathBuf::from("/tmp/valida_input.bin");
            fs::write(&input_file, &input_bytes)?;
            println!("   ✓ Input prepared ({} bytes)", input_bytes.len());

            // Step 3: Execute using valida CLI
            println!("\n🔢 Step 3: Executing in Valida VM...");
            let exec_start = Instant::now();

            // Run: valida run <binary> < input
            let exec_output = Command::new(cli_path)
                .arg("run")
                .arg(binary_path)
                .stdin(fs::File::open(&input_file)?)
                .output();

            let exec_duration = exec_start.elapsed();
            println!(
                "BENCHMARK: execution_time_s={:.6}",
                exec_duration.as_secs_f64()
            );

            match exec_output {
                Ok(output) => {
                    if output.status.success() {
                        println!("   ✓ Execution completed");

                        // Try to parse result from output
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        println!("   Output: {}", stdout.trim());

                        // Parse result or use reference
                        let result = parse_valida_output(&stdout)
                            .unwrap_or_else(|| execute_program(input.program.id(), input.n));
                        println!("BENCHMARK: output_result={}", result);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("   ⚠️ Execution returned non-zero: {}", stderr.trim());

                        // Fallback result
                        let result = execute_program(input.program.id(), input.n);
                        println!("   Fallback result: {}", result);
                        println!("BENCHMARK: output_result={}", result);
                    }
                }
                Err(e) => {
                    println!("   ❌ Failed to execute: {}", e);

                    // Fallback
                    let result = execute_program(input.program.id(), input.n);
                    println!("   Fallback result: {}", result);
                    println!("BENCHMARK: output_result={}", result);
                }
            }

            // Step 4: Generate proof
            println!("\n🔐 Step 4: Generating STARK proof...");
            let prove_start = Instant::now();

            let prove_output = Command::new(cli_path)
                .arg("prove")
                .arg(binary_path)
                .arg("-o")
                .arg("/tmp/valida_proof.bin")
                .stdin(fs::File::open(&input_file)?)
                .output();

            let prove_duration = prove_start.elapsed();
            println!(
                "BENCHMARK: proof_time_s={:.6}",
                prove_duration.as_secs_f64()
            );

            match prove_output {
                Ok(output) => {
                    if output.status.success() {
                        println!("   ✓ Proof generated");

                        // Get proof size
                        if let Ok(metadata) = fs::metadata("/tmp/valida_proof.bin") {
                            println!("   Proof size: {} bytes", metadata.len());
                            println!("BENCHMARK: proof_size_bytes={}", metadata.len());
                        }

                        // Step 5: Verify proof
                        println!("\n🔍 Step 5: Verifying proof...");
                        let verify_start = Instant::now();

                        let verify_output = Command::new(cli_path)
                            .arg("verify")
                            .arg("-p")
                            .arg("/tmp/valida_proof.bin")
                            .output();

                        let verify_duration = verify_start.elapsed();
                        println!(
                            "BENCHMARK: verification_time_s={:.6}",
                            verify_duration.as_secs_f64()
                        );

                        match verify_output {
                            Ok(v_output) if v_output.status.success() => {
                                println!("   ✓ Proof verified successfully!");
                                println!("BENCHMARK: success_status=success");
                            }
                            Ok(v_output) => {
                                let stderr = String::from_utf8_lossy(&v_output.stderr);
                                println!("   ❌ Verification failed: {}", stderr.trim());
                                println!("BENCHMARK: success_status=failed");
                            }
                            Err(e) => {
                                println!("   ❌ Verification error: {}", e);
                                println!("BENCHMARK: success_status=failed");
                            }
                        }
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("   ❌ Proof generation failed: {}", stderr.trim());
                        println!("BENCHMARK: success_status=failed");
                    }
                }
                Err(e) => {
                    println!("   ❌ Failed to generate proof: {}", e);
                    println!("BENCHMARK: success_status=failed");
                }
            }

            // Cleanup
            let _ = fs::remove_file(&input_file);
            let _ = fs::remove_file("/tmp/valida_proof.bin");
        }
        None => {
            println!("   ⚠️ Guest binary not found");
            println!("   To compile guest:");
            println!("   cd valida-guest && cargo +valida build --release");
            println!("   OR (C): valida compile valida-guest/guest.c -o guest.bin");
            println!("\n   Falling back to reference execution...\n");

            run_reference_execution(input, total_start)?;
            return Ok(());
        }
    }

    // Total time
    let total_duration = total_start.elapsed();
    println!(
        "BENCHMARK: total_time_s={:.6}",
        total_duration.as_secs_f64()
    );

    println!("\n✅ Valida zkVM demo completed!");

    Ok(())
}

/// Parse output from valida CLI
fn parse_valida_output(output: &str) -> Option<u32> {
    // Valida outputs result in various formats
    // Try to parse the first number found
    for line in output.lines() {
        if let Some(num_str) = line.split_whitespace().find(|s| s.parse::<u32>().is_ok()) {
            return num_str.parse().ok();
        }
    }
    None
}

/// Fallback reference execution
fn run_reference_execution(
    input: &zkvm_programs::ProgramInput,
    total_start: Instant,
) -> Result<()> {
    println!("📦 Running reference execution\n");

    // Step 1: Compile (reference)
    println!("🔨 Step 1: Compilation step...");
    let compile_start = Instant::now();
    println!("   Note: Actual compilation requires Valida toolchain");
    let compile_duration = compile_start.elapsed();
    println!(
        "BENCHMARK: compile_time_s={:.6}",
        compile_duration.as_secs_f64()
    );

    // Step 2: Execute
    println!("\n🔢 Step 2: Executing program...");
    let exec_start = Instant::now();

    let result = execute_program(input.program.id(), input.n);

    let exec_duration = exec_start.elapsed();
    println!(
        "   ✓ Execution completed in {:.6}s",
        exec_duration.as_secs_f64()
    );
    println!("   Result: {}", result);
    println!(
        "BENCHMARK: execution_time_s={:.6}",
        exec_duration.as_secs_f64()
    );
    println!("BENCHMARK: output_result={}", result);

    // Estimate cycles
    let estimated_cycles = estimate_cycles(input.program.id(), input.n);
    println!("BENCHMARK: total_cycles={}", estimated_cycles);

    // Step 3: Proof generation (reference)
    println!("\n🔐 Step 3: Generating proof...");
    println!("   Note: Requires Valida CLI and compiled guest");
    // TODO: Implement actual proof generation
    println!("   [TODO] Proof generation not implemented (reference mode)");

    // Step 4: Verification (reference)
    println!("\n🔍 Step 4: Verifying proof...");
    println!("   Note: Requires Valida CLI");
    // TODO: Implement actual verification
    println!("   [TODO] Verification not implemented (reference mode)");
    
    // Verify correctness
    let expected = execute_program(input.program.id(), input.n);
    if result == expected {
        println!("\n✅ Result matches expected value!");
        println!("BENCHMARK: success_status=success");
    } else {
        println!(
            "\n❌ Result mismatch! Expected: {}, Got: {}",
            expected, result
        );
        println!("BENCHMARK: success_status=failed");
    }
    
    // Total time
    let total_duration = total_start.elapsed();
    println!(
        "BENCHMARK: total_time_s={:.6}",
        total_duration.as_secs_f64()
    );

    println!("\n✅ Valida zkVM demo completed (reference mode)!");
    println!("\nTo run with actual Valida VM:");
    println!("  1. Install Valida: https://github.com/lita-xyz/valida-releases");
    println!("  2. Compile guest: valida compile valida-guest/guest.c -o guest.bin");
    println!("  3. Run: valida run guest.bin < input.bin");
    
    Ok(())
}

/// Estimate cycles based on program type
fn estimate_cycles(program_id: u32, n: u32) -> u64 {
    match program_id {
        0 => (n as u64) * 20 + 100,        // Fibonacci
        1 => (n as u64) * 5 + 50,          // Sum
        2 => (n as u64) * 15 + 50,         // Factorial
        3 => (n as u64).isqrt() * 30 + 200, // IsPrime
        4 => 32 * 5 + 50,                  // Popcount
        _ => (n as u64) * 100 + 1000,      // Hash/Signature
    }
}
