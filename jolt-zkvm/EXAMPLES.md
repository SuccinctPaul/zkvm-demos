# Jolt zkVM 使用示例

本文档提供了 Jolt zkVM Demo 的各种使用示例。

## 基础示例

### 示例 1: 默认运行

```bash
cd jolt-zkvm
./run_demo.sh
```

**预期输出:**
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
```

### 示例 2: 自定义 Fibonacci 数值

```bash
# 计算 fibonacci(5)
FIBONACCI_N=5 ./run_demo.sh
```

**预期结果:** fibonacci(5) = 8

```bash
# 计算 fibonacci(15)
FIBONACCI_N=15 ./run_demo.sh
```

**预期结果:** fibonacci(15) = 987

### 示例 3: 启用详细日志

```bash
RUST_LOG=debug FIBONACCI_N=10 ./run_demo.sh
```

这会显示更详细的执行信息，包括：
- 编译过程详情
- 证明生成步骤
- 内部状态信息

## 高级示例

### 示例 4: 仅构建项目

```bash
./run_demo.sh build
```

这个命令会：
- 编译 guest 程序
- 编译 host 程序
- 不运行证明生成

### 示例 5: 清理构建产物

```bash
./run_demo.sh clean
```

清理所有编译产物以进行全新构建。

### 示例 6: 直接使用 Cargo

```bash
cd jolt-host

# 运行 demo
FIBONACCI_N=10 cargo run --release

# 仅构建
cargo build --release

# 检查代码
cargo check

# 查看构建详情
cargo build --release --verbose
```

## 测试示例

### 示例 7: 测试不同输入值

```bash
#!/bin/bash
# test_multiple.sh

for n in 5 7 10 12 15; do
    echo "Testing fibonacci($n)..."
    FIBONACCI_N=$n ./run_demo.sh
    echo "---"
done
```

### 示例 8: 性能基准测试

使用 `hyperfine` 进行基准测试：

```bash
# 安装 hyperfine
cargo install hyperfine

# 运行基准测试
cd jolt-host
hyperfine --warmup 3 \
  'FIBONACCI_N=5 cargo run --release' \
  'FIBONACCI_N=10 cargo run --release' \
  'FIBONACCI_N=15 cargo run --release'
```

### 示例 9: 测试脚本的所有模式

```bash
# 运行模式
./run_demo.sh run

# 构建模式
./run_demo.sh build

# 清理模式
./run_demo.sh clean

# 帮助信息
./run_demo.sh help
```

## 开发示例

### 示例 10: 修改 Guest 程序

编辑 `jolt-guest/src/lib.rs`，添加新函数：

```rust
#![cfg_attr(feature = "guest", no_std)]
#![cfg_attr(feature = "guest", no_main)]

#[jolt::provable]
pub fn fibonacci(n: u32) -> u32 {
    fib::fibonacci(n)
}

#[jolt::provable]
pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

然后在 `jolt-host/src/main.rs` 中使用：

```rust
// 添加到 main 函数
let (prove_factorial, verify_factorial) = jolt_guest::build_factorial();
let (output, proof) = prove_factorial(5);
println!("factorial(5) = {}", output);
```

### 示例 11: 添加自定义输入处理

编辑 `jolt-host/src/main.rs`：

```rust
use std::env;

pub fn main() {
    // 从命令行参数读取
    let args: Vec<String> = env::args().collect();
    let fib_n = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(10)
    } else {
        common::load_fib_n()
    };
    
    println!("Computing fibonacci({})...", fib_n);
    // ... rest of code
}
```

使用：
```bash
cd jolt-host
cargo run --release 15  # 计算 fibonacci(15)
```

### 示例 12: 添加证明保存功能

```rust
use std::fs::File;
use std::io::Write;

pub fn main() {
    // ... existing code ...
    
    let (output, proof) = prove_fibonacci(fib_n);
    
    // 保存证明到文件
    let proof_bytes = serialize_proof(&proof);
    let mut file = File::create("proof.bin").unwrap();
    file.write_all(&proof_bytes).unwrap();
    println!("Proof saved to proof.bin");
    
    // ... rest of code
}
```

## 集成示例

### 示例 13: 作为库使用

创建一个新项目并使用 jolt-guest：

```toml
# Cargo.toml
[dependencies]
jolt-guest = { path = "../jolt-zkvm/jolt-guest" }
jolt-sdk = { git = "https://github.com/a16z/jolt", rev = "55b9830..." }
```

```rust
// src/main.rs
fn main() {
    let (prove_fib, verify_fib) = jolt_guest::build_fibonacci();
    
    let inputs = vec![5, 10, 15];
    for n in inputs {
        let (result, proof) = prove_fib(n);
        println!("fib({}) = {}", n, result);
        assert!(verify_fib(proof));
    }
}
```

### 示例 14: Web API 集成

```rust
// 使用 actix-web
use actix_web::{web, App, HttpServer, Responder};

async fn prove_fibonacci(n: web::Path<u32>) -> impl Responder {
    let (prove_fib, _) = jolt_guest::build_fibonacci();
    let (result, proof) = prove_fib(*n);
    
    web::Json(serde_json::json!({
        "input": *n,
        "output": result,
        "proof": format!("{:?}", proof)
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/fib/{n}", web::get().to(prove_fibonacci))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 调试示例

### 示例 15: 使用 Rust 调试器

```bash
# 安装 rust-gdb 或 rust-lldb
cd jolt-host

# 使用 gdb
rust-gdb target/release/jolt-host

# 使用 lldb
rust-lldb target/release/jolt-host
```

### 示例 16: 输出调试信息

在 `jolt-host/src/main.rs` 中：

```rust
pub fn main() {
    println!("Debug: Starting demo...");
    
    let fib_n = common::load_fib_n();
    println!("Debug: fib_n = {}", fib_n);
    
    let (prove_fibonacci, verify_fibonacci) = jolt_guest::build_fibonacci();
    println!("Debug: Guest program built");
    
    let (output, proof) = prove_fibonacci(fib_n);
    println!("Debug: Proof generated, output = {}", output);
    
    let is_valid = verify_fibonacci(proof);
    println!("Debug: Verification result = {}", is_valid);
}
```

## 环境配置示例

### 示例 17: 使用不同的 .env 配置

```bash
# 创建多个配置文件
cat > .env.dev << EOF
FIBONACCI_N=5
RUST_LOG=debug
EOF

cat > .env.prod << EOF
FIBONACCI_N=20
RUST_LOG=info
EOF

# 使用不同配置
cp .env.dev .env && ./run_demo.sh
cp .env.prod .env && ./run_demo.sh
```

### 示例 18: 使用 direnv 自动切换环境

```bash
# 安装 direnv
brew install direnv  # macOS
apt install direnv   # Ubuntu

# 创建 .envrc
cat > .envrc << EOF
export FIBONACCI_N=10
export RUST_LOG=info
export PATH=$PWD/target/release:$PATH
EOF

# 允许 direnv
direnv allow

# 现在进入目录会自动设置环境变量
cd jolt-zkvm  # 自动加载环境
```

## CI/CD 示例

### 示例 19: GitHub Actions

```yaml
# .github/workflows/jolt-zkvm.yml
name: Jolt zkVM CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly-2024-10-30
          override: true
      
      - name: Install Jolt
        run: |
          cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
          jolt install-toolchain
      
      - name: Build
        run: cd jolt-zkvm && ./run_demo.sh build
      
      - name: Test
        run: cd jolt-zkvm && FIBONACCI_N=5 ./run_demo.sh
```

### 示例 20: Docker 构建

```dockerfile
# Dockerfile
FROM rust:nightly-slim

# 安装依赖
RUN apt-get update && apt-get install -y git

# 安装 Jolt
RUN cargo +nightly install --git https://github.com/a16z/jolt --force --bins jolt
RUN jolt install-toolchain

# 复制代码
WORKDIR /app
COPY . .

# 构建
WORKDIR /app/jolt-zkvm
RUN cargo build --release

# 运行
CMD ["./run_demo.sh"]
```

使用 Docker：
```bash
docker build -t jolt-zkvm-demo .
docker run -e FIBONACCI_N=10 jolt-zkvm-demo
```

## 总结

这些示例涵盖了：
- ✅ 基础使用
- ✅ 高级配置
- ✅ 测试方法
- ✅ 开发扩展
- ✅ 集成方案
- ✅ 调试技巧
- ✅ CI/CD 集成

更多问题请参考：
- [README.md](./README.md) - 完整文档
- [QUICKSTART.md](./QUICKSTART.md) - 快速开始
- [IMPLEMENTATION_NOTES.md](./IMPLEMENTATION_NOTES.md) - 实现细节

