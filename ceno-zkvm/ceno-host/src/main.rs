// CENO zkVM Host Program - Multi-Program Demo
// Note: Uses Nexus zkVM as PoC

use nexus_sdk::nexus_sdk_macros::profile;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::time::Instant;
use common::load_program_input;

const GUEST_PACKAGE: &str = "ceno-guest";

#[profile]
fn main() {
    println!("=== CENO zkVM Multi-Program Demo ===\n");
    
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Load input
    let input = load_program_input();
    println!("📊 Input: Program={} (ID={}) N={}\n", 
             input.program.as_str(), input.program.id(), input.n);
             
    // Pack inputs into a single u64 (Nexus limitation workaround)
    let input_packed = (input.program.id() as u64) << 32 | (input.n as u64);
    
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
    let prove_start = Instant::now();
    
    let (view, proof) = match prover.prove_with_input::<(), u64>(&(), &input_packed) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("❌ Proof generation failed: {}", e);
            std::process::exit(1);
        }
    };
    
    let prove_duration = prove_start.elapsed();
    println!("✅ Proof generated successfully!");
    println!("   Proving time: {:.2}s", prove_duration.as_secs_f64());
    
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
    if let Ok(code) = view.exit_code() {
        if code == nexus_sdk::KnownExitCodes::ExitSuccess as u32 {
            println!("✅ Guest program executed successfully");
        } else {
            eprintln!("❌ Guest program failed (Exit code: {})", code);
            std::process::exit(1);
        }
    }
    
    // Step 4: Verify the proof
    println!("\n🔍 Step 4: Verifying zero-knowledge proof...");
    let verify_start = Instant::now();
    
    match proof.verify_expected::<u64, ()>(
        &input_packed,
        nexus_sdk::KnownExitCodes::ExitSuccess as u32,
        &(),
        &elf,
        &[],
    ) {
        Ok(_) => println!("✅ Proof verified successfully!"),
        Err(e) => {
            eprintln!("❌ Proof verification failed: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("\n✨ CENO zkVM demo completed successfully!");
}
