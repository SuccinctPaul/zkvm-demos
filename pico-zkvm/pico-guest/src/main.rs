#![no_main]
#![no_std]

// Import pico-zkvm guest environment
use pico_zkvm::guest::env;

pico_zkvm::entry!(main);

pub fn main() {
    // Read input from the host
    let n: u32 = env::read();

    // Log the input (for debugging)
    env::log(&"Computing fibonacci for n: ");
    env::log_u32(n);

    // Compute fibonacci using the shared fib library
    let result = fib::fibonacci(n);

    // Log the result
    env::log(&"Fibonacci result: ");
    env::log_u32(result);

    // Commit the result as public output
    env::commit(&result);
}



