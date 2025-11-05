#!/bin/bash
# Build shared ZKVM base image

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$(dirname "$SCRIPT_DIR")"
PROJECT_ROOT="$(dirname "$DOCKER_DIR")"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }

echo "🔧 Building ZKVM Base Image"
echo "============================"
echo ""

# Check Docker
if ! docker info &>/dev/null; then
    echo "❌ Docker is not running"
    exit 1
fi

info "Building shared base image with Rust and common dependencies..."

if docker build -f "$DOCKER_DIR/dockerfiles/Dockerfile.base" \
                -t zkvm-base:latest \
                "$PROJECT_ROOT"; then
    echo ""
    success "Base image built successfully!"
    echo ""
    docker images zkvm-base:latest --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}"
    echo ""
    info "Next steps:"
    echo "  cd $DOCKER_DIR"
    echo "  docker compose build sp1-zkvm    # or nexus-zkvm, risc0-zkvm, zkm-zkvm"
    echo ""
else
    echo "❌ Build failed"
    exit 1
fi
