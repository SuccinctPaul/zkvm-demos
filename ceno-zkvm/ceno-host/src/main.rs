// CENO zkVM Host Program - Real Proof Generation
// 
// Note: This implementation uses Nexus zkVM as a proof-of-concept
// demonstrating real zero-knowledge proof generation and verification.
// It will be replaced with actual CENO SDK once officially released by Scroll.
// 
// References:
// - CENO Paper: https://eprint.iacr.org/2024/387
// - Scroll Blog: https://scroll.io/blog/ceno
// - Nexus zkVM: https://github.com/nexus-xyz/nexus-zkvm

use nexus_sdk::nexus_sdk_macros::profile;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::time::Instant;

const GUEST_PACKAGE: &str = "ceno-guest";

#[profile]
fn main() {
    println!("=== CENO zkVM Fibonacci Demo (Real Proof Generation) ===\n");
    
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Load fibonacci input from environment
    let fib_n = common::load_fib_n();
    println!("📊 Configuration:");
    println!("   Input: n = {}", fib_n);
    println!("   Expected result: fib({}) = {}\n", fib_n, fib::fibonacci(fib_n));
    
    // Step 1: Compile guest program
    println!("🔨 Step 1: Compiling guest program...");
    let compile_start = Instant::now();
    
    let mut prover_compiler = Compiler::<CargoPackager>::new(GUEST_PACKAGE);
    let prover: Stwo<Local> = match Stwo::compile(&mut prover_compiler) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Compilation failed: {}", e);
            std::process::exit(1);
        }
    };
    
    let compile_duration = compile_start.elapsed();
    println!("✅ Compilation completed in {:.2}s", compile_duration.as_secs_f64());
    
    let elf = prover.elf.clone();
    println!("   ELF instructions: {}", elf.instructions.len());
    
    // Step 2: Generate zero-knowledge proof
    println!("\n🔐 Step 2: Generating zero-knowledge proof...");
    println!("   This process may take several minutes for real ZK proof generation.");
    let prove_start = Instant::now();
    
    let (view, proof) = match prover.prove_with_input::<(), u32>(&(), &fib_n) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("❌ Proof generation failed: {}", e);
            std::process::exit(1);
        }
    };
    
    let prove_duration = prove_start.elapsed();
    println!("✅ Proof generated successfully!");
    println!("   Proving time: {:.2}s", prove_duration.as_secs_f64());
    println!("   Proof size: {} bytes", proof.size_estimate());
    
    // Display execution logs
    println!("\n📝 Step 3: Execution logs:");
    println!("-------------------");
    match view.logs() {
        Ok(logs) => {
            for log in logs {
                print!("{}", log);
            }
        }
        Err(e) => eprintln!("⚠️  Warning: Failed to retrieve logs - {}", e),
    }
    println!("-------------------");
    
    // Check exit code
    match view.exit_code() {
        Ok(code) => {
            if code == nexus_sdk::KnownExitCodes::ExitSuccess as u32 {
                println!("✅ Guest program executed successfully (Exit code: {})", code);
            } else {
                eprintln!("❌ Guest program failed (Exit code: {})", code);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to retrieve exit code: {}", e);
            std::process::exit(1);
        }
    }
    
    // Step 4: Verify the proof
    println!("\n🔍 Step 4: Verifying zero-knowledge proof...");
    let verify_start = Instant::now();
    
    match proof.verify_expected::<u32, ()>(
        &fib_n,
        nexus_sdk::KnownExitCodes::ExitSuccess as u32,
        &(),
        &elf,
        &[],
    ) {
        Ok(_) => {
            let verify_duration = verify_start.elapsed();
            println!("✅ Proof verified successfully!");
            println!("   Verification time: {:.2}s", verify_duration.as_secs_f64());
        }
        Err(e) => {
            eprintln!("❌ Proof verification failed: {}", e);
            std::process::exit(1);
        }
    }
    
    // Summary
    let total_time = compile_duration + prove_duration;
    println!("\n{}", "=".repeat(60));
    println!("📊 Summary:");
    println!("{}", "=".repeat(60));
    println!("  Input:             n = {}", fib_n);
    println!("  Result:            fib({}) = {}", fib_n, fib::fibonacci(fib_n));
    println!("  Compilation time:  {:.2}s", compile_duration.as_secs_f64());
    println!("  Proving time:      {:.2}s", prove_duration.as_secs_f64());
    println!("  Verification time: {:.2}s", verify_start.elapsed().as_secs_f64());
    println!("  Total time:        {:.2}s", total_time.as_secs_f64());
    println!("  Proof size:        {} bytes", proof.size_estimate());
    println!("{}", "=".repeat(60));
    
    println!("\n✨ CENO zkVM demo completed successfully!");
    println!("\n📌 Note: This demo uses Nexus zkVM to demonstrate real ZK proof");
    println!("   generation. It will be updated to use the official CENO SDK");
    println!("   once released by Scroll.");
    println!("   For updates, check: https://github.com/scroll-tech/ceno");
}
