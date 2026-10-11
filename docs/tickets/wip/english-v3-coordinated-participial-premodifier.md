---
needs: []
---
# Coordinate an Adjective with a participle before a noun: create a tapped and attacking token that's a copy of it.

## Why

*tapped and attacking* reads as a Predicate ("This token enters tapped and
attacking." 1; "Create a token that's tapped and attacking." 1) and each word
reads alone as a premodifier ("Create a tapped 1/1 red Devil creature token."
1; "Create an attacking 1/1 red Devil creature token." 1). The coordination
does not read before a noun. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.json`): key `at:and attacking` 13
sole-blocker faces. `cargo xtask english-v3 grep '(a|an|two|three|X) tapped
and attacking'` finds **17** faces, all unread; 17 failing lines match,
**15** of them the only failing line of their face. *tapped and attacking*
overall is on 115 faces, 80 covered (the predicative use).

## Goal

A syndetic coordination of a state Adjective and a gerund-participle reads as
one attributive Modifier of a Nominal, before the rest of the premodifiers
(*a tapped and attacking 1/1 red Devil creature token*).

## Analysis

CGEL Ch. 15 §3.2(d), p. 1328, "Modifier in NP structure": "Mixed
coordinations in pre-head (attributive) modifier position, however, are not
common"; they "tend to involve adjectives and nominals belonging within a
single semantic set", [17iib] "*the state and federal laws*". That backs
unlike-category attributive coordination as possible and restricted. Admitting
Adjective + participle for the combat-state set is a project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`PremodifierCoordination` (category and feature), policies
`NounPremodifierConcord` and `AdjectiveStructureMerge` (`export
PremodifierCoordination = Yes`, `AdjectiveOrigin =
coordinated_adjective_origin(…)`), series `AdjectivePhraseSeries` and
`VerbalPremodifierSeries`. Like-category pairs read: "Destroy target
attacking or blocking creature." 1; "Tap target white or blue creature." 1.
The mixed pair fails in every host: "Destroy target tapped or blocking
creature." 0 (Tetsuo Umezawa, attested); "Tapped and attacking creatures get
+1/+1." 0.

Orchestrator rulings 2026-10-10:

- CP1. Refine the premodifier coordination so a coordinate from
  `AdjectivePhraseSeries` and one from `VerbalPremodifierSeries` may combine;
  no new Construction beside it. `coordinated_adjective_origin` (or its
  verbal counterpart) gets the mixed row.
- CP2. The licence reads the coordinates' declared premodifier-use features.
  No test on *tapped*, *attacking* or *token*. Do not add a "combat state"
  feature whose only job is to block other mixed pairs.
- CP3. One analysis per string. *a tapped and attacking token*: one
  coordinated Modifier. It must not also read as NP coordination (*a tapped*
  + *attacking token*), and *tapped* must not double as participle and
  Adjective inside the coordination: whichever category the bare premodifier
  *tapped* has today is the one the coordinate has.
- CP4. *or* (*target tapped or blocking creature*, *target attacking,
  blocking, or tapped creature*: Tetsuo Umezawa, Sonar Strike) is the same
  coordination and in scope; report the faces.
- CP5. Newly covered faces are gains. Reading-count increases are possible on
  covered faces where an Adjective and a participle premodifier are joined
  before a noun and currently read some other way; report each with its
  shape. The 80 covered predicative *tapped and attacking* faces must not
  change.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Furnace-Blessed Conqueror: "Whenever this creature attacks, create a tapped
  and attacking token that's a copy of it." (0; *a tapped token that's a copy
  of it* 3)
- Pugnacious Pugilist: "Whenever this creature attacks, create a tapped and
  attacking 1/1 red Devil creature token with "When this token dies, it deals
  1 damage to any target."" (0)
- Krothuss, Lord of the Deep: "Create a tapped and attacking token that's a
  copy of another target attacking creature." (0)
- Tetsuo Umezawa: "{U}{B}{B}{R}, {T}: Destroy target tapped or blocking
  creature." (sentence 0)

Controls that must not change: "This token enters tapped and attacking." 1;
"It's tapped and attacking." 1; "Create a 1/1 red Devil creature token that's
tapped and attacking." 3; "Create a tapped 1/1 red Devil creature token." 1;
"Destroy target attacking or blocking creature." 1; "Tap target white or blue
creature." 1; "Tap all attacking and blocking creatures." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\b(tapped|untapped|attacking|blocking|blocked|enchanted|equipped) (and|or) [a-z]+` or
`[a-z]+ (and|or) (tapped|untapped|attacking|blocking|blocked)\b` (coordinations with a state word)
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
CP5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (the coordination as a single attributive Modifier of the Nominal,
each coordinate's category, the CP3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *create a token that's a copy of … and that's tapped and attacking*
  (Flamerush Rider, Gyrus): coordinated relatives.
- *a copy of Shredder tapped and attacking that player* (postpositive with an
  Object).
- *put … onto the battlefield tapped and attacking* on faces with another
  blocker (16 touched faces).

## Landing record

1. **Census delta** (`ptokxtuo` before/after implementation; before tree equals
   claim parent `ywzorprm`): bounded 610 faces, covered 542 → 559, total Readings
   10,302 → 31,446; 17 gains, no lost faces, decreases or increases on covered
   faces. 116 digest-only changes encode the newly explicit, previously inferred
   `VerbalPremodifier` Category after its promotion to a shared schema; no
   grammatical analysis is retired. The selector is the Method pattern, with
   comma-list `attacking, blocking, or tapped` included, plus 300 covered controls
   from the supplied baseline, seed 20261008 (312 surface faces; two random
   overlaps). Manifest and before/after/diff evidence are under
   `target/english-v3/english-v3-coordinated-participial-premodifier/`.
   Pre-refresh review reach (`ywzorprm` tree via empty `wqokolrq`, versus `ptokxtuo`
   tree via empty `rzszunwr`): 4,375 faces, covered 4,042 → 4,059,
   Readings 207,140 → 228,284; the same 17 gains, 0 lost/decreases/increases,
   1,309 digest-only faces and 3,049 unchanged. `english-v3 diff --lines`
   identifies 1,359 digest-only lines; claim-parent Reading inspection attests a retained
   `VerbalPremodifier` in every one (explicit Category encoding only).
   No node-level selector is available. `reach-manifest.json` unions the
   requested fallback grep (2,771 faces), supplied BEFORE-baseline attestations
   of `VerbalPremodifier`/noun/adjective members (1,366 faces), and the original
   610-face manifest, including the same 300 covered controls, seed 20261008.
   Fallback regex:
   `(?i)\b[a-z]+(ed|ing|en)? (and|or) [a-z]+ (creature|permanent|card|token|spell|land|artifact|enchantment|player)s?\b|, [a-z]+,? (and|or) [a-z]+ (creature|permanent|card|token|spell)`.
   `reach-before.json`, `reach-after.json`, `reach-diff.json`/`.log` and
   `reach-line-attestation-batch-parallel.json` hold the wider evidence in the same directory.
   Refresh census (`wmkwztxy` after importing trunk): the same 4,375 faces,
   covered 4,042 → 4,062 and Readings 207,140 → 228,327 against the saved
   claim-parent census; 20 gains, 0 lost/decreases/increases, 1,309 digest-only.
   Against the reviewed after-census, official `diff --lines` reports only
   three gains: Loamcrafter Faun 1, Nyssa of Traken 5, The Earth King 37,
   attributable to `xzwlutqp` (up-to-that-many); all other 4,372 faces retain
   counts and digests. The original 17 gains and 1,359 digest-only lines are
   unchanged. Evidence: `reach-after-refresh.json`, `reach-refresh-diff.json`
   and `reach-refresh-incoming-diff.json` in the same directory.
   The claim parent has 84 covered *tapped and attacking* faces: all retain their
   counts, with encoding-only digest changes on Hans Eriksson, Kinscaer Sentry,
   Najeela, the Blade-Blossom, Thousand-Faced Shadow, Warren Warleader, and
   Zareth San, the Trickster. No wrong-only gains found.
2. **Witnesses** (`ptokxtuo`, complete probe enumeration): Furnace-Blessed
   Conqueror 0 → 5 (tapped-only control 5); Pugnacious Pugilist 0 → 6 (control 6);
   Krothuss, Lord of the Deep 0 → 5 (control 5); Tetsuo Umezawa 0 → 1 (control 1,
   Document category because the cost-prefixed control also has zero Sentence
   Readings). Sonar Strike's serial target constituent reads through one route.
   All witnesses were probed before implementation. The seven ticket controls
   retain counts 1, 1, 5, 1, 1, 1, 1; the ticket's reported 3 was an enumeration
   cap. W9-A preserves the independent copy/relative and quoted-ability ambiguity.
   Review ambiguity measurements (`ptokxtuo`): the top five gained faces are
   Altaïr Ibn-La'Ahad 15,751; Shaun, Father of Synths 4,759; Loki, the Deceiver
   231; Ghalta and Mavren 85; Sauron, the Necromancer 76. Schema-v8 face
   counts sum line counts. Altaïr = keyword line 1 + paragraph line
   (75 × 210 × 1) = 15,751; its three sentence probes give 75, 210, 1.
   The 210-reading sentence's tapped-only control also gives 210. Shaun =
   first line 4,757 + second line 2; the first line's tapped-only control
   also gives 4,757. Coordination contributes a factor of 1 in both controls.
   Isolated NounPhrase probes (CLI `--category noun-phrase`, complete):
   Furnace's `a tapped and attacking token` / `a tapped token` /
   `an attacking token`: 1 / 1 / 1; Pugilist's same heads with
   `1/1 red Devil creature token`: 1 / 1 / 1; Krothuss's same heads with
   `that's a copy of another target attacking creature`: 5 / 5 / 5;
   Tetsuo's `target tapped or blocking creature` / `target tapped creature` /
   `target blocking creature`: 1 / 1 / 1. Every coordinated Reading has one
   shared Modifier, adjectival tapped, and no verbal-tap or NP-coordination
   twin. Krothuss's five copy-relative/attachment Readings are present in
   both single-premodifier controls (W9-A). Evidence: `np-*.json`,
   `altair-sentence-*.json`, `altair-line-control.json`, `shaun-line-control.json`.
3. **Spot checks** (`ptokxtuo`): Furnace-Blessed Conqueror, Pugnacious Pugilist,
   Krothuss, Lord of the Deep, Maestros Diabolist, Phantom Steed, Redoubled
   Stormsinger, Satya, Aetherflux Genius, and Sauron, the Necromancer each use
   one *tapped and attacking* Modifier of their token Nominal, with Adjective
   *tapped* and verbal *attacking*. Tetsuo Umezawa uses the same mixed Modifier
   with *or*; Sonar Strike uses a flat *attacking, blocking, or tapped* Modifier.
   Full-face probe samples realize exactly and have no validation issues.
4. **Deviations and STOPs** (`ptokxtuo`): no new named Construction, lexical
   frame, lexeme, loader change or glossary gap; no STOP. Shared
   `VerbalPremodifier` schema instances supply its existing standalone Category
   and its new coordination-member projection, retaining the declared participial
   licence. Grammatical category and syndetic-tail summaries keep like-category
   routes and the existing asyndetic route distinct. No existing route is
   superseded or lexical analysis narrowed. Added seven role/roundtrip tests and
   re-spelled existing exact values for the schema's explicit Category field.
   The selector additionally includes Sonar Strike's comma-separated surface;
   seed 20261008 follows the binding session budget rather than Method's older
   seed. CP5's protected analyses are unchanged; the encoding changes are
   disclosed in item 1.
   Review reach inventory (forms unchanged unless stated): existing
   `NounPremodifierMember` and `AdjectivePremodifierMember` export the new
   category/tail summaries; `PremodifierConcord` adds category/tail requirements
   and exports, affecting existing `SerialCoordination: PremodifierCoordination`
   and `CoordinationSeriesContinuation: PremodifierCoordination`; the latter
   additionally uses `AsyndeticPremodifierTail`. Existing standalone
   `VerbalPremodifier` becomes a shared schema with the same form/licence and
   `VerbalPremodifier: VerbalPremodifier` instance: its formerly inferred Category
   is explicit in the value. `Premodifier` and `PremodifierCoordination` add
   category/tail features. Newly added instances are
   `VerbalPremodifier: Premodifier`, `Coordination: PremodifierCoordination`,
   `CoordinationSeriesEnd: Premodifier`, and
   `CoordinationSeriesContinuation: Premodifier`, with new
   `SyndeticPremodifierConcord`/`AsyndeticPremodifierTail` policies. No other
   existing declaration changes form, requirements or exports.
   **Refresh STOP, unresolved:** the full English suite fails
   `tests/suite/selected_complement_clusters.rs::all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets`
   at its chart-item ceiling (12,017 > 12,000). It passed before refresh.
   The failing final fixture is the two-line artifact damage/gain document;
   the direct probe gives exactly one Reading, complete enumeration and zero
   issues, with 12,017 items and 12,332 families. Exact value comparison in
   that fixture is blocked by the earlier item guard. All three requested
   review measurements pass. The review instruction prohibits grammar changes
   unless one fails; neither grammar nor the test guard has been changed.
   Evidence: `tests-refresh.log`, `refresh-performance-probe.json`.
   **CP6 measurement, STOP (orchestrator ruling, 2026-10-11):** a fresh
   `cargo build -p xtask` on the refreshed claim parent `ywzorprm`, without this
   landing, supplies the baseline. The default workspace's available binary
   predates its current source; its preliminary counts are excluded. All nine
   same unscoped Document witnesses were probed with `--packed --readings 2
   --json` on the refreshed parent and this landing. Chart items, parent →
   landing (delta): Brothers of Fire 7,732 → 7,815 (+83); Fireslinger 7,690 →
   7,773 (+83); Goblin Artillery 7,690 → 7,773 (+83); Orcish Artillery 7,690 →
   7,773 (+83); Orcish Cannoneers 7,690 → 7,773 (+83); Psionic Entity 7,684 →
   7,767 (+83); Reckless Embermage 7,705 → 7,788 (+83); Forge Devil 9,089 →
   9,184 (+95); the final two-line artifact damage/gain fixture 11,895 →
   12,017 (+122). Every witness grew; all eighteen probes have exactly one
   Reading, complete enumeration and zero issues. The maximum delta is 122,
   exceeding CP6's 1% of the 12,000 budget (120 items) by two items. The parent
   itself is below the ceiling. CP6 therefore requires STOP: no budget change,
   grammar change or hygiene-ticket append; attribution sufficient to permit
   re-budgeting has not been established. The exact-Reading-set assertions
   remain untouched. Evidence: `cp6-nine-measurements.json`, the eighteen
   `cp6-*-refreshed-parent.json` / `cp6-*-feature.json` packed probes, and
   `cp6-parent-build.log`. The workspace is restored to its landing tree;
   cached xtask currently contains the diagnostic parent build. No refresh or
   integration was run for CP6. Re-spelled remains six, with no budget
   re-spelling. Full suite/clippy were not repeated after this measurement:
   no code changed and CP6's excessive-delta branch requires STOP.

5. **Tests** (`ptokxtuo`): `cargo test -p deckmaste_english_v3` passed all 1,064
   tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   Restored 0 / re-spelled 6 / ignored 0 / added 7 / removed 0. Re-spelled:
   `tests/suite/grammar.rs::independent_participial_premodifier_preserves_verb_form_and_distribution`
   adds Category to the defending-player expected modifier and invalid-frame
   value; exact parse/realization and frame rejection are unchanged.
   `tests/suite/participial_uses.rs::authentic_attacking_and_blocking_modifiers_retain_verbal_identity`
   adds Category to the attacking/blocking expected modifier; the same authentic
   sentences retain their exact verbal values.
   `tests/suite/participial_uses.rs::righteous_blow_keeps_coordinated_verbal_modifiers_under_the_nominal`
   adds Category to both attacking/blocking expected coordinates; the same
   Righteous Blow sentence retains its exact shared-head value.
   `tests/suite/past_participial_premodifiers.rs::independently_built_attested_modifiers_preserve_passive_form_and_both_laws`
   adds Category to expected exiled/revealed/triggered/discarded/milled
   modifiers and the invalid-frame value; exact values, both laws and rejection
   are unchanged on the same surfaces.
   `tests/suite/preposition_function_licences.rs::of_shapes_keep_their_nominal_and_predicative_functions`
   adds Category in the shared chosen-type expected value; the same PP and
   predicative sentences retain exact expected Reading sets.
   `tests/suite/targeting_projection.rs::adjective_and_participial_premodifiers_stay_below_targeting`
   adds Category to expected attacking-creature modifiers; the same singular
   and plural surfaces retain exact values and the reversed-order rejection.
   All six retain value comparisons; no discriminant/matches substitution.
   Citation checks: 0 noncompliant, 0 stale;
   diff audit selected no new citation sites. No other crate code/data or plugin
   definitions changed in this feature. Refresh imports code/grammar/plugins:
   `cargo build -p xtask` passed. Full
   `cargo test -p deckmaste_english_v3 -p xtask -p deckmaste_semantics_v2 -p deckmaste_construction_core -p macro_ron`
   failed only at the English chart guard: 1,068 English tests pass, one fails;
   all six English test binaries ran. To complete suites aborted by that failure,
   `cargo test -p xtask -p deckmaste_semantics_v2 -p macro_ron` passed and
   `cargo test -p deckmaste_construction_core` passed, including doctests.
   `cargo clippy -p deckmaste_english_v3 -p xtask -p deckmaste_semantics_v2 -p deckmaste_construction_core -p macro_ron --tests -- -D warnings`
   passed. xtask retains its existing ignored
   `macros::templates::tests::macro_schema_census_count_matches_21` (live corpus
   cross-check, run on demand); no new ignored or removed tests.
   `cargo xtask lean check` first failed because concurrent xtask tests removed
   generated Lean files; its serialized retry after all Rust suites passed
   (all 245 cards prove `Card.check = []`).
   Decimal timing text triggered two citation-looking false positives; spelling
   seconds/milliseconds separately fixes those. Final citation checks pass with
   0 noncompliant and 0 stale; no citation was added.
   Refresh logs are under the evidence directory (`tests-refresh*.log`,
   `clippy-refresh.log`, `lean-refresh*.log`).
6. **Timing** (`ptokxtuo`): bounded after-census 44.84 seconds wall time,
   six workers, zero samples per face, under `/usr/bin/time -v`; no full-corpus
   run. Wider review census: 123 seconds 850 milliseconds before and 128 seconds 920 milliseconds after,
   six workers, zero samples, both under `/usr/bin/time -v`. Review changes only
   this record; grammar and test code remain `ptokxtuo`, so the prior full suite
   and clippy results apply to pre-refresh code. Post-refresh bounded census
   (`wmkwztxy`): 238 seconds 730 milliseconds wall, six workers and zero
   samples, under `/usr/bin/time -v`. No full-corpus run.
