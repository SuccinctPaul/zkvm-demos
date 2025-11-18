# SP1 zkVM Multi-Program Demo

This is a demonstration of using [Succinct SP1 zkVM](https://github.com/succinctlabs/sp1) to run multiple programs with zero-knowledge proofs.

## 🎯 Multi-Program Support

This demo showcases a **single guest binary** that can execute **7 different programs**:
- Fibonacci, Sum, Factorial, IsPrime, PopCount, Hash (SHA256), Signature (ECDSA)

**Key Innovation:** Runtime program selection via `program_id` parameter, eliminating the need for multiple guest binaries.

## About SP1 zkVM

SP1 (Succinct Processor 1) is a high-performance, open-source zkVM that provides:
- RISC-V instruction set compatibility
- Blazing-fast proof generation (10x faster than alternatives)
- Multiple proof backends (Core, Compressed, PLONK, Groth16)
- Rust development toolchain support
- Extensive precompiles for common cryptographic operations
- Production-ready infrastructure

## Project Structure

```
sp1-zkvm/
├── sp1-guest/          # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # Fibonacci computation logic
├── sp1-host/           # Host program (manages proving/verification)
│   ├── Cargo.toml
│   ├── build.rs        # Build script for guest program
│   └── src/
│       ├── main.rs     # Prover and verifier logic
│       └── cli.rs      # Command-line interface
├── rust-toolchain.toml # Rust toolchain specification
└── .gitignore
```

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install SP1 zkVM toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_sp1_sdk.sh
   ```

   This script will:
   - Install `sp1up` (SP1 toolchain installer)
   - Install `cargo-prove` CLI tool
   - Install the Succinct Rust toolchain
   - Setup the SP1 proving infrastructure

## Available Programs

| Program | Environment Variable | Description | Recommended Test Value |
|---------|---------------------|-------------|------------------------|
| Fibonacci | `fibonacci` or `fib` | Compute nth Fibonacci number | 20, 30, 100 |
| Sum | `sum` | Sum integers from 1 to n | 100, 1000 |
| Factorial | `factorial` or `fact` | Compute n factorial | 5, 10, 12 |
| IsPrime | `isprime` or `prime` | Check if n is prime | 97, 1009 |
| PopCount | `popcount` or `bitcount` | Count set bits in n | 255, 65535 |
| Hash | `hash` or `sha256` | SHA256 hash computation | 256, 1024 |
| Signature | `signature` or `sig` | ECDSA signature verification | 10, 50 |

**Example:**
```bash
cd sp1-zkvm/sp1-host

# Run Fibonacci
PROGRAM=fibonacci INPUT_N=20 cargo run --release -- --execute

# Run Sum
PROGRAM=sum INPUT_N=100 cargo run --release -- --execute

# Run Factorial
PROGRAM=factorial INPUT_N=10 cargo run --release -- --execute
```

## 📚 Documentation

- **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** - Quick start guide and program reference
- **[MULTI_PROGRAM_ANALYSIS.md](./MULTI_PROGRAM_ANALYSIS.md)** - Detailed architecture analysis and verification
- **[FIBONACCI_VS_SUM.md](./FIBONACCI_VS_SUM.md)** - Mathematical comparison and common confusion explained
- **[compare_programs.sh](./compare_programs.sh)** - Script to compare different programs

**Important Note:** When testing with `n=10`, both `fibonacci(10)` and `sum(10)` return `55`. This is a **mathematical coincidence**, not a bug. See [FIBONACCI_VS_SUM.md](./FIBONACCI_VS_SUM.md) for detailed explanation.

## Configuration

Configure the program to run via environment variables:

```bash
# Select program and input value
export PROGRAM=fibonacci  # or sum, factorial, isprime, popcount, hash, signature
export INPUT_N=20         # input parameter
```

Or create a `.env` file in the workspace root:
```
PROGRAM=fibonacci
INPUT_N=20
```

**Backward Compatibility:** The old `FIBONACCI_N` environment variable is still supported for Fibonacci program.

## Building

```bash
cd sp1-zkvm/sp1-host
cargo build --release
```

The build process will:
1. Use `sp1-build` to compile the guest program
2. Generate the guest ELF binary
3. Compile the host program with SP1 SDK dependencies

## Running

### Execute Only (Development Mode)

Fast execution without proof generation:

```bash
cd sp1-zkvm/sp1-host

# Fibonacci
PROGRAM=fibonacci INPUT_N=20 cargo run --release -- --execute

# Sum
PROGRAM=sum INPUT_N=100 cargo run --release -- --execute

# Factorial
PROGRAM=factorial INPUT_N=10 cargo run --release -- --execute
```

Output example (Sum):
```
╔════════════════════════════════════════╗
║         SP1 Multi-Program Demo        ║
╚════════════════════════════════════════╝
📋 Program: sum (Sum integers from 1 to n)
📊 Input N: 100

⚙️  Executing program...
stdout: SP1 Guest: program_id=1, n=100
stdout: Running Sum(100)
stdout: Result: 5050

✅ Execution Results:
─────────────────────────────────────
📤 Output: 5050
📊 Instructions: 9263
🔄 Cycles: 25

✨ Program executed successfully!
```

### Generate and Verify Proof

Generate a zero-knowledge proof:

```bash
cd sp1-zkvm/sp1-host
PROGRAM=fibonacci INPUT_N=20 cargo run --release -- --prove
```

Output:
```
╔════════════════════════════════════════╗
║         SP1 Multi-Program Demo        ║
╚════════════════════════════════════════╝
📋 Program: fibonacci (Compute the nth Fibonacci number)
📊 Input N: 20

🔧 Setting up proving environment...
🔐 Generating proof...

✅ Successfully generated proof!

📊 Proof Information:
─────────────────────────────────────
🔒 Mode: Groth16
📦 Size: XXXX bytes (XX.XX KB)

🔍 Verifying proof...

✨ Successfully verified proof!
╚════════════════════════════════════════╝
```

## Proof Modes

SP1 supports multiple proof types. Edit `sp1-host/src/main.rs` to change modes:

```rust
// Available options:
let proof_mode = SP1ProofMode::Core;       // Fast STARK proof (large)
let proof_mode = SP1ProofMode::Compressed; // Compressed STARK proof (medium)
let proof_mode = SP1ProofMode::Plonk;      // PLONK proof (small)
let proof_mode = SP1ProofMode::Groth16;    // Groth16 proof (smallest, default)
```

### Proof Mode Comparison

| Mode | Proof Size | Generation Time | Verification Time | Gas Cost (On-chain) | Use Case |
|------|-----------|-----------------|-------------------|---------------------|----------|
| Core | ~500KB | Fast | Fast | N/A | Development/Testing |
| Compressed | ~100KB | Medium | Medium | ~500K gas | Off-chain Verification |
| Plonk | ~5KB | Slow | Fast | ~300K gas | On-chain (Flexible) |
| Groth16 | ~256B | Slowest | Very Fast | ~280K gas | On-chain (Optimal) |

## Features

- **Flexible Execution Modes**: Execute or prove via command-line flags
- **Multiple Proof Types**: Support for Core, Compressed, PLONK, and Groth16
- **Performance Profiling**: Built-in profiling for cycle and instruction counting
- **Native Gnark Integration**: Optimized proof generation for Groth16/PLONK
- **Public Input/Output**: Support for committed values in proofs
- **Extensive Logging**: Detailed execution and proving logs

## CLI Usage

The host program supports command-line arguments:

```bash
# Execute only (no proof)
cargo run --release -- --execute

# Generate proof
cargo run --release -- --prove

# Note: Must specify exactly one of --execute or --prove
```

## Performance Optimization

### Using the SP1 Network

For faster proof generation, use the SP1 proving network:

```bash
# Sign up and get an API key at https://network.succinct.xyz
export SP1_PROVER=network
export SP1_PRIVATE_KEY=your_private_key

cargo run --release -- --prove
```

### Local Hardware Acceleration

```bash
# Use multiple CPU cores
export RAYON_NUM_THREADS=16

# Enable network features for better performance
cargo run --release --features network -- --prove
```

## Working with Public Inputs/Outputs

### Committing Values in Guest

```rust
// In guest program (sp1-guest/src/main.rs)
let result = fib::fibonacci(n);
sp1_zkvm::io::commit(&result);  // Make result public
```

### Reading Committed Values in Host

```rust
// In host program
let (public_values, _) = client.execute(ELF, &stdin).run().unwrap();
let result: u32 = public_values.read();
println!("Public result: {}", result);
```

## Resources

- [SP1 Documentation](https://docs.succinct.xyz/)
- [SP1 GitHub Repository](https://github.com/succinctlabs/sp1)
- [SP1 Book](https://succinctlabs.github.io/sp1/)
- [SP1 Examples](https://github.com/succinctlabs/sp1/tree/main/examples)
- [SP1 Network](https://network.succinct.xyz/) (Managed proving service)
- [Succinct Blog](https://blog.succinct.xyz/)

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
  - Using SP1 Network for faster proof generation
  - Optimizing guest code to minimize cycles
- The `--execute` flag is useful for rapid development and testing

## Version Information

- SP1 SDK: 5.2.2
- Rust Edition: 2021
- Minimum Rust Version: 1.85
- RISC-V Target: Managed by SP1 toolchain

## Troubleshooting

### Installation Issues

If SP1 installation fails:

```bash
# Manual installation
curl -L https://sp1up.succinct.xyz | bash
source ~/.bashrc  # or ~/.zshrc
sp1up

# Verify installation
cargo prove --version
rustup toolchain list | grep succinct
```

### Build Errors

```bash
# Clean build artifacts
cargo clean

# Update SP1 toolchain
sp1up

# Rebuild
cd sp1-host
cargo build --release
```

### Performance Issues

For slow proof generation:
- Use `--execute` mode during development
- Consider using SP1 Network for production
- Try Core or Compressed mode for faster local proving
- Optimize guest code to reduce cycle count
- Ensure sufficient system resources (16GB+ RAM recommended)

### Network Proving

If network proving fails:
```bash
# Check network status
curl https://network.succinct.xyz/status

# Verify API key
echo $SP1_PRIVATE_KEY

# Check balance
sp1 network status
```

## Advanced Usage

### Custom Proof Configuration

```rust
use sp1_sdk::{ProverClient, SP1ProofMode, SP1ProofWithPublicValues};

let client = ProverClient::from_env();
let (pk, vk) = client.setup(ELF);

// Customize proof generation
let proof = client.prove(&pk, &stdin)
    .mode(SP1ProofMode::Groth16)
    .timeout(std::time::Duration::from_secs(300))
    .run()
    .expect("proving failed");
```

### Benchmarking

```bash
# Enable detailed profiling
RUST_LOG=info cargo run --release -- --execute

# Analyze cycle counts
# Check output for "Number of instructions" and "Number of cycles"
```

## License

MIT OR Apache-2.0

