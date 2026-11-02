# Accept or revise the hexagonal, tenant, and ingestion boundaries

Status: DRAFT — awaiting explicit approval before execution.

This ExecPlan is a living document. Keep Constraints, Tolerances, Risks,
Progress, Surprises & discoveries, Decision log, Outcomes & retrospective,
Conformance basis, and Verification plan current at every milestone.

## Purpose / big picture

Ratify roadmap item 1.1.3 so Corbusier, Axinite, Codex, Claude, and manual
import implementors can target one canonical conversation delta while keeping
authenticated push and collector-batch trust and retry rules separate. Success
is a coherent accepted decision set, worked conformance examples, and explicit
verification handoffs that later implementation slices can execute.

This is a documentation decision slice. It does not implement ingestion or
claim that the current binary can ingest, browse sessions, or recall memories.
Those capabilities remain in roadmap phases 2 and 3. Approval authorizes the
three documentation milestones below, not the deferred runtime handoffs.

## Constraints

Preserve inward dependencies, explicit tenant authority, the port budget,
SQLite/PostgreSQL parity from ADR 013, and local safety defaults from ADR 015.
Provider payloads are evidence, never an authority source. Workspace identity
is tenant-scoped. Preserve all three ingestion ports and one canonical delta.

Do not add executable Rust, dependencies, migrations, transport formats,
provider parsers, runnable fixtures, or speculative Verus scaffolding in this
slice. Do not advance graph, themes, Axinite write-back, hosted administration,
or RFC 0006's post-1.0 epistemic-health decisions. Do not mark 1.1.3 done in
this planning PR. Mark it done only after the approved execution is complete.

No `assume`, `admit`, trusted/external proof body, or equivalent shortcut may
stand in for a proof. Unproved statements remain prose until their owning
runtime milestone. Do not manufacture compatibility wrappers for this
pre-implementation API. Persisted or deployed peer compatibility must be based
on actual evidence, not speculative callers.

## Tolerances (exception triggers)

Stop and set status BLOCKED if execution needs runtime changes, dependencies, a
new trust boundary, changed ADR 013/015 policy, or a materially different
public contract from the proposed decisions below. Record the affected
requirements, options, and upstream changes; obtain explicit approval before
continuing. Ordinary wording and cross-reference corrections are authorized.

Limit execution to 12 non-ExecPlan documentation files and 1,000 net added
lines outside ExecPlans. ExecPlans are excluded from both counts. Escalate
unresolved material ambiguity after expert review. There is no time limit. Do
not repair unrelated baseline gate failures by expanding this task.

## Risks

Cursor ambiguity can silently skip evidence (high impact, medium likelihood).
Ratify commit, retry, partial-record, and source-generation semantics together.
Identity conflation can merge tenants or providers (high impact, medium
likelihood); include same-identifier/different-tenant counterexamples.

A broad canonical schema can invent missing source data (medium impact, medium
likelihood). Separate required fields, optional fields, and preserved provider
metadata; compare only genuine shared semantics. Provider formats can change
(medium impact, high likelihood); require versioned fixture provenance in the
implementing slices. Current sibling-project references in ADRs are historical
pointers, not evidence of a live supported format.

Accepted documentation may be mistaken for shipped enforcement (medium impact,
medium likelihood). Label user guidance as planned and retain explicit runtime
owners. Formal models may omit real transaction or parsing behaviour (high
impact, medium likelihood); require a representation relationship and real
adapter tests before claiming implementation correctness.

## Progress

- [x] (2026-11-02) Rename branch; inspect instructions and prerequisite status.
- [x] (2026-11-02) Two Wyvern agents inspect boundary and verification sources.
- [x] (2026-11-02) Draft contracts, proof sketches, and runtime handoffs.
- [x] (2026-11-02) Six expert perspectives reviewed the plan; reconciled
  ownership, bounds, operation identity, generation, and batch-scope findings.
- [x] (2026-11-02) Planning validation passed: formatting, Markdown, Mermaid,
  Rustdoc, Clippy, Whitaker, and 150 tests (zero skipped). Publication and
  final candidate readback evidence accompany the draft PR.
- [ ] Obtain explicit execution approval.
- [ ] EP-M1: Ratify boundary decisions and synchronize the design.
- [ ] EP-M2: Record conformance cases and verification handoffs in guides.
- [ ] EP-M3: Reconcile evidence, mark 1.1.3 done, and deliver.

## Surprises & discoveries

ADRs 005–007 remain Proposed although their boundaries already structure the
roadmap. They must receive an explicit acceptance disposition. Prerequisites
1.1.1 and 1.1.2 are checked complete and governed by ADRs 013 and 015.

The design specifies cursor fields and provider key examples, but does not
fully specify advancement, replay, or conflicting duplicate behaviour. The
line-hash input is explicitly deferred to 2.2.3 in design §17; this task must
preserve its detector-independent requirement without inventing an encoding.

`src/main.rs` remains a stub. No domain ports, ingestion kernel, or Verus
project exists. `make lint` includes Rustdoc and Whitaker as well as Clippy;
`make test` chooses nextest when installed. Use current Makefile targets rather
than the simplified commands in generic guidance.

## Decision log

- 2026-11-02: Scope 1.1.3 as ratification, following roadmap §1.1 and the
  completed 1.1.2 precedent. Runtime traits belong to 1.2.1; evidence and
  ingestion implementation belong to 2.1 and 2.2.
- 2026-11-02: Recommend accepting ADRs 005–007 with targeted clarifications.
  They are still Proposed, so revise them directly before acceptance; do not
  create a redundant replacement ADR. A substantive departure discovered during
  execution requires a separate ADR and explicit deviation approval.
- 2026-11-02: Keep proof sketches here and later in the design. No Verus
  milestone executes in this documentation slice; future unbounded semantic
  guarantees require substantive Verus proofs alongside integration tests.

- 2026-11-02: Expert review tightened internal contract visibility, deferred
  later-store strategy, and corrected runtime owners to 1.2.6–1.2.8. Bounded
  inputs and one-workspace batches avoid unbounded acceptance work. Request
  identity, batch cursor vectors, persistent sequence namespaces, and stable
  source generations make retry claims precise. The trade-off is smaller atomic
  scope at the cost of collector-side partitioning.

## Outcomes & retrospective

The reviewed planning draft is ready for approval. All six local gates passed
before this progress update; the publication handoff records validation of the
final candidate and draft PR readback. No boundary decision has been ratified,
no runtime behaviour has changed, and no theorem is claimed proved. During
execution, record accepted decisions, remaining runtime owners, and exact gate
evidence per milestone; do not describe a moving branch head as a permanently
tested SHA.

## Context and orientation

Start with `docs/contents.md` and `docs/repository-layout.md`.
`docs/memoryd-design.md` §§5.3–6.1 owns the architecture and normative port
shape. ADR 005 owns dependency direction; ADR 006 owns tenant authority; ADR
007 owns canonical ingestion. Domain means pure business types and policy;
application means orchestration through ports; an adapter translates an
external source or implements infrastructure access. A composition root wires
concrete implementations at a binary entrypoint.

A delta is a bounded, ordered set of conversation changes and provenance. A
cursor is a source-specific resume position, not a global timestamp.
Idempotency means retrying the same logical operation does not duplicate its
durable effects. A source generation identifies the incarnation of a source
whose path may be reused. A conformance fixture pairs a versioned source sample
with independently specified expected canonical semantics.

Load `execplans`, `codegraph-mcp`, `hexagonal-architecture`, and `rust-router`.
Use CodeGraph for structural questions, known-file reads for documentation,
`firecrawl-mcp` for external gaps, and `logisphere-experts` for review. Use
Wyvern for reconnaissance, context_pack for team references, and scrutineer for
sequential gates. Later Rust work should route through `rust-verification` and
then `verus` or `proptest` as appropriate, plus `leta` for code navigation.
Read the applicable SKILL.md from the installed catalogue before use. Recheck
compiler posture before borrow-sensitive API work; this plan requires no
borrow-checker workaround or compiler change.

## Conformance basis

The inspected baseline is commit `4429d5163ba37cee467dba9a94e8e536b70d58fb`.
All repository references below refer to that revision until execution records
a deliberate update.

| Requirement                                         | Authority                                         | Milestone and evidence                                    |
| --------------------------------------------------- | ------------------------------------------------- | --------------------------------------------------------- |
| B1: inward dependency, ownership, port budget       | Design §5.3; ADR 005                              | EP-M1 accepted rule; EP-M2 architecture negative controls |
| B2: authenticated tenant and explicit context       | TOR §§8–9; design §5.4; ADR 006                   | EP-M1 authority matrix; EP-M2 two-tenant examples         |
| B3: one delta, distinct push/batch/source contracts | Roadmap 1.1.3; design §6.1; ADR 007               | EP-M1 port contract; EP-M2 five producer mappings         |
| B4: replay, commit, cursor, correction rules        | Design §§6.1–7, §17; ADRs 007 and 015             | EP-M1 transition rules; EP-M2 proof and crash cases       |
| B5: first-value scope and prerequisite preservation | TOR §§8–9; design §§16–17; ADRs 013/015; RFC 0006 | EP-M3 deferral audit and roadmap completion               |

_Table 1: Requirements to delivery evidence._

The full RFC path is
`docs/rfcs/rfc-0006-epistemic-health-empiricism-falsification-and-semirings.md`.
It governs deferral, not new first-slice ingestion requirements. Use
`docs/documentation-style-guide.md` for ADR status/date conventions and prose.
Do not silently close the TOR's exact evidence-schema question: ratification
settles shared semantics; executable normalization and exact encodings remain
later work.

## Proposed boundary decisions

### Ownership and budget (B1)

Accept the dependency rule and prohibit adapter-to-adapter calls. Clarify that
collector orchestration belongs to an application service wired at the
collector composition root; it invokes injected source and ingestion ports, not
concrete adapters directly. Preserve the domain ownership of shared port
contracts and the application ownership of the two ingest use cases.

Record a port ledger in design §5.3: push has Corbusier, Axinite outbox, and
manual/MCP adapters; source reading has Codex, Claude, and Axinite pull;
collected ingestion currently has the collector as its single driving adapter.
Keep that boundary internal until it meets the existing rule of two adapters,
two distinct use cases, or one use case and a roadmap-scheduled second adapter.
A required logical boundary need not be a public crate API. Do not invent a
second adapter merely to satisfy the budget. Here, internal means a workspace
implementation contract rather than a supported external crate API; it does not
permit duplicate delta definitions or source-specific ingestion contracts
across crates.

### Authority and context (B2)

Accept tenant/workspace pairing and immutable propagation of authenticated
`RequestContext`. At the driving boundary resolve the tenant, principal or
service identity, session, correlation, optional causation, and authorized
workspace scope. A configured local tenant is selected only by explicit local
mode; missing Corbusier credentials do not fall back to it. Worker jobs carry
the resolved context through retries, storage calls, and audit events.

Canonical scope fields describe provenance and must agree with the resolved
context. A mismatch is a typed rejection, never an instruction to replace
context. Allowed-workspace hints cannot broaden authenticated permissions. Keep
database, vector, graph, and clustering enforcement mechanisms in adapters;
only SQL evidence-store contracts gate this slice's first implementation.
Retain later-store isolation requirements without implementing those stores.
Acceptance ratifies isolation invariants, not a fixed Qdrant collection layout
or Oxigraph/Chutoro operational strategy. Mark those choices as deferred to
their owning delivery slices within ADR 006 rather than accepting them by
implication.

### Canonical delta and command contracts (B3)

Preserve the three signatures in design §6.1 and its producer-minimum table.
Keep source, scope, metadata, ordered events, cursor, and correction/tombstone
fields, plus all eight named content-part kinds. Missing optional source data
stays absent; adapters must not fabricate timestamps, actors, or authority.
Unknown payload stays redacted provider metadata unless distinct domain
behaviour warrants a reviewed content-kind change.

Push accepts one logical delta under `memory.ingest`; its request identity is
separate from provider event identity. Collected ingestion additionally checks
collector identity, batch identity, sequence, source binding, and cursor
preconditions. A push token is not a collector token. Push cursor metadata is
provenance only and cannot advance collector resume state. Deduplicate push
requests by tenant, authenticated caller, and request ID; independently
deduplicate events by their tenant-scoped provider keys. Reusing a request ID
with changed logical content is a conflict, even when its events have already
been seen. Pull-mode Axinite uses the source and collected path; outbox-mode
Axinite uses push. Manual import validates configured roots and uses push with
an explicit import request ID. Filesystem reading remains at the edge even for
manual imports.

Define report semantics in prose: accepted, identical retry, or typed failure;
report counts distinguish new and already-seen events. Distinguish malformed
input, scope/capability denial, conflicting duplicate, stale cursor/sequence,
and retryable storage failure. Error text must not echo raw source payloads. No
serialization or exact Rust enum layout is settled here. Require limits on
encoded input bytes, events per delta, deltas per batch, and source page size.
Reject oversized input before acceptance effects or cursor advancement. Numeric
defaults belong to configuration/runtime delivery; test below, at, and above
each configured bound, including byte-heavy events with low event counts.

### Cursor and idempotency contract (B4)

Recommend atomic acceptance of a collected batch within one tenant and one
authorized workspace: evidence, spans, jobs, accepted batch identity, and
cursor transitions commit together. Invalid member or storage failure leaves
these acceptance effects unchanged; a separate content-free rejection audit may
still be recorded. Keep the initial batch scope to one workspace; the collector
partitions work before submission. Cross-workspace atomicity has no
demonstrated first-value consumer. Namespace operation identity by tenant,
operation kind, and authenticated producer or collector-stream identity. Bind
it to the canonical logical payload and entire expected/next cursor vector.
Same identity with a changed vector or payload is a conflict, never an
identical retry. Sequence numbers belong to a stable (tenant, workspace,
collector-stream) namespace, survive worker restart, and advance on accepted
fresh batches only. A new worker process cannot reset that namespace; source
reconciliation or a new stream requires explicit audited registration.
Acknowledgement follows durable commit. A lost acknowledgement permits replay
of the same batch with no duplicate effects.

Bind cursors to tenant, workspace, source identity, and source generation.
Treat provider checkpoint tokens as opaque. `after` means exclusive of the last
committed complete item. Only the source adapter interprets its ordering; never
compare opaque tokens lexically or across sources. Require an expected previous
cursor or equivalent version check to reject competing stale updates. An
identical committed retry is recognized before that stale-update rejection. Do
not regress the cursor when replaying older accepted evidence.

A partial trailing record is not consumed. Malformed complete records block
advancement until an explicit, audited disposition exists. A mandatory safety
exclusion may advance only with a durable content-free disposition committed
with the cursor, so no prohibited text is retained. Source truncation,
rotation, or identity-fingerprint mismatch requires reconciliation of the
source generation; never reset an offset silently. Ordinary append or a
changing content digest alone does not create a new source generation. Keep
stable source-incarnation identity distinct from observed content fingerprints.
Empty reads do not advance a cursor unless they carry an explicit valid source
checkpoint/disposition.

Within a tenant, retain provider-specific event keys from design §7. Bind each
key to its full source/workspace identity and detector-independent logical
content identity: an identical retry is a no-op; the same key with conflicting
identity/content is rejected. A correction or tombstone is an explicit new
versioned operation referencing the old event, not an overwrite disguised as a
retry. Preserve provenance and ADR 015's purge recapture fence. Exact hash
encoding remains 2.2.3; redacted bytes must not define ingestion identity.

## Verification plan

### Abstract model and proof sketch before executable design

Model state independently of SQL and Rust as accepted evidence map E,
processed-operation map O, source cursor map C, and durable dispositions D,
partitioned by tenant and workspace. Inputs carry authenticated authority A,
source identity/generation S, logical events, operation identity, and expected
cursor state. Observable acceptance effects comprise E, O, C, D and the jobs
and acceptance audit records tied to the transaction.

Preconditions are valid authenticated authority, permitted workspaces,
well-formed bounded input, and valid source-specific successor relations. Fresh
operations require matching expected cursor state; identical committed retries
are a separate admissible case. Postconditions are either one atomic acceptance
transition, an identical-retry no-op on acceptance effects, or a rejection
leaving those effects unchanged. Rejection auditing is separate.

Proposed semantic lemmas, to refine during implementation, are scope
preservation (a transition affects only authorized partitions), retry
idempotence (applying the same accepted logical operation twice has the same
acceptance state as once), commit-prefix safety (C never passes an item without
accepted evidence or explicit disposition), and stale-writer exclusion (two
operations based on one cursor version cannot both commit incompatible next
states). These are proof hypotheses, not a fixed decomposition.

The representation relationship must map tenant-scoped rows and unique keys to
E/O, committed cursor rows and source generations to C, and exclusion records
to D. Runtime validation establishes the abstract preconditions; transaction
adapters implement each abstract atomic step. Proving a pure transition
function does not establish that SQL transactions or provider parsers satisfy
this relationship.

### Obligations, methods, and non-vacuity

B1 uses document review now and Cargo-metadata/source-path architecture tests
in 1.2.7. Later negative fixtures deliberately import an adapter or SDK into
domain/application code; the fitness check must reject them. A legal inward
edge must pass. No Verus proof is warranted for import checking.

B2 uses worked two-tenant examples now, then `rstest` tables and `rstest-bdd`
scenarios in 1.2.8 and 2.2.2. Cover trusted local, trusted Corbusier, missing
context, forged tenant, widened workspace, and worker/push token substitution.
Use the same workspace and provider identifiers in tenants A and B. A seeded
implementation that trusts payload tenant must fail the denial case. Later
SQLite and PostgreSQL tests exercise actual guards, including a non-owner
PostgreSQL role without BYPASSRLS and pooled transaction context reuse.

B3 uses five independently written fixture specifications in EP-M2. Roadmap
1.2.6 establishes the reusable fixture corpus; later 2.2.1/2.2.2 implement
`rstest` cases with `googletest` matchers and `pretty_assertions` for
structural diffs. Compare shared semantic projections, not transport IDs or
invented missing fields. Include each producer's minimum fields, optional
absence, content variants, and redacted unknown metadata. Wrong event order or
dropped tool-result linkage is a required negative control. Use `insta` only
for meaningful stable multivariant representations; never approve a snapshot
merely because it was generated by the mapper.

B4's unbounded retry/scope/commit-prefix properties warrant Verus for a small
pure transition kernel in 2.1.3/2.2.2. Before implementation, exhibit empty and
non-empty valid states, an accepted write, identical retry, rejected conflict,
and two competing writers. A deliberately advanced cursor without evidence or
disposition must fail verification. Do not assume the conclusion, hash
injectivity, or database correctness as a theorem. Defer solver triggers,
auxiliary induction lemmas, and collection algebra until actual verification
conditions exist. No proof artefact or proof check exists in this slice.

The future proof milestone must pin a Verus version and add an explicit project
command, proposed as `make verify-ingest`, which invokes Verus on the actual
kernel and fails on any undisclosed trusted body or undischarged goal. Its
owning ExecPlan must replace this proposed command with the implemented command
before approval. Acceptance requires zero verification errors, a reviewed
assumption list, executable-to-model correspondence, and ordinary
`make check-fmt`, `make lint`, and `make test` success. A proof of an unrelated
model or merely assumed postcondition is not acceptance evidence.

Pair that proof with `proptest` operation histories: retries, conflicts,
interleaved tenants, generation changes, exclusions, and corrections. Start
with bounded lengths 0–64, two or more tenants, and retained regression seeds;
record class coverage and reject excessive filtering. Generated tests supply
integration confidence, not an unbounded theorem. Kani is not currently
selected: the desired semantic guarantee is unbounded and bounded histories
already serve a distinct executable-test purpose.

Later end-to-end tests must exercise actual daemon/collector wiring and both
SQL backends: failure before commit, commit followed by lost acknowledgement,
restart/retry, a competing stale batch, and replay after source rotation. Check
evidence/job counts, cursor position, tenant isolation, and reports. Use
`pg-embed-setup-unpriv` guidance for PostgreSQL fixtures. Crash tests must
observe real durable state; in-memory mocks cannot discharge this boundary.

External assumptions are authenticated token validation, source adapter
successor correctness, database atomic commit/durability, filesystem
observation semantics, and the Rust/Verus toolchains. Test repository-owned
integration against those interfaces. Hashes are not assumed collision-free
authority; retain enough identity for conflict detection and document residual
risks of any later digest-only representation. Parsing, diagnostics, wiring,
and formatting use conventional tests, not artificial formal lemmas.

### Behavioural specification handed to later slices

The following future feature belongs at
`tests/features/conversation_ingestion.feature`; it is specification text in
this slice, not a runnable test file. Later bindings must add equivalent
scenarios for every rejection and transition class above.

```gherkin
Feature: Canonical ingestion preserves authority and progress
  Scenario: Provider scope cannot replace authenticated scope
    Given a push caller authenticated for tenant A and workspace W
    When its delta claims tenant B
    Then ingestion is denied and no acceptance effects change

  Scenario: Lost acknowledgement does not duplicate a collected batch
    Given a collector batch committed evidence and cursor C for tenant A
    And the acknowledgement was lost
    When the same collector retries the identical batch
    Then no evidence or jobs are duplicated and cursor C is unchanged

  Scenario: Invalid batch member prevents progress
    Given a valid collector batch with one invalid delta
    When the batch is submitted
    Then no member is accepted and its prior cursor is retained

  Scenario: A source record is incomplete
    Given a source ends with a partial trailing record
    When the collector reads after the last complete record
    Then the cursor remains before the partial record
```

For later code, use Red-Green-Refactor: add the named failing test and run it
before production edits; record the expected semantic failure; implement the
minimum fix; rerun the focused suite, then all gates. The runtime ExecPlan must
name actual Cargo test targets once crates exist. This documentation slice uses
reviewable counterexamples and consistency checks instead: adding tests against
nonexistent APIs would widen 1.1.3 into 1.2/2.2.

## Plan of work and milestones

### EP-M1 — Accept a coherent decision set

After approval, recheck prerequisites and repository state. Revise and accept
`docs/adrs/adr-0005-hexagonal-architecture-boundary.md`,
`docs/adrs/adr-0006-tenant-isolation-and-corbusier-context.md`, and
`docs/adrs/adr-0007-standard-conversation-ingestion-port.md`. Record the actual
acceptance date and summary; preserve alternatives and consequences. Add the
port ledger, authority matrix, and cursor/idempotency decisions above.
Synchronize design §§5.3–6.1, §7 where the key explanation needs qualification,
and §§16–17 for remaining decisions. Update TOR §§8–9 only where acceptance
changes the stated assumptions or open questions.

This plateau discharges B1–B4 as documented contracts. Acceptance evidence is
an explicit disposition for each ADR, consistent normative text, and worked
accept/reject examples for authority, retry, and cursor progress. Review each
requirement link and ensure ADRs 013/015 remain unchanged in meaning. Run all
six gates below and commit. If review fails, revise the document set together;
revert the atomic documentation commit if rollback is needed. Runtime
implementation remains deferred. No compatibility layer is required.

### EP-M2 — Publish conformance and verification handoffs

In `docs/developers-guide.md`, add a boundary-contract section with all five
producer mappings, the obligation-to-owner map, negative controls, and the
proof/test relationship above. Each fixture specification must name source
version/provenance, input, expected shared fields, provider-only fields,
expected rejection, and its owning roadmap item. Use one common human-message
example plus tool call/result, compaction, correction/tombstone, absent
metadata, unauthorized scope, and malformed/truncated-source cases. Include
push cursor injection, same operation ID with changed payload or cursor vector,
collector restart, and ordinary append alongside rotation/truncation. Do not
claim a synthetic example proves compatibility with a real provider version.

Document shared delta and fixture reuse: domain owns canonical values,
application owns acceptance policy, adapters own parsing, and a later shared
test harness owns conformance assertions. Search for existing equivalents
before adding any future abstraction. Concrete adapters reuse that harness;
production code must not depend on fixture helpers. Keep core proof obligations
and representation choices in the design as well as the developer guide.

Update `docs/users-guide.md` with planned authority, duplicate/conflict, retry,
and stalled-source behaviour; explicitly state these are not yet implemented.
Update `docs/contents.md` with the plan and relevant guide sections. Use the
required reference documents listed below when writing handoffs.

This plateau discharges B2–B4's implementor guidance. Acceptance means all five
producer families have independently specified common-semantic expectations,
all failure classes have observable outcomes, and every future test/proof has
an owning roadmap slice. Run all gates, check scope/trace links, and commit.
Recover by revising or reverting this documentation commit; executable fixtures
and proofs remain deferred, with no compatibility shim.

### EP-M3 — Reconcile and close the decision task

Review the full change with all six Logisphere experts: Pandalump for
architecture, Wafflecat for simpler alternatives, Buzzy Bee for bounded work,
Telefono for contracts, Doggylump for recovery, and Dinolump for maintenance.
Resolve material findings and record the reasoning in Decision log. Verify that
acceptance has not silently introduced runtime work or closed deferred schema,
hash-encoding, graph, or source-health decisions.

Only after B1–B5 have their evidence, mark roadmap 1.1.3 done in
`docs/roadmap.md`, preserving its dependencies and adding a concise decision
summary and links. Use the `mapsplice` skill for this structural roadmap edit.
Update this plan's outcomes and progress. Run all gates again, commit, and
publish the implementation PR for the documentation decision slice. Status
COMPLETE means ratification is delivered, not that ingestion exists. If any
upstream assumption was falsified, update the TOR/design/ADR and obtain any
required deviation approval before completion. Recovery is an ordinary revert
of this closure commit; never erase review evidence or force-reset others' work.

## Concrete steps and validation

Run from the repository root. Delegate gates to scrutineer; run them strictly
sequentially, with pipefail and separate logs. This same sequence applies to
the planning candidate and every major execution milestone:

```bash
set -euo pipefail
make fmt 2>&1 | tee /tmp/fmt-memoryd-1-1-3.out
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-3.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-3.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-3.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-3.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-3.out
```

Stop at the first failed command; inspect its log before changing anything.
Expect each exit status to be zero, no formatting differences, no Markdown or
Mermaid errors, no Rustdoc/Clippy/Whitaker warnings, and no failed tests.
Record actual test counts, not predicted counts. Archive milestone logs with
distinct suffixes before the next run. Inspect formatter changes for unrelated
drift. The Makefile owns the dev-fast Cargo configuration; do not bypass it or
use an isolated Cargo cache. Never apply dev-fast to future verification builds.

At every plateau, check assigned requirements, upstream assumptions, ownership,
public contracts, dependencies, trust boundaries, persisted formats, and trace
links against the approved design. Record any deviation before proceeding.
Documentation gates prove document quality; existing code gates prove no
baseline regression, not the promised future ingestion semantics.

## Required reading and interfaces

Read `docs/developers-guide.md` for the actual workflow;
`docs/rust-testing-with-rstest-fixtures.md` for shared setup and tables;
`docs/reliable-testing-in-rust-via-dependency-injection.md` for deterministic
clock, filesystem, environment, and failure injection;
`docs/rust-doctest-dry-guide.md` for executable API examples;
`docs/complexity-antipatterns-and-refactoring-strategies.md` for bounded
helpers; `docs/pg-embed-setup-unpriv-users-guide.md` for database fixtures; and
`docs/rstest-bdd-users-guide.md` for future Gherkin bindings.

No dependencies are added here. Later tests use `rstest`, `rstest-bdd`,
`googletest`, and `pretty_assertions`, with `insta` and `proptest` where the
specified obligations warrant them. Versions must use caret requirements. The
existing normative port shape remains the interface reference; do not turn
illustrative signatures into a promise of a serialized protocol.

Firecrawl research on 2026-11-02 checked the upstream
[GoogleTest Rust documentation](https://github.com/google/googletest-rust),
which documents `rstest` integration and placing `#[gtest]` before `#[rstest]`,
and the [Verus guide](https://verus-lang.github.io/verus/guide/), which
describes static verification against specifications. These support the future
testing and proof approach; neither validates Memoryd's provider contracts or
supplies a proof of its transaction implementation.

## Idempotence, recovery, and publication

Documentation edits and gates are repeatable. Preserve existing changes and
review the complete diff before each atomic commit. Commit via a temporary
message file after gates pass. No data migration or destructive operation is
needed. Inspect new upstream changes before resuming after a pause.

For this planning delivery, push `1-1-3-accept-or-revise-boundaries` over SSH
with the authenticated `leynos` identity, refresh its origin tracking ref, and
set upstream to `origin/1-1-3-accept-or-revise-boundaries`. Use standard Git/gh
with the clean environment required by AGENTS.md. Create a draft PR titled
`Plan: Accept or revise boundaries (1.1.3)`, link this document using the
branch URL, and end its body with `## References` containing the Lody session
URL. Read back draft state, head, and upstream. The PR contains only this
ExecPlan and its contents-index entry. Execution remains approval-gated.

## Revision notes

- 2026-11-02: Initial plan separates ratification from runtime delivery and
  adds proof-sketch-first handoffs for authority, retry, and cursor semantics.

- 2026-11-02: Reconciled all six expert lenses; clarified operation identity,
  cursor ownership, limits, source generations, and deferred delivery owners.
  The documentation-only scope and approval gate remain unchanged.

- 2026-11-02: Recorded passing planning gates and 150 tests; execution remains
  unapproved. Final publication evidence belongs to the draft PR handoff.
