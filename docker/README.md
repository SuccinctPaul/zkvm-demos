# ZKVM Docker Environment

Isolated Docker environments for multiple ZKVMs (SP1, Nexus, Risc0, ZKM) to avoid toolchain conflicts.

## 🚀 Quick Start

```bash
# 1. Build base image (first time only)
cd docker/scripts
./build-base.sh

# 2. Build ZKVM
./docker-manager.sh build sp1

# 3. Run
cd ..
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm
```

## 📋 Two Ways to Use

### Option 1: Docker Compose (Recommended)

```bash
cd docker

# Build once
docker compose build sp1-zkvm

# Run with different arguments
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-zkvm

# Interactive shell
docker compose run --rm sp1-zkvm bash

# Rebuild when needed
docker compose --profile sp1 up --build sp1-zkvm
```

### Option 2: Convenience Script

```bash
cd docker/scripts

# Build once
./docker-manager.sh build sp1

# Run multiple times (fast)
./docker-manager.sh run sp1 --execute
./docker-manager.sh run sp1 --prove

# Rebuild when needed
./docker-manager.sh run sp1 --build --execute

# Interactive shell
./docker-manager.sh shell sp1
```

## 🎯 All ZKVM Examples

### 🔵 SP1 ZKVM

```bash
# Using Docker Compose
cd docker
docker compose build sp1-zkvm
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm  # Execute mode
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-zkvm    # Prove mode
docker compose run --rm sp1-zkvm bash                            # Interactive shell

# Using script
cd docker/scripts
./docker-manager.sh build sp1
./docker-manager.sh run sp1 --execute
./docker-manager.sh run sp1 --prove
./docker-manager.sh shell sp1
```

### 🟢 Nexus ZKVM

```bash
# Using Docker Compose
cd docker
docker compose build nexus-zkvm
ZKVM_ARGS="--nocapture" docker compose --profile nexus up nexus-zkvm
docker compose run --rm nexus-zkvm bash

# Using script
cd docker/scripts
./docker-manager.sh build nexus
./docker-manager.sh run nexus --nocapture
./docker-manager.sh shell nexus
```

### 🔴 Risc0 ZKVM

```bash
# Using Docker Compose
cd docker
docker compose build risc0-zkvm
docker compose --profile risc0 up risc0-zkvm
docker compose run --rm risc0-zkvm bash

# Using script
cd docker/scripts
./docker-manager.sh build risc0
./docker-manager.sh run risc0
./docker-manager.sh shell risc0
```

### 🟡 ZKM ZKVM

```bash
# Using Docker Compose
cd docker
docker compose build zkm-zkvm
ZKVM_ARGS="--execute" docker compose --profile zkm up zkm-zkvm  # Execute mode
ZKVM_ARGS="--prove" docker compose --profile zkm up zkm-zkvm    # Prove mode
docker compose run --rm zkm-zkvm bash

# Using script
cd docker/scripts
./docker-manager.sh build zkm
./docker-manager.sh run zkm --execute
./docker-manager.sh run zkm --prove
./docker-manager.sh shell zkm
```

## 💡 Key Points

- **Build once, run many times** - No rebuild needed unless dependencies change
- **Pass arguments via `ZKVM_ARGS`** - Environment variable controls behavior
- **Code changes auto-sync** - Modify source code without rebuilding images
- **Use `--build` to rebuild** - Only when Cargo.toml or Dockerfile changes

## 🛠️ Common Commands

```bash
# Background mode
docker compose --profile sp1 up -d sp1-zkvm
docker compose logs -f sp1-zkvm
docker compose --profile sp1 down

# Using script
./docker-manager.sh logs sp1
./docker-manager.sh stop sp1

# Clean cache
./docker-manager.sh clean

# List containers
docker ps --filter "name=zkvm"
```

## 📁 Directory Structure

```
docker/
├── docker-compose.yml       # Main configuration
├── dockerfiles/             # All Dockerfiles
│   ├── Dockerfile.base      # Shared base image
│   ├── Dockerfile.nexus
│   ├── Dockerfile.risc0
│   ├── Dockerfile.sp1
│   └── Dockerfile.zkm
└── scripts/
    ├── build-base.sh        # Build base image
    └── docker-manager.sh    # Management tool
```

## 🔍 Troubleshooting

**"zkvm-base:latest not found"** - Run `./build-base.sh` first

**Risc0 on Apple Silicon (M1/M2/M3)** - Risc0 toolchain doesn't support ARM64. Build uses `linux/amd64` with emulation (
works but slower)

**Slow first run** - Normal (5-10 mins). Cached runs: 30s-2mins

**Code changes not applied** - Don't rebuild, code is mounted and auto-syncs

**Out of disk space** - Run `./docker-manager.sh clean` or `docker system prune -a`

