# zkvm-benchmarks

Unified benchmarking framework for Zero-Knowledge Virtual Machines (zkVMs). Measures execution time, proof metrics, and
resource usage across implementations using standardized guest programs.

## Support zkVMs

| zkVM          | Version     | Status   | Programs      | Notes                 |
|:--------------|:------------|:---------|:--------------|:----------------------|
| **SP1**       | v5.2.2      | 🟢 Ready | All           | Production-ready SDK  |
| **Risc0**     | v3.0.3      | 🟢 Ready | All           | Production-ready SDK  |
| **Jolt**      | v0.3.0-alpha| 🟢 Ready | All           | Full SDK Support      |
| **Nexus**     | v0.3.4      | 🟢 Ready | All           | Full SDK Support      |
| **Pico**      | v1.1.6      | 🟢 Ready | All           | Full SDK Support      |
| **Powdr**     | v0.1.0      | 🟢 Ready | All           | Full SDK Support      |
| **ZKM**       | v0.3.0      | 🟢 Ready | All           | Network/Local Proving |
| **zkWasm**    | v0.1.0      | 🟢 Ready | All           | Delphinus CLI         |
| **Valida**    | v1.0.0      | 🟢 Ready | All           | CLI Integration       |
| **SnarkVM**   | v0.16.0     | 🟢 Ready | All           | Aleo SDK              |
| **Miden**     | v0.11.0     | 🟢 Ready | Fib, Sum, Fac | MASM Support          |
| **Cairo**     | v2.8.5      | 🟢 Ready | All           | Stwo/Cairo SDK        |
| **Airbender** | v0.5.0      | 🟢 Ready | All           | Nightly Rust          |
| **Zisk**      | v0.10.0     | 🔴 Linux | All           | No macOS support      |
| **Ceno**      | v0.1.0      | 🟡 Ref   | All           | SDK Private/Simulated |
| **Lean**      | v0.1.0      | 🟡 Ref   | All           | SDK Private/Simulated |
| **Cairo-M**   | v0.1.0      | 🟡 Ref   | All           | Simulated Proofs      |
| **Novanet**   | v0.1.0      | 🟡 Ref   | All           | WASM Runner           |
| **o1vm**      | v0.1.0      | 🟡 Ref   | All           | Framework Only        |
| **OpenVM**    | v1.4.0      | ⚪ Off    | All           | Disabled by default   |

> **Legend**: 🟢 Ready (Full Integration) | 🟡 Ref (Reference/Simulated) | 🔴 Linux (OS Dependent) | ⚪ Off (Disabled)

## Quick Start
> Notes: Before running benchmarks, make sure your environment can run individual zkVMs natively (e.g., you can run SP1 or Risc0 programs without Docker). This ensures that the benchmarks will run smoothly and generate valid results.
> 

* Haredware Requirements
  * CPU: 16+ cores
  * RAM: 32GB+ (for heavy zkVMs like SP1)
  * Storage: 100GB+ free space (for SDKs, toolchains, and benchmark results)
  * OS: macOS/Linux (recommended: macos)


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
  > If you meet any problems, we recommend to reference official docs of related zkVM to set up the environment and run their example programs, which can help you understand the underlying mechanics and debug potential issues during benchmarking.


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


## Acknowledgements
This project aims to provide a unified benchmarking framework for measuring and comparing the performance of different zkVM implementations and their example programs on custom metrics such as execution time, proof metrics, and etc.
During the development, The following resources and projects have been invaluable for reference and inspiration:

* https://github.com/blocksense-network/zkVMs-benchmarks
* https://github.com/kkrt-labs/zkvm-benchmarks

We acknowledge the efforts of the zkVM community in developing these innovative technologies and hope that this framework can contribute to their evaluation and improvement.
And welcome contributions from the community to expand zkVM support, add new benchmark programs, and improve the framework.
