# Architectural decision record (ADR) 008: Source health and coverage foundation

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

`memoryd` ingests external evidence streams that can go stale, disappear,
change format, or become inaccessible without producing a direct failure in
recall. Codex and Claude sources can stop writing expected files. Axinite and
Corbusier adapters can fall behind their source systems. Manual imports can
leave important configured roots untouched. A memory system that only records
observed evidence cannot distinguish "nothing relevant happened" from "the
expected source was not observed".

The v1 design already reports daemon health, collector lag, dependency health,
and adapter status. That is necessary but not enough. Axinite's post-1.0
epistemic-health work will need richer coverage expectations and omission
alerts, but the core v1 daemon should establish the source-health substrate
before evidence capture ships.

## Decision drivers

- Source freshness and parse failures are broadly useful before semantic
  projection exists.
- Health data must be tenant-scoped and workspace-aware.
- Provider-specific discovery remains in adapters.
- The daemon must own the normalized source-health vocabulary.
- Read surfaces should expose missing or stale sources without exposing raw
  transcript content.
- The design must preserve the hexagonal dependency rule from ADR 005 and the
  tenant boundary from ADR 006.

## Options considered

| Option                                   | Consequence                                                                                                                 |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Treat source health as log output only   | Simple, but leaves recall, explanation, and post-incident review unable to reason about missing evidence.                   |
| Keep provider-specific health structures | Preserves adapter detail, but forces every caller to understand Codex, Claude, Axinite, and Corbusier status differently.   |
| Add normalized source-health records     | Adds a small v1 persistence surface and gives all providers one way to report freshness, parse, and accessibility problems. |

_Table 1: Source-health options._

## Decision outcome / proposed direction

Adopt a normalized, pre-1.0 source-health foundation.

The domain model will include:

- `SourceId`;
- `SourceKind`;
- `SourceRegistryEntry`;
- `SourceHealthSnapshot`;
- `SourceHealthStatus`;
- `SourceObservation`;
- `SourceHealthReason`.

Each source-health record is scoped by tenant and optionally by workspace. A
source registry entry describes what a configured source is and how fresh it is
expected to be. A source-health snapshot records the latest observed state:

- last successful discovery;
- last successful read;
- last successful parse;
- last cursor update;
- observed lag;
- expected freshness;
- last error code;
- redaction or deny-pattern skips;
- status: `healthy`, `stale`, `blocked`, `misconfigured`, `degraded`, or
  `unknown`.

Provider adapters discover source-specific facts, but they report normalized
health through domain-owned ports. `memory_health`, session browsing, and
`memory_explain` may surface source-health status when it affects recall or
evidence interpretation.

This ADR does not adopt post-1.0 coverage expectations, missing claim-class
alerts, or omission scoring. It establishes the source-health substrate those
features will use later.

## Consequences

- Evidence capture can report "source stale" separately from "no matching
  evidence".
- `memoryd-collector health` and `memory_health` can expose provider state
  without raw transcript reads.
- Recall and explanation can warn when a relevant source is stale or blocked.
- Every source-health query must enforce the caller's tenant context.
- Adapters must implement source-health contract tests alongside ingestion
  fixtures.
- Post-1.0 coverage and omission-alert work can build on persisted source
  status rather than inventing a parallel observability store.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0001-standalone-evidence-inbox.md`.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`.
