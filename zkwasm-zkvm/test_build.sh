#!/bin/bash

# zkWasm Build Test Script
# This script tests the build process without running the full proof generation

set -e

echo "======================================"
echo "  zkWasm Build Test"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Rust not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust found: $(rustc --version)${NC}"

# Check wasm32 target
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo -e "${BLUE}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi
echo -e "${GREEN}✓ wasm32-unknown-unknown target installed${NC}"
echo ""

# Build guest
echo -e "${BLUE}[1/3] Building WASM guest program...${NC}"
cargo build --release --target wasm32-unknown-unknown --manifest-path zkwasm-guest/Cargo.toml
echo -e "${GREEN}✓ Guest build successful${NC}"
echo ""

# Build host
echo -e "${BLUE}[2/3] Building host program...${NC}"
cargo build --release --manifest-path zkwasm-host/Cargo.toml
echo -e "${GREEN}✓ Host build successful${NC}"
echo ""

# Check WASM file
echo -e "${BLUE}[3/3] Checking WASM output...${NC}"
WASM_FILE="target/wasm32-unknown-unknown/release/zkwasm_guest.wasm"
if [ -f "$WASM_FILE" ]; then
    SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo -e "${GREEN}✓ WASM file generated: $WASM_FILE ($SIZE)${NC}"
else
    echo -e "${RED}✗ WASM file not found${NC}"
    exit 1
fi
echo ""

# Summary
echo "======================================"
echo -e "${GREEN}Build test completed successfully!${NC}"
echo "======================================"
echo ""
echo "Next steps:"
echo "  1. Install zkWasm CLI: cd scripts/sdk_installers && ./install_zkwasm_sdk.sh"
echo "  2. Run full demo: ./run_demo.sh"
echo "  3. Or use host directly: cd zkwasm-host && cargo run -- run"

