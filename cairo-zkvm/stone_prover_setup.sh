#!/bin/bash
# Stone Prover Setup and Proof Generation Script
# For Cairo 2.x Fibonacci Demo

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STONE_DIR="$HOME/.stone-prover"

echo "=========================================="
echo "Stone Prover Setup for Cairo 2.x"
echo "=========================================="
echo

# Step 1: Check dependencies
echo "Step 1: Checking dependencies..."
if ! command -v brew &> /dev/null; then
    echo "❌ Homebrew not found. Please install from https://brew.sh"
    exit 1
fi

# Install dependencies
echo "Installing dependencies..."
brew list cmake &>/dev/null || brew install cmake
brew list gmp &>/dev/null || brew install gmp
brew list boost &>/dev/null || brew install boost

echo "✓ Dependencies installed"
echo

# Step 2: Clone and build Stone Prover
echo "Step 2: Setting up Stone Prover..."
if [ ! -d "$STONE_DIR" ]; then
    echo "Cloning Stone Prover..."
    git clone https://github.com/starkware-libs/stone-prover.git "$STONE_DIR"
else
    echo "Stone Prover already cloned at $STONE_DIR"
fi

cd "$STONE_DIR"

# Build Stone Prover
if [ ! -f "build/src/starkware/main/cpu/cpu_air_prover" ]; then
    echo "Building Stone Prover (this may take 10-20 minutes)..."
    mkdir -p build
    cd build
    cmake ..
    make -j$(sysctl -n hw.ncpu)
    echo "✓ Stone Prover built successfully"
else
    echo "✓ Stone Prover already built"
fi

echo
echo "=========================================="
echo "Stone Prover Installation Complete!"
echo "=========================================="
echo
echo "Prover location: $STONE_DIR/build/src/starkware/main/cpu/cpu_air_prover"
echo "Verifier location: $STONE_DIR/build/src/starkware/main/cpu/cpu_air_verifier"
echo
echo "Next steps:"
echo "1. Convert Cairo 2.x Sierra to Cairo 0 format (if needed)"
echo "2. Run: ./generate_stone_proof.sh"
echo

