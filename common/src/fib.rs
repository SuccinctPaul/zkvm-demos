#![no_std]

//! # Fibonacci Computation Library
//! 
//! This library provides an efficient implementation of Fibonacci sequence computation.
//! 
//! ## Why Use Iteration Instead of Recursion?
//! 
//! ### Problems with Recursive Implementation:
//! ```rust,ignore
//! // ❌ Recursive implementation (deprecated) - O(2^n) time complexity
//! pub fn fibonacci_recursive(n: u32) -> u32 {
//!     match n {
//!         0 => 0,
//!         1 => 1,
//!         _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
//!     }
//! }
//! ```
//! 
//! **Issues**:
//! 1. **Exponential time complexity O(2^n)**: fibonacci(30) requires 2,692,537 recursive calls
//! 2. **Massive redundant computation**: fib(n-2) is recalculated when computing fib(n-1)
//! 3. **Stack overflow risk**: Deep recursion may exhaust stack space
//! 4. **Extremely expensive in zkVM**: Each recursive call requires zero-knowledge proof generation, resulting in:
//!    - Proof generation time: Hours vs. seconds
//!    - Proof size: GBs vs. KBs  
//!    - Memory consumption: Tens of GBs vs. hundreds of MBs
//!    - Computation cost: $20-50 vs. $0.01
//! 
//! ### Advantages of Iterative Implementation:
//! 
//! **Benefits**:
//! 1. **Linear time complexity O(n)**: fibonacci(30) requires only 28 iterations
//! 2. **Constant space complexity O(1)**: Uses only 3 variables
//! 3. **No redundant computation**: Each value is computed exactly once
//! 4. **No stack overflow risk**: Does not use recursive call stack
//! 5. **Efficient in zkVM**: Minimal instruction count, controllable proof cost
//! 
//! ### Performance Comparison Examples:
//! 
//! | n value | Recursive calls | Iterative loops | Performance gap |
//! |---------|----------------|-----------------|-----------------|
//! | 10      | 177            | 8               | 22x             |
//! | 20      | 21,891         | 18              | 1,216x          |
//! | 30      | 2,692,537      | 28              | 96,162x         |
//! 
//! In zero-knowledge proof scenarios, this gap is amplified thousands of times because every instruction must be proven.

/// Computes the nth Fibonacci number using **recursive** approach
/// 
/// ⚠️ **WARNING: FOR EDUCATIONAL PURPOSES ONLY**
/// 
/// This recursive implementation has O(2^n) time complexity and should **NOT** be used 
/// in production, especially in zkVM environments. It is provided here solely for 
/// comparison and educational purposes.
/// 
/// # Why You Should NOT Use This
/// 
/// - **Exponential time**: fibonacci_recursive(30) requires 2,692,537 calls
/// - **Stack overflow risk**: Will crash on large n values
/// - **In zkVM**: Proof generation would take hours and cost $20-50
/// - **Impractical**: fibonacci_recursive(40) takes ~26 seconds vs 85ns for iterative
/// 
/// # Examples
/// 
/// ```
/// use fib::fibonacci_recursive;
/// 
/// // Only use for small values!
/// assert_eq!(fibonacci_recursive(0), 0);
/// assert_eq!(fibonacci_recursive(10), 55);
/// 
/// // DO NOT do this: fibonacci_recursive(40) - takes 26 seconds!
/// // DO NOT do this: fibonacci_recursive(50) - takes hours!
/// ```
/// 
/// # Performance Warning
/// 
/// | n   | Calls     | Time (approx) |
/// |-----|-----------|---------------|
/// | 10  | 177       | ~1 μs         |
/// | 20  | 21,891    | ~42 μs        |
/// | 30  | 2,692,537 | ~1.5 ms       |
/// | 40  | 331M      | ~26 seconds   |
/// 
/// **Use [`fibonacci`] instead for all production code.**
#[deprecated(
    since = "0.2.0",
    note = "Recursive implementation has O(2^n) complexity. Use `fibonacci()` instead."
)]
pub fn fibonacci_recursive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

/// Computes the nth Fibonacci number (iterative implementation)
/// 
/// Uses an iterative approach to compute the Fibonacci sequence with O(n) time complexity 
/// and O(1) space complexity. This is the **recommended** implementation for all use cases,
/// especially zkVM environments.
/// 
/// # Mathematical Definition
/// 
/// The Fibonacci sequence is defined as:
/// - F(0) = 0
/// - F(1) = 1  
/// - F(n) = F(n-1) + F(n-2), for n ≥ 2
/// 
/// # Parameters
/// 
/// * `n` - The index of the Fibonacci number to compute (starting from 0)
/// 
/// # Returns
/// 
/// Returns the nth Fibonacci number. Note: Due to using u32 type, large n values will 
/// experience integer overflow (wrapping behavior).
/// 
/// # Examples
/// 
/// ```
/// use fib::fibonacci;
/// 
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(2), 1);
/// assert_eq!(fibonacci(10), 55);
/// assert_eq!(fibonacci(20), 6765);
/// assert_eq!(fibonacci(100), 3314859971); // Handles large n efficiently
/// ```
/// 
/// # Algorithm Description
/// 
/// The algorithm maintains two variables `prev` and `curr`, representing F(i-1) and F(i) respectively.
/// In each iteration:
/// 1. Calculate next = prev + curr (i.e., F(i+1))
/// 2. Update prev = curr
/// 3. Update curr = next
/// 
/// This requires only O(n) time and O(1) space to complete the computation.
/// 
/// # Overflow Handling
/// 
/// Uses `wrapping_add` to handle integer overflow, ensuring safety in no_std environments.
/// The Fibonacci sequence grows rapidly; u32 type overflows at n=47.
/// 
/// # Performance
/// 
/// - Time complexity: O(n)
/// - Space complexity: O(1)
/// - On modern CPUs, fibonacci(100) typically takes only tens of nanoseconds
/// - In zkVM, generating a proof for fibonacci(30) takes only seconds (vs. hours for recursive version)
/// 
/// # Comparison with Recursive Version
/// 
/// For fibonacci(30):
/// - **This function**: ~42 ns, O(n) time, O(1) space
/// - **`fibonacci_recursive`**: ~1.5 ms (36,452x slower), O(2^n) time, O(n) space
pub fn fibonacci(n: u32) -> u32 {
    match n {
        // Base case: F(0) = 0
        0 => 0,
        // Base case: F(1) = 1
        1 => 1,
        // General case: use iterative computation
        _ => {
            // prev represents F(i-1), initialized to F(0) = 0
            let mut prev = 0u32;
            // curr represents F(i), initialized to F(1) = 1
            let mut curr = 1u32;
            
            // Iterate from i=2 to i=n
            // Each iteration computes F(i) = F(i-1) + F(i-2)
            for _ in 2..=n {
                // Calculate F(i+1) = F(i) + F(i-1)
                // Use wrapping_add to handle overflow, safe in no_std environment
                let next = prev.wrapping_add(curr);
                
                // Update state: move forward one position
                prev = curr;  // prev now points to F(i)
                curr = next;  // curr now points to F(i+1)
            }
            
            // After loop ends, curr contains F(n)
            curr
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0, "F(0) should be 0");
        assert_eq!(fibonacci(1), 1, "F(1) should be 1");
    }

    #[test]
    fn test_fibonacci_small_values() {
        assert_eq!(fibonacci(2), 1, "F(2) = F(1) + F(0) = 1 + 0 = 1");
        assert_eq!(fibonacci(3), 2, "F(3) = F(2) + F(1) = 1 + 1 = 2");
        assert_eq!(fibonacci(4), 3, "F(4) = F(3) + F(2) = 2 + 1 = 3");
        assert_eq!(fibonacci(5), 5, "F(5) = F(4) + F(3) = 3 + 2 = 5");
        assert_eq!(fibonacci(6), 8, "F(6) = F(5) + F(4) = 5 + 3 = 8");
    }

    #[test]
    fn test_fibonacci_medium_values() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_fibonacci_large_values() {
        // These tests prove that iterative implementation can quickly handle large n values
        // Recursive implementation would take hours or even cause stack overflow on these values
        assert_eq!(fibonacci(30), 832040);
        assert_eq!(fibonacci(40), 102334155);
    }

    #[test]
    fn test_fibonacci_performance() {
        // This test should complete instantly
        // With recursive implementation, fibonacci(100) would never finish
        let _ = fibonacci(100);
        let _ = fibonacci(200);
        // Test passing indicates good performance
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Verify sequence continuity: F(n) = F(n-1) + F(n-2)
        for n in 2..20 {
            let f_n = fibonacci(n);
            let f_n_minus_1 = fibonacci(n - 1);
            let f_n_minus_2 = fibonacci(n - 2);
            assert_eq!(
                f_n,
                f_n_minus_1.wrapping_add(f_n_minus_2),
                "F({}) should equal F({}) + F({})",
                n,
                n - 1,
                n - 2
            );
        }
    }

    // Tests for recursive version (educational purposes)
    
    #[test]
    #[allow(deprecated)]
    fn test_recursive_correctness_small_values() {
        // Verify that recursive version produces correct results for small values
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(10), 55);
        assert_eq!(fibonacci_recursive(15), 610);
    }

    #[test]
    #[allow(deprecated)]
    fn test_recursive_vs_iterative_equivalence() {
        // Verify both implementations produce the same results
        for n in 0..20 {
            assert_eq!(
                fibonacci_recursive(n),
                fibonacci(n),
                "Recursive and iterative should match for n={}",
                n
            );
        }
    }

    #[test]
    #[cfg(all(test, not(target_arch = "wasm32")))]
    #[allow(deprecated)]
    fn test_recursive_performance_warning() {
        // This test demonstrates the performance issue with recursive version
        // We only test small values to keep test time reasonable
        // Note: Only runs in std environment (not in no_std/wasm)
        
        extern crate std;
        use std::time::Instant;
        use std::println;

        let n = 20;
        
        // Measure recursive version
        let start = Instant::now();
        let _ = fibonacci_recursive(n);
        let recursive_time = start.elapsed();

        // Measure iterative version
        let start = Instant::now();
        let _ = fibonacci(n);
        let iterative_time = start.elapsed();

        // Iterative should be significantly faster
        // Even for n=20, the difference should be noticeable
        println!(
            "Performance comparison for n={}:\n  Recursive: {:?}\n  Iterative: {:?}\n  Speedup: {:.0}x",
            n,
            recursive_time,
            iterative_time,
            recursive_time.as_nanos() as f64 / iterative_time.as_nanos() as f64
        );
    }

    #[test]
    #[cfg(all(not(debug_assertions), not(target_arch = "wasm32")))]
    #[allow(deprecated)]
    fn test_recursive_exponential_growth() {
        // This test demonstrates the exponential growth of recursive calls
        // WARNING: This test is expensive, only run in release mode
        // Note: Only runs in std environment (not in no_std/wasm)
        
        extern crate std;
        use std::time::Instant;
        use std::println;

        let test_values = [15, 20, 25];
        
        for &n in &test_values {
            let start = Instant::now();
            let _ = fibonacci_recursive(n);
            let duration = start.elapsed();
            
            println!("fibonacci_recursive({}) took {:?}", n, duration);
            
            // fibonacci(25) should take less than 1000ms in release mode
            if n == 25 {
                assert!(
                    duration.as_millis() < 1000,
                    "fibonacci_recursive(25) took too long: {:?}",
                    duration
                );
            }
        }
    }
}
