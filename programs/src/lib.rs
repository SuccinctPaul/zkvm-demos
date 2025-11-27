//! # zkvm_programs
//!
//! Benchmark programs for zkVM environments.
//!
//! This crate provides computation programs designed to run in
//! zero-knowledge virtual machines for benchmarking purposes.
//!
//! ## Features
//!
//! - `std` - Enable standard library features (env loading, debug output)
//!
//! ## Programs
//!
//! | Program | Description | Input N |
//! |---------|-------------|---------|
//! | `Fibonacci` | Compute nth Fibonacci number | n = index |
//! | `Sum` | Sum integers 1..=n | n = upper bound |
//! | `Factorial` | Compute n! | n = number |
//! | `IsPrime` | Check if n is prime | n = number to check |
//! | `PopCount` | Count set bits in n | n = number |
//! | `Hash` | SHA256 hash of n bytes | n = data length (max 1024) |
//! | `Signature` | ECDSA verification simulation | n = iterations (1-100) |
//!
//! ## Usage
//!
//! ```rust,ignore
//! use zkvm_programs::{Program, execute, fibonacci};
//!
//! // Execute via Program enum
//! let result = execute(Program::Fibonacci, 20);
//! assert_eq!(result, 6765);
//!
//! // Or call directly
//! let fib = fibonacci(20);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod compute;
mod program;

#[cfg(feature = "std")]
mod host;

/// Backward compatibility: re-export compute as benchmarks
pub mod benchmarks {
    pub use crate::compute::*;
}

// Public exports
pub use compute::*;
pub use program::{Program, ProgramInput};

#[cfg(feature = "std")]
pub use host::*;

/// Default input value for programs
pub const DEFAULT_N: u32 = 10;
