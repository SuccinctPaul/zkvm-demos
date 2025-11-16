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

### Note on Current Build Status

⚠️ **Important**: The Pico zkVM SDK (versions v1.1.6 and v1.1.7) currently has compatibility issues with certain Rust
nightly toolchains. The `cargo pico build` command may fail due to:

- Unstable feature incompatibilities
- Deprecated API usage in dependencies

**Current Workaround**: A pre-built ELF binary is provided in `pico-guest/elf/` for testing purposes.

### Building Guest Program (when toolchain issues are resolved)

```bash
cd pico-zkvm/pico-guest
cargo pico build
```

The built ELF will be placed in `target/riscv32im-pico-zkvm-elf/release/pico-guest`.

### Verifying Host Code

```bash
cd pico-zkvm
cargo check
cargo build --release
```

## Running

✅ **Verified Working**: Proof generation has been tested and verified.
See [PROOF_VERIFICATION.md](PROOF_VERIFICATION.md) for detailed test results.

### Prerequisites for Running

1. Set the Fibonacci number to compute:
   ```bash
   export FIBONACCI_N=10
   ```

2. Ensure a guest ELF binary exists in one of these locations:
    - `pico-guest/elf/riscv32im-pico-zkvm-elf` (pre-built ✅ included)
    - `pico-guest/target/riscv32im-pico-zkvm-elf/release/pico-guest` (from cargo pico build)

### Execute the program and generate proof:

```bash
cd pico-zkvm/pico-host
cargo run --release
```

### Known Issues

1. **Build Tool Compatibility**: The `cargo pico` build tool has compatibility issues with current Rust nightly versions
2. **Guest Program API**: Uses `pico-sdk` v1.1.6 which may not match all examples in official documentation
3. **Toolchain Requirements**: Requires specific nightly version `nightly-2025-08-04`
