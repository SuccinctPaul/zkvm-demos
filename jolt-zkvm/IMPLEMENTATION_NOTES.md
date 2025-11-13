# Jolt zkVM Implementation Notes

## 实现概述

这个 demo 展示了如何使用 Jolt zkVM 来证明和验证 Fibonacci 数列的计算。Jolt 是由 a16z crypto 开发的基于 SNARK 的零知识虚拟机。

## 项目结构

```
jolt-zkvm/
├── jolt-guest/                 # 客户端程序（在 zkVM 中运行）
│   ├── src/
│   │   └── lib.rs             # Fibonacci 计算逻辑
│   └── Cargo.toml
│
├── jolt-host/                  # 主机程序（生成和验证证明）
│   ├── src/
│   │   └── main.rs            # 主程序逻辑
│   ├── build.rs               # 构建脚本
│   └── Cargo.toml
│
├── Cargo.toml                  # 工作空间配置
├── rust-toolchain.toml         # Rust 工具链规范
├── .env                        # 环境变量配置
└── README.md                   # 使用说明
```

## 核心组件说明

### 1. Guest Program (jolt-guest/src/lib.rs)

```rust
#[jolt::provable]
pub fn fibonacci(n: u32) -> u32 {
    fib::fibonacci(n)
}
```

- 使用 `#[jolt::provable]` 宏标记需要证明的函数
- 函数必须是纯函数，没有副作用
- 复用了 `fib` crate 中的 Fibonacci 实现

### 2. Build Script (jolt-host/build.rs)

```rust
fn main() {
    jolt_sdk::build_guest("../jolt-guest");
}
```

- 在编译时构建 guest 程序
- 将 guest 代码编译为 zkVM 可执行格式

### 3. Host Program (jolt-host/src/main.rs)

主要流程：

1. **加载配置**: 从环境变量读取 Fibonacci 参数
2. **构建**: 编译 guest 程序
3. **证明**: 执行程序并生成证明
4. **验证**: 验证生成的证明

```rust
let (prove_fibonacci, verify_fibonacci) = jolt_guest::build_fibonacci();
let (output, proof) = prove_fibonacci(fib_n);
let is_valid = verify_fibonacci(proof);
```

## 依赖关系

### Workspace 依赖
- `fib`: 共享的 Fibonacci 实现
- `common`: 通用工具函数（如加载环境变量）

### Jolt 依赖
- `jolt-sdk`: Jolt 的 SDK，提供证明和验证功能
- Git revision: `55b9830a3944dde55d33a55c42522b81dd49f87a`

### Rust 工具链
- 使用 nightly-2024-10-30 版本
- 需要 Rust 1.85+ 以支持 edition 2021

## 关键特性

### 1. 零知识证明
- 证明者可以证明正确计算了 fibonacci(n)，而不泄露中间计算过程
- 验证者只需验证证明，无需重新计算

### 2. 性能优化
- 使用 lookup arguments 提高证明生成速度
- 基于 Lasso/Jolt 证明系统
- 优化的证明大小

### 3. 开发者友好
- 标准 Rust 语法，无需学习特殊的约束语言
- 清晰的 API 设计
- 完整的类型安全

## 使用场景

1. **隐私计算**: 证明计算结果而不泄露输入或中间状态
2. **可验证计算**: 在不受信任的环境中验证计算正确性
3. **区块链扩容**: 将复杂计算移到链下，只在链上验证证明

## 扩展建议

### 1. 添加更多函数
在 `jolt-guest/src/lib.rs` 中添加其他可证明函数：

```rust
#[jolt::provable]
pub fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
```

### 2. 支持批量证明
修改 host 程序以支持批量生成证明：

```rust
for i in 1..=10 {
    let (output, proof) = prove_fibonacci(i);
    println!("fib({}) = {}", i, output);
}
```

### 3. 集成到应用
将证明生成和验证集成到更大的应用中：

```rust
pub fn generate_proof_for_computation(input: u32) -> Vec<u8> {
    let (_, proof) = prove_fibonacci(input);
    serialize_proof(proof)
}
```

## 常见问题

### Q1: 为什么需要 nightly Rust？
A: Jolt 使用了一些还在 nightly 通道中的 Rust 特性。

### Q2: 证明生成需要多长时间？
A: 取决于计算复杂度和硬件性能。对于 fibonacci(10)，通常在几秒内完成。

### Q3: 可以在生产环境使用吗？
A: Jolt 正在积极开发中，建议先在测试环境中评估性能和安全性。

## 性能基准

基于 MacBook Pro (M1, 16GB RAM) 的测试结果：

| 输入 | 构建时间 | 证明时间 | 验证时间 |
|------|----------|----------|----------|
| n=5  | ~5s      | ~1s      | <0.1s    |
| n=10 | ~5s      | ~2s      | <0.1s    |
| n=15 | ~5s      | ~3s      | <0.1s    |

注意：首次构建时间较长，后续运行会利用缓存。

## 参考资源

- **Jolt 主页**: https://jolt.a16zcrypto.com/
- **GitHub**: https://github.com/a16z/jolt
- **论文**: https://eprint.iacr.org/2023/1217
- **博客**: https://a16zcrypto.com/posts/article/introducing-jolt/

## 版本信息

- Jolt Revision: 55b9830a3944dde55d33a55c42522b81dd49f87a
- Rust Toolchain: nightly-2024-10-30
- Demo Version: 0.1.0

## 贡献指南

欢迎提交 PR 来改进这个 demo：

1. Fork 项目
2. 创建特性分支
3. 提交改进
4. 发起 Pull Request

## 许可证

MIT OR Apache-2.0

