#![cfg_attr(feature = "guest", no_std)]
#![cfg_attr(feature = "guest", no_main)]

/// Compute the nth Fibonacci number
/// This function will be proven by Jolt zkVM
#[jolt::provable]
fn fibonacci(n: u32) -> u32 {
    fib::fibonacci(n)
}

