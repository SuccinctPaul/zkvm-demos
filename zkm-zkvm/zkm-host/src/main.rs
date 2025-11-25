use zkm_sdk::{include_elf, ProverClient, ZKMProofKind, ZKMStdin};
use common::load_program_input;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const GUEST_ELF: &[u8] = include_elf!("zkm-guest");

fn main() {
    // Setup the logger.
    zkm_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let input = load_program_input();
    println!("╔════════════════════════════════════════╗");
    println!("║        ZKM Multi-Program Demo         ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("📊 Input N: {}", input.n);
    println!();
    
    let mut stdin = ZKMStdin::new();
    stdin.write(&input.program.id());
    stdin.write(&input.n);

    // Execute the program
    println!("⚙️  Executing program...");
    let (_output, report) = client.execute(GUEST_ELF, stdin.clone()).run().unwrap();
    
    println!("✅ Execution completed");
    println!("📊 Instructions: {}", report.total_instruction_count());

    println!("\n");
    // Setup the program for proving.
    println!("🔧 Setting up proving environment...");
    let (pk, vk) = client.setup(GUEST_ELF);

    // Generate the proof
    println!("🔐 Generating proof...");
    let proof_mode = ZKMProofKind::Groth16;
    let prover = match proof_mode {
        ZKMProofKind::Core => client.prove(&pk, stdin).core(),
        ZKMProofKind::Compressed => client.prove(&pk, stdin).compressed(),
        ZKMProofKind::Plonk => client.prove(&pk, stdin).plonk(),
        ZKMProofKind::Groth16 => client.prove(&pk, stdin).groth16(),
        ZKMProofKind::CompressToGroth16 => client.prove(&pk, stdin).compress_to_groth16(),
    };
    let proof = prover.run().expect("failed to generate proof");

    println!("✅ Successfully generated proof!");
    println!(
        "proof_mode: {:?} , proof size: {:?} Bytes",
        proof_mode,
        proof.bytes().len()
    );

    // Verify the proof.
    println!("🔍 Verifying proof...");
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("✨ Successfully verified proof!");
}
