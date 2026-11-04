# Architectural decision record (ADR) 006: Tenant isolation and Corbusier context

## Status

Accepted on 2026-11-03. Tenant isolation is a first-class Memoryd boundary. The
normative identity is the authenticated pair `(tenant_id, workspace_id)`; a
`RequestContext` is resolved at the driving boundary and propagated immutably;
canonical payload scope is provenance and must agree with that context; and
each driven adapter enforces the same contract with the strongest mechanism its
store offers. Enforcement is scheduled for roadmap items 1.2.5, 1.2.8, 2.1.1,
2.2.2, 2.2.4, 3.1.1, 5.1.1, 6.1.1, and 6.1.2; this ADR records the contract,
not an implementation.

## Date

2026-11-03.

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

### Authority resolution and propagation

A driving boundary resolves the complete context before any application logic
runs: tenant, principal or service identity, session, correlation, optional
causation, and the authorized workspace scope. The configured local tenant is
selected only when the deployment is explicitly in local mode. A missing
Corbusier credential in Corbusier mode is an authentication failure; it is not
a reason to fall back to the local tenant.

The resolved context is immutable for the life of the operation. Worker jobs
carry it through retries, storage calls, and audit events; a retry does not
re-resolve authority from the retried payload. Table 2 separates what may
establish authority from what may only describe or narrow it.

| Input                                               | Resolved at                    | Permitted effect                                                    | Not permitted                                                 |
| --------------------------------------------------- | ------------------------------ | ------------------------------------------------------------------- | ------------------------------------------------------------- |
| Authenticated capability token or Corbusier context | Driving adapter                | Establish tenant, principal or service identity, session, and scope | Be replaced by payload fields                                 |
| Explicit local mode configuration                   | Composition root               | Select the configured local tenant                                  | Act as a fallback when Corbusier credentials are missing      |
| Canonical delta scope fields                        | Application command validation | Describe provenance and confirm agreement with the resolved context | Establish or widen authority; a mismatch is a typed rejection |
| Caller workspace filter or allowed-workspace hint   | Driving adapter and use case   | Narrow within the authoritatively permitted set                     | Broaden the authenticated workspace scope                     |
| Scheduled-job record                                | Job runner                     | Carry the context resolved when the job was created                 | Acquire a fresh tenant or workspace from job arguments        |

_Table 2: Authority inputs and their permitted effects._

Canonical scope fields describe provenance. They must agree with the resolved
context; a disagreement is a typed rejection and never an instruction to
replace the context. Allowed-workspace hints supplied in configuration or
payloads cannot broaden authenticated permissions.

### Worked authority examples

Table 3 works the boundary through accept and reject cases. Tenants A and B
reuse the same workspace label and the same provider identifiers deliberately
because identifier reuse is the failure this boundary must survive.

| Case                          | Resolved context                       | Payload or request claim                    | Outcome                                                           |
| ----------------------------- | -------------------------------------- | ------------------------------------------- | ----------------------------------------------------------------- |
| Trusted local                 | Explicit local mode, tenant L          | Workspace W under tenant L                  | Accepted; tenant L owns the evidence                              |
| Trusted Corbusier             | Token for tenant A, workspace W        | Delta scope tenant A, workspace W           | Accepted                                                          |
| Missing Corbusier context     | None; Corbusier mode                   | Delta scope tenant A, workspace W           | Denied before the use case; no fallback to tenant L               |
| Forged tenant                 | Token for tenant A, workspace W        | Delta scope tenant B, workspace W           | Typed scope rejection; no acceptance effects                      |
| Widened workspace             | Token permits W only                   | Filter or hint names W2                     | Denied; hints cannot broaden the permitted set                    |
| Same label, different tenants | Tenant A accepts W; tenant B accepts W | Identical workspace label and provider keys | Both accepted in separate partitions; neither read sees the other |
| Token substitution            | Push token, collected batch            | Collector batch identity                    | Denied; a push token is not collector authority                   |

_Table 3: Worked accept/reject authority examples._

A seeded implementation that trusts the payload tenant fails the forged-tenant
case. An implementation that maps a missing Corbusier context onto the local
tenant fails the missing-context case. Both are required negative controls for
the later executable tests.

### Deferred store strategy

This decision ratifies isolation invariants, not a fixed storage layout.
Database, vector, graph, and clustering enforcement mechanisms stay in their
adapters. Only the SQL evidence-store contract gates the first implementation.
The specific Qdrant collection layout and the Oxigraph and Chutoro operational
strategies are deferred to their owning delivery slices — roadmap 3.1, 5.1, and
6.1 respectively — and are not accepted by implication here. Their isolation
requirements remain binding when those slices implement them.

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
- A missing authenticated context in Corbusier mode fails the request. The
  local default is a deliberate deployment mode, not an error-recovery path.
- Payload scope fields are evidence about the source, not authority. An
  implementation that trusts a payload tenant is non-conforming even if it
  happens to pass a happy-path test.
- Deferring the Qdrant, Oxigraph, and Chutoro layouts keeps this decision
  stable while their owning slices learn operational requirements. It does not
  relax the isolation invariant those layouts must satisfy.

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
