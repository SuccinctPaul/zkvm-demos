# Cairo 2.x Examples

This directory contains additional Cairo examples demonstrating various features and patterns.

## Available Examples

### 1. Batch Fibonacci (`batch_fib.cairo`)

Computes multiple Fibonacci numbers in a single execution.

**Features:**
- Array manipulation
- Loop constructs
- Efficient batch computation

**Usage:**
```bash
# Copy to src/main.cairo and run
cp examples/batch_fib.cairo src/main.cairo
scarb cairo-run --available-gas=300000000
```

**Expected Output:**
```
[0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

## Creating Your Own Examples

### Step 1: Create a new file

```bash
touch examples/my_example.cairo
```

### Step 2: Write your code

```cairo
use cairo_fibonacci::fib_iterative;

fn main() -> felt252 {
    // Your implementation here
    fib_iterative(20)
}
```

### Step 3: Run it

```bash
# Replace main.cairo temporarily
cp examples/my_example.cairo src/main.cairo
scarb build
scarb cairo-run --available-gas=200000000

# Or create a separate crate
scarb new my_example
```

## Common Patterns

### Pattern 1: Simple Computation

```cairo
fn main() -> felt252 {
    let x = 10;
    let y = 20;
    x + y
}
```

### Pattern 2: Multiple Outputs

```cairo
fn main() -> (felt252, felt252, felt252) {
    let a = 1;
    let b = 2;
    let c = a + b;
    (a, b, c)
}
```

### Pattern 3: Array Construction

```cairo
fn main() -> Array<felt252> {
    let mut arr = ArrayTrait::new();
    arr.append(1);
    arr.append(2);
    arr.append(3);
    arr
}
```

### Pattern 4: Conditional Logic

```cairo
fn main() -> felt252 {
    let n = 15;
    if n > 10 {
        fib_iterative(n)
    } else {
        fib_recursive(n)
    }
}
```

### Pattern 5: Loop Construction

```cairo
fn main() -> felt252 {
    let mut sum = 0;
    let mut i = 0;
    
    loop {
        if i > 10 {
            break sum;
        }
        sum += i;
        i += 1;
    }
}
```

## Tips for Writing Cairo Programs

### 1. Type Annotations

Always use explicit types:
```cairo
let x: felt252 = 42;  // Good
let x = 42;           // Also OK, type inferred
```

### 2. Mutability

Declare mutability explicitly:
```cairo
let mut count = 0;    // Mutable
count += 1;           // OK

let value = 5;        // Immutable
// value += 1;        // Error!
```

### 3. Return Values

Use explicit return or implicit return:
```cairo
fn explicit_return() -> felt252 {
    return 42;
}

fn implicit_return() -> felt252 {
    42  // No semicolon!
}
```

### 4. Gas Management

Estimate gas requirements:
```cairo
// Simple computation: ~100K gas
// Loop with 10 iterations: ~500K gas
// Recursive calls: varies exponentially
```

### 5. Debugging

Use assertions for debugging:
```cairo
let result = fib_iterative(10);
assert(result == 55, 'fib(10) should be 55');
```

## Performance Guidelines

| Operation | Gas Cost | Notes |
|-----------|----------|-------|
| Addition | ~1 | Field addition |
| Multiplication | ~1 | Field multiplication |
| Function call | ~10 | Overhead |
| Loop iteration | ~5 | Per iteration |
| Array append | ~10 | Memory allocation |
| Recursive call | Variable | Avoid deep recursion |

## Testing Examples

Add tests to your examples:

```cairo
#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_main() {
        let result = main();
        assert(result == expected_value, 'test failed');
    }
}
```

Run tests:
```bash
scarb test
```

## Next Steps

1. Try modifying existing examples
2. Create your own examples
3. Experiment with different patterns
4. Optimize for gas efficiency
5. Add comprehensive tests

## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Cairo by Example](https://cairo-by-example.com/)
- [StarkNet Documentation](https://docs.starknet.io/)

---

Happy coding with Cairo! 🚀

