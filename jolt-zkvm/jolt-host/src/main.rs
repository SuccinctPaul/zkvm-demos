use std::time::Instant;
use zkvm_programs::load_program_input;

pub fn main() {
    // Load program input from environment
    let input = load_program_input();
    
    println!("╔════════════════════════════════════════╗");
    println!("║       Jolt Multi-Program Demo         ║");
    println!("╚════════════════════════════════════════╝");
    println!("📋 Program: {} (ID={})", input.program.as_str(), input.program.id());
    println!("ℹ️  Description: {}", input.program.description());
    println!("📊 Input N: {}", input.n);
    println!();

    // Compile the guest program
    println!("1️⃣  Compiling guest program...");
    let compile_start = Instant::now();

    let target_dir = "/tmp/jolt-guest-targets";
    let mut program = guest::compile_execute_program(target_dir);

    let compile_duration = compile_start.elapsed();
    println!(
        "   ✓ Compilation completed in {:.2}s\n",
        compile_duration.as_secs_f64()
    );

    // Preprocessing
    println!("2️⃣  Preprocessing...");
    let preprocess_start = Instant::now();

    let prover_preprocessing = guest::preprocess_prover_execute_program(&mut program);
    let verifier_preprocessing = guest::preprocess_verifier_execute_program(&mut program);

    let preprocess_duration = preprocess_start.elapsed();
    println!(
        "   ✓ Preprocessing completed in {:.2}s\n",
        preprocess_duration.as_secs_f64()
    );

    // Build prover and verifier
    println!("3️⃣  Building prover and verifier...");
    let build_start = Instant::now();

    let prove_exec = guest::build_prover_execute_program(program, prover_preprocessing);
    let verify_exec = guest::build_verifier_execute_program(verifier_preprocessing);

    let build_duration = build_start.elapsed();
    println!(
        "   ✓ Build completed in {:.2}s\n",
        build_duration.as_secs_f64()
    );

    // Generate proof
    println!("4️⃣  Generating proof...");
    let prove_start = Instant::now();

    let (output, proof, _commitments) = prove_exec(input.program.id(), input.n);

    let prove_duration = prove_start.elapsed();
    println!(
        "   ✓ Proof generated in {:.2}s",
        prove_duration.as_secs_f64()
    );
    println!("   ✓ Result: {}\n", output);

    // Verify proof
    println!("5️⃣  Verifying proof...");
    let verify_start = Instant::now();

    let is_valid = verify_exec(input.program.id(), input.n, output, true, proof);

    let verify_duration = verify_start.elapsed();

    if is_valid {
        println!(
            "   ✓ Proof verified successfully in {:.2}s\n",
            verify_duration.as_secs_f64()
        );

        println!("========================================");
        println!("📈 Performance Summary");
        println!("========================================");
        println!("Compile time:     {:.2}s", compile_duration.as_secs_f64());
        println!(
            "Preprocess time:  {:.2}s",
            preprocess_duration.as_secs_f64()
        );
        println!("Build time:       {:.2}s", build_duration.as_secs_f64());
        println!("Prove time:       {:.2}s", prove_duration.as_secs_f64());
        println!("Verify time:      {:.2}s", verify_duration.as_secs_f64());
        println!("========================================");
        
        // Output BENCHMARK metrics
        println!("BENCHMARK: program_name={}_{}", input.program.as_str(), input.n);
        println!("BENCHMARK: zkvm_name=jolt");
        println!("BENCHMARK: zkvm_version=v1.0.0");
        println!("BENCHMARK: proof_mode=core");
        println!("BENCHMARK: compile_time_s={:.6}", compile_duration.as_secs_f64());
        println!("BENCHMARK: preprocess_time_s={:.6}", preprocess_duration.as_secs_f64());
        println!("BENCHMARK: build_time_s={:.6}", build_duration.as_secs_f64());
        println!("BENCHMARK: proof_time_s={:.6}", prove_duration.as_secs_f64());
        println!("BENCHMARK: output_result={}", output);
        println!("BENCHMARK: verification_time_s={:.6}", verify_duration.as_secs_f64());
        println!("BENCHMARK: success_status=success");
        println!("BENCHMARK: total_time_s={:.6}", (compile_duration + preprocess_duration + build_duration + prove_duration + verify_duration).as_secs_f64());
        
        println!("✅ Jolt zkVM Demo completed successfully!");
    } else {
        println!("BENCHMARK: success_status=failed");
        eprintln!("❌ Proof verification failed!");
        eprintln!("This should not happen with a correctly generated proof.");
        std::process::exit(1);
    }
}
