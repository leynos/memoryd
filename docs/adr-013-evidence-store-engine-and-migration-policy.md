# Architectural decision record (ADR) 013: Evidence-store engine and migration policy

## Status

Accepted on 2026-05-26. Memoryd v1 uses a joint SQLite and PostgreSQL
evidence-store policy through Diesel, with SQLite as the local default,
PostgreSQL as a first-class deployment path, and lockstep migrations for both
stores.

## Date

2026-05-26.

## Context and problem statement

The evidence inbox is Memoryd's durable boundary for provider transcripts,
manual memory writes, projection repair, purge, and later recall explanation.
Before that inbox can be implemented, the project needs one storage decision
that settles the default local store, the enterprise-ready store, migration
shape, backup expectations, and first implementation test matrix.

The terms of reference still asks whether the standalone evidence inbox should
use SQLite, libSQL, PostgreSQL, or a supported set. RFC 0001 asks the same
question for the default evidence inbox. The design document also lists the
evidence-store engine and migration format as an open design decision. This ADR
closes that decision for v1, so later slices can implement evidence capture
without reworking the storage boundary.

## Decision drivers

- Local users need a quick-start deployment that does not require operating a
  database server.
- Hosted, enterprise, and Corbusier-adjacent deployments need a credible
  PostgreSQL path with stronger operational and isolation controls.
- The project needs one relational abstraction that can support SQLite and
  PostgreSQL without leaking database-specific types into domain contracts.
- Store parity matters because evidence is authoritative; backend differences
  must not change tenant scoping, idempotency, retention, purge, or recall
  repair semantics.
- Migration history must be reviewable and reproducible across both stores.
- PostgreSQL row-level security (RLS) is useful for hosted isolation, but the
  domain must not depend on PostgreSQL-only enforcement.
- libSQL and Turso remain interesting, but their Diesel support and operational
  surface are still too unsettled for the v1 evidence-store contract.

## Requirements

### Functional requirements

- SQLite is the default local evidence store for release 0.1 and v1 local
  daemon deployments.
- PostgreSQL is a first-class supported evidence-store deployment path for v1.
- Both stores preserve the same logical evidence inbox schema and tenant
  scoping contract.
- Evidence-store migrations are applied before the daemon accepts writes that
  depend on those schema versions.
- Operators are expected to take explicit evidence-store backups before
  irreversible purge operations and before production schema migrations until
  automatic backup tooling is implemented.
- The first evidence-inbox implementation slice must define and run a
  cross-backend test matrix for migration parity, repository behaviour, tenant
  scoping, idempotency, and downgrade or drift failures.

### Technical requirements

- Diesel is the relational storage abstraction for SQLite and PostgreSQL
  evidence-store adapters.
- Migration files live in paired backend trees, modelled after `mxd`:
  `migrations/sqlite/<version>_<name>/` and
  `migrations/postgres/<version>_<name>/`.
- Every migration version must exist in both backend trees with the same
  numeric version and semantic name.
- Each backend migration may use backend-specific SQL, but the resulting
  logical schema contract must remain equivalent.
- SQLite migrations are required in the default test path.
- PostgreSQL migration and repository tests must use `POSTGRES_TEST_URL` when
  it is configured, and otherwise use `pg_embedded_setup_unpriv` test support
  where local prerequisites are available.
- Configured external PostgreSQL failures fail closed. Missing optional local
  embedded PostgreSQL support may produce an explicit skip only in tests that
  are documented as local-prerequisite-dependent.
- Domain and application ports use Memoryd domain types and semantic errors.
  Diesel, SQL connection types, migration harnesses, and
  `pg_embedded_setup_unpriv` stay in adapters and test support.

## Options considered

| Option                                  | Consequence                                                                                                                                        |
| --------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| SQLite only                             | Keeps local setup simple, but leaves no credible enterprise-ready path and weakens hosted tenant-isolation options.                                |
| PostgreSQL only                         | Gives strong operational controls, but makes local single-user installation too heavy for the first useful Memoryd deployment.                     |
| SQLite and PostgreSQL through Diesel    | Preserves local quick start and enterprise readiness while keeping relational repository contracts behind ports and paired migration discipline.   |
| libSQL or Turso as the primary v1 store | Could support local and remote modes later, but Diesel support and operational maturity are not stable enough to make it the v1 evidence contract. |

_Table 1: Evidence-store engine options._

## Decision outcome / proposed direction

Use a joint SQLite and PostgreSQL evidence-store policy for v1.

SQLite is the default local store. It supports the minimum useful Memoryd
deployment: one daemon, local evidence, Qdrant as a serving index, Ollama for
local embeddings, curated memory writes, and `flat_v1` recall. SQLite adapters
must still carry tenant identifiers and enforce tenant predicates in queries
and contract tests.

PostgreSQL is a first-class supported deployment path. PostgreSQL adapters may
add database-enforced isolation with non-owner application roles,
transaction-local tenant settings, and RLS on tenant-owned tables. Those
controls strengthen the adapter, but the domain and application layers remain
backend-neutral and continue to require explicit tenant context.

Diesel is the relational abstraction for both stores. The project will follow
the `mxd` migration shape: separate SQLite and PostgreSQL migration trees with
matching versions and semantic names. Schema drift between stores is a test
failure. Backend-specific SQL is allowed only when it preserves the same
logical evidence-store contract.

The first evidence-inbox implementation slice must add migration and repository
contract tests that prove both stores behave equivalently for tenant scoping,
idempotency, happy-path writes and reads, duplicate events, cross-tenant access
attempts, missing tenant context, unsupported downgrade attempts, and migration
checksum or manifest drift.

## Goals and non-goals

Goals:

- Close the storage-engine and migration-format decision that blocks evidence
  inbox implementation.
- Preserve a low-friction local deployment through SQLite.
- Preserve a credible PostgreSQL path for hosted and enterprise deployments.
- Keep store-specific mechanics outside the domain and application layers.
- Require migration parity before evidence rows become authoritative.

Non-goals:

- Implement Diesel dependencies, migrations, repositories, or configuration in
  this ADR.
- Define the complete evidence inbox schema. RFC 0001 and roadmap item 2.1.1
  own that implementation detail.
- Select libSQL, Turso, MySQL, or another relational engine for v1.
- Implement automatic backup or restore tooling.
- Decide all tenant storage strategies for Qdrant, Oxigraph, Chutoro, and
  future projections.

## Migration plan

1. This ADR records the accepted storage and migration policy.
2. Roadmap item 2.1.1 implements the evidence inbox migrations and repository
   APIs using paired SQLite and PostgreSQL migration trees.
3. The implementation slice adds a migration runner using Diesel's migration
   infrastructure. If migrations are embedded in Rust binaries, the crate must
   ensure migration-directory changes trigger rebuilds.
4. SQLite migration tests become part of the always-run test path.
5. PostgreSQL migration tests use `POSTGRES_TEST_URL` where configured and
   otherwise the local `pg_embedded_setup_unpriv` test path described in
   `docs/pg-embed-setup-unpriv-users-guide.md`.
6. Later deployment and backup work may add automatic backup tooling. Until
   then, operator documentation must treat pre-migration and pre-purge backups
   as explicit operational steps.

## Known risks and limitations

- SQLite and PostgreSQL use different SQL dialects. The mitigation is paired
  migrations plus contract tests against both stores rather than a false claim
  that one SQL file can cover every future schema change.
- PostgreSQL RLS has no SQLite equivalent. The mitigation is to keep tenant
  isolation as an application and domain contract, then add RLS as an adapter
  hardening layer for PostgreSQL.
- Supporting two stores increases test cost. The mitigation is to keep SQLite
  in the default path and use `POSTGRES_TEST_URL` or embedded PostgreSQL for
  the PostgreSQL path.
- Automatic backup tooling is not part of this decision. Until it exists,
  production operators must make explicit backups before migrations and purge.
- libSQL and Turso could become attractive later. A future ADR may add them if
  Diesel support, migration tooling, and operational behaviour are stable
  enough to preserve the evidence-store contract.

## Consequences

- Future evidence-store migrations must land as paired SQLite and PostgreSQL
  migrations.
- Future repository tests must prove backend parity instead of testing only
  the default SQLite path.
- Storage adapters may use backend-specific enforcement, but they must expose
  the same domain-facing behaviour through Memoryd ports.
- Documentation and roadmap items that previously treated the evidence store
  as undecided must now reference this ADR.
- Backup and purge documentation must avoid promising automatic recovery until
  the project implements backup tooling.
- CI may need additional jobs or feature combinations once PostgreSQL support
  is implemented.

## References

- `docs/roadmap.md`, roadmap item 1.1.1 and the evidence-inbox implementation
  slice.
- `docs/terms-of-reference.md`, section 9 open questions.
- `docs/memoryd-design.md`, sections 2, 7, 13, 14, and 17.
- `docs/rfcs/0001-standalone-evidence-inbox.md`, compatibility and migration.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/pg-embed-setup-unpriv-users-guide.md`.
- `docs/rstest-bdd-users-guide.md`.
- `mxd` commit `153671c2c8794b8fb122545afdbea1df8e35cbc1`.
- Diesel getting-started guide, <https://diesel.rs/guides/getting-started/>.
- `diesel_migrations::embed_migrations!` documentation,
  <https://docs.rs/diesel_migrations/latest/diesel_migrations/macro.embed_migrations.html>.
