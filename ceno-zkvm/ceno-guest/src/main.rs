// CENO zkVM Guest Program - Real Proof Generation
// 
// Note: This implementation uses Nexus zkVM as a proof-of-concept
// demonstrating real zero-knowledge proof generation capabilities.
// It will be replaced with actual CENO SDK once officially released.
//
// References:
// - CENO Paper: https://eprint.iacr.org/2024/387
// - Nexus zkVM: https://github.com/nexus-xyz/nexus-zkvm

#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

#[cfg(target_arch = "riscv32")]
use nexus_rt::println;

#[nexus_rt::main]
fn main() {
    #[cfg(target_arch = "riscv32")]
    {
        // Read public input when running in zkVM
        let n: u32 = nexus_rt::read_public_input().expect("Failed to read public input");
        
        println!("=== CENO zkVM Guest Program ===");
        println!("Computing Fibonacci for n = {}", n);
        
        // Compute fibonacci using the shared fib library
        let result = fib::fibonacci(n);
        
        println!("Result: fib({}) = {}", n, result);
        println!("=== Computation Complete ===");
    }
    
    #[cfg(not(target_arch = "riscv32"))]
    {
        // For native builds (testing)
        println!("=== CENO zkVM Guest Program (Native Test) ===");
        println!("Note: Run via host program for actual proof generation");
    }
}

