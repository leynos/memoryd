# Architectural decision record (ADR) 003: Memoryd owns theme management

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

Chutoro provides density-based clustering over arbitrary data sources and is
the planned substrate for semantic-carrier grouping.[^1] Axinite ADR 003
decides that Chutoro labels are not durable theme IDs and that memory-specific
theme policy belongs in `memoryd`.[^2] Standalone `memoryd` keeps that boundary
because provider diversity makes provenance, workspace isolation, retractions,
and purge semantics more important, not less.

## Decision drivers

- Theme IDs must remain stable enough for browseability and recall traces.
- Theme membership must remain purgeable and auditable per workspace.
- Chutoro should stay reusable as a clustering engine.
- Memory-specific rules, such as curated-memory precedence and retraction
  propagation, do not belong in `chutoro-core`.
- Lost or stale Chutoro checkpoints must be rebuildable from accepted semantic
  carriers.

## Options considered

| Option                                           | Trade-off                                                                                       |
| ------------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| Chutoro owns themes                              | Centralizes clustering and themes, but pushes memory semantics into a generic clustering crate. |
| Chutoro proposes clusters; `memoryd` owns themes | Preserves a clean boundary and makes rebuilds possible from semantic carriers.                  |
| `memoryd` implements clustering itself           | Avoids a dependency but duplicates Chutoro’s FISHDBC and HNSW work.                             |

_Table 1: Theme ownership options._

## Decision outcome / proposed direction

`memoryd` owns the `ThemeManager`, stable theme IDs, lineage, attach policy,
split policy, merge policy, and retrieval-aware balancing. Chutoro supplies
bootstrap clustering, local split proposals, sessions, snapshots, and
diagnostics.

Authoritative membership and lineage live in `memoryd` stores. Chutoro
snapshots are acceleration artefacts.

## Consequences

- `memoryd` must maintain both durable theme state and optional clustering
  checkpoints.
- Full rebuilds may change cluster proposals, so theme-ID churn needs an
  explicit policy.
- Theme summaries remain navigation aids and must never become evidence.

## References

[^1]: `../chutoro/README.md`.
[^2]: `../axinite/docs/adr-003-theme-management-belongs-in-memoryd.md`.
