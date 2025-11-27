//! Program definitions and metadata

use core::fmt;

/// Available programs for zkVM benchmarking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Program {
    Fibonacci = 0,
    Sum = 1,
    Factorial = 2,
    IsPrime = 3,
    PopCount = 4,
    Hash = 5,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramInput {
    pub program: Program,
    pub n: u32,
}

impl ProgramInput {
    #[inline]
    pub const fn new(program: Program, n: u32) -> Self {
        Self { program, n }
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
}

