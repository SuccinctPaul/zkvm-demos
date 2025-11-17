# Stwo-Cairo 集成指南

## 概述

本文档说明如何将当前的 Cairo zkVM 项目迁移到使用 [Stwo-Cairo](https://github.com/starkware-libs/stwo-cairo) 证明器。

Stwo-Cairo 是 StarkWare 推出的新一代超快速证明器，基于 Circle STARKs 密码学突破技术。

## 兼容性分析

### ✅ 当前项目优势
- **Cairo 2.x**: 项目使用 Cairo 2.8.5 (edition 2024_07)
- **完整的 main() 函数**: 已有可执行的主函数
- **无 Syscalls**: 当前代码不使用系统调用
- **纯计算逻辑**: Fibonacci 计算非常适合 zkVM 证明

### ⚠️ 需要修改的部分

| 项目 | 当前状态 | Stwo-Cairo 要求 | 修改难度 |
|------|----------|-----------------|----------|
| **Scarb 版本** | 2.8.5 | ≥ 2.10.0 (推荐 nightly) | 🟡 中等 |
| **Gas 配置** | 启用 Gas tracking | `enable-gas = false` | 🟢 简单 |
| **Main 函数签名** | `fn main() -> (...)` | `fn main(n: u32)` | 🟢 简单 |
| **cairo-prove 工具** | 未安装 | 必需 | 🟡 中等 |

## 详细实施步骤

### 步骤 1: 升级 Scarb (必需)

```bash
# 使用 asdf 升级到最新 nightly 版本
asdf install scarb latest:nightly
asdf local scarb latest:nightly

# 或者安装最新稳定版 (≥ 2.10.0)
asdf install scarb 2.10.0
asdf local scarb 2.10.0

# 验证版本
scarb --version  # 应显示 ≥ 2.10.0
```

### 步骤 2: 安装 Stwo-Cairo 工具链

```bash
# 克隆 stwo-cairo 仓库
cd /tmp
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo/cairo-prove

# 构建 cairo-prove 工具
./build.sh

# 将二进制文件添加到 PATH
sudo cp target/release/cairo-prove /usr/local/bin/

# 验证安装
cairo-prove --help
```

### 步骤 3: 修改项目配置

#### 3.1 更新 `Scarb.toml`

```toml
[package]
name = "cairo_fibonacci"
version = "0.1.0"
edition = "2024_07"

[dependencies]
starknet = ">=2.10.0"

# ⚠️ 关键配置：禁用 Gas tracking
[cairo]
enable-gas = false

[dev-dependencies]
cairo_test = "2.10.0"

[lib]

[scripts]
test = "scarb cairo-test"
```

#### 3.2 修改 `src/lib.cairo` 主函数

当前的 main() 函数：

```cairo
fn main() -> (felt252, felt252, felt252, felt252, felt252) {
    let n: felt252 = 10;
    let result_recursive = fib_recursive(n);
    let result_iterative = fib_iterative(n);
    let (fib_n, fib_n_plus_1) = fib_pair(n);
    (n, result_recursive, result_iterative, fib_n, fib_n_plus_1)
}
```

**修改为接受参数的版本**：

```cairo
/// Stwo-Cairo 兼容的主函数
/// 
/// # Arguments
/// * `n` - Fibonacci 序列位置 (例如: 10)
/// 
/// # Returns
/// 第 n 个 Fibonacci 数
fn main(n: u32) -> u32 {
    // 使用迭代方法计算（更高效）
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut i: u32 = 2;
    
    loop {
        if i > n {
            break;
        }
        let temp = a + b;
        a = b;
        b = temp;
        i += 1;
    };
    
    b
}
```

**或者保留原有功能的增强版本**：

```cairo
/// Stwo-Cairo 兼容的主函数（完整版）
/// 计算多个 Fibonacci 结果并返回
fn main(n: u32) -> (u32, u32, u32) {
    let n_felt: felt252 = n.into();
    let result_recursive = fib_recursive(n_felt);
    let result_iterative = fib_iterative(n_felt);
    let (fib_n, _) = fib_pair(n_felt);
    
    // 转换回 u32 类型
    let r1: u32 = result_recursive.try_into().unwrap();
    let r2: u32 = result_iterative.try_into().unwrap();
    let r3: u32 = fib_n.try_into().unwrap();
    
    (r1, r2, r3)
}
```

### 步骤 4: 构建和证明生成

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 1. 构建项目
scarb build

# 2. 生成 Fibonacci(10) 的零知识证明
cairo-prove prove \
  target/dev/cairo_fibonacci.executable.json \
  ./stwo_proof.json \
  --arguments 10

# 3. 验证证明
cairo-prove verify ./stwo_proof.json
```

### 步骤 5: 创建自动化脚本

创建 `run_stwo_demo.sh`:

```bash
#!/bin/bash
set -e

echo "🚀 Stwo-Cairo zkVM Demo - Fibonacci Proof Generation"
echo "=================================================="

# 检查 cairo-prove 是否安装
if ! command -v cairo-prove &> /dev/null; then
    echo "❌ Error: cairo-prove not found"
    echo "Please install stwo-cairo first. See STWO_INTEGRATION_GUIDE.md"
    exit 1
fi

# 检查 Scarb 版本
SCARB_VERSION=$(scarb --version | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1)
REQUIRED_VERSION="2.10.0"

if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$SCARB_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    echo "⚠️  Warning: Scarb version $SCARB_VERSION < $REQUIRED_VERSION"
    echo "Recommended: upgrade to Scarb 2.10.0+"
fi

# 构建项目
echo ""
echo "📦 Building project..."
scarb build

# 检查是否生成了可执行文件
EXECUTABLE="target/dev/cairo_fibonacci.executable.json"
if [ ! -f "$EXECUTABLE" ]; then
    echo "❌ Error: Executable not found at $EXECUTABLE"
    exit 1
fi

# 生成证明
echo ""
echo "🔐 Generating zero-knowledge proof for Fibonacci(10)..."
cairo-prove prove \
  "$EXECUTABLE" \
  ./stwo_proof.json \
  --arguments 10

# 验证证明
echo ""
echo "✅ Verifying proof..."
cairo-prove verify ./stwo_proof.json

echo ""
echo "=================================================="
echo "✨ Success! Proof generated and verified."
echo "Proof file: ./stwo_proof.json"
echo "=================================================="
```

## 性能对比

| 证明系统 | 证明生成时间 | 证明大小 | 验证时间 | 状态 |
|---------|------------|---------|---------|------|
| **Stone Prover** | ~30-60秒 | ~200KB | ~2秒 | ✅ 已部署 |
| **Stwo-Cairo** | ~10-20秒 | ~150KB | ~1秒 | 🚀 新一代 |
| **StarkNet Katana** | ~5-10秒 | ~100KB | ~1秒 | ✅ 推荐开发环境 |

## Stwo-Cairo 的优势

### 🚀 技术优势
1. **Circle STARKs**: 基于密码学突破，比传统 STARKs 更快
2. **无需 FFT**: 减少了计算开销
3. **更小的证明**: 证明大小通常减少 20-30%
4. **更快的验证**: 验证速度提升 2-3 倍

### 📈 适用场景
- ✅ 纯计算型任务（如 Fibonacci）
- ✅ 数学算法验证
- ✅ 密码学原语计算
- ❌ 需要复杂 Syscalls 的应用
- ❌ Gas tracking 敏感的应用

## 限制和注意事项

### ⚠️ Stwo-Cairo 限制

1. **No Gas Tracking** (必须禁用)
   ```toml
   [cairo]
   enable-gas = false
   ```

2. **No Syscalls** (不支持以下功能)
   - `sha256`, `keccak`
   - `secp256k1` / `secp256r1` 操作
   - 任何直接或间接的系统调用

3. **Padding** (当前版本)
   - 执行资源会填充到下一个 2 的幂次
   - 未来版本将移除此限制

4. **Pedersen Builtin** (特殊处理)
   - 如果使用 Pedersen，验证时需添加 `--with-pedersen` 标志

### 📋 当前项目兼容性检查

```bash
# 检查是否使用了不兼容的功能
grep -r "syscall" src/
grep -r "sha256\|keccak" src/
grep -r "secp256" src/

# ✅ 当前项目：没有使用任何不兼容的功能
# ✅ 纯计算逻辑，非常适合 Stwo-Cairo
```

## 快速测试清单

```bash
# 1. 检查 Scarb 版本
scarb --version  # ≥ 2.10.0 ?

# 2. 检查 cairo-prove 安装
cairo-prove --help

# 3. 构建项目
scarb build

# 4. 生成测试证明
cairo-prove prove \
  target/dev/cairo_fibonacci.executable.json \
  ./test_proof.json \
  --arguments 5

# 5. 验证证明
cairo-prove verify ./test_proof.json

# 6. 清理测试文件
rm -f test_proof.json
```

## 故障排除

### 问题 1: "Scarb version too old"
```bash
# 解决方案：升级 Scarb
asdf install scarb latest:nightly
asdf local scarb latest:nightly
```

### 问题 2: "enable-gas must be false"
```bash
# 解决方案：在 Scarb.toml 中添加
[cairo]
enable-gas = false
```

### 问题 3: "cairo-prove: command not found"
```bash
# 解决方案：克隆并构建 stwo-cairo
cd /tmp
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo/cairo-prove
./build.sh
sudo cp target/release/cairo-prove /usr/local/bin/
```

### 问题 4: "main function signature mismatch"
```bash
# 解决方案：修改 main 函数接受参数
# 从: fn main() -> ReturnType
# 到:  fn main(n: u32) -> ReturnType
```

## 结论和建议

### ✅ 推荐使用 Stwo-Cairo 的场景
- 当前 Fibonacci 项目**非常适合** Stwo-Cairo
- 纯计算任务，无需 Gas tracking
- 追求最高性能和最小证明体积

### 📊 迁移工作量估算
- **简单修改**: 2-3 小时
  - Scarb 升级: 30 分钟
  - cairo-prove 安装: 30 分钟
  - 代码修改: 1 小时
  - 测试验证: 1 小时

### 🎯 建议行动方案

**选项 A: 全面迁移到 Stwo-Cairo** (推荐)
```bash
# 按照本文档步骤 1-5 执行
# 优势：最新技术、最佳性能
# 工作量：2-3 小时
```

**选项 B: 创建 Stwo 分支** (保守)
```bash
# 保留原有实现，创建 stwo-cairo 分支
git checkout -b stwo-cairo-integration
# 按照本文档进行修改
# 优势：保留两种实现，可以对比
```

**选项 C: 暂时不迁移** (观望)
```bash
# 继续使用 Stone Prover 或 Katana
# 优势：稳定可靠
# 劣势：错过性能提升
```

## 参考资源

- [Stwo-Cairo GitHub](https://github.com/starkware-libs/stwo-cairo)
- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [Circle STARKs Paper](https://eprint.iacr.org/2024/278)

---

**文档版本**: 1.0  
**创建日期**: 2025-11-16  
**适用于**: Cairo 2.x + Stwo-Cairo  
**维护者**: zkVM Demos Project

