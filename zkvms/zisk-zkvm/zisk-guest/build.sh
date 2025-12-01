#!/bin/bash
# ZisK zkVM Build Script
# Builds the guest program for ZisK RISC-V target

set -e

# Setup PATH for cargo-zisk
export PATH="$HOME/.zisk/bin:$PATH"

echo "Building ZisK guest program..."
cargo-zisk build --release

echo "Build completed successfully"
