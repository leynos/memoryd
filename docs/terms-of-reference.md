# Memoryd – terms of reference

- **Status:** Draft v0.1.
- **Audience:** Product owners, engineering leads, maintainers of `memoryd`,
  Axinite integrators, and agent-tooling maintainers.
- **Companion documents:** See the references appendix for the Axinite RFCs,
  Axinite ADRs, Dear Diary MCP precedent, and Chutoro clustering documents that
  inform this terms of reference.
- **Last substantive revision:** 2026-05-25.

## 1. Background and motivation

`memoryd` exists to turn coding-agent session history into durable, local,
evidence-backed memory. The immediate trigger is a shift in how coding agents
are used: Codex CLI, Claude Code, and Axinite can all generate useful
conversation records, tool traces, file-edit evidence, and compaction
summaries, but those records remain bound to individual tools and sessions.
When context windows close or conversations compact, useful decisions and
evidence become hard to recover.

Axinite already contains a proposed `memoryd` sidecar design. That design
describes a local Rust sidecar that consumes memory events, uses Ollama for
structured extraction and embeddings, stores retrieval vectors in Qdrant,
stores normalized facts and provenance in Oxigraph, and exposes a narrow Unix
domain socket (UDS) remote procedure call (RPC) surface protected by capability
tokens.[^1] The standalone project keeps that problem framing but removes
Axinite as the only producer. Codex CLI rollout logs, Claude Code hooks and
transcripts, manual imports, and future Axinite adapters all become sources of
evidence.

The work is worth doing now because coding agents increasingly operate across
long-lived repositories, repeated worktrees, compacted conversations, and
multiple tools. The current default is manual memory: `AGENTS.md`, project
notes, copied summaries, grep over transcript files, or a simple vector store.
Those defaults do not reliably preserve provenance, epistemic status,
contradictions, retractions, or workspace-local deletion.

## 2. Domain

The domain is local-first memory infrastructure for software-engineering
agents. It overlaps with semantic search, vector databases, transcript
indexing, knowledge graphs, Model Context Protocol (MCP) tools, and
agent-workflow observability.

The field has several conventions that shape `memoryd`:

- Coding-agent transcripts mix user intent, assistant speculation, tool calls,
  tool output, filesystem changes, approvals, errors, and compaction summaries.
- Useful recall requires more than nearest-neighbour search. A retrieved item
  must expose where it came from, who asserted it, how trusted it is, and
  whether later evidence retracts or supersedes it.
- Local-first deployments must treat transcript data as sensitive because logs
  can include secrets, source code, shell output, credentials, customer data,
  and private repository paths.
- Agent-facing memory should be pull-based by default. A client asks for a
  bounded context pack rather than receiving ambient, uninspected memory in
  every prompt.

The main prior art inside the project set is Axinite's memory sidecar RFC and
follow-on memory RFCs. Those documents define a projection taxonomy of
`episode`, `summary`, `concept`, `fact`, and `profile`, with epistemic states
such as `explicit`, `curated`, `deduced`, `hypothesized`, and `retracted`.[^2]
They also define a hierarchy from raw evidence to episodes, semantic carriers,
and optional themes, while keeping themes as navigation structures rather than
truth claims.[^3]

Dear Diary is the closest MCP precedent. It exposes a small Rust MCP server for
persistent semantic memory with `qdrant_store`, `qdrant_find`, and
`qdrant_deprecate`, backed directly by Qdrant and local embeddings.[^4] That
shape proves the usefulness of a small MCP memory tool, but standalone
`memoryd` must treat Qdrant as a serving index rather than the source of truth.

Chutoro is the clustering prior art. It implements FISHDBC, a scalable
density-based clustering method that replaces an all-pairs distance matrix with
an approximate Hierarchical Navigable Small World (HNSW) graph and supports
arbitrary distance functions through a `DataSource` trait.[^5] Axinite's
theme-management ADR already places memory-specific theme identity, lineage,
workspace isolation, and balancing policy in `memoryd`, with Chutoro serving as
the clustering substrate.[^6]

Corbusier is the tenant-isolation prior art inside the project set. It carries
tenant identity through a request context alongside correlation, causation,
user, and session identifiers, and its PostgreSQL adapter work plans to set a
transaction-local tenant value for row-level security (RLS). Standalone
`memoryd` must therefore treat tenant context as a first-class boundary when
Corbusier supplies or consumes memory.[^7]

## 3. Market context

The project competes against existing ways developers and agent operators try
to preserve context:

| Alternative                        | Current role                                                | Gap addressed by `memoryd`                                                           |
| ---------------------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Manual notes and `AGENTS.md` files | Durable, human-edited instructions and project memory       | Manual upkeep, no transcript provenance, and weak recall over raw work history       |
| Agent transcript search            | Grep, file viewers, or ad hoc scripts over session logs     | Exact-text search misses semantic links and does not classify evidence               |
| Simple vector-memory MCP servers   | Lightweight semantic store and retrieve workflows           | Vectors alone do not preserve facts, retractions, contradictions, or promotion rules |
| Axinite workspace memory           | Database-backed documents with hybrid search inside Axinite | Bound to Axinite unless adapted; does not by itself ingest Codex or Claude sessions  |
| Do nothing                         | Rely on the current context window and human recollection   | Repeated rediscovery, lost decisions, and weak continuity across tools               |

_Table 1: Current alternatives and the specific deficiency addressed._

The target gap is a trust gap, not only a retrieval gap. The product must help
agents recover context while preserving why that context should be believed. A
design that exposes Qdrant collections directly through MCP would fill the
search gap but miss the provenance, retraction, and epistemic-status gap that
motivates the standalone service.

## 4. Users and stakeholders

| Group                                     | Context                                                                                   | Cares about                                                                                          | Will ignore or dislike                                                                            | Current alternative                                    |
| ----------------------------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| Primary user: local coding-agent operator | Runs Codex CLI, Claude Code, Axinite, or similar tools across software projects           | Recovering prior decisions, preserving evidence, keeping data local, and avoiding memory drift       | Opaque automatic prompt injection, direct exposure of raw transcripts, and manual curation burden | Notes, transcript search, and simple MCP memory stores |
| Primary user: agent-tool maintainer       | Builds integrations, adapters, and local services for agent workflows                     | Clear provider boundaries, idempotent ingestion, inspectable recall, and testable contracts          | Provider-specific core models or hard-coded assumptions about one CLI                             | Custom import scripts and per-tool storage             |
| Secondary user: reviewer or teammate      | Reads summaries, context packs, and audit trails produced by the system                   | Traceability from memory to evidence and clear status labels                                         | Unverifiable claims and unbounded transcript dumps                                                | Asking the original operator or reading raw logs       |
| Stakeholder: Axinite maintainer           | Wants Axinite's planned memory capabilities to remain usable through a standalone service | Adapter compatibility, optional projection write-back, and preservation of existing memory semantics | A standalone model that forces Axinite to impersonate Codex or Claude                             | Axinite-only sidecar design                            |
| Stakeholder: Corbusier maintainer         | Wants Corbusier tenants to use `memoryd` without cross-tenant memory leakage              | Request-context compatibility, tenant-scoped recall, auditability, and storage enforcement           | Treating tenant IDs as optional metadata or relying on workspace IDs alone                        | Corbusier-local memory or tenant-specific silos        |
| Stakeholder: security-conscious operator  | Runs agents on private repositories or sensitive workspaces                               | Local-only defaults, redaction, purge semantics, and least-privilege interfaces                      | Network-first storage, broad file access, and unreviewed hook commands                            | Disabling memory or using manual notes                 |
| Non-user: hosted analytics buyer          | Wants cloud dashboards over all organizational agent activity                             | Fleet analytics, central administration, and business reporting                                      | Local-first per-workspace operation                                                               | Commercial observability or data platform products     |

_Table 2: User and stakeholder map._

## 5. Job to be done

When a local coding-agent operator returns to a project after context has been
lost, compacted, or split across tools, they want to retrieve the relevant
prior decisions and supporting evidence so they can continue work without
reconstructing the history manually.

When an agent-tool maintainer adds a new producer such as Axinite, Codex CLI,
or Claude Code, they want to map provider-specific records into a canonical
evidence model so the memory pipeline can preserve source semantics without
becoming provider-shaped.

When a reviewer inspects recalled memory, they want each claim to expose its
evidence, confidence, status, and retraction state so they can decide whether
to trust it in the current task.

When an Axinite maintainer adopts standalone `memoryd`, they want Axinite to
remain a first-class producer and optional projection consumer so Axinite's
conversation, workspace, episode, and fact semantics remain compatible with the
standalone service.

When a Corbusier maintainer adopts standalone `memoryd`, they want every
evidence, projection, recall, graph, vector, clustering, audit, and purge path
to be scoped by Corbusier-compatible tenant context so one tenant cannot infer,
retrieve, corrupt, or delete another tenant's memory.

## 6. Scope

### 6.1 Goals

- Capture Codex CLI, Claude Code, Axinite, and manual session evidence without
  requiring those tools to link against a shared library.
- Provide a standardized conversation ingestion port so Corbusier, Axinite,
  Codex, Claude, and manual import adapters can emit one canonical conversation
  delta shape.
- Normalize provider records into evidence objects that preserve source
  session, workspace, actor, event kind, text spans, payload hashes, and
  evidence references.
- Distinguish raw evidence, episodes, semantic carriers, facts, profiles, and
  themes so retrieval does not flatten all memory into undifferentiated text.
- Preserve provenance, epistemic status, contradiction records, retractions,
  and purge behaviour for claim-bearing memory.
- Return bounded MCP recall context packs with projection class, epistemic
  status, confidence, selected evidence, and explanation data.
- Keep Axinite compatibility through adapters that supply evidence and may
  receive optional, policy-gated projections.
- Support Corbusier-style tenant-scoped operation through a request context,
  tenant-aware storage keys, tenant-scoped indexes, tenant-scoped graph
  namespaces, and tenant-scoped clustering checkpoints.
- Preserve pre-1.0 epistemic substrate records for source health, stable claim
  identity, interpretive claim kind, typed support edges, projection activity
  lineage, and optional durable recall audits.
- Use Qdrant, Ollama, Oxigraph, and Chutoro in roles compatible with the
  Axinite memory RFCs and ADRs.
- Default to local-first operation with conservative redaction before storage
  and embedding.

### 6.2 Non-goals

- `memoryd` will not generate final user answers. It returns context and
  evidence for an agent or MCP client to use.
- `memoryd` will not make Qdrant the source of truth for facts, provenance, or
  retractions. Qdrant serves recall indexes and denormalized payloads.
- `memoryd` will not write back into Codex CLI or Claude Code transcript logs.
  Those logs are append-only external evidence streams.
- `memoryd` will not inject memory automatically into every agent session by
  default. Ambient memory injection is out of scope until explicit policy and
  safety rules exist.
- `memoryd` will not expose arbitrary transcript-file reads through MCP.
  Provider adapters read configured roots; MCP clients ask for recall,
  explanation, sessions, imports, and health.
- `memoryd` will not make Chutoro cluster labels durable theme identifiers.
  Chutoro proposes cluster structure; `memoryd` owns memory theme identity and
  lineage.
- `memoryd` will not infer causality, run experiments, or model organizational
  uptake in v1. Those post-1.0 support capabilities are proposed separately in
  RFC 0006 as substrate records for Axinite-style agentic workflows.
- `memoryd` will not solve organization-wide hosted analytics, central
  compliance reporting, or tenant lifecycle administration in v1. Tenant
  isolation for Corbusier-compatible callers is in scope; a hosted control
  plane is not.

## 7. Success criteria

### 7.1 User-facing success

- A local operator can ask an MCP client for prior project decisions and
  receive a bounded context pack that includes evidence references and status
  labels rather than an unstructured transcript dump.
- An operator can browse ingested sessions by provider, repository,
  workspace, branch, model, and time range.
- A recalled claim can be explained: the system can show why it exists, which
  evidence supports it, which typed support edges validate it, which source
  health affected that support, and whether it has been retracted or superseded.
- Decision-relevant recall can be audited through bounded, tenant-scoped recall
  records without storing raw query text by default.
- Axinite can use the standalone service through adapters without treating
  Axinite conversations as Codex or Claude transcripts.
- Corbusier can call `memoryd` with an authenticated tenant context and receive
  only tenant-scoped sessions, recall results, explanations, and health data.
- Corbusier and Axinite conversation adapters can use the same daemon
  ingestion port as the worker process that scrapes Codex and Claude session
  files.

### 7.2 Operational success

- Ingestion is idempotent across restarts, repeated file observations, hook
  retries, and manual imports.
- Redaction happens before storage and before embedding for configured secret
  classes and deny patterns.
- Purging a workspace removes raw evidence rows, graph namespaces, Qdrant
  collections or workspace-scoped payloads, and Chutoro checkpoints.
- Tenant isolation is enforced in every storage-backed path: application ports
  require tenant context, tenant-owned persistence rows carry tenant identity,
  vector searches use tenant filters or tenant-scoped collections, graph reads
  use tenant-scoped named graphs, and Chutoro sessions never mix tenants.
- Recall has a flat fallback profile when hierarchical structures are absent,
  stale, or disabled.
- Health checks report daemon state, collector lag, Qdrant, Ollama, Oxigraph,
  Chutoro theme state, adapter status, source freshness, parse failures, and
  inaccessible configured sources.

### 7.3 Strategic success

- The standalone project preserves the planned Axinite memory capabilities
  while making them reusable by non-Axinite agent workflows.
- The MCP surface stays small enough for agent clients to use safely but rich
  enough to support recall, curated writes, retraction, explanation, session
  browsing, imports, profiles, and health.
- The project can move from evidence capture to hierarchical recall in stages
  without discarding early data or changing the canonical evidence model.

## 8. Constraints and assumptions

### 8.1 Hard constraints

- The implementation must follow the repository's Rust and documentation
  standards in `AGENTS.md` and `docs/documentation-style-guide.md`.
- Qdrant, Ollama, and Chutoro remain planned dependencies for the target
  capability set because the prompt explicitly requires retaining those
  capabilities.
- Axinite compatibility remains in scope. The standalone service must support
  an adapter boundary for Axinite conversations, workspace documents, episodes,
  facts, and optional projection write-back.
- Corbusier compatibility remains in scope. The standalone service must accept
  tenant context from Corbusier-style authenticated callers and enforce that
  context before any tenant-owned read, write, recall, projection, repair, or
  purge operation.
- Claude Code integration must account for hook security. Claude Code command
  hooks run with the user's permissions, and hook handlers receive JSON on
  standard input for command hooks.[^8]
- The default deployment posture is local-first. Network-facing service
  exposure, if added, must be an explicit mode rather than the default.

### 8.2 Assumptions

- Codex CLI rollout files and Claude Code transcripts provide enough durable
  session evidence to build useful memory. If this assumption fails, the first
  product slice must add stronger explicit import or hook capture.
- Users will accept local infrastructure dependencies when they receive better
  provenance and privacy than a hosted memory service. If they do not, a
  smaller single-binary mode or dependency-light profile becomes necessary.
- Local embedding and extraction models through Ollama can provide adequate
  quality for summaries, semantic extraction, and optional recall gating. If
  they cannot, the product must keep encoder-only and flat-recall fallbacks.
- Provider adapters can preserve enough ordering, actor, span, and workspace
  metadata to avoid provider-specific logic in the core pipeline. If they
  cannot, the canonical model needs explicit extension points before design
  work continues.
- Workspace identity can usually derive from repository origin, repository
  root, and optional profile name. If this proves unstable, purge and recall
  isolation become unsafe.
- Tenant identity is available from authenticated callers, capability tokens,
  or a configured local default. If tenant context is missing in Corbusier
  mode, the request must fail before application use cases run.

### 8.3 Dependencies

- Qdrant supplies vector serving indexes for episodes, summaries, semantic
  carriers, themes, and profiles.
- Ollama supplies local embeddings, structured extraction, summarization, and
  optional judge-model support.
- Oxigraph supplies graph-shaped provenance, facts, contradictions,
  retractions, theme lineage, and temporal edges when the full Axinite memory
  capability set is retained.
- Chutoro supplies clustering proposals over accepted semantic carriers.
- Claude Code supplies hook events and transcript paths through its documented
  hook system.
- Codex CLI supplies rollout JSONL files and session metadata as external
  evidence streams.
- Axinite supplies future source and projection adapters over conversations,
  workspace documents, episodes, and facts.
- Corbusier supplies a compatible request-context model for tenant-aware
  invocation, and may use PostgreSQL RLS with transaction-local tenant settings
  in deployments that need database-enforced isolation.

## 9. Open questions

| Question                                                                                                     | Why it matters                                                                           | Criteria for resolution                                                              | Suggested path                     |
| ------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | ---------------------------------- |
| Which local store should back the standalone evidence inbox: SQLite, libSQL, PostgreSQL, or a supported set? | It affects installation complexity, transactional guarantees, and Axinite compatibility  | A selected default plus documented migration and backup rules                        | Technical design decision          |
| Is Oxigraph mandatory for v1, or may an MVP use graph-shaped relational tables?                              | Dropping Oxigraph reduces dependencies but weakens the planned graph source of truth     | A decision that states which Axinite capabilities are deferred if Oxigraph is absent | Architecture Decision Record (ADR) |
| What is the exact canonical evidence schema for Codex, Claude, Axinite, and manual imports?                  | The schema gates adapters, idempotency, redaction, projection, and tests                 | Provider examples round-trip through normalization with stable evidence references   | Implementation spike               |
| How should workspace identity be derived when repository origin, path, branch, or profile changes?           | Incorrect identity risks cross-project recall or incomplete purge                        | A deterministic rule with collision handling and operator override                   | Technical design decision          |
| What redaction policy is sufficient for the first release?                                                   | Redaction governs whether transcripts can be stored and embedded safely                  | A documented default deny list, secret detector set, and override model              | Security review                    |
| Should Axinite projection write-back be manual, approved, or automatic by default?                           | Write-back can duplicate or strengthen derived facts if loops are not controlled         | A policy that prevents self-reinforcing projection loops                             | ADR                                |
| Which MCP tools are required in the first public slice?                                                      | The MCP surface affects immediate usefulness and implementation order                    | A minimum useful set tied to evidence capture and flat recall                        | Roadmap                            |
| What recall quality signals will decide whether hierarchical recall is better than flat recall?              | Without evaluation signals, theme and episode expansion can add complexity without value | A shadow-mode evaluation set with traceable disagreement and token-cost metrics      | Evaluation plan                    |
| Which tenant storage strategy should each supported deployment use?                                          | SQLite, PostgreSQL, Qdrant, Oxigraph, and Chutoro enforce isolation differently          | A selected local default, Corbusier mode, and hosted-ready extension point           | ADR                                |

_Table 3: Open questions for the next design iteration._

## Appendices

### Appendix A. Candidate context glossary entries

`docs/context.md` does not yet exist. The following terms should be promoted
there when the project creates a ubiquitous language document:

- **Evidence event:** A normalized record from a provider stream, such as a
  user message, assistant message, tool call, tool result, compaction event,
  file edit, approval, or error.
- **Evidence reference:** A stable pointer from a memory artefact back to the
  source session, file path, line or byte range, hash, and provider context
  that support it.
- **Episode:** A bounded, chronological memory unit materialized from raw
  evidence.
- **Semantic carrier:** A reusable statement distilled from accepted evidence
  that may support facts, concepts, profile material, or themes.
- **Epistemic status:** The trust status of a claim-bearing artefact, such as
  `explicit`, `curated`, `deduced`, `hypothesized`, or `retracted`.
- **Theme:** A derived navigation grouping over semantic carriers. A theme is
  not evidence and is not a truth claim.
- **Projection:** A derived memory artefact or index entry created from raw
  evidence, such as a summary, fact, Qdrant payload, or graph edge.
- **Tenant:** The authority boundary for access to evidence, projections,
  recall, graph state, vector indexes, themes, audit records, and purge.
- **Request context:** Authenticated invocation context carrying tenant,
  principal, session, correlation, and optional causation identifiers.

### Appendix B. ADR candidates

- Decide whether Oxigraph is mandatory in v1 or whether a relational prototype
  is acceptable.
- Decide workspace identity derivation and collision behaviour.
- Decide tenant isolation and Corbusier request-context compatibility.
- Decide Axinite projection write-back policy and loop prevention.
- Decide redaction guarantees and whether encrypted raw-text storage is in
  scope.

### Appendix C. Downstream design inputs

The technical design should start from these constraints:

- Treat logs as evidence, not memory.
- Keep Qdrant as an index, not a truth store.
- Keep Chutoro as a cluster proposal engine, not the owner of memory themes.
- Keep Ollama outputs behind schema validation and support-reference checks.
- Keep provider adapters above raw storage and below semantic projection.
- Keep provider-specific conversation parsing in adapters and converge on a
  canonical conversation delta before evidence inbox writes.
- Keep Axinite as a first-class adapter, not as a special case baked into the
  core model.
- Keep tenant context as part of every tenant-owned use case and port, not as
  an optional payload field.

These points are design inputs, not implementation sequence. A separate roadmap
should decide the delivery order.

### References

[^1]: `../axinite/docs/rfcs/0007-secure-memory-sidecar-design.md`, "Executive
    summary".
[^2]: `../axinite/docs/rfcs/0014-memory-projection-tiers-and-promotion-rules.md`,
    "Projection classes" and "Epistemic status".
[^3]: `../axinite/docs/rfcs/0015-hierarchical-memory-materialization-for-memoryd.md`,
    "Summary" and "Source-of-truth boundaries".
[^4]: `../dear-diary/README.md`, "Core functionality" and "Features"; also
    `../dear-diary/crates/dear-diary-mcp/src/server.rs`.
[^5]: `../chutoro/README.md`, "Why chutoro" and "Features"; also
    `../chutoro/docs/chutoro-design.md`, "The FISHDBC Algorithm
    Deconstructed".
[^6]: `../axinite/docs/adr-003-theme-management-belongs-in-memoryd.md`,
    "Decision outcome / proposed direction".
[^7]: Corbusier tenant context references:
    `../corbusier/src/context/request_context.rs`,
    `../corbusier/src/context/ids.rs`,
    `../corbusier/src/message/adapters/postgres/tenant_tx.rs`,
    `../corbusier/docs/roadmap.md`, and
    `../corbusier/docs/users-guide.md`.
[^8]: Claude Code documentation, "Hooks reference", accessed 2026-05-25:
    <https://code.claude.com/docs/en/hooks>.
