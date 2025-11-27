//! Program definitions and metadata
//!
//! # Input Parameter `n` Semantics
//!
//! Each program interprets the input parameter `n` differently:
//!
//! | Program     | Input `n` Meaning                    | Example           |
//! |-------------|--------------------------------------|-------------------|
//! | Fibonacci   | Index of Fibonacci number to compute | n=20 → fib(20)=6765 |
//! | Sum         | Upper bound of sum (1+2+...+n)       | n=100 → 5050      |
//! | Factorial   | Number to compute factorial of       | n=10 → 3628800    |
//! | IsPrime     | Number to check for primality        | n=17 → 1 (prime)  |
//! | PopCount    | Number to count set bits in          | n=255 → 8 bits    |
//! | Hash        | Bytes of data to hash (max 1024)     | n=500 → hash 500B |
//! | Signature   | Iterations of verification (1-100)   | n=50 → 50 iters   |

use core::fmt;

/// Available programs for zkVM benchmarking
///
/// Each variant represents a different benchmark program with specific
/// input semantics. See module documentation for input `n` meanings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Program {
    /// Compute the nth Fibonacci number
    ///
    /// **Input `n`**: Index in Fibonacci sequence (0-indexed)
    /// - n=0 → 0
    /// - n=1 → 1  
    /// - n=10 → 55
    /// - n=20 → 6765
    ///
    /// **Complexity**: O(n) time, O(1) space
    Fibonacci = 0,

    /// Sum integers from 1 to n
    ///
    /// **Input `n`**: Upper bound of summation
    /// - n=10 → 1+2+...+10 = 55
    /// - n=100 → 5050
    ///
    /// **Complexity**: O(n) time, O(1) space
    Sum = 1,

    /// Compute n factorial (n!)
    ///
    /// **Input `n`**: Number to compute factorial of
    /// - n=5 → 120
    /// - n=10 → 3628800
    /// - n=12 → 479001600 (max without overflow)
    ///
    /// **Complexity**: O(n) time, O(1) space
    Factorial = 2,

    /// Check if n is a prime number
    ///
    /// **Input `n`**: Number to check for primality
    /// - Returns 1 if prime, 0 if not prime
    /// - n=17 → 1 (prime)
    /// - n=100 → 0 (not prime)
    ///
    /// **Complexity**: O(√n) time, O(1) space
    IsPrime = 3,

    /// Count set bits in n (population count)
    ///
    /// **Input `n`**: Number to count bits in
    /// - n=7 (binary: 111) → 3
    /// - n=255 (binary: 11111111) → 8
    ///
    /// **Complexity**: O(log n) time, O(1) space
    PopCount = 4,

    /// Compute SHA256 hash of n bytes of test data
    ///
    /// **Input `n`**: Number of bytes to hash (capped at 1024)
    /// - Generates deterministic test data: bytes 0,1,2,...,(n-1)%256
    /// - Returns first 4 bytes of hash as u32
    /// - n=100 → hash of 100 bytes
    /// - n=500 → hash of 500 bytes
    /// - n=2000 → hash of 1024 bytes (capped)
    ///
    /// **Complexity**: O(n) time, O(1) space
    Hash = 5,

    /// Simulate ECDSA signature verification
    ///
    /// **Input `n`**: Number of verification iterations (clamped to 1-100)
    /// - Each iteration performs SHA256 hash operations
    /// - Simulates signature verification workload
    /// - n=1 → 1 iteration
    /// - n=50 → 50 iterations
    /// - n=200 → 100 iterations (clamped)
    ///
    /// **Returns**: 1 if verification succeeds, 0 if fails
    ///
    /// **Complexity**: O(n) time, O(1) space
    Signature = 6,
}

impl Program {
    /// All available programs
    pub const ALL: &'static [Program] = &[
        Program::Fibonacci,
        Program::Sum,
        Program::Factorial,
        Program::IsPrime,
        Program::PopCount,
        Program::Hash,
        Program::Signature,
    ];

    /// Get the program's numeric ID
    #[inline]
    pub const fn id(self) -> u32 {
        self as u32
    }

    /// Create program from numeric ID
    #[inline]
    pub const fn from_id(id: u32) -> Option<Self> {
        match id {
            0 => Some(Program::Fibonacci),
            1 => Some(Program::Sum),
            2 => Some(Program::Factorial),
            3 => Some(Program::IsPrime),
            4 => Some(Program::PopCount),
            5 => Some(Program::Hash),
            6 => Some(Program::Signature),
            _ => None,
        }
    }

    /// Get program name
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Program::Fibonacci => "fibonacci",
            Program::Sum => "sum",
            Program::Factorial => "factorial",
            Program::IsPrime => "isprime",
            Program::PopCount => "popcount",
            Program::Hash => "hash",
            Program::Signature => "signature",
        }
    }

    /// Get program description
    #[inline]
    pub const fn description(self) -> &'static str {
        match self {
            Program::Fibonacci => "Compute nth Fibonacci number",
            Program::Sum => "Sum integers 1..=n",
            Program::Factorial => "Compute n!",
            Program::IsPrime => "Check if n is prime (returns 0 or 1)",
            Program::PopCount => "Count set bits in n",
            Program::Hash => "SHA256 hash of n bytes",
            Program::Signature => "ECDSA verification (n iterations)",
        }
    }

    /// Get description of what input `n` means for this program
    #[inline]
    pub const fn input_description(self) -> &'static str {
        match self {
            Program::Fibonacci => "n = index in Fibonacci sequence (0-indexed)",
            Program::Sum => "n = upper bound of sum (1+2+...+n)",
            Program::Factorial => "n = number to compute factorial of",
            Program::IsPrime => "n = number to check for primality",
            Program::PopCount => "n = number to count set bits in",
            Program::Hash => "n = bytes of data to hash (max 1024)",
            Program::Signature => "n = verification iterations (1-100)",
        }
    }

    /// Parse program from string (std only)
    #[cfg(feature = "std")]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fibonacci" | "fib" | "0" => Some(Program::Fibonacci),
            "sum" | "1" => Some(Program::Sum),
            "factorial" | "fact" | "2" => Some(Program::Factorial),
            "isprime" | "prime" | "3" => Some(Program::IsPrime),
            "popcount" | "bitcount" | "4" => Some(Program::PopCount),
            "hash" | "sha256" | "5" => Some(Program::Hash),
            "signature" | "sig" | "ecdsa" | "6" => Some(Program::Signature),
            _ => None,
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Program execution input
///
/// Combines a program with its input parameter `n`.
/// The meaning of `n` depends on the program - see [`Program`] documentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramInput {
    /// The program to execute
    pub program: Program,
    /// Input parameter (meaning depends on program)
    pub n: u32,
}

impl ProgramInput {
    /// Create a new program input
    #[inline]
    pub const fn new(program: Program, n: u32) -> Self {
        Self { program, n }
    }

    /// Get description of what `n` means for this program
    #[inline]
    pub const fn input_description(&self) -> &'static str {
        self.program.input_description()
    }
}

impl fmt::Display for ProgramInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.program.name(), self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_id_roundtrip() {
        for &program in Program::ALL {
            assert_eq!(Program::from_id(program.id()), Some(program));
        }
        assert_eq!(Program::from_id(99), None);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_program_parse() {
        assert_eq!(Program::parse("fibonacci"), Some(Program::Fibonacci));
        assert_eq!(Program::parse("fib"), Some(Program::Fibonacci));
        assert_eq!(Program::parse("0"), Some(Program::Fibonacci));
        assert_eq!(Program::parse("hash"), Some(Program::Hash));
        assert_eq!(Program::parse("invalid"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(Program::Fibonacci.to_string(), "fibonacci");
        assert_eq!(Program::Hash.to_string(), "hash");
    }

    #[test]
    fn test_program_input_display() {
        let input = ProgramInput::new(Program::Fibonacci, 20);
        assert_eq!(input.to_string(), "fibonacci(20)");
    }
}
