#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};

pub fn main() {
    // Read input from the host
    let n: u32 = read_as();

    // Compute fibonacci using the shared fib library
    let result = fib::fibonacci(n);

    // Commit the result as public output
    commit(&result);
}



