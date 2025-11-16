#!/bin/bash

set -e

echo "========================================"
echo "o1vm zkVM Demo - Build and Run"
echo "========================================"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the o1vm-zkvm directory"
    exit 1
fi

# Step 1: Try to build MIPS guest program
echo "Step 1: Building MIPS guest program..."
echo ""

if command -v mips-linux-gnu-gcc &> /dev/null; then
    echo "✓ Found MIPS cross-compiler"
    cd o1vm-guest
    make clean
    make
    cd ..
    echo "✓ MIPS guest program compiled successfully"
    echo ""
else
    echo "⚠️  MIPS cross-compiler not found"
    echo ""
    echo "To install MIPS toolchain:"
    echo "  Ubuntu/Debian: sudo apt-get install gcc-mips-linux-gnu"
    echo "  macOS: Use Docker or cross-compilation toolchain"
    echo ""
    echo "Alternatively, use Docker:"
    echo "  docker run --rm -v \$(pwd):/work -w /work ubuntu:22.04 bash -c \\"
    echo "    'apt-get update && apt-get install -y gcc-mips-linux-gnu && \\"
    echo "     cd o1vm-guest && make'"
    echo ""
    echo "Continuing without guest binary compilation..."
    echo ""
fi

# Step 2: Build the host program
echo "Step 2: Building Rust host program..."
echo ""

cd o1vm-host

# Clean previous builds
cargo clean

# Build with release optimizations
echo "Building with release optimizations..."
cargo build --release

echo ""
echo "✓ Host program built successfully"
echo ""

# Step 3: Run the demo
echo "Step 3: Running the demo..."
echo ""

RUST_LOG=info cargo run --release

echo ""
echo "========================================"
echo "Demo completed!"
echo "========================================"

