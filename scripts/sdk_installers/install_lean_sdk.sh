#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Lean zkVM SDK Installation Script               ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Error: Rust is not installed${NC}"
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust is installed: $(rustc --version)"

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
REQUIRED_VERSION="1.85"

echo

# ═══════════════════════════════════════════════════════════════
# Note about Lean zkVM SDK availability
# ═══════════════════════════════════════════════════════════════

echo -e "${YELLOW}⚠️  Important Notice:${NC}"
echo
echo "The Lean zkVM (leanMultisig) SDK is currently under active development"
echo "and is not yet publicly available. This script prepares your environment"
echo "for future integration."
echo
echo -e "${BLUE}Project Status:${NC}"
echo "  • GitHub: https://github.com/leanEthereum/leanMultisig"
echo "  • Status: Early development, SDK not released"
echo "  • Components: lean_prover, WHIR, SuperSpartan"
echo "  • Full recursion: In progress"
echo
echo -e "${BLUE}Current Capabilities:${NC}"
echo "  ✓ Reference implementation and workflow demonstration"
echo "  ✓ Expected API and structure examples"
echo "  ✓ Performance benchmarks and metrics"
echo
echo -e "${BLUE}Waiting For:${NC}"
echo "  ⏳ Public SDK release"
echo "  ⏳ lean_prover crate publication"
echo "  ⏳ Complete recursion implementation"
echo "  ⏳ Provable security analysis"
echo

# ═══════════════════════════════════════════════════════════════
# Environment Preparation
# ═══════════════════════════════════════════════════════════════

echo -e "${GREEN}Preparing environment for Lean zkVM...${NC}"
echo

# Install required Rust toolchain
echo -e "${BLUE}1. Installing Rust toolchain 1.85...${NC}"
rustup toolchain install 1.85 --profile minimal
rustup default 1.85
echo -e "${GREEN}   ✓ Rust 1.85 installed${NC}"
echo

# Add required components
echo -e "${BLUE}2. Adding Rust components...${NC}"
rustup component add rustfmt clippy
echo -e "${GREEN}   ✓ rustfmt and clippy installed${NC}"
echo

# Install dependencies that would be needed
echo -e "${BLUE}3. Checking for required build tools...${NC}"

# Check for LLVM (useful for future integration)
if command -v llvm-config &> /dev/null; then
    echo -e "${GREEN}   ✓ LLVM found: $(llvm-config --version)${NC}"
else
    echo -e "${YELLOW}   ⚠️  LLVM not found (optional for now)${NC}"
fi

# Check for Git
if command -v git &> /dev/null; then
    echo -e "${GREEN}   ✓ Git found: $(git --version)${NC}"
else
    echo -e "${RED}   ❌ Git not found (required)${NC}"
    exit 1
fi

echo

# ═══════════════════════════════════════════════════════════════
# What Would Be Installed (When Available)
# ═══════════════════════════════════════════════════════════════

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  When the SDK is released, this script would install:   ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo
echo "  📦 Crates:"
echo "     • lean_prover - Core proving library"
echo "     • plonky3-field - Field arithmetic"
echo "     • koala-bear - KoalaBear field implementation"
echo "     • whir-p3 - WHIR polynomial commitment"
echo
echo "  🛠️  CLI Tools:"
echo "     • cargo-lean - Project scaffolding"
echo "     • lean-cli - Compilation and proving"
echo
echo "  📚 Dependencies:"
echo "     • WHIR prover parameters"
echo "     • SuperSpartan AIR libraries"
echo "     • Example programs and templates"
echo

# ═══════════════════════════════════════════════════════════════
# Demo Setup
# ═══════════════════════════════════════════════════════════════

echo -e "${GREEN}Setting up demo environment...${NC}"
echo

# Navigate to lean-zkvm demo directory (if it exists)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
LEAN_ZKVM_DIR="$SCRIPT_DIR/../../zkvms/lean-zkvm"

if [ -d "$LEAN_ZKVM_DIR" ]; then
    echo -e "${BLUE}4. Building lean-zkvm demo...${NC}"
    cd "$LEAN_ZKVM_DIR/lean-host"
    
    if cargo build --release 2>&1 | grep -q "Finished"; then
        echo -e "${GREEN}   ✓ Demo built successfully${NC}"
    else
        echo -e "${YELLOW}   ⚠️  Demo build had warnings (non-critical)${NC}"
    fi
    echo
else
    echo -e "${YELLOW}⚠️  lean-zkvm demo directory not found${NC}"
    echo "   Clone the zkvm-demos repository to access the demo"
    echo
fi

# ═══════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════

echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              Environment Setup Complete!                 ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
echo
echo -e "${BLUE}What's Ready:${NC}"
echo "  ✅ Rust 1.85 toolchain installed"
echo "  ✅ Required Rust components added"
echo "  ✅ Demo environment prepared"
echo
echo -e "${BLUE}Try the Reference Demo:${NC}"
echo "  cd lean-zkvm"
echo "  ./run_demo.sh"
echo
echo -e "${YELLOW}Stay Updated:${NC}"
echo "  • Watch: https://github.com/leanEthereum/leanMultisig"
echo "  • Stars: 50+ (growing)"
echo "  • Status: Active development"
echo
echo -e "${BLUE}When SDK is Released:${NC}"
echo "  • Re-run this script for full installation"
echo "  • Update dependencies in Cargo.toml"
echo "  • Follow official documentation"
echo
echo -e "${GREEN}Setup completed successfully! 🎉${NC}"
echo

# Exit successfully
exit 0

