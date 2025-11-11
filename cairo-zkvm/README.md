# Cairo zkVM Fibonacci Demo

This directory contains a demonstration of computing Fibonacci numbers using Cairo, the zkVM developed by StarkWare.

## 📚 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Get started in under 5 minutes
- **[COMPARISON.md](COMPARISON.md)** - How Cairo differs from other zkVMs in this repo
- **[README.md](README.md)** - You are here (comprehensive guide)

## About Cairo

Cairo is a Turing-complete programming language designed specifically for creating STARK-based provable programs. It's used in StarkNet and StarkEx for generating cryptographic proofs of computation.

## Project Structure

```
cairo-zkvm/
├── src/
│   ├── fib.cairo         # Main Fibonacci implementation
│   └── fib_simple.cairo  # Simplified version for learning
└── README.md
```

## Prerequisites

### Installation

1. Install Cairo using pip:

```bash
pip install cairo-lang
```

2. Verify the installation:

```bash
cairo-compile --version
```

For more detailed installation instructions, visit the [Cairo documentation](https://www.cairo-lang.org/docs/quickstart.html).

## Running the Demo

### Method 1: Run Directly (Recommended)

This compiles and runs the Cairo program in one step:

```bash
cd cairo-zkvm
cairo-run --program=src/fib_simple.cairo --print_output --layout=small
```

### Method 2: Compile Then Run

First compile the Cairo program to JSON:

```bash
cairo-compile src/fib_simple.cairo --output fib_simple.json
```

Then run the compiled program:

```bash
cairo-run --program=fib_simple.json --print_output --layout=small
```

### Running Different Examples

To run the main implementation:

```bash
cairo-run --program=src/fib.cairo --print_output --layout=small
```

## Expected Output

When you run the program, you should see output similar to:

```
Program output:
  10
  55
```

This means: For input n=10, the 10th Fibonacci number is 55.

The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...

## Code Explanation

### fib_simple.cairo

```cairo
func fib(n: felt) -> (res: felt) {
    if (n == 0) {
        return (res=0);
    }
    if (n == 1) {
        return (res=1);
    }
    
    let (a) = fib(n - 1);
    let (b) = fib(n - 2);
    return (res=a + b);
}
```

This is a recursive implementation of the Fibonacci function. In Cairo:
- `felt` is the basic field element type (252-bit integer)
- Functions use explicit return syntax
- The recursion is proven in the STARK proof

## Modifying the Input

To compute Fibonacci for a different value of n:

1. Open `src/fib_simple.cairo`
2. Find the line `local n = 10;`
3. Change `10` to your desired value
4. Run the program again

## Generating Proofs

To generate a full STARK proof (for production use):

```bash
cairo-run \
  --program=src/fib_simple.cairo \
  --print_output \
  --print_info \
  --proof_mode \
  --layout=small
```

This will generate:
- Execution trace
- Memory values
- STARK proof data

## Resources

* [Cairo Documentation](https://www.cairo-lang.org/docs/)
* [Cairo Language Reference](https://www.cairo-lang.org/docs/reference/index.html)
* [StarkWare](https://starkware.co/)
* [Cairo Playground](https://www.cairo-lang.org/playground/)
* [Cairo Book](https://book.cairo-lang.org/)

## Comparison with Other zkVMs

Unlike other zkVMs in this repository (SP1, Risc0, Nexus, ZKM), Cairo:
- Uses its own domain-specific language (not Rust)
- Is based on STARK proofs (not SNARKs)
- Requires a different toolchain (`cairo-lang` instead of Rust)
- Has a unique memory model optimized for provable computation

## Troubleshooting

### Installation Issues

If `pip install cairo-lang` fails, try:

```bash
# On macOS with Apple Silicon
arch -x86_64 pip install cairo-lang

# Or use a virtual environment
python3 -m venv cairo-env
source cairo-env/bin/activate
pip install cairo-lang
```

### Runtime Issues

If you get "cairo-run: command not found":

```bash
# Ensure Cairo is in your PATH
which cairo-run

# Or run with full path
python -m cairo_lang.run --program=src/fib_simple.cairo --print_output
```

## Next Steps

1. Try modifying the Fibonacci input value
2. Experiment with other Cairo programs
3. Learn about Cairo's memory model
4. Explore proof generation and verification
5. Check out StarkNet for deploying Cairo programs on-chain

