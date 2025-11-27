//! Host-side utilities (std only)
//!
//! Functions for loading program configuration from environment variables.

use crate::{Program, ProgramInput, DEFAULT_N};
use dotenv::dotenv;
use std::env;

/// Load program from PROGRAM environment variable
pub fn load_program() -> Program {
    dotenv().ok();

    env::var("PROGRAM")
        .ok()
        .and_then(|s| Program::parse(&s))
        .unwrap_or(Program::Fibonacci)
}

/// Load input N from environment variables
///
/// Checks in order: INPUT_N, PROGRAM_N, FIBONACCI_N
pub fn load_n() -> u32 {
    dotenv().ok();

    ["INPUT_N", "PROGRAM_N", "FIBONACCI_N"]
        .iter()
        .find_map(|key| env::var(key).ok()?.parse().ok())
        .unwrap_or(DEFAULT_N)
}

/// Load complete program input from environment
pub fn load_input() -> ProgramInput {
    ProgramInput::new(load_program(), load_n())
}

/// Print available programs to stdout
pub fn print_programs() {
    println!("\n📋 Available Programs:");
    println!("─────────────────────────────────────────");
    for program in Program::ALL {
        println!("  {:>2}  {:<12} {}", program.id(), program.name(), program.description());
    }
    println!("─────────────────────────────────────────");
    println!("\n💡 Usage: PROGRAM=<name> INPUT_N=<value> cargo run");
    println!("   Example: PROGRAM=fibonacci INPUT_N=30 cargo run\n");
}

