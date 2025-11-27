// zkWasm guest program - Multi-Program Support
// The entry point must be named 'zkmain' as required by zkWasm

#![no_std]

use core::panic::PanicInfo;
use zkvm_programs::execute_program;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// External host functions that can be called from the guest
// These are provided by the zkWasm runtime
extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

/// Main entry point for zkWasm
/// This function will be proved in zero-knowledge
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        // Read inputs (1 = public input)
        let program_id = wasm_input(1) as u32;
        let n = wasm_input(1) as u32;
        
        // Execute selected program
        let result = execute_program(program_id, n);
        
        // Output result
        wasm_output(result as i64);
        
        result as i64
    }
}
