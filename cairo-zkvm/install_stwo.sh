#!/bin/bash
# Stwo-Cairo Installation Script
# This script installs the Stwo-Cairo prover toolchain

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Stwo-Cairo Installation Script${NC}"
echo "=================================================="

# Function to print colored status
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC}  $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check prerequisites
echo ""
echo "📋 Checking prerequisites..."

# Check Rust
if ! command -v rustc &> /dev/null; then
    print_error "Rust not found"
    echo ""
    echo "Install Rust from: https://rustup.rs/"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
RUST_VERSION=$(rustc --version | awk '{print $2}')
print_status "Rust $RUST_VERSION installed"

# Check cargo
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found"
    exit 1
fi
print_status "Cargo installed"

# Check git
if ! command -v git &> /dev/null; then
    print_error "Git not found"
    echo "Please install git first"
    exit 1
fi
print_status "Git installed"

# Check if cairo-prove is already installed
if command -v cairo-prove &> /dev/null; then
    print_warning "cairo-prove is already installed"
    CAIRO_PROVE_VERSION=$(cairo-prove --version 2>&1 || echo "unknown")
    echo "   Current version: $CAIRO_PROVE_VERSION"
    echo ""
    read -p "Reinstall? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Installation cancelled"
        exit 0
    fi
fi

# Set installation directory
INSTALL_DIR="${HOME}/.stwo-cairo"
TEMP_DIR="/tmp/stwo-cairo-build"

echo ""
echo "📦 Installation settings:"
echo "   Source directory: $TEMP_DIR"
echo "   Binary location:  /usr/local/bin/cairo-prove"
echo ""

# Clean up old temporary directory
if [ -d "$TEMP_DIR" ]; then
    print_warning "Removing old temporary directory"
    rm -rf "$TEMP_DIR"
fi

# Clone the repository
echo ""
echo "📥 Cloning stwo-cairo repository..."
if git clone https://github.com/starkware-libs/stwo-cairo.git "$TEMP_DIR"; then
    print_status "Repository cloned successfully"
else
    print_error "Failed to clone repository"
    exit 1
fi

# Navigate to cairo-prove directory
cd "$TEMP_DIR/cairo-prove"

# Check for rust-toolchain.toml
if [ -f "rust-toolchain.toml" ]; then
    REQUIRED_RUST=$(grep 'channel' rust-toolchain.toml | cut -d'"' -f2)
    print_status "Required Rust version: $REQUIRED_RUST"
fi

# Build the project
echo ""
echo "🔨 Building cairo-prove..."
echo "   This may take several minutes..."

# Check if build.sh exists
if [ -f "build.sh" ]; then
    if ./build.sh; then
        print_status "Build completed successfully"
    else
        print_error "Build failed"
        echo ""
        echo "Try building manually:"
        echo "  cd $TEMP_DIR/cairo-prove"
        echo "  cargo build --release"
        exit 1
    fi
else
    # No build.sh, try cargo directly
    if cargo build --release; then
        print_status "Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
fi

# Check if binary was created
BINARY_PATH="$TEMP_DIR/cairo-prove/target/release/cairo-prove"
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binary not found at $BINARY_PATH"
    exit 1
fi

print_status "Binary created: $BINARY_PATH"

# Get binary size
BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
echo "   Binary size: $BINARY_SIZE"

# Install the binary
echo ""
echo "📦 Installing cairo-prove..."

# Check if we have write permissions for /usr/local/bin
if [ -w "/usr/local/bin" ]; then
    cp "$BINARY_PATH" /usr/local/bin/
    print_status "Installed to /usr/local/bin/cairo-prove"
else
    # Need sudo
    print_warning "Requesting sudo access to install to /usr/local/bin"
    sudo cp "$BINARY_PATH" /usr/local/bin/
    print_status "Installed to /usr/local/bin/cairo-prove"
fi

# Verify installation
echo ""
echo "✅ Verifying installation..."
if command -v cairo-prove &> /dev/null; then
    print_status "cairo-prove is available in PATH"
    
    # Try to run cairo-prove
    echo ""
    echo "Testing cairo-prove:"
    cairo-prove --version || cairo-prove --help | head -5
else
    print_error "cairo-prove not found in PATH"
    echo ""
    echo "You may need to add /usr/local/bin to your PATH:"
    echo "  export PATH=\"/usr/local/bin:\$PATH\""
    exit 1
fi

# Clean up
echo ""
echo "🧹 Cleaning up..."
cd /
rm -rf "$TEMP_DIR"
print_status "Temporary files removed"

# Optional: Install to separate directory
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
fi

# Summary
echo ""
echo "=================================================="
echo -e "${GREEN}✨ Installation completed successfully!${NC}"
echo ""
echo "Cairo-prove is now installed and ready to use."
echo ""
echo "Quick test:"
echo "  cairo-prove --help"
echo ""
echo "Next steps:"
echo "  1. Read the integration guide:"
echo "     cat STWO_INTEGRATION_GUIDE.md"
echo ""
echo "  2. Run the demo:"
echo "     ./run_stwo_demo.sh"
echo ""
echo "=================================================="

