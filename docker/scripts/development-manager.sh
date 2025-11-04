#!/bin/bash
# Development Environment Manager for ZKVM
# Simplified wrapper around docker compose

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCKER_DIR="$(dirname "$SCRIPT_DIR")"
COMPOSE_FILE="$DOCKER_DIR/docker-compose.yml"

# Print functions
info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Show usage
show_usage() {
    cat << EOF
╔══════════════════════════════════════════════════════════╗
║         ZKVM Development Environment Manager             ║
╚══════════════════════════════════════════════════════════╝

Usage: $0 [COMMAND] [ZKVM] [OPTIONS]

Commands:
  build <zkvm>           Build image (add --no-cache for clean build)
  run <zkvm>             Run container (use docker compose directly for args)
  shell <zkvm>           Enter interactive shell
  logs <zkvm>            View container logs
  stop <zkvm|all>        Stop container(s)
  ps                     List running containers
  clean                  Clean cache volumes

Available ZKVMs: nexus, risc0, sp1, zkm, all

Examples:
  # Build
  $0 build sp1                  # Build SP1 image
  $0 build sp1 --no-cache       # Clean build

  # Run (recommended: use docker compose directly)
  cd $DOCKER_DIR
  ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-dev
  
  # Or use this script for quick access
  $0 run sp1                    # Run with default args
  $0 shell sp1                  # Interactive shell
  $0 logs sp1                   # View logs
  $0 stop sp1                   # Stop container

Tip: For custom arguments, use docker compose directly:
  cd $DOCKER_DIR
  ZKVM_ARGS="your-args" docker compose --profile <zkvm> up <zkvm>-dev

EOF
}

# Validate ZKVM name
validate_zkvm() {
    local zkvm=$1
    case $zkvm in
        nexus|risc0|sp1|zkm|all) return 0 ;;
        *) error "Unknown ZKVM: $zkvm"; return 1 ;;
    esac
}

# Get service name
service_name() {
    echo "${1}-zkvm"
}

# Check base image
check_base_image() {
    if ! docker image inspect zkvm-base:latest &>/dev/null; then
        warn "Base image not found, building..."
        docker build -f "$DOCKER_DIR/dockerfiles/Dockerfile.base" \
                     -t zkvm-base:latest \
                     "$(dirname "$DOCKER_DIR")"
        success "Base image built"
    fi
}

# Build command
cmd_build() {
    local zkvm=$1
    local opts=$2
    
    if [ -z "$zkvm" ]; then
        error "Please specify ZKVM to build"
        show_usage
        exit 1
    fi
    
    validate_zkvm "$zkvm" || exit 1
    check_base_image
    
    local build_opts=""
    [ "$opts" = "--no-cache" ] && build_opts="--no-cache" && warn "Building without cache"
    
    if [ "$zkvm" = "all" ]; then
        info "Building all ZKVM images..."
        docker compose -f "$COMPOSE_FILE" build $build_opts nexus-zkvm risc0-zkvm sp1-zkvm zkm-zkvm
        success "All images built"
    else
        info "Building ${zkvm^^} image..."
        docker compose -f "$COMPOSE_FILE" build $build_opts "$(service_name "$zkvm")"
        success "${zkvm^^} image built"
    fi
}

# Run command (simplified - docker compose already has the command)
cmd_run() {
    local zkvm=$1
    
    if [ -z "$zkvm" ]; then
        error "Please specify ZKVM to run"
        show_usage
        exit 1
    fi
    
    validate_zkvm "$zkvm" || exit 1
    
    info "Running ${zkvm^^}..."
    info "Tip: Use docker compose directly for custom args:"
    info "  cd $DOCKER_DIR"
    info "  ZKVM_ARGS='your-args' docker compose --profile $zkvm up $(service_name "$zkvm")"
    echo ""
    
    docker compose -f "$COMPOSE_FILE" --profile "$zkvm" up "$(service_name "$zkvm")"
}

# Shell command
cmd_shell() {
    local zkvm=$1
    
    if [ -z "$zkvm" ]; then
        error "Please specify ZKVM"
        show_usage
        exit 1
    fi
    
    validate_zkvm "$zkvm" || exit 1
    
    info "Starting ${zkvm^^} interactive shell..."
    docker compose -f "$COMPOSE_FILE" run --rm "$(service_name "$zkvm")" bash
}

# Logs command
cmd_logs() {
    local zkvm=$1
    
    if [ -z "$zkvm" ]; then
        error "Please specify ZKVM"
        show_usage
        exit 1
    fi
    
    validate_zkvm "$zkvm" || exit 1
    
    docker compose -f "$COMPOSE_FILE" logs -f "$(service_name "$zkvm")"
}

# Stop command
cmd_stop() {
    local zkvm=$1
    
    if [ -z "$zkvm" ]; then
        error "Please specify ZKVM or 'all'"
        show_usage
        exit 1
    fi
    
    if [ "$zkvm" = "all" ]; then
        info "Stopping all containers..."
        docker compose -f "$COMPOSE_FILE" down
        success "All containers stopped"
    else
        validate_zkvm "$zkvm" || exit 1
        info "Stopping ${zkvm^^}..."
        docker compose -f "$COMPOSE_FILE" stop "$(service_name "$zkvm")" 2>/dev/null || warn "Container not running"
        success "Container stopped"
    fi
}

# PS command
cmd_ps() {
    info "Running containers:"
    docker ps --filter "name=-zkvm" --format "table {{.Names}}\t{{.Status}}\t{{.Image}}"
}

# Clean cache
cmd_clean() {
    warn "This will clean all build cache volumes"
    docker volume ls --filter "name=cache" --format "table {{.Name}}\t{{.Driver}}"
    echo ""
    read -p "Confirm deletion? (y/N): " confirm
    
    if [[ "$confirm" =~ ^[Yy]$ ]]; then
        info "Cleaning cache volumes..."
        docker volume ls -q --filter "name=cache" | xargs -r docker volume rm
        success "Cache cleaned"
    else
        info "Cancelled"
    fi
}

# Main dispatcher
main() {
    case ${1:-} in
        build)      cmd_build "$2" "$3" ;;
        run)        cmd_run "$2" ;;
        shell)      cmd_shell "$2" ;;
        logs)       cmd_logs "$2" ;;
        stop)       cmd_stop "$2" ;;
        ps)         cmd_ps ;;
        clean)      cmd_clean ;;
        -h|--help)  show_usage ;;
        "")         show_usage; exit 1 ;;
        *)          error "Unknown command: $1"; show_usage; exit 1 ;;
    esac
}

main "$@"
