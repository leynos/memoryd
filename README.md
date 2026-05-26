# Memoryd

*Local-first memory infrastructure for coding agents.*

Memoryd is a Rust daemon project for turning Codex CLI, Claude Code, Axinite,
and manual session history into evidence-backed memory. The implementation is
still at scaffold stage; the current repository defines the product boundary,
architecture, ADRs, and RFCs that will drive the daemon, collector, and MCP
front end.

______________________________________________________________________

## Why Memoryd?

- **Keep context recoverable**: preserve useful coding-agent decisions after a
  session is split, compacted, or closed.
- **Treat logs as evidence**: keep provenance, epistemic status, retractions,
  and source references instead of flattening everything into vector search.
- **Stay local-first**: use local storage and local model providers by default
  for sensitive repository and transcript data.
- **Integrate without lock-in**: adapt Codex CLI, Claude Code, Axinite, and
  manual imports through provider adapters.

______________________________________________________________________

## Quick start

### Installation

Clone the repository and build the scaffolded binary:

```bash
git clone https://github.com/leynos/memoryd.git
cd memoryd
make build
```

### Basic usage

Run the current application stub:

```bash
cargo run --quiet
```

Expected output:

```text
Hello from Memoryd!
```

For the full local validation workflow, run:

```bash
make all
```

______________________________________________________________________

## Features

- Standalone daemon design for `memoryd`, `memoryd-collector`, and
  `memoryd-mcp`.
- Provider-neutral evidence inbox for Codex CLI, Claude Code, Axinite, and
  manual imports.
- Projection model for episodes, semantic carriers, facts, profiles, and
  themes.
- Storage boundaries for Qdrant, Oxigraph, Ollama, Chutoro, and the local
  evidence store.
- MCP front-end design modelled on Dear Diary, with recall, explain, store,
  retract, sessions, import, profile, and health tools.
- ADR and RFC set covering serving indexes, extraction, theme ownership, recall
  gating, materialization, and promotion rules.

______________________________________________________________________

## Architecture Overview

The full architecture is defined in the
[Memoryd design](docs/memoryd-design.md) and the supporting ADR set. The most
important implementation boundary is recorded in
[ADR 005: Hexagonal architecture boundary](docs/adr-005-hexagonal-architecture-boundary.md).
Domain and application code own the memory rules, use cases, and port traits;
adapters own provider parsing, local persistence, Qdrant, Ollama, Oxigraph,
Chutoro, MCP, UDS, loopback HTTP, and filesystem watching.

Port traits are the contract between those layers. Application services such as
ingestion, recall, curated memory storage, retraction, session listing, and
purge depend on domain-owned ports and domain value objects. Infrastructure
adapters implement those ports and translate external types into canonical
evidence, episode, projection, recall, audit, and health concepts. This keeps
the core testable without running storage engines, model servers, vector
indexes, graph stores, clustering runtimes, or MCP transports.

Tenant isolation is a first-class part of the same boundary, as recorded in
[ADR 006: Tenant isolation and Corbusier context](docs/adr-006-tenant-isolation-and-corbusier-context.md).
Tenant-owned use cases carry an authenticated request context, and the
normative memory scope is `(tenant_id, workspace_id)`. Storage rows, Qdrant
payload filters, Oxigraph named graphs, Chutoro checkpoints, audit records, and
purge plans all preserve that scope so local single-user deployments and
Corbusier-style multi-tenant deployments exercise the same isolation contract.

Conversation ingestion follows
[ADR 007: Standard conversation ingestion port](docs/adr-007-standard-conversation-ingestion-port.md).
All producers emit the same canonical conversation delta, but the application
ports separate trust and sequencing semantics: `ConversationPushIngestPort`
handles authenticated push producers such as Corbusier, Axinite outboxes, and
manual MCP imports; `CollectedConversationIngestPort` handles worker-collected
batches from filesystem or pull-mode sources; and `ConversationSourcePort` is
the collector-facing discovery, tailing, snapshot, and replay contract for
Codex, Claude, Axinite pull mode, and future source adapters.

______________________________________________________________________

## Learn more

- [Documentation contents](docs/contents.md) - the full documentation index.
- [Terms of reference](docs/terms-of-reference.md) - problem space, users,
  scope, constraints, and open questions.
- [Memoryd design](docs/memoryd-design.md) - standalone daemon architecture and
  implementation strategy.
- [Roadmap](docs/roadmap.md) - implementation phases, dependencies, and
  review-sized tasks.
- [Users' Guide](docs/users-guide.md) - generated project commands and local
  usage.
- [Developers' Guide](docs/developers-guide.md) - contributor workflow and
  validation gates.

______________________________________________________________________

## Licence

ISC Licence - see [LICENSE](LICENSE) for details.

______________________________________________________________________

## Contributing

Contributions are welcome. Please start with [AGENTS.md](AGENTS.md) for the
repository workflow, code style, and validation requirements.
