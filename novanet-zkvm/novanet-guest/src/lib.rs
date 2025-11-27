//! Novanet zkVM Guest Program - Multi-Program Support
//! 
//! This guest program executes dispatched algorithms
//! in a zero-knowledge proof environment using Novanet zkVM.

use serde::{Deserialize, Serialize};
use programs::execute_program;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInput {
    pub program_id: u32,
    pub n: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramOutput {
    pub result: u32,
}

/// Execute the selected program
/// This function will be proven by Novanet zkVM
pub fn compute_program(input: ProgramInput) -> ProgramOutput {
    let result = execute_program(input.program_id, input.n);
    ProgramOutput { result }
}

/// Guest program entry point for Novanet zkVM
pub fn guest_main(program_id: u32, n: u32) -> u32 {
    let input = ProgramInput { program_id, n };
    let output = compute_program(input);
    output.result
}
