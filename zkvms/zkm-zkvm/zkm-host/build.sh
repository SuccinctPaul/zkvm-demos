#!/bin/bash
# ZKM zkVM Build Script
# Sources the toolchain environment and builds the project

set -e

# Source ZKM toolchain environment
if [ -f "$HOME/.zkm-toolchain/env" ]; then
    source "$HOME/.zkm-toolchain/env"
fi

echo "Building ZKM host program..."
cargo build --release

echo "Build completed successfully"
