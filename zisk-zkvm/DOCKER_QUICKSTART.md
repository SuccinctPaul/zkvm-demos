# ZisK Docker 快速开始

在 macOS 上使用 Docker 生成 ZisK proof 的快速指南。

---

## ⚡ 一键开始

```bash
# 1. 构建基础镜像（首次，5-10 分钟）
cd docker/scripts && ./build-base.sh && cd ..

# 2. 构建 ZisK 镜像（首次，10-15 分钟）
docker compose build zisk-zkvm

# 3. 测试运行（1-2 分钟）
docker compose --profile zisk up zisk-zkvm

# 4. 生成 proof（首次 5-10 分钟含 ROM setup，后续 1-2 分钟）
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 🎯 常用命令

```bash
cd docker

# 测试（快速验证）
docker compose --profile zisk up zisk-zkvm

# 生成 proof
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 自定义输入
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 交互式 shell
docker compose run --rm zisk-zkvm bash
```

---

## 📋 在容器内的命令

```bash
# 进入容器
docker compose run --rm zisk-zkvm bash

# 执行命令
cd zisk-guest
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y
cargo-zisk verify -p ../proof/vadcop_final_proof.bin
```

---

## ✅ 预期输出

### 测试模式
```
ZisK ZKVM - Fibonacci Demo
Mode: test
FIBONACCI_N: 10

=== Building guest program ===
Compiling zisk-guest v0.1.0
Finished `release` profile [optimized] target(s) in 1.2s

=== Testing with emulator ===
Computing Fibonacci for n = 10
Fibonacci(10) = 89
00000059
```

### Proof 模式
```
=== Generating proof ===
[INFO] Generating proof...
[INFO] Proof generated successfully

=== Verifying proof ===
[INFO] ProofMan:     ✓ Vadcop Final proof was verified
```

---

## ⏱️ 时间预期（macOS Apple Silicon）

| 操作 | 首次 | 后续 |
|-----|------|------|
| 基础镜像构建 | 5-10 分钟 | - |
| ZisK 镜像构建 | 10-15 分钟 | - |
| 测试运行 | 1-2 分钟 | 30-60 秒 |
| ROM setup | 5-10 分钟 | （缓存，跳过）|
| Proof 生成 | 1-2 分钟 | 1-2 分钟 |

---

## 🔍 常见问题

**Q: 为什么需要 Docker？**  
A: ZisK 不支持在 macOS 上生成 proof，需要 Linux x86_64 环境。

**Q: 为什么第一次这么慢？**  
A: 需要下载镜像、安装依赖、ROM setup。这些都会缓存，后续会快很多。

**Q: 修改代码后需要重建镜像吗？**  
A: 不需要！代码是挂载的，直接运行即可。

**Q: proof 文件在哪里？**  
A: 在 `zisk-zkvm/proof/` 目录，会自动同步到 host。

---

## 📚 详细文档

查看 `DOCKER_GUIDE.md` 获取完整的使用说明和故障排除。

---

**快速开始完成！** 🎉

