// Cairo 2.x Batch Fibonacci Computation Example
// Demonstrates computing multiple Fibonacci numbers efficiently

use cairo_fibonacci::fib_iterative;

/// Compute Fibonacci numbers for a range of values
/// Returns an array of results
fn compute_batch(start: felt252, end: felt252) -> Array<felt252> {
    let mut results = ArrayTrait::new();
    let mut i = start;
    
    loop {
        if i > end {
            break;
        }
        results.append(fib_iterative(i));
        i += 1;
    };
    
    results
}

fn main() -> Array<felt252> {
    // Compute Fibonacci numbers from 0 to 10
    compute_batch(0, 10)
}

// Expected output: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]

