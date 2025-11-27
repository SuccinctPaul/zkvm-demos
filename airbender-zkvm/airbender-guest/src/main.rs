// ZKsync Airbender Guest Program - Multi-Program Support
//
// Based on: https://github.com/matter-labs/zksync-airbender/tree/main/examples/basic_fibonacci
//
// This is a RISC-V guest program that runs inside the Airbender zkVM.
// It uses the riscv_common library for I/O operations.

#![no_std]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![no_main]
#![no_builtins]

use airbender_riscv_common::zksync_os_finish_success;

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

/// Main workload - execute the selected program
unsafe fn workload() -> ! {
    // Read program_id and n from input (via CSR or memory-mapped I/O)
    // For now, use defaults for compilation check
    let program_id: u32 = 0; // Fibonacci
    let n: u32 = 50;

    // Execute the program
    let result = execute_program(program_id, n);

    // Output result through registers (Airbender convention)
    // Registers 10-17 are output values, 18-25 are set to 0 for recursion chain
    zksync_os_finish_success(&[result, 0, 0, 0, 0, 0, 0, 0]);
}

/// Execute program based on ID
fn execute_program(program_id: u32, n: u32) -> u32 {
    match program_id {
        0 => fibonacci(n),
        1 => factorial(n),
        2 => prime_count(n),
        3 => gcd_sum(n),
        4 => collatz_steps(n),
        _ => fibonacci(n), // Default
    }
}

/// Fibonacci sequence
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 2..=n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    b
}

/// Factorial (with overflow wrapping)
fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 2..=n {
        result = result.wrapping_mul(i);
    }
    result
}

/// Count primes up to n
fn prime_count(n: u32) -> u32 {
    if n < 2 {
        return 0;
    }
    let mut count = 0u32;
    for num in 2..=n {
        if is_prime(num) {
            count += 1;
        }
    }
    count
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Sum of GCDs
fn gcd_sum(n: u32) -> u32 {
    let mut sum = 0u32;
    for i in 1..=n {
        for j in 1..=i {
            sum = sum.wrapping_add(gcd(i, j));
        }
    }
    sum
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Collatz sequence steps
fn collatz_steps(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }
    let mut total_steps = 0u32;
    for start in 2..=n {
        let mut num = start;
        let mut steps = 0u32;
        while num != 1 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num = num.wrapping_mul(3).wrapping_add(1);
            }
            steps += 1;
            if steps > 1000 {
                break; // Prevent infinite loops
            }
        }
        total_steps = total_steps.wrapping_add(steps);
    }
    total_steps
}

#[inline(never)]
fn main() -> ! {
    unsafe { workload() }
}

// Panic handler for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
