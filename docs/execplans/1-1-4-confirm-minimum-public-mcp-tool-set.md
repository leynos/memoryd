# Confirm the minimum public MCP tool set

Status: DRAFT — awaiting explicit approval; no implementation is authorized.

This ExecPlan is a living document. Keep Constraints, Tolerances, Risks,
Progress, Surprises & discoveries, Decision log, Outcomes & retrospective,
Conformance basis, and Verification plan current at every milestone.

## Purpose / big picture

Ratify roadmap 1.1.4 so the first public release has an explicit Model Context
Protocol (MCP) tool contract. Release 0.1 is the checkpoint at the end of phase
3, not merely the earlier session-browsing slice. Require seven tools:
`memory_store`, `memory_recall`, `memory_retract`, `memory_sessions`,
`memory_import_session`, `memory_explain`, and `memory_health`. Defer
`memory_profile` until graph-backed profiles in roadmap 5.2.3.

Success for this decision slice is an accepted decision record, a complete
required/deferred tool matrix, worked rejection examples, and named runtime
verification owners. A later MCP implementor can distinguish deliberately
unsupported operations from missing required release functionality.

Approval authorizes the documentation milestones below. It does not authorize
implementing the MCP server, tool handlers, storage, or proofs. Those remain
separate roadmap slices. Publishing this draft does not complete 1.1.4.

## Constraints

Preserve ADRs 005–007, 013, and 015: inward dependencies, daemon-owned memory
policy, authenticated tenant/workspace authority, canonical ingestion,
SQLite/PostgreSQL parity, configured-root restrictions, redaction, and purge
safeguards. MCP is a driving adapter, not a second memory-policy engine.

Do not change Rust source, dependencies, migrations, configuration, persistent
formats, or transport code in this ratification. Do not require Oxigraph,
Chutoro, graph promotion, profile materialization, hierarchical recall, or RFC
0006 post-1.0 epistemic-health features for release 0.1. Keep source-health
freshness policy with 1.1.5 and durable recall-audit policy with 3.2.4.

Do not describe documented behaviour as shipped enforcement. Retraction is
logical withdrawal, not irreversible purge. Never add arbitrary transcript
reads to MCP. Do not invent source compatibility wrappers for an API that has
not shipped. No `assume`, `admit`, trusted/external proof bodies, or equivalent
shortcuts are permitted; unproved statements remain prose.

## Tolerances (exception triggers)

Stop, record options in Decision log, set status BLOCKED, and seek explicit
approval if execution requires runtime changes, new dependencies, a different
required tool set, a changed trust boundary, or changed prerequisite policy.
Ordinary wording, reference repairs, and clarification within the proposed
contract are allowed. Stop if more than ten non-ExecPlan documentation files or
900 net added lines outside ExecPlans are needed. Exclude all ExecPlan files
from both limits. There is no time limit. Report unrelated gate failures
without expanding this work to repair them.

## Risks

Confusing the first evidence slice with release 0.1 could omit explanation or
retraction (high impact, medium likelihood). Use the phase-3 checkpoint and
explicit owners below as the completeness oracle.

The target-v1 catalogue in design §11 is broader than release 0.1 (medium
impact, high likelihood). Separate required tool names from supported operand
kinds and dependency capabilities. A known tool must not silently accept a
future operand it cannot implement.

Read-only discovery could be mistaken for authorization (high impact, medium
likelihood). Keep daemon capability and scope checks authoritative even for
callers that bypass discovery. Import could accidentally become arbitrary
filesystem access; preserve the application-mediated configured-root boundary.

Protocol and SDK versions may change before implementation (medium impact,
medium likelihood). Record the researched specification revision, recheck the
chosen SDK at the owning runtime slice, and exercise the real protocol
boundary. Do not select an MCP SDK in this decision task.

## Progress

- [x] (2026-11-04) Rename branch and verify prerequisites 1.1.1–1.1.3.
- [x] (2026-11-04) Two Wyvern agents inspect scope and verification evidence.
- [x] (2026-11-04) Inspect MCP protocol error guidance with Firecrawl.
- [x] (2026-11-04) Draft seven-tool decision, contracts, and test handoffs.
- [x] (2026-11-04) Six expert perspectives reviewed the draft; incorporated
  authority, generator, bounded-output, health, and gate-sequence corrections.
- [x] (2026-11-04) Planning checks passed sequentially: formatting, Markdown,
  Mermaid, Rustdoc, Clippy, Whitaker, and all 150 tests (none skipped).
  Publication receipts are recorded in the draft PR and delivery handoff.
- [ ] Obtain explicit plan approval before EP-M1.
- [ ] EP-M1: Ratify the tool matrix and reconcile upstream documents.
- [ ] EP-M2: Document conformance cases and verification ownership.
- [ ] EP-M3: Reconcile decisions, mark roadmap 1.1.4 done, and deliver.

## Surprises & discoveries

`src/main.rs` is a stub and there is no MCP crate or production dependency in
`Cargo.toml`. The crate split belongs to 1.2.2. No executable tool rejection
can truthfully be demonstrated by this task. The completed 1.1.3 plan provides
the decision-only precedent.

RFC 0005 uses named headings rather than numbered sections. Its “Proposed
design”, including “Profiles” and “Durable recall audit”, and “Compatibility
and migration” carry the relevant material referred to as §5 in the roadmap. Its
`flat_v1` table mentions graph retrieval, while roadmap 3.2.1 explicitly
requires graph-free recall. Clarify optional later graph enrichment without
making it a release-0.1 prerequisite.

CodeGraph MCP tools were unavailable during planning. Direct file inspection
and a shared context pack (`pk_x57dmw4t`) supplied the reconnaissance evidence.
The plan is self-contained and does not depend on that temporary pack.

## Decision log

- 2026-11-04: Propose a documentation ratification slice, following roadmap
  §1.1 and the completed 1.1.3 precedent. Runtime work remains in phases 2–3.
- 2026-11-04: Propose seven required tools and defer only `memory_profile` of
  the eight candidates. A four-tool minimum was considered and rejected: 2.2.5,
  3.2.3, and 3.3.1 already require import, explanation, and retraction before
  the first public checkpoint.
- 2026-11-04: Propose explicit discovery and rejection contracts, with finite
  truth-table testing rather than speculative Verus scaffolding. No new
  unbounded business algorithm is introduced by this decision.

## Outcomes & retrospective

The draft is prepared, expert-reviewed, and validated. No contract is accepted,
runtime tool is implemented, or roadmap item completed by this draft. On
execution, record each milestone's changes, gate evidence, conformance
findings, and remaining runtime handoffs. Before marking COMPLETE, reconcile
every discovery with the conformance basis; leave no unaccepted architecture
deviation hidden in this plan.

## Context and orientation

Start at [documentation contents](../contents.md) and
[repository layout](../repository-layout.md). The single Cargo package holds
build and workflow contract tests, not a working daemon. The proposed domain,
application, adapter, and composition boundaries live in design §5.3 and ADR
005. The composition root wires adapters; application use cases coordinate
work; domain-owned ports keep infrastructure types out of business logic.

Design §11 currently lists all eight target-v1 tools without first-release
classification. The terms of reference §9 still asks which tools are required.
The roadmap already assigns manual import to 2.2.5, session/health MCP to
2.4.2, store/recall MCP to 3.2.2, explanation to 3.2.3, and retraction to
3.3.1. The release-0.1 end-to-end suite is 3.3.3. Profiles belong to 5.2.3.

## Conformance basis

The planning baseline is Git revision
`088a4faeaa484000f99dbbc69b1abff92b3ebfaa`. References below mean that revision
unless an approved execution revision is explicitly recorded.

- C1: [Roadmap](../roadmap.md) §1.1.4 and the phase-3 release checkpoint;
  [terms of reference](../terms-of-reference.md) §§6–9. Complete tool selection
  maps to EP-M1 and evidence E1, the eight-row decision matrix.
- C2: [Design](../memoryd-design.md) §§2.3, 5.3–5.4, 11–12, and 16–17;
  [ADR 005](../adrs/adr-0005-hexagonal-architecture-boundary.md), [ADR 006](../adrs/adr-0006-tenant-isolation-and-corbusier-context.md),
  and [ADR 007](../adrs/adr-0007-standard-conversation-ingestion-port.md).
  Ownership, authority, discovery, and refusal map to EP-M1/EP-M2 and E2, the
  conformance matrix and worked protocol examples.
- C3: [ADR 013](../adrs/adr-0013-evidence-store-engine-and-migration-policy.md)
  and [ADR 015](../adrs/adr-0015-local-safety-defaults.md). Import safety and
  authoritative memory preservation map to EP-M2 and E3, the runtime handoff.
- C4: [RFC 0005](../rfcs/rfc-0005-hierarchical-recall.md), “Proposed design”
  and “Compatibility and migration”; [RFC 0006][rfc6], “Interfaces”. Flat
  recall and deferred capabilities map to EP-M1/EP-M2 and E1/E3.
- C5: The [MCP tools specification dated 2025-11-25][mcp-tools], inspected
  through Firecrawl on 2026-11-04. Discovery and protocol-versus-execution
  errors map to EP-M2/E2. This is a research baseline, not an SDK selection or
  promise of a negotiated protocol version.

E4 is the final cross-document reconciliation and sequential gate record for
EP-M3. If C1–C5 change, update affected decisions, handoffs, and evidence IDs
before continuing; material departures require approval.

## Proposed contract

### Tool selection and semantic limits

| Tool                    | Release 0.1 disposition | Minimum meaning and runtime owner                                                                          |
| ----------------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------- |
| `memory_store`          | Required; write         | Evidence-backed curated memory through the daemon; 3.1.3 and 3.2.2.                                        |
| `memory_recall`         | Required; read          | Bounded, tenant-scoped `flat_v1` context with evidence and selection diagnostics; 3.2.1–3.2.2.             |
| `memory_retract`        | Required; write         | Retract curated memories and exclude serving payloads from default recall while preserving history; 3.3.1. |
| `memory_sessions`       | Required; read          | Bounded browsing of ingested sessions with authorized filters; 2.4.1–2.4.2.                                |
| `memory_import_session` | Required; write         | Explicit import inside configured roots using canonical ingestion and safety policy; 2.2.5.                |
| `memory_profile`        | Deferred                | Stable profile read/update and graph-backed promotion; 5.2.3.                                              |
| `memory_explain`        | Required; read          | Curated-memory evidence, projection state, scores, filter reasons, and relevant source health; 3.2.3.      |
| `memory_health`         | Required; read          | Scoped daemon, configured dependencies, collector/source and projection health; 2.4.2 and 3.3.2.           |

_Table 1: Proposed release-0.1 tool contract and implementation owners._

Do not make graph facts, episodes, themes, or source-session retraction an
implicit consequence of shipping `memory_retract`; later slices own those
operand kinds. Reject unsupported target kinds explicitly. Retraction does not
expose purge or bypass ADR 015 confirmation and backup requirements.

`memory_explain` must not imply typed claim support or graph lineage before
those exist. Required explanation covers stored evidence and flat-recall
selection, not compulsory durable audit persistence; 3.2.4 retains that policy.
`memory_health` must distinguish disabled/not configured components from failed
configured dependencies. It must not present absent graph/theme functionality
as either working or a first-release startup requirement. Source-health detail
comes from 1.1.5 and ADR 008; epistemic-health reports remain in RFC 0006. A
successful health query can report degraded Qdrant or Ollama without itself
being an MCP execution error. Use `isError: true` only when the health query
cannot execute; never block it on embedding, repair, or absent graph services.
Bound source-health lists and explanation evidence/selection details using the
developers' guide pagination and response-limit conventions. Numeric defaults
remain with 2.4.2 and 3.2.3; include oversized-result boundary fixtures there
rather than introducing new resource policy in this decision.

Only `flat_v1` is required. A request for a future recall profile must produce
an explicit unsupported-profile execution result in release 0.1, not silently
pretend to run hierarchy. Later hierarchy fallback remains governed by RFC
0005; its response must name the actual profile and fallback reason.

### Discovery, dispatch, and authority

For a release-0.1 server with all required operations implemented, advertise
all seven tools in read-write mode, and only the four reads in read-only mode:
recall, sessions, explain, and health. Do not advertise `memory_profile` or the
RFC 0006 tools. Incremental development may expose fewer implemented tools, but
that state is not release-0.1 completeness. Dependency outages do not silently
remove implemented tools from discovery; report operation failure and health.

An authenticated `tools/call` for an unadvertised name, including a deferred
tool or a read-only-hidden write, returns a JSON-RPC protocol error with code
`-32602` (unsupported/unknown tool). It must not invoke a daemon mutation.
Distinguish this from a recognized, advertised tool whose execution fails:
return an MCP tool result with `isError: true` and safe semantic error details,
for example unsupported target kind, denied import, or dependency unavailable.
The unsupported-operand execution case presumes a schema-valid request whose
operand is unavailable in this release. Invalid argument shapes instead follow
the negotiated protocol and SDK validation rules; do not force them through
application execution. Exact SDK envelope construction belongs to the runtime
adapter, not domain interfaces. Do not add a private JSON-RPC error code for
deferred names.

The daemon independently rejects unauthorized writes and tenant/workspace scope
violations even if a caller bypasses MCP read-only discovery. MCP request
payloads cannot establish authority. Avoid exposing tenant identities, local
paths, raw transcripts, or backend diagnostics in error messages. Capability
changes and `tools/list_changed` handling must follow the eventual negotiated
protocol; this decision does not introduce dynamic configuration reload.

Manual import is application-mediated: MCP must not call a filesystem adapter
directly. Use the accepted canonical delta and push-ingestion contract through
the application service; root enforcement and parsing stay behind their owned
boundaries. No new catch-all tool registry port or second ingestion abstraction
is justified. Sweep existing abstractions again when the crate spine exists.

## Verification plan

### Abstract specification before executable design

Let U be the eight names in Table 1, R the seven required names, D the
singleton profile name, and W the three writes (store, retract, import). Let I
be the implemented subset of R and ro the server's read-only flag. Discovery is
`L = I minus W` when ro is true, otherwise `L = I`. Release completeness means
`I = R`; an unavailable dependency does not change I. A call can dispatch only
if its name belongs to L; successful execution additionally requires valid
arguments and authoritative daemon permission. Discovery is not a grant.

V1: R and D partition U without overlap or omissions. V2: L contains no
unimplemented, deferred, or read-only-disabled name. V3: a name outside L
causes no tool dispatch; daemon authorization still gates supported mutations.
V4: the first-release semantic limits hold even when the tool name is valid.
V5: documentation labels planned behaviour accurately and preserves later
owners.

The corresponding reasoning obligations are partition completeness, agreement
between listing and dispatch, exclusion of writes in read-only mode, and
separation of discovery from authority. The representation relationship is that
each future typed tool variant has exactly one canonical wire name and mode
matching an independently maintained expected table. Tests must not calculate
their oracle from the production registry being tested.

The preconditions are an established local/authenticated context, fixed server
mode for each request, and a well-formed MCP request. Invalid or unknown names
are still tested at the transport boundary. For names outside L, the
postcondition is zero tool use-case invocations. For recognized-tool execution
denials, application validation may run, but no forbidden content read or
memory mutation may occur. Security audit events remain permitted in both
cases. No conclusion is claimed about arbitrary malformed transport bytes from
the finite tool-name model.

### Methods, evidence, and non-vacuity

For this documentation slice, E1 enumerates all eight names, all three writes,
and all four reads; E2 records both modes, known-deferred names, unknown names,
and valid tools with unsupported operands. Reviewers must reject a matrix
missing `memory_retract` or admitting `memory_profile`: those are concrete
negative controls. E3 names the tests below and their owning roadmap items; E4
records formatting, documentation, lint, and test gate results. These checks
discharge ratification completeness, not runtime enforcement.

For runtime V1/V2, hand off an independent `rstest` truth table covering all 8
names × 2 modes and all implemented-subset masks (128 subsets). These are
test-model cases, not a requirement for configurable runtime masks. Assert
exact listing equality and uniqueness; use `googletest` assertions and
`pretty_assertions` for structured differences. The release test separately
requires I = R, so an empty implementation cannot pass as a complete release.
Own the shared MCP catalogue fixtures within the future MCP crate and reuse
them in its boundary tests; avoid adding a shared utility crate for one user.

For runtime V3, use a mixed `proptest` strategy: arbitrary UTF-8 names plus
explicit sampling of every canonical name and empty, case-changed, prefix/
suffix, and near-miss examples. Compare to an independent exact-name oracle and
assert no handler invocation on refusal. Record seed and reached
classifications; do not filter away recognized names or the three write cases.
Seed a deliberate mutation that dispatches a hidden write or advertises
profile; the suite must fail, then restore the code.

For V3/V4, bind the scenarios below with `rstest-bdd`; use a counting fake at
the application boundary and real MCP serialization in integration tests. Add
end-to-end coverage under 2.4.3 and 3.3.3 for tenant isolation, allowed and
denied imports, repeated import, read-only bypass rejection, store/recall/
explain/retract, dependency degradation, and projection repair. Use real SQLite
and PostgreSQL adapters where persistence is involved per ADR 013; ordinary
catalogue tests need neither database. Valid witnesses include a local
authorized curated write, an allowed import, and a read-only session list.

Use `insta` snapshots for the structured and text representations of discovery,
unsupported-tool errors, explanation, and health when those multiple output
forms exist. Redact unstable IDs, not semantic status, scope, or error fields.
Snapshots supplement semantic assertions; they do not replace them.

External assumptions are the negotiated MCP request/result interfaces, SDK
serialization, authenticated context establishment under ADR 006, and the
application/ingestion contracts under ADR 007/015. Exercise repository-owned
mapping against a real MCP client and daemon. Do not claim to verify SDK,
filesystem, database, or model-provider internals.

Verus is not warranted for this documentation-only decision or the finite
catalogue/mode truth table. No new unbounded algorithm or semantic lemma is
introduced here. Existing tenant noninterference, ingestion idempotency, and
cursor proofs remain with their owners in the completed 1.1.3 handoff. Should
runtime work introduce a new unbounded policy kernel, stop and revise its plan
first: independently specify states, premises, postconditions, representation,
and candidate semantic lemmas; then implement a substantive Verus proof with an
explicit project verification command and satisfiable witnesses. Never claim
finite tests prove those unbounded properties. This plan contains no Verus
milestone and therefore invents no currently nonexistent proof command.

### Behaviour specification for the runtime handoff

Proposed future paths, created only by the owning runtime slices, are
`tests/features/mcp_tool_surface.feature` and `tests/mcp_tool_surface.rs`.
After 1.2.2 establishes crate paths, update E3 to the actual equivalents before
writing tests. Preserve these acceptance scenarios:

```gherkin
Feature: Release 0.1 MCP tool surface
  Scenario: Complete writable discovery
    Given an authenticated writable release 0.1 server
    When the client lists tools
    Then exactly the seven required tools are listed once
    And memory_profile is absent

  Scenario: Read-only call bypass
    Given an authenticated read-only release 0.1 server
    When the client calls memory_store without listing tools
    Then the protocol error code is -32602
    And no memory mutation use case is invoked

  Scenario: Deliberately deferred tool
    Given an authenticated writable release 0.1 server
    When the client calls memory_profile
    Then the protocol error code is -32602
    And no application use case is invoked

  Scenario: Future operand on a supported tool
    Given an authenticated writable release 0.1 server
    When the client submits a schema-valid request for theme retraction
    Then memory_retract returns an unsupported-target tool error
    And no memory mutation is committed

  Scenario: Missing optional infrastructure
    Given a release 0.1 server without graph or theme components
    When the client lists tools and reads health
    Then all seven implemented tools remain discoverable
    And health distinguishes disabled components from failed dependencies

  Scenario: Health remains useful during an embedding outage
    Given a release 0.1 server with an unavailable configured Ollama
    When the client reads health
    Then health returns a successful report identifying the degraded dependency
    And no embedding or repair operation is awaited
```

Read-only is false in the missing-optional-infrastructure scenario. Extend the
scenario table with direct daemon unauthorized-write rejection, forbidden
import paths, unavailable Qdrant/Ollama, and forged tenant filters in their
owning end-to-end suites.

## Plan of work and milestones

### EP-M1: Ratify the minimum surface

After approval, recheck prerequisites and the baseline. Add
`docs/adrs/adr-0016-minimum-public-mcp-tool-set.md` using the repository ADR
format, provided 0016 is still free; otherwise choose the next unused number
and update all references. Record the seven-tool decision, alternatives,
read-only contract, semantic limits, and consequences. Set Accepted only as
part of executing the explicitly approved decision.

Update design §11 with the matrix and refusal semantics, §2.3 and §16 with the
release boundary, and §17 with the resolved decision. Resolve the matching
question in terms of reference §9 without reducing its target-v1 goals in
§§6–7. Clarify RFC 0005's `flat_v1` description as graph-free for release 0.1
with later optional enrichment; do not accept its entire proposed hierarchy.
Index the ADR in `docs/contents.md`.

C1/C2/C4 advance through E1/E2. Acceptance: every candidate has exactly one
disposition, rationale, semantic limit, and runtime owner; the RFC no longer
implies graph is mandatory. Run all gates below and commit this coherent
ratification. Remaining gap: worked guide examples and handoffs in EP-M2.
Recovery: amend uncommitted documentation or revert the atomic documentation
commit; no persisted state needs migration. No compatibility layer is needed.

### EP-M2: Publish verification and user-facing handoffs

Update `docs/developers-guide.md` with the abstract catalogue specification,
independent oracle, refusal examples, verification owner mapping, fixture reuse
policy, and proof/test distinction. Use design §11 as the component
architecture reference; there is no separate MCP architecture document yet.
Update `docs/users-guide.md` with the planned seven-tool surface, read-only
behaviour, import limits, and deferred profile/graph/theme features, explicitly
stating that the current binary does not implement them.

E2 must include a worked `tools/list` result for each mode and JSON-RPC call
examples for `memory_profile`, a read-only write, an unknown name, and a valid
tool with an unsupported operand. E3 maps catalogue/error tests to 2.4.2,
imports to 2.2.5/2.4.3, store/recall to 3.2.2, explanation to 3.2.3, and
retraction plus the complete release oracle to 3.3.1/3.3.3. Explicitly assign
the MCP retraction binding to 3.3.1; its current wording emphasizes the daemon
use case, while 3.3.3 requires the MCP path.

C2/C3/C4 advance through E2/E3. Acceptance: each contract has a positive and
negative example and an executable-test owner, and none is claimed implemented.
Run all gates and commit. Recovery is a documentation revert. Remaining gap:
final cross-document reconciliation; compatibility machinery remains absent.

### EP-M3: Reconcile and close the decision task

Review C1–C5 against all changed documents and evidence E1–E3. Correct
contradictory wording and stale links. Use the `mapsplice` skill to update only
roadmap 1.1.4's status and decision note, with links to the ADR and this plan,
after the ratification acceptance is satisfied. Record the status as done using
the roadmap's checked-checkbox convention. Do not mark runtime owners complete.
Refresh `docs/contents.md` descriptions if needed.

E4 requires all gates below, a reviewed eight-tool completeness check, and
explicit unresolved-runtime handoffs. Commit the final coherent state and
update the implementation PR. Set this plan COMPLETE only for ratification.
Recovery is a revert of the final documentation commit. No API compatibility
work is authorized. At every plateau check assigned requirements, approved
architecture, upstream assumptions, dependency/trust/persistence boundaries,
and current trace links; record material deviations and stop for approval.

## Concrete steps and validation

Run from the repository root on the task branch. Read `AGENTS.md` and inspect
`git status --short` before editing. For each major milestone, use one gate
runner to run the following sequence without overlapping gates. The implementor
applies formatting first; a scrutineer then owns all check gates and does not
edit files. Capture exit codes with `pipefail`; replace `epm1` in log names for
later milestones.

```bash
set -euo pipefail
make fmt 2>&1 | tee /tmp/fmt-memoryd-1-1-4-epm1.out
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-4-epm1.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-4-epm1.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-4-epm1.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-4-epm1.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-4-epm1.out
```

Stop on the first failure; do not treat a later success as a substitute.
Expected evidence is exit zero for every command, no formatting differences, no
Markdown/Mermaid errors, no Rustdoc/Clippy/Whitaker warnings, and all tests
passing. Record actual counts instead of assuming a baseline count. Inspect
`git diff --check` and the entire diff after formatting; preserve unrelated
work. The existing formatter owns whitespace normalization.

`make lint` includes Rustdoc, Clippy, and Whitaker. `make test` selects nextest
when available. The Makefile passes `tools/dev-fast/config.toml` for ordinary
development; direct Cargo development invocations must do likewise. Do not
apply that fragment to verification, coverage, or release builds. Use shared
Cargo caches and compiler admission; never bypass wrappers or place build
output in `/tmp`. Queued compilation is not failure.

This ratification cannot have an executable Red-Green-Refactor cycle because it
changes no runtime. Its observable substitute is the independent matrix review
and cross-document consistency check. For each later code owner, first write
the relevant failing `rstest`/BDD case and run the focused target with
`make test TEST_FLAGS='--test mcp_tool_surface'` once that target exists.
Record an assertion failure for the missing behaviour, not a compile error or
missing test target; then implement the minimal handler/registry change, rerun
the focused target, refactor, and run the complete sequential gates. Adjust the
target path/package after the crate split and record the exact working command
before the red run. No expected-failure markers remain in passing
implementation.

## Interfaces, dependencies, and reference material

No executable signatures or dependencies are added by 1.1.4. Keep domain types
and semantic errors inside the core, and MCP/JSON-RPC serialization in the
adapter. The future catalogue owns canonical wire names, modes, and supported
operand metadata; application use cases own policy. Before extracting a shared
helper, search for an existing equivalent and document ownership, allowed
call-sites, and composition rules in the developers' guide.

Consult these repository guides when executing the handoff:

- [Developers' guide](../developers-guide.md): errors, pagination, authority,
  development commands, and existing proof/test handoffs.
- [rstest fixtures](../rust-testing-with-rstest-fixtures.md): independent
  parameterized cases and fallible fixtures.
- [Dependency injection](../reliable-testing-in-rust-via-dependency-injection.md):
  injected clocks, environment, and application fakes; no direct test
  environment mutation.
- [Rust doctest guide](../rust-doctest-dry-guide.md): executable public examples
  once an API exists.
- [Complexity and refactoring](../complexity-antipatterns-and-refactoring-strategies.md):
  small cohesive handlers and avoiding a monolithic dispatch function.
- [Unprivileged PostgreSQL setup](../pg-embed-setup-unpriv-users-guide.md):
  actual PostgreSQL integration fixtures in persistence-owning slices.
- [rstest-bdd guide](../rstest-bdd-users-guide.md): bindings for the feature
  specification above.
- [Documentation style](../documentation-style-guide.md): ADR format, British
  Oxford spelling, captions, and wrapping.

Use skills `execplans`, `codegraph-mcp`, `hexagonal-architecture`, and
`rust-router`; route verification questions through `rust-verification`, finite
test design through `rust-unit-testing`, and new proof work through `verus`
only when warranted. Use `firecrawl-mcp` for external protocol/SDK research,
`logisphere-experts` for design review, `mapsplice` for roadmap structure, and
`commit-message`/`pr-creation` for publication.

## Idempotence and recovery

Documentation and gates are safely repeatable. Recheck ADR numbering before
creation; never overwrite an existing record. Commit only green milestones. Do
not reset unrelated work. A failed gate leaves the candidate uncommitted; read
its log, apply an in-scope correction, then rerun affected gates and the
required sequence. Report environmental failures distinctly from source
failures. No data migration, service restart, or destructive action is needed.

## Artefacts and revision notes

The planning pull request contains this draft and its documentation-index entry
only. Publication evidence belongs in the PR body and final handoff. The
implementor records actual milestone logs and conformance findings here.

Revision 2026-11-04: Initial draft separates release-0.1 selection from later
runtime implementation, specifies deliberate refusal, and assigns verification
without speculative proof scaffolding. Approval is still required.

Revision 2026-11-04 after expert review: Pandalump and Telefono narrowed
zero-invocation refusal to catalogue gates; Telefono clarified mixed name
generation and schema-valid operands; Wafflecat kept subset masks test-only;
Buzzy Bee and Doggylump added bounded diagnostics and successful degraded
health; Dinolump made gate commands fail fast. These changes refine E2/E3 and
do not expand implementation scope.

[rfc6]: ../rfcs/rfc-0006-epistemic-health-empiricism-falsification-and-semirings.md
[mcp-tools]: https://modelcontextprotocol.io/specification/2025-11-25/server/tools
