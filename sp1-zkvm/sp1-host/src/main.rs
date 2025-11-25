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

        // Read the output result
        let result: u32 = public_values.read();
        
        println!();
        println!("✅ Execution Results:");
        println!("─────────────────────────────────────");
        println!("📤 Output: {}", result);
        println!("📊 Instructions: {}", report.total_instruction_count());
        println!("🔄 Cycles: {}", report.total_syscall_count());
        println!();
        println!("✨ Program executed successfully!");
    } else {
        // Setup the program for proving.
        println!("🔧 Setting up proving environment...");
        let (pk, vk) = client.setup(GUEST_ELF);

        // Generate the proof
        println!("🔐 Generating proof...");
        
        // Allow configuring proof mode via environment variable
        let proof_mode = match std::env::var("PROOF_MODE").unwrap_or_default().as_str() {
            "core" => SP1ProofMode::Core,
            "compressed" => SP1ProofMode::Compressed,
            "plonk" => SP1ProofMode::Plonk,
            _ => SP1ProofMode::Groth16,
        };
        
        let prover = client.prove(&pk, &stdin).mode(proof_mode);
        let proof = prover.run().expect("failed to generate proof");

        println!();
        println!("✅ Successfully generated proof!");

        // Calculate proof size
        let proof_size = match proof.proof.clone() {
            SP1Proof::Core(core_proof) => {
                let mut total_proof_bytes = 0;
                for cp in core_proof {
                    let proof_bytes = serde_json::to_vec(&cp).unwrap();
                    total_proof_bytes += proof_bytes.len();
                }
                total_proof_bytes
            }
            SP1Proof::Compressed(compress) => {
                let proof_bytes = serde_json::to_vec(&compress.proof).unwrap();
                proof_bytes.len()
            }
            _ => proof.bytes().len(),
        };
        
        println!();
        println!("📊 Proof Information:");
        println!("─────────────────────────────────────");
        println!("🔒 Mode: {:?}", proof_mode);
        println!("📦 Size: {} bytes ({:.2} KB)", proof_size, proof_size as f64 / 1024.0);

        // Verify the proof.
        println!();
        println!("🔍 Verifying proof...");
        client.verify(&proof, &vk).expect("failed to verify proof");
        
        println!();
        println!("✨ Successfully verified proof!");
        println!("╚════════════════════════════════════════╝");
    }
}
