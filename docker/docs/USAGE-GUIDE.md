# Docker Usage Guide

## 📋 Overview

This project provides **two Docker solutions** for different scenarios, completely isolating different ZKVM toolchains to avoid conflicts on the same machine.

## 🎯 Two Modes Comparison

### 1. Production Mode (Precompiled) - For Deployment

**Characteristics:**
- ✅ Application precompiled during image build
- ✅ Fast startup, suitable for production deployment
- ✅ Fixed commands, stable and reliable
- ❌ Code changes require image rebuild
- ❌ Command parameters fixed, less flexible

**Use Cases:**
- Production environment deployment
- CI/CD pipelines
- Demos and testing fixed versions

**Dockerfiles:**
- `production/Dockerfile.{nexus,risc0,sp1,zkm}`

**Management Script:**
- `scripts/production-manager.sh`

### 2. Development Mode (Toolchain) - For Development

**Characteristics:**
- ✅ Image contains only environment and toolchain
- ✅ Source code mounted via volume, real-time updates
- ✅ Flexible command parameters (--execute, --prove, etc.)
- ✅ Persistent cache to accelerate repeated builds
- ⚠️ First run needs compilation (slower)
- ✅ Ideal for debugging and experimentation

**Use Cases:**
- Daily development
- Code debugging
- Parameter experimentation
- Quick testing after code changes

**Dockerfiles:**
- `development/Dockerfile.{nexus,risc0,sp1,zkm}.toolchain`

**Management Script:**
- `scripts/development-manager.sh`

## 🚀 Quick Start

### Method 1: Interactive Script (Recommended for Beginners)

```bash
cd docker/scripts
./quick-start.sh
```

Then select:
- Options 1-5: Production mode
- Options 6-9: Development mode (can input custom parameters)
- Option 10: Build base image
- Option 11: Clean everything

### Method 2: Management Scripts (Recommended for Daily Use)

#### Production Mode Examples

```bash
cd docker/scripts

# Build precompiled image
./production-manager.sh build sp1

# Run precompiled image
./production-manager.sh run sp1

# View logs
./production-manager.sh logs sp1

# Stop container
./production-manager.sh stop sp1
```

#### Development Mode Examples

```bash
cd docker/scripts

# Build toolchain image
./development-manager.sh build sp1

# Run in execute mode
./development-manager.sh run sp1 --execute

# Run in prove mode
./development-manager.sh run sp1 --prove

# Enter interactive shell
./development-manager.sh shell sp1

# Execute command in container
./development-manager.sh exec sp1 cargo build --release
```

### Method 3: Direct Docker Compose

#### Production Mode

```bash
# Build
docker-compose -f docker/production/docker-compose.yml build sp1-zkvm

# Run
docker-compose -f docker/production/docker-compose.yml --profile sp1 up sp1-zkvm
```

#### Development Mode

```bash
# Build
docker-compose -f docker/development/docker-compose.yml build sp1-dev

# Run - execute mode
docker-compose -f docker/development/docker-compose.yml run --rm sp1-dev bash -c "cd sp1-zkvm/sp1-host && cargo run --release -- --execute"

# Run - prove mode
docker-compose -f docker/development/docker-compose.yml run --rm sp1-dev bash -c "cd sp1-zkvm/sp1-host && cargo run --release -- --prove"
```

## 📊 Supported Parameters for Each ZKVM

### SP1 ZKVM

```bash
# Development mode
./development-manager.sh run sp1 --execute  # Execute mode
./development-manager.sh run sp1 --prove    # Prove mode
```

### Nexus ZKVM

```bash
# Development mode
./development-manager.sh run nexus --nocapture  # Standard output
```

### Risc0 ZKVM

```bash
# Development mode (environment variables pre-configured)
./development-manager.sh run risc0
```

### ZKM ZKVM

```bash
# Development mode
./development-manager.sh run zkm --execute  # Execute mode
```

## 🔧 Development Mode Advantages

### 1. Persistent Cache

Development mode uses Docker volumes to cache build artifacts:

```yaml
volumes:
  - sp1-cargo-cache:/usr/local/cargo/registry  # Cargo dependency cache
  - sp1-target-cache:/workspace/target         # Build artifact cache
```

**Benefits:**
- Significantly faster builds after first compilation
- Shared cache across containers
- Save disk space

### 2. Real-time Source Code Sync

```yaml
volumes:
  - ../:/workspace  # Mount entire project directory
```

**Benefits:**
- Code changes take effect immediately
- No need to rebuild images
- Support IDE and container collaboration

### 3. Flexible Command Parameters

Pass parameters via environment variables:

```bash
docker-compose run --rm sp1-dev bash -c "cd sp1-zkvm/sp1-host && cargo run --release -- --prove"
```

**Benefits:**
- Same image supports multiple run modes
- Can pass arbitrary command-line arguments
- Easy to test different configurations

## 🎓 Best Practices

### Development Workflow

1. **First Time Setup**
```bash
# Build base image (all ZKVMs share this)
./docker/scripts/build-base.sh

# Build needed toolchain image
./docker/scripts/development-manager.sh build sp1
```

2. **Daily Development**
```bash
# Run directly after code changes, auto recompile
./docker/scripts/development-manager.sh run sp1 --execute

# Test different modes
./docker/scripts/development-manager.sh run sp1 --prove
```

3. **Debug Environment**
```bash
# Enter interactive shell
./docker/scripts/development-manager.sh shell sp1

# Inside container, run manually
cd sp1-zkvm/sp1-host
cargo run --release -- --execute
cargo run --release -- --prove
```

4. **Production Deployment**
```bash
# Build precompiled image
./docker/scripts/production-manager.sh build sp1

# Deploy and run
./docker/scripts/production-manager.sh run sp1
```

### Resource Management

```bash
# View all images
docker images | grep zkvm

# Clean cache volumes (free space)
docker volume ls | grep cache
docker volume rm sp1-cargo-cache sp1-target-cache

# Complete cleanup
./docker/scripts/production-manager.sh clean
./docker/scripts/development-manager.sh clean-cache
```

## 💡 Example: SP1 Development Workflow

### Scenario 1: Development & Debugging (Toolchain Mode)

```bash
cd docker/scripts

# 1. Build toolchain image (first time)
./development-manager.sh build sp1

# 2. Run execute mode test
./development-manager.sh run sp1 --execute

# 3. Modify code (in your IDE on host machine)
# ... edit sp1-zkvm/sp1-host/src/main.rs ...

# 4. Run prove mode directly (no rebuild needed!)
./development-manager.sh run sp1 --prove

# 5. Enter container for debugging
./development-manager.sh shell sp1
# Inside container:
cd sp1-zkvm/sp1-host
cargo run --release -- --execute
cargo run --release -- --prove
```

### Scenario 2: Production Deployment (Precompiled Mode)

```bash
cd docker/scripts

# 1. Build precompiled image
./production-manager.sh build sp1

# 2. Quick start (no compilation)
./production-manager.sh run sp1

# 3. Deploy to production
docker-compose -f docker/production/docker-compose.yml --profile sp1 up -d sp1-zkvm
```

## 🔍 Mode Comparison Table

| Feature | Production Mode | Development Mode |
|---------|----------------|------------------|
| **Startup Speed** | ⚡ Fast (precompiled) | 🐢 First time slow (needs compilation) |
| **Image Size** | 📦 Larger (includes app) | 📦 Smaller (toolchain only) |
| **Code Updates** | 🔄 Requires image rebuild | ✅ Real-time effect |
| **Parameter Flexibility** | ❌ Fixed commands | ✅ Dynamic parameters |
| **Build Cache** | ❌ None | ✅ Persistent cache |
| **Use Case** | 🚀 Production deployment | 💻 Daily development |
| **Dockerfile Location** | `production/` | `development/` |
| **Management Script** | `production-manager.sh` | `development-manager.sh` |

## 📚 More Resources

- [Main README](../README.md) - Quick start guide
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues and solutions
- [Optimization Summary](OPTIMIZATION-SUMMARY.md) - Directory structure details

---

**Choose the mode that fits your scenario and enjoy efficient ZKVM development!** 🚀

