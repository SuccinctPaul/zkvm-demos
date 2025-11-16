# zkvm-demos

A collection of Zero-Knowledge Virtual Machine (zkVM) demonstrations for SP1, Risc0, Nexus, Novanet, ZKM, ZisK, Valida, OpenVM,
Pico, Powdr, CENO, Cairo, Cairo-M, Jolt, Lean, Miden, Airbender, and other zkVM platforms. This repository shows how to build and run programs on different zkVM implementations.

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

## Airbender zkvm

### Resources

* https://docs.zksync.io/zk-stack/components/zksync-airbender
* https://github.com/eth-act/ere (reference implementation)
* Performance: ~21.8 MHz proving speed on H100 GPU (6x faster than competitors)
* Transaction cost: ~$0.0001 per transaction

### how to run the Airbender demo

⚠️ **Note**: Airbender is zkSync's high-performance RISC-V zkVM. This is a reference implementation showing the expected workflow. The actual SDK integration will be updated once officially released.

* cd to the target demo directory

```bash
cd airbender-zkvm
```

* run the Airbender demo

```bash
# Using the convenient run script
./run_demo.sh

# Or directly with cargo
cargo run --release --bin airbender-host
```

* run with custom Fibonacci input

```bash
# Set custom input value
FIB_N=15 ./run_demo.sh

# Or with cargo
FIB_N=15 cargo run --release --bin airbender-host
```

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

## Cairo-M zkvm

### Resources

* https://github.com/kkrt-labs/cairo-m
* Field: M31 (Mersenne 31) prime field
* Prover: Stwo (Starkware's STARK prover)
* Target: Mobile-first proving on consumer devices

### About Cairo-M

Cairo-M is a Mobile-first CPU AIR (zkVM) optimized for consumer hardware:

- **M31 Field**: Uses Mersenne 31 prime (2^31-1) for efficient mobile arithmetic
- **Minimal Registers**: Only PC and FP for simplified constraints
- **Stwo Integration**: Built on Starkware's Stwo prover for efficient STARKs
- **Mobile-Optimized**: Designed to generate proofs on phones and edge devices
- **Cairo-like Syntax**: Familiar language for Cairo developers

### how to run the Cairo-M demo

* Install Cairo-M first (if not already installed):

```bash
# Use the installation script
cd scripts/sdk_installers
./install_cairo_m_sdk.sh
```

This will install:
- `cairo-m-compiler` - Compiles .cm files
- `cairo-m-runner` - Executes Cairo-M programs
- `cairo-m-prover` - Generates STARK proofs with Stwo
- `cargo-cairo-m` - Project scaffolding tool

* Navigate to the demo directory

```bash
cd cairo-m-zkvm
```

* Run the demo

```bash
# Using the convenient run script
./run_demo.sh

# Or directly with cargo
cd cairo-m-host
RUST_LOG=info cargo run --release
```

* Run with custom Fibonacci input

```bash
# Set custom input value
FIBONACCI_N=20 ./run_demo.sh

# Or with cargo
FIBONACCI_N=20 RUST_LOG=info cargo run --release
```

* Manual workflow (compile → run → prove)

```bash
# Compile Cairo-M program
cairo-m-compiler --input programs/fibonacci.cm --output compiled/fibonacci.json

# Execute and generate trace
cairo-m-runner compiled/fibonacci.json --entrypoint fibonacci --arguments 10

# Generate STARK proof
cairo-m-prover compiled/fibonacci.json --entrypoint fibonacci --arguments 10
```

**Key Features:**
- 📱 **Mobile-Optimized**: Efficient proving on phones (~3-5s for typical programs)
- 🔢 **M31 Field**: Fast 32-bit arithmetic for mobile processors
- ⚡ **Stwo Prover**: Advanced STARK optimizations from Starkware
- 🎯 **Minimal Design**: Only 2 registers (PC, FP) for simplified constraints
- 📝 **Cairo-like**: Familiar syntax for Cairo developers

**See also:**
- [QUICK_START.md](cairo-m-zkvm/QUICK_START.md) - Get started in 5 minutes
- [PROJECT_OVERVIEW.md](cairo-m-zkvm/PROJECT_OVERVIEW.md) - Architecture deep dive
- [GitHub Repository](https://github.com/kkrt-labs/cairo-m) - Official Cairo-M repo
- [CairoMlings](https://github.com/kkrt-labs/cairo-m/tree/main/tutorials/cairomlings) - Interactive tutorial

## Jolt zkvm

### Resources

* https://jolt.a16zcrypto.com/
* https://github.com/a16z/jolt
* https://eprint.iacr.org/2023/1217

### how to run the Jolt demo

* Install Jolt first (if not already installed):

```bash
# Option 1: Use the installation script
./scripts/sdk_installers/install_jolt_sdk.sh

# Option 2: Manual installation
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
jolt install-toolchain
```

* cd to the target demo directory

```bash
cd jolt-zkvm/jolt-host
```

* run the Jolt demo

```bash
FIBONACCI_N=10 RUST_LOG=info cargo run --release
```

## Lean zkvm

### Resources

* https://github.com/leanEthereum/leanMultisig
* XMSS + minimal zkVM = lightweight post-quantum signatures with unbounded aggregation
* Performance: 1.0-1.7 MHz on consumer hardware (i9-12900H: 1.0 MHz, M4 Max: 1.7 MHz)
* Proof size: 400-500 KiB (target: 128-256 KiB with optimizations)
* Security: ~128 bits (WHIR + SuperSpartan)

### About Lean zkVM

Lean zkVM (leanMultisig) is a **minimal, high-performance zkVM** designed for:
- **Post-Quantum Ready**: XMSS signature aggregation
- **Ultra-fast proving**: 1.0-1.7 MHz on consumer hardware
- **Compact proofs**: Target 128-256 KiB (currently 400-500 KiB)
- **Advanced proof systems**: WHIR, SuperSpartan (AIR-optimized), Univariate Skip
- **Cairo-inspired design**: Minimal VM with efficient constraints

Key Features:
- 🚀 **Blazing Fast**: 1.7 MHz on M4 Max, 2x faster than competitors
- 🔐 **Post-Quantum**: XMSS signature unbounded aggregation
- 📦 **Compact**: Targeting 128-256 KiB proofs
- 🎯 **Minimal**: Cairo-inspired, optimized for efficiency
- ⚡ **Modern**: KoalaBear field, Poseidon2, WHIR + SuperSpartan

### how to run the Lean demo

⚠️ **Note**: Lean zkVM (leanMultisig) is under active development. The SDK is not yet publicly available. This is a **reference implementation** showing the expected workflow and structure.

* Install dependencies (optional, prepares environment):

```bash
# Option 1: Use the installation script
./scripts/sdk_installers/install_lean_sdk.sh

# Option 2: Manual setup (just ensure Rust 1.85+ is installed)
rustup toolchain install 1.85
```

* cd to the target demo directory

```bash
cd lean-zkvm
```

* run the Lean demo

```bash
# Using the convenient run script
./run_demo.sh

# Or directly with cargo
cd lean-host
FIBONACCI_N=10 RUST_LOG=info cargo run --release
```

* run with custom Fibonacci input

```bash
# Set custom input value
./run_demo.sh 20

# Or with environment variable
FIBONACCI_N=30 ./run_demo.sh
```

* run with native CPU optimizations

```bash
RUSTFLAGS='-C target-cpu=native' ./run_demo.sh
```

**Technical Details:**

- **Proof System**: WHIR (polynomial commitment) + SuperSpartan (AIR-optimized)
- **Field**: KoalaBear (p = 2^31 - 2^24 + 1) for efficient mobile arithmetic
- **Design**: Cairo-inspired minimal VM with optimized constraints
- **Target**: Post-quantum signature aggregation, mobile proving
- **Status**: Early development, recursion in progress, SDK not yet public

**Benchmarks** (from leanMultisig project):
- Fibonacci (n=2M): 2.0s on i9-12900H, 1.2s on M4 Max
- Poseidon2 (2^20 perms): Efficient batch hashing
- XMSS (990 sigs): Unbounded aggregation with constant verification

**See also:**
- [QUICK_START.md](lean-zkvm/QUICK_START.md) - Get started in 5 minutes
- [PROJECT_OVERVIEW.md](lean-zkvm/PROJECT_OVERVIEW.md) - Architecture deep dive
- [GitHub Repository](https://github.com/leanEthereum/leanMultisig) - Official project

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

## Novanet zkvm

### Resources

* https://eprint.iacr.org/2021/370 (Nova: Recursive Zero-Knowledge Arguments from Folding Schemes)
* https://github.com/microsoft/nova (Nova implementation)
* https://github.com/kkrt-labs/zkvm-benchmarks (reference benchmarks)

### About Novanet

Novanet zkVM is based on the Nova proof system, which provides:
- **Recursive SNARKs**: Without trusted setup
- **Incrementally Verifiable Computation (IVC)**: Efficient for iterative computations
- **Constant-size Proofs**: Regardless of computation depth
- **Proof Composition**: Native support for composing multiple proofs

Nova is particularly well-suited for applications requiring:
- Iterative algorithms (like Fibonacci, loops)
- Multi-step verifiable computations
- Blockchain state transitions
- Proof aggregation and batching

### how to run the Novanet demo

⚠️ **Note**: This is a demonstration implementation showing the expected structure and workflow. A production implementation would require integration with actual Nova proof system libraries.

* Install dependencies (optional):

```bash
# Option 1: Use the installation script
./scripts/sdk_installers/install_novanet_sdk.sh

# Option 2: Manual setup (just ensure Rust 1.85+ is installed)
rustup toolchain install 1.85
```

* cd to the target demo directory

```bash
cd novanet-zkvm
```

* run the Novanet demo

```bash
# Using the convenient run script
./run_demo.sh

# Or directly with cargo
cargo run --release -p novanet-host
```

* run with custom Fibonacci input

```bash
# Set custom input value
FIBONACCI_N=15 ./run_demo.sh

# Or with cargo
FIBONACCI_N=15 cargo run --release -p novanet-host
```

* run tests

```bash
cargo test
```

### Implementation Notes

This demo provides a template for Nova-based zkVM workflows:
1. **Guest Program**: Computation to be proven (in `novanet-guest/`)
2. **Host Program**: Proof orchestration (in `novanet-host/`)
3. **Workflow**: Compile → Setup → Prove → Verify

For production use, consider:
- Integrating `nova-snark` or similar Nova implementation
- Adding circuit compilation infrastructure
- Implementing actual proof generation and verification
- Optimizing for your specific use case

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

## Powdr zkvm

### Resources

* https://github.com/powdr-labs/powdr
* https://docs.powdr.org/
* https://github.com/kkrt-labs/zkvm-benchmarks (reference implementation)

### how to run the Powdr demo

⚠️ **Note**: Powdr is a modular zkVM toolkit under active development. This is a reference implementation showing the expected workflow. Full integration will be updated once the SDK matures.

* Install Powdr first (if not already installed):

```bash
# Option 1: Use the installation script
./scripts/sdk_installers/install_powdr_sdk.sh

# Option 2: Manual installation
cargo install --git https://github.com/powdr-labs/powdr --branch main powdr-cli
```

* cd to the target demo directory

```bash
cd powdr-zkvm
```

* run the Powdr demo

```bash
# Using the convenient run script
./run_demo.sh

# Or directly with cargo
cd powdr-host
RUST_LOG=info cargo run --release
```

* run with custom Fibonacci input

```bash
# Set custom input value
FIBONACCI_N=15 ./run_demo.sh

# Or with cargo
FIBONACCI_N=15 RUST_LOG=info cargo run --release
```

### About Powdr

Powdr is a **modular zkVM toolkit** that allows developers to build custom zero-knowledge virtual machines by combining different components:

- **Multiple Frontends**: RISC-V, WASM, custom instruction sets
- **Flexible Backends**: Halo2, Plonky2, STARK, or custom proving systems
- **PIL Language**: Define custom circuit constraints
- **Application-Specific**: Optimize for your specific use case

Unlike monolithic zkVMs, Powdr provides building blocks for creating specialized zkVMs tailored to specific needs.

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

## zkWasm zkvm

### Resources

* https://github.com/DelphinusLab/zkWasm
* https://ieeexplore.ieee.org/document/10587123 (IEEE Paper)
* https://www.delphinus-lab.com/
* Target: WebAssembly bytecode (unique among zkVMs)
* Browser-compatible zero-knowledge proofs

### How to Run the zkWasm Demo

zkWasm is unique as it targets WebAssembly instead of RISC-V, allowing any WASM-compiled language to generate zero-knowledge proofs.

* Install zkWasm CLI:

```bash
cd scripts/sdk_installers
./install_zkwasm_sdk.sh
```

This will:
- Clone the zkWasm repository
- Build the `delphinus-cli` tool
- Install it to `~/.local/bin`

* Navigate to the demo directory:

```bash
cd zkwasm-zkvm
```

* Run the complete demo:

```bash
./run_demo.sh
```

This will:
1. Build the WASM guest program
2. Setup the zkWasm circuit
3. Generate a proof for Fibonacci(10)
4. Verify the proof

* Run with custom input:

```bash
# Compute Fibonacci(20)
./run_demo.sh 20

# Compute Fibonacci(50) with larger circuit (k=20)
./run_demo.sh 50 20
```

* Step-by-step execution:

```bash
cd zkwasm-host

# Build WASM
cargo run -- build

# Setup circuit
cargo run -- setup --k 18

# Generate proof
cargo run -- prove --n 10

# Verify proof
cargo run -- verify

# All-in-one
cargo run -- run --n 15 --k 18
```

**Key Features:**
- 🌐 **WebAssembly Target**: Any language → WASM → Proof
- 🖥️ **Browser Compatible**: Can run in web browsers
- 📦 **Small Proofs**: Compact proof size for efficient verification
- 🔧 **No Code Changes**: Existing WASM apps work without modification

**See also:**
- [QUICK_START.md](zkwasm-zkvm/QUICK_START.md) - Get started in 5 minutes
- [PROJECT_OVERVIEW.md](zkwasm-zkvm/PROJECT_OVERVIEW.md) - Deep technical dive
- [Official Examples](https://github.com/DelphinusLab/zkWasm#project-bootstrap)



## Reference
* https://github.com/blocksense-network/zkVMs-benchmarks
* https://github.com/kkrt-labs/zkvm-benchmarks
  