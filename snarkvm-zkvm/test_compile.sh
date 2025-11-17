#!/bin/bash

# Quick compile test for snarkVM demo

set -e

cd "$(dirname "$0")/snarkvm-host"

echo "Testing snarkVM demo compilation..."
echo "This may take a few minutes on first run..."
echo ""

# Try to compile
if cargo build --release 2>&1 | tee /tmp/snarkvm_build.log; then
    echo ""
    echo "✅ Compilation successful!"
    exit 0
else
    echo ""
    echo "❌ Compilation failed. Check /tmp/snarkvm_build.log for details"
    exit 1
fi

