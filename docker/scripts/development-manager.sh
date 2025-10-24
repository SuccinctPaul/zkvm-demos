#!/bin/bash
# Development Environment Manager
# Manages toolchain Docker images for flexible development

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
    echo "Development Environment Manager"
    echo "Purpose: Manage toolchain images for daily development and debugging"
    echo ""
    echo "Usage: $0 [COMMAND] [ZKVM] [ARGS...]"
    echo ""
    echo "Commands:"
    echo "  build [zkvm]           Build toolchain image"
    echo "  run [zkvm] [args]      Run toolchain mode with arguments"
    echo "  shell [zkvm]           Enter interactive shell"
    echo "  exec [zkvm] [cmd]      Execute command in container"
    echo "  stop [zkvm]            Stop running container"
    echo "  logs [zkvm]            View container logs"
    echo "  ps                     View running containers"
    echo "  clean-cache            Clean build cache"
    echo ""
    echo "ZKVMs: nexus, risc0, sp1, zkm, all"
    echo ""
    echo "Examples:"
    echo "  # Build toolchain image"
    echo "  $0 build sp1"
    echo ""
    echo "  # Run different modes"
    echo "  $0 run sp1 --execute          # SP1 execute mode"
    echo "  $0 run sp1 --prove            # SP1 prove mode"
    echo "  $0 run nexus --nocapture      # Nexus with stdout"
    echo ""
    echo "  # Interactive development"
    echo "  $0 shell sp1                  # Enter SP1 container"
    echo "  $0 exec sp1 cargo build       # Execute command in container"
    echo ""
    echo "Features:"
    echo "  ✅ Real-time source code sync (volume mount)"
    echo "  ✅ Flexible parameter passing"
    echo "  ✅ Persistent build cache"
    echo "  ✅ Suitable for rapid iterative development"
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

build_toolchain() {
    local zkvm=$1
    
    # Ensure base image exists
    check_base_image
    
    case $zkvm in
        "nexus")
            print_info "Building Nexus ZKVM toolchain image..."
            docker-compose -f ../development/docker-compose.yml build nexus-zkvm-toolchain
            print_success "Nexus toolchain image built successfully"
            ;;
        "risc0")
            print_info "Building Risc0 ZKVM toolchain image..."
            docker-compose -f ../development/docker-compose.yml build risc0-zkvm-toolchain
            print_success "Risc0 toolchain image built successfully"
            ;;
        "sp1")
            print_info "Building SP1 ZKVM toolchain image..."
            docker-compose -f ../development/docker-compose.yml build sp1-zkvm-toolchain
            print_success "SP1 toolchain image built successfully"
            ;;
        "zkm")
            print_info "Building ZKM ZKVM toolchain image..."
            docker-compose -f ../development/docker-compose.yml build zkm-zkvm-toolchain
            print_success "ZKM toolchain image built successfully"
            ;;
        "all")
            print_info "Building all ZKVM toolchain images..."
            docker-compose -f ../development/docker-compose.yml build nexus-zkvm-toolchain risc0-zkvm-toolchain sp1-zkvm-toolchain zkm-zkvm-toolchain
            print_success "All toolchain images built successfully"
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

run_toolchain() {
    local zkvm=$1
    shift
    local args="$@"
    
    if [ -z "$args" ]; then
        print_warning "No arguments provided, will use default parameters"
    fi
    
    case $zkvm in
        "nexus")
            print_info "Running Nexus ZKVM (toolchain mode, args: ${args:-default})..."
            ZKVM_ARGS="$args" docker-compose -f ../development/docker-compose.yml --profile nexus-toolchain up nexus-zkvm-toolchain
            ;;
        "risc0")
            print_info "Running Risc0 ZKVM (toolchain mode, args: ${args:-default})..."
            ZKVM_ARGS="$args" docker-compose -f ../development/docker-compose.yml --profile risc0-toolchain up risc0-zkvm-toolchain
            ;;
        "sp1")
            print_info "Running SP1 ZKVM (toolchain mode, args: ${args:-default})..."
            ZKVM_ARGS="$args" docker-compose -f ../development/docker-compose.yml --profile sp1-toolchain up sp1-zkvm-toolchain
            ;;
        "zkm")
            print_info "Running ZKM ZKVM (toolchain mode, args: ${args:-default})..."
            ZKVM_ARGS="$args" docker-compose -f ../development/docker-compose.yml --profile zkm-toolchain up zkm-zkvm-toolchain
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

start_shell() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Starting Nexus interactive shell..."
            docker-compose -f ../development/docker-compose.yml --profile dev up -d nexus-dev
            docker exec -it nexus-dev bash
            ;;
        "risc0")
            print_info "Starting Risc0 interactive shell..."
            docker-compose -f ../development/docker-compose.yml --profile dev up -d risc0-dev
            docker exec -it risc0-dev bash
            ;;
        "sp1")
            print_info "Starting SP1 interactive shell..."
            docker-compose -f ../development/docker-compose.yml --profile dev up -d sp1-dev
            docker exec -it sp1-dev bash
            ;;
        "zkm")
            print_info "Starting ZKM interactive shell..."
            docker-compose -f ../development/docker-compose.yml --profile dev up -d zkm-dev
            docker exec -it zkm-dev bash
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

exec_command() {
    local zkvm=$1
    shift
    local cmd="$@"
    
    if [ -z "$cmd" ]; then
        print_error "Please specify the command to execute"
        exit 1
    fi
    
    local container_name="${zkvm}-zkvm-toolchain"
    
    print_info "Executing in $container_name: $cmd"
    docker exec -it $container_name bash -c "$cmd"
}

stop_container() {
    local zkvm=$1
    
    case $zkvm in
        "nexus")
            print_info "Stopping Nexus container..."
            docker-compose -f ../development/docker-compose.yml stop nexus-zkvm-toolchain nexus-dev
            ;;
        "risc0")
            print_info "Stopping Risc0 container..."
            docker-compose -f ../development/docker-compose.yml stop risc0-zkvm-toolchain risc0-dev
            ;;
        "sp1")
            print_info "Stopping SP1 container..."
            docker-compose -f ../development/docker-compose.yml stop sp1-zkvm-toolchain sp1-dev
            ;;
        "zkm")
            print_info "Stopping ZKM container..."
            docker-compose -f ../development/docker-compose.yml stop zkm-zkvm-toolchain zkm-dev
            ;;
        "all")
            print_info "Stopping all development containers..."
            docker-compose -f ../development/docker-compose.yml down
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
            docker-compose -f ../development/docker-compose.yml logs -f nexus-zkvm-toolchain
            ;;
        "risc0")
            docker-compose -f ../development/docker-compose.yml logs -f risc0-zkvm-toolchain
            ;;
        "sp1")
            docker-compose -f ../development/docker-compose.yml logs -f sp1-zkvm-toolchain
            ;;
        "zkm")
            docker-compose -f ../development/docker-compose.yml logs -f zkm-zkvm-toolchain
            ;;
        *)
            print_error "Unknown ZKVM: $zkvm"
            show_usage
            exit 1
            ;;
    esac
}

show_ps() {
    print_info "Running development containers:"
    docker ps --filter "name=toolchain\|dev" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
}

clean_cache() {
    print_warning "This will clean all build cache volumes"
    print_info "Cache volume list:"
    docker volume ls | grep "cache"
    echo ""
    read -p "Confirm to continue? (y/N): " confirm
    
    if [ "$confirm" = "y" ] || [ "$confirm" = "Y" ]; then
        print_info "Cleaning cache..."
        docker volume rm $(docker volume ls -q | grep "cache") 2>/dev/null || true
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
        build_toolchain $2
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
    "shell")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM"
            show_usage
            exit 1
        fi
        start_shell $2
        ;;
    "exec")
        if [ -z "$2" ]; then
            print_error "Please specify the ZKVM"
            show_usage
            exit 1
        fi
        zkvm_name=$2
        shift 2
        exec_command $zkvm_name "$@"
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
