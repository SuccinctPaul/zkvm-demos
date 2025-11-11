# zkvm-demos

A collection of Zero-Knowledge Virtual Machine (zkVM) demonstrations for SP1, Risc0, Nexus, ZKM, ZisK, Valida, OpenVM,
Pico, CENO, and Cairo. This repository shows how to build and run programs on different zkVM platforms.

## ⚠️ Toolchain Conflict Management

**Important:** Installing multiple zkVMs on the same machine can cause conflicts between toolchains, Rust versions, and
CLI tools. We provide two solutions:

### 🐳 Docker Isolation (Recommended for Most Users)

Complete isolation using Docker containers - works out of the box with minimal setup.

```bash
# Quick start
cd docker/scripts
./build-base.sh
./development-manager.sh build sp1
./development-manager.sh run sp1 --execute
```

**Advantages:**

- ✅ No conflicts between zkVMs
- ✅ Easy team collaboration
- ✅ Consistent CI/CD integration
- ✅ Simple cleanup

📖 **Full Guide:** [docs/ISOLATION-DOCKER.md](docs/ISOLATION-DOCKER.md) | [中文版](docs/ISOLATION-DOCKER.zh-CN.md)

### 🏠 Local Environment Managers (Advanced Users)

Native performance using workspace-based isolation with optional direnv for automatic switching.

```bash
# Setup workspaces
./scripts/local_env/setup-sp1-workspace.sh ~/zkvm-workspaces/sp1-workspace
./scripts/local_env/setup-risc0-workspace.sh ~/zkvm-workspaces/risc0-workspace

# Activate workspace
source ~/zkvm-workspaces/sp1-workspace/activate.sh
```

**Advantages:**

- ✅ Native performance (5-10% faster)
- ✅ Seamless IDE integration
- ✅ Automatic switching with direnv
- ✅ Full control over environment

📖 **Full Guide:
** [docs/ISOLATION-LOCAL-ENV.md](docs/ISOLATION-LOCAL-ENV.md) | [中文版](docs/ISOLATION-LOCAL-ENV.zh-CN.md)

### 📊 Which Approach Should You Use?

| Your Situation             | Recommended Solution |
|----------------------------|----------------------|
| Working in a team          | 🐳 Docker            |
| Need CI/CD                 | 🐳 Docker            |
| Just getting started       | 🐳 Docker            |
| Solo dev, frequent builds  | 🏠 Local + direnv    |
| Need hardware access (GPU) | 🏠 Local             |

📖 **Detailed Comparison:
** [docs/ISOLATION-COMPARISON.md](docs/ISOLATION-COMPARISON.md) | [中文版](docs/ISOLATION-COMPARISON.zh-CN.md)

### 🔍 Understanding the Conflicts

Learn about specific conflicts and why isolation is necessary:

📖 **Conflict Analysis:
** [docs/TOOLCHAIN-CONFLICTS.md](docker/docs/TOOLCHAIN-CONFLICTS.mdICTS.md) | [中文版](docs/TOOLCHAIN-CONFLICTS.zh-CN.md)

---

## Running Individual ZKVMs

The following sections show how to run each zkVM **without isolation** (not recommended if you plan to use multiple
zkVMs). For production use, see the isolation guides above.

## CENO zkvm

### Resources

* https://eprint.iacr.org/2024/387
* https://scroll.io/blog/ceno
* https://github.com/scroll-tech/ceno (expected)

### how to run the CENO demo

⚠️ **Note**: CENO zkVM is under active development by Scroll. This is a template implementation that will be updated
once the CENO SDK is publicly available.

* cd to the target demo directory

```bash
cd ceno-zkvm/ceno-host
```

* run the CENO demo (once SDK is available)

```bash
RUST_LOG=info cargo run --release
```

## Cairo zkvm

### Resources

* https://www.cairo-lang.org/docs/
* https://github.com/starkware-libs/cairo-lang
* https://book.cairo-lang.org/

### how to run the Cairo demo

* Install Cairo first (if not already installed):

```bash
# Option 1: Use the installation script
./scripts/sdk_installers/install_cairo_sdk.sh

# Option 2: Manual installation
pip install cairo-lang
```

* cd to the target demo directory

```bash
cd cairo-zkvm
```

* run the Cairo demo

```bash
cairo-run --program=src/fib_simple.cairo --print_output --layout=small
```

* run with compiled JSON (alternative method)

```bash
# First compile
cairo-compile src/fib_simple.cairo --output fib_simple.json

# Then run
cairo-run --program=fib_simple.json --print_output --layout=small
```

## Nexus zkvm

### Resources

* https://docs.nexus.xyz/zkvm/nexus-zkvm
* https://github.com/nexus-xyz/nexus-zkvm

### how to run the Nexus demo

* cd to the target demo directory

```bash
cd nexus-zkvm/nexus-host
```

* run the Nexus demo

```bash
RUST_LOG=info cargo run -r -- --nocapture
```

## OpenVM zkvm

### Resources

* https://docs.openvm.dev/
* https://github.com/openvm-org/openvm

### how to run the OpenVM demo

* cd to the target demo directory

```bash
cd openvm-zkvm/openvm-host
```

* run the OpenVM demo

```bash
RUST_LOG=info cargo run --release
```

## Pico zkvm

### Resources

* https://docs.brevis.network/
* https://github.com/brevis-network/pico

### how to run the Pico demo

* cd to the target demo directory

```bash
cd pico-zkvm/pico-host
```

* run the Pico demo

```bash
RUST_LOG=info cargo run --release
```

## Risc0 zkvm

### Resources

* https://dev.risczero.com/api/zkvm/quickstart
* https://github.com/risc0/risc0

### how to run the Risc0 demo

* cd to the target demo directory

```bash
cd risc0/risc0-host
```

* run in dev mode

```bash
RISC0_DEV_MODE=1 RUST_LOG=info RISC0_INFO=1 cargo run --release
```

* run in production mode

```bash
RISC0_DEV_MODE=0 RUST_LOG=debug RISC0_INFO=1 cargo run --release
```

## Sp1 zkvm

### Resources

* https://docs.succinct.xyz/docs/sp1/getting-started/quickstart
* https://github.com/succinctlabs/sp1

### how to run the Sp1 demo

* cd to the target demo directory

```bash
cd sp1-zkvm/sp1-host
```

* run in dev mode

```bash
RUST_LOG=info cargo run --release -- --execute
```

* run in production mode

```bash
RUST_LOG=debug cargo run --release -- --prove
```

## Valida zkvm

### Resources

* https://www.lita.foundation/blog/introducing-valida-zkvm-1-0
* https://github.com/litaio/valida
* https://www.lita.foundation/blog/announcing-valida-0-10-0

### how to run the Valida demo

Valida zkVM primarily uses C for guest programs and provides both Docker and local toolchain options.

#### Option 1: Using Docker (Recommended)

* Pull the Valida Docker image

```bash
docker pull lita-xyz/valida
```

* Navigate to the demo directory

```bash
cd valida-zkvm
```

* Run the complete workflow using Docker

```bash
# Compile the guest program
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida-cc -o /workspace/fib.elf /workspace/valida-guest/fib.c

# Execute in the zkVM
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida run /workspace/fib.elf

# Generate proof
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida prove /workspace/fib.elf -o /workspace/proof.bin

# Verify proof
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida verify /workspace/proof.bin
```

#### Option 2: Using Local Toolchain

* Install the Valida toolchain (requires LLVM 18.1.7+ and Rust 1.86+)

* Navigate to the demo directory

```bash
cd valida-zkvm
```

* Run the compilation and proving workflow

```bash
# Compile the guest program
valida-cc -o fib.elf valida-guest/fib.c

# Execute in the zkVM
valida run fib.elf

# Generate proof
valida prove fib.elf -o proof.bin

# Verify proof
valida verify proof.bin
```

#### Option 3: Using the Rust Host Program (Demo Structure)

* Navigate to the host directory

```bash
cd valida-zkvm/valida-host
```

* Run the demonstration

```bash
RUST_LOG=info cargo run --release
```

**Note:** The Rust host program is a demonstration structure. For actual Valida zkVM usage, use Docker or the local
toolchain.

## ZKM zkvm

### Resources

* https://docs.zkm.io/introduction/quickstart.html
* https://github.com/ProjectZKM/Ziren

### how to run

* cd to the target demo directory

```bash
cd zkm-zkvm/zkm-host
```

* run in dev mode

```bash
RUST_LOG=info cargo run --release -- --execute
```

* run in production mode

```bash
RUST_LOG=info cargo run --release -- --<PROOF_TYPE> // for core and compressed proofs 
RUST_LOG=info cargo run --release --bin evm -- --system <PROOF_TYPE>  // for EVM-compatible proofs
```

* core proof

```bash
RUST_LOG=debug cargo run --release -- --core
```

* compressed proof

```bash
RUST_LOG=debug cargo run --release -- --compressed
```

* evm proof

```bash
cargo run --release --bin evm -- --system groth16

cargo run --release --bin evm -- --system plonk
```

## ZisK zkvm

### Resources

* https://github.com/0xPolygonHermez/zisk
* https://polygon.technology/blog/zisk-announcement
* Performance: 1.5 GHz RISC-V trace generation (~10x faster than other zkVMs)

### Installation

```bash
cd scripts/sdk_installers
./install_zisk_sdk.sh
```

This will:

- Install `ziskup` toolchain manager
- Install `cargo-zisk` CLI tool (v0.10.0)

### How to Run

* Navigate to the ZisK demo directory:

```bash
cd zisk-zkvm/zisk-host
```

* Build the project:

```bash
cargo build --release
```

* Run the demo:

```bash
RUST_LOG=info cargo run --release
```

### GPU Acceleration (Optional)

For NVIDIA GPUs with CUDA:

```bash
# Build with GPU support (done by install script)
# Run with GPU acceleration
cargo-zisk-gpu run --release
```