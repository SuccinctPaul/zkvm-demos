#!/bin/bash
# Development Environment Manager
# Simplified manager for ZKVM development containers

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$(dirname "$SCRIPT_DIR")"
COMPOSE_FILE="$DOCKER_DIR/development/docker-compose.yml"

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
    echo "╔══════════════════════════════════════════════════════════╗"
    echo "║         ZKVM Development Environment Manager             ║"
    echo "╚══════════════════════════════════════════════════════════╝"
    echo ""
    echo "Usage: $0 [COMMAND] [ZKVM] [OPTIONS] [ARGS...]"
    echo ""
    echo "Commands:"
    echo "  build [zkvm]                Build/rebuild toolchain image"
    echo "  run [zkvm] [options] [args] Run ZKVM with custom arguments"
    echo "  up [zkvm] [options]         Start container in background"
    echo "  shell [zkvm]                Enter interactive shell"
    echo "  stop [zkvm]                 Stop running container"
    echo "  logs [zkvm]                 View container logs"
    echo "  ps                          List running containers"
    echo "  clean-cache                 Clean build cache volumes"
    echo ""
    echo "Run Options:"
    echo "  --build                     Rebuild image before running"
    echo "  --no-cache                  Build without cache"
    echo ""
    echo "Available ZKVMs: nexus, risc0, sp1, zkm, all"
    echo ""
    echo "Examples:"
    echo "  # Build once, then run multiple times (fast)"
    echo "  $0 build sp1                      # Build image first"
    echo "  $0 run sp1 --execute              # Fast: use cached build"
    echo "  $0 run sp1 --execute              # Fast: reuse same build"
    echo ""
    echo "  # Force rebuild when needed"
    echo "  $0 run sp1 --build --execute      # Rebuild and run"
    echo "  $0 build sp1 --no-cache           # Clean rebuild"
    echo ""
    echo "  # Background mode"
    echo "  $0 up sp1                         # Start in background"
    echo "  $0 up sp1 --build                 # Rebuild and start"
    echo ""
    echo "  # Interactive mode"
    echo "  $0 shell sp1                      # Enter bash shell"
    echo ""
    echo "Features:"
    echo "  ✅ Manual build control - rebuild only when needed"
    echo "  ✅ Real-time source code mounting"
    echo "  ✅ Persistent build cache"
    echo "  ✅ Background mode support"
}

# Check and build base image if needed
check_base_image() {
    if ! docker image inspect zkvm-base:latest > /dev/null 2>&1; then
        print_warning "Base image zkvm-base:latest not found, building..."
        docker-compose -f "$DOCKER_DIR/base/docker-compose.yml" build zkvm-base
        print_success "Base image built successfully"
    fi
}

# Get service name for ZKVM
get_service_name() {
    local zkvm=$1
    case $zkvm in
        "nexus"|"risc0"|"sp1"|"zkm")
            echo "${zkvm}-dev"
            ;;
        *)
            echo ""
            ;;
    esac
}

# Get profile name for ZKVM
get_profile() {
    local zkvm=$1
    case $zkvm in
        "nexus"|"risc0"|"sp1"|"zkm")
            echo "${zkvm}"
            ;;
        "all")
            echo "all"
            ;;
        *)
            echo ""
            ;;
    esac
}

build_toolchain() {
    local zkvm=$1
    local no_cache=$2
    
    # Ensure base image exists
    check_base_image
    
    local build_opts=""
    if [ "$no_cache" = "--no-cache" ]; then
        build_opts="--no-cache"
        print_warning "Building without cache (this will take longer)"
    fi
    
    if [ "$zkvm" = "all" ]; then
        print_info "Building all ZKVM toolchain images..."
        docker-compose -f "$COMPOSE_FILE" build $build_opts nexus-dev risc0-dev sp1-dev zkm-dev
        print_success "All toolchain images built successfully"
    else
        local service=$(get_service_name "$zkvm")
        if [ -z "$service" ]; then
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
        fi
        
        print_info "Building $(echo $zkvm | tr '[:lower:]' '[:upper:]') ZKVM toolchain image..."
        docker-compose -f "$COMPOSE_FILE" build $build_opts "$service"
        print_success "$(echo $zkvm | tr '[:lower:]' '[:upper:]') toolchain image built successfully"
    fi
}

run_toolchain() {
    local zkvm=$1
    shift
    
    # Check for --build flag
    local should_build=false
    if [ "$1" = "--build" ]; then
        should_build=true
        shift
    fi
    
    local args="$@"
    
    local service=$(get_service_name "$zkvm")
    local profile=$(get_profile "$zkvm")
    
    if [ -z "$service" ] || [ -z "$profile" ]; then
        print_error "Unknown ZKVM: $zkvm"
        show_usage
        exit 1
    fi
    
    # Rebuild if requested
    if [ "$should_build" = true ]; then
        print_info "Rebuilding image first..."
        build_toolchain "$zkvm"
    fi
    
    # Build run command based on ZKVM type
    local run_cmd=""
    case $zkvm in
        "nexus")
            run_cmd="cd nexus-zkvm/nexus-host && cargo run -r -- ${args:---nocapture}"
            ;;
        "risc0")
            run_cmd="cd risc0-zkvm/risc0-host && RISC0_DEV_MODE=1 RUST_LOG=info RISC0_INFO=1 cargo run --release ${args}"
            ;;
        "sp1")
            run_cmd="cd sp1-zkvm/sp1-host && RUST_LOG=info cargo run --release -- ${args:---execute}"
            ;;
        "zkm")
            run_cmd="cd zkm-zkvm/zkm-host && RUST_LOG=info cargo run --release -- ${args:---execute}"
            ;;
    esac
    
    print_info "Running $(echo $zkvm | tr '[:lower:]' '[:upper:]') ZKVM (args: ${args:-default})..."
    docker-compose -f "$COMPOSE_FILE" run -it "$service" bash -c "$run_cmd"
}

up_container() {
    local zkvm=$1
    shift
    
    # Check for --build flag
    local build_flag=""
    if [ "$1" = "--build" ]; then
        build_flag="--build"
        print_info "Rebuilding image first..."
        shift
    fi
    
    local service=$(get_service_name "$zkvm")
    local profile=$(get_profile "$zkvm")
    
    if [ -z "$service" ] || [ -z "$profile" ]; then
        print_error "Unknown ZKVM: $zkvm"
        show_usage
        exit 1
    fi
    
    print_info "Starting $(echo $zkvm | tr '[:lower:]' '[:upper:]') container in background..."
    docker-compose -f "$COMPOSE_FILE" --profile "$profile" up -d $build_flag "$service"
    print_success "Container started. Use '$0 logs $zkvm' to view logs"
}

start_shell() {
    local zkvm=$1
    
    local service=$(get_service_name "$zkvm")
    if [ -z "$service" ]; then
        print_error "Unknown ZKVM: $zkvm"
        show_usage
        exit 1
    fi
    
    print_info "Starting $(echo $zkvm | tr '[:lower:]' '[:upper:]') interactive shell..."
#    docker-compose -f "$COMPOSE_FILE" run --rm "$service" bash
    docker-compose -f "$COMPOSE_FILE" run -it "$service" bash
}

stop_container() {
    local zkvm=$1
    
    if [ "$zkvm" = "all" ]; then
        print_info "Stopping all development containers..."
        docker-compose -f "$COMPOSE_FILE" down
        print_success "All containers stopped"
    else
        local service=$(get_service_name "$zkvm")
        if [ -z "$service" ]; then
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
        fi
        
        print_info "Stopping $(echo $zkvm | tr '[:lower:]' '[:upper:]') container..."
        docker stop "$service" 2>/dev/null || print_warning "Container not running"
        print_success "$(echo $zkvm | tr '[:lower:]' '[:upper:]') container stopped"
    fi
}

show_logs() {
    local zkvm=$1
    
    local service=$(get_service_name "$zkvm")
    if [ -z "$service" ]; then
        print_error "Unknown ZKVM: $zkvm"
        show_usage
        exit 1
    fi
    
    docker-compose -f "$COMPOSE_FILE" logs -f "$service"
}

show_ps() {
    print_info "Running development containers:"
    docker ps --filter "name=-dev" --format "table {{.Names}}\t{{.Status}}\t{{.Image}}"
}

clean_cache() {
    print_warning "This will clean all build cache volumes"
    print_info "Cache volumes:"
    docker volume ls --filter "name=cache" --format "table {{.Name}}\t{{.Driver}}"
    echo ""
    read -p "Confirm deletion? (y/N): " confirm
    
    if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
        print_info "Cleaning cache volumes..."
        docker volume ls -q --filter "name=cache" | xargs -r docker volume rm
        print_success "Cache cleanup completed"
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
        zkvm_name=$2
        no_cache_flag=""
        if [ "$3" = "--no-cache" ]; then
            no_cache_flag="--no-cache"
        fi
        build_toolchain $zkvm_name $no_cache_flag
        ;;
    "run")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to run"
            show_usage
            exit 1
        fi
        zkvm_name=$2
        shift 2
        run_toolchain $zkvm_name "$@"
        ;;
    "up")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM to start"
            show_usage
            exit 1
        fi
        zkvm_name=$2
        shift 2
        up_container $zkvm_name "$@"
        ;;
    "shell")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM"
            show_usage
            exit 1
        fi
        start_shell $2
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
    "clean-cache")
        clean_cache
        ;;
    *)
        show_usage
        exit 1
        ;;
esac
