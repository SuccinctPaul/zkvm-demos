// Cairo 2.x Fibonacci Main Program
// Demonstrates proving Fibonacci computation with zero-knowledge proofs

use cairo_fibonacci::{fib_recursive, fib_iterative};

fn main() -> (felt252, felt252, felt252) {
    // Compute Fibonacci for n = 10
    let n: felt252 = 10;
    let result_recursive = fib_recursive(n);
    let result_iterative = fib_iterative(n);
    
    // Return the results as public output
    // These values will be part of the proof
    (n, result_recursive, result_iterative)
}

