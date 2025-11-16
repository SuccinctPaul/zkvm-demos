#!/bin/bash

# Novanet zkVM Demo Runner Script
# This script runs the Novanet zkVM demonstration

set -e  # Exit on error

echo "=========================================="
echo "Novanet zkVM Demo Runner"
echo "=========================================="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed.${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓ Rust toolchain found${NC}"
rustc --version
cargo --version
echo ""

# Set default Fibonacci number if not provided
if [ -z "$FIBONACCI_N" ]; then
    FIBONACCI_N=10
    echo -e "${YELLOW}No FIBONACCI_N set, using default: ${FIBONACCI_N}${NC}"
else
    echo -e "${GREEN}Using FIBONACCI_N: ${FIBONACCI_N}${NC}"
fi
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Parse command line arguments
MODE="run"
PROFILE="release"

while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            PROFILE="debug"
            shift
            ;;
        --test)
            MODE="test"
            shift
            ;;
        --build)
            MODE="build"
            shift
            ;;
        --clean)
            MODE="clean"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --debug      Build and run in debug mode (default: release)"
            echo "  --test       Run tests instead of the demo"
            echo "  --build      Only build, don't run"
            echo "  --clean      Clean build artifacts"
            echo "  --help, -h   Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  FIBONACCI_N  The Fibonacci number to compute (default: 10)"
            echo ""
            echo "Examples:"
            echo "  ./run_demo.sh"
            echo "  ./run_demo.sh --debug"
            echo "  ./run_demo.sh --test"
            echo "  FIBONACCI_N=15 ./run_demo.sh"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Execute based on mode
case $MODE in
    clean)
        echo -e "${YELLOW}Cleaning build artifacts...${NC}"
        cargo clean
        echo -e "${GREEN}✓ Clean complete${NC}"
        ;;
    
    build)
        echo -e "${YELLOW}Building Novanet zkVM demo (${PROFILE} mode)...${NC}"
        if [ "$PROFILE" = "release" ]; then
            cargo build --release
        else
            cargo build
        fi
        echo -e "${GREEN}✓ Build complete${NC}"
        ;;
    
    test)
        echo -e "${YELLOW}Running tests...${NC}"
        cargo test
        echo -e "${GREEN}✓ Tests complete${NC}"
        ;;
    
    run)
        echo -e "${YELLOW}Building and running Novanet zkVM demo (${PROFILE} mode)...${NC}"
        echo ""
        
        if [ "$PROFILE" = "release" ]; then
            FIBONACCI_N=$FIBONACCI_N cargo run --release -p novanet-host
        else
            FIBONACCI_N=$FIBONACCI_N cargo run -p novanet-host
        fi
        
        echo ""
        echo -e "${GREEN}=========================================="
        echo "Demo execution completed!"
        echo -e "==========================================${NC}"
        ;;
esac

echo ""
echo -e "${GREEN}Done!${NC}"

