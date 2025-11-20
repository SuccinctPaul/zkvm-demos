#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::println;
use common::execute_program;

#[nexus_rt::main]
#[nexus_rt::public_input(input_packed)]
fn main(input_packed: u64) {
    let program_id = (input_packed >> 32) as u32;
    let n = (input_packed & 0xFFFFFFFF) as u32;
    
    // Dispatch execution
    let result = execute_program(program_id, n);
    
    // Print result (Nexus doesn't have explicit commit API yet in this version)
    println!("Result: {}", result);
}
