#!/bin/bash

# Airbender zkVM Demo Runner Script
# Convenient script to build and run the Airbender Fibonacci demo

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║     Airbender zkVM - Fibonacci Demo Runner               ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Get Fibonacci input from environment or use default
# Supports both FIBONACCI_N and FIB_N environment variables
FIB_N=${FIBONACCI_N:-${FIB_N:-10}}

# Parse command line arguments
BUILD_MODE="release"
CLEAN_BUILD=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            BUILD_MODE="debug"
            shift
            ;;
        --release)
            BUILD_MODE="release"
            shift
            ;;
        --clean)
            CLEAN_BUILD=true
            shift
            ;;
        --fib)
            FIB_N="$2"
            export FIBONACCI_N="$FIB_N"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --debug          Build in debug mode (default: release)"
            echo "  --release        Build in release mode"
            echo "  --clean          Clean build artifacts before building"
            echo "  --fib N          Set Fibonacci input to N (default: 10)"
            echo "  --help, -h       Show this help message"
            echo ""
            echo "Environment Variables:"
            echo "  FIBONACCI_N      Set the Fibonacci number to compute"
            echo "  FIB_N            Alternative to FIBONACCI_N"
            echo ""
            echo "Examples:"
            echo "  $0                         # Run with default settings"
            echo "  $0 --fib 15                # Compute fib(15)"
            echo "  FIBONACCI_N=20 $0          # Compute fib(20)"
            echo "  FIB_N=30 $0                # Compute fib(30)"
            echo "  $0 --debug --clean         # Clean build in debug mode"
            exit 0
            ;;
        *)
            echo "❌ Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Export the final FIBONACCI_N value
export FIBONACCI_N=$FIB_N

# Display configuration
echo "📊 Configuration:"
echo "   Input: n = $FIB_N"
echo "   Mode: $BUILD_MODE"
if [ "$CLEAN_BUILD" = true ]; then
    echo "   Clean build: yes"
fi
echo ""

# Clean build if requested
if [ "$CLEAN_BUILD" = true ]; then
    echo "🧹 Cleaning build artifacts..."
    cargo clean
    echo ""
fi

# Build the project
echo "🔨 Building Airbender zkVM demo ($BUILD_MODE mode)..."
if [ "$BUILD_MODE" = "release" ]; then
    cargo build --release
    BINARY_PATH="target/release/airbender-host"
else
    cargo build
    BINARY_PATH="target/debug/airbender-host"
fi
echo ""

# Run the demo
echo "🚀 Running Airbender zkVM demo..."
echo ""
echo "═══════════════════════════════════════════════════════════"
echo ""

if [ "$BUILD_MODE" = "release" ]; then
    cargo run --release --bin airbender-host
else
    cargo run --bin airbender-host
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "✅ Demo completed!"
echo ""

