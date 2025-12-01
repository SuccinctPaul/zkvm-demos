#![no_main]
#![no_std]

use zkvm_programs::execute_program;
openvm::entry!(main);

pub fn main() {
    // Read program ID and input N from the host
    let program_id: u32 = openvm::io::read();
    let n: u32 = openvm::io::read();

    // Execute common logic
    let result = execute_program(program_id, n);

    // Commit the result as public output
    openvm::io::commit(&result);
}
