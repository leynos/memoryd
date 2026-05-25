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

## Learn more

- [Documentation contents](docs/contents.md) - the full documentation index.
- [Terms of reference](docs/terms-of-reference.md) - problem space, users,
  scope, constraints, and open questions.
- [Memoryd design](docs/memoryd-design.md) - standalone daemon architecture and
  implementation strategy.
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
