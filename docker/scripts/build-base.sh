#!/bin/bash
# Build script for shared ZKVM base image

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

echo "🔧 Building Shared ZKVM Base Image"
echo "=================================="
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

print_info "Building shared base image with Rust and common dependencies..."

# Build the base image
if docker build -f docker/base/Dockerfile.base -t zkvm-base:latest ../..; then
    print_success "Shared base image built successfully!"
    echo ""
    
    # Show image information
    print_info "Base image details:"
    docker images zkvm-base:latest --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"
    echo ""
    
    print_info "You can now build individual ZKVM images:"
    echo "  ./production-manager.sh build nexus"
    echo "  ./production-manager.sh build risc0"
    echo "  ./production-manager.sh build sp1"
    echo "  ./production-manager.sh build zkm"
    echo "  ./production-manager.sh build all"
    echo ""
    echo "Or build toolchain images for development:"
    echo "  ./development-manager.sh build nexus"
    echo "  ./development-manager.sh build sp1"
    echo ""
    
    print_success "Base image is ready for use by all ZKVMs! 🎉"
else
    echo "❌ Failed to build base image"
    exit 1
fi



