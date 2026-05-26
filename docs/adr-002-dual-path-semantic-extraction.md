# Architectural decision record (ADR) 002: Dual-path semantic extraction

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

`memoryd` must derive semantic carriers, episode summaries, facts, concepts,
and profile candidates from untrusted provider evidence. Ollama-backed
structured extraction can produce useful canonical statements, relations, and
summaries, but generative models can produce unsupported support links. A
purely extractive encoder path is cheaper and easier to bound, but produces
lower-quality abstractions.

This decision imports Axinite ADR 004 and adapts it to standalone provider logs
rather than Axinite-only messages and document revisions.[^1]

## Decision drivers

- No semantic carrier becomes retrievable unless its support references
  resolve to stored evidence.
- Local-only deployments need a cheap fallback when no generative model is
  available.
- Rich deployments should still be able to use Ollama for higher-quality
  canonical statements and summaries.
- The projection pipeline must not branch on model family after extraction.

## Options considered

| Option                                  | Provenance reliability                                | Quality                                | Fallback behaviour                           |
| --------------------------------------- | ----------------------------------------------------- | -------------------------------------- | -------------------------------------------- |
| LLM-only structured extraction          | Weak unless support references are validated strictly | Rich                                   | Poor when the generative path is unavailable |
| Encoder-only extractive projection      | Strong because spans remain structural                | Lower abstraction quality              | Strong                                       |
| Dual-path extraction with shared schema | Strong after shared validation                        | Rich when available, bounded otherwise | Strong                                       |

_Table 1: Extraction options._

## Decision outcome / proposed direction

Use dual-path extraction with a shared schema:

- `encoder_extractive` emits extractive statements, structural support
  references, confidence, semantic kind, temporal hints, and extraction mode.
- `llm_structured` uses Ollama to emit canonical text, entities, relations,
  facts, confidence, temporal hints, and evidence spans.

Both paths pass through one support-reference validator. Unsupported outputs
remain diagnostics and never enter Qdrant, Oxigraph, or theme management.

## Consequences

- The encoder path can be implemented first and used as the shadow baseline.
- The Ollama path can be compared against the encoder path before promotion.
- Extractor disagreement is expected and must be recorded for evaluation.
- Sentence segmentation and span mapping become explicit design work because
  both paths depend on structural support references.

## References

[^1]: `../axinite/docs/adr-004-dual-path-semantic-extraction-with-validated-provenance.md`.
