.PHONY: help all clean test build release coverage lint fmt fmt-tools check-fmt markdownlint nixie


TARGET ?= memoryd

USER_WHITAKER := $(HOME)/.local/bin/whitaker
USER_BIN_PATH := $(HOME)/.cargo/bin:$(HOME)/.local/bin:$(HOME)/.bun/bin
CARGO ?= cargo
BUILD_JOBS ?=
RUST_FLAGS ?=
RUST_FLAGS := -D warnings $(RUST_FLAGS)
RUSTDOC_FLAGS ?=
RUSTDOC_FLAGS := -D warnings $(RUSTDOC_FLAGS)
CARGO_FLAGS ?= --all-targets --all-features
CLIPPY_FLAGS ?= $(CARGO_FLAGS) -- $(RUST_FLAGS)
TEST_FLAGS ?= $(CARGO_FLAGS)
TEST_CMD := $(if $(shell $(CARGO) nextest --version 2>/dev/null),nextest run,test)
COVERAGE_LINKER_FLAGS ?= -fuse-ld=lld
COVERAGE_RUST_FLAGS ?= $(RUST_FLAGS) -C link-arg=$(COVERAGE_LINKER_FLAGS)
MDLINT ?= markdownlint-cli2
MDFORMAT_ALL ?= mdformat-all
NIXIE ?= nixie
WHITAKER ?= $(or $(shell command -v whitaker 2>/dev/null),$(wildcard $(USER_WHITAKER)),whitaker)

build: target/debug/$(TARGET) ## Build debug binary
release: target/release/$(TARGET) ## Build release binary

all: check-fmt lint test ## Perform a comprehensive check of code

clean: ## Remove build artifacts
	$(CARGO) clean

test: ## Run tests with warnings treated as errors
	RUSTFLAGS="$(RUST_FLAGS)" $(CARGO) --config "$(DEV_FAST_CONFIG)" $(TEST_CMD) $(TEST_FLAGS) $(BUILD_JOBS)


target/%/$(TARGET): ## Build binary in debug or release mode
	$(CARGO) $(if $(findstring release,$(@)),,--config "$(DEV_FAST_CONFIG)") build $(BUILD_JOBS) $(if $(findstring release,$(@)),--release) --bin $(TARGET)

coverage: ## Generate lcov coverage with lld for llvm-tools compatibility
	@echo "coverage linker flags: $(COVERAGE_LINKER_FLAGS)"
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang \
		RUSTFLAGS="$(COVERAGE_RUST_FLAGS)" \
		CFLAGS="$(COVERAGE_LINKER_FLAGS)" \
		LDFLAGS="$(COVERAGE_LINKER_FLAGS)" \
		$(CARGO) llvm-cov --lcov --output-path lcov.info $(TEST_FLAGS)

lint: ## Run Clippy with warnings denied
	RUSTDOCFLAGS="$(RUSTDOC_FLAGS)" $(CARGO) --config "$(DEV_FAST_CONFIG)" doc --no-deps
	$(CARGO) --config "$(DEV_FAST_CONFIG)" clippy $(CLIPPY_FLAGS)
	@echo "Whitaker binary: $(WHITAKER)"
	PATH="$(USER_BIN_PATH):$(PATH)" RUSTFLAGS="$(RUST_FLAGS)" $(WHITAKER) --all -- $(CARGO_FLAGS)

typecheck: ## Type-check without building
	RUSTFLAGS="$(RUST_FLAGS)" $(CARGO) --config "$(DEV_FAST_CONFIG)" check $(CARGO_FLAGS)

fmt: fmt-tools ## Format Rust and Markdown sources
	$(CARGO) +nightly fmt --all
	$(MDFORMAT_ALL)

fmt-tools: ## Verify Markdown formatting tools are installed
	@command -v $(MDFORMAT_ALL) >/dev/null || { echo "Install $(MDFORMAT_ALL) from agent-helper-scripts"; exit 1; }

check-fmt: ## Verify formatting
	$(CARGO) fmt --all -- --check

markdownlint: ## Lint Markdown files
	$(MDLINT) '**/*.md'

nixie: ## Validate Mermaid diagrams
	$(NIXIE) --no-sandbox

help: ## Show available targets
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS=":"; printf "Available targets:\n"} {printf "  %-20s %s\n", $$1, $$2}'

# Opt-in accelerated debug builds (Cranelift + mold); requires a nightly
# toolchain. See AGENTS.md and tools/dev-fast/config.toml.
DEV_FAST_CONFIG ?= tools/dev-fast/config.toml

.PHONY: dev-build dev-test
dev-build: ## Build debug binaries with Cranelift and mold
	cargo --config "$(DEV_FAST_CONFIG)" build

dev-test: ## Run tests with Cranelift and mold
	cargo --config "$(DEV_FAST_CONFIG)" test
