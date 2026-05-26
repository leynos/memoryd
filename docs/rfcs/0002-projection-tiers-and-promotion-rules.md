# RFC 0002: Projection tiers and promotion rules

## Preamble

- **RFC number:** 0002
- **Status:** Proposed
- **Created:** 2026-05-25
- **Adapted from:**
  `../axinite/docs/rfcs/0014-memory-projection-tiers-and-promotion-rules.md`

## Summary

This RFC defines the standalone `memoryd` projection taxonomy, epistemic status
model, observer and subject scope, promotion rules, contradiction handling, and
reconciliation metadata. It keeps Axinite’s memory semantics but applies them
to evidence from multiple providers.

## Problem

Provider transcripts contain mixed epistemic material: user assertions,
assistant hypotheses, tool outputs, file edits, failed commands, summaries, and
imported notes. If `memoryd` flattens those records into "facts", recall will
treat model guesses and explicit human instructions as equivalent. The service
needs a closed projection model and promotion rules before it can claim
trustworthy recall.

## Goals and non-goals

- Goals:
  - Define first-class projection classes.
  - Define epistemic status for claim-bearing artefacts.
  - Define stable claim identity and interpretive claim kind.
  - Define typed support edges for claim-bearing artefacts.
  - Define observer and subject scope.
  - Define promotion, demotion, contradiction, and reconciliation rules.
- Non-goals:
  - Define the evidence inbox schema. RFC 0001 owns that.
  - Define episode boundaries or theme structures. RFC 0003 and RFC 0004 own
    those.
  - Specify embedding models or extraction prompts.

## Proposed design

### Projection classes

| Class     | Meaning                                           | Primary authority                                  |
| --------- | ------------------------------------------------- | -------------------------------------------------- |
| `episode` | Bounded chronological interaction or event record | Evidence inbox and Oxigraph provenance             |
| `summary` | Distilled representation of episodes              | Evidence inbox metadata and Qdrant serving payload |
| `concept` | Named entity, topic, or category                  | Oxigraph                                           |
| `fact`    | Discrete verifiable claim                         | Oxigraph                                           |
| `profile` | Stable representation of an entity                | Oxigraph and Qdrant serving payload                |

_Table 1: Projection classes._

### Epistemic status

| Status         | Meaning                                                                          |
| -------------- | -------------------------------------------------------------------------------- |
| `explicit`     | Directly stated by a human or trusted source.                                    |
| `curated`      | Reviewed and approved by an operator.                                            |
| `deduced`      | Logically derived from explicit or curated premises.                             |
| `hypothesized` | Inductively or abductively inferred.                                             |
| `retracted`    | Previously held and later withdrawn, contradicted, or purged from active recall. |

_Table 2: Epistemic status values._

Episodes and summaries are evidence or evidence summaries. They do not carry
epistemic status as truth claims. Concepts inherit the strongest supporting
status.

### Claim identity and kind

Every claim-bearing semantic carrier, fact, and profile candidate receives a
stable `ClaimId`. The claim ID is independent of Qdrant point IDs, projection
retry IDs, and graph edge IDs.

Each claim also carries `ClaimKind`. The first set is `observation`,
`user_assertion`, `assistant_inference`, `decision`, `preference`,
`instruction`, `hypothesis`, `causal_candidate`, `profile_trait`,
`recommendation_support`, and `unknown`.

`ClaimKind` describes the interpretive role of a claim. Epistemic status
describes trust state.

### Support edges

Claim-bearing artefacts use typed support edges rather than only flat evidence
reference arrays. The first `SupportRole` set is `direct_support`,
`corroborates`, `weak_support`, `derived_from`, `context`, `contradicts`,
`supersedes`, and `refutes`.

Support edges carry validation state, validation reason, lifecycle state,
freshness, and source-health data where available. Only validated support edges
can contribute to promotion or graph-backed trust.

### Scope

Every projection carries:

- `observer_id`;
- `subject_id`;
- `workspace_id`;
- `scope`, one of `private`, `workspace`, or `shared`;
- optional `audience`;
- evidence references.

### Promotion rules

- Direct human statements become `explicit`.
- Operator-approved MCP writes become `curated`.
- Model-derived statements default to `hypothesized`.
- `hypothesized` statements can become `deduced` only when a rule or
  reconciler proves they follow from trusted premises.
- Profile promotion requires `explicit` or `curated` status, stability across
  a configured duration, no active contradiction, and a durable trait rather
  than a transient state.

### Contradiction rules

- New `explicit` evidence retracts conflicting `hypothesized` or `deduced`
  facts automatically.
- Conflicts between `explicit` and `curated` facts become operator-resolution
  records.
- `hypothesized` evidence never retracts `explicit` or `curated` facts.

### Reconciliation metadata

Each projection target records:

- `projection_id`;
- target name;
- status: `pending`, `synced`, or `failed`;
- retry count;
- last error;
- last synced timestamp;
- soft and hard deletion flags.

## Compatibility and migration

Axinite-derived facts, documents, and workspace memories map directly into
these projection classes. Codex and Claude evidence enters with weaker default
status until direct human assertion or curation upgrades it.

## Open questions

- Which explicit user statements should auto-promote to profile material?
- Which contradiction detector runs before an operator reviews conflicts?
- How should multi-agent observer identity be represented in the first
  implementation?

## Recommendation

Adopt Axinite’s projection taxonomy unchanged, but require provider adapters to
supply enough observer, subject, workspace, and evidence metadata for the
taxonomy to remain meaningful outside Axinite.
