# Novanet zkVM 使用指南

## 📋 目录

- [快速开始](#快速开始)
- [基本使用](#基本使用)
- [高级用法](#高级用法)
- [常见问题](#常见问题)
- [性能优化](#性能优化)
- [故障排除](#故障排除)

## 🚀 快速开始

### 1. 环境要求

- **Rust**: 1.85 或更高版本
- **操作系统**: macOS, Linux, Windows
- **内存**: 最小 2GB RAM
- **磁盘空间**: 500MB

### 2. 安装

```bash
# 方法 1: 使用安装脚本（推荐）
cd scripts/sdk_installers
./install_novanet_sdk.sh

# 方法 2: 手动安装
rustup toolchain install 1.85
rustup default 1.85
```

### 3. 第一次运行

```bash
cd novanet-zkvm

# 运行默认演示
./run_demo.sh
```

预期输出：
```
✅ Novanet zkVM Demo completed successfully!
```

## 📖 基本使用

### 运行演示

```bash
# 使用默认输入 (Fibonacci 5)
./run_demo.sh

# 使用自定义输入
FIBONACCI_N=10 ./run_demo.sh

# 使用 cargo 直接运行
cargo run --release -p novanet-host

# 设置环境变量
export FIBONACCI_N=15
cargo run --release -p novanet-host
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_fibonacci

# 显示测试输出
cargo test -- --nocapture

# 运行验证脚本
./verify.sh
```

### 构建选项

```bash
# Debug 构建（更快的编译，较慢的运行）
cargo build
./run_demo.sh --debug

# Release 构建（较慢的编译，更快的运行）
cargo build --release
./run_demo.sh

# 仅构建，不运行
./run_demo.sh --build

# 清理构建产物
cargo clean
./run_demo.sh --clean
```

## 🎯 高级用法

### 1. 修改 Guest 程序

编辑 `novanet-guest/src/lib.rs`：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyInput {
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyOutput {
    pub result: u32,
}

/// 自定义计算函数
pub fn my_computation(input: MyInput) -> MyOutput {
    // 在这里实现你的逻辑
    let result = input.value * input.value;
    MyOutput { result }
}
```

### 2. 修改 Host 程序

编辑 `novanet-host/src/main.rs`：

```rust
use guest::{my_computation, MyInput};

fn main() -> Result<()> {
    let input = MyInput { value: 42 };
    
    // 生成证明
    let proof = prover.prove(input)?;
    
    // 验证证明
    let is_valid = prover.verify(&proof)?;
    
    println!("Result: {}", proof.output.result);
    Ok(())
}
```

### 3. 批量测试

创建测试脚本 `test_batch.sh`：

```bash
#!/bin/bash

for n in 5 10 15 20 25 30; do
    echo "Testing Fibonacci($n)..."
    FIBONACCI_N=$n cargo run --release -q -p novanet-host | grep "Result:"
done
```

### 4. 性能测量

```bash
# 使用 time 命令
time FIBONACCI_N=20 cargo run --release -p novanet-host

# 使用 hyperfine 基准测试
hyperfine 'FIBONACCI_N=10 cargo run --release -p novanet-host'

# 使用 perf 分析（Linux）
perf record -g cargo run --release -p novanet-host
perf report
```

### 5. 自定义配置

创建 `.env` 文件：

```bash
# .env
FIBONACCI_N=15
RUST_LOG=debug
RUST_BACKTRACE=1
```

然后运行：

```bash
cargo run --release -p novanet-host
```

## 💡 使用场景

### 场景 1: 教学演示

```bash
# 展示零知识证明的基本概念
echo "演示 1: 计算 Fibonacci(10)"
FIBONACCI_N=10 ./run_demo.sh

echo -e "\n演示 2: 验证证明而不透露计算过程"
# 输出显示证明被验证，但不显示中间步骤
```

### 场景 2: 开发测试

```bash
# 测试不同输入
for n in {1..20}; do
    echo "Testing n=$n"
    FIBONACCI_N=$n cargo run --release -p novanet-host 2>&1 | \
        grep -E "Result|verified"
done
```

### 场景 3: 性能基准

```bash
# 创建性能报告
echo "Input,ProveTime,VerifyTime" > benchmark.csv

for n in 5 10 15 20 25; do
    FIBONACCI_N=$n cargo run --release -p novanet-host 2>&1 | \
        grep -E "time" >> benchmark.csv
done
```

## ❓ 常见问题

### Q1: 如何更改默认的 Fibonacci 数？

**A**: 设置环境变量：

```bash
export FIBONACCI_N=20
./run_demo.sh
```

### Q2: 如何查看详细的执行日志？

**A**: 设置 RUST_LOG 环境变量：

```bash
RUST_LOG=debug cargo run --release -p novanet-host
```

### Q3: 证明数据保存在哪里？

**A**: 当前实现在内存中生成和验证证明。要保存证明：

```rust
// 在 host 程序中添加
use std::fs;
let proof_json = serde_json::to_string(&proof)?;
fs::write("proof.json", proof_json)?;
```

### Q4: 如何验证已保存的证明？

**A**: 添加验证功能：

```rust
// 读取证明
let proof_json = fs::read_to_string("proof.json")?;
let proof: NovanetProof = serde_json::from_str(&proof_json)?;

// 验证
let is_valid = prover.verify(&proof)?;
```

### Q5: 可以证明其他类型的计算吗？

**A**: 可以！修改 guest 程序以实现你需要的计算。参考"高级用法"部分。

### Q6: 这个实现是否安全用于生产？

**A**: 不是。这是一个演示实现。生产环境需要集成真实的 Nova 证明系统。

## 🔧 性能优化

### 1. 编译优化

在 `Cargo.toml` 中：

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

### 2. 并行处理

```rust
// 使用 rayon 并行处理多个证明
use rayon::prelude::*;

let inputs: Vec<u32> = (1..=10).collect();
let proofs: Vec<_> = inputs
    .par_iter()
    .map(|&n| {
        let input = FibInput { n };
        prover.prove(input)
    })
    .collect();
```

### 3. 缓存优化

```rust
// 缓存编译结果
lazy_static! {
    static ref PROVER: NovanetProver = 
        NovanetProver::compile_guest().unwrap();
}
```

## 🐛 故障排除

### 问题 1: 编译错误

**症状**: `error: failed to compile`

**解决方案**:
```bash
# 清理并重建
cargo clean
cargo build --release

# 检查 Rust 版本
rustc --version  # 应该是 1.85+

# 更新 Rust
rustup update
```

### 问题 2: 运行时错误

**症状**: 程序崩溃或 panic

**解决方案**:
```bash
# 启用详细日志
RUST_BACKTRACE=full RUST_LOG=trace cargo run

# 使用 debug 构建获取更多信息
cargo build
cargo run -p novanet-host
```

### 问题 3: 性能问题

**症状**: 运行缓慢

**解决方案**:
```bash
# 确保使用 release 构建
cargo build --release
cargo run --release

# 检查优化级别
cargo rustc --release -- --print cfg | grep opt_level
```

### 问题 4: 内存不足

**症状**: `out of memory` 错误

**解决方案**:
```bash
# 使用较小的输入
FIBONACCI_N=10 ./run_demo.sh

# 增加系统交换空间
# 或升级硬件
```

### 问题 5: 测试失败

**症状**: `cargo test` 失败

**解决方案**:
```bash
# 查看具体失败原因
cargo test -- --nocapture

# 运行单个测试
cargo test test_fibonacci -- --nocapture

# 重新生成测试数据
cargo clean
cargo test
```

## 📊 输出说明

### 正常输出

```
========================================
Novanet zkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(10)...

1️⃣  Compiling guest program...
   ✓ Compilation completed in 0.00s
```

各部分说明：
- **1️⃣ Compiling**: 编译 guest 程序为电路
- **2️⃣ Setting up**: 初始化证明系统
- **3️⃣ Generating proof**: 生成零知识证明
- **4️⃣ Verifying proof**: 验证证明正确性

### 性能指标

```
Compile time:     0.00s  ← 编译时间
Setup time:       0.00s  ← 设置时间
Prove time:       0.00s  ← 证明生成时间
Verify time:      0.00s  ← 验证时间
Total time:       0.00s  ← 总时间
```

### 证明信息

```
✓ Result: fibonacci(10) = 89     ← 计算结果
✓ Proof size: 29 bytes           ← 证明大小
✓ Proof verified successfully    ← 验证成功
```

## 🔗 相关资源

### 文档

- [README.md](README.md) - 项目概述
- [QUICK_START.md](QUICK_START.md) - 5分钟快速入门
- [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md) - 技术深度解析
- [VERIFICATION_REPORT.md](VERIFICATION_REPORT.md) - 验证报告

### 工具

- `./run_demo.sh` - 运行演示
- `./verify.sh` - 验证测试
- `cargo test` - 单元测试
- `cargo bench` - 性能测试（如果配置）

### 外部资源

- [Nova 论文](https://eprint.iacr.org/2021/370)
- [Nova GitHub](https://github.com/microsoft/nova)
- [zkVM 基准测试](https://github.com/kkrt-labs/zkvm-benchmarks)

## 📝 最佳实践

### 1. 开发流程

```bash
# 1. 修改代码
vim novanet-guest/src/lib.rs

# 2. 运行测试
cargo test

# 3. 本地验证
./verify.sh

# 4. 性能测试
time ./run_demo.sh

# 5. 提交前检查
cargo fmt
cargo clippy
```

### 2. 调试技巧

```bash
# 使用 println! 调试
# 在代码中添加：
println!("Debug: value = {:?}", value);

# 使用调试器
rust-lldb target/debug/novanet-host

# 使用日志
RUST_LOG=debug cargo run
```

### 3. 测试策略

```bash
# 单元测试
cargo test

# 集成测试
./verify.sh

# 边界测试
FIBONACCI_N=0 ./run_demo.sh
FIBONACCI_N=30 ./run_demo.sh

# 压力测试
for i in {1..100}; do
    FIBONACCI_N=$((RANDOM % 20 + 1)) ./run_demo.sh
done
```

## 🎓 学习路径

### 初学者

1. 阅读 [QUICK_START.md](QUICK_START.md)
2. 运行基本演示
3. 修改输入值实验
4. 理解输出含义

### 中级用户

1. 阅读 [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
2. 修改 guest 程序
3. 自定义计算逻辑
4. 添加新的测试用例

### 高级用户

1. 研究 Nova 论文
2. 集成真实 Nova 实现
3. 优化电路设计
4. 贡献改进

## 📞 获取帮助

如果遇到问题：

1. 查看 [常见问题](#常见问题)
2. 查看 [故障排除](#故障排除)
3. 阅读详细文档
4. 查看示例代码
5. 提交 Issue（如果是 bug）

---

**祝你使用愉快！ 🎉**

如有问题或建议，欢迎反馈！

