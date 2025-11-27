//! ZisK zkVM Guest Program - Multi-Program Support
//! 
//! This program runs inside the ZisK zero-knowledge virtual machine.

// Disable the standard Rust entry point when targeting zkVM
#![cfg_attr(all(target_os = "zkvm", target_vendor = "zisk"), no_main)]

use ziskos::*;
use programs::execute_program;

// Entry point for the ZisK guest program (when targeting zkVM)
#[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
ziskos::entrypoint!(main);

// Regular main function (when building natively for testing)
#[cfg(not(all(target_os = "zkvm", target_vendor = "zisk")))]
fn main() {
    // Mock input for native testing
    let program_id = 0;
    let n = 10;
    let result = execute_program(program_id, n);
    println!("Result: {}", result);
}

#[cfg(all(target_os = "zkvm", target_vendor = "zisk"))]
pub fn main() {
    // Read input from the host (input.bin file)
    let input = read_input();
    
    if input.len() < 8 {
        // Panic or handle error
        return;
    }
    
    // Parse the input as two u32s (little-endian)
    let program_id = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);
    let n = u32::from_le_bytes([input[4], input[5], input[6], input[7]]);
    
    // Dispatch execution
    let result = execute_program(program_id, n);
    
    // Set the result as public output
    set_output(0, result);
}
