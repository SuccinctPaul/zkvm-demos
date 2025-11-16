//! SP1 Host - Multi-Program Support
//!
//! An end-to-end example of using the SP1 SDK to generate a proof of multiple programs.
//!
//! You can run this script using the following command:
//! ```shell
//! PROGRAM=fibonacci INPUT_N=30 cargo run --release -- --execute
//! PROGRAM=hash INPUT_N=256 cargo run --release -- --execute
//! PROGRAM=signature INPUT_N=10 cargo run --release -- --prove
//! ```

mod cli;

use clap::Parser;
use cli::Args;
use sp1_sdk::{include_elf, ProverClient, SP1Proof, SP1ProofMode, SP1Stdin};
use std::time::Instant;

use common::load_program_input;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const GUEST_ELF: &[u8] = include_elf!("sp1-guest");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Load program input from environment
    let input = load_program_input();

    println!("╔════════════════════════════════════════╗");
    println!("║         SP1 Multi-Program Demo        ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} ({})", input.program.as_str(), input.program.description());
    println!("📊 Input N: {}", input.n);
    println!();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Get program ID directly from the common library
    // This ensures consistency between Host and Guest
    let program_id = input.program.id();

    // Setup the inputs for the guest program
    let mut stdin = SP1Stdin::new();
    stdin.write(&program_id);
    stdin.write(&input.n);

    if args.execute {
        // Execute the program
        println!("⚙️  Executing program...");
        let (mut public_values, report) = client.execute(GUEST_ELF, &stdin).run().unwrap();

        // Record the number of cycles executed.
        println!(
            "Number of instructions: {}",
            report.total_instruction_count()
        );
        println!("Number of cycles: {}", report.total_syscall_count());
        println!("Program executed successfully.");
    } else {
        // ============================================================
        // BENCHMARK MODE: Output detailed metrics
        // ============================================================

        println!("\n========== BENCHMARK START ==========");

        // Metadata
        println!("BENCHMARK: program_name=fibonacci_{}", fib_n);
        println!("BENCHMARK: zkvm_name=SP1");
        println!("BENCHMARK: zkvm_version=v5.0.0");

        // Setup the program for proving.
        println!("🔧 Setting up proving environment...");
        let (pk, vk) = client.setup(GUEST_ELF);

        // ========== Stage 0: Execution ==========
        let exec_start = Instant::now();
        let (_output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        let exec_time = exec_start.elapsed();

        println!("\n--- Execution Phase ---");
        println!("BENCHMARK: total_cycles={}", report.total_syscall_count());
        println!("BENCHMARK: total_instruction_count={}", report.total_instruction_count());
        println!("BENCHMARK: execution_time_s={:.6}", exec_time.as_secs_f64());

        // Determine proof mode from environment or default to Groth16
        let proof_mode = std::env::var("SP1_PROOF_MODE")
            .unwrap_or_else(|_| "groth16".to_string())
            .to_lowercase();

        let mode = match proof_mode.as_str() {
            "core" => SP1ProofMode::Core,
            "compressed" => SP1ProofMode::Compressed,
            "plonk" => SP1ProofMode::Plonk,
            _ => SP1ProofMode::Groth16,
        };

        println!("\n--- Proving Phase (mode: {:?}) ---", mode);

        // Start total proving timer
        let total_prove_start = Instant::now();

        match mode {
            SP1ProofMode::Core => {
                // Stage 1: Core STARK proof
                println!("Stage 1: Generating Core STARK proof...");
                let stage1_start = Instant::now();

                let proof = client.prove(&pk, &stdin)
                    .mode(SP1ProofMode::Core)
                    .run()
                    .expect("Core proof generation failed");

                let stage1_time = stage1_start.elapsed();

                // Measure core proof size
                let core_proof_size = match &proof.proof {
                    SP1Proof::Core(core_proofs) => {
                        let mut total = 0;
                        for cp in core_proofs {
                            if let Ok(bytes) = bincode::serialize(cp) {
                                total += bytes.len();
                            }
                        }
                        total
                    }
                    _ => 0,
                };

                println!("BENCHMARK: stage1_vm_prove_time_s={:.6}", stage1_time.as_secs_f64());
                println!("BENCHMARK: vm_core_proof_size_kb={:.2}", core_proof_size as f64 / 1024.0);
                println!("BENCHMARK: final_proof_size_bytes={}", core_proof_size);

                let total_prove_time = total_prove_start.elapsed();
                println!("BENCHMARK: total_prove_time_s={:.6}", total_prove_time.as_secs_f64());

                // Verify
                let verify_start = Instant::now();
                client.verify(&proof, &vk).expect("Verification failed");
                let verify_time = verify_start.elapsed();

                println!("\n--- Verification Phase ---");
                println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
                println!("BENCHMARK: verification_time_ms={:.3}", verify_time.as_secs_f64() * 1000.0);
            }

            SP1ProofMode::Compressed => {
                // Stage 1: Core proof (internal)
                println!("Stage 1: Core proof (internal)...");

                // Stage 2: Compressed proof
                println!("Stage 2-3: Generating Compressed proof...");
                let stage2_start = Instant::now();

                let proof = client.prove(&pk, &stdin)
                    .mode(SP1ProofMode::Compressed)
                    .run()
                    .expect("Compressed proof generation failed");

                let stage2_time = stage2_start.elapsed();

                // Measure compressed proof size
                let compressed_size = match &proof.proof {
                    SP1Proof::Compressed(compressed) => {
                        bincode::serialize(&compressed.proof)
                            .map(|bytes| bytes.len())
                            .unwrap_or(0)
                    }
                    _ => 0,
                };

                println!("BENCHMARK: stage2_recursive_time_s={:.6}", stage2_time.as_secs_f64());
                println!("BENCHMARK: compressed_proof_size_kb={:.2}", compressed_size as f64 / 1024.0);
                println!("BENCHMARK: final_proof_size_bytes={}", compressed_size);

                let total_prove_time = total_prove_start.elapsed();
                println!("BENCHMARK: total_prove_time_s={:.6}", total_prove_time.as_secs_f64());

                // Verify
                let verify_start = Instant::now();
                client.verify(&proof, &vk).expect("Verification failed");
                let verify_time = verify_start.elapsed();

                println!("\n--- Verification Phase ---");
                println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
                println!("BENCHMARK: verification_time_ms={:.3}", verify_time.as_secs_f64() * 1000.0);
            }

            SP1ProofMode::Groth16 => {
                // Full pipeline with all stages
                println!("Running full Groth16 pipeline with detailed timing...");

                let proof = client.prove(&pk, &stdin)
                    .mode(SP1ProofMode::Groth16)
                    .run()
                    .expect("Groth16 proof generation failed");

                let total_prove_time = total_prove_start.elapsed();

                // Measure Groth16 proof size
                let groth16_size = proof.bytes().len();

                // Note: For detailed stage breakdown, we would need to instrument SP1 SDK
                // For now, we output total time and final proof size
                println!("BENCHMARK: total_prove_time_s={:.6}", total_prove_time.as_secs_f64());
                println!("BENCHMARK: groth16_proof_size_bytes={}", groth16_size);
                println!("BENCHMARK: final_proof_size_bytes={}", groth16_size);

                // Verify
                let verify_start = Instant::now();
                client.verify(&proof, &vk).expect("Verification failed");
                let verify_time = verify_start.elapsed();

                println!("\n--- Verification Phase ---");
                println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
                println!("BENCHMARK: verification_time_ms={:.3}", verify_time.as_secs_f64() * 1000.0);
                println!("BENCHMARK: on_chain_gas_estimate=280000");
            }

            SP1ProofMode::Plonk => {
                // Plonk mode
                println!("Running Plonk pipeline...");

                let proof = client.prove(&pk, &stdin)
                    .mode(SP1ProofMode::Plonk)
                    .run()
                    .expect("Plonk proof generation failed");

                let total_prove_time = total_prove_start.elapsed();
                let plonk_size = proof.bytes().len();

                println!("BENCHMARK: total_prove_time_s={:.6}", total_prove_time.as_secs_f64());
                println!("BENCHMARK: final_proof_size_bytes={}", plonk_size);

                // Verify
                let verify_start = Instant::now();
                client.verify(&proof, &vk).expect("Verification failed");
                let verify_time = verify_start.elapsed();

                println!("\n--- Verification Phase ---");
                println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
                println!("BENCHMARK: verification_time_ms={:.3}", verify_time.as_secs_f64() * 1000.0);
            }
        }

        // Summary
        println!("\n--- Summary ---");
        println!("BENCHMARK: success_status=success");

        println!("========== BENCHMARK END ==========\n");

        println!("✓ Successfully generated and verified proof!");
    }
}
