# Remediate the Logisphere design-stage review

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
 `Tolerances`, `Risks`, `Progress`, `Surprises & Discoveries`, `Decision Log`,
and `Outcomes & Retrospective` must be kept up to date as work proceeds.

Status: COMPLETE

## Purpose / big picture

The Logisphere design-stage review in
`docs/memoryd-logisphere-design-stage-review.md` found that the Memoryd design
is structurally sound but still leaves several operational, interface, and
scope-control decisions implicit. This plan turns every actionable mitigation,
recommendation, and observation from that review into repository documentation
changes.

Success is observable when the design, ADRs, terms of reference, and roadmap
state how Memoryd handles collector lifecycle, push versus worker ingestion,
schema evolution, degraded modes, model identity, workspace identity,
projection backpressure, minimum useful deployment, and first-release scope.
Running the documentation gates must pass after the changes.

## Constraints

- Modify documentation only unless implementation work becomes unavoidable.
- Preserve the existing long-term architecture: Qdrant remains a serving index,
  Oxigraph remains the full graph-shaped truth store, Ollama remains the local
  model provider, Chutoro remains a clustering proposal engine, and the daemon
  owns policy.
- Preserve the earlier phase-one perimeter reduction. Do not reintroduce a
  large pre-value ratification block.
- Keep the project hexagonal: domain and application contracts may be
  described, but adapters, storage clients, MCP, and transport details must
  remain outside domain ownership.
- Keep multi-tenancy first-class. Every tenant-owned read, write, recall,
  projection, repair, and purge path must remain scoped by authenticated
  request context.
- Follow `AGENTS.md` and `docs/documentation-style-guide.md`: en-GB Oxford
  spelling, wrapped Markdown, formatted tables, and Mermaid validation.

## Tolerances (exception triggers)

- Scope: if remediation requires changing production Rust code, stop and
  escalate before editing code.
- Scope: if more than ten documentation files require changes, stop and
  re-evaluate whether the remediation is too broad for one commit.
- Interface: if a public MCP tool is removed rather than staged or deferred,
  stop and document the trade-off.
- Architecture: if a mitigation requires Qdrant to become authoritative for
  facts, provenance, or retractions, stop and escalate.
- Validation: if documentation gates still fail after two focused repair
  attempts, stop and record the blocker.

## Risks

- Risk: The review includes many observations, so the remediation could bloat
  phase 1 again. Severity: high. Likelihood: medium. Mitigation: Resolve
  design-contract gaps in the design documents while moving
  implementation-heavy work to the slice that first uses it.

- Risk: Splitting push and worker ingestion could accidentally create two
  canonical evidence models. Severity: high. Likelihood: low. Mitigation: Keep
  one canonical conversation delta and split only the application commands and
  capability paths.

- Risk: A dependency-light first release could be misread as abandoning
  Oxigraph or Chutoro. Severity: medium. Likelihood: medium. Mitigation:
  Describe it as release 0.1 minimum useful deployment, with explicit
  capability limits and migration to the full architecture.

- Risk: Adding versioning and degraded-mode rules in prose without roadmap
  tasks leaves them unenforced. Severity: medium. Likelihood: medium.
  Mitigation: Add matching roadmap tasks or task criteria to the earliest
  relevant delivery slices.

## Remediation plan

The review's actionable red findings are:

- Split the overloaded `ConversationIngestPort` responsibility into distinct
  authenticated push and worker-submitted batch use cases while retaining a
  shared canonical delta.
- Define additive-only MCP and internal RPC evolution rules, including version
  or capability negotiation for internal RPC.
- Reduce cognitive-load and shipping risk by documenting a minimum useful
  deployment and treating phase 3 as a release 0.1 checkpoint.

The actionable yellow mitigations and recommendations are:

- Add a port-budget discipline so ports graduate only when justified by
  multiple adapters or use cases.
- Specify the collector lifecycle model before the first binary ships.
- Add an alternatives section to the design document.
- Make the phase-one perimeter reduction visible and keep pre-value work small.
- Decide how the dependency-light flat-recall path works without Oxigraph.
- Specify which Ollama calls are hot-path and which run as background jobs.
- Document degraded modes for Ollama and Qdrant.
- Document expected Qdrant collection counts and lazy collection creation.
- Mark internal RPC envelopes as commands, queries, or schedules for audit,
  capability, and idempotency logic.
- Define required and optional canonical delta fields per provider kind.
- Make purge irreversibility and pre-purge backup expectations explicit.
- Document daemon startup reconciliation for orphaned projection work.
- Document the Oxigraph rebuild-from-evidence procedure.
- Track embedding model identity and vector dimension for Qdrant collections
  and stored vectors.
- Resolve workspace identity derivation before evidence capture ships.
- Define projection catch-up concurrency and batch limits before active mode.

The actionable green observations are already preserved by the design, but the
remediation should avoid weakening them: the hexagonal boundary, vocabulary,
source-of-truth split, bounded load profile, explicit token budgets, minimal
MCP surface, tenant-scoped idempotency keys, projection repair, and explicit
recall fallback must remain intact.

## Implementation steps

First, update `docs/memoryd-design.md`. Add explicit sections for minimum
useful deployment, alternatives considered, collector lifecycle, port budget,
split ingestion commands, provider delta minimums, model hot paths, Qdrant
metadata and lazy creation, degraded modes, versioned RPC/MCP evolution,
startup reconciliation, purge backup warnings, Oxigraph rebuild, and projection
backpressure.

Second, update ADR and RFC files that currently describe the old ingestion
shape. `docs/adr-007-standard-conversation-ingestion-port.md` and
`docs/rfcs/0001-standalone-evidence-inbox.md` must describe separate push and
worker-batch commands over the same canonical delta.

Third, update `docs/terms-of-reference.md` to reflect the now-decided workspace
identity rule, minimum useful deployment, and remaining open questions.

Fourth, update `docs/roadmap.md` so each remediation has an implementation home
and phase 3 is labelled as release 0.1.

Fifth, update `docs/contents.md` to include this execplan if project
documentation indexes execution plans.

Sixth, update `docs/developers-guide.md` with the developer-facing interface
conventions implied by the remediation: structured errors, bounded pagination,
capability-derived request context, tenant-aware authorization, and port
boundary expectations.

Finally, run the documentation gates sequentially through `tee`, inspect the
diff, commit with a file-based message, push, update the existing draft pull
request, and check remote status.

## Progress

- [x] 2026-05-25: Read the current branch and confirmed the working tree was
  clean on `project-design`.
- [x] 2026-05-25: Reviewed
  `docs/memoryd-logisphere-design-stage-review.md` and extracted actionable
  red, yellow, green, and idea items.
- [x] 2026-05-25: Created this remediation plan at
  `docs/execplans/project-design.md`.
- [x] 2026-05-25: Updated the design document with the review mitigations for
  collector lifecycle, port budget, split ingestion, provider delta minimums,
  Qdrant model identity, degraded modes, schema evolution, release 0.1, purge,
  startup reconciliation, and backpressure.
- [x] 2026-05-25: Updated ADR/RFC contracts for split push versus
  worker-batch ingestion and embedding model lineage.
- [x] 2026-05-25: Updated the terms of reference and roadmap so workspace
  identity, release 0.1, and each mitigation have an implementation home.
- [x] 2026-05-25: Ran `make fmt`, `make markdownlint`, `make nixie`,
  `make all`, and `git diff --check`. All passed. Reverted only incidental
  formatter churn in the source review report.
- [x] 2026-05-25: Prepared the final commit, push, pull-request update, and
  remote-check step. The actual commit SHA and remote status are reported in
  the chat transcript because recording them here would require a follow-up
  commit.
- [x] 2026-05-25: Added the missing developer-guide documentation alignment
  for structured errors, pagination, authentication, authorization, tenant
  context, and port-boundary conventions.

## Surprises & Discoveries

- The earlier roadmap update already reduced phase 1.1 from a broad
  ratification block to six first-value decisions. The remaining remediation
  should strengthen that reduction rather than add new phase-one gates.
- `docs/execplans` did not exist, so this plan created the directory.
- The design had already decided enough of the Oxigraph-free path to frame it
  as a release 0.1 deployment profile rather than a permanent graph fallback.

## Decision Log

- Decision: Treat the user's instruction, "Once the plan is set out, action
  the plan", as explicit approval to proceed after drafting the execplan.
  Rationale: the normal execplan approval gate is satisfied by the same message
  that requested the plan and implementation.

- Decision: Keep one canonical conversation delta but split ingestion into
  authenticated push and worker-submitted batch application commands.
  Rationale: the review identified trust and sequencing differences in the
  caller paths, but not a need for separate evidence schemas.

- Decision: Describe release 0.1 as "minimum useful deployment" rather than as
  a separate architecture. Rationale: this preserves the full
  Axinite-compatible design while giving operators an early Dear
  Diary-equivalent milestone.

- Decision: Treat workspace identity derivation as resolved in design rather
  than leave it in the open-question table. Rationale: the pre-mortem
  cross-workspace contamination scenario has a concrete mitigation: combine
  origin, local root hash, and optional profile, then fail on collisions
  without operator override.

## Outcomes & Retrospective

The remediation converted the Logisphere review into concrete documentation
contracts and roadmap work. The design now has an explicit release 0.1
checkpoint, split push versus worker ingestion commands, provider delta
minimums, port-budget discipline, collector sidecar lifecycle, workspace
identity rules, Qdrant model identity checks, degraded dependency modes,
contract evolution rules, purge warnings, startup reconciliation, Oxigraph
rebuild guidance, and projection backpressure limits.

The main lesson is that most review findings could be handled without widening
phase 1. The correct shape was not more ratification; it was making the first
useful deployment smaller while assigning operational hardening to the slices
that first need it.
