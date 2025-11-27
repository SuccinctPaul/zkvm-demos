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
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);

    // Benchmark header
    println!("\n========== BENCHMARK START ==========");
    println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
    println!("BENCHMARK: zkvm_name=pico");
    println!("BENCHMARK: zkvm_version=v1.1.6");

    // Proof mode from environment (Pico uses prove_fast which maps to "core" mode)
    let proof_mode = std::env::var("PICO_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    println!("\n--- Initialization Phase ---");
    let init_start = Instant::now();

    // Load the guest program ELF
    // Try multiple possible ELF locations
    let elf_paths = vec![
        "../pico-guest/elf/riscv32im-pico-zkvm-elf",
        "../pico-guest/target/riscv32im-pico-zkvm-elf/release/pico-guest",
        "pico-guest/elf/riscv32im-pico-zkvm-elf",
        "pico-guest/target/riscv32im-pico-zkvm-elf/release/pico-guest",
    ];

    let mut elf = None;
    for path in &elf_paths {
        if let Ok(data) = std::fs::read(path) {
            println!("Loaded ELF from: {}", path);
            elf = Some(data);
            break;
        }
    }

    let elf = elf.expect(
        "Failed to read guest ELF. Please build the guest program first with 'cargo pico build' or place a pre-built ELF in pico-guest/elf/riscv32im-pico-zkvm-elf"
    );

    let init_duration = init_start.elapsed();
    println!("BENCHMARK: elf_size_bytes={}", elf.len());
    println!("BENCHMARK: initialization_time_s={:.6}", init_duration.as_secs_f64());

    // Initialize the prover client
    let client = DefaultProverClient::new(&elf);

    println!("\n--- Execution Phase ---");
    let exec_start = Instant::now();

    // Create input for the guest program
    let mut stdin_builder = client.new_stdin_builder();
    stdin_builder.write(&input.program.id());
    stdin_builder.write(&input.n);

    let exec_setup_duration = exec_start.elapsed();
    println!("BENCHMARK: execution_setup_time_s={:.6}", exec_setup_duration.as_secs_f64());

    println!("\n--- Proving Phase ---");
    let prove_start = Instant::now();

    // Generate proof
    let proof = client.prove_fast(stdin_builder)?;

    let prove_duration = prove_start.elapsed();
    println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());

    // Read the result from public values
    if let Some(public_buffer) = &proof.pv_stream {
        let result: u32 =
            bincode::deserialize(public_buffer).expect("Failed to deserialize public values");
        
        let proof_size = public_buffer.len();
        println!("BENCHMARK: proof_size_bytes={}", proof_size);
        println!("BENCHMARK: output_result={}", result);

        println!("\n============ Summary ============");
        println!("Program: {}", input.program.as_str());
        println!("Output: {}", result);
        println!("Proof size: {} bytes", proof_size);
        println!("Prove time: {:.2}s", prove_duration.as_secs_f64());
        println!("=================================\n");
    } else {
        println!("BENCHMARK: proof_size_bytes=0");
        println!("Warning: No public values in proof");
    }

    // Pico doesn't have built-in verification in prove_fast mode
    // Mark success based on proof generation
    println!("BENCHMARK: success_status=success");
    println!("\n========== BENCHMARK END ==========");
    println!("✅ Proof generated successfully!");

    Ok(())
}
