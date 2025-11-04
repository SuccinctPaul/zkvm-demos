# ZKVM Docker Environment

Isolated Docker environments for multiple ZKVMs (SP1, Nexus, Risc0, ZKM) to avoid toolchain conflicts.

## 🚀 Quick Start

### First Time Setup
```bash
cd docker/scripts

# Build base image (required once)
./build-base.sh

# Build your ZKVM toolchain
./development-manager.sh build sp1
```

### Development Mode - Two Ways to Use

#### Option 1: Using Docker Compose Directly (Recommended)
```bash
cd docker/development

# Build once (only needed first time or when dependencies change)
docker compose build sp1-dev

# Run multiple times (fast, no rebuild)
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-dev
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-dev

# Run in background
docker compose --profile sp1 up -d sp1-dev

# Rebuild and run when needed
docker compose --profile sp1 up --build sp1-dev

# Interactive shell
docker compose run --rm sp1-dev bash

# Stop container
docker compose --profile sp1 down
```

#### Option 2: Using Convenience Script
```bash
cd docker/scripts

# Build once
./development-manager.sh build sp1

# Run multiple times (fast)
./development-manager.sh run sp1 --execute
./development-manager.sh run sp1 --prove

# Rebuild and run
./development-manager.sh run sp1 --build --execute

# Interactive shell
./development-manager.sh shell sp1
```

## 📊 Development Mode

Development mode provides a flexible environment with real-time code changes and interactive debugging capabilities.

## 📋 All ZKVM Examples

### 🔵 SP1 ZKVM
```bash
# Using Docker Compose
cd docker/development
docker compose build sp1-dev                                    # Build once
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-dev  # Execute mode
ZKVM_ARGS="--prove" docker compose --profile sp1 up sp1-dev    # Prove mode
docker compose run --rm sp1-dev bash                            # Interactive shell

# Using script
cd docker/scripts
./development-manager.sh build sp1
./development-manager.sh run sp1 --execute
./development-manager.sh run sp1 --prove
./development-manager.sh shell sp1
```

### 🟢 Nexus ZKVM
```bash
# Using Docker Compose
cd docker/development
docker compose build nexus-dev
ZKVM_ARGS="--nocapture" docker compose --profile nexus up nexus-dev
docker compose run --rm nexus-dev bash

# Using script
cd docker/scripts
./development-manager.sh build nexus
./development-manager.sh run nexus --nocapture
./development-manager.sh shell nexus
```

### 🔴 Risc0 ZKVM
```bash
# Using Docker Compose
cd docker/development
docker compose build risc0-dev
docker compose --profile risc0 up risc0-dev
docker compose run --rm risc0-dev bash

# Using script
cd docker/scripts
./development-manager.sh build risc0
./development-manager.sh run risc0
./development-manager.sh shell risc0
```

### 🟡 ZKM ZKVM
```bash
# Using Docker Compose
cd docker/development
docker compose build zkm-dev
ZKVM_ARGS="--execute" docker compose --profile zkm up zkm-dev
ZKVM_ARGS="--prove" docker compose --profile zkm up zkm-dev
docker compose run --rm zkm-dev bash

# Using script
cd docker/scripts
./development-manager.sh build zkm
./development-manager.sh run zkm --execute
./development-manager.sh run zkm --prove
./development-manager.sh shell zkm
```

## 📊 Usage Comparison

| Method | Command | When to Use |
|--------|---------|-------------|
| **Docker Compose** | `docker compose --profile sp1 up [--build]` | Direct control, standard Docker workflow |
| **Script** | `./development-manager.sh run sp1 [--build]` | Convenient wrapper, simpler syntax |

**Key Points:**
- ✅ Build once with `docker compose build` or script `build` command
- ✅ Run multiple times without rebuilding (very fast)
- ✅ Use `--build` flag only when dependencies change
- ✅ Both methods support the same features

## 🎯 Common Workflows

### Workflow 1: Rapid Development (No Rebuild Needed)
```bash
# Using Docker Compose
cd docker/development
docker compose build sp1-dev                                   # Build once
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-dev # Fast run
ZKVM_ARGS="--execute" docker compose --profile sp1 up sp1-dev # Fast run again

# Using script
cd docker/scripts
./development-manager.sh build sp1          # Build once
./development-manager.sh run sp1 --execute  # Fast run
./development-manager.sh run sp1 --execute  # Fast run again
```

### Workflow 2: When Dependencies Change
```bash
# Using Docker Compose
cd docker/development
docker compose --profile sp1 up --build sp1-dev  # Rebuild and run

# Using script
cd docker/scripts
./development-manager.sh run sp1 --build --execute  # Rebuild and run
```

### Workflow 3: Background Mode
```bash
# Using Docker Compose
cd docker/development
docker compose --profile sp1 up -d sp1-dev        # Start in background
docker compose logs -f sp1-dev                    # View logs
docker compose --profile sp1 down                 # Stop

# Using script
cd docker/scripts
./development-manager.sh up sp1                   # Start in background
./development-manager.sh logs sp1                 # View logs
./development-manager.sh stop sp1                 # Stop
```

### Workflow 4: Debug Issues
```bash
# Using Docker Compose
cd docker/development
docker compose run --rm sp1-dev bash              # Interactive shell

# Using script
cd docker/scripts
./development-manager.sh shell sp1                # Interactive shell

# Inside container (both methods)
cd sp1-zkvm/sp1-host
cargo build --release
RUST_LOG=debug cargo run --release -- --execute
```

## 🛠️ Management Commands

### Development Manager
```bash
./development-manager.sh build <zkvm>        # Build toolchain image
./development-manager.sh run <zkvm> [args]   # Run with args
./development-manager.sh shell <zkvm>        # Interactive shell
./development-manager.sh logs <zkvm>         # View logs
./development-manager.sh clean-cache         # Free space
```

## 🔍 Troubleshooting

### Error: "zkvm-base:latest not found"
**Auto-fix:** Scripts detect and build automatically
```bash
./development-manager.sh build sp1  # Will build base if needed
```

### Slow First Run?
**Normal:** First compilation takes 5-10 mins. Cached runs: 30s-2mins.

### Code Changes Not Applied?
**Solution:** Development mode automatically picks up code changes
```bash
# Development: automatic ✅
./development-manager.sh run sp1 --execute
```

### Out of Disk Space?
```bash
# Clean caches
./development-manager.sh clean-cache

# Clean everything
docker system prune -a
```

## 📁 Directory Structure

```
docker/
├── base/                    # Shared base image
├── development/             # Toolchain images  
├── scripts/                 # Management scripts
└── docs/                    # Detailed guides
    ├── DOCKER-USAGE-EXAMPLES.md  # Complete examples
    └── USAGE-GUIDE.md            # Detailed guide
```

## 📚 Documentation

- **[DOCKER-USAGE-EXAMPLES.md](docs/DOCKER-USAGE-EXAMPLES.md)** - Complete examples for all ZKVMs
- **[USAGE-GUIDE.md](docs/USAGE-GUIDE.md)** - Detailed usage patterns and best practices

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
cd docker/scripts && ./development-manager.sh build sp1 && ./development-manager.sh run sp1 --execute

# Build all ZKVMs
cd docker/scripts && for z in sp1 nexus risc0 zkm; do ./development-manager.sh build $z; done

# Interactive SP1 debug
cd docker/scripts && ./development-manager.sh shell sp1
```

## 🆘 Need Help?

1. Check [DOCKER-USAGE-EXAMPLES.md](docs/DOCKER-USAGE-EXAMPLES.md) for specific examples
2. Try the interactive menu: `./quick-start.sh`
3. View logs: `./development-manager.sh logs <zkvm>`
4. Enter container: `./development-manager.sh shell <zkvm>`

---

**Choose your mode, run your ZKVM, done! 🚀**
