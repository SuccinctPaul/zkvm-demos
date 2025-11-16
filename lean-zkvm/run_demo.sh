#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Lean zkVM Fibonacci Demo Runner                 ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo

# Get Fibonacci input from command line or environment
FIBONACCI_N=${1:-${FIBONACCI_N:-10}}

echo -e "${GREEN}Configuration:${NC}"
echo "  • Fibonacci input: n = $FIBONACCI_N"
echo "  • Build mode: Release (optimized)"
echo "  • Compiler flags: -C target-cpu=native"
echo

# Navigate to host directory
cd "$(dirname "$0")/lean-host"

echo -e "${YELLOW}Building lean-host...${NC}"
RUSTFLAGS='-C target-cpu=native' cargo build --release

echo
echo -e "${GREEN}Running Lean zkVM demo...${NC}"
echo

# Run the demo with optimizations
FIBONACCI_N=$FIBONACCI_N RUSTFLAGS='-C target-cpu=native' RUST_LOG=info cargo run --release

echo
echo -e "${GREEN}✅ Demo completed successfully!${NC}"

