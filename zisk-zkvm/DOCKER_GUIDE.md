# ZisK Docker 使用指南

由于 ZisK 不支持在 macOS 上生成 proof，本指南介绍如何使用 Docker 在 macOS 上通过 Linux 容器生成 proof。

---

## 🎯 为什么需要 Docker？

**ZisK 的限制：**
- ✅ macOS: 支持开发和测试（编译、模拟器执行）
- ❌ macOS: **不支持 proof 生成**（ROM setup、prove、verify）
- ✅ Linux x86_64: 完全支持所有功能

**Docker 解决方案：**
- 在 macOS 上运行 Linux x86_64 容器
- 完整支持 proof 生成
- 代码挂载，无需重复构建

---

## 🚀 快速开始

### 1. 首次设置

```bash
# 进入 docker 目录
cd /Users/paul/zkp/zkvms/zkvm-demos/docker

# 构建基础镜像（仅首次需要，约 5-10 分钟）
cd scripts
./build-base.sh
cd ..

# 构建 ZisK 镜像（约 10-15 分钟，包含所有依赖）
docker compose build zisk-zkvm
```

### 2. 测试模式（快速验证）

```bash
# 运行测试：构建 + 模拟器执行
docker compose --profile zisk up zisk-zkvm
```

**预期输出：**
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

To generate proof, set ZKVM_MODE=prove
```

### 3. 生成 Proof

```bash
# 完整流程：构建 + 执行 + ROM setup + 生成 proof + 验证
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

**第一次运行会包含 ROM setup（2-5 分钟），之后会快很多。**

---

## 📋 常用命令

### 基本操作

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/docker

# 测试模式（默认，快速）
docker compose --profile zisk up zisk-zkvm

# Proof 模式（完整流程）
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 自定义 Fibonacci 数值
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 后台运行
docker compose --profile zisk up -d zisk-zkvm

# 查看日志
docker compose logs -f zisk-zkvm

# 停止容器
docker compose down
```

### 交互式 Shell（推荐，完全控制）

```bash
# 进入容器
docker compose run --rm zisk-zkvm bash

# 在容器内手动执行命令
cd zisk-guest

# 构建
cargo-zisk build --release

# 测试
cargo-zisk run --release -i ../build/input.bin

# ROM setup（仅首次）
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest

# 生成 proof
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y

# 验证 proof
cargo-zisk verify -p ../proof/vadcop_final_proof.bin

# 退出容器
exit
```

---

## 🔧 环境变量

| 变量 | 默认值 | 说明 |
|-----|-------|------|
| `ZKVM_MODE` | `test` | `test`: 仅测试，`prove`: 生成 proof |
| `FIBONACCI_N` | `10` | Fibonacci 数列的第 N 项 |
| `RUST_LOG` | `info` | 日志级别 |

**使用示例：**
```bash
# 计算 Fibonacci(30) 并生成 proof
FIBONACCI_N=30 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 📊 性能预期

### macOS Apple Silicon (M1/M2/M3)

由于使用 x86_64 仿真，性能会有所下降：

| 操作 | 原生 Linux | Docker on macOS (仿真) |
|-----|-----------|----------------------|
| 构建 | ~1-2s | ~2-3s |
| 执行（模拟器）| <0.01s | <0.1s |
| ROM setup | 2-5 分钟 | 5-10 分钟 |
| Proof 生成 | 30-45s | 60-90s |
| Proof 验证 | ~2s | ~4s |

**优化建议：**
- ROM setup 只需一次，结果会缓存
- 后续 proof 生成不包含 ROM setup，速度更快
- 使用交互式 shell 可以避免重复启动容器

---

## 💾 数据持久化

Docker 使用卷来缓存数据，避免重复下载和构建：

```bash
# 查看 ZisK 相关的卷
docker volume ls --filter "name=zisk"

# 输出：
# zisk-cargo-cache    - Rust 依赖缓存
# zisk-target-cache   - 编译产物缓存
# zisk-zisk-cache     - ZisK 工具链和 ROM setup 缓存
```

**清理缓存（如果需要）：**
```bash
# 清理所有 ZisK 缓存
docker volume rm docker_zisk-cargo-cache docker_zisk-target-cache docker_zisk-zisk-cache

# 或清理所有未使用的卷
docker volume prune
```

---

## 🔍 故障排除

### 问题 1: "zkvm-base:latest not found"

**原因：** 未构建基础镜像

**解决：**
```bash
cd docker/scripts
./build-base.sh
```

### 问题 2: ROM setup 很慢

**原因：** macOS 上使用 x86_64 仿真，正常现象

**解决：**
- 第一次会慢（5-10 分钟），但结果会缓存
- 后续运行不需要 ROM setup
- 如果清理了 `zisk-zisk-cache` 卷，需要重新 setup

### 问题 3: Proof 生成失败

**检查：**
```bash
# 进入容器查看详细错误
docker compose run --rm zisk-zkvm bash
cd zisk-guest
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y
```

**常见原因：**
- ROM setup 未完成
- 输入文件不存在（需要先运行 `cargo build` 生成 input.bin）
- 磁盘空间不足

### 问题 4: 磁盘空间不足

**检查：**
```bash
docker system df
```

**清理：**
```bash
# 清理未使用的镜像和容器
docker system prune

# 清理所有（包括缓存卷）
docker system prune -a --volumes
```

### 问题 5: 代码修改后没有生效

**原因：** 代码是挂载的，不需要重新构建镜像

**正确做法：**
```bash
# 修改代码后，直接运行即可
vim zisk-zkvm/zisk-guest/src/main.rs
docker compose --profile zisk up zisk-zkvm  # 不需要 --build
```

**何时需要重建：**
- 只在修改了 `Cargo.toml` 或 `Dockerfile` 后需要重建
```bash
docker compose build zisk-zkvm
```

---

## 📝 完整工作流程示例

### 场景 1: 开发和测试（macOS 本地）

```bash
# 1. 本地修改代码
vim zisk-zkvm/zisk-guest/src/main.rs

# 2. 本地测试（快速）
cd zisk-zkvm/zisk-guest
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin

# 3. 确认无误后，提交代码
git add .
git commit -m "Update fibonacci implementation"
```

### 场景 2: 生成 Proof（Docker）

```bash
# 1. 使用 Docker 生成 proof
cd docker
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 2. 查看生成的 proof
ls -lh ../zisk-zkvm/proof/

# 3. proof 文件会保存在 host 的 zisk-zkvm/proof/ 目录
```

### 场景 3: 批量生成不同输入的 Proof

```bash
cd docker

# Fibonacci(10)
FIBONACCI_N=10 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Fibonacci(20)  
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# Fibonacci(30)
FIBONACCI_N=30 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

---

## 🎓 高级用法

### 使用交互式 Shell 进行调试

```bash
# 进入容器
docker compose run --rm zisk-zkvm bash

# 检查工具链
cargo-zisk --version
ziskemu --version
rustup toolchain list | grep zisk

# 查看文件结构
ls -la
cd zisk-guest
ls -la target/riscv64ima-zisk-zkvm-elf/release/

# 查看 ROM setup 状态
ls -la ~/.zisk/

# 运行单个步骤
cargo-zisk build --release
cargo-zisk run --release -i ../build/input.bin

# 退出
exit
```

### 挂载额外的目录

修改 `docker-compose.yml`：
```yaml
volumes:
  - ../:/workspace
  - /path/to/custom/data:/data  # 添加自定义挂载
```

### 使用 GPU 加速（需要 NVIDIA GPU + Linux）

修改 `Dockerfile.zisk`，移除 `CI=true`：
```dockerfile
# 原来：
CI=true /tmp/install_zisk_sdk.sh

# 改为：
/tmp/install_zisk_sdk.sh
```

然后在 `docker-compose.yml` 中添加 GPU 支持：
```yaml
zisk-zkvm:
  deploy:
    resources:
      reservations:
        devices:
          - driver: nvidia
            count: all
            capabilities: [gpu]
```

---

## 📚 参考链接

- [ZisK 官方文档](https://0xpolygonhermez.github.io/zisk/)
- [ZisK GitHub](https://github.com/0xPolygonHermez/zisk)
- [Docker 官方文档](https://docs.docker.com/)

---

## ✅ 总结

### 优势
- ✅ 在 macOS 上可以生成 proof
- ✅ 完整的 Linux 环境
- ✅ 缓存机制，避免重复构建
- ✅ 代码挂载，修改即时生效

### 注意事项
- ⚠️ 首次构建需要 10-15 分钟
- ⚠️ macOS 上使用仿真，性能下降 2-3 倍
- ⚠️ ROM setup 是一次性操作，需要 5-10 分钟
- ⚠️ 需要足够的磁盘空间（推荐 10GB+）

### 推荐做法
1. 在 macOS 上开发和测试（使用本地 ziskemu）
2. 使用 Docker 生成 proof
3. 如果需要频繁生成 proof，考虑使用 Linux 机器或 VM

---

**创建时间**: 2025-11-16  
**适用版本**: ZisK 0.10.0  
**平台**: macOS (Docker) / Linux (原生)

