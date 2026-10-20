# User Guide

This guide explains how to use the generated Memoryd project after rendering it
from the template.

## Local safety defaults (planned)

These defaults are accepted policy in
[ADR 015](adr-015-local-safety-defaults.md) but are **not yet enforced**: the
daemon, collector, and MCP server do not implement workspace derivation,
redaction, deny patterns, or purge. Treat this section as the target contract
for planned behaviour; the build and test commands later in this guide are the
tooling that actually runs today.

### What a workspace is

A workspace is a tenant-owned evidence namespace. Its authoritative identity is
the pair of authenticated tenant and workspace; a workspace name on its own is
never authority. Memoryd derives a workspace from a versioned descriptor:

- for a Git repository, the normalized `remote.origin.url`, a digest of the
  canonical local repository root path, and an optional profile name;
- for a non-Git directory, a digest of the canonical root path and the profile
  name.

Consequences to plan for:

- **A moved root is a new workspace.** Renaming or moving the local directory
  changes the derived workspace, and existing evidence stays with the old one.
  Use an explicit operator mapping when the two must line up.
- **A changed origin is a new workspace.** Changing `remote.origin.url`, or
  switching between SSH and HTTPS forms of the same repository, yields a
  different workspace unless an operator maps them explicitly.
- **Linked worktrees are separate workspaces.** Git's shared common directory
  does not merge them, so a checkout and its linked worktree do not share
  evidence by default.
- **Cross-machine identity depends on the path, not the machine.** The root
  path is part of the identity, so different canonical paths always give
  different workspaces. Two different machines are not separated by themselves:
  if the same repository is cloned to the *same* canonical path under the same
  tenant, origin, and profile, the two clones derive the same descriptor and
  register as one workspace. Nothing in the identity records which machine it
  came from. To keep two machines separate, give them different canonical
  paths; to share evidence between divergent paths deliberately, use an
  explicit operator mapping rather than relying on the paths happening to match.

Operators can map an exact descriptor to a chosen workspace identifier, scoped
to one tenant. An intentional merge requires listing the descriptors
explicitly. Memoryd never learns a merge from provider payloads, and a digest
collision is an error rather than an automatic merge.

### What is excluded and redacted

Redaction runs before storage and before embedding, and neither can be disabled
in the first release. The planned default configuration is:

```toml
[privacy]
redact_before_store = true
redact_before_embedding = true
store_raw_text = "redacted" # none | redacted | encrypted
```

`redacted` is the default and keeps sanitized text for search and explanation.
`none` keeps identities and minimal structural metadata but drops all free-text
payloads, so text-derived recall and summaries are unavailable for that
workspace. `encrypted` is **rejected as unsupported** until a separate
key-management decision exists; selecting it is a configuration error, not a
silent fallback. There is no plaintext mode.

The first detector corpus covers provider API keys, OAuth bearer and basic
values, JSON Web Tokens, PEM and OpenSSH private-key blocks, dotenv and
password assignments, cookies, cloud credentials, credential-bearing database
URLs, and high-entropy blobs. Known secret prefixes are redacted aggressively,
so a short or malformed token after a known prefix is removed too. Expect false
positives: a random-looking 32-byte string can become `[REDACTED]` when it is
not a secret. Redaction is a mitigation, not a guarantee, so do not treat
Memoryd as making a sensitive workspace safe to store.

A redacted value is a marker, not a masked copy of the original: inside JSON it
is the string `"[REDACTED]"`, even where the value it replaces was an object or
an array. Whole-value replacement avoids retaining the original value's
internal structure, which can itself disclose sensitive information, so do not
write code that depends on the JSON type of a field redaction may have touched,
and compare the marker by value rather than by raw bytes.

These paths are excluded by default and cannot be re-enabled in the first
release: `.env`, `.env.*`, `**/.env`, `**/.env.*`, `.git/**`, `**/.git/**`,
`.ssh/**`, and `**/.ssh/**`. The `.ssh` exclusion deliberately covers public
keys as well as private ones. Operators may add exclusions but not remove the
mandatory defaults. Deny patterns are case-sensitive, root-relative globs where
`*` stops at a path separator and `**` spans directories; a match always denies.

Always check whether the thing was excluded on purpose: an **intentional
exclusion** — a mandatory default exclusion, a deny-pattern match, or your own
audited skip of one source position — is recorded as deliberately skipped and
does advance the cursor, because re-reading it would only be rejected again. A
**processing failure** — a failed redaction, a detector or policy error, a
breached processing ceiling, or a store error — is not stored, partially or
otherwise, its source cursor does not advance, and it surfaces as a
source-health error. An operator can then fix the source or explicitly skip
that one source position; skipping is audited and is never automatic.

### Purging a workspace

Purge is irreversible and disabled by default. Read-only MCP mode exposes no
purge path, and tenant-wide purge stays administratively disabled in local
mode. A workspace purge requires an authenticated tenant, an allowed workspace,
and a purge capability before Memoryd will even show a preview.

The preview names the tenant, the workspace, the active stores that will be
affected, and the backup limitations. Interactive confirmation requires
retyping the complete displayed scope; empty input, end of input, a mismatch,
or cancellation changes nothing. Non-interactive use requires an explicit scope
and a confirmation bound to that exact preview.

Before purging you must acknowledge that a restorable backup was taken. Stop
writers for the scope before the backup and leave them stopped through the
purge; a write in between invalidates the acknowledgement and requires a fresh
backup and preview. Back up every active store for the scope:

- Use SQLite's online backup API or `.backup` command rather than copying the
  database file while writers run, and a consistent custom-format `pg_dump` for
  PostgreSQL.
- Where graph storage exists, back up its directory under that store's own
  documented quiescence procedure. Graph and theme stores can be absent from a
  minimum deployment; their absence is neither an error nor a reason to start
  those services.
- Separate SQL and graph backups are not an atomic snapshot of each other.
  Sequence them under the same quiesced window and never describe the result as
  a single coordinated point-in-time copy.

There is no automatic backup and no automatic undo.

What purge cannot do: it does not securely erase filesystem blocks, it does not
delete the provider's original transcripts, and it does not reach backups you
made earlier. A pre-purge backup still contains the removed evidence, so its
access controls, retention, and deletion are your responsibility. After a
successful purge, Memoryd keeps a content-free tombstone for that workspace and
blocks re-collection until an operator explicitly re-enables the scope; that
re-enabling is a separate action, not a side effect of a retry or a restart. If
a purge cannot finish, it reports the incomplete cleanup without exposing
payloads, and retrying over the same scope is safe to repeat.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

See the [developers' guide](developers-guide.md) for the local build and linker
configuration.

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
  variants. They use the Cranelift backend, and also `-Zthreads=8` and, on
  Linux, the `mold` linker, like the other development targets; see the
  [developers' guide](developers-guide.md) for details.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

See the [developers' guide](developers-guide.md) for the toolchain
prerequisites needed to run the full generated workflow locally.

## Building from source

The repository's Cargo defaults and the Makefile development targets
(`make test`, `make lint`, `make typecheck` and the debug build) use the
parallel `rustc` frontend (`-Zthreads=8`) and, on Linux, the `mold` linker. On
Linux, install `mold` and `clang` before building, because the configuration
names them and a build without them fails at link time. A release build
(`make release`) and the coverage build use neither flag, because an assigned
`RUSTFLAGS` displaces the configuration's flags. `make release` assigns
`RUSTFLAGS="${RUSTFLAGS-}"`, which passes the caller's value through unchanged
and adds nothing, so a shipped artefact keeps the platform linker. The coverage
build assigns its own flags and ignores caller-provided flags. To build with
the platform linker directly, assign an empty `RUSTFLAGS`:

```bash
RUSTFLAGS="" cargo build --release
```
