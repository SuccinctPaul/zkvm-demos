# Jolt zkVM 快速开始指南

## 🚀 5 分钟快速上手

### 步骤 1: 安装 Jolt

```bash
# 在项目根目录执行
cd scripts/sdk_installers
./install_jolt_sdk.sh
```

### 步骤 2: 配置环境

```bash
cd ../../jolt-zkvm
cp .env.example .env
```

### 步骤 3: 运行 Demo

```bash
./run_demo.sh
```

就这么简单！

## 📝 详细说明

### 安装验证

检查 Jolt 是否正确安装：

```bash
jolt --version
```

### 自定义 Fibonacci 数值

编辑 `.env` 文件：

```bash
FIBONACCI_N=15  # 修改为你想要的数值
```

### 查看详细日志

```bash
RUST_LOG=debug ./run_demo.sh
```

## 🎯 预期输出

```
========================================
Jolt zkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(10)...

1️⃣  Building guest program...
   ✓ Build completed in 5.23s

2️⃣  Generating proof...
   ✓ Proof generated in 2.15s
   ✓ Result: fibonacci(10) = 89

3️⃣  Verifying proof...
   ✓ Proof verified successfully in 0.34s

========================================
📈 Performance Summary
========================================
Build time:   5.23s
Prove time:   2.15s
Verify time:  0.34s
Total time:   7.72s
========================================
✅ Jolt zkVM Demo completed successfully!
```

## 🔧 故障排除

### 问题: "jolt: command not found"

**解决方案:**
```bash
# 重新安装 Jolt
cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
jolt install-toolchain

# 确保 ~/.cargo/bin 在 PATH 中
export PATH="$HOME/.cargo/bin:$PATH"
```

### 问题: "FIBONACCI_N not set"

**解决方案:**
```bash
# 方式 1: 创建 .env 文件
echo "FIBONACCI_N=10" > .env

# 方式 2: 直接设置环境变量
export FIBONACCI_N=10
```

### 问题: 构建失败

**解决方案:**
```bash
# 清理并重新构建
cargo clean
cargo build --release
```

## 📚 下一步

- 📖 阅读 [README.md](./README.md) 了解更多细节
- 🔍 查看 [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) 了解实现细节
- 🎓 访问 [Jolt 文档](https://jolt.a16zcrypto.com/) 学习更多

## 💡 提示

- 首次构建会比较慢（5-10秒），后续会更快
- 使用较小的 FIBONACCI_N 值（如 5-15）以获得更快的证明时间
- 启用 RUST_LOG=debug 可以看到更详细的执行信息

## 🤝 需要帮助？

- 查看 [Jolt GitHub Issues](https://github.com/a16z/jolt/issues)
- 阅读 [zkvm-demos 主 README](../README.md)
- 查看其他 zkVM 示例对比

---

**Happy Proving! 🎉**

