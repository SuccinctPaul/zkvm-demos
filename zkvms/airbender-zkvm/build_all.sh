#!/bin/bash
set -e

# Get absolute path to linker script
LINKER_SCRIPT="$(pwd)/airbender-guest/linker.ld"

echo "Building Airbender Guest..."
RUSTFLAGS="-C link-arg=-T${LINKER_SCRIPT}" cargo build --release -p airbender-guest --target riscv32im-unknown-none-elf

echo "Generating flat binary..."
# Use rust-objcopy directly to avoid cargo re-checking dependencies which triggers false positives for std
rust-objcopy -O binary target/riscv32im-unknown-none-elf/release/airbender-guest target/riscv32im-unknown-none-elf/release/airbender-guest.bin

echo "Building Airbender Host..."
# Host build doesn't need the linker script
cargo build --release -p airbender-host
