#!/bin/bash
# Generate input.bin for ZisK zkVM benchmark
# Usage: ./generate_input.sh [program_id] [n]

PROGRAM_ID=${PROGRAM_ID:-0}
N=${PROGRAM_N:-${FIBONACCI_N:-10}}

mkdir -p build

# Create input.bin with two little-endian u32 values: [program_id, n]
printf "$(printf '\\x%02x\\x%02x\\x%02x\\x%02x' $((PROGRAM_ID & 0xFF)) $(((PROGRAM_ID >> 8) & 0xFF)) $(((PROGRAM_ID >> 16) & 0xFF)) $(((PROGRAM_ID >> 24) & 0xFF)))" > build/input.bin
printf "$(printf '\\x%02x\\x%02x\\x%02x\\x%02x' $((N & 0xFF)) $(((N >> 8) & 0xFF)) $(((N >> 16) & 0xFF)) $(((N >> 24) & 0xFF)))" >> build/input.bin

echo "Generated input.bin with program_id=$PROGRAM_ID, n=$N"

