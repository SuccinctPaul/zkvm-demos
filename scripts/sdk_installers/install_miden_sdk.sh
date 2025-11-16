#!/bin/bash

# Miden VM SDK Installation Script
# 
# This script installs the necessary dependencies for Miden VM development.
# Unlike other zkVMs, Miden VM doesn't have a separate SDK installer.
# It's distributed as Rust crates via Cargo.

set -e

echo "======================================"
echo "  Miden VM Setup"
echo "======================================"
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed!"
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✓ Rust is installed"
rustc --version

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed!"
    exit 1
fi

echo "✓ Cargo is installed"
cargo --version

# Install/update to required Rust version
REQUIRED_RUST_VERSION="1.83.0"
echo ""
echo "Installing Rust toolchain ${REQUIRED_RUST_VERSION}..."
rustup install "${REQUIRED_RUST_VERSION}"
rustup default "${REQUIRED_RUST_VERSION}"

echo ""
echo "======================================"
echo "  Installation Summary"
echo "======================================"
echo ""
echo "Miden VM dependencies are installed via Cargo.toml"
echo "No additional SDK installation is required."
echo ""
echo "The following crates will be used:"
echo "  - miden-vm: Core Miden VM runtime and proving"
echo "  - miden-assembly: Miden Assembly compiler"
echo "  - miden-processor: VM execution engine"
echo "  - miden-stdlib: Standard library for Miden"
echo ""
echo "To build your Miden project:"
echo "  cd miden-zkvm/miden-host"
echo "  cargo build --release"
echo ""
echo "✓ Miden VM setup completed successfully!"
echo ""

