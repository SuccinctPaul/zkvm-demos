# ZisK zkVM Fibonacci Demo

> ⚠️ **注意**: ZisK 目前**不支持在 macOS 上生成 proof**，仅支持 Linux x86_64。
> 
> 🐳 **macOS 用户**: 请使用 Docker 来生成 proof（见下文）

---

## 🚀 快速开始

### macOS 用户（推荐使用 Docker）

```bash
# 1. 构建 Docker 环境（首次，约 15-20 分钟）
cd docker/scripts && ./build-base.sh && cd ..
docker compose build zisk-zkvm

# 2. 测试运行
docker compose --profile zisk up zisk-zkvm

# 3. 生成 proof
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

📖 **详细 Docker 使用说明**: 
- 快速开始: [`DOCKER_QUICKSTART.md`](DOCKER_QUICKSTART.md)
- 完整指南: [`DOCKER_GUIDE.md`](DOCKER_GUIDE.md)

### Linux 用户（原生支持）

#### 1. 安装依赖

**Ubuntu 22.04+:**
```bash
sudo apt-get install -y xz-utils jq curl build-essential qemu-system libomp-dev \
  libgmp-dev nlohmann-json3-dev protobuf-compiler uuid-dev libgrpc++-dev \
  libsecp256k1-dev libsodium-dev libpqxx-dev nasm libopenmpi-dev openmpi-bin
```

#### 2. 安装 ZisK 工具链

```bash
# 使用 ziskup 安装
curl https://raw.githubusercontent.com/0xPolygonHermez/zisk/main/ziskup/install.sh | bash

# 或使用提供的脚本
cd /path/to/zkvm-demos
./scripts/sdk_installers/install_zisk_sdk.sh
```

#### 3. 验证安装

```bash
cargo-zisk --version
ziskemu --version
rustup toolchain list | grep zisk
```

---

## 📋 使用方法

### 构建

```bash
cd zisk-zkvm/zisk-guest
cargo-zisk build --release
```

### 执行（模拟器）

```bash
# 使用默认输入（Fibonacci(10)）
cargo-zisk run --release -i ../build/input.bin

# 自定义输入
export FIBONACCI_N=20
cargo build --release  # 重新生成 input.bin
cargo-zisk run --release -i ../build/input.bin
```

### 生成 Proof（仅 Linux）

```bash
# 1. ROM setup（首次，约 2-5 分钟）
cargo-zisk rom-setup -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest

# 2. 生成 proof（约 30-45 秒）
cargo-zisk prove -e target/riscv64ima-zisk-zkvm-elf/release/zisk-guest \
                 -i ../build/input.bin -o ../proof -a -y

# 3. 验证 proof（约 2 秒）
cargo-zisk verify -p ../proof/vadcop_final_proof.bin
```

**预期输出:**
```
[INFO ] ProofMan:     ✓ Vadcop Final proof was verified
```

---

## 🐳 Docker 使用（macOS 推荐）

### 快速命令

```bash
cd docker

# 测试模式（构建 + 执行）
docker compose --profile zisk up zisk-zkvm

# Proof 模式（完整流程）
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 自定义输入
FIBONACCI_N=20 ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm

# 交互式 shell
docker compose run --rm zisk-zkvm bash
```

### 在容器内手动执行

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

## 📁 项目结构

```
zisk-zkvm/
├── zisk-guest/              # Guest 程序（在 zkVM 中运行）
│   ├── Cargo.toml
│   ├── build.rs            # 生成 input.bin
│   └── src/
│       └── main.rs         # Fibonacci 计算逻辑
├── build/                  # 由 build.rs 生成
│   └── input.bin          # 输入文件
├── proof/                  # proof 输出目录（生成后）
├── Cargo.toml             # Workspace 配置
├── rust-toolchain.toml    # ZisK 工具链
├── test.sh                # 测试脚本
├── README.md              # 本文件
├── DOCKER_QUICKSTART.md   # Docker 快速开始
└── DOCKER_GUIDE.md        # Docker 详细指南
```

---

## ⚙️ 配置

通过环境变量配置：

```bash
# 设置 Fibonacci 数值（默认: 10）
export FIBONACCI_N=20

# 重新生成 input.bin
cd zisk-guest
cargo build --release

# 运行
cargo-zisk run --release -i ../build/input.bin
```

---

## 📊 性能参考

### Linux x86_64 原生性能

| 操作 | Fib(10) | Fib(20) | Fib(30) |
|-----|---------|---------|---------|
| 构建 | ~1-2s | ~1-2s | ~1-2s |
| 执行 | <0.01s | <0.01s | <0.01s |
| ROM setup | 2-5 分钟（一次） | 同左 | 同左 |
| Proof 生成 | ~30s | ~35s | ~45s |
| Proof 验证 | ~2s | ~2s | ~2s |

### macOS (Docker with emulation)

性能约为原生 Linux 的 40-60%：
- 执行: 正常
- ROM setup: 5-10 分钟（一次）
- Proof 生成: 60-90 秒

---

## 🧪 测试

运行完整测试流程：

```bash
cd zisk-zkvm
bash test.sh
```

**预期输出:**
```
=========================================
ZisK zkVM Fibonacci Demo Test Script
=========================================

Step 1: Building guest program... ✓
Step 2: Running with ziskemu... ✓
Step 3: Testing cargo-zisk run... ✓

All tests passed! ✓
```

---

## 🔧 故障排除

### macOS: "Command is not supported"

这是正常的！ZisK 不支持在 macOS 上生成 proof。

**解决方案**: 使用 Docker
```bash
cd docker
ZKVM_MODE=prove docker compose --profile zisk up zisk-zkvm
```

### Linux: 依赖缺失

```bash
# 安装所有必需的依赖
sudo apt-get install -y xz-utils jq curl build-essential qemu-system \
  libomp-dev libgmp-dev nlohmann-json3-dev protobuf-compiler uuid-dev \
  libgrpc++-dev libsecp256k1-dev libsodium-dev libpqxx-dev nasm \
  libopenmpi-dev openmpi-bin
```

### cargo-zisk 未找到

```bash
# 添加到 PATH
export PATH="$HOME/.zisk/bin:$PATH"

# 永久添加
echo 'export PATH="$HOME/.zisk/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### input.bin 不存在

```bash
# input.bin 由 build.rs 生成
cd zisk-guest
cargo build --release
```

---

## 📚 资源

- [ZisK 官方文档](https://0xpolygonhermez.github.io/zisk/)
- [ZisK GitHub](https://github.com/0xPolygonHermez/zisk)
- [Docker 使用指南](DOCKER_GUIDE.md)
- [快速开始](DOCKER_QUICKSTART.md)

---

## ❓ FAQ

**Q: 为什么 macOS 不支持 proof 生成？**  
A: ZisK 的 proof 生成依赖 Linux 特定的系统功能和 x86_64 架构优化。

**Q: Docker 方案性能如何？**  
A: 在 macOS 上使用 Docker 会有 40-60% 的性能，但完全可用。

**Q: 什么时候会支持 macOS？**  
A: ZisK 团队正在开发 macOS 支持，但尚未公布时间表。

**Q: 有其他支持 macOS 的 zkVM 吗？**  
A: 是的，SP1 和 Risc0 都完全支持 macOS proof 生成。

---

## 📝 License

MIT OR Apache-2.0

---

**版本**: ZisK 0.10.0  
**更新**: 2025-11-16
