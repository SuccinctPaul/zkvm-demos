#![cfg_attr(feature = "guest", no_std)]
#![cfg_attr(feature = "guest", no_main)]

use programs::execute_program as common_execute;

/// Execute a selected program by ID
/// This function will be proven by Jolt zkVM
#[jolt::provable]
fn execute_program(id: u32, n: u32) -> u32 {
    common_execute(id, n)
}
