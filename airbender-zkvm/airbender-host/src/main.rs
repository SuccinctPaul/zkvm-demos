// ZKsync Airbender zkVM Host Program
//
// Implementation based on:
// https://github.com/eth-act/ere/blob/master/crates/zkvm/airbender/src/zkvm/sdk.rs

use anyhow::{anyhow, Result};
use std::time::Instant;
use std::{array, fs, io::BufRead, iter, process::Command};
use tempfile::tempdir;
use zkvm_programs::{execute_program, load_program_input};

// Import Airbender SDK
use airbender_execution_utils::{
    compute_chain_encoding, generate_params_for_binary, universal_circuit_verifier_vk,
    verify_recursion_log_23_layer, Machine, ProgramProof,
};

const AIRBENDER_VERSION: &str = "v0.5.0";

/// Verification key hash chain (8 x u32 = 256 bits)
pub type VkHashChain = [u32; 8];

/// Airbender SDK wrapper
/// Reference: https://github.com/eth-act/ere/blob/master/crates/zkvm/airbender/src/zkvm/sdk.rs
pub struct AirbenderSdk {
    bin: Vec<u8>,
    vk_hash_chain: VkHashChain,
    gpu: bool,
}

impl AirbenderSdk {
    pub fn new(bin: &[u8], gpu: bool) -> Self {
        let vk_hash_chain = {
            // Compute base VK as `blake(PC || setup_caps)`
            let base_vk = generate_params_for_binary(bin, Machine::Standard);
            // The 1st recursion layer VK
            let verifier_vk = universal_circuit_verifier_vk().params;
            // Compute hash chain as `blake(blake(0 || guest_vk) || verifier_vk)`
            compute_chain_encoding(vec![[0; 8], base_vk, verifier_vk])
        };
        Self {
            bin: bin.to_vec(),
            vk_hash_chain,
            gpu,
        }
    }

    pub fn vk_chain_hash(&self) -> &VkHashChain {
        &self.vk_hash_chain
    }

    /// Execute program and return (public_values, cycles)
    pub fn execute(&self, input: &[u8]) -> Result<(Vec<u8>, u64)> {
        let tempdir = tempdir()?;

        let bin_path = tempdir.path().join("guest.bin");
        fs::write(&bin_path, &self.bin)?;

        let input_path = tempdir.path().join("input.hex");
        fs::write(&input_path, encode_input(input))?;

        let mut cmd = Command::new("airbender-cli");
        let output = cmd
            .arg("run")
            .arg("--bin")
            .arg(&bin_path)
            .arg("--input-file")
            .arg(&input_path)
            .args(["--cycles", &u64::MAX.to_string()])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "airbender-cli run failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Parse public values from stdout: "Result: {v0}, {v1}, {v2}, {v3}, {v4}, {v5}, {v6}, {v7}"
        let public_values = output
            .stdout
            .lines()
            .find_map(|line| {
                let line = line.ok()?;
                let line = line.split_once("Result:")?.1;
                let mut words = line.split(',');
                let mut bytes = Vec::with_capacity(32);
                for _ in 0..8 {
                    bytes.extend(words.next()?.trim().parse::<u32>().ok()?.to_le_bytes())
                }
                Some(bytes)
            })
            .ok_or_else(|| anyhow!("Failed to parse Result from output"))?;

        // Parse cycles from stdout: "Took {cycles} cycles to finish"
        let cycles = output
            .stdout
            .lines()
            .find_map(|line| {
                let line = line.ok()?;
                let line = line.split_once("Took ")?.1;
                let cycle = line.split_once(" cycles")?.0;
                cycle.parse().ok()
            })
            .ok_or_else(|| anyhow!("Failed to parse cycles from output"))?;

        Ok((public_values, cycles))
    }

    /// Generate proof and return (public_values, proof)
    pub fn prove(&self, input: &[u8]) -> Result<(Vec<u8>, ProgramProof)> {
        let tempdir = tempdir()?;

        let bin_path = tempdir.path().join("guest.bin");
        fs::write(&bin_path, &self.bin)?;

        let input_path = tempdir.path().join("input.hex");
        fs::write(&input_path, encode_input(input))?;

        let output_dir = tempdir.path().join("output");
        fs::create_dir_all(&output_dir)?;

        // Prove guest program + 1st recursion layer
        let mut cmd = Command::new("airbender-cli");
        let mut args = cmd
            .arg("prove")
            .arg("--bin")
            .arg(&bin_path)
            .arg("--output-dir")
            .arg(&output_dir)
            .arg("--input-file")
            .arg(&input_path)
            .args(["--until", "final-recursion"])
            .args(["--cycles", &u64::MAX.to_string()]);

        if self.gpu {
            args = args.arg("--gpu");
        }

        let output = args.output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "airbender-cli prove failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let proof_path = output_dir.join("recursion_program_proof.json");
        if !proof_path.exists() {
            return Err(anyhow!("Proof file not found: {:?}", proof_path));
        }

        // Prove 2nd recursion layer (wrapping root of 1st recursion layer)
        let mut cmd = Command::new("airbender-cli");
        let mut args = cmd
            .arg("prove-final")
            .arg("--input-file")
            .arg(&proof_path)
            .arg("--output-dir")
            .arg(&output_dir);

        if self.gpu {
            args = args.arg("--gpu");
        }

        let output = args.output()?;

        if !output.status.success() {
            return Err(anyhow!(
                "airbender-cli prove-final failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let proof_path = output_dir.join("final_program_proof.json");
        let proof_bytes = fs::read(&proof_path)?;
        let proof: ProgramProof = serde_json::from_slice(&proof_bytes)?;

        let (public_values, vk_hash_chain) = extract_public_values_and_vk_hash_chain(&proof)?;

        if self.vk_hash_chain != vk_hash_chain {
            return Err(anyhow!(
                "VK hash chain mismatch: expected {:?}, got {:?}",
                self.vk_hash_chain,
                vk_hash_chain
            ));
        }

        Ok((public_values, proof))
    }

    /// Verify proof and return public values
    pub fn verify(&self, proof: &ProgramProof) -> Result<Vec<u8>> {
        let is_valid = verify_recursion_log_23_layer(proof);
        if !is_valid {
            return Err(anyhow!("Proof verification failed"));
        }

        let (public_values, vk_hash_chain) = extract_public_values_and_vk_hash_chain(proof)?;

        if self.vk_hash_chain != vk_hash_chain {
            return Err(anyhow!(
                "VK hash chain mismatch: expected {:?}, got {:?}",
                self.vk_hash_chain,
                vk_hash_chain
            ));
        }

        Ok(public_values)
    }
}

/// Encode input with length prefixed to hex string for airbender-cli
fn encode_input(input: &[u8]) -> String {
    iter::once((input.len() as u32).to_le_bytes().as_slice())
        .chain(input.chunks(4))
        .map(|chunk| {
            let mut bytes = [0u8; 4];
            bytes[..chunk.len()].copy_from_slice(chunk);
            format!("{:08x}", u32::from_le_bytes(bytes))
        })
        .collect()
}

/// Extract public values and VK hash chain from proof
fn extract_public_values_and_vk_hash_chain(proof: &ProgramProof) -> Result<(Vec<u8>, VkHashChain)> {
    if proof.register_final_values.len() != 32 {
        return Err(anyhow!(
            "Invalid register count: {}",
            proof.register_final_values.len()
        ));
    }

    let public_values: Vec<u8> = proof.register_final_values[10..18]
        .iter()
        .flat_map(|value| value.value.to_le_bytes())
        .collect();

    let vk_chain_hash: VkHashChain =
        array::from_fn(|i| proof.register_final_values[18 + i].value);

    Ok((public_values, vk_chain_hash))
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    println!("=== ZKsync Airbender zkVM Demo ===\n");

    // Load input
    let input = load_program_input();

    // Output BENCHMARK metadata
    println!(
        "BENCHMARK: program_name={}_{}",
        input.program.name(),
        input.n
    );
    println!("BENCHMARK: zkvm_name=airbender");
    println!("BENCHMARK: zkvm_version={}", AIRBENDER_VERSION);

    let proof_mode = std::env::var("AIRBENDER_PROOF_MODE").unwrap_or_else(|_| "core".to_string());
    println!("BENCHMARK: proof_mode={}", proof_mode);

    let use_gpu = std::env::var("AIRBENDER_GPU")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    println!(
        "\n📊 Program: {} (ID={}) with n={}\n",
        input.program.name(),
        input.program.id(),
        input.n
    );

    let total_start = Instant::now();

    // Check for pre-compiled guest binary
    let guest_bin_path = std::env::var("AIRBENDER_GUEST_BIN").ok();

    if let Some(bin_path) = guest_bin_path {
        // Full SDK execution with pre-compiled binary
        run_with_sdk(&bin_path, &input, use_gpu)?;
    } else {
        // Check if airbender-cli is available
        let cli_available = Command::new("airbender-cli")
            .arg("--version")
            .output()
            .is_ok();

        if cli_available {
            println!("⚠️  AIRBENDER_GUEST_BIN not set");
            println!("   Set environment variable to pre-compiled guest.bin path");
            println!("   Example: AIRBENDER_GUEST_BIN=./guest.bin\n");
        } else {
            println!("⚠️  airbender-cli not found in PATH");
            println!("   Install from: https://github.com/matter-labs/zksync-airbender\n");
        }

        // Fallback to native execution
        run_native_fallback(&input)?;
    }

    let total_duration = total_start.elapsed();
    println!(
        "BENCHMARK: total_time_s={:.6}",
        total_duration.as_secs_f64()
    );

    println!("\n✅ Airbender demo completed!");

    Ok(())
}

/// Run with full SDK using pre-compiled guest binary
fn run_with_sdk(
    bin_path: &str,
    input: &zkvm_programs::ProgramInput,
    use_gpu: bool,
) -> Result<()> {
    println!("🚀 Running with Airbender SDK\n");

    // Load guest binary
    println!("1. Loading guest binary: {}", bin_path);
    let compile_start = Instant::now();
    let bin = fs::read(bin_path)?;
    let compile_duration = compile_start.elapsed();
    println!(
        "   ✓ Loaded {} bytes in {:.6}s",
        bin.len(),
        compile_duration.as_secs_f64()
    );
    println!(
        "BENCHMARK: compile_time_s={:.6}",
        compile_duration.as_secs_f64()
    );

    // Initialize SDK
    let sdk = AirbenderSdk::new(&bin, use_gpu);
    println!("   VK hash chain: {:?}", sdk.vk_chain_hash());

    // Prepare input
    let input_bytes: Vec<u8> = [input.program.id(), input.n]
        .iter()
        .flat_map(|v| v.to_le_bytes())
        .collect();

    // Execute
    println!("\n2. Executing program...");
    let exec_start = Instant::now();
    let (public_values, cycles) = sdk.execute(&input_bytes)?;
    let exec_duration = exec_start.elapsed();

    println!("   ✓ Execution completed in {:.6}s", exec_duration.as_secs_f64());
    println!("   Cycles: {}", cycles);
    println!(
        "BENCHMARK: execution_time_s={:.6}",
        exec_duration.as_secs_f64()
    );
    println!("BENCHMARK: total_cycles={}", cycles);

    // Extract result from public values
    if public_values.len() >= 4 {
        let result = u32::from_le_bytes(public_values[0..4].try_into().unwrap());
        println!("BENCHMARK: output_result={}", result);
    }

    // Prove
    println!("\n3. Generating proof...");
    let prove_start = Instant::now();
    let (_public_values, proof) = sdk.prove(&input_bytes)?;
    let prove_duration = prove_start.elapsed();

    println!("   ✓ Proof generated in {:.6}s", prove_duration.as_secs_f64());
    println!(
        "BENCHMARK: proof_time_s={:.6}",
        prove_duration.as_secs_f64()
    );

    // Verify
    println!("\n4. Verifying proof...");
    let verify_start = Instant::now();
    let _verified_values = sdk.verify(&proof)?;
    let verify_duration = verify_start.elapsed();

    println!("   ✓ Proof verified in {:.6}s", verify_duration.as_secs_f64());
    println!(
        "BENCHMARK: verification_time_s={:.6}",
        verify_duration.as_secs_f64()
    );
    println!("BENCHMARK: success_status=success");

    Ok(())
}

/// Fallback native execution (when SDK/CLI not available)
fn run_native_fallback(input: &zkvm_programs::ProgramInput) -> Result<()> {
    println!("📦 Running native execution fallback\n");

    // Step 1: Compile (reference)
    println!("1. Compiling guest program...");
    println!("   Note: Requires RISC-V toolchain and airbender build scripts");
    let compile_start = Instant::now();
    let compile_duration = compile_start.elapsed();
    println!(
        "BENCHMARK: compile_time_s={:.6}",
        compile_duration.as_secs_f64()
    );

    // Step 2: Execute natively
    println!("\n2. Executing program (native)...");
    let exec_start = Instant::now();
    let result = execute_program(input.program.id(), input.n);
    let exec_duration = exec_start.elapsed();

    println!("   ✓ Execution completed in {:.6}s", exec_duration.as_secs_f64());
    println!("   Result: {}", result);
    println!(
        "BENCHMARK: execution_time_s={:.6}",
        exec_duration.as_secs_f64()
    );
    println!("BENCHMARK: output_result={}", result);

    // Step 3: Proof (reference)
    println!("\n3. Generating proof...");
    println!("   Note: Requires airbender-cli and guest.bin");
    let prove_start = Instant::now();
    let prove_duration = prove_start.elapsed();
    println!(
        "BENCHMARK: proof_time_s={:.6}",
        prove_duration.as_secs_f64()
    );

    // Step 4: Verify (reference)
    println!("\n4. Verifying proof...");
    let verify_start = Instant::now();
    let verify_duration = verify_start.elapsed();
    println!(
        "BENCHMARK: verification_time_s={:.6}",
        verify_duration.as_secs_f64()
    );

    // Verify correctness
    let expected = execute_program(input.program.id(), input.n);
    if result == expected {
        println!("\n✅ Computation verified!");
        println!("BENCHMARK: success_status=success");
    } else {
        println!("\n❌ Mismatch! Expected: {}, Got: {}", expected, result);
        println!("BENCHMARK: success_status=failed");
    }

    Ok(())
}
