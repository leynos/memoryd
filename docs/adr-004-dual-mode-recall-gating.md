# Architectural decision record (ADR) 004: Dual-mode recall gating

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

Hierarchical recall expands from profile, fact, theme, and semantic-carrier
candidates into full episodes and optional raw-message blocks. Expansion must
respect token budgets and avoid redundant evidence. Axinite ADR 005 chooses a
dual-mode gate so recall works without a judge model but can spend local model
capacity when available.[^1]

Standalone `memoryd` keeps this decision because local installations will vary
widely in Ollama model availability and hardware capacity.

## Decision drivers

- `Recall` must work in cheap, air-gapped, and model-limited deployments.
- Rich deployments should use a local judge model when it improves expansion.
- Every expansion decision must expose a reason code for shadow evaluation.
- Recall must not depend on one model family’s uncertainty or log-probability
  surface.

## Options considered

| Option                                                    | Cost    | Portability | Risk                                                             |
| --------------------------------------------------------- | ------- | ----------- | ---------------------------------------------------------------- |
| Mandatory model-assisted gating                           | Highest | Weak        | Couples recall to model availability and latency.                |
| Proxy-only gating                                         | Lowest  | Strong      | Misses cases where a model can detect useful evidence expansion. |
| Shared gate with proxy and model-assisted implementations | Mixed   | Strong      | More implementation work, but comparable reason codes.           |

_Table 1: Recall gating options._

## Decision outcome / proposed direction

Use one gain interface with two implementations:

- proxy gate: deterministic score from novelty, support density, temporal fit,
  reinforcement, and token cost;
- model-assisted gate: local Ollama judge or reader model where configured.

Both implementations return `estimated_gain`, `estimated_token_cost`, and
`reason_code`.

## Consequences

- `cheap_v2` can ship before a judge model path exists.
- `evidence_v2` can be enabled per workspace or deployment after shadow
  comparison.
- Disagreement between proxy and model-assisted gates becomes an observable
  evaluation signal.

## References

[^1]: `../axinite/docs/adr-005-dual-mode-uncertainty-gating-for-hierarchical-recall.md`.
