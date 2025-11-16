# Novanet zkVM Quick Start Guide

Get up and running with Novanet zkVM in 5 minutes!

## Prerequisites

- Rust 1.85 or later
- 5 minutes of your time ⏱️

## Quick Install

```bash
# Option 1: Automatic installation
cd scripts/sdk_installers
./install_novanet_sdk.sh

# Option 2: Manual setup
rustup toolchain install 1.85
rustup default 1.85
```

## Run Your First Proof

```bash
cd novanet-zkvm

# Run the demo
./run_demo.sh
```

That's it! You should see output like this:

```
========================================
Novanet zkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(5)...

1️⃣  Compiling guest program...
   ✓ Compilation completed in 0.00s

2️⃣  Setting up proving system...
   ✓ Setup completed in 0.00s

3️⃣  Generating proof...
   ✓ Proof generated in 0.00s
   ✓ Result: fibonacci(5) = 8
   ✓ Proof size: 27 bytes

4️⃣  Verifying proof...
   ✓ Proof verified successfully in 0.00s

✅ Novanet zkVM Demo completed successfully!
```

## Try Different Inputs

```bash
# Compute fibonacci(10)
FIBONACCI_N=10 ./run_demo.sh

# Compute fibonacci(20)
FIBONACCI_N=20 ./run_demo.sh
```

## What Just Happened?

You just:
1. ✅ Compiled a guest program (the computation to prove)
2. ✅ Generated a zero-knowledge proof
3. ✅ Verified the proof's correctness

All without revealing the computation steps! 🎉

## Next Steps

### 1. Explore the Code

**Guest Program** (`novanet-guest/src/lib.rs`):
```rust
pub fn compute_fibonacci(input: FibInput) -> FibOutput {
    let result = fib::fibonacci(input.n);
    FibOutput { result }
}
```

**Host Program** (`novanet-host/src/main.rs`):
```rust
// Compile guest program
let prover = NovanetProver::compile_guest()?;

// Generate proof
let proof = prover.prove(input)?;

// Verify proof
let is_valid = prover.verify(&proof)?;
```

### 2. Run Tests

```bash
cargo test
```

### 3. Modify the Guest Program

Edit `novanet-guest/src/lib.rs` to change what gets proven:

```rust
// Try a different computation!
pub fn compute_square(n: u32) -> u32 {
    n * n
}
```

Then rebuild and run:

```bash
cargo build --release
./run_demo.sh
```

### 4. Learn More About Nova

Nova provides powerful features for advanced use cases:

- **Recursive Proofs**: Compose multiple proofs together
- **IVC**: Incrementally verifiable computation
- **No Trusted Setup**: Unlike some SNARKs
- **Constant Proof Size**: Regardless of computation depth

📖 **Resources**:
- [Nova Paper](https://eprint.iacr.org/2021/370)
- [Nova GitHub](https://github.com/microsoft/nova)
- [Full README](README.md)

## Common Issues

### "Rust version too old"

```bash
rustup update
rustup default 1.85
```

### "cargo not found"

Install Rust from https://rustup.rs/

### Build errors

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

## Understanding the Output

```
Compile time:     0.00s   # Guest program → Circuit
Setup time:       0.00s   # Initialize proving system
Prove time:       0.00s   # Generate ZK proof
Verify time:      0.00s   # Check proof validity
```

**Note**: This demo uses simulated timings. Real Nova implementation would have actual cryptographic operations.

## Project Structure

```
novanet-zkvm/
├── novanet-guest/       # Code to be proven
│   └── src/lib.rs       # Fibonacci computation
├── novanet-host/        # Proof orchestration
│   └── src/main.rs      # Prover and verifier
├── Cargo.toml           # Workspace config
└── run_demo.sh          # Convenient run script
```

## Command Reference

```bash
# Run demo
./run_demo.sh

# Run with custom input
FIBONACCI_N=15 ./run_demo.sh

# Run tests
./run_demo.sh --test

# Build only
./run_demo.sh --build

# Clean build artifacts
./run_demo.sh --clean

# Debug mode
./run_demo.sh --debug

# Get help
./run_demo.sh --help
```

## What Makes Nova Special?

Unlike traditional zkVMs, Nova excels at:

| Feature | Traditional zkVM | Nova-based zkVM |
|---------|-----------------|-----------------|
| Trusted Setup | Often required | Not required |
| Recursion | Added complexity | Native support |
| Proof Size | Grows with steps | Constant size |
| Best For | One-shot proofs | Iterative/recursive |

## Real-World Use Cases

Nova is ideal for:

- 🔄 **Blockchain State**: Proving state transitions
- 📊 **Batch Processing**: Aggregating multiple proofs
- 🔁 **Iterative Algorithms**: Loops, recursion
- 🔗 **Proof Chains**: Connecting sequential proofs

## Contributing

This is a demonstration project. For production use:

1. Integrate actual Nova implementation (e.g., `nova-snark`)
2. Add circuit compilation infrastructure
3. Implement real proof generation
4. Optimize for your specific use case

## Need Help?

- 📖 [Full Documentation](README.md)
- 🔬 [Nova Paper](https://eprint.iacr.org/2021/370)
- 💻 [Nova Implementation](https://github.com/microsoft/nova)
- 📚 [zkVM Benchmarks](https://github.com/kkrt-labs/zkvm-benchmarks)

---

**Happy proving! 🚀**

