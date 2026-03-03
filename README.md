# zkvm-benchmarks

Unified benchmarking framework for Zero-Knowledge Virtual Machines (zkVMs). Measures execution time, proof metrics, and
resource usage across implementations using standardized guest programs.

## Support Matrix

| zkVM          | Status   | Programs      | Notes                 |
|:--------------|:---------|:--------------|:----------------------|
| **SP1**       | 🟢 Ready | All           | Production-ready SDK  |
| **Risc0**     | 🟢 Ready | All           | Production-ready SDK  |
| **Jolt**      | 🟢 Ready | All           | Full SDK Support      |
| **Nexus**     | 🟢 Ready | All           | Full SDK Support      |
| **Pico**      | 🟢 Ready | All           | Full SDK Support      |
| **Powdr**     | 🟢 Ready | All           | Full SDK Support      |
| **ZKM**       | 🟢 Ready | All           | Network/Local Proving |
| **zkWasm**    | 🟢 Ready | All           | Delphinus CLI         |
| **Valida**    | 🟢 Ready | All           | CLI Integration       |
| **SnarkVM**   | 🟢 Ready | All           | Aleo SDK              |
| **Miden**     | 🟢 Ready | Fib, Sum, Fac | MASM Support          |
| **Cairo**     | 🟢 Ready | All           | Stwo/Cairo SDK        |
| **Airbender** | 🟢 Ready | All           | Nightly Rust          |
| **Zisk**      | 🔴 Linux | All           | No macOS support      |
| **Ceno**      | 🟡 Ref   | All           | SDK Private/Simulated |
| **Lean**      | 🟡 Ref   | All           | SDK Private/Simulated |
| **Cairo-M**   | 🟡 Ref   | All           | Simulated Proofs      |
| **Novanet**   | 🟡 Ref   | All           | WASM Runner           |
| **o1vm**      | 🟡 Ref   | All           | Framework Only        |
| **OpenVM**    | ⚪ Off    | All           | Disabled by default   |


> **Legend**: 🟢 Ready (Full Integration) | 🟡 Ref (Reference/Simulated) | 🔴 Linux (OS Dependent) | ⚪ Off (Disabled)

## Quick Start

* Haredware Requirements
  * CPU: 16+ cores (for parallel benchmarks)
  * RAM: 32GB+ (for heavy zkVMs like SP1)
  * Storage: 100GB+ free space (for SDKs, toolchains, and benchmark results)
  * OS: macOS/Linux (recommended: MacBook Pro M4 Pro)


* Prerequisites
    * Rust toolchain
    * golang toolchain
    * Docker
    * Make,curl,git etc.

* Install zkVM SDKs/Toolchains

  Run benchmarks natively needs installed zkVM toolchains/SDKs. Install them using the provided scripts.
  
  ```bash
  cd scripts/sdk_installers
  bash install_xxx.sdk.sh
  ```

* Run
  
  Run benchmarks natively using the provided `Makefile`.
  
  ```bash
  cd zkvm_benchmark_utils
  
  # 1. Run specific zkVM (e.g., SP1)
  make bench_sp1
  
  # 2. Run fast benchmark set (SP1, Risc0, Jolt)
  make bench_fast
  
  # 3. Run all enabled zkVMs
  make bench_all
  ```

* Result

  You can find the results in the `zkvm_benchmark_utils/benchmark-results/reports` directory. Each run will generate a timestamped subdirectory containing raw logs, parsed metrics, and generated reports. 
  And the final reporter(csv) can be found at `zkvm_benchmark_utils/benchmark-results/reports`, organized by zkVM, program and input.

## Benchmark Utils

For details on metrics, configuration, and advanced usage, see:
👉 [**zkvm_benchmark_utils/README.md**](./zkvm_benchmark_utils/README.md)

## Reference

Inspired by and builds upon previous zkVM benchmarking efforts:

* https://github.com/blocksense-network/zkVMs-benchmarks
* https://github.com/kkrt-labs/zkvm-benchmarks