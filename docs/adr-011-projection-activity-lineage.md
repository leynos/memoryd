# Architectural decision record (ADR) 011: Projection activity lineage

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

`memoryd` will summarize episodes, extract semantic carriers, validate support
references, embed text, promote graph facts, repair projections, and rebuild
serving indexes. The current evidence inbox has projection state and audit
logs, but those records do not fully explain which activity produced a claim,
which model or extractor was used, which validator version ran, or which input
artefacts were consumed.

That gap matters before v1 because failed extraction and repair need operator
diagnostics. It matters even more after v1 because Axinite's epistemic-health
work needs to replay or recompute claim validity when evidence, support,
models, extractors, or source-health state changes.

## Decision drivers

- Projection work should be replayable and explainable.
- Activity lineage is not the same as user-facing audit logs.
- Model, extractor, validator, and configuration identity must be captured
  without leaking infrastructure types into the domain.
- Lineage records must be tenant-scoped.
- The design should support post-1.0 validity recomputation without requiring
  every v1 projection to be rewritten.

## Options considered

| Option                                  | Consequence                                                                                                           |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Rely on projection state and logs only  | Keeps schema small, but loses activity inputs, outputs, model identity, and validator identity needed for replay.     |
| Store lineage inside adapter logs       | Helps local debugging, but makes provenance non-queryable and provider-specific.                                      |
| Add projection activity lineage records | Adds a small truth-plane record for extraction, validation, embedding, promotion, repair, and recall-audit producers. |

_Table 1: Projection-activity options._

## Decision outcome / proposed direction

Adopt projection activity lineage records before v1.

The domain model will include:

- `ProjectionActivityId`;
- `ProjectionActivityKind`;
- `ProjectionActivityInput`;
- `ProjectionActivityOutput`;
- `ProjectionActivityStatus`;
- `ProducerIdentity`;
- `ConfigurationDigest`;
- `DiagnosticRef`.

The first activity kinds are:

- `episode_finalization`;
- `episode_summary`;
- `semantic_extraction`;
- `support_validation`;
- `embedding`;
- `fact_promotion`;
- `profile_promotion`;
- `theme_assignment`;
- `projection_repair`;
- `recall_audit_capture`.

Activity records link input artefacts to output artefacts. They record
extractor, model, validator, and configuration identities as domain strings or
digests, not SDK objects. They do not replace audit logs: audit logs explain
access and decisions; activity lineage explains derivation.

## Consequences

- Operators can inspect which activity created or rejected a semantic carrier.
- Reprojection and repair can target artefacts affected by a model,
  configuration, or validator change.
- Post-1.0 claim-validity recomputation can start from existing derivation
  activities.
- The evidence store will need additional lineage tables, and graph projection
  will need provenance edges for selected activities.
- Adapters must not bypass activity recording when they write derived artefacts.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0001-standalone-evidence-inbox.md`.
- `docs/rfcs/0003-hierarchical-materialization.md`.
- `docs/adr-002-dual-path-semantic-extraction.md`.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`.
