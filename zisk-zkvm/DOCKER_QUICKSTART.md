# ZisK Docker Quick Start

Quick guide to generating ZisK proofs on macOS using Docker.

---

## ⚡ One-Click Start

```bash
# 1. Build base image (first time, 5-10 mins)
cd docker/scripts && ./build-base.sh && cd ..

# 2. Build ZisK image (first time, 10-15 mins)
docker compose build zisk-zkvm

# 3. Test run (1-2 mins)
docker compose --profile zisk up zisk-zkvm

# 4. Generate proof (first time 5-10 mins incl. ROM setup, then 1-2 mins)
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 🎯 Common Commands

```bash
cd docker

# Test (Quick Verification)
docker compose --profile zisk up zisk-zkvm

# Generate proof
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Custom Input
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Interactive shell
docker compose run --rm zisk-zkvm bash
```

---

## 📋 Commands Inside Container

```bash
# Enter container
docker compose run --rm zisk-zkvm bash

# Execute commands
cd zisk-guest
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y
cargo-zisk verify -p ../proof/vadcop_final_proof.bin
```

---

## ✅ Expected Output

### Test Mode
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
```

### Proof Mode
```
=== Generating proof ===
[INFO] Generating proof...
[INFO] Proof generated successfully

=== Verifying proof ===
[INFO] ProofMan:     ✓ Vadcop Final proof was verified
```

---

## ⏱️ Expected Time (macOS Apple Silicon)

| Operation | First Time | Subsequent |
|-----|------|------|
| Base Image Build | 5-10 mins | - |
| ZisK Image Build | 10-15 mins | - |
| Test Run | 1-2 mins | 30-60 secs |
| ROM setup | 5-10 mins | (Cached, skipped)|
| Proof Generation | 1-2 mins | 1-2 mins |

---

## 🔍 FAQ

**Q: Why is Docker needed?**  
A: ZisK does not support proof generation on macOS, requiring a Linux x86_64 environment.

**Q: Why is it so slow the first time?**  
A: Needs to download images, install dependencies, ROM setup. These are cached, subsequent runs are much faster.

**Q: Do I need to rebuild the image after modifying code?**  
A: No! Code is mounted, just run it.

**Q: Where is the proof file?**  
A: In the `zisk-zkvm/proof/` directory, automatically synced to host.

---

## 📚 Detailed Documentation

See `DOCKER_GUIDE.md` for full usage instructions and troubleshooting.

---

**Quick Start Complete!** 🎉
