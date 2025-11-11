#!/bin/bash

# Cairo 1.0+ SDK Installation Script
# This script installs the Cairo 1.0+ toolchain using Scarb

set -e

# Target Cairo version
CAIRO_VERSION="2.12.0"
# Corresponding Scarb version (Scarb 2.8.5 includes Cairo 2.12.0)
SCARB_VERSION="2.8.5"

echo "======================================"
echo "Cairo 1.0+ SDK Installation (Scarb)"
echo "======================================"
echo "Target Cairo version: $CAIRO_VERSION"
echo "Installing Scarb version: $SCARB_VERSION"
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo "Detected OS: $MACHINE"
echo ""

# Check if curl is installed
if ! command -v curl &> /dev/null; then
    echo "❌ Error: curl is required but not installed."
    echo "Please install curl first."
    exit 1
fi

# Install Scarb (Cairo's package manager)
echo "Installing Scarb v${SCARB_VERSION} (includes Cairo v${CAIRO_VERSION})..."
echo ""

SCARB_INSTALL_SCRIPT="https://docs.swmansion.com/scarb/install.sh"

# Download and run Scarb installer with specific version
if curl --proto '=https' --tlsv1.2 -sSf "$SCARB_INSTALL_SCRIPT" | bash -s -- -v "$SCARB_VERSION"; then
    echo ""
    echo "✓ Scarb v${SCARB_VERSION} installed successfully!"
else
    echo "❌ Scarb installation failed."
    echo "Please check your internet connection and try again."
    echo "You can also try installing manually from: https://docs.swmansion.com/scarb/download"
    exit 1
fi

# Add Scarb to PATH for current session
export PATH="$HOME/.local/bin:$PATH"

# Verify Scarb installation
echo ""
echo "Verifying Scarb installation..."
if command -v scarb &> /dev/null; then
    SCARB_VERSION=$(scarb --version)
    echo "✓ Scarb is now available!"
    echo "  Version: $SCARB_VERSION"
else
    echo "⚠️  Scarb was installed but not found in PATH."
    echo "Please restart your terminal or run:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

# Check for Cairo compiler version
echo ""
echo "Verifying Cairo compiler version..."
if command -v scarb &> /dev/null; then
    INSTALLED_CAIRO_VERSION=$(scarb --version | grep -oE 'cairo: [0-9]+\.[0-9]+\.[0-9]+' | cut -d' ' -f2 || echo "unknown")
    if [ "$INSTALLED_CAIRO_VERSION" = "$CAIRO_VERSION" ]; then
        echo "✓ Cairo v${INSTALLED_CAIRO_VERSION} is correctly installed"
    else
        echo "⚠️  Installed Cairo version ($INSTALLED_CAIRO_VERSION) differs from target ($CAIRO_VERSION)"
    fi
fi

echo ""
echo "======================================"
echo "Cairo 1.0+ SDK Installation Complete!"
echo "======================================"
echo ""

# Add to shell configuration
echo "Adding Scarb to your shell configuration..."
SHELL_CONFIG=""
if [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
elif [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
fi

if [ -n "$SHELL_CONFIG" ] && [ -f "$SHELL_CONFIG" ]; then
    if ! grep -q 'export PATH="$HOME/.local/bin:$PATH"' "$SHELL_CONFIG"; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_CONFIG"
        echo "✓ Added Scarb to $SHELL_CONFIG"
    fi
fi

echo ""
echo "Quick Start - Create a new Cairo project:"
echo "  scarb new my_project"
echo "  cd my_project"
echo "  scarb build"
echo ""
echo "To run an existing Cairo project:"
echo "  cd cairo-zkvm"
echo "  scarb build"
echo "  scarb cairo-run"
echo ""
echo "Important: Restart your terminal or run:"
echo "  source ~/.bashrc  # or ~/.zshrc depending on your shell"
echo ""
echo "Installed versions:"
echo "  • Cairo: v${CAIRO_VERSION}"
echo "  • Scarb: v${SCARB_VERSION}"
echo ""
echo "Resources:"
echo "  • Official Installation Guide: https://www.cairo-lang.org/tutorial/getting-started-with-cairo/#installing-cairo"
echo "  • Cairo Book (1.0+): https://book.cairo-lang.org/"
echo "  • Scarb Documentation: https://docs.swmansion.com/scarb/"
echo "  • Cairo GitHub: https://github.com/starkware-libs/cairo"
echo "  • Cairo Playground: https://www.cairo-lang.org/playground/"
echo "  • Scarb Releases: https://docs.swmansion.com/scarb/download"
echo ""
echo "Note: This installs Cairo 1.0+ which has different syntax from Cairo 0.x"
echo "      Legacy Cairo 0.x can be installed with: pip install cairo-lang"
echo "      To change versions, edit CAIRO_VERSION and SCARB_VERSION in this script"
echo ""


