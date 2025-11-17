#!/bin/bash
# Cairo-M zkVM SDK Installer
# This script installs the Cairo-M toolchain including compiler, runner, and prover

set -e

echo "========================================="
echo "Cairo-M zkVM SDK Installer"
echo "========================================="
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust is installed"
echo ""

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "Rust version: $RUST_VERSION"
echo ""

# Install nightly toolchain
echo "Installing Rust nightly toolchain..."
rustup install nightly-2025-01-10
rustup component add rustfmt clippy --toolchain nightly-2025-01-10
echo -e "${GREEN}✓${NC} Nightly toolchain installed"
echo ""

# Clone cairo-m repository
CAIRO_M_DIR="$HOME/.cairo-m"
if [ -d "$CAIRO_M_DIR" ]; then
    echo "Cairo-M repository already exists at $CAIRO_M_DIR"
    echo "Updating repository..."
    cd "$CAIRO_M_DIR"
    git pull
else
    echo "Cloning Cairo-M repository..."
    git clone https://github.com/kkrt-labs/cairo-m.git "$CAIRO_M_DIR"
    cd "$CAIRO_M_DIR"
fi
echo -e "${GREEN}✓${NC} Cairo-M repository ready"
echo ""

# Initialize submodules (Stwo)
echo "Initializing git submodules (Stwo)..."
git submodule update --init --recursive
echo -e "${GREEN}✓${NC} Submodules initialized"
echo ""

# Install LLVM and LLD (required for MacOS)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Detected MacOS - checking for LLVM and LLD..."
    if ! command -v llvm-config &> /dev/null; then
        echo -e "${YELLOW}Warning: LLVM not found${NC}"
        echo "Installing LLVM via Homebrew..."
        if command -v brew &> /dev/null; then
            brew install llvm lld
            echo ""
            echo -e "${YELLOW}Important: Add these to your shell profile:${NC}"
            echo "  export CC=/opt/homebrew/opt/llvm/bin/clang"
            echo "  export CXX=/opt/homebrew/opt/llvm/bin/clang++"
            echo "  export AR=/opt/homebrew/opt/llvm/bin/llvm-ar"
            echo "  export RANLIB=/opt/homebrew/opt/llvm/bin/llvm-ranlib"
            echo ""
        else
            echo -e "${RED}Error: Homebrew not found. Please install LLVM manually.${NC}"
            exit 1
        fi
    else
        echo -e "${GREEN}✓${NC} LLVM is installed"
    fi
    echo ""
fi

# Build and install cairo-m-compiler
echo "Building cairo-m-compiler..."
cd "$CAIRO_M_DIR"
cargo install --path crates/compiler --force --locked
echo -e "${GREEN}✓${NC} cairo-m-compiler installed"
echo ""

# Build and install cairo-m-runner
echo "Building cairo-m-runner..."
cargo install --path crates/runner --force --locked
echo -e "${GREEN}✓${NC} cairo-m-runner installed"
echo ""

# Build and install cairo-m-prover
echo "Building cairo-m-prover..."
cargo install --path crates/prover --force --locked
echo -e "${GREEN}✓${NC} cairo-m-prover installed"
echo ""

# Build and install cargo-cairo-m
echo "Building cargo-cairo-m..."
if [ -d "$CAIRO_M_DIR/crates/cargo-cairo-m" ]; then
    cargo install --path crates/cargo-cairo-m --force --locked
    echo -e "${GREEN}✓${NC} cargo-cairo-m installed"
else
    echo -e "${YELLOW}Warning: cargo-cairo-m crate not found, skipping${NC}"
fi
echo ""

# Verify installations
echo "Verifying installations..."
echo ""

if command -v cairo-m-compiler &> /dev/null; then
    COMPILER_VERSION=$(cairo-m-compiler --version 2>&1 || echo "version detection failed")
    echo -e "${GREEN}✓${NC} cairo-m-compiler: $COMPILER_VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-compiler not found in PATH"
fi

if command -v cairo-m-runner &> /dev/null; then
    RUNNER_VERSION=$(cairo-m-runner --version 2>&1 || echo "version detection failed")
    echo -e "${GREEN}✓${NC} cairo-m-runner: $RUNNER_VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-runner not found in PATH"
fi

if command -v cairo-m-prover &> /dev/null; then
    PROVER_VERSION=$(cairo-m-prover --version 2>&1 || echo "version detection failed")
    echo -e "${GREEN}✓${NC} cairo-m-prover: $PROVER_VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-prover not found in PATH"
fi

if command -v cargo-cairo-m &> /dev/null; then
    CARGO_CAIRO_M_VERSION=$(cargo-cairo-m --version 2>&1 || cargo cairo-m --version 2>&1 || echo "version detection failed")
    echo -e "${GREEN}✓${NC} cargo-cairo-m: $CARGO_CAIRO_M_VERSION"
else
    echo -e "${YELLOW}⚠${NC} cargo-cairo-m not found (optional)"
fi

echo ""
echo "========================================="
echo "Installation Summary"
echo "========================================="
echo ""
echo "Cairo-M toolchain has been installed!"
echo ""
echo "Installed tools:"
echo "  • cairo-m-compiler - Compiles .cm files to executable format"
echo "  • cairo-m-runner   - Executes Cairo-M programs and generates traces"
echo "  • cairo-m-prover   - Generates STARK proofs using Stwo"
echo "  • cargo-cairo-m    - Project scaffolding tool (optional)"
echo ""
echo "Repository location: $CAIRO_M_DIR"
echo ""
echo "Next steps:"
echo "  1. Run the Cairo-M demo:"
echo "     cd cairo-m-zkvm/cairo-m-host"
echo "     cargo run --release"
echo ""
echo "  2. Try the CairoMlings tutorial:"
echo "     cargo install --path $CAIRO_M_DIR/tutorials/cairomlings"
echo "     cairomlings init"
echo ""
echo "  3. Create a new Cairo-M project:"
echo "     cargo cairo-m init my-project"
echo ""
echo "Resources:"
echo "  • Documentation: https://github.com/kkrt-labs/cairo-m/tree/main/docs"
echo "  • Examples: https://github.com/kkrt-labs/cairo-m/tree/main/examples"
echo "  • Design Doc: https://github.com/kkrt-labs/cairo-m/blob/main/docs/design-document.md"
echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo "========================================="

