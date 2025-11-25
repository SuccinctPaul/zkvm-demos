#!/bin/bash
# Powdr zkVM SDK Installer
# Installs the Powdr toolkit for building custom zkVMs
# https://github.com/powdr-labs/powdr

set -e

echo "=========================================="
echo "  Installing Powdr zkVM Toolkit"
echo "=========================================="
echo ""

# Check prerequisites
if ! command -v curl &> /dev/null; then
    echo "❌ Error: curl could not be found, please install it first." >&2
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: cargo could not be found. Please install Rust first:" >&2
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh" >&2
    exit 1
fi

if ! command -v git &> /dev/null; then
    echo "❌ Error: git could not be found, please install it first." >&2
    exit 1
fi

# Define installation directory
DEFAULT_POWDR_DIR="${HOME}/.powdr"
export POWDR_DIR="${POWDR_DIR:-${DEFAULT_POWDR_DIR}}"

echo "📦 Installation directory: ${POWDR_DIR}"
echo ""

# Create installation directory
mkdir -p "${POWDR_DIR}"

# Install Powdr CLI from source
echo "🔨 Step 1: Installing Powdr CLI..."
echo "   This may take several minutes as we compile from source..."
echo ""

# Install from crates.io or GitHub
# Note: As of now, Powdr may not be published to crates.io, so we install from GitHub
if cargo install --list | grep -q "^powdr-cli"; then
    echo "✅ Powdr CLI is already installed"
    powdr --version
else
    echo "   Installing from GitHub (main branch)..."
    # Install Powdr CLI
    # This will install the powdr binary to ~/.cargo/bin/
    if cargo install --git https://github.com/powdr-labs/powdr --branch main powdr-cli --locked; then
        echo "✅ Powdr CLI installed successfully!"
    else
        echo "⚠️  Direct installation failed. Trying alternative method..."
        
        # Clone and build manually
        TEMP_DIR=$(mktemp -d)
        cd "${TEMP_DIR}"
        
        echo "   Cloning Powdr repository..."
        git clone https://github.com/powdr-labs/powdr.git
        cd powdr
        
        echo "   Building Powdr CLI (this may take 10-15 minutes)..."
        cargo build --release --bin powdr-cli
        
        # Copy binary to cargo bin directory
        cp target/release/powdr-cli ~/.cargo/bin/powdr
        
        # Cleanup
        cd ~
        rm -rf "${TEMP_DIR}"
        
        echo "✅ Powdr CLI built and installed successfully!"
    fi
fi

echo ""
echo "🔧 Step 2: Verifying installation..."

# Verify installation
if ! command -v powdr &> /dev/null; then
    echo "❌ Error: powdr command not found after installation." >&2
    echo "   Please ensure ~/.cargo/bin is in your PATH" >&2
    exit 1
fi

echo "   Powdr version:"
powdr --version || echo "   (Version command not available yet)"

echo ""
echo "🛠️  Step 3: Installing additional dependencies..."

# Install WASM target for Rust (if not already installed)
if rustup target list | grep "wasm32-unknown-unknown (installed)" > /dev/null; then
    echo "✅ WASM target already installed"
else
    echo "   Installing WASM target for Rust..."
    rustup target add wasm32-unknown-unknown
    echo "✅ WASM target installed"
fi

# Install RISC-V target (if not already installed)
if rustup target list | grep "riscv32im-unknown-none-elf (installed)" > /dev/null; then
    echo "✅ RISC-V target already installed"
else
    echo "   Installing RISC-V target for Rust..."
    rustup target add riscv32im-unknown-none-elf
    echo "✅ RISC-V target installed"
fi

echo ""
echo "=========================================="
echo "  ✅ Powdr zkVM Toolkit Installed!"
echo "=========================================="
echo ""
echo "📚 Next Steps:"
echo ""
echo "   1. Verify installation:"
echo "      powdr --version"
echo ""
echo "   2. Run the Fibonacci demo:"
echo "      cd powdr-zkvm/powdr-host"
echo "      cargo run --release"
echo ""
echo "   3. Explore Powdr documentation:"
echo "      https://docs.powdr.org/"
echo ""
echo "   4. Check out examples:"
echo "      https://github.com/powdr-labs/powdr/tree/main/examples"
echo ""
echo "💡 Tips:"
echo "   - Powdr is a toolkit, not a ready-to-use zkVM"
echo "   - You can combine different frontends (RISC-V, WASM) with backends (Halo2, Plonky2)"
echo "   - See the README in powdr-zkvm/ for more information"
echo ""
echo "⚠️  Note: Powdr is under active development. Some features may change."
echo ""

# Save installation info
cat > "${POWDR_DIR}/install_info.txt" << EOF
Powdr zkVM Toolkit Installation
================================

Installed: $(date)
Installation Directory: ${POWDR_DIR}
Powdr CLI: $(which powdr || echo "not found in PATH")

Environment Variables:
  POWDR_DIR=${POWDR_DIR}

To use Powdr in your shell, ensure ~/.cargo/bin is in your PATH:
  export PATH="\$HOME/.cargo/bin:\$PATH"

For more information, visit:
  https://github.com/powdr-labs/powdr
  https://docs.powdr.org/
EOF

echo "📝 Installation info saved to: ${POWDR_DIR}/install_info.txt"
echo ""

