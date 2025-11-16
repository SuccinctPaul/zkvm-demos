# Airbender zkVM - Project Overview

## 📖 Summary

This is a reference implementation of a Fibonacci computation demo for **Airbender**, zkSync's high-performance RISC-V zero-knowledge virtual machine. The project demonstrates the complete zkVM workflow structure that will be used once the official Airbender SDK is released.

## 🎯 Purpose

The primary goals of this demo are:

1. **Educational**: Show developers the expected workflow for Airbender zkVM
2. **Reference Implementation**: Provide a template that can be updated with official SDK
3. **Integration Testing**: Verify compatibility with the zkvm-demos repository structure
4. **Documentation**: Demonstrate best practices for RISC-V zkVM development

## 🏗️ Architecture

### Components

```
airbender-zkvm/
├── airbender-guest/          # Guest program (RISC-V)
│   ├── src/main.rs           # Fibonacci computation
│   └── Cargo.toml            # Guest dependencies
│
├── airbender-host/           # Host program (native)
│   ├── src/main.rs           # Prover & verifier
│   └── Cargo.toml            # Host dependencies
│
├── Cargo.toml                # Workspace configuration
├── rust-toolchain.toml       # Rust toolchain spec
├── README.md                 # User documentation
├── PROJECT_OVERVIEW.md       # This file
└── run_demo.sh               # Convenience script
```

### Workflow

1. **Guest Compilation**: Compile Rust code to RISC-V bytecode
2. **zkVM Execution**: Execute the RISC-V program in the zkVM
3. **Proof Generation**: Generate a zero-knowledge proof of execution
4. **Verification**: Verify the proof cryptographically

## 🔬 Technical Details

### Guest Program

- **Target**: `riscv32im-unknown-none-elf`
- **Environment**: `no_std` bare-metal RISC-V
- **Input**: Fibonacci sequence index (n)
- **Output**: nth Fibonacci number
- **Library**: Uses shared `fib` crate for computation

### Host Program

- **Responsibilities**:
  - Compile guest program to RISC-V ELF
  - Load and execute in zkVM
  - Generate zero-knowledge proof
  - Verify proof correctness
  
- **Dependencies** (planned):
  - `airbender_execution_utils` - Core zkVM runtime
  - `bincode` - Serialization
  - `serde` - Data structures
  - `anyhow` - Error handling

## 📊 Performance Characteristics

Airbender is designed for exceptional performance:

| Metric | Value |
|--------|-------|
| Proving Speed | ~21.8 MHz (H100 GPU) |
| Performance Gain | 6x faster than competitors |
| Transaction Cost | ~$0.0001 per transaction |
| Target Architecture | RISC-V ISA |
| GPU Support | NVIDIA (optimized) |

## 🔄 Current Status

### ✅ Implemented

- [x] Project structure and build configuration
- [x] Guest program (RISC-V compatible)
- [x] Host program workflow structure
- [x] Documentation (README, guides)
- [x] Installation scripts
- [x] Demo runner script
- [x] Integration with common libraries (`fib`, `common`)

### ⏳ Pending Official SDK

- [ ] Actual zkVM execution with Airbender
- [ ] Real proof generation
- [ ] Cryptographic verification
- [ ] Official `airbender_execution_utils` crate
- [ ] Official Airbender runtime APIs

## 🔧 Development Workflow

### Building

```bash
# Check compilation
cargo check

# Build debug
cargo build

# Build release
cargo build --release

# Build guest for RISC-V
cargo build --target riscv32im-unknown-none-elf
```

### Running

```bash
# Quick run with defaults
./run_demo.sh

# Custom Fibonacci input
FIB_N=20 ./run_demo.sh

# Debug mode
./run_demo.sh --debug

# Clean build
./run_demo.sh --clean
```

### Testing

```bash
# Run tests (when available)
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

## 🎓 Learning Resources

### Airbender Specific
- [zkSync Airbender Docs](https://docs.zksync.io/zk-stack/components/zksync-airbender)
- [ere Project Integration](https://github.com/eth-act/ere)

### RISC-V zkVM Concepts
- [RISC-V ISA Specification](https://riscv.org/technical/specifications/)
- [Zero-Knowledge Proofs Overview](https://zkproof.org/)

### zkSync Ecosystem
- [zkSync Era Documentation](https://docs.zksync.io/)
- [Matter Labs GitHub](https://github.com/matter-labs)

## 🔮 Future Enhancements

Once the official Airbender SDK is available:

1. **Direct Integration**
   - Replace placeholder code with actual SDK calls
   - Implement real proof generation
   - Add cryptographic verification

2. **Advanced Features**
   - GPU acceleration support
   - Parallel proof generation
   - Proof batching
   - Custom proving parameters

3. **Optimizations**
   - Memory usage optimization
   - Proof size reduction
   - Verification time improvements

4. **Additional Examples**
   - More complex computations
   - State machine examples
   - Cryptographic primitives
   - Smart contract verification

## 📝 Implementation Notes

### Design Decisions

1. **Workspace Structure**: Follows standard Rust zkVM pattern (guest + host)
2. **Shared Libraries**: Uses common `fib` crate for consistency across demos
3. **Error Handling**: Uses `anyhow` for ergonomic error propagation
4. **Configuration**: Environment variables for runtime configuration

### Compatibility

- **Rust Version**: 1.85+ required
- **Targets**: Native (host) + `riscv32im-unknown-none-elf` (guest)
- **Platform**: Linux, macOS (Windows via WSL)
- **GPU**: Optional but recommended (NVIDIA H100 for best performance)

### Migration Path

When updating to official SDK:

1. Update `Cargo.toml` dependencies
2. Replace placeholder implementations in `airbender-host/src/main.rs`
3. Update guest runtime in `airbender-guest/src/main.rs`
4. Test with official examples
5. Update documentation

## 🤝 Contributing

Contributions welcome once SDK is available:

1. Fork the repository
2. Create a feature branch
3. Implement improvements
4. Add tests
5. Update documentation
6. Submit pull request

## 📄 License

MIT OR Apache-2.0

## 📞 Support & Community

- **zkSync Discord**: https://discord.gg/zksync
- **GitHub Discussions**: https://github.com/matter-labs/zksync-era/discussions
- **Twitter**: [@zksync](https://twitter.com/zksync)
- **Blog**: https://blog.matter-labs.io/

## 🙏 Acknowledgments

- **zkSync Team** for developing Airbender
- **Matter Labs** for advancing zkVM technology
- **ere Project** for early integration examples
- **RISC-V Foundation** for the ISA specification

---

**Last Updated**: November 16, 2025  
**Status**: Reference Implementation - Awaiting Official SDK  
**Version**: 0.1.0

