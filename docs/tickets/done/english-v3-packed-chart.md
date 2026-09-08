---
needs: [english-v3-lexical-model]
---
# Pack incomplete and complete v3 derivations

Replace the prototype chart's explicit `PartialFamily` child vectors with a
packed representation for incomplete as well as completed derivations. Earley
items identify recognizer state; shared intermediate nodes or equivalent packed
edges retain alternative histories without enumerating their Cartesian product
through the agenda. Completed nodes preserve every family needed to recover a
Reading lazily.

Consume position-indexed lexical occurrences from `deckmaste_lexical`; retain
overlapping and multiword alternatives without retokenizing or contextual POS
selection. Define grammatical summary identity by future admissibility: two
constituents may share a packed node exactly when every possible parent would
treat them alike. Preserve open agreement, frame, extraction, sharing and
recoverability constraints until their governing context is available. Surface
positions and spelling evidence remain on lexical/literal leaves rather than in
grammatical summaries.

Exercise prediction, scanning, completion and nullable productions, including
shared-child growth after an item was first observed. Distinguish one Reading
with duplicate derivations from two grammatical Readings. Generated Reading
identity consists of lexical identities, constituent structure and grammatical
feature values; distinct source spellings remain evidence on their leaves.
Deduplicate equivalent derivations by that identity during lazy
materialization, never by materializing values to decide chart admission. Lazy
materialization must also detect cyclic derivations.

Acceptance measures lexical alternatives, items, intermediate nodes, completed
nodes, families, completion work and requested derivations on homographs,
agreement, overlapping multiword forms, nullable rules, ambiguous attachment
and a correlated-alternative counterexample. State the actual complexity bound
provided by the chosen representation and disclose where grammar/feature
cardinality enters it; do not claim cubic behavior from Earley ancestry alone.
Standard constraints apply.

## Landing record

Measured on change `nmtozomv`, with the unchanged production coverage lock at
20,254 covered identities. The new `deckmaste_english_v3` crate provides the
runtime contract for the construction-compiler ticket. Its
[README](../../../crates/deckmaste_english_v3/README.md) defines summary identity,
the packed representation, materialization obligations and the actual bounds.

### PROVE

Incomplete derivations use one intermediate node per
`(production, dot, origin, end, state)`, with empty or binary prefix/child edges.
Completed nodes group `(category, origin, end, summary)` families. The agenda
never carries a child vector or constructs a Reading. Waiters consume each
completed summary once; subsequent child-family growth is visible through
existing edges. The nullable witness exercises both waiter/child discovery
orders and retains all four combinations after late shared-child growth.

Admission callbacks see declared lexical features and grammatical state, not
lexeme names, source positions or materialized children. Equal summaries must
have identical future admissibility; prefix states additionally retain local
grammatical choices needed at construction. Correlated Number/Person tuples
admit the two licensed pairings and reject the independently plausible crossed
pairing. Agreement, frame, extraction, sharing and recoverability obligations
survive an intervening constituent until the governing parent checks them.
The lexical-frame witness consumes a declared selected slot.

Lazy materialization preserves lexical identities, constituent structure,
feature values, capitalization and declared spelling variants. Duplicate
productions yield one Reading; unrelated homographs yield two. Local feature
choices and lexical count/mass uses survive even when their outer summaries
coincide. All 13 `cast`, both `counters`, and all three `one` analyses survive
their synthetic grammar. Multiword and bound overlaps retain both whole and
segmented structures. Independent lexical values with declared variants and
Unicode spellings satisfy realization/reanalysis; every finite acceptance
census checks byte-exact realization. The attachment ownership witness checks
adjacent, nonoverlapping scalar ranges through every tree and reproduces the
lexical traversal order, including newlines and Unicode capitalization.

Cyclic derivations report an explicit error without suppressing finite sibling
paths or confusing repeated nullable siblings with cycles. Malformed occurrence
positions, unlicensed variants and inconsistent token input report input errors.
Injected materializer failures are explicit internal errors, not grammatical
rejections. All ordinary acceptance censuses report zero internal failures and
zero cycles; the cycle/error tests deliberately exercise those error channels.

No silent production loss: no existing parser, lexical declaration, construction
declaration, coverage identity or production consumer changed. No production
identity is newly covered or retired, and no ledger residue is created. There
are no new word-naming licensing guards. Runtime lexical identity lookup only
validates the supplied lexical occurrence against its immutable environment;
grammatical callbacks receive declared properties without the lexeme identity.

### DISCLOSE

The runtime's generic summary contract is not a proof that every future grammar
implements sound equivalence. Generated admission and materializer obligations
belong to `english-v3-construction-compiler`; the witnesses here establish the
runtime's preservation and exclusion behavior for the supplied finite grammars.
Likewise, these are handwritten synthetic fixtures; declaration-generated
roundtrip evidence remains with that ticket and the generated-slice ticket.

The representation has `O(D S n²)` items, `O(C Q n²)` completed nodes,
`O(D S Q n³)` completion pairs, and
`O(B D S (Q n³ + n L) + D S n²)` intermediate edges. Here `S` and `Q` are
feature-sensitive state/summary cardinalities, `B` transition branching, `D`
dotted grammar positions, `C` Categories, and `L` projected lexical alternatives.
Tree-map factors, callback/key costs, literal lengths and stored feature sizes
also enter runtime. There is no unconditional cubic bound or polynomial-delay
Reading enumeration claim. A cyclic forest cannot certify a complete finite
Reading census by merely skipping its error. Continuations can clone partial
values; deduplication stores returned values and can traverse many duplicate
derivations before finding another Reading.

Deviations and additions:

- The current tracked tree has no v3 prototype crate. Added the fresh v3 runtime
  required by the accepted architecture, rather than rewriting the v2 engine's
  `PartialFamily` in place. The archived experiment remains optional salvage;
  no compatibility adapter or v2 dependency was introduced.
- Added the workspace member/lock entry and runtime README. No authored English
  construction was added or removed.
- Added local-feature and lexical-use retention tests to establish that the
  materializer can recover distinctions hidden by outer-summary equivalence.
  Added malformed-input and explicit materializer-error tests to cover the new
  public error boundary. The other fixtures implement the ticket's named cases.
- Tests: restored 0, re-spelled 0, ignored 0, added 18, removed 0. No existing
  assertion was weakened or deleted. During development the distinct-summary
  prefix test's assumed subtree reuse count was corrected to the bound for two
  requested trees: traversal order can rebuild both trees, and no minimum reuse
  count is part of the contract.

No unresolved STOP, recorded-ruling contradiction or glossary gap remains.
No CR citation changed. Production selection/preference, environment-loader
errors and licensing-checker inventories are unchanged and were not remeasured;
the new runtime has no preference or representative-selection mechanism.

### REPORT

All counts below are stamped with `nmtozomv`, lock covered 20,254. Items and
intermediate nodes coincide. Families include intermediate edges and completed
families. Requests include a final exhaustion call for exhaustive censuses;
the long-prefix rows request only two Readings.

| Witness | Lexical alternatives | Items / intermediate nodes | Completed nodes | Families | Completion work | Requests | Derivations / Readings |
|---|---:|---:|---:|---:|---:|---:|---:|
| Duplicate homographs | 2 | 4 | 1 | 8 | 0 | 3 | 4 / 2 |
| Correlated agreement | 4 | 13 | 6 | 19 | 6 | 3 | 2 / 2 |
| Crossed agreement exclusion | 3 | 10 | 3 | 13 | 4 | 1 | 0 / 0 |
| Correlated-parent exclusion | 2 | 4 | 2 | 6 | 2 | 1 | 0 / 0 |
| Overlapping multiword | 4 | 11 | 2 | 14 | 0 | 3 | 2 / 2 |
| Bound overlap | 3 | 10 | 2 | 13 | 0 | 3 | 2 / 2 |
| Nullable late growth | 0 | 11 | 5 | 17 | 5 | 5 | 4 / 4 |
| Attachment, 3 words | 3 | 21 | 6 | 28 | 10 | 3 | 2 / 2 |
| Attachment, 5 words | 5 | 50 | 15 | 75 | 35 | 15 | 14 / 14 |
| Attachment, 7 words | 7 | 91 | 28 | 154 | 84 | 133 | 132 / 132 |
| Distinct-summary prefix, 4 words | 8 | 20 | 9 | 33 | 8 | 2 | 2 / 2 |
| Distinct-summary prefix, 16 words | 32 | 80 | 33 | 129 | 32 | 2 | 2 / 2 |
| Distinct-summary prefix, 64 words | 128 | 320 | 129 | 513 | 128 | 2 | 2 / 2 |

The 64-word distinct-summary prefix represents two choices at each position
(`2^64` Readings), with 384 intermediate edges and 129 completed families.
Requesting two Readings completes exactly two derivations and performs 130
construction builds, including their descendants. The four-word case also
exhaustively checks all 16 Readings. Lexical count/mass projection separately
measures one occurrence becoming two grammatical leaf alternatives; both survive.

Fixture homograph/overlap inventory: `Light` (two adjective identities), `cast`
(13 forms/bundles), `counters` (Noun/Verb), `one`
(Noun/Determinative/Numeral), `first strike` (whole/segmented), `islandwalk`
(whole/bound). There is no lexical/form-literal overlap in the fixture grammar:
its literals are separators, punctuation or empty strings. Production
construction count, production homograph and form-literal inventories,
selection census and permitted licensing-checker total were not remeasured;
their code and declaration inputs are unchanged. V3 production construction
count is zero pending the compiler and activation sequence.

`cargo xtask gate --changed --clippy --run` derived and passed
`cargo test -p deckmaste -p deckmaste_english_v3` and
`cargo clippy -p deckmaste -p deckmaste_english_v3 --all-targets -- -D warnings`.
All 18 active tests passed, with no ignored tests. The root package has no tests;
it is included because the workspace manifest changed. Repository `jj fix` and
nightly formatting checks passed; the final formatted tree's acceptance census
and the same derived Clippy closure passed. `kata kanban check` passed.

Performance advisory: the single-worker acceptance tests took 0.02 s; the
command including incremental compilation took 0.883 s on a host sampled at
load averages 6.83 / 7.64 / 6.94. This is synthetic chart evidence, not a
production coverage command or accepted-byte CPU measurement. The 16.26 s
quiet-host coverage ceiling and production ns/B telemetry were not remeasured
because v3 has no corpus consumer and the production parser is unchanged.
Session evidence is in `/tmp/english-v3-packed-chart-acceptance.log` and
`/tmp/english-v3-packed-chart-gate.log`; the README command reproduces the
acceptance counts without those files.
