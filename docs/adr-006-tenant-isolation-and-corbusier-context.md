# Architectural decision record (ADR) 006: Tenant isolation and Corbusier context

## Status

Proposed.

## Date

2026-05-25.

## Context and problem statement

Corbusier is expected to use `memoryd` and already models tenant identity as a
cross-cutting request concern. Its `RequestContext` carries `tenant_id`,
correlation, causation, user, and session identifiers, and tenant-owned
repository operations receive that context explicitly. Corbusier's roadmap also
plans PostgreSQL row-level security (RLS) with a transaction-local tenant
setting and tenant-scoped schema constraints.

The first standalone `memoryd` design scopes memory by workspace. That is
necessary but not sufficient for Corbusier. A workspace identifies a project or
repository area; a tenant identifies the authority boundary that decides which
caller may read, write, project, recall, or purge that workspace. Without a
tenant boundary, two Corbusier tenants could collide on repository-derived
workspace identifiers, share vector index payloads, leak graph facts, or
receive each other's recalled evidence.

Prior art points to a layered approach. PostgreSQL RLS centralizes row
filtering and modification rules in the database, with default-deny behaviour
when RLS is enabled but no policy exists. The `current_setting(...)` pattern
lets applications bind tenant context per transaction rather than create one
database role per tenant. Qdrant recommends shared collections with mandatory
payload filters and tenant keyword indexes for high-cardinality multitenancy,
while still allowing dedicated collections or shards where operationally
appropriate. OWASP's multitenant guidance emphasizes establishing tenant
context early, binding it to authenticated identity, propagating it through all
layers, validating resource ownership, and auditing tenant-aware access.

## Decision drivers

- Corbusier compatibility requires `memoryd` to accept a tenant context rather
  than infer all isolation from local workspace paths.
- Tenant isolation must be visible in domain and application APIs, not hidden
  in SQL adapter conventions.
- Local single-user deployments must keep a low-friction default.
- PostgreSQL deployments must not rely solely on every query remembering a
  `WHERE tenant_id = ...` clause.
- Qdrant, Oxigraph, and Chutoro state must be scoped so recall, purge, repair,
  and rebuild operations cannot cross tenant boundaries.
- The decision must preserve the hexagonal architecture boundary: tenant
  policy belongs in domain/application contracts; enforcement details belong in
  adapters.

## Options considered

| Option                                     | Consequence                                                                                                                                           |
| ------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Workspace-only isolation                   | Simple for local use, but incompatible with Corbusier and unsafe for tenants sharing repository names, paths, or imported evidence formats.           |
| Application filters only                   | Works for SQLite and early tests, but leaves hosted or PostgreSQL deployments vulnerable to one missed predicate.                                     |
| Dedicated database and indexes per tenant  | Strong isolation, but adds operational cost and slows local-first adoption.                                                                           |
| Tenant request context plus adapter guards | Keeps tenant scope explicit in use cases, supports local defaults, and lets each adapter enforce the same contract with the strongest available tool. |

_Table 1: Tenant-isolation options._

## Decision outcome / proposed direction

Adopt Corbusier-compatible tenant isolation as a first-class `memoryd` boundary.

The domain owns tenant-aware identity types:

- `TenantId`;
- `PrincipalId` or `UserId`;
- `SessionId`;
- `CorrelationId`;
- `CausationId`;
- `RequestContext`.

Every application use case and every domain-owned driven port that touches
tenant-owned state receives a `RequestContext` or a command that embeds one.
Provider adapters, MCP tools, daemon RPC handlers, scheduled jobs, and repair
commands must derive the context before invoking application services. Tenant
identity is taken from an authenticated capability token, Corbusier request
context, or configured local default, never from an untrusted request field by
itself.

Workspace identity becomes tenant-scoped. The normative identity boundary is
`(tenant_id, workspace_id)`, not `workspace_id` alone. Local single-user mode
uses a stable default local tenant so the existing local-first workflow remains
simple while exercising the same code paths as Corbusier mode.

Repository-derived workspace IDs combine normalized Git origin URL, a hash of
the local repository root path, and optional configured profile name. Non-Git
workspaces combine the canonical configured root path hash and optional profile
name. Operators may provide explicit aliases or overrides. If two derived
workspaces collide inside one tenant, ingestion fails with an auditable
collision diagnostic instead of merging evidence.

Persistence adapters enforce the boundary as follows:

- SQLite or libSQL uses tenant columns, composite keys, application-level
  predicates, and two-tenant contract tests.
- PostgreSQL uses the same application-level predicates plus RLS policies based
  on a transaction-local setting such as `memoryd.tenant_id`.
- PostgreSQL application roles must not own tenant tables and must not have
  `BYPASSRLS`.
- Tenant-owned unique constraints include `tenant_id` where collisions are
  possible, including idempotency keys, source-session identities, and
  workspace-scoped names.

Qdrant adapters must inject tenant and workspace filters from the request
context for every upsert, search, delete, and repair operation. The default
local strategy may keep collection-per-tenant-workspace to simplify purge. A
hosted or high-cardinality strategy may use shared collections per embedding
model, but only with mandatory payload fields for `tenant_id` and
`workspace_id`, a keyword tenant payload index, and contract tests that prove
cross-tenant recall and deletion are impossible through the adapter.

Oxigraph adapters use tenant-and-workspace named graphs, for example:

- `urn:memoryd:tenant:{tenant}:ws:{workspace}:facts`;
- `urn:memoryd:tenant:{tenant}:ws:{workspace}:provenance`;
- `urn:memoryd:tenant:{tenant}:ws:{workspace}:retractions`;
- `urn:memoryd:tenant:{tenant}:ws:{workspace}:themes`;
- `urn:memoryd:tenant:{tenant}:ws:{workspace}:temporal`.

Chutoro sessions, checkpoints, and theme managers are scoped to
`(tenant_id, workspace_id)`. No clustering, split, merge, theme summary, or
checkpoint compaction operation may mix semantic carriers from different
tenants.

## Consequences

- The roadmap must add tenant context before evidence capture, provider
  adapters, Qdrant indexing, graph projection, or Chutoro checkpoints become
  durable.
- Tenant isolation joins provenance, idempotency, purge, and architecture
  conformance as a design-level verification target.
- The evidence schema, projection state, graph namespaces, vector payloads,
  audit records, and capability tokens all carry tenant context.
- Corbusier can use `memoryd` through an adapter that maps Corbusier's
  authenticated request context into `memoryd`'s `RequestContext`.
- Local single-user deployments retain a default tenant, but the default is a
  compatibility mode, not a separate tenant-free implementation.
- Fleet-wide tenant administration, billing, analytics, and tenant lifecycle
  management remain deferred product scope. The v1 requirement is isolation and
  compatibility, not a hosted control plane.

## References

- `../corbusier/src/context/request_context.rs`.
- `../corbusier/src/context/ids.rs`.
- `../corbusier/src/message/adapters/postgres/tenant_tx.rs`.
- `../corbusier/docs/roadmap.md`, section 2.5.
- `../corbusier/docs/users-guide.md`, "Tenant context".
- PostgreSQL row security policies, accessed 2026-05-25:
  <https://www.postgresql.org/docs/current/ddl-rowsecurity.html>.
- AWS Database Blog, "Multi-tenant data isolation with PostgreSQL row level
  security", accessed 2026-05-25:
  <https://aws.amazon.com/blogs/database/multi-tenant-data-isolation-with-postgresql-row-level-security/>.
- Qdrant documentation, "Multitenancy", accessed 2026-05-25:
  <https://qdrant.tech/documentation/manage-data/multitenancy/>.
- OWASP Cheat Sheet Series, "Multi Tenant Security Cheat Sheet", accessed
  2026-05-25:
  <https://cheatsheetseries.owasp.org/cheatsheets/Multi_Tenant_Security_Cheat_Sheet.html>.
- W3C RDF 1.1 Concepts and Abstract Syntax, accessed 2026-05-25:
  <https://www.w3.org/TR/rdf11-concepts/>.
