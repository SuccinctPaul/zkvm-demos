//! Host-side utilities (std only)
//!
//! Functions for loading program configuration from environment variables.

use crate::{Program, ProgramInput, DEFAULT_N};
use dotenv::dotenv;
use std::env;

/// Load program from PROGRAM environment variable
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

    eprintln!("[zkvm_programs] ✅ Loaded program: {} (id={})", program.name(), program.id());
    program
}

/// Load input N from environment variables
///
/// Checks in order: INPUT_N, PROGRAM_N, FIBONACCI_N
pub fn load_n() -> u32 {
    dotenv().ok();

    let env_vars = ["INPUT_N", "PROGRAM_N", "FIBONACCI_N"];
    
    for key in &env_vars {
        if let Ok(val) = env::var(key) {
            if let Ok(n) = val.parse::<u32>() {
                eprintln!("[zkvm_programs] 📖 {} = {} (parsed)", key, n);
                return n;
            } else {
                eprintln!("[zkvm_programs] ⚠️  {} = '{}' (invalid, skipping)", key, val);
            }
        }
    }

    eprintln!("[zkvm_programs] 📖 No input env var set, using default: {}", DEFAULT_N);
    DEFAULT_N
}

/// Load complete program input from environment
pub fn load_input() -> ProgramInput {
    eprintln!("[zkvm_programs] ───────────────────────────────────────────────────────────");
    eprintln!("[zkvm_programs] 📥 Loading program input from environment...");
    
    let program = load_program();
    let n = load_n();
    let input = ProgramInput::new(program, n);
    
    eprintln!("[zkvm_programs] ───────────────────────────────────────────────────────────");
    eprintln!("[zkvm_programs] ✅ ProgramInput {{ program: {}, n: {} }}", input.program, input.n);
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
pub fn print_programs() {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║               zkvm_programs - Available Programs              ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║ ID │ Name        │ Description                               ║");
    println!("╠════╪═════════════╪═══════════════════════════════════════════╣");
    for program in Program::ALL {
        println!("║ {:>2} │ {:<11} │ {:<41} ║", 
            program.id(), 
            program.name(), 
            program.description()
        );
    }
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("💡 Usage:");
    println!("   PROGRAM=<name|id> INPUT_N=<value> cargo run");
    println!();
    println!("📝 Examples:");
    println!("   PROGRAM=fibonacci INPUT_N=30 cargo run");
    println!("   PROGRAM=hash INPUT_N=500 cargo run");
    println!("   PROGRAM=5 INPUT_N=1000 cargo run  # hash by ID");
    println!();
}
