//! Powdr zkVM Guest Program - Multi-Program Support
//! 
//! This is the guest program that runs inside the Powdr zkVM.

#![no_main]
#![no_std]

use core::panic::PanicInfo;
use common::execute_program;

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
    // Read inputs from the host
    let program_id = unsafe { powdr_read_u32() };
    let n = unsafe { powdr_read_u32() };
    
    // Execute selected program
    let result = execute_program(program_id, n);
    
    // Write result back to the host
    unsafe {
        powdr_write_u32(result);
    }
}
