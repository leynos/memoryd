# User Guide

This guide explains how to use the generated Memoryd project after rendering it
from the template.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

See the [developers' guide](developers-guide.md) for the local build and
linker configuration.

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
- `make dev-build` and `make dev-test` provide opt-in accelerated build
  variants; see the [developers' guide](developers-guide.md) for details.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

See the [developers' guide](developers-guide.md) for the toolchain
prerequisites needed to run the full generated workflow locally.
