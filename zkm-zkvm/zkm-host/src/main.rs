use zkm_sdk::{include_elf, ProverClient, ZKMProofKind, ZKMStdin};
use zkvm_programs::load_program_input;

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
    println!("📋 Program: {} (ID={})", input.program.name(), input.program.id());
    println!("📊 Input N: {}", input.n);
    println!();

    let mut stdin = ZKMStdin::new();
    stdin.write(&input.program.id());
    stdin.write(&input.n);

    // Execute the program
    let start_execute = std::time::Instant::now();
    let (_output, report) = client.execute(GUEST_ELF, stdin.clone()).run().unwrap();
    let duration_execute = start_execute.elapsed();

    // Read the output.
    // let expect = fib::fibonacci(fib_n);
    // assert_eq!(a, expected_a);
    // assert_eq!(b, expected_b);
    // println!("Values are correct!");

    // Record the number of cycles executed.
    println!(
        "BENCHMARK: total_instruction_count={}",
        report.total_instruction_count()
    );
    println!(
        "BENCHMARK: total_cycles={}",
        report.total_instruction_count()
    );
    println!(
        "BENCHMARK: execute_time_s={:.4}",
        duration_execute.as_secs_f64()
    );
    // println!("Number of cycles: {}", report.total_syscall_count());
    println!("execution report (totals): \n{}", report);
    println!("Program executed successfully.");

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
    println!("BENCHMARK: proof_size_bytes={}", proof.bytes().len());
    println!("BENCHMARK: proof_mode={:?}", proof_mode);

    // Verify the proof.
    println!("🔍 Verifying proof...");
    let verify_start = std::time::Instant::now();
    client.verify(&proof, &vk).expect("failed to verify proof");
    let verify_duration = verify_start.elapsed();
    println!("✨ Successfully verified proof!");
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=zkm");
    println!("BENCHMARK: zkvm_version=v0.1.0");
    println!("BENCHMARK: success_status=success");
}
