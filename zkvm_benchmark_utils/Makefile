# zkVM Benchmark Makefile
# ========================
# Convenient targets to run benchmarks for different zkVMs

# Default output directory
OUTPUT_DIR ?= benchmark-results

# Optional: programs filter (e.g., "fibonacci,hash")
PROGRAMS ?=

# Optional: scales filter (e.g., "10,20,100")
SCALES ?=

# Build arguments
CARGO_RUN = cargo run --release --bin zkvm-benchmark --

# Build the base command with optional filters (always outputs CSV)
define BENCH_CMD
$(CARGO_RUN) run --output $(OUTPUT_DIR) --report-formats csv \
	$(if $(PROGRAMS),--programs $(PROGRAMS)) \
	$(if $(SCALES),--scales $(SCALES))
endef

# ============================================================================
##@ General
# ============================================================================

.PHONY: help
help: ## Display this help
	@awk 'BEGIN {FS = ":.*##"; printf "\n\033[1m╔══════════════════════════════════════════════════════════════════╗\033[0m\n\033[1m║           zkVM Benchmark Makefile                                ║\033[0m\n\033[1m╚══════════════════════════════════════════════════════════════════╝\033[0m\n\nUsage:\n  make \033[36m<target>\033[0m [OPTIONS]\n\nOptions:\n  OUTPUT_DIR=<path>       Output directory (default: benchmark-results)\n  PROGRAMS=<list>         Programs to run (comma-separated)\n  SCALES=<list>           Scales to test (comma-separated)\n\nNote: All benchmarks output CSV format.\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }' $(MAKEFILE_LIST)
	@echo ""
	@echo "Examples:"
	@echo "  make bench_sp1"
	@echo "  make bench_risc0 PROGRAMS=fibonacci SCALES=10,20"
	@echo "  make bench_all OUTPUT_DIR=./results"
	@echo ""

# ============================================================================
##@ Build
# ============================================================================

.PHONY: build clean

build: ## Build the benchmark tool (release)
	cargo build --release --bin zkvm-benchmark

clean: ## Clean build artifacts and results
	cargo clean
	rm -rf $(OUTPUT_DIR)

# ============================================================================
##@ Benchmark All
# ============================================================================

.PHONY: bench_all

bench_all: ## Run benchmarks for all enabled zkVMs
	$(BENCH_CMD)

# ============================================================================
##@ Individual zkVM Benchmarks
# ============================================================================

.PHONY: bench_airbender bench_cairo bench_cairom bench_ceno bench_jolt bench_lean
.PHONY: bench_miden bench_nexus bench_novanet bench_o1vm bench_openvm bench_pico
.PHONY: bench_powdr bench_risc0 bench_snarkvm bench_sp1 bench_valida bench_zisk
.PHONY: bench_zkm bench_zkwasm

bench_airbender: ## Benchmark Airbender zkVM
	$(BENCH_CMD) --zkvms airbender

bench_cairo: ## Benchmark Cairo zkVM
	$(BENCH_CMD) --zkvms cairo

bench_cairom: ## Benchmark Cairo-M zkVM
	$(BENCH_CMD) --zkvms cairom

bench_ceno: ## Benchmark Ceno zkVM
	$(BENCH_CMD) --zkvms ceno

bench_jolt: ## Benchmark Jolt zkVM
	$(BENCH_CMD) --zkvms jolt

bench_lean: ## Benchmark Lean zkVM
	$(BENCH_CMD) --zkvms lean

bench_miden: ## Benchmark Miden zkVM
	$(BENCH_CMD) --zkvms miden

bench_nexus: ## Benchmark Nexus zkVM
	$(BENCH_CMD) --zkvms nexus

bench_novanet: ## Benchmark NovaNet zkVM
	$(BENCH_CMD) --zkvms novanet

bench_o1vm: ## Benchmark o1VM zkVM
	$(BENCH_CMD) --zkvms o1vm

bench_openvm: ## Benchmark OpenVM zkVM
	$(BENCH_CMD) --zkvms openvm

bench_pico: ## Benchmark Pico zkVM
	$(BENCH_CMD) --zkvms pico

bench_powdr: ## Benchmark Powdr zkVM
	$(BENCH_CMD) --zkvms powdr

bench_risc0: ## Benchmark RISC0 zkVM
	$(BENCH_CMD) --zkvms risc0

bench_snarkvm: ## Benchmark SnarkVM
	$(BENCH_CMD) --zkvms snarkvm

bench_sp1: ## Benchmark SP1 zkVM
	$(BENCH_CMD) --zkvms sp1

bench_valida: ## Benchmark Valida zkVM
	$(BENCH_CMD) --zkvms valida

bench_zisk: ## Benchmark Zisk zkVM
	$(BENCH_CMD) --zkvms zisk

bench_zkm: ## Benchmark ZKM zkVM
	$(BENCH_CMD) --zkvms zkm

bench_zkwasm: ## Benchmark zkWasm zkVM
	$(BENCH_CMD) --zkvms zkwasm

# ============================================================================
##@ Group Benchmarks
# ============================================================================

.PHONY: bench_risc_based bench_wasm_based bench_custom_vm bench_fast

bench_risc_based: ## Run all RISC-V based zkVMs
	$(BENCH_CMD) --zkvms sp1,risc0,jolt,ceno,nexus,openvm,pico,zkm,novanet,lean,zisk,powdr

bench_wasm_based: ## Run WebAssembly based zkVMs
	$(BENCH_CMD) --zkvms zkwasm

bench_custom_vm: ## Run custom VM architectures
	$(BENCH_CMD) --zkvms miden,cairo,cairom,valida,snarkvm,airbender,o1vm

bench_fast: ## Quick benchmarks (sp1,risc0,jolt with scale=10)
	$(BENCH_CMD) --zkvms sp1,risc0,jolt --scales 10

# ============================================================================
##@ Utilities
# ============================================================================

.PHONY: list_configs check_configs

list_configs: ## List all available zkVM configs
	@echo "Available zkVM configurations:"
	@ls -1 configs/*.toml | xargs -I {} basename {} .toml | grep -v template

check_configs: ## Check which zkVMs are enabled
	@echo "Checking enabled zkVMs..."
	@for config in configs/*.toml; do \
		name=$$(basename $$config .toml); \
		if [ "$$name" != "template" ]; then \
			enabled=$$(grep -E "^enabled\s*=" $$config | head -1 | grep -o "true\|false"); \
			if [ "$$enabled" = "true" ]; then \
				echo "  ✅ $$name"; \
			else \
				echo "  ❌ $$name (disabled)"; \
			fi \
		fi \
	done
