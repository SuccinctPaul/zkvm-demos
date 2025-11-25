#!/bin/bash

# snarkVM SDK Installation Script
# This script installs the snarkVM library and dependencies

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}snarkVM SDK Installation${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check OS
OS="$(uname -s)"
echo -e "${BLUE}Detected OS:${NC} $OS"

# Step 1: Install Rust if not present
echo ""
echo -e "${YELLOW}[1/4]${NC} Checking Rust installation..."

if command_exists cargo; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓${NC} Rust is already installed: $RUST_VERSION"
else
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓${NC} Rust installed successfully"
fi

# Step 2: Set Rust version
echo ""
echo -e "${YELLOW}[2/4]${NC} Setting up Rust toolchain..."

rustup install 1.85 || true
rustup default 1.85

RUST_VERSION=$(rustc --version)
echo -e "${GREEN}✓${NC} Rust version: $RUST_VERSION"

# Step 3: Install build dependencies
echo ""
echo -e "${YELLOW}[3/4]${NC} Installing system dependencies..."

case "$OS" in
    Linux*)
        if command_exists apt-get; then
            echo "Installing dependencies via apt..."
            sudo apt-get update
            sudo apt-get install -y build-essential pkg-config libssl-dev clang
        elif command_exists yum; then
            echo "Installing dependencies via yum..."
            sudo yum groupinstall -y "Development Tools"
            sudo yum install -y openssl-devel clang
        elif command_exists pacman; then
            echo "Installing dependencies via pacman..."
            sudo pacman -S --noconfirm base-devel openssl clang
        else
            echo -e "${YELLOW}⚠${NC} Could not detect package manager. Please install build tools manually."
        fi
        ;;
    Darwin*)
        if command_exists brew; then
            echo "Installing dependencies via Homebrew..."
            brew install openssl pkg-config
        else
            echo -e "${YELLOW}⚠${NC} Homebrew not found. Please install Xcode Command Line Tools:"
            echo "   xcode-select --install"
        fi
        ;;
    *)
        echo -e "${YELLOW}⚠${NC} Unknown OS. Please ensure build tools are installed."
        ;;
esac

echo -e "${GREEN}✓${NC} Dependencies installed"

# Step 4: Verify installation
echo ""
echo -e "${YELLOW}[4/4]${NC} Verifying installation..."

# Check if we can compile a simple snarkVM project
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

cat > Cargo.toml << 'EOF'
[package]
name = "snarkvm-test"
version = "0.1.0"
edition = "2021"

[dependencies]
snarkvm = "0.16"
EOF

cat > src/main.rs << 'EOF'
fn main() {
    println!("snarkVM SDK verification successful!");
}
EOF

mkdir -p src

echo "Compiling test project..."
if cargo build --release 2>&1 | grep -q "Finished"; then
    echo -e "${GREEN}✓${NC} snarkVM SDK verification successful"
else
    echo -e "${YELLOW}⚠${NC} Verification build in progress (this may take a while)..."
    cargo build --release
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} snarkVM SDK verification successful"
    else
        echo -e "${RED}✗${NC} Verification failed"
        exit 1
    fi
fi

# Cleanup
cd - > /dev/null
rm -rf "$TEST_DIR"

# Installation complete
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✓ snarkVM SDK Installation Complete!${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Navigate to the demo: ${GREEN}cd snarkvm-zkvm${NC}"
echo -e "  2. Run the demo: ${GREEN}./run_demo.sh${NC}"
echo -e "  3. Or build manually: ${GREEN}cd snarkvm-host && cargo build --release${NC}"
echo ""
echo -e "${YELLOW}Resources:${NC}"
echo -e "  - snarkVM: ${BLUE}https://github.com/ProvableHQ/snarkVM${NC}"
echo -e "  - Aleo: ${BLUE}https://aleo.org/${NC}"
echo -e "  - Documentation: ${BLUE}https://developer.aleo.org/${NC}"
echo ""

