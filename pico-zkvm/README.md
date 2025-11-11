# Pico zkVM Fibonacci Demo

This is a demonstration of using [Brevis Pico zkVM](https://github.com/brevis-network/pico) to compute Fibonacci numbers with zero-knowledge proofs.

## About Pico zkVM

Pico is a high-performance, modular, general-purpose zkVM developed by Brevis. It provides:
- RISC-V instruction set compatibility
- Rust development toolchain support
- Modular architecture for flexible proof backend integration
- Real-time Ethereum proofs capability

## Project Structure

```
pico-zkvm/
├── pico-guest/          # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Fibonacci computation logic
├── pico-host/           # Host program (manages proving/verification)
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

2. Install Pico zkVM toolchain:
   ```bash
   cd scripts/sdk_installers
   ./install_pico_sdk.sh
   ```

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
cd pico-zkvm/pico-host
cargo build --release
```

## Running

### Execute the program and generate proof:

```bash
cd pico-zkvm/pico-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Build and load the guest program
3. Execute the program in the zkVM
4. Generate a zero-knowledge proof
5. Verify the proof
6. Display execution statistics (cycles, proof size, timing)

## Expected Output

```
fib_n = 10

1. Initializing Pico zkVM prover...
   Initialization completed in X.XXs

2. Building guest program...
   Build completed in X.XXs
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

## Resources

- [Pico zkVM GitHub Repository](https://github.com/brevis-network/pico)
- [Brevis Official Website](https://brevis.network/)
- [Pico Documentation](https://docs.brevis.network/)

## Notes

- This demo uses the recursive Fibonacci implementation from the shared `fib` library
- The guest program runs in a `no_std` environment
- Proof generation time depends on the input size and system performance
- For production use, consider iterative Fibonacci implementation for better performance

