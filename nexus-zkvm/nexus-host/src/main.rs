use nexus_sdk::nexus_sdk_macros::profile;
use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::Stwo,
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
};
use std::time::Instant;

const GUEST_PACKAGE: &str = "nexus-guest";

// Build with `cargo build --release` to build in release mode.
//
// When run this will generate the `.pb` file, can be opened with `go tool pprof -http=127.0.0.1:8000 [function_name].pb`
//
// reference: https://github.com/nexus-xyz/nexus-zkvm/blob/releases/0.3.4/sdk/macros/README.md
#[profile]
fn main() {
    let fib_n = common::load_fib_n();
    println!("fib_n = {}", fib_n);

    print!("1. Compiling guest program...");
    let compile_start = Instant::now();
    let mut prover_compiler = Compiler::<CargoPackager>::new(GUEST_PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).unwrap();
    let compile_duration = compile_start.elapsed();
    println!("====== Compile Cost: {}s", compile_duration.as_secs_f64());

    let elf = prover.elf.clone(); // save elf for use with verification
    println!("ELF: instructions num: {:?}", elf.instructions.len());

    println!("Proving execution of vm...");
    let now = std::time::Instant::now();
    let (view, proof) = prover
        .prove_with_input::<(), u32>(&(), &fib_n)
        .expect("failed to prove program");
    println!(
        "Prove cost: {:?} s, proof size: {:?} Bytes",
        std::time::Instant::now().duration_since(now).as_secs_f64(),
        proof.size_estimate()
    );

    println!("\n3. Execution Logs:");
    println!("-------------------");
    println!(
        "View: view_tracked_ram_size: {:?}, view_associated_data: {:?}, view_debug_logs: {:?}",
        view.view_tracked_ram_size(),
        view.view_associated_data(),
        view.view_debug_logs()
    );
    match view.logs() {
        Ok(logs) => println!("{}", logs.join("")),
        Err(e) => eprintln!("Error: Failed to retrieve debug logs - {}", e),
    }
    println!("-------------------");

    match view.exit_code() {
        Ok(code) => {
            if code == nexus_sdk::KnownExitCodes::ExitSuccess as u32 {
                println!(
                    "\n4. Execution completed successfully (Exit code: {})",
                    code
                );
            } else {
                eprintln!("\n4. Execution failed (Exit code: {})", code);
                return;
            }
        }
        Err(e) => {
            eprintln!("\nError: Failed to retrieve exit code - {}", e);
            return;
        }
    }

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
