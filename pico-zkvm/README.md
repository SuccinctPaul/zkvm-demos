# Pico zkVM Fibonacci Demo

This is a demonstration of using [Brevis Pico zkVM](https://github.com/brevis-network/pico) to compute Fibonacci numbers with zero-knowledge proofs.

## About Pico zkVM

Pico is a high-performance, modular, general-purpose zkVM developed by Brevis Network. It provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular architecture for flexible proof backend integration
- Real-time Ethereum proofs capability
- Optimized for blockchain integration
- Support for complex ZK applications

## Project Structure

```
pico-zkvm/
├── pico-guest/          # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Fibonacci computation logic
├── pico-host/           # Host program (manages proving/verification)
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

2. Install Pico zkVM toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_pico_sdk.sh
   ```

   This script will:
   - Install the Pico SDK and toolchain
   - Install the required Rust toolchain
   - Setup necessary dependencies

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
cd pico-zkvm/pico-host
cargo build --release
```

The build process will:
1. Use the build script to compile the guest program
2. Generate the guest ELF binary
3. Compile the host program with Pico SDK dependencies

## Running

### Execute the program and generate proof:

```bash
cd pico-zkvm/pico-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Initialize the Pico zkVM prover
3. Build and load the guest program
4. Execute the program in the zkVM
5. Generate a zero-knowledge proof
6. Verify the proof
7. Display execution statistics (cycles, proof size, timing)

## Expected Output

```
fib_n = 10

1. Initializing Pico zkVM prover...
   Initialization completed in 0.03s

2. Building guest program...
   Build completed in 0.01s
   ELF size: XXXX bytes

3. Executing program in zkVM...
   Execution completed in 0.20s
   Cycle count: XXXX
   Fibonacci(10) = 89

4. Generating zero-knowledge proof...
   Proof generation completed in 6.50s
   Proof size: XXXX bytes

5. Verifying proof...
   Verification completed in 0.15s
   ✓ Proof verified successfully!

============ Summary ============
Input: n = 10
Output: fibonacci(10) = 89
Total cycles: XXXX
Proof size: XXXX bytes
Prove time: 6.50s
=================================
```

## Features

- **Modular Architecture**: Flexible proof backend integration
- **Ethereum Integration**: Optimized for real-time blockchain proofs
- **Performance**: Efficient execution and proving times
- **RISC-V Compatibility**: Standard RISC-V instruction set support
- **Public Input/Output**: Support for committed values via `env::commit`
- **Debug Logging**: Guest program logging with `env::log`

## Working with Inputs/Outputs

### Reading Input in Guest

```rust
// In guest program (pico-guest/src/main.rs)
use pico_zkvm::guest::env;

let n: u32 = env::read();
```

### Logging in Guest

```rust
// Log values for debugging
env::log(&"Computing fibonacci for n: ");
env::log_u32(n);
```

### Committing Output in Guest

```rust
// Make result public
let result = fib::fibonacci(n);
env::commit(&result);
```

### Reading Committed Output in Host

```rust
// In host program
let (output, _report) = client.execute(elf, stdin)?;
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
use pico_sdk::{ProverClient, ProverConfig};

// Create custom configuration
let config = ProverConfig::default();
let client = ProverClient::new(config)?;
```

### Analyzing Execution Reports

```rust
// Get detailed execution report
let (_output, report) = client.execute(elf, stdin)?;
println!("Cycle count: {}", report.cycle_count());
```

## Resources

- [Pico zkVM GitHub Repository](https://github.com/brevis-network/pico)
- [Brevis Network Official Website](https://brevis.network/)
- [Brevis Documentation](https://docs.brevis.network/)
- [Pico zkVM Examples](https://github.com/brevis-network/pico/tree/main/examples)

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
  - Integration with Brevis Network for on-chain verification

## Version Information

- Pico SDK: Latest stable version
- Rust Edition: 2021
- Minimum Rust Version: 1.85

## Troubleshooting

### Installation Issues

If Pico installation fails:

```bash
# Manual installation
# Follow instructions at https://github.com/brevis-network/pico

# Verify installation
cargo --version
rustc --version
```

### Build Errors

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Rebuild
cd pico-host
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
- Use release builds with optimizations: `--release` flag
- Consider reducing input size for testing
- Monitor system resource usage during proving

## Advanced Usage

### Ethereum Integration

Pico zkVM is designed for blockchain integration. For production use with Ethereum:

```rust
// Example: Generate proof for on-chain verification
let proof = prover.prove_for_ethereum(elf, stdin)?;

// Proof can be verified on-chain via Brevis contracts
```

### Benchmarking

```bash
# Run with detailed profiling
RUST_LOG=info cargo run --release

# Monitor execution statistics
# Check cycle counts and timing information
```

## License

MIT OR Apache-2.0


