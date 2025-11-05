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

mod cli;

use clap::Parser;
use cli::Args;
use sp1_sdk::{include_elf, ProverClient, SP1ProofMode, SP1Stdin};
/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("sp1-guest");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);
    let mut stdin = SP1Stdin::new();
    stdin.write(&fib_n);

    if args.execute {
        // Execute the program
        let (_output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();

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
        println!("Program executed successfully.");
    // println!("report: {}", report);sp1-zkvm  | opcode counts (7187 total instructions):
        // sp1-zkvm  |     1970 add
        // sp1-zkvm  |      713 xor
        // sp1-zkvm  |      697 srl
        // sp1-zkvm  |      696 lw
        // sp1-zkvm  |      671 or
        // sp1-zkvm  |      659 sll
        // sp1-zkvm  |      631 sw
        // sp1-zkvm  |      275 and
        // sp1-zkvm  |      222 jalr
        // sp1-zkvm  |      128 bltu
        // sp1-zkvm  |      111 auipc
        // sp1-zkvm  |      105 bgeu
        // sp1-zkvm  |       89 lbu
        // sp1-zkvm  |       52 beq
        // sp1-zkvm  |       48 bne
        // sp1-zkvm  |       36 sb
        // sp1-zkvm  |       28 sub
        // sp1-zkvm  |       24 sltu
        // sp1-zkvm  |       20 ecall
        // sp1-zkvm  |        4 mul
        // sp1-zkvm  |        3 lb
        // sp1-zkvm  |        3 jal
        // sp1-zkvm  |        1 lhu
        // sp1-zkvm  |        1 bge
        // sp1-zkvm  | syscall counts (20 total syscall instructions):
        // sp1-zkvm  |     8 commit
        // sp1-zkvm  |     8 commit_deferred_proofs
        // sp1-zkvm  |     1 halt
        // sp1-zkvm  |     1 write
        // sp1-zkvm  |     1 hint_len
        // sp1-zkvm  |     1 hint_read
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .mode(SP1ProofMode::Compressed)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
