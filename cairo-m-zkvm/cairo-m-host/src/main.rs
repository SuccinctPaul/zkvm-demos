//! Cairo-M zkVM Host Program
//!
//! This program demonstrates how to use Cairo-M zkVM to compile, run, and benchmark
//! zero-knowledge proof generation. Cairo-M is a Mobile-first CPU AIR using M31 field
//! and Stwo prover.
//!
//! ## Usage
//! ```bash
//! PROGRAM=fibonacci INPUT_N=100 cargo run --release
//! ```

use anyhow::{Context, Result};
use cairo_m_common::{CairoMValue, InputValue, Program};
use cairo_m_compiler::{compile_cairo, CompilerOptions};
use cairo_m_prover::{
    adapter::import_from_runner_output, prover::prove_cairo_m, prover_config::REGULAR_96_BITS,
    verifier::verify_cairo_m,
};
use cairo_m_runner::{run_cairo_program, RunnerOptions, RunnerOutput};
use once_cell::sync::Lazy;
use std::fs;
use std::time::Instant;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use zkvm_programs::{execute_program, load_program_input};

const CAIRO_M_VERSION: &str = "v0.1.0-alpha";

/// Compiled program cache - shared across runs
static COMPILED_PROGRAM: Lazy<Result<Program, String>> = Lazy::new(|| {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let project_path = std::path::Path::new(manifest_dir)
        .parent()
        .expect("Failed to get project path");
    let programs_dir = project_path.join("programs");

    // TODO:
    let source_path = programs_dir.join("main.cm");
    let source = fs::read_to_string(&source_path)
        .map_err(|e| format!("Failed to read source file {}: {}", source_path.display(), e))?;

    let compiled_output = compile_cairo(
        source,
        format!("{}/", programs_dir.display()),
        CompilerOptions::default(),
    )
    .map_err(|e| format!("Failed to compile Cairo-M program: {:?}", e))?;

    Ok(compiled_output.program.as_ref().clone())
});

fn main() -> Result<()> {
    // Initialize environment and logger
    dotenv::dotenv().ok();
    env_logger::init();

    println!("========================================");
    println!("  Cairo-M zkVM - Multi-Program Demo");
    println!("========================================\n");

    // Load program input from environment
    let input = load_program_input();

    // Output BENCHMARK metadata
    println!(
        "BENCHMARK: program_name={}_{}",
        input.program.name(),
        input.n
    );
    println!("BENCHMARK: zkvm_name=cairo_m");
    println!("BENCHMARK: zkvm_version={}", CAIRO_M_VERSION);

    // Get proof mode from environment (default: core)
    let proof_mode = std::env::var("CAIRO_M_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    println!("📊 Configuration:");
    println!(
        "   Program: {} (ID={})",
        input.program.name(),
        input.program.id()
    );
    println!("   Input N: {}", input.n);
    println!("   Field: M31 (Mersenne 31)");
    println!("   Prover: Stwo (STARK)");
    println!();

    // Calculate expected result using reference implementation
    let expected = execute_program(input.program.id(), input.n);
    println!("   Expected result: {}", expected);
    println!("BENCHMARK: output_result={}", expected);
    println!();

    let total_start = Instant::now();

    // Step 1: Compile Cairo-M program
    println!("🔨 Step 1: Compiling Cairo-M program...");
    let compile_start = Instant::now();

    let program = COMPILED_PROGRAM
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let compile_duration = compile_start.elapsed();
    println!(
        "   ✅ Program compiled in {:.6}s",
        compile_duration.as_secs_f64()
    );
    println!(
        "BENCHMARK: compile_time_s={:.6}",
        compile_duration.as_secs_f64()
    );
    println!();

    // Step 2: Execute program with Cairo-M runner
    println!("🚀 Step 2: Executing program...");
    let exec_start = Instant::now();

    let execution_result = execute_cairo_m_program(program, input.program.id(), input.n)?;

    let exec_duration = exec_start.elapsed();
    println!(
        "   ✅ Execution completed in {:.6}s",
        exec_duration.as_secs_f64()
    );
    println!("   Result: {}", execution_result.result);
    println!("   Cycles: {}", execution_result.cycles);
    println!(
        "BENCHMARK: execution_time_s={:.6}",
        exec_duration.as_secs_f64()
    );
    println!("BENCHMARK: total_cycles={}", execution_result.cycles);
    println!();

    // Verify result matches expected
    if execution_result.result != expected {
        println!("BENCHMARK: success_status=failed");
        anyhow::bail!(
            "Result mismatch! Expected: {}, Got: {}",
            expected,
            execution_result.result
        );
    }

    // Step 3: Proof generation
    println!("🔐 Step 3: Proof generation...");
    let prove_start = Instant::now();

    let segment = execution_result
        .runner_output
        .vm
        .segments
        .into_iter()
        .next()
        .context("No segments found in runner output")?;

    let mut prover_input = import_from_runner_output(
        segment,
        execution_result.runner_output.public_address_ranges,
    )
    .context("Failed to import from runner output")?;

    let pcs_config = REGULAR_96_BITS;

    let proof = prove_cairo_m::<Blake2sMerkleChannel>(&mut prover_input, Some(pcs_config))
        .context("Failed to generate proof")?;

    let prove_duration = prove_start.elapsed();
    let proof_size = proof.stark_proof.size_estimate();

    println!(
        "   ✅ Proof generated in {:.6}s",
        prove_duration.as_secs_f64()
    );
    println!(
        "BENCHMARK: proof_time_s={:.6}",
        prove_duration.as_secs_f64()
    );
    println!("BENCHMARK: proof_size_bytes={}", proof_size);
    println!();

    // Step 4: Verification
    println!("✓ Step 4: Verification...");
    let verify_start = Instant::now();

    verify_cairo_m::<Blake2sMerkleChannel>(proof, Some(pcs_config))
        .context("Failed to verify proof")?;

    let verify_duration = verify_start.elapsed();
    println!(
        "   ✅ Proof verified in {:.6}s",
        verify_duration.as_secs_f64()
    );
    println!(
        "BENCHMARK: verification_time_s={:.6}",
        verify_duration.as_secs_f64()
    );
    println!(
        "BENCHMARK: verification_time_ms={:.3}",
        verify_duration.as_secs_f64() * 1000.0
    );
    println!("BENCHMARK: success_status=success");
    println!();

    // Summary
    let total_duration = total_start.elapsed();
    println!(
        "BENCHMARK: total_time_s={:.6}",
        total_duration.as_secs_f64()
    );

    println!("========================================");
    println!("  📈 Performance Summary");
    println!("========================================");
    println!("Compile time:     {:.6}s", compile_duration.as_secs_f64());
    println!("Execution time:   {:.6}s", exec_duration.as_secs_f64());
    println!("Prove time:       {:.6}s", prove_duration.as_secs_f64());
    println!("Verify time:      {:.6}s", verify_duration.as_secs_f64());
    println!("Total time:       {:.6}s", total_duration.as_secs_f64());
    println!("Proof size:       {} bytes", proof_size);
    println!("Cycles:           {}", execution_result.cycles);
    println!("========================================\n");

    println!("✅ Cairo-M zkVM Demo completed successfully!");

    Ok(())
}

/// Cairo-M execution result
struct CairoMExecutionResult {
    result: u32,
    cycles: u64,
    runner_output: RunnerOutput,
}

/// Execute a Cairo-M program using the runner
fn execute_cairo_m_program(
    program: &Program,
    program_id: u32,
    n: u32,
) -> Result<CairoMExecutionResult> {
    // Determine entrypoint based on program type
    let entrypoint = get_entrypoint(program_id);

    // Prepare arguments
    let args = vec![InputValue::Number(n as i64)];

    // Run the program with cairo-m-runner
    let runner_output = run_cairo_program(program, entrypoint, &args, RunnerOptions::default())
        .context("Failed to run Cairo-M program")?;

    // Extract result from return values
    let result = extract_u32_result(&runner_output.return_values)?;

    // Get cycle count from VM trace length
    let cycles = runner_output.vm.trace.len() as u64;

    Ok(CairoMExecutionResult {
        result,
        cycles,
        runner_output,
    })
}

/// Get entrypoint name for program ID
fn get_entrypoint(program_id: u32) -> &'static str {
    match program_id {
        0 => "fibonacci",
        1 => "sum",
        2 => "factorial",
        3 => "isprime",
        4 => "popcount",
        5 => "hash",
        6 => "signature",
        _ => "main",
    }
}

/// Extract u32 result from Cairo-M return values
fn extract_u32_result(return_values: &[CairoMValue]) -> Result<u32> {
    if return_values.is_empty() {
        anyhow::bail!("No return values from Cairo-M program");
    }

    match &return_values[0] {
        CairoMValue::U32(val) => Ok(*val),
        CairoMValue::Felt(m31) => {
            // M31 is a field element, convert to u32
            // M31 internally stores as u32
            Ok(m31.0)
        }
        CairoMValue::Bool(b) => Ok(if *b { 1 } else { 0 }),
        CairoMValue::Pointer(m31) => Ok(m31.0),
        other => anyhow::bail!("Unexpected return type: {:?}", other),
    }
}
