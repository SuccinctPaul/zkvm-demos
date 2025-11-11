# Cairo zkVM Quick Start Guide

Get started with Cairo in under 5 minutes!

## Installation

### Option 1: Using the Installation Script (Recommended)

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos
./scripts/sdk_installers/install_cairo_sdk.sh
```

### Option 2: Manual Installation

```bash
pip install cairo-lang
```

Verify installation:

```bash
cairo-compile --version
```

## Running Your First Program

### Hello World

```bash
cd cairo-zkvm
cairo-run --program=src/hello.cairo --print_output --layout=small
```

Expected output: `42`

### Fibonacci Demo

```bash
cairo-run --program=src/fib_simple.cairo --print_output --layout=small
```

Expected output:
```
Program output:
  10
  55
```

This calculates the 10th Fibonacci number, which is 55.

## Understanding the Output

Cairo programs can output values using `serialize_word`. The output shows:
1. First line: Input value (n = 10)
2. Second line: Result (fib(10) = 55)

## Modifying the Input

1. Open `src/fib_simple.cairo`
2. Find the line: `local n = 10;`
3. Change 10 to any value (e.g., `local n = 15;`)
4. Run again

Example:
```cairo
local n = 15;  // Calculate 15th Fibonacci number
```

## Next Steps

- Read the full [README.md](README.md) for more details
- Explore the [Cairo documentation](https://www.cairo-lang.org/docs/)
- Try the [Cairo Book](https://book.cairo-lang.org/)
- Experiment with the [Cairo Playground](https://www.cairo-lang.org/playground/)

## Troubleshooting

### Command not found

If you get "cairo-run: command not found", try:

```bash
python3 -m cairo_lang.run --program=src/fib_simple.cairo --print_output
```

### Installation issues on macOS with Apple Silicon

```bash
arch -x86_64 pip install cairo-lang
```

### Need help?

Check the [README.md](README.md) for detailed troubleshooting and more information.


