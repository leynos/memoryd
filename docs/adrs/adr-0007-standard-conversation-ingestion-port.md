# Architectural decision record (ADR) 007: Standard conversation ingestion port

## Status

Accepted on 2026-11-03. Memoryd defines one canonical conversation delta and
three standard ports: an authenticated push command, a collector-originated
batch command, and a source-reader contract. Push and collected paths keep
separate rules for trust, capability, sequencing, cursors, and idempotency
while producing the same evidence records. This ADR records the shared
semantics, acceptance and retry rules, cursor progress, correction handling,
and reporting limits. Runtime implementation belongs to roadmap items 1.2.1,
1.2.6, 2.1.3, 2.2.1, 2.2.2, and 2.2.3.

## Date

2026-11-03.

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

Missing optional source data stays absent. An adapter must not fabricate a
timestamp, actor, workspace, or authority value to fill a canonical field.
Producer-specific minimum fields are listed in the design document's
producer-minimum table; fields outside that minimum are optional or best-effort.

### Producer routing

Table 2 assigns each producer family to one path. The routing follows the
producer's trust and sequencing evidence, not its transport convenience.

| Producer          | Ingestion path                                         | Request or operation identity          | Cursor ownership       |
| ----------------- | ------------------------------------------------------ | -------------------------------------- | ---------------------- |
| Corbusier push    | `ConversationPushIngestPort`                           | Caller-supplied request ID             | Provenance only        |
| Axinite outbox    | `ConversationPushIngestPort`                           | Outbox event identity                  | Provenance only        |
| Axinite pull      | `ConversationSourcePort` then collected batch          | Collector operation and batch identity | Collector resume state |
| Codex rollout     | `ConversationSourcePort` then collected batch          | Collector operation and batch identity | Collector resume state |
| Claude transcript | `ConversationSourcePort` then collected batch          | Collector operation and batch identity | Collector resume state |
| Manual import     | `ConversationPushIngestPort` with an import request ID | Explicit import request ID             | Provenance only        |

_Table 2: Producer routing for the canonical delta._

A push caller uses one logical delta per command and requires `memory.ingest`
through a push token. A push token is not a collector token, and the two are
never interchangeable. A push delta's cursor metadata is provenance: it cannot
advance collector resume state and is not compared with a source cursor. Manual
import validates configured roots and keeps filesystem reading at the edge; it
does not gain collector trust by reading files.

### Push acceptance

A push command accepts one canonical delta. Its request identity is separate
from any provider event identity in the delta. Push requests are deduplicated
by tenant, authenticated caller, and request ID. Events are independently
deduplicated by their tenant-scoped provider keys.

Reusing a request ID with changed logical content is a conflict, even when
every event in the changed delta has already been seen. An identical retry of
the same request identity and payload is a no-op on acceptance effects.

### Collected batch acceptance

Collected ingestion additionally checks collector identity, batch identity,
sequence number, source binding, and cursor preconditions. Acceptance is atomic
within one tenant and one authorized workspace: evidence, spans, jobs, accepted
batch identity, and cursor transitions commit together.

An invalid member, or a storage failure, leaves those acceptance effects
unchanged. A separate content-free rejection audit may still be recorded; it
must not retain prohibited payload text. The initial batch scope is one
workspace, and the collector partitions work before submission because
cross-workspace atomicity has no demonstrated first-value consumer.

Acknowledgement follows durable commit. A lost acknowledgement permits the
collector to replay the identical batch with no duplicate effects.

### Rejection stages

Per-record safety rejection and collected-batch atomicity apply at different
stages and do not conflict. The collector applies ADR 015's per-record
redaction, detector, and processing-ceiling checks before submission. A record
that fails those checks is not submitted, its source cursor does not advance
past it, a sanitized source-health error identifies the blocked record, and
records already committed from earlier batches in the same pass stand. Later
records wait until the operator repairs the source or records an audited
disposition. Valid neighbours are therefore never discarded: they remain
pending or committed rather than being silently dropped.

Only an admissible set of records becomes a collected batch. The daemon then
accepts that batch atomically, so an invalid member or storage failure at the
daemon boundary commits no member and leaves cursor transitions unchanged. A
daemon rejection is not a per-record partial commit; the collector resolves the
cause and resubmits the whole batch. ADR 015's "same submission" rule is read
at the collector stage, where the offending record is separated from its valid
neighbours, while this ADR's atomic rule governs the daemon commit unit.

### Operation identity and retry

Operation identity is namespaced by tenant, operation kind, and authenticated
producer or collector-stream identity. A push request's identity is bound to
the canonical logical payload; its cursor metadata is provenance only and
contributes no expected cursor. A collected batch's identity is bound to the
canonical logical payload and to the entire expected and next cursor vector.
The accepted batch identity is the durable record of the committed operation
identity, including its sequence and cursor vector.

The same identity with a changed payload, or with a changed cursor vector where
one applies, is a conflict, never an identical retry. This prevents a retry
from silently replacing accepted evidence.

Sequence numbers belong to a stable `(tenant, workspace, collector-stream)`
namespace. They survive worker restart and advance only for accepted fresh
batches. A new worker process cannot reset that namespace; source
reconciliation or a genuinely new stream requires explicit, audited
registration.

### Cursor semantics

Cursors are bound to tenant, workspace, source identity, and source generation.
Provider checkpoint tokens are opaque. An `after` cursor is exclusive of the
last committed complete item. Only the source adapter interprets its own
ordering; no component compares opaque tokens lexically or across sources.

A fresh update requires an expected previous cursor or an equivalent version
check, so competing stale updates are rejected. An identical committed retry is
recognized before that stale-update rejection, so a harmless replay is not
reported as a conflict. Replaying older accepted evidence must not regress the
cursor.

A partial trailing record is not consumed. A malformed complete record blocks
advancement until an explicit, audited disposition exists. A mandatory safety
exclusion may advance only with a durable content-free disposition committed
with the cursor, so no prohibited text is retained.

Source truncation, rotation, or an identity-fingerprint mismatch requires
reconciliation of the source generation; an offset is never reset silently.
Reconciliation compares the structural signal with the cursor's recorded
generation. It either confirms the same generation and resumes from a verified
offset, or registers a new generation and re-reads under the new identity; the
outcome is audited and surfaces through source health. Ordinary appends, or a
changing content digest alone, do not create a new source generation. Stable
source-incarnation identity stays distinct from observed content fingerprints.
An empty read does not advance a cursor unless it carries an explicit valid
source checkpoint or disposition.

### Event identity and correction

Within a tenant, the provider-specific event keys from the design document's
idempotency-key list are retained. Each key is bound to its full source and
workspace identity and to a detector-independent logical content identity. An
identical retry is a no-op; the same key with conflicting identity or content
is rejected.

A correction or tombstone is an explicit new versioned operation that
references the old event. It does not replace the old event during a retry.
Corrections preserve provenance and ADR 015's purge recapture fence. The exact
hash encoding remains roadmap 2.2.3's decision, and redacted bytes must not
define ingestion identity.

### Worked retry and cursor examples

Table 3 works the acceptance, retry, and cursor rules through accept and reject
cases. These are documentation examples, not runnable fixtures; the executable
conformance corpus belongs to roadmap 1.2.6 and 2.2.1–2.2.2.

| Case                          | Prior state                      | Request                                        | Outcome                                                          |
| ----------------------------- | -------------------------------- | ---------------------------------------------- | ---------------------------------------------------------------- |
| Identical push retry          | Request R committed              | Same R, same payload                           | Accepted as identical retry; counts already-seen; no new effects |
| Changed payload under same ID | Request R committed              | Same R, changed event content                  | Conflict; no acceptance effects                                  |
| Lost acknowledgement          | Batch B committed, cursor C      | Identical B replayed                           | No duplicate evidence or jobs; cursor stays C                    |
| Competing stale batch         | Cursor C0 committed              | Two batches both expecting C0                  | First commits; second is rejected as stale                       |
| Partial trailing record       | Cursor before a partial record   | Read after the last complete record            | Cursor remains before the partial record                         |
| Malformed complete record     | Cursor before a malformed record | Read continues                                 | Advancement blocked until an audited disposition exists          |
| Mandatory safety exclusion    | Cursor before an excluded record | Read continues                                 | Advances only with a durable content-free disposition            |
| Source rotation               | Cursor bound to generation G1    | Source now reports generation G2               | Reconciliation required; no silent offset reset                  |
| Ordinary append               | Cursor bound to generation G1    | File grows; content digest changes             | Same generation; cursor advances normally                        |
| Empty read                    | Cursor C                         | Read returns no items and no checkpoint        | Cursor stays C                                                   |
| Empty read with checkpoint    | Cursor C                         | Read returns no items and a valid checkpoint D | Cursor advances to D; no evidence is invented                    |
| Correction                    | Event E accepted                 | New versioned operation referencing E          | Correction accepted; E is not overwritten                        |
| Push cursor injection         | Collector cursor C               | Push delta carries a cursor field              | Provenance only; collector cursor stays C                        |

_Table 3: Worked retry and cursor examples._

### Report semantics and limits

An ingest call reports an accepted result, an identical retry, or a typed
failure. Report counts distinguish newly accepted events from already-seen
events. Failures distinguish at least malformed input, scope or capability
denial, a conflicting duplicate, a stale cursor or sequence, and a retryable
storage failure. Error text must not echo raw source payloads.

This ADR does not settle a serialization format or an exact Rust enum layout.
It requires configured limits on encoded input bytes, events per delta, deltas
per batch, and source page size. Oversized input is rejected before acceptance
effects or cursor advancement. The expected and next cursor vectors are bounded
by the configured source registry, and the report is a fixed set of count
fields, so neither grows with batch size. Numeric defaults belong to
configuration and runtime delivery; tests must cover below, at, and above each
configured bound, including byte-heavy events with low event counts.

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
- A retry and a correction are different operations. Retrying is idempotent;
  correcting is a new versioned operation. Collapsing the two would let a
  replay silently change accepted evidence.
- Cursor progress is evidence-backed. A cursor may pass an item only when that
  item has accepted evidence or a durable disposition, so an empty read does
  not advance without an explicit valid checkpoint, and a partial record is not
  consumed.
- One-workspace batch atomicity is the first-value contract. Cross-workspace
  atomicity is deliberately deferred until a consumer demonstrates the need;
  the collector partitions work instead.
- Providers whose paths may be reused must expose a stable source-incarnation
  identity. A content digest is not a substitute because appends change it
  without creating a new source. This ADR records contracts; it does not
  enforce them, and the runtime remains incomplete until the named roadmap
  items land.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/rfc-0001-standalone-evidence-inbox.md`.
- `docs/adrs/adr-0005-hexagonal-architecture-boundary.md`.
- `docs/adrs/adr-0006-tenant-isolation-and-corbusier-context.md`.
- `../corbusier/src/message/ports/conversation.rs`.
- `../corbusier/src/message/domain/conversation.rs`.
- `../corbusier/src/message/domain/message.rs`.
- `../corbusier/docs/users-guide.md`, "Conversation history".
- `../axinite/src/history/store/conversations.rs`.
