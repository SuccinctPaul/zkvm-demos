#!/bin/bash
set -e

echo "=========================================="
echo "Verifying mise Setup for zkvm-demos"
echo "=========================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if mise is installed
echo "1. Checking mise installation..."
if command -v mise &> /dev/null; then
    echo -e "${GREEN}✓${NC} mise is installed"
    mise --version
else
    echo -e "${RED}✗${NC} mise is not installed"
    echo "   Run: ./scripts/install_mise.sh"
    exit 1
fi
echo ""

# Check if mise is activated
echo "2. Checking mise activation..."
if [ -n "$MISE_SHELL" ]; then
    echo -e "${GREEN}✓${NC} mise is activated in shell: $MISE_SHELL"
else
    echo -e "${YELLOW}⚠${NC}  mise is not activated in current shell"
    echo "   Add to your shell config (~/.zshrc or ~/.bashrc):"
    echo "   eval \"\$(mise activate bash)\"  # or zsh"
    echo ""
    echo "   Then run: source ~/.zshrc"
fi
echo ""

# Check for .mise.toml files
echo "3. Checking mise configuration files..."
ZKVM_DIRS=(
    "."
    "jolt-zkvm"
    "cairo-zkvm"
    "cairo-m-zkvm"
    "risc0-zkvm"
    "sp1-zkvm"
    "valida-zkvm"
    "nexus-zkvm"
    "openvm-zkvm"
    "powdr-zkvm"
    "zkm-zkvm"
    "miden-zkvm"
    "pico-zkvm"
    "ceno-zkvm"
    "zisk-zkvm"
    "zkwasm-zkvm"
    "airbender-zkvm"
    "lean-zkvm"
    "novanet-zkvm"
    "o1vm-zkvm"
)

CONFIG_COUNT=0
for dir in "${ZKVM_DIRS[@]}"; do
    if [ -f "$dir/.mise.toml" ]; then
        CONFIG_COUNT=$((CONFIG_COUNT + 1))
        echo -e "${GREEN}✓${NC} $dir/.mise.toml"
    else
        echo -e "${YELLOW}⚠${NC}  $dir/.mise.toml (not found)"
    fi
done
echo ""
echo "Found $CONFIG_COUNT/.mise.toml configuration files"
echo ""

# Check if tools are defined
echo "4. Checking defined tools in root .mise.toml..."
if [ -f ".mise.toml" ]; then
    echo "Tools defined:"
    grep -A 10 '\[tools\]' .mise.toml | grep -v '\[' | grep '=' || echo "  (none)"
else
    echo -e "${RED}✗${NC} Root .mise.toml not found"
fi
echo ""

# Test automatic switching
echo "5. Testing automatic toolchain switching..."
echo ""

CURRENT_DIR=$(pwd)

# Test root directory
echo "Testing root directory..."
cd "${CURRENT_DIR}"
if command -v mise &> /dev/null && [ -n "$MISE_SHELL" ]; then
    RUST_VERSION=$(mise current rust 2>/dev/null || echo "not set")
    echo "  Root: rust = ${RUST_VERSION}"
else
    echo "  (Skipped - mise not activated)"
fi

# Test jolt-zkvm
if [ -d "jolt-zkvm" ] && [ -f "jolt-zkvm/.mise.toml" ]; then
    echo "Testing jolt-zkvm..."
    cd "${CURRENT_DIR}/jolt-zkvm"
    if command -v mise &> /dev/null && [ -n "$MISE_SHELL" ]; then
        RUST_VERSION=$(mise current rust 2>/dev/null || echo "not set")
        echo "  jolt-zkvm: rust = ${RUST_VERSION}"
    else
        echo "  (Skipped - mise not activated)"
    fi
fi

# Test cairo-zkvm
if [ -d "cairo-zkvm" ] && [ -f "cairo-zkvm/.mise.toml" ]; then
    echo "Testing cairo-zkvm..."
    cd "${CURRENT_DIR}/cairo-zkvm"
    if command -v mise &> /dev/null && [ -n "$MISE_SHELL" ]; then
        PYTHON_VERSION=$(mise current python 2>/dev/null || echo "not set")
        echo "  cairo-zkvm: python = ${PYTHON_VERSION}"
    else
        echo "  (Skipped - mise not activated)"
    fi
fi

cd "${CURRENT_DIR}"
echo ""

# Check .gitignore
echo "6. Checking .gitignore..."
if grep -q ".mise.local.toml" .gitignore 2>/dev/null; then
    echo -e "${GREEN}✓${NC} mise entries found in .gitignore"
else
    echo -e "${YELLOW}⚠${NC}  mise entries not found in .gitignore"
    echo "   Add these lines to .gitignore:"
    echo "   .mise.local.toml"
    echo "   .mise/"
    echo "   .mise.lock"
fi
echo ""

# Summary
echo "=========================================="
echo "Verification Summary"
echo "=========================================="
echo ""

if command -v mise &> /dev/null; then
    if [ -n "$MISE_SHELL" ]; then
        echo -e "${GREEN}✓ mise is installed and activated${NC}"
        echo ""
        echo "Next steps:"
        echo "  1. Trust mise configurations: mise trust"
        echo "  2. Install tools: mise install"
        echo "  3. Check tools: mise run check-tools"
        echo "  4. Test switching: cd jolt-zkvm && rustc --version"
    else
        echo -e "${YELLOW}⚠ mise is installed but not activated${NC}"
        echo ""
        echo "To activate mise:"
        echo "  1. Add to ~/.zshrc (or ~/.bashrc):"
        echo "     eval \"\$(mise activate zsh)\"  # or bash"
        echo "  2. Restart terminal or: source ~/.zshrc"
        echo "  3. Run this script again to verify"
    fi
else
    echo -e "${RED}✗ mise is not installed${NC}"
    echo ""
    echo "To install mise:"
    echo "  ./scripts/install_mise.sh"
fi

echo ""
echo "Documentation:"
echo "  • MISE_SETUP.md - Complete guide"
echo "  • https://mise.jdx.dev/ - Official docs"
echo ""

