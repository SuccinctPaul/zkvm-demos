#!/bin/bash

# zkWasm Demo Runner Script
# This script demonstrates the complete workflow of zkWasm proving system

set -e  # Exit on error

echo "=================================="
echo "    zkWasm Fibonacci Demo"
echo "=================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if zkwasm-cli is installed
if ! command -v delphinus-cli &> /dev/null; then
    echo -e "${RED}Error: delphinus-cli not found${NC}"
    echo ""
    echo "Please install zkWasm CLI first:"
    echo "  cd scripts/sdk_installers"
    echo "  ./install_zkwasm_sdk.sh"
    echo ""
    echo "Or install manually:"
    echo "  git clone --recurse-submodules https://github.com/DelphinusLab/zkwasm"
    echo "  cd zkwasm"
    echo "  cargo build --release"
    echo "  export PATH=\$PATH:\$(pwd)/target/release"
    exit 1
fi

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo -e "${BLUE}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Configuration
INPUT_N=${1:-10}  # Default to Fibonacci(10)
CIRCUIT_K=${2:-18}  # Default circuit size

echo -e "${BLUE}Configuration:${NC}"
echo "  Input: Fibonacci($INPUT_N)"
echo "  Circuit size (k): $CIRCUIT_K"
echo ""

# Step 1: Build WASM
echo -e "${BLUE}[1/4] Building WASM guest program...${NC}"
cd zkwasm-host
cargo run --release -- build
cd ..
echo -e "${GREEN}✓ WASM built successfully${NC}"
echo ""

# Step 2: Setup Circuit
echo -e "${BLUE}[2/4] Setting up zkWasm circuit...${NC}"
cd zkwasm-host
cargo run --release -- setup --k $CIRCUIT_K
cd ..
echo -e "${GREEN}✓ Circuit setup complete${NC}"
echo ""

# Step 3: Generate Proof
echo -e "${BLUE}[3/4] Generating zero-knowledge proof...${NC}"
echo "  (This may take a few minutes)"
cd zkwasm-host
cargo run --release -- prove --n $INPUT_N
cd ..
echo -e "${GREEN}✓ Proof generated${NC}"
echo ""

# Step 4: Verify Proof
echo -e "${BLUE}[4/4] Verifying proof...${NC}"
cd zkwasm-host
cargo run --release -- verify
cd ..
echo -e "${GREEN}✓ Proof verified${NC}"
echo ""

# Summary
echo "=================================="
echo -e "${GREEN}Demo completed successfully!${NC}"
echo "=================================="
echo ""
echo "What happened:"
echo "  1. Compiled Rust code to WebAssembly"
echo "  2. Setup zkSNARK circuit for WASM execution"
echo "  3. Generated a proof for Fibonacci($INPUT_N)"
echo "  4. Verified the proof cryptographically"
echo ""
echo "Output files:"
echo "  - output/guest.wasm   (WASM binary)"
echo "  - output/proof.json   (Zero-knowledge proof)"
echo "  - params/             (Circuit parameters)"
echo ""
echo "Try with different values:"
echo "  ./run_demo.sh 20      (Compute Fibonacci(20))"
echo "  ./run_demo.sh 30 20   (Compute Fibonacci(30) with k=20)"

