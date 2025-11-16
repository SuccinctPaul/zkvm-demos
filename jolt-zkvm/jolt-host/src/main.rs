use std::time::Instant;

pub fn main() {
    println!("========================================");
    println!("Jolt zkVM Demo - Fibonacci Computation");
    println!("========================================\n");

    // Load fibonacci number from environment
    let fib_n = common::load_fib_n();
    println!("📊 Computing fibonacci({})...\n", fib_n);

    // Compile the guest program
    println!("1️⃣  Compiling guest program...");
    let compile_start = Instant::now();
    
    let target_dir = "/tmp/jolt-guest-targets";
    let program = guest::compile_fibonacci(target_dir);
    
    let compile_duration = compile_start.elapsed();
    println!("   ✓ Compilation completed in {:.2}s\n", compile_duration.as_secs_f64());

    // Preprocessing
    println!("2️⃣  Preprocessing...");
    let preprocess_start = Instant::now();
    
    let prover_preprocessing = guest::preprocess_prover_fibonacci(&program);
    let verifier_preprocessing = guest::preprocess_verifier_fibonacci(&program);
    
    let preprocess_duration = preprocess_start.elapsed();
    println!("   ✓ Preprocessing completed in {:.2}s\n", preprocess_duration.as_secs_f64());

    // Build prover and verifier
    println!("3️⃣  Building prover and verifier...");
    let build_start = Instant::now();
    
    let prove_fibonacci = guest::build_prover_fibonacci(program, prover_preprocessing);
    let verify_fibonacci = guest::build_verifier_fibonacci(verifier_preprocessing);
    
    let build_duration = build_start.elapsed();
    println!("   ✓ Build completed in {:.2}s\n", build_duration.as_secs_f64());

    // Generate proof
    println!("4️⃣  Generating proof...");
    let prove_start = Instant::now();
    
    let (output, proof) = prove_fibonacci(fib_n);
    
    let prove_duration = prove_start.elapsed();
    println!("   ✓ Proof generated in {:.2}s", prove_duration.as_secs_f64());
    println!("   ✓ Result: fibonacci({}) = {}\n", fib_n, output);

    // Verify proof
    println!("5️⃣  Verifying proof...");
    let verify_start = Instant::now();
    
    let is_valid = verify_fibonacci(fib_n, output, proof);
    
    let verify_duration = verify_start.elapsed();
    
    if is_valid {
        println!("   ✓ Proof verified successfully in {:.2}s\n", verify_duration.as_secs_f64());
        
        println!("========================================");
        println!("📈 Performance Summary");
        println!("========================================");
        println!("Compile time:     {:.2}s", compile_duration.as_secs_f64());
        println!("Preprocess time:  {:.2}s", preprocess_duration.as_secs_f64());
        println!("Build time:       {:.2}s", build_duration.as_secs_f64());
        println!("Prove time:       {:.2}s", prove_duration.as_secs_f64());
        println!("Verify time:      {:.2}s", verify_duration.as_secs_f64());
        println!("Total time:       {:.2}s", 
            (compile_duration + preprocess_duration + build_duration + prove_duration + verify_duration).as_secs_f64());
        println!("========================================");
        println!("✅ Jolt zkVM Demo completed successfully!");
        println!("\n💡 Tip: Try running with different FIBONACCI_N values!");
        println!("   Example: FIBONACCI_N=15 cargo run --release");
    } else {
        eprintln!("❌ Proof verification failed!");
        eprintln!("This should not happen with a correctly generated proof.");
        std::process::exit(1);
    }
}
