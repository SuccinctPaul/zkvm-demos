use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    // Setup environment
    dotenv::dotenv().ok();
    init_logger();

    // Load fibonacci input from environment
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);

    println!("\n1. Initializing Pico zkVM prover...");
    let init_start = Instant::now();

    // Load the guest program ELF
    // Try multiple possible ELF locations
    let elf_paths = vec![
        "../pico-guest/elf/riscv32im-pico-zkvm-elf",
        "../pico-guest/target/riscv32im-pico-zkvm-elf/release/pico-guest",
        "pico-guest/elf/riscv32im-pico-zkvm-elf",
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

    println!(
        "Initialization completed in {:.2}s",
        init_start.elapsed().as_secs_f64()
    );
    println!("ELF size: {} bytes", elf.len());

    // Initialize the prover client
    let client = DefaultProverClient::new(&elf);

    println!("\n2. Executing program in zkVM...");
    let exec_start = Instant::now();

    // Create input for the guest program
    let mut stdin_builder = client.new_stdin_builder();
    stdin_builder.write(&fib_n);

    println!(
        "Execution setup completed in {:.2}s",
        exec_start.elapsed().as_secs_f64()
    );

    println!("\n3. Generating zero-knowledge proof...");
    let prove_start = Instant::now();

    // Generate proof
    let proof = client.prove_fast(stdin_builder)?;

    let prove_duration = prove_start.elapsed();
    println!(
        "Proof generation completed in {:.2}s",
        prove_duration.as_secs_f64()
    );

    // Read the result from public values
    if let Some(public_buffer) = &proof.pv_stream {
        let result: u32 =
            bincode::deserialize(public_buffer).expect("Failed to deserialize public values");
        println!("Fibonacci({}) = {}", fib_n, result);

        println!("\n============ Summary ============");
        println!("Input: n = {}", fib_n);
        println!("Output: fibonacci({}) = {}", fib_n, result);
        println!("Proof size: {} bytes", public_buffer.len());
        println!("Prove time: {:.2}s", prove_duration.as_secs_f64());
        println!("=================================\n");
    } else {
        println!("Warning: No public values in proof");
    }

    println!("Proof generated successfully!");

    Ok(())
}
