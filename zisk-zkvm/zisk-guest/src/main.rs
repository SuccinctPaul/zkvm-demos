//! ZisK zkVM Guest Program - Fibonacci Computation
//! 
//! This program runs inside the ZisK zero-knowledge virtual machine.
//! It computes Fibonacci numbers and commits the result to the public output.

#![no_main]
#![no_std]

// Import ZisK guest environment
use ziskos::*;

// Entry point for the ZisK guest program
ziskos::entry!(main);

pub fn main() {
    // Read input from the host
    let n: u32 = ziskos::io::read();
    
    // Log the input (for debugging)
    ziskos::io::log(&format!("Computing Fibonacci for n = {}", n));
    
    // Compute Fibonacci using the shared library
    let result = fib::fibonacci(n);
    
    // Log the result
    ziskos::io::log(&format!("Fibonacci({}) = {}", n, result));
    
    // Commit the result as public output
    // This makes the result part of the proof's public data
    ziskos::io::commit(&result);
}

