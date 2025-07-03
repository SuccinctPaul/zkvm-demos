use risc0_zkvm::guest::env;

fn main() {
    // read the input
    let input: u32 = env::read();

    let start = env::cycle_count();
    let res = fib::fibonacci(input);
    let end = env::cycle_count();
    eprintln!("fibonacci (cycle tracker): {}", end - start);

    println!("fibonacci result: {res}");

    // write public output to the journal
    // env::commit(&input);
}
