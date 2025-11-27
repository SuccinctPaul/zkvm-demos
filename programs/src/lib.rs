//! # Programs
//!
//! Benchmark programs for zkVM environments.
//!
//! This crate provides a collection of computation programs designed to be
//! executed in zero-knowledge virtual machines for benchmarking purposes.
//!
//! ## Features
//!
//! - `std` - Enable standard library features (env loading, debug output)
//! - `crypto` - Enable cryptographic programs (SHA256 hash, ECDSA signature)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use programs::{Program, execute};
//!
//! // Execute fibonacci(20)
//! let result = execute(Program::Fibonacci, 20);
//! assert_eq!(result, 6765);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod program;
mod compute;

#[cfg(feature = "std")]
mod host;

// Public exports
pub use program::{Program, ProgramInput};
pub use compute::*;

#[cfg(feature = "std")]
pub use host::*;

/// Default input value for programs
pub const DEFAULT_N: u32 = 10;
