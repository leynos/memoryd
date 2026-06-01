# Architectural decision record (ADR) 007: Standard conversation ingestion port

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

`memoryd` must ingest conversation history from different kinds of producer:

- Corbusier conversations, messages, tenant request context, and canonical
  message metadata;
- Axinite conversations, messages, metadata, optional outbox events, and
  workspace memory surfaces;
- Codex CLI rollout files under `CODEX_HOME`;
- Claude Code hook payloads and transcript files under configured Claude
  roots;
- manual imports.

These sources differ in transport, consistency, cursor shape, and semantic
richness. Corbusier and Axinite can expose structured repository or outbox
surfaces. Codex and Claude initially require a worker process to discover,
tail, parse, and normalize files. A single ad hoc "message import" function
would either become too narrow for Corbusier and Axinite, or too coupled to
Codex and Claude filesystem details.

The ingestion design therefore needs a standard port boundary that can support
push, pull, tail, replay, and import adapters while preserving the canonical
evidence model.

## Decision drivers

- Provider-specific parsing must remain in adapters.
- Domain and application code must own the canonical conversation vocabulary.
- Corbusier tenant request context must be preserved through ingestion.
- Axinite must be able to ingest conversations without pretending to be Codex
  or Claude.
- Codex and Claude filesystem scraping must live in a worker adapter, not in
  the daemon core.
- Cursor, idempotency, tombstone, and replay semantics must be consistent
  across sources.
- The design must preserve the hexagonal dependency rule from ADR 005.

## Options considered

| Option                                       | Consequence                                                                                                                           |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| One source-specific importer per tool        | Fast to implement, but creates divergent semantics for replay, idempotency, cursoring, redaction, and evidence references.            |
| Direct raw-event RPC only                    | Keeps the daemon simple, but forces every adapter to duplicate session, message, cursor, and lifecycle normalization.                 |
| One canonical ingestion method               | Keeps the API small, but hides different trust, sequencing, and idempotency invariants behind caller checks inside one function body. |
| Split commands with one canonical delta      | Keeps source parsing in adapters while separating authenticated push from worker-originated collected batches.                        |
| Separate source-specific canonical contracts | Models each producer precisely, but makes Corbusier, Axinite, Codex, Claude, and manual import drift into parallel memory pipelines.  |

_Table 1: Conversation-ingestion options._

## Decision outcome / proposed direction

Define one canonical delta and three standard ports.

`ConversationPushIngestPort` is the authenticated push command. Corbusier,
Axinite transactional outboxes, and MCP/manual import surfaces use it when the
caller already owns a trustworthy request context and submits one logical
canonical conversation delta. It requires `memory.ingest` capability and uses
caller-supplied request, correlation, causation, and workspace context.

`CollectedConversationIngestPort` is the worker-originated batch command. The
collector sidecar uses it after discovering, tailing, redacting, and
normalizing Codex, Claude, Axinite pull-mode, or future worker-read sources. It
requires a collector token with `memory.ingest`, accepts a collected batch of
canonical deltas and cursor updates, and records worker provenance and batch
sequencing.

`ConversationSourcePort` is the worker-facing source contract. Pull, tail, and
snapshot adapters implement it when `memoryd-collector` must discover or read
source material itself. Codex rollout tailing, Claude transcript tailing,
Axinite pull mode, and any future filesystem or database import source use this
port. Push-mode sources such as a Corbusier service hook or Axinite
transactional outbox may bypass `ConversationSourcePort` and call
`ConversationPushIngestPort` directly, but they still emit the same canonical
delta.

Both ingest commands create source sessions, raw events, raw spans, ingest
jobs, and audit records from conversation material. The canonical delta remains
shared; the trust, capability, sequencing, cursor, and idempotency paths differ
at the application-command boundary.

The canonical delta contains:

- source identity: provider, provider conversation ID, optional provider event
  ID, source URI, source kind, source version, and source fingerprint;
- request scope: tenant ID, principal or user ID, session ID, correlation ID,
  optional causation ID, workspace ID, and optional allowed-workspace policy;
- conversation metadata: title, lifecycle, parent conversation, channel,
  thread, model, agent, repository metadata, start time, update time, and end
  time;
- ordered events: ordinal, role, actor, event kind, timestamp, content parts,
  tool metadata, compaction markers, file-edit summaries, payload hash,
  redaction state, and evidence references;
- cursor metadata: source cursor, source offset, sequence, timestamp, provider
  checkpoint token, and replay mode;
- deletion and correction metadata: tombstones, source redactions, superseded
  events, and provider-side compaction.

Content parts are canonical and extensible. The first set is `text`,
`tool_call`, `tool_result`, `file_reference`, `file_edit`, `compaction_summary`,
`attachment_reference`, and `provider_metadata`. Unknown provider-specific
payload remains in redacted JSON metadata and does not become a new event kind
unless the domain needs different behaviour.

## Required adapters

The first implementation must provide these adapter shapes:

- `CorbusierConversationAdapter`: maps Corbusier `RequestContext`,
  conversations, immutable messages, sequence numbers, roles, content parts,
  and message metadata into canonical deltas.
- `AxiniteConversationAdapter`: maps Axinite conversations, channels, user and
  thread metadata, message role/content/timestamp, outbox IDs, and workspace
  context into canonical deltas.
- `CodexRolloutSourceAdapter`: implements `ConversationSourcePort` over
  `CODEX_HOME/sessions/**/*.jsonl` and archived rollout files, using path, line
  number, byte offset, and line hash as cursor and idempotency evidence.
- `ClaudeTranscriptSourceAdapter`: implements `ConversationSourcePort` over
  Claude hook wake-ups and transcript files, using hook payloads as discovery
  signals and transcript offsets as durable cursor material.
- `ManualConversationImportAdapter`: validates configured roots and emits one
  canonical delta per imported transcript or rollout.

## Consequences

- The daemon does not scrape `~/.codex` or `~/.claude` directly. The collector
  worker owns filesystem discovery and tailing adapters.
- Corbusier and Axinite can choose push mode, pull mode, or both without
  changing daemon ingestion semantics.
- The evidence inbox can enforce one evidence, audit, redaction, and
  tenant-isolation contract across all conversation sources while still
  applying different idempotency and cursor checks to push and worker-batch
  paths.
- Adapter tests must prove that semantically equivalent Corbusier, Axinite,
  Codex, and Claude fixtures produce equivalent canonical conversation deltas
  where their source data overlaps.
- The ports must be domain-owned. Adapter implementations may depend on
  Corbusier, Axinite, Codex rollout, Claude transcript, SQL, filesystem, or
  JSON parser types; the domain and application crates must not.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0001-standalone-evidence-inbox.md`.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `../corbusier/src/message/ports/conversation.rs`.
- `../corbusier/src/message/domain/conversation.rs`.
- `../corbusier/src/message/domain/message.rs`.
- `../corbusier/docs/users-guide.md`, "Conversation history".
- `../axinite/src/history/store/conversations.rs`.
