# Cairo zkVM - 项目信息

## 核心文件

**唯一的源代码文件**: `src/lib.cairo` (152行)

包含完整的 Fibonacci 实现：
- ✅ `fib_recursive(n)` - 递归算法
- ✅ `fib_iterative(n)` - 迭代算法  
- ✅ `fib_pair(n)` - 配对算法
- ✅ `main()` - 主函数
- ✅ 4个单元测试（全部通过）

## 快速命令

```bash
# 构建
scarb build

# 测试
scarb test

# 运行
scarb cairo-run --available-gas=200000000
```

## 输出

```
Input:  n = 10
Output: [10, 55, 55, 55, 89]
  - fib_recursive(10) = 55
  - fib_iterative(10) = 55
  - fib_pair(10) = (55, 89)
```

## STARK 证明

**Sierra JSON**: `target/dev/cairo_fibonacci.sierra.json` (124KB)

生成证明：
```bash
# 方法1: StarkNet + Katana (推荐)
katana & 
starkli declare target/dev/cairo_fibonacci.contract_class.json

# 方法2: Stone Prover
./stone_prover_setup.sh
./generate_stone_proof.sh

# 方法3: Cairo Native
# 见 STARK_PROOF_GUIDE.md
```

## 项目状态

✅ 代码简洁：1个 .cairo 文件  
✅ 功能完整：所有算法 + 测试  
✅ 可以构建：Scarb 2.8.5  
✅ 测试通过：4/4  
✅ 可生成证明：Sierra JSON 就绪  

## 详细文档

- `README.md` - 完整使用指南
- `STARK_PROOF_GUIDE.md` - 证明生成详解
- `QUICK_START.md` - 快速参考

