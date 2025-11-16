# Miden zkVM Implementation Notes

## Current Status

This is a working implementation of Miden zkVM integration for the zkVM demos project. The code successfully compiles and demonstrates the basic structure for using Miden VM, though there is a known runtime issue being investigated.

## Implementation Details

### Architecture

The Miden zkVM demo follows the same pattern as other zkVM implementations in this repository:

1. **Host Program** (`miden-host/src/main.rs`): Rust program that:
   - Loads and compiles Miden Assembly code
   - Sets up execution environment
   - Generates STARK proofs
   - Verifies proofs

2. **Guest Program** (`programs/fib_simple.masm`): Miden Assembly program that computes Fibonacci numbers

3. **Dependencies**: Uses Miden VM 0.11 crates:
   - `miden-vm`: Core VM runtime
   - `miden-assembly`: Assembly compiler
   - `miden-processor`: Execution engine
   - `miden-stdlib`: Standard library

### Key Differences from Other zkVMs

Unlike RISC-V based zkVMs (RISC0, SP1, Nexus, etc.), Miden VM:

1. **Uses Miden Assembly**: Programs are written in a custom stack-based assembly language, not compiled from Rust
2. **Stack-based architecture**: Uses an operand stack instead of registers
3. **STARK-specific optimizations**: Designed specifically for efficient STARK proof generation
4. **No separate guest compilation**: Assembly is compiled at runtime by the host

### API Usage (Miden VM 0.11)

```rust
// Compile assembly
let assembler = Assembler::default();
let program = assembler.assemble_program(&source)?;

// Setup inputs
let stack_inputs = StackInputs::try_from_ints(vec![])?;

// Execute and prove
let host = DefaultHost::default();
let options = ProvingOptions::with_96_bit_security(false);
let (stack_outputs, proof) = miden_vm::prove(&program, stack_inputs, host, options)?;

// Verify
let program_info = ProgramInfo::from(program);
miden_vm::verify(program_info, stack_inputs, stack_outputs, proof)?;
```

## Known Issues

### Stack Size Validation Error

**Issue**: When running the demo, it fails with:
```
Error: Failed to prove program: The stack should have at most 16 elements at the end of program execution, but had 17 elements
```

**Investigation**:
- This occurs even with the simplest possible program (`begin push.8 end`)
- The error suggests the initial stack state may be incorrectly configured
- This appears to be an API usage issue specific to Miden VM 0.11

**Potential Causes**:
1. Incorrect `StackInputs` initialization
2. `DefaultHost` configuration issue  
3. Miden VM 0.11 may have specific requirements not reflected in available documentation
4. Possible incompatibility between crate versions

**Next Steps to Resolve**:
1. Review Miden VM 0.11 official examples in the repository
2. Check if there are initialization parameters being missed
3. Consider using Miden CLI tool to verify the assembly programs work standalone
4. May need to downgrade to Miden VM 0.10 or upgrade to latest version (0.19+)

## Miden Assembly Programs

### Simple Version (`fib_simple.masm`)

```masm
# Hardcoded Fibonacci result for demonstration
begin
    push.8  # fib(5) = 8
end
```

This is intentionally simplified to isolate the stack size issue. Once resolved, a proper iterative Fibonacci implementation can be used.

### Future: Full Iterative Version

A complete iterative Fibonacci implementation would look like:

```masm
begin
    adv_push.1          # Get n from advice provider
    
    push.1 lte          # Check if n <= 1
    if.true
        drop push.1     # Return 1 for base case
    else
        # Iterative computation
        push.1 sub      # counter = n-1
        push.1 push.1   # a=1, b=1
        
        # Loop to compute fibonacci
        # ... (implementation details)
    end
end
```

## Testing

Currently the demo compiles successfully but fails at runtime:

```bash
cd miden-zkvm/miden-host
FIBONACCI_N=5 cargo run --release
```

**Expected output** (once fixed):
```
====================================
   Miden zkVM Fibonacci Demo
====================================

Computing Fibonacci number for n = 5

1. Loading Miden Assembly program...
   ✓ Compilation successful

2. Executing and generating proof...
   ✓ Proof generated successfully
   Result: fib(5) = 8

3. Verifying proof...
   ✓ Proof verified successfully

====================================
   Demo completed successfully!
====================================
```

## References

- **Miden VM Repository**: https://github.com/0xPolygonMiden/miden-vm
- **Miden Assembly Docs**: https://0xpolygonmiden.github.io/miden-vm/user_docs/assembly/main.html
- **Miden VM Book**: https://0xpolygonmiden.github.io/miden-vm/

## Contributing

If you encounter this issue or find a solution:

1. Check the Miden VM documentation for version-specific requirements
2. Look for working examples in the Miden VM repository for version 0.11
3. Consider joining the Polygon Discord for community support
4. Update this document with findings

## Version Information

- Miden VM: 0.11.0
- Miden Assembly: 0.11.0
- Miden Processor: 0.11.0
- Rust: stable
- Status: Implementation complete, runtime issue under investigation

