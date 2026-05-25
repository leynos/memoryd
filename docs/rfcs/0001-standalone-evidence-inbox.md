# RFC 0001: Standalone evidence inbox

## Preamble

- **RFC number:** 0001
- **Status:** Proposed
- **Created:** 2026-05-25
- **Adapted from:** `../axinite/docs/rfcs/0007-secure-memory-sidecar-design.md`

## Summary

This RFC adapts Axinite’s secure memory sidecar design to a standalone daemon.
Axinite’s transactional outbox remains a supported provider mode, but standalone
 `memoryd` introduces a local evidence inbox so Codex CLI, Claude Code,
Axinite, and manual imports can all feed the same projection pipeline.

## Problem

Axinite can publish memory events from inside its own database transactions.
Codex CLI and Claude Code are external producers. They expose rollout files,
transcripts, hooks, and metadata, not a shared PostgreSQL transaction. A
standalone service therefore needs durable cursors, idempotency keys, raw event
storage, redaction, audit records, and retryable projection state that do not
depend on one host application.

## Current state

`docs/terms-of-reference.md` identifies evidence capture as the first delivery
slice. Axinite RFC 0007 supplies the original local sidecar constraints: Qdrant
for vectors, Ollama for embeddings and extraction, Oxigraph for graph facts and
relations, UDS RPC, capability scopes, and a security-first posture.

## Goals and non-goals

- Goals:
  - Define the standalone evidence inbox entities.
  - Define provider-adapter responsibilities.
  - Define a standard conversation ingestion port for structured application
    sources and filesystem-backed worker sources.
  - Preserve Axinite compatibility through a provider adapter.
  - Keep ingestion idempotent across restarts, file rotations, hook retries,
    and manual imports.
  - Keep collector privileges narrower than daemon privileges.
- Non-goals:
  - Define projection classes. RFC 0002 owns that.
  - Define episode materialization. RFC 0003 owns that.
  - Define final MCP tool behaviour. The design document owns the top-level
    tool contract.

## Proposed design

### Provider adapters

Provider adapters convert native records into canonical evidence:

- `CorbusierConversationAdapter` maps Corbusier request context,
  conversations, immutable messages, sequence numbers, roles, content parts,
  and message metadata into canonical conversation deltas.
- `CodexRolloutAdapter` tails configured rollout JSONL roots.
- `ClaudeCodeAdapter` handles hook wake-ups and tails transcripts.
- `AxiniteConversationSource` reads Axinite conversations and messages.
- `AxiniteWorkspaceSource` reads Axinite workspace documents and revisions.
- `AxiniteProjectionSink` optionally writes curated projections back to
  Axinite with provenance metadata.
- `ManualImportAdapter` imports explicitly requested files under configured
  roots.

### Standard conversation ingestion ports

Conversation ingestion has two standard ports.

`ConversationIngestPort` is the daemon-facing driving port. It accepts a
tenant-scoped request context and a canonical conversation delta. Corbusier,
Axinite, the collector worker, and manual import tools all use this port when
they want conversation material to become evidence inbox records.

`ConversationSourcePort` is the source-reader contract implemented by adapters
that the worker must discover, tail, page, or replay. Codex rollout scraping,
Claude transcript scraping, Axinite pull mode, and future filesystem or
database imports use this port. Push-mode sources, including Corbusier service
hooks or Axinite transactional outbox events, may bypass the source-reader port
and call `ConversationIngestPort` directly, but they must still emit the same
canonical conversation delta.

The canonical delta contains source identity, tenant request scope,
conversation metadata, ordered events, content parts, cursor metadata,
tombstones, correction metadata, and evidence references. Source-specific
payload remains in redacted metadata unless the domain needs a new canonical
event kind.

The daemon does not scrape `~/.codex` or `~/.claude` directly. The
`memoryd-collector` worker owns filesystem source adapters, persistent source
cursors, and replay scheduling, then calls the daemon ingestion port with
canonical deltas.

### Evidence inbox tables

The logical entities are:

- `source_session`
- `source_cursor`
- `raw_event`
- `raw_span`
- `ingest_job`
- `projection_state`
- `audit_log`

The exact SQL dialect depends on the selected store, but the logical contract
is normative. The primary design document contains the schema outline.

### Event kinds

Canonical event kinds are:

- `session_meta`
- `user_message`
- `assistant_message`
- `tool_call`
- `tool_result`
- `compaction`
- `file_edit`
- `approval`
- `error`
- `document_revision`
- `manual_memory`

Provider-specific event names stay in payload metadata and do not become core
pipeline branches unless the canonical enum lacks the needed behaviour.

### Idempotency

Every ingest job has an idempotency key:

- Codex: `codex:{rollout_path}:{line_no}:{line_hash}`.
- Claude transcript: `claude:{transcript_path}:{line_no}:{line_hash}`.
- Claude hook-only event:
  `claude-hook:{session_id}:{hook_event}:{monotonic_or_hash}`.
- Axinite outbox: `axinite:{outbox_id}:{event_id}`.
- Corbusier conversation:
  `corbusier:{conversation_id}:{sequence_no}:{message_id}`.
- MCP/manual write: `mcp:{client_id}:{request_id}`.

## Compatibility and migration

Axinite remains compatible in two modes:

- push mode, where Axinite writes its own outbox and notifies `memoryd`;
- pull mode, where `memoryd` reads Axinite conversations and workspace
  documents through adapters.

Push mode gives stronger consistency. Pull mode is easier to retrofit but must
track watermarks and tombstones carefully.

## Open questions

- Which store is the default evidence inbox: SQLite, libSQL, PostgreSQL, or a
  supported set?
- Which file-locking and cursor rules are needed for Windows Subsystem for
  Linux and native Linux?
- Which exact Codex rollout item variants should become first-class parser
  fixtures?
- Which Corbusier message metadata fields should become first-class canonical
  content parts rather than redacted provider metadata?

## Recommendation

Adopt the evidence inbox as the standalone replacement for Axinite’s
single-host transactional outbox. Keep Axinite’s outbox as one provider source,
not as the only ingestion model.
