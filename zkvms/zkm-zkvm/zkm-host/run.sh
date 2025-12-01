#!/bin/bash
# ZKM zkVM Run Script
# Sources the toolchain environment and runs the benchmark

set -e

# Source ZKM toolchain environment
if [ -f "$HOME/.zkm-toolchain/env" ]; then
    source "$HOME/.zkm-toolchain/env"
fi

echo "Running ZKM benchmark..."
cargo run --release

echo "Run completed"
