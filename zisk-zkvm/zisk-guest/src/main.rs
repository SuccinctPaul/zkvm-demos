//! ZisK zkVM Guest Program - Fibonacci Computation
//! 
//! This program runs inside the ZisK zero-knowledge virtual machine.
//! It computes Fibonacci numbers and commits the result to the public output.

// Disable the standard Rust entry point when targeting zkVM
#![cfg_attr(all(target_os = "zkvm", target_vendor = "zisk"), no_main)]

use ziskos::*;

// Entry point for the ZisK guest program (when targeting zkVM)
#[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
ziskos::entrypoint!(fibonacci_main);

// Regular main function (when building natively for testing)
#[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
fn main() {
    fibonacci_main();
}

pub fn fibonacci_main() {
    // Read input from the host (input.bin file)
    let input = read_input();
    
    // Parse the input as a u32 (little-endian)
    let n = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);
    
    println!("Computing Fibonacci for n = {}", n);
    
    // Compute Fibonacci using the shared library
    let result = fib::fibonacci(n);
    
    println!("Fibonacci({}) = {}", n, result);
    
    // Set the result as public output
    // This makes the result part of the proof's public data
    set_output(0, result);
}

