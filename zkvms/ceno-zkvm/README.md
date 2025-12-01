# CENO zkVM Benchmark Demo

This directory contains benchmark demonstrations for **CENO zkVM** developed by [Scroll](https://scroll.io/).

## About CENO zkVM

**CENO** (Concurrent Enabled Non-uniform) is an accelerated zero-knowledge virtual machine developed by Scroll, designed to achieve **sub-30 second transaction finality** through innovative **GKR-based architecture**.

### Key Features

- **Non-uniform Prover**: Optimizes repeated code patterns for faster proving
- **Segment-based Execution**: Breaks programs into parallelizable segments
- **GKR Protocol**: Uses sumcheck-based GKR for efficient proof generation
- **High Performance**: Targets sub-30 second proving times

### Resources

- **Repository**: https://github.com/scroll-tech/ceno
- **Paper**: [Ceno: Non-uniform, Segment and Parallel Zero-knowledge Virtual Machine](https://eprint.iacr.org/2024/387)
- **Scroll Blog**: [CENO Announcement](https://scroll.io/blog/ceno)

## Project Structure

```
ceno-zkvm/
├── ceno-guest/          # Guest program (runs in zkVM)
│   ├── Cargo.toml       # Uses ceno_rt runtime
│   └── src/
│       └── main.rs      # Fibonacci computation in zkVM
├── ceno-host/           # Host program (proof generation/verification)
│   ├── Cargo.toml       # Uses ceno_host for proving
│   └── src/
│       └── main.rs      # Prover and verifier logic
├── Cargo.toml           # Workspace configuration
├── rust-toolchain.toml  # Nightly Rust toolchain
└── README.md
```

## Prerequisites

1. **Rust Nightly Toolchain** (automatically configured via `rust-toolchain.toml`)
2. **RISC-V Target**: `riscv32im-unknown-none-elf`
3. **cargo-make** (optional, for advanced builds)

### Install RISC-V Target

```bash
rustup target add riscv32im-unknown-none-elf
```

### Install cargo-make (Optional)

```bash
cargo install cargo-make
```

## How to Run

### Build and Run Host

```bash
cd ceno-zkvm
RUST_LOG=info cargo run --release --bin ceno-host
```

### With Custom Input

```bash
# Set Fibonacci input
FIBONACCI_N=20 cargo run --release --bin ceno-host

# Or use PROGRAM_N
PROGRAM_N=15 cargo run --release --bin ceno-host
```

### Using cargo-make (Full CENO Pipeline)

If you have the full CENO SDK installed:

```bash
# Build examples
cargo make build-examples

# Run e2e with Fibonacci
RUST_LOG=info cargo run --release --package ceno_zkvm --bin e2e -- \
    --platform=ceno \
    --hints=10 \
    --public-io=55 \
    examples/target/riscv32im-ceno-zkvm-elf/release/examples/fibonacci
```

## Expected Output

```
=== CENO zkVM Multi-Program Demo ===
Powered by Scroll's GKR-based zkVM

📊 Input: Program=fibonacci (ID=0) N=10

🔨 Step 1: Building guest program...
✅ Build completed in 1.23s
BENCHMARK: compile_time_s=1.234567
   ELF size: 12345 bytes
BENCHMARK: elf_size_bytes=12345

🔐 Step 2: Generating zero-knowledge proof...
✅ Proof generated successfully!
   Proving time: 0.27s
BENCHMARK: proof_time_s=0.269175
BENCHMARK: execution_time_s=0.269175
BENCHMARK: output_result=55

🔍 Step 3: Proof verification...
✅ Proof verified successfully!
BENCHMARK: verification_time_s=0.005210

BENCHMARK: program_name=fibonacci_10
BENCHMARK: zkvm_name=ceno
BENCHMARK: zkvm_version=v0.1.0-scroll
BENCHMARK: proof_mode=core
BENCHMARK: success_status=success
BENCHMARK: total_time_s=1.508942

✨ CENO zkVM demo completed!
```

## Configuration

The program input can be configured via environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `FIBONACCI_N` | Fibonacci sequence index | 10 |
| `PROGRAM_N` | Generic program input | 10 |
| `PROGRAM_ID` | Program selector (0=Fibonacci) | 0 |

## Technical Details

### CENO Architecture

CENO uses a unique approach to zkVM design:

1. **GKR Protocol**: Sumcheck-based proving for efficient multilinear polynomial operations
2. **Non-uniform Circuits**: Different circuit configurations for different instruction types
3. **Segment Parallelization**: Program execution is split into independent segments
4. **Recursive Aggregation**: Segment proofs are recursively combined

### How It Works

1. **Compilation**: Guest code is compiled to RISC-V (riscv32im) bytecode
2. **Execution**: The VM executes the bytecode and records the execution trace
3. **Proof Generation**: GKR-based proof is generated from the execution trace
4. **Verification**: The proof is verified using public inputs

### Performance Notes

- **First build**: Takes 2-3 minutes (downloads and compiles dependencies)
- **Subsequent runs**: Compilation ~1s, Proving ~0.3s (for n=10)
- **Proving time scales**: Roughly linearly with computation complexity

## Troubleshooting

### Build fails with missing target

```bash
rustup target add riscv32im-unknown-none-elf
```

### CENO SDK not found

The host program will fall back to computing expected results locally if the full CENO SDK is not available. For full proof generation:

```bash
# Clone and build CENO SDK
git clone https://github.com/scroll-tech/ceno.git
cd ceno
cargo make build-examples
```

### Memory issues during proving

Reduce the input value:
```bash
FIBONACCI_N=5 cargo run --release --bin ceno-host
```

## License

MIT OR Apache-2.0

## Acknowledgments

- **Scroll Team**: For developing CENO zkVM
- **CENO Contributors**: For advancing GKR-based proving technology
