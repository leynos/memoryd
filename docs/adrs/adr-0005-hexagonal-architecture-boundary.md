# Architectural decision record (ADR) 005: Hexagonal architecture boundary

## Status

Accepted on 2026-11-03. Memoryd adopts hexagonal architecture with an inward
dependency rule, domain-owned port contracts, application-owned use cases, and
composition-root wiring. Adapters never call one another, and the port-budget
discipline governs which boundaries become public crate application programming
interfaces (APIs). The runtime crate spine and its architecture lint are built
by roadmap items 1.2.1, 1.2.2, and 1.2.7; this ADR records the contract, not an
implementation.

## Date

2026-11-03.

## Context and problem statement

`memoryd` coordinates several infrastructure-heavy concerns: provider logs,
local storage, Qdrant, Ollama, Oxigraph, Chutoro, Unix domain socket (UDS)
remote procedure call (RPC), command-line entrypoints, and Model Context
Protocol (MCP) tools. The domain rules are more stable than those dependencies:
evidence must stay distinct from memory, facts need provenance, themes are
navigation, and serving indexes are rebuildable.

Without an explicit boundary, storage clients, MCP request types, provider log
formats, or model responses could leak into domain services and make the memory
rules hard to test without running infrastructure.

## Decision drivers

- Domain logic must remain testable without databases, vector stores, model
  servers, graph engines, filesystems, sockets, or MCP runtimes.
- Provider adapters must not shape the canonical evidence model around Codex,
  Claude Code, or Axinite implementation details.
- Storage, model, clustering, and transport dependencies are expected to
  evolve independently.
- Adapters must not call each other directly; application services coordinate
  use cases through ports.
- Port signatures must speak `memoryd` domain language and return domain
  types, not Qdrant, Oxigraph, Ollama, Chutoro, SQL, JSON-RPC, or MCP runtime
  types.

## Options considered

| Option                              | Consequence                                                                                             |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Infrastructure-first layering       | Speeds early adapter development but couples memory rules to storage and transport choices.             |
| Hexagonal architecture              | Keeps domain and application services independent of provider, storage, model, clustering, and MCP I/O. |
| Framework-owned application service | Simplifies one entrypoint but weakens support for multiple binaries, MCP, scheduled jobs, and adapters. |

_Table 1: Architecture boundary options._

## Decision outcome / proposed direction

Adopt hexagonal architecture for the implementation. The dependency rule is:
domain and application code define and depend on ports; adapters implement
those ports; binaries and configuration compose concrete adapters at the edge.

The domain owns:

- value objects and aggregates for workspaces, evidence, episodes, semantic
  carriers, facts, profiles, themes, retractions, and recall context packs;
- domain services for redaction policy, episode boundary decisions,
  projection validation, promotion, contradiction handling, purge planning,
  theme policy, and recall selection;
- driven port traits for evidence repositories, graph repositories, vector
  indexes, embedding and extraction providers, clustering providers, clocks,
  identifier generation, and audit sinks.

The application layer owns use cases such as `IngestSourceEvent`, `Recall`,
`StoreCuratedMemory`, `Retract`, `ListSessions`, and `PurgeWorkspace`. Use
cases depend only on domain types and driven ports.

Adapters own provider parsing, persistence drivers, Qdrant, Ollama, Oxigraph,
Chutoro, MCP, CLI, UDS, loopback HTTP, and filesystem watching. Adapters map
infrastructure data into domain commands and domain results back into
transport-specific responses.

### Collector orchestration

Collector orchestration is an application service, not an adapter. It is wired
at the collector composition root, which injects the configured
`ConversationSourcePort` implementations and the collected-ingestion command.
The orchestrator invokes those injected ports; it never reaches a concrete
source adapter or a concrete daemon client directly. This keeps discovery,
tailing, redaction, and batching policy in one place while leaving provider
parsing in the adapters and acceptance policy in the daemon's application layer.

### Port ledger

The port-budget rule is applied to the conversation-ingestion boundaries as
follows. A boundary that does not yet meet the rule is an internal workspace
implementation contract, not a supported external crate API. Internal
visibility does not permit a second delta definition or a source-specific
ingestion contract in another crate.

| Boundary                          | Kind                | Driving adapters today                                     | Visibility | Graduation trigger                                                                  |
| --------------------------------- | ------------------- | ---------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------- |
| `ConversationPushIngestPort`      | Application command | Corbusier, Axinite transactional outbox, manual/MCP import | Public     | Already serves three distinct adapters.                                             |
| `ConversationSourcePort`          | Driven source read  | Codex rollout, Claude transcript, Axinite pull mode        | Public     | Already serves three distinct adapters.                                             |
| `CollectedConversationIngestPort` | Application command | `memoryd-collector` only                                   | Internal   | A second driving adapter, a second use case, or a roadmap-scheduled second adapter. |

_Table 2: Conversation-ingestion port ledger._

The collected-ingestion boundary is required even while it is internal: it
separates collector trust, batch sequencing, and cursor preconditions from the
authenticated push path. It must not be collapsed into the push command, and no
second adapter is invented merely to satisfy the budget.

## Consequences

- The roadmap must define domain types and ports before implementing provider,
  storage, model, clustering, transport, and MCP adapters.
- Domain tests must run without external infrastructure.
- Application-service tests use fakes or mocks for ports.
- Adapter tests verify port conformance against real or fixture-backed
  infrastructure.
- End-to-end tests verify wiring across the composition root.
- Architecture fitness checks should prevent domain and application crates from
  importing adapter crates or infrastructure software development kit (SDK)
  types.
- The first architecture fitness check should be a repo-local Rust tool, not a
  general-purpose prose convention. It should be modelled on existing local
  Rust prior art: Wildside's `tools/architecture-lint` source-path and `syn`
  -based guardrail, plus Corbusier's domain-oriented module convention where
  each bounded context exposes domain, ports, services, and adapters
  deliberately.
- In a multi-crate Memoryd workspace, the lint should combine Cargo metadata
  dependency-graph checks with source-path checks. Cargo metadata catches
  forbidden crate edges; source-path checks catch intra-crate module leaks and
  direct infrastructure SDK imports while crates are still being extracted.
- The port ledger is a living record. A boundary graduates or retreats by
  evidence — a second adapter, a second use case, or a scheduled adapter — not
  by the convenience of exposing it. Keeping the collected-ingestion boundary
  internal is a visibility decision, not permission to duplicate its contract.
- Collector orchestration sits in the application layer so that the collector
  binary composes adapters without becoming one. A future second consumer of
  the same orchestration is the evidence that would justify extracting it.

## References

- `docs/memoryd-design.md`.
- `docs/roadmap.md`.
- `docs/rfcs/rfc-0001-standalone-evidence-inbox.md`.
- `docs/rfcs/rfc-0002-projection-tiers-and-promotion-rules.md`.
- `docs/rfcs/rfc-0003-hierarchical-materialization.md`.
