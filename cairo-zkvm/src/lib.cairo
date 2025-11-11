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

/// Compute the nth Fibonacci number iteratively (more efficient)
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
    
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;
    let mut i: felt252 = 2;
    
    loop {
        if i == n + 1 {
            break b;
        }
        
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    }
}

/// Compute Fibonacci pair (fib(n-1), fib(n))
/// 
/// # Arguments
/// * `n` - The position in the Fibonacci sequence
/// 
/// # Returns
/// A tuple containing (fib(n-1), fib(n))
pub fn fib_pair(n: felt252) -> (felt252, felt252) {
    if n == 0 {
        return (0, 0);
    }
    if n == 1 {
        return (0, 1);
    }
    
    let mut prev: felt252 = 0;
    let mut curr: felt252 = 1;
    let mut i: felt252 = 2;
    
    loop {
        if i == n + 1 {
            break (prev, curr);
        }
        
        let next = prev + curr;
        prev = curr;
        curr = next;
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{fib_recursive, fib_iterative, fib_pair};

    #[test]
    fn test_fib_recursive_base_cases() {
        assert(fib_recursive(0) == 0, 'fib(0) should be 0');
        assert(fib_recursive(1) == 1, 'fib(1) should be 1');
    }

    #[test]
    fn test_fib_recursive() {
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
        assert(fib_iterative(5) == 5, 'fib(5) should be 5');
        assert(fib_iterative(10) == 55, 'fib(10) should be 55');
        assert(fib_iterative(15) == 610, 'fib(15) should be 610');
    }

    #[test]
    fn test_fib_pair() {
        let (prev, curr) = fib_pair(10);
        assert(prev == 34, 'fib(9) should be 34');
        assert(curr == 55, 'fib(10) should be 55');
    }

    #[test]
    fn test_recursive_vs_iterative() {
        let n = 12;
        assert(fib_recursive(n) == fib_iterative(n), 'methods should match');
    }
}

