# Architectural decision record (ADR) 010: Typed support edges

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

The v1 design already requires every retrievable semantic artefact to preserve
evidence references. It also requires support-reference validation before model
or extractor output can become retrievable. A flat list of evidence references
is sufficient to prove that a span exists. It is not sufficient to explain how
the span relates to a claim, whether the relation was direct or weak, whether
the source was fresh at validation time, or whether the support was later
superseded by contradictory evidence.

Axinite's post-1.0 semiring and falsification proposals need support to be a
typed relation rather than an unstructured list. Current `memoryd` users also
benefit immediately: `memory_explain` can show why a claim exists, not merely
which raw spans were attached to it.

## Decision drivers

- Support relation is part of memory truth, not Qdrant payload convention.
- Evidence references must remain validated against tenant, workspace, hash,
  span, temporal, and redaction boundaries.
- The support model must be useful before full post-1.0 claim graphs exist.
- Contradiction, retraction, and recall explanation need relation semantics.
- The design must preserve the source-of-truth boundary from ADR 001.

## Options considered

| Option                                  | Consequence                                                                                                 |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Keep flat evidence-reference arrays     | Minimal implementation, but weak explanations and painful migration to claim graphs or provenance formulas. |
| Put support relation only in Qdrant     | Helps recall filtering, but makes serving payloads authoritative for truth and violates ADR 001.            |
| Add typed support edges to truth stores | Adds one domain relation now and lets Qdrant carry denormalized summaries without owning provenance.        |

_Table 1: Support-edge options._

## Decision outcome / proposed direction

Adopt typed support edges for claim-bearing semantic carriers, facts, profile
candidates, and future claim graph records.

The domain model will include:

- `SupportEdgeId`;
- `ClaimId`;
- `EvidenceRef`;
- `SupportRole`;
- `SupportValidationState`;
- `SupportValidationReason`;
- `SupportFreshness`;
- `SupportSourceHealth`;
- `SupportLifecycleState`.

The first `SupportRole` set is:

- `direct_support`;
- `corroborates`;
- `weak_support`;
- `derived_from`;
- `context`;
- `contradicts`;
- `supersedes`;
- `refutes`.

The support-reference validator writes the validation state for each edge.
Accepted edges can be projected to Oxigraph as provenance relations and to
Qdrant as denormalized payload fields. Qdrant remains a serving index; it does
not decide whether the edge is valid.

## Consequences

- `memory_explain` can show support roles, validation state, freshness, and
  source health.
- Contradiction and retraction workflows can target typed evidence relations.
- Post-1.0 semiring provenance can treat support edges as the atoms in
  compositional provenance expressions.
- Extraction adapters and Ollama structured output schemas must emit proposed
  support roles or leave them as `context` until validation narrows them.
- Validation fixtures must prove that dangling, stale, redacted, or
  cross-tenant support edges do not become authoritative.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0002-projection-tiers-and-promotion-rules.md`.
- `docs/adr-001-qdrant-is-a-serving-index.md`.
- `docs/adr-002-dual-path-semantic-extraction.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`.
