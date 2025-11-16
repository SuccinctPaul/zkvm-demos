//! Powdr zkVM Guest Program - Fibonacci Computation
//! 
//! This is the guest program that runs inside the Powdr zkVM.
//! It computes the nth Fibonacci number using the shared fib library.

#![no_main]
#![no_std]

use core::panic::PanicInfo;

// Panic handler for no_std environment
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// External functions provided by Powdr runtime
extern "C" {
    fn powdr_read_u32() -> u32;
    fn powdr_write_u32(value: u32);
}

// Entry point for the guest program
#[no_mangle]
pub extern "C" fn main() {
    // Read input from the host
    let n = unsafe { powdr_read_u32() };
    
    // Compute Fibonacci number using the shared library
    let result = fib::fibonacci(n);
    
    // Write result back to the host
    unsafe {
        powdr_write_u32(result);
    }
}

