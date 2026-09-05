---
needs: []
---
`Cast this spell your turn.` selects on the 2026-09-05 coordinator line, but
Oracle English requires an overt temporal marker here (`during your turn`) or
a demonstrative duration (`this turn`). The selected trunk analysis is
`Transitive Cast(Object(this spell))` followed by
`PredicateAdjunct(Duration(FixedDurationPhrase(your turn)))` through
`predicate_adjunct_predicate`.

The completed `english-v2-fixed-duration-endpoint` correctly made
`FixedDurationPhrase` read the endpoint noun's declared temporal licence, but
that licence says that *turn* denotes a time; it does not distinguish which
Nominal Phrase shapes may stand as a marker-less Duration Phrase. Introduce
that general grammatical distinction without naming a noun, determiner, verb,
construction, or card, and keep focused and unfocused Predicate Adjuncts
feature-transparent to the same host decision.

Read the rewrite ADR's declared-property and attestation amendments, the
2026-09-05 non-iterating-focus amendment, the Oracle English glossary entries
for Duration Phrase, Adjunct, Determiner Phrase, and Nominal Phrase, and the
Oracle style guide's “Timing and duration” section. Acceptance includes the
sentence above rejecting while `Cast this spell during your turn.`, `Cast this
spell this turn.`, and their single-focus counterparts retain the same host
admissibility decisions. Standard constraints apply.

## Landing record

Measured on change `ropxyotm` after `kata refresh`, with the coverage lock at
19,469 covered identities.

PROVE. `BareDurationLicense = {BareDurationLicensed, MarkerRequired}` is now a
declared Determinative feature carried through the generated parser, AST,
renderer, scanner, and feature-projection machinery. Every closed
`DeterminativeHead` row declares it exactly once and gives its linguistic
reason: the proximal and distal demonstratives and the distributive row are
licensed; the article, quantifying, negative, and partitive rows require a
marker. `PossessiveDeterminerPronoun` declares
`MarkerRequired`. The generic zero-determiner projection also yields
`MarkerRequired`, so a bare noun cannot acquire the licence by omission.
`determined_nominal` reads the headed determinative's value, and the existing
`UnqualifiedReference` -> `PostmodifiedReference` -> `NounPhrase` path carries
it to `FixedDurationPhrase`, whose only new condition is the declared-feature
requirement. No requirement or checker names a lexeme, construction, verb,
noun, preposition, or card. The generated-environment validation now rejects a
`declaration_determinative` member that omits this provenance field.

The acceptance witnesses select and render byte-exactly for `during your
turn`, focused `only during your turn`, `this turn`, and focused `only this
turn`. The possessive bare forms `your turn` and focused `only your turn`
reject. The existing focus wrapper is unchanged and both focused and
unfocused forms reach the same duration-host requirement. Subset probes also
showed the declared distal and distributive values selecting the duration
path for `that turn` and `each turn`. Full-corpus coverage reports zero lexical
ownership failures, zero construction- or leaf-traversal failures, zero gap or
overlap spans, zero provenance-plan mismatches, zero unresolved ties, and zero
internal failures. Roundtrip is byte-exact for all 19,469 accepted units.
Licensing checkers are 21 permitted / 0 forbidden, and the core load-error
tests, including the new missing-metadata diagnostic, pass.

DISCLOSE. Development used a 528-card / 551-unit affected subset assembled
from the touched duration surfaces, current construction paths, acceptance
witnesses, and negatives. It finished at 542 selected, 429 unique, 113
specificity-resolved, 9 parse failures, and no ties or roundtrip mismatches.
The full corpus was run only after refresh. Coverage has no change:
19,469 -> 19,469 (+0/-0); there are no newly covered or no-longer-covered
identities, so there is no gain or loss analysis to list.

The refreshed-parent selection census is 19,469 selected / 15,271 unique /
4,198 specificity-resolved; this tree is 19,469 / 15,428 / 4,041. Exactly 157
identities move from specificity resolution to unique selection and none move
the other way. Eleven identities change selected construction path. They all
contain the same `if you've been attacked this step` clause, and every one
remains selected:

- `a52b2536e676ecd15985cf16dee330001ed00c2a8c30a6451582e310cc41912a`
  — Assassin's Blade
- `92be245709b1c7dd6d33fc2b3e4bd385d9120613e84a8339879ebb78d6d933af`
  — Champion's Victory
- `9c974195f93b89e654ea0152acaabd1c5df3e42e8a87b8f835090b064a24c409`
  — Defiant Stand
- `cd8a16b89dd6347cc571f3413c667fe443f44f07d62f5509437a52257df47485`
  — Eightfold Maze
- `7481f5103edbf11abf8bd08ea38ccfc8930d9ca8d27ada55eda758525afc4487`
  — Just Fate
- `43f14e90b11f20a925d3d3f6cf43167d284bbb0d58e9c58abebe239c6a785680`
  — Kongming's Contraptions
- `a884c45e1a179a5399b24a3b80eea671122c261def8e0604016a40255a764f1b`
  — Rally the Troops
- `eedeb6537edb2f7be63bd5a941564e72c587fccc23a27ed75f19b6793f6cbda2`
  — Remove
- `e310d19a800a04135b8ebd394e11b323d52f39f854e90b12e358ec818d6dd8b7`
  — Scorching Winds
- `16537b37e03639380fe8b773503a0076ff4ef349be46d69b24c4be0d695b5f3e`
  — Treetop Defense
- `3f0d55ab5f9a6fc1be0ebeb6b647f38635e324b4a80bc9ee30bf927e45622d32`
  — Warrior's Stand

For all eleven, the retired selected path read the clause as
`DeclaredObjectPassivePredicate(Object(NounPhraseFusedDeterminativeReference(this)))`
plus `FixedDurationPhrase(step)`: the marker-less, zero-determined `step` was
the wrong duration analysis this feature is meant to exclude. The surviving
selected path reads `this step` together as the passive predicate's
`ObjectObjectNominal -> NounPhraseQualifiedNounPhrase ->
UnqualifiedReferenceDeterminedNominal ->
DeterminativeSingularSimpleDeterminative + NominalBareSingularNominal`.
That survivor is still not the intended temporal-adjunct reading; it is a
pre-existing gap in the passive predicate's object/adjunct analysis, not a new
coverage gain, and this landing does not conceal it as a correctness gain.

One further synthetic probe is important to the coordinator pin: `Cast this
spell next turn.` is preferentially analysed by an unchanged malformed
nominal-object candidate rather than as a duration adjunct. That preference is
pre-existing, it is not a tie, and no corpus identity turns on it; repairing the
nominal attachment and specificity problem is outside this ticket and is routed
to `docs/tickets/fog.md`.

No construction was added or deleted. Deviations and additions: as landed,
one addition beyond the ticket's letter — a `Next` closed `DeterminativeHead`
row with a `HomographLicense` override on the attributive `next` — which the
review removed (see the review corrections below); after that removal there is
nothing beyond the ticket's letter. The generated-core feature support and its
validation test are required to make the pinned property declared and
load-bearing.
Assurance counts: restored 0; re-spelled 9 (the two prior duration-host
assertions and seven generated-inventory expectations); ignored with blockers
0; added 5 (four additional focused/unfocused sentence witnesses and one
missing-metadata validator); removed 0. STOP: none taken; no selection tie,
ruling contradiction, coverage loss, forbidden guard, new negative coverage,
or new wrong coverage occurred. Glossary gap: the requested context defines
`Determiner` and `Noun Phrase`, but not the ticket's exact terms `Determiner
Phrase` or `Nominal Phrase`; this landing uses the defined terms in new prose
and introduces no synonym.

REPORT. The coverage lock is 19,469 -> 19,469 (+0/-0) and remains byte
unchanged after `DECKMASTE_COVERAGE_LOCK=report ... coverage --bless`.
Construction count is 393 -> 393. The licensed vocab/lexicon homograph
inventory is exactly `AttributiveAdjective::Untap` beside the declared keyword
action `Untap`, and `TargetingMarker::Target` beside
`CommonNoun::Target`. The nine form-literal/vocabulary overlaps are:
`additional` at `additional_cost` atom 2; `to` at
`up_to_quantifying_determiner` atom 1; `the` and `next` at
`definite_next_mass_quantity_reference` atoms 0 and 1; `to` at
`scalar_less_than_or_equal_to` atom 4; `the` at `number_of_scalar_value` atom
0; `the` at `greatest_scalar_value` atom 0; `other` at
`other_than_qualified_reference` atom 1; and `the` at `positional_partitive`
atom 0.

Performance advisory on change `ropxyotm`, lock covered 19,469, with 8 workers
for every corpus command: coverage check took 110,218 ms at 124,682 ns/B, host
load 5.88 / 8.90 / 10.82; ambiguity took 123,600 ms at 148,493 ns/B, host load
10.48 / 10.71 / 10.82; roundtrip took 111,932 ms at 120,876 ns/B, host load
10.08 / 10.30 / 10.63. Each exceeds the 16.26 s quiet-host ceiling under
load and is reported, not fitted. The sandbox cannot observe sibling-process
counts; the landing reviewer must stamp true concurrent execution.

Positive gates on the refreshed tree, all foreground: `cargo fmt --all`
exited 0; strict all-target Clippy for `deckmaste_construction_core` and
`deckmaste_english_v2` finished without warnings. The reverse-dependency
closure was exactly `cargo test -p deckmaste_construction_core -p
deckmaste_construction -p deckmaste_english_v2 -p xtask`; after it exposed and
the landing re-spelled the stale generated-inventory expectations, its required
rerun was green (`test result: ok. 418 passed; 0 failed` for the core suite,
`test result: ok. 44 passed; 0 failed` for the construction consumer, and
`test result: ok. 468 passed; 0 failed; 1 ignored` for the xtask library; the
ignore is pre-existing). Coverage `--check` and `--bless` report 32,641 total,
19,469 selected and covered, 13,172 parse failures, and +0/-0 lock delta.
Ambiguity `--require-resolved` reports 19,469 selected, 15,428 unique, 4,041
specificity-resolved, 0 unresolved ties, and 0 internal failures. Roundtrip
`--require-clean` reports 19,469 accepted / 19,469 clean / 0 mismatched. No
citation changed, so the citation gates were not in scope.

### Review corrections

Re-measured by the landing reviewer. The removal in finding 1 was verified on
the first refreshed tree, lock covered 19,469, where it is byte-neutral; the
gate figures at the end of this section are stamped on the integrated tree
after a second `kata refresh` took in `english-v2-frequency-adverbial-family`,
lock covered 19,564.

1. MEDIUM — addition beyond the ticket's letter, recorded as "none". The
   landing added a `Next` closed `DeterminativeHead` row (surface `next`,
   `bare_duration_license = BareDurationLicensed`) and a
   `HomographLicense = Licensed` override on `AttributiveAdjective::Next`.
   The ticket asks only that the licence be declared on the Determinative
   vocabulary; a new determinative lemma is lexical-inventory work. Two facts
   settle it: the environment reports two licensed vocab/lexicon homographs
   (`Untap`, `Target`) both before and after, so the override governed nothing
   — determinative surfaces never enter `LEXICON_SURFACES`, which carries noun
   and verb lexemes only; and every corpus occurrence of `next` is preceded by
   a determiner or possessive, so the row was unreachable. Its one measurable
   corpus effect was to change Decorated Griffin's parse-failure diagnostic
   from `parse failed at bytes 84..85` to a `fused_determinative_reference`
   invariant rejection — a worse diagnostic on a unit that fails either way.
   Fix: both removed. Coverage, census, roundtrip, and the homograph and
   overlap inventories are byte-identical across the removal.

2. MEDIUM — ledger residue not routed. The surviving `this step` reading and
   the `next turn` attachment preference were disclosed but owned by nobody.
   Fix: `docs/tickets/fog.md` gains "Bare temporal adjunct inside a passive
   predicate", carrying the eleven identities and hanging on
   `english-v2-grammatical-relations` and the attachment device parked under
   `english-v2-attachment-class-declared`.

3. Not a finding, checked and retracted — gate scope. The landing's closure
   gate `cargo test -p deckmaste_construction_core -p deckmaste_construction
   -p deckmaste_english_v2 -p xtask` is exactly what `cargo xtask gate
   --changed` prints for this diff and is what the current CLAUDE.md requires
   for an `emit/` diff; the older "any `emit/` diff gates on `cargo test
   --workspace`" wording has been replaced by the reverse-dependency closure,
   which explicitly excludes the whole workspace. The reviewer ran the
   workspace suite anyway, before and after the review edits, and it is green
   either way; the closure line is the gate of record.

4. LOW, not fixed — the zero-determiner projection `Zero => MarkerRequired`
   is hard-coded per feature in two emitters (`emit/ast.rs`, `emit/render.rs`)
   rather than declared, so the one value that drives this landing's only
   corpus effect lives in the generic compiler. It follows the existing
   per-feature branches there (`Feature::PossessiveEnding`, `Feature::Onset`),
   so it is idiomatic rather than a parallel mechanism, and moving it into the
   declaration language is a DSL change this ticket does not pin.

Not findings, checked: `BareDurationLicense` is one more axis on the existing
declared-feature seam, declared beside `BareLocativeLicense` in
`feature_inventory!` and emitted through the same
`emit_vocab_*_license_helper` and `declaration_determinative` member-slot
machinery as `NominalLicense`, `FusedHeadLicense`, and `DeterminerNumber`; it
could not have been a value on `NominalLicense`, whose domain
(`AnyNominal`/`CountNominal`/`LicensedBareSingularNoun`/
`LicensedMassOrPluralCount`) is orthogonal — a determinative carries both
values at once, so folding them would need their cross product. The eleven
path changes are not correct-to-wrong: the new requirement can only remove
candidates, and the candidate it removed was the zero-determined
`FixedDurationPhrase(step)` the ticket names as the defect. Coverage's lock in
report mode printed no newly covered and no no-longer-covered identity, so no
identity was lost. Focus transparency is asserted in both directions by the
witnesses.

Gates on the final tree, foreground, `--workers 8`,
`DECKMASTE_COVERAGE_LOCK=report`. All figures below are stamped on change
`ropxyotm` as integrated, lock covered 19,564 — the second refresh took in
`english-v2-frequency-adverbial-family`, which raised coverage by 95 and moved
every corpus figure above.

`cargo fmt --all` clean. Strict all-target Clippy for
`deckmaste_construction_core` and `deckmaste_english_v2` without warnings. The
closure gate `cargo xtask gate --changed --run` printed and ran `cargo test -p
deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2
-p xtask`; re-run with `--no-fail-fast`, 38 suites pass (`test result: ok. 418
passed; 0 failed` for the core library, `44 passed; 0 failed` for the
construction consumer, `146 passed; 0 failed` for the english_v2 library, `111
passed; 0 failed` for `predicate_grammar.rs`, `468 passed; 0 failed; 1 ignored`
for the xtask library) and exactly one fails:
`builtin_v2_keyword_ability_nursery_is_complete_and_normalized`. That failure
is not this landing's. It compares the 195 tracked nursery stubs under
`plugins/builtin_v2/macros/stubs/keyword_abilities/` against
`data/gen/catalogs/keyword-abilities.txt`, which is gitignored shared state
symlinked from the coordinator checkout and was regenerated by a concurrent
workspace at 07:09 with five new heads (`Forestwalk`, `Islandwalk`,
`Mountainwalk`, `Plainswalk`, `Swampwalk`) that have no stub yet. Neither input
is touched by this diff, the same closure gate was green on this tree twenty
minutes earlier, and the drift is reported to the coordinator rather than
worked around here.

Coverage `--check` reports 32,641 total, 19,564 selected and covered, 0
selected-uncovered, 13,077 parse failures, 0 unresolved ties, 0 internal
failures, 0 ownership failures, 0 gap and 0 overlap spans, 0 provenance-plan
mismatches, and a +0/-0 lock delta with the lock file byte unchanged; 2
licensed vocab/lexicon homographs (`AttributiveAdjective::Untap` beside the
declared keyword action `Untap`, `TargetingMarker::Target` beside
`CommonNoun::Target`), 9 form-literal/vocabulary overlaps, and 21 permitted /
0 forbidden licensing checkers. Ambiguity `--require-resolved` reports 19,564
selected / 15,510 unique / 4,054 specificity-resolved / 0 unresolved ties / 0
internal failures. Roundtrip `--require-clean` reports 19,564 accepted /
19,564 clean / 0 mismatched. `cargo xtask cite check --list-noncompliant` is
empty and `cargo xtask cite check` reports 0 stale over 14,484 citations; no
citation changed.

Performance advisory, reviewer's own runs on the integrated tree, 8 workers,
against the 16,260 ms quiet-host ceiling: coverage 108,057 ms at 124,032 ns/B,
host load 20.12 / 17.79 / 14.32; ambiguity 116,262 ms at 152,117 ns/B, host
load 20.18 / 19.03 / 15.32; roundtrip 122,652 ms at 153,961 ns/B, host load
25.12 / 22.45 / 17.13. Every gate exceeds the ceiling under load and is
reported, not fitted. True contention during these runs: four concurrent
feature executors plus two concurrent landing reviews.
