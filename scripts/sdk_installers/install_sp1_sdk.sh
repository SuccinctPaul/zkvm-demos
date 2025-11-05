#!/bin/bash
# Disable exit on error temporarily for retry logic
set +e

echo "Installing Succinct SP1 Toolchain..."

# Ensure prerequisites like curl are there
if ! command -v curl &> /dev/null; then
    echo "Error: curl could not be found, please install it first." >&2
    exit 1
fi
if ! command -v bash &> /dev/null; then # sp1up script uses bash
    echo "Error: bash could not be found, please install it first." >&2
    exit 1
fi

# Define default homes if not set, useful for Docker context
DEFAULT_SP1_DIR="${HOME}/.sp1"

# Use existing ENV var or default. Docker ENV will make these available.
# For local use, user might need to add these to their .bashrc/.zshrc
export SP1_DIR="${SP1_DIR:-${DEFAULT_SP1_DIR}}"

# Run sp1up installer script
curl -L https://sp1up.succinct.xyz | bash

# Add sp1up and sp1 binaries to PATH for this script's execution context
# and for subsequent commands if this script is sourced.
export PATH="${SP1_DIR}/bin:$PATH"

export SP1_VERSION="${SP1_VERSION:-latest}"

# Run sp1up to install/update the toolchain
if ! command -v sp1up &> /dev/null; then
    echo "Error: sp1up command not found after installation script. Check PATH or installation." >&2
    exit 1
fi

# Install with retry logic for network issues
echo "Installing SP1 toolchain (this may take several minutes, please be patient)..."
echo "Note: Large files are being downloaded, this is normal."

MAX_RETRIES=5
RETRY_COUNT=0
INSTALL_SUCCESS=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    echo "Attempt $((RETRY_COUNT + 1)) of $MAX_RETRIES..."
    
    # Try to install with timeout
    if timeout 900 sp1up -v ${SP1_VERSION}; then
        echo "✅ SP1 toolchain installed successfully!"
        INSTALL_SUCCESS=1
        break
    else
        EXIT_CODE=$?
        RETRY_COUNT=$((RETRY_COUNT + 1))
        
        if [ $EXIT_CODE -eq 124 ]; then
            echo "⏱️  Installation timed out after 15 minutes"
        else
            echo "❌ Installation failed with exit code $EXIT_CODE"
        fi
        
        if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
            WAIT_TIME=$((5 * RETRY_COUNT))
            echo "Waiting ${WAIT_TIME} seconds before retry..."
            sleep $WAIT_TIME
            
            # Clean up partial installation
            echo "Cleaning up partial installation..."
            rm -rf ~/.sp1/toolchains/* || true
        else
            echo ""
            echo "❌ Error: Failed to install SP1 toolchain after $MAX_RETRIES attempts." >&2
            echo "This is likely due to network issues or slow connection." >&2
            echo ""
            echo "You can try one of the following:" >&2
            echo "  1. Rebuild the image later when network is more stable" >&2
            echo "  2. Enter the container and manually run: sp1up -v latest" >&2
            echo "  3. Use the host machine's SP1 installation with volume mounting" >&2
            exit 1
        fi
    fi
done

if [ $INSTALL_SUCCESS -eq 0 ]; then
    echo "Failed to install SP1 toolchain"
    exit 1
fi

echo "Verifying SP1 installation..."
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo command not found. Ensure Rust is installed and in PATH." >&2
    exit 1 # cargo prove needs cargo
fi

cargo prove --version
rustup toolchain list | grep succinct || (echo "Error: SP1 Toolchain (succinct) not found after install!" >&2 && exit 1)

echo "Succinct SP1 Toolchain installation successful."
echo "If running locally (not in Docker), to make SP1 commands available in your current shell or new shells, ensure the following are in your shell profile (e.g., ~/.bashrc, ~/.zshrc):"
echo "  export SP1_DIR=\"${SP1_DIR}\""
echo "  export PATH=\"${SP1_DIR}/bin:\$PATH\""
echo "Then source your profile or open a new terminal."
