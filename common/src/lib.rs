#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use dotenv::dotenv;
#[cfg(feature = "std")]
use std::env;

mod hash;
pub mod programs;
pub mod benchmarks;

pub use programs::{Program, ProgramInput};
pub use benchmarks::*;

/// Default input value for programs
pub const DEFAULT_INPUT: u32 = 10;

// Host-only functions (require std)
#[cfg(feature = "std")]
pub mod host {
    use super::*;
    
    /// Load program selection from environment variable
    pub fn load_program() -> Program {
        dotenv().ok();
        
        let program_name = env::var("PROGRAM")
            .unwrap_or_else(|_| "fibonacci".to_string());
        
        Program::from_str(&program_name)
            .unwrap_or_else(|| {
                eprintln!("⚠️  Unknown program '{}', using fibonacci", program_name);
                Program::Fibonacci
            })
    }

    /// Load input value from environment variable
    pub fn load_input_n() -> u32 {
        dotenv().ok();
        
        env::var("INPUT_N")
            .or_else(|_| env::var("FIBONACCI_N")) // Backward compatibility
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_else(|| {
                eprintln!("ℹ️  INPUT_N not set, using default value: {}", DEFAULT_INPUT);
                DEFAULT_INPUT
            })
    }

    /// Load program input (program + n value)
    pub fn load_program_input() -> ProgramInput {
        let program = load_program();
        let n = load_input_n();
        ProgramInput::new(program, n)
    }

    /// Load fib param from environment variable (backward compatibility)
    pub fn load_fib_n() -> u32 {
        load_input_n()
    }

    /// Print available programs
    pub fn print_available_programs() {
        println!("\n📋 Available Programs:");
        println!("────────────────────────────────────────");
        for program in Program::all() {
            println!("  • {:<12} - {}", program.as_str(), program.description());
        }
        println!("────────────────────────────────────────");
        println!("\n💡 Usage:");
        println!("  PROGRAM=<name> INPUT_N=<value> cargo run");
        println!("\n📝 Examples:");
        println!("  PROGRAM=fibonacci INPUT_N=30 cargo run");
        println!("  PROGRAM=sum INPUT_N=1000 cargo run");
        println!("  PROGRAM=prime INPUT_N=97 cargo run");
        println!();
    }
}

// Re-export host functions at top level for backward compatibility
#[cfg(feature = "std")]
pub use host::*;
