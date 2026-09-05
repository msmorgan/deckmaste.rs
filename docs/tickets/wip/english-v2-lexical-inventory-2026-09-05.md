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
