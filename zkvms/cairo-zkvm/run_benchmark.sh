#!/bin/bash
# Cairo zkVM Benchmark Script with Stwo-Cairo STARK Proof Generation
# Default prover: Stwo-Cairo (cairo-prove)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Get input from environment or use default
PROGRAM_ID="${PROGRAM_ID:-0}"
# Support both PROGRAM_N and FIBONACCI_N (benchmark framework uses FIBONACCI_N for Fibonacci)
PROGRAM_N="${PROGRAM_N:-${FIBONACCI_N:-10}}"

echo "==================================================="
echo "Cairo zkVM Benchmark with Stwo-Cairo Prover"
echo "==================================================="
echo "Program ID: $PROGRAM_ID"
echo "Input N: $PROGRAM_N"
echo ""

# Output metadata early
echo "BENCHMARK: zkvm_name=cairo"
echo "BENCHMARK: zkvm_version=v2.8.0"
echo "BENCHMARK: proof_mode=core"
echo "BENCHMARK: prover_backend=stwo-cairo"

# Helper function for timing
get_time() {
    python3 -c "import time; print(time.time())"
}

# Check if cairo-prove (Stwo-Cairo) is installed
if command -v cairo-prove &> /dev/null; then
    PROOF_MODE="stwo"
    CAIRO_PROVE_VERSION=$(cairo-prove --version 2>&1 | head -1 || echo "unknown")
    echo "BENCHMARK: prover_version=$CAIRO_PROVE_VERSION"
    echo "✓ Stwo-Cairo prover detected"
    USE_STWO_CONFIG=true
else
    echo "⚠️  cairo-prove not found, running execution-only mode"
    echo "   To install Stwo-Cairo, run: ./install_stwo.sh"
    PROOF_MODE="execution_only"
    USE_STWO_CONFIG=false
fi

# Step 1: Setup configuration
echo ""
echo "Step 1: Setting up configuration..."

# Backup and restore function
cleanup() {
    if [ -f "Scarb.toml.bak" ]; then
        mv "Scarb.toml.bak" "Scarb.toml"
    fi
    if [ -f "src/lib.cairo.bak" ]; then
        mv "src/lib.cairo.bak" "src/lib.cairo"
    fi
}
trap cleanup EXIT

if [ "$USE_STWO_CONFIG" = true ]; then
    # Use Stwo configuration
    if [ -f "Scarb.stwo.toml" ]; then
        cp "Scarb.toml" "Scarb.toml.bak"
        cp "Scarb.stwo.toml" "Scarb.toml"
        echo "   ✓ Using Stwo configuration"
    fi
    
    if [ -f "src/lib.stwo.cairo" ]; then
        cp "src/lib.cairo" "src/lib.cairo.bak"
        cp "src/lib.stwo.cairo" "src/lib.cairo"
        echo "   ✓ Using Stwo-compatible source"
    fi
else
    # For execution-only mode, dynamically modify the source to use PROGRAM_N
    cp "src/lib.cairo" "src/lib.cairo.bak"
    
    # Update the main function to use the correct N and PROGRAM_ID values
    sed -i.tmp "s/let n: felt252 = [0-9]*;/let n: felt252 = $PROGRAM_N;/" "src/lib.cairo"
    sed -i.tmp "s/let program_id: felt252 = [0-9]*;/let program_id: felt252 = $PROGRAM_ID;/" "src/lib.cairo"
    rm -f "src/lib.cairo.tmp"
    echo "   ✓ Configured for N=$PROGRAM_N, ID=$PROGRAM_ID"
fi

# Step 2: Build
echo ""
echo "Step 2: Building Cairo program..."
BUILD_START=$(get_time)

scarb clean 2>/dev/null || true
BUILD_OUTPUT=$(scarb build 2>&1)
echo "$BUILD_OUTPUT"

BUILD_END=$(get_time)
BUILD_TIME=$(python3 -c "print(f'{$BUILD_END - $BUILD_START:.6f}')")
echo "BENCHMARK: compile_time_s=$BUILD_TIME"
echo "✓ Build complete"

# Check compiled artifacts
if [ "$USE_STWO_CONFIG" = true ]; then
    EXECUTABLE_FILE="target/dev/cairo_fibonacci.executable.json"
else
    EXECUTABLE_FILE="target/dev/cairo_fibonacci.sierra.json"
fi

if [ -f "$EXECUTABLE_FILE" ]; then
    FILE_SIZE=$(stat -f%z "$EXECUTABLE_FILE" 2>/dev/null || stat -c%s "$EXECUTABLE_FILE" 2>/dev/null)
    echo "BENCHMARK: executable_size_bytes=$FILE_SIZE"
    echo "   ✓ Executable: $EXECUTABLE_FILE ($FILE_SIZE bytes)"
fi

# Step 3: Prove or Execute
if [ "$PROOF_MODE" = "stwo" ]; then
    echo ""
    echo "Step 3: Generating STARK proof with Stwo-Cairo..."
    PROVE_START=$(get_time)
    
    mkdir -p output
    
    # Run cairo-prove with arguments
    if cairo-prove prove \
        "$EXECUTABLE_FILE" \
        output/proof.json \
        --arguments "$PROGRAM_N" 2>&1; then
        
        PROVE_END=$(get_time)
        PROVE_TIME=$(python3 -c "print(f'{$PROVE_END - $PROVE_START:.6f}')")
        
        if [ -f "output/proof.json" ]; then
            PROOF_SIZE=$(stat -f%z "output/proof.json" 2>/dev/null || stat -c%s "output/proof.json" 2>/dev/null)
            echo "BENCHMARK: proof_time_s=$PROVE_TIME"
            echo "BENCHMARK: proof_size_bytes=$PROOF_SIZE"
            echo "✓ Proof generated ($PROOF_SIZE bytes)"
            
            # Extract result
            if command -v jq &> /dev/null; then
                RESULT=$(jq -r '.public_input.output[0] // .output[0] // empty' output/proof.json 2>/dev/null)
                if [ -n "$RESULT" ]; then
                    echo "BENCHMARK: output_result=$RESULT"
                fi
            fi
            
            # Step 4: Verify proof
            echo ""
            echo "Step 4: Verifying STARK proof..."
            VERIFY_START=$(get_time)
            
            if cairo-prove verify output/proof.json 2>&1; then
                VERIFY_END=$(get_time)
                VERIFY_TIME=$(python3 -c "print(f'{$VERIFY_END - $VERIFY_START:.6f}')")
                echo "BENCHMARK: verification_time_s=$VERIFY_TIME"
                echo "BENCHMARK: success_status=success"
                echo "✓ Proof verified"
            else
                echo "BENCHMARK: success_status=verification_failed"
            fi
        else
            echo "BENCHMARK: success_status=proof_failed"
        fi
    else
        echo "⚠️  Proof generation failed, falling back to execution"
        PROOF_MODE="execution_only"
    fi
fi

# Execution-only mode
if [ "$PROOF_MODE" = "execution_only" ]; then
    echo ""
    echo "Step 3: Executing Cairo program..."
    EXEC_START=$(get_time)
    
    EXEC_OUTPUT=$(scarb cairo-run --available-gas=200000000 2>&1)
    EXEC_STATUS=$?
    
    EXEC_END=$(get_time)
    EXEC_TIME=$(python3 -c "print(f'{$EXEC_END - $EXEC_START:.6f}')")
    
    echo "$EXEC_OUTPUT"
    echo ""
    echo "BENCHMARK: execution_time_s=$EXEC_TIME"
    
    # Parse output
    if echo "$EXEC_OUTPUT" | grep -q "returning"; then
        RESULT=$(echo "$EXEC_OUTPUT" | grep -o 'returning \[[0-9]*\]' | grep -o '[0-9]*')
        echo "BENCHMARK: output_result=$RESULT"
    fi
    
    if echo "$EXEC_OUTPUT" | grep -q "Remaining gas"; then
        REMAINING_GAS=$(echo "$EXEC_OUTPUT" | grep -o 'Remaining gas: [0-9]*' | grep -o '[0-9]*')
        INITIAL_GAS=200000000
        GAS_USED=$((INITIAL_GAS - REMAINING_GAS))
        echo "BENCHMARK: gas_used=$GAS_USED"
        echo "BENCHMARK: total_cycles=$GAS_USED"
    fi
    
    if [ $EXEC_STATUS -eq 0 ] && echo "$EXEC_OUTPUT" | grep -q "Run completed successfully"; then
        echo "BENCHMARK: success_status=success"
    else
        echo "BENCHMARK: success_status=failed"
    fi
fi

# Calculate total time
TOTAL_TIME=$(python3 -c "print(f'{$BUILD_TIME + ${EXEC_TIME:-0} + ${PROVE_TIME:-0}:.6f}')")

echo ""
echo "BENCHMARK: program_name=fibonacci_$PROGRAM_N"
echo "BENCHMARK: total_time_s=$TOTAL_TIME"

echo ""
echo "==================================================="
echo "Cairo Benchmark Complete"
echo "==================================================="
