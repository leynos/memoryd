# Memoryd roadmap

This roadmap translates the current terms of reference, standalone design,
ADRs, and RFCs into an outcome-oriented implementation sequence. It does not
promise dates. Each phase carries a testable GIST idea, each step is a
workstream that answers a sequencing question, and each task is a review-sized
execution unit with explicit source citations.

The primary design sources are [Memoryd design](memoryd-design.md), the
[terms of reference](terms-of-reference.md), ADRs 001-004, and RFCs 0001-0005.
The roadmap keeps their central boundary intact: logs are evidence, Qdrant is a
serving index, Oxigraph owns graph-shaped truth, Ollama is an extractor and
embedding provider, Chutoro proposes clusters, and `memoryd` owns memory policy.

## 1. Foundational contracts and build spine

Idea: if Memoryd settles the contracts that would otherwise reshape storage,
process boundaries, provider adapters, and safety policy before feature work
starts, later slices can converge on one coherent v1 architecture instead of
reworking the same interfaces.

This phase is deliberately foundational. It turns the design documents into
ratified implementation contracts, crate boundaries, configuration surfaces,
fixtures, and local development gates before the first user-facing slice lands.

### 1.1. Ratify the v1 decisions that gate implementation

This step answers which unsettled design choices must be fixed before storage,
adapters, projection, and recall can be implemented safely. Its outcome informs
the crate layout, migrations, configuration, and first public MCP surface. See
terms-of-reference.md §§8-9 and memoryd-design.md §§16-17.

- [ ] 1.1.1. Record the evidence-store engine and migration policy as an ADR.
  - Decide whether v1 defaults to SQLite, libSQL, PostgreSQL, or a supported
    set.
  - See terms-of-reference.md §9, memoryd-design.md §7, and RFC 0001 §7.
  - Success: one accepted ADR defines the default store, migration format,
    backup expectations, and test matrix for the evidence inbox.
- [ ] 1.1.2. Record the workspace identity and purge-isolation policy as an
  ADR.
  - Requires 1.1.1.
  - Define repository-origin normalization, path hashing, profile overrides,
    collision handling, and operator-visible identifiers.
  - See terms-of-reference.md §§7-9 and memoryd-design.md §§8.4 and 13.
  - Success: workspace IDs can be derived deterministically and used in
    evidence, Qdrant, Oxigraph, Chutoro, and purge tests.
- [ ] 1.1.3. Record the redaction and raw-text retention policy as an ADR.
  - Requires 1.1.1.
  - Decide the first detector set, deny-pattern behaviour, encrypted raw-text
    mode, and what may be embedded.
  - See terms-of-reference.md §§7.2 and 9, memoryd-design.md §13, and RFC
    0001 §§3 and 7.
  - Success: redaction guarantees are explicit enough to implement provider
    adapters and security-sensitive regression fixtures.
- [ ] 1.1.4. Record the Oxigraph requirement and fallback policy as an ADR.
  - Requires 1.1.1.
  - Decide whether v1 requires Oxigraph or permits graph-shaped relational
    tables in a constrained mode.
  - See terms-of-reference.md §§8.3-9, memoryd-design.md §§8.1 and 17, and
    ADR 001.
  - Success: later graph, fact, retraction, and purge tasks know whether they
    target one graph implementation or a capability-limited fallback.
- [ ] 1.1.5. Record the Axinite write-back policy and loop-prevention contract
  as an ADR.
  - Requires 1.1.2 and 1.1.4.
  - Choose manual, approved, or automatic write-back defaults, and define
    projection metadata that prevents self-reinforcing memory loops.
  - See terms-of-reference.md §§5 and 9, memoryd-design.md §§6 and 16, and RFC
    0001 §8.
  - Success: Axinite can be planned as a provider and optional projection sink
    without weakening provenance or duplication controls.
- [ ] 1.1.6. Record the Chutoro checkpoint and theme-ID churn policy.
  - Requires 1.1.2.
  - Decide checkpoint ownership, rebuild behaviour, acceptable theme churn, and
    when shadow proposals can become active changes.
  - See memoryd-design.md §§9 and 17, ADR 003, and RFC 0004 §§4-5.
  - Success: theme implementation can separate rebuildable Chutoro state from
    durable `memoryd` theme identity.
- [ ] 1.1.7. Confirm the minimum public MCP tool set for the first public
  slice.
  - Requires 1.1.1-1.1.4.
  - Choose which of `memory_store`, `memory_recall`, `memory_retract`,
    `memory_sessions`, `memory_import_session`, `memory_profile`,
    `memory_explain`, and `memory_health` must ship before broader projection.
  - See terms-of-reference.md §§6-7, memoryd-design.md §11, and RFC 0005 §5.
  - Success: the MCP crate can reject out-of-scope tools deliberately rather
    than accidentally omitting required v1 behaviour.

### 1.2. Establish the process, crate, and configuration spine

This step answers whether the repository shape can support the three visible
processes and their shared contracts without leaking infrastructure details
into domain code. Its outcome unlocks all vertical slices. See
memoryd-design.md §§5, 12, and 14.

- [ ] 1.2.1. Split the scaffold into reviewable crates for domain contracts,
  daemon runtime, collector runtime, MCP front end, and provider adapters.
  - Requires 1.1.1 and 1.1.7.
  - Keep shared types in a domain crate and keep binary entrypoints thin.
  - See memoryd-design.md §§5.1, 6, 11, and 12.
  - Success: `memoryd`, `memoryd-collector`, and `memoryd-mcp` build as
    separate binaries with no direct Qdrant, Oxigraph, Ollama, or Chutoro
    dependency in provider-only code.
- [ ] 1.2.2. Implement the initial TOML configuration model and validation
  errors.
  - Requires 1.2.1.
  - Cover daemon, store, Qdrant, Ollama, graph, Chutoro, provider, and privacy
    sections.
  - See memoryd-design.md §14.
  - Success: invalid configurations fail with semantic errors and valid
    minimal configurations can start in `observe` mode.
- [ ] 1.2.3. Implement process startup, shutdown, and structured diagnostics
  for all binaries.
  - Requires 1.2.1 and 1.2.2.
  - Add tracing spans, health-oriented state fields, and graceful shutdown
    handling for foreground and daemon modes.
  - See memoryd-design.md §§5.1, 12, and 15.
  - Success: each binary reports startup configuration, dependency mode, and
    shutdown reason without using unstructured standard output in library code.
- [ ] 1.2.4. Implement the internal RPC envelope and capability-token model.
  - Requires 1.2.1 and 1.2.2.
  - Define UDS defaults, loopback debug mode, bearer or capability token
    parsing, request IDs, and error envelopes.
  - See memoryd-design.md §§5.2 and 12 and RFC 0001 §5.
  - Success: collector and MCP callers can authenticate to a test daemon with
    scoped capabilities, and unauthorized methods are rejected before domain
    handlers run.
- [ ] 1.2.5. Add the shared contract fixture harness.
  - Requires 1.2.1-1.2.4.
  - Store provider input examples, normalized evidence JSON, redaction
    examples, recall request examples, and projection examples as stable
    fixtures.
  - See memoryd-design.md §§6-8 and 15 and RFCs 0001-0005.
  - Success: each later slice can add fixture-backed behaviour without
    inventing a parallel test format.

### 1.3. Build the day-one operator surface

This step answers whether operators and developers can inspect a running
scaffold before ingestion exists. It reduces local adoption risk and provides
the diagnostic pattern used by later slices. See README.md, users-guide.md,
developers-guide.md, and memoryd-design.md §§12 and 15.

- [ ] 1.3.1. Replace the stub binary with `memoryd health` and
  `memoryd config check`.
  - Requires steps 1.1-1.2.
  - Keep the commands local-only and safe when no external dependencies are
    running.
  - See memoryd-design.md §§12, 14, and 15.
  - Success: the README quick start can demonstrate a real command that
    validates configuration and reports daemon readiness.
- [ ] 1.3.2. Add `memoryd-collector health` and `memoryd-mcp health` command
  stubs backed by the shared configuration and RPC envelope.
  - Requires 1.2.2 and 1.2.4.
  - See memoryd-design.md §§5.1, 11, and 12.
  - Success: all three binaries expose a consistent operator health contract
    before provider ingestion starts.
- [ ] 1.3.3. Update the README, users' guide, and developers' guide for the
  real operator surface.
  - Requires 1.3.1 and 1.3.2.
  - See README.md, docs/users-guide.md, docs/developers-guide.md, and
    docs/contents.md.
  - Success: the documented quick start, public Makefile targets, and binary
    examples all run against the implemented commands.

## 2. Vertical slice 1: Evidence capture and session browsing

Idea: if Memoryd can capture external provider records as redacted, idempotent,
browseable evidence before projection exists, it proves the logs-as-evidence
model and leaves behind useful session inspection even if semantic extraction
is disabled.

This slice delivers the first usable product surface: configured providers can
feed the daemon, the daemon persists normalized evidence, and an operator can
browse sessions and health state through CLI and MCP surfaces.

### 2.1. Persist canonical evidence without semantic projection

This step answers whether the evidence inbox can act as the durable boundary
between untrusted provider input and later memory projection. It informs every
adapter and replay path. See memoryd-design.md §7 and RFC 0001.

- [ ] 2.1.1. Implement evidence inbox migrations and repository APIs.
  - Requires 1.1.1 and 1.2.5.
  - Cover `source_session`, `source_cursor`, `raw_event`, `raw_span`,
    `ingest_job`, `projection_state`, and `audit_log`.
  - See memoryd-design.md §7 and RFC 0001 §§6-7.
  - Success: fixture data can be inserted, replayed, and queried through
    typed APIs without exposing SQL details to adapters.
- [ ] 2.1.2. Implement provider-neutral evidence, span, and evidence-reference
  domain types.
  - Requires 2.1.1.
  - Include actor, event kind, ordinal, observed time, payload hash, offset,
    text span, tool metadata, and workspace context.
  - See memoryd-design.md §§4 and 7 and RFC 0001 §§7-8.
  - Success: Codex, Claude, Axinite, and manual fixtures can round-trip through
    one canonical evidence schema.
- [ ] 2.1.3. Implement idempotent ingest jobs and cursor updates.
  - Requires 2.1.1 and 2.1.2.
  - Encode provider-specific idempotency keys and retry states.
  - See memoryd-design.md §7 and RFC 0001 §9.
  - Success: property tests over repeated lines, hook retries, file rotations,
    and manual request IDs produce one stored event per idempotency key.
- [ ] 2.1.4. Implement audit records for ingest, import, retraction-ready
  placeholders, and purge-ready placeholders.
  - Requires 2.1.1.
  - See memoryd-design.md §§7, 12, and 13 and RFC 0001 §7.
  - Success: every mutating ingest path records actor, method, workspace,
    target, decision, reason, and timestamp.

### 2.2. Capture provider evidence through bounded adapters

This step answers whether external tools can be supported without making the
core model Codex-shaped, Claude-shaped, or Axinite-shaped. It informs the
projection pipeline and compatibility story. See memoryd-design.md §6 and RFC
0001 §§6 and 8.

- [ ] 2.2.1. Implement the provider adapter trait and adapter registry.
  - Requires 2.1.2.
  - Include session discovery, event reads, evidence-span reads, cursor
    persistence, and capability checks.
  - See memoryd-design.md §6 and RFC 0001 §6.
  - Success: a fixture adapter can feed canonical events through the same
    daemon ingest RPC as real adapters.
- [ ] 2.2.2. Implement the redaction pipeline before storage and embedding.
  - Requires 1.1.3 and 2.2.1.
  - Detect configured secret classes, deny patterns, high-entropy blobs, and
    raw-text storage mode.
  - See memoryd-design.md §13 and terms-of-reference.md §§7.2 and 8.1.
  - Success: redaction fixtures prove that sensitive text is replaced before
    evidence rows or future embedding payloads are written.
- [ ] 2.2.3. Implement workspace derivation for provider evidence.
  - Requires 1.1.2 and 2.2.1.
  - Derive workspace IDs from repository origin, root path hash, and optional
    profile name, with configured overrides.
  - See terms-of-reference.md §8.2 and memoryd-design.md §§8.4 and 13.
  - Success: fixtures for Git, non-Git, moved paths, and origin aliases resolve
    to expected workspace IDs or explicit collision errors.
- [ ] 2.2.4. Implement manual import for explicitly configured transcript and
  rollout paths.
  - Requires 2.2.1-2.2.3.
  - Enforce configured roots and reject arbitrary file reads.
  - See memoryd-design.md §§6, 11, and 13 and RFC 0001 §6.
  - Success: `memory_import_session` can import allowed files and refuses
    paths outside configured roots with an auditable denial.

### 2.3. Deliver Codex and Claude ingestion in observe mode

This step answers whether the first external producers can be captured with
their native persistence surfaces and without long-running work inside hooks.
It informs adapter abstractions before Axinite support lands. See
memoryd-design.md §§6-7 and RFC 0001.

- [ ] 2.3.1. Implement Codex rollout discovery and tailing.
  - Requires 2.2.1-2.2.3.
  - Honour `CODEX_HOME`, discover session and archived-session JSONL roots,
    persist byte offsets, and map rollout items to canonical events.
  - See memoryd-design.md §6 and RFC 0001 §§6 and 9.
  - Success: representative Codex rollout fixtures ingest incrementally across
    restart without duplicated events.
- [ ] 2.3.2. Implement Claude Code hook intake and transcript tailing.
  - Requires 2.2.1-2.2.3.
  - Keep hook handling as a fast wake-up path and tail transcript content
    asynchronously.
  - See terms-of-reference.md §8.1, memoryd-design.md §§6 and 13, and RFC
    0001 §§6 and 9.
  - Success: hook fixtures for session start, prompt, compaction, stop, and
    session end enqueue ingest without running projection in the hook command.
- [ ] 2.3.3. Implement provider lag, cursor, and parse diagnostics.
  - Requires 2.3.1 and 2.3.2.
  - Surface unreadable files, parse errors, stale cursors, last offsets,
    ignored deny-pattern matches, and retry state.
  - See memoryd-design.md §§12 and 15 and RFC 0001 §7.
  - Success: `memoryd-collector health` reports each provider state without
    exposing raw transcript content.

### 2.4. Expose session browsing and health through CLI and MCP

This step answers whether captured evidence is useful before memory projection
exists. It informs the MCP request and response conventions used by later
recall tools. See memoryd-design.md §§11-12.

- [ ] 2.4.1. Implement `ListSessions` and session-detail daemon RPC methods.
  - Requires steps 2.1-2.3.
  - Filter by provider, workspace, repository, model, branch, status, and time
    range.
  - See terms-of-reference.md §7.1 and memoryd-design.md §§11-12.
  - Success: operators can browse captured sessions without reading raw
    transcript files.
- [ ] 2.4.2. Implement `memory_sessions` and `memory_health` MCP tools.
  - Requires 1.1.7, 1.2.4, and 2.4.1.
  - Preserve read-only mode and capability enforcement.
  - See memoryd-design.md §§11-12.
  - Success: MCP clients can inspect sessions and daemon health in read-only
    mode, while write tools remain unavailable.
- [ ] 2.4.3. Add an end-to-end observe-mode ingest suite.
  - Requires 2.4.1 and 2.4.2.
  - Cover Codex, Claude, manual import, restart, repeated observation, and
    read-only MCP session browsing.
  - See memoryd-design.md §15 and RFC 0001 §9.
  - Success: the suite proves provider ingestion is idempotent and browseable
    without enabling projection.

## 3. Vertical slice 2: Curated memory and flat recall

Idea: if Memoryd can provide Dear Diary-like usefulness through the daemon
while still preserving provenance, audit, and retraction semantics, it can
deliver immediate value without exposing Qdrant as the public memory contract.

This slice creates the first memory loop: operators can store curated memory,
embed it, retrieve it through `flat_v1`, retract it, and explain why it was
selected. It deliberately avoids episodes, themes, and graph promotion until
the daemon-mediated flat path is dependable.

### 3.1. Index curated memory through daemon-owned Qdrant projections

This step answers whether Qdrant can serve recall without becoming the source
of truth. It informs projection-state repair and later episode indexing. See
ADR 001 and memoryd-design.md §§8.1 and 8.4.

- [ ] 3.1.1. Implement the Qdrant client port and collection manager.
  - Requires 1.1.2, 1.2.2, and 2.1.1.
  - Support per-workspace collections, named vectors, payload schemas, and
    rebuildable projection writes.
  - See memoryd-design.md §§8.1 and 8.4 and ADR 001.
  - Success: projection writes can be replayed after collection deletion
    without losing evidence or audit state.
- [ ] 3.1.2. Implement the Ollama embedding provider and embedding-model
  contract.
  - Requires 1.2.2.
  - Validate vector dimensions and record model identity with projections.
  - See memoryd-design.md §§8.3, 8.4, and 14.
  - Success: ingestion and query paths reject mismatched embedding models
    before corrupting Qdrant collections.
- [ ] 3.1.3. Implement `StoreCuratedMemory` with evidence-backed manual
  records.
  - Requires 2.1.4, 3.1.1, and 3.1.2.
  - Persist manual memory as evidence, mark it `curated`, and write serving
    payloads only through projection state.
  - See memoryd-design.md §§8.1, 8.2, 11, and 12 and RFC 0002 §§5-6.
  - Success: curated writes can be recalled, audited, and rebuilt without
    relying on Qdrant as authority.

### 3.2. Deliver `flat_v1` recall and explanation

This step answers whether a small MCP-facing recall loop can return useful
context packs before hierarchical materialization exists. It informs the
context-block format and trace fields used by later profiles. See
memoryd-design.md §10 and RFC 0005.

- [ ] 3.2.1. Implement `Recall` with the `flat_v1` profile.
  - Requires 3.1.1-3.1.3.
  - Embed the query once, retrieve Qdrant candidates, apply graph-free filters,
    and return bounded context blocks.
  - See memoryd-design.md §10 and RFC 0005 §§4-6.
  - Success: recall returns projection class, status, confidence, token
    estimate, evidence references, and fallback reason fields.
- [ ] 3.2.2. Implement `memory_recall` and `memory_store` MCP tools.
  - Requires 1.1.7, 3.1.3, and 3.2.1.
  - Keep tool request types stable, hide Qdrant collection names, and enforce
    read-only gates.
  - See memoryd-design.md §11 and ADR 001.
  - Success: an MCP client can store curated memory and recall it without any
    direct Qdrant request parameters.
- [ ] 3.2.3. Implement `memory_explain` for curated and flat-recall results.
  - Requires 3.2.1 and 3.2.2.
  - Return evidence references, projection IDs, serving-index state, recall
    scores, and selected filter reasons.
  - See terms-of-reference.md §7.1, memoryd-design.md §§10-12, and RFC 0005
    §6.
  - Success: each recalled curated memory can be traced back to stored evidence
    and its Qdrant projection state.

### 3.3. Make retraction and repair real before adding richer projection

This step answers whether daemon-owned memory can be safely corrected. It
reduces risk before model-derived claims, graph facts, and themes arrive. See
RFC 0002 and memoryd-design.md §§8 and 13.

- [ ] 3.3.1. Implement retraction for curated memories and serving payloads.
  - Requires 3.1.3 and 3.2.1.
  - Soft-delete projections, mark recall exclusion state, and preserve audit
    history.
  - See memoryd-design.md §§8.1, 11, and 12 and RFC 0002 §§8-10.
  - Success: retracted curated memory is excluded from default recall and
    visible only when explicitly requested by a privileged caller.
- [ ] 3.3.2. Implement Qdrant projection repair and reconciliation reporting.
  - Requires 3.1.1 and 3.3.1.
  - Retry failed writes, rebuild missing collections, and surface projection
    failures in health output.
  - See ADR 001 and RFC 0002 §10.
  - Success: deleting a workspace collection and running repair restores
    non-retracted serving payloads from authoritative stores.
- [ ] 3.3.3. Add an end-to-end flat-memory MCP suite.
  - Requires 3.2.2, 3.2.3, and 3.3.1.
  - Cover store, recall, explain, retract, read-only denials, and projection
    repair.
  - See memoryd-design.md §15 and RFC 0005 §§5-7.
  - Success: the suite proves the first public memory loop works without
    episodes, Oxigraph facts, or themes.

## 4. Vertical slice 3: Episodes and semantic projection

Idea: if Memoryd can turn noisy session evidence into episodes and validated
semantic carriers while rejecting unsupported model output, it can move from
"search my logs" to trustworthy derived memory without losing provenance.

This slice introduces the projection hierarchy below facts and themes:
episodes, summaries, semantic carriers, extraction outputs, support-reference
validation, and projection state.

### 4.1. Materialize episodes from provider evidence

This step answers whether provider sessions can be grouped into useful memory
units without losing chronology or hard boundary rules. It informs semantic
extraction and future raw-block expansion. See memoryd-design.md §8 and RFC
0003.

- [ ] 4.1.1. Implement draft and finalized episode materializations.
  - Requires phase 2.
  - Store source session IDs, observed start and end, title, summary slots,
    message counts, tool counts, files touched, evidence references, and
    lifecycle state.
  - See memoryd-design.md §§8 and 16 and RFC 0003 §§4-5.
  - Success: Codex, Claude, and manual fixtures produce stable episode IDs and
    evidence references across repeated projection.
- [ ] 4.1.2. Implement hard episode boundary rules for coding-agent logs.
  - Requires 4.1.1.
  - Split on provider session, workspace change, compaction, idle gap, tool
    burst, file-edit/test sequence, and model or agent switch.
  - See memoryd-design.md §8 and RFC 0003 §5.
  - Success: boundary fixtures produce expected episode partitions and never
    cross workspace IDs.
- [ ] 4.1.3. Implement episode summary projection and Qdrant indexing.
  - Requires 3.1.1, 3.1.2, and 4.1.1.
  - Use Ollama summarization where configured and a bounded extractive summary
    fallback where it is not.
  - See memoryd-design.md §§8.3-8.4 and RFC 0003 §§4 and 6.
  - Success: episode summaries are retrievable in `flat_v1` with evidence
    references and are rebuildable from finalized episodes.

### 4.2. Extract semantic carriers through the dual-path validator

This step answers whether model and non-model extraction can share one contract
and one provenance gate. It informs graph promotion and theme assignment. See
ADR 002, memoryd-design.md §8.3, and RFC 0003 §6.

- [ ] 4.2.1. Implement sentence and span mapping for evidence-backed
  extraction.
  - Requires 4.1.1.
  - Preserve source positions, content hashes, role, tool metadata, and
    redaction state through episode text windows.
  - See ADR 002 and memoryd-design.md §§7 and 8.3.
  - Success: extracted spans resolve to stored raw spans even after redaction
    and episode summarization.
- [ ] 4.2.2. Implement the `encoder_extractive` semantic extractor.
  - Requires 4.2.1.
  - Emit canonical or extractive text, semantic kind, support references,
    confidence, temporal hints, and extraction mode.
  - See ADR 002 and RFC 0003 §6.
  - Success: fixtures can produce semantic-carrier candidates without a
    generative model.
- [ ] 4.2.3. Implement the `llm_structured` Ollama extractor in shadow mode.
  - Requires 3.1.2 and 4.2.1.
  - Emit structured JSON for summaries, entities, relations, candidate facts,
    confidence, temporal hints, and evidence spans.
  - See ADR 002 and memoryd-design.md §8.3.
  - Success: invalid JSON, missing support references, and unsupported claims
    become diagnostics rather than retrievable memory.
- [ ] 4.2.4. Implement the shared support-reference validator.
  - Requires 4.2.2 and 4.2.3.
  - Validate evidence references, spans, hashes, workspace scope, temporal
    basis, and redaction boundaries.
  - See memoryd-design.md §§5.2 and 8.3 and ADR 002.
  - Success: only validated semantic carriers can enter Qdrant, Oxigraph, or
    theme management.

### 4.3. Serve semantic projection with explainable failure states

This step answers whether extracted memory can be inspected and repaired before
graph promotion. It informs operator trust and shadow evaluation. See
memoryd-design.md §§10-12 and RFC 0005.

- [ ] 4.3.1. Index accepted semantic carriers and rejected-extraction
  diagnostics.
  - Requires 3.1.1, 4.2.4, and 4.1.3.
  - Write accepted carriers to Qdrant and keep rejected extractor output as
    diagnostics only.
  - See memoryd-design.md §§8.1-8.4 and ADR 001.
  - Success: accepted carriers are recallable, rejected carriers are
    explainable, and neither path loses support-reference details.
- [ ] 4.3.2. Extend `memory_explain` to cover episodes, summaries, semantic
  carriers, and extraction failures.
  - Requires 4.3.1.
  - See terms-of-reference.md §7.1, memoryd-design.md §§10-12, and RFC 0005
    §6.
  - Success: an operator can inspect why a semantic carrier exists or why an
    extractor output was rejected.
- [ ] 4.3.3. Add an end-to-end projection provenance suite.
  - Requires steps 4.1-4.3.
  - Cover episode boundaries, encoder extraction, LLM shadow extraction,
    support validation, Qdrant projection, repair, and recall.
  - See memoryd-design.md §15, ADR 002, and RFC 0003 §§5-8.
  - Success: unsupported semantic carriers never reach serving indexes, graph
    state, or theme assignment.

## 5. Vertical slice 4: Facts, profiles, and graph-backed trust

Idea: if Memoryd can promote validated semantic carriers into graph-backed
facts and profiles with contradiction, retraction, and purge semantics, it can
make recalled memory trustworthy enough for repeated agent use.

This slice adds the graph source of truth, promotion rules, contradiction
records, profile material, and workspace purge completeness. It also adds
Axinite source adapters once the canonical evidence and graph contracts are
stable.

### 5.1. Make Oxigraph the graph-shaped authority

This step answers whether the graph boundary can own facts, provenance,
temporal edges, contradictions, retractions, and theme lineage without leaking
into Qdrant payload conventions. See memoryd-design.md §8.1, ADR 001, and RFC
0002.

- [ ] 5.1.1. Implement the graph repository and named-graph workspace layout.
  - Requires 1.1.4, 1.1.2, and 4.2.4.
  - Create graph namespaces for facts, provenance, retractions, themes, and
    temporal edges.
  - See memoryd-design.md §8.1 and ADR 001.
  - Success: graph writes and reads are scoped by workspace and never require
    clients to address Oxigraph directly.
- [ ] 5.1.2. Implement projection classes, epistemic status, scope, and
  reconciliation metadata in graph state.
  - Requires 5.1.1.
  - Represent `episode`, `summary`, `concept`, `fact`, and `profile` links,
    plus `explicit`, `curated`, `deduced`, `hypothesized`, and `retracted`
    statuses.
  - See memoryd-design.md §8.2 and RFC 0002 §§5-9.
  - Success: graph reads can distinguish direct human assertions, curated
    writes, model hypotheses, deductions, and retractions.
- [ ] 5.1.3. Implement temporal edges and valid-time basis.
  - Requires 5.1.2.
  - Track observed time, valid time, temporal basis, precedes, overlaps, and
    supersedes edges.
  - See RFC 0003 §7 and memoryd-design.md §§8.1-8.2.
  - Success: temporal recall and explanation can show whether time came from
    metadata, explicit evidence, inference, or unknown basis.

### 5.2. Promote and reconcile claim-bearing memory

This step answers whether validated semantic carriers can become facts and
profiles without treating model guesses as equivalent to human or curated
evidence. See RFC 0002 and ADR 002.

- [ ] 5.2.1. Implement promotion rules for explicit, curated, hypothesized,
  deduced, and profile candidate material.
  - Requires 5.1.2 and 4.3.1.
  - Keep model-derived statements hypothesized unless trusted evidence,
    operator curation, or rule-backed deduction upgrades them.
  - See RFC 0002 §§5-8 and memoryd-design.md §8.2.
  - Success: fixture claims promote or remain weak according to their evidence
    source and status.
- [ ] 5.2.2. Implement contradiction records and automatic weak-claim
  retraction.
  - Requires 5.2.1.
  - Auto-retract weaker conflicting hypotheses or deductions when explicit
    evidence arrives, and require operator resolution for strong conflicts.
  - See RFC 0002 §9 and terms-of-reference.md §7.1.
  - Success: contradiction fixtures produce auditable state transitions and do
    not erase historical evidence.
- [ ] 5.2.3. Implement `ReadFacts`, `memory_profile`, and graph-backed
  `memory_explain`.
  - Requires 5.1.3 and 5.2.2.
  - Preserve read-only mode and require write scope for profile updates.
  - See memoryd-design.md §§11-12 and RFC 0002 §§5-8.
  - Success: clients can read facts and profile material with status,
    confidence, scope, and evidence refs.

### 5.3. Prove purge, retraction, and Axinite compatibility

This step answers whether the system can correct or delete memory across all
authoritative and serving stores, then bring Axinite in without changing the
core evidence model. See terms-of-reference.md §§7-8, memoryd-design.md §§6,
13, and 15.

- [ ] 5.3.1. Implement workspace purge across evidence, graph, Qdrant, and
  checkpoint state.
  - Requires 3.3.2 and 5.1.1.
  - Require high-privilege capability and explicit confirmation string.
  - See terms-of-reference.md §7.2 and memoryd-design.md §§12-13 and 15.
  - Success: purge removes raw evidence, graph namespaces, Qdrant collections,
    and future checkpoint state for the target workspace.
- [ ] 5.3.2. Implement Axinite conversation and workspace source adapters.
  - Requires 2.2.1, 5.1.2, and 5.2.1.
  - Map Axinite conversations to source sessions, messages to evidence events,
    and workspace documents or revisions to document-revision evidence.
  - See terms-of-reference.md §§5-6, memoryd-design.md §6, and RFC 0001 §8.
  - Success: Axinite fixtures ingest without pretending to be Codex or Claude
    records.
- [ ] 5.3.3. Implement policy-gated Axinite projection sink in shadow mode.
  - Requires 1.1.5, 5.2.3, and 5.3.2.
  - Write no Axinite document by default; emit proposed write-back records with
    provenance and loop-prevention metadata.
  - See memoryd-design.md §§6 and 16 and RFC 0001 §6.
  - Success: operators can inspect proposed Axinite write-back without
    triggering self-reinforcing projection loops.
- [ ] 5.3.4. Add an end-to-end purge and Axinite-compatibility suite.
  - Requires 5.3.1-5.3.3.
  - Cover graph facts, profiles, contradictions, Qdrant projections, Axinite
    source evidence, shadow write-back, and purge completeness.
  - See memoryd-design.md §15 and terms-of-reference.md §§7.2 and 7.3.
  - Success: purge and Axinite ingestion remain correct across all stores and
    serving surfaces.

## 6. Vertical slice 5: Themes and hierarchical recall

Idea: if Memoryd can use Chutoro-backed themes to improve recall while keeping
theme identity, provenance, and expansion decisions inside `memoryd`, it can
scale from flat memory search to bounded context assembly without weakening
trust.

This slice adds the ThemeManager, Chutoro sessions, theme lineage, split and
merge shadowing, hierarchical recall profiles, and recall evaluation traces.

### 6.1. Bootstrap workspace-local theme management

This step answers whether accepted semantic carriers can be grouped into
navigation themes without making Chutoro authoritative for memory identity. See
memoryd-design.md §9, ADR 003, and RFC 0004.

- [ ] 6.1.1. Implement the `ThemeManager` domain service and durable theme
  records.
  - Requires 1.1.6, 4.3.1, and 5.1.1.
  - Store stable theme IDs, membership edges, lineage, summary state, and
    workspace scope.
  - See memoryd-design.md §9, ADR 003, and RFC 0004 §§4-5.
  - Success: theme state remains browseable and purgeable even if Chutoro
    checkpoints are missing.
- [ ] 6.1.2. Integrate Chutoro bootstrap clustering over accepted semantic
  carriers.
  - Requires 6.1.1 and 3.1.2.
  - Map Chutoro point indices to semantic-carrier IDs and cluster proposals to
    durable theme IDs.
  - See memoryd-design.md §9, ADR 003, and RFC 0004 §5.
  - Success: a workspace crossing `bootstrap_min_semantics` receives theme
    proposals without changing semantic-carrier identity.
- [ ] 6.1.3. Implement theme summaries as navigation artefacts.
  - Requires 6.1.1 and 6.1.2.
  - Use Ollama where configured and keep summaries out of fact promotion.
  - See memoryd-design.md §§8.1 and 9 and RFC 0004 §§4 and 8.
  - Success: theme summaries can be recalled and explained without becoming
    evidence or facts.

### 6.2. Keep themes balanced as workspaces grow

This step answers whether theme grouping can evolve without destabilizing
recall traces or auditability. It informs when hierarchical recall can become
the default. See RFC 0004 and ADR 003.

- [ ] 6.2.1. Implement incremental carrier attach and singleton handling.
  - Requires 6.1.2.
  - Route new semantic carriers to nearby themes or create singleton themes
    according to policy.
  - See RFC 0004 §§4-5.
  - Success: new carriers update theme state without full workspace
    reclustering.
- [ ] 6.2.2. Implement split and merge proposal jobs in shadow mode.
  - Requires 6.2.1.
  - Apply cooldowns, size thresholds, cohesion checks, and dominant-theme ID
    preservation rules.
  - See RFC 0004 §§5-6 and ADR 003.
  - Success: split and merge proposals are auditable and can be accepted or
    rejected without losing semantic-carrier provenance.
- [ ] 6.2.3. Implement Chutoro checkpoint compaction and rebuild.
  - Requires 6.2.2.
  - Compact sessions when tombstones or drift exceed thresholds and rebuild
    from active semantic carriers.
  - See memoryd-design.md §§9 and 17 and RFC 0004 §§5-7.
  - Success: deleting checkpoints affects performance but not durable theme
    membership or recall correctness.

### 6.3. Deliver hierarchical recall profiles

This step answers whether themes and semantic carriers improve recall enough to
justify additional expansion complexity. It informs default profile choice and
evaluation policy. See memoryd-design.md §10, ADR 004, and RFC 0005.

- [ ] 6.3.1. Implement the proxy expansion gate and `cheap_v2` recall profile.
  - Requires 5.2.3 and 6.1.3.
  - Score novelty, support density, temporal fit, reinforcement, and token
    cost with explicit reason codes.
  - See memoryd-design.md §10, ADR 004, and RFC 0005 §§5-7.
  - Success: hierarchical recall works without a judge model and exposes
    expansion decisions in the selection trace.
- [ ] 6.3.2. Implement `hierarchical_v2` context assembly.
  - Requires 6.3.1 and 6.2.2.
  - Select profile and fact material, theme summaries, semantic carriers,
    episodes, and optional raw-message blocks within a token budget.
  - See memoryd-design.md §10 and RFC 0005 §§5-7.
  - Success: returned context blocks remain bounded, ordered, and explainable
    by projection class and evidence reference.
- [ ] 6.3.3. Implement optional model-assisted gating and `evidence_v2`.
  - Requires 6.3.1 and a configured Ollama judge model.
  - Record disagreement between proxy and model-assisted gates for shadow
    evaluation.
  - See ADR 004 and RFC 0005 §§5 and 8.
  - Success: deployments can enable model-assisted expansion per workspace
    without making it mandatory for recall.
- [ ] 6.3.4. Add a combinatorial recall and mode-coverage suite.
  - Requires 6.3.1-6.3.3.
  - Cover provider, daemon mode, recall profile, read-only mode, fallback
    reason, purge state, and stale-theme state combinations.
  - See memoryd-design.md §15 and RFC 0005 §§7-8.
  - Success: every recall profile has at least one end-to-end path, and each
    fallback mode returns a stable reason code.

## 7. Vertical slice 6: Active operation and release hardening

Idea: if Memoryd can operate in active mode with bounded resource use,
inspectable repair paths, and documentation that matches implemented behaviour,
the project can move from design prototype to usable local daemon.

This slice hardens the already-delivered evidence, recall, graph, and theme
surfaces. It focuses on active-mode operation, repair, documentation, and
release packaging rather than adding new memory semantics.

### 7.1. Promote shadow paths into active operation

This step answers whether projection, recall, and theme paths can run
continuously without surprising the operator. It informs release readiness and
default configuration. See memoryd-design.md §§14-16.

- [ ] 7.1.1. Implement daemon mode transitions for `disabled`, `observe`,
  `project_shadow`, `recall_shadow`, and `active`.
  - Requires phases 2-6.
  - Gate projection writes, recall defaults, theme jobs, and write tools by
    mode.
  - See memoryd-design.md §§14-16.
  - Success: each mode has explicit behaviour, health output, and regression
    coverage for disabled capabilities.
- [ ] 7.1.2. Implement background job scheduling, backpressure, and retry
  limits.
  - Requires 7.1.1.
  - Cover ingest, finalization, extraction, projection repair, theme refresh,
    Chutoro compaction, and consolidation jobs.
  - See memoryd-design.md §§8, 9, 12, and 15.
  - Success: active mode can fall behind, recover, and report lag without
    unbounded task growth.
- [ ] 7.1.3. Implement operator-visible repair commands.
  - Requires 7.1.2.
  - Provide commands to replay evidence, rebuild Qdrant collections, rebuild
    graph projections where safe, compact themes, and inspect failed jobs.
  - See memoryd-design.md §§8.1, 9, 12, and 15.
  - Success: common projection failures can be repaired without direct store
    manipulation.

### 7.2. Make release packaging and documentation match reality

This step answers whether the project can be installed, evaluated, and
contributed to using documented commands. It informs the first public release
candidate. See README.md, users-guide.md, developers-guide.md, and
memoryd-design.md §15.

- [ ] 7.2.1. Implement installation and packaging metadata for all public
  binaries.
  - Requires 7.1.1.
  - Align Cargo metadata, release assets, `cargo binstall` expectations, and
    documented binary names.
  - See README.md, docs/users-guide.md, and Cargo.toml.
  - Success: a release asset can install `memoryd`, `memoryd-collector`, and
    `memoryd-mcp` with documented commands.
- [ ] 7.2.2. Update user documentation for evidence capture, flat recall,
  hierarchical recall, purge, and Axinite compatibility.
  - Requires phases 2-6.
  - See docs/contents.md, docs/users-guide.md, docs/memoryd-design.md, and
    terms-of-reference.md §7.
  - Success: user-facing documentation describes implemented behaviour, not
    design-only capability.
- [ ] 7.2.3. Update developer documentation for crate boundaries, fixtures,
  adapters, graph state, Qdrant projections, and Chutoro checkpoints.
  - Requires phases 2-6.
  - See docs/developers-guide.md, docs/repository-layout.md, and
    memoryd-design.md §§5-15.
  - Success: a contributor can add a provider adapter or projection path using
    documented contracts and fixture expectations.
- [ ] 7.2.4. Add release-readiness end-to-end validation.
  - Requires 7.2.1-7.2.3.
  - Cover local install, first-run config, observe-mode ingest, curated store,
    flat recall, hierarchical recall, health, repair, and purge.
  - See memoryd-design.md §15 and terms-of-reference.md §7.
  - Success: the release gate exercises the same user journeys documented in
    the README and users' guide.

## 8. Deferred extensions after the core v1 promise

Idea: if the core v1 promise is already trustworthy and boring to operate, the
project can evaluate broader extensions on product value instead of letting
them destabilize local-first memory, provenance, and recall.

This phase collects design-adjacent work that is explicitly out of scope for
the core roadmap or depends on evidence from earlier phases. These items should
not block the v1 path.

### 8.1. Evaluate ambient memory injection

This step answers whether push-style context injection can be made safe enough
to complement pull-based MCP recall. See terms-of-reference.md §6.2 and
memoryd-design.md §2.2.

- [ ] 8.1.1. Decide whether ambient injection graduates from deferred scope.
  - Requires phase 7.
  - Compare explicit MCP pull recall against hook-based or client-driven
    context injection with consent, audit, and workspace policy.
  - See terms-of-reference.md §6.2 and memoryd-design.md §2.2.
  - Success: one ADR either rejects ambient injection for the next release or
    defines strict consent and safety requirements.

### 8.2. Evaluate hosted or fleet operation

This step answers whether organization-wide memory observability belongs in the
product after the local-first daemon is stable. See terms-of-reference.md §§4
and 6.2.

- [ ] 8.2.1. Decide whether hosted analytics or multi-tenant administration is
  a separate product.
  - Requires phase 7.
  - Preserve local-first defaults and avoid weakening workspace purge or
    privacy guarantees.
  - See terms-of-reference.md §§4 and 6.2 and memoryd-design.md §2.2.
  - Success: hosted or fleet scope is either deferred explicitly or split into
    a separate design document.

### 8.3. Evaluate alternative dependency profiles

This step answers whether smaller deployments can keep useful memory behaviour
without the full Qdrant, Ollama, Oxigraph, and Chutoro capability set. See
terms-of-reference.md §§8.2-8.3 and memoryd-design.md §17.

- [ ] 8.3.1. Decide whether a dependency-light profile is worth supporting.
  - Requires phase 7.
  - Compare local setup complexity against lost provenance, graph, theme, and
    recall capabilities.
  - See terms-of-reference.md §§8.2-8.3, memoryd-design.md §17, ADR 001, ADR
    003, and ADR 004.
  - Success: one ADR defines any supported reduced profile and the exact
    capabilities it defers.
