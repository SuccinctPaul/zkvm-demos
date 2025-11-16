# Cairo-M zkVM - Project Overview

## What is Cairo-M?

Cairo-M is a Mobile-first CPU AIR (Algebraic Intermediate Representation) zkVM developed by KKRT Labs. It represents a new approach to zero-knowledge virtual machines optimized for consumer hardware, especially mobile devices.

## Key Design Principles

### 1. M31 Field Arithmetic
Cairo-M uses the M31 (Mersenne 31) prime field as its native field:
- **Prime**: 2^31 - 1 (Mersenne prime)
- **Benefits**: Extremely efficient on 32-bit processors (mobile ARM)
- **Operations**: Fast modular reduction due to Mersenne prime properties
- **Mobile-optimized**: Ideal for phone processors

### 2. Minimal Register Architecture
Unlike traditional VMs, Cairo-M uses only two registers:
- **PC (Program Counter)**: Tracks current instruction
- **FP (Frame Pointer)**: Manages stack frames

This minimal design:
- Simplifies the constraint system
- Reduces proof complexity
- Makes verification more efficient
- Enables faster mobile proving

### 3. Deterministic Frame Sizes
Each function call creates a stack frame of deterministic, constant size:
- Predictable memory layout
- Easier constraint generation
- Better cache locality
- Simplified memory proofs

### 4. Read-Write Memory
Unlike many zkVMs that use read-only memory or complex memory schemes:
- Direct read-write memory access
- More intuitive programming model
- Efficient memory operations
- Reduced constraint overhead

### 5. Variable-Size Instruction Encoding
Inspired by x86 instruction encoding:
- Instructions have variable lengths
- More compact programs
- Efficient encoding of common operations
- Better code density

## Architecture Overview

```
┌─────────────────────────────────────────┐
│         Cairo-M Language (.cm)          │
│  (Cairo-like syntax for M31 field)     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      cairo-m-compiler                   │
│  • Parses .cm source files              │
│  • Type checking and validation         │
│  • Generates instruction sequence       │
│  • Outputs JSON format                  │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      cairo-m-runner                     │
│  • Executes compiled program            │
│  • Generates execution trace            │
│  • Tracks memory accesses               │
│  • Outputs trace for proving            │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      cairo-m-prover (Stwo)              │
│  • Builds AIR constraints               │
│  • Generates STARK proof                │
│  • Uses Stwo prover backend             │
│  • Optimized for mobile devices         │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Verifier                        │
│  • Verifies STARK proof                 │
│  • Fast verification (~100ms)           │
│  • Can run on-chain or off-chain        │
└─────────────────────────────────────────┘
```

## Cairo-M Language

### Syntax
Cairo-M uses Cairo-like syntax adapted for the M31 field:

```cairo-m
// Function definition
func fibonacci(n: felt) -> felt {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let a = 0;
    let b = 1;
    let i = 2;
    
    while i <= n {
        let temp = a + b;
        a = b;
        b = temp;
        i = i + 1;
    }
    
    return b;
}
```

### Type System
- `felt`: Field element (M31)
- `u32`: 32-bit unsigned integer
- `bool`: Boolean
- Arrays: `[felt]`, `[u32]`, etc.

### Control Flow
- `if/else`: Conditional branching
- `while`: Loops
- `return`: Function returns
- Function calls with multiple parameters

## Component System

Cairo-M leverages Stwo's component system for efficient constraint handling:

### Components
Each operation type has its own component:
- **ALU Component**: Arithmetic operations
- **Memory Component**: Load/store operations
- **Control Flow Component**: Branches and jumps
- **Range Check Component**: Boundary validation

### Benefits
- Modular constraint system
- Efficient constraint generation
- Easy to add new operations
- Better proof composition

## Stwo Prover Integration

Cairo-M uses Starkware's Stwo prover:

### Features
- Circle STARK protocol
- M31 field native support
- Efficient polynomial commitments
- FRI-based polynomial opening
- Parallel proof generation

### Performance
- Proving time: ~2-5s for typical programs on mobile
- Verification time: ~50-100ms
- Proof size: 30-100KB for most programs
- Memory usage: 200-500MB

## Comparison with Other zkVMs

### vs Cairo (StarkNet)
| Feature | Cairo-M | Cairo |
|---------|---------|-------|
| Field | M31 (2^31-1) | Large prime |
| Registers | 2 (PC, FP) | Many registers |
| Target | Mobile devices | Starknet ecosystem |
| Memory | Read-write | Complex model |
| Proving | Stwo | Custom prover |

### vs RISC Zero
| Feature | Cairo-M | RISC Zero |
|---------|---------|-----------|
| ISA | Custom | RISC-V |
| Language | Cairo-M | Rust |
| Field | M31 | Goldilocks |
| Mobile | Optimized | Heavy |
| Setup | None | None |

### vs Miden
| Feature | Cairo-M | Miden |
|---------|---------|-------|
| Architecture | Register-based | Stack-based |
| Language | Cairo-M | Miden Assembly |
| Field | M31 | Goldilocks |
| Proving | Stwo | Winterfell |
| Mobile | Optimized | Moderate |

## Use Cases

### Ideal For
1. **Mobile Applications**
   - Proof generation on phones
   - Privacy-preserving mobile apps
   - Decentralized mobile identity

2. **Consumer Devices**
   - IoT device authentication
   - Edge computing with proofs
   - Client-side verification

3. **Lightweight Proofs**
   - Fast verification needed
   - Small proof sizes important
   - Limited computational resources

### Not Ideal For
1. Large-scale batch processing (use RISC Zero/SP1)
2. Ethereum L2 sequencing (use Cairo)
3. Complex memory-intensive programs (use traditional zkVMs)

## Development Status

⚠️ **Alpha Stage**: Cairo-M is actively under development

### Current State
- ✅ Core VM architecture complete
- ✅ M31 field arithmetic working
- ✅ Basic instruction set implemented
- ✅ Stwo prover integration
- ✅ Compiler infrastructure
- 🚧 Standard library in progress
- 🚧 Optimizations ongoing
- 🚧 Production hardening needed

### Roadmap
- Q1 2025: Beta release with complete instruction set
- Q2 2025: Mobile SDK release
- Q3 2025: Production-ready 1.0
- Q4 2025: Advanced optimizations

## Performance Characteristics

### Desktop (x86_64)
- **Compilation**: ~100-200ms for small programs
- **Execution**: Microseconds to milliseconds
- **Proving**: ~1-3s for n=100 fibonacci
- **Verification**: ~50ms
- **Memory**: 200-400MB

### Mobile (ARM)
- **iPhone 13 Pro**:
  - Proving: ~3-5s for n=100 fibonacci
  - Verification: ~80ms
  - Memory: 300-500MB

- **Android Flagship**:
  - Proving: ~4-6s for n=100 fibonacci
  - Verification: ~100ms
  - Memory: 300-500MB

### Scaling
- Linear scaling with program size
- Efficient for programs up to ~1M cycles
- Proof size stays relatively constant

## Resources

### Official
- **GitHub**: https://github.com/kkrt-labs/cairo-m
- **Design Document**: https://github.com/kkrt-labs/cairo-m/blob/main/docs/design-document.md
- **Getting Started**: https://github.com/kkrt-labs/cairo-m/blob/main/docs/getting-started.md

### Learning
- **CairoMlings**: Interactive tutorial for Cairo-M
- **Examples**: https://github.com/kkrt-labs/cairo-m/tree/main/examples
- **Test Programs**: https://github.com/kkrt-labs/cairo-m/tree/main/test_data

### Related Projects
- **Stwo**: https://github.com/starkware-libs/stwo
- **M31 Field**: Research papers on Mersenne primes
- **STARK Papers**: Academic resources on STARKs

## Contributing

Cairo-M is open source and welcomes contributions:

### Areas for Contribution
1. **Standard Library**: Math, crypto, data structures
2. **Optimizations**: Compiler and prover optimizations
3. **Documentation**: Tutorials, guides, examples
4. **Tooling**: IDE support, debuggers, profilers
5. **Testing**: Test cases, benchmarks, fuzzing

### Getting Started
1. Fork the repository
2. Set up development environment
3. Read the design document
4. Pick an issue or propose a feature
5. Submit a pull request

## License

Cairo-M is dual-licensed under:
- Apache License 2.0
- MIT License

You may choose either license for your use.

## Acknowledgments

- **KKRT Labs**: Core development team
- **Starkware**: Stwo prover and STARK research
- **Cairo Team**: Inspiration for language design
- **ZK Community**: Research and collaboration

