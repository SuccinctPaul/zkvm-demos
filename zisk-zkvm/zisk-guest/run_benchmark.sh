#!/bin/bash
# ZisK zkVM Benchmark Runner
# Generates input.bin, executes, generates proof, and verifies
#
# Note: Proof generation only supported on Linux x86_64
# On macOS, only execution will run

set -e

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

# Step 1: Execute program
echo "=========================================="
echo "Step 1: Executing program..."
echo "=========================================="
EXEC_START=$(python3 -c "import time; print(time.time())")
cargo-zisk run --release -i "$INPUT_PATH"
EXEC_END=$(python3 -c "import time; print(time.time())")
EXEC_TIME=$(python3 -c "print(f'{$EXEC_END - $EXEC_START:.6f}')")
echo "BENCHMARK: execution_time_s=$EXEC_TIME"

# Check platform for proof generation
OS=$(uname -s)
ARCH=$(uname -m)

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
