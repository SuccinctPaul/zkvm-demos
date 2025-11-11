# CENO zkVM Fibonacci Demo

This directory contains a Fibonacci sequence demonstration for CENO zkVM developed by Scroll.

## About CENO zkVM

CENO (Concurrent Enabled Non-uniform) is a zero-knowledge virtual machine developed by Scroll, designed to achieve sub-30 second transaction finality through innovative GKR-based architecture.

### Key Features

- **Non-uniform proving**: Optimizes repeated code patterns
- **Segment-based execution**: Breaks programs into parallelizable segments  
- **High performance**: Targets sub-30 second proving times

## Status

⚠️ **Note**: CENO zkVM is under active development by Scroll. This demo provides a template structure based on the CENO paper and common zkVM patterns. The actual implementation will be updated once the CENO SDK is publicly available.

## Resources

* **Paper**: [Ceno: Non-uniform, Segment and Parallel Zero-knowledge Virtual Machine](https://eprint.iacr.org/2024/387)
* **Scroll Blog**: [CENO Announcement](https://scroll.io/blog/ceno)
* **Expected Repository**: https://github.com/scroll-tech/ceno (check for availability)

## Project Structure

```
ceno-zkvm/
├── ceno-guest/          # Guest program (runs in zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Fibonacci computation logic
├── ceno-host/           # Host program (proof generation/verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Prover and verifier logic
├── Cargo.toml           # Workspace configuration
└── rust-toolchain.toml  # Rust toolchain specification
```

## Prerequisites

Once CENO SDK is available:

1. Rust toolchain (stable or as specified by CENO)
2. CENO SDK and CLI tools
3. RISC-V target support

```bash
# Install RISC-V target
rustup target add riscv32im-unknown-none-elf
```

## How to Run (Template)

When CENO SDK becomes available, the usage will likely follow this pattern:

### Development/Execution Mode

```bash
cd ceno-zkvm/ceno-host
RUST_LOG=info cargo run --release
```

### Proof Generation Mode

```bash
cd ceno-zkvm/ceno-host
RUST_LOG=info cargo run --release -- --prove
```

## Configuration

The Fibonacci input value can be configured via the `.env` file in the workspace root:

```bash
FIB_N=10  # Compute the 10th Fibonacci number
```

## What This Demo Shows

This demonstration computes the nth Fibonacci number using zero-knowledge proofs:

1. **Guest Program**: Runs the Fibonacci calculation inside the zkVM
2. **Host Program**: Generates and verifies the zero-knowledge proof
3. **Verification**: Confirms the computation is correct without revealing intermediate steps

## Expected Output

```
=== CENO zkVM Fibonacci Demo ===

Computing Fibonacci for n = 10

Step 1: Compiling guest program...
✓ Compilation completed in X.XXs

Step 2: Executing program and generating proof...
Expected result: 89
✓ Proof generated in X.XXs
  Proof size: TBD

Step 3: Verifying proof...
✓ Proof verified in X.XXs

=== Summary ===
Total time: X.XXs
Result: 89

✓ CENO zkVM demo completed successfully!
```

## Next Steps

1. Monitor the [Scroll GitHub organization](https://github.com/scroll-tech) for CENO SDK release
2. Update dependencies in `Cargo.toml` files once SDK is available
3. Replace placeholder code with actual CENO API calls
4. Test with the official CENO toolchain

## Contributing

Once CENO SDK is released, contributions are welcome to update this demo with the actual implementation.

## License

MIT OR Apache-2.0

