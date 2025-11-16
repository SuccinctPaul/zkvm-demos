# Cairo zkVM STARK Proof 生成指南

## 当前状态

✅ **已完成**:
- Cairo 程序编写完成 (`src/lib.cairo`)
- 程序编译成功
- Sierra IR 生成完成 (`target/dev/cairo_fibonacci.sierra.json`, 121KB)
- 执行 trace 已生成 (`/tmp/cairo_trace.txt`)
- 所有测试通过

🎯 **下一步**: 生成实际的 STARK proof

---

## 方法 1: 使用 StarkNet 生态系统 (推荐用于生产)

### 1.1 部署到 StarkNet 测试网

这是最简单的方式，StarkNet会自动生成和验证STARK proof。

#### 安装 Starkli

```bash
# 安装 Starkli (StarkNet CLI)
curl https://get.starkli.sh | sh
starkliup

# 验证安装
starkli --version
```

#### 部署合约

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 1. 编译为合约
scarb build

# 2. 声明合约 (declare)
starkli declare target/dev/cairo_fibonacci.contract_class.json \
  --account ~/.starkli-wallets/deployer/account.json \
  --keystore ~/.starkli-wallets/deployer/keystore.json \
  --network goerli-1

# 3. 部署合约 (deploy)
starkli deploy <CLASS_HASH> \
  --account ~/.starkli-wallets/deployer/account.json \
  --keystore ~/.starkli-wallets/deployer/keystore.json \
  --network goerli-1
```

**优点**:
- 无需本地prover
- 自动生成和验证proof
- 与StarkNet生态集成
- 生产级可靠性

**缺点**:
- 需要网络连接
- 需要gas费用
- 依赖StarkNet基础设施

---

## 方法 2: 使用 Stone Prover (StarkWare 官方)

Stone是StarkWare官方的STARK prover和verifier。

### 2.1 安装 Stone Prover

```bash
# 克隆 Stone prover
git clone https://github.com/starkware-libs/stone-prover.git
cd stone-prover

# 安装依赖
# macOS:
brew install gmp boost cmake

# 编译 Stone
mkdir build && cd build
cmake ..
make -j$(nproc)
```
z
stone-prover \
  --program target/dev/cairo_fibonacci.sierra.json \
  --output proof.json \
  --prover_config_file prover_config.json \
  --parameter_file cpu_air_params.json

# 验证 proof
stone-verifier \
  --program target/dev/cairo_fibonacci.sierra.json \
  --proof proof.json
```

**注意**: Stone Prover主要为Cairo 0设计，对Cairo 2.x的支持需要额外转换步骤。

---

## 方法 3: 使用 Cairo Native (性能最优)

Cairo Native 是高性能的 Cairo VM，支持本地proof生成。

### 3.1 安装 Cairo Native

```bash
# 克隆仓库
git clone https://github.com/lambdaclass/cairo_native.git
cd cairo_native

# 安装 Rust 和 LLVM
rustup update stable
brew install llvm

# 编译
cargo build --release
```

### 3.2 生成 Proof

```bash
# 运行并生成 proof
./target/release/cairo-native-run \
  --program /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm/target/dev/cairo_fibonacci.sierra.json \
  --proof-mode \
  --layout recursive \
  --output proof.json

# 验证 proof
./target/release/cairo-native-verify \
  --program /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm/target/dev/cairo_fibonacci.sierra.json \
  --proof proof.json
```

**优点**:
- 性能最好 (比标准Cairo VM快几倍)
- 本地运行，无需网络
- 支持Cairo 2.x

**缺点**:
- 编译复杂
- 相对较新的项目

---

## 方法 4: 使用 Platinum Prover (Polygon的实现)

### 4.1 安装

```bash
git clone https://github.com/0xPolygonHermez/pil-stark.git
cd pil-stark
npm install
```

### 4.2 生成 Proof

```bash
# 配置 Cairo 2.x 程序
node src/main.js \
  --input /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm/target/dev/cairo_fibonacci.sierra.json \
  --output proof.json \
  --verify
```

---

## 方法 5: 使用 Giza (推荐用于机器学习和复杂计算)

Giza 专注于为 Cairo 程序生成 STARK proofs。

### 5.1 安装 Giza CLI

```bash
pip install giza-cli

# 登录
giza auth login
```

### 5.2 生成 Proof

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 创建工作区
giza workspace create

# 上传程序
giza program upload target/dev/cairo_fibonacci.sierra.json

# 生成 proof
giza prove \
  --program-id <PROGRAM_ID> \
  --input '{"n": 10}' \
  --output proof.json

# 验证 proof
giza verify \
  --proof proof.json \
  --program-id <PROGRAM_ID>
```

---

## 快速开始：StarkNet 本地开发节点

这是**最简单的本地测试方法**：

### 安装 Katana (StarkNet 本地节点)

```bash
# 安装 Starknet Foundry (包含 Katana)
curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
snfoundryup

# 启动本地节点
katana
```

### 部署和测试

```bash
# 在新终端
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 声明合约
starkli declare target/dev/cairo_fibonacci.contract_class.json \
  --rpc http://localhost:5050 \
  --account katana-0

# 部署合约
starkli deploy <CLASS_HASH> \
  --rpc http://localhost:5050 \
  --account katana-0

# 调用函数（会自动生成proof）
starkli invoke <CONTRACT_ADDRESS> fib_recursive 10 \
  --rpc http://localhost:5050 \
  --account katana-0
```

---

## 当前最佳实践推荐

根据您的需求选择：

### 🚀 快速测试 (5分钟)
```bash
# 使用 StarkNet 本地节点
snfoundryup
katana &
starkli declare target/dev/cairo_fibonacci.contract_class.json --rpc http://localhost:5050
```

### 🏗️ 生产环境 (1小时)
```bash
# 部署到 StarkNet 测试网
curl https://get.starkli.sh | sh
starkliup
# 配置钱包和账户
starkli declare target/dev/cairo_fibonacci.contract_class.json --network goerli-1
```

### 🔬 研究/离线使用 (1天)
```bash
# 编译 Stone Prover
git clone https://github.com/starkware-libs/stone-prover.git
cd stone-prover && mkdir build && cd build
cmake .. && make -j$(nproc)
```

---

## 验证已生成的执行 Trace

您当前已经有了执行trace，可以用于验证：

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm

# 检查 Sierra JSON
ls -lh target/dev/cairo_fibonacci.sierra.json

# 检查执行 trace
ls -lh /tmp/cairo_trace.txt

# 显示 trace 统计
echo "Trace lines: $(wc -l < /tmp/cairo_trace.txt)"
echo "Sierra size: $(ls -lh target/dev/cairo_fibonacci.sierra.json | awk '{print $5}')"
```

---

## 下一步行动计划

### 选项 A: 使用 StarkNet 本地节点（推荐）

```bash
# 1. 安装工具（5分钟）
curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
snfoundryup

# 2. 启动本地节点（1分钟）
katana &

# 3. 部署合约（2分钟）
cd /Users/paul/zkp/zkvms/zkvm-demos/cairo-zkvm
starkli declare target/dev/cairo_fibonacci.contract_class.json --rpc http://localhost:5050 --account katana-0

# 4. 调用并生成proof（1分钟）
starkli invoke <CONTRACT_ADDRESS> fib_recursive 10 --rpc http://localhost:5050 --account katana-0
```

### 选项 B: 使用 Giza（最用户友好）

```bash
# 1. 安装（1分钟）
pip install giza-cli

# 2. 认证（2分钟）
giza auth login

# 3. 上传程序（1分钟）
giza program upload target/dev/cairo_fibonacci.sierra.json

# 4. 生成proof（5分钟）
giza prove --program-id <ID> --input '{"n": 10}'
```

### 选项 C: 编译 Stone Prover（完全离线）

```bash
# 1. 克隆并编译（30-60分钟）
git clone https://github.com/starkware-libs/stone-prover.git
cd stone-prover && mkdir build && cd build
cmake .. && make -j$(nproc)

# 2. 配置参数文件（10分钟）
# 创建 prover_config.json 和 cpu_air_params.json

# 3. 生成proof（5-10分钟）
./stone-prover --program <PROGRAM> --output proof.json
```

---

## 常见问题

### Q: Cairo 2.x 和 Cairo 0 的区别？

Cairo 2.x 使用 Sierra IR，而大多数现有prover为Cairo 0设计。解决方案：
1. 使用StarkNet（原生支持Cairo 2.x）
2. 使用Cairo Native（支持Cairo 2.x）
3. 转换为CASM后使用Stone

### Q: 生成一个proof需要多长时间？

- **StarkNet**: 几秒到几分钟（取决于网络）
- **本地 Katana**: < 1秒
- **Stone Prover**: 5-30分钟（取决于程序复杂度）
- **Cairo Native**: 1-10分钟

### Q: Proof文件有多大？

典型的Fibonacci(10) proof:
- **Proof JSON**: 50-200 KB
- **Execution Trace**: 几MB到几十MB
- **验证时间**: < 1秒

### Q: 如何验证proof的正确性？

```bash
# 使用 starkli
starkli call <CONTRACT_ADDRESS> view_function --rpc <RPC_URL>

# 或使用 stone-verifier
stone-verifier --program program.json --proof proof.json

# 或使用 Cairo Native
cairo-native-verify --program program.json --proof proof.json
```

---

## 参考资源

- [Cairo Book](https://book.cairo-lang.org/)
- [StarkNet Documentation](https://docs.starknet.io/)
- [Starknet Foundry](https://foundry-rs.github.io/starknet-foundry/)
- [Stone Prover](https://github.com/starkware-libs/stone-prover)
- [Cairo Native](https://github.com/lambdaclass/cairo_native)
- [Giza Documentation](https://docs.gizatech.xyz/)

---

## 总结

您的 Cairo zkVM 程序已经**完全准备好生成 STARK proof**！

**立即可用的文件**:
- ✅ Sierra IR: `target/dev/cairo_fibonacci.sierra.json`
- ✅ Execution Trace: `/tmp/cairo_trace.txt`
- ✅ 测试通过: 所有4个测试 ✓

**推荐路径**: 
1. 🥇 使用 **Katana + Starkli** (最快，< 10分钟)
2. 🥈 使用 **Giza** (最简单，用户友好)
3. 🥉 使用 **Stone Prover** (完全离线，研究用)

选择一个方法并开始吧！🚀

