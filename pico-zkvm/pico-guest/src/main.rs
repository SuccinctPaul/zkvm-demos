#![no_main]

use zkvm_programs::execute_program;
pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};

pub fn main() {
    // Read input from the host
    let program_id: u32 = read_as();
    let n: u32 = read_as();

    // Compute result using common dispatcher
    let result = execute_program(program_id, n);

    // Commit the result as public output
    commit(&result);
}
