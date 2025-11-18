//! SP1 Guest Program - Multi-Program Support
//!
//! This guest program supports multiple benchmark programs:
//! - Fibonacci, Sum, Factorial, IsPrime, PopCount, Hash, Signature

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::{fibonacci, sum, factorial, is_prime, popcount, hash_sha256, verify_signature};

pub fn main() {
    // Read program ID (0-6) and input parameter
    let program_id = sp1_zkvm::io::read::<u32>();
    let n = sp1_zkvm::io::read::<u32>();

    println!("SP1 Guest: program_id={}, n={}", program_id, n);

    // Execute the selected program
    let result = match program_id {
        0 => {
            println!("Running Fibonacci({})", n);
            fibonacci(n)
        }
        1 => {
            println!("Running Sum({})", n);
            sum(n)
        }
        2 => {
            println!("Running Factorial({})", n);
            factorial(n)
        }
        3 => {
            println!("Running IsPrime({})", n);
            is_prime(n)
        }
        4 => {
            println!("Running PopCount({})", n);
            popcount(n)
        }
        5 => {
            println!("Running Hash({})", n);
            hash_sha256(n)
        }
        6 => {
            println!("Running Signature({})", n);
            verify_signature(n)
        }
        _ => {
            println!("Unknown program_id: {}, defaulting to Fibonacci", program_id);
            fibonacci(n)
        }
    };

    println!("Result: {}", result);

    // Commit the result as public output
    sp1_zkvm::io::commit(&result);
}
