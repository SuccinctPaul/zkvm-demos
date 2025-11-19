# SP1 zkVM Benchmark 指标全面解析

本文档详细说明了 `sp1.toml` 配置文件中定义的所有 benchmark 指标。

## 📊 指标分类概览

### **总计: 150+ 个独立指标**

- **P0 核心指标**: 10个 - 必须提取的关键指标
- **P1 高优先级指标**: 60个 - 重要的性能和时间指标
- **P2 中优先级指标**: 30个 - 辅助性能分析指标
- **P3 详细指标**: 50+ 个 - 深度分析和优化指标

---

## 🎯 P0 核心指标 (Core Metrics)

这些是最关键的 benchmark 指标，必须在所有测试中提取：

| 指标名称 | 类型 | 说明 | 示例值 |
|---------|------|------|--------|
| `total_cycles` | int | 总执行周期数 | 20 |
| `total_instruction_count` | int | 总指令数量 | 7409 |
| `total_prove_time_s` | float | 总证明时间（秒） | 139.80 |
| `final_proof_size_bytes` | int | 最终证明大小（字节） | 260 |
| `verification_time_s` | float | 验证时间（秒） | 0.001831 |
| `verification_time_ms` | float | 验证时间（毫秒） | 1.831 |
| `success_status` | string | 执行状态 | "success" |
| `program_name` | string | 程序名称 | "fibonacci_10" |
| `zkvm_name` | string | ZKVM 名称 | "SP1" |
| `zkvm_version` | string | ZKVM 版本 | "v5.0.0" |

---

## 🚀 P1 执行阶段指标 (Execution Phase Metrics)

### 基础执行指标

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `execution_time_s` | float | 程序执行时间（秒） |
| `gas` | int | Gas 消耗量 |
| `total_syscall_cycles` | int | 系统调用总周期数 |
| `touched_memory_addresses` | int | 访问的内存地址数 |

---

## ⚡ P1 证明阶段指标 (Proving Phase Metrics)

### Core Proof 阶段（~4.04秒）

证明的第一阶段，生成核心证明：

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `core_proof_total_time_s` | float | Core 证明总时间 |
| `phase2_trace_generation_time_ms` | float | 阶段2轨迹生成时间 |
| `main_traces_generation_time_ms` | float | 主轨迹生成时间 |
| `recursion_program_build_time_ms` | float | 递归程序构建时间 |
| `recursion_program_compile_time_ms` | float | 递归程序编译时间 |
| `core_merkle_tree_build_time_ms` | float | Merkle树构建时间 |
| `core_permutation_generation_time_ms` | float | 排列生成时间 |
| `core_quotient_compute_time_ms` | float | 商值计算时间 |
| `core_multi_batch_opening_time_s` | float | 多批次开启时间 |

### Compress 阶段（~4.69秒）

压缩核心证明，减小证明大小：

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `compress_total_time_s` | float | 压缩总时间 |
| `compress_generate_records_time_ms` | float | 记录生成时间 |
| `compress_setup_time_ms` | float | 设置时间 |
| `compress_commit_time_ms` | float | 承诺时间 |
| `compress_permutation_time_ms` | float | 排列时间 |
| `compress_quotient_compute_time_ms` | float | 商计算时间 |
| `compress_opening_time_s` | float | 开启时间 |
| `compress_merkle_tree_build_time_ms` | float | Merkle树构建时间 |

### Shrink 阶段（~4.65秒）

进一步收缩证明大小：

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `shrink_total_time_s` | float | 收缩总时间 |
| `shrink_program_build_time_ms` | float | 程序构建时间 |
| `shrink_program_compile_time_ms` | float | 程序编译时间 |
| `shrink_setup_time_ms` | float | 设置时间 |
| `shrink_prove_time_s` | float | 证明时间 |
| `shrink_merkle_tree_build_time_ms` | float | Merkle树构建时间 |
| `shrink_permutation_time_ms` | float | 排列时间 |
| `shrink_quotient_compute_time_ms` | float | 商计算时间 |

**Shrink 事件计数：**

| 指标名称 | 说明 |
|---------|------|
| `shrink_total_events` | 总事件数 |
| `shrink_base_alu_events` | 基础ALU事件数 |
| `shrink_batch_fri_events` | BatchFRI事件数 |
| `shrink_ext_alu_events` | 扩展ALU事件数 |
| `shrink_mem_const_count` | 常量内存计数 |
| `shrink_mem_var_events` | 变量内存事件数 |
| `shrink_poseidon2_events` | Poseidon2哈希事件数 |
| `shrink_select_events` | Select事件数 |
| `shrink_exp_reverse_bits_len_events` | 指数反向位长度事件数 |

### Wrap BN254 阶段（~110秒）

包装为 BN254 椭圆曲线证明：

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `wrap_total_time_s` | float | Wrap总时间 |
| `wrap_program_build_time_ms` | float | 程序构建时间 |
| `wrap_program_compile_time_ms` | float | 程序编译时间 |
| `wrap_setup_time_s` | float | 设置时间（包含大量预计算） |
| `wrap_prove_time_s` | float | 证明时间 |
| `wrap_verify_time_ms` | float | 验证时间 |
| `wrap_merkle_tree_build_time_s` | float | Merkle树构建时间 |
| `wrap_permutation_time_ms` | float | 排列时间 |

**Wrap 事件计数：**

| 指标名称 | 说明 |
|---------|------|
| `wrap_total_events` | 总事件数 |
| `wrap_base_alu_events` | 基础ALU事件数 |
| `wrap_ext_alu_events` | 扩展ALU事件数 |
| `wrap_mem_const_count` | 常量内存计数 |
| `wrap_mem_var_events` | 变量内存事件数 |
| `wrap_poseidon2_events` | Poseidon2事件数 |
| `wrap_select_events` | Select事件数 |

---

## 🔐 P1 Groth16 阶段指标 (Groth16 Phase Metrics)

生成最终的 Groth16 zkSNARK 证明：

| 指标名称 | 类型 | 说明 | 典型值 |
|---------|------|------|--------|
| `groth16_proof_size_bytes` | int | Groth16证明大小 | 260 bytes |
| `groth16_r1cs_reading_time_s` | float | R1CS读取时间 | ~6.48s |
| `groth16_proving_key_reading_time_s` | float | Proving Key读取时间 | ~1.13s |
| `groth16_witness_reading_time_ms` | float | Witness读取时间 | ~0.49ms |
| `groth16_witness_generation_time_ms` | float | Witness生成时间 | ~116.48ms |
| `groth16_constraint_count` | int | 约束数量 | 8382715 |
| `groth16_solver_time_s` | float | Solver时间 | ~1.31s |
| `groth16_prover_time_s` | float | Prover时间 | ~7.85s |
| `groth16_generation_time_s` | float | 总生成时间 | ~9.16s |
| `groth16_verifier_time_ms` | float | 验证器时间 | ~0.82ms |
| `groth16_total_time_s` | float | Groth16总时间 | ~16.9s |

---

## ✅ P2 验证指标 (Verification Metrics)

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `on_chain_gas_estimate` | int | 链上Gas估算 |

---

## 📈 P2 性能效率指标 (Performance Metrics)

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `khz` | float | 每秒千周期数（kHz） |
| `e2e_time_s` | float | 端到端时间 |
| `cycles_per_second` | float | 每秒周期数 |

---

## 🔧 P2 初始化指标 (Initialization Metrics)

| 指标名称 | 类型 | 说明 |
|---------|------|------|
| `prover_initialization_time_s` | float | 证明器初始化时间 |
| `compress_program_build_total_time_ms` | float | 压缩程序总构建时间 |
| `compress_program_compile_total_time_ms` | float | 压缩程序总编译时间 |

---

## 💻 P3 指令统计 (Instruction Statistics)

### RISC-V 基础指令

| 指令类别 | 指标名称 |
|---------|---------|
| 算术运算 | `opcode_add`, `opcode_sub`, `opcode_mul` |
| 逻辑运算 | `opcode_and`, `opcode_or`, `opcode_xor` |
| 移位操作 | `opcode_sll`, `opcode_srl`, `opcode_sra` |

### 内存访问指令

| 指令类别 | 指标名称 |
|---------|---------|
| 加载指令 | `opcode_lw`, `opcode_lb`, `opcode_lbu`, `opcode_lh`, `opcode_lhu` |
| 存储指令 | `opcode_sw`, `opcode_sb`, `opcode_sh` |

### 分支和跳转指令

| 指令类别 | 指标名称 |
|---------|---------|
| 条件分支 | `opcode_beq`, `opcode_bne`, `opcode_blt`, `opcode_bge`, `opcode_bltu`, `opcode_bgeu` |
| 跳转 | `opcode_jal`, `opcode_jalr` |

### 其他指令

- `opcode_lui` - 加载立即数高位
- `opcode_auipc` - PC相对地址
- `opcode_ecall` - 系统调用
- `opcode_sltu` - 无符号比较

---

## 🔌 P3 系统调用统计 (Syscall Statistics)

### 基础系统调用

| 系统调用 | 指标名称 | 说明 |
|---------|---------|------|
| Commit | `syscall_commit` | 提交操作 |
| Deferred Proofs | `syscall_commit_deferred_proofs` | 延迟证明提交 |
| Halt | `syscall_halt` | 程序终止 |
| Write | `syscall_write` | 写操作 |
| Hint | `syscall_hint_len`, `syscall_hint_read` | 提示操作 |

### 密码学操作

| 类别 | 系统调用 |
|------|---------|
| SHA | `syscall_sha_compress` |
| Keccak | `syscall_keccak_permute` |
| Edwards | `syscall_ed_add`, `syscall_ed_decompress` |
| Secp256k1 | `syscall_secp256k1_add`, `syscall_secp256k1_double`, `syscall_secp256k1_decompress` |
| BN254 | `syscall_bn254_add`, `syscall_bn254_double` |
| BLS12-381 | `syscall_bls12381_add`, `syscall_bls12381_double`, `syscall_bls12381_decompress` |
| BigInt | `syscall_uint256_mul` |

---

## 🖥️ P3 芯片资源统计 (Chip Resource Statistics)

### Core 阶段主要芯片

| 芯片名称 | Rows 指标 | Cells 指标 | 说明 |
|---------|----------|-----------|------|
| CPU | `chip_cpu_rows` | `chip_cpu_cells` | CPU执行核心 |
| Program | `chip_program_rows` | `chip_program_cells` | 程序存储 |
| Global | `chip_global_rows` | `chip_global_cells` | 全局状态 |
| Byte | `chip_byte_rows` | `chip_byte_cells` | 字节查找表 |
| MemoryGlobalInit | `chip_memory_global_init_rows` | `chip_memory_global_init_cells` | 内存初始化 |
| MemoryGlobalFinalize | `chip_memory_global_finalize_rows` | `chip_memory_global_finalize_cells` | 内存终结 |

### Compress 阶段主要芯片

| 芯片名称 | Rows 指标 | Cells 指标 |
|---------|----------|-----------|
| BatchFRI | `chip_compress_batch_fri_rows` | `chip_compress_batch_fri_cells` |
| MemoryVar | `chip_compress_memory_var_rows` | `chip_compress_memory_var_cells` |
| Poseidon2 | `chip_compress_poseidon2_rows` | `chip_compress_poseidon2_cells` |
| BaseAlu | `chip_compress_base_alu_rows` | `chip_compress_base_alu_cells` |
| ExtAlu | `chip_compress_ext_alu_rows` | `chip_compress_ext_alu_cells` |

---

## 🌲 P3 Merkle Tree 维度信息

记录各阶段 Merkle Tree 的结构信息：

| 指标名称 | 示例值 |
|---------|--------|
| `core_merkle_dimensions` | `"[14x1048576, 11x131072]"` |
| `compress_merkle_dimensions` | `"[13x1048576, 8x1048576, ...]"` |
| `shrink_merkle_dimensions` | `"[48x1048576, 8x1048576, ...]"` |
| `wrap_merkle_dimensions` | `"[51x4194304, 32x2097152, ...]"` |

格式说明：`"[列数x行数, 列数x行数, ...]"`

---

## 📊 报告配置说明

### 默认报告指标

配置文件中 `[reporting]` 部分定义了默认输出的关键指标（~30个），包括：

- 基础性能指标（cycles, instructions, time）
- 各阶段时间分解（core, compress, shrink, wrap, groth16）
- 证明大小和验证指标
- Groth16 关键参数
- 资源使用统计

### 分组报告

配置文件还定义了多个指标组，用于生成分类报告：

1. **execution** - 执行阶段指标
2. **proving** - 证明阶段时间分解
3. **core_details** - Core 阶段详细指标
4. **compress_details** - Compress 阶段详细指标
5. **shrink_details** - Shrink 阶段详细指标
6. **wrap_details** - Wrap 阶段详细指标
7. **groth16_details** - Groth16 阶段详细指标
8. **instruction_stats** - 指令统计
9. **syscall_stats** - 系统调用统计
10. **chip_resources** - 芯片资源统计

---

## 🎯 使用建议

### 快速分析
关注 P0 和 P1 指标，快速评估性能：
- 执行效率：`total_cycles`, `execution_time_s`, `khz`
- 证明性能：`total_prove_time_s` 及各阶段时间
- 证明大小：`final_proof_size_bytes`
- 验证性能：`verification_time_s`

### 深度优化
使用 P2 和 P3 指标进行深度分析：
- 指令热点：分析 `opcode_*` 统计
- 系统调用开销：分析 `syscall_*` 统计
- 内存使用：分析芯片 `rows` 和 `cells` 统计
- 瓶颈识别：对比各阶段时间分布

### 对比测试
在不同配置下运行测试，对比：
- 不同 scale 的扩展性
- 不同证明模式的权衡
- 优化前后的性能变化

---

## 📝 注意事项

1. **正则表达式匹配**：所有指标使用正则表达式从日志中提取，确保日志格式一致
2. **可选指标**：部分指标可能在某些模式下不存在（如 core 模式无 Groth16 指标）
3. **单位统一**：注意时间单位（秒/毫秒）和大小单位（字节/KB）
4. **多行匹配**：指令和系统调用统计使用 `\n` 进行多行匹配
5. **数值解析**：支持整数和浮点数，使用 `[\d.]+` 或 `\d+` 匹配

---

## 🔗 相关文件

- **配置文件**：`configs/sp1.toml`
- **原始日志**：`benchmark-results/raw-logs/*.log`
- **解析结果**：`benchmark-results/parsed-metrics/*.json`
- **报告输出**：根据 `output_formats` 配置生成

---

*最后更新：2025-11-19*
*配置版本：v1.0*

