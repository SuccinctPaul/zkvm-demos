// Cairo 2.x Fibonacci Library - Stwo-Cairo Compatible Version
// This version is optimized for Stwo-Cairo prover
// 
// Key differences from standard version:
// 1. Main function accepts parameters (required by stwo-cairo)
// 2. Uses simpler types (u32) for better performance
// 3. No gas tracking (disabled in Scarb.toml)

/// Compute the nth Fibonacci number iteratively
/// This is the most efficient implementation for zkVM proving
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// The nth Fibonacci number
pub fn fib_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut i: u32 = 2;
    
    loop {
        if i > n {
            break;
        }
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    };
    
    b
}

/// Compute the nth Fibonacci number recursively
/// Note: Less efficient than iterative, but demonstrates recursion
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// The nth Fibonacci number
pub fn fib_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

/// Compute both F(n) and F(n+1) efficiently
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// A tuple (F(n), F(n+1))
pub fn fib_pair(n: u32) -> (u32, u32) {
    if n == 0 {
        return (0, 1);
    }
    
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut i: u32 = 0;
    
    loop {
        if i >= n {
            break;
        }
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    };
    
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::{fib_recursive, fib_iterative, fib_pair};

    #[test]
    fn test_fib_recursive() {
        assert(fib_recursive(0) == 0, 'fib(0) should be 0');
        assert(fib_recursive(1) == 1, 'fib(1) should be 1');
        assert(fib_recursive(2) == 1, 'fib(2) should be 1');
        assert(fib_recursive(3) == 2, 'fib(3) should be 2');
        assert(fib_recursive(4) == 3, 'fib(4) should be 3');
        assert(fib_recursive(5) == 5, 'fib(5) should be 5');
        assert(fib_recursive(10) == 55, 'fib(10) should be 55');
    }

    #[test]
    fn test_fib_iterative() {
        assert(fib_iterative(0) == 0, 'fib(0) should be 0');
        assert(fib_iterative(1) == 1, 'fib(1) should be 1');
        assert(fib_iterative(2) == 1, 'fib(2) should be 1');
        assert(fib_iterative(3) == 2, 'fib(3) should be 2');
        assert(fib_iterative(4) == 3, 'fib(4) should be 3');
        assert(fib_iterative(5) == 5, 'fib(5) should be 5');
        assert(fib_iterative(10) == 55, 'fib(10) should be 55');
    }

    #[test]
    fn test_fib_pair() {
        let (a, b) = fib_pair(0);
        assert(a == 0 && b == 1, 'fib_pair(0) wrong');
        
        let (a, b) = fib_pair(1);
        assert(a == 1 && b == 1, 'fib_pair(1) wrong');
        
        let (a, b) = fib_pair(5);
        assert(a == 5 && b == 8, 'fib_pair(5) wrong');
        
        let (a, b) = fib_pair(10);
        assert(a == 55 && b == 89, 'fib_pair(10) wrong');
    }
}

/// Main function for Stwo-Cairo proof generation
/// 
/// This function is the entry point for generating zero-knowledge proofs
/// using the Stwo-Cairo prover. It accepts the input parameter and returns
/// the computation result.
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence to compute
/// 
/// # Returns
/// The nth Fibonacci number
/// 
/// # Example Usage with cairo-prove:
/// ```bash
/// # Generate proof for Fibonacci(10)
/// cairo-prove prove \
///   target/dev/cairo_fibonacci.executable.json \
///   ./proof.json \
///   --arguments 10
/// 
/// # Expected output in proof: 55
/// 
/// # Verify the proof
/// cairo-prove verify ./proof.json
/// ```
/// 
/// # Input Format:
/// When using --arguments, provide a single number:
///   --arguments 10
/// 
/// When using --arguments-file, provide JSON array:
///   ["0xa"]  // hex format for 10
fn main(n: u32) -> u32 {
    // Compute Fibonacci using the iterative method (most efficient)
    fib_iterative(n)
}

// Alternative main functions for different use cases:

/// Version 2: Return multiple results
/// Uncomment this and comment out the simple main() above to use
/*
fn main(n: u32) -> (u32, u32, u32) {
    let recursive_result = fib_recursive(n);
    let iterative_result = fib_iterative(n);
    let (fib_n, fib_n_plus_1) = fib_pair(n);
    
    // Returns: (recursive, iterative, pair_result)
    (recursive_result, iterative_result, fib_n)
}
*/

/// Version 3: Verify consistency between methods
/// This proves that both recursive and iterative give same result
/*
fn main(n: u32) -> bool {
    let recursive_result = fib_recursive(n);
    let iterative_result = fib_iterative(n);
    
    // Prove that both methods agree
    recursive_result == iterative_result
}
*/

