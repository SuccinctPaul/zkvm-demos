# Docker Usage Examples for ZKVM Projects

## 📖 Overview

This guide provides detailed examples of how to use Docker to build, execute, and prove programs using different ZKVMs (Nexus, Risc0, SP1, and ZKM).

## 🎯 Two Modes: Production vs Development

### Production Mode
- **Best for**: Quick demos, production deployment, CI/CD
- **Characteristics**: Pre-compiled binaries, fast startup
- **Commands**: Fixed at build time

### Development Mode
- **Best for**: Development, testing, experimentation
- **Characteristics**: Real-time code sync, flexible parameters
- **Commands**: Dynamic at runtime

---

## 1. 🔵 SP1 ZKVM

SP1 (Succinct Proof 1) supports both execution and proving modes.

### Quick Reference

| Mode | Command | Description |
|------|---------|-------------|
| Execute | `--execute` | Run program without generating proof |
| Prove | `--prove` | Generate and verify zero-knowledge proof |

### Development Mode (Recommended for Testing)

```bash
cd docker/scripts

# 1. Build toolchain image (first time only)
./development-manager.sh build sp1

# 2. Execute mode - Fast testing
./development-manager.sh run sp1 --execute
# Output: Executes program and shows cycle count

# 3. Prove mode - Generate proof
./development-manager.sh run sp1 --prove
# Output: Generates proof and verifies it

# 4. Interactive shell for debugging
./development-manager.sh shell sp1
# Inside container:
cd sp1-zkvm/sp1-host
RUST_LOG=info cargo run --release -- --execute
RUST_LOG=info cargo run --release -- --prove
```

### Production Mode

```bash
cd docker/scripts

# 1. Build precompiled image
./production-manager.sh build sp1

# 2. Run (default: --execute mode)
./production-manager.sh run sp1

# Note: Production mode has fixed command (--execute)
# To change, modify docker/production/Dockerfile.sp1 line 59
```

### Direct Docker Compose

```bash
cd docker

# Development mode with custom arguments
docker-compose -f development/docker-compose.yml run --rm sp1-dev bash -c "cd sp1-zkvm/sp1-host && cargo run --release -- --execute"
docker-compose -f development/docker-compose.yml run --rm sp1-dev bash -c "cd sp1-zkvm/sp1-host && cargo run --release -- --prove"

# Production mode
docker-compose -f production/docker-compose.yml --profile sp1 up sp1-zkvm
```

### SP1 Official Documentation
- Website: https://docs.succinct.xyz/
- GitHub: https://github.com/succinctlabs/sp1

---

## 2. 🟢 Nexus ZKVM

Nexus automatically performs both execution and proving.

### Quick Reference

| Feature | Behavior |
|---------|----------|
| Build | Compiles guest program |
| Execute | Runs in Nexus VM |
| Prove | Generates STARK proof |
| Verify | Verifies proof |

### Development Mode (Recommended)

```bash
cd docker/scripts

# 1. Build toolchain image
./development-manager.sh build nexus

# 2. Run (automatically proves and verifies)
./development-manager.sh run nexus --nocapture
# --nocapture: Shows stdout from guest program

# 3. Interactive debugging
./development-manager.sh shell nexus
# Inside container:
cd nexus-zkvm/nexus-host
cargo run -r -- --nocapture
```

### Production Mode

```bash
cd docker/scripts

# 1. Build precompiled image
./production-manager.sh build nexus

# 2. Run
./production-manager.sh run nexus
```

### Direct Docker Compose

```bash
cd docker

# Development mode
docker-compose -f development/docker-compose.yml run --rm nexus-dev bash -c "cd nexus-zkvm/nexus-host && cargo run -r -- --nocapture"

# Production mode
docker-compose -f production/docker-compose.yml --profile nexus up nexus-zkvm
```

### Nexus Official Documentation
- Website: https://docs.nexus.xyz/
- GitHub: https://github.com/nexus-xyz/nexus-zkvm

---

## 3. 🔴 Risc0 ZKVM

Risc0 automatically generates and verifies proofs.

### Quick Reference

| Feature | Behavior |
|---------|----------|
| Build | Compiles guest using risc0-build |
| Execute | Runs in Risc0 VM |
| Prove | Generates STARK proof (Succinct mode) |
| Verify | Verifies proof |

### Development Mode (Recommended)

```bash
cd docker/scripts

# 1. Build toolchain image
./development-manager.sh build risc0

# 2. Run (automatically proves and verifies)
./development-manager.sh run risc0
# Environment variables are pre-configured:
#   RISC0_DEV_MODE=1
#   RUST_LOG=info
#   RISC0_INFO=1

# 3. Interactive debugging
./development-manager.sh shell risc0
# Inside container:
cd risc0-zkvm/risc0-host
RUST_LOG=info RISC0_DEV_MODE=1 cargo run --release
```

### Production Mode

```bash
cd docker/scripts

# 1. Build precompiled image
./production-manager.sh build risc0

# 2. Run
./production-manager.sh run risc0
```

### Direct Docker Compose

```bash
cd docker

# Development mode
docker-compose -f development/docker-compose.yml run --rm risc0-dev bash -c "cd risc0-zkvm/risc0-host && cargo run --release"

# Production mode
docker-compose -f production/docker-compose.yml --profile risc0 up risc0-zkvm
```

### Risc0 Official Documentation
- Website: https://dev.risczero.com/
- GitHub: https://github.com/risc0/risc0

---

## 4. 🟡 ZKM (Zero Knowledge Mips)

ZKM provides proving functionality similar to SP1.

### Quick Reference

| Mode | Behavior |
|------|----------|
| Default | Generates and verifies proof |

### Development Mode (Recommended)

```bash
cd docker/scripts

# 1. Build toolchain image
./development-manager.sh build zkm

# 2. Run (automatically proves and verifies)
./development-manager.sh run zkm
# Default behavior: setup, prove, verify

# 3. Interactive debugging
./development-manager.sh shell zkm
# Inside container:
cd zkm-zkvm/zkm-host
RUST_LOG=info cargo run --release
```

### Production Mode

```bash
cd docker/scripts

# 1. Build precompiled image
./production-manager.sh build zkm

# 2. Run
./production-manager.sh run zkm
```

### Direct Docker Compose

```bash
cd docker

# Development mode
docker-compose -f development/docker-compose.yml run --rm zkm-dev bash -c "cd zkm-zkvm/zkm-host && cargo run --release"

# Production mode
docker-compose -f production/docker-compose.yml --profile zkm up zkm-zkvm
```

### ZKM Official Documentation
- Website: https://docs.zkm.io/
- GitHub: https://github.com/zkMIPS/zkm

---

## 🔄 Complete Workflow Examples

### Example 1: SP1 Full Development Cycle

```bash
cd docker/scripts

# Step 1: Build base image (shared by all ZKVMs)
./build-base.sh

# Step 2: Build SP1 toolchain image
./development-manager.sh build sp1

# Step 3: Modify your code (edit sp1-zkvm/sp1-guest/src/main.rs or sp1-host)
# Code changes are automatically reflected in the container!

# Step 4: Test with execute mode (fast)
./development-manager.sh run sp1 --execute

# Step 5: Generate proof when ready
./development-manager.sh run sp1 --prove

# Step 6: Check logs if needed
./development-manager.sh logs sp1
```

### Example 2: Compare All ZKVMs

```bash
cd docker/scripts

# Build all toolchain images
for zkvm in nexus risc0 sp1 zkm; do
    ./development-manager.sh build $zkvm
done

# Test execution on all ZKVMs
./development-manager.sh run sp1 --execute
./development-manager.sh run nexus --nocapture
./development-manager.sh run risc0
./development-manager.sh run zkm

# Generate proofs on all ZKVMs
./development-manager.sh run sp1 --prove
./development-manager.sh run nexus
./development-manager.sh run risc0
./development-manager.sh run zkm
```

### Example 3: Production Deployment

```bash
cd docker/scripts

# Build production images for deployment
./production-manager.sh build sp1
./production-manager.sh build nexus

# Run in background (detached mode)
docker-compose -f docker/production/docker-compose.yml --profile sp1 up -d sp1-zkvm
docker-compose -f docker/production/docker-compose.yml --profile nexus up -d nexus-zkvm

# Check status
./production-manager.sh ps

# View logs
./production-manager.sh logs sp1
./production-manager.sh logs nexus
```

---

## 🎓 Understanding Build, Execute, and Prove

### Build Phase

**What happens:**
- Guest program is compiled to target architecture (RISC-V, MIPS, etc.)
- Host program links with guest ELF binary
- SDK-specific build tools prepare the executable

**In Docker:**
```bash
# Development mode - builds on first run
./development-manager.sh run sp1 --execute

# Production mode - pre-built in image
./production-manager.sh build sp1
```

### Execute Phase

**What happens:**
- Guest program runs in the VM
- No proof generation (faster)
- Good for testing and debugging
- Shows cycle count and outputs

**Supported by:**
- ✅ SP1 (with `--execute` flag)
- ✅ All others (but they also prove by default)

**Example:**
```bash
./development-manager.sh run sp1 --execute
```

### Prove Phase

**What happens:**
- Guest program runs in the VM
- Execution trace is recorded
- Zero-knowledge proof is generated
- Proof is verified
- Takes longer but provides cryptographic guarantee

**Supported by:**
- ✅ All ZKVMs (SP1, Nexus, Risc0, ZKM)

**Example:**
```bash
./development-manager.sh run sp1 --prove
```

---

## 🔍 Debugging Tips

### View Detailed Logs

```bash
# Set RUST_LOG environment variable
cd docker/scripts

# For SP1
RUST_LOG=debug ./development-manager.sh run sp1 --execute

# For Risc0 (pre-configured in docker-compose)
RISC0_INFO=1 ./development-manager.sh run risc0
```

### Interactive Shell Debugging

```bash
cd docker/scripts

# Enter container
./development-manager.sh shell sp1

# Inside container - manual testing
cd sp1-zkvm/sp1-host
cargo build --release
RUST_LOG=debug cargo run --release -- --execute
RUST_LOG=debug cargo run --release -- --prove
```

### Check Container Status

```bash
cd docker/scripts

# View running containers
./development-manager.sh ps
./production-manager.sh ps

# View logs
docker logs sp1-zkvm-dev
docker logs -f sp1-zkvm-dev  # follow logs
```

### Build Cache Issues

```bash
cd docker/scripts

# Clean development cache
./development-manager.sh clean-cache

# Clean production images
./production-manager.sh clean

# Rebuild from scratch
docker system prune -a
./build-base.sh
./development-manager.sh build sp1
```

---

## 📊 Performance Comparison

| ZKVM | Execute Time | Prove Time | Proof Size | Verification |
|------|--------------|------------|------------|--------------|
| SP1 | ~1s | ~30s-2m | Small | Fast |
| Nexus | N/A | ~1-5m | Medium | Fast |
| Risc0 | N/A | ~30s-3m | Small | Fast |
| ZKM | N/A | ~1-5m | Small | Fast |

*Note: Times are approximate and depend on program complexity*

---

## 🚀 Best Practices

### 1. Use Development Mode for Iteration

```bash
# Fast iteration cycle
./development-manager.sh run sp1 --execute  # Quick test
# Edit code
./development-manager.sh run sp1 --execute  # Test again
# When satisfied
./development-manager.sh run sp1 --prove    # Generate proof
```

### 2. Cache Management

```bash
# Development mode uses persistent volumes
# List cache volumes
docker volume ls | grep cache

# Clean old cache if disk space is low
./development-manager.sh clean-cache
```

### 3. Production Deployment

```bash
# Build production images with specific tags
docker build -f docker/production/Dockerfile.sp1 -t sp1-zkvm:v1.0 ../..
docker build -f docker/production/Dockerfile.nexus -t nexus-zkvm:v1.0 ../..

# Run with resource limits
docker run --memory="4g" --cpus="2" sp1-zkvm:v1.0
```

### 4. CI/CD Integration

```bash
# In your CI pipeline
cd docker/scripts

# Build and test
./build-base.sh
./development-manager.sh build sp1
./development-manager.sh run sp1 --execute

# Or use production images
./production-manager.sh build sp1
./production-manager.sh run sp1
```

---

## 📚 Additional Resources

### Official Documentation Links

- **SP1**: https://docs.succinct.xyz/
- **Nexus**: https://docs.nexus.xyz/
- **Risc0**: https://dev.risczero.com/
- **ZKM**: https://docs.zkm.io/

### Project Documentation

- [Main README](../README.md) - Quick start and overview
- [Usage Guide](USAGE-GUIDE.md) - Detailed Docker usage
- [Troubleshooting](TROUBLESHOOTING.md) - Common issues

### Example Code

All example code is in the `zkvm-demos` repository:
- `sp1-zkvm/` - SP1 Fibonacci example
- `nexus-zkvm/` - Nexus Fibonacci example
- `risc0-zkvm/` - Risc0 Fibonacci example
- `zkm-zkvm/` - ZKM Fibonacci example

---

## 🎉 Summary

### Quick Command Reference

```bash
cd docker/scripts

# Build
./build-base.sh                        # Build shared base image
./development-manager.sh build sp1     # Build SP1 toolchain
./production-manager.sh build sp1      # Build SP1 production

# Run
./development-manager.sh run sp1 --execute    # SP1 execute mode
./development-manager.sh run sp1 --prove      # SP1 prove mode
./development-manager.sh run nexus            # Nexus (auto-prove)
./development-manager.sh run risc0            # Risc0 (auto-prove)
./development-manager.sh run zkm              # ZKM (auto-prove)

# Debug
./development-manager.sh shell sp1     # Interactive shell
./development-manager.sh logs sp1      # View logs
./development-manager.sh ps            # Container status

# Cleanup
./development-manager.sh clean-cache   # Clear build cache
./production-manager.sh clean          # Remove images
```

**Happy proving! 🚀**

