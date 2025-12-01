# RISC Zero zkVM Fibonacci Demo

This is a demonstration of using [RISC Zero zkVM](https://github.com/risc0/risc0) to compute Fibonacci numbers with zero-knowledge proofs.

## About RISC Zero zkVM

RISC Zero is a general-purpose zero-knowledge virtual machine that provides:
- RISC-V instruction set compatibility
- Industry-leading performance for proof generation
- Multiple proof backends (STARK, Groth16, SNARK)
- Rust development toolchain support
- Hardware acceleration support (CUDA for NVIDIA GPUs)
- Extensive tooling and debugging capabilities

## Project Structure

```
risc0-zkvm/
├── risc0-guest/         # Guest program (runs inside zkVM)
│   ├── Cargo.toml       # Build configuration
│   ├── build.rs         # Build script
│   └── guest/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs  # Fibonacci computation logic
├── risc0-host/          # Host program (manages proving/verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Prover and verifier logic
└── rust-toolchain.toml  # Rust toolchain specification
```

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install RISC Zero toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_risc0_sdk.sh
   ```

   This script will:
   - Install `rzup` (RISC Zero toolchain installer)
   - Install `cargo-risczero` (version 3.0.3)
   - Install RISC-V compiler toolchain
   - Install r0vm (RISC Zero VM runtime)
   - Optionally build CUDA-enabled version for GPU acceleration

## Configuration

Set the Fibonacci number to compute via environment variable:

```bash
# Create .env file in the project root or set environment variable
export FIBONACCI_N=10
```

Or create a `.env` file in the workspace root:
```
FIBONACCI_N=10
```

## Building

```bash
cd risc0-zkvm/risc0-host
cargo build --release
```

The build process will:
1. Use `risc0-build` to compile the guest program to RISC-V
2. Generate the method ELF and image ID
3. Compile the host program with RISC Zero SDK dependencies

## Running

### Generate and verify proof:

```bash
cd risc0-zkvm/risc0-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Create an executor environment with the input
3. Generate a proof using the specified proof mode (Groth16)
4. Display proof statistics (size, generation time)
5. Verify the proof against the image ID

## Expected Output

```
fib_n = 10

Executing program in zkVM...
fibonacci result: 89

Generating proof...
proof mode: Groth16, proof size: XXXX Bytes, proof time total cost(s): X.XXX

Verifying proof...
✓ Proof verified successfully!
```

## Proof Modes

RISC Zero supports multiple proof types. Edit `risc0-host/src/main.rs` to change modes:

```rust
// Available options:
let opts = ProverOpts::stark();      // Pure STARK proof
let opts = ProverOpts::groth16();    // STARK + Groth16 (default)
let opts = ProverOpts::compressed(); // Compressed STARK proof
```

### Proof Mode Comparison

| Mode | Proof Size | Generation Time | Verification Time | Use Case |
|------|-----------|-----------------|-------------------|----------|
| STARK | Large (~100KB+) | Fast | Fast | Development/Testing |
| Compressed | Medium (~10KB) | Medium | Medium | General Purpose |
| Groth16 | Small (~256B) | Slow | Very Fast | On-chain Verification |

## Features

- **Cycle Tracking**: Guest program includes cycle counting for performance analysis
- **Multiple Proof Types**: Support for STARK, Groth16, and compressed proofs
- **Journal Output**: Ability to commit public outputs to the proof
- **Hardware Acceleration**: CUDA support for faster proving on NVIDIA GPUs
- **Flexible Input**: Support for complex input types via serialization

## GPU Acceleration

To use GPU acceleration (NVIDIA CUDA):

1. Ensure CUDA is installed on your system
2. Use the CUDA-enabled r0vm:
   ```bash
   export RISC0_PROVER=cuda
   cargo run --release
   ```

This can provide 10-100x speedup for proof generation.

## Development Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

### Cycle Count Analysis

The guest program tracks cycle counts:

```rust
let start = env::cycle_count();
// ... computation ...
let end = env::cycle_count();
eprintln!("fibonacci (cycle tracker): {}", end - start);
```

### Working with Journal Output

To read public outputs from the proof:

```rust
// In guest (main.rs)
env::commit(&result);

// In host
let output: u32 = receipt.journal.decode().unwrap();
println!("Public output: {}", output);
```

## Resources

- [RISC Zero Documentation](https://dev.risczero.com/)
- [RISC Zero GitHub Repository](https://github.com/risc0/risc0)
- [RISC Zero Study Club](https://www.risczero.com/studyclub)
- [RISC Zero Developer Portal](https://dev.risczero.com/api)
- [Performance Best Practices](https://dev.risczero.com/api/zkvm/benchmarking)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on:
  - Input size (problem complexity)
  - Proof mode selected
  - Hardware capabilities (CPU/GPU)
  - System memory available
- For production use, consider iterative Fibonacci implementation for better performance
- The build script automatically handles guest program compilation

## Version Information

- RISC Zero zkVM: 3.0.3
- Rust Toolchain: As specified in `rust-toolchain.toml`
- Rust Edition: 2021
- RISC-V Target: `riscv32im-unknown-none-elf`

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Reinstall RISC Zero toolchain
rzup update
cargo risczero install

# Clean build artifacts
cargo clean
cargo build --release
```

### Performance Issues

For slow proof generation:
- Try using GPU acceleration if available
- Consider using STARK mode for development
- Reduce input size for testing
- Ensure system has sufficient RAM (8GB+ recommended)

### CUDA Issues

If GPU acceleration isn't working:
- Verify CUDA installation: `nvcc --version`
- Check GPU compatibility: `nvidia-smi`
- Ensure CUDA version matches RISC Zero requirements
- Rebuild r0vm with CUDA support (see install script)

## License

MIT OR Apache-2.0

