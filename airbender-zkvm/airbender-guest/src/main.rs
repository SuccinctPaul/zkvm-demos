// Airbender zkVM Guest Program - Multi-Program Support
//
// Note: This is a reference implementation based on the RISC-V zkVM architecture.

#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use programs::execute_program;

// RISC-V zkVM entry point
#[cfg(target_arch = "riscv32")]
#[no_mangle]
pub extern "C" fn main() {
    // In a real Airbender implementation, public inputs would be read
    // through the Airbender runtime API.
    
    // Placeholder: Read input (simulated)
    // In real implementation: let program_id: u32 = airbender::read();
    let program_id: u32 = 0; // Default to Fibonacci for compilation check
    
    // In real implementation: let n: u32 = airbender::read();
    let n: u32 = 10; 
    
    // Execute selected program
    let result = execute_program(program_id, n);
    
    // In a real implementation, results would be committed
    // core::hint::black_box(result);
}

#[cfg(target_arch = "riscv32")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Native build for testing
#[cfg(not(target_arch = "riscv32"))]
#[allow(dead_code)]
fn main() {
    println!("=== Airbender zkVM Guest Program (Native Test) ===");
    println!("This is a test build. Run via the host program for actual proof generation.");
    
    // Simulate inputs
    let program_id = 0;
    let n = 10;
    
    let result = execute_program(program_id, n);
    println!("Program(id={}) input={} result={}", program_id, n, result);
}
