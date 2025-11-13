# Pico zkVM Fibonacci Demo

This is a demonstration of using [Brevis Pico zkVM](https://github.com/brevis-network/pico) to compute Fibonacci numbers
with zero-knowledge proofs.

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

   This script will:
    - Install the Pico SDK and toolchain
    - Install the required Rust toolchain
    - Setup necessary dependencies

    * Or install by cargo
   ```bash
   cargo +nightly-2025-08-04 install --git https://github.com/brevis-network/pico pico-cli
   ```

    * check
   ```bash
   cargo pico --version
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
cargo pico build
```

## Running

### Execute the program and generate proof:

```bash
cd pico-zkvm/pico-host
RUST_LOG=info cargo run --release
```
