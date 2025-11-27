//! Computation implementations for benchmark programs

use crate::Program;

#[cfg(feature = "crypto")]
use sha2::{Digest, Sha256};

// ============================================================================
// Main Entry Point
// ============================================================================

/// Execute a program with the given input
#[inline]
pub fn execute(program: Program, n: u32) -> u32 {
    match program {
        Program::Fibonacci => fibonacci(n),
        Program::Sum => sum(n),
        Program::Factorial => factorial(n),
        Program::IsPrime => is_prime(n),
        Program::PopCount => popcount(n),
        Program::Hash => hash(n),
        Program::Signature => signature(n),
    }
}

/// Execute a program by ID (for guest programs)
#[inline]
pub fn execute_by_id(program_id: u32, n: u32) -> u32 {
    Program::from_id(program_id)
        .map(|p| execute(p, n))
        .unwrap_or(0)
}

// ============================================================================
// Basic Computations
// ============================================================================

/// Compute the nth Fibonacci number
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u32, 1u32);
            for _ in 2..=n {
                (a, b) = (b, a.wrapping_add(b));
            }
            b
        }
    }
}

/// Sum integers from 1 to n
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn sum(n: u32) -> u32 {
    // Could use n*(n+1)/2, but loop is more representative for zkVM
    (1..=n).fold(0u32, |acc, i| acc.wrapping_add(i))
}

/// Compute n factorial
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn factorial(n: u32) -> u32 {
    (1..=n).fold(1u32, |acc, i| acc.wrapping_mul(i))
}

/// Check if n is prime
///
/// Returns 1 if prime, 0 otherwise.
/// Time: O(√n), Space: O(1)
#[inline]
pub fn is_prime(n: u32) -> u32 {
    if n < 2 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    if n % 2 == 0 {
        return 0;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return 0;
        }
        i += 2;
    }
    1
}

/// Count set bits (population count)
///
/// Time: O(log n), Space: O(1)
#[inline]
pub fn popcount(mut n: u32) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

// ============================================================================
// Cryptographic Operations
// ============================================================================

/// Compute SHA256 hash of n bytes of test data
///
/// Returns first 4 bytes of hash as u32.
/// Data length is capped at 1024 bytes.
#[cfg(feature = "crypto")]
pub fn hash(n: u32) -> u32 {
    let len = n.min(1024);
    let mut hasher = Sha256::new();

    // Generate deterministic test data
    for i in 0..len {
        hasher.update(&[(i % 256) as u8]);
    }

    let result = hasher.finalize();
    u32::from_be_bytes([result[0], result[1], result[2], result[3]])
}

#[cfg(not(feature = "crypto"))]
pub fn hash(_n: u32) -> u32 {
    0
}

/// Verify ECDSA signature (simulated)
///
/// Performs n iterations of hash operations.
/// Returns 1 if verification succeeds, 0 otherwise.
/// Iterations capped at 1-100.
#[cfg(feature = "crypto")]
pub fn signature(n: u32) -> u32 {
    let iterations = n.clamp(1, 100);

    // Simulated signature data
    let pubkey = [0x02u8; 32];
    let message = b"zkVM signature benchmark";
    let sig_r = [0x42u8; 32];
    let sig_s = [0x69u8; 32];

    for i in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&pubkey);
        hasher.update(&sig_r);
        hasher.update(&sig_s);
        hasher.update(&[i as u8]);

        let result = hasher.finalize();

        // Check hash is not all zeros (would never happen in practice)
        if result[..4].iter().all(|&b| b == 0) {
            return 0;
        }
    }

    1
}

#[cfg(not(feature = "crypto"))]
pub fn signature(_n: u32) -> u32 {
    1
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(0), 0);
        assert_eq!(sum(1), 1);
        assert_eq!(sum(10), 55);
        assert_eq!(sum(100), 5050);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(0), 0);
        assert_eq!(is_prime(1), 0);
        assert_eq!(is_prime(2), 1);
        assert_eq!(is_prime(3), 1);
        assert_eq!(is_prime(4), 0);
        assert_eq!(is_prime(17), 1);
        assert_eq!(is_prime(97), 1);
        assert_eq!(is_prime(100), 0);
    }

    #[test]
    fn test_popcount() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(7), 3);
        assert_eq!(popcount(15), 4);
        assert_eq!(popcount(255), 8);
    }

    #[test]
    fn test_execute() {
        assert_eq!(execute(Program::Fibonacci, 10), 55);
        assert_eq!(execute(Program::Sum, 10), 55);
        assert_eq!(execute(Program::Factorial, 5), 120);
        assert_eq!(execute(Program::IsPrime, 17), 1);
        assert_eq!(execute(Program::PopCount, 15), 4);
    }

    #[test]
    fn test_execute_by_id() {
        assert_eq!(execute_by_id(0, 10), 55); // Fibonacci
        assert_eq!(execute_by_id(1, 10), 55); // Sum
        assert_eq!(execute_by_id(99, 10), 0); // Invalid
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_hash() {
        let h1 = hash(100);
        let h2 = hash(100);
        assert_eq!(h1, h2, "Hash should be deterministic");

        let h3 = hash(200);
        assert_ne!(h1, h3, "Different inputs should give different hashes");
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_signature() {
        assert_eq!(signature(1), 1);
        assert_eq!(signature(50), 1);
    }
}

