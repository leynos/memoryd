# User Guide

This guide explains how to use the generated Memoryd project after rendering it
from the template.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

On Linux targets, `.cargo/config.toml` configures clang to link with `mold`
so local debug builds link quickly. Coverage generation uses `lld` instead
because LLVM coverage tools expect LLVM-compatible linker behaviour. The
pinned nightly toolchain still installs the Cranelift codegen backend, but
`.cargo/config.toml` no longer activates it by default for every build;
only the automatic activation was removed, not the component itself. It
remains available as an opt-in accelerated build via
`tools/dev-fast/config.toml` and the `make dev-build`/`make dev-test`
targets below, so ordinary builds, coverage, and verification keep the
supported LLVM backend.

## Makefile Targets

The generated `Makefile` exposes these public targets:

- `make all` runs formatting checks, linting, and tests.
- `make check-fmt` verifies Rust formatting.
- `make lint` runs rustdoc, Clippy, and Whitaker with warnings denied.
- `make test` runs `cargo nextest run` when cargo-nextest is installed and
  falls back to `cargo test` otherwise. Library projects also run doctests.
- `make build` builds the debug target.
- `make release` builds the release target.
- `make coverage` writes `lcov.info` using `cargo llvm-cov` and `lld`.
- `make dev-build` and `make dev-test` build and test with the opt-in
  Cranelift-plus-mold configuration in `tools/dev-fast/config.toml`.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

Install `clang`, `lld`, and `mold` before running the full generated workflow
locally on Linux.
