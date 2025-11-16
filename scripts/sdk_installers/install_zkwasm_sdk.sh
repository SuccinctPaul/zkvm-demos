#!/bin/bash

# zkWasm SDK Installation Script
# This script installs the zkWasm CLI (delphinus-cli) from source

set -e

echo "=================================="
echo "   zkWasm SDK Installer"
echo "=================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✓ Rust found${NC}"

# Check for clang/lld
if [[ "$OSTYPE" == "darwin"* ]]; then
    if ! command -v clang &> /dev/null; then
        echo -e "${YELLOW}Warning: clang not found${NC}"
        echo "Installing via Homebrew..."
        brew install llvm
    fi
else
    if ! command -v clang &> /dev/null; then
        echo -e "${YELLOW}Warning: clang not found${NC}"
        echo "Please run: sudo apt-get install clang lld"
        exit 1
    fi
fi
echo -e "${GREEN}✓ clang/lld found${NC}"
echo ""

# Installation directory
INSTALL_DIR="${HOME}/.zkwasm"
ZKWASM_REPO="${INSTALL_DIR}/zkwasm"

echo -e "${BLUE}Installation directory: ${INSTALL_DIR}${NC}"
echo ""

# Create installation directory
mkdir -p "${INSTALL_DIR}"

# Clone or update repository
if [ -d "${ZKWASM_REPO}" ]; then
    echo -e "${YELLOW}zkWasm repository already exists. Updating...${NC}"
    cd "${ZKWASM_REPO}"
    git fetch origin
    git checkout main
    git pull origin main
    git submodule update --init --recursive
else
    echo -e "${BLUE}Cloning zkWasm repository...${NC}"
    git clone --recurse-submodules https://github.com/DelphinusLab/zkWasm.git "${ZKWASM_REPO}"
    cd "${ZKWASM_REPO}"
fi
echo -e "${GREEN}✓ Repository ready${NC}"
echo ""

# Build zkWasm CLI
echo -e "${BLUE}Building zkWasm CLI (this may take 10-20 minutes)...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build complete${NC}"
echo ""

# Create symlink to make it accessible
BIN_DIR="${HOME}/.local/bin"
mkdir -p "${BIN_DIR}"

ln -sf "${ZKWASM_REPO}/target/release/delphinus-cli" "${BIN_DIR}/delphinus-cli"
echo -e "${GREEN}✓ Created symlink in ${BIN_DIR}${NC}"
echo ""

# Check if bin directory is in PATH
if [[ ":$PATH:" != *":${BIN_DIR}:"* ]]; then
    echo -e "${YELLOW}Note: ${BIN_DIR} is not in your PATH${NC}"
    echo ""
    echo "Add the following line to your shell configuration file:"
    echo ""
    if [[ "$SHELL" == *"zsh"* ]]; then
        echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
        echo "  source ~/.zshrc"
    else
        echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
        echo "  source ~/.bashrc"
    fi
    echo ""
fi

# Verify installation
if command -v delphinus-cli &> /dev/null; then
    VERSION=$(delphinus-cli --version 2>&1 || echo "unknown")
    echo "=================================="
    echo -e "${GREEN}Installation successful!${NC}"
    echo "=================================="
    echo ""
    echo "zkWasm CLI installed at: ${BIN_DIR}/delphinus-cli"
    echo "Version: ${VERSION}"
    echo ""
    echo "Try it out:"
    echo "  delphinus-cli --help"
    echo ""
    echo "Run the zkWasm demo:"
    echo "  cd zkwasm-zkvm"
    echo "  ./run_demo.sh"
else
    echo "=================================="
    echo -e "${GREEN}Build successful!${NC}"
    echo "=================================="
    echo ""
    echo "To use delphinus-cli, either:"
    echo "  1. Add ${BIN_DIR} to your PATH (recommended)"
    echo "  2. Use the full path: ${BIN_DIR}/delphinus-cli"
    echo "  3. Run from: ${ZKWASM_REPO}/target/release/delphinus-cli"
fi

echo ""
echo "Documentation: https://github.com/DelphinusLab/zkWasm"
echo "Paper: https://ieeexplore.ieee.org/document/10587123"

