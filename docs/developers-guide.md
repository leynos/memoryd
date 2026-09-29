# Developer Guide

This guide explains the contributor workflow for the generated Memoryd project.

## Local Workflow

Use `make all` as the public entrypoint for formatting, linting, and tests.
`make lint` runs rustdoc, Clippy, and Whitaker. `make test` prefers
`cargo nextest run` and falls back to `cargo test` when cargo-nextest is not
available. `make coverage` uses `cargo llvm-cov` with `lld`. `make dev-build`
and `make dev-test` are opt-in accelerated variants; see below.

## Tooling

Development builds use the build standard described below: `-Zthreads=8` and,
on Linux, clang linking with `mold`, from `.cargo/config.toml`. The standard
make targets also select the Cranelift backend by passing
`--config tools/dev-fast/config.toml`, as do `make dev-build` and
`make dev-test`; that fragment holds only the backend selection and is never
applied to release, coverage, or verification builds (see
[Fast development builds](../AGENTS.md#fast-development-builds) in
`AGENTS.md`). Coverage generation uses `lld` and LLVM because coverage tooling
expects them. The pinned nightly toolchain retains the `llvm-tools-preview` and
`rustc-codegen-cranelift-preview` components, so both paths have what they need
installed.

Install `clang`, `lld`, and `mold` before running the full generated workflow
locally on Linux.

## The build standard

Development, test, lint, and typecheck builds use the parallel `rustc` frontend
(`-Zthreads=8`) and, on Linux, the `mold` linker (`-Clink-arg=-fuse-ld=mold`).
These are defaults in `.cargo/config.toml`, which Cargo discovers on its own,
so a bare `cargo build` gets them. `mold` ships for Linux only, so the linker
flag lives in a Linux-only table and macOS and Windows keep their platform
linker. Cargo selects one `rustflags` source rather than merging them, so every
source repeats the same flags apart from the linker.

An assigned `RUSTFLAGS` replaces the configuration's flags, so the Makefile
recipes that set it compose the standard's flags onto any inherited value (CI's
`setup-rust` exports one). Two builds are deliberately excluded: coverage
assigns `RUSTFLAGS` without the fast flags, because a measurement should not
depend on them, and the release recipe and workflow keep the platform linker,
because they assign `RUSTFLAGS` (even an empty value displaces the
configuration). Cargo has no per-profile `rustflags`, so a direct
`cargo build --release` takes the configuration's flags unless `RUSTFLAGS` is
assigned too.

On Linux, install `mold` before building: the configuration names it, so a
build without it fails at link time. CI installs it through `setup-rust`'s
`install-mold` input. `tests/build_standard_contract.rs` holds the standard. It
reads the configuration sources, and the commands `make -n` prints for each
development target on a Linux host and a macOS host, and for each coverage and
release target on a Linux host, so a flag lost through a recipe edit fails
there.

### Cranelift

Exception: Cranelift is not the default backend in `.cargo/config.toml`. The
repository pins `nightly-2026-05-21`, but the release workflow
(`cross +stable build --release`) builds on a stable toolchain against
`.cargo/config.toml`, and stable Cargo refuses a
`[profile.dev] codegen-backend` key ("config profile `dev` is not valid"), so a
backend selected there would break every release (recorded 2026-09-29). The
standard make targets select Cranelift instead, by passing
`--config tools/dev-fast/config.toml`, which holds only the backend selection;
coverage and release builds never pass it. A contract fails if a
`codegen-backend` key reaches `.cargo/config.toml` while the release builds on
`+stable`. `tests/stable_cargo_config.rs` also asks stable Cargo itself, through
`rustup run stable cargo build --release --bin no-such-bin`: stable Cargo
resolves every configured profile before it looks up the target, so a refused
configuration and an accepted one differ in the message, and nothing compiles.
The probe must run on stable, because a nightly Cargo accepts a backend that
stable refuses. The test therefore needs the stable toolchain installed
(`rustup toolchain install stable --profile minimal`); CI installs it before
the tests run. Revisit if the release moves to the pinned nightly.

## Lint baseline

Memoryd is a single crate with no `[workspace]` table, so its lint tables live
directly under `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]` in
`Cargo.toml`, rather than under `[workspace.lints]` with per-member
inheritance. They implement the estate's phase 2 Rust baseline. `Cargo.toml` is
authoritative for the exact set and level of each lint; this section summarizes
intent rather than duplicating the list.

Violations must be fixed, not silenced. Where a violation is a genuine,
scheduled deferral, annotate the site with
`#[expect(clippy::<lint>, reason = "...")]`, never `#[allow(...)]`: once the
site is fixed, the unfulfilled expectation itself becomes a warning, so the
backlog announces its own shrinkage instead of rotting silently.

`clippy.toml` carries the numeric thresholds (cognitive complexity, argument
count, function length, nesting depth) and the `disallowed-methods` list that
forbids direct `std::env::var`/`set_var`/ `remove_var` calls and their `_os`/
`vars` siblings; inject an environment reader instead so environment access
stays testable.

The pinned nightly toolchain in `rust-toolchain.toml` supplies the `rustfmt`,
`clippy`, and `rust-analyzer` components the lint and formatting gates depend
on.

Behavioural tests that describe externally observable workflows should use
`rstest-bdd` so Gherkin scenarios, `rstest` fixtures, and Rust assertions run
under the standard Cargo test harness. PostgreSQL migration and repository
tests for the evidence store must prefer `POSTGRES_TEST_URL` when an external
database is configured, and otherwise use `pg_embedded_setup_unpriv` when the
local host satisfies its prerequisites. See
[rstest-bdd user's guide](rstest-bdd-users-guide.md) and
[pg_embedded_setup_unpriv user guide](pg-embed-setup-unpriv-users-guide.md) for
the detailed setup and test-support APIs.

## Interface Conventions

Application services expose domain use cases through port traits. Keep those
traits in `memoryd` language: request context, tenant, workspace, evidence,
source session, recall, projection, and health types belong in the core;
Qdrant, Ollama, Oxigraph, Chutoro, filesystem, transport, and MCP software
development kit (SDK) types belong in adapters.

Driving adapters are responsible for translating external requests into domain
commands. Driven adapters are responsible for translating domain port calls
into infrastructure operations. Neither adapter family should make
memory-policy decisions that belong in the domain or application layer.

Evidence-store adapters follow
[Architecture Decision Record (ADR) 013](adr-013-evidence-store-engine-and-migration-policy.md):
SQLite is the local default, PostgreSQL is a first-class deployment path,
Diesel remains behind adapter boundaries, and paired SQLite and PostgreSQL
migrations must preserve one logical evidence-store contract. Domain ports must
not expose Diesel, SQL connection, migration harness, or
`pg_embedded_setup_unpriv` types.

## Error Contracts

Use semantic error types inside libraries and convert them to structured
transport errors at the boundary. Do not expose `eyre::Report`, raw provider
errors, SQL errors, Qdrant errors, Ollama errors, filesystem paths, or
unredacted payload snippets through public MCP or RPC responses.

MCP and internal RPC responses should use a stable error envelope containing:

- a machine-readable error code;
- a human-readable summary safe for logs and clients;
- the request ID and correlation ID where available;
- whether the caller may retry the operation;
- the envelope kind, method, tenant, and workspace scope when safe to expose;
- optional redacted details for validation, compatibility, or dependency
  failures.

Prefer specific error codes over string matching. The first implementation
should distinguish at least validation errors, unauthenticated requests,
unauthorized capability use, tenant or workspace scope violations, not found
results, conflicts, idempotency replays, incompatible schema features,
dependency unavailable states, and internal errors.

Authorization and tenant-scope failures must be audited with the authenticated
tenant context when one exists. Missing authentication must not be converted
into a default tenant unless the caller is explicitly running through the local
single-user configuration path.

## Pagination Contracts

All list, browse, and transcript-like reads must be bounded. Do not add
unbounded `list_all` style methods to ports, RPC, or MCP tools. Session
listing, source-event browsing, source-health history, recall audit browsing,
and future claim or projection browsing should use cursor-based pagination.

Pagination requests should include a caller-supplied limit subject to a
server-side maximum. Pagination responses should include the returned items, an
opaque next cursor when more data exists, and enough stable ordering metadata
for tests to prove deterministic replay.

Cursors are tenant-scoped capabilities, not trusted query predicates. They must
be derived from domain state or adapter state, validated before use, and
intersected with the authenticated request context. A cursor from one tenant,
workspace, provider, or filter set must not page through another tenant's data.

Adapters may keep provider-specific cursor material such as file offsets, line
numbers, source hashes, or database watermarks. That material stays behind the
`ConversationSourcePort` or persistence adapter boundary unless the domain
needs it as redacted evidence metadata.

## Authentication and Authorization

Every tenant-owned command, query, and scheduled operation must carry or derive
a `RequestContext` before application logic runs. The context is established
from an authenticated capability token, Corbusier request context, scheduled
job record, or configured local default. Tenant or workspace identifiers found
inside provider logs, MCP request bodies, or import files may narrow a request,
but they must not establish authority by themselves.

Capability checks happen at the driving-adapter boundary and are preserved in
the application command. Internal RPC envelopes carry the required capability
scope, envelope kind, method, schema version, request context, accepted
features, and idempotency key where the method mutates state. Unknown required
features fail with a compatibility error before the method runs.

Workspace filters supplied by callers are always intersected with the allowed
workspace set from the authenticated context. Read-only mode disables write
tools at the MCP layer, but daemon-side capability checks remain authoritative.
`memory.purge` requires a high-privilege tenant-bound token and an explicit
confirmation string for the target tenant workspace.

Tests for new ports or adapters should include negative fixtures for missing
context, mismatched tenant and workspace, insufficient capability, reused
cursor from another scope, and cross-tenant read or write attempts.
