# Nexus zkVM Fibonacci Demo

This is a demonstration of using [Nexus zkVM](https://github.com/nexus-xyz/nexus-zkvm) to compute Fibonacci numbers with zero-knowledge proofs.

## About Nexus zkVM

Nexus is a modular zkVM that provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular prover architecture with pluggable backends
- Support for various proof systems (STARK, SNARK)
- Efficient proof generation and verification
- Cycle counting and profiling capabilities

## Project Structure

```
nexus-zkvm/
├── nexus-guest/        # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # Fibonacci computation logic
├── nexus-host/         # Host program (manages proving/verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs     # Prover and verifier logic
├── Cargo.toml          # Workspace configuration
└── rust-toolchain.toml # Rust toolchain specification
```

## Prerequisites

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install Nexus zkVM toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_nexus_sdk.sh
   ```

   This script will:
   - Install `cargo-nexus` CLI tool (version 0.3.4)
   - Install the Rust nightly toolchain (nightly-2025-04-06)
   - Add RISC-V target (`riscv32i-unknown-none-elf`)

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
cd zkvms/nexus-zkvm/nexus-host
cargo build --release
```

The build process will:
1. Compile the guest program for RISC-V target
2. Compile the host program with Nexus SDK dependencies
3. Apply optimizations (LTO enabled in release mode)

## Running

### Generate and verify proof:

```bash
cd zkvms/nexus-zkvm/nexus-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Compile the guest program
3. Execute the program in the zkVM and generate a proof
4. Display execution logs and cycle count
5. Verify the proof
6. Display execution statistics (prove time, proof size)

## Expected Output

```
fib_n = 10

1. Compiling guest program...
====== Compile Cost: X.XXs

ELF: instructions num: XXXX

Proving execution of vm...
Prove cost: X.XXX s, proof size: XXXX Bytes

3. Execution Logs:
-------------------
View: view_tracked_ram_size: ..., view_associated_data: ..., view_debug_logs: ...
Read public input:  10
fib result:  89
-------------------

4. Execution completed successfully (Exit code: 0)

Verifying execution...  Succeeded!
```

## Features

- **Modular Architecture**: Support for different proof backends (Stwo, Jolt, etc.)
- **Profiling Support**: Built-in profiling with `#[profile]` macro for performance analysis
- **Public Inputs**: Support for public input/output data
- **Debug Logging**: Comprehensive logging of execution details
- **Cycle Tracking**: Optional cycle counting for performance analysis

## Profiling

The host program uses the `#[profile]` macro to generate profiling data:

```bash
# Run to generate profiling data
cargo run --release

# View the profile with pprof (requires Go)
go tool pprof -http=127.0.0.1:8000 main.pb
```

This will open a web interface showing:
- Function call graphs
- Execution time breakdown
- Memory allocation patterns

## Resources

- [Nexus zkVM Documentation](https://docs.nexus.xyz/)
- [Nexus zkVM GitHub Repository](https://github.com/nexus-xyz/nexus-zkvm)
- [Nexus Blog](https://blog.nexus.xyz/)
- [Nexus SDK Macros Documentation](https://github.com/nexus-xyz/nexus-zkvm/blob/releases/0.3.4/sdk/macros/README.md)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on the input size and system performance
- For production use, consider iterative Fibonacci implementation for better performance
- The workspace requires both guest and host to be in the same workspace for proper compilation
- Dev builds are optimized by default (`opt-level = 3`) for reasonable performance during development

## Version Information

- Nexus SDK: 0.3.4
- Rust Toolchain: nightly-2025-04-06
- Rust Edition: 2021 (host), 2024 (guest)
- Minimum Rust Version: 1.85

## Troubleshooting

### Compilation Errors

If you encounter compilation errors, ensure:
- The correct Rust toolchain is installed: `rustup toolchain list`
- The RISC-V target is added: `rustup target list --installed | grep riscv32i`
- All dependencies are up to date: `cargo update`

### Runtime Errors

For runtime issues:
- Check that environment variables are set correctly
- Ensure sufficient memory is available for proof generation
- Enable debug logging: `RUST_LOG=debug cargo run --release`

## License

MIT OR Apache-2.0

