# Record local safety defaults for workspace, redaction, and purge

Status: IN PROGRESS — EP-M1 complete; EP-M2 and EP-M3 remaining.

This ExecPlan is a living document. Keep Constraints, Tolerances, Risks,
Progress, Surprises & discoveries, Decision log, Outcomes & retrospective,
Conformance basis, and Verification plan current at every milestone.

## Purpose / big picture

Record the decisions required by roadmap item 1.1.2 so provider implementors
can resolve a workspace, exclude forbidden content, redact evidence, and refuse
unsafe purge requests without waiting for graph, theme, or Axinite write-back
policy. Success is an accepted, internally consistent local-safety ADR and
linked operator and developer guidance with concrete examples and verification
handoffs. This task records contracts; roadmap 2.2.3 implements redaction,
2.2.4 implements workspace derivation, and 5.3.1 implements cross-store purge.
It does not turn the current application stub into those features.

The planning PR contained this plan and its documentation-index entry.
Execution was approved on 2026-10-15 by the user's explicit instruction to
implement the plan, which the `sdlc-implementation` contract treats as
supplying that approval. The defaults are therefore ratified as proposed, in
ADR 015, with no requested changes recorded.

## Constraints

Preserve the tenant authority boundary: identity is the pair of authenticated
tenant and workspace, never a workspace string alone. Provider fields cannot
supply authority. Preserve ADRs 005–007 and 013, local-first operation, both
SQLite and PostgreSQL, and the split between push and collected ingestion. No
graph, theme, or Axinite write-back decision may become a prerequisite.

This slice edits documentation only. Do not add runtime modules, dependencies,
CLI flags, migrations, executable fixtures, or speculative proof scaffolding.
Do not mark roadmap 1.1.2 done in the planning PR. Never claim a policy has
runtime enforcement merely because its decision record is accepted.

Use the requested skills: `execplans`, `codegraph-mcp`,
`hexagonal-architecture`, and `rust-router`; use `firecrawl-mcp` for external
research and `logisphere-experts` for review. For later verification handoffs,
load `rust-verification`, then `verus` or `proptest` as appropriate. Read each
skill's `SKILL.md` from the available skill catalogue. The explicit prohibition
on `assume`, `admit`, trusted/external proof bodies, or equivalent shortcuts
applies throughout. Unproved claims stay in prose until a real proof milestone.

## Tolerances (exception triggers)

Stop and record a proposed deviation if this task needs executable code, a new
library, storage schema, transport, cryptographic key management, a changed
trust boundary, or a contradiction of ADR 013. Stop if any safety default
remains materially ambiguous after review. Ordinary wording and cross-reference
repairs within the named documents are permitted. Do not widen scope to repair
unrelated baseline gate failures; record the exact failure and request a scoped
decision. Infrastructure refusal is not evidence of a source defect.

## Risks

Identity over-normalization can silently merge unrelated repositories (high
impact, medium likelihood). Preserve protocol and path semantics, prefer false
separation, and require explicit descriptor joins for intentional equivalence.
Hashes are identifiers, not anonymization or proof of uniqueness.

Redaction cannot detect every secret (high impact, high likelihood). Specify an
initial detector corpus, conservative exclusions, and fail-closed processing
errors; make no universal secrecy claim. Heuristics can remove useful text.

Backups preserve data that purge removes (high impact, medium likelihood).
Explain retention and manual destruction of copies, and never promise secure
erasure or automatic undo. A consistent SQL backup is not a cross-store atomic
snapshot.

A documentation decision can be mistaken for shipped behaviour (medium impact,
medium likelihood). Label user-facing defaults as planned until the owning
runtime slice implements and verifies them.

## Progress

- [x] (2026-10-14) Read repository instructions, prerequisite ADR, and roadmap;
  rename the local branch to `1-1-2-record-local-safety-defaults`.
- [x] (2026-10-14) Start two Wyvern reconnaissance agents and primary-source
  research; draft the contracts and proof sketches.
- [x] (2026-10-14) Review all six community-of-experts perspectives and
  revise collision witnesses, SSH identity, recursive denies, and purge fencing.
- [x] (2026-10-14) Format the draft; initial `make check-fmt`,
  `make markdownlint`, `make nixie`, and full `make lint` passed.
- [x] (2026-10-14) `make test` passed: 146 tests, zero skipped.
- [x] (2026-10-14) Publication readback complete; planning PR #46 opened as a
  draft with the plan link and the planning session reference.
- [x] (2026-10-15) Execution approved by the user's explicit instruction to
  implement this plan, which the `sdlc-implementation` contract treats as
  supplying that approval.
- [x] (2026-10-15) EP-M1: ADR 015 accepted; design §5.4, §13, §14, §15, and
  §17, TOR §7.2, §8.2, and §9, RFC 0001's Proposed design and Compatibility and
  migration headings, and the contents index synchronized.
- [ ] EP-M2: Publish operator/developer contracts and verification handoffs.
- [ ] EP-M3: Reconcile acceptance evidence and mark roadmap 1.1.2 done.

## Surprises & discoveries

The checked-out baseline already marks 1.1.1 complete and contains accepted ADR
013. The application is still `src/main.rs` with a greeting and
`tests/stub.rs`; there is no safety kernel to prove yet. Creating runtime
redaction or purge here would duplicate explicitly later roadmap work.

The cited RFC 0001 uses named headings, not numbered sections. Interpret the
request's §§3 and 7 through its Current state, Proposed design, and
Compatibility and migration headings; link named headings when editing it.
Design §8.4 is Qdrant layout; workspace derivation is specified in §5.4.

The Makefile is more authoritative than the abbreviated commands in AGENTS:
`make lint` runs rustdoc, Clippy, and Whitaker; `make test` selects nextest
when available. Development targets use `tools/dev-fast/config.toml`.
Verification must not inherit that Cranelift configuration.

ADR 014 was allocated by an unrelated change (main-owned CodeScene coverage
publication) before this branch was published, so the plan's fallback applied
and the local-safety ADR is **015**. The same change never added its ADR to
`docs/contents.md`, leaving an index gap; EP-M1 repairs that omission alongside
the ADR 015 entry.

TOR §9 records the first-release redaction question as explicitly open, with "a
documented default deny list, secret detector set, and override model" as its
resolution criteria. ADR 015 is that resolution, so it closes a recorded open
question rather than overriding a settled norm.

The TOR contains no purge-confirmation or pre-purge-backup requirement at all;
those norms live in design §12 (the confirmation string and the local-mode
disablement of `PurgeTenant`) and §13 (irreversibility, scope display, and the
ADR 013 backup expectation). The plan's Table 1 cites "TOR §7.2" for
SAFE-PURGE, which is accurate only for the deletion-scope sentence; the
confirmation and backup obligations trace to the design and ADR 013.

No source defines deny-pattern syntax, workspace-descriptor serialization,
tombstone semantics, or redaction processing ceilings. ADR 015 originates
those, which is why it states them as decisions with explicit examples rather
than as restatements of upstream norms.

## Decision log

- (2026-10-14, planning) Keep 1.1.2 a decision-record slice, matching 1.1.1
  and the explicit 2.2.3/2.2.4/5.3.1 ownership. Preserve test and proof
  obligations as acceptance requirements for those consumers.
- (2026-10-14, proposed) Prefer conservative, versioned identity derivation,
  redacted-only text by default, and explicit scope-bound purge confirmation.
  These are proposals awaiting approval, not accepted architectural decisions.
- (2026-10-14, proposed) Defer encrypted raw text because key provisioning,
  rotation, and recovery are independent decisions. Reject its selection until
  implemented rather than falling back silently.
- (2026-10-15, ratified) All three proposals are accepted as ADR 015. Approval
  came from the user's explicit instruction to implement this plan; the
  contracts are transferred with worked examples and expected outcomes, and
  remain labelled unenforced until 2.2.3, 2.2.4, and 5.3.1 implement them.
- (2026-10-15, EP-M1) Numbered the ADR 015 rather than 014 because 014 was
  already allocated, and repaired the missing ADR 014 index entry in the same
  commit so the contents list is complete.

## Outcomes & retrospective

The draft has completed Wyvern reconnaissance and all six expert perspectives.
All review findings were revised and closure-checked. Full `make lint` passed,
including rustdoc, Clippy, and Whitaker. `make test` passed all 146 tests, with
zero skipped. Cargo emitted the pre-existing redundant `package.homepage`
manifest warning; no manifest change belongs to this documentation task.

EP-M1 completed on 2026-10-15. ADR 015 answers all ten requested defaults with
worked examples and explicit expected outcomes, and the design, terms of
reference, RFC 0001, and contents index now reference it. The most valuable
episode was the ADR-number collision: the plan reserved `adr-014` conditionally
and the fallback triggered, which also exposed a missing contents entry for the
ADR that had taken 014.

No runtime behaviour has changed, no proof is claimed, and roadmap 1.1.2
remains open until EP-M3. Runtime verification stays with roadmap items 2.2.3,
2.2.4, and 5.3.1, and the ADR states plainly that nothing is enforced yet.

## Context and orientation

Start with `docs/contents.md` and `docs/repository-layout.md`. The normative
sources are `docs/terms-of-reference.md`, `docs/memoryd-design.md`,
`docs/roadmap.md`, `docs/rfcs/0001-standalone-evidence-inbox.md`, and
`docs/adr-013-evidence-store-engine-and-migration-policy.md`. RFC 0006,
`docs/rfcs/0006-epistemic-health-empiricism-falsification-and-semirings.md`,
identifies later epistemic work; it supplies no reason to delay local safety.

A workspace is a tenant-owned evidence namespace. A canonical root is the
absolute filesystem-resolved directory selected by an operator or discovered as
the Git worktree top level. A descriptor is the structured input to identity
derivation. A collision means different descriptors request the same identity
within one tenant. A deny pattern excludes source material before capture. A
purge removes a specified scope from active stores; it cannot erase external
provider transcripts or operator backup copies.

Read `docs/documentation-style-guide.md` before creating the ADR. Reserve
`docs/adr-014-local-safety-defaults.md` after checking that 014 is still free;
if another change allocated it, use the next free number and update all links.
No existing safety abstraction is implemented at the baseline. Future runtime
work must repeat the repository sweep before adding helpers or ports.

## Conformance basis

Baseline: commit `8639da6fefaf1a03ab74901d4c1af322dd310f0e` for all repository
sources below. Re-read their current revisions before execution. There are no
upstream numeric requirement IDs for these individual defaults; the identifiers
below are local trace labels, not invented upstream IDs.

| Contract                                     | Source                                                  | Milestone and observable acceptance                                     |
| -------------------------------------------- | ------------------------------------------------------- | ----------------------------------------------------------------------- |
| SAFE-ID: identity and collision isolation    | TOR §8.2; design §5.4; ADR 006                          | EP-M1: complete identity examples and collision decisions               |
| SAFE-TEXT: exclusions, detectors, text modes | TOR §7.2; design §13 and §14; RFC 0001 Proposed design  | EP-M1: every detector and failure mode has a decision                   |
| SAFE-PURGE: authority, confirmation, backup  | TOR §7.2; design §§8.4,12–13; ADR 013                   | EP-M1: both stores and failure paths covered                            |
| SAFE-BOUNDARY: ports and first-value scope   | ADRs 005–007; roadmap 1.1/2.2; design §§16–17; RFC 0006 | EP-M2: reusable policy ownership and later owners explicit              |
| SAFE-VERIFY: proof/testing relationship      | design §15; AGENTS; this request                        | EP-M2: obligations, witnesses, negative controls, and commands recorded |
| SAFE-CLOSE: decision completion              | roadmap 1.1.2                                           | EP-M3: accepted ADR, consistent guides, gates, then done marker         |

_Table 1: Requirements traced to decision and acceptance evidence._

## Proposed safety contracts

### SAFE-ID: workspace derivation

Use a versioned descriptor with distinct Git and non-Git tags. Hash with
SHA-256 and retain the full 256-bit digest in lowercase hexadecimal; do not
truncate for storage or authorization. Serialize fields as UTF-8 bytes with
explicit unsigned 64-bit big-endian byte lengths, preceded by the ASCII domain
separator `memoryd.workspace.v1` and the descriptor tag. This avoids delimiter
ambiguity. Define overflow as an error. The tenant stays a separate key field.

For Git, fields are normalized origin, the SHA-256 root digest, and profile.
For non-Git, fields are root digest and profile. Root digests use domain
separator `memoryd.root.v1` and the length-prefixed canonical absolute path.
Absent profile is the empty string; an explicitly empty profile means the same
thing. Preserve profile case and Unicode code points. A root move, different
worktree, origin change, or profile change yields a new descriptor. Symlinks
resolving to the same worktree root yield the same root. Do not use Git's
shared common directory to collapse linked worktrees.

Obtain `remote.origin.url` locally without fetching, executing remote helpers,
or resolving network aliases. For URL forms, lowercase ASCII scheme and DNS
host, preserve repository path case and percent escapes, remove password/token
credentials from URL userinfo and reject query/fragment components. Preserve
the SSH username as an identity field because it can select a different
server-side repository namespace. For HTTP URLs, omit authentication userinfo;
the authenticated tenant and local root remain mandatory identity inputs.
Preserve explicit ports, including default ports, and preserve trailing slashes
and `.git`. Keep the scheme: SSH and HTTPS are not automatically
interchangeable. Treat SCP-like syntax as a separate tagged form, preserving
relative versus absolute path semantics and user-relative paths; usernames that
affect home-relative resolution must remain identity inputs but must never
appear in diagnostics. Require an explicit descriptor join to equate SCP and
URL forms or different transports. Reject ambiguous or unsupported
remote-helper syntax. Missing origin is a Git descriptor with an explicit
absent-origin tag, not a guessed remote. Local/file origins must resolve
relative to the repository root and use a separate local-origin tag;
unavailable targets fail closed. Never log raw origins or credentials. Redacted
display labels are separate from identity.

Resolve roots through filesystem adapters, then pass validated values into the
pure policy. Missing/inaccessible roots, non-UTF-8 paths, and unresolved roots
are explicit errors for the first implementation. Do not hash lossy strings. No
case folding or Unicode normalization of local paths is imposed beyond the
platform's canonical path result. Cross-machine identity stability is not
promised. Hashing paths does not protect guessable paths from disclosure.

Persist the version and descriptor binding at registration, including the
canonical root path before hashing. Equality checks compare that original
canonical path as well as origin and profile, never only the root digest. Keep
this sensitive binding metadata local to the authorized registry; it is not a
public workspace label. Compare the full canonical descriptor when a
tenant/workspace key already exists: equal means idempotent registration;
unequal means a typed collision error and no write. Do not assume SHA-256 is
injective or use another hash as proof of equality. Descriptor storage is
sensitive metadata and follows access and purge policy; raw URL credentials are
never stored. Namespace derived IDs and configured IDs separately so user
strings cannot imitate a derived key.

Operator configuration may map an exact descriptor to an explicit workspace ID,
scoped to one authenticated tenant. An alias is a display name only. An
intentional join requires an explicit list of descriptors authorized to share
that configured ID; never learn this list from provider payloads or auto-merge
on collision. Configuration changes do not move existing evidence. Relocation
and reclassification of historical evidence require a separate migration.
Duplicate/conflicting mappings are startup errors. Audit changes with opaque
identifiers, outcome, policy version, and reason code, never raw paths/tokens.

### SAFE-TEXT: redaction and capture policy

Default `store_raw_text = "redacted"`, `redact_before_store = true`, and
`redact_before_embedding = true`. The two redaction requirements cannot be
disabled in the first release. `none` retains identities and minimal structural
metadata but drops all free-text payload fields, including provider metadata,
arguments, tool output, and audit query text. It produces no text-derived
embedding or summary, and explanation reports text unavailable. `encrypted` is
rejected as unsupported until a separate key-management decision and tests
exist. There is no plaintext mode or silent fallback.

The initial detector corpus must cover every class already promised by design
§13: provider API keys with recognized prefixes, OAuth bearer tokens, JWTs,
PEM/OpenSSH private-key blocks, SSH private material, dotenv assignments,
password assignments, Cookie/Set-Cookie and authorization values, cloud access
and secret credentials, credential-bearing database URLs, and high-entropy
blobs. SSH public keys are public material, but redact whole private-key blocks
and sensitive agent/config secrets. The initial prefix table is `sk-`,
`sk-proj-`, `sk-ant-`, `ghp_`, `github_pat_`, and AWS `AKIA`/`ASIA`; redact the
complete non-whitespace token after a prefix, including short or malformed
tokens. This deliberately favours false positives over retaining partial
credentials. Recognize case-insensitive `Authorization: Bearer` and
`Authorization: Basic` values, three dot-separated base64url JWT segments, and
complete PEM/OpenSSH private key blocks; an unterminated opening block redacts
through end of record. For assignments, redact the entire value for
case-insensitive names containing `password`, `passwd`, `secret`, `token`,
`api_key`, or `private_key`; include `AWS_ACCESS_KEY_ID`,
`AWS_SECRET_ACCESS_KEY`, and `AWS_SESSION_TOKEN`. Treat quoted assignment
values as values, including embedded whitespace. Redact entire
Cookie/Set-Cookie values and entire recognized database URLs carrying userinfo
(`postgres`, `postgresql`, `mysql`, `mongodb`, `redis`, `rediss`). For JSON,
inspect keys structurally using the same case-insensitive sensitive name rules
plus exact `Authorization`, `Cookie`, and `Set-Cookie` keys, replacing the
entire associated value, including objects and arrays, with the marker; do not
rely on scanning the value string alone. Unknown fields are recursively
scanned, including arrays and escaped string content. The ADR must turn these
rules into a versioned positive/negative corpus and explicit grammar choices,
including malformed JSON, escaped strings, and multiline values; unresolved
choices prevent E1 acceptance. Never imply all providers or all secret formats
are detected.

Propose a conservative entropy fallback for maximal ASCII tokens composed of
letters, digits, `+`, `/`, `_`, `-`, and `=`: at least 32 bytes and empirical
Shannon entropy at least 4.5 bits per byte. Exact known secret formats are
checked independently, even below that threshold. Publish boundary examples at
31/32 bytes and just below/at/above the threshold. High entropy is a warning
heuristic, not a proof that text is secret or safe. Detector version changes
must be explicit and must not silently redefine provider idempotency keys.

Collect all detected byte intervals from the original input, validate UTF-8
boundaries, union overlaps, and replace each covered interval with a fixed
`[REDACTED]` marker. Never include secret hashes, prefixes, or suffixes in the
marker. Run the same policy for text, nested JSON strings, and output-bound
metadata before persistence, logging, RPC export, extraction, or embedding. The
collector applies it before sending; the daemon revalidates push and worker
inputs rather than trusting a `redacted` flag. Invalid encoding, invalid
policy, detector failure, or exhausted processing budget rejects the record
without recording its raw payload in diagnostics. Propose a 1 MiB UTF-8 content
budget per record, JSON depth 64, and at most 4,096 detected intervals;
exceeding any bound rejects the record without advancing its cursor. Limit
configured deny patterns to 256 per workspace and 1,024 UTF-8 bytes per
pattern. Reject invalid/excess policy at startup. Do not truncate first and
thereby turn incomplete scanning into success. A blocked record surfaces a
persistent sanitized source-health error; the operator may repair the source or
explicitly skip that exact source position through an audited action. Skipping
records no payload and is never automatic; later cursor code must persist the
skip so restart does not turn it into an endless retry. These are safety
ceilings, not performance promises; later profiling may justify an approved
policy revision. Emit structured reason codes and aggregate counters with
bounded labels, never path, raw origin, token, or payload labels.

Deny patterns are UTF-8, case-sensitive, root-relative globs over `/`-separated
paths, with `*` excluding separators and `**` spanning directories. If using
`globset`, set `literal_separator(true)` and `backslash_escape(true)`
explicitly on every platform; reject backslash path separators in patterns. No
negation or allow-pattern precedence is supported; any match denies. Reject
absolute patterns, parent traversal, and malformed patterns at startup. Match
both the lexical root-relative path and resolved target; an escaping symlink is
denied. Default exclusions are `.env`, `.env.*`, `**/.env`, `**/.env.*`,
`.git/**`, `**/.git/**`, `.ssh/**`, and `**/.ssh/**`; the conservative SSH
directory exclusion includes public material as well as private keys; an
operator may add exclusions, not disable mandatory defaults in the first
release. Deny before opening file contents, with capability-based filesystem
access to resist path replacement races.

Transcript containers require a separate rule: if an event identifies a
forbidden source path, suppress that content event, including attached tool
arguments/results, before capture. If provenance is absent, do not claim path
exclusion can recognize copied file text; apply secret detectors to all text. A
source-level deny excludes the whole source. Record only a sanitized skipped
reason and stable source identity; intentional exclusion may advance its
cursor, whereas processing failure must not acknowledge successful capture. The
later cursor contract must make retry versus intentional skip explicit.

### SAFE-PURGE: confirmation and backup

Default purge is disabled through read-only MCP capabilities; tenant-wide purge
remains administratively disabled in local mode. Workspace purge requires an
authenticated tenant, allowed workspace, and purge capability before preview.
Never infer scope from current directory or choose all tenants implicitly. Show
canonical tenant/workspace IDs, affected active stores, irreversible semantics,
and backup limitations before asking for confirmation.

Interactive confirmation requires retyping the complete displayed scope; empty
input, EOF, mismatch, or cancellation makes no mutation. Non-interactive
execution requires explicit scope and explicit confirmation bound to the exact
preview and policy revision; a bare global yes is insufficient. The later
transport slice owns syntax and token mechanics. Revalidate authority and scope
immediately before mutation. Scope or relevant state changes invalidate the
preview. Confirmation never grants authority.

Require an explicit operator acknowledgement that a restorable evidence-store
backup was made before purge; bind the acknowledgement to the preview scope and
backup identifier/time without putting filesystem locations into logs. The
operator must quiesce writers for the scope before backup and keep them
quiesced through purge; revalidate the scope generation after backup. Any
intervening write invalidates the acknowledgement and requires a new backup and
preview. No automatic backup or undo is promised. For SQLite use the online
backup API or CLI `.backup`, not a copy of the main file while WAL writers run.
For PostgreSQL use a consistent custom-format `pg_dump` and a documented
restore rehearsal. Where graph storage exists, also back up its directory under
its documented quiescence procedure. State which stores are absent in a minimum
deployment; absence is not an error or a reason to start graph/theme services.
Do not claim that separate SQL and graph backups are an atomic snapshot.
Backups may contain deleted secrets and require their own access controls,
retention, and operator deletion.

Purge must hide the target from recall while deletion is in progress and after
partial failure, fence concurrent ingestion/repair that could recreate it, and
retry idempotently over the same scope. A successful purge retains a
content-free workspace ingestion tombstone: collection, push ingestion, and
repair remain blocked until an authorized operator explicitly re-enables that
scope with an acknowledgement that provider originals may be captured again.
Re-enabling is a separate action, not an implicit effect of retry or restart.
The tombstone contains only tenant/workspace IDs and operation state; it is
control metadata, not retained evidence or a recall audit. Do not report
success until all active store deletions are acknowledged. Report incomplete
cleanup without raw payloads. These execution obligations belong to 5.3.1; this
decision cannot promise filesystem secure erasure, deletion of provider
originals, or removal from external backups. Keep only minimal content-free
operation status where needed for retry; scoped evidence and recall audits are
deleted.

## Verification plan

### Abstract model and proof sketches

Define these specifications independently of Rust structures. Let T be an
authenticated tenant, D a versioned descriptor, K a workspace key, R a partial
map from (T,K) to authorized descriptor sets, X a sequence of input bytes, S a
set of valid half-open sensitive intervals, and P a purge scope. An authorized
explicit join is represented by membership in the configured descriptor set,
not by assuming descriptors equal. The representation relation must map actual
runtime records and variants to these abstract values when implemented.

V-ID: registration preserves existing bindings and never adds an unapproved D
to R(T,K), even if the digest function maps distinct descriptors to K.
Preconditions are a well-formed registry and validated configuration.
Postconditions distinguish unchanged collision/error from insert or idempotent
success. Proposed lemmas are preservation under one registration and induction
over arbitrary registration sequences. Prove the registration transition, not
collision resistance. Use Verus in the 2.2.4 handoff if this kernel remains as
specified, with a refinement argument from executable binding checks. Witness:
two distinct descriptors forced to the same final digest, and two distinct
canonical roots forced to the same root digest; negative control: remove
descriptor comparison and require verification/tests to reject it.

V-TEXT: for every source byte position covered by the union of S, no output
segment copies that position; all copied segments preserve source order and
come from its complement. Preconditions require in-bounds UTF-8 intervals;
invalid intervals return an error. This is provenance exclusion, not a claim
that an equal byte value cannot occur elsewhere. Sketch coverage preservation
by interval union, complement partition, and ordered rendering. Prove the
unbounded interval transformation in Verus during 2.2.3, then connect runtime
byte offsets and renderer branches to the model. Detector completeness is an
external limitation, not a premise claiming all secrets were found. Witnesses:
empty/full/overlapping/adjacent intervals and multibyte text; negative control:
copy one covered byte and require a counterexample or failed proof.

V-PURGE: deleting P preserves every item outside P; rejected authorization or
confirmation leaves state unchanged; repeated deletion has the same result as
one deletion. Model records with explicit tenant and workspace ownership and a
pure scoped-delete transition. Sketch partition preservation and idempotence
for arbitrary finite collections. During 5.3.1, use Verus for this pure kernel
and its runtime refinement, not for SQL, asynchronous retries, or physical
erasure. Witness: two tenants with the same workspace label and a third
workspace in the target tenant. Negative control: drop the tenant predicate.

These are candidate semantic lemmas, not a fixed proof decomposition. Refine or
remove them when executable verification conditions exist. No claim is
discharged in 1.1.2. Do not create solver scaffolding to make prose look
proved. The future proof milestones must add pinned `tools/verus/VERSION` and
`tools/verus/SHA256SUMS`, use `rust-prover-tools`, and record zero failed goals
and the trusted dependency audit. Their explicit acceptance commands are:

```bash
prover-tools verus run --repo-root . --proof-file verus/workspace_binding.rs
prover-tools verus run --repo-root . --proof-file verus/redaction_intervals.rs
prover-tools verus run --repo-root . --proof-file verus/purge_scope.rs
```

Run each in its owning slice, alongside `make check-fmt`, `make lint`, and
`make test`; these proof files and a project proof target do not yet exist. A
spec mirror alone is insufficient: document the production mapping, verify the
executable kernel or a substantive refinement, and exercise integration. No
proof milestone is part of the documentation execution below.

### Tests, assumptions, and residual gaps

External assumptions are correct Git discovery, filesystem canonicalization,
SHA-256 interface determinism (never injectivity), authenticated request
context, documented glob semantics, consistent database backup interfaces, and
storage deletion acknowledgements. Exercise adapter interactions against real
boundaries; do not try to prove third-party implementations.

For 2.2.4, plan `tests/workspace_safety.rs` with `rstest` cases for transport
separation, explicit origin joins, absent origin, linked worktrees, root moves,
non-Git roots, profiles, invalid encodings, cross-tenant keys, forced root and
final-digest collisions, different SSH usernames, and override conflicts. Use
`proptest` for deterministic serialization, field-boundary separation, and
registration sequences. Include zero, one, and many descriptors with both
success and collision classes; report generator classification and reject
excessive filtering. These tests complement V-ID.

For 2.2.3, plan `tests/redaction_safety.rs`: each detector has positive,
negative, malformed, boundary, and nested-metadata cases; test deny matches,
symlink escapes, policy errors, `none`, `redacted`, and rejected `encrypted`.
Use `googletest` assertions with `rstest`, and `pretty_assertions` for readable
expected structures. Use `proptest` for generated intervals and strings,
including overlaps and Unicode. Do not infer detector completeness from the
renderer proof. Use `insta` only for stable human/JSON diagnostics where
multiple output formats exist, with synthetic credentials and deterministic IDs.

For 5.3.1 and the first runnable safety workflow, plan
`tests/local_safety_e2e.rs` and `tests/features/local_safety.feature` with
`rstest-bdd`. Preserve these concrete acceptance scenarios in the later plan:

```gherkin
Feature: Local safety defaults
  Scenario: Forbidden source content is not captured
    Given an authenticated tenant and a workspace denying .env files
    When an adapter offers an event attributed to .env
    Then no content is persisted or sent for embedding
    And a sanitized exclusion reason is recorded

  Scenario: A derived identifier collides
    Given two different descriptors forced to the same tenant workspace key
    When the second descriptor is registered without an explicit join
    Then registration fails and the first binding is unchanged

  Scenario: Purge confirmation cannot cross tenants
    Given two tenants with the same workspace label and stored evidence
    When one tenant confirms its exact authorized purge scope
    Then only that tenant workspace is removed from active stores

  Scenario: Purge has no interactive confirmation
    Given a purge preview and a backup acknowledgement
    When confirmation is empty or input ends
    Then no store is mutated
```

Run Red-Green-Refactor in those code slices: add a focused scenario, observe
its intended assertion failure, implement the smallest change, rerun, then
refactor and run all gates. Use the owning slice's Make target and record exact
test filters after its runner exists. Also test denied capability, stale
preview, missing backup acknowledgement, partial store failure, restart,
concurrent ingest/repair, delayed ingest after successful purge, restart with
the tombstone present, explicit re-enablement, nested sensitive JSON keys
(including `{"Authorization":"Basic dTpw"}`), nested denied paths,
bounded-record failure and explicit skip recovery, writes after backup, and
successful idempotent retry. SQLite is mandatory; PostgreSQL uses
`POSTGRES_TEST_URL` or the documented unprivileged embedded fixture. Configured
external failures fail closed; never silently skip them. Assert secret canaries
are absent from SQL rows, outgoing embedding requests, logs, and diagnostics.
Fault-inject each sink failure and observe scope fencing.

Conventional tests cover parsing, diagnostics, configuration, hooks, transport,
and database interactions. Kani adds no planned value over these specific
unbounded kernels and property tests. No artificial proof obligation is added
for formatting or straightforward control flow. Documentation-only execution
uses a reviewed decision corpus and cross-document consistency checks instead
of fake failing runtime tests.

## Interfaces and dependencies

The policy belongs to domain-owned values and pure decisions; application use
cases orchestrate it. Filesystem/Git discovery, SQL bindings, capabilities,
terminal input, and backups remain adapters. Providers reuse the same policy
rather than implementing independent normalization or redaction. Both existing
ingestion ports enforce it at their trust boundary without calling each other.
No new universal safety port or framework is required by this decision.

EP-M2 must document reuse in `docs/developers-guide.md`: ownership, permitted
callers, validated-input requirements, semantic errors, and composition order
(authority, workspace, deny, redact, persist/project). Runtime names and
signatures belong to the owning implementation slice after its abstraction
sweep; this document specifies contracts without pretending those APIs exist.

Authorized testing libraries are `rstest`, `rstest-bdd`, `googletest`,
`pretty_assertions`, `insta`, and `proptest`, to be added only by runtime
slices with caret versions. Use `mockall` for ports and injected
configuration/clock/ filesystem dependencies; never mutate process environment
without the shared guard required by AGENTS. Domain tests must require no live
infrastructure.

Required reading for implementors: `docs/developers-guide.md`,
`docs/rust-testing-with-rstest-fixtures.md`,
`docs/reliable-testing-in-rust-via-dependency-injection.md`,
`docs/rust-doctest-dry-guide.md`,
`docs/complexity-antipatterns-and-refactoring-strategies.md`,
`docs/pg-embed-setup-unpriv-users-guide.md`, and
`docs/rstest-bdd-users-guide.md`. These govern fixtures, dependency injection,
executable examples, small cohesive helpers, database prerequisites, and BDD
bindings respectively.

## Plan of work and milestones

### EP-M1: Ratify the decision

**Complete (2026-10-15).** ADR 015 uses the required Status, Date, Context and
problem statement, Decision Drivers, Requirements, Options considered, Decision
outcome, Goals and non-goals, Migration plan, Known risks and limitations,
Outstanding decisions, and Architectural rationale sections. It transfers
SAFE-ID, SAFE-TEXT, and SAFE-PURGE with worked examples, expected outcomes, and
the verification limits, and records the accepted date and the approval
authority. Options tables compare conservative identity against automatic
transport merging, `redacted` against `none`/plaintext/`encrypted`, and
scope-bound confirmation against a bare `--yes`, a prompt-only contract, and
capability-only authority.

Design §§5.4, 13, 14, 15, and 17, TOR §§7.2, 8.2, and 9, RFC 0001's Proposed
design and Compatibility and migration headings, and `docs/contents.md` are
synchronized. Design §8.4 needed no edit: the Qdrant layout's purge and
lazy-creation rules are unchanged by this decision. Only settled questions were
closed; `encrypted` mode, the Axinite write-back policy, the operator backup
format, and the deployment tenant strategy remain explicitly deferred.
Normative TOML field names are preserved and `encrypted` is marked unsupported.

Acceptance E1: all ten requested defaults have unambiguous decisions, all
cross-references resolve, and the identity, detector, and purge examples have
explicit expected outcomes. Conformance: SAFE-ID/TEXT/PURGE checked against
every source in Table 1; no interface, trust, or format expansion was
introduced beyond the TOR §9 resolution the criteria anticipated. Rollback is a
normal revert of the documentation commit. Runtime enforcement remains a
documented later gap.

Acceptance E1: all ten requested defaults have unambiguous decisions, all
cross-references resolve, and the identity/detector/purge examples have
explicit expected outcomes. Conformance: check SAFE-ID/TEXT/PURGE against every
source in Table 1 and reject unapproved interface, trust, or format expansion.
Run all gates below, record results, and commit this coherent decision plateau.
Rollback is a normal revert of the documentation commit. Runtime enforcement
remains a documented later gap. No compatibility layer is needed.

### EP-M2: Publish the implementation handoff

Update `docs/users-guide.md` with planned defaults, root-move consequences,
override limitations, exclusions, text modes, scope confirmation, backup and
restore expectations, and partial-purge behaviour. Distinguish planned commands
from currently runnable tooling. Update `docs/developers-guide.md` with policy
reuse and the proof-to-test relationships; retain detailed architecture in the
design/ADR rather than only this plan. Capture the abstract models and
invariants that influence storage bindings, interval provenance, and purge
fencing there.

Acceptance E2: a new provider author can determine every safe ingest outcome
from the ADR and guides without choosing graph/theme policy. An operator can
identify what to back up, what confirmation means, and what purge cannot erase.
Each V-ID/TEXT/PURGE obligation has its later roadmap owner, witnesses,
negative control, proof command, and integration-test requirement. Conformance:
verify SAFE-BOUNDARY and SAFE-VERIFY, matching source revisions and preserving
all first-value scope boundaries. Run all gates and commit. Revert the
milestone commit to recover; no persisted data exists to migrate. Remaining gap
is closure only for this decision slice, runtime work stays in its owning tasks.

### EP-M3: Close only the decision slice

Review the complete change with the six Logisphere perspectives: architecture,
alternatives, performance/observability, correctness, operations, and developer
experience. Reconcile findings in the decision log. Check every requested
default and every upstream trace, then mark roadmap 1.1.2 `[x]` with a concise
`Done` note linking the accepted ADR. Use the `mapsplice` skill for structural
roadmap edits. Do not mark downstream runtime tasks complete.

Acceptance E3: accepted ADR, consistent documentation, recorded approval and
review, successful gates on the final candidate, and the single correct done
marker. Commit the closure change. Set this plan COMPLETE only for its decision
recording outcome and summarize outstanding runtime verification. Any
unaccepted deviation keeps it BLOCKED. Reverting the closure commit reopens the
task.

## Concrete steps and validation

Run from the repository root on `1-1-2-record-local-safety-defaults`. Before
execution inspect `git status --short`, `git branch --show-current`, and
current upstream documents. Preserve unrelated changes. At each milestone, use
the scrutineer gate runner, sequentially, with pipe failure propagation and
logs under `/tmp`; never overlap suites or use `/tmp` as a build target.

```bash
set -o pipefail
make fmt 2>&1 | tee /tmp/fmt-memoryd-1-1-2-record-local-safety-defaults.out
make check-fmt 2>&1 | tee /tmp/check-fmt-memoryd-1-1-2-record-local-safety-defaults.out
make markdownlint 2>&1 | tee /tmp/markdownlint-memoryd-1-1-2-record-local-safety-defaults.out
make nixie 2>&1 | tee /tmp/nixie-memoryd-1-1-2-record-local-safety-defaults.out
make lint 2>&1 | tee /tmp/lint-memoryd-1-1-2-record-local-safety-defaults.out
make test 2>&1 | tee /tmp/test-memoryd-1-1-2-record-local-safety-defaults.out
```

Run each line separately and stop on failure. Expected evidence is exit zero
from each command, no formatting differences, no Markdown/Mermaid errors, no
Clippy/Whitaker/rustdoc warnings, and the existing test suite passing. Existing
stub tests are baseline health evidence, not safety-policy coverage. Review
formatter changes and retain only intended edits. Run `git diff --check` and
inspect the full diff before every commit. Write commit messages to temporary
files and use `git commit -F`; do not commit failing gates.

For the plan PR, use the same validation sequence, inspect the full branch
diff, verify `leynos` with standard `/usr/bin/gh` in the clean environment
required by AGENTS, and verify SSH identity. Push over SSH, fetch the explicit
branch into `refs/remotes/origin/1-1-2-record-local-safety-defaults`, set the
named upstream, and compare local, tracking, and remote SHAs. Create a draft PR
titled `Plan: Record local safety defaults (1.1.2)` with a branch-wise plan
link. End the body with `## References` and this session link:
<https://lody.ai/leynos/sessions/f4b8ecd8-40e8-4637-8b52-d3df57a3e3a8>. Read
back title, body, head, base, and draft status. Publication is authorized.
This paragraph described the state before approval; execution was authorized on
2026-10-15, and the Progress section records the milestones completed since.

## Idempotence and recovery

Reading, linting, and validation can be repeated. Formatting can change files:
inspect the diff and undo only accidental changes owned by this task. Do not
reset other agents' work. If a gate fails, inspect its log, repair only an
in-scope cause, and rerun that gate after the fix. Respect shared compiler
admission and cache locks. Do not bypass wrappers, restart services, or kill
unrelated processes. Full disks or admission refusal require reporting.

No live evidence is purged, moved, or backed up by this plan's execution. Its
recovery mechanism is version-controlled documentation. Later runtime plans
must implement and test the explicit failure and retry semantics above.

## Artefacts and research notes

Primary-source research through Firecrawl on 2026-10-14 supports the proposed
choices, rather than dictating product policy:

- Git documents distinct URL, SCP-like, and local path forms; this supports
  preserving path semantics rather than guessing cross-transport
  equivalence.[^1]
- SQLite supplies online backup facilities; copying an active database file is
  not the selected backup procedure.[^2]
- PostgreSQL documents consistent concurrent exports through `pg_dump`.[^3]
- `globset` documents glob syntax; later adapters must configure and test the
  selected separator/escaping behaviour against this contract.[^4]
- Verus provides specifications and deductive verification; this plan leaves
  solver details until an executable kernel exists.[^5]

[^1]: [Git clone documentation](https://git-scm.com/docs/git-clone), Git URLs.
[^2]: [SQLite online backup API](https://www.sqlite.org/backup.html).
[^3]: [PostgreSQL pg_dump](https://www.postgresql.org/docs/current/app-pgdump.html).
[^4]: [globset documentation](https://docs.rs/globset/latest/globset/).
[^5]: [Verus guide](https://verus-lang.github.io/verus/guide/).

Revision note (2026-10-14): initial approval-gated draft grounded in the
existing decision/runtime roadmap split; all runtime claims remain proposals
and proof obligations remain unproved until actually verified.

Review revision (2026-10-14): Pandalump and Telefono identified hidden
root-hash collisions and identity-bearing SSH usernames; retain sensitive
canonical roots and SSH principals for comparison. Dinolump requested exact
recursive exclusions. Doggylump identified post-purge recapture; retain a
content-free fence until explicit re-enablement. Wafflecat supported the
decision-only scope. Buzzy Bee's operational lens prompted explicit processing
ceilings and bounded diagnostics. The trade-off is additional sensitive
registry metadata and conservative exclusions in exchange for avoiding silent
merges and recapture. These changes refine proposed contracts, not runtime
scope.
