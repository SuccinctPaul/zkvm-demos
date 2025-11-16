use anyhow::Result;
use colored::*;
use std::time::Instant;

use snarkvm::prelude::{
    Field, Testnet3, Uniform, R1CS, 
};
use snarkvm::circuit::{
    AleoV0, Environment, Assignment, Mode, Eject, Inject,
};
use snarkvm::console::program::Network;

type CurrentNetwork = Testnet3;
type CurrentAleo = AleoV0;

fn main() -> Result<()> {
    println!("{}", "========================================".bright_cyan());
    println!("{}", "snarkVM Demo - Fibonacci Computation".bright_cyan().bold());
    println!("{}", "========================================".bright_cyan());
    println!();

    // Load fibonacci number from environment
    let fib_n = common::load_fib_n();
    println!("📊 Computing fibonacci({})...\n", fib_n);

    // Initialize R1CS constraint system
    println!("{}", "1️⃣  Initializing constraint system...".bright_green());
    let init_start = Instant::now();
    
    // Create a new R1CS constraint system
    CurrentAleo::reset();
    
    let init_duration = init_start.elapsed();
    println!(
        "   ✓ Initialization completed in {:.2}s\n",
        init_duration.as_secs_f64()
    );

    // Setup circuit
    println!("{}", "2️⃣  Building circuit...".bright_green());
    let setup_start = Instant::now();

    // Compute fibonacci in circuit
    let result = fibonacci_circuit::<CurrentAleo>(fib_n)?;

    let setup_duration = setup_start.elapsed();
    println!(
        "   ✓ Circuit built in {:.2}s",
        setup_duration.as_secs_f64()
    );
    println!("   ✓ Result: fibonacci({}) = {}\n", fib_n, result);

    // Get constraint metrics
    println!("{}", "3️⃣  Analyzing circuit...".bright_green());
    let analyze_start = Instant::now();
    
    let num_public = CurrentAleo::num_public();
    let num_private = CurrentAleo::num_private();
    let num_constraints = CurrentAleo::num_constraints();
    
    let analyze_duration = analyze_start.elapsed();
    println!(
        "   ✓ Analysis completed in {:.2}s",
        analyze_duration.as_secs_f64()
    );
    println!("   📈 Circuit Statistics:");
    println!("      - Public inputs:    {}", num_public);
    println!("      - Private inputs:   {}", num_private);
    println!("      - Constraints:      {}", num_constraints);
    println!();

    // Verify circuit is satisfied
    println!("{}", "4️⃣  Verifying circuit satisfaction...".bright_green());
    let verify_start = Instant::now();
    
    let is_satisfied = CurrentAleo::is_satisfied();
    
    let verify_duration = verify_start.elapsed();
    
    if is_satisfied {
        println!(
            "   {} Circuit is satisfied in {:.2}s\n",
            "✓".bright_green(),
            verify_duration.as_secs_f64()
        );

        println!("{}", "========================================".bright_cyan());
        println!("{}", "📈 Performance Summary".bright_cyan().bold());
        println!("{}", "========================================".bright_cyan());
        println!("Initialize time:  {:.2}s", init_duration.as_secs_f64());
        println!("Setup time:       {:.2}s", setup_duration.as_secs_f64());
        println!("Analyze time:     {:.2}s", analyze_duration.as_secs_f64());
        println!("Verify time:      {:.2}s", verify_duration.as_secs_f64());
        println!(
            "Total time:       {:.2}s",
            (init_duration + setup_duration + analyze_duration + verify_duration).as_secs_f64()
        );
        println!("{}", "========================================".bright_cyan());
        println!("{}", "✅ snarkVM Demo completed successfully!".bright_green().bold());
        println!("\n💡 Tip: Try running with different FIBONACCI_N values!");
        println!("   Example: FIBONACCI_N=15 cargo run --release");
    } else {
        eprintln!("{}", "❌ Circuit is NOT satisfied!".bright_red().bold());
        eprintln!("This indicates an error in the circuit construction.");
        std::process::exit(1);
    }

    Ok(())
}

/// Compute Fibonacci number in a snarkVM circuit
fn fibonacci_circuit<A: Aleo>(n: u32) -> Result<u64> 
where
    <A::Network as Network>::Field: PrimeField,
{
    use snarkvm::circuit::prelude::*;
    
    // Convert input to circuit type
    let n_circuit = U32::<A>::new(Mode::Public, n);
    
    // Initialize first two fibonacci numbers
    let mut prev = U64::<A>::new(Mode::Private, 0);
    let mut curr = U64::<A>::new(Mode::Private, 1);
    
    // Compute fibonacci iteratively in the circuit
    for i in 0..n {
        let temp = curr.clone();
        // curr = prev + curr
        curr = prev.add(&curr);
        prev = temp;
    }
    
    // Extract the result
    let result = prev.eject_value();
    
    Ok(result)
}

/// Aleo trait to abstract over different Aleo versions
trait Aleo: Environment {
    type Network: Network;
}

impl Aleo for CurrentAleo {
    type Network = CurrentNetwork;
}

/// Import Field trait
use snarkvm::prelude::PrimeField;

