# Architectural decision record (ADR) 001: Qdrant is a serving index

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

Dear Diary exposes direct Qdrant-backed MCP tools for storing, finding, and
deprecating semantic memories.[^1] That model is intentionally small and useful
for simple persistent memory. Standalone `memoryd` has a different requirement:
it must preserve evidence, provenance, epistemic status, contradictions,
retractions, promotion rules, and workspace purge semantics from the Axinite
memory RFCs.[^2][^3]

The design question is whether Qdrant should become the memory source of truth
or a serving index rebuilt from stronger stores.

## Decision drivers

- Every claim-bearing artefact must retain validated evidence references.
- Retractions and contradictions must remain auditable.
- Workspace purge must remove all memory surfaces consistently.
- Recall needs vector search, but vector payloads cannot safely model graph
  provenance by themselves.
- MCP tools must not expose internal collection names as the public contract.

## Options considered

| Option                      | Consequence                                                                                                                   |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Qdrant as source of truth   | Simplifies early storage but forces provenance, contradiction, and promotion semantics into payload conventions.              |
| Qdrant as serving index     | Preserves vector recall performance while keeping truth, provenance, and retraction state in the evidence store and Oxigraph. |
| No Qdrant in early versions | Reduces dependencies but loses the required vector-serving layer from the Axinite design.                                     |

_Table 1: Qdrant ownership options._

## Decision outcome / proposed direction

Use Qdrant as a serving index. `memoryd` writes denormalized payloads to Qdrant
only after evidence and projection validation succeed. The evidence inbox owns
raw events and audit history. Oxigraph owns facts, provenance, contradiction
records, retractions, temporal edges, and theme lineage.

MCP tools call the daemon, not Qdrant. The daemon decides which collections to
read or write, applies workspace filters, and reconciles failed projection
state.

## Consequences

- Qdrant collections can be dropped and rebuilt from evidence and graph state.
- Recall can filter by projection class, epistemic status, retraction state,
  source provider, workspace, and time without making those payload fields the
  authority.
- The implementation must maintain projection state and repair jobs for Qdrant
  writes.
- Direct Qdrant collection selection stays out of MCP request types.

## References

[^1]: `../dear-diary/README.md`.
[^2]: `docs/rfcs/0002-projection-tiers-and-promotion-rules.md`.
[^3]: `docs/rfcs/0003-hierarchical-materialization.md`.
