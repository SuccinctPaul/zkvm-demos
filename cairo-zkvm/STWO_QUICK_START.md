# Stwo-Cairo 快速开始指南

## 简介

本指南将帮助您快速上手使用 Stwo-Cairo 证明器来生成 Cairo 程序的零知识证明。

**Stwo-Cairo** 是 StarkWare 最新的证明器，基于 Circle STARKs 技术，比传统 STARK 证明器更快、更高效。

## 一键安装和运行

### 选项 A: 完整自动化（推荐）

```bash
# 1. 安装 Stwo-Cairo 工具链
./install_stwo.sh

# 2. 运行演示
./run_stwo_demo.sh
```

就这么简单！脚本会自动处理所有步骤。

### 选项 B: 手动安装

如果您想更细致地控制安装过程：

```bash
# 1. 克隆 stwo-cairo 仓库
cd /tmp
git clone https://github.com/starkware-libs/stwo-cairo.git
cd stwo-cairo/cairo-prove

# 2. 构建
./build.sh

# 3. 安装
sudo cp target/release/cairo-prove /usr/local/bin/

# 4. 验证
cairo-prove --version
```

## 运行第一个证明

### 步骤 1: 构建项目

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 使用 Stwo 配置
cp Scarb.stwo.toml Scarb.toml
cp src/lib.stwo.cairo src/lib.cairo

# 构建
scarb build
```

### 步骤 2: 生成证明

```bash
# 为 Fibonacci(10) 生成证明
cairo-prove prove \
  target/dev/cairo_fibonacci.executable.json \
  ./my_first_proof.json \
  --arguments 10
```

**预期输出**：
```
[INFO] Generating proof for target: "target/dev/cairo_fibonacci.executable.json"
[INFO] Executing program...
[INFO] Program executed successfully.
[INFO] Generating input for the prover...
[INFO] Input for the prover generated successfully.
[INFO] Proof saved to: "./my_first_proof.json"
[INFO] Proof generation completed in 15.74s
```

### 步骤 3: 验证证明

```bash
# 验证证明
cairo-prove verify ./my_first_proof.json
```

**预期输出**：
```
[INFO] Verifying proof from: "./my_first_proof.json"
[INFO] Verification successful
```

## 性能基准测试

使用自动化脚本测试不同输入的性能：

```bash
./run_stwo_demo.sh
```

这将生成多个证明并显示性能数据：

| Fibonacci(n) | 证明时间 | 证明大小 | 验证时间 |
|--------------|----------|---------|---------|
| n=5          | ~10s     | ~150KB  | ~1s     |
| n=10         | ~15s     | ~150KB  | ~1s     |
| n=15         | ~20s     | ~160KB  | ~1s     |
| n=20         | ~25s     | ~170KB  | ~1s     |

## 理解参数

### 输入格式

Cairo-prove 支持两种输入格式：

#### 1. 命令行参数（`--arguments`）

```bash
# 单个参数
cairo-prove prove executable.json proof.json --arguments 10

# 多个参数（逗号分隔）
cairo-prove prove executable.json proof.json --arguments 1,2,3

# 对应 Cairo 函数:
# fn main(n: u32) -> u32                    // --arguments 10
# fn main(a: u32, b: u32) -> u32           // --arguments 1,2
# fn main(a: u32, b: u32, c: u32) -> u32   // --arguments 1,2,3
```

#### 2. 参数文件（`--arguments-file`）

```bash
# 创建参数文件 args.json
echo '["0xa"]' > args.json  # 0xa = 10 in hex

# 使用参数文件
cairo-prove prove executable.json proof.json --arguments-file args.json
```

## 常见问题解决

### 问题 1: "cairo-prove: command not found"

**解决方案**：
```bash
# 运行安装脚本
./install_stwo.sh

# 或者手动添加到 PATH
export PATH="/usr/local/bin:$PATH"
```

### 问题 2: "Scarb version too old"

**解决方案**：
```bash
# 使用 asdf 升级 Scarb
asdf install scarb latest:nightly
asdf local scarb latest:nightly

# 验证版本
scarb --version  # 应显示 ≥ 2.10.0
```

### 问题 3: "enable-gas must be false"

**解决方案**：
```bash
# 在 Scarb.toml 中添加：
[cairo]
enable-gas = false

# 或者使用提供的 Stwo 配置
cp Scarb.stwo.toml Scarb.toml
```

### 问题 4: "Build failed"

**解决方案**：
```bash
# 清理并重新构建
rm -rf target/
scarb build

# 如果还是失败，检查 Scarb 版本
scarb --version

# 查看详细错误信息
scarb build --verbose
```

### 问题 5: 证明生成很慢

**可能的原因**：
- 输入值太大（n > 30 时会变慢）
- 使用递归算法而非迭代

**解决方案**：
```bash
# 使用较小的输入进行测试
cairo-prove prove executable.json proof.json --arguments 5

# 确保使用迭代算法（lib.stwo.cairo 已优化）
```

## 与其他证明系统对比

| 特性 | Stwo-Cairo | Stone Prover | StarkNet Katana |
|------|-----------|--------------|-----------------|
| **证明时间** | 🟢 快 (~15s) | 🟡 中等 (~40s) | 🟢 快 (~10s) |
| **证明大小** | 🟢 小 (~150KB) | 🟡 中等 (~200KB) | 🟢 小 (~100KB) |
| **设置复杂度** | 🟡 中等 | 🔴 复杂 | 🟢 简单 |
| **生产就绪** | 🟡 Beta | 🟢 是 | 🟢 是 |
| **Gas 支持** | 🔴 否 | 🟢 是 | 🟢 是 |
| **Syscalls** | 🔴 否 | 🟢 是 | 🟢 是 |

## 高级用法

### 批量生成证明

```bash
#!/bin/bash
# batch_prove.sh

for i in {1..20}; do
  echo "Generating proof for Fibonacci($i)..."
  cairo-prove prove \
    target/dev/cairo_fibonacci.executable.json \
    "./proofs/fib_${i}.json" \
    --arguments $i
done

echo "Generated 20 proofs!"
```

### 自定义 Main 函数

编辑 `src/lib.stwo.cairo`：

```cairo
// 示例 1: 返回多个值
fn main(n: u32) -> (u32, u32, u32) {
    let fib = fib_iterative(n);
    let (a, b) = fib_pair(n);
    (fib, a, b)
}

// 示例 2: 验证属性
fn main(n: u32) -> bool {
    let recursive = fib_recursive(n);
    let iterative = fib_iterative(n);
    recursive == iterative  // 证明两种方法一致
}

// 示例 3: 计算总和
fn main(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    loop {
        if i > n { break; }
        sum += fib_iterative(i);
        i += 1;
    };
    sum
}
```

### 性能优化技巧

1. **使用迭代而非递归**：
   ```cairo
   // ✅ 推荐：迭代（O(n)）
   fib_iterative(n)
   
   // ❌ 避免：递归（O(2^n)）
   fib_recursive(n)
   ```

2. **减小输入大小**：
   - n ≤ 10: 非常快 (~10s)
   - n ≤ 20: 快速 (~20s)
   - n ≤ 30: 可接受 (~30s)
   - n > 30: 开始变慢

3. **使用简单类型**：
   ```cairo
   // ✅ 推荐：u32（更快）
   fn main(n: u32) -> u32
   
   // 🟡 可以但较慢：felt252
   fn main(n: felt252) -> felt252
   ```

## 学习资源

### 文档
- **STWO_INTEGRATION_GUIDE.md** - 完整集成指南
- **STARK_PROOF_GUIDE.md** - STARK 证明系统对比
- **PROJECT_INFO.md** - 项目背景信息

### 在线资源
- [Stwo-Cairo GitHub](https://github.com/starkware-libs/stwo-cairo)
- [Cairo Book](https://book.cairo-lang.org/)
- [Circle STARKs 论文](https://eprint.iacr.org/2024/278)
- [StarkWare 博客](https://medium.com/starkware)

## 下一步

1. **实验不同的计算**：
   - 修改 `lib.stwo.cairo` 实现新算法
   - 尝试质数检测、哈希函数（非 syscall）等

2. **集成到您的项目**：
   - 将 Stwo-Cairo 添加到 CI/CD 流程
   - 创建自动化测试

3. **优化性能**：
   - 分析证明生成的瓶颈
   - 尝试不同的算法实现

4. **探索高级特性**：
   - 使用 Pedersen builtin
   - 实现自定义约束

## 获取帮助

如果遇到问题：

1. 查看 `STWO_INTEGRATION_GUIDE.md` 的故障排除部分
2. 检查 [Stwo-Cairo Issues](https://github.com/starkware-libs/stwo-cairo/issues)
3. 运行诊断脚本：
   ```bash
   # 检查环境
   scarb --version
   cairo-prove --version
   rustc --version
   
   # 测试基本功能
   scarb build
   scarb test
   ```

---

**祝您使用愉快！** 🚀

有问题或建议？欢迎提交 Issue 或 Pull Request。

