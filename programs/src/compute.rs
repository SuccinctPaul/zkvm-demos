//! Computation implementations for benchmark programs
//!
//! # Input Parameter Semantics
//!
//! | Function    | Input `n`                            | Output           |
//! |-------------|--------------------------------------|------------------|
//! | fibonacci   | Index (0-indexed)                    | nth Fib number   |
//! | sum         | Upper bound                          | 1+2+...+n        |
//! | factorial   | Number                               | n!               |
//! | is_prime    | Number to test                       | 1=prime, 0=not   |
//! | popcount    | Number                               | Set bit count    |
//! | hash        | Bytes to hash (max 1024)             | Hash as u32      |
//! | signature   | Signing iterations (1-100)           | 1=success, 0=fail|

use crate::Program;
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
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
///
/// The meaning of `n` depends on the program:
/// - Fibonacci: index in sequence (n=20 → 6765)
/// - Sum: upper bound (n=100 → 5050)
/// - Factorial: number (n=10 → 3628800)
/// - IsPrime: number to test (returns 0 or 1)
/// - PopCount: number to count bits in
/// - Hash: bytes to hash (max 1024)
/// - Signature: iterations (1-100)
pub fn execute(program: Program, n: u32) -> u32 {
    log_info!("═══════════════════════════════════════════════════════════");
    log_info!("🚀 Executing: {} (id={})", program.name(), program.id());
    log_info!("   Input: n = {}", n);
    log_info!("   Meaning: {}", program.input_description());
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
///
/// This is the primary entry point for zkVM guest programs.
/// Program IDs: 0=Fibonacci, 1=Sum, 2=Factorial, 3=IsPrime,
///              4=PopCount, 5=Hash, 6=Signature
#[inline]
pub fn execute_by_id(program_id: u32, n: u32) -> u32 {
    match Program::from_id(program_id) {
        Some(program) => execute(program, n),
        None => {
            log_info!("❌ Unknown program_id: {} (valid: 0-6)", program_id);
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

/// Compute the nth Fibonacci number (0-indexed)
///
/// # Input
/// - `n`: Index in Fibonacci sequence (0-indexed)
///   - n=0 → 0
///   - n=1 → 1
///   - n=10 → 55
///   - n=20 → 6765
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
///
/// # Overflow
/// Uses wrapping arithmetic. Results wrap around for n > 47 (u32).
#[inline]
pub fn fibonacci(n: u32) -> u32 {
    log_info!("   [fibonacci] Computing F({}) iteratively", n);
    log_info!("   [fibonacci] Formula: F(0)=0, F(1)=1, F(n)=F(n-1)+F(n-2)");

    let result = match n {
        0 => {
            log_info!("   [fibonacci] Base case: F(0) = 0");
            0
        }
        1 => {
            log_info!("   [fibonacci] Base case: F(1) = 1");
            1
        }
        _ => {
            let (mut a, mut b) = (0u32, 1u32);
            for _ in 2..=n {
                let next = a.wrapping_add(b);
                a = b;
                b = next;
            }
            log_info!("   [fibonacci] Computed {} iterations", n - 1);
            b
        }
    };

    log_info!("   [fibonacci] Result: F({}) = {}", n, result);
    result
}

/// Sum integers from 1 to n
///
/// # Input
/// - `n`: Upper bound of summation (inclusive)
///   - n=0 → 0
///   - n=10 → 1+2+...+10 = 55
///   - n=100 → 5050
///
/// # Complexity
/// - Time: O(n) - intentionally uses loop for zkVM benchmarking
/// - Space: O(1)
///
/// # Note
/// Uses iterative loop instead of formula n*(n+1)/2 to better
/// represent typical zkVM workload.
#[inline]
pub fn sum(n: u32) -> u32 {
    log_info!("   [sum] Computing 1 + 2 + ... + {}", n);

    let result = (1..=n).fold(0u32, |acc, i| acc.wrapping_add(i));

    // Verify with formula (for logging only)
    #[cfg(feature = "std")]
    {
        let expected = (n as u64 * (n as u64 + 1) / 2) as u32;
        log_info!(
            "   [sum] Loop result: {}, Formula n(n+1)/2: {}",
            result,
            expected
        );
        if result != expected {
            log_info!("   [sum] ⚠️  Mismatch due to overflow!");
        }
    }

    log_info!("   [sum] Result: Sum(1..={}) = {}", n, result);
    result
}

/// Compute n factorial (n!)
///
/// # Input
/// - `n`: Number to compute factorial of
///   - n=0 → 1 (by definition: 0! = 1)
///   - n=5 → 120
///   - n=10 → 3628800
///   - n=12 → 479001600 (max for u32)
///   - n=13+ → overflow (uses wrapping)
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
#[inline]
pub fn factorial(n: u32) -> u32 {
    log_info!("   [factorial] Computing {}!", n);

    if n == 0 {
        log_info!("   [factorial] Base case: 0! = 1 (by definition)");
        return 1;
    }

    let result = (1..=n).fold(1u32, |acc, i| acc.wrapping_mul(i));

    if n > 12 {
        log_info!(
            "   [factorial] ⚠️  {}! overflows u32 (max exact: 12! = 479001600)",
            n
        );
        log_info!("   [factorial] Result is wrapped/truncated");
    }

    log_info!("   [factorial] Result: {}! = {}", n, result);
    result
}

/// Check if n is a prime number
///
/// # Input
/// - `n`: Number to check for primality
///
/// # Output
/// - Returns `1` if n is prime
/// - Returns `0` if n is not prime
///
/// # Algorithm
/// Trial division up to √n, checking only odd divisors after 2.
///
/// # Complexity
/// - Time: O(√n)
/// - Space: O(1)
#[inline]
pub fn is_prime(n: u32) -> u32 {
    log_info!("   [is_prime] Testing primality of {}", n);

    // Handle small cases
    if n < 2 {
        log_info!("   [is_prime] {} < 2: NOT PRIME", n);
        return 0;
    }
    if n == 2 {
        log_info!("   [is_prime] 2 is the smallest prime: PRIME");
        return 1;
    }
    if n % 2 == 0 {
        log_info!("   [is_prime] {} is even (divisible by 2): NOT PRIME", n);
        return 0;
    }

    // Check odd divisors from 3 to √n
    let mut i = 3u32;
    while i.saturating_mul(i) <= n {
        if n % i == 0 {
            log_info!("   [is_prime] {} = {} × {}: NOT PRIME", n, i, n / i);
            return 0;
        }
        i += 2;
    }

    log_info!(
        "   [is_prime] No divisors found up to √{} ≈ {}: PRIME",
        n,
        (n as f64).sqrt() as u32
    );
    1
}

/// Count set bits in n (population count / Hamming weight)
///
/// # Input
/// - `n`: Number to count set bits in
///
/// # Output
/// - Number of 1-bits in binary representation
///
/// # Examples
/// - n=0 → 0 (no bits set)
/// - n=7 (0b111) → 3
/// - n=255 (0b11111111) → 8
/// - n=0xFFFFFFFF → 32
///
/// # Complexity
/// - Time: O(log n) = O(32) for u32
/// - Space: O(1)
#[inline]
pub fn popcount(n: u32) -> u32 {
    log_info!("   [popcount] Counting bits in {} (0x{:08x})", n, n);
    log_info!("   [popcount] Binary: {:032b}", n);

    // Manual bit counting (more instructive than n.count_ones())
    let mut val = n;
    let mut count = 0u32;
    while val > 0 {
        count += val & 1;
        val >>= 1;
    }

    log_info!("   [popcount] Result: {} has {} set bit(s)", n, count);
    count
}

// ============================================================================
// Cryptographic Operations
// ============================================================================

/// Compute SHA256 hash of n bytes of deterministic test data
///
/// # Input
/// - `n`: Number of bytes to hash (capped at 1024)
///
/// # Output
/// - First 4 bytes of SHA256 hash interpreted as big-endian u32
///
/// # Test Data Generation
/// Generates bytes: [0, 1, 2, ..., (n-1) mod 256]
/// This ensures deterministic, reproducible results.
///
/// # Examples
/// - n=100 → hash of [0,1,2,...,99]
/// - n=500 → hash of [0,1,...,255,0,1,...,243]
/// - n=2000 → hash of 1024 bytes (capped)
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn hash(n: u32) -> u32 {
    let len = n.min(1024);

    log_info!("   [hash] SHA256 hash of {} bytes", len);
    if n > 1024 {
        log_info!("   [hash] ⚠️  Input {} capped to 1024 bytes", n);
    }
    log_info!(
        "   [hash] Data pattern: [0, 1, 2, ..., {}] mod 256",
        len.saturating_sub(1)
    );

    let mut hasher = Sha256::new();

    // Generate deterministic test data
    for i in 0..len {
        hasher.update(&[(i % 256) as u8]);
    }

    let digest = hasher.finalize();
    let hash_value = u32::from_be_bytes([digest[0], digest[1], digest[2], digest[3]]);

    log_info!(
        "   [hash] Full digest: {:02x}{:02x}{:02x}{:02x}...",
        digest[0],
        digest[1],
        digest[2],
        digest[3]
    );
    log_info!(
        "   [hash] Result (first 4 bytes as u32): 0x{:08x} = {}",
        hash_value,
        hash_value
    );

    hash_value
}

// ============================================================================
// ECDSA Test Vectors (secp256k1)
// ============================================================================

/// Pre-computed ECDSA test vectors for deterministic benchmarking
///
/// Test vectors for ECDSA secp256k1 signing:
/// - Private key: 0x0123456789abcdef... (repeated, 32 bytes)
/// - Message: "zkVM ECDSA benchmark message"
mod test_vectors {
    /// Private key (32 bytes) for ECDSA signing
    /// Fixed key for reproducibility: 0x0123456789abcdef... (repeated)
    pub const PRIVATE_KEY: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xcd, 0xef,
    ];

    /// Test message to sign
    pub const MESSAGE: &[u8] = b"zkVM ECDSA benchmark message";
}

/// Perform real ECDSA secp256k1 signature generation
///
/// # Input
/// - `n`: Number of signing iterations (clamped to 1-100)
///
/// # Output
/// - Returns `1` if all signatures succeed
/// - Returns `0` if any signature fails
///
/// # Algorithm
/// Uses the k256 crate for real ECDSA secp256k1 signing:
/// 1. Parse the private key
/// 2. Sign the message using ECDSA
/// 3. Repeat n times to benchmark signing performance
///
/// # Test Vectors
/// Uses a fixed private key to ensure deterministic results.
///
/// # Complexity
/// - Time: O(n) * ECDSA signing cost
/// - Space: O(1)
pub fn signature(n: u32) -> u32 {
    let rounds = n.clamp(1, 100);

    log_info!("   [signature] ECDSA secp256k1 signing ({} rounds)", rounds);
    if n != rounds {
        log_info!(
            "   [signature] ⚠️  Input {} clamped to {} (valid: 1-100)",
            n,
            rounds
        );
    }

    // Parse private key
    let signing_key = match SigningKey::from_bytes((&test_vectors::PRIVATE_KEY).into()) {
        Ok(key) => {
            log_info!("   [signature] ✓ Private key parsed (32 bytes)");
            log_info!(
                "   [signature]   Key: {:02x}{:02x}{:02x}{:02x}...",
                test_vectors::PRIVATE_KEY[0],
                test_vectors::PRIVATE_KEY[1],
                test_vectors::PRIVATE_KEY[2],
                test_vectors::PRIVATE_KEY[3]
            );
            key
        }
        Err(_) => {
            log_info!("   [signature] ❌ Failed to parse private key");
            return 0;
        }
    };

    log_info!(
        "   [signature] Message: \"{}\"",
        core::str::from_utf8(test_vectors::MESSAGE).unwrap_or("...")
    );
    log_info!("   [signature] Running {} ECDSA signing(s)...", rounds);

    // Perform signing n times
    #[allow(unused_variables)]
    for round in 0..rounds {
        let sig: Signature = signing_key.sign(test_vectors::MESSAGE);
        // Use sig to prevent optimization
        if sig.to_bytes()[0] == 0xff && sig.to_bytes()[1] == 0xff {
            // Extremely unlikely, just to prevent dead code elimination
            log_info!("   [signature] Rare signature pattern detected");
        }
    }

    log_info!("   [signature] ✅ All {} signatures completed", rounds);
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
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
        assert_eq!(fibonacci(46), 1836311903); // Last non-overflowing
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(0), 0);
        assert_eq!(sum(1), 1);
        assert_eq!(sum(10), 55);
        assert_eq!(sum(100), 5050);
        assert_eq!(sum(1000), 500500);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(12), 479001600);
    }

    #[test]
    fn test_is_prime() {
        // Not prime
        assert_eq!(is_prime(0), 0);
        assert_eq!(is_prime(1), 0);
        assert_eq!(is_prime(4), 0);
        assert_eq!(is_prime(100), 0);
        assert_eq!(is_prime(1000), 0);

        // Prime
        assert_eq!(is_prime(2), 1);
        assert_eq!(is_prime(3), 1);
        assert_eq!(is_prime(17), 1);
        assert_eq!(is_prime(97), 1);
        assert_eq!(is_prime(7919), 1); // 1000th prime
    }

    #[test]
    fn test_popcount() {
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(1), 1);
        assert_eq!(popcount(7), 3);
        assert_eq!(popcount(15), 4);
        assert_eq!(popcount(255), 8);
        assert_eq!(popcount(0xFFFFFFFF), 32);
    }

    #[test]
    fn test_hash_deterministic() {
        // Same input should give same output
        let h1 = hash(100);
        let h2 = hash(100);
        assert_eq!(h1, h2, "Hash should be deterministic");

        // Different inputs should give different outputs
        let h3 = hash(200);
        assert_ne!(h1, h3, "Different inputs should give different hashes");
    }

    #[test]
    fn test_hash_capping() {
        // Values above 1024 should be capped
        let h1 = hash(1024);
        let h2 = hash(2000);
        assert_eq!(h1, h2, "Hash should cap at 1024 bytes");
    }

    #[test]
    fn test_signature_valid() {
        // Test that ECDSA signing works correctly
        assert_eq!(signature(1), 1, "Single signing should succeed");
        assert_eq!(signature(10), 1, "Multiple signings should succeed");
    }

    #[test]
    fn test_signature_clamping() {
        // Values outside 1-100 should be clamped
        assert_eq!(signature(0), 1); // Clamped to 1
                                     // Note: 200 would be clamped to 100, still succeeds
    }

    #[test]
    fn test_execute() {
        assert_eq!(execute(Program::Fibonacci, 10), 55);
        assert_eq!(execute(Program::Sum, 10), 55);
        assert_eq!(execute(Program::Factorial, 5), 120);
        assert_eq!(execute(Program::IsPrime, 17), 1);
        assert_eq!(execute(Program::PopCount, 15), 4);
        assert_eq!(execute(Program::Hash, 100), hash(100));
        assert_eq!(execute(Program::Signature, 1), 1);
    }

    #[test]
    fn test_execute_by_id() {
        assert_eq!(execute_by_id(0, 10), 55); // Fibonacci
        assert_eq!(execute_by_id(1, 10), 55); // Sum
        assert_eq!(execute_by_id(5, 100), hash(100)); // Hash
        assert_eq!(execute_by_id(6, 1), 1); // Signature
        assert_eq!(execute_by_id(99, 10), 0); // Invalid ID
    }
}

// ============================================================================
// Test Vector Generation (for development only)
// ============================================================================

/// Generate test vectors for ECDSA signature verification
///
/// This function is used during development to create valid test vectors.
/// Run with: cargo test --features std gen_test_vectors -- --nocapture
#[cfg(all(test, feature = "std"))]
#[test]
#[ignore] // Only run manually when regenerating test vectors
fn gen_test_vectors() {
    use k256::ecdsa::{
        signature::{Signer, Verifier},
        SigningKey,
    };

    // Fixed private key for reproducibility
    let private_key_bytes: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd,
        0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
        0xcd, 0xef,
    ];

    let signing_key = SigningKey::from_bytes(&private_key_bytes.into()).unwrap();
    let verifying_key = signing_key.verifying_key();

    // Get compressed public key
    let pubkey_compressed = verifying_key.to_sec1_bytes();
    println!("pub const PUBKEY_COMPRESSED: [u8; 33] = [");
    print!("    0x{:02x}, // Compressed prefix", pubkey_compressed[0]);
    for (i, byte) in pubkey_compressed[1..].iter().enumerate() {
        if i % 8 == 0 {
            println!();
            print!("    ");
        }
        print!("0x{:02x}, ", byte);
    }
    println!("\n];");

    // Sign the message
    let message = b"zkVM ECDSA benchmark message";
    let sig: Signature = Signer::sign(&signing_key, message);
    let sig_bytes = sig.to_bytes();

    println!(
        "\npub const MESSAGE: &[u8] = b\"{}\";",
        core::str::from_utf8(message).unwrap()
    );

    println!("\npub const SIGNATURE_BYTES: [u8; 64] = [");
    println!("    // r (32 bytes)");
    for i in 0..32 {
        if i % 8 == 0 && i > 0 {
            println!();
        }
        if i % 8 == 0 {
            print!("    ");
        }
        print!("0x{:02x}, ", sig_bytes[i]);
    }
    println!("\n    // s (32 bytes) - low-s");
    for i in 32..64 {
        if (i - 32) % 8 == 0 && i > 32 {
            println!();
        }
        if (i - 32) % 8 == 0 {
            print!("    ");
        }
        print!("0x{:02x}, ", sig_bytes[i]);
    }
    println!("\n];");

    // Verify it works
    println!("\nVerification test:");
    match verifying_key.verify(message, &sig) {
        Ok(()) => println!("✅ Signature verified successfully!"),
        Err(e) => println!("❌ Verification failed: {:?}", e),
    }
}
