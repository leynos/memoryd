# Architectural decision record (ADR) 015: Local safety defaults for workspace identity, redaction, and purge

## Status

Accepted on 2026-10-15. Memoryd v1 is required to derive workspace identity
from a versioned descriptor, to store redacted text by default, to refuse
unsupported raw-text modes, and to require explicit scope-bound confirmation
before an irreversible purge. This ADR records that policy; it does not
implement or enforce it. The runtime that performs these checks is built by
roadmap items 2.2.3, 2.2.4, and 5.3.1, and until those land the present
behaviour is the application stub described in the ExecPlan.

## Date

2026-10-15.

## Context and problem statement

Roadmap item 1.1.2 asks Memoryd to settle the safety defaults that provider
adapters need before evidence capture can begin: how a workspace is identified,
which content is excluded or redacted, and under what authority an irreversible
purge may run. Until those defaults exist, every adapter would invent its own
normalization and redaction, and purge would be specified only as a capability
name.

Three decisions are entangled in this question.

First, workspace identity must be stable enough to group evidence for one
project, yet must never silently merge evidence from two unrelated projects.
The design document already requires that "if derivation detects a collision
inside one tenant, ingestion must fail with an auditable collision diagnostic
rather than merging evidence" (memoryd-design.md §5.4), but it does not define
the derivation inputs precisely enough to implement or test.

Second, redaction cannot detect every secret, so the decision must state a
concrete detector corpus and an explicit failure posture rather than a general
promise of secrecy. The design document lists the secret classes the first
release must detect and defers the detector set itself (memoryd-design.md §13
and §17).

Third, purge is irreversible, so the decision must state what confirmation
means, what a backup must cover, and what purge cannot do. ADR 013 already
requires explicit operator backups before purge until automatic backup tooling
exists; the design defers the operator backup format (memoryd-design.md §17).

The tenant authority boundary constrains all three. Identity is the pair of
authenticated tenant and workspace, never a workspace string alone, and
provider fields cannot supply authority (ADR 006, memoryd-design.md §5.4).

## Decision drivers

- Provider implementors must be able to resolve a workspace, exclude forbidden
  content, redact evidence, and refuse unsafe purge requests without waiting
  for graph, theme, or Axinite write-back policy.
- Silent identity merges are worse than false separation: two unrelated
  projects sharing one evidence namespace is an unrecoverable privacy fault,
  while two namespaces for one project is a recoverable operator annoyance.
- Redaction is a mitigation, not a guarantee. The decision must state what is
  detected, what is deliberately over-detected, and what remains undetectable.
- Backups preserve data that purge removes. Documentation must not promise
  secure erasure, deletion of provider originals, or automatic undo.
- Both SQLite and PostgreSQL must remain first-class (ADR 013), so the safety
  policy must not depend on backend-specific features.
- A documentation decision must not be mistaken for shipped behaviour. Defaults
  remain planned until the owning runtime slice implements and verifies them.

## Requirements

### Functional requirements

- Repository workspaces derive from a normalized Git origin, a digest of the
  local repository root path, and an optional profile name.
- Non-Git workspaces derive from a digest of the canonical configured root path
  and the profile name.
- Identity derivation is versioned, so a later change to the derivation inputs
  is an explicit new descriptor version rather than a silent rekeying.
- Derivation never merges two distinct descriptors, even if the digest function
  maps them to the same key.
- Operators may map an exact descriptor to an explicit workspace identifier and
  may authorize an explicit join list; the list is never learned from provider
  payloads.
- Redaction happens before storage and before embedding, and the first release
  cannot disable either.
- Free-text payloads are dropped entirely in the metadata-only raw-text mode.
- Encrypted raw-text storage is rejected as unsupported until a separate
  key-management decision exists.
- Per-workspace deny patterns exclude configured paths before content is read.
- Workspace purge requires an authenticated tenant, an allowed workspace, a
  high-privilege capability, an explicit scope-bound confirmation, and an
  explicit pre-purge backup acknowledgement.
- Tenant-wide purge remains administratively disabled in local mode.

### Technical requirements

- The policy is a pure domain decision over validated inputs. Filesystem, Git,
  SQL, capability, terminal, and backup behaviour stay behind adapters.
- Domain ports and error types carry Memoryd vocabulary; no adapter, SDK, or
  filesystem error type crosses the domain boundary.
- Both ingestion ports (`ConversationPushIngestPort` and
  `CollectedConversationIngestPort`) enforce the same policy at their own trust
  boundary without calling each other.
- Detector and policy versions are explicit, so a detector change cannot
  silently redefine provider idempotency keys.
- Diagnostics and metrics carry structured reason codes with bounded labels,
  never paths, raw origins, tokens, or payload text.
- The decision introduces no new dependency. Any later implementation adds
  libraries with caret version requirements through its own slice.

## Options considered

### Workspace identity

| Option                                          | Consequence                                                                                                                                                   |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Origin URL only                                 | Simple, but collapses two checkouts of the same repository on one machine, and gives no identity at all for non-Git roots.                                    |
| Root path only                                  | Distinguishes local checkouts, but merges a repository cloned into two machines' divergent paths and ignores the remote that makes the workspace shared.      |
| Origin URL, root digest, and profile (selected) | Separates checkouts and machines by default, supports non-Git roots, and gives operators an explicit join mechanism instead of an implicit merge.             |
| Auto-merge on digest collision                  | Recovers the rare collision automatically, but cannot distinguish a true collision from two genuinely different projects, so it risks silent evidence mixing. |

_Table 1: Workspace identity options._

### Raw-text storage mode

| Option                | Consequence                                                                                                                                             |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `redacted` (selected) | Retains sanitized text for recall and explanation while meeting the design's before-storage and before-embedding requirements.                          |
| `none`                | Retains identities and minimal structural metadata, but drops all free-text, so text-derived recall and summaries are unavailable for that workspace.   |
| Plaintext             | Maximizes recall quality, but contradicts memoryd-design.md §13 and would store exactly the material the redaction policy exists to remove.             |
| `encrypted`           | Could preserve fidelity with a key boundary, but key provisioning, rotation, recovery, and audit are independent decisions with no owner in this slice. |

_Table 2: Raw-text storage mode options._

### Purge confirmation

| Option                                       | Consequence                                                                                                                                            |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Explicit scope-bound confirmation (selected) | Binds the operator's intent to a specific preview and policy revision, and fails closed on empty input, EOF, mismatch, or cancellation.                |
| Bare global `--yes`                          | Convenient for scripting, but does not bind the confirmation to a scope, so a stale or mistaken preview is confirmed just as readily as a correct one. |
| Interactive prompt only                      | Safe for humans, but leaves automation with no defined contract, which pushes the decision into whichever transport is written first.                  |
| No confirmation (capability alone)           | The capability would become both authority and confirmation, removing the deliberate second act that an irreversible operation warrants.               |

_Table 3: Purge confirmation options._

## Decision outcome / proposed direction

Three contracts are accepted together because they share one authority model.

### SAFE-ID: workspace derivation

A workspace descriptor is versioned and tagged. Git and non-Git descriptors
carry distinct tags so a non-Git root can never collide with a Git descriptor
by construction. The descriptor is serialized as UTF-8 bytes with explicit
unsigned 64-bit big-endian byte lengths for every field, preceded by the ASCII
domain separator `memoryd.workspace.v1` and the descriptor tag. Length
prefixing removes delimiter ambiguity; a length that overflows the encoding is
an error, not a truncation. The authenticated tenant stays a separate key field
and is never hashed into the descriptor.

For a Git workspace the fields are the normalized origin, the root digest, and
the profile. For a non-Git workspace the fields are the root digest and the
profile. A root digest uses the domain separator `memoryd.root.v1` followed by
the length-prefixed canonical absolute path. Absent profile is the empty
string, and an explicitly empty profile means the same thing. Profile case and
Unicode code points are preserved.

Identity uses SHA-256 and retains the full 256-bit digest in lowercase
hexadecimal. The digest is not truncated for storage or authorization. A hash
is an identifier, not anonymization and not proof of uniqueness: the policy
never assumes SHA-256 is injective and never treats a digest match as evidence
that two descriptors are equal.

Origin normalization preserves protocol and path semantics:

- lowercase the ASCII scheme and DNS host, and preserve repository path case
  and percent escapes;
- remove password and token credentials from URL userinfo, and reject query and
  fragment components;
- preserve the SSH username as an identity field, because it can select a
  different server-side repository namespace, but never emit it in diagnostics;
- for HTTP URLs omit authentication userinfo entirely;
- preserve explicit ports, including ports equal to the scheme default;
- preserve trailing slashes and a `.git` suffix.

The scheme is part of identity. SSH and HTTPS forms of the same repository are
not automatically interchangeable, and SCP-like syntax is a separately tagged
form that preserves relative versus absolute path semantics and user-relative
paths. Equating an SCP form with a URL form, or two different transports,
requires an explicit descriptor join. Ambiguous or unsupported remote-helper
syntax is rejected.

A missing origin is a Git descriptor with an explicit absent-origin tag, never
a guessed remote. Local and `file` origins must resolve relative to the
repository root and use a separate local-origin tag; an unavailable target
fails closed. Raw origins and credentials are never logged. A redacted display
label is separate from identity and may be shown to operators.

The origin comes from local Git configuration (`remote.origin.url`) without
fetching, executing remote helpers, or resolving network aliases. Roots are
resolved through filesystem adapters and then passed as validated values into
the pure policy. Missing or inaccessible roots, non-UTF-8 paths, and unresolved
roots are explicit errors in the first implementation. Lossy strings are never
hashed, and no case folding or Unicode normalization of local paths is imposed
beyond the platform's canonical path result. Cross-machine identity stability
is not promised, and hashing a path does not protect a guessable path from
disclosure.

A root move, a different worktree, an origin change, or a profile change all
yield a new descriptor. Symlinks that resolve to the same worktree root yield
the same root digest. Git's shared common directory is not used to collapse
linked worktrees.

Registration persists the descriptor version and the descriptor binding,
including the canonical root path as it was hashed. Equality checks compare
that original canonical path as well as the origin and profile, never only the
root digest. This binding metadata is sensitive and stays local to the
authorized registry; it is not a public workspace label, and it follows the
same access and purge policy as other workspace metadata. When a
tenant/workspace key already exists, an equal descriptor makes registration
idempotent, and an unequal descriptor raises a typed collision error with no
write.

Derived and configured identifiers are namespaced separately, so a user-chosen
string cannot imitate a derived key.

Operator configuration may map an exact descriptor to an explicit workspace
identifier, scoped to one authenticated tenant. An alias is a display name
only. An intentional join requires an explicit list of descriptors authorized
to share that configured identifier. Configuration changes do not move existing
evidence; relocation or reclassification of historical evidence requires a
separate migration. Duplicate or conflicting mappings are startup errors.
Configuration changes are audited with opaque identifiers, outcome, policy
version, and reason code, and never with raw paths or tokens.

Table 4 gives worked examples with their expected outcomes.

| Input                                                            | Expected outcome                                                                                       |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Origin `https://GitHub.COM/Owner/Repo.git`                       | Scheme and host lowercased to `https://github.com/Owner/Repo.git`; path case preserved.                |
| Origin `https://github.com/Owner/Repo.git/`                      | Distinct descriptor from the previous row: a trailing slash is preserved.                              |
| Origin `https://user:token@github.com/Owner/Repo.git`            | Credentials removed from userinfo; descriptor equals the credential-free HTTPS form of the same path.  |
| Origin `https://github.com:443/Owner/Repo.git`                   | Explicit port preserved; distinct descriptor from the portless form.                                   |
| Origin `git@github.com:Owner/Repo.git`                           | SCP-like form under its own tag; distinct from every URL form until an explicit join equates them.     |
| Origin `ssh://git@github.com/Owner/Repo.git`                     | SSH username preserved as an identity input; a different username yields a different descriptor.       |
| No configured origin                                             | Absent-origin Git tag; distinct from every present-origin descriptor, and no remote is guessed.        |
| Origin `file:///srv/repo`                                        | Local-origin tag after resolution relative to the repository root; an unavailable target fails closed. |
| Non-Git root `/srv/notes`                                        | Non-Git descriptor: root digest plus profile, with no origin field.                                    |
| Non-Git root `/srv/notes` reached through a symlink              | Same canonical root, therefore the same root digest and the same descriptor.                           |
| Root `/srv/notes`, profile `work` versus profile `Work`          | Different descriptors: profile case is preserved.                                                      |
| Root `/srv/notes` after moving to `/srv/archive/notes`           | Different descriptor: the canonical root path changed.                                                 |
| Two descriptors, one tenant, forced to the same final digest     | Typed collision error, no write, and the first binding unchanged.                                      |
| Two descriptors configured for one explicit workspace identifier | Accepted join: both descriptors map to that identifier for that tenant only.                           |

_Table 4: Workspace identity examples and expected outcomes._

### SAFE-TEXT: redaction and capture policy

The default configuration retains the design's normative field names and sets
`store_raw_text = "redacted"`, `redact_before_store = true`, and
`redact_before_embedding = true`. The two redaction requirements cannot be
disabled in the first release. The `encrypted` value is rejected as unsupported
until a separate key-management decision and tests exist. There is no plaintext
mode and no silent fallback: an unsupported value is a configuration error, not
a downgrade.

The `none` value retains identities and minimal structural metadata but drops
all free-text payload fields, including provider metadata, arguments, tool
output, and audit query text. It produces no text-derived embedding or summary,
and explanation reports that text is unavailable rather than returning an empty
string.

The initial detector corpus must cover every class already promised by
memoryd-design.md §13:

- provider API keys with recognized prefixes, with an initial prefix table of
  `sk-`, `sk-proj-`, `sk-ant-`, `ghp_`, `github_pat_`, and the AWS `AKIA` and
  `ASIA` forms. The complete non-whitespace token after a prefix is redacted,
  including short or malformed tokens. This deliberately favours false
  positives over retaining partial credentials;
- OAuth bearer tokens: case-insensitive `Authorization: Bearer` values;
- `Authorization: Basic` values, redacted case-insensitively;
- JSON Web Tokens: three dot-separated base64url segments;
- PEM and OpenSSH private-key blocks, redacted as complete blocks. An
  unterminated opening block redacts through the end of the record;
- SSH private material and sensitive agent or configuration secrets. SSH public
  keys are public material, but whole private-key blocks are still redacted;
- dotenv assignments and password assignments: the entire value is redacted for
  case-insensitive names containing `password`, `passwd`, `secret`, `token`,
  `api_key`, or `private_key`, including `AWS_ACCESS_KEY_ID`,
  `AWS_SECRET_ACCESS_KEY`, and `AWS_SESSION_TOKEN`. Quoted assignment values
  are values, including embedded whitespace;
- cookies: entire `Cookie` and `Set-Cookie` values;
- cloud access and secret credentials;
- credential-bearing database URLs: entire recognized URLs carrying userinfo,
  for `postgres`, `postgresql`, `mysql`, `mongodb`, `redis`, and `rediss`;
- high-entropy blobs, by the fallback rule below.

For JSON, keys are inspected structurally rather than by scanning the value
string. The same case-insensitive sensitive-name rules apply, plus the exact
keys `Authorization`, `Cookie`, and `Set-Cookie`. The entire associated value
is replaced with the marker, including objects and arrays. Unknown fields are
scanned recursively, including arrays and escaped string content.

A conservative entropy fallback applies to maximal ASCII tokens composed of
letters, digits, and the characters `+`, `/`, `_`, `-`, and `=`. Such a token
is redacted when it is at least 32 bytes long and its empirical Shannon entropy
is at least 4.5 bits per byte. Known secret formats are checked independently
and are redacted even below that threshold. High entropy is a warning
heuristic, not a proof that text is secret and not a proof that other text is
safe.

Redaction collects all detected byte intervals from the original input,
validates UTF-8 boundaries, unions overlapping intervals, and replaces each
covered interval with a fixed `[REDACTED]` marker. The marker never includes a
secret hash, prefix, or suffix. The same policy runs for text, nested JSON
strings, and output-bound metadata before persistence, logging, RPC export,
extraction, or embedding. The collector applies it before sending, and the
daemon revalidates push and worker inputs rather than trusting a `redacted`
flag supplied by a caller.

Invalid encoding, an invalid policy, a detector failure, or an exhausted
processing budget rejects the record without recording its raw payload in
diagnostics. The processing ceilings are a 1 MiB UTF-8 content budget per
record, JSON depth 64, and at most 4,096 detected intervals. Each bound applies
to one record, and no batch ceiling is fixed here. Rejection is also per
record: the offending record is not stored and the cursor does not advance past
it, while records already accepted in the same submission keep their own
idempotency keys and stand. The submission reports a failure naming the blocked
record, so a batch never fails silently and never discards its valid
neighbours. Roadmap 2.2.3 owns the concrete batch bound for
`CollectedConversationBatch` under the same revision rule. These are safety
ceilings, not performance promises; later profiling may justify an approved
policy revision.

A blocked record surfaces a persistent sanitized source-health error. The
operator may repair the source, or explicitly skip that exact source position
through an audited action. Skipping records no payload and is never automatic,
and later cursor code must persist the skip so a restart does not turn it into
an endless retry.

Deny patterns are UTF-8, case-sensitive, root-relative globs over `/`-separated
paths, where `*` excludes separators and `**` spans directories. If `globset`
is used, `literal_separator(true)` and `backslash_escape(true)` must be set
explicitly on every platform, and patterns containing backslash path separators
are rejected. Deny patterns support no negation and no allow-pattern
precedence: any match denies. Absolute patterns, parent traversal, and
malformed patterns are rejected at startup. The root that a pattern is relative
to is the workspace's canonical root, as the workspace definition above fixes
it: the same root whose digest is part of the workspace descriptor. For a Git
workspace that is the worktree top level, not the repository's shared common
directory, so a linked worktree's deny patterns are evaluated against that
worktree's own root. Matching considers both the lexical root-relative path and
the resolved target, so an escaping symlink is denied. Configured deny patterns
are limited to 256 per workspace and 1,024 UTF-8 bytes per pattern, and invalid
or excessive policy is rejected at startup rather than truncated.

The mandatory default exclusions are `.env`, `.env.*`, `**/.env`, `**/.env.*`,
`.git/**`, `**/.git/**`, `.ssh/**`, and `**/.ssh/**`. The conservative SSH
directory exclusion deliberately includes public material alongside private
keys. An operator may add exclusions but cannot disable the mandatory defaults
in the first release. Denial happens before file contents are opened, using
capability-based filesystem access to resist path-replacement races.

Transcript containers require a separate rule, because a transcript can copy
file text with no path provenance. When an event identifies a forbidden source
path, that content event is suppressed before capture, including any attached
tool arguments and results. When provenance is absent, path exclusion cannot
recognize copied file text, so secret detectors apply to all text instead. A
source-level deny excludes the whole source. Only a sanitized skipped reason
and a stable source identity are recorded.

The cursor rule has exactly two cases, and every document that states it must
use these words:

- an **intentional exclusion** — a mandatory default exclusion, a configured
  deny-pattern match, or an operator's audited skip of one source position — is
  recorded as deliberately skipped and **does advance** the cursor, because
  re-reading it would only be rejected again;
- a **processing failure** — a failed redaction, a detector or policy error, a
  breached processing ceiling, or a store error — is not acknowledged as
  captured, so the cursor **does not advance** and the record is retried until
  it succeeds or an operator skips it.

"Does not advance" therefore never describes a denial by itself; it describes
the state that makes an audited skip necessary. The later cursor contract owns
the persisted representation of both cases.

Detector version changes are explicit and must not silently redefine provider
idempotency keys. The Codex and Claude keys in memoryd-design.md §7 embed a
`line_hash` without stating whether it hashes the raw or the redacted line, so
that memoryd-design.md §17 records which input each key hashes; hashing the
redacted line would rewrite the key for every line whose redaction differs
between detector versions, which is exactly what this requirement forbids.
Emitted diagnostics use structured reason codes and aggregate counters with
bounded labels, never paths, raw origins, tokens, or payload labels.

Table 5 gives worked detector and exclusion examples.

| Input                                                    | Expected outcome                                                                            |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `sk-ant-abc123` in assistant text                        | The whole token becomes `[REDACTED]`.                                                       |
| `sk-` followed by one character                          | Redacted anyway: short or malformed tokens after a known prefix are still redacted.         |
| `Authorization: Bearer eyJhbGciOi...`                    | The bearer value is redacted; the header name is retained.                                  |
| `Authorization: Basic dTpw`                              | The basic value is redacted, case-insensitively.                                            |
| `password = "hunter two"`                                | The entire quoted value, including its space, is redacted.                                  |
| `{"Authorization": "Basic dTpw"}`                        | The nested value is replaced structurally; the key is retained.                             |
| `{"outer": {"token": "abc"}}`                            | The nested `token` value is replaced by recursive structural scanning.                      |
| `-----BEGIN OPENSSH PRIVATE KEY-----` with no end marker | Redacted through the end of the record.                                                     |
| A 31-byte random alphanumeric token                      | Not redacted by the entropy rule: below the 32-byte floor.                                  |
| A 32-byte token whose entropy is at least 4.5 bits/byte  | Redacted by the entropy rule.                                                               |
| A long base64 body below 4.5 bits/byte                   | Not redacted by the entropy rule; other detectors still apply.                              |
| Two overlapping matches in one record                    | One unioned interval is replaced by a single marker.                                        |
| An event attributed to the path `.env`                   | Denied before the file is opened; no content is persisted and a sanitized reason is logged. |
| A path resolving through a symlink into `.ssh`           | Denied: the resolved target is matched as well as the lexical path.                         |
| A record exceeding the 1 MiB content budget              | Rejected without advancing the cursor and without logging the payload.                      |

_Table 5: Redaction and exclusion examples and expected outcomes._

### SAFE-PURGE: confirmation and backup

Purge is disabled by default. Read-only MCP capabilities expose no purge path,
and tenant-wide purge remains administratively disabled in local mode, matching
memoryd-design.md §12. Workspace purge requires an authenticated tenant, an
allowed workspace, and a purge capability before any preview is computed. Scope
is never inferred from the current directory and never defaults to all tenants.

Before asking for confirmation, the preview shows the canonical tenant and
workspace identifiers, the affected active stores, the irreversible semantics,
and the backup limitations.

Interactive confirmation requires retyping the complete displayed scope. Empty
input, end of input, a mismatch, or a cancellation performs no mutation.
Non-interactive execution requires an explicit scope and an explicit
confirmation bound to the exact preview and policy revision; a bare global
affirmative is insufficient. The later transport slice owns the syntax and
token mechanics. Authority and scope are revalidated immediately before
mutation, and a scope or relevant-state change invalidates the preview.
Confirmation never grants authority.

Before purge the operator must explicitly acknowledge that a restorable
evidence-store backup was made. The acknowledgement is bound to the preview
scope and to a backup identifier or time, without putting filesystem locations
into logs. The operator must quiesce writers for the scope before the backup
and keep them quiesced through the purge. A scope generation is the
monotonically increasing counter of accepted writes to the scoped tenant and
workspace; revalidating it after the backup means re-reading it and refusing
the purge if it moved. The concrete representation and its storage location are
owned by roadmap 5.3.1 with the rest of the purge kernel. Any intervening write
invalidates the acknowledgement and requires a new backup and a new preview. No
automatic backup and no automatic undo are promised.

For SQLite the backup uses the online backup API or the CLI `.backup` command,
never a copy of the main database file while WAL writers are running. For
PostgreSQL the backup is a consistent custom-format `pg_dump` with a documented
restore rehearsal. Where graph storage exists, its directory is backed up under
its own documented quiescence procedure. Minimum deployments must state which
stores are absent; absence is neither an error nor a reason to start graph or
theme services. Separate SQL and graph backups are not an atomic snapshot and
are never described as one.

Backups may contain deleted secrets. They require their own access controls,
retention policy, and operator deletion procedure, and this ADR makes no claim
that purge reaches them.

Purge must hide the target from recall while deletion is in progress and after
a partial failure, must fence concurrent ingestion or repair that could
recreate the target, and must retry idempotently over the same scope. After a
successful purge, a content-free workspace ingestion tombstone is retained:
collection, push ingestion, and repair remain blocked until an authorized
operator explicitly re-enables that scope with an acknowledgement that provider
originals may be captured again. Re-enabling is a separate action, never an
implicit effect of retry or restart. The tombstone contains only tenant and
workspace identifiers plus operation state; it is control metadata, not
retained evidence and not a recall audit.

Success is not reported until every active store deletion is acknowledged, and
incomplete cleanup is reported without raw payloads. These execution
obligations belong to roadmap item 5.3.1. This decision cannot promise
filesystem secure erasure, deletion of provider originals, or removal from
external backups. Only minimal content-free operation status is kept for retry;
scoped evidence and recall audits are deleted.

Table 6 gives worked purge examples.

| Situation                                               | Expected outcome                                                                                    |
| ------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Purge attempted through a read-only MCP capability      | Denied before any preview is computed.                                                              |
| Tenant-wide purge in local mode                         | Administratively disabled unless explicitly configured for migration or offboarding.                |
| Confirmation input left empty                           | No store is mutated.                                                                                |
| Confirmation input ends before a value is read          | No store is mutated.                                                                                |
| Confirmation does not match the displayed scope         | No store is mutated.                                                                                |
| Correct scope retyped, but no backup acknowledgement    | Refused: no backup acknowledgement means no purge.                                                  |
| A write lands after the backup and before the purge     | The acknowledgement is invalidated; a new backup and preview are required.                          |
| Two tenants share a workspace label                     | Only the confirmed tenant's workspace is removed from active stores.                                |
| The same purge is retried after a partial store failure | Idempotent retry over the same scope; success is reported only when all deletions are acknowledged. |
| Ingestion runs after a successful purge                 | Blocked by the content-free tombstone until an operator explicitly re-enables the scope.            |
| Operator inspects a backup made before purge            | The backup may still contain the removed evidence; deletion is a separate manual step.              |

_Table 6: Purge examples and expected outcomes._

## Goals and non-goals

- Goals:
  - Close the ten defaults roadmap item 1.1.2 names: repository-origin
    normalization, repository-root path hashing, non-Git workspace hashing,
    collision handling, operator overrides, first redaction detectors,
    deny-pattern behaviour, raw-text storage mode, purge confirmation defaults,
    and pre-purge backup expectations.
  - Let provider adapters ingest safely without waiting for graph, theme, or
    Axinite write-back decisions.
  - Preserve the tenant authority boundary and both storage backends.
- Non-goals:
  - Implement workspace derivation, redaction, deny patterns, or purge in this
    ADR. Roadmap items 2.2.4, 2.2.3, and 5.3.1 own that work.
  - Decide encrypted raw-text mode. It stays unsupported until a separate
    key-management decision exists, and selecting it must fail rather than
    fall back silently.
  - Decide the Axinite projection write-back policy, the Chutoro checkpoint
    format, source-health freshness defaults, or recall-audit retention.
  - Provide automatic backup, automatic restore, or any undo window.
  - Promise secure erasure of filesystem blocks, deletion of provider
    originals, or removal of evidence from operator or provider backups.
  - Claim that any detector corpus finds every secret.

## Migration plan

1. This ADR records the accepted policy and closes the corresponding items in
   terms-of-reference.md, memoryd-design.md, and RFC 0001.
2. Roadmap item 2.2.4 implements workspace derivation and registration,
   including the descriptor codec, Git and non-Git discovery adapters, the
   collision error, and configured override and join handling.
3. Roadmap item 2.2.3 implements the redaction policy, the detector corpus, the
   interval renderer, deny-pattern evaluation, and the raw-text modes.
4. Roadmap item 5.3.1 implements purge across evidence, graph, Qdrant, and
   checkpoint state, including the preview, confirmation, backup
   acknowledgement, fencing, tombstone, and idempotent retry.
5. Each slice repeats the repository sweep for existing helpers or ports before
   adding new ones, runs the repository gates, and records its own acceptance
   evidence.
6. Until those slices land, user-facing documentation labels these defaults as
   planned rather than active.

## Known risks and limitations

- Identity over-normalization can silently merge unrelated repositories. The
  mitigation is preserving protocol and path semantics, preferring false
  separation, and requiring an explicit descriptor join for intentional
  equivalence.
- A digest collision could present two descriptors as one key. The mitigation
  is comparing the full canonical descriptor on registration and raising a
  typed collision error instead of merging.
- Persisting the canonical root path is sensitive metadata that could disclose
  local directory structure. The mitigation is keeping it inside the authorized
  registry, keeping it out of logs and public labels, and subjecting it to the
  same access and purge policy as other workspace metadata.
- Redaction cannot detect every secret. The mitigation is a stated detector
  corpus, conservative exclusions, deliberate over-detection of known prefixes,
  and fail-closed processing errors. No universal secrecy claim is made.
- Heuristic detection can remove useful text. The mitigation is publishing
  boundary examples and treating the entropy rule as a warning heuristic rather
  than a proof.
- Backups preserve data that purge removes. The mitigation is documenting
  retention and manual destruction of copies and never promising secure erasure
  or automatic undo. A consistent SQL backup is not a cross-store atomic
  snapshot.
- A documentation decision can be mistaken for shipped behaviour. The
  mitigation is labelling user-facing defaults as planned until the owning
  runtime slice implements and verifies them.
- Cross-machine identity stability is not promised, so two machines with
  different local roots will register different workspaces for the same
  repository. The mitigation is the explicit operator join.

## Outstanding decisions

- The exact key-management design for an `encrypted` raw-text mode, including
  provisioning, rotation, recovery, and audit. Until it exists, `encrypted` is
  rejected as unsupported.
- The operator backup format used before irreversible purge (memoryd-design.md
  §17), which remains open while the restore rehearsal procedure is unwritten.
- Whether recall-audit retention ever stores redacted query text by default
  (memoryd-design.md §17).
- The transport syntax and confirmation-token mechanics for non-interactive
  purge, owned by a later transport slice.
- The workspace-registry storage surface (memoryd-design.md §17). This ADR
  requires registration to persist and compare the full descriptor binding, but
  the design's evidence schema has no workspace table; roadmap 2.2.4 owns the
  table, its tenant-scoped uniqueness rule, the configured override and
  explicit-join configuration surface, and whether the registry is purged with
  the workspace or retained as control metadata.
- Whether the Codex and Claude idempotency keys' `line_hash` hashes the raw or
  the redacted line (memoryd-design.md §§7 and 17). Roadmap 2.2.3 owns the
  answer, because hashing redacted text would let a detector-version change
  redefine an idempotency key.

## Architectural rationale

The three contracts share one authority model: the authenticated tenant is the
only source of authority, and every other field is data. Workspace identity is
derived data that must be compared rather than trusted; redaction operates on
data that must never leave the trust boundary unredacted; purge acts only after
authority, scope, confirmation, and backup are all established independently.

Placing the policy in pure domain decisions keeps the hexagonal boundary from
ADR 005 intact. Filesystem and Git discovery, SQL bindings, capability checks,
terminal input, and backup tooling remain adapters, so the same policy can be
exercised by both ingestion ports and by later repair and recall paths without
duplicating normalization or redaction logic.

Preferring false separation over silent merging matches the design's stated
collision requirement, and treating the decision as recorded-but-unenforced
until its owning slice lands matches the roadmap's separation between decision
slices and runtime slices.

## References

- `docs/roadmap.md`, roadmap items 1.1.1, 1.1.2, 2.2.3, 2.2.4, and 5.3.1.
- `docs/terms-of-reference.md`, sections 7.2, 8.1, 8.2, and 9.
- `docs/memoryd-design.md`, sections 5.4, 8.4, 13, 14, 15, and 17.
- `docs/rfcs/0001-standalone-evidence-inbox.md`, the Current state, Proposed
  design, and Compatibility and migration headings.
- `docs/adr-005-hexagonal-architecture-boundary.md`.
- `docs/adr-006-tenant-isolation-and-corbusier-context.md`.
- `docs/adr-007-standard-conversation-ingestion-port.md`.
- `docs/adr-013-evidence-store-engine-and-migration-policy.md`.
- `docs/execplans/1-1-2-record-local-safety-defaults.md`.
- `docs/documentation-style-guide.md`.
- Git clone documentation, Git URLs, <https://git-scm.com/docs/git-clone>.[^1]
- SQLite online backup API, <https://www.sqlite.org/backup.html>.[^2]
- PostgreSQL `pg_dump`,
  <https://www.postgresql.org/docs/current/app-pgdump.html>.[^3]
- `globset` documentation,
  <https://docs.rs/globset/latest/globset/>.[^4]

[^1]: Used to confirm that Git documents distinct URL, SCP-like, and local path
    forms, which supports preserving path semantics rather than guessing
    cross-transport equivalence.

[^2]: Used to confirm that SQLite supplies an online backup facility, so copying
    an active database file is not the selected pre-purge procedure.

[^3]: Used to confirm that PostgreSQL documents consistent concurrent exports
    through `pg_dump`.

[^4]: Used to confirm that `globset` documents the glob syntax and separator
    options that the deny-pattern contract requires adapters to configure
    explicitly.

## Revision note

Initial accepted revision (2026-10-15). Transfers the SAFE-ID, SAFE-TEXT, and
SAFE-PURGE contracts from the roadmap 1.1.2 ExecPlan into an accepted decision
record, adding worked examples with explicit expected outcomes for identity
derivation, detector behaviour, and purge confirmation. Runtime enforcement,
proofs, and verification remain with roadmap items 2.2.3, 2.2.4, and 5.3.1;
nothing in this ADR is claimed as implemented.

EP-M3 revision (2026-10-16). States the cursor rule once, in the wording the
users' guide, RFC 0001, and the ExecPlan now reuse. Fixes the deny-pattern root
to the workspace's canonical root, so linked worktrees evaluate their own
patterns. Names the scope generation as the scoped-write counter. Records the
workspace-registry storage surface and the `line_hash` input as outstanding
decisions with owners, because this slice may not choose a storage schema.
