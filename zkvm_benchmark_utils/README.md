# zkVM Benchmark Framework

Automated zkVM performance testing framework supporting unified benchmarking for multiple Zero-Knowledge Virtual Machines.

## 🏗 Architecture

The framework uses a modular design with the following core components:

*   **Core (`src/core`)**: Defines the unified configuration structure (`config.rs`) and standardized metrics system (`metrics.rs`).
*   **Execution (`src/execution`)**: Handles building and running zkVMs, scheduling external commands via `executor.rs`.
*   **Analysis (`src/analysis`)**: `log_parser.rs` parses zkVM output based on regex patterns to extract performance metrics.
*   **Reporting (`src/reporting`)**: Generates test reports in JSON/CSV formats and supports multi-dimensional performance comparisons.
*   **System (`src/system`)**: Automatically collects hardware environment information (CPU, Memory, OS).

## 📊 Metrics System

The metrics cover the entire lifecycle of a zkVM execution:

1.  **Execution**
    *   `total_cycles`: Total computation cycles
    *   `instruction_count`: Total number of instructions
    *   `execution_time_s`: Execution time (Witness generation time)
    *   `touched_memory_addresses`: Number of memory addresses accessed

2.  **VM Circuit**
    *   `vm_prove_time_s`: Base proof generation time
    *   `vm_prove_khz`: Proving throughput (Cycles/Sec)
    *   `vm_chunk_count`: Number of shards/chunks for parallel computation

3.  **Aggressive (Recursion/Aggregation)**
    *   `recursion_layers`: Number of recursion layers
    *   `aggressive_prove_time_s`: Aggregated proof generation time
    *   `aggressive_proof_size_bytes`: Aggregated proof size

4.  **SNARK (Final Proof)**
    *   `snark_proof_time_s`: Final SNARK generation time (Groth16/Plonk)
    *   `snark_proof_size_bytes`: Final on-chain proof size
    *   `snark_setup_time_s`: Setup time

5.  **Verification**
    *   `verification_time_s`: Verification time
    *   `on_chain_gas_estimate`: On-chain verification gas estimate

6.  **Resources**
    *   `peak_memory_mb`: Peak memory usage
    *   `total_time_s`: End-to-end total duration

## 🚀 Usage (Make Commands)

We recommend using `make` commands for quick testing:

```bash
# 1. Run specific zkVM
make bench_sp1
make bench_risc0

# 2. Run specific groups
make bench_risc_based   # Run all RISC-V architectures (SP1, Risc0, Jolt, etc.)
make bench_wasm_based   # Run WASM architectures
make bench_custom_vm    # Run custom architectures (Cairo, Miden, Valida, etc.)

# 3. Run all enabled zkVMs
make bench_all

# 4. Quick test (Scale=10)
make bench_fast

# 5. Run with custom parameters
# Run SP1, testing only fibonacci and hash programs, with scales 10 and 100
make bench_sp1 PROGRAMS=fibonacci,hash SCALES=10,100

# Specify output directory
make bench_all OUTPUT_DIR=./my-benchmark-results
```

## ⚙️ Configuration & Development

*   **Configuration Files**: Located in `configs/*.toml`. You can configure the enabled status, run commands, log patterns, etc.
*   **Adding New zkVM**: Copy `configs/template.toml` and modify `run_command` and `log_patterns` to integrate a new zkVM.

## 📋 Support List

### Supported zkVMs

| Architecture | zkVM | Status | Architecture | zkVM | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **RISC-V** | SP1, Risc0, Jolt, Ceno, Nexus, OpenVM, Pico, Powdr, ZKM, Zisk, Novanet, Lean | ✅ | **WASM** | zkWasm | ✅ |
| **Custom** | Cairo, Cairo-M, Miden, Valida, SnarkVM, Airbender, o1VM | ✅ | | | |

### Supported Programs

Can be specified via the `PROGRAMS` environment variable:
`fibonacci`, `sum`, `factorial`, `isprime`, `popcount`, `hash` (sha256), `signature` (ecdsa)
