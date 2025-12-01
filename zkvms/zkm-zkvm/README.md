# ZKM zkVM Fibonacci Demo

This is a demonstration of using [ZKM (Zero-Knowledge Mips)](https://github.com/ProjectZKM/Ziren) to compute Fibonacci numbers with zero-knowledge proofs.

## About ZKM zkVM

ZKM (Zero-Knowledge Mips) is a high-performance zkVM that provides:
- RISC-V instruction set compatibility
- Multiple proof backends (Core, Compressed, PLONK, Groth16)
- Rust development toolchain support
- Network-based proving infrastructure
- Optimized proof generation with Gnark integration
- Production-ready performance

## Project Structure

```
zkm-zkvm/
├── zkm-guest/          # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # Fibonacci computation logic
├── zkm-host/           # Host program (manages proving/verification)
│   ├── Cargo.toml
│   ├── build.rs        # Build script for guest program
│   └── src/
│       └── main.rs     # Prover and verifier logic
├── rust-toolchain.toml # Rust toolchain specification
└── .gitignore
```

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install ZKM zkVM toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_zkm_sdk.sh
   ```

   This script will:
   - Install `zkm-cli` tool
   - Install the required Rust toolchain
   - Setup the ZKM proving infrastructure

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
cd zkm-zkvm/zkm-host
cargo build --release
```

The build process will:
1. Use `zkm-build` to compile the guest program
2. Generate the guest ELF binary
3. Compile the host program with ZKM SDK dependencies

## Running

### Execute and Generate Proof

Generate a zero-knowledge proof:

```bash
cd zkm-zkvm/zkm-host
RUST_LOG=info cargo run --release
```

Output:
```
fib_n = 10

Executing program in zkVM...
fib result: 89

Setting up prover...
Generating proof...

Successfully generated proof!
proof_mode: Groth16, proof size: XXXX Bytes

Verifying proof...
Successfully verified proof!
```

## Proof Modes

ZKM supports multiple proof types. Edit `zkm-host/src/main.rs` to change modes:

```rust
// Available options:
let proof_mode = ZKMProofKind::Core;              // Fast STARK proof (large)
let proof_mode = ZKMProofKind::Compressed;        // Compressed STARK proof (medium)
let proof_mode = ZKMProofKind::Plonk;             // PLONK proof (small)
let proof_mode = ZKMProofKind::Groth16;           // Groth16 proof (smallest, default)
let proof_mode = ZKMProofKind::CompressToGroth16; // Compress then convert to Groth16
```

### Proof Mode Comparison

| Mode | Proof Size | Generation Time | Verification Time | Gas Cost (On-chain) | Use Case |
|------|-----------|-----------------|-------------------|---------------------|----------|
| Core | ~500KB | Fast | Fast | N/A | Development/Testing |
| Compressed | ~100KB | Medium | Medium | ~500K gas | Off-chain Verification |
| Plonk | ~5KB | Slow | Fast | ~300K gas | On-chain (Flexible) |
| Groth16 | ~256B | Slowest | Very Fast | ~280K gas | On-chain (Optimal) |
| CompressToGroth16 | ~256B | Very Slow | Very Fast | ~280K gas | Maximum Compression |

## Features

- **Multiple Proof Types**: Support for Core, Compressed, PLONK, Groth16, and CompressToGroth16
- **Performance Profiling**: Built-in profiling for cycle and instruction counting
- **Native Gnark Integration**: Optimized proof generation for Groth16/PLONK
- **Network Proving**: Optional network-based proof generation
- **Public Input/Output**: Support for committed values in proofs
- **Extensive Logging**: Detailed execution and proving logs

## Performance Optimization

### Using the ZKM Network

For faster proof generation, use the ZKM proving network:

```bash
# Configure network prover (if available)
export ZKM_PROVER=network
export ZKM_PRIVATE_KEY=your_private_key

cargo run --release
```

### Local Hardware Acceleration

```bash
# Use multiple CPU cores
export RAYON_NUM_THREADS=16

# Enable network features for better performance
cargo run --release
```

## Working with Public Inputs/Outputs

### Committing Values in Guest

```rust
// In guest program (zkm-guest/src/main.rs)
let result = fib::fibonacci(n);
println!("fib result: {}", result);

// Optionally commit values to public output
// zkm_zkvm::io::commit(&result);
```

### Reading Input in Guest

```rust
// In guest program
let n = zkm_zkvm::io::read::<u32>();
```

## Resources

- [ZKM GitHub Repository](https://github.com/ProjectZKM/Ziren)
- [ZKM Documentation](https://github.com/ProjectZKM/Ziren/tree/main/docs)
- [Project ZKM Website](https://www.zkm.io/)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on:
  - Input size (problem complexity)
  - Proof mode selected
  - Hardware capabilities
  - Network vs. local proving
- For production use, consider:
  - Iterative Fibonacci implementation for better performance
  - Using ZKM Network for faster proof generation (if available)
  - Optimizing guest code to minimize cycles

## Version Information

- ZKM SDK: 1.2.1
- Rust Edition: 2021
- Minimum Rust Version: 1.85

## Troubleshooting

### Installation Issues

If ZKM installation fails:

```bash
# Manual installation
# Follow instructions at https://github.com/ProjectZKM/Ziren

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
cd zkm-host
cargo build --release
```

### Performance Issues

For slow proof generation:
- Try Core or Compressed mode for faster local proving
- Consider using ZKM Network for production (if available)
- Optimize guest code to reduce cycle count
- Ensure sufficient system resources (16GB+ RAM recommended)

### Runtime Errors

For runtime issues:
- Check that environment variables are set correctly
- Ensure sufficient memory is available for proof generation
- Enable debug logging: `RUST_LOG=debug cargo run --release`

## Advanced Usage

### Custom Proof Configuration

```rust
use zkm_sdk::{ProverClient, ZKMProofKind, ZKMStdin};

let client = ProverClient::new();

// Setup inputs
let mut stdin = ZKMStdin::new();
stdin.write(&fib_n);

// Setup the program
let (pk, vk) = client.setup(FIBONACCI_ELF);

// Generate proof with specific mode
let proof = client.prove(&pk, stdin)
    .groth16()
    .run()
    .expect("failed to generate proof");

// Verify proof
client.verify(&proof, &vk).expect("failed to verify proof");
```

### Benchmarking

```bash
# Enable detailed profiling
RUST_LOG=info cargo run --release

# Analyze execution statistics
# Check output for cycle counts and proof sizes
```

## License

MIT OR Apache-2.0

