# o1vm Project Overview

## What is o1vm?

**o1vm** is a zero-knowledge virtual machine developed by O(1) Labs as part of their `proof-systems` repository. It is designed to prove the correct execution of MIPS programs using zero-knowledge proofs.

## Key Characteristics

### Architecture
- **Target**: MIPS32 instruction set architecture
- **Language**: Guest programs written in C or MIPS assembly
- **Proof System**: Kimchi (PLONK-based)
- **Curves**: Pasta curves (Pallas and Vesta)

### Design Philosophy

1. **RISC Architecture**: MIPS is a Reduced Instruction Set Computer (RISC) architecture
   - Simple, uniform instructions
   - Easy to reason about and optimize
   - Efficient circuit representations

2. **Production-Ready**: Used in the Mina Protocol
   - Battle-tested in production blockchain
   - Optimized for recursive proof composition
   - Designed for zkApp smart contracts

3. **Academic Foundation**: Based on well-studied cryptography
   - PLONK protocol for efficient ZK proofs
   - Polynomial commitment schemes
   - Pasta curves for recursion

## Technical Components

### 1. MIPS Interpreter

The o1vm interprets MIPS32 instructions and generates execution traces:

```
MIPS Instruction → Decode → Execute → Record State Changes
```

Key MIPS instructions supported:
- Arithmetic: ADD, SUB, MUL, DIV
- Logic: AND, OR, XOR, NOR
- Memory: LW, SW, LB, SB
- Control: BEQ, BNE, J, JAL
- Comparison: SLT, SLTU

### 2. Execution Trace

Records all state changes during program execution:
- Register values at each step
- Memory reads/writes
- Program counter updates
- Branch decisions

### 3. Witness Generation

Converts execution trace into polynomial witnesses:
- Encodes state as field elements
- Creates polynomial representations
- Prepares for constraint checking

### 4. Kimchi Proof System

Uses custom gates to efficiently prove MIPS operations:
- Polynomial commitments
- Permutation arguments
- Lookup tables for complex operations
- Recursive proof composition

## Comparison with Other Systems

### o1vm vs RISC Zero

| Aspect | o1vm | RISC Zero |
|--------|------|-----------|
| Architecture | MIPS32 | RISC-V |
| Proof System | Kimchi (PLONK) | STARK |
| Curves | Pasta | BN254 |
| Primary Use | Mina Protocol | General purpose |
| Maturity | Production | Production |
| Recursion | Native support | Yes |

### o1vm vs SP1

| Aspect | o1vm | SP1 |
|--------|------|-----|
| Architecture | MIPS32 | RISC-V |
| Proof System | Kimchi (PLONK) | STARK |
| Backend | Pasta curves | BN254 |
| Precompiles | Limited | Extensive |
| Speed | Fast | Very fast |

### o1vm vs Jolt

| Aspect | o1vm | Jolt |
|--------|------|------|
| Architecture | MIPS32 | RISC-V |
| Proof System | Kimchi (PLONK) | Lookup-based |
| Approach | Traditional circuits | Jolt lookups |
| Maturity | Production | Research |
| Performance | Proven | Promising |

## Use Cases

### 1. Smart Contracts (zkApps)
```
User Code → Compile to MIPS → Prove Execution → On-chain Verification
```

### 2. Private Computation
```
Private Inputs → MIPS Program → Prove Result → Public Verification
```

### 3. Scalability
```
Off-chain Computation → Generate Proof → On-chain Verification
```

### 4. Interoperability
```
Bridge Logic → Prove Correctly → Cross-chain Verification
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  (Smart Contracts, dApps, Privacy Applications)             │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                     Compiler Layer                           │
│  (C/C++ → MIPS32 Cross-compilation)                         │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                      o1vm Core                               │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   MIPS      │→ │  Execution   │→ │   Witness    │      │
│  │ Interpreter │  │    Trace     │  │  Generation  │      │
│  └─────────────┘  └──────────────┘  └──────────────┘      │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                   Kimchi Proof System                        │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Circuit   │→ │    Prover    │→ │   Verifier   │      │
│  │  Generation │  │              │  │              │      │
│  └─────────────┘  └──────────────┘  └──────────────┘      │
└────────────────────────┬────────────────────────────────────┘
                         │
┌────────────────────────▼────────────────────────────────────┐
│                   Cryptographic Backend                      │
│  (Pasta Curves: Pallas & Vesta)                             │
└─────────────────────────────────────────────────────────────┘
```

## Development Workflow

### 1. Write Guest Program
```c
// fibonacci.c
uint32_t fibonacci(uint32_t n) {
    if (n <= 1) return n;
    uint32_t a = 0, b = 1;
    for (uint32_t i = 2; i <= n; i++) {
        uint32_t temp = a + b;
        a = b;
        b = temp;
    }
    return b;
}
```

### 2. Compile to MIPS
```bash
mips-linux-gnu-gcc -O2 -static -o fibonacci.elf fibonacci.c
```

### 3. Run in o1vm
```rust
let binary = load_mips_binary("fibonacci.elf")?;
let interpreter = MipsInterpreter::new(binary);
let trace = interpreter.execute()?;
```

### 4. Generate Proof
```rust
let witnesses = generate_witnesses(&trace)?;
let proof = kimchi_prove(&witnesses)?;
```

### 5. Verify Proof
```rust
let is_valid = kimchi_verify(&proof)?;
```

## Performance Characteristics

### Proof Generation
- **Small Programs** (< 1K instructions): ~1-10 seconds
- **Medium Programs** (1K-10K instructions): ~10-60 seconds
- **Large Programs** (> 10K instructions): Minutes to hours

### Proof Size
- Constant size regardless of program size
- Typically 5-20 KB
- Optimized for recursion

### Verification Time
- Constant time verification
- Typically < 100ms
- Suitable for on-chain verification

## Security Model

### Assumptions
1. **Cryptographic**: Polynomial commitment scheme security
2. **Arithmetic**: Field arithmetic correctness
3. **Implementation**: Bug-free MIPS interpreter
4. **Circuits**: Correct constraint system

### Threat Model
- **Malicious Prover**: Cannot forge proofs for incorrect execution
- **Circuit Bugs**: Formal verification recommended
- **Side Channels**: Implementation must be constant-time

## Future Directions

### Short Term
- [ ] Improved instruction coverage
- [ ] Performance optimizations
- [ ] Better tooling and debugging

### Medium Term
- [ ] Extended MIPS instruction set
- [ ] Precompiled circuits for common operations
- [ ] Better developer experience

### Long Term
- [ ] Multiple architecture support
- [ ] Advanced optimization techniques
- [ ] Formal verification of circuits

## Resources

### Source Code
- Main Repository: https://github.com/o1-labs/proof-systems
- o1vm Module: https://github.com/o1-labs/proof-systems/tree/master/o1vm

### Documentation
- Kimchi: https://o1-labs.github.io/proof-systems/kimchi/overview.html
- Book: https://o1-labs.github.io/proof-systems/

### Papers
- PLONK: https://eprint.iacr.org/2019/953
- Pasta Curves: https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/

### Community
- Mina Discord: https://discord.gg/minaprotocol
- Forum: https://forums.minaprotocol.com/

## Conclusion

o1vm represents a production-ready zkVM for MIPS programs, backed by the robust Kimchi proof system and battle-tested in the Mina Protocol. While this demo provides a structural overview, full integration requires deep understanding of both MIPS architecture and advanced cryptographic protocols.

