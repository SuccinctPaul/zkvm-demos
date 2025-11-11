// Cairo 2.x Fibonacci Main Program
// Demonstrates proving Fibonacci computation with zero-knowledge proofs

use cairo_fibonacci::{fib_iterative, fib_recursive, fib_pair};

fn main() -> (felt252, felt252, felt252) {
    // Compute Fibonacci for n = 10 using iterative method
    let n: felt252 = 10;
    let result = fib_iterative(n);
    
    // Also compute using recursive method (for small n)
    let n_small: felt252 = 8;
    let result_recursive = fib_recursive(n_small);
    
    // Return the results as public output
    // These values will be part of the proof
    (n, result, result_recursive)
}

// Example: Advanced computation with pair
fn compute_with_pair() -> (felt252, felt252) {
    let n: felt252 = 15;
    let (prev, curr) = fib_pair(n);
    (prev, curr)
}

