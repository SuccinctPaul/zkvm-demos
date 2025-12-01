//! CENO zkVM Guest Program - Multi-Program Support
//! 
//! This guest program runs inside the CENO zkVM and executes
//! various benchmark programs (Fibonacci, Sum, etc.)
//!
//! CENO is developed by Scroll and uses GKR protocol for
//! accelerated zero-knowledge proofs.

#![no_std]
#![no_main]

use ceno_rt::println;
use zkvm_programs::execute_program;

/// Main entry point for CENO guest program
/// 
/// Reads program_id and n from hints, executes the program,
/// and outputs the result.
#[ceno_rt::main]
fn main() {
    // Read inputs from hints
    // CENO uses hints for private inputs
    let program_id: u32 = ceno_rt::read();
    let n: u32 = ceno_rt::read();
    
    println!("=== CENO zkVM Guest Program ===");
    println!("Program ID: {}", program_id);
    println!("Input N: {}", n);
    
    // Execute the selected program
    let result = execute_program(program_id, n);
    
    println!("Result: {}", result);
    println!("=== Computation Complete ===");
    
    // Write result to public output
    ceno_rt::write(&result);
}
