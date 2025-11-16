# Jolt zkVM Demo

This demo showcases how to build, prove, and verify programs using [Jolt zkVM](https://github.com/a16z/jolt), a zkVM designed by a16z crypto.


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
RUST_LOG=info FIBONACCI_N=10 cargo run --release
```

## Resources

- **Jolt Repository**: https://github.com/a16z/jolt
- **Documentation**: https://jolt.a16zcrypto.com/
- **Paper**: [Jolt: SNARKs for Virtual Machines via Lookups](https://eprint.iacr.org/2023/1217)
- **Blog Post**: [Introducing Jolt](https://a16zcrypto.com/posts/article/introducing-jolt/)
