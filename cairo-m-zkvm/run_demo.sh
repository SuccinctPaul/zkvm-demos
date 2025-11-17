#!/bin/bash
# Cairo-M zkVM Demo Runner Script

set -e

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "========================================="
echo "Cairo-M zkVM - Fibonacci Demo"
echo "========================================="
echo ""

# Check if FIBONACCI_N is set
if [ -z "$FIBONACCI_N" ]; then
    FIBONACCI_N=10
    echo -e "${YELLOW}FIBONACCI_N not set, using default: $FIBONACCI_N${NC}"
else
    echo "FIBONACCI_N: $FIBONACCI_N"
fi
echo ""

# Check if cairo-m tools are installed
echo "Checking Cairo-M toolchain..."

MISSING_TOOLS=0

if ! command -v cairo-m-compiler &> /dev/null; then
    echo -e "${RED}✗${NC} cairo-m-compiler not found"
    MISSING_TOOLS=1
else
    echo -e "${GREEN}✓${NC} cairo-m-compiler found"
fi

if ! command -v cairo-m-runner &> /dev/null; then
    echo -e "${RED}✗${NC} cairo-m-runner not found"
    MISSING_TOOLS=1
else
    echo -e "${GREEN}✓${NC} cairo-m-runner found"
fi

if ! command -v cairo-m-prover &> /dev/null; then
    echo -e "${RED}✗${NC} cairo-m-prover not found"
    MISSING_TOOLS=1
else
    echo -e "${GREEN}✓${NC} cairo-m-prover found"
fi

if [ $MISSING_TOOLS -eq 1 ]; then
    echo ""
    echo -e "${YELLOW}Some Cairo-M tools are missing.${NC}"
    echo "The demo will run in simulation mode using Rust implementation."
    echo ""
    echo "To install Cairo-M tools, run:"
    echo "  cd ../scripts/sdk_installers"
    echo "  ./install_cairo_m_sdk.sh"
    echo ""
    echo "Continuing with simulation mode..."
    echo ""
fi

# Navigate to host directory
cd "$(dirname "$0")/cairo-m-host"

# Run the demo
echo "Running Cairo-M zkVM demo..."
echo ""

export RUST_LOG=info
export FIBONACCI_N=$FIBONACCI_N

cargo run --release

echo ""
echo -e "${GREEN}Demo completed!${NC}"
echo ""
echo "To try different input values:"
echo "  FIBONACCI_N=20 ./run_demo.sh"
echo ""
echo "To see detailed logs:"
echo "  RUST_LOG=debug FIBONACCI_N=10 ./run_demo.sh"

