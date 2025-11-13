#!/bin/bash
# ZisK zkVM Test Script
# This script demonstrates the complete workflow for ZisK

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================="
echo "ZisK zkVM Fibonacci Demo Test Script"
echo "========================================="

# Configuration
export FIBONACCI_N=${FIBONACCI_N:-10}
echo ""
echo "Configuration:"
echo "  FIBONACCI_N = $FIBONACCI_N"
echo ""

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

# Step 3: Alternative - use cargo-zisk run
echo "Step 3: Testing cargo-zisk run..."
cd zisk-guest
cargo-zisk run --release -i ../build/input.bin
echo "✓ cargo-zisk run completed"
echo ""

# Summary
echo "========================================="
echo "All tests passed! ✓"
echo "========================================="
echo ""
echo "Next steps (Linux only - macOS not supported):"
echo "  1. Generate ROM setup:"
echo "     cd zisk-guest"
echo "     cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest"
echo ""
echo "  2. Generate and verify proof:"
echo "     cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \\"
echo "                      -i ../build/input.bin -o ../proof -a -y"
echo ""
echo "  3. Verify proof:"
echo "     cargo-zisk verify -p ../proof/vadcop_final_proof.bin"
echo ""

