#!/bin/bash

# Novanet zkVM 验证脚本
# 快速验证 demo 是否能够正确执行和生成证明

set -e

echo "=========================================="
echo "Novanet zkVM 验证脚本"
echo "=========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 测试计数器
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# 测试函数
run_test() {
    local test_name=$1
    local fib_n=$2
    local expected_result=$3
    
    echo -e "${YELLOW}测试 $((TESTS_RUN + 1)): $test_name${NC}"
    echo "  输入: FIBONACCI_N=$fib_n"
    echo "  预期结果: fib($fib_n) = $expected_result"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    # 运行测试
    output=$(FIBONACCI_N=$fib_n cargo run --release -p novanet-host 2>&1)
    
    # 检查是否包含预期结果
    if echo "$output" | grep -q "Result: fibonacci($fib_n) = $expected_result"; then
        if echo "$output" | grep -q "Proof verified successfully"; then
            if echo "$output" | grep -q "completed successfully"; then
                echo -e "  ${GREEN}✓ 测试通过${NC}"
                TESTS_PASSED=$((TESTS_PASSED + 1))
            else
                echo -e "  ${RED}✗ 测试失败: 未完整执行${NC}"
                TESTS_FAILED=$((TESTS_FAILED + 1))
            fi
        else
            echo -e "  ${RED}✗ 测试失败: 证明验证失败${NC}"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        echo -e "  ${RED}✗ 测试失败: 计算结果错误${NC}"
        echo "  实际输出: $(echo "$output" | grep "Result:" | head -1)"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    echo ""
}

# 获取脚本所在目录
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "1. 检查项目结构..."
if [ -f "Cargo.toml" ] && [ -d "novanet-guest" ] && [ -d "novanet-host" ]; then
    echo -e "   ${GREEN}✓ 项目结构完整${NC}"
else
    echo -e "   ${RED}✗ 项目结构不完整${NC}"
    exit 1
fi
echo ""

echo "2. 检查 Rust 工具链..."
if command -v cargo &> /dev/null; then
    echo -e "   ${GREEN}✓ Cargo 已安装: $(cargo --version)${NC}"
else
    echo -e "   ${RED}✗ Cargo 未安装${NC}"
    exit 1
fi
echo ""

echo "3. 构建项目..."
if cargo build --release > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓ 构建成功${NC}"
else
    echo -e "   ${RED}✗ 构建失败${NC}"
    exit 1
fi
echo ""

echo "4. 运行单元测试..."
if cargo test > /dev/null 2>&1; then
    echo -e "   ${GREEN}✓ 单元测试通过${NC}"
else
    echo -e "   ${RED}✗ 单元测试失败${NC}"
    exit 1
fi
echo ""

echo "5. 运行证明生成和验证测试..."
echo ""

# 运行多个测试用例
run_test "Fibonacci(5)" 5 8
run_test "Fibonacci(10)" 10 89
run_test "Fibonacci(15)" 15 987
run_test "Fibonacci(20)" 20 10946

# 显示测试摘要
echo "=========================================="
echo "测试摘要"
echo "=========================================="
echo "总测试数: $TESTS_RUN"
echo -e "通过: ${GREEN}$TESTS_PASSED${NC}"
echo -e "失败: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}=========================================="
    echo "✅ 所有验证通过！"
    echo "==========================================${NC}"
    echo ""
    echo "Novanet zkVM demo 已验证能够："
    echo "  ✓ 正确构建和编译"
    echo "  ✓ 执行 guest 程序"
    echo "  ✓ 生成零知识证明"
    echo "  ✓ 验证证明正确性"
    echo "  ✓ 处理多种输入值"
    echo ""
    echo "你可以开始使用了！"
    echo ""
    echo "快速开始："
    echo "  ./run_demo.sh"
    echo "  FIBONACCI_N=25 ./run_demo.sh"
    echo ""
    exit 0
else
    echo -e "${RED}=========================================="
    echo "❌ 验证失败！"
    echo "==========================================${NC}"
    echo ""
    echo "请检查错误信息并修复问题。"
    echo ""
    exit 1
fi

