use nexus_sdk::{
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
    compile::{Compile, Compiler, cargo::CargoPackager},
    stwo::seq::Stwo,
};

const GUEST_PACKAGE: &str = "nexus-guest";

fn main() {
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);

    println!("Compiling nexus-zkvm-guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(GUEST_PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile nexus-zkvm-guest program");

    let elf = prover.elf.clone(); // save elf for use with verification

    println!("Proving execution of vm...");
    let now = std::time::Instant::now();
    let (view, proof) = prover
        .prove_with_input::<(), u32>(&(), &fib_n)
        .expect("failed to prove program");
    println!(
        "prove cost: {:?} s ",
        std::time::Instant::now().duration_since(now).as_secs_f64()
    );

    println!(
        ">>>>> Logging\n{}<<<<<",
        view.logs().expect("failed to retrieve debug logs").join("")
    );
    // assert_eq!(
    //     view.exit_code().expect("failed to retrieve exit code"),
    //     nexus_sdk::KnownExitCodes::ExitSuccess as u32
    // );

    print!("Verifying execution...");

    #[rustfmt::skip]
    proof
        .verify_expected::<u32, ()>(
            &fib_n,  // public input
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &(),  // no public output
            &elf, // expected elf (program binary)
            &[],  // no associated data,
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");
}
