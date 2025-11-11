use pico_sdk::{Prover, ProverClient, ProverConfig};
use std::time::Instant;

// Include the guest program ELF binary
pico_sdk::include_guest!();

fn main() -> anyhow::Result<()> {
    // Setup environment
    dotenv::dotenv().ok();
    env_logger::init();

    // Load fibonacci input from environment
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);

    println!("\n1. Initializing Pico zkVM prover...");
    let init_start = Instant::now();
    
    // Create prover configuration
    let config = ProverConfig::default();
    let client = ProverClient::new(config)?;
    
    println!("   Initialization completed in {:.2}s", init_start.elapsed().as_secs_f64());

    println!("\n2. Building guest program...");
    let build_start = Instant::now();
    
    // Load the guest program
    let elf = GUEST_ELF;
    
    println!("   Build completed in {:.2}s", build_start.elapsed().as_secs_f64());
    println!("   ELF size: {} bytes", elf.len());

    println!("\n3. Executing program in zkVM...");
    let exec_start = Instant::now();
    
    // Create input for the guest program
    let mut stdin = pico_sdk::Stdin::new();
    stdin.write(&fib_n);
    
    // Execute the program
    let (output, report) = client.execute(elf, stdin)?;
    
    println!("   Execution completed in {:.2}s", exec_start.elapsed().as_secs_f64());
    println!("   Cycle count: {}", report.cycle_count());
    
    // Read the result from output
    let result: u32 = output.read();
    println!("   Fibonacci({}) = {}", fib_n, result);

    println!("\n4. Generating zero-knowledge proof...");
    let prove_start = Instant::now();
    
    // Create stdin again for proving
    let mut stdin = pico_sdk::Stdin::new();
    stdin.write(&fib_n);
    
    // Generate proof
    let prover = client.prove(elf, stdin)?;
    let proof = prover.run()?;
    
    let prove_duration = prove_start.elapsed();
    println!("   Proof generation completed in {:.2}s", prove_duration.as_secs_f64());
    println!("   Proof size: {} bytes", proof.bytes().len());

    println!("\n5. Verifying proof...");
    let verify_start = Instant::now();
    
    // Verify the proof
    client.verify(&proof)?;
    
    println!("   Verification completed in {:.2}s", verify_start.elapsed().as_secs_f64());
    println!("   ✓ Proof verified successfully!");

    println!("\n============ Summary ============");
    println!("Input: n = {}", fib_n);
    println!("Output: fibonacci({}) = {}", fib_n, result);
    println!("Total cycles: {}", report.cycle_count());
    println!("Proof size: {} bytes", proof.bytes().len());
    println!("Prove time: {:.2}s", prove_duration.as_secs_f64());
    println!("=================================\n");

    Ok(())
}

