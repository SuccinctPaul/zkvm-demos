# CENO zkVM Fibonacci Demo

This directory contains a Fibonacci sequence demonstration for CENO zkVM developed by Scroll.

## About CENO zkVM

CENO (Concurrent Enabled Non-uniform) is a zero-knowledge virtual machine developed by Scroll, designed to achieve sub-30 second transaction finality through innovative GKR-based architecture.

### Key Features

- **Non-uniform proving**: Optimizes repeated code patterns
- **Segment-based execution**: Breaks programs into parallelizable segments  
- **High performance**: Targets sub-30 second proving times

## Current Implementation Status

✅ **Real Zero-Knowledge Proof Generation Enabled!**

Since the official CENO SDK is still under development by Scroll, this implementation uses **Nexus zkVM** as a proof-of-concept to demonstrate real zero-knowledge proof generation and verification capabilities.

### What This Means

- ✅ **Generates real ZK proofs**: Not just simulation, actual cryptographic proofs
- ✅ **Verifies proofs**: Full verification of zero-knowledge properties
- ✅ **Production-ready zkVM**: Uses battle-tested Nexus zkVM technology
- 🔄 **Will migrate to CENO**: Once Scroll releases the official CENO SDK

## Resources

* **Paper**: [Ceno: Non-uniform, Segment and Parallel Zero-knowledge Virtual Machine](https://eprint.iacr.org/2024/387)
* **Scroll Blog**: [CENO Announcement](https://scroll.io/blog/ceno)
* **Expected Repository**: https://github.com/scroll-tech/ceno
* **Current Implementation**: [Nexus zkVM](https://github.com/nexus-xyz/nexus-zkvm)

## Project Structure

```
ceno-zkvm/
├── ceno-guest/          # Guest program (runs in zkVM)
│   ├── Cargo.toml       # Uses nexus-rt for real proof generation
│   └── src/
│       └── main.rs      # Fibonacci computation in zkVM
├── ceno-host/           # Host program (proof generation/verification)
│   ├── Cargo.toml       # Uses nexus-sdk for proving
│   └── src/
│       └── main.rs      # Prover and verifier logic
├── Cargo.toml           # Workspace configuration
└── rust-toolchain.toml  # Nightly Rust toolchain (required by Nexus)
```

## Prerequisites

1. Rust toolchain (nightly-2025-04-06, automatically configured)
2. RISC-V target support (installed automatically)
3. Internet connection (for first build only)

## How to Run

### Generate and Verify Zero-Knowledge Proofs

```bash
cd ceno-zkvm
RUST_LOG=info cargo run --release --bin ceno-host
```

This will:
1. **Compile** the guest program to RISC-V
2. **Execute** the program in the zkVM
3. **Generate** a real zero-knowledge proof
4. **Verify** the proof cryptographically
5. Display detailed statistics and logs

### Expected Output

```
=== CENO zkVM Fibonacci Demo (Real Proof Generation) ===

📊 Configuration:
   Input: n = 5
   Expected result: fib(5) = 8

🔨 Step 1: Compiling guest program...
✅ Compilation completed in 1.00s
   ELF instructions: 2304

🔐 Step 2: Generating zero-knowledge proof...
   This process may take several minutes for real ZK proof generation.
✅ Proof generated successfully!
   Proving time: 0.27s
   Proof size: 50576 bytes

📝 Step 3: Execution logs:
-------------------
=== CENO zkVM Guest Program ===
Computing Fibonacci for n = 5
Result: fib(5) = 8
=== Computation Complete ===
-------------------
✅ Guest program executed successfully (Exit code: 0)

🔍 Step 4: Verifying zero-knowledge proof...
✅ Proof verified successfully!
   Verification time: 0.00s

============================================================
📊 Summary:
============================================================
  Input:             n = 5
  Result:            fib(5) = 8
  Compilation time:  1.00s
  Proving time:      0.27s
  Verification time: 0.00s
  Total time:        1.27s
  Proof size:        50576 bytes
============================================================

✨ CENO zkVM demo completed successfully!
```

## Configuration

The Fibonacci input value can be configured via the `.env` file in the workspace root:

```bash
FIBONACCI_N=10  # Compute the 10th Fibonacci number
```

## What This Demo Shows

This demonstration computes the nth Fibonacci number using **real zero-knowledge proofs**:

1. **Guest Program**: Runs the Fibonacci calculation inside the zkVM (RISC-V)
2. **Proof Generation**: Creates a cryptographic proof of correct execution
3. **Proof Verification**: Verifies the proof without re-executing the program
4. **Zero-Knowledge Properties**: The verifier learns only the result, not the intermediate steps

## Performance Notes

- **First build**: Takes 2-3 minutes (downloads and compiles dependencies)
- **Subsequent runs**: Compilation ~1s, Proving ~0.3s (for n=5)
- **Proof size**: ~50KB for this simple program
- **Proving time scales**: Roughly linearly with computation complexity

## Technical Details

### Why Nexus zkVM?

While awaiting the official CENO SDK, we use Nexus zkVM because:

- ✅ Production-ready and battle-tested
- ✅ Real zero-knowledge proof generation
- ✅ RISC-V based (similar to CENO's expected architecture)
- ✅ Active development and good documentation
- ✅ Similar API patterns to other zkVMs

### How It Works

1. **Compilation**: Guest code is compiled to RISC-V bytecode
2. **Execution**: The VM executes the bytecode and records the trace
3. **Proof Generation**: A STARK/SNARK proof is generated from the execution trace
4. **Verification**: The proof is verified using the public inputs and program hash

## Migration Path

Once Scroll releases the official CENO SDK, the migration will involve:

1. Update `Cargo.toml` dependencies (replace `nexus-sdk` with `ceno-sdk`)
2. Update guest code to use CENO's input/output APIs
3. Update host code to use CENO's prover/verifier APIs
4. Update `rust-toolchain.toml` to CENO's requirements
5. Test with CENO's toolchain

The core logic (Fibonacci computation) will remain unchanged.

## Troubleshooting

### Build fails with network errors

```bash
# Clear cargo cache and retry
rm -rf ~/.cargo/git/checkouts/nexus-zkvm-*
cargo clean
cargo run --release --bin ceno-host
```

### Compilation is slow

- This is normal for the first build
- Subsequent builds are much faster
- Use `--release` for optimal performance

### Out of memory during proving

```bash
# Reduce the Fibonacci input in .env
echo "FIBONACCI_N=3" > ../.env
```

## Contributing

Once CENO SDK is released, contributions are welcome to:
- Update to official CENO implementation
- Optimize performance
- Add more complex examples
- Improve documentation

## License

MIT OR Apache-2.0

## Acknowledgments

- **Scroll Team**: For developing CENO zkVM
- **Nexus Team**: For providing the zkVM implementation used in this proof-of-concept
- **zkVM Community**: For advancing zero-knowledge technology
