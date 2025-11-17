use anyhow::Result;
use colored::*;
use std::time::Instant;

fn main() -> Result<()> {
    println!("{}", "========================================".bright_cyan());
    println!("{}", "snarkVM Demo - Fibonacci Computation".bright_cyan().bold());
    println!("{}", "========================================".bright_cyan());
    println!();

    // Load fibonacci number from environment
    let fib_n = common::load_fib_n();
    println!("📊 Computing fibonacci({})...\n", fib_n);

    // Compute fibonacci using native computation
    println!("{}", "1️⃣  Computing Fibonacci natively...".bright_green());
    let compute_start = Instant::now();
    
    let result = compute_fibonacci(fib_n);
    
    let compute_duration = compute_start.elapsed();
    println!(
        "   ✓ Computation completed in {:.2}s",
        compute_duration.as_secs_f64()
    );
    println!("   ✓ Result: fibonacci({}) = {}\n", fib_n, result);

    // Demonstrate snarkVM field arithmetic
    println!("{}", "2️⃣  Demonstrating snarkVM field arithmetic...".bright_green());
    let field_start = Instant::now();
    
    demo_field_arithmetic()?;
    
    let field_duration = field_start.elapsed();
    println!(
        "   ✓ Field operations completed in {:.2}s\n",
        field_duration.as_secs_f64()
    );

    // Demonstrate snarkVM curve operations
    println!("{}", "3️⃣  Demonstrating snarkVM curve operations...".bright_green());
    let curve_start = Instant::now();
    
    demo_curve_operations()?;
    
    let curve_duration = curve_start.elapsed();
    println!(
        "   ✓ Curve operations completed in {:.2}s\n",
        curve_duration.as_secs_f64()
    );

    // Performance summary
    println!("{}", "========================================".bright_cyan());
    println!("{}", "📈 Performance Summary".bright_cyan().bold());
    println!("{}", "========================================".bright_cyan());
    println!("Fibonacci computation: {:.2}s", compute_duration.as_secs_f64());
    println!("Field arithmetic:      {:.2}s", field_duration.as_secs_f64());
    println!("Curve operations:      {:.2}s", curve_duration.as_secs_f64());
    println!(
        "Total time:            {:.2}s",
        (compute_duration + field_duration + curve_duration).as_secs_f64()
    );
    println!("{}", "========================================".bright_cyan());
    println!("{}", "✅ snarkVM Demo completed successfully!".bright_green().bold());
    println!("\n💡 About snarkVM:");
    println!("   snarkVM is the virtual machine powering Aleo blockchain");
    println!("   This demo showcases basic cryptographic primitives");
    println!("   For full zkVM features, use Leo programming language");
    println!("\n💡 Next Steps:");
    println!("   - Install Leo: https://leo-lang.org/");
    println!("   - Write Aleo programs in Leo language");
    println!("   - Deploy to Aleo testnet");
    println!("\n💡 Tip: Try running with different FIBONACCI_N values!");
    println!("   Example: FIBONACCI_N=20 cargo run --release");

    Ok(())
}

/// Compute Fibonacci number (native Rust implementation)
fn compute_fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut prev = 0u64;
    let mut curr = 1u64;
    
    for _ in 2..=n {
        let next = prev.wrapping_add(curr);
        prev = curr;
        curr = next;
    }
    
    curr
}

/// Demonstrate snarkVM field arithmetic operations
fn demo_field_arithmetic() -> Result<()> {
    use snarkvm::prelude::{Field, One, Uniform, Zero};
    use snarkvm::console::network::Testnet3;
    
    type CurrentField = <Testnet3 as snarkvm::console::network::Environment>::Field;
    
    // Create field elements
    let a = CurrentField::one();
    let b = CurrentField::from(2u64);
    let c = CurrentField::from(3u64);
    
    println!("   • Field element a = 1");
    println!("   • Field element b = 2");
    println!("   • Field element c = 3");
    
    // Perform arithmetic
    let sum = a + b;
    let product = b * c;
    let difference = c - a;
    
    println!("   • a + b = {}", sum);
    println!("   • b * c = {}", product);
    println!("   • c - a = {}", difference);
    
    // Field inversion
    let b_inv = b.inverse().unwrap();
    let should_be_one = b * b_inv;
    
    println!("   • b^(-1) exists (field inversion)");
    println!("   • b * b^(-1) = {} (should be 1)", should_be_one);
    
    Ok(())
}

/// Demonstrate snarkVM elliptic curve operations
fn demo_curve_operations() -> Result<()> {
    use snarkvm::prelude::{Group, One, Uniform, Zero};
    use snarkvm::console::network::Testnet3;
    
    type CurrentGroup = <Testnet3 as snarkvm::console::network::Environment>::Affine;
    
    // Get generator point
    let generator = CurrentGroup::generator();
    
    println!("   • Generator point G (base point on curve)");
    
    // Scalar multiplication
    let two_g = generator + generator;
    let three_g = two_g + generator;
    
    println!("   • Computed 2G (point doubling)");
    println!("   • Computed 3G (point addition)");
    
    // Verify group operation
    let g = generator;
    let result1 = g + g + g;
    let result2 = three_g;
    
    if result1 == result2 {
        println!("   • Verified: G + G + G = 3G ✓");
    } else {
        println!("   • Verification failed ✗");
    }
    
    // Scalar multiplication
    use snarkvm::prelude::Scalar;
    type CurrentScalar = <Testnet3 as snarkvm::console::network::Environment>::Scalar;
    
    let scalar = CurrentScalar::from(5u64);
    let five_g = generator * scalar;
    
    println!("   • Computed 5G (scalar multiplication)");
    
    // Verify scalar multiplication
    let manual_five_g = generator + generator + generator + generator + generator;
    if five_g == manual_five_g {
        println!("   • Verified: 5 * G = G + G + G + G + G ✓");
    }
    
    Ok(())
}

