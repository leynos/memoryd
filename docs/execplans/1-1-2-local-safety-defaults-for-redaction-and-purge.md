# Record local safety defaults for workspace, redaction, and purge

This ExecPlan (execution plan) is a living document. The sections `Constraints`,
`Tolerances`, `Risks`, `Progress`, `Surprises & Discoveries`, `Decision Log`,
and `Outcomes & Retrospective` must be kept up to date as work proceeds.

Status: DRAFT

## Purpose / big picture

Roadmap item 1.1.2 must settle Memoryd's local safety defaults before any
provider adapter, evidence inbox, or recall use case is implemented. Three
families of decisions currently sit in the open-decisions table of
`docs/memoryd-design.md` §17 and the open-questions table of
`docs/terms-of-reference.md` §9:

- workspace identity (Git-origin normalization, repository-root path hashing,
  non-Git workspace hashing, collision handling, and operator overrides);
- redaction (first detector classes, deny-pattern behaviour, raw-text storage
  modes); and
- purge (confirmation defaults and pre-purge backup expectations).

Item 1.1.2 turns those decisions into **two** Architectural Decision
Records (ADRs), both written in this slice, and aligns every source
document that previously framed those defaults as open:

- **ADR 014 (Accepted): workspace identity and purge safety.** These
  decisions are firm enough to ratify before any provider adapter exists,
  because they gate workspace derivation (roadmap item 2.2.4) and the
  eventual purge slice (roadmap item 5.3.1) without depending on real
  transcript content.
- **ADR 015 (Proposed): redaction defaults.** The first-release detector
  classes, entropy thresholds, deny-pattern behaviour, and raw-text
  storage mode are written now as a concrete proposal, but they are
  marked `Proposed` rather than `Accepted` because the detector list and
  entropy thresholds should be ratified by people who have read real
  Codex and Claude transcripts. ADR 015 is explicitly revisited and
  ratified in roadmap item 2.2.3 (the redaction pipeline slice).

Splitting the decision into two documents makes the difference in
confidence legible: a reader sees immediately that workspace identity and
purge are settled while the redaction detector set is a recorded proposal
awaiting transcript-informed ratification. Writing both now means the
proposal is captured in full rather than left as a vague forward
reference.

After this plan is implemented, a maintainer can open ADR 014 to see the
settled workspace-identity and purge defaults and ADR 015 to see the
proposed redaction defaults, while the design document, terms of
reference, RFC 0001, contents index, and roadmap reference both records.
Success is observable when roadmap item 1.1.2 is marked done, the
backup-format bullet in `docs/memoryd-design.md` §17 is removed (closed by
ADR 014), the redaction-detector bullet in §17 is reworded to point at the
proposed ADR 015 with ratification scheduled in roadmap item 2.2.3, the
redaction policy row in `docs/terms-of-reference.md` §9 records ADR 015 as
the proposed direction, both ADRs appear in `docs/contents.md`, the
relevant roadmap tasks (2.2.3, 2.2.4, 5.3.1) signpost the decisions they
inherit, and the full documentation gate set passes.

This plan is pre-implementation. Do not execute it until the user explicitly
approves the plan.

## Constraints

- Implement documentation and contract decisions only. Do not add Rust crates,
  modules, configuration parsing, detectors, hashing code, repositories, or
  tests in this slice unless the user explicitly approves a scope expansion.
- Create two ADRs as the authoritative decision records:
  `docs/adr-014-local-safety-defaults-workspace-and-purge.md` (Accepted)
  and `docs/adr-015-redaction-defaults.md` (Proposed). ADR 013 is the
  current newest sequence number, so 014 and 015 are the next two.
- ADR 015 is `Proposed`, not `Accepted`, and is explicitly revisited and
  ratified in roadmap item 2.2.3. Its detector list and entropy thresholds
  are a concrete proposal that the redaction-pipeline slice will confirm
  or revise against real transcripts; the plan must not mark ADR 015
  accepted.
- Preserve the boundary already accepted in ADR 005 (hexagonal architecture),
  ADR 006 (tenant-isolation and workspace identity), and ADR 013 (evidence
  store and migrations). ADRs 014 and 015 refine defaults inside those
  boundaries; they do not reopen them.
- Keep Memoryd hexagonal. The domain owns the vocabulary for workspace
  identity, redaction state, raw-text storage mode, purge confirmation, and
  pre-purge backup expectations. Concrete detector engines, BLAKE3 hashing
  implementations, SQLite `VACUUM INTO`, PostgreSQL `pg_dump`, and Qdrant
  snapshot calls stay in adapters and operator tooling.
- Keep tenant isolation first-class. Every default named in ADR 014 or ADR
  015 must apply inside an authenticated tenant context: workspace
  derivation is scoped by tenant, redaction runs before tenant-owned
  storage, purge confirmation names the tenant and workspace, and pre-purge
  backup expectations are described per tenant scope.
- Bind the workspace identity defaults (ADR 014) to ADR 006's contract:
  repository workspaces use a normalized Git origin URL plus a hash of the
  repository root path plus an optional profile name; non-Git workspaces
  hash the canonical configured root path plus the optional profile name;
  operator overrides remain explicit; in-tenant collisions fail with an
  auditable diagnostic. ADR 014 names the specific normalization rules, the
  hash function, and the truncation length.
- Bind the redaction defaults (ADR 015) to the design document §13
  first-release detector list (API keys, OAuth tokens, JSON Web Tokens
  (JWTs), private keys, SSH material, `.env` content, passwords, cookies,
  cloud credentials, database URLs, and high-entropy blobs) and to the
  `[privacy]` block in §14 (`redact_before_store`, `redact_before_embedding`,
  `store_raw_text`).
- Bind the purge defaults (ADR 014) to the design document §13
  irreversibility statement and to ADR 013 §§"Migration plan" and "Known
  risks and limitations" (no automated backup tooling; operators take
  explicit pre-migration and pre-purge backups).
- Keep the raw-text mode enumeration aligned with `docs/memoryd-design.md` §14
  but tightened to exactly `none | redacted` for v1 (ADR 015). `encrypted`
  is rejected at configuration-parse time and reserved for a future ADR
  with key-management contracts.
- Use en-GB Oxford spelling in documentation while preserving external
  identifiers such as `redact_before_store`, `store_raw_text`,
  `VACUUM INTO`, `pg_dump`, `BLAKE3`, environment variable names, and Cargo
  feature names.
- Follow `AGENTS.md`: run gates sequentially, capture command output with
  `tee`, commit only gated changes, and do not run test, lint, or formatting
  commands in parallel.
- Use the loaded skills as signposts: `leta` for code navigation,
  `rust-router` for any Rust boundary questions that surface in review,
  `hexagonal-architecture` for port/adapter discipline, `execplans` for
  maintaining this document, `firecrawl` for external prior-art checks, and
  `logisphere-design-review` for the pre-implementation community-of-experts
  review.
- Run `coderabbit review --agent` only after deterministic gates for the
  current milestone pass. Clear any applicable concerns before moving to the
  next milestone.
- Do not mark roadmap item 1.1.2 done until both ADRs, all required
  cross-document updates (including the roadmap signposts in items 2.2.3,
  2.2.4, and 5.3.1), and the gate set have all landed.

## Tolerances (exception triggers)

- Scope: if implementation requires production Rust code changes, stop,
  document the reason in `Decision Log`, and ask for approval before editing
  code.
- Scope: if more than nine repository files need changes, stop and ask
  whether to split the work. The expected files are the two new ADRs
  (`docs/adr-014-local-safety-defaults-workspace-and-purge.md` and
  `docs/adr-015-redaction-defaults.md`) plus `docs/roadmap.md`,
  `docs/terms-of-reference.md`, `docs/memoryd-design.md`,
  `docs/rfcs/0001-standalone-evidence-inbox.md`, `docs/contents.md`,
  `docs/developers-guide.md`, and `docs/users-guide.md`.
- Interface: if the plan appears to require a public command-line interface,
  MCP tool, internal RPC method, or configuration key beyond the
  already-documented `[privacy]` block in `docs/memoryd-design.md` §14, stop
  and defer that interface to the implementation slice that first uses it.
- Architecture: if any wording forces the domain to depend on a specific
  detector library, hashing crate, snapshot command, or filesystem layout,
  stop and rewrite the wording so that domain code names the contract while
  adapters select the implementation.
- Algorithm: if the workspace identity algorithm conflicts with ADR 006 or
  forces a Git-only model, stop and rewrite to keep non-Git workspaces
  first-class.
- Detector set: if the first detector list cannot be implemented behind a
  port without leaking detector-engine types into the domain, stop and either
  rewrite the list or move the offending category to a later ADR.
- Validation: if `make check-fmt`, `make typecheck`, `make lint`,
  `make test`, `make markdownlint`, or `make nixie` fails twice after focused
  repair, stop and record the blocker with the failing log path.
- Review: if `coderabbit review --agent` is unavailable because of local
  tooling or authentication, record the command and failure in
  `Surprises & Discoveries`, then continue only after deterministic gates
  have passed.

## Risks

- Risk: ADR 015 could over-commit to a specific detector library and force
  the implementation slice to ship that library. Severity: medium.
  Likelihood: medium. Mitigation: ADR 015 names the detector classes and the
  community-standard starting entropy thresholds, but keeps the concrete
  engine (Gitleaks, detect-secrets, TruffleHog, or a Memoryd-internal regex
  set) selectable at the adapter boundary, and is `Proposed` so roadmap item
  2.2.3 can revise it.
- Risk: the entropy thresholds inherited from credential scanning misfire on
  Codex and Claude transcripts (file hashes, UUIDs, lockfile content, and
  base64 tool attachments inflate false positives). Severity: medium.
  Likelihood: medium. Mitigation: ADR 015 records `b64 ≥ 4.5` / `hex ≥ 3.0`
  / length `≥ 20` as starting defaults and schedules tuning against observed
  false-positive rates in roadmap item 2.2.3. This is why ADR 015 stays
  `Proposed`.
- Risk: workspace identity defaults could lock in a Git-shaped model that
  breaks for monorepos, worktrees, or non-Git imports. Severity: high.
  Likelihood: medium. Mitigation: ADR 014 separates the Git-origin
  normalization rules from the non-Git canonical root path rules, keeps
  operator overrides first-class, keeps the canonical root path
  symlink-stable, and requires a registration-time collision diagnostic
  rather than automatic merge.
- Risk: purge confirmation could read as policy theatre and still permit a
  fat-finger or clipboard-paste destructive call. Severity: high.
  Likelihood: medium. Mitigation: ADR 014 requires a typed-name match
  against `{tenant}/{workspace}` plus a small per-invocation randomised
  challenge derived from the resolved scope, refuses tenant-wide purge on
  the MCP and RPC surfaces entirely, and gates tenant-wide purge behind the
  operator CLI plus `--force-tenant-scope`.
- Risk: pre-purge backup expectations could read as a promise to ship backup
  tooling. Severity: medium. Likelihood: medium. Mitigation: ADR 014 phrases
  backup expectations as documented operator procedures (SQLite
  `VACUUM INTO`, PostgreSQL `pg_dump`, Qdrant snapshot API), records audited
  `(collection, snapshot_id)` pairs, and keeps automated tooling out of v1
  in line with ADR 013.
- Risk: raw-text storage mode could be silently weakened by a future ADR.
  Severity: low. Likelihood: medium. Mitigation: ADR 015 enumerates the v1
  enum as exactly `none | redacted`, selects `redacted` as default, and
  rejects `encrypted` at configuration-parse time pending a follow-up ADR
  with key-management contracts.
- Risk: `make lint` and `make test` may be slow for a documentation-only
  change. Severity: low. Likelihood: medium. Mitigation: run them anyway
  because the task explicitly requests these gates after major milestones.
- Risk: detector class list could imply unsupported guarantees about secret
  recovery rate. Severity: medium. Likelihood: low. Mitigation: ADR 015
  states that the first release detects the listed classes on a best-effort
  basis with the community-standard entropy thresholds, while accepting that
  adversarial inputs can still evade pattern-based redaction.
- Risk: per-tenant deny patterns and per-collection Qdrant snapshots could
  be costly at scale (a tenant with many workspaces or large collections).
  Severity: medium. Likelihood: low. Mitigation: ADR 014 allows an explicit
  `--skip-snapshot --collection-empty` acknowledgement for verified-empty
  collections and ADR 015 fixes deny-pattern syntax to `globset` so adapters
  share one matcher rather than divergent interpreters.

## Relevant documentation and skills

The implementer must read these local documents before editing:

- `AGENTS.md`, for branch, gate, commit, and test execution rules.
- `docs/roadmap.md`, especially roadmap item 1.1.2, the dependent items
  1.1.3-1.1.5, and the evidence-capture slice 2.2.3-2.2.4 that will rely on
  these defaults.
- `docs/terms-of-reference.md`, especially §§7.2 (operational success), 8.1
  (hard constraints), 8.2 (workspace identity assumption), 8.3
  (dependencies), 9 (open questions table), and Appendix B (ADR candidates).
- `docs/memoryd-design.md`, especially §5.4 (tenant isolation and workspace
  derivation), §7 (canonical evidence model), §8.4 (Qdrant layout and
  purge), §13 (security and privacy), §14 (configuration `[privacy]` block),
  and §17 (open design decisions).
- `docs/rfcs/0001-standalone-evidence-inbox.md`, especially §3 (provider
  adapters), §7 (compatibility and migration), and any open-questions list
  still referring to the evidence-store default.
- `docs/adr-005-hexagonal-architecture-boundary.md`,
  `docs/adr-006-tenant-isolation-and-corbusier-context.md`,
  `docs/adr-008-source-health-and-coverage-foundation.md`, and
  `docs/adr-013-evidence-store-engine-and-migration-policy.md`.
- `docs/documentation-style-guide.md`, especially the ADR section that
  records the required ADR section ordering and the en-GB Oxford spelling
  rules.
- `docs/developers-guide.md`, especially the authentication-and-authorization
  section that already names `memory.purge` capability gating and the
  explicit confirmation string requirement.
- `docs/contents.md`, `docs/users-guide.md`,
  `docs/pg-embed-setup-unpriv-users-guide.md`, and
  `docs/rstest-bdd-users-guide.md`.

The implementer must use these skills as working rules:

- `execplans`: keep this document self-contained and update the living
  sections as work proceeds.
- `leta`: use `leta files`, `leta grep`, `leta refs`, and `leta show` for any
  navigation that touches symbols. The repository is currently almost
  entirely documentation plus a `src/main.rs` stub and a `tests/stub.rs`
  template; production prior art does not exist for these defaults.
- `rust-router`: route any incidental Rust shape question to the smallest
  relevant Rust skill before changing code.
- `hexagonal-architecture`: keep domain vocabulary, ports, and adapter
  responsibilities separated. The domain names what is detected, hashed,
  confirmed, and backed up; adapters and operator procedures select the
  engine, library, command, or filesystem layout that implements those
  contracts.
- `firecrawl`: external prior art may be re-checked when the
  community-of-experts review (`logisphere-design-review`) raises gaps about
  the cited detector lists, hash construction, or backup tooling.
- `logisphere-design-review`: the multi-agent design review has already been
  run against this plan; its eight must-fix concerns, three unresolved-risk
  signposts, and four improvements were applied before the plan was
  delivered for approval (see `Decision Log`). Re-run it only if a milestone
  materially changes a default.

## Prior art and external references

Local prior art lives entirely in the existing documentation set. ADR 006
already states that repository workspace IDs combine the normalized Git
origin URL, a hash of the local repository root path, and an optional
configured profile name, with explicit operator overrides and auditable
in-tenant collision diagnostics. ADR 013 already states that the evidence
store ships in joint SQLite/PostgreSQL form through Diesel with lockstep
migrations and that operators take explicit pre-migration and pre-purge
backups until automated backup tooling exists. ADR 005 keeps detector
engines, hashing libraries, snapshot commands, and configuration parsing in
adapters. The repository currently has no production Rust code that touches
workspace identity, redaction, hashing, or purge; the prior-art-checking
agent confirmed the source tree is the `src/main.rs` stub and the
`tests/stub.rs` template only.

External prior art was checked with Firecrawl during plan drafting:

- **Redaction detector classes.** Gitleaks' default configuration ships a
  large named-rule set covering cloud-provider tokens, OAuth and platform
  tokens, JWTs (`jwt`, `jwt-base64`), private-key markers, generic API keys,
  and URL credentials; see
  <https://github.com/gitleaks/gitleaks> and the bundled
  `config/gitleaks.toml`. Yelp's detect-secrets ships a comparable default
  plugin set (`AWSKeyDetector`, `JwtTokenDetector`, `PrivateKeyDetector`,
  `BasicAuthDetector`, `KeywordDetector`, plus `Base64HighEntropyString` and
  `HexHighEntropyString` entropy plugins) at
  <https://github.com/Yelp/detect-secrets>. TruffleHog v3 (
  <https://github.com/trufflesecurity/trufflehog>) advertises an even
  broader, verifier-aware detector list.
- **Entropy thresholds.** The long-standing community defaults inherited
  from the original truffleHog are `b64 ≥ 4.5` and `hex ≥ 3.0` over
  substrings of length `≥ 20`, with the standard alphabets
  (`A-Za-z0-9+/=` for base64, `0-9A-Fa-f` for hex). detect-secrets'
  baseline confirms `Base64HighEntropyString` defaults to `limit: 4.5` and
  `HexHighEntropyString` to `limit: 3.0`.
- **Git URL normalization.** Git's documented URL forms include
  `ssh://[user@]host[:port]/path`, `git://`, `http[s]://`, `ftp[s]://`, local
  paths, `file://`, the scp-like ssh shorthand `[user@]host:path` (recognized
  only when no slash precedes the first colon), and `~user` expansion for
  ssh/git, per <https://git-scm.com/docs/git-clone#_git_urls>. npm's
  `normalize-git-url` (<https://github.com/npm/normalize-git-url>)
  canonicalizes `git+ssh://git@host:org/repo.git#ref` to
  `ssh://git@host/org/repo.git`, deliberately stopping short of host
  lowercasing or `.git` stripping; those steps are added by Renovate and
  Sourcegraph as platform layers
  (<https://docs.renovatebot.com/configuration-options/>).
- **Path hashing.** BLAKE3 (<https://github.com/BLAKE3-team/BLAKE3>) ships a
  first-class `derive_key(context, key_material)` mode that takes a
  hardcoded human-readable context string ("application + purpose"). Docker
  established a 12-hex-character short ID convention for SHA-256-derived
  identifiers
  (<https://nickjanetakis.com/blog/docker-tip-52-referencing-containers-and-images-by-their-short-ids>);
  the same truncation suits a deterministic workspace short identifier.
  UUIDv5 was considered and rejected because it mandates SHA-1, hides the
  namespacing context inside an opaque UUID, and offers no audit benefit
  over a `derive_key` context string carried in source.
- **Purge confirmation.** GitHub's repository deletion flow requires the
  operator to read a danger-zone acknowledgement and then type the exact
  repository name into a confirmation box, per
  <https://docs.github.com/en/repositories/creating-and-managing-repositories/deleting-a-repository>.
  AWS S3's `aws s3 rb --force` only deletes non-versioned objects in the
  bucket and refuses to recurse otherwise
  (<https://docs.aws.amazon.com/cli/latest/reference/s3/rb.html>). Both
  patterns reject pure yes/no prompts in favour of a typed match against the
  target plus an explicit force flag for the irreversible path.
- **Pre-destructive backups.** SQLite's `VACUUM INTO` is documented as the
  recommended transactional snapshot of a live database
  (<https://www.sqlite.org/lang_vacuum.html>); the CLI `.backup` dot-command
  remains available for concurrent-writer scenarios via the online backup
  API (<https://sqlite.org/cli.html>). PostgreSQL's `pg_dump` is the
  authoritative logical backup tool
  (<https://www.postgresql.org/docs/current/app-pgdump.html>) and supports
  custom, directory, and plain formats with `pg_restore`. Qdrant exposes a
  snapshot API at
  <https://qdrant.tech/documentation/concepts/snapshots/>, including
  per-collection `POST /collections/{name}/snapshots` creation and
  `PUT /collections/{name}/snapshots/recover` restore with
  `priority: replica | snapshot | no_sync`; per-node snapshots are required
  in distributed clusters.

These external references inform the ADRs' defaults but do not bind the
implementation to any specific library. The ADRs name the detector classes,
entropy thresholds, normalization rules, hash function, and confirmation
shape; adapter selection of Gitleaks, detect-secrets, TruffleHog, a
Memoryd-internal regex set, BLAKE3, SQLite `VACUUM INTO`, `pg_dump`, or the
Qdrant snapshot API remains a deployment concern.

## Architecture target for the future implementation

The ADRs should describe the target architecture without implementing it in
this slice. ADR 014 owns the workspace-identity and purge defaults; ADR 015
owns the redaction defaults and is `Proposed` pending ratification in
roadmap item 2.2.3.

### Workspace identity defaults (ADR 014)

Repository workspaces derive their workspace identifier from three inputs
inside one tenant context:

- a normalized Git origin URL;
- a stable hash of the local repository root path; and
- an optional configured profile name.

The normalization rules are:

- accept the scp-like ssh shorthand `[user@]host:path` only when no slash
  precedes the first colon, then rewrite to `ssh://[user@]host/path`;
- accept and rewrite `git+ssh://` to `ssh://`, `git+https://` to `https://`,
  and `git+file://` to `file://`;
- drop any embedded userinfo such as `git@`;
- lowercase the host component;
- strip a trailing `.git`;
- strip a trailing slash;
- lowercase the path component for hosts on a documented
  case-insensitive list (initially `github.com`, `gitlab.com`,
  `bitbucket.org`, and `dev.azure.com`); preserve path case for every
  other host so self-hosted Gitea, Gerrit, or Sourcehut deployments are
  not silently collapsed;
- reject `file://` and bare local path forms as Git origins for workspace
  derivation (operators must use the non-Git path instead).

Workspace identity has two layers. The **storage identifier** is the full
BLAKE3 `derive_key` output (32 bytes) over the canonical identity tuple
described below; it is what every tenant-owned row, payload, named graph,
and Chutoro checkpoint key stores. The **short identifier** is the first
twelve hex characters of that storage identifier and is presentational
only: it appears in human-facing output, MCP responses, source-health
displays, and the purge confirmation banner. Storage adapters never key
state on the short form. The short form follows the Docker short-ID
convention but is never relied on for uniqueness within a tenant.

The canonical identity tuple for a repository workspace is
`(normalized_git_origin, canonical_repository_root_path, profile_name?)`,
serialised as a UTF-8 NFC byte string with explicit separators. The BLAKE3
`derive_key` context string is `"memoryd v1 workspace-id"`. The `v1` is
intentional: a future ADR that changes the algorithm bumps the context
string and triggers a deliberate workspace re-registration migration. The
canonical repository root path is the operator-configured absolute path,
not a runtime symlink-resolved path, so identifier stability survives
symlinks, worktrees, and bind mounts. Case folding of the path component
is platform-dependent and configured per workspace; ADR 014 does not
auto-detect case-insensitive filesystems.

Non-Git workspaces hash the canonical identity tuple
`(canonical_configured_root_path, profile_name?)` with the same context
string and the same construction. The construction differs only in input
shape, not in hashing parameters or output length.

Profile names compose into the identity tuple as a separate field. They do
not appear as a textual suffix on either the storage identifier or the
short identifier; presentational rendering of `workspace:profile` happens
in display adapters.

Operator overrides are an explicit configuration concept. ADR 014 records
the override semantics: an override replaces the derived storage identifier
for a single `(tenant_id, configured_source)` pair, must be unique within
the tenant, and is audited at workspace registration. Overrides cannot be
applied retroactively to evidence already ingested under a derived
identifier; doing so requires explicit data migration that is out of scope
for v1.

Collision detection runs at **workspace registration**, not at ingestion.
The first time a configured source resolves to a storage identifier inside
a tenant, the workspace registry records the canonical inputs alongside
the identifier. A subsequent registration that resolves to the same
storage identifier from different canonical inputs fails with an
auditable `WorkspaceCollision` diagnostic naming both source
configurations. Failing at registration means the operator sees the
collision before evidence accumulates; failing at first conflicting ingest
is too late because evidence has already cross-contaminated. Ingestion
remains responsible for rejecting writes against unregistered identifiers.

### Redaction defaults (ADR 015, Proposed)

ADR 015 is `Proposed`. It records a concrete first-release redaction
proposal so the contract is captured in full, but the detector list and
entropy thresholds are ratified or revised in roadmap item 2.2.3 against
real Codex and Claude transcripts. Until then, the defaults below are the
recommended starting point, not a frozen contract.

The first release should detect the categories already named in
`docs/memoryd-design.md` §13:

- API keys and bearer tokens;
- OAuth tokens;
- JSON Web Tokens (JWTs);
- private keys, including RSA, EC, OpenSSH, PGP, and `age` material;
- SSH host material and authentication material;
- `.env` file content;
- passwords and credential prompts;
- HTTP cookies and session identifiers;
- cloud provider credentials (AWS, GCP, Azure, DigitalOcean, Alibaba,
  Cloudflare, IBM, Heroku);
- database connection URLs containing inline credentials;
- high-entropy blobs.

The high-entropy rule uses the truffleHog-derived community-standard
**starting thresholds**: base64 substrings of length `≥ 20` with Shannon
entropy `≥ 4.5`, and hexadecimal substrings of length `≥ 20` with Shannon
entropy `≥ 3.0`. These thresholds are inherited from credential-scanning
work where the input is mostly source code; Codex and Claude transcripts
include file hashes, UUIDs, lockfile content, and base64-encoded tool
attachments that may inflate the false-positive rate. ADR 015 records
the thresholds as starting defaults that roadmap item 2.2.3 ratifies or
revises based on observed false-positive rates. Detection runs before
storage and before embedding. The implementation slice picks the concrete
detector engine.

Deny-pattern behaviour is per-workspace. Deny patterns are
`globset`-compatible glob expressions evaluated against the source path
(matching the `.gitignore` syntax operators already use); they are not
arbitrary path predicates. A matched deny pattern causes the collector to
skip the source, record a source-health diagnostic with the deny-pattern
identity (not the matched secret content), and refuse to enqueue evidence.
Deny patterns are scoped by `(tenant_id, workspace_id)` so one tenant
cannot silently bypass another tenant's policy.

The `[privacy].store_raw_text` enum is binding for v1 and contains exactly
two values:

- `none`: store only redacted text and redacted JSON payloads;
- `redacted` (default): store redacted text in `raw_span.text_redacted` and
  redacted JSON payloads in `raw_event.payload_redacted_json`, never raw
  bytes.

A configuration value of `encrypted` (or anything outside the two values
above) must be rejected at configuration-parse time with a clear semantic
error pointing to ADR 015. Encrypted raw-text storage is a deferred
follow-up: a future ADR that ships explicit key-management, rotation, and
recovery contracts will reintroduce `encrypted` as an accepted value at
the same time as the storage layer learns to honour it. The configuration
parser must not silently accept unknown values, because fail-closed parsing
is the only safe behaviour when storage semantics are at stake.

The `redact_before_store` and `redact_before_embedding` flags default to
`true` and remain `true` for v1 in **every tenant mode**, including
`local_single`. Disabling either flag is forbidden in normal operation
because local-first deployments are exactly the deployments where a
disabled redactor is most likely to leak secrets into a long-lived local
store. An operator who genuinely needs unredacted capture for diagnostic
work may set the explicit escape hatch `[privacy].unsafe_disable_redaction
= true`, which writes a startup warning to the daemon log, emits a
`tracing` `WARN` span on every ingest, and prevents the daemon from ever
serving recall to a tenant other than the configured local-single tenant.
The escape hatch is not a quiet `false` toggle on either flag.

### Purge defaults (ADR 014)

`PurgeWorkspace` and `PurgeTenant` remain irreversible unless a future
backup-tooling ADR adds an undo window.

**Confirmation surface separation is normative.** MCP tools and internal
RPC methods do not accept `PurgeTenant` at all. Tenant-wide purge is
reachable only through the operator-facing `memoryd` CLI, which runs in
the operator's terminal, accepts typed input, and refuses to read its
confirmation answer from a pipe or here-document. MCP and internal RPC
expose only `PurgeWorkspace`, which requires the small randomised challenge
confirmation described below plus the `memory.purge` capability already
named in `docs/memoryd-design.md` §12.

Workspace confirmation uses a **small randomised challenge** as its
primary defence. The operator must answer a short per-invocation prompt
derived from the resolved scope, for example "type the third character of
the workspace slug, then the seventh". The challenge is generated
server-side from a cryptographic random source, is never reused, and is
short enough to be answered without friction. A small randomised challenge
is preferred over a plain typed-name match because it defeats the
clipboard-paste failure mode: a slug pasted from shell history satisfies a
typed-name prompt but cannot satisfy a challenge the operator has not seen
before. Empty answers, partial answers, whitespace-padded answers, and
case-folded answers are rejected. Before reading the answer, the CLI (or
MCP/RPC response preamble) prints the resolved tenant and workspace
identifiers, the count of evidence rows, the count of Qdrant collections,
the list of graph named-graph URIs, and the Chutoro checkpoint file paths,
so the operator confirms against the displayed scope, not against memory.

Tenant confirmation uses the same small randomised challenge plus the
explicit `--force-tenant-scope` flag, and is reachable only through the
operator CLI. The flag exists to make tenant-wide blast radius a
deliberate, separate act from workspace purge; the randomised challenge
still carries the clipboard-paste defence. `PurgeTenant` is disabled by
default in `tenant.mode = "local_single"` and requires a high-privilege
capability token in every tenant mode. ADR 014 confirms the
confirmation shape without changing the capability vocabulary already
named in `docs/memoryd-design.md` §12 and the developers' guide.

**Pre-purge backup completion is binding, not advisory.** Every purge
command runs through a deterministic pre-purge phase that completes (or
is explicitly skipped with a documented flag) before any tenant-owned row
is touched. The phase emits a per-step status to the audit log:

- operators are expected to take an evidence-store backup before any
  purge that affects authoritative rows (SQLite `VACUUM INTO` for the
  default local store, `pg_dump --format=custom` for PostgreSQL
  deployments). The CLI prompts the operator to supply a backup
  acknowledgement identifier (operator-supplied free-form string) that is
  written verbatim to the audit log;
- operators are expected to take a Qdrant snapshot for every affected
  collection via the Qdrant snapshot API. The CLI lists each collection,
  reads back the snapshot identifier the operator supplies (or
  `--skip-snapshot --collection-empty` if the operator has verified the
  collection has no points), and writes each `(collection, snapshot_id)`
  pair to the audit log. **Partial snapshot failure aborts the purge.**
  If the operator answers `--skip-snapshot --collection-empty` for a
  collection that turns out to contain points, the purge aborts with a
  diagnostic referencing the collection name and ADR 014;
- the recorded `(collection, snapshot_id)` pairs are not automated
  recovery artefacts; they exist so an operator paged after an incident
  can find the snapshots they took. Automatic correlation, restore, and
  recovery windows remain out of scope for v1;
- automatic backup tooling is explicitly out of scope for v1, consistent
  with ADR 013.

The purge audit log records the resolved `(tenant_id, workspace_id)`
scope, the actor identity, the challenge prompt that was issued (but not
the operator's answer, and not the slug, so the audit row cannot itself
leak workspace names), the operator-supplied backup acknowledgement
identifier, and every `(collection, snapshot_id)` pair from the Qdrant
phase. The audit row does not record backup file paths because the file
system is the operator's domain, not Memoryd's.

### Hexagonal placement of these defaults

ADR 014 binds **behaviour** to layers; it does not bind Rust type names.
Roadmap item 1.2.1 owns the domain model and picks the concrete newtype
and port-trait names. The behavioural contracts ADRs 014 and 015 place by
layer are as follows (ADR 014 for workspace identity and purge, ADR 015 for
redaction).

The domain layer owns the vocabulary that names:

- a workspace's tenant-scoped identity (storage form and presentational
  short form), the registration record that carries its canonical inputs,
  an override that replaces a derived identifier, and the collision
  diagnostic that names both colliding source configurations;
- the redaction class taxonomy listed above, the per-event redaction
  outcome (clean, redacted-with-class-set, refused), the per-workspace
  deny-pattern set, and the binding two-value raw-text storage mode
  enumeration;
- the purge scope (workspace or tenant), the multi-step purge plan that
  enumerates affected evidence rows, Qdrant collections, graph
  named-graphs, and Chutoro checkpoints, the confirmation record (resolved
  scope display plus the small randomised challenge), and the per-step
  pre-purge backup acknowledgement record.

The application layer owns:

- a workspace registration use case that derives a storage identifier
  from the canonical identity tuple, records the canonical inputs, and
  raises a collision diagnostic if a second registration resolves to an
  already-registered identifier from different canonical inputs;
- an ingestion redaction use case that evaluates every canonical
  conversation delta against the workspace redaction policy before
  storage and before embedding, refusing to enqueue evidence when a deny
  pattern matches;
- a workspace purge use case that drives the pre-purge backup phase to
  completion, records every `(collection, snapshot_id)` pair, validates
  the small randomised challenge answer against the resolved scope, then
  executes the purge plan through evidence-store, graph-store,
  vector-store, and clustering-store ports.

The adapter layer owns:

- BLAKE3 derivation, Git origin parsing, host-case-list lookup, and
  override resolution;
- the detector engine (Gitleaks, detect-secrets, TruffleHog, or a
  Memoryd-internal regex/entropy set; the choice is a deployment concern,
  not a domain concern);
- SQLite `VACUUM INTO`, PostgreSQL `pg_dump`, and Qdrant snapshot API
  invocations, executed as operator procedures invoked from the
  `memoryd` CLI in v1, not in-process commands invoked from the daemon;
- the `memoryd` CLI surface that owns the resolved-scope display, the
  small randomised challenge generation, and `--force-tenant-scope` flag
  parsing.

No new Rust crate, module, configuration key, RPC method, or MCP tool is
added by these ADRs. The `[privacy]` block already documented in
`docs/memoryd-design.md` §14 covers the redaction defaults; the
`memory.purge` capability already documented in §12 covers the purge
defaults; the workspace-derivation contract already documented in §5.4
and ADR 006 covers the workspace defaults. The new
`[privacy].unsafe_disable_redaction` escape hatch (ADR 015) and the new
operator-CLI purge command (ADR 014) are the only configuration and
surface additions these ADRs record; both land in the implementation
slices that first use them.

## Implementation plan

Before editing, confirm the branch is
`1-1-2-local-safety-defaults-for-redaction-and-purge` and the working tree
is clean:

```sh
git branch --show-current
git status --short
```

If the branch is not correct, rename it before making changes. If the
working tree contains unrelated user changes, leave them alone and do not
stage them.

### Milestone 0: Community-of-experts review of this plan (complete)

The `logisphere-design-review` skill has already been run against this
plan during drafting. The full panel (Pandalump, Wafflecat, Buzzy Bee,
Telefono, Doggylump, Dinolump) examined the workspace-identity algorithm,
detector-selection alternatives, per-tenant deny-pattern and
per-collection snapshot scaling, the port boundaries, failure modes for
collision diagnostics and aborted purges, and long-term viability. Its
eight must-fix concerns, three unresolved-risk signposts, and four
improvements were applied to this plan before it was delivered for
approval, and Wafflecat's split-ADR alternative was adopted (see the
`Decision Log`). No further review is required before milestone 1 unless a
milestone materially changes a default, in which case re-run the skill and
record the outcome in `Decision Log`.

### Milestone 1: Write both ADRs

Create two ADRs in this milestone. Both use the documentation style
guide's ADR structure (the required sections in order, en-GB Oxford
spelling, captioned tables, language-tagged code blocks).

#### ADR 014: Local safety defaults for workspace identity and purge (Accepted)

Create `docs/adr-014-local-safety-defaults-workspace-and-purge.md` with:

- `Status`: `Accepted`, with the date and a one-sentence summary closing
  the workspace-identity and purge defaults.
- `Date`: the implementation date in `YYYY-MM-DD` format.
- `Context and problem statement`: explain that roadmap item 1.1.2 must
  close the workspace-identity and purge default decisions before any
  provider adapter implementation begins, and that the design document §17
  and the terms-of-reference §9 currently carry these as open questions.
- `Decision drivers`: local-first quick start, Corbusier compatibility,
  reproducible workspace identifiers across worktrees and symlinks,
  irreversible purge safety, defence against fat-finger and clipboard-paste
  purges, and explicit operator responsibility for backups.
- `Requirements`: functional and technical requirements for workspace
  identity (normalization, two-layer hashing, registration-time collision
  diagnostics, overrides) and purge (small randomised challenge,
  surface separation, scope display, pre-purge backup completion).
- `Options considered`: contrast at least
  (a) a Git-only workspace identity model versus a Git+non-Git model with
  operator overrides;
  (b) a plain yes/no purge prompt versus a typed-name match versus a small
  randomised challenge against the resolved scope.
- `Decision outcome / proposed direction`: state the final defaults in
  binding language.
- `Goals and non-goals`: goals (closing the workspace and purge
  decisions); non-goals (implementing hashing, snapshot tooling, or backup
  automation in this slice).
- `Migration plan`: note that roadmap item 2.2.4 (workspace derivation) and
  the eventual purge implementation slice (roadmap item 5.3.1) apply these
  defaults; ADR 013 already supplies the joint SQLite/PostgreSQL test
  matrix this work will share.
- `Known risks and limitations`: Git URL edge cases (`file://`,
  scp-shorthand, IPv6 literals), 48-bit short-ID collision space (mitigated
  by keying storage on the full hash), Qdrant snapshot cost for large
  collections, and the operability of the randomised challenge in scripted
  environments.
- `Consequences`: how later slices implement these defaults behind ports,
  add adapter contract tests, document operator backup procedures, and
  surface workspace-collision diagnostics through source-health and
  audit-log surfaces.
- `References`: ADR 005, ADR 006, ADR 008, ADR 013, ADR 015, design
  §§5.4/§7/§8.4/§13/§14/§17, terms of reference §§7-9, RFC 0001 §§3, 7, and
  the external prior-art URLs cited above.

ADR 014 must explicitly state:

- the normalized Git origin URL form, including the documented
  case-insensitive host list (`github.com`, `gitlab.com`, `bitbucket.org`,
  `dev.azure.com`) for path-component lowercasing;
- the two-layer workspace identity contract: full BLAKE3 `derive_key`
  storage identifier with context string `"memoryd v1 workspace-id"`, and a
  12-hex-character presentational short form that storage never keys on;
- the intentional `v1` in the BLAKE3 context string so future algorithm
  changes trigger an explicit re-registration migration;
- the non-Git canonical configured root path identity tuple, hashed with
  the same context string and construction;
- explicit operator overrides scoped to one tenant, audited at workspace
  registration, and non-retroactive;
- failure-on-collision at workspace registration with an auditable
  collision diagnostic naming both source configurations;
- a small per-invocation randomised challenge against the resolved scope as
  the primary `PurgeWorkspace` confirmation, with MCP and internal RPC
  surfaces refusing `PurgeTenant` entirely;
- the same small randomised challenge plus `--force-tenant-scope` for
  `PurgeTenant`, reachable only via the operator CLI;
- pre-purge backup completion semantics: SQLite `VACUUM INTO`, PostgreSQL
  `pg_dump`, and per-collection Qdrant snapshot API steps must complete (or
  be explicitly skipped through documented flags) with
  `(collection, snapshot_id)` pairs recorded in the audit log before any
  tenant-owned row is touched; partial failure aborts the purge;
- automated backup tooling and automated snapshot correlation are
  explicitly out of v1.

#### ADR 015: Redaction defaults (Proposed)

Create `docs/adr-015-redaction-defaults.md` with:

- `Status`: `Proposed`, with the date and a one-sentence summary noting
  that the redaction defaults are a concrete proposal ratified or revised
  in roadmap item 2.2.3.
- `Date`: the implementation date in `YYYY-MM-DD` format.
- `Context and problem statement`: explain that the first-release detector
  list and entropy thresholds should be set against real Codex and Claude
  transcripts, that they are recorded now so the contract is captured in
  full, and that roadmap item 2.2.3 owns ratification.
- `Decision drivers`: security posture against transcript exfiltration,
  redaction before storage and before embedding, low false-positive rate on
  real transcripts, and a fail-closed configuration parser.
- `Requirements`: detector classes, starting entropy thresholds,
  `globset` deny-pattern syntax and per-workspace scope, the two-value
  raw-text storage mode, and the binding `redact_before_*` flags.
- `Options considered`: a fixed first-release detector list versus a fully
  library-deferred list; an asymmetric `redact_before_*` policy per tenant
  mode versus a non-disablable policy with an explicit escape hatch.
- `Decision outcome / proposed direction`: state the proposed defaults and
  mark them for ratification in roadmap item 2.2.3.
- `Goals and non-goals`: goals (capturing the redaction proposal in full);
  non-goals (implementing detectors or selecting a detector engine in this
  slice; deciding encrypted raw-text storage).
- `Migration plan`: roadmap item 2.2.3 ratifies or revises this ADR against
  real transcripts and flips its status to `Accepted` (or supersedes it).
- `Known risks and limitations`: detector-engine lock-in, entropy
  thresholds inherited from credential scanning, and adversarial inputs
  that evade pattern-based redaction.
- `Outstanding decisions`: the threshold tuning and detector-engine
  selection deferred to roadmap item 2.2.3, and encrypted raw-text storage
  deferred to a future ADR.
- `References`: ADR 005, ADR 006, ADR 014, design §§7/§13/§14/§17, terms of
  reference §§7.2/8.1/9, RFC 0001 §3, and the external detector and entropy
  references cited above.

ADR 015 must explicitly state:

- the first-release redaction detector class list;
- the truffleHog-derived starting entropy thresholds (`b64 ≥ 4.5`,
  `hex ≥ 3.0`, length `≥ 20`) and their tunability in roadmap item 2.2.3;
- per-workspace `globset`-compatible deny patterns;
- `redact_before_store = true` and `redact_before_embedding = true` as
  binding defaults in every tenant mode, with the
  `[privacy].unsafe_disable_redaction` escape hatch as the only safe
  disablement path for local-single diagnostic work;
- `store_raw_text` enumerated as exactly `none | redacted` for v1, with
  `redacted` as the default and `encrypted` rejected at configuration-parse
  time pending a follow-up ADR;
- that the ADR is a proposal whose detector list and thresholds are
  ratified in roadmap item 2.2.3.

After both ADRs are written, run the documentation gates:

```sh
set -o pipefail
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-2-adr.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-2-adr.out
```

Then run CodeRabbit for this milestone:

```sh
coderabbit review --agent
```

If CodeRabbit reports actionable documentation or correctness concerns, fix
them and rerun the deterministic gates before asking CodeRabbit again. If
the command is unavailable or cannot authenticate, record that in
`Surprises & Discoveries` and continue only if deterministic gates pass.

Commit both ADRs with a file-based commit message:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Record local safety defaults ADRs

Accept the workspace identity and purge defaults (ADR 014) for roadmap
item 1.1.2, including normalized Git origin URLs, two-layer
BLAKE3-derived workspace identifiers, registration-time collision
diagnostics, and the small-randomised-challenge purge confirmation with
binding pre-purge backup completion. Propose the redaction defaults
(ADR 015) covering the first-release detector classes, starting entropy
thresholds, globset deny patterns, and the two-value store_raw_text enum,
to be ratified against real transcripts in roadmap item 2.2.3.
ENDOFMSG
git add docs/adr-014-local-safety-defaults-workspace-and-purge.md \
  docs/adr-015-redaction-defaults.md
git commit -F "$COMMIT_MSG_DIR/COMMIT_MSG.md"
rm -rf "$COMMIT_MSG_DIR"
```

### Milestone 2: Align source documents with the ADRs

Update `docs/memoryd-design.md`:

- §5.4: keep the existing workspace-derivation paragraph and add a sentence
  pointing readers to ADR 014 for the exact normalization rules, hash
  function, truncation length, and collision-diagnostic shape.
- §13: keep the first-release detector list, add a sentence stating that
  ADR 015 (proposed) records the entropy thresholds and per-workspace
  deny-pattern scope pending ratification in roadmap item 2.2.3, and add a
  sentence stating that ADR 014 binds the small-randomised-challenge purge
  confirmation and the pre-purge backup completion semantics.
- §14: keep the `[privacy]` block; add a sentence to the paragraph
  introducing the listing stating that ADR 015 binds the `store_raw_text`
  enum (`none | redacted`) and the `redact_before_*` defaults in every
  tenant mode, and names the `unsafe_disable_redaction` escape hatch.
- §17: remove the bullet "Define the operator backup format used before
  irreversible purge" (closed by ADR 014). Reword the bullet "Define the
  redaction detector set and encrypted raw-text mode" so it points at ADR
  015 as the proposed direction, ratified in roadmap item 2.2.3, rather
  than reading as an unframed open question.

Update `docs/terms-of-reference.md`:

- §9: update the prelude paragraph to note that ADR 014 closes the
  workspace-identity and pre-purge backup defaults and that ADR 015
  proposes the redaction defaults for ratification in roadmap item 2.2.3.
  Update the "What redaction policy is sufficient for the first release?"
  row so its suggested path references ADR 015 (proposed) and roadmap item
  2.2.3 rather than an open security review.
- Appendix B: reword the ADR candidate "Decide redaction guarantees and
  whether encrypted raw-text storage is in scope" so it references ADR 015
  for the v1 proposal and notes that encrypted raw-text storage remains a
  future ADR.

Update `docs/rfcs/0001-standalone-evidence-inbox.md`:

- §7 (Compatibility and migration): append a sentence noting that ADR 014
  binds the workspace-identity defaults and pre-purge backup expectations
  and that ADR 015 proposes the redaction policy the evidence inbox uses.

Update `docs/contents.md`:

- Add an ADR 014 entry and an ADR 015 entry to the design-records list
  immediately after the ADR 013 row, following the existing one-line
  description style. The ADR 015 line must note its `Proposed` status.

Update `docs/developers-guide.md`:

- Authentication-and-authorization section: append a sentence noting that
  ADR 014 binds the small-randomised-challenge confirmation shape for
  `memory.purge`, the surface separation that keeps `PurgeTenant` off the
  MCP and RPC surfaces, and the pre-purge backup procedures.

Update `docs/users-guide.md`:

- Add one short subsection titled "Workspace, redaction, and purge
  defaults" that summarises, for an operator, how workspace identity is
  derived, that transcripts are redacted before storage and embedding, and
  that purge is irreversible and gated by a confirmation challenge plus
  pre-purge backups. Keep it to a few sentences and link ADR 014 and ADR
  015. This subsection documents the policy a user should know about even
  before the operator CLI ships; the command-specific usage detail lands in
  roadmap items 1.3 and 2.2.3-2.2.4.

Update `docs/roadmap.md`:

- Align the item 1.1.2 task text with the now-settled decisions, but do not
  mark item 1.1.2 done yet in this milestone.
- Signpost the inherited decisions and open questions in the relevant
  tasks:
  - item 2.2.3 (redaction pipeline): add a note that this task ratifies or
    revises ADR 015's proposed detector list and entropy thresholds against
    real Codex and Claude transcripts and flips ADR 015 to `Accepted` (or
    supersedes it);
  - item 2.2.4 (workspace derivation): add a note that this task implements
    ADR 014's workspace-identity contract (normalization, two-layer
    hashing, registration-time collision diagnostics, overrides);
  - item 5.3.1 (workspace purge): add a note that this task implements ADR
    014's purge confirmation and pre-purge backup completion semantics.

Run the required gates sequentially:

```sh
set -o pipefail
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-2-docs.out
make typecheck 2>&1 | tee /tmp/typecheck-memoryd-1-1-2-docs.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-2-docs.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-2-docs.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-2-docs.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-2-docs.out
```

Run CodeRabbit after the deterministic gates pass:

```sh
coderabbit review --agent
```

Clear applicable concerns. Then commit the cross-document alignment:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Align safety-defaults documentation

Update the design, terms of reference, RFC 0001, contents index,
developers' guide, users' guide, and roadmap so the accepted workspace
and purge defaults (ADR 014) and the proposed redaction defaults
(ADR 015) are discoverable from every source document that previously
carried the open decisions, and so roadmap items 2.2.3, 2.2.4, and 5.3.1
signpost the decisions they inherit.
ENDOFMSG
git add docs/memoryd-design.md docs/terms-of-reference.md \
  docs/rfcs/0001-standalone-evidence-inbox.md docs/contents.md \
  docs/developers-guide.md docs/users-guide.md docs/roadmap.md
git commit -F "$COMMIT_MSG_DIR/COMMIT_MSG.md"
rm -rf "$COMMIT_MSG_DIR"
```

### Milestone 3: Mark roadmap item 1.1.2 done

Verify that both ADRs exist (ADR 014 `Accepted`, ADR 015 `Proposed`),
every source document links or names them, the design §17 list no longer
carries the backup-format bullet and reframes the redaction bullet as the
proposed ADR 015 awaiting ratification in roadmap item 2.2.3, and roadmap
items 2.2.3, 2.2.4, and 5.3.1 carry their signposts. Then update
`docs/roadmap.md` to mark item 1.1.2 done.

Run the full gate set again:

```sh
set -o pipefail
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-2-final.out
make typecheck 2>&1 | tee /tmp/typecheck-memoryd-1-1-2-final.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-2-final.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-2-final.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-2-final.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-2-final.out
```

Run the final CodeRabbit review:

```sh
coderabbit review --agent
```

Clear applicable concerns, then commit the roadmap completion:

```sh
COMMIT_MSG_DIR=$(mktemp -d)
cat > "$COMMIT_MSG_DIR/COMMIT_MSG.md" << 'ENDOFMSG'
Mark safety-defaults task complete

Mark roadmap item 1.1.2 done after ADR 014 (accepted) and ADR 015
(proposed) and the supporting documentation close the workspace identity
and purge decisions and capture the proposed redaction defaults for
ratification in roadmap item 2.2.3.
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

For the future implementation slices that will apply these defaults, the
ADRs must require this test matrix:

- Unit tests with `rstest` covering Git origin normalization rules
  (scp-shorthand, `git+ssh`, `git+https`, embedded userinfo, host
  lowercasing, path-component case folding for the documented host list,
  `.git` stripping, trailing-slash stripping, IPv6 literals, `file://`
  rejection), BLAKE3 storage-identifier derivation and short-form
  truncation, profile-field composition, override resolution, and
  registration-time collision detection.
- Behavioural tests with `rstest-bdd` covering scenarios such as
  "operator imports the same repository from two paths within one tenant",
  "operator overrides workspace identity for a moved repository", and
  "non-Git workspace identifier survives a symlink relocation".
- Snapshot tests with `insta` covering normalized URL output, storage and
  short identifier derivation, collision diagnostic shape, redacted JSON
  envelopes for each detector class, and the purge confirmation challenge
  prompt text.
- Property tests with `proptest` covering Git URL canonicalization
  idempotence (`normalize(normalize(x)) == normalize(x)`), redaction
  idempotence on already-redacted text, and challenge-answer validation
  (rejecting empty, case-folded, whitespace-padded, and partial answers).
- End-to-end tests covering the SQLite default path for workspace
  registration, redaction-before-store, redaction-before-embedding, and the
  small-randomised-challenge purge confirmation; PostgreSQL equivalents
  under `POSTGRES_TEST_URL` or `pg_embedded_setup_unpriv` per ADR 013.
- Source-health and audit-log assertions covering deny-pattern matches,
  workspace-collision diagnostics, and pre-purge backup acknowledgement
  records.
- Optional Kani or Verus harnesses only if a later implementation slice
  introduces a bounded state machine for purge progression or a
  contractual lemma for redaction idempotence. The documentation-only ADRs
  must not introduce formal verification work by themselves.

`googletest` assertions and `pretty_assertions` apply to all of the above.

## Idempotence and recovery

Every step in this plan is idempotent. Re-running a milestone simply
re-executes the gates and either reports that the documents already match
the ADRs or fails the gates with diagnostic output. The ADR files
themselves are overwrite-safe because Git tracks history. The
cross-document alignment edits are scoped to specific named sections so
repeated edits converge to the same outcome.

If a gate fails twice after focused repair, stop and record the blocker in
`Surprises & Discoveries` with the failing log path under `/tmp`, then ask
the user for direction before retrying.

If CodeRabbit's review surfaces a concern that contradicts the accepted
defaults, do not silently weaken the ADRs; instead record the conflict in
`Decision Log`, ask the user to confirm whether the default should change,
and only then revise the relevant ADR plus all linked documents.

## Interfaces and dependencies

This slice does not add new Rust interfaces or commit to Rust type names.
ADR 014 and ADR 015 bind the **behavioural** contracts that subsequent
slices must honour; roadmap item 1.2.1 owns the concrete domain newtypes,
port traits, and module layout.

The behavioural contracts the ADRs fix are:

- a tenant-scoped workspace identity contract (ADR 014) with a two-layer
  (storage / presentational) shape and a deterministic registration-time
  collision diagnostic;
- a per-workspace redaction policy contract (ADR 015, proposed) that
  classifies events into the first-release detector classes, enforces
  deny-pattern skipping before ingest, and binds the two-value raw-text
  storage mode;
- a multi-step purge contract (ADR 014) whose pre-purge backup phase runs
  to completion (or aborts) before any tenant-owned row is touched, whose
  confirmation is a small randomised challenge against the resolved scope,
  and whose tenant-wide variant is reachable only from the operator CLI
  behind `--force-tenant-scope`.

Adapter responsibilities under those contracts are: BLAKE3 derivation,
Git origin parsing, host-case-list lookup, detector engine selection
(Gitleaks-style, detect-secrets-style, TruffleHog-style, or
Memoryd-internal regex/entropy), SQLite `VACUUM INTO`, PostgreSQL
`pg_dump`, Qdrant snapshot API invocations, and `memoryd` CLI prompt
handling. None of these are committed as Rust crate or module choices in
this slice.

No new external dependency is committed by these ADRs.

## Progress

- [x] 2026-06-05: Loaded the requested `leta`, `rust-router`, and
  `hexagonal-architecture` skills; created a Leta workspace for the
  repository.
- [x] 2026-06-05: Renamed the local branch to
  `1-1-2-local-safety-defaults-for-redaction-and-purge` and pushed it with
  upstream tracking.
- [x] 2026-06-05: Launched a repository reconnaissance agent and a
  Firecrawl prior-art research agent in parallel to gather citations,
  external detector references, BLAKE3 / Git URL normalization references,
  and SQLite / PostgreSQL / Qdrant backup references.
- [x] 2026-06-05: Drafted this pre-implementation ExecPlan.
- [x] 2026-06-05: Ran `logisphere-design-review` over the draft and
  applied the eight must-fix concerns plus the nice-to-have refinements
  before delivery.
- [x] 2026-06-14: Applied the reviewer-direction refinements: split the
  decision into ADR 014 (workspace + purge, Accepted) and ADR 015
  (redaction, Proposed) written in the same milestone; switched purge
  confirmation to a small randomised challenge as the primary defence;
  committed to one short users-guide subsection; and added roadmap
  signposts for items 2.2.3, 2.2.4, and 5.3.1.
- [ ] Received explicit user approval to implement this ExecPlan.
- [ ] Milestone 1: wrote ADR 014 and ADR 015, passed `make markdownlint`
  and `make nixie`, and cleared CodeRabbit review.
- [ ] Milestone 2: aligned source documents with both ADRs and added the
  roadmap signposts, passed `make check-fmt`, `make typecheck`,
  `make lint`, `make test`, `make markdownlint`, and `make nixie`, and
  cleared CodeRabbit review.
- [ ] Milestone 3: verified the backup-format question is closed and the
  redaction bullet points at proposed ADR 015 / roadmap item 2.2.3, marked
  roadmap item 1.1.2 done, passed the full final gate set, and cleared the
  final CodeRabbit review.

## Surprises & Discoveries

(none recorded yet)

## Decision Log

- 2026-06-05: Treat this slice as a documentation-only contract slice that
  ratifies workspace, redaction, and purge defaults without writing any
  production code. Rationale: roadmap item 1.1.2 must close decisions that
  block later implementation, and ADR 013 already established the
  documentation-only pattern for ratifying foundational contracts.
- 2026-06-05: Plan ADR 014 as the target decision record. Rationale: ADR
  013 is the current newest accepted ADR, and the documentation style
  guide requires sequential ADR filenames in `docs/`.
- 2026-06-05: Bind the workspace identifier short form to BLAKE3
  `derive_key("memoryd v1 workspace-id", canonical)` truncated to 12 hex
  characters. Rationale: BLAKE3 ships `derive_key` as a first-class mode
  with a human-readable context string, the Docker short-ID convention
  legitimises 12-hex truncation, and SHA-1-based UUIDv5 offers no audit
  benefit.
- 2026-06-05: Bind the redaction entropy thresholds to `b64 ≥ 4.5`,
  `hex ≥ 3.0`, length `≥ 20`. Rationale: the truffleHog-derived defaults
  are reused verbatim by detect-secrets and entro.py and represent the
  community-standard pattern memoryd integrators already trust.
- 2026-06-14: Make a small randomised challenge the primary purge
  confirmation, superseding the earlier typed-name match. Rationale:
  reviewer direction preferred the small randomised challenge because a
  typed-name prompt can be satisfied by a slug pasted from shell history,
  whereas a short per-invocation challenge the operator has not seen before
  cannot. Tenant-wide purge keeps the additional `--force-tenant-scope`
  flag and remains operator-CLI-only; GitHub's danger-zone flow and AWS
  S3's `aws s3 rb --force` remain the cited precedents for explicit-scope
  destructive actions.
- 2026-06-05: Defer `store_raw_text = "encrypted"` to a future ADR and
  enumerate the v1 enum as exactly `none | redacted`. Rationale: a
  fail-closed configuration parser is safer than an "accept-but-reject"
  reserved value, and encrypted raw-text storage requires explicit
  key-management, rotation, and recovery contracts that exceed this
  slice's scope.
- 2026-06-05: Ran the `logisphere-design-review` skill against the
  initial draft and applied its eight must-fix concerns and several
  nice-to-have refinements before delivery. The applied changes are:
  (1) two-layer workspace identity contract distinguishing the full
  BLAKE3 storage identifier from the 12-hex presentational short form;
  (2) removal of Rust newtype and port-trait names from the ADR-binding
  surface, leaving 1.2.1 to pick concrete names;
  (3) dropping `encrypted` from the v1 `store_raw_text` enum and
  rejecting it at configuration-parse time;
  (4) making `redact_before_*` non-disablable in every tenant mode,
  with a single explicit `[privacy].unsafe_disable_redaction` escape
  hatch instead of an implicit `local_single` exemption;
  (5) augmenting purge confirmation with a randomised challenge prompt to
  defeat clipboard-paste mistakes;
  (6) confirming MCP and internal RPC surfaces refuse `PurgeTenant`
  entirely and that `--force-tenant-scope` is an operator-CLI concept;
  (7) moving collision detection to workspace registration time with a
  named collision diagnostic;
  (8) binding pre-purge backup completion semantics with audited
  `(collection, snapshot_id)` pairs and partial-failure abort. Nice-to-
  have refinements applied: documented case-insensitive host list for
  path-component lowercasing, `globset` syntax for deny patterns,
  intentional `v1` in the BLAKE3 context string, and entropy thresholds
  flagged as starting defaults to be tuned later.
- 2026-06-14: On reviewer direction, adopted Wafflecat's split-ADR
  alternative: ADR 014 (workspace identity + purge, Accepted) and ADR 015
  (redaction defaults, Proposed), both written in milestone 1, with ADR
  015 explicitly revisited and ratified in roadmap item 2.2.3. Rationale:
  the user preferred a separate document so the difference in confidence is
  legible — workspace and purge are settled, while the redaction detector
  list and thresholds are a recorded proposal awaiting transcript-informed
  ratification. Writing both now captures the proposal in full rather than
  leaving a vague forward reference. This supersedes the earlier decision
  to keep a single ADR.
- 2026-06-14: On reviewer direction, made the purge confirmation a small
  randomised challenge (primary defence) and dropped the separate
  `--i-understand-this-is-irreversible` flag for workspace purge to keep the
  flow lean; tenant-wide purge retains `--force-tenant-scope`. Rationale:
  the user explicitly preferred the small randomised challenge.
- 2026-06-14: On reviewer direction, committed to one short
  `docs/users-guide.md` subsection ("Workspace, redaction, and purge
  defaults") in milestone 2, reversing the earlier decision to omit the
  users' guide. Rationale: the user wants the policy a user should know
  about documented now, with command-specific usage detail still deferred
  to the implementation slices. The expected-file tolerance rose to nine
  accordingly.
- 2026-06-14: On reviewer direction, signposted the inherited open
  questions in the relevant roadmap tasks (2.2.3 redaction ratification,
  2.2.4 workspace derivation, 5.3.1 workspace purge). Rationale: the user
  asked for the open questions to be visible from the tasks that resolve
  them rather than only in the execplan and ADRs.

## Outcomes & Retrospective

(filled in after Milestone 3 completes)
