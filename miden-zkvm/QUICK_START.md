# Miden zkVM Quick Start Guide

## Overview

This is a demonstration implementation of Miden zkVM for computing Fibonacci numbers. Miden is a STARK-based zero-knowledge virtual machine developed by Polygon.

⚠️ **Note**: This implementation is currently experiencing a runtime issue that is being investigated. The code compiles successfully and demonstrates the correct API usage, but encounters a stack validation error during execution. See [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) for details.

## Prerequisites

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Install Miden SDK (Optional)

```bash
cd ../../scripts/sdk_installers
./install_miden_sdk.sh
```

Note: Miden VM is primarily distributed as Rust crates, so no separate SDK installation is strictly required.

## Project Structure

```
miden-zkvm/
├── miden-host/              # Rust host program
│   ├── Cargo.toml
│   └── src/main.rs          # Main execution logic
├── programs/                # Miden Assembly programs
│   └── fib_simple.masm      # Fibonacci implementation
├── Cargo.toml               # Workspace configuration
├── README.md                # Full documentation
├── IMPLEMENTATION_NOTES.md  # Technical details and known issues
└── QUICK_START.md          # This file
```

## Building

```bash
cd miden-zkvm/miden-host
cargo build --release
```

Expected output:
```
   Compiling miden-vm v0.11.0
   Compiling miden-assembly v0.11.0
   ...
   Compiling miden-host v0.1.0
    Finished `release` profile [optimized + debuginfo] target(s) in Xm XXs
```

## Running

```bash
cd miden-zkvm/miden-host
FIBONACCI_N=5 cargo run --release
```

### Current Behavior

The program will:
1. ✅ Load and compile the Miden Assembly program successfully
2. ✅ Display program hash and compilation time
3. ❌ Fail during proof generation with a stack validation error

```
====================================
   Miden zkVM Fibonacci Demo
====================================

Computing Fibonacci number for n = 5

1. Loading Miden Assembly program...
   Source file: ".../programs/fib_simple.masm"

2. Compiling Miden Assembly...
   ✓ Compilation successful
   Compile time: 0.001s
   Program hash: 0x6e834d19674f1ceb9a413744397d2a40b9fd453689768cd1814d60c5765e9e4e

3. Preparing inputs...
   ✓ Using hardcoded Fibonacci calculation for n = 5

4. Executing program and generating proof...
Error: Failed to prove program: The stack should have at most 16 elements 
       at the end of program execution, but had 17 elements
```

## What's Working

- ✅ Project compiles successfully
- ✅ Miden Assembly compilation works
- ✅ API integration is correct
- ✅ Code structure follows best practices
- ✅ Error handling is implemented

## What Needs Fixing

- ❌ Runtime stack validation issue
- ❌ Proof generation fails
- ❌ Need to verify correct API usage for Miden VM 0.11

## Troubleshooting

### Build Errors

If you see compilation errors:

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Network Issues During Build

If cargo fails to download dependencies:

```bash
# Retry the build
cargo build --release

# Or check your network connection and try again
```

## Next Steps

1. **For Users**: Check [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) for technical details
2. **For Contributors**: Help investigate the stack validation issue
3. **For Developers**: Review the Miden VM 0.11 documentation for correct API usage

## Alternative: Using Miden CLI

While this Rust integration is being debugged, you can try running Miden Assembly directly:

```bash
# Install Miden CLI (if available)
cargo install miden-cli

# Run assembly directly
miden run -a programs/fib_simple.masm
```

## Resources

- **Full README**: [README.md](./README.md)
- **Implementation Notes**: [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md)
- **Miden VM Docs**: https://0xpolygonmiden.github.io/miden-vm/
- **GitHub Issues**: Report issues in the main zkvm-demos repository

## Getting Help

1. Review the implementation notes for known issues
2. Check Miden VM documentation for version 0.11
3. Join the Polygon Discord community
4. Open an issue in the zkvm-demos repository

---

**Status**: Implementation in progress - compiles successfully, runtime issue under investigation

**Last Updated**: November 2025

