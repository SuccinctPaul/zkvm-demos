// Airbender zkVM Guest Program - Fibonacci Computation
//
// Note: This is a reference implementation based on the RISC-V zkVM architecture.
// Airbender is a high-performance RISC-V zkVM developed by zkSync that provides
// efficient zero-knowledge proof generation for RISC-V bytecode execution.
//
// References:
// - zkSync Airbender: https://docs.zksync.io/zk-stack/components/zksync-airbender
// - ere project: https://github.com/eth-act/ere
//
// This implementation follows the standard RISC-V zkVM pattern and should be
// updated once the official Airbender SDK is released.

#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

// RISC-V zkVM entry point
#[cfg(target_arch = "riscv32")]
#[no_mangle]
pub extern "C" fn main() {
    // In a real Airbender implementation, public inputs would be read
    // through the Airbender runtime API. For now, we use a fixed value.
    let n: u32 = 10;
    
    // Compute Fibonacci number
    let result = fib::fibonacci(n);
    
    // In a real implementation, results would be committed through
    // the Airbender zkVM's output mechanism
    core::hint::black_box(result);
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
    
    let n = 10;
    let result = fib::fibonacci(n);
    println!("fib({}) = {}", n, result);
}

