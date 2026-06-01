# Memoryd Logisphere design-stage review

- **Status:** Complete.
- **Panel:** Full (all six experts).
- **Documents reviewed:** `memoryd-design.md`, `terms-of-reference.md`,
  `roadmap.md`, RFCs 0001-0006, ADRs 001-012.
- **Date:** 2026-05-25.

______________________________________________________________________

## 1. Proposal summary

`memoryd` is a standalone local daemon that turns coding-agent session history
into durable, evidence-backed memory. It replaces Axinite's single-host
transactional outbox with a provider-adapter pipeline, keeps Qdrant as a
serving index, Oxigraph as graph-shaped truth, Ollama as the local model
provider, and Chutoro as a clustering proposal engine. It exposes MCP tools for
recall, curation, retraction, explanation, and health. Tenant isolation follows
Corbusier's request-context model.

______________________________________________________________________

## 2. Core bets

The design makes the following structural bets:

| #   | Bet                                                                                                                                      | Confidence  | Risk if wrong                                                                                                           |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------- |
| 1   | Local operators will accept four infrastructure dependencies (Qdrant, Ollama, Oxigraph, Chutoro) for better provenance and privacy.      | Medium      | High adoption friction; the daemon becomes shelfware.                                                                   |
| 2   | Codex CLI rollout files and Claude Code transcripts provide enough structured session evidence to derive useful memory.                  | Medium-high | First slice has no useful input and adapters must be redesigned.                                                        |
| 3   | Local embedding and extraction models through Ollama can produce adequate quality for summaries, semantic extraction, and recall gating. | Medium      | Recall quality falls below "grep my transcripts" and the hierarchy adds cost without value.                             |
| 4   | The hexagonal architecture boundary can be enforced by a solo or small team without degenerating into ceremony.                          | Medium      | Port proliferation and adapter boilerplate dominate development time.                                                   |
| 5   | Tenant isolation can be exercised meaningfully in local single-user mode and later extended to Corbusier multi-tenant without rework.    | High        | Either the local path ignores tenant code (dead code risk) or the multi-tenant path discovers gaps at integration time. |
| 6   | The canonical conversation delta is expressive enough for all current providers without per-provider extensions.                         | Medium      | Provider adapters cannot faithfully represent source semantics, forcing core pipeline changes.                          |
| 7   | A pre-1.0 epistemic substrate (ADRs 008-012) can be implemented affordably before it is needed by post-1.0 consumers.                    | Low-medium  | Substrate investment is premature; it ships but is never activated or is redesigned when RFC 0006 lands.                |

______________________________________________________________________

## 3. Panel findings

### 3.1 Pandalump 🐼 — Structural integrity

🟢 **Hexagonal boundary is well-articulated.** The ownership table (Table 3),
the dependency rule, and the explicit commitment to architecture-lint tooling
give the design a stronger-than-usual structural foundation. The prior-art
references to Wildside and Corbusier conventions are practical.

🟢 **Naming forms a coherent vocabulary.** The terminology table (Table 2) is
internally consistent. "Evidence event", "semantic carrier", "projection
activity", and "recall context pack" all name concepts at the right level of
abstraction. No God objects are visible.

🟡 **Port proliferation risk.** The design names approximately 20 driven ports
and 7 driving adapter categories. For a project in early implementation with a
small team, this is a large surface. The risk is not the architecture itself
but whether each port earns its place before it accumulates test doubles and
adapter maintenance. The roadmap (phase 1.2.1) schedules all port traits before
any adapter exists, which front-loads the decision cost.

- *Mitigation:* Consider a "port budget" discipline: each port must justify its
  existence by serving at least two distinct adapters or two distinct use cases
  before it graduates from internal to public crate API. Ports that exist only
  to separate one adapter from the domain may be premature.

🟡 **Collector boundary is clear but the collector process model is
underspecified.** The collector "owns provider discovery, file cursors, hook
handling, and redaction before ingest". It communicates with the daemon via UDS
ingest RPC. But the design does not specify whether the collector is a
long-running sidecar, a periodic job, a socket-activated process, or
event-triggered. This choice affects resource use, restart behaviour, and hook
latency.

- *Mitigation:* Record the collector lifecycle model as an explicit design
  decision before the first binary ships.

🟢 **Conversation ingestion split has been remediated.** The original review
finding was that a single overloaded `ConversationIngestPort` could not cleanly
enforce the distinct trust, sequencing, and idempotency invariants of push-mode
producers versus collected worker batches. ADR 007 now records the canonical
ingestion-port contract and implementations should use that standard
`ConversationIngestPort` contract where applicable:
`ConversationPushIngestPort` handles authenticated push from Corbusier, Axinite
transactional outboxes, and MCP/manual imports, while
`CollectedConversationIngestPort` handles collector-submitted batches from
Codex, Claude, Axinite pull mode, and future worker-read sources. Both ports
share the canonical conversation delta, but their trust, sequencing, cursor,
capability, and idempotency paths are separate.

- *Closed action:* The "split port" mitigation is now addressed in ADR 007 and
  in the design document. Reviewers should treat ADR 007 as the canonical
  ingestion-port specification rather than the original red finding.

______________________________________________________________________

### 3.2 Wafflecat 🐈🧇 — Alternative futures

🟡 **No alternatives section in the design document.** The terms of reference
describe "what we are building" thoroughly but do not record rejected
alternatives. This is an observation, not necessarily a flaw, since the design
imports extensive Axinite prior art. But it makes the review less able to
confirm that the design space was explored.

🟡 **The 80/20 question: could a simpler design deliver most of the value?**
The design specifies six recall profiles, five epistemic status values, eleven
claim kinds, eight support roles, five audit modes, multiple daemon modes, and
a full hexagonal architecture. A "Dear Diary with redaction and evidence
references" would deliver useful memory with dramatically less machinery. The
risk is not that the full design is wrong, but that it never ships because the
foundation phase (1.1 alone has 15 tasks) delays first user value.

- *Observation:* The roadmap does phase this correctly (curated memory and
  flat recall ship in phase 3). The question is whether the phase 1 contract
  work (15 ADR decisions + 8 scaffolding tasks + 3 operator surface tasks = 26
  pre-value tasks) creates a moat that a solo developer cannot cross before
  motivation or context decay. This is a team-capacity concern, not a design
  flaw.

🟢 **Source-of-truth boundary is the strongest design decision.** Keeping
Qdrant as a serving index and Oxigraph as graph authority is the right call.
Most MCP memory projects conflate the vector store with memory truth, which
makes retraction, contradiction, and purge unreliable. This design avoids that
trap.

💡 **What would a "Qdrant-only fallback" look like?** The design notes a
possible "graph-shaped relational fallback" when Oxigraph is disabled (open
question §17, ADR 1.1.4). This is worth deciding early because it affects
whether `flat_v1` recall can ship without Oxigraph at all. If the answer is
yes, the minimum-viable product becomes smaller. If no, Oxigraph becomes a hard
v1 dependency that raises the installation bar.

______________________________________________________________________

### 3.3 Buzzy Bee 🐝 — Scaling and cost

🟢 **Load profile is modest and well-bounded.** Local coding-agent sessions
produce tens to hundreds of events per session, not thousands per second. The
design's data volumes are small enough that SQLite, embedded Oxigraph, and
local Qdrant are reasonable. There is no fan-out bomb hiding in the
architecture.

🟡 **Ollama latency dominates the critical path.** The design uses Ollama for
embeddings, structured extraction, summarization, and optional judge-model
recall gating. On typical local hardware, each Ollama call takes hundreds of
milliseconds to seconds. The design does not specify whether these calls are
synchronous in the recall path or pre-computed during projection.

- *Mitigation:* The design should explicitly state that:
  - Embeddings for recall queries are synchronous (unavoidable).
  - Extraction, summarization, and embedding of evidence are background
    projection jobs (not in the recall hot path).
  - Judge-model gating is explicitly optional and latency-sensitive deployments
    should use `cheap_v2` rather than `evidence_v2`.
  - If Ollama is unavailable, recall degrades to cached embeddings and
    `flat_v1` fallback rather than failing entirely.

🟡 **Qdrant collection-per-tenant-workspace may not scale to many workspaces.**
The default layout creates five collections per (tenant, workspace) pair:
episodes, summaries, semantic carriers, themes, and profiles. A developer
working across 20 repositories creates 100 Qdrant collections. Qdrant handles
this, but it is not zero-cost for memory, startup time, or backup.

- *Mitigation:* Document the expected collection count for typical local use
  (1-5 workspaces = 5-25 collections). Consider deferring collection creation
  until the first projection write rather than at workspace registration.

🟢 **Token budgets in recall are explicit.** The context-pack assembly respects
a token budget parameter. This is the right mechanism for preventing unbounded
recall cost.

______________________________________________________________________

### 3.4 Telefono ☎️ — Contracts and interfaces

🟢 **MCP tool surface is minimal and well-scoped.** Eight tools (Table 8) is a
good count. The tools are named by intent, not by storage backend. The
read-only gate is sensible.

🟢 **Idempotency key design is provider-specific and tenant-scoped.** This is
correct and avoids the common mistake of using generic UUIDs that cannot detect
replays.

🟡 **Internal RPC methods mix CRUD and domain commands.** `IngestSourceEvent`,
`IngestTranscriptLine`, `FinalizeSession`, `Recall`, `ReadFacts`, `ReadEpisode`,
`ReadTheme`, `StoreCuratedMemory`, `Retract`, `Reinforce`,
`ScheduleConsolidation`, `ImportTranscript`, `ListSessions`, `Health`,
`PurgeWorkspace`, `PurgeTenant`. This list mixes command-style operations
(Ingest, Store, Retract, Purge) with query-style operations (Read, List,
Health) and scheduling operations (Schedule). The design does not make CQRS
explicit, but it would benefit from a clearer separation between commands that
change state and queries that do not.

- *Observation:* This is a minor hygiene point. The hexagonal boundary already
  separates concerns at the port level. But the RPC envelope should make it
  possible for audit, capability, and idempotency logic to distinguish "this is
  a read" from "this is a write" without per-method knowledge.

🟡 **Canonical conversation delta is large.** The delta carries source
identity, request scope, conversation metadata, ordered events (each with
ordinal, role, actor, event kind, timestamp, content parts, tool metadata,
compaction markers, file-edit summaries, payload hash, redaction state, and
evidence references), cursor metadata, and deletion/correction metadata. This
is comprehensive but raises the question: will every adapter actually populate
all these fields, or will many fields be perennially `None` for certain
providers?

- *Mitigation:* Define which fields are required vs optional per
  `ConversationSourcePort` kind. Document the minimum viable delta for each
  known provider so adapter authors do not over-populate or under-populate.

🔴 **No versioning strategy for the internal RPC or MCP tools.** The design
specifies capability scopes but does not describe how RPC methods or MCP tool
schemas will evolve. If `memory_recall` adds new fields to the context pack
format, how do existing MCP clients handle that? If the internal RPC adds a
method, how does version negotiation work?

- *Mitigation:* Define additive-only evolution rules for MCP tool responses
  (new fields are optional; existing fields never change meaning). For internal
  RPC, define an envelope version or capability-negotiation handshake.

______________________________________________________________________

### 3.5 Doggylump 🐶 — Failure modes and operational readiness

🟡 **Ollama unavailability is the most likely runtime failure.** Ollama is a
local process that the user must start manually. It can crash, run out of
memory loading a model, or simply not be running when the daemon starts.

- *Mitigation:* Define explicit degraded modes:
  - Ollama down during ingest: evidence capture succeeds but projection queues.
  - Ollama down during recall: cached embeddings (if available) or error with
    explanation.
  - Ollama model missing: fail with a clear diagnostic, not a generic
    connection error.

🟡 **Qdrant unavailability has unclear blast radius.** The design says Qdrant
is a serving index and not authoritative. But if Qdrant is down, can the daemon
still ingest? Still retract? Still serve health? The boundary is implied but
not stated.

- *Mitigation:* Explicitly document which use cases require Qdrant and which
  can proceed without it. Ingest should not block on Qdrant. Recall should fail
  gracefully. Health should report Qdrant state.

🟡 **Purge is irreversible and has a large blast radius.** `PurgeWorkspace` and
`PurgeTenant` delete across evidence inbox, Oxigraph, Qdrant, and Chutoro
checkpoints. The design requires a high-privilege token and confirmation
string, which is good. But there is no soft-delete or "undo within N hours"
path.

- *Observation:* For a local-first daemon, irreversible purge is probably
  acceptable because the user controls backups. But the documentation should
  make the irreversibility very clear and recommend a backup procedure before
  purge.

🟢 **Projection repair is explicitly in scope.** The design anticipates
projection failures and includes repair as a daemon capability. This is mature
operational thinking.

🟢 **Recall fallback is well-designed.** The explicit `fallback_reason` field
and the `flat_v1` fallback when hierarchy is absent prevent silent degradation.

💡 **What happens to in-flight projection during daemon restart?** Ingest jobs
have retry state, but the design does not describe whether a crashed daemon
leaves orphaned projection-state records that need explicit recovery. Document
the startup reconciliation path.

______________________________________________________________________

### 3.6 Dinolump 🦕 — Long-term viability and team impact

🔴 **Cognitive load is very high for a solo or small team.** The design
describes four infrastructure dependencies (Qdrant, Ollama, Oxigraph, Chutoro),
three processes (daemon, collector, MCP), six source-of-truth stores (evidence
inbox, Oxigraph, Qdrant, Chutoro checkpoints, configuration, plus the source
files themselves), five projection classes, five epistemic statuses, eleven
claim kinds, eight support roles, five recall profiles, five daemon modes, and
twelve ADRs before v1 ships. Each concept is individually justified. The
aggregate burden is the concern.

- *Mitigation:* The roadmap already sequences delivery sensibly. The risk is
  not poor sequencing but context decay between phases. Consider:
  - Keeping the first public release at phase 3 (curated memory + flat recall)
    and treating phases 4-6 as post-first-release.
  - Documenting an explicit "minimum useful deployment" that requires only
    daemon + SQLite + Qdrant + Ollama, with Oxigraph and Chutoro as opt-in
    enhancements.
  - Accepting that the pre-1.0 epistemic substrate (ADRs 008-012) may ship as
    schema but not as actively-used features until a consumer exists.

🟡 **Bus factor of one.** The project set (memoryd, Axinite, Corbusier,
Chutoro, Dear Diary, Wildside) shares a consistent design vocabulary, which
suggests a single designer. The comprehensive documentation mitigates this risk
somewhat, but the sheer volume of inter-project references makes onboarding
expensive.

- *Observation:* This is acknowledged rather than fixable by design. The
  documentation quality is the best mitigation available.

🟡 **Oxigraph operational experience.** Oxigraph is a correct, well-maintained
embedded triple store. But it is less battle-tested in production than SQLite
or Qdrant. The design makes it the authority for facts, provenance,
contradictions, and retractions. If Oxigraph has correctness or durability
issues under concurrent access from the daemon, recovery involves rebuilding
graph state from the evidence inbox and projection activities.

- *Mitigation:* The design's rebuild-from-evidence philosophy already
  addresses this: the evidence inbox is the ultimate authority, and graph state
  is reconstructable. Document the rebuild path explicitly as an operational
  procedure.

______________________________________________________________________

## 4. Pre-mortem (Doggylump leads)

*It is six months from now. `memoryd` has caused an incident. Working
backwards:*

### Scenario A: Silent recall degradation

**What happened:** Ollama model files were updated by the user (a routine
`ollama pull`). The new model produces embeddings with a different vector
space. Existing Qdrant collections contain vectors from the old model. Recall
quality silently degrades because cosine similarity between old and new vectors
is meaningless. No alert fires.

**Blast radius:** All recall for affected workspaces returns irrelevant
results. Curated memory that was stored correctly becomes unfindable.

**Signal missed:** The design records `model` in configuration but does not
track embedding model identity per Qdrant collection or per stored vector.

**Bet that was wrong:** Bet 3 (Ollama quality) is partially wrong, but the
deeper issue is that model identity is not tracked at the vector level.

**Prevention:** Record embedding model name and vector dimension with each
Qdrant collection. On startup or recall, compare the configured model against
stored model metadata. If they differ, refuse recall (or trigger re-embedding)
rather than returning silently wrong results. The projection-activity lineage
(ADR 011) can support this if it records the model identity.

### Scenario B: Accidental cross-workspace recall

**What happened:** A developer uses the same Git origin URL for a fork and the
upstream repository. Workspace derivation produces the same workspace ID for
both. Evidence from the fork contaminates recall for the upstream workspace.
The developer acts on a recalled "decision" that was actually from a personal
experimental branch.

**Blast radius:** One user's recall is contaminated. No tenant boundary is
crossed, so audit does not fire.

**Signal missed:** Workspace identity collision within a single tenant.

**Bet that was wrong:** Bet 6 partially — the canonical delta captures
`repo_origin` but workspace derivation from origin alone is insufficient.

**Prevention:** Include the local repository root path (hashed) in workspace
derivation alongside origin. Document that workspace collision within a tenant
is possible and provide an operator override. The open question in §17
("workspace identity derivation") acknowledges this risk but it must be
resolved before evidence capture ships.

### Scenario C: Unbounded projection backlog after Ollama outage

**What happened:** Ollama was down for several hours while sessions continued.
The collector captured evidence normally (evidence capture does not require
Ollama). When Ollama returned, the projection queue contained hundreds of
pending episodes and extraction jobs. The daemon attempted to process them all
synchronously, saturating CPU and memory and causing the daemon to be
OOM-killed.

**Blast radius:** Daemon restart loop until the operator manually clears the
queue or increases resources.

**Signal missed:** No backpressure or batch-size limit on projection catch-up.

**Bet that was wrong:** Not a design bet failure, but an operational gap. The
design mentions "backpressure" in roadmap 7.1.2 but does not specify limits.

**Prevention:** Define maximum concurrent projection jobs and maximum batch
size for catch-up. If the queue exceeds a configured threshold, process in
bounded batches with pauses rather than draining eagerly.

______________________________________________________________________

## 5. Alternatives checkpoint (Wafflecat leads)

### Strongest alternative: "Enriched Dear Diary"

**Description:** Instead of a full hexagonal daemon with multiple stores,
extend Dear Diary's architecture: a single Rust MCP binary backed by Qdrant and
SQLite. Add evidence references (source path, line, hash) to Qdrant payloads.
Add a "provider collector" as a separate CLI that writes to the same SQLite +
Qdrant. Add redaction. Add epistemic-status payload fields. Skip Oxigraph
entirely. Skip Chutoro. Skip the formal projection hierarchy. Use Qdrant
payload filtering for "retracted" state.

**What it trades away:**

- Graph-backed provenance and contradiction handling.
- Formal epistemic promotion rules (curated/explicit/deduced distinctions
  would be metadata-only, not enforced).
- Theme-based hierarchical recall.
- Projection replay and rebuild from evidence.
- Clean hexagonal architecture (Qdrant is the primary store).
- The post-1.0 epistemic-health substrate.

**What it gains:**

- Ships to first user value in weeks rather than months.
- Two dependencies instead of four (Qdrant + Ollama; SQLite for cursors).
- Single binary for the memory service.
- Lower cognitive load for contributors and operators.
- Proves or disproves core bet 2 (are transcripts useful input?) cheaply.

**Assessment:** This alternative is genuinely viable for the "local operator
who wants better recall" user. It is not viable for the "Axinite maintainer who
needs trustworthy provenance" or "Corbusier maintainer who needs tenant
isolation" stakeholders. The proposed design is more ambitious because it
serves a broader stakeholder set and plans for post-1.0 growth. The risk is
that it serves none of them if it does not ship.

**Recommendation:** The proposed design is the correct long-term architecture.
But the roadmap should treat phase 3 (curated memory + flat recall) as a
"release 0.1" milestone with its own announcement, documentation, and user
feedback loop. This gives the project a Dear-Diary-equivalent checkpoint
without abandoning the richer architecture.

______________________________________________________________________

## 6. Verdict

### ⚠️ Proceed with conditions

The design is structurally sound, well-documented, and architecturally
coherent. The domain boundaries, source-of-truth rules, tenant isolation model,
and provenance commitment are all strong. The design's weaknesses are not in
what it proposes but in what it defers or underspecifies.

### Conditions for proceeding

1. **Resolve the collector lifecycle model** (Pandalump 🐼). Specify whether it
   is a long-running sidecar, periodic job, or socket-activated process before
   shipping the first binary.

2. **Define embedding model identity tracking** (Doggylump 🐶, pre-mortem A).
   Record model name and dimension with each Qdrant collection. Detect and
   refuse mismatched recall rather than returning silently wrong results.

3. **Define explicit degraded modes for Ollama and Qdrant unavailability**
   (Doggylump 🐶). Document which use cases proceed, which queue, and which
   fail when each dependency is absent.

4. **Define additive-only evolution rules for MCP and RPC** (Telefono ☎️).
   Existing fields never change meaning; new fields are optional. Document this
   as a contract before external clients exist.

5. **Resolve workspace identity derivation before evidence capture ships**
   (pre-mortem B, open question §17). The design acknowledges this; it must be
   closed before data is stored.

6. **Define projection backpressure and catch-up batch limits** (pre-mortem C).
   The roadmap schedules this (7.1.2) but it should be specified as a
   constraint for the first active-mode deployment.

### Core bets summary

| Bet                                      | Confidence  | Recommended hedge                                                                                                           |
| ---------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------- |
| Users accept four dependencies           | Medium      | Document a minimum viable deployment (daemon + SQLite + Qdrant + Ollama only). Treat Oxigraph and Chutoro as opt-in.        |
| Transcripts provide useful input         | Medium-high | Ship evidence capture first (phase 2) and validate before investing in projection hierarchy.                                |
| Local Ollama quality is adequate         | Medium      | Keep `encoder_extractive` as a non-model fallback. Measure disagreement in shadow mode.                                     |
| Hexagonal architecture is affordable     | Medium      | Apply the port-budget discipline: justify each port by usage before publishing.                                             |
| Tenant isolation exercises in local mode | High        | The "local default tenant" design is correct; just ensure the code path is exercised in CI.                                 |
| Canonical delta is expressive enough     | Medium      | Document minimum viable delta per provider. Accept that the first extension point will arrive with the second real adapter. |
| Pre-1.0 substrate is worthwhile          | Low-medium  | Accept that ADRs 008-012 may ship as schema only. Do not block v1 on active consumers.                                      |

### Findings by severity

| Severity | Expert         | Finding                                                                                    |
| -------- | -------------- | ------------------------------------------------------------------------------------------ |
| 🟢       | 🐼 Pandalump   | Conversation ingestion now splits push and collected-batch ports per ADR 007.              |
| 🔴       | ☎️ Telefono    | No versioning strategy for internal RPC or MCP tool schemas.                               |
| 🔴       | 🦕 Dinolump    | Cognitive load is very high; risk of never shipping without an intermediate release.       |
| 🟡       | 🐼 Pandalump   | Port proliferation risk (~20 driven ports before first adapter).                           |
| 🟡       | 🐼 Pandalump   | Collector lifecycle model (sidecar vs job vs activated) is unspecified.                    |
| 🟡       | 🐈🧇 Wafflecat | No alternatives section in the design document.                                            |
| 🟡       | 🐈🧇 Wafflecat | 26 pre-value tasks in phase 1 may exceed solo capacity.                                    |
| 🟡       | 🐝 Buzzy Bee   | Ollama latency model (sync vs async, hot path vs projection) is implicit.                  |
| 🟡       | 🐝 Buzzy Bee   | Collection-per-workspace may create many collections for active developers.                |
| 🟡       | ☎️ Telefono    | Internal RPC mixes commands and queries without explicit CQRS marker.                      |
| 🟡       | ☎️ Telefono    | Canonical conversation delta is large; per-provider minimum viable subset is undocumented. |
| 🟡       | 🐶 Doggylump   | Ollama unavailability degrades silently unless degraded modes are explicit.                |
| 🟡       | 🐶 Doggylump   | Qdrant unavailability blast radius is unclear.                                             |
| 🟡       | 🐶 Doggylump   | Purge irreversibility needs prominent documentation.                                       |
| 🟡       | 🦕 Dinolump    | Bus factor of one across the project set.                                                  |
| 🟡       | 🦕 Dinolump    | Oxigraph operational experience is less proven than other components.                      |
| 🟢       | 🐼 Pandalump   | Hexagonal boundary is well-articulated with tooling commitment.                            |
| 🟢       | 🐼 Pandalump   | Naming vocabulary is coherent and intention-revealing.                                     |
| 🟢       | 🐈🧇 Wafflecat | Source-of-truth boundary (Qdrant as index, not authority) is the strongest decision.       |
| 🟢       | 🐝 Buzzy Bee   | Load profile is modest and well-bounded for local use.                                     |
| 🟢       | 🐝 Buzzy Bee   | Token budgets in recall are explicit.                                                      |
| 🟢       | ☎️ Telefono    | MCP tool surface is minimal and intent-named.                                              |
| 🟢       | ☎️ Telefono    | Idempotency keys are provider-specific and tenant-scoped.                                  |
| 🟢       | 🐶 Doggylump   | Projection repair is in scope from the start.                                              |
| 🟢       | 🐶 Doggylump   | Recall fallback with explicit reason codes is well-designed.                               |
| 💡       | 🐈🧇 Wafflecat | What does an Oxigraph-free deployment look like? Decide before v1 ships.                   |
| 💡       | 🐶 Doggylump   | Startup reconciliation after daemon crash needs documentation.                             |

### Pre-mortem scenarios

| Scenario                         | Trigger                                        | Mitigation                                                                            |
| -------------------------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| A. Silent recall degradation     | Ollama model change invalidates stored vectors | Track embedding model identity per collection; detect mismatch at recall time.        |
| B. Cross-workspace contamination | Workspace ID collision from repo forks         | Include repository root path hash in workspace derivation; provide operator override. |
| C. Projection backlog OOM        | Ollama outage creates unbounded catch-up queue | Define max concurrent projection jobs and batch-size limits for catch-up.             |

### Strongest alternative

"Enriched Dear Diary" — single Rust MCP binary + Qdrant + SQLite, no Oxigraph,
no Chutoro, no projection hierarchy. Ships faster; serves fewer stakeholders;
does not support the post-1.0 epistemic direction. Not recommended as a
replacement, but recommended as a mental model for the phase 3 milestone: treat
curated-memory-plus-flat-recall as a self-contained release.

### Recommended next steps (priority order)

1. Close the workspace identity derivation open question (ADR candidate).
2. Record embedding model identity tracking as a design requirement for the
   Qdrant adapter.
3. Specify collector lifecycle model (sidecar or job) in the design document.
4. Add a "degraded mode" section to the design document for Ollama and Qdrant
   unavailability.
5. Add additive-only contract evolution rules to the RPC and MCP sections.
6. Treat the split-ingest work as closed and use ADR 007's
   `ConversationPushIngestPort` and `CollectedConversationIngestPort` as the
   canonical implementation target.
7. Declare phase 3 as "release 0.1" with its own documentation and user
   feedback loop.
8. Define projection backpressure limits before active mode is enabled.
