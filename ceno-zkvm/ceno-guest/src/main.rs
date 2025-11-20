// CENO zkVM Guest Program - Multi-Program Support

#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

#[cfg(target_arch = "riscv32")]
use nexus_rt::println;

use common::execute_program;

#[nexus_rt::main]
#[nexus_rt::public_input(input_packed)]
fn main(input_packed: u64) {
    #[cfg(target_arch = "riscv32")]
    {
        // Unpack inputs
        let program_id = (input_packed >> 32) as u32;
        let n = (input_packed & 0xFFFFFFFF) as u32;
        
        // Execute selected program
        let result = execute_program(program_id, n);
        
        println!("Result: {}", result);
    }
    
    #[cfg(not(target_arch = "riscv32"))]
    {
        // For native builds (testing)
        println!("=== CENO zkVM Guest Program (Native Test) ===");
        // Mock execution
        execute_program(0, 10);
    }
}
