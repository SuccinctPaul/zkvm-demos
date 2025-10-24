#!/bin/bash
# Quick start script for zkvm-demos Docker environment
# Uses the new production-manager.sh and development-manager.sh scripts

set -e

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🚀 ZKVM Demos Docker Quick Start${NC}"
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose > /dev/null 2>&1; then
    echo "❌ docker-compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "✅ Docker is running"
echo ""

# Show available options
echo "=== Production Mode (Precompiled - Fast Start) ==="
echo "  1) Nexus ZKVM"
echo "  2) Risc0 ZKVM" 
echo "  3) SP1 ZKVM"
echo "  4) ZKM ZKVM"
echo "  5) All ZKVMs"
echo ""
echo "=== Development Mode (Toolchain - Flexible) ==="
echo "  6) Nexus ZKVM"
echo "  7) Risc0 ZKVM"
echo "  8) SP1 ZKVM"
echo "  9) ZKM ZKVM"
echo ""
echo "=== Other Options ==="
echo "  10) Build Base Image"
echo "  11) Clean Everything"
echo ""

read -p "Select an option (1-11): " choice

case $choice in
    # Production Mode
    1)
        echo -e "${GREEN}Building and running Nexus ZKVM (Production)...${NC}"
        "$SCRIPT_DIR/production-manager.sh" build nexus
        "$SCRIPT_DIR/production-manager.sh" run nexus
        ;;
    2)
        echo -e "${GREEN}Building and running Risc0 ZKVM (Production)...${NC}"
        "$SCRIPT_DIR/production-manager.sh" build risc0
        "$SCRIPT_DIR/production-manager.sh" run risc0
        ;;
    3)
        echo -e "${GREEN}Building and running SP1 ZKVM (Production)...${NC}"
        "$SCRIPT_DIR/production-manager.sh" build sp1
        "$SCRIPT_DIR/production-manager.sh" run sp1
        ;;
    4)
        echo -e "${GREEN}Building and running ZKM ZKVM (Production)...${NC}"
        "$SCRIPT_DIR/production-manager.sh" build zkm
        "$SCRIPT_DIR/production-manager.sh" run zkm
        ;;
    5)
        echo -e "${GREEN}Building and running all ZKVMs (Production)...${NC}"
        "$SCRIPT_DIR/production-manager.sh" build all
        "$SCRIPT_DIR/production-manager.sh" run all
        ;;
    
    # Development Mode (Toolchain)
    6)
        echo -e "${GREEN}Nexus ZKVM (Development Mode)${NC}"
        echo "Enter arguments (default: --nocapture, or press Enter for interactive shell):"
        read -p "Args: " args
        if [ -z "$args" ]; then
            "$SCRIPT_DIR/development-manager.sh" build nexus
            "$SCRIPT_DIR/development-manager.sh" shell nexus
        else
            "$SCRIPT_DIR/development-manager.sh" build nexus
            "$SCRIPT_DIR/development-manager.sh" run nexus $args
        fi
        ;;
    7)
        echo -e "${GREEN}Risc0 ZKVM (Development Mode)${NC}"
        echo "Enter arguments (or press Enter for interactive shell):"
        read -p "Args: " args
        if [ -z "$args" ]; then
            "$SCRIPT_DIR/development-manager.sh" build risc0
            "$SCRIPT_DIR/development-manager.sh" shell risc0
        else
            "$SCRIPT_DIR/development-manager.sh" build risc0
            "$SCRIPT_DIR/development-manager.sh" run risc0 $args
        fi
        ;;
    8)
        echo -e "${GREEN}SP1 ZKVM (Development Mode)${NC}"
        echo "Enter arguments (--execute or --prove, or press Enter for interactive shell):"
        read -p "Args: " args
        if [ -z "$args" ]; then
            "$SCRIPT_DIR/development-manager.sh" build sp1
            "$SCRIPT_DIR/development-manager.sh" shell sp1
        else
            "$SCRIPT_DIR/development-manager.sh" build sp1
            "$SCRIPT_DIR/development-manager.sh" run sp1 $args
        fi
        ;;
    9)
        echo -e "${GREEN}ZKM ZKVM (Development Mode)${NC}"
        echo "Enter arguments (default: --execute, or press Enter for interactive shell):"
        read -p "Args: " args
        if [ -z "$args" ]; then
            "$SCRIPT_DIR/development-manager.sh" build zkm
            "$SCRIPT_DIR/development-manager.sh" shell zkm
        else
            "$SCRIPT_DIR/development-manager.sh" build zkm
            "$SCRIPT_DIR/development-manager.sh" run zkm $args
        fi
        ;;
    
    # Other Options
    10)
        echo -e "${GREEN}Building shared base image...${NC}"
        "$SCRIPT_DIR/build-base.sh"
        ;;
    11)
        echo -e "${YELLOW}⚠️  This will clean all Docker images and containers${NC}"
        read -p "Are you sure? (y/N): " confirm
        if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
            "$SCRIPT_DIR/production-manager.sh" clean
            "$SCRIPT_DIR/development-manager.sh" clean-cache
        else
            echo "Cancelled"
        fi
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}✅ Done!${NC}"
echo ""
echo -e "${BLUE}Tip:${NC} You can also use the manager scripts directly:"
echo "  Production: $SCRIPT_DIR/production-manager.sh [command] [zkvm]"
echo "  Development: $SCRIPT_DIR/development-manager.sh [command] [zkvm]"
