# Record the evidence-store engine and migration policy

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
 `Tolerances`, `Risks`, `Progress`, `Surprises & Discoveries`, `Decision Log`,
and `Outcomes & Retrospective` must be kept up to date as work proceeds.

Status: COMPLETE

## Purpose / big picture

Memoryd cannot implement the evidence inbox safely until the project records
which relational stores it supports, how schema changes are versioned, and how
SQLite and PostgreSQL remain behaviourally equivalent. Roadmap item 1.1.1
therefore creates one accepted Architecture Decision Record (ADR) that fixes
the v1 evidence-store engine and migration policy before evidence capture,
session browsing, or flat recall work begins.

After this plan is implemented, a maintainer can open the new ADR and see that
Memoryd uses a joint SQLite/PostgreSQL evidence-store approach through Diesel,
with lockstep migrations modelled after `mxd`, local SQLite as the quick-start
path, PostgreSQL as the enterprise-ready path, documented backup expectations,
and a first-slice test matrix for the evidence inbox. Success is observable
when roadmap item 1.1.1 is marked done, the open storage question is closed in
the terms of reference and RFC 0001, and the documentation gates pass.

This plan is pre-implementation. Do not implement it until the user explicitly
approves the plan.

## Constraints

- Implement documentation and contract decisions only. Do not add Diesel
  dependencies, migrations, schema modules, repositories, runtime configuration
  parsing, or tests in this slice unless the user explicitly approves a scope
  expansion.
- Create one accepted ADR as the authoritative decision record. The expected
  filename is `docs/adr-013-evidence-store-engine-and-migration-policy.md`
  because ADR 012 is currently the newest accepted sequence number.
- Preserve the decision stated in the request: v1 uses a joint
  SQLite/PostgreSQL approach, abstracted with Diesel and lockstep migrations
  following `mxd`; libSQL/Turso is not selected for v1 because Diesel support
  and project stability are not yet strong enough.
- Keep Memoryd hexagonal. Domain and application contracts may name evidence
  store policies and ports, but domain code must not depend on Diesel,
  PostgreSQL, SQLite, migration files, or `pg_embedded_setup_unpriv`.
- Keep tenant isolation first-class. PostgreSQL may enforce tenant isolation
  with row-level security (RLS) and transaction-local settings in adapters;
  SQLite must still carry tenant identifiers and pass adapter contract tests.
- Use en-GB Oxford spelling in documentation while preserving external API
  names such as `postgres`, `sqlite`, `DATABASE_URL`, and Cargo feature names.
- Follow `AGENTS.md`: run gates sequentially, capture command output with
  `tee`, commit only gated changes, and do not run test, lint, or formatting
  commands in parallel.
- Use the loaded skills as signposts: `leta` for code navigation,
  `rust-router` to route Rust boundary questions, `hexagonal-architecture` for
  port and adapter boundaries, `arch-crate-design` for any crate-boundary
  decisions that become necessary, `firecrawl` for external prior-art checks,
  and `execplans` for maintaining this document.
- Run `coderabbit review --agent` only after deterministic gates for the
  current milestone pass. Clear any applicable concerns before moving to the
  next milestone.
- Do not mark roadmap item 1.1.1 done until the accepted ADR and all required
  cross-document updates have landed and passed validation.

## Tolerances (exception triggers)

- Scope: if implementation requires production Rust code changes, stop,
  document the reason in `Decision Log`, and ask for approval before editing
  code.
- Scope: if more than six repository files need changes, stop and ask whether
  to split the work. The expected files are the new ADR plus `docs/roadmap.md`,
  `docs/terms-of-reference.md`, `docs/memoryd-design.md`,
  `docs/rfcs/0001-standalone-evidence-inbox.md`, and `docs/contents.md`.
- Interface: if the plan appears to require a public command-line interface,
  MCP tool, or configuration key before the storage ADR is accepted, stop and
  defer that interface to the implementation slice that first uses it.
- Architecture: if any wording makes SQLite the only durable truth store or
  makes PostgreSQL the only supported v1 store, stop and correct the decision
  back to the approved joint-store policy.
- Migration policy: if a migration cannot be expressed as paired SQLite and
  PostgreSQL migrations with identical semantic versions, stop and require an
  explicit ADR exception.
- Validation: if `make check-fmt`, `make typecheck`, `make lint`, or
  `make test` fails twice after focused repair, stop and record the blocker
  with the failing log path.
- Review: if `coderabbit review --agent` is unavailable because of local
  tooling or authentication, record the command and failure in
  `Surprises & Discoveries`, then continue only after deterministic gates have
  passed.

## Risks

- Risk: The ADR could smuggle implementation work into a decision slice.
  Severity: medium. Likelihood: medium. Mitigation: keep this slice to
  documentation contracts and schedule concrete migrations for roadmap item
  2.1.1.
- Risk: Two supported stores can drift semantically over time. Severity: high.
  Likelihood: medium. Mitigation: require lockstep migration directories,
  identical migration version numbers, shared repository contract tests, and a
  first-slice matrix that exercises SQLite and PostgreSQL.
- Risk: PostgreSQL row-level security could leak into domain contracts.
  Severity: high. Likelihood: low. Mitigation: describe RLS as an adapter
  enforcement mechanism; keep tenant-scoped domain types and ports backend
  neutral.
- Risk: Backup expectations could be over-promised before tooling exists.
  Severity: medium. Likelihood: medium. Mitigation: require operator
  documentation to recommend explicit pre-migration and pre-purge backups while
  making clear that automatic backup tooling is a later feature unless
  implemented.
- Risk: `make lint` and `make test` may be slow for a documentation-only
  change. Severity: low. Likelihood: medium. Mitigation: run them anyway
  because the task explicitly requests these gates after major milestones.
- Risk: The imported `rstest-bdd` and `pg_embedded_setup_unpriv` guides may
  become stale as upstream evolves. Severity: low. Likelihood: medium.
  Mitigation: cite the pinned local docs for implementation guidance and update
  them only in a separate dependency-documentation slice.

## Relevant documentation and skills

The implementer must read these local documents before editing:

- `AGENTS.md`, for branch, gate, commit, and test execution rules.
- `docs/roadmap.md`, especially roadmap item 1.1.1 and its dependent tasks.
- `docs/terms-of-reference.md`, especially the open decision table for the
  evidence inbox store and tenant storage strategy.
- `docs/memoryd-design.md`, especially the minimum useful deployment,
  evidence schema outline, tenant isolation, purge, configuration, and open
  decisions sections.
- `docs/rfcs/0001-standalone-evidence-inbox.md`, especially compatibility,
  migration, and open questions.
- `docs/documentation-style-guide.md`, especially ADR naming, section, and
  formatting rules.
- `docs/developers-guide.md`, `docs/contents.md`, `docs/users-guide.md`,
  `docs/pg-embed-setup-unpriv-users-guide.md`, and
  `docs/rstest-bdd-users-guide.md`.

The implementer must use these skills as working rules:

- `leta`: use `leta files`, `leta grep`, `leta refs`, and `leta show` for code
  navigation instead of ad hoc source browsing when symbols are involved.
- `rust-router`: route any Rust implementation or API-shape question to the
  smallest relevant Rust skill before changing code.
- `hexagonal-architecture`: keep domain policy, ports, and adapters separated.
- `arch-crate-design`: if implementation pressure creates crate or feature
  boundary questions, decide the smallest useful crate/module split.
- `firecrawl`: use external sources only to resolve gaps in open-source
  tooling, protocols, formats, or prior art.
- `execplans`: keep this document self-contained and update the living
  sections as work proceeds.

## Prior art and external references

Local prior art is `mxd` at commit `153671c2c8794b8fb122545afdbea1df8e35cbc1`.
Its relevant pattern is:

- parallel migration directories at `migrations/sqlite/` and
  `migrations/postgres/` with matching numeric versions;
- backend-selected embedded migrations using
  `embed_migrations!("migrations/sqlite")` or
  `embed_migrations!("migrations/postgres")`;
- compile-time backend feature selection so exactly one storage backend is
  linked for a build;
- PostgreSQL integration tests that use `POSTGRES_TEST_URL` when supplied and
  otherwise bootstrap an embedded test cluster with `pg_embedded_setup_unpriv`;
- a test helper that distinguishes PostgreSQL unavailability from genuine
  initialization failure, so local optional skips do not hide configured
  database failures.

Firecrawl was used during plan drafting to check external tooling references.
The Diesel getting-started guide confirms that Diesel supports PostgreSQL and
SQLite, that migrations use `up.sql` and `down.sql`, and that applying and
reverting a migration should leave the schema unchanged. The
`diesel_migrations::embed_migrations!` documentation confirms that Diesel can
embed migrations at compile time and that a build script should emit
`cargo:rerun-if-changed=...` for migration directories because proc macros
cannot otherwise notice changed migration files.

## Architecture target for the future implementation

The ADR should describe the target architecture without implementing it in this
slice:

- Domain model: tenant-scoped evidence concepts, store-engine policy names,
  migration version identity, and backup-policy vocabulary. No SQL or Diesel
  types.
- Application layer: ports such as `EvidenceRepository` and
  `MigrationCoordinator`, expressed in domain types and semantic errors.
- SQLite adapter: Diesel-backed local default for quick-start single-user and
  local daemon deployments. It enforces tenant predicates in adapter queries
  and contract tests.
- PostgreSQL adapter: Diesel-backed deployment path for enterprise and
  hosted-ready use. It sets transaction-local tenant context, enables RLS for
  tenant-owned tables, and uses non-owner application roles without `BYPASSRLS`.
- Migration layout: paired `migrations/sqlite/<version>_<name>/` and
  `migrations/postgres/<version>_<name>/` directories. Every version must exist
  in both trees, use the same semantic migration name, and preserve the same
  logical schema contract even where SQL differs.
- Testing: repository contract tests run against SQLite and PostgreSQL. The
  PostgreSQL path uses `POSTGRES_TEST_URL` when set and otherwise uses
  `pg_embedded_setup_unpriv` through `rstest` fixtures modelled after `mxd`.

## Implementation plan

Before editing, confirm the branch is
`1-1-1-record-evidence-store-engine-and-migration-policy` and the working tree
is clean:

```sh
git branch --show-current
git status --short
```

If the branch is not correct, rename it before making changes. If the working
tree contains unrelated user changes, leave them alone and do not stage them.

### Milestone 1: Write the accepted ADR

Create `docs/adr-013-evidence-store-engine-and-migration-policy.md`.

The ADR must use the documentation style guide's ADR structure and include
these sections:

- `Status`: `Accepted`, with the date and a one-sentence summary of the joint
  SQLite/PostgreSQL decision.
- `Date`: the implementation date in `YYYY-MM-DD` format.
- `Context and problem statement`: explain that roadmap item 1.1.1 must close
  the default evidence-store, migration, backup, and test-matrix decision
  before evidence inbox implementation begins.
- `Decision drivers`: include quick local startup, enterprise-ready
  PostgreSQL, Diesel support, parity between stores, tenant isolation,
  migration auditability, and deferral of libSQL/Turso.
- `Requirements`: record the default local store, the PostgreSQL supported
  path, Diesel abstraction, lockstep migration trees, backup expectations, and
  first-slice test matrix.
- `Options considered`: compare SQLite-only, PostgreSQL-only,
  SQLite/PostgreSQL with Diesel, and libSQL/Turso. The selected option is the
  joint SQLite/PostgreSQL policy.
- `Decision outcome / proposed direction`: state the final policy in binding
  language.
- `Migration plan`: explain that this ADR lands the contract only; roadmap
  item 2.1.1 implements the evidence inbox migrations and repository APIs.
- `Known risks and limitations`: describe drift, dialect differences, RLS
  asymmetry, backup tooling limits, and PostgreSQL test setup cost.
- `Consequences`: describe how later slices must add paired migrations,
  backend contract tests, documentation, and CI gates.
- `References`: link local docs and external prior art.

The ADR must explicitly say:

- v1 local default is SQLite;
- v1 also supports PostgreSQL as a first-class deployment path;
- Diesel is the storage abstraction for relational evidence-store adapters;
- migrations are maintained in lockstep per backend, following `mxd`;
- every migration version must have SQLite and PostgreSQL entries with the
  same version and logical schema effect;
- schema drift is a test failure, not a tolerated backend difference;
- pre-migration and pre-purge operator backups are expected until automated
  backup tooling exists;
- the first implementation slice must test SQLite always and PostgreSQL via
  `POSTGRES_TEST_URL` or `pg_embedded_setup_unpriv`.

After the ADR is written, run the documentation gates:

```sh
set -o pipefail
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-1-adr.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-1-adr.out
```

Then run CodeRabbit for this milestone:

```sh
coderabbit review --agent
```

If CodeRabbit reports actionable documentation or correctness concerns, fix
them and rerun the deterministic gates before asking CodeRabbit again. If the
command is unavailable or cannot authenticate, record that in
`Surprises & Discoveries` and continue only if deterministic gates pass.

Commit the ADR with a file-based commit message:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Record evidence-store policy ADR

Accept the joint SQLite and PostgreSQL evidence-store policy for
roadmap item 1.1.1, including Diesel-backed lockstep migrations,
backup expectations, and the first implementation test matrix.
ENDOFMSG
git add docs/adr-013-evidence-store-engine-and-migration-policy.md
git commit -F "$COMMIT_MSG_DIR/COMMIT_MSG.md"
rm -rf "$COMMIT_MSG_DIR"
```

### Milestone 2: Align source documents with the ADR

Update `docs/memoryd-design.md` so it no longer says the evidence schema may
use SQLite, libSQL, or PostgreSQL as an unresolved implementation choice.
Instead, state that the logical evidence schema is implemented through paired
SQLite and PostgreSQL stores, with SQLite as the local default and PostgreSQL
as the enterprise-ready deployment path. Reference ADR 013 from the evidence
schema, tenant isolation, purge backup, configuration, and open decisions
sections as appropriate. Remove the now-resolved open decision
`Choose the default evidence-store engine and migration format`.

Update `docs/terms-of-reference.md` so the open question about the standalone
evidence inbox store records the selected answer. Keep any still-open tenant
storage strategy questions if they extend beyond the evidence store, but do not
leave the default evidence store undecided.

Update `docs/rfcs/0001-standalone-evidence-inbox.md` so its open question about
SQLite, libSQL, PostgreSQL, or a supported set is resolved by ADR 013. The RFC
should either remove that bullet from `Open questions` or replace it with a
sentence in the compatibility section that says ADR 013 selects the joint
SQLite/PostgreSQL policy.

Update `docs/contents.md` to add ADR 013 to the design records list.

Update `docs/roadmap.md` only to align task text with the now-settled decision.
Do not mark item 1.1.1 done yet in this milestone unless all acceptance
criteria are complete.

Run the required gates sequentially:

```sh
set -o pipefail
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-1-docs.out
make typecheck 2>&1 | tee /tmp/typecheck-memoryd-1-1-1-docs.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-1-docs.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-1-docs.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-1-docs.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-1-docs.out
```

Run CodeRabbit after the deterministic gates pass:

```sh
coderabbit review --agent
```

Clear applicable concerns. Then commit the cross-document alignment:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Align storage policy documentation

Update the design, terms, RFC, contents, and roadmap references so
the accepted evidence-store policy is discoverable from every source
document that previously carried the open decision.
ENDOFMSG
git add docs/memoryd-design.md docs/terms-of-reference.md \
  docs/rfcs/0001-standalone-evidence-inbox.md docs/contents.md \
  docs/roadmap.md
git commit -F "$COMMIT_MSG_DIR/COMMIT_MSG.md"
rm -rf "$COMMIT_MSG_DIR"
```

### Milestone 3: Mark roadmap item 1.1.1 done

Verify that the accepted ADR exists, every source document links or names it,
and no source document still frames the evidence-store engine as unresolved.
Then update `docs/roadmap.md` to mark item 1.1.1 done.

Run the full gate set again:

```sh
set -o pipefail
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-1-final.out
make typecheck 2>&1 | tee /tmp/typecheck-memoryd-1-1-1-final.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-1-final.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-1-final.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-1-final.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-1-final.out
```

Run the final CodeRabbit review:

```sh
coderabbit review --agent
```

Clear applicable concerns, then commit the roadmap completion:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Mark evidence-store policy task complete

Mark roadmap item 1.1.1 done after the accepted ADR and supporting
documentation close the evidence-store engine and migration policy
decision.
ENDOFMSG
git add docs/roadmap.md
git commit -F "$COMMIT_MSG_DIR/COMMIT_MSG.md"
rm -rf "$COMMIT_MSG_DIR"
```

## Validation strategy

This task is documentation-first, but the required gates still include Rust
formatting, type-checking, linting, and tests. Run them after each major
milestone exactly as shown above. The final successful run should include:

```plaintext
make check-fmt
make typecheck
make lint
make test
make markdownlint
make nixie
```

For the future evidence inbox implementation, the ADR must require this first
slice test matrix:

- Unit tests with `rstest` for store-engine policy parsing, migration manifest
  validation, backup-policy rules, and semantic errors.
- Behavioural tests with `rstest-bdd` where a scenario describes applying
  evidence-store migrations and rejecting drift or missing backend parity.
- SQLite integration tests that run by default and validate schema versioning,
  idempotent re-runs, tenant-scoped uniqueness, and rollback expectations.
- PostgreSQL integration tests that use `POSTGRES_TEST_URL` when provided and
  otherwise `pg_embedded_setup_unpriv`; configured external PostgreSQL failures
  must fail closed, while missing optional local embedded support may produce
  an explicit skip.
- Cross-backend contract tests that prove equivalent repository behaviour for
  happy paths, duplicate idempotency keys, missing tenant context, cross-tenant
  access attempts, migration checksum drift, and unsupported downgrade attempts.
- Property tests with `proptest` only if implementation introduces an invariant
  over generated migration manifests, schema versions, tenant scopes, or
  idempotency keys.
- Kani or Verus only if implementation introduces bounded state-machine or
  proof-worthy contractual logic. The documentation-only ADR should not add
  formal verification work by itself.

## Progress

- [x] 2026-05-26: Loaded the requested `leta`, `rust-router`, and
  `hexagonal-architecture` skills; created a Leta workspace for the repository.
- [x] 2026-05-26: Renamed the local branch to
  `1-1-1-record-evidence-store-engine-and-migration-policy`.
- [x] 2026-05-26: Created context pack `memoryd-1-1-1-plan` for team planning
  context and launched a Wyvern agent team for documentation and repository
  shape reconnaissance.
- [x] 2026-05-26: Used Firecrawl to check Diesel migration documentation and
  mxd prior art, including Diesel support for SQLite/PostgreSQL and
  `embed_migrations!`.
- [x] 2026-05-26: Drafted this pre-implementation ExecPlan.
- [x] 2026-05-26: Received explicit user approval to implement this ExecPlan.
- [x] 2026-05-26: Milestone 1: wrote ADR 013, passed `make markdownlint`
  and `make nixie`, and cleared CodeRabbit review with zero findings.
- [x] 2026-05-26: Milestone 2: aligned source documents with ADR 013,
  passed `make check-fmt`, `make typecheck`, `make lint`, `make test`,
  `make markdownlint`, and `make nixie`, and cleared CodeRabbit review with
  zero findings.
- [x] 2026-05-26: Milestone 3: verified the evidence-store engine question is
  no longer open in source documents, marked roadmap item 1.1.1 done, passed
  the full final gate set, and cleared final CodeRabbit review with zero
  findings.

## Surprises & Discoveries

- The current repository is mostly documentation and scaffold code:
  `src/main.rs` is a temporary binary stub and `tests/stub.rs` is a disposable
  generated-template test.
- The Wyvern agents could not read the context pack directly in their
  environments, but both completed useful local reconnaissance from repository
  files.
- `mxd` uses parallel `migrations/sqlite` and `migrations/postgres` trees with
  matching numeric versions, backend-selected `embed_migrations!`, and
  PostgreSQL test helpers that prefer `POSTGRES_TEST_URL` but can bootstrap
  embedded PostgreSQL with `pg_embedded_setup_unpriv`.
- Diesel's `embed_migrations!` documentation notes that migration files are
  read at compile time and require a `build.rs` rerun hint if changed
  migrations should force a rebuild.
- `make fmt` is not a usable milestone gate for this documentation slice:
  `markdownlint --fix` reports pre-existing line-length findings across
  unrelated Markdown files and touched files outside the expected scope. The
  unrelated formatter side effects were discarded; deterministic validation is
  continuing with the explicit gates required by this ExecPlan.

## Decision Log

- 2026-05-26: Keep this branch as a pre-implementation plan branch. Rationale:
  the user explicitly stated that the plan must be approved before
  implementation.
- 2026-05-26: Treat roadmap completion as a final implementation milestone,
  not part of this draft plan. Rationale: item 1.1.1 is not complete until the
  accepted ADR and cross-document updates exist.
- 2026-05-26: Plan ADR 013 as the target decision record. Rationale: ADR 012
  is the current newest ADR, and the documentation style guide requires
  sequential ADR filenames in `docs/`.
- 2026-05-26: Follow `mxd`'s paired migration-tree policy rather than a single
  lowest-common-denominator SQL tree. Rationale: evidence-store schema is
  likely to need PostgreSQL-specific enforcement such as RLS while SQLite must
  remain a first-class local path.
- 2026-05-26: Do not add users-guide changes to the expected implementation
  file set unless the ADR introduces user-visible behaviour. Rationale: a
  decision record and roadmap alignment do not by themselves change how users
  operate the server.
- 2026-05-26: Move the ExecPlan to `IN PROGRESS` after explicit approval.
  Rationale: the approval gate is now satisfied, so milestone implementation
  can proceed within the documented tolerances.
- 2026-05-26: Keep the first milestone to ADR 013 plus the living ExecPlan
  update. Rationale: CodeRabbit review confirmed the decision record itself is
  clean, and source-document alignment is deliberately isolated in Milestone 2.
- 2026-05-26: Keep the tenant storage strategy open after resolving the
  evidence-store engine question. Rationale: ADR 013 settles the evidence
  store, but Qdrant, Oxigraph, Chutoro, local, Corbusier, and hosted-ready
  isolation still need a broader deployment decision.

## Outcomes & Retrospective

Implemented as a documentation-only contract slice. ADR 013 now records the
accepted joint SQLite/PostgreSQL evidence-store policy, Diesel-backed adapter
expectation, lockstep migration rule, backup expectation, and first-slice test
matrix. The design document, terms of reference, RFC 0001, contents index, and
roadmap now point at that accepted decision instead of treating the
evidence-store engine as unresolved.

Validation passed for each committed milestone. The final successful gate set
was `make check-fmt`, `make typecheck`, `make lint`, `make test`,
`make markdownlint`, and `make nixie`. CodeRabbit reviewed the ADR milestone,
the source-document alignment milestone, and the final roadmap-completion
milestone; all final reviews completed with zero findings.

The only deviation from the ideal workflow was `make fmt`: it is not currently
usable as a scoped documentation formatter because its Markdown fixer reports
pre-existing line-length findings and touches unrelated files. Those formatter
side effects were discarded, and the explicit required validation gates passed.
