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
    let proof_mode_env = std::env::var("PROOF_MODE").unwrap_or_else(|_| "groth16".to_string());
    let proof_mode = match proof_mode_env.to_lowercase().as_str() {
        "core" => ZKMProofKind::Core,
        "compressed" => ZKMProofKind::Compressed,
        "plonk" => ZKMProofKind::Plonk,
        "groth16" => ZKMProofKind::Groth16,
        "compress_to_groth16" => ZKMProofKind::CompressToGroth16,
        _ => ZKMProofKind::Groth16,
    };
    println!("BENCHMARK: proof_mode={:?}", proof_mode);

    let total_prove_start = std::time::Instant::now();

    let proof = match proof_mode {
        ZKMProofKind::Core => {
            println!("Stage 1: Generating Core proof...");
            let start = std::time::Instant::now();
            let proof = client
                .prove(&pk, stdin)
                .core()
                .run()
                .expect("Core proof generation failed");
            let duration = start.elapsed();

            println!(
                "BENCHMARK: stage1_vm_prove_time_s={:.6}",
                duration.as_secs_f64()
            );
            println!(
                "BENCHMARK: vm_core_proof_size_bytes={}",
                proof.bytes().len()
            );
            println!(
                "BENCHMARK: final_proof_size_bytes={}",
                proof.bytes().len()
            );
            proof
        }
        ZKMProofKind::Compressed => {
            println!("Stage 2: Generating Compressed proof...");
            let start = std::time::Instant::now();
            let proof = client
                .prove(&pk, stdin)
                .compressed()
                .run()
                .expect("Compressed proof generation failed");
            let duration = start.elapsed();

            println!(
                "BENCHMARK: stage2_recursive_time_s={:.6}",
                duration.as_secs_f64()
            );
            println!(
                "BENCHMARK: compressed_proof_size_bytes={}",
                proof.bytes().len()
            );
            println!(
                "BENCHMARK: final_proof_size_bytes={}",
                proof.bytes().len()
            );
            proof
        }
        ZKMProofKind::Groth16 => {
            println!("Running full Groth16 pipeline...");
            let start = std::time::Instant::now();
            let proof = client
                .prove(&pk, stdin)
                .groth16()
                .run()
                .expect("Groth16 proof generation failed");
            let duration = start.elapsed();

            println!(
                "BENCHMARK: total_prove_time_s={:.6}",
                duration.as_secs_f64()
            );
            println!(
                "BENCHMARK: groth16_proof_size_bytes={}",
                proof.bytes().len()
            );
            println!(
                "BENCHMARK: final_proof_size_bytes={}",
                proof.bytes().len()
            );
            proof
        }
        ZKMProofKind::CompressToGroth16 => {
            println!("Running CompressToGroth16 pipeline...");
            let start = std::time::Instant::now();
            let proof = client
                .prove(&pk, stdin)
                .compress_to_groth16()
                .run()
                .expect("CompressToGroth16 proof generation failed");
            let duration = start.elapsed();

            println!(
                "BENCHMARK: total_prove_time_s={:.6}",
                duration.as_secs_f64()
            );
            println!(
                "BENCHMARK: groth16_proof_size_bytes={}",
                proof.bytes().len()
            );
            println!(
                "BENCHMARK: final_proof_size_bytes={}",
                proof.bytes().len()
            );
            proof
        }
        ZKMProofKind::Plonk => {
            println!("Running Plonk pipeline...");
            let start = std::time::Instant::now();
            let proof = client
                .prove(&pk, stdin)
                .plonk()
                .run()
                .expect("Plonk proof generation failed");
            let duration = start.elapsed();

            println!(
                "BENCHMARK: total_prove_time_s={:.6}",
                duration.as_secs_f64()
            );
            println!(
                "BENCHMARK: plonk_proof_size_bytes={}",
                proof.bytes().len()
            );
            println!(
                "BENCHMARK: final_proof_size_bytes={}",
                proof.bytes().len()
            );
            proof
        }
    };

    let total_prove_time = total_prove_start.elapsed();
    if matches!(
        proof_mode,
        ZKMProofKind::Core | ZKMProofKind::Compressed
    ) {
        println!(
            "BENCHMARK: total_prove_time_s={:.6}",
            total_prove_time.as_secs_f64()
        );
    }

    println!("✅ Successfully generated proof!");

    // Verify the proof.
    println!("🔍 Verifying proof...");
    let verify_start = std::time::Instant::now();
    client.verify(&proof, &vk).expect("failed to verify proof");
    let verify_duration = verify_start.elapsed();
    println!("✨ Successfully verified proof!");
    println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
    println!("BENCHMARK: verification_time_ms={:.3}", verify_duration.as_secs_f64() * 1000.0);
    println!("BENCHMARK: program_name={}_{}", input.program.name(), input.n);
    println!("BENCHMARK: zkvm_name=zkm");
    println!("BENCHMARK: zkvm_version=v0.1.0");
    println!("BENCHMARK: success_status=success");
}
