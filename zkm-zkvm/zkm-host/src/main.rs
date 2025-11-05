//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use zkm_sdk::{include_elf, ProverClient, ZKMStdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("zkm-guest");

fn main() {
    // Setup the logger.
    zkm_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);
    let mut stdin = ZKMStdin::new();
    stdin.write(&fib_n);

    // Execute the program
    let (_output, report) = client.execute(FIBONACCI_ELF, stdin.clone()).run().unwrap();

    // Read the output.
    // let expect = fib::fibonacci(fib_n);
    // assert_eq!(a, expected_a);
    // assert_eq!(b, expected_b);
    // println!("Values are correct!");

    // Record the number of cycles executed.
    println!(
        "Number of instructions: {}",
        report.total_instruction_count()
    );
    println!("Number of cycles: {}", report.total_syscall_count());
    println!("report: {}", report);
    println!("Program executed successfully.");

    println!("\n");
    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Generate the proof
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("failed to generate proof");

    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}
