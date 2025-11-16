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
echo "Installing CENO zkVM Toolchain"
echo "=========================================="
echo ""
echo "NOTE: CENO zkVM by Scroll is under active development."
echo "This installer sets up Nexus zkVM as the current implementation,"
echo "which provides real zero-knowledge proof generation capabilities."
echo ""
echo "Once the official CENO SDK is released by Scroll, you can migrate"
echo "by updating the dependencies in your project."
echo ""
echo "Resources:"
echo "  - CENO Paper: https://eprint.iacr.org/2024/387"
echo "  - Scroll Blog: https://scroll.io/blog/ceno"
echo "  - Expected Repo: https://github.com/scroll-tech/ceno"
echo "  - Current Implementation: Nexus zkVM"
echo ""
echo "=========================================="
echo ""

# Prerequisites for Nexus (used by CENO demo)
ensure_tool_installed "rustup" "for managing Rust toolchains"
ensure_tool_installed "cargo" "as cargo-nexus is a cargo subcommand"

# Default versions (can be overridden via environment variables)
NEXUS_TOOLCHAIN_VERSION="${NEXUS_TOOLCHAIN_VERSION:-nightly-2025-04-06}"
NEXUS_CLI_VERSION_TAG="${NEXUS_CLI_VERSION_TAG:-v0.3.4}"

echo "Installing Nexus zkVM (for CENO demo)..."
echo "Using Nexus toolchain: ${NEXUS_TOOLCHAIN_VERSION}"
echo "Using Nexus CLI version tag: ${NEXUS_CLI_VERSION_TAG}"
echo ""

# Install the Nexus CLI
echo "Step 1: Installing Nexus CLI from GitHub repository..."
cargo "+${NEXUS_TOOLCHAIN_VERSION}" install --git https://github.com/nexus-xyz/nexus-zkvm cargo-nexus --tag "$NEXUS_CLI_VERSION_TAG"

echo ""
echo "Step 2: Installing RISC-V target for Nexus..."
# Install Nexus's target
rustup "+${NEXUS_TOOLCHAIN_VERSION}" target add riscv32i-unknown-none-elf

echo ""
echo "Step 3: Verifying installation..."
# Verify Nexus installation
if cargo-nexus --version; then
    echo "✓ Nexus CLI installed successfully!"
else
    echo "Error: 'cargo-nexus --version' failed. Nexus CLI might not have installed correctly." >&2
    echo "       Ensure ${HOME}/.cargo/bin is in your PATH for new shells." >&2
    exit 1
fi

echo ""
if rustup "+${NEXUS_TOOLCHAIN_VERSION}" target list --installed | grep -q "riscv32i-unknown-none-elf"; then
    echo "✓ RISC-V target installed successfully!"
else
    echo "Error: RISC-V target not installed correctly." >&2
    exit 1
fi

echo ""
echo "=========================================="
echo "CENO zkVM Setup Complete!"
echo "=========================================="
echo ""
echo "What's installed:"
echo "  • Nexus zkVM CLI (cargo-nexus)"
echo "  • RISC-V target (riscv32i-unknown-none-elf)"
echo "  • Rust toolchain: ${NEXUS_TOOLCHAIN_VERSION}"
echo ""
echo "Quick Start:"
echo "  1. Navigate to ceno-zkvm directory:"
echo "     cd ceno-zkvm"
echo ""
echo "  2. Run the Fibonacci demo:"
echo "     RUST_LOG=info cargo run --release --bin ceno-host"
echo ""
echo "  3. Expected output:"
echo "     - Compilation of guest program to RISC-V"
echo "     - Real zero-knowledge proof generation"
echo "     - Proof verification"
echo "     - Performance metrics"
echo ""
echo "Migration Path:"
echo "  When Scroll releases the official CENO SDK:"
echo "  1. Update Cargo.toml dependencies (nexus-sdk → ceno-sdk)"
echo "  2. Update guest/host code to use CENO APIs"
echo "  3. Update rust-toolchain.toml if needed"
echo "  4. Run tests and benchmarks"
echo ""
echo "Documentation:"
echo "  • See ceno-zkvm/README.md for detailed information"
echo "  • CENO Paper: https://eprint.iacr.org/2024/387"
echo "  • Nexus Docs: https://docs.nexus.xyz/"
echo ""
echo "Happy proving with CENO zkVM! 🚀"

