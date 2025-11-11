# zkvm-demos

A collection of Zero-Knowledge Virtual Machine (zkVM) demonstrations for SP1, Risc0, Nexus, and ZKM. This repository
shows how to build and run programs on different zkVM platforms.

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
