//! Computation implementations for benchmark programs

use crate::Program;
use sha2::{Digest, Sha256};

// ============================================================================
// Logging Macros (std only)
// ============================================================================

#[cfg(feature = "std")]
macro_rules! log_info {
    ($($arg:tt)*) => {
        eprintln!("[zkvm_programs] {}", format!($($arg)*));
    };
}

#[cfg(not(feature = "std"))]
macro_rules! log_info {
    ($($arg:tt)*) => {};
}

// ============================================================================
// Main Entry Point
// ============================================================================

/// Execute a program with the given input
pub fn execute(program: Program, n: u32) -> u32 {
    log_info!("═══════════════════════════════════════════════════════════");
    log_info!("🚀 Executing: {} (id={}, n={})", program.name(), program.id(), n);
    log_info!("   Description: {}", program.description());
    log_info!("═══════════════════════════════════════════════════════════");

    let result = match program {
        Program::Fibonacci => fibonacci(n),
        Program::Sum => sum(n),
        Program::Factorial => factorial(n),
        Program::IsPrime => is_prime(n),
        Program::PopCount => popcount(n),
        Program::Hash => hash(n),
        Program::Signature => signature(n),
    };

    log_info!("───────────────────────────────────────────────────────────");
    log_info!("✅ Result: {}({}) = {}", program.name(), n, result);
    log_info!("───────────────────────────────────────────────────────────\n");

    result
}

/// Execute a program by ID (for guest programs)
#[inline]
pub fn execute_by_id(program_id: u32, n: u32) -> u32 {
    match Program::from_id(program_id) {
        Some(program) => execute(program, n),
        None => {
            log_info!("❌ Unknown program_id: {}", program_id);
            0
        }
    }
}

/// Backward compatibility alias for execute_by_id
#[inline]
pub fn execute_program(program_id: u32, n: u32) -> u32 {
    execute_by_id(program_id, n)
}

// ============================================================================
// Basic Computations
// ============================================================================

/// Compute the nth Fibonacci number
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn fibonacci(n: u32) -> u32 {
    log_info!("   Computing Fibonacci({})...", n);
    
    let result = match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0u32, 1u32);
            for _ in 2..=n {
                (a, b) = (b, a.wrapping_add(b));
            }
            b
        }
    };
    
    log_info!("   Fibonacci({}) = {}", n, result);
    result
}

/// Sum integers from 1 to n
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn sum(n: u32) -> u32 {
    log_info!("   Computing Sum(1..={})...", n);
    
    // Could use n*(n+1)/2, but loop is more representative for zkVM
    let result = (1..=n).fold(0u32, |acc, i| acc.wrapping_add(i));
    
    log_info!("   Sum(1..={}) = {}", n, result);
    result
}

/// Compute n factorial
///
/// Time: O(n), Space: O(1)
#[inline]
pub fn factorial(n: u32) -> u32 {
    log_info!("   Computing Factorial({})...", n);
    
    let result = (1..=n).fold(1u32, |acc, i| acc.wrapping_mul(i));
    
    log_info!("   Factorial({}) = {}", n, result);
    result
}

/// Check if n is prime
///
/// Returns 1 if prime, 0 otherwise.
/// Time: O(√n), Space: O(1)
#[inline]
pub fn is_prime(n: u32) -> u32 {
    log_info!("   Checking IsPrime({})...", n);
    
    let result = if n < 2 {
        0
    } else if n == 2 {
        1
    } else if n % 2 == 0 {
        0
    } else {
        let mut i = 3;
        let mut is_p = 1;
        while i * i <= n {
            if n % i == 0 {
                is_p = 0;
                break;
            }
            i += 2;
        }
        is_p
    };
    
    log_info!("   IsPrime({}) = {} ({})", n, result, if result == 1 { "PRIME" } else { "NOT PRIME" });
    result
}

/// Count set bits (population count)
///
/// Time: O(log n), Space: O(1)
#[inline]
pub fn popcount(n: u32) -> u32 {
    log_info!("   Computing PopCount({}) [binary: {:032b}]...", n, n);
    
    let mut val = n;
    let mut count = 0;
    while val > 0 {
        count += val & 1;
        val >>= 1;
    }
    
    log_info!("   PopCount({}) = {} set bits", n, count);
    count
}

// ============================================================================
// Cryptographic Operations
// ============================================================================

/// Compute SHA256 hash of n bytes of test data
///
/// Returns first 4 bytes of hash as u32.
/// Data length is capped at 1024 bytes.
pub fn hash(n: u32) -> u32 {
    let len = n.min(1024);
    log_info!("   Computing SHA256 hash of {} bytes...", len);
    
    let mut hasher = Sha256::new();

    // Generate deterministic test data
    for i in 0..len {
        hasher.update(&[(i % 256) as u8]);
    }

    let result = hasher.finalize();
    let hash_value = u32::from_be_bytes([result[0], result[1], result[2], result[3]]);
    
    log_info!("   SHA256({} bytes) = 0x{:08x}", len, hash_value);
    hash_value
}

/// Verify ECDSA signature (simulated)
///
/// Performs n iterations of hash operations to simulate signature verification.
/// Returns 1 if verification succeeds, 0 otherwise.
/// Iterations capped at 1-100.
pub fn signature(n: u32) -> u32 {
    let iterations = n.clamp(1, 100);
    log_info!("   Simulating ECDSA signature verification ({} iterations)...", iterations);

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
            log_info!("   ❌ Signature verification FAILED at iteration {}", i);
            return 0;
        }
    }

    log_info!("   ✅ Signature verification SUCCEEDED ({} iterations)", iterations);
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
    fn test_hash() {
        let h1 = hash(100);
        let h2 = hash(100);
        assert_eq!(h1, h2, "Hash should be deterministic");

        let h3 = hash(200);
        assert_ne!(h1, h3, "Different inputs should give different hashes");
    }

    #[test]
    fn test_signature() {
        assert_eq!(signature(1), 1);
        assert_eq!(signature(50), 1);
    }
}
