# mise 工具链管理 - 使用指南

## 📦 已完成的工作

我们已经为 zkvm-demos 项目成功实现了 **mise 工具链管理系统**，解决了多个 zkVM 之间的工具链冲突问题。

### ✅ 创建的文件

#### 配置文件（20个）
- ✅ `.mise.toml` - 根目录配置（默认工具链）
- ✅ `.mise.local.toml.example` - 本地覆盖配置模板
- ✅ 19 个 zkVM 专用配置文件：
  - `jolt-zkvm/.mise.toml`
  - `cairo-zkvm/.mise.toml`
  - `cairo-m-zkvm/.mise.toml`
  - `risc0-zkvm/.mise.toml`
  - `sp1-zkvm/.mise.toml`
  - `valida-zkvm/.mise.toml`
  - 以及其他 13 个...

#### 文档（4个）
- ✅ `MISE_SETUP.md` - 完整设置指南（英文）
- ✅ `MISE_SETUP.zh-CN.md` - 完整设置指南（中文）
- ✅ `MISE_QUICK_START.md` - 5分钟快速入门
- ✅ `MISE_IMPLEMENTATION_SUMMARY.md` - 实现总结

#### 脚本（2个）
- ✅ `scripts/install_mise.sh` - 自动安装 mise
- ✅ `scripts/verify_mise_setup.sh` - 验证配置

#### 更新（2个）
- ✅ `README.md` - 添加 mise 章节
- ✅ `.gitignore` - 添加 mise 相关忽略规则

**总计：28 个文件已创建/修改**

## 🚀 如何开始使用

### 第一步：安装 mise

```bash
# macOS
brew install mise

# Linux
curl https://mise.run | sh

# 或使用项目提供的脚本
./scripts/install_mise.sh
```

### 第二步：激活 mise

添加到你的 shell 配置（`~/.zshrc` 或 `~/.bashrc`）：

```bash
eval "$(mise activate zsh)"  # 或 bash
```

然后重启终端或运行：
```bash
source ~/.zshrc
```

### 第三步：信任并安装

```bash
cd zkvm-demos

# 信任配置
mise trust

# 安装所有工具
mise install

# 验证安装
./scripts/verify_mise_setup.sh
```

### 第四步：测试自动切换

```bash
# 根目录 - 使用 nightly
cd zkvm-demos
rustc --version  # → nightly-2025-06-05

# Jolt - 使用稳定版 1.88
cd jolt-zkvm
rustc --version  # → 1.88.0

# Cairo - 有 Python 环境
cd ../cairo-zkvm
python --version  # → 3.11.x
```

## 💡 核心功能

### 1. 自动工具链切换

当你 `cd` 到不同目录时，工具链自动切换：

```bash
cd jolt-zkvm     # → Rust 1.88
cd sp1-zkvm      # → Rust nightly
cd cairo-zkvm    # → Python 3.11
```

### 2. 便捷任务命令

每个 zkVM 都有预定义的任务：

```bash
# 查看可用任务
mise tasks

# 运行任务
mise run install-<zkvm>  # 安装工具链
mise run run             # 运行演示
mise run build-guest     # 构建 guest 程序
```

### 3. 环境变量管理

每个 zkVM 有预配置的环境变量：

```toml
[env]
RUST_LOG = "info"
FIBONACCI_N = "10"
```

### 4. 本地自定义

创建 `.mise.local.toml` 用于个人配置：

```bash
cp .mise.local.toml.example .mise.local.toml
vim .mise.local.toml
```

## 📚 使用示例

### 示例 1：使用 SP1

```bash
cd sp1-zkvm
mise run install-sp1     # 安装 SP1 工具链
mise run run-execute     # 执行演示
mise run run-prove       # 生成证明
```

### 示例 2：使用 Cairo

```bash
cd cairo-zkvm
mise run install-scarb   # 安装 Scarb
mise run build           # 构建项目
mise run test            # 运行测试
mise run run             # 运行程序
```

### 示例 3：在多个 zkVM 之间切换

```bash
cd jolt-zkvm
rustc --version          # → 1.88.0
cargo run --release

cd ../sp1-zkvm
rustc --version          # → nightly-2025-06-05
cargo run --release      # 工具链自动切换！
```

### 示例 4：查看工具状态

```bash
# 查看已安装的工具
mise list

# 查看当前激活的工具
mise current

# 检查所有工具
mise run check-tools

# 运行诊断
mise doctor
```

## 🎯 常用命令

| 命令 | 说明 |
|------|------|
| `mise list` | 显示已安装的工具 |
| `mise current` | 显示当前激活的工具 |
| `mise install` | 安装配置文件中的所有工具 |
| `mise tasks` | 列出当前目录可用的任务 |
| `mise run <task>` | 运行指定任务 |
| `mise doctor` | 检查 mise 设置 |
| `mise upgrade` | 升级所有工具 |

## 🔧 工具链映射表

| zkVM | Rust 版本 | 额外工具 |
|------|----------|----------|
| Jolt | 1.88 (stable) | RISC-V targets |
| Risc0 | nightly-2025-06-05 | cargo-risczero |
| SP1 | nightly-2025-06-05 | cargo-prove |
| Cairo | - | Python 3.11, Scarb |
| Cairo-M | 1.86 | cairo-m-* tools |
| Valida | 1.86 | LLVM 18+ |
| zkWasm | 1.86 | Node 20 |
| Lean | 1.85 | - |
| Novanet | 1.85 | - |
| Others | 1.86 或 nightly | 各种工具 |

## 📖 文档资源

1. **快速入门**（推荐新手）
   - [MISE_QUICK_START.md](MISE_QUICK_START.md) - 5分钟上手

2. **完整指南**
   - [MISE_SETUP.md](MISE_SETUP.md) - 英文完整指南
   - [MISE_SETUP.zh-CN.md](MISE_SETUP.zh-CN.md) - 中文完整指南

3. **实现细节**
   - [MISE_IMPLEMENTATION_SUMMARY.md](MISE_IMPLEMENTATION_SUMMARY.md) - 技术总结

4. **官方文档**
   - https://mise.jdx.dev/ - mise 官方文档

## 🆚 与其他方案对比

| 特性 | mise | Docker | Local Env |
|------|------|--------|-----------|
| 设置时间 | 5分钟 | 10分钟 | 15分钟 |
| 性能 | 100%原生 | ~90% | 100%原生 |
| 自动切换 | ✅ 是 | ❌ 否 | ⚠️ 需配置 |
| 学习曲线 | 简单 | 中等 | 复杂 |
| IDE 集成 | ✅ 无缝 | ⚠️ 需设置 | ✅ 无缝 |
| 团队协作 | ✅ 好 | ✅ 最好 | ⚠️ 较难 |
| CI/CD | ✅ 好 | ✅ 最好 | ❌ 不适合 |

**推荐使用场景：**
- 🔧 **mise**：独立开发、频繁构建
- 🐳 **Docker**：团队协作、CI/CD
- 🏠 **Local Env**：高级用户、需要 GPU

## ❓ 常见问题

### Q1: 工具没有自动切换？

```bash
# 检查 mise 是否激活
echo $MISE_SHELL  # 应显示 "zsh" 或 "bash"

# 如果为空，检查 shell 配置
grep "mise activate" ~/.zshrc

# 重新加载配置
source ~/.zshrc
```

### Q2: Rust 版本不对？

```bash
# 查看当前版本
mise current rust

# 清除缓存并重装
mise cache clear
mise install rust@1.88
mise install rust@nightly-2025-06-05
```

### Q3: 找不到 LLVM（Valida）？

mise 不管理 LLVM，需要手动安装：

```bash
# macOS
brew install llvm@18

# Ubuntu
wget https://apt.llvm.org/llvm.sh
sudo bash llvm.sh 18
```

然后在 `.mise.local.toml` 中设置路径：

```toml
[env]
LLVM_SYS_180_PREFIX = "/opt/homebrew/opt/llvm@18"
```

### Q4: Python 版本冲突？

```bash
# 检查 Python
mise current python

# 安装特定版本
mise install python@3.11

# 设为默认
mise use -g python@3.11
```

## 🎉 优势总结

### ✅ 解决的问题
- ✅ 工具链冲突：每个 zkVM 独立的工具链
- ✅ 手动切换：`cd` 自动激活正确的工具
- ✅ 复杂设置：一次 `mise install` 安装所有
- ✅ 文档缺失：提供中英文完整文档

### ✅ 保持的兼容性
- ✅ `rust-toolchain.toml`：仍然被 cargo 尊重
- ✅ Docker：仍可用于 CI/CD
- ✅ 现有脚本：所有 run 脚本仍然工作
- ✅ 手动安装：可与 mise 共存

### ✅ 新增的便利
- ✅ 任务系统：`mise run <task>` 快速操作
- ✅ 环境管理：预配置的环境变量
- ✅ 版本检查：`mise current` 查看工具
- ✅ 多语言支持：中英文文档

## 🔄 迁移指南

### 从手动管理迁移

```bash
# 1. 安装 mise
./scripts/install_mise.sh

# 2. 信任配置
mise trust

# 3. 安装工具（mise 会检测已安装的工具）
mise install

# 4. 开始使用
cd jolt-zkvm  # 自动切换！
```

### 从 Docker 迁移

你可以同时使用两者：
- **开发时**：使用 mise（更快）
- **CI/CD**：继续使用 Docker（更可靠）

```bash
# mise 用于本地开发
mise install
cd sp1-zkvm && cargo run --release

# Docker 用于 CI
docker run ... sp1-zkvm
```

## 📝 验证检查清单

运行验证脚本：

```bash
./scripts/verify_mise_setup.sh
```

应该看到：
- ✅ mise 已安装并激活
- ✅ 20/20 配置文件存在
- ✅ 工具已定义：rust, python, node
- ✅ .gitignore 已更新

## 🎓 下一步

1. **快速体验**（5分钟）
   ```bash
   ./scripts/install_mise.sh
   mise trust && mise install
   cd jolt-zkvm && mise run run
   ```

2. **深入学习**（30分钟）
   - 阅读 [MISE_SETUP.zh-CN.md](MISE_SETUP.zh-CN.md)
   - 尝试不同 zkVM 的任务
   - 创建自定义 `.mise.local.toml`

3. **团队推广**
   - 分享文档给团队成员
   - 在 CI 中保持使用 Docker
   - 本地开发使用 mise

## 📞 获取帮助

1. **查看文档**
   - `mise help` - 命令帮助
   - [MISE_SETUP.zh-CN.md](MISE_SETUP.zh-CN.md) - 完整指南

2. **运行诊断**
   ```bash
   mise doctor
   ./scripts/verify_mise_setup.sh
   ```

3. **官方资源**
   - https://mise.jdx.dev/ - 官方文档
   - https://github.com/jdx/mise - GitHub 仓库
   - https://discord.gg/mise - Discord 社区

## ✨ 总结

🎉 **mise 工具链管理已完全实现！**

- ✅ 28 个文件已创建/修改
- ✅ 20 个 zkVM 配置完成
- ✅ 中英文文档齐全
- ✅ 安装和验证脚本就绪
- ✅ 与现有方案兼容

**现在你有三种工具链管理方式可选：**

1. 🔧 **mise** - 独立开发者（新增，推荐）
2. 🐳 **Docker** - 团队和 CI/CD（已有）
3. 🏠 **Local workspaces** - 高级用户（已有）

**立即开始：**

```bash
./scripts/install_mise.sh
mise trust && mise install
cd jolt-zkvm && mise run run
```

---

**祝你使用 mise 愉快！🚀**

如有问题，请查看 [MISE_SETUP.zh-CN.md](MISE_SETUP.zh-CN.md) 获取详细帮助。

