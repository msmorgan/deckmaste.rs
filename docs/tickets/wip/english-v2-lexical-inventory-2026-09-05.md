---
needs: []
---
**One round's lexical gaps, batched.** Every row below is a *data* line: a
member added to an inventory the grammar already reads, reaching consumer
constructions that are already standalone `lex(...)`/`verb(head)` forms, adding
zero constructions, sums, seams or features. Minted as a batch by coordinator
ruling (2026-09-05) after `english-v2-tail-color-property-adjectives` spent a
whole claim/executor/review/integrate cycle on a single vocabulary member.

Measured on change `ptoxwkmmrqno` (19,469 / 32,641 covered, 13,172 parse
failures, first-failure byte attribution; re-measure at claim). **545 units**
across six rows.

Every row was verified by probe against a *control* — a sentence identical in
shape whose lexeme already exists. The control selecting is what proves the
consumer is reachable and the gap is the member alone.

| surface | units | inventory home | consumers (all standalone forms) | licence | witness / control |
|---|---:|---|---|---|---|
| `kicked` | 194 | keyword-action/verb lexeme with a participle; no `Kick` identity exists anywhere (`core_verbs.ron` has none) | `predicative_declared_participle` (`:1777-1781`, `form = verb(head)`) | *kicked* is the participle of the kicker keyword action | `Activate only if this creature was kicked this turn.` fails; control `…was exiled this turn.` **selects** |
| `swampwalk`, `islandwalk`, `forestwalk`, `mountainwalk`, `plainswalk` | 127 | `plugins/builtin_v2/macros/stubs/keyword_abilities/` — `Landwalk.ron` exists, the printed fused surfaces do not | `bare_keyword_line_item` (`:4878-4882`, `form = lex(keyword)`) | each is a printed keyword ability in its own right; `macro-keyword-templates` states typed landwalk "stays bare keyword names", i.e. not templated | corpus: Anaconda `Swampwalk`, Boggart Loggers `Forestwalk` |
| `share` / `shares` | 91 | verb identity, `core_verbs.ron` | `finite_subject_gap_relative_clause` (`:4420-4423`, `form = licensed("that") verb(head)`) and the transitive subject-gap relative | ordinary transitive verb | `Destroy target creature that shares a color with it.` fails; control `…that controls a land.` **selects** |
| `win` | 77 | verb identity, `core_verbs.ron` | finite clause predicate | ordinary verb | `If you win, draw a card.` fails; control `If you control a Goblin, draw a card.` **selects** |
| `unlock` | 30 | verb identity, `core_verbs.ron` | same as `win` | ordinary verb | corpus: Bottomless Pool // Locker Room `When you unlock this door, …` |
| `exert` | 26 | verb identity, `core_verbs.ron`; the `Exert` keyword-**action** stub covers only the declaration line, not the predicative verb | finite/bare predicate | ordinary verb | corpus: Ahn-Crop Champion `You may exert this creature as it attacks.` |

Affected subset. Surfaces: `\b(kicked|swampwalk|islandwalk|forestwalk|mountainwalk|plainswalk|shares?|win|unlock|exert)\b`.
Touched constructions: `predicative_declared_participle`, `reduced_relative_modifier`,
`bare_keyword_line_item`, `finite_subject_gap_relative_clause` and the transitive
subject-gap relative, plus every card whose parent-tip selected path contains one
of them. Witnesses: the six rows' cards. Negatives: any card using `exiled`,
`controls`, `landwalk`, or a bare keyword line must not move.

Routed OUT of this batch, each with the reason and the owner — do not add them here:

- `cycling` fused surfaces (`landcycling`, `plainscycling`, …), 91 units →
  `macro-keyword-templates`, which design-gates typed cycling explicitly
  ("a bounded slot-reader codec … a codec change no existing macro uses").
- `exploits` and the other ability-derived verbs, 23 units →
  `english-ability-derived-verb-batch`, which names `exploit` and records a
  prior over-fire regression.
- `dealt by` / `controlled by`, 154 units → `english-v2-remaining-prepositions`.
  Verified: `Prevent all combat damage that would be dealt by target creature
  this turn.` fails **at `by`**, not at `dealt`, and `Deal` already declares its
  participle. Adding lexemes here would gain nothing.
- `addition` (`in addition to its other types`), 170 units →
  `english-v2-locative-licence-set`. The noun is missing *and*
  `nominal_preposition_is_licensed` refuses every `to`-postmodifier; the member
  alone cannot land it.
- `died`, `attacked`, `entered`, `left`, `gained`, … → the preterite has no
  finite realization at all, which is structural:
  `english-v2-tail-preterite-finite-clause`. `Attack` already has its frames and
  `…if a creature attacked this turn.` still fails, so these are not lexeme gaps.
- `named` — dropped. Zero units under this round's bucketing key and the
  consumer was not verified; re-derive before proposing it again.

`english-v2-rename-color-vocabulary` is **not** folded in: its own body leaves
the replacement name and any glossary amendment to be chosen at implementation,
which is a design dimension, not a data line.

Ruled against: a construction, sum, feature or seam added for any row — a row
needing one is misclassified and must be split out and reported; a `require` or
`checked by` naming any of these surfaces; narrowing an existing form to keep a
number.

Acceptance: the standard landing record; each row's witness selects and its
control still selects with an unchanged analysis; **the construction count is
unchanged** (that is the batch's defining property — a change to it means a row
was structural); both byte-exact laws green with total ownership, zero ties.
Report gains per row, since a row that gains nothing was misdiagnosed.

Baseline: change `ptoxwkmmrqno`, 19,469 covered of 32,641, 13,172 parse
failures, 0 ties, 0 internal failures. Re-measure at claim.

Tier: **terra**. Standard constraints apply.

## Landing record

### PROVE

- `kata refresh` completed before the corpus measurements.
- The affected-surface subset contained 801 card names and 743 corpus units;
  its ambiguity census had 0 unresolved ties and 0 internal failures.
- On the refreshed feature tree, `DECKMASTE_COVERAGE_LOCK=report cargo xtask
  english_v2 coverage --check --workers 8` completed: 19,844 selected and
  covered units of 32,641, 12,797 parse failures, 0 ties, and 0 internal
  failures. The subsequent `--bless` delta was +375 covered identities and
  -0; the lock was then retracted with the unsafe payload below.
- Refreshed ambiguity (`--require-resolved --json --workers 8`) completed with
  19,844 selected units, 15,529 unique selections, 4,315
  specificity-resolved selections, 0 ties, and 0 internal failures. Its exact
  parent diff had 375 changes, each a parent parse failure becoming selected;
  no selected identity changed analyses and no identity was lost.
- Refreshed roundtrip (`--require-clean --workers 8`) completed cleanly.

### DISCLOSE

- **STOP — negative selected.** The changed closure reached an existing modal
  negative in `crates/deckmaste_english_v2/tests/ability_logic.rs:4411`; it
  selected after the proposed participial inventory member was added. This is
  an explicitly forbidden newly selected negative. I did not narrow a form,
  add a guard, or reinterpret the negative. The entire feature payload and
  generated coverage lock are restored to the claimed parent; this record is
  the only safe retained change.
- The closure printed
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and its matching strict clippy command. It is red at the STOP condition, so
  neither is reported as a passing final gate.
- The temporary inventory-count and catalog consistency corrections exposed by
  the closure were also retracted. No construction, sum, seam, feature, or
  word-naming guard is retained.

### REPORT

- Wall clock: 2026-09-05 06:26 PDT to 2026-09-05 07:02 PDT.
- Coverage before the proposed payload: 19,469 / 32,641. Pre-STOP measurement:
  19,844 / 32,641; lock delta +375 / -0. No coverage result is claimed for the
  reverted tree.
- Construction count: unchanged by the proposed diff; no construction source
  file was changed. Selection census after reversion is not re-measured because
  this ticket stops rather than lands.
- Pre-STOP performance advisory: coverage 125,797 ms and 132 ns/B; ambiguity
  131,912 ms and 158 ns/B; roundtrip 111,157 ms and 137 ns/B; workers 8;
  observed host load 17 / 16 / 15, then 13 / 13 / 14, then 9 / 11 / 13.
  The 16,260 ms advisory ceiling was exceeded under concurrent load; this was
  advisory only.
- Assurance counts: restored 11 payload paths; re-spelled 0; ignored 0; added
  0; removed 0. The temporary scratch census and coverage artifacts will be
  deleted before handoff.
- No glossary gap, citation change, or decision is requested beyond the STOP.
