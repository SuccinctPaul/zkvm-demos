//! Novanet zkVM Guest Program
//! 
//! This guest program computes the nth Fibonacci number
//! in a zero-knowledge proof environment using Novanet zkVM.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibInput {
    pub n: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibOutput {
    pub result: u32,
}

/// Compute the nth Fibonacci number
/// This function will be proven by Novanet zkVM
pub fn compute_fibonacci(input: FibInput) -> FibOutput {
    let result = fib::fibonacci(input.n);
    FibOutput { result }
}

/// Guest program entry point for Novanet zkVM
/// This would be called by the Novanet zkVM runtime
pub fn guest_main(n: u32) -> u32 {
    let input = FibInput { n };
    let output = compute_fibonacci(input);
    output.result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let input = FibInput { n: 10 };
        let output = compute_fibonacci(input);
        assert_eq!(output.result, 89);
    }
}

