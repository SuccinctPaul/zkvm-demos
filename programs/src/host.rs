//! Host-side utilities (std only)
//!
//! Functions for loading program configuration from environment variables.
//!
//! # Environment Variables
//!
//! | Variable     | Description                         | Example           |
//! |--------------|-------------------------------------|-------------------|
//! | PROGRAM      | Program name or ID                  | fibonacci, hash, 5 |
//! | INPUT_N      | Input parameter (primary)           | 20, 500           |
//! | PROGRAM_N    | Input parameter (fallback)          | 20, 500           |
//! | FIBONACCI_N  | Input parameter (legacy fallback)   | 20                |

use crate::{Program, ProgramInput, DEFAULT_N};
use dotenv::dotenv;
use std::env;

/// Load program from PROGRAM environment variable
///
/// Supports both program names and IDs:
/// - Names: fibonacci, fib, sum, factorial, hash, signature, sig, etc.
/// - IDs: 0, 1, 2, 3, 4, 5, 6
///
/// Defaults to Fibonacci if not set or invalid.
pub fn load_program() -> Program {
    dotenv().ok();

    let program = env::var("PROGRAM")
        .ok()
        .and_then(|s| {
            eprintln!("[zkvm_programs] 📖 PROGRAM env var: '{}'", s);
            Program::parse(&s)
        })
        .unwrap_or_else(|| {
            eprintln!("[zkvm_programs] 📖 PROGRAM env var not set, using default: fibonacci");
            Program::Fibonacci
        });

    eprintln!(
        "[zkvm_programs] ✅ Selected program: {} (id={})",
        program.name(),
        program.id()
    );
    eprintln!("[zkvm_programs]    Description: {}", program.description());
    eprintln!(
        "[zkvm_programs]    Input meaning: {}",
        program.input_description()
    );
    program
}

/// Load input N from environment variables
///
/// Checks in order: INPUT_N, PROGRAM_N, FIBONACCI_N
/// Returns DEFAULT_N (10) if none set.
///
/// The meaning of N depends on the program:
/// - Fibonacci: index in sequence
/// - Sum: upper bound
/// - Hash: bytes to hash
/// - etc.
pub fn load_n() -> u32 {
    dotenv().ok();

    let env_vars = ["INPUT_N", "PROGRAM_N", "FIBONACCI_N"];

    for key in &env_vars {
        if let Ok(val) = env::var(key) {
            if let Ok(n) = val.parse::<u32>() {
                eprintln!("[zkvm_programs] 📖 {} = {}", key, n);
                return n;
            } else {
                eprintln!(
                    "[zkvm_programs] ⚠️  {} = '{}' (invalid number, skipping)",
                    key, val
                );
            }
        }
    }

    eprintln!("[zkvm_programs] 📖 No input env var set (checked: INPUT_N, PROGRAM_N, FIBONACCI_N)");
    eprintln!("[zkvm_programs]    Using default: {}", DEFAULT_N);
    DEFAULT_N
}

/// Load complete program input from environment
///
/// Combines load_program() and load_n() to create a ProgramInput.
/// Logs the full configuration including what the input means.
pub fn load_input() -> ProgramInput {
    eprintln!("[zkvm_programs] ╔═══════════════════════════════════════════════════════════╗");
    eprintln!("[zkvm_programs] ║           Loading Program Configuration                   ║");
    eprintln!("[zkvm_programs] ╚═══════════════════════════════════════════════════════════╝");

    let program = load_program();
    let n = load_n();
    let input = ProgramInput::new(program, n);

    eprintln!("[zkvm_programs] ───────────────────────────────────────────────────────────");
    eprintln!("[zkvm_programs] 📦 ProgramInput Summary:");
    eprintln!(
        "[zkvm_programs]    Program: {} (id={})",
        input.program,
        input.program.id()
    );
    eprintln!("[zkvm_programs]    Input N: {}", input.n);
    eprintln!("[zkvm_programs]    Meaning: {}", input.input_description());
    eprintln!(
        "[zkvm_programs]    Will execute: {}({})",
        input.program, input.n
    );
    eprintln!("[zkvm_programs] ───────────────────────────────────────────────────────────\n");

    input
}

/// Backward compatibility alias for load_input
#[inline]
pub fn load_program_input() -> ProgramInput {
    load_input()
}

/// Backward compatibility alias for load_n
#[inline]
pub fn load_input_n() -> u32 {
    load_n()
}

/// Backward compatibility alias for load_n (fibonacci specific)
#[inline]
pub fn load_fib_n() -> u32 {
    load_n()
}

/// Print available programs to stdout
///
/// Shows all programs with their IDs, names, descriptions, and input semantics.
pub fn print_programs() {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    zkvm_programs - Available Programs                     ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════╣");
    println!("║ ID │ Name        │ Description                  │ Input N Meaning        ║");
    println!("╠════╪═════════════╪══════════════════════════════╪════════════════════════╣");
    for program in Program::ALL {
        println!(
            "║ {:>2} │ {:<11} │ {:<28} │ {:<22} ║",
            program.id(),
            program.name(),
            program.description(),
            &program.input_description()[4..]
                .chars()
                .take(22)
                .collect::<String>()
        );
    }
    println!("╚═══════════════════════════════════════════════════════════════════════════╝");
    println!();
    println!("💡 Environment Variables:");
    println!("   PROGRAM    Program name or ID (fibonacci, hash, 5, etc.)");
    println!("   INPUT_N    Input parameter value");
    println!();
    println!("📝 Examples:");
    println!("   PROGRAM=fibonacci INPUT_N=20 cargo run    # Compute F(20) = 6765");
    println!("   PROGRAM=hash INPUT_N=500 cargo run        # Hash 500 bytes");
    println!("   PROGRAM=signature INPUT_N=50 cargo run    # 50 verification iterations");
    println!("   PROGRAM=5 INPUT_N=1000 cargo run          # Hash by ID (capped to 1024)");
    println!();
}
