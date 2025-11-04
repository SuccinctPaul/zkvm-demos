# ZKVM Docker Environment

Isolated Docker environments for multiple ZKVMs (SP1, Nexus, Risc0, ZKM) to avoid toolchain conflicts.

## 🚀 Quick Start

### First Time Setup
```bash
cd docker/scripts

# Build base image (required once)
./build-base.sh

# Build your ZKVM toolchain
./docker-manager.sh build sp1
```

### Development Mode - Two Ways to Use

#### Option 1: Using Docker Compose Directly (Recommended)
```bash
cd docker

# Build once (only needed first time or when dependencies change)
docker compose build sp1-zkvm

# Run multiple times (fast, no rebuild)
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-zkvm

# Run in background
docker compose --profile sp1 up -d sp1-zkvm

# Rebuild and run when needed
docker compose --profile sp1 up --build sp1-zkvm

# Interactive shell
docker compose run --rm sp1-zkvm bash

# Stop container
docker compose --profile sp1 down
```

#### Option 2: Using Convenience Script
```bash
cd docker/scripts

# Build once
./docker-manager.sh build sp1

# Run multiple times (fast)
./docker-manager.sh run sp1 --execute
./docker-manager.sh run sp1 --prove

# Rebuild and run
./docker-manager.sh run sp1 --build --execute

# Interactive shell
./docker-manager.sh shell sp1
```

## 📊 Development Mode

Development mode provides a flexible environment with real-time code changes and interactive debugging capabilities.

## 📋 All ZKVM Examples

### 🔵 SP1 ZKVM
```bash
# Using Docker Compose
cd docker
docker compose build sp1-zkvm                                    # Build once
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
ZKVM_ARGS="--execute" docker compose --profile zkm up zkm-zkvm
ZKVM_ARGS="--prove" docker compose --profile zkm up zkm-zkvm
docker compose run --rm zkm-zkvm bash

# Using script
cd docker/scripts
./docker-manager.sh build zkm
./docker-manager.sh run zkm --execute
./docker-manager.sh run zkm --prove
./docker-manager.sh shell zkm
```

## 📊 Usage Comparison

| Method | Command | When to Use |
|--------|---------|-------------|
| **Docker Compose** | `docker compose --profile sp1 up [--build]` | Direct control, standard Docker workflow |
| **Script** | `./docker-manager.sh run sp1 [--build]` | Convenient wrapper, simpler syntax |

**Key Points:**
- ✅ Build once with `docker compose build` or script `build` command
- ✅ Run multiple times without rebuilding (very fast)
- ✅ Use `--build` flag only when dependencies change
- ✅ Both methods support the same features

## 🎯 Common Workflows

### Workflow 1: Rapid Development (No Rebuild Needed)
```bash
# Using Docker Compose
cd docker
docker compose build sp1-zkvm                                   # Build once
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm # Fast run
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-zkvm # Fast run again

# Using script
cd docker/scripts
./docker-manager.sh build sp1          # Build once
./docker-manager.sh run sp1 --execute  # Fast run
./docker-manager.sh run sp1 --execute  # Fast run again
```

### Workflow 2: When Dependencies Change
```bash
# Using Docker Compose
cd docker
docker compose --profile sp1 up --build sp1-zkvm  # Rebuild and run

# Using script
cd docker/scripts
./docker-manager.sh run sp1 --build --execute  # Rebuild and run
```

### Workflow 3: Background Mode
```bash
# Using Docker Compose
cd docker
docker compose --profile sp1 up -d sp1-zkvm        # Start in background
docker compose logs -f sp1-zkvm                    # View logs
docker compose --profile sp1 down                 # Stop

# Using script
cd docker/scripts
./docker-manager.sh up sp1                   # Start in background
./docker-manager.sh logs sp1                 # View logs
./docker-manager.sh stop sp1                 # Stop
```

### Workflow 4: Debug Issues
```bash
# Using Docker Compose
cd docker
docker compose run --rm sp1-zkvm bash              # Interactive shell

# Using script
cd docker/scripts
./docker-manager.sh shell sp1                # Interactive shell

# Inside container (both methods)
cd sp1-zkvm/sp1-host
cargo build --release
RUST_LOG=debug cargo run --release -- --execute
```

## 🛠️ Management Commands

### Development Manager
```bash
./docker-manager.sh build <zkvm>        # Build toolchain image
./docker-manager.sh run <zkvm> [args]   # Run with args
./docker-manager.sh shell <zkvm>        # Interactive shell
./docker-manager.sh logs <zkvm>         # View logs
./docker-manager.sh clean-cache         # Free space
```

## 🔍 Troubleshooting

### Error: "zkvm-base:latest not found"
**Auto-fix:** Scripts detect and build automatically
```bash
./docker-manager.sh build sp1  # Will build base if needed
```

### Slow First Run?
**Normal:** First compilation takes 5-10 mins. Cached runs: 30s-2mins.

### Code Changes Not Applied?
**Solution:** Development mode automatically picks up code changes
```bash
# Development: automatic ✅
./docker-manager.sh run sp1 --execute
```

### Out of Disk Space?
```bash
# Clean caches
./docker-manager.sh clean-cache

# Clean everything
docker system prune -a
```

## 📁 Directory Structure

```
docker/
├── docker-compose.yml       # Main compose file
├── dockerfiles/             # All Dockerfiles
│   ├── Dockerfile.base      # Shared base image
│   ├── Dockerfile.nexus     # Nexus ZKVM toolchain
│   ├── Dockerfile.risc0     # Risc0 ZKVM toolchain
│   ├── Dockerfile.sp1       # SP1 ZKVM toolchain
│   └── Dockerfile.zkm       # ZKM ZKVM toolchain
├── scripts/                 # Management scripts
│   ├── build-base.sh        # Build base image
│   └── docker-manager.sh    # Container management
└── README.md                # This file
```

## 📚 Documentation

- **[USAGE.md](USAGE.md)** - Detailed usage guide with examples (Chinese)

## 💡 Tips

1. **Development mode** provides real-time code sync - no rebuilds needed
2. **Interactive shell** is great for debugging (`shell` command)
3. **First build is slow**, subsequent builds are cached
4. **Clean cache** when disk space is low

## 🎯 ZKVM Feature Matrix

| ZKVM | Execute Mode | Prove Mode | Interactive Shell |
|------|--------------|------------|-------------------|
| SP1 | `--execute` ✅ | `--prove` ✅ | ✅ |
| Nexus | Auto ⚡ | Auto ⚡ | ✅ |
| Risc0 | Auto ⚡ | Auto ⚡ | ✅ |
| ZKM | Auto ⚡ | Auto ⚡ | ✅ |

## ⚡ One-Line Commands

```bash
# Quick test SP1
cd docker/scripts && ./docker-manager.sh build sp1 && ./docker-manager.sh run sp1 --execute

# Build all ZKVMs
cd docker/scripts && for z in sp1 nexus risc0 zkm; do ./docker-manager.sh build $z; done

# Interactive SP1 debug
cd docker/scripts && ./docker-manager.sh shell sp1
```

## 🆘 Need Help?

1. Check [DOCKER-USAGE-EXAMPLES.md](docs/DOCKER-USAGE-EXAMPLES.md) for specific examples
2. Try the interactive menu: `./quick-start.sh`
3. View logs: `./docker-manager.sh logs <zkvm>`
4. Enter container: `./docker-manager.sh shell <zkvm>`

---

**Choose your mode, run your ZKVM, done! 🚀**
