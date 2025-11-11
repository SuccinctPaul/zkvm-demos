#!/bin/bash
set -e

echo "=========================================="
echo "Installing Cairo 2.x Toolchain (Scarb)"
echo "=========================================="
echo ""

# Common utility functions
is_tool_installed() {
    command -v "$1" &> /dev/null
}

ensure_tool_installed() {
    local tool_name="$1"
    local purpose_message="$2"

    if ! is_tool_installed "${tool_name}"; then
        echo "Error: Required tool '${tool_name}' could not be found." >&2
        if [ -n "${purpose_message}" ]; then
            echo "       It is needed ${purpose_message}." >&2
        fi
        echo "       Please install it first and ensure it is in your PATH." >&2
        exit 1
    fi
}

echo "Checking prerequisites..."
ensure_tool_installed "curl" "to download the Scarb installer"
ensure_tool_installed "bash" "to run installation scripts"

echo "✓ Prerequisites satisfied"
echo ""

# Install Scarb (Cairo 2.x package manager and compiler)
echo "Installing Scarb (Cairo 2.x Package Manager)..."
echo "Scarb is the official build toolchain and package manager for Cairo 2.x"
echo ""

if is_tool_installed "scarb"; then
    echo "Scarb is already installed:"
    scarb --version
    echo ""
    read -p "Do you want to update Scarb to the latest version? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Skipping Scarb installation."
        echo "✓ Cairo toolchain already installed"
        exit 0
    fi
fi

echo "Downloading and installing Scarb..."
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh

# Add Scarb to PATH for current session
SCARB_BIN_DIR="$HOME/.local/bin"
if [ -d "${SCARB_BIN_DIR}" ] && [[ ":$PATH:" != *":${SCARB_BIN_DIR}:"* ]]; then
    echo "Adding ${SCARB_BIN_DIR} to PATH for current session."
    export PATH="${SCARB_BIN_DIR}:$PATH"
fi

echo ""
echo "Verifying installation..."

if is_tool_installed "scarb"; then
    echo "✓ Scarb installed successfully!"
    scarb --version
    echo ""
    
    # Check for Cairo compiler
    if is_tool_installed "cairo-compile"; then
        echo "✓ Cairo compiler available!"
        cairo-compile --version
    fi
    
    if is_tool_installed "cairo-run"; then
        echo "✓ Cairo runner available!"
        cairo-run --version
    fi
else
    echo "❌ Error: Scarb installation failed or not found in PATH." >&2
    echo "   Please check if ${SCARB_BIN_DIR} is in your PATH." >&2
    echo "   You may need to restart your terminal or run:" >&2
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\"" >&2
    exit 1
fi

echo ""
echo "=========================================="
echo "Cairo 2.x Installation Complete!"
echo "=========================================="
echo ""
echo "Installed tools:"
echo "  • scarb      - Package manager and build tool"
echo "  • cairo-*    - Cairo compiler and tooling"
echo ""
echo "Quick Start:"
echo "  1. Navigate to cairo-zkvm directory:"
echo "     cd cairo-zkvm"
echo ""
echo "  2. Build the project:"
echo "     scarb build"
echo ""
echo "  3. Run tests:"
echo "     scarb test"
echo ""
echo "  4. Run the program:"
echo "     scarb cairo-run --available-gas=200000000"
echo ""
echo "Documentation:"
echo "  • Cairo Book:   https://book.cairo-lang.org/"
echo "  • Scarb Docs:   https://docs.swmansion.com/scarb/"
echo "  • StarkNet:     https://docs.starknet.io/"
echo ""
echo "Note: If commands are not found, add this to your shell profile:"
echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
echo ""
echo "Then restart your terminal or run:"
echo "  source ~/.bashrc  (or ~/.zshrc for Zsh)"
echo ""
echo "Happy coding with Cairo 2.x! 🚀"
