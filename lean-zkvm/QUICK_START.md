# Lean zkVM - Quick Start Guide

Get started with Lean zkVM in 5 minutes!

## Prerequisites

- Rust 1.85+ installed
- ~10 MB disk space
- macOS, Linux, or Windows with WSL2

## Installation

⚠️ **Note**: The lean_prover SDK is not yet publicly available. This is a reference implementation.

```bash
# Clone the zkvm-demos repository
git clone https://github.com/your-org/zkvm-demos.git
cd zkvm-demos/lean-zkvm
```

## Run Your First Proof

### Option 1: Using the Run Script (Recommended)

```bash
./run_demo.sh
```

### Option 2: Using Cargo Directly

```bash
cd lean-host
FIBONACCI_N=10 cargo run --release
```

## What You'll See

```
╔══════════════════════════════════════════════════════════╗
║          Lean zkVM Fibonacci Demo (Reference)           ║
╚══════════════════════════════════════════════════════════╝

📊 Configuration
   Fibonacci input: n = 10
   Target: ~128 bits of security
   Proof system: WHIR + SuperSpartan (AIR-optimized)

🔢 Step 1: Computing Fibonacci(10)...
   Result: fib(10) = 55

🔧 Step 2: Setting up prover...

🔐 Step 3: Generating zero-knowledge proof...
   ✅ Proof generated successfully!
   Proving time: 0.100s
   Proof size: ~450 KiB

🔍 Step 4: Verifying proof...
   ✅ Proof verified successfully!
```

## Try Different Inputs

```bash
# Small input
./run_demo.sh 5

# Medium input
./run_demo.sh 20

# Large input
./run_demo.sh 100
```

## Customize the Guest Program

Edit `lean-guest/src/main.rs`:

```rust
fn fibonacci_iterative(n: u32) -> u64 {
    // Your custom computation here
}
```

## Next Steps

1. **Read the full README**: [README.md](README.md)
2. **Understand the architecture**: [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
3. **Explore the actual project**: https://github.com/leanEthereum/leanMultisig

## Performance Tips

### Enable CPU Optimizations

```bash
RUSTFLAGS='-C target-cpu=native' ./run_demo.sh
```

### Use Release Mode

```bash
# Always use --release for realistic performance
cargo run --release
```

### Logging

```bash
# Info level
RUST_LOG=info cargo run --release

# Debug level
RUST_LOG=debug cargo run --release
```

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Missing Dependencies

```bash
# Update Rust
rustup update

# Check Rust version
rustc --version  # Should be 1.85+
```

## What's Next?

Once the lean_prover SDK is released:

1. **Install the SDK**:
   ```bash
   cargo install lean_prover
   ```

2. **Integrate with your app**:
   ```rust
   use lean_prover::{prove, verify};
   ```

3. **Generate real proofs**:
   - WHIR commitments
   - SuperSpartan AIR proofs
   - ~128-256 KiB proof sizes

## Key Concepts

### What is Lean zkVM?

A minimal zero-knowledge virtual machine optimized for:
- **Speed**: 1.0-1.7 MHz proving
- **Compactness**: 128-450 KiB proofs
- **Post-Quantum**: XMSS signature aggregation

### How Does It Work?

```
Your Program → Bytecode → Execution → AIR Constraints
            → SuperSpartan → WHIR → Proof (~450 KB)
```

### Why Use Lean zkVM?

- ✅ Fastest proving speed (1.7 MHz on M4 Max)
- ✅ Mobile-optimized (KoalaBear field)
- ✅ Post-quantum ready (XMSS)
- ✅ Minimal design (Cairo-inspired)

## Resources

- **GitHub**: https://github.com/leanEthereum/leanMultisig
- **Paper**: See `minimal_zkVM.pdf` in repository
- **Benchmarks**: See README for detailed metrics

## Support

For questions about:
- **This demo**: See [README.md](README.md)
- **Lean zkVM**: Open issue at https://github.com/leanEthereum/leanMultisig

---

**Happy Proving! 🚀**

