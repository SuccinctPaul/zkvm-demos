#![no_main]
#![no_std]

openvm::entry!(main);

pub fn main() {
    // Read input from the host
    let n: u32 = openvm::io::read();

    // Compute fibonacci using the shared fib library
    let result = fib::fibonacci(n);

    // Log the result for debugging
    openvm::println!("Computing fibonacci({}) = {}", n, result);

    // Commit the result as public output
    openvm::io::commit(&result);
}



