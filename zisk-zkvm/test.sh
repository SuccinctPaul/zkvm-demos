#!/bin/bash
# ZisK zkVM Test Script
# This script demonstrates the complete workflow for ZisK

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================="
echo "ZisK zkVM Multi-Program Demo"
echo "========================================="

# Configuration
PROGRAM_ID=${PROGRAM_ID:-0} # Default to 0 (Fibonacci)
INPUT_N=${INPUT_N:-10}

echo ""
echo "Configuration:"
echo "  PROGRAM_ID = $PROGRAM_ID"
echo "  INPUT_N = $INPUT_N"
echo ""

# Generate input binary (8 bytes: program_id (u32) + n (u32))
echo "Generating input.bin..."
mkdir -p build
python3 -c "import sys, struct; sys.stdout.buffer.write(struct.pack('<II', $PROGRAM_ID, $INPUT_N))" > build/input.bin
echo "✓ Input generated"

# Step 1: Build the guest program
echo "Step 1: Building guest program with cargo-zisk..."
cd zisk-guest
cargo-zisk build --release
echo "✓ Build completed"
echo ""

# Step 2: Run with emulator
echo "Step 2: Running with ziskemu..."
cd ..
ziskemu -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest -i build/input.bin
echo "✓ Execution completed"
echo ""

# Summary
echo "========================================="
echo "Test passed! ✓"
echo "========================================="
