# zkvm-benchmarks

A unified framework for benchmarking Zero-Knowledge Virtual Machines (zkVMs). Measures execution time, proof generation, memory usage, and proof size across different implementations using standardized guest programs.

## Features

- **Unified Metrics**: Consistent collection via `zkvm_benchmark_utils`.
- **Standardized Programs**: Fibonacci, Hash (SHA2), Signature Verification (ECDSA), and more ported across all zkVMs.
- **Isolation**: Docker and local workspace management to handle conflicting toolchains.

## Support Matrix

| zkVM | Programs | Config | Status |
| :--- | :--- | :--- | :--- |
| **Airbender** | Fib, Hash, Sig | ✅ | Full Integration (Nightly Rust) |
| **Cairo** | Fib, Hash, Sig | ✅ | Full Integration (Stwo/Cairo) |
| **Cairo-M** | Fib, Hash, Sig | ❌ | Reference (Simulated Proofs) |
| **Ceno** | Fib, Hash, Sig | ❌ | Reference (SDK Private/Simulated) |
| **Jolt** | Fib, Hash, Sig | ✅ | Full Integration |
| **Lean** | Fib, Hash, Sig | ❌ | Reference (SDK Private) |
| **Miden** | Fib, Sum, Fac | ✅ | Full Integration (MASM) |
| **Nexus** | Fib, Hash, Sig | ✅ | Full Integration |
| **Novanet** | Fib, Hash, Sig | ✅ | Reference (WASM Runner) |
| **o1vm** | Fib, Hash, Sig | ❌ | Reference (MIPS/Kimchi Complexities) |
| **OpenVM** | Fib, Hash, Sig | ❌ | Full Integration (Disabled by default) |
| **Pico** | Fib, Hash, Sig | ✅ | Full Integration |
| **Powdr** | Fib, Hash, Sig | ✅ | Full Integration |
| **Risc0** | Fib, Hash, Sig | ✅ | Full Integration |
| **SnarkVM** | Fib, Hash, Sig | ✅ | Full Integration (Aleo) |
| **SP1** | Fib, Hash, Sig | ✅ | Full Integration |
| **Valida** | Fib, Hash, Sig | ✅ | Full Integration (CLI) |
| **Zisk** | Fib, Hash, Sig | ❌ | OS Dependent (Linux/Docker Only) |
| **ZKM** | Fib, Hash, Sig | ✅ | Full Integration |
| **zkWasm** | Fib, Hash, Sig | ✅ | Full Integration (Delphinus) |

> **Reference**: Structural integration only. Uses simulated proving due to pending SDKs or complex setup requirements.

## Quick Start

### Docker (Recommended)
Avoids toolchain pollution.

```bash
cd docker/scripts
./build-base.sh
./development-manager.sh build sp1
./development-manager.sh run sp1 --execute
```

### Local Environment
Native performance.

```bash
# Setup
./scripts/local_env/setup-sp1-workspace.sh ~/zkvm-workspaces/sp1-workspace

# Activate
source ~/zkvm-workspaces/sp1-workspace/activate.sh
```

## References
* [zkvm_benchmark_utils](./zkvm_benchmark_utils/README.md)
* [blocksense-network/zkVMs-benchmarks](https://github.com/blocksense-network/zkVMs-benchmarks)
