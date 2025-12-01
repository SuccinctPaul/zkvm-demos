// ZKsync Airbender Guest Program - Multi-Program Support
//
// Based on: https://github.com/matter-labs/zksync-airbender/tree/main/examples/dynamic_fibonacci
//
// This is a RISC-V guest program that runs inside the Airbender zkVM.
// It uses the riscv_common library for I/O operations.
//
// Input reading: Uses csr_read_word() to read program_id and n from host
// Output: Uses zksync_os_finish_success() to return result

#![no_std]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![no_main]
#![no_builtins]

use airbender_riscv_common::{csr_read_word, zksync_os_finish_success};

// Use the common programs library for multi-program support
use zkvm_programs::execute_program;

// Assembly entry point (from Airbender scripts)
core::arch::global_asm!(include_str!("asm_reduced.S"));

#[no_mangle]
extern "C" fn eh_personality() {}

#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
unsafe extern "C" fn start_rust() -> ! {
    main()
}

#[export_name = "_setup_interrupts"]
pub unsafe fn custom_setup_interrupts() {
    extern "C" {
        fn _machine_start_trap();
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MachineTrapFrame {
    pub registers: [u32; 32],
}

/// Trap handler
#[link_section = ".trap.rust"]
#[export_name = "_machine_start_trap_rust"]
pub extern "C" fn machine_start_trap_rust(_trap_frame: *mut MachineTrapFrame) -> usize {
    unsafe { core::hint::unreachable_unchecked() }
}

/// Main workload - read inputs from CSR and execute the selected program
unsafe fn workload() -> ! {
    // Read program_id and n from CSR input (provided by host via input.hex)
    // First call reads program_id, second reads n
    let program_id = csr_read_word();
    let n = csr_read_word();

    // Execute the program using the common programs library
    // This ensures consistency with all other zkVM implementations
    let result = execute_program(program_id, n);

    // Output result through registers (Airbender convention)
    // Registers 10-17 are output values, 18-25 are set to 0 for recursion chain
    zksync_os_finish_success(&[result, program_id, n, 0, 0, 0, 0, 0]);
}

#[inline(never)]
fn main() -> ! {
    unsafe { workload() }
}
