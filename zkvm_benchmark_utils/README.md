# zkVM Benchmark Framework

自动化 zkVM 性能测试框架，支持多种零知识虚拟机的统一 benchmark。

## 🚀 快速开始

### 构建

```bash
cd zkvm_benchmark_utils
cargo build --release
```

### 运行 Benchmark

```bash
# 运行指定 zkVM（从 configs/{zkvm}.toml 加载配置）
cargo run --bin zkvm-benchmark -- run --zkvms sp1

# 运行多个 zkVM
cargo run --bin zkvm-benchmark -- run --zkvms sp1,risc0

# 运行所有启用的 zkVM（自动扫描 configs/ 目录）
cargo run --bin zkvm-benchmark -- run

# 指定测试规模
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --scales 10,100,1000

# 生成 JSON 报告
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --report-formats json
```

## 📁 配置结构

```
zkvm_benchmark_utils/
├── configs/               # 配置目录
│   ├── sp1.toml          # SP1 配置
│   ├── risc0.toml        # Risc0 配置
│   ├── nexus.toml        # Nexus 配置
│   ├── jolt.toml         # Jolt 配置
│   └── template.toml     # 新 zkVM 模板
└── benchmark-results/     # 输出目录
    ├── raw-logs/
    ├── parsed-metrics/
    └── reports/
```

## ⚙️ 配置文件

### 配置加载逻辑

- **运行指定 zkVM**: `--zkvms sp1` → 加载 `configs/sp1.toml`
- **运行多个 zkVM**: `--zkvms sp1,risc0` → 加载 `configs/sp1.toml` 和 `configs/risc0.toml`
- **运行所有**: 不指定 `--zkvms` → 扫描 `configs/` 目录，加载所有 `enabled = true` 的配置

### 配置文件示例：`configs/sp1.toml`

```toml
name = "sp1"
version = "v5.0.0"
enabled = true
default_mode = "groth16"

# 测试规模（向后兼容，如果配置了 programs 则忽略）
test_scales = [10, 20]

# 程序配置（推荐方式）
[[programs]]
name = "fibonacci"
scales = [10, 20]

# 可以为不同程序配置不同的参数
[[programs]]
name = "hash"
scales = [100, 1000]
timeout_seconds = 7200  # 可选：程序特定超时

[[programs]]
name = "sum"
scales = [100, 1000, 10000]

# 证明模式
prove_modes = ["core", "compressed", "groth16"]

# 项目路径
working_dir = "../sp1-zkvm/sp1-host"

# 命令
build_command = "cargo build --release"
run_command = "cargo run --release -- --prove"

# 超时设置
timeout_seconds = 3600
repeat_count = 1

[env_vars]
RUST_LOG = "debug"

[log_patterns]
total_cycles = 'BENCHMARK: total_cycles=(\d+)'
total_prove_time_s = 'BENCHMARK: total_prove_time_s=([\d.]+)'
final_proof_size_bytes = 'BENCHMARK: final_proof_size_bytes=(\d+)'
verification_time_s = 'BENCHMARK: verification_time_s=([\d.]+)'
```

## 📊 添加新 zkVM

### 步骤

1. **复制模板**：
```bash
cp configs/template.toml configs/your_zkvm.toml
```

2. **编辑配置**：
```toml
name = "your_zkvm"
version = "v1.0.0"
enabled = true
working_dir = "../your-zkvm/host"
run_command = "cargo run --release"

[log_patterns]
total_cycles = 'BENCHMARK: total_cycles=(\d+)'
total_prove_time_s = 'BENCHMARK: total_prove_time_s=([\d.]+)'
final_proof_size_bytes = 'BENCHMARK: final_proof_size_bytes=(\d+)'
verification_time_s = 'BENCHMARK: verification_time_s=([\d.]+)'
```

3. **zkVM Host 输出 BENCHMARK 格式**：
```rust
println!("BENCHMARK: total_cycles={}", cycles);
println!("BENCHMARK: total_prove_time_s={:.6}", time.as_secs_f64());
println!("BENCHMARK: final_proof_size_bytes={}", proof.len());
println!("BENCHMARK: verification_time_s={:.6}", verify_time.as_secs_f64());
```

4. **运行**：
```bash
cargo run --bin zkvm-benchmark -- run --zkvms your_zkvm
```

## 📈 输出格式

### JSON 报告示例

```json
{
  "metrics": [{
    "metadata": {
      "program_name": "fibonacci_10",
      "zkvm_name": "sp1",
      "zkvm_version": "v5.0.0",
      "hardware": {
        "cpu_brand": "Apple M4 Pro",
        "cpu_cores": 14,
        "total_memory_mb": 50331648
      }
    },
    "execution_phase": {
      "total_cycles": 20,
      "total_instruction_count": 7409,
      "execution_time_s": 0.003
    },
    "proving_phase": {
      "proof_mode": "groth16",
      "total_prove_time_s": 141.64,
      "proof_size_evolution": {
        "final_proof_size_bytes": 260
      }
    },
    "verification_phase": {
      "verification_time_s": 0.00194,
      "on_chain_gas_estimate": 280000
    }
  }],
  "summary": {
    "total_runs": 1,
    "successful": 1,
    "failed": 0
  }
}
```

## 🔧 命令参考

```bash
# 运行指定 zkVM
cargo run --bin zkvm-benchmark -- run --zkvms sp1

# 运行多个 zkVM
cargo run --bin zkvm-benchmark -- run --zkvms sp1,risc0

# 运行所有启用的 zkVM
cargo run --bin zkvm-benchmark -- run

# 指定测试规模（覆盖配置文件）
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --scales 10,100

# 指定要运行的程序（覆盖配置文件）
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --programs fibonacci,hash,sum

# 同时指定程序和规模
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --programs fibonacci,hash --scales 10,100

# 指定报告格式
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --report-formats json,csv

# 指定输出目录
cargo run --bin zkvm-benchmark -- run --zkvms sp1 --output ./my-results
```

## 🐛 故障排查

### 检查配置文件

```bash
# 查看所有配置
ls -1 configs/*.toml

# 查看 SP1 配置
cat configs/sp1.toml
```

### 检查日志

```bash
# 查看原始日志
cat benchmark-results/raw-logs/*.log | grep "BENCHMARK:"

# 查看解析后的 metrics
cat benchmark-results/parsed-metrics/*.json | jq '.'
```

### 测试单个 zkVM

```bash
# 运行并查看详细输出
RUST_LOG=debug cargo run --bin zkvm-benchmark -- run --zkvms sp1
```

## 📝 核心指标

### P0 必需指标
- `total_cycles`: 总计算周期
- `total_prove_time_s`: 证明时间
- `final_proof_size_bytes`: 最终 proof 大小
- `verification_time_s`: 验证时间

### P1 高优先级
- `vm_core_proof_size_kb`: VM STARK proof
- `compressed_proof_size_kb`: 压缩后 proof
- `groth16_proof_size_bytes`: Groth16 proof

### 硬件信息（自动采集）
- CPU 型号和核心数
- 内存大小
- 操作系统信息

## 📊 支持的 zkVM

| zkVM | 配置文件 | 状态 |
|------|---------|------|
| SP1 | `configs/sp1.toml` | ✅ 完整测试 |
| Risc0 | `configs/risc0.toml` | 🔧 待配置 |
| Nexus | `configs/nexus.toml` | 🔧 待添加 |
| Jolt | `configs/jolt.toml` | 🔧 待添加 |

## 📋 支持的程序

框架支持以下程序（可通过 `--programs` 参数或配置文件指定）：

- `fibonacci` / `fib`: 计算第 n 个斐波那契数
- `sum`: 计算 1 到 n 的和
- `factorial` / `fact`: 计算 n 的阶乘
- `isprime` / `prime`: 判断 n 是否为质数
- `popcount` / `bitcount`: 计算 n 的二进制中 1 的个数
- `hash` / `sha256`: SHA256 哈希计算
- `signature` / `sig` / `ecdsa`: ECDSA 签名验证

所有程序都通过 `PROGRAM_ID` 环境变量传递给 zkVM host，参数通过 `PROGRAM_N` 环境变量传递（对于 Fibonacci，也支持 `FIBONACCI_N` 以保持向后兼容）。

---

**Version**: 2.0.0  
**Last Updated**: 2025-11-18  
**Status**: ✅ Production Ready
