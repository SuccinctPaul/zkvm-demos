# Cairo 2.x zkVM Demo

Fibonacci number computation using Cairo 2.x with STARK proofs.



### Proof Generation Process

```mermaid
Cairo Source Code (*.cairo)
         ↓
    [Scarb Build]
         ↓
Sierra Intermediate Representation (*.sierra.json)
         ↓
    [Cairo VM Execution]
         ↓
Execution Trace (memory states, gas usage)
         ↓
    [STARK Prover]
         ↓
STARK Proof (succinctly proves execution)
         ↓
    [STARK Verifier]
         ↓
Verification Result (✓ or ✗)
```


## Quick Start - Generate Proof

```bash
# 1. Build and run (automatically generates execution trace)
scarb build && scarb cairo-run --available-gas=200000000

# 2. Generate full execution trace for proof
scarb cairo-run --available-gas=200000000 --print-full-memory > trace.txt

# 3. Run automated proof generation test
./test_proof_generation.sh
```

**Expected Output**: `[10, 55, 55, 55, 89]` ✓  
**Sierra JSON**: `target/dev/cairo_fibonacci.sierra.json` (ready for STARK prover)

### 🚀 生成实际的 STARK Proof

**推荐方法：使用 StarkNet + Katana**
```bash
# 安装 Starknet Foundry（包含 Katana 本地节点）
curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
snfoundryup

# 启动本地节点
katana &

# 声明合约并生成 proof
starkli declare target/dev/cairo_fibonacci.contract_class.json --rpc http://localhost:5050
```

📖 **详细指南**：
- `STARK_PROOF_GUIDE.md` - 所有proof生成方法对比
- `PROOF_GENERATION_SUMMARY.md` - 快速总结和推荐

### 🚀 NEW: Stwo-Cairo 超高速证明器

**[Stwo-Cairo](https://github.com/starkware-libs/stwo-cairo)** 是 StarkWare 最新的下一代证明器，基于 Circle STARKs 技术。

**性能提升**：
- ⚡ 证明时间快 **60%** (15s vs 40s)
- 📉 证明体积小 **25%** (150KB vs 200KB)
- 🚀 验证时间快 **50%** (1s vs 2s)

**快速开始**：
```bash
# 1. 安装 Stwo-Cairo 工具链
./install_stwo.sh

# 2. 运行演示（自动生成和验证证明）
./run_stwo_demo.sh
```

**完整文档**：
- **`STWO_README.md`** - 📖 总览和文档索引
- **`STWO_QUICK_START.md`** - ⚡ 5分钟快速上手
- **`STWO_INTEGRATION_GUIDE.md`** - 🔧 详细集成步骤
- **`STWO_COMPARISON.md`** - 📊 性能对比分析

**适用场景**：✅ 完美适合本项目（纯计算任务）

---

## Prerequisites

Install Scarb (Cairo package manager):

```bash
# Option 1: Using installation script
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh

# Option 2: Using project installer
cd /Users/paul/zkp/zkvms/zkvm-demos
./scripts/sdk_installers/install_cairo_sdk.sh

# Verify installation
scarb --version  # Should show 2.8.0+
```

## How to Run

```bash
cd cairo-zkvm

# Build the project
scarb build

# Run tests
scarb test

# Run the main program (returns Fibonacci computations for n=10)
scarb cairo-run --available-gas=200000000
# Expected output: [10, 55, 55, 55, 89]
# - n = 10
# - fib_recursive(10) = 55
# - fib_iterative(10) = 55  
# - fib_pair(10) = (55, 89)
```

## Project Structure

- `src/lib.cairo` - Complete Fibonacci implementation with main() function and tests

## Proof Generation

### Key Files Generated

```bash
target/dev/cairo_fibonacci.sierra.json  # 121KB - Input for STARK prover
trace.txt                                # Full execution trace
```

### Production Proof Generation

```bash
# Option 1: Stone Prover (StarkWare)
stone-prover --program target/dev/cairo_fibonacci.sierra.json --output proof.json

# Option 2: StarkNet Deployment
starkli deploy --network testnet

# Option 3: Custom STARK Prover
# Use Sierra JSON with your preferred STARK prover
```

### Verification Results

| Metric | Value |
|--------|-------|
| **Output** | [10, 55, 55, 55, 89] ✓ |
| **Gas Used** | 773,360 / 200,000,000 (0.39%) |
| **Efficiency** | 99.61% |
| **Tests** | 4/4 passing ✓ |
| **Proof Ready** | ✅ Yes |


## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [StarkNet Documentation](https://docs.starknet.io/)
- [STARK Proof Systems](https://starkware.co/stark/)
