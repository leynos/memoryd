# Architectural decision record (ADR) 009: Claim identity and interpretive kind

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

`memoryd` distinguishes raw evidence, episodes, summaries, semantic carriers,
facts, profiles, and themes. It also distinguishes epistemic status values such
as `explicit`, `curated`, `deduced`, `hypothesized`, and `retracted`. That
prevents model guesses from being treated as equivalent to direct human
statements.

However, Axinite's post-1.0 epistemic-health work needs a stable object for
claim validation, recomputation, falsification, and provenance composition. A
fact ID, semantic-carrier ID, Qdrant point ID, or graph edge ID is not enough:
those identifiers describe storage or projection artefacts, not the underlying
claim being evaluated. The v1 design also needs to distinguish what kind of
interpretive move a claim represents. A hypothesized preference, a hypothesized
causal explanation, and a hypothesized project decision should not be filtered
or promoted the same way.

## Decision drivers

- Claim-bearing memory needs a stable identity independent of storage backend.
- Interpretation kind is orthogonal to epistemic status.
- The v1 daemon should avoid a disruptive migration when post-1.0 claim graphs
  arrive.
- Claim identity must remain tenant-scoped and workspace-aware.
- The domain must own the claim vocabulary; Qdrant, Oxigraph, and MCP adapters
  must not define it.

## Options considered

| Option                              | Consequence                                                                                                             |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Use projection IDs as claim IDs     | Avoids a new identifier, but couples claim identity to projection retries, storage targets, and rebuild behaviour.      |
| Defer claim identity until post-1.0 | Keeps v1 smaller, but forces migration across graph facts, Qdrant payloads, explanations, and retraction records later. |
| Add stable claim identity in v1     | Adds a small domain concept now and gives later claim graphs, falsification, and semiring provenance a stable anchor.   |

_Table 1: Claim-identity options._

## Decision outcome / proposed direction

Adopt stable claim identity and interpretive kind for every claim-bearing
semantic carrier, fact, and profile candidate before v1.

The domain model will include:

- `ClaimId`;
- `ClaimStatement`;
- `ClaimKind`;
- `ClaimRef`;
- `ClaimValidityState`;
- `ClaimLifecycleState`.

The first `ClaimKind` set is:

- `observation`;
- `user_assertion`;
- `assistant_inference`;
- `decision`;
- `preference`;
- `instruction`;
- `hypothesis`;
- `causal_candidate`;
- `profile_trait`;
- `recommendation_support`;
- `unknown`.

`ClaimKind` describes the interpretive role of the claim. Epistemic status
continues to describe trust state. For example, a `causal_candidate` can remain
`hypothesized`, and a `profile_trait` can become `curated`.

Claim identity is not a promise that the statement is true. It is a stable
handle for support edges, contradiction records, retractions, recall audit
records, future claim graphs, and post-1.0 falsification workflows.

## Consequences

- Claim-bearing records can be explained, retracted, recomputed, and audited by
  stable claim ID.
- Recall filters can distinguish factual observations from hypotheses,
  instructions, preferences, decisions, and causal candidates.
- Post-1.0 claim graphs can attach richer provenance and validity state to
  existing v1 claims.
- The first implementation must avoid treating `ClaimKind` as a promotion
  shortcut; promotion still depends on epistemic status, support, policy, and
  contradiction state.
- Adapter payloads may suggest a claim kind, but the application layer validates
  or normalizes it before persistence.

## References

- `docs/memoryd-design.md`.
- `docs/rfcs/0002-projection-tiers-and-promotion-rules.md`.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`.
