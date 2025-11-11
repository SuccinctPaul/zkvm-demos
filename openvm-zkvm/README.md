# OpenVM zkVM Fibonacci Demo

This is a demonstration of using [OpenVM](https://github.com/openvm-org/openvm) to compute Fibonacci numbers with zero-knowledge proofs.

## About OpenVM

OpenVM is a high-performance, modular zkVM that provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular architecture with customizable proving backends
- Optimized performance for various proof types
- Flexible configuration options

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
   ./install_openvm_sdk.sh
   ```

   This script will:
   - Install `cargo-openvm` CLI tool
   - Setup aggregation keys
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

## Running

### Execute the program and generate proof:

```bash
cd openvm-zkvm/openvm-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Initialize the OpenVM prover
3. Load and execute the guest program in the zkVM
4. Generate a zero-knowledge proof
5. Verify the proof
6. Display execution statistics (cycles, proof size, timing)

## Expected Output

```
fib_n = 10

1. Initializing OpenVM prover...
   Initialization completed in X.XXs

2. Loading guest program...
   Guest program loaded in X.XXs
   ELF size: XXXX bytes

3. Executing program in zkVM...
   Execution completed in X.XXs
   Cycle count: XXXX
   Fibonacci(10) = 89

4. Generating zero-knowledge proof...
   Proof generation completed in X.XXs
   Proof size: XXXX bytes

5. Verifying proof...
   Verification completed in X.XXs
   ✓ Proof verified successfully!

============ Summary ============
Input: n = 10
Output: fibonacci(10) = 89
Total cycles: XXXX
Proof size: XXXX bytes
Prove time: X.XXs
=================================
```

## Features

- **Modular Design**: OpenVM's modular architecture allows for flexible configuration
- **Performance**: Optimized execution and proving times
- **RISC-V Compatibility**: Supports standard RISC-V instruction set
- **Rust Integration**: Seamless Rust development experience

## Resources

- [OpenVM Documentation](https://docs.openvm.dev/)
- [OpenVM GitHub Repository](https://github.com/openvm-org/openvm)
- [OpenVM Blog](https://blog.openvm.dev/)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on the input size and system performance
- For production use, consider iterative Fibonacci implementation for better performance
- OpenVM provides various configuration options for optimizing proof generation

