# zkWasm Examples and Use Cases

This document provides additional examples and use cases for zkWasm beyond the simple Fibonacci demo.

## Table of Contents

1. [Basic Examples](#basic-examples)
2. [Advanced Examples](#advanced-examples)
3. [Real-World Use Cases](#real-world-use-cases)
4. [Integration Patterns](#integration-patterns)

## Basic Examples

### Example 1: Simple Addition

Prove that you correctly added two numbers:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let a = wasm_input(1);  // Public input
        let b = wasm_input(1);  // Public input
        let sum = a + b;
        wasm_output(sum);
        sum
    }
}
```

**Use case:** Prove you computed a sum correctly without revealing intermediate steps.

### Example 2: Hash Verification

Prove you know a preimage of a hash:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

// Simple hash function (use proper crypto in production)
fn simple_hash(x: u64) -> u64 {
    let mut result = x;
    for _ in 0..100 {
        result = result.wrapping_mul(1103515245).wrapping_add(12345);
        result = (result / 65536) % 32768;
    }
    result
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let preimage = wasm_input(0);     // Private input (hidden)
        let expected_hash = wasm_input(1); // Public input (visible)
        
        let computed_hash = simple_hash(preimage as u64);
        
        // Output result
        wasm_output(computed_hash as i64);
        
        // Check if hash matches (in real use, this would be verified off-chain)
        if computed_hash == expected_hash as u64 {
            1 // Success
        } else {
            0 // Failure
        }
    }
}
```

**Use case:** Prove you know a password without revealing it.

### Example 3: Range Proof

Prove a number is within a range without revealing the number:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let secret_value = wasm_input(0); // Private
        let min = wasm_input(1);          // Public
        let max = wasm_input(1);          // Public
        
        let in_range = if secret_value >= min && secret_value <= max {
            1
        } else {
            0
        };
        
        wasm_output(in_range);
        in_range
    }
}
```

**Use case:** Prove your age is over 18 without revealing your exact age.

## Advanced Examples

### Example 4: Private Vote Counting

Aggregate votes while keeping individual votes private:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let num_votes = wasm_input(1) as usize; // Public: number of votes
        
        let mut yes_count = 0i64;
        let mut no_count = 0i64;
        
        // Read votes (private)
        for _ in 0..num_votes {
            let vote = wasm_input(0); // 0 = private
            if vote == 1 {
                yes_count += 1;
            } else {
                no_count += 1;
            }
        }
        
        // Output totals
        wasm_output(yes_count);
        wasm_output(no_count);
        
        yes_count
    }
}
```

**How to run:**
```bash
# Configure for 5 votes (3 yes, 2 no)
delphinus-cli --params params vote prove \
  --wasm output/guest.wasm \
  --output output \
  --public 5:i64 \
  --private 1:i64 \
  --private 1:i64 \
  --private 1:i64 \
  --private 0:i64 \
  --private 0:i64
```

### Example 5: Simple State Machine

Prove correct state transitions:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

// State machine: Idle -> Running -> Completed
#[derive(Clone, Copy, PartialEq)]
#[repr(i64)]
enum State {
    Idle = 0,
    Running = 1,
    Completed = 2,
}

impl State {
    fn from_i64(val: i64) -> Self {
        match val {
            0 => State::Idle,
            1 => State::Running,
            2 => State::Completed,
            _ => State::Idle,
        }
    }
}

fn transition(current: State, action: i64) -> State {
    match (current, action) {
        (State::Idle, 1) => State::Running,
        (State::Running, 2) => State::Completed,
        _ => current,
    }
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let initial_state = State::from_i64(wasm_input(1));
        let num_actions = wasm_input(1) as usize;
        
        let mut current_state = initial_state;
        
        for _ in 0..num_actions {
            let action = wasm_input(0); // Private action
            current_state = transition(current_state, action);
        }
        
        wasm_output(current_state as i64);
        current_state as i64
    }
}
```

### Example 6: Array Sum with Bounds Check

Prove you correctly summed an array within bounds:

```rust
#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn wasm_input(is_public: i32) -> i64;
    fn wasm_output(value: i64);
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let array_len = wasm_input(1) as usize; // Public
        let max_allowed = wasm_input(1);        // Public
        
        let mut sum = 0i64;
        let mut all_valid = 1i64;
        
        for _ in 0..array_len {
            let value = wasm_input(0); // Private
            
            // Check bounds
            if value < 0 || value > max_allowed {
                all_valid = 0;
            }
            
            sum = sum.wrapping_add(value);
        }
        
        // Only output if all values were valid
        if all_valid == 1 {
            wasm_output(sum);
            sum
        } else {
            wasm_output(-1);
            -1
        }
    }
}
```

## Real-World Use Cases

### Use Case 1: Private Credential Verification

**Scenario:** Prove you have a valid credential without revealing the credential itself.

**Example:** Age verification for online services
- Input: Birthday (private), Current date (public)
- Output: Boolean (is over 18)
- No personal information leaked

**Implementation:**
```rust
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let birth_year = wasm_input(0);   // Private
        let current_year = wasm_input(1); // Public
        let age = current_year - birth_year;
        
        let is_adult = if age >= 18 { 1 } else { 0 };
        wasm_output(is_adult);
        is_adult
    }
}
```

### Use Case 2: Confidential Transactions

**Scenario:** Prove a transaction is valid without revealing amounts.

**Example:** Private payment verification
- Input: Sender balance, Amount (both private)
- Output: Boolean (transaction valid)
- Proves: sender_balance >= amount

```rust
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let sender_balance = wasm_input(0);  // Private
        let amount = wasm_input(0);          // Private
        let recipient_id = wasm_input(1);    // Public
        
        let valid = if sender_balance >= amount && amount > 0 {
            wasm_output(1); // Valid
            1
        } else {
            wasm_output(0); // Invalid
            0
        };
        
        valid
    }
}
```

### Use Case 3: Verifiable Random Number Generation

**Scenario:** Prove random number generation was fair.

**Example:** Lottery or gaming
- Input: Seed (private), Block hash (public)
- Output: Random number
- Proves: number derived fairly from inputs

```rust
fn prng(seed: u64, block_hash: u64) -> u64 {
    let mut x = seed ^ block_hash;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let seed = wasm_input(0) as u64;       // Private
        let block_hash = wasm_input(1) as u64; // Public
        let max_value = wasm_input(1) as u64;  // Public
        
        let random = prng(seed, block_hash) % max_value;
        wasm_output(random as i64);
        random as i64
    }
}
```

### Use Case 4: Privacy-Preserving Machine Learning

**Scenario:** Run ML inference without revealing input data.

**Example:** Medical diagnosis
- Input: Patient data (private)
- Output: Diagnosis score
- Proves: Model ran correctly without revealing patient data

```rust
// Simplified neural network inference
fn inference(input: &[i64; 4], weights: &[i64; 4]) -> i64 {
    let mut sum = 0i64;
    for i in 0..4 {
        sum += input[i] * weights[i];
    }
    // Simple activation (ReLU)
    if sum > 0 { sum } else { 0 }
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        // Load private input data
        let mut input = [0i64; 4];
        for i in 0..4 {
            input[i] = wasm_input(0); // Private
        }
        
        // Load public model weights
        let mut weights = [0i64; 4];
        for i in 0..4 {
            weights[i] = wasm_input(1); // Public
        }
        
        let result = inference(&input, &weights);
        wasm_output(result);
        result
    }
}
```

## Integration Patterns

### Pattern 1: Browser-Based zkApp

**JavaScript Integration:**

```javascript
// Load WASM
const wasmModule = await WebAssembly.instantiateStreaming(
  fetch('guest.wasm')
);

// Prepare inputs
const inputs = {
  public: [10n],
  private: [secretValue]
};

// Generate proof (using zkWasm service)
const proof = await zkwasmProve(wasmModule, inputs);

// Verify locally or on-chain
const isValid = await verifyProof(proof);
```

### Pattern 2: Serverless Proving Service

**Architecture:**
```
Client → API Gateway → Lambda (zkWasm) → Store Proof → Return to Client
```

**AWS Lambda handler:**
```python
import subprocess
import json

def lambda_handler(event, context):
    # Extract inputs
    n = event['n']
    
    # Generate proof
    result = subprocess.run([
        'delphinus-cli',
        '--params', '/tmp/params',
        'demo', 'prove',
        '--wasm', '/opt/guest.wasm',
        '--output', '/tmp/output',
        '--public', f'{n}:i64'
    ], capture_output=True)
    
    # Read proof
    with open('/tmp/output/proof.json') as f:
        proof = json.load(f)
    
    return {
        'statusCode': 200,
        'body': json.dumps(proof)
    }
```

### Pattern 3: Smart Contract Integration

**Solidity Verifier:**

1. Generate verifier contract:
```bash
delphinus-cli export-verifier --output Verifier.sol
```

2. Deploy and integrate:
```solidity
pragma solidity ^0.8.0;

import "./Verifier.sol";

contract zkWasmApp {
    Verifier public verifier;
    
    constructor(address _verifier) {
        verifier = Verifier(_verifier);
    }
    
    function submitProof(
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external {
        require(
            verifier.verify(proof, publicInputs),
            "Invalid proof"
        );
        
        // Process verified computation result
        processResult(publicInputs[0]);
    }
    
    function processResult(uint256 result) internal {
        // Your business logic here
    }
}
```

### Pattern 4: Microservices Architecture

**Docker Compose:**

```yaml
version: '3.8'
services:
  zkwasm-prover:
    image: zkwasm:latest
    volumes:
      - ./params:/params
      - ./proofs:/output
    environment:
      - RUST_LOG=info
    command: |
      delphinus-cli --params /params demo prove 
        --wasm /app/guest.wasm 
        --output /output 
        --public ${INPUT}:i64
  
  zkwasm-verifier:
    image: zkwasm:latest
    volumes:
      - ./params:/params
      - ./proofs:/output
    command: |
      delphinus-cli --params /params demo verify 
        --output /output
```

## Testing and Development

### Unit Testing Guest Programs

Create a test harness:

```rust
// tests/guest_test.rs
#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        // Mock wasm_input/output for testing
        let result = fibonacci(10);
        assert_eq!(result, 89);
    }
}
```

### Integration Testing

```bash
#!/bin/bash
# integration_test.sh

# Test with various inputs
for n in 5 10 15 20; do
    echo "Testing with n=$n"
    cargo run -- prove --n $n --mock
    cargo run -- verify
done
```

## Performance Optimization

### Optimize WASM Size

```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Strip symbols
panic = "abort"        # Smaller panic handler
```

### Optimize Proving Time

```bash
# Use larger circuit for complex programs
cargo run -- setup --k 20

# Enable parallelization
export RAYON_NUM_THREADS=8
```

## Conclusion

These examples demonstrate zkWasm's versatility for:
- 🔐 Privacy-preserving computations
- ✅ Verifiable computations
- 🌐 Web-native zero-knowledge proofs
- 🔗 Smart contract integration

For more examples, see:
- [Official Examples](https://github.com/DelphinusLab/zkWasm#project-bootstrap)
- [C Template](https://github.com/DelphinusLab/zkWasm-C)
- [Browser Demo](https://github.com/zkcrossteam/g1024/)

