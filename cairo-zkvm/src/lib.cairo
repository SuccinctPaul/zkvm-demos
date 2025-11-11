// Cairo 2.x Fibonacci Library
// This library provides Fibonacci number computation functions

/// Compute the nth Fibonacci number recursively
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// The nth Fibonacci number
pub fn fib_recursive(n: felt252) -> felt252 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    fib_recursive(n - 1) + fib_recursive(n - 2)
}