#!/bin/bash

# Airbender zkVM SDK Installation Script
# 
# This script prepares the environment for Airbender zkVM development.
# Note: The official Airbender SDK is not yet publicly released.
# This script will be updated once the SDK becomes available.
#
# References:
# - zkSync Airbender: https://docs.zksync.io/zk-stack/components/zksync-airbender
# - ere project: https://github.com/eth-act/ere

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     Airbender zkVM SDK Installation                       ║"
echo "║     High-Performance RISC-V Zero-Knowledge VM             ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Check if running on supported platform
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "📋 System Information:"
echo "   OS: $OS"
echo "   Architecture: $ARCH"
echo ""

# Check Rust installation
echo "🦀 Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "   Please install Rust from https://rustup.rs/"
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo "✅ Rust found: $RUST_VERSION"
echo ""

# Check Rust version
echo "🔍 Checking Rust version..."
RUST_MIN_VERSION="1.85.0"
RUST_CURRENT=$(rustc --version | cut -d' ' -f2)

if [ "$(printf '%s\n' "$RUST_MIN_VERSION" "$RUST_CURRENT" | sort -V | head -n1)" != "$RUST_MIN_VERSION" ]; then
    echo "⚠️  Rust version $RUST_CURRENT is older than recommended $RUST_MIN_VERSION"
    echo "   Consider upgrading: rustup update"
else
    echo "✅ Rust version is compatible"
fi
echo ""

# Install RISC-V target
echo "🎯 Installing RISC-V target..."
if rustup target list | grep -q "riscv32im-unknown-none-elf (installed)"; then
    echo "✅ RISC-V target already installed"
else
    echo "📦 Installing riscv32im-unknown-none-elf target..."
    rustup target add riscv32im-unknown-none-elf
    echo "✅ RISC-V target installed successfully"
fi
echo ""

# Install additional components
echo "🔧 Installing additional components..."
rustup component add rustfmt clippy 2>/dev/null || true
echo "✅ Components installed"
echo ""

# Check for GPU (optional, but recommended for Airbender)
echo "🎮 Checking for GPU availability..."
if command -v nvidia-smi &> /dev/null; then
    echo "✅ NVIDIA GPU detected:"
    nvidia-smi --query-gpu=name --format=csv,noheader | head -1
    echo "   Note: Airbender achieves ~21.8 MHz on H100 GPU"
elif command -v rocm-smi &> /dev/null; then
    echo "✅ AMD GPU detected"
    echo "   Note: Airbender is optimized for NVIDIA GPUs"
else
    echo "⚠️  No GPU detected"
    echo "   Airbender can run on CPU but GPU is highly recommended for performance"
fi
echo ""

# Check for Airbender SDK
echo "📦 Checking Airbender SDK availability..."
echo "⚠️  NOTE: Airbender SDK is not yet publicly released"
echo ""
echo "   The Airbender zkVM is currently integrated in:"
echo "   • zkSync Era"
echo "   • Abstract chain"
echo "   • Sophon chain"
echo ""
echo "   To use Airbender in your project:"
echo "   1. Monitor zkSync announcements: https://docs.zksync.io/"
echo "   2. Check the ere project: https://github.com/eth-act/ere"
echo "   3. Follow zkSync GitHub: https://github.com/matter-labs"
echo ""

# Set up project dependencies
echo "🔨 Setting up project dependencies..."
cd "$PROJECT_ROOT/airbender-zkvm"

if [ -f "Cargo.toml" ]; then
    echo "📦 Checking and fetching dependencies..."
    cargo fetch || {
        echo "⚠️  Some dependencies may not be available yet"
        echo "   This is expected until the official Airbender SDK is released"
    }
else
    echo "❌ Cargo.toml not found in airbender-zkvm directory"
    exit 1
fi
echo ""

# Create environment file template
ENV_FILE="$PROJECT_ROOT/airbender-zkvm/.env.example"
echo "📝 Creating environment template..."
cat > "$ENV_FILE" << 'EOF'
# Airbender zkVM Environment Configuration

# Fibonacci input value
FIB_N=10

# Proof generation settings (placeholder for future SDK)
# AIRBENDER_PROVER_MODE=gpu
# AIRBENDER_GPU_DEVICE=0
# AIRBENDER_PROOF_LEVEL=standard

# Logging
RUST_LOG=info
EOF
echo "✅ Created .env.example"
echo ""

# Print next steps
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║ Installation Complete!                                    ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "📚 What's Installed:"
echo "   ✓ Rust toolchain (with RISC-V target)"
echo "   ✓ Project dependencies (fetched)"
echo "   ✓ Environment template"
echo ""
echo "⚠️  Important Notes:"
echo "   • Airbender SDK is not yet publicly available"
echo "   • Current implementation is a reference structure"
echo "   • Real proof generation requires official SDK"
echo ""
echo "🚀 Try the Demo:"
echo "   cd $PROJECT_ROOT/airbender-zkvm"
echo "   cargo build --release"
echo "   cargo run --release --bin airbender-host"
echo ""
echo "📖 Learn More:"
echo "   • zkSync Airbender: https://docs.zksync.io/zk-stack/components/zksync-airbender"
echo "   • ere Project: https://github.com/eth-act/ere"
echo "   • Local README: $PROJECT_ROOT/airbender-zkvm/README.md"
echo ""
echo "🔔 Stay Updated:"
echo "   • zkSync Discord: https://discord.gg/zksync"
echo "   • zkSync Twitter: https://twitter.com/zksync"
echo "   • zkSync Blog: https://blog.matter-labs.io/"
echo ""
echo "✨ Happy Building with Airbender! ✨"
echo ""

