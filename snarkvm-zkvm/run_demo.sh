#!/bin/bash

# snarkVM Demo Runner
# This script builds and runs the snarkVM fibonacci demo

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}snarkVM Demo Runner${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo -e "${GREEN}✓${NC} Found Rust version: $RUST_VERSION"

# Set default fibonacci number if not set
if [ -z "$FIBONACCI_N" ]; then
    export FIBONACCI_N=10
    echo -e "${YELLOW}ℹ${NC} Using default FIBONACCI_N=$FIBONACCI_N"
else
    echo -e "${GREEN}✓${NC} Using FIBONACCI_N=$FIBONACCI_N"
fi

# Navigate to host directory
cd "$(dirname "$0")/snarkvm-host"

# Build the project
echo ""
echo -e "${BLUE}Building snarkVM demo...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi

# Run the demo
echo ""
echo -e "${BLUE}Running snarkVM demo...${NC}"
echo ""

cargo run --release

EXIT_CODE=$?

echo ""
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✓${NC} Demo completed successfully"
else
    echo -e "${RED}✗${NC} Demo failed with exit code $EXIT_CODE"
    exit $EXIT_CODE
fi

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${YELLOW}💡 Tips:${NC}"
echo -e "  - Try different values: ${GREEN}FIBONACCI_N=15 ./run_demo.sh${NC}"
echo -e "  - View source: ${GREEN}snarkvm-host/src/main.rs${NC}"
echo -e "  - Check Aleo program: ${GREEN}programs/fibonacci.aleo${NC}"
echo -e "${BLUE}========================================${NC}"

