# OpenVM zkVM Fibonacci Demo

This is a demonstration of using [OpenVM](https://github.com/openvm-org/openvm) to compute Fibonacci numbers with zero-knowledge proofs.

## About OpenVM

OpenVM is a high-performance, modular zkVM developed by OpenVM Foundation that provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular architecture with customizable proving backends
- Optimized performance for various proof types
- Flexible configuration options
- High-level API for proof generation and verification

## Project Structure

```
openvm-zkvm/
├── openvm-guest/        # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Fibonacci computation logic
├── openvm-host/         # Host program (manages proving/verification)
│   ├── Cargo.toml
│   ├── build.rs         # Build script for guest program
│   └── src/
│       └── main.rs      # Prover and verifier logic
├── Cargo.toml           # Workspace configuration
└── rust-toolchain.toml  # Rust toolchain specification
```

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install OpenVM toolchain:
   ```bash
   cd scripts/sdk_installers
e
   ```

   This script will:
   - Install `cargo-openvm` CLI tool
   - Setup aggregation keys for proof generation
   - Install the required Rust toolchain (nightly-2025-02-14)

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
cd openvm-zkvm/openvm-host
cargo build --release
```

The build process will:
1. Use the build script to compile the guest program
2. Generate the guest ELF binary
3. Compile the host program with OpenVM SDK dependencies

## Running

### Execute the program and generate proof:

```bash
cd openvm-zkvm/openvm-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Initialize the OpenVM prover with default configuration
3. Load and execute the guest program in the zkVM
4. Generate a zero-knowledge proof
5. Verify the proof
6. Display execution statistics (cycles, proof size, timing)

## Expected Output

```
fib_n = 10

1. Initializing OpenVM prover...
   Initialization completed in 0.05s

2. Loading guest program...
   Guest program loaded in 0.01s
   ELF size: XXXX bytes

3. Executing program in zkVM...
   Execution completed in 0.15s
   Cycle count: XXXX
   Fibonacci(10) = 89

4. Generating zero-knowledge proof...
   Proof generation completed in 5.50s
   Proof size: XXXX bytes

5. Verifying proof...
   Verification completed in 0.25s
   ✓ Proof verified successfully!

============ Summary ============
Input: n = 10
Output: fibonacci(10) = 89
Total cycles: XXXX
Proof size: XXXX bytes
Prove time: 5.50s
=================================
```

## Features

- **Modular Design**: OpenVM's modular architecture allows for flexible configuration
- **Performance**: Optimized execution and proving times
- **RISC-V Compatibility**: Supports standard RISC-V instruction set
- **Rust Integration**: Seamless Rust development experience
- **Public Input/Output**: Support for committed values via `openvm::io::commit`
- **Logging**: Guest program logging with `openvm::println!`

## Working with Inputs/Outputs

### Reading Input in Guest

```rust
// In guest program (openvm-guest/src/main.rs)
let n: u32 = openvm::io::read();
```

### Committing Output in Guest

```rust
// Make result public
let result = fib::fibonacci(n);
openvm::io::commit(&result);
```

### Reading Committed Output in Host

```rust
// In host program
let (output, _report) = prover.execute(elf, stdin)?;
let result: u32 = output.read();
println!("Result: {}", result);
```

## Development Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

### Custom Prover Configuration

```rust
use openvm_sdk::{config::ProverConfig, Prover};

// Create custom configuration
let config = ProverConfig::default()
    .with_log_level("info");

let prover = Prover::new(&config)?;
```

## Resources

- [OpenVM Documentation](https://docs.openvm.dev/)
- [OpenVM GitHub Repository](https://github.com/openvm-org/openvm)
- [OpenVM Examples](https://github.com/openvm-org/openvm/tree/main/examples)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on:
  - Input size (problem complexity)
  - System hardware capabilities
  - Prover configuration settings
- For production use, consider:
  - Iterative Fibonacci implementation for better performance
  - Custom prover configurations for optimization
  - Profiling to identify bottlenecks

## Version Information

- OpenVM SDK: Latest stable version
- Rust Toolchain: nightly-2025-02-14
- Rust Edition: 2021

## Troubleshooting

### Installation Issues

If OpenVM installation fails:

```bash
# Manual installation
cargo install cargo-openvm

# Setup aggregation keys
cargo openvm setup

# Verify installation
cargo openvm --version
```

### Build Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cd openvm-host
cargo build --release
```

### Runtime Errors

For runtime issues:
- Check that environment variables are set correctly
- Ensure sufficient memory is available for proof generation (8GB+ recommended)
- Enable debug logging: `RUST_LOG=debug cargo run --release`

### Performance Issues

For slow proof generation:
- Ensure system has sufficient resources
- Try building with optimizations: `--release` flag
- Consider reducing input size for testing
- Check system resource usage during proving

## License

MIT OR Apache-2.0


