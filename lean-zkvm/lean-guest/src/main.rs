/// Lean zkVM Guest Program - Fibonacci Computation
/// 
/// This is a reference implementation showing the expected guest program structure
/// for leanMultisig zkVM. The actual implementation would use lean_prover APIs
/// once they are publicly available.
///
/// Note: This is a simplified version for demonstration purposes.
/// The actual guest program would run in a no_std environment with
/// custom memory allocation and panic handling when integrated with
/// the lean_prover SDK.

/// Iterative Fibonacci implementation (more efficient than recursive)
pub fn fibonacci_iterative(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

fn main() {
    // In the actual lean zkVM, input would be read from the execution context
    // For this reference implementation, we compute Fibonacci(10)
    let n = 10u32;
    
    let result = fibonacci_iterative(n);
    
    // In the actual lean zkVM, the result would be committed to the proof
    // This demonstrates the expected computation pattern
    
    println!("Guest: Computing Fibonacci({}) = {}", n, result);
    
    // The result would be: fib(10) = 55
    assert_eq!(result, 55, "Fibonacci computation failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_values() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(2), 1);
        assert_eq!(fibonacci_iterative(3), 2);
        assert_eq!(fibonacci_iterative(4), 3);
        assert_eq!(fibonacci_iterative(5), 5);
        assert_eq!(fibonacci_iterative(10), 55);
    }
}

