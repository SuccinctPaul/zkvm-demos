//! SP1 Guest Program - Multi-Program Support
//!
//! This guest program supports multiple benchmark programs:
//! - Fibonacci, Sum, Factorial, IsPrime, PopCount, Hash, Signature

#![no_main]
sp1_zkvm::entrypoint!(main);

use common::execute_program;

pub fn main() {
    // Read program ID (0-6) and input parameter
    let program_id = sp1_zkvm::io::read::<u32>();
    let n = sp1_zkvm::io::read::<u32>();

    // Execute the selected program using common dispatcher
    let result = execute_program(program_id, n);

    // Commit the result as public output
    sp1_zkvm::io::commit(&result);
}
