# Architectural decision record (ADR) 005: Hexagonal architecture boundary

## Status

Proposed.

## Date

2026-05-25.

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

| Option                              | Consequence                                                                                               |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Infrastructure-first layering       | Speeds early adapter work but couples memory rules to storage and transport choices.                      |
| Hexagonal architecture              | Keeps domain and application services independent from provider, storage, model, clustering, and MCP I/O. |
| Framework-owned application service | Simplifies one entrypoint but weakens support for multiple binaries, MCP, scheduled jobs, and adapters.   |

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

## Consequences

- The roadmap must define domain types and ports before implementing provider,
  storage, model, clustering, transport, and MCP adapters.
- Domain tests must run without external infrastructure.
- Application-service tests use fakes or mocks for ports.
- Adapter tests verify port conformance against real or fixture-backed
  infrastructure.
- End-to-end tests verify wiring across the composition root.
- Architecture fitness checks should prevent domain and application modules
  from importing adapter crates or infrastructure SDK types.

## References

- `docs/memoryd-design.md`.
- `docs/roadmap.md`.
- `docs/rfcs/0001-standalone-evidence-inbox.md`.
- `docs/rfcs/0002-projection-tiers-and-promotion-rules.md`.
- `docs/rfcs/0003-hierarchical-materialization.md`.
