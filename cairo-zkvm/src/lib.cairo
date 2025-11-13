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

/// Compute the nth Fibonacci number iteratively
/// This is more efficient than the recursive version
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// The nth Fibonacci number
pub fn fib_iterative(n: felt252) -> felt252 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // Convert felt252 to u32 for comparison
    let n_u32: u32 = n.try_into().unwrap();
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;
    let mut i: u32 = 2;
    
    loop {
        if i > n_u32 {
            break;
        }
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    };
    
    b
}

/// Compute the nth and (n+1)th Fibonacci numbers as a pair
/// This is useful for efficient sequential computation
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// A tuple (F(n), F(n+1))
pub fn fib_pair(n: felt252) -> (felt252, felt252) {
    if n == 0 {
        return (0, 1);
    }
    
    // Convert felt252 to u32 for comparison
    let n_u32: u32 = n.try_into().unwrap();
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;
    let mut i: u32 = 0;
    
    loop {
        if i >= n_u32 {
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

    #[test]
    fn test_recursive_vs_iterative() {
        assert(fib_recursive(7) == fib_iterative(7), 'fib methods mismatch at 7');
        assert(fib_recursive(8) == fib_iterative(8), 'fib methods mismatch at 8');
        assert(fib_recursive(9) == fib_iterative(9), 'fib methods mismatch at 9');
    }
}

/// Main function for Cairo program execution
/// This demonstrates computation that can be proven with STARK proofs
fn main() -> (felt252, felt252, felt252, felt252, felt252) {
    // Compute Fibonacci for n = 10
    let n: felt252 = 10;
    let result_recursive = fib_recursive(n);
    let result_iterative = fib_iterative(n);
    
    // Get a pair of consecutive Fibonacci numbers
    let (fib_n, fib_n_plus_1) = fib_pair(n);
    
    // Return the results as public output
    // These values will be part of the proof
    // Expected: (10, 55, 55, 55, 89)
    (n, result_recursive, result_iterative, fib_n, fib_n_plus_1)
}