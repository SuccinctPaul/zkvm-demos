#!/bin/bash
# Docker management scripts for zkvm-demos

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  build [zkvm]     Build Docker images for specific zkvm or all"
    echo "  build-base        Build the shared base image"
    echo "  run [zkvm]       Run specific zkvm demo"
    echo "  dev [zkvm]       Start development shell for specific zkvm"
    echo "  stop [zkvm]      Stop running containers"
    echo "  clean            Clean up Docker images and containers"
    echo "  logs [zkvm]      Show logs for specific zkvm"
    echo ""
    echo "ZKVMs: nexus, risc0, sp1, zkm, all"
    echo ""
    echo "Examples:"
    echo "  $0 build nexus          # Build Nexus ZKVM image"
    echo "  $0 run risc0            # Run Risc0 ZKVM demo"
    echo "  $0 dev sp1              # Start SP1 development shell"
    echo "  $0 build all            # Build all ZKVM images"
    echo "  $0 run all              # Run all ZKVM demos"
}

# Function to build base image
build_base() {
    print_info "Building shared base image..."
    docker-compose -f docker/docker-compose.yml build zkvm-base
    print_success "Shared base image built successfully"
}

# Function to build Docker images
build_images() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Building Nexus ZKVM image..."
            docker-compose -f docker/docker-compose.yml build nexus-zkvm
            print_success "Nexus ZKVM image built successfully"
            ;;
        "risc0")
            print_info "Building Risc0 ZKVM image..."
            docker-compose -f docker/docker-compose.yml build risc0-zkvm
            print_success "Risc0 ZKVM image built successfully"
            ;;
        "sp1")
            print_info "Building SP1 ZKVM image..."
            docker-compose -f docker/docker-compose.yml build sp1-zkvm
            print_success "SP1 ZKVM image built successfully"
            ;;
        "zkm")
            print_info "Building ZKM ZKVM image..."
            docker-compose -f docker/docker-compose.yml build zkm-zkvm
            print_success "ZKM ZKVM image built successfully"
            ;;
        "all")
            print_info "Building all ZKVM images..."
            docker-compose -f docker/docker-compose.yml build
            print_success "All ZKVM images built successfully"
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

# Function to run ZKVM demos
run_demo() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Running Nexus ZKVM demo..."
            docker-compose -f docker/docker-compose.yml --profile nexus up nexus-zkvm
            ;;
        "risc0")
            print_info "Running Risc0 ZKVM demo..."
            docker-compose -f docker/docker-compose.yml --profile risc0 up risc0-zkvm
            ;;
        "sp1")
            print_info "Running SP1 ZKVM demo..."
            docker-compose -f docker/docker-compose.yml --profile sp1 up sp1-zkvm
            ;;
        "zkm")
            print_info "Running ZKM ZKVM demo..."
            docker-compose -f docker/docker-compose.yml --profile zkm up zkm-zkvm
            ;;
        "all")
            print_info "Running all ZKVM demos..."
            docker-compose -f docker/docker-compose.yml --profile all up
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

# Function to start development shell
start_dev() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Starting Nexus development shell..."
            docker-compose -f docker/docker-compose.yml --profile dev up -d nexus-dev
            docker exec -it nexus-dev bash
            ;;
        "risc0")
            print_info "Starting Risc0 development shell..."
            docker-compose -f docker/docker-compose.yml --profile dev up -d risc0-dev
            docker exec -it risc0-dev bash
            ;;
        "sp1")
            print_info "Starting SP1 development shell..."
            docker-compose -f docker/docker-compose.yml --profile dev up -d sp1-dev
            docker exec -it sp1-dev bash
            ;;
        "zkm")
            print_info "Starting ZKM development shell..."
            docker-compose -f docker/docker-compose.yml --profile dev up -d zkm-dev
            docker exec -it zkm-dev bash
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

# Function to stop containers
stop_containers() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Stopping Nexus containers..."
            docker-compose -f docker/docker-compose.yml stop nexus-zkvm nexus-dev
            ;;
        "risc0")
            print_info "Stopping Risc0 containers..."
            docker-compose -f docker/docker-compose.yml stop risc0-zkvm risc0-dev
            ;;
        "sp1")
            print_info "Stopping SP1 containers..."
            docker-compose -f docker/docker-compose.yml stop sp1-zkvm sp1-dev
            ;;
        "zkm")
            print_info "Stopping ZKM containers..."
            docker-compose -f docker/docker-compose.yml stop zkm-zkvm zkm-dev
            ;;
        "all")
            print_info "Stopping all containers..."
            docker-compose -f docker/docker-compose.yml down
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

# Function to show logs
show_logs() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            docker-compose -f docker/docker-compose.yml logs -f nexus-zkvm
            ;;
        "risc0")
            docker-compose -f docker/docker-compose.yml logs -f risc0-zkvm
            ;;
        "sp1")
            docker-compose -f docker/docker-compose.yml logs -f sp1-zkvm
            ;;
        "zkm")
            docker-compose -f docker/docker-compose.yml logs -f zkm-zkvm
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

# Function to clean up Docker resources
clean_up() {
    print_info "Cleaning up Docker resources..."
    
    # Stop all containers
    docker-compose -f docker/docker-compose.yml down --remove-orphans
    
    # Remove images
    docker rmi $(docker images "zkvm-demos*" -q) 2>/dev/null || true
    
    # Remove unused containers, networks, images
    docker system prune -f
    
    print_success "Cleanup completed"
}

# Main script logic
case $1 in
    "build")
        if [ -z "$2" ]; then
            print_error "Please specify a ZKVM to build"
            show_usage
            exit 1
        fi
        build_images $2
        ;;
    "build-base")
        build_base
        ;;
    "run")
        if [ -z "$2" ]; then
            print_error "Please specify a ZKVM to run"
            show_usage
            exit 1
        fi
        run_demo $2
        ;;
    "dev")
        if [ -z "$2" ]; then
            print_error "Please specify a ZKVM for development"
            show_usage
            exit 1
        fi
        start_dev $2
        ;;
    "stop")
        if [ -z "$2" ]; then
            print_error "Please specify a ZKVM to stop"
            show_usage
            exit 1
        fi
        stop_containers $2
        ;;
    "logs")
        if [ -z "$2" ]; then
            print_error "Please specify a ZKVM for logs"
            show_usage
            exit 1
        fi
        show_logs $2
        ;;
    "clean")
        clean_up
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
