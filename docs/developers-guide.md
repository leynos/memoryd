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
reads the configuration sources, the commands `make -n` prints for each
development target on a Linux host and a macOS host (each keeping the caller's
own `RUSTFLAGS`) and for each coverage and release target on a Linux host, and
the `setup-rust` steps of the CI workflows (each must pass `install-mold`), so
a flag lost through a recipe or workflow edit fails there.

CI also installs `clang` and `lld` through `setup-rust`'s `install-clang-lld`
input, which installs both on Linux and fails the job unless `clang` and
`ld.lld` resolve on `PATH`; the workflows carry no hand-rolled `apt-get` step.
Both inputs skip with a notice on other platforms and set no linker flag, so
`.cargo/config.toml` and the coverage step's environment still choose which
linker runs. `tests/linker_provisioning_contract.rs` reads the parsed
`setup-rust` step of each CI workflow and asserts both inputs are `'true'` and
that no step installs a linker by hand.

### Cranelift

Exception: Cranelift is not the default backend in `.cargo/config.toml`. The
repository pins `nightly-2026-10-11`, but the release workflow
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
the tests run. Revisit if the release moves to the pinned nightly. The
exception rests on that release constraint, not on a suite measurement under
Cranelift, so moving the pin from `nightly-2026-05-21` to `nightly-2026-10-11`
needed no new measurement and no deferral date.

## Development build footprint

The repository follows the Concordat `rust-dev-build-footprint` rule.
`Cargo.toml`'s default development profile sets `incremental = false` and
`debug = 0`, and the test profile inherits both, so builds keep no incremental
state and no debug information and write far fewer intermediate artefacts. The
trade-off is that a debugger sees no symbols or line tables in a default
development build and every rebuild recompiles whole crates. To debug, run with
`CARGO_PROFILE_DEV_DEBUG=true`; to rebuild incrementally, set
`CARGO_INCREMENTAL=1`. `.cargo/config.toml` also sets
`[unstable] embed-metadata = false`, the native Cargo key for nightlies from
2026-07-31, so Cargo coordinates metadata output itself. No raw rustc metadata
flags are used, and the pin is not moved to suit the key. Stable Cargo, which
builds the release, ignores the `[unstable]` table.
`tests/footprint_contract.rs` holds all three settings.

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

## Local safety policy reuse

[ADR 015](adrs/adr-0015-local-safety-defaults.md) settles the local safety
defaults for workspace identity, redaction, and purge. The policy is one
domain-owned decision used by every ingestion and purge path; it is not a
per-adapter convention, and it is not a library that adapters may reimplement.
Read the ADR for the normative rules and this section for where the
implementing code belongs.

Ownership: the workspace descriptor and its versioned serialization, the digest
and normalization rules, exclusion and deny-pattern evaluation, the redaction
decision, the processing limits, and the purge scope and tombstone types are
domain values and pure decisions. Application use cases orchestrate them.
Filesystem and Git discovery, SQL and Diesel bindings, capability resolution,
terminal input, and backup tooling remain adapters. Runtime names and
signatures belong to the owning implementation slice after its abstraction
sweep; this section fixes contracts, not APIs.

Permitted callers: both existing ingestion ports (`ConversationPushIngestPort`
and `CollectedConversationIngestPort`), the collector's source-reader
implementations, and the purge use case. Adapters may call the policy, never
bypass it. A pushing provider need not redact locally, because the daemon
revalidates push and worker input and must not trust a caller-supplied redacted
flag. Do not add a universal safety port or framework, and do not introduce a
provider-specific branch into the shared pipeline to reach a provider's own
normalization or redaction helper.

Composition order is fixed: establish authority, resolve the workspace, apply
exclusions and deny patterns, redact, then persist or project. A stage may
transform a record — redaction replaces matched intervals — but no stage may
persist, project, or embed partial output. Each stage resolves a record to
exactly one of three outcomes:

- **accepted**, possibly transformed, and carried to the next stage;
- an **intentional exclusion** — a mandatory default exclusion, a deny-pattern
  match, or an operator's audited skip of one source position — recorded as
  deliberately skipped, which **does advance** the source cursor, because
  re-reading it would only be excluded again;
- a **processing failure** — a failed redaction, a detector or policy error, a
  breached processing ceiling, or a store error — which stores nothing, partial
  or otherwise, **does not advance** the source cursor, and surfaces as a
  source-health error for an operator to repair or explicitly skip.

[ADR 015](adrs/adr-0015-local-safety-defaults.md) states this cursor rule
canonically and in full; the three outcomes above restate it for implementers
and must not diverge from it. Authority comes from the authenticated request
context; the pair of authenticated tenant and workspace is the authority, and a
workspace string, a provider field, or a descriptor alone is not.

Validated inputs: workspace registration requires a structurally valid
versioned descriptor, and the daemon rejects an unsupported version rather than
guessing. Digests are compared in full; a collision is a conflict error and
never an implicit join. A descriptor joins an existing workspace only through
an explicitly configured mapping scoped to one tenant, and the registry never
learns a join from provider payloads. Redaction requires in-bounds, valid UTF-8
intervals; an invalid interval is an error, not an empty or pass-through
result. Inside a field the schema declares as JSON, the renderer emits the
marker as a valid JSON string rather than pasting bare marker bytes over the
matched span, so a redacted document still parses and the daemon's
malformed-JSON rule is not tripped by the renderer's own output. The marker is
one logical value with two serialized forms, `[REDACTED]` in text and
`"[REDACTED]"` in JSON, and a consumer compares it by value. The replacement is
a string even where it stands in for an object or an array: whole-value
replacement avoids retaining the original value's internal structure, which can
itself disclose sensitive information, so callers must not rely on the JSON
type of a field redaction may have touched. Purge requires an authenticated
tenant, an allowed workspace, and a purge capability before it produces a
preview, and the confirmation must match that preview's scope and policy
revision.

Semantic errors: use the envelope from [Error Contracts](#error-contracts) and
prefer specific codes over string matching. A breach of the policy is an error;
a deliberate exclusion is not, though it is reported so operators and callers
can see what was skipped and why. The safety surface must at least distinguish
an unsupported descriptor version, a workspace registration conflict, a
cross-tenant scope violation, a redaction failure, a processing-bound failure,
insufficient capability, a stale or mismatched purge preview, a missing or
invalidated backup acknowledgement, and a partial purge with pending cleanup.
It must also report an intentional exclusion — a denied path, a mandatory
default exclusion, or an audited skip — as a distinct skipped outcome carrying
its sanitized reason code, not as an error and not as a silent drop.
Diagnostics carry the exclusion reason, never the matched content: report a
sanitized reason and a bounded location, not the payload, the pattern text, or
a raw filesystem path.

These rules constrain storage bindings. The tombstone recorded after a
successful purge is control metadata holding tenant and workspace identifiers
and operation state only; it is not retained evidence and not a recall audit,
so it must not live in the evidence or audit tables it fences off. Tenant and
workspace ownership must be explicit columns on every record the scoped-delete
transition touches, because the purge kernel's correctness depends on a tenant
predicate that SQL must not supply by convention. Interval provenance must
survive whatever representation the store uses, so a stored redaction record
identifies source positions instead of re-deriving them from rendered output.

### Obligations, witnesses, and proof-to-test relationships

The abstract models V-ID, V-TEXT, and V-PURGE are introduced here and stated as
delivery obligations in the ExecPlan for roadmap task 1.1.2; the design
document's verification strategy carries the corresponding design-level
targets. No claim is discharged by this documentation slice, and the proof
files and a project proof target do not exist yet. Each obligation below names
its later roadmap owner, its witness, its negative control, the proof command
planned for that owner, and the integration test that must accompany it. Refine
or remove a lemma once executable verification conditions exist; nothing here
is a frozen proof decomposition.

V-ID (workspace registration preserves existing bindings and never adds an
unapproved descriptor). Owner: roadmap 2.2.4. Witnesses: two distinct
descriptors forced to the same final digest, and two distinct canonical roots
forced to the same root digest. Negative control: remove descriptor comparison
and require verification or tests to reject it. Proof command:
`prover-tools verus run --repo-root . --proof-file verus/workspace_binding.rs`.
Integration test: `tests/workspace_safety.rs`, with `rstest` cases for
transport separation, explicit origin joins, absent origin, linked worktrees,
root moves, non-Git roots, profiles, invalid encodings, cross-tenant keys,
forced root and final-digest collisions, different SSH usernames, and override
conflicts, plus `proptest` for deterministic serialization and field-boundary
separation.

V-TEXT (no output segment copies a covered source byte position, copied
segments preserve source order from the complement, and a rendered JSON field
still parses). Owner: roadmap 2.2.3. Witnesses: empty, full, overlapping, and
adjacent intervals, multibyte text, and a redacted field whose replacement must
remain valid JSON. Negative control: copy one covered byte and require a
counterexample or a failed proof; separately emit the bare marker inside a JSON
field and require daemon revalidation to reject it; and require that
`SSH-2.0-OpenSSH_9.6` is not matched as a JWT, which fails any implementation
that counts dot-separated base64url-shaped segments without checking that the
first decodes to a JSON object. Proof command:
`prover-tools verus run --repo-root . --proof-file verus/redaction_intervals.rs`.
Integration test: `tests/redaction_safety.rs`, covering each detector with
positive, negative, malformed, boundary, and nested-metadata cases; deny
matches; symlink escapes; policy errors; all three branches of the
escaped-string rule, including a value-scanning match whose escape falls inside
the matched span; a collector-to-daemon round trip in which every redacted JSON
document must still parse; and the `none`, `redacted`, and rejected `encrypted`
modes. Detector completeness is an external limitation and must not be inferred
from this proof.

V-PURGE (deleting a scope preserves every item outside it; a rejected
authorization or confirmation leaves state unchanged; repeated deletion is
idempotent). Owner: roadmap 5.3.1. Witnesses: two tenants with the same
workspace label, plus a third workspace in the target tenant. Negative control:
drop the tenant predicate. Proof command:
`prover-tools verus run --repo-root . --proof-file verus/purge_scope.rs`.
Integration test: `tests/local_safety_e2e.rs` and
`tests/features/local_safety.feature`, using `rstest-bdd` and preserving the
four acceptance scenarios recorded in the ExecPlan, plus denied capability,
stale preview, missing backup acknowledgement, partial store failure, restart
with the tombstone present, explicit re-enablement, delayed ingest after a
successful purge, and a successful idempotent retry.

Treat the external interfaces as assumptions rather than proof targets: Git
discovery, filesystem canonicalization, SHA-256 determinism (never
injectivity), the authenticated request context, glob semantics, database
backup interfaces, and storage deletion acknowledgements. Exercise those at
their real boundaries. Because a later proof must connect runtime byte offsets,
renderer branches, SQL, asynchronous retries, and physical erasure to the
models, record the production mapping with the implementation and verify the
executable kernel or a substantive refinement; a spec mirror alone does not
discharge an obligation.

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
[Architecture Decision Record (ADR) 013](adrs/adr-0013-evidence-store-engine-and-migration-policy.md):
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

## CodeScene coverage and the CV-005 contract

Main owns coverage publication (estate rule CV-005). A pull request measures
coverage for its own ratchet check only, in the `build-test` job of `ci.yml`
(`with-ratchet: 'true'`, `publish-artefact: 'false'`); it never contacts
CodeScene. `.github/workflows/coverage-main.yml` runs on every push to `main`
in the `codescene` environment, generates coverage again and uploads it with
`mode: upload`, guarded on `github.ref == 'refs/heads/main'` alone. The token
reaches the uploader only through its `access-token` input, never an `env`.
When the repository has no `CS_ACCESS_TOKEN` (no CodeScene project yet) the
uploader records a `CodeScene upload skipped` notice and a step-summary line
and the run still succeeds.

`make test-workflow-contracts` checks that shape. It runs the shared
`cv005-contracts check --repository .` from the commit named by
`CV005_CONTRACTS_REF` in the `Makefile` (a bump of that pin is a reviewed
change, never a silent upgrade), then the local wiring tests in
`tests/workflow_contracts`. It needs `uv`, Python 3.13 for the checker, and
`pytest` and `pyyaml`, which `uv run --with` supplies. Both runs go through
`UV` and `UV_ENV`, so a `uv` outside `PATH` can be injected. `make all` and CI
run the target; `.github/cv005.toml` holds the repository name and nothing
else. A Dependabot pull request merged by the automerge workflow fires no push
event, so that merge publishes nothing until the next push to `main`
(shared-actions #518 tracks the fix).
