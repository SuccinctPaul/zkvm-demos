#!/bin/bash
# Cairo-M zkVM SDK Installer
# Based on https://github.com/kkrt-labs/cairo-m official documentation
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
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust is installed"
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "  Version: $RUST_VERSION"
echo ""

# Install nightly toolchain
echo "Checking Rust nightly toolchain..."
if rustup toolchain list | grep -q nightly; then
    echo -e "${GREEN}✓${NC} Nightly toolchain is available"
    # Try to update if possible
    rustup update nightly 2>/dev/null || true
else
    echo "Installing nightly toolchain..."
    rustup install nightly
fi
echo ""

# Clone cairo-m repository
CAIRO_M_DIR="$HOME/.cairo-m"
if [ -d "$CAIRO_M_DIR" ]; then
    echo "Cairo-M repository already exists at $CAIRO_M_DIR"
    read -p "Update existing repository? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cd "$CAIRO_M_DIR"
        git pull
        echo -e "${GREEN}✓${NC} Repository updated"
    fi
else
    echo "Cloning Cairo-M repository..."
    git clone https://github.com/kkrt-labs/cairo-m.git "$CAIRO_M_DIR"
    cd "$CAIRO_M_DIR"
    echo -e "${GREEN}✓${NC} Repository cloned"
fi
echo ""

# Initialize submodules (Stwo)
echo "Initializing git submodules (Stwo)..."
cd "$CAIRO_M_DIR"
git submodule update --init --recursive
echo -e "${GREEN}✓${NC} Submodules initialized"
echo ""

# Handle MacOS-specific build requirements
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "========================================="
    echo "MacOS Build Environment Setup"
    echo "========================================="
    echo ""
    
    # Check if Homebrew is available
    if ! command -v brew &> /dev/null; then
        echo -e "${RED}Error: Homebrew not found.${NC}"
        echo "Please install Homebrew first:"
        echo "  /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
        exit 1
    fi
    
    # Install LLVM
    echo "Checking LLVM installation..."
    if ! brew list llvm &> /dev/null 2>&1; then
        echo -e "${YELLOW}Installing LLVM...${NC}"
        brew install llvm
    else
        echo -e "${GREEN}✓${NC} LLVM is installed"
    fi
    
    # Install LLD
    echo "Checking LLD installation..."
    if ! brew list lld &> /dev/null 2>&1; then
        echo -e "${YELLOW}Installing LLD...${NC}"
        brew install lld
    else
        echo -e "${GREEN}✓${NC} LLD is installed"
    fi
    echo ""
    
    # Set environment variables
    LLVM_PREFIX=$(brew --prefix llvm)
    LLD_PREFIX=$(brew --prefix lld)
    
    export CC="$LLVM_PREFIX/bin/clang"
    export CXX="$LLVM_PREFIX/bin/clang++"
    export AR="$LLVM_PREFIX/bin/llvm-ar"
    export RANLIB="$LLVM_PREFIX/bin/llvm-ranlib"
    
    echo -e "${BLUE}Build environment configured:${NC}"
    echo "  CC=$CC"
    echo "  CXX=$CXX"
    echo "  AR=$AR"
    echo "  RANLIB=$RANLIB"
    echo "  LLD path: $LLD_PREFIX"
    echo ""
    
    # Verify LLD linker exists
    LLD_LINKER="$LLD_PREFIX/bin/ld64.lld"
    if [ -f "$LLD_LINKER" ]; then
        echo -e "${GREEN}✓${NC} LLD linker found at: $LLD_LINKER"
    else
        echo -e "${YELLOW}⚠${NC}  LLD linker not found at expected location"
        echo "  Checking alternative locations..."
        if command -v ld64.lld &> /dev/null; then
            LLD_LINKER=$(which ld64.lld)
            echo -e "${GREEN}✓${NC} Found ld64.lld at: $LLD_LINKER"
        fi
    fi
    echo ""
    
    echo -e "${YELLOW}Important: Add these to your ~/.zshrc or ~/.bash_profile:${NC}"
    echo "  export CC=$LLVM_PREFIX/bin/clang"
    echo "  export CXX=$LLVM_PREFIX/bin/clang++"
    echo "  export AR=$LLVM_PREFIX/bin/llvm-ar"
    echo "  export RANLIB=$LLVM_PREFIX/bin/llvm-ranlib"
    echo ""
    echo "Press Enter to continue with installation..."
    read
fi

echo "========================================="
echo "Building Cairo-M Toolchain"
echo "========================================="
echo ""

# List available crates
if [ -d "$CAIRO_M_DIR/crates" ]; then
    echo -e "${BLUE}Available crates:${NC}"
    ls -1 "$CAIRO_M_DIR/crates" | sed 's/^/  - /'
    echo ""
fi

# Build and install compiler
echo "Building cairo-m-compiler..."
cd "$CAIRO_M_DIR"
if [ -f "crates/compiler/Cargo.toml" ]; then
    cargo install --path crates/compiler --force --locked 2>&1 | grep -E "(Installing|Compiling|Finished|Installed|error|warning:)" || true
    if command -v cairo-m-compiler &> /dev/null; then
        echo -e "${GREEN}✓${NC} cairo-m-compiler installed successfully"
    else
        echo -e "${RED}✗${NC} Failed to install cairo-m-compiler"
    fi
else
    echo -e "${RED}✗${NC} Compiler crate not found at crates/compiler"
fi
echo ""

# Build and install runner
echo "Building cairo-m-runner..."
if [ -f "crates/runner/Cargo.toml" ]; then
    cargo install --path crates/runner --force --locked 2>&1 | grep -E "(Installing|Compiling|Finished|Installed|error|warning:)" || true
    if command -v cairo-m-runner &> /dev/null; then
        echo -e "${GREEN}✓${NC} cairo-m-runner installed successfully"
    else
        echo -e "${RED}✗${NC} Failed to install cairo-m-runner"
    fi
else
    echo -e "${RED}✗${NC} Runner crate not found at crates/runner"
fi
echo ""

# Build and install prover
echo "Building cairo-m-prover..."
if [ -f "crates/prover/Cargo.toml" ]; then
    cargo install --path crates/prover --force --locked 2>&1 | grep -E "(Installing|Compiling|Finished|Installed|error|warning:)" || true
    if command -v cairo-m-prover &> /dev/null; then
        echo -e "${GREEN}✓${NC} cairo-m-prover installed successfully"
    else
        echo -e "${RED}✗${NC} Failed to install cairo-m-prover"
    fi
else
    echo -e "${RED}✗${NC} Prover crate not found at crates/prover"
fi
echo ""

# Build and install cargo-cairo-m (optional)
echo "Building cargo-cairo-m (optional)..."
if [ -f "crates/cargo-cairo-m/Cargo.toml" ]; then
    cargo install --path crates/cargo-cairo-m --force --locked 2>&1 | grep -E "(Installing|Compiling|Finished|Installed|error|warning:)" || true
    if command -v cargo-cairo-m &> /dev/null; then
        echo -e "${GREEN}✓${NC} cargo-cairo-m installed successfully"
    else
        echo -e "${YELLOW}⚠${NC}  cargo-cairo-m installation failed (optional)"
    fi
else
    echo -e "${YELLOW}⚠${NC}  cargo-cairo-m crate not found (optional)"
fi
echo ""

echo "========================================="
echo "Installation Verification"
echo "========================================="
echo ""

# Verify installations
INSTALL_SUCCESS=true

if command -v cairo-m-compiler &> /dev/null; then
    VERSION=$(cairo-m-compiler --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓${NC} cairo-m-compiler: $VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-compiler not found in PATH"
    INSTALL_SUCCESS=false
fi

if command -v cairo-m-runner &> /dev/null; then
    VERSION=$(cairo-m-runner --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓${NC} cairo-m-runner: $VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-runner not found in PATH"
    INSTALL_SUCCESS=false
fi

if command -v cairo-m-prover &> /dev/null; then
    VERSION=$(cairo-m-prover --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓${NC} cairo-m-prover: $VERSION"
else
    echo -e "${RED}✗${NC} cairo-m-prover not found in PATH"
    INSTALL_SUCCESS=false
fi

if command -v cargo-cairo-m &> /dev/null; then
    VERSION=$(cargo-cairo-m --version 2>&1 || echo "unknown")
    echo -e "${GREEN}✓${NC} cargo-cairo-m: $VERSION"
else
    echo -e "${YELLOW}⚠${NC}  cargo-cairo-m not found (optional)"
fi

echo ""
echo "========================================="
echo "Installation Summary"
echo "========================================="
echo ""

if [ "$INSTALL_SUCCESS" = true ]; then
    echo -e "${GREEN}✓ Installation completed successfully!${NC}"
else
    echo -e "${YELLOW}⚠ Installation completed with some errors${NC}"
    echo "  Please check the error messages above"
fi

echo ""
echo "Repository location: $CAIRO_M_DIR"
echo ""
echo "Installed tools:"
echo "  • cairo-m-compiler - Compiles .cm files to executable format"
echo "  • cairo-m-runner   - Executes Cairo-M programs and generates traces"
echo "  • cairo-m-prover   - Generates STARK proofs using Stwo"
echo "  • cargo-cairo-m    - Project scaffolding tool (optional)"
echo ""
echo "Next steps:"
echo "  1. Test the installation:"
echo "     cd $CAIRO_M_DIR"
echo "     cairo-m-compiler --help"
echo ""
echo "  2. Run the Cairo-M demo from zkvm-demos:"
echo "     cd cairo-m-zkvm/cairo-m-host"
echo "     cargo run --release"
echo ""
echo "  3. Try the CairoMlings tutorial:"
echo "     cargo install --path $CAIRO_M_DIR/tutorials/cairomlings"
echo "     cairomlings init"
echo ""
echo "  4. Create a new Cairo-M project:"
echo "     cargo-cairo-m init my-project"
echo ""
echo "Resources:"
echo "  • Documentation: https://github.com/kkrt-labs/cairo-m/tree/main/docs"
echo "  • Examples: https://github.com/kkrt-labs/cairo-m/tree/main/examples"
echo "  • Design Doc: https://github.com/kkrt-labs/cairo-m/blob/main/docs/design-document.md"
echo "  • Getting Started: https://github.com/kkrt-labs/cairo-m/blob/main/docs/getting-started.md"
echo ""

if [[ "$OSTYPE" == "darwin"* ]]; then
    echo -e "${YELLOW}MacOS Users:${NC}"
    echo "Remember to add the environment variables to your shell profile!"
    echo ""
fi

echo "========================================="
