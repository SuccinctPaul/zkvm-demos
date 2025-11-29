# SP1 zkVM Benchmark Metrics Comprehensive Analysis

This document details all benchmark metrics defined in the `sp1.toml` configuration file.

## 📊 Metrics Classification Overview

### **Total: 150+ Independent Metrics**

- **P0 Core Metrics**: 10 - Key metrics that must be extracted
- **P1 High Priority Metrics**: 60 - Important performance and time metrics
- **P2 Medium Priority Metrics**: 30 - Auxiliary performance analysis metrics
- **P3 Detailed Metrics**: 50+ - Deep analysis and optimization metrics

---

## 🎯 P0 Core Metrics (Core Metrics)

These are the most critical benchmark metrics and must be extracted in all tests:

| Metric Name | Type | Description | Example Value |
|---------|------|------|--------|
| `total_cycles` | int | Total execution cycles | 20 |
| `total_instruction_count` | int | Total instruction count | 7409 |
| `total_prove_time_s` | float | Total proving time (seconds) | 139.80 |
| `final_proof_size_bytes` | int | Final proof size (bytes) | 260 |
| `verification_time_s` | float | Verification time (seconds) | 0.001831 |
| `verification_time_ms` | float | Verification time (milliseconds) | 1.831 |
| `success_status` | string | Execution status | "success" |
| `program_name` | string | Program name | "fibonacci_10" |
| `zkvm_name` | string | ZKVM name | "SP1" |
| `zkvm_version` | string | ZKVM version | "v5.0.0" |

---

## 🚀 P1 Execution Phase Metrics (Execution Phase Metrics)

### Basic Execution Metrics

| Metric Name | Type | Description |
|---------|------|------|
| `execution_time_s` | float | Program execution time (seconds) |
| `gas` | int | Gas consumption |
| `total_syscall_cycles` | int | Total system call cycles |
| `touched_memory_addresses` | int | Number of accessed memory addresses |

---

## ⚡ P1 Proving Phase Metrics (Proving Phase Metrics)

### Core Proof Phase (~4.04s)

The first stage of proving, generating the core proof:

| Metric Name | Type | Description |
|---------|------|------|
| `core_proof_total_time_s` | float | Core proving total time |
| `phase2_trace_generation_time_ms` | float | Phase 2 trace generation time |
| `main_traces_generation_time_ms` | float | Main trace generation time |
| `recursion_program_build_time_ms` | float | Recursion program build time |
| `recursion_program_compile_time_ms` | float | Recursion program compile time |
| `core_merkle_tree_build_time_ms` | float | Merkle tree build time |
| `core_permutation_generation_time_ms` | float | Permutation generation time |
| `core_quotient_compute_time_ms` | float | Quotient compute time |
| `core_multi_batch_opening_time_s` | float | Multi-batch opening time |

### Compress Phase (~4.69s)

Compressing the core proof to reduce proof size:

| Metric Name | Type | Description |
|---------|------|------|
| `compress_total_time_s` | float | Compress total time |
| `compress_generate_records_time_ms` | float | Record generation time |
| `compress_setup_time_ms` | float | Setup time |
| `compress_commit_time_ms` | float | Commit time |
| `compress_permutation_time_ms` | float | Permutation time |
| `compress_quotient_compute_time_ms` | float | Quotient compute time |
| `compress_opening_time_s` | float | Opening time |
| `compress_merkle_tree_build_time_ms` | float | Merkle tree build time |

### Shrink Phase (~4.65s)

Further shrinking proof size:

| Metric Name | Type | Description |
|---------|------|------|
| `shrink_total_time_s` | float | Shrink total time |
| `shrink_program_build_time_ms` | float | Program build time |
| `shrink_program_compile_time_ms` | float | Program compile time |
| `shrink_setup_time_ms` | float | Setup time |
| `shrink_prove_time_s` | float | Proving time |
| `shrink_merkle_tree_build_time_ms` | float | Merkle tree build time |
| `shrink_permutation_time_ms` | float | Permutation time |
| `shrink_quotient_compute_time_ms` | float | Quotient compute time |

**Shrink Event Counts:**

| Metric Name | Description |
|---------|------|
| `shrink_total_events` | Total events |
| `shrink_base_alu_events` | Base ALU events |
| `shrink_batch_fri_events` | Batch FRI events |
| `shrink_ext_alu_events` | Extended ALU events |
| `shrink_mem_const_count` | Constant memory count |
| `shrink_mem_var_events` | Variable memory events |
| `shrink_poseidon2_events` | Poseidon2 hash events |
| `shrink_select_events` | Select events |
| `shrink_exp_reverse_bits_len_events` | Exponent reverse bits length events |

### Wrap BN254 Phase (~110s)

Wrapping as BN254 elliptic curve proof:

| Metric Name | Type | Description |
|---------|------|------|
| `wrap_total_time_s` | float | Wrap total time |
| `wrap_program_build_time_ms` | float | Program build time |
| `wrap_program_compile_time_ms` | float | Program compile time |
| `wrap_setup_time_s` | float | Setup time (includes heavy pre-computation) |
| `wrap_prove_time_s` | float | Proving time |
| `wrap_verify_time_ms` | float | Verification time |
| `wrap_merkle_tree_build_time_s` | float | Merkle tree build time |
| `wrap_permutation_time_ms` | float | Permutation time |

**Wrap Event Counts:**

| Metric Name | Description |
|---------|------|
| `wrap_total_events` | Total events |
| `wrap_base_alu_events` | Base ALU events |
| `wrap_ext_alu_events` | Extended ALU events |
| `wrap_mem_const_count` | Constant memory count |
| `wrap_mem_var_events` | Variable memory events |
| `wrap_poseidon2_events` | Poseidon2 events |
| `wrap_select_events` | Select events |

---

## 🔐 P1 Groth16 Phase Metrics (Groth16 Phase Metrics)

Generating the final Groth16 zkSNARK proof:

| Metric Name | Type | Description | Typical Value |
|---------|------|------|--------|
| `groth16_proof_size_bytes` | int | Groth16 proof size | 260 bytes |
| `groth16_r1cs_reading_time_s` | float | R1CS reading time | ~6.48s |
| `groth16_proving_key_reading_time_s` | float | Proving Key reading time | ~1.13s |
| `groth16_witness_reading_time_ms` | float | Witness reading time | ~0.49ms |
| `groth16_witness_generation_time_ms` | float | Witness generation time | ~116.48ms |
| `groth16_constraint_count` | int | Constraint count | 8382715 |
| `groth16_solver_time_s` | float | Solver time | ~1.31s |
| `groth16_prover_time_s` | float | Prover time | ~7.85s |
| `groth16_generation_time_s` | float | Total generation time | ~9.16s |
| `groth16_verifier_time_ms` | float | Verifier time | ~0.82ms |
| `groth16_total_time_s` | float | Groth16 total time | ~16.9s |

---

## ✅ P2 Verification Metrics (Verification Metrics)

| Metric Name | Type | Description |
|---------|------|------|
| `on_chain_gas_estimate` | int | On-chain Gas estimate |

---

## 📈 P2 Performance Metrics (Performance Metrics)

| Metric Name | Type | Description |
|---------|------|------|
| `khz` | float | Kilo-cycles per second (kHz) |
| `e2e_time_s` | float | End-to-end time |
| `cycles_per_second` | float | Cycles per second |

---

## 🔧 P2 Initialization Metrics (Initialization Metrics)

| Metric Name | Type | Description |
|---------|------|------|
| `prover_initialization_time_s` | float | Prover initialization time |
| `compress_program_build_total_time_ms` | float | Compress program total build time |
| `compress_program_compile_total_time_ms` | float | Compress program total compile time |

---

## 💻 P3 Instruction Statistics (Instruction Statistics)

### RISC-V Basic Instructions

| Instruction Category | Metric Name |
|---------|---------|
| Arithmetic | `opcode_add`, `opcode_sub`, `opcode_mul` |
| Logic | `opcode_and`, `opcode_or`, `opcode_xor` |
| Shift | `opcode_sll`, `opcode_srl`, `opcode_sra` |

### Memory Access Instructions

| Instruction Category | Metric Name |
|---------|---------|
| Load | `opcode_lw`, `opcode_lb`, `opcode_lbu`, `opcode_lh`, `opcode_lhu` |
| Store | `opcode_sw`, `opcode_sb`, `opcode_sh` |

### Branch and Jump Instructions

| Instruction Category | Metric Name |
|---------|---------|
| Branch | `opcode_beq`, `opcode_bne`, `opcode_blt`, `opcode_bge`, `opcode_bltu`, `opcode_bgeu` |
| Jump | `opcode_jal`, `opcode_jalr` |

### Other Instructions

- `opcode_lui` - Load Upper Immediate
- `opcode_auipc` - Add Upper Immediate to PC
- `opcode_ecall` - System Call
- `opcode_sltu` - Set Less Than Unsigned

---

## 🔌 P3 Syscall Statistics (Syscall Statistics)

### Basic System Calls

| System Call | Metric Name | Description |
|---------|---------|------|
| Commit | `syscall_commit` | Commit operation |
| Deferred Proofs | `syscall_commit_deferred_proofs` | Deferred proof commit |
| Halt | `syscall_halt` | Program termination |
| Write | `syscall_write` | Write operation |
| Hint | `syscall_hint_len`, `syscall_hint_read` | Hint operation |

### Cryptographic Operations

| Category | System Call |
|------|---------|
| SHA | `syscall_sha_compress` |
| Keccak | `syscall_keccak_permute` |
| Edwards | `syscall_ed_add`, `syscall_ed_decompress` |
| Secp256k1 | `syscall_secp256k1_add`, `syscall_secp256k1_double`, `syscall_secp256k1_decompress` |
| BN254 | `syscall_bn254_add`, `syscall_bn254_double` |
| BLS12-381 | `syscall_bls12381_add`, `syscall_bls12381_double`, `syscall_bls12381_decompress` |
| BigInt | `syscall_uint256_mul` |

---

## 🖥️ P3 Chip Resource Statistics (Chip Resource Statistics)

### Core Phase Major Chips

| Chip Name | Rows Metric | Cells Metric | Description |
|---------|----------|-----------|------|
| CPU | `chip_cpu_rows` | `chip_cpu_cells` | CPU Execution Core |
| Program | `chip_program_rows` | `chip_program_cells` | Program Storage |
| Global | `chip_global_rows` | `chip_global_cells` | Global State |
| Byte | `chip_byte_rows` | `chip_byte_cells` | Byte Lookup Table |
| MemoryGlobalInit | `chip_memory_global_init_rows` | `chip_memory_global_init_cells` | Memory Initialization |
| MemoryGlobalFinalize | `chip_memory_global_finalize_rows` | `chip_memory_global_finalize_cells` | Memory Finalization |

### Compress Phase Major Chips

| Chip Name | Rows Metric | Cells Metric |
|---------|----------|-----------|
| BatchFRI | `chip_compress_batch_fri_rows` | `chip_compress_batch_fri_cells` |
| MemoryVar | `chip_compress_memory_var_rows` | `chip_compress_memory_var_cells` |
| Poseidon2 | `chip_compress_poseidon2_rows` | `chip_compress_poseidon2_cells` |
| BaseAlu | `chip_compress_base_alu_rows` | `chip_compress_base_alu_cells` |
| ExtAlu | `chip_compress_ext_alu_rows` | `chip_compress_ext_alu_cells` |

---

## 🌲 P3 Merkle Tree Dimensions

Record the structure information of Merkle Trees in each stage:

| Metric Name | Example Value |
|---------|--------|
| `core_merkle_dimensions` | `"[14x1048576, 11x131072]"` |
| `compress_merkle_dimensions` | `"[13x1048576, 8x1048576, ...]"` |
| `shrink_merkle_dimensions` | `"[48x1048576, 8x1048576, ...]"` |
| `wrap_merkle_dimensions` | `"[51x4194304, 32x2097152, ...]"` |

Format description: `"[cols x rows, cols x rows, ...]"`

---

## 📊 Reporting Configuration

### Default Report Metrics

The `[reporting]` section in the configuration file defines the key metrics (about 30) for default output, including:

- Basic performance metrics (cycles, instructions, time)
- Time breakdown by phase (core, compress, shrink, wrap, groth16)
- Proof size and verification metrics
- Key Groth16 parameters
- Resource usage statistics

### Grouped Reports

The configuration file also defines multiple metric groups for generating categorized reports:

1. **execution** - Execution phase metrics
2. **proving** - Proving phase time breakdown
3. **core_details** - Core phase detailed metrics
4. **compress_details** - Compress phase detailed metrics
5. **shrink_details** - Shrink phase detailed metrics
6. **wrap_details** - Wrap phase detailed metrics
7. **groth16_details** - Groth16 phase detailed metrics
8. **instruction_stats** - Instruction statistics
9. **syscall_stats** - System call statistics
10. **chip_resources** - Chip resource statistics

---

## 🎯 Usage Suggestions

### Quick Analysis
Focus on P0 and P1 metrics to quickly evaluate performance:
- Execution efficiency: `total_cycles`, `execution_time_s`, `khz`
- Proving performance: `total_prove_time_s` and phase times
- Proof size: `final_proof_size_bytes`
- Verification performance: `verification_time_s`

### Deep Optimization
Use P2 and P3 metrics for deep analysis:
- Instruction hotspots: Analyze `opcode_*` statistics
- Syscall overhead: Analyze `syscall_*` statistics
- Memory usage: Analyze chip `rows` and `cells` statistics
- Bottleneck identification: Compare time distribution across phases

### Comparative Testing
Run tests under different configurations to compare:
- Scalability across different scales
- Trade-offs between different proving modes
- Performance changes before and after optimization

---

## 📝 Notes

1. **Regex Matching**: All metrics are extracted from logs using regular expressions, ensuring consistent log format.
2. **Optional Metrics**: Some metrics may not exist in certain modes (e.g., no Groth16 metrics in core mode).
3. **Unit Unification**: Pay attention to time units (seconds/milliseconds) and size units (bytes/KB).
4. **Multi-line Matching**: Instruction and syscall statistics use `\n` for multi-line matching.
5. **Numerical Parsing**: Supports integers and floating-point numbers, using `[\d.]+` or `\d+` for matching.

---

## 🔗 Related Files

- **Configuration File**: `configs/sp1.toml`
- **Raw Logs**: `benchmark-results/raw-logs/*.log`
- **Parsed Results**: `benchmark-results/parsed-metrics/*.json`
- **Report Output**: Generated according to `output_formats` configuration

---

*Last Updated: 2025-11-19*
*Configuration Version: v1.0*
