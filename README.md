# zkvm-demos

A comprehensive collection of Zero-Knowledge Virtual Machine (zkVM) demonstrations and benchmarks. This repository integrates multiple zkVM implementations into a unified framework for building, running, and benchmarking zero-knowledge proofs.

## 🚀 Project Overview

This project provides a standardized environment to explore and benchmark various zkVMs. It includes:

- **Unified Benchmarking Framework**: Consistent metrics collection (execution time, proof size, memory usage) via `zkvm_benchmark_utils`.
- **Multi-Program Support**: Standardized guest programs (Fibonacci, Hash, Signature Verification) ported across different zkVMs.
- **Isolation Strategies**: Docker and local workspace management to handle conflicting toolchains.

## 📊 zkVM Support Matrix

This table summarizes the supported programs and configuration status for each zkVM in this repository.

| zkVM | Supported Programs | Enabled in `@configs` | Implementation Status / Notes |
| :--- | :--- | :--- | :--- |
| **Airbender** | Fib, Hash, Sig | ✅ True | Full SDK Integration (Nightly Rust) |
| **Cairo** | Fib, Hash, Sig | ✅ True | Full SDK Support (Stwo/Cairo) |
| **Cairo-M** | Fib, Hash, Sig | ❌ False | Requires `cairo-m-compiler` (Native) |
| **Ceno** | Fib, Hash, Sig | ❌ False | Reference Mode (SDK not public) |
| **Jolt** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **Lean** | Fib, Hash, Sig | ❌ False | Reference Mode (SDK not public) |
| **Miden** | Fib, Sum, Factorial | ✅ True | MASM Multi-Program Support |
| **Nexus** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **Novanet** | Fib, Hash, Sig | ✅ True | Reference Mode (WASM runner limited) |
| **o1vm** | Fib, Hash, Sig | ❌ False | Reference Mode (MIPS proving complex) |
| **OpenVM** | Fib, Hash, Sig | ❌ False | Integrated (Disabled by default) |
| **Pico** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **Powdr** | Fib, Hash, Sig | ✅ True | Full SDK Support (with fallback) |
| **Risc0** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **SnarkVM** | Fib, Hash, Sig | ✅ True | Full SDK Support (Aleo) |
| **SP1** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **Valida** | Fib, Hash, Sig | ✅ True | Full SDK Support (CLI) |
| **Zisk** | Fib, Hash, Sig | ❌ False | Requires Linux/Docker (OS dependent) |
| **ZKM** | Fib, Hash, Sig | ✅ True | Full SDK Support |
| **zkWasm** | Fib, Hash, Sig | ✅ True | Full SDK Support (Delphinus CLI) |

> **Note**: "Reference Mode" indicates that the zkVM integration is structurally complete but uses simulated or placeholder proof generation due to pending SDK releases or complex setup requirements.

## ⚠️ Toolchain Conflict Management

**Important:** Installing multiple zkVMs on the same machine can cause conflicts between toolchains, Rust versions, and CLI tools. We provide two solutions:

### 🐳 Docker Isolation (Recommended)
Complete isolation using Docker containers.
```bash
cd docker/scripts
./build-base.sh
./development-manager.sh build sp1
./development-manager.sh run sp1 --execute
```
[Full Guide](docs/ISOLATION-DOCKER.md)

### 🏠 Local Environment Managers
Native performance using workspace-based isolation.
```bash
./scripts/local_env/setup-sp1-workspace.sh ~/zkvm-workspaces/sp1-workspace
source ~/zkvm-workspaces/sp1-workspace/activate.sh
```
[Full Guide](docs/ISOLATION-LOCAL-ENV.md)

## Reference
* https://github.com/blocksense-network/zkVMs-benchmarks
* https://github.com/kkrt-labs/zkvm-benchmarks
