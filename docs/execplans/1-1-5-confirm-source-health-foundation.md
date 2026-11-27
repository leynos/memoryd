# Confirm the source-health foundation for observe mode

Status: DRAFT — awaiting explicit approval before execution.

This living plan keeps Constraints, Tolerances, Risks, Progress, Surprises &
discoveries, Decision log, Outcomes & retrospective, Conformance basis, and
Verification plan current at each milestone.

## Purpose / big picture

Ratify roadmap 1.1.5 so later evidence capture can report an unobserved source
separately from a successful observation that found no new evidence. A healthy
source is not proof that nothing happened in the world. Success here is an
accepted ADR 008, consistent design and guide contracts, worked examples, and
explicit verification owners before projection implementation starts.

This is a documentation decision slice, as in completed roadmap 1.1.3. Approval
authorizes the three documentation milestones below. It does not implement
storage, collectors, health commands, or proofs. Those have later roadmap
owners. No application behaviour is claimed shipped by this plan.

## Constraints

Preserve ADRs 005–007's inward dependencies, authenticated tenant context,
workspace authority, ingestion atomicity, cursor rules, and port budget.
Preserve ADR 013's SQLite/PostgreSQL parity and ADR 015's safety, redaction,
exclusion, and purge rules. Source health must work without projection, Qdrant,
Ollama, raw transcripts, or evidence-text reads.

Do not add Rust, executable tests, dependencies, migrations, wire encodings,
provider parsers, or speculative Verus files in this slice. Keep coverage
expectations, omission alerts, claim-class completeness, and omission scoring
in RFC 0006. Do not make 1.1.4 an extra prerequisite or ratify its tool set. Do
not mark roadmap 1.1.5 done until approved execution completes.

No `assume`, `admit`, external/trusted proof bodies, or equivalent shortcuts
may discharge a proof. Unproved statements remain prose until their owning
runtime milestone. No speculative compatibility wrappers are required.

## Tolerances (exception triggers)

Stop, record a proposed deviation, set status BLOCKED, and seek approval if
execution needs runtime work, a dependency, a new trust boundary, changed
accepted ADR policy, or a materially different contract from this proposal.
Ordinary wording and cross-reference corrections are within scope.

Limit execution to ten non-ExecPlan documentation files and 1,000 net added
lines outside ExecPlans. ExecPlans are excluded from both counts. There is no
time limit. Do not expand scope to fix unrelated baseline gate failures.
Unresolved conflicting requirements after expert review require escalation.

## Risks

False assurance from a healthy label is high impact and plausible. Define
observation success separately from evidence yield, cursor advancement, and
coverage. A missing report must never become a successful empty observation.

Old successes can mask current failure (high impact, medium likelihood). Define
status precedence, observation ordering, expiry, and recovery together. A
shared provider name can leak another tenant's state (high impact, medium
likelihood); require composite ownership and hostile two-tenant examples.

Default durations may be mistaken for measured service guarantees (medium
impact, medium likelihood). Label them product defaults, permit validated
per-source overrides, and avoid inferring push-source heartbeat support.
Accepted documentation may look like shipped code (medium impact, high
likelihood); label all user-facing guidance as planned until runtime delivery.

## Progress

- [x] (2026-11-04) Rename branch and inspect instructions and baseline.
- [x] (2026-11-04) Load requested skills and commission two Wyvern reviews.
- [x] (2026-11-04) Inspect prerequisite 1.1.3 and source-health authorities.
- [x] (2026-11-04) Research staleness prior art using Firecrawl.
- [x] (2026-11-04) Six expert perspectives reviewed the draft; ordering,
  delayed delivery, cadence, precedence, and lifecycle findings reconciled.
- [x] (2026-11-04) Initial planning gates passed: formatting, Markdown,
  Mermaid, Rustdoc, Clippy, Whitaker, and 150 tests with zero skipped. Logs use
  `/tmp/{gate}-memoryd-1-1-5-plan.out`. Cargo reported the existing non-fatal
  redundant-homepage manifest warning. Final evidence and wording updates
  require the same sequential gates before publication.
- [ ] Publish the gated draft; publication evidence accompanies the PR.
- [ ] Obtain explicit execution approval.
- [ ] EP-M1: Ratify the source-health contract and synchronize design.
- [ ] EP-M2: Record conformance cases, proof sketches, and runtime handoffs.
- [ ] EP-M3: Reconcile documentation and mark roadmap 1.1.5 done.

## Surprises & discoveries

The baseline is `088a4faeaa484000f99dbbc69b1abff92b3ebfaa`. `src/main.rs`
prints a greeting; no source-health types, repositories, migrations, or Verus
target exist. ADR 008 is Proposed. Roadmap 1.1.3 is complete and ADRs 005–007
are accepted. Design §17 still leaves freshness defaults open. The SQL outline
omits several fields named in ADR 008 and does not express same-tenant
source/snapshot ownership constraints.

CodeGraph tools were unavailable after discovery. Wyvern used targeted file
reads and shared evidence through context packs. `make lint` includes Rustdoc,
Clippy, and Whitaker; `make test` selects nextest when installed. Use actual
Makefile targets rather than simplified generic Cargo commands.

## Decision log

- 2026-11-04: Scope this plan as ratification, following roadmap §1.1 and
  the completed 1.1.3 precedent. Runtime work remains in 1.2 and phase 2.
- 2026-11-04: Propose revising and accepting existing ADR 008 rather than
  adding a redundant ADR. A substantive departure from accepted boundaries
  requires a separately approved decision.
- 2026-11-04: Separate observation freshness from event-production cadence.
  A quiet source can be healthy; push silence without a heartbeat agreement
  cannot establish either failure or completeness.
- 2026-11-04: Keep formal obligations as independent specifications here.
  The later implementation must prove the pure policy and test its adapters;
  documentation ratification does not claim any theorem is discharged.

## Outcomes & retrospective

The reviewed planning draft passed its initial validation, including all 150
tests. The final evidence update must pass the same gates before publication.
No runtime behaviour, ADR acceptance, roadmap completion, or proof result is
delivered yet. Record review reconciliation, validation evidence, and execution
outcomes here as they become available.

## Context and orientation

Start with `docs/contents.md` and `docs/repository-layout.md`. The normative
source-health decision will be
`docs/adrs/adr-0008-source-health-and-coverage-foundation.md`.
`docs/memoryd-design.md` owns architecture and logical schema; RFC 0001's
Proposed design section lists the evidence-inbox entities. Its roadmap
reference to §6 means that heading, not a currently numbered section.

A registry entry identifies a configured source. An observation is a bounded
report of an attempted check. A snapshot preserves normalized diagnostic facts
and their capture time. A health report evaluates those facts at a specified
query time. A cursor is an ingestion resume position, not a freshness clock. A
source generation distinguishes incarnations after replacement or reset.

Load `execplans`, `codegraph-mcp`, `hexagonal-architecture`, and `rust-router`.
Use `firecrawl-mcp` for external gaps and `logisphere-experts` for review. Use
Wyvern for reconnaissance, context_pack for team evidence, and scrutineer for
sequential gates. Later Rust implementors should route through
`rust-verification`, then `verus` and `proptest` as appropriate, and `leta` for
navigation. Inspect toolchain posture before borrow-sensitive design; this
documentation slice requires no compiler or ownership workaround.

Read the following guides before writing later executable tests:
`docs/rust-testing-with-rstest-fixtures.md`,
`docs/reliable-testing-in-rust-via-dependency-injection.md`,
`docs/rust-doctest-dry-guide.md`,
`docs/complexity-antipatterns-and-refactoring-strategies.md`,
`docs/pg-embed-setup-unpriv-users-guide.md`, and
`docs/rstest-bdd-users-guide.md`. Use injected clocks and environments,
fallible shared fixtures, public-behaviour assertions, real unprivileged
PostgreSQL integration, and independent expected outcomes.

## Conformance basis

All upstream references below use baseline commit
`088a4faeaa484000f99dbbc69b1abff92b3ebfaa`; record any execution-time revision.

- SH1: Registry and snapshot semantics: roadmap 1.1.5; design §§4 and 7;
  ADR 008; RFC 0001 Proposed design. EP-M1 ratifies the field contract; EP-M2
  supplies registry, empty-observation, and missing-report examples.
- SH2: Status and freshness: ADR 008; design §§7 and 15. EP-M1 defines the
  total classifier and defaults; EP-M2 supplies boundary and recovery cases.
- SH3: Scope and purity: terms of reference §§8–9; design §§5.3–5.4;
  ADRs 005–007 and 015. EP-M1 fixes ownership; EP-M2 supplies isolation and
  sanitization counterexamples.
- SH4: Exposure: design §§11–12 and 15; roadmap 2.3.3 and 2.4.2. EP-M1
  defines read semantics; EP-M2 specifies CLI/MCP and observe-mode evidence.
- SH5: First-value boundary: terms of reference §§8–9; design §§16–17;
  RFC 0006. EP-M3 audits deferrals and closes only roadmap 1.1.5.

RFC 0006 is
`docs/rfcs/rfc-0006-epistemic-health-empiricism-falsification-and-semirings.md`.
It supplies the deferral boundary, not additional first-slice requirements.

## Proposed contracts to ratify

### Registry, observations, and ownership (SH1, SH3)

Retain ADR 008's `SourceId`, `SourceKind`, `SourceRegistryEntry`,
`SourceObservation`, `SourceHealthSnapshot`, `SourceHealthStatus`, and
`SourceHealthReason`. Specify semantics before Rust signatures in 1.2.1. Each
entry has tenant, optional workspace, opaque source identity, provider, source
kind, protected locator, enabled state, generation, freshness policy, and
creation/update times. Separate registry enabled state from health status; do
not overload the existing ambiguous registry `status` column.

Registry identity is scoped to a tenant. Workspace, when present, must belong
to that tenant. Snapshot identity must reference the same tenant, source,
workspace scope, and generation. A null workspace means explicitly tenant-wide,
never global or permission to read arbitrary workspaces. Provider payloads
cannot create or change tenant authority. Unregistered or wrong-scope reports
are rejected without registering sources implicitly.

Snapshots preserve successful discovery/read/parse times, durable cursor update
time, optional measured lag, effective freshness policy, observation outcome
and ordering identity, capture time, normalized reason codes, and
redaction/deny-skip counts. Missing values mean unknown or not applicable,
never timestamp zero or fabricated success. Record a completed successful check
separately from last parse: a legitimate empty read can complete a check
without parsing any new event or advancing a cursor.

Reuse ADR 007 source identity and generation, but give health observations a
separate durable revision scoped by tenant, source, source generation, and
health-only observer epoch. The registered authenticated observer allocates
revisions at check start and persists them across restart. Only one observer
owns a health epoch; changing ownership advances an authorized health-only
epoch, rejecting old writers. Empty checks and failures advance health
revisions without advancing the accepted-batch sequence. Specify ordered, whole
observations: identical retries are no-ops; an older report cannot clear a
newer fault; conflicting reports at the same identity are rejected. A new
generation starts without inherited success. Failed checks preserve historic
success times but record current failure. Health receipt never advances the
evidence cursor or ingestion sequence; only ADR 007's durable acceptance can do
that. Disable/re-enable invalidates prior health eligibility and requires a new
observation in a new health epoch before healthy is possible; this health reset
must not discard or bypass durable ingestion checkpoints or purge fences. ADR
007 source generation changes only on source-incarnation reconciliation, never
on health-only disable/re-enable or observer handoff. Represent reasons as a
bounded set from the normalized vocabulary, and skips as checked counts, never
per-record text. Roadmap 2.1.5 and 2.3.3 must fix and test numeric
report/reason limits before accepting external observations.

### Freshness and status (SH2)

Propose a 300-second observation freshness default for enabled periodic Codex
and Claude filesystem checks and Axinite pull checks. This is a Memoryd product
default, not a provider service guarantee. Corbusier push, Axinite outbox push,
and manual imports default to on-demand freshness with no recurring deadline.
Do not infer staleness from event silence in those modes. Unknown providers
must declare periodic or on-demand policy explicitly. Per-source periodic
overrides must be positive whole seconds; zero and negative values are invalid.
Omission selects the mode default; explicit on-demand is distinct from a
missing configuration value. A periodic policy must declare cadence and a
positive completion/delivery allowance whose sum fits the deadline. A
600-second cadence with a 300-second deadline is misconfigured; an explicit
660-second deadline with 60-second allowance is valid. Scheduling and retries
are later runtime work, not new components here. On-demand healthy means the
last attempt succeeded, with ongoing freshness explicitly unassessed.

For periodic checks, use a conservative daemon-established check-start anchor
bound to the authenticated source, source generation, health epoch, and health
revision. Capture it before source inspection, retain it with the completed
observation, and never replace it with receipt or retry time. The collector
application obtains the anchor through its domain-owned reporting boundary; the
exact transport encoding belongs to the later runtime slice. Daemon-local
checks use the same clock directly. A success without a verifiable anchor has
unknown freshness and cannot be healthy under periodic policy. This
deliberately measures an upper bound on time since successful inspection rather
than trusting remote wall clocks or designing a distributed clock protocol.

Freshness age is query time minus that anchor for the latest successful
completed check in the current source generation and health epoch. Age equal to
the deadline is fresh; greater is stale. An anchor in the future or a detected
clock regression produces `unknown` with a clock reason. Event timestamps and
file modification times remain diagnostics. Read-only queries recompute age
without mutation. For example, a check anchored at 100 and received
successfully at 500 is stale under a 300-second deadline; its retry at 501
remains stale. A report whose anchor was lost across restart is unknown, never
newly fresh. Reports expose this conservative age basis and whether freshness
is unassessed.

Classify in this order, keeping additional reasons alongside the primary
status: invalid configuration is `misconfigured`; disabled or inaccessible
source is `blocked`; no usable observation in the current source generation and
health epoch, invalid clock, or a successful periodic observation with an
unverifiable anchor is `unknown`; an expired periodic success is `stale`;
current partial read, parse, or ingestion failure is `degraded`; otherwise a
successful completed check is `healthy`. A failed attempt before any successful
check is `degraded` when the attempt itself is usable; no attempt is `unknown`.
For periodic sources with usable failure but no success, report `degraded`
until success, with freshness unknown. Never invent a deadline anchor.

Successful empty checks are healthy within their freshness policy. Explicit
policy skips are counted and disclosed; they are not parse failures or claims
of full coverage. A partial read or rejected parse adds a degradation reason
even when other records succeeded; the primary status follows the precedence
above. Success anchored at 100 followed by a partial failure at 401 is stale
with a parse-failure reason under a 300-second deadline. A later fully
successful check clears current transient faults; historical error metadata
must not keep it degraded. Disabled sources remain visible as blocked with an
explicit disabled reason. Unconfigured providers produce no invented source
rows; an empty registry reports no configured sources, not an aggregate
assertion of health.

### Reports and hexagonal boundaries (SH3, SH4)

Adapters report facts; the domain owns classification. Application services
coordinate registry, observation, and query operations through domain-owned
ports. Domain code must not import SQL, provider SDKs, filesystem APIs, or
transport types. Reuse the existing planned evidence-store boundary before
adding a port; respect ADR 005's port budget. The composition root wires
adapters; adapters never call each other. Record the abstraction search,
permitted callers, ownership, and reuse strategy in the developers' guide.

Health reports enumerate authorized configured sources, including those with no
snapshot, by joining from the registry rather than from observations. Require
authenticated tenant context and workspace authorization on every read and
write. Workspace-filtered reports include that workspace and explicit
tenant-wide records only when the caller is authorized for those records.
Cross-tenant identifiers yield no data and no identifying error.

Return evaluation time, source identity/kind, workspace scope, enabled state,
status, normalized reasons, freshness policy and age, success times, lag, and
skip counts. Never expose protected locators, credentials, free-form provider
errors, or transcript content. Distinguish no matching evidence, no configured
source, and configured source not observed. Health cannot prove absence outside
the explicitly observed scope and time. Storage/query errors are errors, never
empty healthy reports. Bound and paginate source listings using existing
developer-guide pagination conventions; apply authorization before pagination
and counts, and use deterministic source ordering.

Selected health surfaces share these normalized source semantics. `Health`
provides the daemon contract; MCP tool inclusion remains governed by 1.1.4, and
`memory_health`, if selected, is implemented by 2.4.2;
`memoryd-collector health` adds safe collector diagnostics. Session browsing
and later `memory_explain` may attach relevant source context without claiming
coverage. Keep source state separate from daemon liveness and dependency
health. Read-only reports do not trigger ingestion, projection, or provider
I/O. Later observability uses structured `tracing` and low-cardinality
`metrics`; identifiers and raw reasons must not become metric labels.

## Verification plan

### Independent model and proof sketch

Model a finite registry R keyed by tenant and source, an ordered observation
history O per source generation and health epoch, query authority A, and
evaluation time t. Let visible(A, R) be the authorized registry subset.
Normalize the latest admissible observation into diagnostic facts F, retaining
prior success facts unless the source generation or health epoch changed.
Define classify(F, policy, t) by the ordered cases above, independently of
database and Rust representation. Evidence content is not an input to
classification.

Preconditions are valid authority, registered source ownership, structurally
valid inputs, and admissible observation identity/order. Policy input is a
tagged valid policy or configuration diagnostic, so invalid configurations and
clock anomalies are explicit classifier inputs, not excluded by proof
preconditions. Postconditions are one of the six statuses with reasons, no
change to evidence/cursors on a health transition, and reports restricted to
visible registry entries. Represent database nulls as optional values, seconds
as checked non-negative integers, and immutable snapshot ownership as a
composite relation. Refinement must connect executable facts and times to this
abstract state, including checked arithmetic and current generation.

Proposed semantic lemmas are hypotheses to refine at implementation time:

- V1, scope preservation: updates affect only their authorized source;
  reports contain no foreign-tenant or unauthorized-workspace entries.
- V2, no fabricated observation: missing success cannot become healthy;
  changing evidence count alone cannot change health; an empty successful check
  and a missing check remain distinguishable.
- V3, ageing: with fixed facts and policy, advancing valid time beyond the
  deadline cannot improve health; equality remains fresh. Fixed fault
  precedence may retain a fault status rather than becoming stale.
- V4, order and recovery: older observations cannot overwrite newer facts;
  identical retries preserve state; source-generation or health-epoch changes
  cannot inherit success; a newer complete success can clear a transient fault.

Verus is warranted for this small pure transition/classification kernel and its
scope projection, because the contracts quantify over all admissible times and
observation histories. The representation relationship is part of the proof
obligation; an unrelated model-only theorem is insufficient. No Verus milestone
executes in this documentation slice. Roadmap 1.2.1 owns the model/type
handoff; 2.1.5 owns the kernel and substantive proof delivery. That future
milestone must add a project-standard `make verify-source-health` target
invoking pinned Verus on `verification/source_health.rs`, without the dev-fast
fragment, and pass it in addition to `make check-fmt`, `make lint`, and
`make test`. The target and path are proposed future artefacts, not existing
commands. Pin/setup instructions belong in the developers' guide. Do not add
solver plumbing until actual verification conditions exist.

For V1, use two tenants sharing the same source identifier and distinct
workspaces; a mutation dropping tenant filtering must be rejected. For V2, use
both a registered source with no observation and a successful zero-record
check; mutating missing observation into success must fail. For V3, use success
at 100 and deadline 300: times 399 and 400 are fresh, 401 stale; reversing the
boundary comparator must fail. For V4, use failure revision 2, late success
revision 1, then success revision 3 and a generation reset; last-arrival-wins
and inherited-success mutations must fail. Exhibit valid precondition witnesses
and inspect proof dependencies; contradiction or an assumption equivalent to a
theorem is not proof. Also witness a zero-duration configuration diagnostic
reaching misconfigured, disable/re-enable reaching unknown, and three health
revisions (failure, empty success, recovery) with unchanged ingestion sequence
and cursor. After disable/re-enable, a delayed report from the old observer is
rejected and health stays unknown until a new-epoch observation arrives.
Evidence identity, ingestion sequence, and cursor remain unchanged. Test
delayed delivery and lost anchors as specified above. Timestamp trust is
bounded by daemon clock assumptions; undetected wall-clock error remains a
stated residual risk.

Trust assumptions are authenticated context construction, truthful adapter
outcomes, authoritative clock sampling, transaction isolation, and ordering
persistence. Proofs do not establish real provider coverage, clock accuracy,
authentication, or database isolation. Real boundary tests must exercise those
interfaces. Treat clock anomalies conservatively. No proof is needed for text
formatting, ordinary wiring, or third-party internals.

### Runtime test handoff and evidence

Roadmap 1.2.6 owns shared fixture conventions, 2.1.1/2.1.2 own lockstep schema
and store adapters, 2.1.5 owns repository/classification tests, 2.3.3 owns
collector diagnostics, and 2.4.2/2.4.3 own MCP and observe-mode end-to-end
checks. Assign V1–V4 to those owners in the developers' guide in EP-M2.

Use `rstest` tables for all six statuses, precedence combinations, exact
freshness boundaries, overrides, no observations, disabled sources, empty
reads, partial parses, skips, future times, and on-demand silence. Use
`googletest` assertions and `pretty_assertions` for rich structural diffs; both
are authorized future dependencies, not additions in this slice. Inject
clock/environment inputs and use fallible fixtures. Add `proptest` properties
for time ranges, tenant permutations, retry/order sequences, and generation
changes with an independent model oracle. Record case coverage and reject
generators that filter away material cases. Kani adds little to this unbounded
pure-kernel proof plus executable property-test combination.

Use real SQLite and unprivileged PostgreSQL integration tests for scope,
foreign-key ownership, restart, transaction rollback, and deterministic
latest-snapshot selection. A query failure must not become an empty result. Use
`insta` snapshots for redacted human and structured health reports when both
formats exist; normalize only nondeterministic fields. Include secret sentinels
in provider errors and paths and assert their absence from every output. Tests
establish rendering and integration, not semantic completeness.

The future `tests/features/source_health.feature`, bound through `rstest-bdd`,
must include this independently specified scenario:

```gherkin
Feature: Source observations remain distinct from evidence yield
  Scenario: Quiet observation and missing observation
    Given tenant A has configured sources quiet and unseen
    And quiet completed a successful empty check at time 100
    And unseen has never reported an observation
    And both sources have freshness deadline 300 seconds
    When tenant A requests health at time 400 in observe mode
    Then quiet is healthy with zero new records
    And unseen is unknown with reason not_observed
    And neither result claims evidence coverage
    And no projection dependency is contacted
```

Add scenarios for time 401, blocked access, partial parse, recovery, restart,
wrong-tenant lookup, and policy skips. End-to-end tests must run ingestion,
restart, and read-only CLI/MCP inspection with projection disabled, then prove
that healthy-empty and unobserved remain different through persistence and
serialization. Do not add a new first-slice `memory_explain` dependency.

At each future code milestone, first add the smallest failing test or open
proof obligation and capture its intended failure. Implement the minimal
change, rerun the focused test/proof, then refactor and run all gates. Use
`make test TEST_FLAGS='--all-targets --all-features source_health'` for focused
Rust cases named with that prefix; `make test` runs the full suite. Record
actual discovered test names and counts. The future proof milestone requires
zero failed proof obligations and the mutation evidence above; ordinary tests
remain mandatory even with proved semantic lemmas.

For this documentation slice, Red-Green-Refactor is a contract-gap review:
record the currently unspecified cases, replace them with explicit decisions
and independent worked outcomes, then check all derived documents agree.
Executable tests cannot fail on prose policy that is not implemented yet.

## Milestones and plateaus

### EP-M1: Ratify the contract

After approval, revise ADR 008's proposed direction into an accepted decision
with date, reasons, alternatives, and consequences. Preserve its seven named
types and six statuses. Add the field, ownership, observation-order, freshness,
status-precedence, and report contracts above. Synchronize design §§4, 7, 12,
15, and 17, including logical schema constraints, policy and skip fields
missing from the outline, and the resolved freshness question. Update RFC
0001's Proposed design with a concise normative ADR link; keep RFC 0006
deferred. Update terms of reference only where an open question is actually
settled by these decisions.

Acceptance E1: SH1–SH4 each have one authoritative rule and consistent derived
summaries, including quiet-versus-missing worked outcomes. Run the sequential
gates below, inspect the diff, and commit this coherent documentation plateau.
Check no accepted trust, safety, or storage rule changed and no runtime or wire
format was added. Remaining gap: verification examples and guide handoff.
Recovery is a normal revert of this documentation commit; no data migration or
compatibility layer is needed.

### EP-M2: Publish conformance and verification handoffs

Add a source-health section to `docs/developers-guide.md` containing the
independent model, V1–V4, assumptions, representation relationship, negative
controls, fixture ownership, reuse policy, and runtime owner mapping. Include
worked inputs and expected outputs for every case listed above, not merely a
list of test names. Add planned source-health interpretation to
`docs/users-guide.md`, explicitly saying it is not implemented and showing
healthy-empty, unknown, stale, blocked, and degraded examples without raw
content. Link new sections from `docs/contents.md` as appropriate.

Acceptance E2: a new contributor can evaluate the worked cases without reading
future Rust code and locate each deferred proof, test, and adapter owner.
SH1–SH4 must retain E1 meaning across all examples. Run all gates, review
conformance, and commit. Remaining gap: closure audit. Recovery is a revert of
this milestone; it changes no public runtime API.

### EP-M3: Reconcile and close only the decision slice

Audit design §§16–17 and RFC 0006 deferrals, reconcile expert findings and
upstream assumptions, and verify prerequisite 1.1.3 remains done. Load
`mapsplice` before editing the roadmap. Mark only 1.1.5 done in
`docs/roadmap.md`, with a concise decision summary and ADR/plan links. Do not
mark runtime implementation done. Keep `docs/contents.md` and this plan's
living sections synchronized. Resolve every remaining deviation before setting
this plan COMPLETE.

Acceptance E3: SH5 is satisfied, all six requested decision areas are ratified,
no later slice has been pulled forward, and all sequential gates pass. Review
trace links, dependency direction, and document claims; commit and publish the
approved execution changes. Revert the completion checkbox with the milestone
if the ratification is withdrawn. No compatibility layer is needed at any
milestone.

## Concrete steps and validation

Run from the repository root on `1-1-5-confirm-source-health-foundation`.
Before each major milestone inspect `git status --short` and the applicable
document diff, and refresh this plan. Preserve other agents' edits. Delegate
the gates to scrutineer, sequentially, with logs and pipeline failure
propagation. Use a unique milestone suffix in place of `epm1` below:

```bash
set -euo pipefail
make fmt 2>&1 | tee /tmp/fmt-memoryd-1-1-5-epm1.out
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-5-epm1.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-5-epm1.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-5-epm1.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-5-epm1.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-5-epm1.out
```

Stop at the first failing gate; do not commit a red candidate. Each command
must exit zero, with no formatting differences, lint warnings, failed tests, or
unexplained skips. Record test counts and log paths rather than inventing
expected counts. Inspect formatter changes for unrelated edits. Re-run the
sequence after candidate edits. A healthy compiler-admission queue is not a
failure; do not bypass wrappers or create isolated Cargo caches. Never use
`/tmp` for build targets. Report disk exhaustion or admission failures.

The future proof-owning milestone additionally runs:

```bash
make verify-source-health 2>&1 | tee /tmp/verus-memoryd-source-health.out
```

That command is not executable until its explicitly deferred target exists. It
must verify the production kernel refinement, report zero errors, and record
tool version, assumptions, and non-vacuity evidence.

## Idempotence and recovery

Documentation edits and checks are repeatable. Review existing ADR status
before accepting it; preserve historical decisions and append new rationale.
Commit only complete green milestones using `commit-message`; undo only the
owned atomic commit, never reset unrelated work. If an upstream assumption
fails, update the conformance basis and record a proposed deviation before
continuing. No persistent data is changed by this plan's execution.

## Planning publication and artefacts

Before execution approval, publish only this plan and its contents-index entry.
Use `pr-creation`, inspect the full branch diff, verify GitHub identity
`leynos` and SSH authentication, push by SSH, refresh the origin tracking ref,
and set upstream to `origin/1-1-5-confirm-source-health-foundation`.

Create a draft PR titled `Plan: Confirm source-health foundation (1.1.5)`. Link
this document using the branch ref. State the approval boundary and the later
runtime owners. End the PR body with `## References` and the Lody session link
<https://lody.ai/leynos/sessions/dc0e0d42-dd3c-4c5c-9dfc-a74e6cee8cf1>. Read
the PR back to verify draft status, head/base, title, and terminal References.
Stop after publication and present the plan for approval.

Firecrawl research on 2026-11-04 checked the Prometheus primary
[staleness documentation](https://prometheus.io/docs/prometheus/latest/querying/basics/#staleness).
It distinguishes stale samples from available values. This is useful prior art
for keeping observation availability explicit; it does not establish Memoryd's
product defaults or prove source completeness. No Prometheus integration or
dependency is proposed.

Revision note (2026-11-04): Initial draft separates decision ratification from
runtime delivery and records the contracts, proof sketches, test handoffs, and
approval boundary before any implementation.

Revision note (2026-11-04): Pandalump and Telefono identified ingestion/health
sequence conflation; separate health revisions now preserve ADR 007. Doggylump
required conservative freshness anchors and re-enable invalidation. Buzzy Bee
added cadence validation and diagnostic bounds. Wafflecat retained simple
on-demand semantics and conditional MCP exposure. Dinolump corrected the
open-question location and fail-fast commands. These changes refine the
proposed contract without adding runtime work or accepting ADR 008 early.

Revision note (2026-11-04): Expert re-review separated health observer epochs
from ADR 007 source generations. Disable/re-enable and observer handoff now
fence old health reports without changing evidence incarnation or replay keys.

Revision note (2026-11-04): Recorded the successful initial gate run and made
current health-epoch eligibility explicit in freshness and classification. The
final candidate is revalidated before commit; publication evidence stays in the
PR to avoid a self-referential commit hash in this living document.
