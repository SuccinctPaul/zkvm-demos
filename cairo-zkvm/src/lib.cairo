// Cairo 2.x Multi-Program Dispatcher
// This library provides various benchmark functions and a dispatcher

use core::traits::TryInto;
use core::option::OptionTrait;

/// Compute the nth Fibonacci number iteratively
pub fn fib_iterative(n: felt252) -> felt252 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    
    let n_u32: u32 = n.try_into().unwrap();
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;
    let mut i: u32 = 2;
    
    loop {
        if i > n_u32 { break; }
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    };
    b
}

/// Compute Sum(n) = n*(n+1)/2
pub fn sum_n(n: felt252) -> felt252 {
    let n_u32: u32 = n.try_into().unwrap();
    let res = n_u32 * (n_u32 + 1) / 2;
    res.into()
}

/// Compute Factorial(n)
pub fn factorial(n: felt252) -> felt252 {
    let n_u32: u32 = n.try_into().unwrap();
    let mut res: u32 = 1;
    let mut i: u32 = 2;
    loop {
        if i > n_u32 { break; }
        res = res * i;
        i += 1;
    };
    res.into()
}

/// Dispatcher function
/// program_id: 0=Fib, 1=Sum, 2=Factorial
pub fn execute_program(program_id: felt252, n: felt252) -> felt252 {
    if program_id == 0 {
        fib_iterative(n)
    } else if program_id == 1 {
        sum_n(n)
    } else if program_id == 2 {
        factorial(n)
    } else {
        0 // Unknown
    }
}

/// Main function demonstrating the dispatcher
fn main() -> felt252 {
    // In a real zkVM execution, these inputs would come from the prover arguments
    let program_id: felt252 = 6; // Run Fibonacci
    let n: felt252 = 1;
    
    execute_program(program_id, n)
}
