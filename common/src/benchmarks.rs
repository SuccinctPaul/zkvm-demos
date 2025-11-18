//! Benchmark Programs for zkVM
//! 
//! This module provides various computation programs that can be
//! executed in zkVM environments for benchmarking purposes.
//! 
//! All programs are designed to be `no_std` compatible and optimized
//! for zero-knowledge proof generation.

#[cfg(feature = "crypto")]
use sha2::{Digest, Sha256};

/// Compute the nth Fibonacci number (iterative)
/// 
/// Time complexity: O(n)
/// Space complexity: O(1)
/// 
/// This is the recommended implementation for zkVM environments.
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0u32;
            let mut curr = 1u32;
            
            for _ in 2..=n {
                let next = prev.wrapping_add(curr);
                prev = curr;
                curr = next;
            }
            
            curr
        }
    }
}

/// Sum integers from 1 to n
/// 
/// Time complexity: O(n)
/// Space complexity: O(1)
pub fn sum(n: u32) -> u32 {
    let mut result = 0u32;
    for i in 1..=n {
        result = result.wrapping_add(i);
    }
    result
}

/// Compute n factorial
/// 
/// Time complexity: O(n)
/// Space complexity: O(1)
pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    
    let mut result = 1u32;
    for i in 2..=n {
        result = result.wrapping_mul(i);
    }
    result
}

/// Check if n is a prime number
/// 
/// Time complexity: O(√n)
/// Space complexity: O(1)
/// 
/// Returns 1 if prime, 0 otherwise
pub fn is_prime(n: u32) -> u32 {
    if n < 2 {
        return 0; // Not prime
    }
    if n == 2 {
        return 1; // Prime
    }
    if n % 2 == 0 {
        return 0; // Not prime (even)
    }
    
    // Check odd divisors up to sqrt(n)
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return 0; // Not prime
        }
        i += 2;
    }
    
    1 // Prime
}

/// Count the number of set bits (population count)
/// 
/// Time complexity: O(log n)
/// Space complexity: O(1)
pub fn popcount(mut n: u32) -> u32 {
    let mut count = 0u32;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Compute SHA256 hash of input data (requires "crypto" feature)
/// 
/// Returns the first 4 bytes of the hash as u32 for simplicity.
/// The parameter n is used to generate test data of length n bytes.
#[cfg(feature = "crypto")]
pub fn hash_sha256(n: u32) -> u32 {
    let mut hasher = Sha256::new();
    
    // Generate test data: repeat pattern based on n
    let data_len = n.min(1024); // Limit to 1KB for performance
    for i in 0..data_len {
        hasher.update(&[(i % 256) as u8]);
    }
    
    let result = hasher.finalize();
    
    // Return first 4 bytes as u32 (for simplicity in zkVM)
    u32::from_be_bytes([result[0], result[1], result[2], result[3]])
}

/// Stub for hash_sha256 when crypto feature is disabled
#[cfg(not(feature = "crypto"))]
pub fn hash_sha256(_n: u32) -> u32 {
    0 // Return dummy value
}

/// Verify ECDSA signature (simplified for zkVM benchmarking)
/// 
/// This is a simplified signature verification for zkVM benchmarking.
/// The parameter n controls the number of hash operations performed.
/// 
/// Returns 1 if verification succeeds, 0 otherwise.
#[cfg(feature = "crypto")]
pub fn verify_signature(n: u32) -> u32 {
    // Simulated public key hash
    let mut pubkey_hash = [0u8; 32];
    pubkey_hash[0] = 0x02; // Compressed pubkey prefix
    
    // Simulated message
    let message = b"zkVM signature benchmark";
    
    // Simulated signature components (r, s)
    let sig_r = [0x42u8; 32];
    let sig_s = [0x69u8; 32];
    
    // Perform n iterations of cryptographic operations
    let iterations = n.max(1).min(100);
    
    for i in 0..iterations {
        // Hash the message + nonce (simulating signature verification steps)
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&pubkey_hash);
        hasher.update(&sig_r);
        hasher.update(&sig_s);
        hasher.update(&[i as u8]);
        
        let hash_result = hasher.finalize();
        
        // Simulate verification check: hash should not be all zeros
        let mut is_zero = true;
        for &byte in hash_result.iter().take(4) {
            if byte != 0 {
                is_zero = false;
                break;
            }
        }
        
        if is_zero {
            return 0; // Verification failed
        }
    }
    
    1 // Verification succeeded
}

/// Stub for verify_signature when crypto feature is disabled
#[cfg(not(feature = "crypto"))]
pub fn verify_signature(_n: u32) -> u32 {
    1 // Return success
}

/// Execute a program based on ID
/// 
/// Program IDs:
/// - 0: Fibonacci
/// - 1: Sum
/// - 2: Factorial
/// - 3: IsPrime
/// - 4: PopCount
/// - 5: Hash (SHA256)
/// - 6: Signature (ECDSA)
pub fn execute_program(program_id: u32, n: u32) -> u32 {
    match program_id {
        0 => fibonacci(n),
        1 => sum(n),
        2 => factorial(n),
        3 => is_prime(n),
        4 => popcount(n),
        5 => hash_sha256(n),
        6 => verify_signature(n),
        _ => 0, // Unknown program
    }
}

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
    fn test_execute_program() {
        assert_eq!(execute_program(0, 10), fibonacci(10));
        assert_eq!(execute_program(1, 10), sum(10));
        assert_eq!(execute_program(2, 5), factorial(5));
        assert_eq!(execute_program(3, 17), is_prime(17));
        assert_eq!(execute_program(4, 15), popcount(15));
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_hash_sha256() {
        let hash1 = hash_sha256(100);
        let hash2 = hash_sha256(100);
        assert_eq!(hash1, hash2, "Hash should be deterministic");
        
        let hash3 = hash_sha256(200);
        assert_ne!(hash1, hash3, "Different inputs should give different hashes");
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_verify_signature() {
        let result = verify_signature(1);
        assert_eq!(result, 1, "Signature verification should succeed");
        
        let result = verify_signature(5);
        assert_eq!(result, 1, "Multiple signature verifications should succeed");
    }
}

