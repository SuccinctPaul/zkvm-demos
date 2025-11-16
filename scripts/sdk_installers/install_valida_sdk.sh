#!/bin/bash
set -e

# --- Utility functions (duplicated) ---
# Checks if a tool is installed and available in PATH.
is_tool_installed() {
    command -v "$1" &> /dev/null
}

# Ensures a tool is installed. Exits with an error if not.
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
# --- End of Utility functions ---

echo "=========================================="
echo "Installing Valida zkVM Toolchain"
echo "=========================================="
echo ""
echo "Valida is a STARK-based zkVM with LLVM-based toolchain"
echo "that supports C/C++ and other LLVM-based languages."
echo ""
echo "Installation Options:"
echo "  1. Docker (Recommended) - Pull pre-built image"
echo "  2. Local Installation - Build from source (Advanced)"
echo ""
echo "Resources:"
echo "  • Official Site: https://www.lita.foundation/"
echo "  • Documentation: https://www.lita.foundation/blog/introducing-valida-zkvm-1-0"
echo "  • GitHub: https://github.com/litaio/valida"
echo ""
echo "=========================================="
echo ""

# Check prerequisites
echo "Checking prerequisites..."
ensure_tool_installed "curl" "to download installers"
ensure_tool_installed "bash" "to run installation scripts"

# Detect OS
OS_TYPE="$(uname -s)"
ARCH_TYPE="$(uname -m)"

echo "✓ Detected OS: ${OS_TYPE} (${ARCH_TYPE})"
echo ""

# Option 1: Check if Docker is available (recommended method)
if is_tool_installed "docker"; then
    echo "=========================================="
    echo "Option 1: Docker Installation (Recommended)"
    echo "=========================================="
    echo ""
    echo "Docker is available on your system."
    echo "You can use the official Valida Docker image:"
    echo ""
    echo "  docker pull lita-xyz/valida"
    echo ""
    echo "Usage examples:"
    echo "  # Compile C program"
    echo "  docker run --rm -v \$(pwd):/workspace lita-xyz/valida \\"
    echo "    valida-cc -o /workspace/output.elf /workspace/program.c"
    echo ""
    echo "  # Run in zkVM"
    echo "  docker run --rm -v \$(pwd):/workspace lita-xyz/valida \\"
    echo "    valida run /workspace/output.elf"
    echo ""
    echo "  # Generate proof"
    echo "  docker run --rm -v \$(pwd):/workspace lita-xyz/valida \\"
    echo "    valida prove /workspace/output.elf -o /workspace/proof.bin"
    echo ""
    echo "  # Verify proof"
    echo "  docker run --rm -v \$(pwd):/workspace lita-xyz/valida \\"
    echo "    valida verify /workspace/proof.bin"
    echo ""
    
    read -p "Do you want to pull the Valida Docker image now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Pulling Valida Docker image..."
        docker pull lita-xyz/valida
        echo ""
        echo "✓ Valida Docker image pulled successfully!"
        echo ""
        echo "You can now use Valida via Docker commands shown above."
        echo "See valida-zkvm/README.md for more examples."
        echo ""
        exit 0
    else
        echo "Skipping Docker image pull."
        echo "You can pull it later with: docker pull lita-xyz/valida"
        echo ""
    fi
fi

# Option 2: Local installation (for advanced users)
echo "=========================================="
echo "Option 2: Local Installation"
echo "=========================================="
echo ""
echo "⚠️  WARNING: Local installation is complex and requires:"
echo "  • LLVM 18.1.7 or higher"
echo "  • Rust 1.86 or higher"
echo "  • Build tools (make, cmake, etc.)"
echo "  • Ubuntu 24.04 or Arch Linux (officially supported)"
echo ""
echo "For other operating systems, using Docker is strongly recommended."
echo ""

if [[ "${OS_TYPE}" != "Linux" ]]; then
    echo "❌ Local installation is only officially supported on Linux."
    echo "   Your system: ${OS_TYPE}"
    echo ""
    echo "Recommendation: Use Docker instead."
    echo "   docker pull lita-xyz/valida"
    echo ""
    exit 1
fi

read -p "Do you want to attempt local installation? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Installation cancelled."
    echo ""
    echo "To use Valida via Docker:"
    echo "  docker pull lita-xyz/valida"
    echo ""
    exit 0
fi

echo ""
echo "Proceeding with local installation..."
echo ""

# Check for Rust
if ! is_tool_installed "rustc"; then
    echo "❌ Rust is not installed."
    echo "   Please install Rust first: https://rustup.rs/"
    exit 1
fi

RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "✓ Rust ${RUST_VERSION} detected"

# Check for LLVM
if ! is_tool_installed "llvm-config"; then
    echo "❌ LLVM is not installed or llvm-config is not in PATH."
    echo ""
    echo "To install LLVM 18:"
    echo ""
    echo "  Ubuntu/Debian:"
    echo "    wget https://apt.llvm.org/llvm.sh"
    echo "    chmod +x llvm.sh"
    echo "    sudo ./llvm.sh 18"
    echo ""
    echo "  Arch Linux:"
    echo "    sudo pacman -S llvm"
    echo ""
    echo "  macOS:"
    echo "    brew install llvm@18"
    echo "    export PATH=\"/opt/homebrew/opt/llvm@18/bin:\$PATH\""
    echo ""
    exit 1
fi

LLVM_VERSION=$(llvm-config --version)
echo "✓ LLVM ${LLVM_VERSION} detected"

# Clone and build Valida from source
echo ""
echo "Cloning Valida repository..."
TEMP_DIR=$(mktemp -d)
VALIDA_REPO="https://github.com/litaio/valida.git"

git clone "${VALIDA_REPO}" "${TEMP_DIR}/valida"
cd "${TEMP_DIR}/valida"

echo ""
echo "Building Valida toolchain (this may take 10-30 minutes)..."
echo "Building valida-cc (C compiler)..."

# Build the Valida C compiler
if ! cargo build --release --bin valida-cc; then
    echo "❌ Failed to build valida-cc"
    echo "   Please check the error messages above."
    echo "   For support, visit: https://github.com/litaio/valida"
    rm -rf "${TEMP_DIR}"
    exit 1
fi

echo ""
echo "Building valida CLI tools..."

# Build other Valida tools
if ! cargo build --release; then
    echo "❌ Failed to build Valida tools"
    echo "   Please check the error messages above."
    rm -rf "${TEMP_DIR}"
    exit 1
fi

# Install binaries to ~/.local/bin or ~/.cargo/bin
INSTALL_DIR="${HOME}/.local/bin"
if [ -d "${HOME}/.cargo/bin" ]; then
    INSTALL_DIR="${HOME}/.cargo/bin"
fi

mkdir -p "${INSTALL_DIR}"

echo ""
echo "Installing Valida binaries to ${INSTALL_DIR}..."

# Copy built binaries
cp target/release/valida "${INSTALL_DIR}/" || true
cp target/release/valida-cc "${INSTALL_DIR}/" || true

# Add to PATH if not already there
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo "Adding ${INSTALL_DIR} to PATH for current session."
    export PATH="${INSTALL_DIR}:$PATH"
fi

# Cleanup
cd /
rm -rf "${TEMP_DIR}"

# Verify installation
echo ""
echo "Verifying installation..."

if is_tool_installed "valida-cc"; then
    echo "✓ valida-cc installed successfully!"
    valida-cc --version || true
else
    echo "⚠️  valida-cc not found in PATH"
    echo "   Manually add ${INSTALL_DIR} to your PATH:"
    echo "   export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

if is_tool_installed "valida"; then
    echo "✓ valida CLI installed successfully!"
    valida --version || true
else
    echo "⚠️  valida not found in PATH"
fi

echo ""
echo "=========================================="
echo "Valida Installation Complete!"
echo "=========================================="
echo ""
echo "Installed tools:"
echo "  • valida-cc  - C/C++ compiler for Valida zkVM"
echo "  • valida     - Valida CLI (run, prove, verify)"
echo ""
echo "Quick Start:"
echo "  1. Navigate to valida-zkvm directory:"
echo "     cd valida-zkvm"
echo ""
echo "  2. Compile a C program:"
echo "     valida-cc -o fib.elf valida-guest/fib.c"
echo ""
echo "  3. Run in the zkVM:"
echo "     valida run fib.elf"
echo ""
echo "  4. Generate a proof:"
echo "     valida prove fib.elf -o proof.bin"
echo ""
echo "  5. Verify the proof:"
echo "     valida verify proof.bin"
echo ""
echo "Documentation:"
echo "  • Valida Docs: https://www.lita.foundation/blog/introducing-valida-zkvm-1-0"
echo "  • GitHub: https://github.com/litaio/valida"
echo "  • See valida-zkvm/README.md for examples"
echo ""
echo "Note: If commands are not found, add this to your shell profile:"
echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
echo ""
echo "Then restart your terminal or run:"
echo "  source ~/.bashrc  (or ~/.zshrc for Zsh)"
echo ""
echo "Happy proving with Valida! 🚀"

