# Memoryd design

- **Status:** Draft v0.1.
- **Scope:** Standalone local `memoryd` daemon, collector sidecar, MCP front
  end, provider adapters, evidence model, projection pipeline, recall path, and
  storage boundaries.
- **Audience:** Implementers, design reviewers, Axinite integrators, and
  operators running local coding-agent memory infrastructure.
- **Companion documents:** `docs/terms-of-reference.md`,
  `docs/rfcs/0001-standalone-evidence-inbox.md`,
  `docs/rfcs/0002-projection-tiers-and-promotion-rules.md`,
  `docs/rfcs/0003-hierarchical-materialization.md`,
  `docs/rfcs/0004-theme-detection-and-rebalancing.md`,
  `docs/rfcs/0005-hierarchical-recall.md`,
  `docs/adr-001-qdrant-is-a-serving-index.md`,
  `docs/adr-002-dual-path-semantic-extraction.md`,
  `docs/adr-003-memoryd-owns-theme-management.md`,
  `docs/adr-004-dual-mode-recall-gating.md`,
  `docs/adr-005-hexagonal-architecture-boundary.md`, and
  `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- **Last substantive revision:** 2026-05-25.

## 1. Problem statement

`memoryd` turns coding-agent session history into durable, local,
evidence-backed memory. Codex CLI, Claude Code, Axinite, and manual imports all
produce useful records, but their native formats are tool-specific and mix
human assertions, assistant guesses, tool calls, tool output, file edits,
errors, and compaction summaries. A standalone service must preserve those
records as evidence, derive memory artefacts with explicit provenance, and
serve bounded recall context without making Qdrant or model output the source
of truth.

The design imports the Axinite `memoryd` RFC set, then changes the ingestion
boundary. Axinite’s original design consumes a transactional PostgreSQL outbox
written inside IronClaw’s persistence transactions.[^1] Standalone `memoryd`
cannot assume external tools share that database. It replaces the single
Axinite outbox with a local evidence inbox fed by provider adapters, while
retaining Axinite’s projection taxonomy, provenance rules, Qdrant/Oxigraph
split, Chutoro theme boundary, and hierarchical recall model.[^2][^3][^4][^5]

## 2. Design goals and non-goals

### 2.1 Goals

- Ingest Codex CLI rollout logs, Claude Code hook and transcript data, Axinite
  conversations, Axinite workspace documents, and manual imports through
  provider adapters.
- Normalize provider records into a canonical evidence model before projection.
- Preserve evidence references from every retrievable semantic artefact back to
  source sessions, ordinals, byte ranges, hashes, and provider metadata.
- Keep Qdrant as a serving index, Oxigraph as graph-shaped provenance and fact
  authority, and the evidence store as raw-event authority.
- Use Ollama for local embeddings, summarization, structured extraction, and
  optional judge-model recall gating.
- Use Chutoro for clustering proposals over accepted semantic carriers while
  keeping durable theme identity inside `memoryd`.
- Expose an MCP front end shaped like Dear Diary’s ergonomic server, but route
  every operation through `memoryd` instead of exposing Qdrant directly.[^6]
- Keep Axinite usable as a first-class provider and optional projection sink.
- Support Corbusier-compatible tenant isolation through authenticated request
  context, tenant-scoped workspaces, tenant-aware storage, tenant-scoped
  serving indexes, tenant-scoped graph namespaces, and tenant-scoped Chutoro
  checkpoints.

### 2.2 Non-goals

- `memoryd` does not generate final answers. It returns context packs and
  provenance for clients to use.
- `memoryd` does not write into Codex CLI or Claude Code transcript files.
- `memoryd` does not inject memory ambiently into every agent session by
  default.
- `memoryd` does not expose arbitrary file reads through MCP.
- `memoryd` does not make Chutoro cluster labels durable theme IDs.
- `memoryd` does not attempt hosted, organization-wide analytics, billing,
  central compliance reporting, or tenant lifecycle administration in this
  design. Tenant isolation for Corbusier-compatible callers is in scope; a
  hosted control plane is not.

## 3. Research summary

| Source                     | Finding                                                                                                                                                                      | Design consequence                                                                                                            |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Axinite RFC 0007           | The original sidecar uses local Rust, UDS RPC, capability scopes, Ollama, Qdrant, Oxigraph, and a transactional outbox.                                                      | Preserve the processing and trust model, but replace the producer boundary with provider adapters and a local evidence inbox. |
| Axinite RFC 0014           | Trustworthy recall needs projection classes, epistemic status, observer/subject scope, promotion, contradiction handling, and reconciliation metadata.                       | Keep these semantics as the standalone memory contract.                                                                       |
| Axinite RFC 0015           | Raw evidence, episodes, semantic carriers, and themes are separate hierarchy levels; themes are not truth claims.                                                            | Build an additive projection pipeline with support-reference validation before recall.                                        |
| Axinite RFCs 0016 and 0017 | Chutoro-backed themes and hierarchical recall improve navigation only if provenance and projection classes remain visible.                                                   | Add theme management and recall profiles after evidence capture and flat recall work.                                         |
| Axinite ADRs 003-005       | Theme identity belongs in `memoryd`; semantic extraction needs dual paths; recall expansion needs proxy and optional model-assisted gating.                                  | Import these decisions as standalone ADRs with provider-adapter wording.                                                      |
| Dear Diary                 | A small Rust MCP server can expose simple store, find, and deprecate tools over Qdrant with read-only gates.                                                                 | Reuse the ergonomic MCP shape, but replace direct Qdrant operations with `memoryd` RPC calls.                                 |
| Chutoro                    | Chutoro implements FISHDBC with HNSW, arbitrary `DataSource` metrics, sessions, and snapshots.                                                                               | Use Chutoro as a cluster proposal engine over semantic-carrier vectors, not as memory policy.                                 |
| Corbusier tenant context   | Corbusier carries tenant, correlation, causation, user, and session identifiers in `RequestContext`, and plans PostgreSQL RLS through transaction-local tenant settings.[^9] | Make tenant context part of every tenant-owned use case and port, rather than optional metadata.                              |
| PostgreSQL RLS             | Row security policies restrict rows returned or modified, default to deny without policies, and can use session or transaction settings for tenant identity.[^10][^11]       | PostgreSQL adapters should combine application scoping with database-enforced tenant policies.                                |
| Qdrant multitenancy        | Qdrant recommends shared collections with tenant payload filters and tenant keyword indexes for high-cardinality multitenancy.[^12]                                          | Support payload-partitioned collections as a hosted strategy, while keeping mandatory daemon-injected tenant filters.         |
| OWASP multitenancy         | Tenant context should be established early, bound to authenticated identity, propagated through layers, and audited with tenant-aware resource checks.[^13]                  | Derive tenant context before application use cases run and audit all tenant-owned access decisions.                           |
| RDF named graphs           | RDF datasets contain named graphs that can keep graph contents separately addressable.[^14]                                                                                  | Scope Oxigraph graph names by tenant and workspace.                                                                           |
| Claude Code hooks          | Hooks provide lifecycle events, command hooks receive JSON on standard input, async hooks cannot block, and command hooks run with the user's permissions.[^7]               | Treat hooks as wake-up signals and source metadata, not as trusted long-running ingestion workers.                            |
| MCP specification          | MCP uses JSON-RPC 2.0, server tools/resources/prompts, capability negotiation, and explicit security guidance around consent and data access.[^8]                            | Keep MCP tools narrow, read-only mode explicit, and write or purge operations capability-gated.                               |

_Table 1: Research findings that shape the standalone design._

## 4. Terminology

| Term                | Definition                                                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| Provider adapter    | Component that converts one producer’s native records into canonical `memoryd` evidence events.                                           |
| Evidence inbox      | Durable local store for source sessions, cursors, raw events, raw spans, ingest jobs, projection sync state, and audit history.           |
| Evidence event      | Canonical record of an observed provider event, such as a user message, assistant message, tool call, compaction, approval, or file edit. |
| Evidence reference  | Stable pointer to source path, line or byte range, provider session, event ordinal, and content hash.                                     |
| Episode             | Bounded chronological memory unit materialized from raw evidence.                                                                         |
| Semantic carrier    | Validated statement distilled from one or more episodes or document spans and used to support facts, concepts, profiles, or themes.       |
| Projection          | Derived artefact or serving record written from evidence, such as an episode summary, fact, graph edge, Qdrant payload, or theme.         |
| Theme               | Durable navigation grouping over accepted semantic carriers. A theme is not evidence and is not a fact.                                   |
| Recall context pack | Bounded response containing context blocks, projection classes, epistemic status, confidence, evidence references, and selection trace.   |
| Tenant              | Authority boundary for evidence, projections, recall, graph state, vector indexes, themes, audit records, and purge.                      |
| Request context     | Authenticated invocation context carrying tenant, principal, session, correlation, and optional causation identifiers.                    |
| Driving adapter     | Edge component that invokes a use case, such as CLI, MCP, provider collector, scheduled job, hook handler, or debug HTTP.                 |
| Driven adapter      | Edge component that implements a domain-owned port, such as persistence, Qdrant, Ollama, Oxigraph, Chutoro, clock, or audit output.       |
| Port                | Domain-owned trait that describes a capability the application needs, using domain types rather than infrastructure types.                |

_Table 2: Normative terminology._

## 5. System architecture

Standalone `memoryd` uses a provider-adapter pipeline. Producers remain
external. The collector and provider adapters read configured sources and send
canonical evidence to the daemon. The daemon owns evidence persistence,
projection, graph state, serving indexes, themes, and recall. The MCP server is
a front end over daemon RPC.

```mermaid
flowchart LR
    Codex[Codex CLI rollout JSONL]
    Claude[Claude Code hooks and transcripts]
    Axinite[Axinite adapters]
    Manual[Manual imports]

    Collector[memoryd-collector]
    Daemon[memoryd daemon]
    MCP[memoryd-mcp]

    Inbox[(Evidence inbox)]
    Graph[(Oxigraph)]
    Vector[(Qdrant)]
    Models[Ollama]
    Cluster[Chutoro]

    Codex --> Collector
    Claude --> Collector
    Axinite --> Collector
    Manual --> Collector
    Collector -->|UDS or loopback ingest RPC| Daemon
    MCP -->|UDS or loopback client RPC| Daemon
    Daemon --> Inbox
    Daemon --> Graph
    Daemon --> Vector
    Daemon --> Models
    Daemon --> Cluster
```

_Figure 1: Standalone `memoryd` process and storage topology._

### 5.1 Process boundaries

`memoryd-collector` owns provider discovery, file cursors, hook handling, and
redaction before ingest. It does not project memory and does not talk to
Qdrant, Oxigraph, Ollama, or Chutoro.

`memoryd` owns durable evidence storage, jobs, projection, reconciliation,
security policy, Qdrant updates, Oxigraph graph writes, Ollama calls, Chutoro
theme operations, and recall.

`memoryd-mcp` exposes MCP tools to clients. It stays thin: it validates MCP
requests, applies read-only mode, requests a scoped daemon token, calls the
daemon, and formats tool responses.

### 5.2 Trust boundaries

Provider logs, hook payloads, transcript text, tool outputs, and manual imports
are untrusted input. The collector redacts and hashes before sending evidence
to the daemon. The daemon validates every support reference emitted by a model
or extractor before a semantic carrier becomes retrievable.

MCP clients are untrusted callers with scoped capabilities. Read-only mode
permits recall, explanation, session listing, and health. It disables curated
writes, imports, retractions, profile updates, and purge.

Qdrant, Ollama, and Oxigraph are local dependencies. Qdrant and Ollama may use
loopback network ports, but Oxigraph remains embedded and private. The daemon
is the only component allowed to write serving indexes, graph state, or
projection state.

### 5.3 Hexagonal dependency rule

`memoryd` follows hexagonal architecture. The domain and application layers own
the memory language and the port traits. Adapters implement those ports or
translate external calls into application commands. Dependencies point inward:
domain code must not import provider parser types, storage clients, model
software development kits (SDKs), clustering SDKs, UDS transport types, HTTP
types, or MCP runtime types.

The intended module boundary is:

| Layer       | Owns                                                                                                                                    | Must not own                                                                                   |
| ----------- | --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| Domain      | Workspaces, evidence, episodes, semantic carriers, facts, profiles, themes, retractions, recall context packs, and pure domain services | Filesystem watching, SQL, Qdrant, Oxigraph, Ollama, Chutoro, MCP, UDS, HTTP, or process config |
| Application | Use cases such as ingest, finalize, recall, read facts, store curated memory, retract, import, list sessions, health, and purge         | Concrete adapter construction or infrastructure-specific request and response types            |
| Adapters    | Provider parsing, persistence drivers, Qdrant, Ollama, Oxigraph, Chutoro, MCP, CLI, UDS, loopback HTTP, and filesystem integration      | Memory policy, promotion rules, contradiction handling, theme identity, or recall selection    |
| Composition | Binary entrypoints, configuration loading, dependency injection, feature wiring, and runtime lifecycle                                  | Domain decisions beyond selecting configured adapter implementations                           |

_Table 3: Hexagonal ownership boundaries._

Domain-owned driven ports include evidence repositories, graph repositories,
vector indexes, embedding providers, extraction providers, clustering
providers, audit sinks, clocks, identifier generation, and policy stores.
Driving adapters include CLI commands, MCP tools, provider collector loops,
collector hook commands, file-watch wake-ups, scheduled jobs, and debug
loopback HTTP handlers.

Adapters never call one another directly. For example, the Claude Code adapter
does not write Qdrant; it emits evidence through the ingest use case. The MCP
adapter does not read Oxigraph or Qdrant; it calls recall or read use cases.
The theme adapter around Chutoro never decides durable theme identity; the
domain `ThemeManager` does.

The dependency rule must be enforced by a Rust-native architecture lint once
the crate spine exists. The intended shape follows existing local Rust prior
art: Wildside uses a repo-local `tools/architecture-lint` crate, `syn` path
collection, and behaviour scenarios to reject wrong-layer imports; Corbusier
uses domain-oriented modules where bounded contexts expose domain, ports,
services, and adapters deliberately. Memoryd should combine both ideas:
Cargo-metadata graph checks for forbidden crate dependencies, and source-path
checks for direct infrastructure SDK imports or intra-crate module leaks during
incremental extraction.

### 5.4 Tenant isolation boundary

Tenant isolation is part of the domain and application contract, not an adapter
afterthought. Every tenant-owned use case receives a `RequestContext` carrying
`TenantId`, principal or user ID, session ID, correlation ID, and optional
causation ID. Driving adapters derive that context from authenticated daemon
capability tokens, Corbusier request context, MCP configuration, scheduled-job
state, or the configured local default before invoking application services.
Untrusted request fields may narrow filters, but they do not establish tenant
identity.

Workspace identity is scoped by tenant. The normative identity boundary is
`(tenant_id, workspace_id)`. A repository-derived workspace ID is not globally
authoritative without the tenant that owns it. Local single-user mode uses a
stable default local tenant, so local-first deployments exercise the same
tenant-aware code paths as Corbusier mode.

Driven adapters enforce tenant context using the strongest mechanism available
for their store:

- persistence adapters carry `tenant_id` in tenant-owned rows and composite
  keys;
- PostgreSQL adapters set a transaction-local tenant setting and enable RLS for
  tenant-owned tables;
- Qdrant adapters inject tenant and workspace filters on every upsert, search,
  delete, and repair operation;
- Oxigraph adapters address only tenant-and-workspace named graphs;
- Chutoro adapters load only tenant-and-workspace sessions and checkpoints.

The same boundary applies to observability and error handling. Audit records
carry tenant context. Metrics labels must avoid unbounded tenant names or raw
IDs unless explicitly approved. Cross-tenant read attempts should fail as
denied or not found without revealing whether the target object exists in a
different tenant.

## 6. Provider adapters

Provider adapters are driving adapters into the ingest use case. They sit above
native provider storage and below semantic projection. They must not leak
provider-specific event shapes into the core pipeline or write to driven
adapters such as persistence, Qdrant, Oxigraph, Ollama, or Chutoro directly.

```mermaid
flowchart TB
    Native[Provider-native records]
    Adapter[Provider adapter]
    Evidence[Canonical evidence events]
    Pipeline[memoryd projection pipeline]

    Native --> Adapter
    Adapter --> Evidence
    Evidence --> Pipeline
```

_Figure 2: Adapter boundary between provider records and memory projection._

| Adapter                      | Source records                                                                      | Boundary rule                                                                                                                     |
| ---------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Codex rollout adapter        | `CODEX_HOME` rollout JSONL and archived rollout JSONL                               | Tail configured roots, persist byte offsets, and key idempotency by path, line, and line hash.                                    |
| Claude Code adapter          | Hook JSON, transcript paths, compaction events, and transcript lines                | Treat command hooks as wake-up signals; tail transcripts asynchronously; never run long projection work inside hooks.             |
| Axinite conversation adapter | Conversations, messages, workspace documents, outbox events, and metadata           | Map Axinite conversations to source sessions and messages to evidence events without pretending they are Codex or Claude records. |
| Axinite projection sink      | Optional curated/profile/fact projection writes back to Axinite workspace documents | Require policy gating and provenance metadata to prevent self-reinforcing write-back loops.                                       |
| Manual import adapter        | Operator-supplied transcript or rollout paths                                       | Read only explicitly supplied files under configured roots and record import actor and request ID.                                |

_Table 4: Provider adapters and their source boundaries._

## 7. Canonical evidence model

The evidence inbox replaces Axinite’s single transactional outbox. It preserves
the same reliability intent: deduplicate inputs, retain source ordering, record
processing state, and expose replay and audit data.

The authoritative relational tables are:

```sql
CREATE TABLE source_session (
  id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  provider_session_id TEXT NOT NULL,
  workspace_id TEXT NOT NULL,
  cwd TEXT,
  repo_origin TEXT,
  git_branch TEXT,
  git_sha TEXT,
  model TEXT,
  cli_version TEXT,
  started_at TEXT,
  ended_at TEXT,
  transcript_uri TEXT,
  parent_session_id TEXT,
  source_kind TEXT NOT NULL
);

CREATE TABLE source_cursor (
  tenant_id TEXT NOT NULL,
  provider TEXT NOT NULL,
  path TEXT NOT NULL,
  device TEXT,
  inode TEXT,
  last_offset INTEGER NOT NULL,
  last_line_no INTEGER NOT NULL,
  last_seen_mtime TEXT,
  file_fingerprint TEXT,
  status TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  PRIMARY KEY (tenant_id, provider, path)
);

CREATE TABLE raw_event (
  event_id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  source_session_id TEXT NOT NULL,
  provider_event_id TEXT,
  ordinal INTEGER NOT NULL,
  observed_at TEXT NOT NULL,
  kind TEXT NOT NULL,
  actor TEXT NOT NULL,
  payload_redacted_json TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  source_offset_start INTEGER,
  source_offset_end INTEGER
);

CREATE TABLE raw_span (
  span_id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  event_id TEXT NOT NULL,
  role TEXT NOT NULL,
  text_redacted TEXT,
  content_hash TEXT NOT NULL,
  start_char INTEGER,
  end_char INTEGER,
  tool_name TEXT,
  tool_call_id TEXT
);

CREATE TABLE ingest_job (
  job_id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  kind TEXT NOT NULL,
  workspace_id TEXT NOT NULL,
  status TEXT NOT NULL,
  idempotency_key TEXT NOT NULL,
  retry_count INTEGER NOT NULL,
  last_error TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE (tenant_id, idempotency_key)
);

CREATE TABLE projection_state (
  projection_id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  raw_event_id TEXT NOT NULL,
  target TEXT NOT NULL,
  status TEXT NOT NULL,
  retry_count INTEGER NOT NULL,
  last_error TEXT,
  last_synced_at TEXT,
  deleted_soft INTEGER NOT NULL,
  deleted_hard INTEGER NOT NULL
);

CREATE TABLE audit_log (
  audit_id TEXT PRIMARY KEY,
  tenant_id TEXT NOT NULL,
  actor TEXT NOT NULL,
  method TEXT NOT NULL,
  workspace_id TEXT,
  target_kind TEXT NOT NULL,
  target_id TEXT NOT NULL,
  decision TEXT NOT NULL,
  reason TEXT,
  at TEXT NOT NULL
);
```

_Listing 1: Evidence inbox schema outline. The implementation may use SQLite,
libSQL, or PostgreSQL, but these entities are the logical contract._

Idempotency keys are provider-specific:

- Codex: `codex:{rollout_path}:{line_no}:{line_hash}`.
- Claude transcript: `claude:{transcript_path}:{line_no}:{line_hash}`.
- Claude hook-only event:
  `claude-hook:{session_id}:{hook_event}:{monotonic_or_hash}`.
- Axinite outbox: `axinite:{outbox_id}:{event_id}`.
- MCP/manual write: `mcp:{client_id}:{request_id}`.

The effective idempotency key is tenant-scoped. The same provider path, manual
request ID, or Axinite outbox identifier may exist for different tenants
without collision. Source-session and projection uniqueness rules follow the
same pattern: tenant identity is part of the logical key even when surrogate
IDs are globally unique.

## 8. Projection and storage model

The daemon projects evidence through the same hierarchy imported from Axinite:

```mermaid
flowchart LR
    Raw[Raw evidence]
    Draft[Draft episode]
    Final[Final episode]
    Summary[Episode summary]
    Extraction[Structured extraction]
    Validation[Support-reference validation]
    Semantic[Semantic carrier]
    Fact[Fact, concept, or profile candidate]
    Theme[Theme assignment]
    Recall[Recall indexes]

    Raw --> Draft --> Final --> Summary
    Final --> Extraction --> Validation --> Semantic
    Semantic --> Fact
    Semantic --> Theme
    Summary --> Recall
    Fact --> Recall
    Theme --> Recall
```

_Figure 3: Projection pipeline from evidence to recall-serving artefacts._

### 8.1 Source-of-truth boundaries

| Store               | Authority                                                                                                         | Not authoritative for                                                    |
| ------------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| Evidence inbox      | Source sessions, cursors, raw events, spans, jobs, projection sync state, and audit history                       | Graph relationships, vector ranking, or durable theme identity by itself |
| Oxigraph            | Facts, concepts, provenance edges, contradiction records, retractions, theme lineage, and temporal edges          | Raw transcript storage or vector ranking                                 |
| Qdrant              | Vector indexes and denormalized serving payloads for episodes, summaries, semantic carriers, themes, and profiles | Truth, provenance, or retraction authority                               |
| Chutoro checkpoints | Rebuildable clustering acceleration state                                                                         | Durable memory identity or theme membership authority                    |

_Table 5: Store authority boundaries._

Every store boundary is tenant-aware. Evidence rows carry `tenant_id`; Oxigraph
named graphs include tenant and workspace; Qdrant payloads and collection names
carry tenant scope; Chutoro checkpoint paths include a tenant-and-workspace
prefix. `memoryd` never treats workspace ID alone as an authorization boundary.

### 8.2 Projection classes and epistemic status

The standalone service keeps Axinite’s projection classes: `episode`, `summary`,
 `concept`, `fact`, and `profile`. Claim-bearing artefacts carry one of
`explicit`, `curated`, `deduced`, `hypothesized`, or `retracted`. Episodes and
summaries remain evidence or summaries of evidence rather than facts.

The extractor defaults model-derived claims to `hypothesized`. It uses
`explicit` for direct human statements and `curated` for operator-approved MCP
writes or trusted document material. Contradiction handling follows RFC 0002:
explicit evidence retracts weaker conflicting hypotheses automatically, while
conflicts between explicit or curated claims require operator resolution.

### 8.3 Extraction

`memoryd` supports two extractors behind a shared schema:

- `encoder_extractive`: cheaper path that emits extractive spans, support
  references, confidence, and temporal hints without a generative model.
- `llm_structured`: Ollama-backed structured extraction that emits canonical
  statements, entities, relations, confidence, temporal hints, and evidence
  spans.

Both paths must pass the same support-reference validator. Unsupported semantic
carriers remain diagnostics and never enter Qdrant, Oxigraph, or theme
management.

### 8.4 Qdrant layout

Qdrant uses collection-per-tenant-workspace by default in local mode to
simplify purge and reduce filter-isolation risk. A shared-collection strategy
is permitted only for hosted or high-cardinality deployments where the adapter
injects mandatory tenant and workspace filters on every upsert, search, delete,
repair, and scroll operation.

Default collections:

- `memoryd_{tenant}_{workspace}_episodes`
- `memoryd_{tenant}_{workspace}_summaries`
- `memoryd_{tenant}_{workspace}_semantic_carriers`
- `memoryd_{tenant}_{workspace}_themes`
- `memoryd_{tenant}_{workspace}_profiles`

Payloads must include projection class, epistemic status, tenant ID, workspace
ID, provider, source session, repository origin, observed and valid time,
confidence, retraction state, support count, evidence references, and optional
theme ID. Shared-collection deployments must also create a tenant keyword
payload index and treat absence of a tenant filter as a programming error, not
as an unfiltered query.

## 9. Theme management

The `ThemeManager` is scoped to `(tenant_id, workspace_id)`. It feeds accepted
semantic-carrier vectors for one tenant workspace into Chutoro, maps Chutoro
point indices back to semantic-carrier IDs, and maps cluster proposals into
durable theme records.

Chutoro remains a cluster proposal engine. `memoryd` owns:

- stable theme IDs;
- lineage for attach, split, merge, retraction, and rebuild;
- tenant workspace purge and security boundaries;
- curated-memory precedence;
- theme summaries and refresh jobs;
- retrieval-aware split and merge policy.

The default policy starts in shadow mode:

| Setting                   | Default | Purpose                                                    |
| ------------------------- | ------- | ---------------------------------------------------------- |
| `bootstrap_min_semantics` | `24`    | First point where workspace clustering becomes meaningful. |
| `max_semantics_per_theme` | `12`    | Upper bound before split evaluation.                       |
| `min_semantics_per_theme` | `3`     | Lower bound before merge or singleton handling.            |
| `theme_knn_k`             | `10`    | Neighbour graph width for routing and merge evaluation.    |
| `split_cooldown`          | `1h`    | Prevents repeated churn in the same dense region.          |
| `merge_cooldown`          | `1h`    | Prevents repeated merge oscillation.                       |

_Table 6: Initial theme-management policy._

## 10. Recall

Recall embeds the query once, gathers projection-aware candidates, selects a
compact high-level skeleton, and expands to episodes or raw-message blocks only
when the gain justifies the token cost.

Required recall profiles:

| Profile           | Behaviour                                                                                    |
| ----------------- | -------------------------------------------------------------------------------------------- |
| `flat_v1`         | Vector and graph retrieval without theme expansion.                                          |
| `cheap_v2`        | Hierarchical recall with deterministic proxy gating and no raw-message expansion by default. |
| `hierarchical_v2` | Default top-down theme, semantic-carrier, and episode retrieval.                             |
| `evidence_v2`     | Hierarchical recall with optional model-assisted expansion gating.                           |

_Table 7: Recall profiles._

The daemon returns context blocks, selected theme IDs, selected semantic IDs,
selected episode IDs, provenance references, fallback reasons, and a selection
trace. It never returns an unbounded transcript dump.

## 11. MCP front end

The MCP server follows Dear Diary’s practical structure: typed request objects,
tool routing, read-only gates, and a small operator-facing tool set. The
semantics differ: tools call `memoryd`, not Qdrant.

| Tool                    | Mode       | Purpose                                                                   |
| ----------------------- | ---------- | ------------------------------------------------------------------------- |
| `memory_store`          | Write      | Store curated or explicit memory through the daemon.                      |
| `memory_recall`         | Read       | Retrieve a bounded context pack.                                          |
| `memory_explain`        | Read       | Explain why a memory exists or why recall selected it.                    |
| `memory_retract`        | Write      | Retract a memory, fact, episode, theme, or source session.                |
| `memory_sessions`       | Read       | Browse ingested sessions by provider, workspace, model, branch, and time. |
| `memory_import_session` | Write      | Import an explicit transcript or rollout file.                            |
| `memory_profile`        | Read/write | Read or update stable profile material.                                   |
| `memory_health`         | Read       | Report daemon, dependency, collector, and theme state.                    |

_Table 8: MCP tool surface._

Read-only mode disables `memory_store`, `memory_retract`,
`memory_import_session`, and profile updates. The daemon also enforces
capability scopes, so read-only mode is a convenience gate rather than the only
authorization layer.

MCP tools do not accept arbitrary tenant claims as trusted input. In Corbusier
mode, the MCP or RPC front end maps authenticated Corbusier identity into a
daemon `RequestContext`. In local mode, the front end uses the configured
default tenant. Tool filters may include workspaces, providers, and time
ranges, but the daemon intersects them with the tenant context before use cases
run.

## 12. Internal RPC

Collector and MCP components call the daemon over Unix domain socket by
default. HTTP loopback is a debug or container mode and requires bearer or
capability tokens. Every RPC envelope carries or derives a `RequestContext`
before it reaches an application use case.

Internal methods:

- `IngestSourceEvent`
- `IngestTranscriptLine`
- `FinalizeSession`
- `Recall`
- `ReadFacts`
- `ReadEpisode`
- `ReadTheme`
- `StoreCuratedMemory`
- `Retract`
- `Reinforce`
- `ScheduleConsolidation`
- `ImportTranscript`
- `ListSessions`
- `Health`
- `PurgeWorkspace`
- `PurgeTenant`

Capability scopes:

- `memory.ingest`
- `memory.recall`
- `memory.read`
- `memory.write_curated`
- `memory.retract`
- `memory.reinforce`
- `memory.consolidate`
- `memory.admin`
- `memory.purge`

Collector tokens receive only `memory.ingest`. MCP clients receive read/write
scopes based on configuration. Tokens are bound to tenant ID and, where
configured, allowed workspace IDs. `PurgeWorkspace` requires both a
high-privilege token and an explicit confirmation string for the tenant
workspace. `PurgeTenant` is an administrative operation and remains disabled in
local mode unless explicitly configured for migration or offboarding tests.

## 13. Security and privacy

The collector redacts before storage and before embedding. The first release
must detect API keys, OAuth tokens, JSON Web Tokens (JWTs), private keys, SSH
material, `.env` content, passwords, cookies, cloud credentials, database URLs,
and high-entropy blobs. Per-workspace deny patterns block configured paths from
transcript capture and import.

Large tool outputs are not embedded directly. The daemon summarizes or extracts
bounded spans, records hashes and evidence references, and keeps raw references
available for explicit explanation or operator inspection.

Claude Code command hooks require special care because they run with the user’s
permissions.[^7] `memoryd` hook installation must call a narrow
`memoryd-collector hook --provider claude-code` command rather than arbitrary
shell pipelines.

Tenant context is security-sensitive. `memoryd` establishes tenant context from
authenticated capability tokens, Corbusier context, or a configured local
default before any tenant-owned command runs. Provider adapters may observe
tenant hints in source material, but those hints cannot override the
authenticated context. Cross-tenant reads, writes, repairs, and purges are
denied before storage adapters run.

PostgreSQL deployments must use non-owner application roles without
`BYPASSRLS`, set `memoryd.tenant_id` transaction-locally, and enable RLS for
tenant-owned tables. SQLite deployments still carry tenant IDs and enforce
tenant predicates in adapter contract tests, but they do not claim
database-enforced isolation.

## 14. Configuration

The configuration file is TOML. This shape is normative for field names even if
the implementation later splits sections by crate.

```toml
[daemon]
data_dir = "~/.local/share/memoryd"
uds_path = "~/.local/share/memoryd/memoryd.sock"
mode = "shadow" # disabled | observe | project_shadow | recall_shadow | active

[store]
driver = "sqlite"
sqlite_path = "~/.local/share/memoryd/memoryd.sqlite3"

[tenant]
mode = "local_single" # local_single | corbusier | hosted
local_tenant_slug = "local"
require_tenant_context = true
storage_strategy = "tenant_workspace" # tenant_workspace | payload_partitioned

[qdrant]
url = "http://127.0.0.1:6334"
api_key_env = "QDRANT_API_KEY"
collection_prefix = "memoryd"
collection_strategy = "per_tenant_workspace"

[ollama]
base_url = "http://127.0.0.1:11434"
embedding_model = "nomic-embed-text"
extraction_model = "qwen2.5:7b-instruct"
judge_model = "qwen2.5:7b-instruct"
strict_local = true

[graph]
driver = "oxigraph"
path = "~/.local/share/memoryd/graph"

[chutoro]
bootstrap_min_semantics = 24
max_semantics_per_theme = 12
min_semantics_per_theme = 3
theme_knn_k = 10
split_shadow = true
merge_shadow = true

[providers.codex]
enabled = true
codex_home_env = "CODEX_HOME"
default_codex_home = "~/.codex"
watch_globs = [
  "{codex_home}/sessions/**/*.jsonl",
  "{codex_home}/archived_sessions/**/*.jsonl",
]

[providers.claude_code]
enabled = true
hook_mode = "command"
watch_transcripts = true
install_scope = "user"

[privacy]
redact_before_store = true
redact_before_embedding = true
store_raw_text = "redacted" # none | redacted | encrypted
```

_Listing 2: Initial configuration shape._

## 15. Verification strategy

The design carries three correctness surfaces that deserve explicit
verification beyond ordinary unit and behavioural tests.

| Property                     | Verification method                                                                                      | Scope                                                                         |
| ---------------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Ingestion idempotency        | Property tests over provider paths, offsets, line hashes, hook retries, and import request IDs           | `source_cursor`, `raw_event`, `raw_span`, and `ingest_job` writes             |
| Projection provenance        | Property tests and fixture-based replay that reject semantic carriers with unresolved support references | Extractor outputs, validator, Qdrant writes, and Oxigraph writes              |
| Workspace purge completeness | End-to-end fixture that creates evidence, graph edges, Qdrant payloads, Chutoro checkpoints, then purges | Evidence inbox, Oxigraph namespaces, Qdrant collections, and checkpoint files |
| Tenant isolation             | Two-tenant fixtures and adapter contract tests for read, write, recall, repair, and purge paths          | Request context, persistence, Qdrant, Oxigraph, Chutoro, MCP, and RPC         |

_Table 9: Design-level verification targets._

The integration surface is combinatorial: provider (`codex`, `claude_code`,
`axinite`, `manual`) × daemon mode (`observe`, `project_shadow`,
`recall_shadow`, `active`) × recall profile (`flat_v1`, `cheap_v2`,
`hierarchical_v2`, `evidence_v2`) × read-only mode. The minimum coverage set
must include each provider in `observe`, one provider in full active recall,
read-only MCP denial of each write tool, and one purge path with projected
Qdrant, Oxigraph, and Chutoro artefacts.

Hexagonal conformance is another correctness surface. Domain tests must run
without infrastructure. Application tests use fake or mocked ports. Adapter
tests verify concrete implementations against port contracts. End-to-end tests
exercise the composition root. Static architecture checks should fail if domain
or application crates import adapter crates or infrastructure SDK types. The
check should live in the repository, run from `make lint` or `make all`, report
all violations found in one pass, and keep the composition-root allow-list
explicit. The first implementation should include negative fixtures proving
that a domain-to-adapter import, application-to-SDK import, inbound-to-outbound
adapter import, and outbound-to-inbound adapter import all fail before review.

## 16. Rollout sequence

1. **Tenant-aware evidence capture.** Implement request context, provider
   discovery, cursor persistence, redaction, raw event normalization, session
   listing, and health checks.
2. **Dear Diary parity through the daemon.** Add curated writes, flat recall,
   retraction, Qdrant indexing, and audit records without exposing collection
   names through MCP.
3. **Episodes and semantic projection.** Add episode finalization, summaries,
   embeddings, structured extraction, and support-reference validation.
4. **Facts, profiles, and graph promotion.** Add Oxigraph facts,
   contradictions, profile promotion, and curated write precedence.
5. **Themes and hierarchical recall.** Add Chutoro bootstrap, incremental
   attach, split/merge shadowing, and recall profiles beyond `flat_v1`.
6. **Axinite write-back.** Add optional projection sink support only after
   loop-prevention metadata and approval policy are implemented.

## 17. Open design decisions

- Choose the default evidence-store engine and migration format.
- Accept or revise the tenant-isolation strategy for local, Corbusier, and
  hosted-ready modes.
- Decide whether a graph-shaped relational fallback is allowed when Oxigraph is
  disabled.
- Define exact Axinite projection write-back policy.
- Define the redaction detector set and encrypted raw-text mode.
- Define the Chutoro checkpoint format and acceptable theme-ID churn during
  rebuild.

## References

[^1]: `../axinite/docs/rfcs/0007-secure-memory-sidecar-design.md`.
[^2]: `../axinite/docs/rfcs/0014-memory-projection-tiers-and-promotion-rules.md`.
[^3]: `../axinite/docs/rfcs/0015-hierarchical-memory-materialization-for-memoryd.md`.
[^4]: `../axinite/docs/rfcs/0016-theme-detection-and-sparsity-rebalancing-for-memoryd.md`.
[^5]: `../axinite/docs/rfcs/0017-hierarchical-recall-for-memoryd.md`.
[^6]: `../dear-diary/README.md` and
    `../dear-diary/crates/dear-diary-mcp/src/server.rs`.
[^7]: Claude Code hooks reference, accessed 2026-05-25:
    <https://code.claude.com/docs/en/hooks>.
[^8]: Model Context Protocol latest specification, accessed 2026-05-25:
    <https://modelcontextprotocol.io/specification/latest>.
[^9]: Corbusier tenant context references:
    `../corbusier/src/context/request_context.rs`,
    `../corbusier/src/context/ids.rs`,
    `../corbusier/src/message/adapters/postgres/tenant_tx.rs`,
    `../corbusier/docs/roadmap.md`, and
    `../corbusier/docs/users-guide.md`.
[^10]: PostgreSQL row security policies, accessed 2026-05-25:
    <https://www.postgresql.org/docs/current/ddl-rowsecurity.html>.
[^11]: AWS Database Blog, "Multi-tenant data isolation with PostgreSQL row
    level security", accessed 2026-05-25:
    <https://aws.amazon.com/blogs/database/multi-tenant-data-isolation-with-postgresql-row-level-security/>.
[^12]: Qdrant documentation, "Multitenancy", accessed 2026-05-25:
    <https://qdrant.tech/documentation/manage-data/multitenancy/>.
[^13]: OWASP Cheat Sheet Series, "Multi Tenant Security Cheat Sheet", accessed
    2026-05-25:
    <https://cheatsheetseries.owasp.org/cheatsheets/Multi_Tenant_Security_Cheat_Sheet.html>.
[^14]: W3C RDF 1.1 Concepts and Abstract Syntax, accessed 2026-05-25:
    <https://www.w3.org/TR/rdf11-concepts/>.
