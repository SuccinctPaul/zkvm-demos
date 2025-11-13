# Jolt zkVM Demo

This demo showcases how to build, prove, and verify programs using [Jolt zkVM](https://github.com/a16z/jolt), a zkVM designed by a16z crypto.

## Overview

Jolt is a SNARK-based zkVM that:
- Provides fast proof generation using lookup arguments
- Supports standard Rust programs
- Uses the Lasso/Jolt proving system
- Optimized for developer experience

## Project Structure

```
jolt-zkvm/
├── jolt-guest/         # Guest program (runs in zkVM)
│   ├── src/
│   │   └── lib.rs      # Fibonacci computation
│   └── Cargo.toml
├── jolt-host/          # Host program (proves and verifies)
│   ├── src/
│   │   └── main.rs     # Main proving/verification logic
│   ├── build.rs        # Build script for guest compilation
│   └── Cargo.toml
├── Cargo.toml          # Workspace configuration
├── rust-toolchain.toml # Rust toolchain specification
└── README.md           # This file
```

## Prerequisites

### Option 1: Using Installation Script (Recommended)

```bash
cd scripts/sdk_installers
./install_jolt_sdk.sh
```

This will:
- Install the Jolt CLI tool
- Set up the required Rust toolchain
- Verify the installation

### Option 2: Manual Installation

1. Install Rust nightly:
```bash
rustup install nightly
```

2. Install Jolt CLI:
```bash
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
```

3. Install Jolt's toolchain:
```bash
jolt install-toolchain
```

4. Verify installation:
```bash
jolt --version
```

## Environment Setup

Create a `.env` file in the project root or set the environment variable:

```bash
# Set the Fibonacci number to compute
export FIBONACCI_N=10
```

Or create a `.env` file:
```
FIBONACCI_N=10
```

## Building

Navigate to the host directory:

```bash
cd jolt-zkvm/jolt-host
```

Build the project (this will also build the guest program):

```bash
cargo build --release
```

## Running the Demo

### Quick Start

```bash
cd jolt-zkvm/jolt-host
FIBONACCI_N=10 cargo run --release
```

### With Logging

```bash
RUST_LOG=info FIBONACCI_N=10 cargo run --release
```

### Expected Output

```
========================================
Jolt zkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(10)...

1️⃣  Building guest program...
   ✓ Build completed in X.XXs

2️⃣  Generating proof...
   ✓ Proof generated in X.XXs
   ✓ Result: fibonacci(10) = 89

3️⃣  Verifying proof...
   ✓ Proof verified successfully in X.XXs

========================================
📈 Performance Summary
========================================
Build time:   X.XXs
Prove time:   X.XXs
Verify time:  X.XXs
Total time:   X.XXs
========================================
✅ Jolt zkVM Demo completed successfully!
```

## How It Works

### 1. Guest Program (`jolt-guest`)

The guest program contains the computation that will be proven:

```rust
#[jolt::provable]
pub fn fibonacci(n: u32) -> u32 {
    fib::fibonacci(n)
}
```

The `#[jolt::provable]` macro marks the function for proof generation.

### 2. Build Script (`jolt-host/build.rs`)

The build script compiles the guest program for the zkVM:

```rust
fn main() {
    jolt_sdk::build_guest("../jolt-guest");
}
```

### 3. Host Program (`jolt-host`)

The host program:
1. **Builds** the guest program for the zkVM
2. **Proves** execution with the input
3. **Verifies** the generated proof

```rust
let (prove_fibonacci, verify_fibonacci) = jolt_guest::build_fibonacci();
let (output, proof) = prove_fibonacci(fib_n);
let is_valid = verify_fibonacci(proof);
```

## Performance Characteristics

Jolt is designed for:
- **Fast proving**: Uses lookup arguments for efficiency
- **Small proof size**: Optimized proof representation
- **Quick verification**: Efficient verification algorithms

Typical performance (fibonacci(10)):
- Build time: ~5-10s (first time)
- Prove time: ~1-5s
- Verify time: <1s

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Ensure you're using the correct Rust toolchain:
```bash
rustup show
```

2. Clean and rebuild:
```bash
cargo clean
cargo build --release
```

3. Verify Jolt installation:
```bash
jolt --version
```

### Runtime Errors

If the demo fails at runtime:

1. Check that FIBONACCI_N is set:
```bash
echo $FIBONACCI_N
```

2. Try with a smaller value:
```bash
FIBONACCI_N=5 cargo run --release
```

3. Enable debug logging:
```bash
RUST_LOG=debug cargo run --release
```

## Resources

- **Jolt Repository**: https://github.com/a16z/jolt
- **Documentation**: https://jolt.a16zcrypto.com/
- **Paper**: [Jolt: SNARKs for Virtual Machines via Lookups](https://eprint.iacr.org/2023/1217)
- **Blog Post**: [Introducing Jolt](https://a16zcrypto.com/posts/article/introducing-jolt/)

## Key Features of Jolt

1. **Lookup-based proving**: Uses Lasso for efficient lookups
2. **Standard Rust**: Write normal Rust code, no special constraints
3. **Fast development**: Quick iteration cycle
4. **Optimized performance**: Designed for real-world applications

## Next Steps

- Modify the guest program to compute different functions
- Experiment with different input sizes
- Explore Jolt's advanced features
- Compare performance with other zkVMs

## Contributing

This demo is part of the zkvm-demos repository. Contributions are welcome!

## License

MIT OR Apache-2.0

