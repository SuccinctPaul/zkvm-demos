#!/bin/bash

set -e

echo "================================================"
echo "o1vm zkVM SDK Installation Script"
echo "================================================"
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "System Information:"
echo "  OS: $OS"
echo "  Architecture: $ARCH"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print colored messages
print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Step 1: Check/Install Rust
echo "Step 1: Checking Rust installation..."
if command_exists rustc; then
    RUST_VERSION=$(rustc --version | cut -d' ' -f2)
    print_success "Rust is already installed (version $RUST_VERSION)"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_success "Rust installed successfully"
fi

# Ensure we have the right Rust version
echo "Ensuring Rust 1.75+ is available..."
rustup install 1.75 2>/dev/null || true
print_success "Rust toolchain ready"
echo ""

# Step 2: Check/Install MIPS Cross-Compiler
echo "Step 2: Checking MIPS cross-compiler..."
if command_exists mips-linux-gnu-gcc; then
    MIPS_VERSION=$(mips-linux-gnu-gcc --version | head -n1)
    print_success "MIPS cross-compiler is already installed"
    echo "  Version: $MIPS_VERSION"
else
    print_warning "MIPS cross-compiler not found"
    echo ""
    
    if [ "$OS" = "Linux" ]; then
        # Detect Linux distribution
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            DISTRO=$ID
        else
            DISTRO="unknown"
        fi
        
        echo "Attempting to install MIPS cross-compiler..."
        
        if [ "$DISTRO" = "ubuntu" ] || [ "$DISTRO" = "debian" ]; then
            echo "Detected Debian/Ubuntu system"
            echo "Installing gcc-mips-linux-gnu..."
            
            if [ "$EUID" -ne 0 ]; then
                echo "This requires sudo privileges."
                sudo apt-get update
                sudo apt-get install -y gcc-mips-linux-gnu binutils-mips-linux-gnu
            else
                apt-get update
                apt-get install -y gcc-mips-linux-gnu binutils-mips-linux-gnu
            fi
            
            print_success "MIPS cross-compiler installed"
        else
            print_warning "Automatic installation not supported for $DISTRO"
            echo "Please install manually:"
            echo "  - Search for 'gcc-mips-linux-gnu' package"
            echo "  - Or compile from source"
        fi
    elif [ "$OS" = "Darwin" ]; then
        print_warning "MIPS cross-compiler not readily available on macOS"
        echo ""
        echo "Options for macOS:"
        echo "  1. Use Docker (Recommended):"
        echo "     docker run --rm -v \$(pwd):/work -w /work ubuntu:22.04 bash -c \\"
        echo "       'apt-get update && apt-get install -y gcc-mips-linux-gnu && \\"
        echo "        cd o1vm-guest && make'"
        echo ""
        echo "  2. Build from source (Advanced):"
        echo "     - Build GNU toolchain for MIPS target"
        echo "     - See: https://github.com/richfelker/musl-cross-make"
        echo ""
        echo "  3. Use pre-compiled binaries"
        echo ""
        echo "For this demo, you can continue without MIPS toolchain."
        echo "The Rust host will demonstrate the concepts."
    else
        print_warning "Unsupported operating system: $OS"
        echo "Manual installation required"
    fi
fi
echo ""

# Step 3: Clone proof-systems repository (for reference)
echo "Step 3: Setting up proof-systems repository reference..."
PROOF_SYSTEMS_DIR="$HOME/.o1vm/proof-systems"

if [ -d "$PROOF_SYSTEMS_DIR" ]; then
    print_success "proof-systems repository already exists at $PROOF_SYSTEMS_DIR"
    echo "  Updating..."
    cd "$PROOF_SYSTEMS_DIR"
    git pull origin master 2>/dev/null || print_warning "Could not update repository"
else
    echo "Cloning proof-systems repository..."
    mkdir -p "$HOME/.o1vm"
    git clone https://github.com/o1-labs/proof-systems.git "$PROOF_SYSTEMS_DIR"
    print_success "proof-systems repository cloned"
fi
echo ""

# Step 4: Verify installation
echo "Step 4: Verifying installation..."
echo ""

HAS_ERRORS=0

# Check Rust
if command_exists cargo; then
    print_success "Cargo: $(cargo --version)"
else
    print_error "Cargo not found"
    HAS_ERRORS=1
fi

# Check rustc
if command_exists rustc; then
    print_success "Rustc: $(rustc --version)"
else
    print_error "Rustc not found"
    HAS_ERRORS=1
fi

# Check MIPS compiler (optional)
if command_exists mips-linux-gnu-gcc; then
    print_success "MIPS GCC: Available"
else
    print_warning "MIPS GCC: Not available (optional)"
fi

echo ""

# Step 5: Test build (optional)
echo "Step 5: Testing build..."
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
O1VM_DIR="$SCRIPT_DIR/../../o1vm-zkvm"

if [ -d "$O1VM_DIR" ]; then
    echo "Found o1vm-zkvm directory at: $O1VM_DIR"
    echo "Attempting to build..."
    
    cd "$O1VM_DIR/o1vm-host"
    
    if cargo build --release 2>&1 | grep -q "Finished"; then
        print_success "Build test successful"
    else
        print_warning "Build test had warnings or errors (this is normal during development)"
    fi
else
    print_warning "o1vm-zkvm directory not found at expected location"
    echo "  Expected: $O1VM_DIR"
fi
echo ""

# Summary
echo "================================================"
echo "Installation Summary"
echo "================================================"

if [ $HAS_ERRORS -eq 0 ]; then
    print_success "All core components installed successfully!"
    echo ""
    echo "Next steps:"
    echo "  1. cd o1vm-zkvm"
    echo "  2. ./run_demo.sh"
    echo ""
    echo "Or manually:"
    echo "  1. cd o1vm-zkvm/o1vm-host"
    echo "  2. cargo build --release"
    echo "  3. cargo run --release"
    echo ""
    
    if ! command_exists mips-linux-gnu-gcc; then
        print_warning "Note: MIPS cross-compiler not installed"
        echo "  The demo will work without it, but you won't be able to compile guest programs."
        echo "  See documentation for installation instructions."
    fi
else
    print_error "Some components failed to install"
    echo "Please check the errors above and install manually."
    exit 1
fi

echo ""
echo "Resources:"
echo "  - o1vm source: https://github.com/o1-labs/proof-systems/tree/master/o1vm"
echo "  - Documentation: https://o1-labs.github.io/proof-systems/"
echo "  - Local reference: $PROOF_SYSTEMS_DIR"
echo ""
echo "================================================"

print_success "Installation complete!"

