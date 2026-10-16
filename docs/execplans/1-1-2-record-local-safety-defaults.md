# Record local safety defaults for workspace, redaction, and purge

Status: COMPLETE — EP-M1, EP-M2, and EP-M3 delivered. Every gate result in
Progress names the commit it gated and is evidence about that commit alone.
Recording a result moves the head it would describe, so the current head's run
is deliberately not recorded here; that evidence is supplied in the PR 46
assessment thread instead. Publication handoff to `pr-babysitting` owns hosted
CI and the review conversations, so no local result here is a claim about CI.

This header deliberately names no commit and no "final candidate". Naming
either would invalidate itself on the next commit that touches this plan, which
is how the earlier wording went stale.

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
Roadmap 1.1.2 is marked done only in EP-M3, after the accepted ADR, the
synchronized guides, and the gates exist; it was not marked done in the
planning PR. Never claim a policy has runtime enforcement merely because its
decision record is accepted.

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
- [x] (2026-10-15) EP-M1 committed as `a2dc412`. All six gates passed on that
  candidate: `make fmt`, `make check-fmt`, `make markdownlint`, and
  `make nixie` exited zero with the tree byte-identical before and after
  formatting; `make lint` and `make test` exited zero, the latter with 146
  tests passed and zero skipped. Long gate durations came from shared
  package-cache and compile admission contention, and an unreachable managed
  sccache means uncached compilation, not a failure.
- [x] (2026-10-15) EP-M1 milestone review completed against `8639da6..a2dc412`
  with full coverage of all six changed paths. Two minor findings were raised
  and both confirmed against source; both are repaired in `893eb8c`.
- [x] (2026-10-15) EP-M2: `docs/users-guide.md` gained a "Local safety defaults
  (planned)" section covering the workspace descriptor and its consequences,
  the exclusions and detector corpus, the three raw-text modes, purge preview
  and confirmation, the backup acknowledgement, and what purge cannot erase.
  `docs/developers-guide.md` gained "Local safety policy reuse" with ownership,
  permitted callers, validated input, semantic errors, storage-binding
  constraints, and the V-ID, V-TEXT, and V-PURGE obligations with owners,
  witnesses, negative controls, proof commands, and integration tests. The
  contents index reflects both. Both guides label the policy as unenforced and
  keep runnable tooling distinct from planned behaviour.
- [x] (2026-10-15) EP-M2 committed as `75a93e3` with the same six-gate sequence
  passing on that candidate.
- [x] (2026-10-16) EP-M3 review ran the six Logisphere perspectives over the
  whole change. Findings were verified against source before repair rather than
  accepted on assertion. Confirmed defects: the developers' guide attributed
  V-ID/V-TEXT/V-PURGE to the ADR and design, neither of which defines those
  identifiers; the cursor rule differed across ADR 015, the users' guide, and
  RFC 0001; the roadmap's "See" line cited non-existent RFC 0001 section
  numbers; terms-of-reference.md §9 still listed the redaction question ADR 015
  resolves; and the tombstone re-enablement obligation had no roadmap owner.
  All five are repaired. Two further gaps were recorded rather than resolved:
  the workspace-registry storage surface and the `line_hash`
  raw-versus-redacted question, both now in ADR 015's Outstanding decisions and
  memoryd-design.md §17 with named owners.
- [x] (2026-10-16) EP-M3 marked roadmap 1.1.2 done in `docs/roadmap.md`. The
  `mapsplice` skill was loaded as the plan requires and its `replace` and
  `delete` commands were tested first; both fail with `dangling_dependency` for
  this anchor because task 1.1.3's `Requires 1.1.1 and 1.1.2` names it, and a
  step-level replace injects a stray blank line before nested sub-tasks. Since
  completing a task changes no anchor and needs no renumbering, the edit was
  made directly, following the `Decided:` note convention that task 1.1.1
  established. The two `mapsplice` defects are recorded here for the tool's
  maintainers; they are not this task's scope to fix.
- [x] (2026-10-16) EP-M3 gates passed on `a082bbf`. `make fmt`,
  `make check-fmt`, `make markdownlint` (42 files, 0 issues), `make nixie`,
  `make typecheck`, `make test-workflow-contracts` (6 passed), and `make lint`
  all exited zero; `make test` passed 146 tests with zero skipped. The first
  gate run reported that `make fmt` had rewritten six files; that did not
  reproduce, and a hunk-level check found no line-break-only reflow, so no
  formatter damage was present. Edit ordering was wrong, though: a later edit
  added a line that satisfied the 80-column rule and `mdtablefix` re-wrapped it
  anyway. The final gate run caught the reflow, and the corrected text is
  committed from a verified fixed point rather than an assumed one. The
  mechanism first recorded for that reflow was wrong; Outcomes states the
  corrected one and the measurement that refuted it.
- [x] (2026-10-16) EP-M3 committed as `a082bbf`, covering all eight tracked
  documents: ADR 015, the users' guide, RFC 0001, the developers' guide, the
  terms of reference, the design document, the roadmap, and this plan. The
  commit message records the five repairs, the two recorded gaps, and the
  `mapsplice` deviation. Publication handoff to `pr-babysitting` is the
  remaining step and is tracked there rather than in this plan.
- [x] (2026-10-16) All eight commit gates passed on commit `ba5b124`, run
  strictly sequentially by the scrutineer with the head verified before and
  after: `make fmt` (no change), `make check-fmt` (42 files unchanged),
  `make markdownlint` (42 files, 0 issues), `make nixie`, `make test` (146
  passed, 0 skipped), `make typecheck`, `make test-workflow-contracts` (6
  passed), and `make lint` (rustdoc, Clippy, and Whitaker). This run belongs to
  `ba5b124` alone. The gate runs recorded above for `a2dc412` and `75a93e3`
  remain true of those commits and are not evidence for this one; the Rust
  gates spent most of their wall time queued on the shared compile-admission
  pool with sccache unreachable, which is an environment fact and not a result.
- [x] (2026-10-16) All eight commit gates passed again on commit `b0647dd`,
  the head carrying the R1-21 repair and the rescoped status header, run
  strictly sequentially with the head verified before and after: `make fmt` (no
  tracked file changed), `make check-fmt` (42 files unchanged),
  `make markdownlint` (42 files, 0 issues), `make nixie`, `make lint`,
  `make test` (146 passed, 0 skipped), `make typecheck`, and
  `make test-workflow-contracts` (6 passed). The Rust gates paid uncached
  compile cost rather than slot-wait cost, and no admission refusal occurred.
- [x] (2026-10-16) Reconciled the assessment findings against `ba5b124` and
  repaired two. R1-21 stated a `note` line containing a quote, which closes the
  password value under the same document's rule that a quoted value ends at its
  closing quote however many lines later, so the row contradicted the grammar
  it exists to test; its next line now carries no quote. The status header no
  longer claims green gates on an unnamed "final candidate", and it names no
  commit in its place: a header that pins a SHA is invalidated by the next
  commit touching this plan, which is how the old wording went stale.
- [x] (2026-10-16) Hosted assessment `EPM-1-1-2-B473C17-20261016` returned five
  findings, all confirmed against source before any edit. The developers' guide
  cursor rule contradicted ADR 015; the detector corpus did not meet this
  plan's own E1 requirement for a versioned corpus and settled grammar; the
  operator backup procedure omitted the graph store; the users' guide
  overstated machine separation; and the formatter correction recorded
  characters where the tool measures display columns. All five are repaired in
  this revision, and the repairs follow the reply's explicit constraints: no
  machine identifier is added, and no executable fixture is added in this
  documentation-only slice.
- [x] (2026-10-16) Hosted assessment `EPM-1-1-2-0BCD857-20261016` closed both
  outstanding findings — R1-21 and the completion claim — and independently
  confirmed the corrected entropy arithmetic, with E3 recorded as closed. It
  raised one new defect: R1-02's outcome cell, which asserted that the trailing
  space was required to prevent redaction. That was verified by construction
  before repair and is genuine, so the assertion is withdrawn in `32418ea`; the
  prefix rule needs a non-whitespace token to be present, and end of input
  after the hyphen supplies none, exactly as trailing whitespace does. The
  input cell is unchanged, so an explanation was corrected rather than a
  fixture. The reply was also asked to name a surviving defect of the class
  that keeps recurring, and this is what it named; a sweep of every normative
  word in all 37 outcome cells was run afterwards, and the six remaining hits
  across four rows — R1-18, R1-21, R1-31, and R1-34 — were each verified
  against the rule rather than dismissed. The first draft of this sentence said
  "five remaining hits" while naming only four rows, and omitted R1-18 from the
  list entirely; the count and the enumeration were both wrong, and both are
  corrected here.

## Surprises & discoveries

`mdtablefix --wrap` budgets Unicode display columns, and the first two
explanations recorded for its reflow were both wrong in different ways. The
byte theory was refuted by measurement, as Outcomes records. The character
theory that replaced it was also wrong, and was itself unmeasured: it rested on
examples where characters and columns coincide. Separating the models needs
lines chosen so the counts disagree, and the discriminating pair is a line of
Cyrillic prose, which is heavy in bytes and light in columns, against a line of
wide characters, which is the reverse. The Cyrillic line survived at 46
characters over 86 bytes; the wide line was wrapped at 46 characters, 126
bytes, and 86 display columns; and wide lines were left alone at 79 columns and
wrapped at 81. The general lesson is narrower than "measure the tool": measure
it with inputs that separate the competing hypotheses, because a probe whose
models all agree cannot fail and therefore cannot inform.

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

The roadmap's "See" line for 1.1.2 cited "RFC 0001 §§3 and 7", but RFC 0001 has
named headings and no numbered sections, so that citation did not resolve.
EP-M3 repaired it to name the headings. Two adjacent defects surfaced while
checking the citation: the same line pointed at design §8.4, which is the
Qdrant layout, not workspace derivation (now §5.4), and `grep -c 'line_hash'`
shows the Codex and Claude idempotency keys embed a hash whose input is never
stated.

`mapsplice` 0.1.0 cannot complete a task in this roadmap. It parses the file
and resolves anchors, but fails closed with `dangling_dependency` when the
anchor is named by any other task's `Requires` clause, which task 1.1.3 does. A
step-level replacement also inserts a blank line before the first nested
sub-task and `mdtablefix` and `markdownlint --fix` both accept it, so the churn
would have reached the commit. A checkbox flip needs no renumbering, so the
direct edit costs nothing here; a genuine structural edit would need these
fixed first.

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
- (2026-10-15, EP-M2) Put the operator contract in `docs/users-guide.md` and the
  reuse and proof-to-test contract in `docs/developers-guide.md`, rather than
  expanding the design document further. The design document already carries
  the architecture, and the guides are what roadmap 2.2.3, 2.2.4, and 5.3.1
  implementors read first.
- (2026-10-15, review) Both milestone-review findings were confirmed against
  source and repaired rather than dismissed: the ADR status line stated
  present-tense behaviour it then disclaimed, and the ExecPlan's publication
  paragraph still denied an approval the plan records. Text corrections only;
  no contract changed.
- (2026-10-16, EP-M3) Reconciled the cursor rule across ADR 015, the users'
  guide, RFC 0001, and this plan into one canonical statement with two cases:
  an intentional exclusion advances the cursor, a processing failure does not.
  The earlier wording left denial and non-advancement entangled, so a reader
  could conclude that a denied path blocks its source forever. The documents
  keep their own voice — the guide says "your own audited skip" where the ADR
  says "an operator's audited skip" — but the two cases and their outcomes are
  identical everywhere, and no document may restate only one of them.
- (2026-10-16, EP-M3) Fixed the deny-pattern root rather than deferring it: the
  root is the workspace's canonical root, so linked worktrees evaluate their
  own patterns. The ADR already defined that root for the descriptor, so naming
  it for deny patterns removed an ambiguity without a new decision.
- (2026-10-16, EP-M3) Recorded the workspace-registry storage surface and the
  `line_hash` input as open in ADR 015 and memoryd-design.md §17 with owners
  2.2.4 and 2.2.3, rather than inventing a schema here. Both are storage
  decisions this documentation slice may not make, and both would otherwise
  have been discovered as blocking questions by the slices that need them.
- (2026-10-16, EP-M3) Deferred the tombstone re-enablement obligation to roadmap
  5.3.1 without editing that task's title or success line. Adding a new roadmap
  task for it would have widened the approved scope; the ADR already names
  5.3.1 as the owner of purge execution, and re-enablement is a purge-lifecycle
  action.
- (2026-10-16, assessment) Distinguished three outcomes in the developers'
  guide rather than two: accepted and possibly transformed, intentional
  exclusion, and processing failure. This follows from ADR 015's cursor rule,
  which advances the cursor for an intentional exclusion. It also removes
  "unchanged" as a requirement for a successful redaction, which transforms the
  record by definition, and it separates an intentional exclusion from the
  error list while still requiring it to be reported with a sanitized reason.
- (2026-10-16, assessment) Named the initial detector corpus
  `memoryd.redaction.v1` and added its case table, because E1 requires a
  versioned corpus and resolved grammar and the ADR supplied neither. The three
  grammar cases are decided in prose, and the entropy boundaries are given as
  measured tokens rather than as descriptions, so a detector is testable
  against them. This is a decision record, not an implementation: no executable
  fixture is added, and roadmap 2.2.3 still owns the corpus code.
- (2026-10-16, assessment) Corrected machine separation by describing what the
  derivation actually does, and added no machine identifier, because adding one
  would change the accepted derivation contract. The guide now states the
  consequence directly, and the ADR states both the cross-machine case and the
  failure mode, that equal descriptors from different machines register as one
  workspace.
- (2026-10-16, reassessment) Added the JSON rendering contract to ADR 015,
  because the structural replacement rule and the malformed-JSON rejection rule
  were jointly unsatisfiable: the bare marker does not parse, so a collector
  following both rules produced a record the daemon had to reject. The marker's
  logical value and its serialized JSON form are now distinguished, and the
  replacement is a JSON string even where it stands in for an object, so a
  redacted document stays valid without preserving a secret's shape.
- (2026-10-16, reassessment) Completed the corpus rows that still described
  their inputs. R1-34's three assignments carry their values, R1-21's and
  R1-22's following lines are stated, the ceiling rows are given constructions
  that exceed exactly one bound each, and R1-37 adds the value-scanning branch
  of the escaped-string rule, which the corpus did not previously exercise.
  Constructing R1-30 and R1-31 as first written showed both would have failed
  for the wrong reason: sub-floor tokens are never detected, and an unvalued
  innermost level is malformed rather than deep.
- (2026-10-16, reassessment) Removed the retained word-count guard from this
  plan, which contradicted the replacement evidence stated elsewhere. The
  instruments are now separated by the question each answers, and the
  perturbation test is limited to what it shows: padding is the formatter's
  only effect on the committed document, not that an earlier run preserved its
  own input.
- (2026-10-16, assessment) Replaced the character-count claim about
  `mdtablefix` with the display-column measurement, and kept the rejection of
  the byte theory and the paragraph re-flow mechanism. One table row changed
  engine in the corpus: an SSH public key that the class description implies is
  out of scope is in fact redacted by the entropy rule, so it is recorded as an
  accepted over-detection rather than as a negative case.
- (2026-10-16, corpus completion) Tightened the JWT rule to require that the
  first segment decodes to a JSON object. Counting dots alone made the rule and
  its own negative jointly unsatisfiable: `SSH-2.0-OpenSSH_9.6` is three
  base64url-shaped segments, so R1-10 as written was redacted by the rule it
  existed to bound. The first-segment test is not a stylistic preference — a
  JWT header is base64url of a JSON object, and that is the property that
  distinguishes a token from a dotted version string.
- (2026-10-16, corpus completion) Gave R1-34 a value with no recognized prefix.
  The row is the corpus's evidence that the by-name rule fires, but its first
  value began with `AKIA`, so the prefix rule redacted it too and the cell's
  attribution of the redaction to the name rule was not exclusive. The
  replacement value keeps the name rule as the only rule that can fire on it,
  and the cell now says which rules cannot.
- (2026-10-16, corpus completion) Moved R1-37's escape inside the matched span.
  As written, the escape sat after the JWT ended, so the row demonstrated a
  value-scanning match in a string that merely contained an escape somewhere
  else, and the stated reason — that a narrow span would split the escape — did
  not hold for it. With the escape inside the span the naive span genuinely
  ends part-way through the escape sequence, and the rule's widening is the
  only correct outcome. The rule text now also states that the widening is
  triggered by the string carrying an escape at all, not by the escape
  position, because deciding the latter requires the mapping the rule declines
  to reconstruct.
- (2026-10-16, corpus completion) Made R1-21 genuinely unterminated. The row
  gave its following line a quote of its own, and the same document's rule says
  a quoted value ends at its closing quote however many lines later, so that
  quote closed the password value: the input produced
  `password = [REDACTED]handbook index"` rather than the whole-record redaction
  the cell claimed. This is the same class as the JWT contradiction below — a
  rule and its own test case cannot both hold — and it was introduced by the
  repair that gave R1-21 a concrete `note` line in the first place, so the fix
  for one finding created the next. The `note` line now carries no quote, which
  is load-bearing rather than cosmetic.
- (2026-10-16, corpus completion) Repaired R1-02 and stated the cell
  convention for `\n`. R1-02 marked its input with double backticks, which
  render as a code span around a backtick-quoted token rather than the bare
  token the cell's reason assumed, so a non-whitespace character did follow the
  prefix and the row contradicted itself in rendered form. This is the row an
  earlier automated check flagged and I dismissed as a checker artifact;
  rendering it settled the question against that dismissal. The `\n` convention
  is now stated because four rows write it and depend on reading it as a line
  terminator — R1-08, R1-09, R1-36, and R1-20 — and read literally those rows
  would each state a different input. R1-21 and R1-22 also span two lines, but
  they name the second line in prose rather than writing `\n`, so they need no
  convention. This entry first said "five rows", which no row set satisfies
  under either reading.
- (2026-10-16, entropy explanation) Corrected the account of how R1-24 and
  R1-25 straddle the 4.5 bits/byte threshold. The text said they differ "in the
  number of equally frequent symbols, 23 against 24", and that is false in two
  independent ways: the symbols are not equally frequent, and a
  uniform-alphabet reading computes R1-24 at `log2(23)` = 4.5236, which is
  *above* the threshold the row exists to sit below. The explanation therefore
  contradicted the outcome it explained, which is the same class as R1-21 and
  the JWT rule. The three boundary rows share one length and one alphabet and
  differ in how often each symbol repeats; the note now gives
  `sum(v * log2(v))` = 18, 16, 12 and
  `H = (n * log2(n) - sum(v * log2(v))) / n` yielding 4.4375, 4.5000, 4.6250.
  Found by recomputing every committed figure rather than re-reading any of
  them. Most of the earlier corpus defects were found the same way; R1-02's
  trailing-space assertion, recorded below, came from hosted review instead,
  but it is the same class — a rule and a stated case that cannot both hold.
  This sentence first quantified that split and got it wrong, for the reason
  the retrospective now records: the figure it divided had gone stale.
- (2026-10-16, R1-02 explanation) Withdrew R1-02's assertion that the trailing
  space is required to prevent redaction. The cell claimed a variant ending at
  the hyphen "would state an input the rule redacts", but the prefix rule
  redacts the non-whitespace token *after* a prefix, so it needs a token to be
  present; with end of input immediately after the hyphen there is none, and
  the row is not redacted — the same outcome as with trailing whitespace. The
  sentence was introduced by the repair recorded two entries above, which fixed
  the input cell and then stated a reason that did not follow: R1-21 was the
  same pattern earlier in this task, where the repair gave the row a `note`
  line and the `note` line then closed the value. Those two are named rather
  than counted, and this entry said "the count is two of six, checked by
  enumerating the defects" until the figure audit below: the denominator was
  the retrospective's stale total, not an enumeration, so the sentence claimed
  a check it had not performed. An earlier draft of it also claimed three
  repair-created defects while naming only two, which is the same defect class
  again. Found by hosted review, which the reply was explicitly asked to
  provide for this class on the argument that execution had caught every
  previous instance. The input is unchanged, so this corrects an explanation
  rather than a fixture.
- (2026-10-16, completion claim) Removed three claims the plan could not keep
  true. The header promised that Progress names the latest green gate run's
  candidate, which is impossible: recording a run moves the head it describes.
  A Progress entry still described the previous header, which named a commit,
  and the EP-M3 entry named no candidate at all. The header now states the
  invariant and says the current head's evidence is supplied in the PR 46
  assessment thread; the entries name the commits they actually describe. Two
  of the three were introduced by the repair for CodeRabbit's completion-claim
  finding, so the fix for one defect created the next — the same pattern as
  R1-21.
- (2026-10-16, figure audit) Audited every countable figure the plan and ADR
  assert about the corpus, by recomputing each from the committed documents
  rather than re-reading it. Five claims failed. The normative-word sweep was
  recorded as "five remaining hits" and is six hits across four rows; the
  enumeration beside it named R1-32 and R1-33, which carry no normative word at
  all, and omitted R1-18, which does — so the tally and its example list were
  each wrong, in the defect class this plan keeps recording. The `\n`
  convention was said to govern five rows and governs four; R1-21 and R1-22 do
  span two lines but state both in prose, so they never needed the convention.
  Both ADR statements of that class list named the unterminated-quote and
  unquoted-trailing rows as dependants of the convention, which they are not,
  and those are two separate failed claims rather than one, since the preamble
  and the revision note are separate sites that each needed its own correction.
  And the retrospective's "All six were found the same way and only that way"
  was contradicted by the paragraph two lines above it, which records that the
  sixth came from hosted review. The last of these was introduced by the commit
  that repaired the others. This entry deliberately does not restate how many
  repair-created defects the task now has: the retrospective counts corpus
  defects, this audit found a plan defect, and the two sets are not the same
  set. Three successive hand-tallied counts in this task were wrong, each for
  this same reason — a figure asserted without a fixed population. Found by an
  audit of the plan's own recorded figures after that third failure, with each
  figure recomputed from the committed text rather than recalled.
- (2026-10-16, figure audit, second round) Extended the same audit past the
  corpus to the counts this plan states about defects, and found the largest
  instance of the class yet: the retrospective's "Five of the six were found
  the same way and only that way" and its "A sixth defect" ordinal. Both rest
  on a total that entered as "Five defects were confirmed and repaired" in
  `a082bbf`, was bumped to six in `a7a7bc9`, and had already been false when it
  was written: the section it totals grew by narrating each successive round,
  and the total was never recomputed. It also double-counted, since the
  paragraph naming five hosted-assessment findings is not the milestone-review
  five the same sentence invokes. The ordinal is false by more than one, not by
  rounding: every corpus defect narrated above the paragraph calling R1-02's
  finding the sixth sits before it, so that paragraph is not the sixth. The
  repair names the milestone review as the population for the EP-M3 sentence —
  the Progress entry's own list — and removes the total and the ordinal rather
  than restating them, because this section also mixes corpus defects with
  plan-level ones, so no total over it has a fixed set to count. The same edit
  corrected the Decision log's "the count is two of six, checked by enumerating
  the defects": the denominator was that stale total and the claimed
  enumeration had not happened. Also repaired the formatter fixed point, which
  `make check-fmt` failed: six prose lines in this plan were filled to 76–78
  columns where `mdtablefix` fills to 79–80, so the committed text was not the
  formatter's own output. That one is not a figure defect, and it is recorded
  because it is the same lesson as the fifth claim above — a claim about what a
  tool produced, asserted without running the tool on the committed bytes.

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

No runtime behaviour has changed, no proof is claimed, and roadmap 1.1.2 was
closed by EP-M3. Runtime verification stays with roadmap items 2.2.3, 2.2.4,
and 5.3.1, and the ADR states plainly that nothing is enforced yet.

EP-M2 completed on 2026-10-15. The milestone review earned its keep: both
findings were real, and the second one — a stale "execution is not yet
authorized" sentence left in the publication instructions — would have told a
later operator to halt already-approved work. The lesson is that a living plan
accumulates stale imperatives at its tail as its head advances, so a status
edit belongs wherever the plan states a permission, not only in Progress.

EP-M3 completed on 2026-10-16. Its milestone review confirmed and repaired the
five defects its Progress entry names, and two genuine gaps were recorded with
owners rather than invented away. The milestone review is the population here,
not the whole task: the detector corpus's later rounds are narrated below and
are counted nowhere, because their total depends on how the class is read. The
review's most useful habit was refusing to accept a finding on its assertion:
the false provenance claim about V-ID/V-TEXT/V-PURGE looked plausible until
`grep -c 'V-ID'` returned zero for both cited documents. Two lessons. First, a
document that names three models in a guide but defines them nowhere is a
provenance bug, not a wording bug; the identifiers were introduced in the guide
and their obligations kept in this plan's abstract-model section, with the
attribution corrected rather than the identifiers relocated. ADR 015 does not
define them and its Outstanding decisions section does not mention them, so
sending a reader there would have replaced one false pointer with another.
Second, the cursor rule was stated three times in three slightly different
ways, and only a side-by-side comparison exposed that one variant implied a
denied path blocks its source permanently. Restating a rule in several
documents requires one canonical statement to copy from. It does not require
identical sentences: a guide addresses the reader as "you" while an ADR speaks
of "the operator", and forcing one voice on both would make one of them read
badly. What must not vary is the rule — the cases, their outcomes, and which
one retries.

A tool limitation was also recorded rather than worked around silently:
`mapsplice` 0.1.0 fails on this roadmap's task 1.1.2 because it treats a
`Requires` reference to the edited anchor as a dangling dependency. The plan
mandated `mapsplice` for structural roadmap edits, so the deviation — a direct
edit for a checkbox flip that needed no renumbering — is recorded in Progress
and in the Surprises section with the reproduction.

The gate runs produced a third lesson, about sequencing rather than
documentation. The first run reported that `make fmt` had rewritten six files;
that did not reproduce, and it was correctly dismissed after a hunk-level check
found no line-break-only reflow. But the dismissal was then over-generalized
into a claim that the candidate was already a formatter fixed point, and that
claim was asserted in this plan before the text supporting it had settled. A
subsequent edit added a line that satisfied the 80-column rule and `mdtablefix`
re-wrapped it anyway, and the final gate run caught the mutation. Two habits
follow. A formatter claim must be re-established on the bytes actually being
committed, not carried forward from an earlier run, because a fixed-point check
only ever describes the snapshot it was taken on. And a length check is not the
formatter's rule: `mdtablefix --wrap` re-flows a whole paragraph or list item
to its own break points, so a line that individually satisfies the limit can
still be re-wrapped once its neighbours change.

The mechanism recorded here at first was wrong, and the error is worth keeping
because it was written as a finding when no measurement stood behind it. This
plan and the commit message for `8d90d02` attributed the reflow to an em-dash
pushing an 80-character line to 82 bytes, and concluded that `mdtablefix`
budgets bytes rather than characters. That explanation was inferred from the
line's width, not from the tool's behaviour; nothing here had measured the
tool. Direct measurement refutes it. `mdtablefix 0.6.1` targets 80 Unicode
display columns, not bytes and not a plain character count: a list item of 80
characters and 82 bytes was left unchanged while one of 81 characters was
wrapped, and a paragraph whose lines were 56, 52, and 13 characters was
re-packed to 72 and 50. The byte-budget theory also predicts the opposite of
what the tree shows, since 41 lines in `docs/` sit within 80 characters but
over 80 bytes, including two in this plan, and `make fmt` leaves every one of
them alone.

The three width models are separable, and measurement separates them. A line of
Cyrillic prose was left unchanged at 46 characters over 86 bytes, which rules
out a byte budget. A line of wide characters was wrapped at 46 characters over
126 bytes and 86 display columns, which rules out a character budget; it is
also why the em-dash row above is no evidence either way, since an em dash is
single-column and cannot distinguish the models from one another. Wide
characters were left alone at 79 display columns and wrapped at 81, which fixes
the threshold to display columns rather than to any count of characters.
Neither threshold caused the reflow; the paragraph-level re-flow did.

The first hosted completeness assessment returned five findings, and all five
held when checked against source, so the repair commit `9964001` addressed
them. The follow-up assessment resolved three and re-opened two, which is the
useful outcome: a partly-resolved verdict is evidence that the first repair was
aimed at the words rather than at the defect. The detector corpus finding is
the clearest case. Its first repair added a corpus table, but several rows
described their input instead of stating it, so the rows that needed a literal
most were the rows a reader could not test. The repair here moves every literal
into its cell, adds the class negatives that were missing, and states the
corpus's three row groups rather than claiming a uniform positive/negative pair
for rows that sit on different axes. That was found by writing the pairing
claim as a check and running it: cloud credentials had no pair at all, and
basic, cookie, and key-block negatives were absent. A claim about a table is
cheap to make true by editing the table and cheap to make false by editing only
the sentence.

One defect was found here rather than by any review. The assignment rule said
the entire value is redacted for names containing six substrings, "including
`AWS_ACCESS_KEY_ID`". Running the rule shows `AWS_ACCESS_KEY_ID` contains none
of those six substrings, so the normative sentence named a value its own rule
left in the clear. It was verified by extracting the substrings from the
sentence and testing them against the name, not by reading. The rule now has an
exact-name list alongside the substring list, and the text says why the two are
not redundant, so a future editor does not delete one as duplication.

The follow-up assessment also rejected the method used to argue content
preservation across the formatter reflows: positive word-count deltas do not
show that no text was lost. Word-count deltas are no longer used as evidence at
all, not even as a lopsided-loss guard, because a delta cannot distinguish a
preserved document from one whose losses and gains happen to cancel.

Two instruments replace them, and each answers a different question. To show
that a formatter run preserved its own input, compare the pre-run and post-run
text after normalizing table padding and line-break placement; that comparison
observes the actual rewrite, and it is the evidence for the earlier runs whose
inputs still exist. To show that the committed document is a fixed point, run
the formatter again and require an empty diff.

The perturbation test answers a third question and must not be read as
answering the first. Reducing every table row's padding to a minimum and then
reformatting recovers the committed file byte-for-byte. That demonstrates the
padding is the formatter's only effect *on this committed document*: it cannot
show that some earlier run, from a different starting state, preserved its own
input. Only the before/after text comparison can speak to that.

A third assessment round left three findings open, and all three were real. The
first was a genuine logical defect rather than a missing example: the
structural JSON rule replaces a field's whole raw value with the bare marker,
the daemon rejects malformed JSON without a text fallback, and those two rules
are unsatisfiable together, because `{"password": [REDACTED]}` does not parse.
R1-19's own expected outcome was an unparseable document that the ADR required
the daemon to reject. The fix is a JSON rendering contract that separates the
marker's logical value from its serialized form, fixed at the layer where the
rules meet rather than left to an adapter. Two lessons. A rule set can be
individually consistent and jointly impossible, and only replaying one rule's
output through the next rule's precondition exposes it. And an "expected
outcome" column is a place where such contradictions surface, so a corpus row
is worth writing even before its fixture exists.

The second finding was the same defect as the round before, one level down: the
rows that had been given literals were the ones short enough to quote, while
the rows that named several values, or referred to a following line, or
described a megabyte-scale boundary still described their input. The repair
gives every remaining row a construction that a fixture can be generated from
without guessing. Verifying those constructions caught two more defects of my
own that no reviewer named. R1-30 specified 4,097 one-character tokens, which
fall below the 32-byte entropy floor and are never detected at all, so the row
would have failed to trip the interval ceiling it exists to test. R1-31 nested
65 `{"k":` units without a value at the innermost level, so the document fails
to parse and the malformed-JSON rule rejects it before depth is evaluated. Both
were found by constructing the stated input and running the stated rule against
it, which is the same method that found the `AWS_ACCESS_KEY_ID` defect, and
both are now constructions that exceed exactly one bound.

The third finding was an inconsistency I introduced in this very plan while
repairing the previous round's methodology criticism. The published text said
the word-count guard had been dropped, while the plan still described retaining
it as a lopsided-loss guard. Both could not be true. The repair removes the
guard from the plan and separates the instruments by the question each answers.
The perturbation test is recorded as evidence about the committed document
only: it recovers this file byte-for-byte from a padding-collapsed state, which
does not show that an earlier run over a different starting state preserved
anything, and claiming more from it than that was the third finding's real
content.

A fourth pass, taken by executing every corpus row against the rules the same
revision states, found three more defects, none of them named by any review.
The decisive one is a rule and its own negative disagreeing. The JWT rule was
written as "three dot-separated base64url segments", which is a shape test:
`SSH-2.0-OpenSSH_9.6` is exactly three base64url-shaped segments, so R1-10 —
the negative case asserting that an SSH version banner carries no detection —
was redacted by the rule it existed to bound. The rule now requires the first
segment to decode to a JSON object, which is what a real JWT header is and what
the banner is not, so the test is structural rather than a dot count. The
lesson generalizes the third round's: a corpus row and the rule it exercises
can contradict each other, and a shape-only detector is the easiest place for
that to hide, because both sides look right in isolation.

The second defect was a non-exclusive attribution, the same class as the
`AWS_ACCESS_KEY_ID` fix two rounds earlier. R1-34's evidence is that the name
rule fires, but its first value began with `AKIA`, so the prefix rule redacted
it too and the cell credited a redaction it did not exclusively cause. The
value is now one no prefix rule recognizes, and the cell names the two rules
that cannot be responsible.

The third was a witness that did not witness. R1-37 exists to exercise the
widened branch, and its reason is that a narrow span would split an escape
sequence — but its escape sat after the JWT ended, outside the span, so the
stated reason did not apply to the case given. The escape is now inside the
span, where the naive span genuinely ends part-way through an escape sequence,
and the rule text states that the widening is triggered by the string carrying
an escape at all rather than by the escape's position, since deciding the
latter needs the mapping the rule declines to reconstruct.

Two more came out of the same pass once the rows were read as a reader sees
them rather than as source text. R1-02 marked its input with double backticks,
which render a code span around a backtick-quoted token, so the input was four
characters and a backtick did follow the prefix — contradicting the cell's own
reason that nothing non-whitespace does. An earlier automated check had flagged
this row and I dismissed the finding as a checker artifact; rendering the cell
settled it against me, which is the second time in this task that dismissing a
finding unexamined has cost a round. And four rows write `\n` for a line
terminator without the table ever saying so, so a fixture built from the
literal cell text would use two characters and state a different input; the
convention is now stated once, in the preamble that claims every row is
reproducible. That sentence said "five rows" until this round, and no row set
satisfies five under any reading of "depends on the convention": four rows write
`\n`, and two more name a following line in prose without it.

One more defect, R1-02's withdrawn trailing-space assertion, came from hosted
review rather than from this pass, and is recorded in the Decision log above.

Every corpus defect narrated in this section but the last was found the same
way and only that way: by building the input the table specifies and running
the rule the same document states, rather than by reading either one. Reading a
row and reading its rule both invite agreement; executing one against the other
does not. R1-02's trailing-space assertion was found by hosted review instead,
as the paragraph above records. This paragraph used to give a total for those
defects and to divide it into found-by-execution and found-by-review, and the
total was wrong: it was written when this section had fewer paragraphs and was
never recomputed as each later round added one. The section also mixes corpus
defects with plan-level ones — the undeclared `\n` convention is a property of
the table's preamble, not of any row — so the set a total would count is not
fixed by the text either. No total is offered here now, and the observation
that the tally went stale is the point rather than a confession. Two lessons
generalize past this corpus. A rule and its own counterexample can contradict
each other, and only execution finds it. And markup is part of the
specification: a cell's rendered text is the input, so a claim about
reproducibility must survive rendering, not merely reading. A third lesson is
about repair rather than detection: in R1-21 and in R1-02's trailing-space
assertion — named rather than counted, for the reason just given — the repair
introduced the next defect, both times by adding an explanation beside a
corrected fixture. A corrected fixture is safe; prose offered alongside it is
not, and should be held to the same execution test as the row.

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

Read `docs/documentation-style-guide.md` before creating the ADR. This plan
originally reserved `docs/adr-014-local-safety-defaults.md`, but 014 was
already allocated by the CodeScene coverage-publication ADR, so the fallback
applied and the file is `docs/adr-015-local-safety-defaults.md`. No existing
safety abstraction is implemented at the baseline. Future runtime work must
repeat the repository sweep before adding helpers or ports.

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

*Table 1: Requirements traced to decision and acceptance evidence.*

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
`Authorization: Basic` values, three dot-separated base64url JWT segments whose
first segment decodes to a JSON object, and complete PEM/OpenSSH private key
blocks; an unterminated opening block redacts through end of record. The
first-segment test is what keeps a three-part version banner out of the JWT
rule. For assignments, redact the entire value for case-insensitive names
containing `password`, `passwd`, `secret`, `token`, `api_key`, or
`private_key`; include `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, and
`AWS_SESSION_TOKEN`. Treat quoted assignment values as values, including
embedded whitespace. Redact entire Cookie/Set-Cookie values and entire
recognized database URLs carrying userinfo (`postgres`, `postgresql`, `mysql`,
`mongodb`, `redis`, `rediss`). For JSON, inspect keys structurally using the
same case-insensitive sensitive name rules plus exact `Authorization`,
`Cookie`, and `Set-Cookie` keys, replacing the entire associated value,
including objects and arrays, with the marker; do not rely on scanning the
value string alone. Unknown fields are recursively scanned, including arrays
and escaped string content. The ADR must turn these rules into a versioned
positive/negative corpus and explicit grammar choices, including malformed
JSON, escaped strings, and multiline values; unresolved choices prevent E1
acceptance. Never imply all providers or all secret formats are detected.

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
reason and stable source identity. The cursor rule has exactly two cases: an
intentional exclusion (a mandatory default exclusion, a deny-pattern match, or
an operator's audited skip of one source position) is deliberately skipped and
does advance the cursor, while a processing failure (failed redaction, detector
or policy error, breached processing ceiling, or store error) is not
acknowledged as captured and the cursor does not advance, so the record is
retried until it succeeds or an operator skips it. ADR 015 is the canonical
statement; the other documents keep their own voice but not their own version
of the rule.

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
by interval union, complement partition, and ordered rendering. The rendering
obligation extends past the interval transformation to its output: where the
surrounding input is a field the schema declares as JSON, the marker is
serialized in its JSON form, so the rendered document still parses and the
daemon's malformed-JSON rule is not tripped by the renderer itself. That is a
separate obligation from provenance exclusion, because a replacement can
preserve every source position's exclusion and still emit a document the next
stage must reject. Prove the unbounded interval transformation in Verus during
2.2.3, then connect runtime byte offsets and renderer branches to the model.
Detector completeness is an external limitation, not a premise claiming all
secrets were found. Witnesses: empty/full/overlapping/adjacent intervals,
multibyte text, and a redacted JSON field that must remain parseable; negative
controls: copy one covered byte and require a counterexample or failed proof,
and separately emit the bare marker inside a schema-declared JSON field and
require daemon revalidation to reject it.

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
`tools/verus/SHA256SUMS`, use the installed `prover-tools` binary, and record
zero failed goals and the trusted dependency audit. Their explicit acceptance
commands are:

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
documented later gap. No compatibility layer is needed.

### EP-M2: Publish the implementation handoff

**Complete (2026-10-15).** `docs/users-guide.md` gained a "Local safety
defaults (planned)" section before the existing build material. It covers the
workspace descriptor and the moved-root, changed-origin, linked-worktree, and
cross-machine consequences; override limitations; the mandatory exclusions and
additive deny patterns; the three raw-text modes with `encrypted` rejected; the
detector corpus and its false positives; the purge preview, scope-bound
confirmation, and capability gate; what to back up and how; and what purge
cannot erase, including the tombstone and re-enablement.

`docs/developers-guide.md` gained "Local safety policy reuse" before the
interface conventions: ownership, permitted callers, the fixed composition
order (authority, workspace, deny, redact, persist/project), validated input,
semantic error classes, the storage-binding constraints that matter for
interval provenance and purge fencing, and a subsection giving each V-ID,
V-TEXT, and V-PURGE obligation its roadmap owner, witnesses, negative control,
proof command, and integration-test requirement. Detailed architecture stays in
the design document and ADR 015. `docs/contents.md` summarises both additions.

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
back title, body, head, base, and draft status. Publication is authorized. This
paragraph described the state before approval; execution was authorized on
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
