// zkWasm guest program - Fibonacci computation
// The entry point must be named 'zkmain' as required by zkWasm

#![no_std]

use core::panic::PanicInfo;

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

/// Compute the nth Fibonacci number
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let mut a = 0u64;
    let mut b = 1u64;
    
    for _ in 2..=n {
        let temp = a.wrapping_add(b);
        a = b;
        b = temp;
    }
    
    b
}

/// Main entry point for zkWasm
/// This function will be proved in zero-knowledge
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        // Read input (1 = public input)
        let n = wasm_input(1) as u64;
        
        // Compute Fibonacci
        let result = fibonacci(n);
        
        // Output result
        wasm_output(result as i64);
        
        result as i64
    }
}

