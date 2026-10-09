---
needs: []
---
# Read *at least* before a cardinal: you control at least two other Merfolk.

## Why

*at least N* never reads although *N or more* does. On census `qsupksky`
(32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of
2026-10-08), the recon bucket "at least" touches **102** unread faces and is
the sole cause on **78**. Counts are unread faces *touched* (at least one
localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does); they are surface counts, not gain forecasts.
`cargo xtask english-v3 grep -i '\bat least\b'` finds the same 102 faces, all
unread at `qsupksky`; *at most* has 0 faces.

The planned `english-v3-postpositive-comparative-determinatives` has
witnesses of the shape *at least 10 life more than …* (Chalice of Life,
Righteous Valkyrie); this ticket unblocks their *at least* half. No `needs`
edge either way.

## Goal

*at least* reads as a lower-bound modifier of a cardinal wherever a cardinal
reads today: as a Determiner (*at least two other Merfolk*, *at least one
Angel card*) and inside a measure phrase (*at least 7 life*, *at least four
mana*), with byte-exact realisation.

## Analysis

CGEL Ch. 5 §11(e), p. 432, [7]: the PPs *at least* / *at most* mark lower and
upper bounds inside the DP and "occur mainly with numerals". That backs the
PP-modifier analysis only.

Existing route: `PrepositionDeterminer`
(`crates/deckmaste_english_v3/src/declarations.rs:4114`,
`[value: QuantitativePrepositionPhrase]`) already reads *up to two* through
the `QuantitativePrepositionPhrase` schema and its instance rows
(`declarations.rs:4053`, `:4070`; *up* declares
`QuantitativeComplement: CardinalPrepositionPhrase`, *to* declares
`QuantitativeComplement: Cardinal`, `lexicon/core.ron`). `CardinalDeterminer`
(`declarations.rs:4107`) and `DegreeDeterminativePhrase` (`:4135`, a
Determinative modifying a Determinative) are the neighbouring routes.
*least* is `vocab:Adjective/Least` (`lexicon/core.ron:1037`, no features);
*at* is `vocab:Preposition/At` (`core.ron:209`).

Orchestrator rulings 2026-10-09:

- R9a. *at least* is a PP Modifier of the cardinal (CGEL p. 432), not a fixed
  multiword Determinative lexeme and not a new Determinative. *least* keeps
  its Adjective lexeme; whatever licenses it as the Complement of *at* is a
  declared feature, never a word test.
- R9b. Extend the existing `QuantitativePrepositionPhrase` /
  `PrepositionDeterminer` family (a new instance row or a widened
  Complement) before adding a Construction. The bounded cardinal must stay
  one constituent so that measure phrases (*at least 7 life*) and Determiners
  share the route; if the measure position needs a second consumer, that is
  a refinement of the existing Cardinal consumer, not a parallel route.
- R9c. Pruning rule: *at most* is unattested (0 faces); do not license it.
- R9d. *at least N* must not also read as *at* + NP (*least two other
  Merfolk*) or as a clause-level *at* Adjunct; one analysis per string.
- R9e. Number follows the cardinal, as for *up to one* / *up to two*.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control 1.

- Sokka, Lateral Strategist: "Whenever Sokka and at least one other creature
  attack, draw a card." (control "… one or more other creatures …")
- Svyelun of Sea and Sky: "Svyelun has indestructible as long as you control
  at least two other Merfolk." (control "… two or more other Merfolk.")
- Renegade Reaper: "If at least one Angel card is milled this way, you gain 4
  life." (control "If one or more Angel cards are milled this way, …")
- Synchronized Eviction: "This spell costs {2} less to cast if you control at
  least two creatures that share a creature type." (control "… two or more
  creatures …")
- Glimmer Lens: "Whenever equipped creature and at least one other creature
  attack, draw a card." (control "… one or more other creatures …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bat least\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *at least* read as *at* + an NP headed by *least*, as a clause-level Adjunct, or detached from its cardinal is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *at least that many*, *at least as many*, *at least one of* (1 face each).
- *at least two more* / *at least 10 life more than …*: the comparative half
  belongs to `english-v3-postpositive-comparative-determinatives`.
- *at most* (unattested).

## Landing record

1. **Census delta** (`ouzqotlx`): claim-parent-equivalent baseline → implementation, 300 → 370 covered faces and 11,515 → 23,371 Readings. The built-in diff reports 70 gained, 0 lost, 0 decreases, 0 increases, 100 digest-only and 232 unchanged; no source changes or unmatched identities. Lost faces: none. The digest-only changes preserve counts and reflect the added absent modifier field in existing cardinal/scalar values. Selector: all 102 supported faces matching case-insensitive `\bat least\b` plus 300 covered controls selected by built-in diff from `mqrlzszk`, seed 20261008: 402 identities. The untouched baseline was measured on `ouzqotlx`, source-equivalent to claim parent `mqunoxxy`. Evidence: `target/english-v3/english-v3-at-least-cardinal/{manifest,before,after,diff}.json`. Refresh imported the per-line census representation from trunk `yntwupzymxxl` (ADR amendment 2026-10-09). Rechecking the identical 402-face manifest preserved 370 covered faces, with 0 lost, 0 gained, 40 decreases, 175 increases, 155 digest-only and 32 unchanged; reported Readings 23,371 → 7,306. All 40 named decreases in `diff-refresh.json` are attributed to `yntwupzymxxl`: counts now sum line Reading sets instead of multiplying Document combinations; the grammatical Reading sets survive. No source changes or issues. Refreshed evidence: `target/english-v3/english-v3-at-least-cardinal/{after-refresh,diff-refresh}.json`. The final refresh then imported grammar/lexicon work `yppxtnmupsyw`; the same bounded manifest at `zsqyotqk` is identical to the verified per-line report: 402 unchanged, 0 lost/gained/decreases/increases/digest-only/source changes, 370 covered, 7,306 Readings, 0 issues. Evidence: `target/english-v3/english-v3-at-least-cardinal/{after-refresh-2,diff-refresh-2}.json`.
2. **Witnesses** (`ouzqotlx`, quoted units, complete enumeration): Sokka, Lateral Strategist 0 → 1; Svyelun of Sea and Sky 0 → 2; Renegade Reaper 0 → 2; Synchronized Eviction 0 → 3; Glimmer Lens 0 → 1. Before probes precede implementation; controls had 1 / 2 / 2 / 3 / 1 Readings. Every admitted witness Reading retains the same independently constructed bounded cardinal; other clause/adjunct scope ambiguities survive.
3. **Spot checks** (`ouzqotlx`, whole-Document counts before the census representation refresh; one selected Reading inspected per face): Sokka (1), *at least one other creature* as the second Subject conjunct; Svyelun (4), *at least two other Merfolk* as Object; Renegade Reaper (2), *at least one Angel card* as passive Subject; Synchronized Eviction (12), *at least two creatures that share a creature type* as Object; Glimmer Lens (1), coordination of the equipped-creature NP with the bounded quantified NP; The Emperor of Palamecia (8), *at least four mana* as a measured passive Subject; Valakut, the Molten Pinnacle (1), *at least five other Mountains* as Object; Ice-Fang Coatl (2), *at least three other snow permanents* as Object; Henge Walker (36), *at least three mana of the same color* as a measured passive Subject; Searing Barrage (4), *at least three red mana* as a measured passive Subject. All preserve *at* + adjectival *least* inside a PP modifying the numeral and realize byte-exactly; none detaches the bound into an NP-complement or free-Adjunct analysis.
4. **Deviations and STOPs** (`ouzqotlx`): added one Construction, `QuantitativeAdjectiveComplement`, to read the declared adjective licence directly without widening ordinary Adjective Phrase admission. Added one instance of the existing quantitative-PP schema and refined four existing numeral consumers; no new frame or superseded parallel lexical/grammar route. Derived modifier spacing permits one optional modifier field rather than incompatible surface forms. A trial admitted a wrong extra Glimmer Lens Reading: wide *equipped* over a coordinated Nominal containing a cardinal with an unresolved Determiner Requirement. `BareStatusNounPhrase` now requires that existing feature to be `No`; the wrong trial Reading is retired, while the valid NP coordination survives. Supported-corpus grep found no direct status-adjective + cardinal wording and only Glimmer Lens in the matching coordination surface. Nine tests cover the five witnesses, agreement, measures, both roundtrip laws and excluded PP functions. Glossary gaps filled: Cardinal Modifier, Quantitative Adjective Complement and its Use; CGEL backs the PP modifier function, R9a backs the complement licence. No STOPs. The binding user instructions supplied seed 20261008 and six workers. Review accepted that the 100 digest-only controls are exactly the covered faces containing `Cardinal` / `LargeCount` / `GroupedScalarNumeral` / `UngroupedScalarNumeral`, whose Debug gained `modifier: None`. The unattested strings *up to at least two* and *at least two or more* now each have one Reading by composition; both are accepted by the review ruling (2026-10-09).
5. **Tests** (`ouzqotlx`): `cargo test -p deckmaste_english_v3`: 662 passed, 0 failed, 0 ignored; `cargo test -p deckmaste_lexical_source`: 35 passed, 0 failed, 0 ignored. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: passed. The first English clippy attempt found an unnecessary clone in the new measure test; its full-value assertion now uses a borrowed slice. `cargo test -p deckmaste_english_v3 at_least_cardinal::bounded_cardinals_compose_with_both_measure_consumers`: passed (1 test). Citation checks: 0 noncompliant, 0 stale; diff audit: 0 new sites. After refresh, at `mynykspm`: `cargo test -p deckmaste_english_v3 --test main at_least_cardinal::` passed 9 tests; `cargo test -p deckmaste_english_v3 --test main packed::` passed 22 tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed. Both targeted groups had 0 failed and 0 ignored. Lexicon and construction core were unchanged by refresh, so lexical-source checks were retained. After the next grammar/lexicon refresh, at `zsqyotqk`: `cargo test -p deckmaste_english_v3 --test main at_least_cardinal::` passed 9 tests; `cargo test -p deckmaste_lexical_source` passed 35 tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. Both test runs had 0 failed and 0 ignored. Review verification on current head `vlptumulvwkklxrxurulllkklzsolxwl`: `cargo test -p deckmaste_english_v3` IN FULL passed 670 tests, 0 failed, 0 ignored (0 filtered out); doc-tests: 0 passed, 0 failed, 0 ignored. This executes the full suite after the inherited grammar, lexicon and forest changes. Evidence: `target/english-v3/english-v3-at-least-cardinal/test-full-review.log`. Test-subject counts: restored 0 / re-spelled 0 / ignored 0 / added 9 / removed 0; no removals to justify. Compatibility updates retain 31 explicit Reading builders in 17 existing test modules, supplying `modifier: None` and preserving their value comparisons, lexical identities and outcomes.
6. **Timing** (`ouzqotlx`): final bounded census 15.52 seconds wall time under `/usr/bin/time -v`, 6 workers, peak RSS 2,099,752 KiB. Refreshed bounded run: 14:25.93 wall including a 14m 18s toolchain rebuild, 4.443 seconds corpus wall, 6 workers, peak RSS 5,944,848 KiB including compilation (`after-refresh.time`). Second refreshed bounded run: 13:11.53 wall including a 13m 06s toolchain rebuild, 2.703 seconds corpus wall, 6 workers, peak RSS 5,924,592 KiB including compilation (`after-refresh-2.time`).
