#!/bin/bash
# ZisK zkVM Benchmark Runner
# Generates input.bin, executes, generates proof, and verifies
#
# Note: Proof generation only supported on Linux x86_64
# On macOS, this script automatically uses Docker to run in a Linux environment

set -e

# Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

# ================================================================
# macOS: Automatically use Docker
# ================================================================
if [[ "$OS" == "Darwin" ]]; then
    echo "=========================================="
    echo "🍎 macOS detected - using Docker for proof generation"
    echo "=========================================="
    
    # Get the project root (two levels up from zisk-guest)
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
    DOCKER_DIR="$PROJECT_ROOT/docker"
    
    # Check if Docker is running
    if ! docker info > /dev/null 2>&1; then
        echo "❌ Error: Docker is not running. Please start Docker and try again."
        exit 1
    fi
    
    # Check if zisk-zkvm image exists
    if ! docker image inspect zisk-zkvm:latest > /dev/null 2>&1; then
        echo "📦 ZisK Docker image not found. Building it first..."
        echo "   This may take 15-20 minutes on first run."
        echo ""
        
        # Build base image if needed
        if ! docker image inspect zkvm-base:latest > /dev/null 2>&1; then
            echo "📦 Building base image first..."
            cd "$DOCKER_DIR/scripts"
            ./build-base.sh
        fi
        
        # Build zisk image
        cd "$DOCKER_DIR"
        docker compose build zisk-zkvm
        echo ""
        echo "✅ Docker image built successfully"
    fi
    
    # Export environment variables for Docker
    export FIBONACCI_N="${FIBONACCI_N:-${PROGRAM_N:-10}}"
    export PROGRAM_N="${PROGRAM_N:-${FIBONACCI_N:-10}}"
    export PROGRAM_ID="${PROGRAM_ID:-0}"
    export ZKVM_MODE="prove"
    
    echo ""
    echo "🐳 Running ZisK in Docker container..."
    echo "   FIBONACCI_N=$FIBONACCI_N"
    echo "   PROGRAM_N=$PROGRAM_N"
    echo "   PROGRAM_ID=$PROGRAM_ID"
    echo ""
    
    # Run in Docker with environment variables
    # Use docker run directly for better control and output capture
    cd "$PROJECT_ROOT"
    docker run --rm \
        --platform linux/amd64 \
        -v "$PROJECT_ROOT:/workspace" \
        -v zisk-cargo-cache:/usr/local/cargo/registry \
        -v zisk-target-cache:/workspace/target \
        -v zisk-zisk-cache:/root/.zisk \
        -w /workspace/zisk-zkvm/zisk-guest \
        -e "FIBONACCI_N=$FIBONACCI_N" \
        -e "PROGRAM_N=$PROGRAM_N" \
        -e "PROGRAM_ID=$PROGRAM_ID" \
        -e "RUST_LOG=${RUST_LOG:-debug}" \
        zisk-zkvm:latest \
        bash -c "./run_benchmark.sh"
    
    exit $?
fi

# ================================================================
# Linux: Native execution
# ================================================================

# Get parameters from environment
PROGRAM_ID=${PROGRAM_ID:-0}
N=${PROGRAM_N:-${FIBONACCI_N:-10}}
ZISK_MODE=${ZISK_MODE:-prove}  # Options: run, prove

# Paths
ELF_PATH="target/riscv64ima-zisk-zkvm-elf/release/zisk-guest"
INPUT_PATH="build/input.bin"
PROOF_DIR="build/proof"

# Create directories
mkdir -p build
mkdir -p "$PROOF_DIR"

# Generate input.bin with two little-endian u32 values: [program_id, n]
python3 -c "
import struct
import sys
program_id = int(sys.argv[1])
n = int(sys.argv[2])
with open('build/input.bin', 'wb') as f:
    f.write(struct.pack('<II', program_id, n))
print(f'BENCHMARK: program_name=fibonacci_{n}')
print(f'BENCHMARK: zkvm_name=zisk')
print(f'BENCHMARK: zkvm_version=v0.10.0')
print(f'BENCHMARK: proof_mode=core')
print(f'Generated input.bin: program_id={program_id}, n={n}')
" "$PROGRAM_ID" "$N"

# Setup PATH for cargo-zisk (in case not in PATH)
export PATH="$HOME/.zisk/bin:$PATH"

# Step 1: Build guest program (if needed)
if [[ ! -f "$ELF_PATH" ]]; then
    echo "=========================================="
    echo "Step 0: Building guest program..."
    echo "=========================================="
    cargo-zisk build --release
fi

# Step 2: Execute program
echo "=========================================="
echo "Step 1: Executing program..."
echo "=========================================="
EXEC_START=$(python3 -c "import time; print(time.time())")
cargo-zisk run --release -i "$INPUT_PATH"
EXEC_END=$(python3 -c "import time; print(time.time())")
EXEC_TIME=$(python3 -c "print(f'{$EXEC_END - $EXEC_START:.6f}')")
echo "BENCHMARK: execution_time_s=$EXEC_TIME"

# Step 3: Proof generation (Linux x86_64 only)
if [[ "$OS" == "Linux" && "$ARCH" == "x86_64" ]]; then
    echo ""
    echo "=========================================="
    echo "Step 2: Generating zero-knowledge proof..."
    echo "=========================================="
    
    # Generate proof with timing
    PROVE_START=$(python3 -c "import time; print(time.time())")
    cargo-zisk prove -e "$ELF_PATH" -i "$INPUT_PATH" -o "$PROOF_DIR" -a -y 2>&1 | tee /tmp/zisk_prove.log
    PROVE_END=$(python3 -c "import time; print(time.time())")
    PROVE_TIME=$(python3 -c "print(f'{$PROVE_END - $PROVE_START:.6f}')")
    echo "BENCHMARK: proof_time_s=$PROVE_TIME"
    
    # Get proof size
    if [[ -f "$PROOF_DIR/vadcop_final_proof.bin" ]]; then
        PROOF_SIZE=$(stat -c%s "$PROOF_DIR/vadcop_final_proof.bin" 2>/dev/null || stat -f%z "$PROOF_DIR/vadcop_final_proof.bin" 2>/dev/null || echo "0")
        echo "BENCHMARK: proof_size_bytes=$PROOF_SIZE"
    fi
    
    echo ""
    echo "=========================================="
    echo "Step 3: Verifying proof..."
    echo "=========================================="
    
    # Verify proof with timing
    VERIFY_START=$(python3 -c "import time; print(time.time())")
    if cargo-zisk verify -p "$PROOF_DIR/vadcop_final_proof.bin" 2>&1; then
        VERIFY_END=$(python3 -c "import time; print(time.time())")
        VERIFY_TIME=$(python3 -c "print(f'{$VERIFY_END - $VERIFY_START:.6f}')")
        echo "BENCHMARK: verification_time_s=$VERIFY_TIME"
        echo "BENCHMARK: success_status=success"
    else
        echo "BENCHMARK: success_status=failed"
    fi
else
    echo ""
    echo "=========================================="
    echo "⚠️  Proof generation not supported on $OS $ARCH"
    echo "    Only Linux x86_64 is supported for ZisK proving"
    echo "    Execution completed successfully"
    echo "=========================================="
    echo "BENCHMARK: success_status=execution_only"
fi

echo ""
echo "=========================================="
echo "Benchmark completed"
echo "=========================================="
