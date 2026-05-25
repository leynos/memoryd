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
- [User guide](users-guide.md) explains how to use the generated project and
  its public build and test commands.
- [Developer guide](developers-guide.md) explains the local workflow and
  implementation tooling for contributors.
- [Repository layout](repository-layout.md) explains the generated project's
  top-level files, directories, and ownership boundaries.
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

## Engineering practice

- [Complexity antipatterns and refactoring strategies](complexity-antipatterns-and-refactoring-strategies.md)
  explains cognitive complexity, the bumpy-road antipattern, and refactoring
  approaches for maintainable code.
- [Scripting standards](scripting-standards.md) explains the preferred Python
  scripting stack, command execution patterns, and test expectations for helper
  scripts.

## Design records

- [ADR 001: Qdrant is a serving index](adr-001-qdrant-is-a-serving-index.md)
  records the boundary between vector indexes and memory truth.
- [ADR 002: Dual-path semantic extraction](adr-002-dual-path-semantic-extraction.md)
  records the shared extractor contract for encoder and Ollama paths.
- [ADR 003: Memoryd owns theme management](adr-003-memoryd-owns-theme-management.md)
  records the boundary between Chutoro clustering and durable themes.
- [ADR 004: Dual-mode recall gating](adr-004-dual-mode-recall-gating.md)
  records the proxy and model-assisted recall expansion strategy.
- [ADR 005: Hexagonal architecture boundary](adr-005-hexagonal-architecture-boundary.md)
  records the dependency rule, port ownership, and adapter boundaries.
- [ADR 006: Tenant isolation and Corbusier context](adr-006-tenant-isolation-and-corbusier-context.md)
  records the tenant-scoped request context, storage, index, graph, and theme
  isolation boundary.
- [ADR 007: Standard conversation ingestion port](adr-007-standard-conversation-ingestion-port.md)
  records the canonical conversation-source and ingestion ports used by
  Corbusier, Axinite, Codex, Claude, and manual import adapters.

## Requests for comments

- [RFC 0001: Standalone evidence inbox](rfcs/0001-standalone-evidence-inbox.md)
  adapts Axinite's transactional outbox into a provider-neutral evidence inbox.
- [RFC 0002: Projection tiers and promotion rules](rfcs/0002-projection-tiers-and-promotion-rules.md)
  adapts Axinite's memory semantics to standalone provider evidence.
- [RFC 0003: Hierarchical materialization](rfcs/0003-hierarchical-materialization.md)
  defines raw evidence, episode, semantic-carrier, and theme hierarchy.
- [RFC 0004: Theme detection and rebalancing](rfcs/0004-theme-detection-and-rebalancing.md)
  defines Chutoro-backed, `memoryd`-owned theme management.
- [RFC 0005: Hierarchical recall](rfcs/0005-hierarchical-recall.md) defines
  recall profiles, context assembly, and fallback behaviour.
