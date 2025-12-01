# Makefile for compiling C programs to MIPS
# Requires MIPS cross-compiler toolchain

# MIPS cross-compiler
CC = mips-linux-gnu-gcc
OBJCOPY = mips-linux-gnu-objcopy

# Compiler flags
CFLAGS = -O2 -static -nostdlib -fno-builtin -march=mips32

# Source files
SRC = main.c

# Output files
ELF = guest.elf
BIN = guest.bin

.PHONY: all clean

all: $(ELF) $(BIN)

$(ELF): $(SRC)
	$(CC) $(CFLAGS) -o $@ $<

$(BIN): $(ELF)
	$(OBJCOPY) -O binary $< $@

clean:
	rm -f $(ELF) $(BIN)

help:
	@echo "Makefile for compiling C programs to MIPS"
	@echo ""
	@echo "Targets:"
	@echo "  all    - Build MIPS ELF and binary files"
	@echo "  clean  - Remove generated files"
	@echo "  help   - Show this help message"
