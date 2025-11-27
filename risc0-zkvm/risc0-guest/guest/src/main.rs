#![no_main]
use risc0_zkvm::guest::env;
use programs::execute_program;

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read program ID and input N
    let program_id: u32 = env::read();
    let n: u32 = env::read();

    // Execute common logic
    let result = execute_program(program_id, n);

    // Commit result
    env::commit(&result);
}
