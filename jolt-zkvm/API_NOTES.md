# Jolt zkVM API 使用说明

## ⚠️ 重要提示

Jolt zkVM 目前处于活跃开发阶段，API 可能会发生变化。本文档记录了当前使用的 API 和可能需要调整的地方。

## 当前实现的 API

### Guest 程序 API

```rust
#![cfg_attr(feature = "guest", no_std)]
#![cfg_attr(feature = "guest", no_main)]

#[jolt::provable]
fn fibonacci(n: u32) -> u32 {
    fib::fibonacci(n)
}
```

**关键点:**
- ✅ 使用 `#[jolt::provable]` 宏标记可证明函数
- ✅ 函数需要是纯函数，没有副作用
- ✅ 支持基本类型（u32, u64, etc.）
- ⚠️  函数可见性（pub 或私有）可能影响生成的 API

### Host 程序 API

```rust
// build_fibonacci 函数由 Jolt 宏系统自动生成
let (prove_fibonacci, verify_fibonacci) = jolt_guest::build_fibonacci();

// 证明生成
let (output, proof) = prove_fibonacci(input);

// 证明验证
let is_valid = verify_fibonacci(proof);
```

**关键点:**
- ✅ `build_fibonacci()` 由 Jolt 根据 guest 中的函数名自动生成
- ✅ 返回两个闭包：`prove_fn` 和 `verify_fn`
- ✅ `prove_fn` 返回 `(output, proof)` 元组
- ✅ `verify_fn` 返回布尔值

### Build Script API

```rust
fn main() {
    jolt_sdk::build_guest("../jolt-guest");
}
```

**关键点:**
- ✅ 在 build.rs 中调用
- ✅ 路径相对于 build.rs 文件
- ✅ 编译 guest 程序为 zkVM 格式

## 可能需要验证的地方

### 1. 函数可见性

```rust
// 选项 A: 私有函数
#[jolt::provable]
fn fibonacci(n: u32) -> u32 { ... }

// 选项 B: 公开函数
#[jolt::provable]
pub fn fibonacci(n: u32) -> u32 { ... }
```

**当前选择:** 私有函数 (fn)

**需要验证:**
- [ ] 检查 Jolt 文档中推荐的可见性
- [ ] 测试两种方式是否都能工作
- [ ] 确认生成的 API 名称

### 2. 输入输出类型

```rust
// 当前支持
#[jolt::provable]
fn fibonacci(n: u32) -> u32

// 可能需要验证
#[jolt::provable]
fn complex_fn(a: u32, b: u64) -> (u32, u64)
```

**需要验证:**
- [ ] 支持的参数类型
- [ ] 支持的返回类型
- [ ] 多参数函数的处理
- [ ] 元组返回值的处理

### 3. Build Function 命名

**当前假设:**
- Guest 函数 `fibonacci` → Host API `build_fibonacci()`

**需要验证:**
- [ ] 命名转换规则
- [ ] 驼峰命名 vs 蛇形命名
- [ ] 是否有命名冲突的处理

### 4. 错误处理

```rust
// 当前实现
let (output, proof) = prove_fibonacci(fib_n);
let is_valid = verify_fibonacci(proof);

// 可能需要
let (output, proof) = prove_fibonacci(fib_n)?;
let is_valid = verify_fibonacci(proof)?;
```

**需要验证:**
- [ ] API 是否返回 Result<T, E>
- [ ] 错误类型
- [ ] panic vs Result

## 验证清单

在首次运行前，请验证：

### ✅ 已确认的部分
- [x] Jolt SDK 安装方法
- [x] Rust toolchain 版本
- [x] 基本项目结构
- [x] Cargo.toml 配置

### ⚠️ 需要验证的部分
- [ ] `#[jolt::provable]` 宏的确切用法
- [ ] `build_fibonacci()` API 是否正确
- [ ] 证明和验证函数的签名
- [ ] 错误处理机制

## 如何验证

### 方法 1: 查看 Jolt 文档

```bash
# 访问 Jolt 文档
open https://jolt.a16zcrypto.com/

# 查看 GitHub 仓库示例
open https://github.com/a16z/jolt/tree/main/examples
```

### 方法 2: 检查 Jolt CLI

```bash
# 查看 Jolt 帮助
jolt --help

# 查看 Jolt 模板
jolt init --help
```

### 方法 3: 运行测试

```bash
# 尝试构建
cd jolt-zkvm/jolt-host
cargo build --release

# 查看编译错误
# 编译错误会提示正确的 API 用法
```

## 可能的 API 变体

### 变体 1: 使用 Result

```rust
let result = prove_fibonacci(fib_n);
match result {
    Ok((output, proof)) => { ... }
    Err(e) => { ... }
}
```

### 变体 2: 分离的 API

```rust
let prover = jolt_sdk::Prover::new();
let output = prover.execute(fibonacci, fib_n);
let proof = prover.prove(fibonacci, fib_n);
let is_valid = prover.verify(proof);
```

### 变体 3: Builder 模式

```rust
let prover = jolt_sdk::ProverBuilder::new()
    .guest_path("../jolt-guest")
    .build()?;
    
let proof = prover.prove(fibonacci, fib_n)?;
```

## 调试建议

### 1. 启用详细日志

```bash
RUST_LOG=debug cargo build
RUST_LOG=trace cargo run --release
```

### 2. 检查宏展开

```bash
cargo expand --package jolt-guest
cargo expand --package jolt-host
```

### 3. 查看生成的代码

```bash
# 查看 build.rs 生成的文件
ls -la target/debug/build/jolt-host-*/out/
cat target/debug/build/jolt-host-*/out/*.rs
```

## 更新建议

如果 API 发生变化，请更新：

1. **jolt-guest/src/lib.rs** - Guest 函数定义
2. **jolt-host/src/main.rs** - Host API 调用
3. **jolt-host/build.rs** - 构建脚本
4. **本文档** - API 说明
5. **README.md** - 使用文档

## 参考资源

- 📖 **Jolt 文档**: https://jolt.a16zcrypto.com/
- 💻 **Jolt GitHub**: https://github.com/a16z/jolt
- 📄 **Jolt 论文**: https://eprint.iacr.org/2023/1217
- 💬 **Jolt Discord**: 查看社区讨论

## 已知问题

### Issue 1: 编译时间长
**问题:** 首次编译可能需要很长时间  
**解决:** 正常现象，后续会有缓存

### Issue 2: nightly Rust 版本
**问题:** 需要特定的 nightly 版本  
**解决:** 使用 rust-toolchain.toml 固定版本

### Issue 3: 依赖下载
**问题:** 从 GitHub 下载依赖可能失败  
**解决:** 使用代理或等待重试

## 版本历史

- **v0.1.0** (2025-11-13)
  - 初始实现
  - 基于 Jolt rev 55b9830
  - API 可能需要验证

## 下一步

1. **立即:** 尝试编译和运行
2. **观察:** 查看编译错误和警告
3. **调整:** 根据实际 API 修改代码
4. **文档:** 更新本文档和其他文档
5. **测试:** 验证所有功能正常工作

---

**最后更新:** 2025-11-13  
**状态:** ⚠️ 需要实际验证  
**优先级:** 🔴 高

