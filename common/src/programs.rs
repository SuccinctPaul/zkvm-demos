/// Program selector for zkVM guest programs
/// 
/// This module provides a unified interface for selecting and running
/// different programs within the same guest binary.

use core::fmt;

/// Available programs that can be executed in the zkVM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Program {
    /// Fibonacci sequence computation
    Fibonacci,
    /// Sum of integers from 1 to n
    Sum,
    /// Factorial computation
    Factorial,
    /// Check if a number is prime
    IsPrime,
    /// Count set bits in a number
    PopCount,
    /// SHA256 hash computation
    Hash,
    /// ECDSA signature verification
    Signature,
}

impl Program {
    /// Get the unique numerical ID for the program
    /// 
    /// This ID is used to communicate between Host and Guest.
    pub fn id(&self) -> u32 {
        match self {
            Program::Fibonacci => 0,
            Program::Sum => 1,
            Program::Factorial => 2,
            Program::IsPrime => 3,
            Program::PopCount => 4,
            Program::Hash => 5,
            Program::Signature => 6,
        }
    }

    /// Get program from numerical ID
    pub fn from_id(id: u32) -> Option<Self> {
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

    /// Parse program name from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "fibonacci" | "fib" => Some(Program::Fibonacci),
            "sum" => Some(Program::Sum),
            "factorial" | "fact" => Some(Program::Factorial),
            "isprime" | "prime" => Some(Program::IsPrime),
            "popcount" | "bitcount" => Some(Program::PopCount),
            "hash" | "sha256" => Some(Program::Hash),
            "signature" | "sig" | "ecdsa" => Some(Program::Signature),
            _ => None,
        }
    }

    /// Get program name as string
    pub fn as_str(&self) -> &'static str {
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
    pub fn description(&self) -> &'static str {
        match self {
            Program::Fibonacci => "Compute the nth Fibonacci number",
            Program::Sum => "Sum integers from 1 to n",
            Program::Factorial => "Compute n factorial",
            Program::IsPrime => "Check if n is a prime number",
            Program::PopCount => "Count the number of set bits in n",
            Program::Hash => "Compute SHA256 hash of test data",
            Program::Signature => "Verify ECDSA signature",
        }
    }

    /// List all available programs
    pub fn all() -> &'static [Program] {
        &[
            Program::Fibonacci,
            Program::Sum,
            Program::Factorial,
            Program::IsPrime,
            Program::PopCount,
            Program::Hash,
            Program::Signature,
        ]
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Program execution input
#[derive(Debug, Clone, Copy)]
pub struct ProgramInput {
    pub program: Program,
    pub n: u32,
}

impl ProgramInput {
    pub fn new(program: Program, n: u32) -> Self {
        Self { program, n }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_id() {
        assert_eq!(Program::Fibonacci.id(), 0);
        assert_eq!(Program::Sum.id(), 1);
        assert_eq!(Program::Hash.id(), 5);
        
        assert_eq!(Program::from_id(0), Some(Program::Fibonacci));
        assert_eq!(Program::from_id(1), Some(Program::Sum));
        assert_eq!(Program::from_id(5), Some(Program::Hash));
        assert_eq!(Program::from_id(99), None);
    }

    #[test]
    fn test_program_from_str() {
        assert_eq!(Program::from_str("fibonacci"), Some(Program::Fibonacci));
        assert_eq!(Program::from_str("fib"), Some(Program::Fibonacci));
        assert_eq!(Program::from_str("sum"), Some(Program::Sum));
        assert_eq!(Program::from_str("hash"), Some(Program::Hash));
        assert_eq!(Program::from_str("sha256"), Some(Program::Hash));
        assert_eq!(Program::from_str("signature"), Some(Program::Signature));
        assert_eq!(Program::from_str("ecdsa"), Some(Program::Signature));
        assert_eq!(Program::from_str("invalid"), None);
    }

    #[test]
    fn test_program_display() {
        assert_eq!(Program::Fibonacci.to_string(), "fibonacci");
        assert_eq!(Program::Sum.to_string(), "sum");
        assert_eq!(Program::Hash.to_string(), "hash");
        assert_eq!(Program::Signature.to_string(), "signature");
    }
}
