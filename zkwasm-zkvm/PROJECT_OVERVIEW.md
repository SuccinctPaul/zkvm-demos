# zkWasm Project Overview

## Introduction

This project demonstrates the [zkWasm](https://github.com/DelphinusLab/zkWasm) proving system, which provides zero-knowledge proofs for WebAssembly programs. zkWasm bridges the gap between Web2 and Web3 by allowing any WASM application to generate cryptographic proofs of correct execution.

## What Makes zkWasm Unique?

### 1. **WebAssembly Target**
Unlike most zkVMs that target RISC-V (like RISC0, SP1, Nexus), zkWasm directly executes WebAssembly bytecode. This offers several advantages:

- **Broader Language Support**: Any language that compiles to WASM (C, C++, Rust, Go, AssemblyScript, etc.)
- **Web Compatibility**: WASM programs can run in browsers and be proved
- **Existing Ecosystem**: Leverage the mature WASM tooling and optimization

### 2. **No Code Modification**
Existing WASM applications work without changes. Just compile and prove:
```
Your Code → WASM → zkWasm Prover → Proof
```

### 3. **Flexible Deployment**
- Run in browsers (via JavaScript)
- Deploy on serverless platforms (AWS Lambda, Azure Functions)
- Execute in blockchain smart contracts

## Technical Architecture

```
┌────────────────────────────────────────────────────────────┐
│                     Source Code                             │
│    (Rust, C, C++, AssemblyScript, etc.)                    │
└─────────────────────┬──────────────────────────────────────┘
                      │
                      │ compile to wasm32-unknown-unknown
                      ▼
┌────────────────────────────────────────────────────────────┐
│                  WASM Binary (.wasm)                        │
│  - Platform independent bytecode                            │
│  - Optimized for size and speed                             │
└─────────────────────┬──────────────────────────────────────┘
                      │
                      │ zkWasm Circuit Construction
                      ▼
┌────────────────────────────────────────────────────────────┐
│              zkSNARK Circuit                                │
│  - WASM VM implemented in arithmetic circuits               │
│  - Execution trace verification                             │
│  - Memory and stack operations                              │
└─────────────────────┬──────────────────────────────────────┘
                      │
                      │ Proving System
                      ▼
┌────────────────────────────────────────────────────────────┐
│            Zero-Knowledge Proof                             │
│  - Succinct proof of correct execution                      │
│  - Public inputs/outputs verifiable                         │
│  - Private computation details hidden                       │
└────────────────────────────────────────────────────────────┘
```

## Key Components

### 1. Guest Program (`zkwasm-guest/`)

The guest program is compiled to WASM and executed inside the zkWasm VM. Key requirements:

- **Entry Point**: Must export a `zkmain` function
- **No Standard Library**: Uses `#![no_std]` for minimal size
- **Host Functions**: Can call zkWasm runtime functions
- **Optimization**: Aggressively optimized for WASM size

Example structure:
```rust
#![no_std]

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    // Your computation here
}
```

### 2. Host Program (`zkwasm-host/`)

The host orchestrates the proving workflow:

1. **Build**: Compile guest to WASM
2. **Setup**: Initialize zkSNARK circuit parameters
3. **Prove**: Generate proof for specific inputs
4. **Verify**: Verify proof cryptographically

### 3. zkWasm CLI (`delphinus-cli`)

The command-line interface provides:
- Circuit setup and parameter generation
- Proof generation with various input types
- Proof verification
- Mock testing for development

## Workflow Explained

### Phase 1: Setup (One-Time)

```bash
delphinus-cli --params params demo setup -k 18 --wasm guest.wasm
```

This creates:
- Circuit parameters (proving key, verification key)
- Constraint system for WASM execution
- Takes ~5-10 minutes depending on circuit size (k)

**Circuit Size (k)**:
- k=18: ~262K constraints (simple programs)
- k=20: ~1M constraints (medium complexity)
- k=22: ~4M constraints (complex programs)

### Phase 2: Prove

```bash
delphinus-cli --params params demo prove \
  --wasm guest.wasm \
  --output output \
  --public 10:i64
```

This:
1. Executes WASM with given inputs
2. Records execution trace
3. Generates zkSNARK proof
4. Takes ~2-5 minutes for simple programs

### Phase 3: Verify

```bash
delphinus-cli --params params demo verify --output output
```

This:
1. Reads proof and public inputs
2. Verifies cryptographically
3. Takes ~10 seconds
4. Can be done by anyone with verification key

## Input/Output Handling

zkWasm supports several input types:

### Public Inputs (visible in proof)
```bash
--public 42:i64                    # Integer
--public "0x48656c6c6f":bytes      # Hex bytes
--public "Hello":bytes-packed       # Packed string
```

### Private Inputs (hidden in proof)
```bash
--private 12345:i64
--private "secret":bytes-packed
```

### Context Inputs (for state/continuations)
```bash
--ctxin 999:i64
```

## Performance Characteristics

### Proof Size
- Constant size (~200-400 KB) regardless of computation
- Much smaller than execution trace
- Suitable for on-chain verification

### Proving Time
Depends on:
- Circuit size (k parameter)
- Computation complexity
- Number of WASM instructions executed

Example (Fibonacci):
- n=10: ~2 minutes
- n=100: ~3 minutes
- n=1000: ~5 minutes

### Verification Time
- Constant ~10 seconds
- Independent of computation complexity
- Can be done on-chain

## Use Cases

### 1. **Private Computation**
Prove you ran a computation correctly without revealing inputs:
```
Private Data → WASM → Proof → Public Verification
```

### 2. **Verifiable Cloud Computing**
Cloud provider proves computation was done correctly:
```
Customer Request → Cloud WASM Execution → Proof → Customer Verification
```

### 3. **Smart Contract Integration**
Execute complex logic off-chain, verify on-chain:
```
Off-chain WASM → Proof → On-chain Verification (gas efficient)
```

### 4. **Gaming & Applications**
Prove game state transitions or app logic:
```
Game State → WASM Rules → Proof → Fair Play Verification
```

## Comparison with Other zkVMs

| Feature | zkWasm | RISC0 | SP1 | Nexus |
|---------|--------|-------|-----|-------|
| **Target ISA** | WebAssembly | RISC-V | RISC-V | RISC-V |
| **Language Support** | Any → WASM | Rust | Rust, C, C++ | Rust |
| **Web Browser** | ✅ Native | ❌ No | ❌ No | ❌ No |
| **Existing Apps** | ✅ WASM apps | ❌ Recompile | ❌ Recompile | ❌ Recompile |
| **Proof System** | Halo2 | STARK | STARK | STARK |
| **Maturity** | 🟡 Moderate | 🟢 Production | 🟢 Production | 🟡 Moderate |
| **Proof Size** | Small | Large | Medium | Medium |

### When to Choose zkWasm?

**Choose zkWasm if:**
- ✅ You have existing WASM applications
- ✅ You need browser-based proving
- ✅ You want language flexibility
- ✅ You prioritize small proof sizes

**Choose RISC-V zkVMs if:**
- ✅ You're writing new Rust code
- ✅ You want fastest proving times
- ✅ You need more mature tooling
- ✅ You don't need browser support

## Advanced Topics

### Custom Host Functions

You can extend zkWasm with custom circuits for specific operations:

```rust
extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
    fn wasm_dbget(key: i64) -> i64;      // Custom DB read
    fn wasm_dbset(key: i64, val: i64);   // Custom DB write
}
```

Contact DelphinusLab for custom circuit development.

### Batching and Aggregation

For multiple proofs, use the [continuation-batcher](https://github.com/DelphinusLab/continuation-batcher):
- Batch multiple proofs into one
- Recursive proof aggregation
- On-chain verifier generation

### On-Chain Verification

Generate Solidity verifier contracts:
```bash
# Generate verifier contract
delphinus-cli export-verifier --output Verifier.sol
```

Deploy to Ethereum/Polygon for on-chain verification.

## Resources

### Documentation
- [zkWasm GitHub](https://github.com/DelphinusLab/zkWasm)
- [IEEE Paper](https://ieeexplore.ieee.org/document/10587123)
- [DelphinusLab](https://www.delphinus-lab.com/)

### Example Projects
- [C Template](https://github.com/DelphinusLab/zkWasm-C)
- [Rust Demo](https://github.com/xgaozoyoe/zkWasm-Rust-Demo)
- [AssemblyScript Demo](https://github.com/DelphinusLab/zkWasm-AssemblyScript-Demo)
- [Browser Game Demo](https://github.com/zkcrossteam/g1024/)

### Community
- GitHub Issues: Report bugs and request features
- Contact: xgao@zoyoe.com (for enterprise integration)

## Troubleshooting

### Common Issues

**Issue**: WASM too large
**Solution**: 
- Use `opt-level = "z"`
- Enable LTO
- Strip symbols
- Use `wasm-opt` tool

**Issue**: Circuit too small
**Solution**: Increase k parameter (setup --k 20)

**Issue**: Proof generation fails
**Solution**: 
- Check WASM is valid
- Verify inputs match expected format
- Try --mock flag for debugging

**Issue**: Out of memory
**Solution**: 
- Reduce circuit size
- Use `--file` flag for file-backed tables
- Increase system RAM

## Future Directions

The zkWasm project is actively developed with planned features:

1. **Performance**: Faster proving with GPU acceleration
2. **Recursion**: Native support for recursive proofs
3. **WASM Extensions**: Support for SIMD and threads
4. **Developer Tools**: Better debugging and profiling
5. **Ecosystem**: More language SDKs and examples

## Conclusion

zkWasm provides a unique approach to zero-knowledge proofs by targeting WebAssembly. This enables:

- 🌐 **Universal**: Any language → WASM → Proof
- 🔒 **Secure**: Cryptographic guarantees of correctness
- 🚀 **Practical**: Works with existing applications
- 📦 **Compact**: Small proofs for efficient verification

Perfect for applications that need verifiable computation with privacy guarantees in the Web3 ecosystem.

