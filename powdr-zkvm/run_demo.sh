#!/bin/bash
# Powdr zkVM Demo Runner
# Convenient script to run the Fibonacci demo with Powdr zkVM

set -e

echo "=========================================="
echo "  Powdr zkVM - Fibonacci Demo"
echo "=========================================="
echo ""

# Check if Powdr is installed (optional for this reference implementation)
if ! command -v powdr &> /dev/null; then
    echo "⚠️  Powdr CLI not found in PATH"
    echo ""
    echo "This is a reference implementation that simulates the Powdr zkVM workflow."
    echo "To install the actual Powdr CLI (optional):"
    echo "  cd scripts/sdk_installers"
    echo "  ./install_powdr_sdk.sh"
    echo ""
    echo "Proceeding with demo..."
    echo ""
fi

# Get Fibonacci input from environment or use default
FIB_N=${FIBONACCI_N:-${FIB_N:-10}}

echo "📊 Configuration:"
echo "   Input: n = ${FIB_N}"
echo "   Set FIBONACCI_N or FIB_N to change"
echo ""

# Navigate to host directory
cd "$(dirname "$0")/powdr-host"

# Run the demo
echo "🚀 Running Powdr zkVM demo..."
echo ""

RUST_LOG=${RUST_LOG:-info} FIBONACCI_N=${FIB_N} cargo run --release

echo ""
echo "✅ Demo completed!"
echo ""
echo "💡 Try with different inputs:"
echo "   FIBONACCI_N=15 ./run_demo.sh"
echo "   FIBONACCI_N=100 ./run_demo.sh"
echo ""

