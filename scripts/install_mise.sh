#!/bin/bash
set -e

echo "=========================================="
echo "Installing mise - Modern Toolchain Manager"
echo "=========================================="
echo ""

# Detect OS
OS_TYPE="$(uname -s)"
ARCH_TYPE="$(uname -m)"

echo "Detected OS: ${OS_TYPE} (${ARCH_TYPE})"
echo ""

# Check if mise is already installed
if command -v mise &> /dev/null; then
    echo "✓ mise is already installed!"
    mise --version
    echo ""
    read -p "Do you want to update mise to the latest version? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Skipping mise installation."
        exit 0
    fi
fi

# Install mise based on OS
echo "Installing mise..."
echo ""

case "${OS_TYPE}" in
    Darwin*)
        # macOS
        if command -v brew &> /dev/null; then
            echo "Installing via Homebrew..."
            brew install mise
        else
            echo "Homebrew not found. Installing mise via standalone installer..."
            curl https://mise.run | sh
        fi
        ;;
    Linux*)
        # Linux
        echo "Installing mise via standalone installer..."
        curl https://mise.run | sh
        ;;
    *)
        echo "Unsupported OS: ${OS_TYPE}"
        echo "Please visit https://mise.jdx.dev/getting-started.html for manual installation"
        exit 1
        ;;
esac

echo ""
echo "✓ mise installed successfully!"
echo ""

# Setup shell integration
echo "=========================================="
echo "Setting Up Shell Integration"
echo "=========================================="
echo ""

SHELL_NAME=$(basename "$SHELL")
SHELL_RC=""

case "${SHELL_NAME}" in
    zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    bash)
        if [ -f "$HOME/.bashrc" ]; then
            SHELL_RC="$HOME/.bashrc"
        else
            SHELL_RC="$HOME/.bash_profile"
        fi
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        ;;
    *)
        echo "Unknown shell: ${SHELL_NAME}"
        echo "Please manually add mise activation to your shell config"
        echo ""
        echo "For bash/zsh, add:"
        echo '  eval "$(mise activate bash)"  # or zsh'
        echo ""
        exit 0
        ;;
esac

echo "Detected shell: ${SHELL_NAME}"
echo "Shell config: ${SHELL_RC}"
echo ""

# Check if mise is already activated
if grep -q "mise activate" "${SHELL_RC}" 2>/dev/null; then
    echo "✓ mise is already activated in ${SHELL_RC}"
else
    echo "Adding mise activation to ${SHELL_RC}..."
    
    # Backup shell config
    cp "${SHELL_RC}" "${SHELL_RC}.backup.$(date +%Y%m%d_%H%M%S)"
    
    # Add mise activation
    if [ "${SHELL_NAME}" = "fish" ]; then
        echo "" >> "${SHELL_RC}"
        echo "# mise activation" >> "${SHELL_RC}"
        echo "mise activate fish | source" >> "${SHELL_RC}"
    else
        echo "" >> "${SHELL_RC}"
        echo "# mise activation" >> "${SHELL_RC}"
        echo "eval \"\$(mise activate ${SHELL_NAME})\"" >> "${SHELL_RC}"
    fi
    
    echo "✓ mise activation added to ${SHELL_RC}"
    echo "  (Backup created at ${SHELL_RC}.backup.*)"
fi

echo ""
echo "=========================================="
echo "mise Installation Complete!"
echo "=========================================="
echo ""
echo "Next Steps:"
echo ""
echo "1. Activate mise in your current shell:"
if [ "${SHELL_NAME}" = "fish" ]; then
    echo "   source ${SHELL_RC}"
else
    echo "   source ${SHELL_RC}"
fi
echo ""
echo "2. Navigate to the project root:"
echo "   cd zkvm-demos"
echo ""
echo "3. Trust and install tools:"
echo "   mise trust"
echo "   mise install"
echo ""
echo "4. Verify installation:"
echo "   mise doctor"
echo "   mise run check-tools"
echo ""
echo "5. Try automatic switching:"
echo "   cd jolt-zkvm     # Uses Rust 1.88"
echo "   rustc --version"
echo "   cd ../sp1-zkvm   # Uses Rust nightly"
echo "   rustc --version"
echo ""
echo "Documentation:"
echo "  • mise Guide: MISE_SETUP.md"
echo "  • Official Docs: https://mise.jdx.dev/"
echo "  • GitHub: https://github.com/jdx/mise"
echo ""
echo "Note: You need to restart your terminal or run 'source ${SHELL_RC}'"
echo "      for mise to be available in your current session."
echo ""
echo "Happy multi-toolchain development! 🚀"

