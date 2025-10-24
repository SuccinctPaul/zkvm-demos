#!/bin/bash
# Quick start script for zkvm-demos Docker environment

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
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
echo "Available options:"
echo "  1) Nexus ZKVM"
echo "  2) Risc0 ZKVM" 
echo "  3) SP1 ZKVM"
echo "  4) ZKM ZKVM"
echo "  5) All ZKVMs"
echo "  6) Development Environment"
echo "  7) Build Base Image"
echo ""

read -p "Select an option (1-7): " choice

case $choice in
    1)
        echo -e "${GREEN}Building and running Nexus ZKVM...${NC}"
        docker-compose -f docker/docker-compose.yml --profile nexus up --build nexus-zkvm
        ;;
    2)
        echo -e "${GREEN}Building and running Risc0 ZKVM...${NC}"
        docker-compose -f docker/docker-compose.yml --profile risc0 up --build risc0-zkvm
        ;;
    3)
        echo -e "${GREEN}Building and running SP1 ZKVM...${NC}"
        docker-compose -f docker/docker-compose.yml --profile sp1 up --build sp1-zkvm
        ;;
    4)
        echo -e "${GREEN}Building and running ZKM ZKVM...${NC}"
        docker-compose -f docker/docker-compose.yml --profile zkm up --build zkm-zkvm
        ;;
    5)
        echo -e "${GREEN}Building and running all ZKVMs...${NC}"
        docker-compose -f docker/docker-compose.yml --profile all up --build
        ;;
    6)
        echo -e "${GREEN}Starting development environment...${NC}"
        echo "Available development environments:"
        echo "  a) Nexus Dev"
        echo "  b) Risc0 Dev"
        echo "  c) SP1 Dev"
        echo "  d) ZKM Dev"
        echo ""
        read -p "Select development environment (a-d): " dev_choice
        
        case $dev_choice in
            a)
                docker-compose -f docker/docker-compose.yml --profile dev up -d nexus-dev
                docker exec -it nexus-dev bash
                ;;
            b)
                docker-compose -f docker/docker-compose.yml --profile dev up -d risc0-dev
                docker exec -it risc0-dev bash
                ;;
            c)
                docker-compose -f docker/docker-compose.yml --profile dev up -d sp1-dev
                docker exec -it sp1-dev bash
                ;;
            d)
                docker-compose -f docker/docker-compose.yml --profile dev up -d zkm-dev
                docker exec -it zkm-dev bash
                ;;
            *)
                echo "❌ Invalid choice"
                exit 1
                ;;
        esac
        ;;
    7)
        echo -e "${GREEN}Building shared base image...${NC}"
        docker-compose -f docker/docker-compose.yml build zkvm-base
        ;;
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}✅ Done!${NC}"
