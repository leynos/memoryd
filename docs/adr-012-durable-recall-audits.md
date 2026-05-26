# Architectural decision record (ADR) 012: Durable recall audits

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

Hierarchical recall returns a selection trace with selected artefacts, fallback
reasons, and expansion diagnostics. That is sufficient for immediate client
use, but it is not enough for post-hoc review. Axinite's post-1.0 world-model
proposals need to know which evidence was selected, which evidence was
rejected, which filters applied, whether source health degraded, and whether
the same query would later select different material.

Persisting every recall request forever would create privacy, storage, and
noise problems. The v1 daemon therefore needs an explicit durable recall-audit
mode that can be enabled where reviewability matters and kept off or sampled
where it does not.

## Decision drivers

- Recall trace persistence is useful for evaluation, debugging, and later
  omission observability.
- Durable audit records must not expose raw prompt text by default.
- The feature must be bounded and configurable.
- The audit record must be tenant-scoped and workspace-aware.
- Public MCP clients should not decide tenant identity or bypass audit policy.

## Options considered

| Option                               | Consequence                                                                                                         |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| Return traces only                   | Keeps v1 simple, but prevents durable review of decision-relevant recall and later omission analysis.               |
| Persist every recall unconditionally | Maximizes reviewability, but creates storage growth, privacy risk, and noisy audit surfaces.                        |
| Add configurable recall audit modes  | Preserves a small v1 surface while allowing decision-relevant or sampled durable traces where policy requires them. |

_Table 1: Recall-audit options._

## Decision outcome / proposed direction

Adopt optional durable recall audits before v1.

The domain model will include:

- `RecallAuditId`;
- `RecallAuditMode`;
- `RecallAuditRecord`;
- `RecallCandidateTrace`;
- `RecallFilterTrace`;
- `RecallFallbackReason`;
- `RecallSourceHealthSnapshot`.

The first audit modes are:

- `none`;
- `errors_only`;
- `decision_relevant`;
- `sampled`;
- `all`.

Recall audit records store bounded metadata:

- tenant and workspace;
- principal or caller;
- query hash and optional redacted query text when policy allows it;
- recall profile;
- token budget;
- filters;
- selected artefact IDs;
- top rejected candidate IDs and reason codes;
- fallback reason;
- source-health summary;
- projection and model configuration digests.

The daemon owns audit-mode policy. MCP request fields may request stricter
audit, but they may not disable a policy-required audit.

## Consequences

- Shadow evaluation and post-hoc review can inspect what recall selected and
  omitted.
- Operators can keep raw query text out of durable storage by default.
- Post-1.0 coverage and salience audits can build on v1 recall audit records.
- The implementation must enforce retention and purge behaviour for recall
  audits alongside evidence, graph, Qdrant, and Chutoro state.
- Recall tests must cover audit modes and read-only MCP behaviour without
  making audit persistence part of every recall path.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0005-hierarchical-recall.md`.
- `docs/adr-004-dual-mode-recall-gating.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`.
