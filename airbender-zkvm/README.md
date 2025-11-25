# Airbender zkVM - Fibonacci Demo

A demonstration of using **Airbender**, zkSync's high-performance RISC-V zero-knowledge virtual machine, to compute Fibonacci numbers with zero-knowledge proofs.

## 🌟 What is Airbender?

**Airbender** is a cutting-edge RISC-V zkVM developed by zkSync that brings unprecedented performance to zero-knowledge proof generation:

- ⚡ **~21.8 MHz** proving speed on H100 GPU (6x faster than competitors)
- 💰 **$0.0001** per transaction cost (significantly reduced on-chain costs)
- 🔧 **Full RISC-V ISA** compatibility
- 🚀 **Production-ready** integration with zkSync Era, Abstract, and Sophon chains

Airbender represents a major breakthrough in zkVM technology, enabling efficient verification of complex computations while maintaining the security guarantees of zero-knowledge proofs.

## 📋 Project Structure

```
airbender-zkvm/
├── airbender-guest/      # Guest program (runs in zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Fibonacci computation in RISC-V
├── airbender-host/       # Host program (proof generation & verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Prover and verifier implementation
├── Cargo.toml            # Workspace configuration
├── rust-toolchain.toml   # Rust toolchain specification
└── README.md             # This file
```

## 🎯 What This Demo Does

This demo showcases a complete zero-knowledge proof workflow:

1. **Guest Program**: A RISC-V program that computes the nth Fibonacci number
2. **Host Program**: Compiles the guest program, executes it in the zkVM, generates a proof, and verifies it
3. **Zero-Knowledge**: The proof demonstrates correct computation without revealing the computation trace

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (1.85 or later)
- RISC-V target: `rustup target add riscv32im-unknown-none-elf`
- Airbender SDK (installation instructions pending)

### Installation

```bash
# Install RISC-V target
rustup target add riscv32im-unknown-none-elf

# Install Airbender SDK (when available)
# The official SDK will be released by zkSync
# Check: https://docs.zksync.io/zk-stack/components/zksync-airbender
```

### Running the Demo

```bash
# Navigate to the airbender-zkvm directory
cd airbender-zkvm

# Build the project
cargo build --release

# Run the host program
cargo run --release --bin airbender-host
```

## 📊 Configuration

The Fibonacci input can be configured via environment variable:

```bash
# Set the Fibonacci number to compute (default: 10)
export FIB_N=15
cargo run --release --bin airbender-host
```

## 🔧 Implementation Details

### Guest Program (`airbender-guest`)

The guest program is compiled to RISC-V bytecode and runs inside the Airbender zkVM:

```rust
#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

#[cfg(target_arch = "riscv32")]
#[no_mangle]
pub extern "C" fn main() {
    let n: u32 = 10;
    let result = fib::fibonacci(n);
    core::hint::black_box(result);
}
```

Key features:
- `#![no_std]` - Runs in a bare-metal RISC-V environment
- Uses shared `fib` library for Fibonacci computation
- Compatible with Airbender's RISC-V execution environment

### Host Program (`airbender-host`)

The host program orchestrates proof generation and verification:

```rust
fn main() -> Result<()> {
    // 1. Compile guest program to RISC-V ELF
    let elf = compile_guest_program()?;
    
    // 2. Execute in zkVM and generate proof
    let (trace, proof) = airbender_execution_utils::prove(
        &elf,
        &input_data,
        ProofConfig::default(),
    )?;
    
    // 3. Verify the proof
    airbender_execution_utils::verify(
        &proof,
        &public_inputs,
        &verification_key,
    )?;
    
    Ok(())
}
```

## ⚠️ Current Status

**Note**: This is a **reference implementation** that demonstrates the expected workflow structure. 

- The actual Airbender SDK APIs may differ once officially released
- This demo is based on:
  - zkSync Airbender documentation
  - The [ere project](https://github.com/eth-act/ere) structure
  - Standard RISC-V zkVM patterns

### What's Working
- ✅ Project structure and build configuration
- ✅ RISC-V guest program compilation
- ✅ Demo workflow and documentation

### What's Pending
- ⏳ Official Airbender SDK integration
- ⏳ Actual proof generation using Airbender
- ⏳ Proof verification implementation

## 📚 Resources

### Official Documentation
- [zkSync Airbender Overview](https://docs.zksync.io/zk-stack/components/zksync-airbender)
- [zkSync GitHub Organization](https://github.com/matter-labs)

### Reference Projects
- [ere Project - Airbender Integration](https://github.com/eth-act/ere/tree/master/crates/zkvm/airbender)

### RISC-V zkVM Background
- [RISC-V ISA Specification](https://riscv.org/technical/specifications/)
- [Zero-Knowledge Proofs for RISC-V](https://zkproof.org/)

## 🔄 Updating to Official SDK

Once the Airbender SDK is officially released, update this demo:

1. **Update Dependencies** in `Cargo.toml`:
```toml
[workspace.dependencies]
airbender_execution_utils = "0.x.x"  # Official version
airbender_runtime = "0.x.x"
```

2. **Update Guest Program** with official runtime:
```rust
use airbender_runtime::*;

#[airbender_runtime::main]
fn main() {
    // Use official SDK APIs
}
```

3. **Update Host Program** with official proving APIs:
```rust
use airbender_sdk::*;

fn main() {
    let prover = AirbenderProver::new()?;
    let proof = prover.prove(elf, input)?;
    // ...
}
```

## 🤝 Contributing

Contributions are welcome! Once the official Airbender SDK is available:

1. Update the implementation to use official APIs
2. Add more comprehensive examples
3. Optimize performance
4. Improve documentation

## 📄 License

MIT OR Apache-2.0

## 🙏 Acknowledgments

- **zkSync Team** for developing Airbender
- **ere Project** for early Airbender integration examples
- **RISC-V Foundation** for the RISC-V ISA specification

---

## 🐛 Troubleshooting

### Build Errors

**Problem**: `target 'riscv32im-unknown-none-elf' not found`
```bash
rustup target add riscv32im-unknown-none-elf
```

**Problem**: Airbender SDK not found
- The official SDK is not yet publicly released
- Check zkSync's documentation for updates: https://docs.zksync.io/

### Running Issues

**Problem**: Demo shows placeholder messages
- This is expected! The demo shows the workflow structure
- Actual proof generation requires the official Airbender SDK

## 📬 Contact & Support

- [zkSync Discord](https://discord.gg/zksync)
- [zkSync GitHub Discussions](https://github.com/matter-labs/zksync-era/discussions)

---

**Last Updated**: November 2025
**Status**: Reference Implementation - Awaiting Official SDK Release

