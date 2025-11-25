# ZKVM Docker Environment

Isolated Docker environments for multiple ZKVMs (SP1, Nexus, Risc0, ZKM, ZisK) to avoid toolchain conflicts.

## 🚀 Quick Start

```bash
# 1. Build base image (first time only)
cd docker/scripts
./build-base.sh

# 2. Build ZKVM
cd ..
docker compose build sp1-zkvm

# 3. Run
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm
```

## 📋 Usage

```bash
cd docker

# Build once
docker compose build sp1-zkvm

# Run with different arguments
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-zkvm

# Interactive shell
docker compose run --rm sp1-zkvm bash

# Background mode
docker compose --profile sp1 up -d sp1-zkvm
docker compose logs -f sp1-zkvm
docker compose --profile sp1 down

# Rebuild when needed
docker compose --profile sp1 up --build sp1-zkvm
```

## 🎯 All ZKVM Examples

### 🟢 SP1 ZKVM

```bash
cd docker
docker compose build sp1-zkvm
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm  # Execute mode
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-zkvm    # Prove mode
docker compose run --rm sp1-zkvm bash                            # Interactive shell
```

### 🟢 Nexus ZKVM

```bash
cd docker
docker compose build nexus-zkvm
ZKVM_ARGS="--nocapture" docker compose --profile nexus up nexus-zkvm
docker compose run --rm nexus-zkvm bash
```

### 🔴 Risc0 ZKVM

```bash
cd docker
docker compose build risc0-zkvm
docker compose --profile risc0 up risc0-zkvm
docker compose run --rm risc0-zkvm bash
```

### 🔴 ZKM ZKVM

```bash
cd docker
docker compose build zkm-zkvm
ZKVM_ARGS="--execute" docker compose --profile zkm up zkm-zkvm  # Execute mode
ZKVM_ARGS="--prove" docker compose --profile zkm up zkm-zkvm    # Prove mode
docker compose run --rm zkm-zkvm bash
```

### 🟡 ZisK ZKVM

```bash
cd docker
docker compose build zisk-zkvm

# Test mode (build + emulator execution)
docker compose --profile zisk up zisk-zkvm

# Prove mode (build + execute + ROM setup + prove + verify)
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Custom Fibonacci number
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Interactive shell for manual control
docker compose run --rm zisk-zkvm bash
# Then inside container:
cd zisk-guest
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y
cargo-zisk verify -p ../proof/vadcop_final_proof.bin
```

**Note**: ZisK proof generation is only supported on Linux x86_64. On macOS with Apple Silicon, Docker will use emulation (slower but works).

## 💡 Key Points

- **Build once, run many times** - No rebuild needed unless dependencies change
- **Pass arguments via `ZKVM_ARGS`** - Environment variable controls behavior
- **Code changes auto-sync** - Modify source code without rebuilding images
- **Use `--build` to rebuild** - Only when Cargo.toml or Dockerfile changes

## 🛠️ Common Commands

```bash
# List containers
docker ps --filter "name=zkvm"

# Stop all
docker compose down

# Clean cache volumes
docker volume ls --filter "name=cache"
docker volume prune

# Clean everything
docker system prune -a
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
│   ├── Dockerfile.zkm
│   └── Dockerfile.zisk      # ZisK with proof generation support
└── scripts/
    └── build-base.sh        # Build base image
```

## 🔍 Troubleshooting

**"zkvm-base:latest not found"** - Run `./build-base.sh` first

**Risc0 on Apple Silicon (M1/M2/M3)** - Risc0 toolchain doesn't support ARM64. Build uses `linux/amd64` with emulation (works but slower)

**ZisK on macOS** - ZisK proof generation requires Linux x86_64. Docker uses `linux/amd64` platform with emulation on macOS (works but slower). For better performance, use a Linux machine or VM.

**ZisK ROM setup takes long time** - ROM setup is a one-time operation that can take 2-5 minutes. Results are cached in the `zisk-zisk-cache` volume.

**ZisK proof generation slow** - First proof generation includes ROM setup. Subsequent proofs are faster (~30-45s). On Apple Silicon with emulation, expect 2-3x slower.

**Slow first run** - Normal (5-10 mins). Cached runs: 30s-2mins

**Code changes not applied** - Don't rebuild, code is mounted and auto-syncs

**Out of disk space** - Run `docker volume prune` or `docker system prune -a`

