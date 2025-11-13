# Jolt zkVM 验证清单

## 🎯 目的

这个清单用于验证 Jolt zkVM demo 的实现是否正确，以及是否能够成功生成和验证证明。

## 📋 验证步骤

### 阶段 1: 环境准备 ✅

- [x] Rust 工具链已安装
- [ ] Jolt CLI 已安装
  ```bash
  jolt --version
  ```
- [ ] 环境变量已配置
  ```bash
  echo $FIBONACCI_N
  ```

### 阶段 2: 依赖检查 ⚠️

- [ ] 检查 Cargo.toml 依赖
  ```bash
  cd jolt-zkvm/jolt-host
  cargo tree
  ```
- [ ] 验证 fib crate 可用
  ```bash
  cargo build --package fib
  ```
- [ ] 验证 common crate 可用
  ```bash
  cargo build --package common
  ```

### 阶段 3: Guest 程序验证 ⚠️

- [ ] Guest 程序可以编译
  ```bash
  cd jolt-zkvm/jolt-guest
  cargo check
  ```
- [ ] 宏展开正常
  ```bash
  cargo expand
  ```
- [ ] 没有编译警告
  ```bash
  cargo build --release 2>&1 | grep warning
  ```

### 阶段 4: Build Script 验证 ⚠️

- [ ] build.rs 可以执行
  ```bash
  cd jolt-zkvm/jolt-host
  cargo clean
  cargo build --release -vv
  ```
- [ ] Guest 程序被正确编译
  ```bash
  # 检查 build 输出
  ls -la target/release/build/jolt-host-*/out/
  ```
- [ ] 生成了必要的文件
  ```bash
  find target -name "*.elf" -o -name "fibonacci*"
  ```

### 阶段 5: Host 程序验证 ⚠️

- [ ] Host 程序可以编译
  ```bash
  cd jolt-zkvm/jolt-host
  cargo build --release
  ```
- [ ] API 调用正确
  ```bash
  # 编译成功即表示 API 正确
  cargo check
  ```
- [ ] 没有链接错误
  ```bash
  cargo build --release 2>&1 | grep error
  ```

### 阶段 6: 运行测试 🔴

- [ ] 可以运行 demo
  ```bash
  cd jolt-zkvm
  FIBONACCI_N=5 cargo run --release --package jolt-host
  ```
- [ ] 证明生成成功
  ```bash
  # 查看输出中的 "Proof generated" 消息
  ```
- [ ] 证明验证成功
  ```bash
  # 查看输出中的 "Proof verified successfully" 消息
  ```
- [ ] 输出结果正确
  ```bash
  # fibonacci(5) = 8
  # fibonacci(10) = 89
  # fibonacci(15) = 987
  ```

### 阶段 7: 不同输入测试 🔴

- [ ] 测试 n=5
  ```bash
  FIBONACCI_N=5 ./run_demo.sh
  # 期望: 8
  ```
- [ ] 测试 n=10
  ```bash
  FIBONACCI_N=10 ./run_demo.sh
  # 期望: 89
  ```
- [ ] 测试 n=15
  ```bash
  FIBONACCI_N=15 ./run_demo.sh
  # 期望: 987
  ```

### 阶段 8: 边界情况测试 🔴

- [ ] 测试 n=0
  ```bash
  FIBONACCI_N=0 ./run_demo.sh
  # 期望: 1 (根据 fib crate 的实现)
  ```
- [ ] 测试 n=1
  ```bash
  FIBONACCI_N=1 ./run_demo.sh
  # 期望: 1
  ```
- [ ] 测试较大的值 n=20
  ```bash
  FIBONACCI_N=20 ./run_demo.sh
  # 观察性能
  ```

## 🔍 常见问题排查

### 问题 1: jolt_guest::build_fibonacci 不存在

**原因:** Jolt 宏没有生成正确的 API

**排查:**
```bash
# 检查宏展开
cd jolt-zkvm/jolt-guest
cargo expand

# 查看生成的函数名
cargo doc --open
```

**可能的解决方案:**
1. 函数需要是 pub
2. 宏名称不对
3. Jolt 版本不匹配

### 问题 2: 编译失败

**原因:** 依赖问题或 API 不匹配

**排查:**
```bash
# 查看详细错误
cargo build --release -vv

# 检查依赖
cargo tree | grep jolt
```

**可能的解决方案:**
1. 更新 Cargo.lock
2. 修改 API 调用
3. 检查 Jolt 文档

### 问题 3: 运行时错误

**原因:** Guest 程序或 Host 程序逻辑错误

**排查:**
```bash
# 启用调试日志
RUST_LOG=debug cargo run --release

# 检查环境变量
env | grep FIBONACCI
```

**可能的解决方案:**
1. 检查输入验证
2. 添加错误处理
3. 查看 Jolt 日志

## 📝 验证报告模板

完成验证后，填写以下报告：

```markdown
# Jolt zkVM 验证报告

## 基本信息
- 验证日期: [填写日期]
- 验证者: [填写姓名]
- Jolt 版本: [git rev]
- Rust 版本: [rustc --version]

## 验证结果

### 编译阶段
- [ ] Guest 编译: 通过/失败
- [ ] Host 编译: 通过/失败
- [ ] Build script: 通过/失败

### 运行阶段
- [ ] 证明生成: 通过/失败
- [ ] 证明验证: 通过/失败
- [ ] 结果正确: 通过/失败

### 测试用例
| 输入 | 期望输出 | 实际输出 | 状态 |
|------|----------|----------|------|
| 5    | 8        | [填写]   | ✅/❌ |
| 10   | 89       | [填写]   | ✅/❌ |
| 15   | 987      | [填写]   | ✅/❌ |

### 性能数据
- 构建时间: [填写]秒
- 证明时间: [填写]秒
- 验证时间: [填写]秒

## 发现的问题

1. [列出问题]
2. [列出问题]

## 需要修改的地方

1. [列出修改]
2. [列出修改]

## 总体评估

- [ ] 完全正常
- [ ] 需要小调整
- [ ] 需要大改动

## 备注

[其他说明]
```

## 🔄 如果验证失败

### 步骤 1: 收集信息

```bash
# 保存完整的编译输出
cargo build --release 2>&1 | tee build.log

# 保存运行时错误
RUST_LOG=debug cargo run --release 2>&1 | tee run.log

# 保存环境信息
rustc --version > env-info.txt
cargo --version >> env-info.txt
jolt --version >> env-info.txt 2>&1
```

### 步骤 2: 查阅文档

- 📖 查看 Jolt 官方文档
- 💻 查看 Jolt GitHub issues
- 📄 查看 API_NOTES.md

### 步骤 3: 尝试修复

1. 更新依赖版本
2. 修改 API 调用
3. 调整配置文件

### 步骤 4: 寻求帮助

- GitHub Issues
- Jolt Discord/Telegram
- 社区论坛

## ✅ 验证通过标准

Demo 被认为验证通过，需要满足：

1. ✅ 所有编译成功
2. ✅ 证明可以生成
3. ✅ 证明可以验证
4. ✅ 结果正确
5. ✅ 至少 3 个不同输入测试通过
6. ✅ 没有运行时错误
7. ✅ 性能在合理范围内

## 📊 验证进度

- [ ] 阶段 1: 环境准备
- [ ] 阶段 2: 依赖检查
- [ ] 阶段 3: Guest 验证
- [ ] 阶段 4: Build Script 验证
- [ ] 阶段 5: Host 验证
- [ ] 阶段 6: 运行测试
- [ ] 阶段 7: 不同输入测试
- [ ] 阶段 8: 边界测试

**总体进度:** 0/8

---

**创建日期:** 2025-11-13  
**状态:** ⚠️ 待验证  
**优先级:** 🔴 高

