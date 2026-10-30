# Documentation contents

[Documentation contents](contents.md) is the index for Memoryd's documentation
set.

## Project guides

- [Terms of reference](terms-of-reference.md) records the problem space,
  users, scope boundaries, constraints, and open questions for Memoryd.
- [Memoryd design](memoryd-design.md) describes the standalone daemon,
  collector, MCP front end, storage boundaries, projection pipeline, and
  verification strategy.
- [Roadmap](roadmap.md) translates the design, ADRs, and RFCs into
  GIST-aligned implementation phases and review-sized tasks.
- [Project-design ExecPlan](execplans/project-design.md) records the live
  remediation plan for the Logisphere design-stage review.
- [Local safety defaults ExecPlan](execplans/1-1-2-record-local-safety-defaults.md)
  records the completed decision work for roadmap item 1.1.2: the accepted
  workspace-identity, redaction, and purge defaults in ADR 015, and the runtime
  work they defer to later roadmap items.
- [Users' guide](users-guide.md) explains how to use Memoryd, the planned local
  safety defaults for workspace identity, redaction, and purge, and its public
  build and test commands.
- [Developers' guide](developers-guide.md) explains the local workflow and
  implementation tooling for contributors, including where the local safety
  policy belongs and the proof-to-test handoffs that implement it.
- [Repository layout](repository-layout.md) explains Memoryd's top-level files,
  directories, and ownership boundaries.
- [Documentation style guide](documentation-style-guide.md) defines the
  spelling, structure, Markdown, Architecture Decision Record (ADR), Request
  for Comments (RFC), and roadmap conventions used by this documentation set.

## Rust reference material

- [Reliable testing in Rust via dependency injection](reliable-testing-in-rust-via-dependency-injection.md)
  explains how to keep tests deterministic by injecting environment, clock,
  filesystem, and other external dependencies.
- [Rust doctest Don't Repeat Yourself guide](rust-doctest-dry-guide.md)
  explains how to write maintainable, executable Rust documentation examples.
- [Rust testing with `rstest` fixtures](rust-testing-with-rstest-fixtures.md)
  explains fixture-based, parameterized, and asynchronous testing with `rstest`.
- [rstest-bdd user guide](rstest-bdd-users-guide.md) covers writing Gherkin
  feature files and binding them to Rust step definitions via `rstest-bdd`.
- [pg_embedded_setup_unpriv user guide](pg-embed-setup-unpriv-users-guide.md)
  covers preparing PostgreSQL binaries and a data directory for unprivileged
  test environments.

## Engineering practice

- [Complexity antipatterns and refactoring strategies](complexity-antipatterns-and-refactoring-strategies.md)
  explains cognitive complexity, the bumpy-road antipattern, and refactoring
  approaches for maintainable code.
- [Scripting standards](scripting-standards.md) explains the preferred Python
  scripting stack, command execution patterns, and test expectations for helper
  scripts.

## Design records

- [ADR 001: Qdrant is a serving index](adrs/adr-0001-qdrant-is-a-serving-index.md)
  records the boundary between vector indexes and memory truth.
- [ADR 002: Dual-path semantic extraction](adrs/adr-0002-dual-path-semantic-extraction.md)
  records the shared extractor contract for encoder and Ollama paths.
- [ADR 003: Memoryd owns theme management](adrs/adr-0003-memoryd-owns-theme-management.md)
  records the boundary between Chutoro clustering and durable themes.
- [ADR 004: Dual-mode recall gating](adrs/adr-0004-dual-mode-recall-gating.md)
  records the proxy and model-assisted recall expansion strategy.
- [ADR 005: Hexagonal architecture boundary](adrs/adr-0005-hexagonal-architecture-boundary.md)
  records the dependency rule, port ownership, and adapter boundaries.
- [ADR 006: Tenant isolation and Corbusier context](adrs/adr-0006-tenant-isolation-and-corbusier-context.md)
  records the tenant-scoped request context, storage, index, graph, and theme
  isolation boundary.
- [ADR 007: Standard conversation ingestion port](adrs/adr-0007-standard-conversation-ingestion-port.md)
  records the canonical conversation-source and ingestion ports used by
  Corbusier, Axinite, Codex, Claude, and manual import adapters.
- [ADR 008: Source health and coverage foundation](adrs/adr-0008-source-health-and-coverage-foundation.md)
  records the pre-1.0 source-health substrate for freshness, parse, and
  accessibility diagnostics.
- [ADR 009: Claim identity and interpretive kind](adrs/adr-0009-claim-identity-and-interpretive-kind.md)
  records stable claim identity and claim-kind labels for claim-bearing memory.
- [ADR 010: Typed support edges](adrs/adr-0010-typed-support-edges.md) records
  relation-aware support links between claims and evidence.
- [ADR 011: Projection activity lineage](adrs/adr-0011-projection-activity-lineage.md)
  records derivation activity tracking for extraction, validation, promotion,
  repair, and future recomputation.
- [ADR 012: Durable recall audits](adrs/adr-0012-durable-recall-audits.md)
  records configurable persistence for decision-relevant recall traces.
- [ADR 013: Evidence-store engine and migration policy](adrs/adr-0013-evidence-store-engine-and-migration-policy.md)
  records the joint SQLite/PostgreSQL evidence-store policy, Diesel-backed
  adapter expectation, lockstep migration rule, backup expectation, and
  first-slice test matrix.
- [ADR 014: Main owns CodeScene coverage publication](adrs/adr-0014-main-owns-coverage-publication.md)
  records why the coverage upload runs on main and what the shared CV-005
  contract requires from pull requests.
- [ADR 015: Local safety defaults for workspace identity, redaction, and
  purge](adrs/adr-0015-local-safety-defaults.md) records the versioned
  workspace descriptor and collision rule, the first redaction detector corpus
  with mandatory deny patterns and raw-text modes, and the scope-bound
  confirmation and pre-purge backup expectations.

## Requests for comments

- [RFC 0001: Standalone evidence inbox](rfcs/rfc-0001-standalone-evidence-inbox.md)
  adapts Axinite's transactional outbox into a provider-neutral evidence inbox.
- [RFC 0002: Projection tiers and promotion rules](rfcs/rfc-0002-projection-tiers-and-promotion-rules.md)
  adapts Axinite's memory semantics to standalone provider evidence.
- [RFC 0003: Hierarchical materialization](rfcs/rfc-0003-hierarchical-materialization.md)
  defines raw evidence, episode, semantic-carrier, and theme hierarchy.
- [RFC 0004: Theme detection and rebalancing](rfcs/rfc-0004-theme-detection-and-rebalancing.md)
  defines Chutoro-backed, `memoryd`-owned theme management.
- [RFC 0005: Hierarchical recall](rfcs/rfc-0005-hierarchical-recall.md) defines
  recall profiles, context assembly, and fallback behaviour.
- [RFC 0006: Epistemic health, empiricism, falsification, and
  semirings][rfc-0006] proposes the post-1.0 substrate for Axinite v1.2 claim
  validity, omission observability, hypotheses, experiments, and outcome
  learning.

[rfc-0006]: rfcs/rfc-0006-epistemic-health-empiricism-falsification-and-semirings.md
