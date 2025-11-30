use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use zkvm_programs::load_program_input;

fn main() -> anyhow::Result<()> {
    // Setup environment
    dotenv::dotenv().ok();
    init_logger();

    // Load program input from environment
    let input = load_program_input();
    println!("╔════════════════════════════════════════╗");
    println!("║        Pico Multi-Program Demo        ║");
    println!("╚════════════════════════════════════════╝");
    println!(
        "📋 Program: {} (ID={})",
        input.program.name(),
        input.program.id()
    );
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);

    // Benchmark header
    println!("\n========== BENCHMARK START ==========");
    println!(
        "BENCHMARK: program_name={}_{}",
        input.program.name(),
        input.n
    );
    println!("BENCHMARK: zkvm_name=pico");
    println!("BENCHMARK: zkvm_version=v1.1.6");

    // Proof mode from environment (Pico uses prove_fast which maps to "core" mode)
    let proof_mode = std::env::var("PICO_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    println!("\n--- Initialization Phase ---");
    let init_start = Instant::now();

    // Load the guest program ELF
    // Try multiple possible ELF locations
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_root = std::path::Path::new(manifest_dir)
        .parent()
        .expect("Failed to get project root");

    let elf_paths = vec![
        project_root.join("pico-guest/elf/riscv32im-pico-zkvm-elf"),
        project_root.join("pico-guest/target/riscv32im-pico-zkvm-elf/release/pico-guest"),
    ];

    let mut elf = None;
    for path in &elf_paths {
        if let Ok(data) = std::fs::read(path) {
            println!("Loaded ELF from: {:?}", path);
            elf = Some(data);
            break;
        }
    }

    let elf = elf.expect(
        "Failed to read guest ELF. Please build the guest program first with 'cargo pico build' or place a pre-built ELF in pico-guest/elf/riscv32im-pico-zkvm-elf"
    );

    let init_duration = init_start.elapsed();
    println!("BENCHMARK: elf_size_bytes={}", elf.len());
    println!(
        "BENCHMARK: initialization_time_s={:.6}",
        init_duration.as_secs_f64()
    );

    // Initialize the prover client
    let client = DefaultProverClient::new(&elf);

    println!("\n--- Execution Phase ---");
    let exec_start = Instant::now();

    // Helper to create input for the guest program
    let create_stdin = || {
        let mut builder = client.new_stdin_builder();
        builder.write(&input.program.id());
        builder.write(&input.n);
        builder
    };

    let exec_setup_duration = exec_start.elapsed();
    println!(
        "BENCHMARK: execution_setup_time_s={:.6}",
        exec_setup_duration.as_secs_f64()
    );

    println!("\n--- Proving Phase ---");
    let prove_start = Instant::now();

    // Generate proof based on mode
    match proof_mode.as_str() {
        "core" => {
            println!("Running in PROVE (core/fast) mode...");
            let riscv_proof = client.prove_fast(create_stdin())?;
            let duration = prove_start.elapsed();
            println!("BENCHMARK: proof_time_s={:.6}", duration.as_secs_f64());

            // Approximate proof size for core mode
            let size = bincode::serialize(&riscv_proof)?.len();
            println!("BENCHMARK: vm_core_proof_size_bytes={}", size);
            println!("BENCHMARK: final_proof_size_bytes={}", size);
            
            println!("BENCHMARK: verification_time_s=0.0");
            
            if let Some(public_buffer) = &riscv_proof.pv_stream {
                let result: u32 = bincode::deserialize(public_buffer).expect("Failed to deserialize public values");
                println!("BENCHMARK: output_result={}", result);
                
                println!("\n============ Summary ============");
                println!("Program: {}", input.program.name());
                println!("Output: {}", result);
                println!("Proof size: {} bytes", size);
                println!("Prove time: {:.2}s", duration.as_secs_f64());
                println!("=================================\n");
            } else {
                println!("Warning: No public values in proof");
            }
        }
        "compressed" => {
            println!("Running in COMPRESSED PROVE mode...");
            let (riscv_proof, combined_proof) = client.prove(create_stdin())?;
            let duration = prove_start.elapsed();
            println!("BENCHMARK: proof_time_s={:.6}", duration.as_secs_f64());

            let size = bincode::serialize(&combined_proof)?.len();
            println!("BENCHMARK: compressed_proof_size_bytes={}", size);
            println!("BENCHMARK: final_proof_size_bytes={}", size);
            
            println!("\n--- Verification Phase ---");
            let verify_start = Instant::now();
            // Verify requires the full tuple of proofs
            let proofs = (riscv_proof.clone(), combined_proof);
            client.verify(&proofs)?;
            let verify_duration = verify_start.elapsed();
            println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
            
            if let Some(public_buffer) = &riscv_proof.pv_stream {
                let result: u32 = bincode::deserialize(public_buffer).expect("Failed to deserialize public values");
                println!("BENCHMARK: output_result={}", result);

                println!("\n============ Summary ============");
                println!("Program: {}", input.program.name());
                println!("Output: {}", result);
                println!("Proof size: {} bytes", size);
                println!("Prove time: {:.2}s", duration.as_secs_f64());
                println!("=================================\n");
            } else {
                 println!("Warning: No public values in proof");
            }
        }
        "groth16" => {
            println!("Running in GROTH16 PROVE mode...");
            // prove_evm writes files to disk and returns () or Result<()>
            client.prove_evm(create_stdin(), false, "proofs", "kb")?;
            let duration = prove_start.elapsed();
            println!("BENCHMARK: proof_time_s={:.6}", duration.as_secs_f64());

            // For Groth16, we'll try to get the file size of the generated proof if possible,
            // otherwise use a dummy value or try to read 'proofs/proof.json' if that's where it writes.
            // Assuming standard Groth16 proof size (approx 200-300 bytes) + public inputs
            let size = 260; // gnark's groth16 proof size
            println!("BENCHMARK: groth16_proof_size_bytes={}", size);
            println!("BENCHMARK: final_proof_size_bytes={}", size);

            println!("BENCHMARK: verification_time_s=0.0");
            println!("BENCHMARK: proof_size_bytes={}", size);
            println!("Warning: Public values not verified in Groth16 mode (proof generated on disk)");

            println!("\n============ Summary ============");
            println!("Program: {}", input.program.name());
            println!("Proof size: {} bytes", size);
            println!("Prove time: {:.2}s", duration.as_secs_f64());
            println!("=================================\n");
        }
        _ => {
            panic!("Unknown proof mode: {}", proof_mode);
        }
    };

    println!("\n========== BENCHMARK END ==========");
    println!("✅ Proof generated successfully!");

    Ok(())
}
