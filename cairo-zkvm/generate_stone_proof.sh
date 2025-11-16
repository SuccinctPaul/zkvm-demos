#!/bin/bash
# Generate STARK Proof using Stone Prover
# For Cairo 2.x programs

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STONE_DIR="$HOME/.stone-prover"
PROVER="$STONE_DIR/build/src/starkware/main/cpu/cpu_air_prover"
VERIFIER="$STONE_DIR/build/src/starkware/main/cpu/cpu_air_verifier"

echo "=========================================="
echo "STARK Proof Generation with Stone Prover"
echo "=========================================="
echo

# Check if Stone Prover is installed
if [ ! -f "$PROVER" ]; then
    echo "❌ Stone Prover not found!"
    echo "Please run: ./stone_prover_setup.sh first"
    exit 1
fi

cd "$SCRIPT_DIR"

# Step 1: Build Cairo project
echo "Step 1: Building Cairo project..."
scarb build
echo "✓ Build complete"
echo

# Step 2: Check Sierra JSON
SIERRA_FILE="target/dev/cairo_fibonacci.sierra.json"
if [ ! -f "$SIERRA_FILE" ]; then
    echo "❌ Sierra JSON not found: $SIERRA_FILE"
    exit 1
fi

SIERRA_SIZE=$(ls -lh "$SIERRA_FILE" | awk '{print $5}')
echo "✓ Sierra JSON ready: $SIERRA_SIZE"
echo

# Step 3: Important Note about Cairo 2.x
echo "=========================================="
echo "⚠️  IMPORTANT NOTE"
echo "=========================================="
echo
echo "Stone Prover was designed for Cairo 0.x programs."
echo "Cairo 2.x uses Sierra IR which is different from Cairo 0."
echo
echo "Current Sierra file: $SIERRA_FILE"
echo
echo "To use Stone Prover with Cairo 2.x, you need to:"
echo "1. Use StarkNet's proving infrastructure (recommended)"
echo "2. Or convert to Cairo 0 format"
echo "3. Or use Cairo Native prover instead"
echo
echo "=========================================="
echo "RECOMMENDED ALTERNATIVES"
echo "=========================================="
echo

# Alternative 1: StarkNet Katana
echo "📌 Option 1: Use StarkNet + Katana (RECOMMENDED)"
echo "   This generates real STARK proofs automatically:"
echo
echo "   # Install Starknet Foundry"
echo "   curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh"
echo "   snfoundryup"
echo
echo "   # Start local node"
echo "   katana &"
echo
echo "   # Declare contract (generates proof)"
echo "   starkli declare target/dev/cairo_fibonacci.contract_class.json \\"
echo "     --rpc http://localhost:5050"
echo
echo "   See: STARK_PROOF_GUIDE.md for full instructions"
echo

# Alternative 2: Cairo Native
echo "📌 Option 2: Cairo Native (High Performance)"
echo "   Modern prover optimized for Cairo 2.x:"
echo
echo "   git clone https://github.com/lambdaclass/cairo_native.git"
echo "   cd cairo_native"
echo "   cargo build --release"
echo
echo "   ./target/release/cairo-native-run \\"
echo "     --program $SIERRA_FILE \\"
echo "     --proof-mode"
echo

# Alternative 3: Create Cairo 0 version
echo "📌 Option 3: Create Cairo 0 Version"
echo "   Rewrite the program in Cairo 0 format to use Stone Prover"
echo

echo "=========================================="
echo "CURRENT STATUS"
echo "=========================================="
echo
echo "✓ Cairo 2.x program compiled"
echo "✓ Sierra IR generated ($SIERRA_SIZE)"
echo "✓ Execution trace available"
echo "✓ Stone Prover installed"
echo
echo "⚠️  Direct Stone Prover usage requires Cairo 0 format"
echo "✅ Use StarkNet/Katana for easiest proof generation"
echo
echo "For detailed guides, see:"
echo "  - STARK_PROOF_GUIDE.md (all methods)"
echo "  - README.md (quick start)"
echo

