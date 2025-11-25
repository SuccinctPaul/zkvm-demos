# ZisK Docker Usage Guide

Since ZisK does not support proof generation on macOS, this guide explains how to use Docker to generate proofs on macOS via a Linux container.

---

## 🎯 Why Docker?

**ZisK Limitations:**
- ✅ macOS: Supports dev and test (compile, emulator)
- ❌ macOS: **Does not support proof generation** (ROM setup, prove, verify)
- ✅ Linux x86_64: Fully supports all features

**Docker Solution:**
- Run Linux x86_64 container on macOS
- Full proof generation support
- Code mounting, no rebuild needed

---

## 🚀 Quick Start

### 1. First Setup

```bash
# Enter docker directory
cd /Users/paul/zkp/zkvms/zkvm-demos/docker

# Build base image (only first time, ~5-10 mins)
cd scripts
./build-base.sh
cd ..

# Build ZisK image (~10-15 mins, includes all dependencies)
docker compose build zisk-zkvm
```

### 2. Test Mode (Quick Verification)

```bash
# Run test: Build + Emulator execution
docker compose --profile zisk up zisk-zkvm
```

**Expected Output:**
```
ZisK ZKVM - Fibonacci Demo
Mode: test
FIBONACCI_N: 10

=== Building guest program ===
Compiling zisk-guest v0.1.0
Finished `release` profile [optimized] target(s) in 1.2s

=== Testing with emulator ===
Computing Fibonacci for n = 10
Fibonacci(10) = 89
00000059

To generate proof, set ZKVM_MODE=prove
```

### 3. Generate Proof

```bash
# Full flow: Build + Exec + ROM setup + Prove + Verify
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

**First run includes ROM setup (2-5 mins), subsequent runs are faster.**

---

## 📋 Common Commands

### Basic Operations

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/docker

# Test Mode (Default, Fast)
docker compose --profile zisk up zisk-zkvm

# Proof Mode (Full Flow)
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Custom Fibonacci Value
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Run in background
docker compose --profile zisk up -d zisk-zkvm

# View logs
docker compose logs -f zisk-zkvm

# Stop containers
docker compose down
```

### Interactive Shell (Recommended, Full Control)

```bash
# Enter container
docker compose run --rm zisk-zkvm bash

# Execute commands inside container
cd zisk-guest

# Build
cargo-zisk build --release

# Test
cargo-zisk run --release -i ../build/input.bin

# ROM setup (First time only)
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest

# Generate proof
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y

# Verify proof
cargo-zisk verify -p ../proof/vadcop_final_proof.bin

# Exit container
exit
```

---

## 🔧 Environment Variables

| Variable | Default | Description |
|-----|-------|------|
| `ZKVM_MODE` | `test` | `test`: Test only, `prove`: Generate proof |
| `FIBONACCI_N` | `10` | Nth Fibonacci number |
| `RUST_LOG` | `info` | Log level |

**Usage Example:**
```bash
# Compute Fibonacci(30) and generate proof
FIBONACCI_N=30 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 📊 Performance Expectations

### macOS Apple Silicon (M1/M2/M3)

Performance is reduced due to x86_64 emulation:

| Operation | Native Linux | Docker on macOS (Emulation) |
|-----|-----------|----------------------|
| Build | ~1-2s | ~2-3s |
| Execute (Emulator) | <0.01s | <0.1s |
| ROM setup | 2-5 mins | 5-10 mins |
| Proof Generation | 30-45s | 60-90s |
| Proof Verification | ~2s | ~4s |

**Optimization Tips:**
- ROM setup is once-only, results cached
- Subsequent proof generation skips ROM setup, faster
- Use interactive shell to avoid restarting container

---

## 💾 Data Persistence

Docker uses volumes to cache data, avoiding repeated downloads and builds:

```bash
# View ZisK related volumes
docker volume ls --filter "name=zisk"

# Output:
# zisk-cargo-cache    - Rust dependency cache
# zisk-target-cache   - Build artifact cache
# zisk-zisk-cache     - ZisK toolchain and ROM setup cache
```

**Clean Cache (If needed):**
```bash
# Clean all ZisK caches
docker volume rm docker_zisk-cargo-cache docker_zisk-target-cache docker_zisk-zisk-cache

# Or clean all unused volumes
docker volume prune
```

---

## 🔍 Troubleshooting

### Issue 1: "zkvm-base:latest not found"

**Reason:** Base image not built

**Solution:**
```bash
cd docker/scripts
./build-base.sh
```

### Issue 2: ROM setup is slow

**Reason:** x86_64 emulation on macOS, normal behavior

**Solution:**
- First time slow (5-10 mins), but cached
- Subsequent runs don't need ROM setup
- If `zisk-zisk-cache` volume is cleaned, setup needed again

### Issue 3: Proof Generation Failed

**Check:**
```bash
# Enter container to check detailed error
docker compose run --rm zisk-zkvm bash
cd zisk-guest
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y
```

**Common Causes:**
- ROM setup not completed
- Input file missing (Run `cargo build` first to generate input.bin)
- Insufficient disk space

### Issue 4: Insufficient Disk Space

**Check:**
```bash
docker system df
```

**Clean:**
```bash
# Clean unused images and containers
docker system prune

# Clean all (including cache volumes)
docker system prune -a --volumes
```

### Issue 5: Code Changes Not Effective

**Reason:** Code is mounted, no image rebuild needed

**Correct Action:**
```bash
# After modifying code, just run
vim zisk-zkvm/zisk-guest/src/main.rs
docker compose --profile zisk up zisk-zkvm  # No --build needed
```

**When Rebuild is Needed:**
- Only after modifying `Cargo.toml` or `Dockerfile`
```bash
docker compose build zisk-zkvm
```

---

## 📝 Complete Workflow Example

### Scenario 1: Dev & Test (macOS Local)

```bash
# 1. Modify code locally
vim zisk-zkvm/zisk-guest/src/main.rs

# 2. Test locally (Fast)
cd zisk-zkvm/zisk-guest
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin

# 3. Commit code after verification
git add .
git commit -m "Update fibonacci implementation"
```

### Scenario 2: Generate Proof (Docker)

```bash
# 1. Generate proof using Docker
cd docker
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 2. View generated proof
ls -lh ../zisk-zkvm/proof/

# 3. proof file saved in host's zisk-zkvm/proof/ directory
```

### Scenario 3: Batch Generate Proofs for Different Inputs

```bash
cd docker

# Fibonacci(10)
FIBONACCI_N=10 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Fibonacci(20)  
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Fibonacci(30)
FIBONACCI_N=30 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 🎓 Advanced Usage

### Interactive Shell Debugging

```bash
# Enter container
docker compose run --rm zisk-zkvm bash

# Check toolchain
cargo-zisk --version
ziskemu --version
rustup toolchain list | grep zisk

# View file structure
ls -la
cd zisk-guest
ls -la target/riscv64ima-zisk-zkvm-elf/release/

# Check ROM setup status
ls -la ~/.zisk/

# Run single step
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin

# Exit
exit
```

### Mount Extra Directories

Modify `docker-compose.yml`:
```yaml
volumes:
  - ../:/workspace
  - /path/to/custom/data:/data  # Add custom mount
```

### Use GPU Acceleration (Requires NVIDIA GPU + Linux)

Modify `Dockerfile.zisk`, remove `CI=true`:
```dockerfile
# Before:
CI=true /tmp/install_zisk_sdk.sh

# After:
/tmp/install_zisk_sdk.sh
```

Then add GPU support in `docker-compose.yml`:
```yaml
zisk-zkvm:
  deploy:
    resources:
      reservations:
        devices:
          - driver: nvidia
            count: all
            capabilities: [gpu]
```

---

## 📚 References

- [ZisK Official Docs](https://0xpolygonhermez.github.io/zisk/)
- [ZisK GitHub](https://github.com/0xPolygonHermez/zisk)
- [Docker Official Docs](https://docs.docker.com/)

---

## ✅ Summary

### Advantages
- ✅ Can generate proofs on macOS
- ✅ Full Linux environment
- ✅ Caching mechanism, avoid repeated builds
- ✅ Code mounting, instant changes

### Notes
- ⚠️ First build takes 10-15 mins
- ⚠️ Emulation on macOS reduces performance by 2-3x
- ⚠️ ROM setup is one-time, takes 5-10 mins
- ⚠️ Sufficient disk space needed (10GB+ recommended)

### Recommended Practices
1. Dev and test on macOS (use local ziskemu)
2. Use Docker to generate proof
3. If frequent proof generation needed, consider Linux machine or VM

---

**Created**: 2025-11-16  
**Version**: ZisK 0.10.0  
**Platform**: macOS (Docker) / Linux (Native)
