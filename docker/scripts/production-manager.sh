#!/bin/bash
# Production Environment Manager
# Manages precompiled Docker images for fast deployment

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_usage() {
    echo "Production Environment Manager"
    echo "Purpose: Manage precompiled ZKVM images for production deployment"
    echo ""
    echo "Usage: $0 [COMMAND] [ZKVM]"
    echo ""
    echo "Commands:"
    echo "  build [zkvm]     Build precompiled image"
    echo "  run [zkvm]       Run precompiled image"
    echo "  stop [zkvm]      Stop running container"
    echo "  logs [zkvm]      View container logs"
    echo "  ps               View running containers"
    echo "  clean            Clean images and containers"
    echo ""
    echo "ZKVMs: nexus, risc0, sp1, zkm, all"
    echo ""
    echo "Examples:"
    echo "  $0 build sp1            # Build SP1 precompiled image"
    echo "  $0 run sp1              # Run SP1 (production mode)"
    echo "  $0 build all            # Build all ZKVM images"
    echo "  $0 stop sp1             # Stop SP1 container"
    echo ""
    echo "Features:"
    echo "  ✅ Fast startup (precompiled in image)"
    echo "  ✅ Suitable for production deployment and CI/CD"
    echo "  ✅ Fixed commands, stable and reliable"
}

# Check and build base image if needed
check_base_image() {
    if ! docker image inspect zkvm-base:latest > /dev/null 2>&1; then
        print_warning "Base image zkvm-base:latest not found, building..."
        docker-compose -f ../base/docker-compose.yml build zkvm-base
        print_success "Base image built successfully"
    else
        print_info "Base image zkvm-base:latest exists"
    fi
}

build_image() {
    local zkvm=$1
    
    # Ensure base image exists
    check_base_image
    
    case $zkvm in
        "nexus")
            print_info "Building Nexus ZKVM precompiled image..."
            docker-compose -f ../production/docker-compose.yml build nexus-zkvm
            print_success "Nexus ZKVM image built successfully"
            ;;
        "risc0")
            print_info "Building Risc0 ZKVM precompiled image..."
            docker-compose -f ../production/docker-compose.yml build risc0-zkvm
            print_success "Risc0 ZKVM image built successfully"
            ;;
        "sp1")
            print_info "Building SP1 ZKVM precompiled image..."
            docker-compose -f ../production/docker-compose.yml build sp1-zkvm
            print_success "SP1 ZKVM image built successfully"
            ;;
        "zkm")
            print_info "Building ZKM ZKVM precompiled image..."
            docker-compose -f ../production/docker-compose.yml build zkm-zkvm
            print_success "ZKM ZKVM image built successfully"
            ;;
        "all")
            print_info "Building all ZKVM precompiled images..."
            docker-compose -f ../production/docker-compose.yml build nexus-zkvm risc0-zkvm sp1-zkvm zkm-zkvm
            print_success "All ZKVM images built successfully"
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

run_image() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Running Nexus ZKVM (production mode)..."
            docker-compose -f ../production/docker-compose.yml --profile nexus up nexus-zkvm
            ;;
        "risc0")
            print_info "Running Risc0 ZKVM (production mode)..."
            docker-compose -f ../production/docker-compose.yml --profile risc0 up risc0-zkvm
            ;;
        "sp1")
            print_info "Running SP1 ZKVM (production mode)..."
            docker-compose -f ../production/docker-compose.yml --profile sp1 up sp1-zkvm
            ;;
        "zkm")
            print_info "Running ZKM ZKVM (production mode)..."
            docker-compose -f ../production/docker-compose.yml --profile zkm up zkm-zkvm
            ;;
        "all")
            print_info "Running all ZKVMs (production mode)..."
            docker-compose -f ../production/docker-compose.yml --profile all up
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

stop_container() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Stopping Nexus container..."
            docker-compose -f ../production/docker-compose.yml stop nexus-zkvm
            ;;
        "risc0")
            print_info "Stopping Risc0 container..."
            docker-compose -f ../production/docker-compose.yml stop risc0-zkvm
            ;;
        "sp1")
            print_info "Stopping SP1 container..."
            docker-compose -f ../production/docker-compose.yml stop sp1-zkvm
            ;;
        "zkm")
            print_info "Stopping ZKM container..."
            docker-compose -f ../production/docker-compose.yml stop zkm-zkvm
            ;;
        "all")
            print_info "Stopping all containers..."
            docker-compose -f ../production/docker-compose.yml down
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

show_logs() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            docker-compose -f ../production/docker-compose.yml logs -f nexus-zkvm
            ;;
        "risc0")
            docker-compose -f ../production/docker-compose.yml logs -f risc0-zkvm
            ;;
        "sp1")
            docker-compose -f ../production/docker-compose.yml logs -f sp1-zkvm
            ;;
        "zkm")
            docker-compose -f ../production/docker-compose.yml logs -f zkm-zkvm
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

show_ps() {
    print_info "Running containers:"
    docker ps --filter "name=zkvm" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
}

clean_up() {
    print_warning "This will remove all ZKVM-related images and containers"
    read -p "Confirm to continue? (y/N): " confirm
    
    if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
        print_info "Cleaning up..."
        docker-compose -f ../production/docker-compose.yml down --remove-orphans
        docker rmi $(docker images "*zkvm*" -q) 2>/dev/null || true
        docker system prune -f
        print_success "Cleanup completed"
    else
        print_info "Cancelled"
    fi
}

# Main script logic
case $1 in
    "build")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to build"
            show_usage
            exit 1
        fi
        build_image $2
        ;;
    "run")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to run"
            show_usage
            exit 1
        fi
        run_image $2
        ;;
    "stop")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to stop"
            show_usage
            exit 1
        fi
        stop_container $2
        ;;
    "logs")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to view logs"
            show_usage
            exit 1
        fi
        show_logs $2
        ;;
    "ps")
        show_ps
        ;;
    "clean")
        clean_up
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
