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

echo "Installing ZisK Toolchain and SDK using ziskup (prebuilt binaries)..."

# Prerequisites for ziskup and ZisK (some of these are for the SDK itself beyond ziskup)
ensure_tool_installed "curl" "to download the ziskup installer"
ensure_tool_installed "bash" "to run the ziskup installer"
ensure_tool_installed "rustup" "for managing Rust toolchains (ZisK installs its own)"
ensure_tool_installed "cargo" "as cargo-zisk is a cargo subcommand"

# Step 1: Download and run ziskup directly (not install.sh) to control architecture
# Export SETUP_KEY=proving to ensure no interactive options in `ziskup`.
export ZISK_VERSION="0.10.0"
export SETUP_KEY=${SETUP_KEY:=proving}

# Create .zisk/bin directory
ZISK_DIR="${HOME}/.zisk"
ZISK_BIN_DIR="${ZISK_DIR}/bin"
mkdir -p "${ZISK_BIN_DIR}"

# Download ziskup binary directly
echo "Downloading ziskup..."
curl -# -L "https://raw.githubusercontent.com/0xPolygonHermez/zisk/main/ziskup/ziskup" -o "${ZISK_BIN_DIR}/ziskup"
chmod +x "${ZISK_BIN_DIR}/ziskup"

# Detect platform and architecture
# ZisK only provides: darwin_arm64 and linux_amd64
# For Docker on Apple Silicon, we need to force linux + amd64
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

ZISKUP_ARGS=""
if [ "$OS" = "linux" ]; then
    # On Linux, always use amd64 (ZisK doesn't provide linux_arm64)
    ZISKUP_ARGS="--platform linux --arch amd64"
    echo "Detected Linux - using linux_amd64 binaries"
elif [ "$OS" = "darwin" ]; then
    # On macOS, use darwin_arm64 (only available option for macOS)
    ZISKUP_ARGS="--platform darwin --arch arm64"
    echo "Detected macOS - using darwin_arm64 binaries"
fi

echo "Running ziskup with args: ${ZISKUP_ARGS}"
"${ZISK_BIN_DIR}/ziskup" --version "v${ZISK_VERSION#v}" ${ZISKUP_ARGS}
unset SETUP_KEY

# Step 2: Ensure the installed cargo-zisk binary is in PATH for this script session.
export PATH="${PATH}:${HOME}/.zisk/bin"

# Verify ZisK installation
echo "Verifying ZisK installation..."

echo "Checking for 'zisk' toolchain..."
if rustup toolchain list | grep -q "^zisk"; then
    echo "ZisK Rust toolchain found."
else
    echo "Error: ZisK Rust toolchain ('zisk') not found after installation!" >&2
    exit 1
fi

echo "Checking for cargo-zisk CLI tool..."
if cargo-zisk --version; then
    echo "cargo-zisk CLI tool verified successfully."
else
    echo "Error: 'cargo-zisk --version' failed." >&2
    exit 1
fi

# Step 3: Build cargo-zisk-gpu from source with GPU features enabled (skip if in CI)
if [ -z $CI ]; then
    TEMP_DIR=$(mktemp -d)
    git clone https://github.com/0xPolygonHermez/zisk.git --single-branch --branch "v$ZISK_VERSION" "$TEMP_DIR/zisk"
    cd "$TEMP_DIR/zisk"
    cargo build --release --features gpu
    cp ./target/release/cargo-zisk "${HOME}/.zisk/bin/cargo-zisk-gpu"
    cp ./target/release/libzisk_witness.so "${HOME}/.zisk/bin/libzisk_witness_gpu.so"
    rm -rf "$TEMP_DIR"

    echo "Checking for cargo-zisk-gpu CLI tool..."
    if cargo-zisk-gpu --version; then
        echo "cargo-zisk-gpu CLI tool verified successfully."
    else
        echo "Error: 'cargo-zisk-gpu --version' failed." >&2
        exit 1
    fi
fi

# Step 4: Make sure `lib-c`'s build script is ran.
#
# `ziskos` provides guest program runtime, and `lib-c` is a dependency of `ziskos`,
# when we need to compile guest, the `build.rs` of `lib-c` will need to be ran once,
# but if there are multiple `build.rs` running at the same time, it will panic.
# So here we make sure it's already ran, and the built thing will be stored in
# `$CARGO_HOME/git/checkouts/zisk-{hash}/{rev}/lib-c/c/build`, so could be
# re-used as long as the `ziskos` has the same version.
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"
cargo init . --name build-lib-c
cargo add lib-c --git https://github.com/0xPolygonHermez/zisk.git --tag "v${ZISK_VERSION}"
cargo build
rm -rf "$TEMP_DIR"
