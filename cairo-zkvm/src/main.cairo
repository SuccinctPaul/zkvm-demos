// Cairo 2.x Fibonacci Main Program
// Demonstrates proving Fibonacci computation with zero-knowledge proofs

use cairo_fibonacci::{fib_recursive};

fn main() -> (felt252, felt252, felt252) {
    // Compute Fibonacci for n = 10 using iterative method
    let n: felt252 = 10;
    let result_recursive = fib_recursive(n_small);
    
    // Return the results as public output
    // These values will be part of the proof
    (n,  result_recursive)
}

