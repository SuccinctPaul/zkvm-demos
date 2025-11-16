#!/bin/bash

# Novanet zkVM SDK Installation Script
# This script would install the Novanet zkVM SDK and dependencies

set -e  # Exit on error

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=========================================="
echo "Novanet zkVM SDK Installer"
echo -e "==========================================${NC}\n"

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

# Check Rust version
RUST_VERSION=$(rustc --version | grep -oE '[0-9]+\.[0-9]+' | head -1)
REQUIRED_VERSION="1.85"

echo -e "${YELLOW}Checking Rust version...${NC}"
if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" = "$REQUIRED_VERSION" ]; then 
    echo -e "${GREEN}✓ Rust version $RUST_VERSION meets requirement (>= $REQUIRED_VERSION)${NC}\n"
else
    echo -e "${RED}✗ Rust version $RUST_VERSION is too old (requires >= $REQUIRED_VERSION)${NC}"
    echo -e "${YELLOW}Updating Rust...${NC}"
    rustup update
    echo -e "${GREEN}✓ Rust updated${NC}\n"
fi

# Note about Novanet zkVM
echo -e "${YELLOW}=========================================="
echo "About Novanet zkVM SDK"
echo -e "==========================================${NC}"
echo ""
echo "Novanet zkVM is based on the Nova proof system, which provides:"
echo "  • Recursive SNARKs without trusted setup"
echo "  • Incrementally Verifiable Computation (IVC)"
echo "  • Efficient proof composition"
echo "  • Constant-size proofs"
echo ""
echo -e "${YELLOW}Note: This is a demonstration implementation.${NC}"
echo "A production Novanet zkVM would require:"
echo "  1. Integration with Nova proof system libraries"
echo "  2. Circuit compilation infrastructure"
echo "  3. Optimized proof generation and verification"
echo ""

# Installation steps
echo -e "${BLUE}=========================================="
echo "Installation Steps"
echo -e "==========================================${NC}\n"

# Step 1: Install Nova dependencies (if this were a real implementation)
echo -e "${YELLOW}1. Installing Nova dependencies...${NC}"
echo "   (In a production implementation, this would install:"
echo "    - nova-snark or similar Nova implementation"
echo "    - bellperson or arkworks for cryptographic primitives"
echo "    - RISC-V toolchain for guest program compilation)"
echo -e "${GREEN}   ✓ Skipped (demo implementation)${NC}\n"

# Step 2: Set up Rust toolchain
echo -e "${YELLOW}2. Setting up Rust toolchain...${NC}"
rustup toolchain install 1.85
rustup default 1.85
echo -e "${GREEN}   ✓ Rust 1.85 toolchain installed${NC}\n"

# Step 3: Install components
echo -e "${YELLOW}3. Installing Rust components...${NC}"
rustup component add rustfmt clippy
echo -e "${GREEN}   ✓ Components installed${NC}\n"

# Step 4: Verify installation
echo -e "${YELLOW}4. Verifying installation...${NC}"
echo "   Testing cargo build..."
cd "$(dirname "$0")/../../novanet-zkvm"
if cargo check --quiet; then
    echo -e "${GREEN}   ✓ Build test passed${NC}\n"
else
    echo -e "${RED}   ✗ Build test failed${NC}\n"
    exit 1
fi

# Success message
echo -e "${GREEN}=========================================="
echo "✅ Novanet zkVM SDK Installation Complete!"
echo -e "==========================================${NC}\n"

echo "Next steps:"
echo "  1. Navigate to the novanet-zkvm directory:"
echo "     cd novanet-zkvm"
echo ""
echo "  2. Run the demo:"
echo "     ./run_demo.sh"
echo "     or"
echo "     cargo run --release -p novanet-host"
echo ""
echo "  3. Try with different Fibonacci numbers:"
echo "     FIBONACCI_N=20 ./run_demo.sh"
echo ""
echo "  4. Run tests:"
echo "     cargo test"
echo ""
echo "For more information, see:"
echo "  • novanet-zkvm/README.md"
echo "  • Nova paper: https://eprint.iacr.org/2021/370"
echo "  • Nova GitHub: https://github.com/microsoft/nova"
echo ""
echo -e "${BLUE}Happy proving! 🎉${NC}"

