# mise 工具链管理指南

本项目使用 [mise](https://mise.jdx.dev/)（前身为 rtx）管理不同 zkVM 实现所需的多个工具链。

## 为什么使用 mise？

本仓库包含 20+ 个 zkVM 实现，每个都有不同的依赖需求：
- **多个 Rust 版本**：Jolt (1.88 稳定版) vs SP1/Risc0 (nightly)
- **非 Rust 依赖**：Cairo (Python/Scarb)、Valida (LLVM 18+)
- **zkVM 专用工具**：每个 zkVM 有不同的 CLI 工具
- **自动切换**：切换目录 → 工具自动切换

mise 解决了主 README 中提到的工具链冲突问题，同时提供原生性能（不像 Docker 有性能损失）。

## 快速开始

### 1. 安装 mise

```bash
# macOS
brew install mise

# Linux
curl https://mise.run | sh

# 或使用 cargo
cargo install mise
```

添加到 shell 配置文件（`~/.zshrc` 或 `~/.bashrc`）：

```bash
# mise 激活
eval "$(mise activate zsh)"  # 或 bash/fish
```

重启终端或运行 `source ~/.zshrc`。

### 2. 信任并激活

进入项目根目录：

```bash
cd zkvm-demos

# 信任 mise 配置
mise trust

# 安装 .mise.toml 中定义的所有工具
mise install

# 验证安装
mise doctor
```

### 3. 检查已安装的工具

```bash
# 查看可用的工具
mise list

# 运行检查工具任务
mise run check-tools

# 查看当前环境
mise current
```

## 使用方法

### 自动工具链切换

当你 `cd` 到不同目录时，mise 会自动切换工具链：

```bash
# 根目录使用 nightly-2025-06-05
cd zkvm-demos
rustc --version  # 1.86.0-nightly (2025-06-05)

# Jolt 使用稳定版 1.88
cd jolt-zkvm
rustc --version  # 1.88.0

# Cairo 环境有 Python + Scarb
cd ../cairo-zkvm
python --version  # 3.11.x
```

### 运行任务

mise 为每个 zkVM 提供了便捷的任务：

```bash
# 根级别任务
mise run check-tools        # 检查已安装的工具
mise run install-rust-tools # 安装 Rust targets
mise run list-zkvms         # 列出所有 zkVM 项目

# Jolt 专用任务（在 jolt-zkvm/ 目录下）
cd jolt-zkvm
mise run install-jolt       # 安装 Jolt 工具链
mise run run                # 运行 Jolt 演示
mise run build-guest        # 构建 guest 程序

# Cairo 专用任务（在 cairo-zkvm/ 目录下）
cd cairo-zkvm
mise run install-scarb      # 安装 Scarb
mise run build              # 构建 Cairo 项目
mise run test               # 运行测试
mise run run                # 运行 Cairo 程序

# Risc0 专用任务（在 risc0-zkvm/ 目录下）
cd risc0-zkvm
mise run install-risc0      # 安装 Risc0
mise run run-dev            # 开发模式运行
mise run run-prod           # 生产模式运行

# SP1 专用任务（在 sp1-zkvm/ 目录下）
cd sp1-zkvm
mise run install-sp1        # 安装 SP1
mise run run-execute        # 仅执行
mise run run-prove          # 生成证明

# Valida 专用任务（在 valida-zkvm/ 目录下）
cd valida-zkvm
mise run check-llvm         # 检查 LLVM 版本
mise run install-valida     # 安装 Valida
mise run compile-c          # 编译 C 到 ELF
mise run prove              # 生成证明
```

### 本地覆盖

为你的机器创建本地配置：

```bash
# 复制示例
cp .mise.local.toml.example .mise.local.toml

# 编辑你的偏好设置
vim .mise.local.toml
```

示例 `.mise.local.toml`：

```toml
[tools]
rust = "nightly-2025-06-15"  # 使用更新的 nightly

[env]
RUST_LOG = "debug"
RISC0_DEV_MODE = "1"
LLVM_SYS_180_PREFIX = "/usr/local/opt/llvm@18"  # 你的 LLVM 路径
```

## 项目结构

```
zkvm-demos/
├── .mise.toml                    # 根配置（默认工具）
├── .mise.local.toml              # 你的本地覆盖（已忽略）
├── jolt-zkvm/
│   └── .mise.toml                # Jolt 专用（Rust 1.88）
├── cairo-zkvm/
│   └── .mise.toml                # Cairo 专用（Python + Scarb）
├── risc0-zkvm/
│   └── .mise.toml                # Risc0 专用（nightly）
├── sp1-zkvm/
│   └── .mise.toml                # SP1 专用（nightly）
└── valida-zkvm/
    └── .mise.toml                # Valida 专用（Rust + LLVM）
```

## 与现有工具集成

### 与 rust-toolchain.toml

mise 与 `rust-toolchain.toml` 文件协同工作：
- mise 提供 Rust 版本
- `rust-toolchain.toml` 提供 targets 和组件
- cargo 会同时尊重两者

### 与 Docker

mise 与 Docker 互补：
- **开发**：使用 mise 获得原生性能和 IDE 集成
- **CI/CD**：使用 Docker 获得可重现的构建
- **团队**：一些成员使用 mise，其他人使用 Docker

### 与 direnv

如果你已经在使用 direnv，mise 可以无缝集成：

```bash
# .envrc
eval "$(mise activate bash --shims)"
```

## 故障排除

### 工具未自动切换

```bash
# 确保 mise 已在 shell 中激活
echo $MISE_SHELL  # 应该显示你的 shell

# 重新加载 shell 配置
source ~/.zshrc

# 检查 mise hook 是否已安装
mise doctor
```

### Rust 版本不匹配

```bash
# 清除 mise 缓存
mise cache clear

# 重新安装 Rust 版本
mise install rust@1.88
mise install rust@nightly-2025-06-05

# 检查当前激活的版本
mise current rust
```

### 找不到 LLVM（Valida）

mise 不直接管理 LLVM。需要手动安装：

```bash
# macOS
brew install llvm@18
echo 'export LLVM_SYS_180_PREFIX="/opt/homebrew/opt/llvm@18"' >> ~/.zshrc

# Ubuntu
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
```

然后添加到你的 `.mise.local.toml`：

```toml
[env]
LLVM_SYS_180_PREFIX = "/path/to/llvm-18"
```

### Python 版本冲突（Cairo）

```bash
# 检查 Python 版本
mise current python

# 安装特定版本
mise install python@3.11

# 全局或本地使用
mise use -g python@3.11
```

### 工具安装失败

```bash
# 尝试详细安装
mise install rust@1.88 --verbose

# 检查日志
mise doctor

# 回退到手动安装
rustup install 1.88
```

## 高级用法

### 按目录固定版本

```bash
# 在任何 zkVM 目录
cd sp1-zkvm

# 固定到特定版本
mise use rust@nightly-2025-06-05
mise use python@3.11

# 这会创建/更新当前目录的 .mise.toml
```

### 列出所有可用版本

```bash
# 查看所有可用的 Rust 版本
mise ls-remote rust

# 查看所有 Python 版本
mise ls-remote python

# 查看所有已安装版本
mise list
```

### 升级工具

```bash
# 升级所有工具到最新版本
mise upgrade

# 升级特定工具
mise upgrade rust

# 安装最新版本
mise install rust@latest
```

### 环境变量

mise 可以管理环境变量：

```bash
# 为当前目录设置
mise set RUST_LOG=debug
mise set FIBONACCI_N=20

# 检查环境
mise env

# 导出到 shell
eval "$(mise env)"
```

## 与其他方案比较

| 功能 | mise | Docker | direnv | asdf |
|------|------|--------|--------|------|
| **设置时间** | 5分钟 | 10分钟 | 2分钟 | 5分钟 |
| **性能** | 原生 | -10%开销 | 原生 | 原生 |
| **自动切换** | ✅ 是 | ❌ 手动 | ✅ 是 | ✅ 是 |
| **多语言** | ✅ 是 | ✅ 是 | ⚠️ 手动 | ✅ 是 |
| **学习曲线** | 简单 | 中等 | 简单 | 简单 |
| **IDE 集成** | ✅ 无缝 | ⚠️ 需设置 | ✅ 好 | ✅ 好 |
| **CI/CD** | ✅ 是 | ✅ 最好 | ❌ 否 | ✅ 是 |
| **隔离性** | 好 | 优秀 | 无 | 好 |
| **速度** | 快 | 较慢 | 快 | 中等 |

**推荐：**
- **独立开发，频繁构建**：mise（本指南）
- **团队协作，CI/CD**：Docker（见 [ISOLATION-DOCKER.md](docs/ISOLATION-DOCKER.md)）
- **仅环境变量**：direnv
- **熟悉 asdf**：mise（相同语法，更快）

## 资源

- **mise 文档**：https://mise.jdx.dev/
- **GitHub**：https://github.com/jdx/mise
- **Discord**：https://discord.gg/mise
- **与 asdf 比较**：https://mise.jdx.dev/comparison-to-asdf.html

## 获取帮助

1. **查看 mise 文档**：`mise help` 或 https://mise.jdx.dev/
2. **运行诊断**：`mise doctor`
3. **查看 zkVM 专用 README**：每个 zkVM 目录都有详细说明
4. **回退到 Docker**：见根目录 [README.md](../README.md) 的 Docker 设置

## 贡献

添加新的 zkVM 时：

1. 在 zkVM 目录创建 `.mise.toml`
2. 指定所需工具（Rust 版本等）
3. 添加便捷任务（install、build、run）
4. 更新本指南，说明新的依赖
5. 测试自动切换：`cd new-zkvm && mise current`

示例模板：

```toml
# new-zkvm/.mise.toml
[tools]
rust = "1.85"

[env]
RUST_LOG = "info"

[tasks.install]
description = "安装此 zkVM 的工具链"
run = "../scripts/sdk_installers/install_new_zkvm.sh"

[tasks.run]
description = "运行演示"
run = "cargo run --release"
```

---

**使用 mise 愉快地进行 ZK 证明！🚀**

Docker 隔离方案请参见 [ISOLATION-DOCKER.md](docs/ISOLATION-DOCKER.md)。

