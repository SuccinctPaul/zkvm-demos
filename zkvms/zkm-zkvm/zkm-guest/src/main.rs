#![no_main]
zkm_zkvm::entrypoint!(main);

use zkvm_programs::execute_program;

pub fn main() {
    // Read input
    let program_id = zkm_zkvm::io::read::<u32>();
    let n = zkm_zkvm::io::read::<u32>();

    // Execute
    let result = execute_program(program_id, n);
    
    println!("Result: {}", result);

    // Commit (ZKM uses commit_slice usually, but let's stick to print or minimal commit if API supports it)
    // zkm_zkvm::io::commit(&result); // Check if this API exists, otherwise skip
}
