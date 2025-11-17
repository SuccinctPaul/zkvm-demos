#!/bin/bash
# Stwo-Cairo Demo Script
# This script demonstrates zero-knowledge proof generation using Stwo-Cairo prover

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Stwo-Cairo zkVM Demo - Fibonacci Proof Generation${NC}"
echo "=================================================="

# Function to print colored status
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC}  $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check prerequisites
echo ""
echo "📋 Checking prerequisites..."

# Check cairo-prove
if ! command -v cairo-prove &> /dev/null; then
    print_error "cairo-prove not found"
    echo ""
    echo "To install cairo-prove:"
    echo "  1. Clone the repository:"
    echo "     git clone https://github.com/starkware-libs/stwo-cairo.git"
    echo ""
    echo "  2. Build and install:"
    echo "     cd stwo-cairo/cairo-prove"
    echo "     ./build.sh"
    echo "     sudo cp target/release/cairo-prove /usr/local/bin/"
    echo ""
    echo "See STWO_INTEGRATION_GUIDE.md for detailed instructions"
    exit 1
fi
print_status "cairo-prove installed"

# Check Scarb version
SCARB_VERSION=$(scarb --version | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1)
REQUIRED_VERSION="2.10.0"

# Simple version comparison
version_greater_equal() {
    [ "$(printf '%s\n' "$1" "$2" | sort -V | head -n1)" == "$2" ]
}

if version_greater_equal "$SCARB_VERSION" "$REQUIRED_VERSION"; then
    print_status "Scarb version $SCARB_VERSION (≥ $REQUIRED_VERSION)"
else
    print_warning "Scarb version $SCARB_VERSION < $REQUIRED_VERSION"
    echo "   Stwo-Cairo requires Scarb ≥ 2.10.0"
    echo "   Install with: asdf install scarb latest:nightly"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if Stwo-compatible files exist
if [ ! -f "Scarb.stwo.toml" ]; then
    print_error "Scarb.stwo.toml not found"
    echo "Please create the Stwo-Cairo compatible configuration first."
    echo "See STWO_INTEGRATION_GUIDE.md for details"
    exit 1
fi

if [ ! -f "src/lib.stwo.cairo" ]; then
    print_error "src/lib.stwo.cairo not found"
    echo "Please create the Stwo-Cairo compatible source code first."
    echo "See STWO_INTEGRATION_GUIDE.md for details"
    exit 1
fi

# Backup original files and use Stwo versions
echo ""
echo "📦 Preparing Stwo-Cairo compatible files..."

# Backup original files if they exist
if [ -f "Scarb.toml" ] && [ ! -f "Scarb.toml.backup" ]; then
    cp Scarb.toml Scarb.toml.backup
    print_status "Backed up original Scarb.toml"
fi

if [ -f "src/lib.cairo" ] && [ ! -f "src/lib.cairo.backup" ]; then
    cp src/lib.cairo src/lib.cairo.backup
    print_status "Backed up original src/lib.cairo"
fi

# Use Stwo versions
cp Scarb.stwo.toml Scarb.toml
cp src/lib.stwo.cairo src/lib.cairo
print_status "Using Stwo-Cairo compatible files"

# Clean previous build
echo ""
echo "🧹 Cleaning previous build..."
rm -rf target/
print_status "Cleaned target directory"

# Build the project
echo ""
echo "🔨 Building project..."
if scarb build; then
    print_status "Build successful"
else
    print_error "Build failed"
    
    # Restore original files
    if [ -f "Scarb.toml.backup" ]; then
        mv Scarb.toml.backup Scarb.toml
    fi
    if [ -f "src/lib.cairo.backup" ]; then
        mv src/lib.cairo.backup src/lib.cairo
    fi
    exit 1
fi

# Check if executable was generated
EXECUTABLE="target/dev/cairo_fibonacci.executable.json"
if [ ! -f "$EXECUTABLE" ]; then
    print_error "Executable not found at $EXECUTABLE"
    
    # Restore original files
    if [ -f "Scarb.toml.backup" ]; then
        mv Scarb.toml.backup Scarb.toml
    fi
    if [ -f "src/lib.cairo.backup" ]; then
        mv src/lib.cairo.backup src/lib.cairo
    fi
    exit 1
fi

print_status "Executable generated: $EXECUTABLE"

# Generate proof for different Fibonacci numbers
echo ""
echo "🔐 Generating zero-knowledge proofs..."

# Test cases
TEST_CASES=(5 10 15 20)

for N in "${TEST_CASES[@]}"; do
    echo ""
    echo "  Testing Fibonacci($N)..."
    
    PROOF_FILE="stwo_proof_fib_${N}.json"
    
    # Generate proof
    if cairo-prove prove \
        "$EXECUTABLE" \
        "./$PROOF_FILE" \
        --arguments "$N" 2>&1 | tee /tmp/cairo_prove_output.log; then
        
        # Extract timing information
        PROVE_TIME=$(grep "completed in" /tmp/cairo_prove_output.log | grep -o '[0-9.]\+s' | tail -1)
        
        print_status "Proof generated in ${PROVE_TIME:-N/A}"
        
        # Verify proof
        if cairo-prove verify "./$PROOF_FILE" 2>&1 | tee /tmp/cairo_verify_output.log; then
            print_status "Proof verified successfully"
            
            # Get proof size
            PROOF_SIZE=$(ls -lh "$PROOF_FILE" | awk '{print $5}')
            print_status "Proof size: $PROOF_SIZE"
        else
            print_error "Proof verification failed"
        fi
    else
        print_error "Proof generation failed"
    fi
done

# Restore original files
echo ""
echo "🔄 Restoring original files..."
if [ -f "Scarb.toml.backup" ]; then
    mv Scarb.toml.backup Scarb.toml
    print_status "Restored original Scarb.toml"
fi

if [ -f "src/lib.cairo.backup" ]; then
    mv src/lib.cairo.backup src/lib.cairo
    print_status "Restored original src/lib.cairo"
fi

# Summary
echo ""
echo "=================================================="
echo -e "${GREEN}✨ Demo completed successfully!${NC}"
echo ""
echo "Generated proofs:"
for N in "${TEST_CASES[@]}"; do
    PROOF_FILE="stwo_proof_fib_${N}.json"
    if [ -f "$PROOF_FILE" ]; then
        SIZE=$(ls -lh "$PROOF_FILE" | awk '{print $5}')
        echo "  - $PROOF_FILE ($SIZE)"
    fi
done
echo ""
echo "To verify a proof again:"
echo "  cairo-prove verify stwo_proof_fib_10.json"
echo ""
echo "To clean up proof files:"
echo "  rm -f stwo_proof_fib_*.json"
echo "=================================================="

