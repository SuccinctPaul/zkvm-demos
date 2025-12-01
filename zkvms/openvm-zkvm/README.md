# OpenVM zkVM Multi-Program Demo

This is a demonstration of using [OpenVM](https://github.com/openvm-org/openvm) to run multiple benchmark programs with zero-knowledge proofs.

## About OpenVM

OpenVM is a high-performance, modular zkVM developed by OpenVM Foundation that provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular architecture with customizable proving backends
- Optimized performance for various proof types
- Flexible configuration options
- High-level API for proof generation and verification

## Supported Programs

| Program | Description | Input `n` |
|---------|-------------|-----------|
| fibonacci | Compute nth Fibonacci number | Index (0-indexed) |
| sum | Sum integers 1..=n | Upper bound |
| factorial | Compute n! | Number |
| isprime | Check if n is prime | Number to check |
| popcount | Count set bits in n | Number |
| hash | SHA256 hash of n bytes | Bytes (max 1024) |
| signature | ECDSA verification simulation | Iterations (1-100) |

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install OpenVM toolchain:
   ```bash
   # Install cargo-openvm CLI
   cargo install cargo-openvm
   
   # Setup aggregation keys (required for proof generation)
   cargo openvm setup
   ```

## Configuration

Set the program and input via environment variables:

```bash
# Program ID (0=fibonacci, 1=sum, 2=factorial, 3=isprime, 4=popcount, 5=hash, 6=signature)
export PROGRAM_ID=0

# Input parameter N
export PROGRAM_N=20

# Or use FIBONACCI_N for backward compatibility
export FIBONACCI_N=20
```

Or create a `.env` file:
```
PROGRAM_ID=0
PROGRAM_N=20
```

## Build Process

The build uses **cargo-openvm** to compile the guest program for the zkVM target.

### Step 1: Build Guest (automatically via build.rs)

When you build the host, the `build.rs` script automatically runs:
```bash
cargo openvm build
```
in the `openvm-guest/` directory to compile the guest program.

### Step 2: Build Host

```bash
cd zkvms/openvm-zkvm/openvm-host
cargo build --release
```

The complete build flow:
1. `build.rs` invokes `cargo openvm build` on the guest
2. Guest ELF is generated for the OpenVM zkVM target
3. Host includes the guest ELF via `openvm_sdk::include_guest!()`
4. Host is compiled with OpenVM SDK dependencies

### Manual Guest Build (optional)

To build the guest separately:
```bash
cd zkvms/openvm-zkvm/openvm-guest
cargo openvm build
```

## Running

### Execute and Generate Proof

```bash
cd zkvms/openvm-zkvm/openvm-host
cargo run --release
```

Or with specific program:
```bash
PROGRAM_ID=5 PROGRAM_N=500 cargo run --release  # Hash 500 bytes
```

The program will:
1. Load program configuration from environment
2. Initialize the OpenVM prover
3. Execute the guest program in the zkVM
4. Generate a zero-knowledge proof
5. Verify the proof
6. Output benchmark metrics

## Expected Output

```
╔════════════════════════════════════════╗
║       OpenVM Multi-Program Demo       ║
╚════════════════════════════════════════╝
📋 Program: fibonacci (ID=0)
ℹ️  Description: Compute nth Fibonacci number
📊 Input N: 20

1. Initializing OpenVM prover...
   Initialization completed in 0.05s

2. Loading guest program...
   Guest program loaded in 0.01s
   ELF size: XXXX bytes

3. Executing program in zkVM...
   Execution completed in 0.15s
   Cycle count: XXXX
   Result: 6765

4. Generating zero-knowledge proof...
   Proof generation completed in 5.50s
   Proof size: XXXX bytes

5. Verifying proof...
   Verification completed in 0.25s
   ✓ Proof verified successfully!

============ Summary ============
Program: fibonacci
Input N: 20
Output: 6765
Total cycles: XXXX
Proof size: XXXX bytes
Prove time: 5.50s
=================================

BENCHMARK: program_name=fibonacci_20
BENCHMARK: zkvm_name=openvm
BENCHMARK: success_status=success
```

## BENCHMARK Output Format

The program outputs standardized benchmark metrics:

```
BENCHMARK: program_name=<program>_<n>
BENCHMARK: zkvm_name=openvm
BENCHMARK: zkvm_version=v0.1.0
BENCHMARK: proof_mode=core
BENCHMARK: elf_size_bytes=<size>
BENCHMARK: execution_time_s=<time>
BENCHMARK: total_cycles=<cycles>
BENCHMARK: output_result=<result>
BENCHMARK: proof_time_s=<time>
BENCHMARK: proof_size_bytes=<size>
BENCHMARK: verification_time_s=<time>
BENCHMARK: success_status=success
BENCHMARK: total_time_s=<total>
```

## Project Structure

```
openvm-zkvm/
├── Cargo.toml              # Workspace configuration
├── rust-toolchain.toml     # Rust nightly toolchain
├── openvm-guest/           # Guest program (runs in zkVM)
│   ├── Cargo.toml
│   └── src/main.rs         # no_std guest code
└── openvm-host/            # Host program (proof generation)
    ├── Cargo.toml
    ├── build.rs            # Runs `cargo openvm build` on guest
    └── src/main.rs         # Host code with OpenVM SDK
```

## Key Files

### `openvm-host/build.rs`
```rust
// Automatically builds guest using cargo-openvm
let status = Command::new("cargo")
    .args(&["openvm", "build"])
    .current_dir("../openvm-guest")
    .status()?;
```

### `openvm-guest/src/main.rs`
```rust
#![no_main]
#![no_std]

openvm::entry!(main);

pub fn main() {
    let program_id: u32 = openvm::io::read();
    let n: u32 = openvm::io::read();
    let result = execute_program(program_id, n);
    openvm::io::commit(&result);
}
```

### `openvm-host/src/main.rs`
```rust
openvm_sdk::include_guest!();  // Include compiled guest ELF

fn main() {
    let prover = Prover::new(&config)?;
    let (output, report) = prover.execute(GUEST_ELF, stdin)?;
    let proof = prover.prove(GUEST_ELF, stdin)?;
    prover.verify(&proof)?;
}
```

## Development Tips

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

### Clean Build

```bash
# Clean all build artifacts
cargo clean

# Rebuild
cd openvm-host
cargo build --release
```

## Troubleshooting

### Installation Issues

```bash
# Install cargo-openvm
cargo install cargo-openvm

# Setup aggregation keys
cargo openvm setup

# Verify installation
cargo openvm --version
```

### Build Errors

If guest build fails:
```bash
# Check cargo-openvm is installed
which cargo-openvm

# Try manual guest build
cd openvm-guest
cargo openvm build

# Check for errors
```

### "Force-skipping unavailable component" Warning

This warning can be safely ignored - it's from an unrelated target configuration.

### Runtime Errors

- Check environment variables are set correctly
- Ensure sufficient memory (8GB+ recommended)
- Enable debug logging: `RUST_LOG=debug`

### Performance Tips

- Always use `--release` flag
- Ensure system has sufficient resources
- Start with smaller input values for testing

## Resources

- [OpenVM Documentation](https://docs.openvm.dev/)
- [OpenVM GitHub Repository](https://github.com/openvm-org/openvm)
- [OpenVM Examples](https://github.com/openvm-org/openvm/tree/main/examples)

## Version Information

- OpenVM SDK: v1.4.0
- Rust Toolchain: nightly-2025-02-14
- Rust Edition: 2021

## License

MIT OR Apache-2.0
