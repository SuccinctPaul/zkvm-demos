#!/bin/bash
set -e

# Jolt zkVM Demo Runner Script
# This script helps you run the Jolt zkVM fibonacci demo

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Jolt zkVM Demo Runner${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if .env exists, if not create from example
if [ ! -f .env ]; then
    echo -e "${YELLOW}Creating .env file from template...${NC}"
    if [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${GREEN}✓ Created .env file${NC}"
    else
        echo "FIBONACCI_N=10" > .env
        echo -e "${GREEN}✓ Created default .env file${NC}"
    fi
    echo ""
fi

# Source .env file
if [ -f .env ]; then
    source .env
    echo -e "${GREEN}Loaded configuration from .env${NC}"
    echo -e "  FIBONACCI_N = ${FIBONACCI_N}"
else
    echo -e "${YELLOW}No .env file found, using default FIBONACCI_N=10${NC}"
    export FIBONACCI_N=10
fi
echo ""

# Check if jolt is installed
echo -e "${BLUE}Checking Jolt installation...${NC}"
if command -v jolt &> /dev/null; then
    JOLT_VERSION=$(jolt --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓ Jolt is installed: ${JOLT_VERSION}${NC}"
else
    echo -e "${RED}✗ Jolt is not installed${NC}"
    echo ""
    echo -e "${YELLOW}To install Jolt, run:${NC}"
    echo -e "  cd ../scripts/sdk_installers"
    echo -e "  ./install_jolt_sdk.sh"
    echo ""
    echo -e "Or manually install:"
    echo -e "  cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt"
    echo -e "  jolt install-toolchain"
    exit 1
fi
echo ""

# Parse command line arguments
MODE="${1:-run}"

case "$MODE" in
    "run"|"")
        echo -e "${BLUE}Running Jolt zkVM demo...${NC}"
        echo ""
        cd jolt-host
        RUST_LOG=${RUST_LOG:-info} cargo run --release
        ;;
    
    "build")
        echo -e "${BLUE}Building Jolt zkVM demo...${NC}"
        echo ""
        cd jolt-host
        cargo build --release
        echo ""
        echo -e "${GREEN}✓ Build completed successfully${NC}"
        ;;
    
    "clean")
        echo -e "${BLUE}Cleaning build artifacts...${NC}"
        echo ""
        cargo clean
        echo -e "${GREEN}✓ Clean completed${NC}"
        ;;
    
    "test")
        echo -e "${BLUE}Running tests...${NC}"
        echo ""
        cd jolt-host
        cargo test --release
        ;;
    
    "help")
        echo "Usage: ./run_demo.sh [COMMAND]"
        echo ""
        echo "Commands:"
        echo "  run     - Run the demo (default)"
        echo "  build   - Build the project"
        echo "  clean   - Clean build artifacts"
        echo "  test    - Run tests"
        echo "  help    - Show this help message"
        echo ""
        echo "Environment variables:"
        echo "  FIBONACCI_N  - The fibonacci number to compute (default: 10)"
        echo "  RUST_LOG     - Logging level (default: info)"
        echo ""
        echo "Examples:"
        echo "  ./run_demo.sh                    # Run with default settings"
        echo "  FIBONACCI_N=15 ./run_demo.sh     # Run with n=15"
        echo "  RUST_LOG=debug ./run_demo.sh     # Run with debug logging"
        ;;
    
    *)
        echo -e "${RED}Unknown command: $MODE${NC}"
        echo "Run './run_demo.sh help' for usage information"
        exit 1
        ;;
esac

