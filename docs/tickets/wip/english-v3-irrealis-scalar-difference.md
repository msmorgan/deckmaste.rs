---
needs: []
---
# Read irrealis *were* with a measured comparative: This creature crews Vehicles as though its power were 2 greater.

## Why

The Pilot line fails, and two shortened variants show two gaps. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): key
`at:greater .` 20 sole-blocker faces (17 of them this line; the other 3 are
*with power 3 or greater* after *only to*, and *power or toughness 4 or
greater*). `cargo xtask english-v3 grep 'were \d+ greater'` finds **18**
faces, all unread; 18 failing lines match, **17** of them the only failing
line of their face (13 inside a quoted token ability).

Gap 1: "This creature crews Vehicles as though its power were 2." 0, while
"Its power is 2." 1 and "This creature attacks as though it weren't tapped."
2. Gap 2: "Its power is 2 greater." 0, while "Its power is greater than 2."
1.

## Goal

(1) Irrealis *were* takes the scalar Complements realis *be* takes. (2) A
comparative Adjective with a measure Modifier and no *than* Complement (*2
greater*) reads as a predicative Complement. Together: the witness line.

## Analysis

CGEL Ch. 13 §5.5, p. 1152: "*… as though he were remembering painful
things*": irrealis *were* in the Complement of *as though*. That backs the
irrealis clause already landed; it says nothing about which Complements
*were* takes, and this ticket relies on no further passage. The measured bare comparative is a
project ruling. Glossary: **Irrealis Clause**, **Irrealis Clause
Complement**, **Measure Modifier Use**, **Scalar Location**.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
`IrrealisClause` (`form [subject: NominativePhrase, " ", predicate:
IrrealisPredicate]`), `IrrealisPredicate` built by `SelectedPredicate` with
`IrrealisHeadAgreement` (`require head.form = Irrealis`) and by
`BareAuxiliaryPredicate` with `ParticipialComplement`;
`IrrealisClauseComplementPreposition` (`require
head.IrrealisClauseComplement = Yes`) for *as though*. The realis copular
scalar route is `MeasuredScalarLocation: ScalarLocationComplement`
(`english-v3-copular-scalar-location`). **Measure Modifier Use** is a
comparative head's declared permission to take a measure as degree Modifier
(`english-v3-cost-scalar-complement`).

Orchestrator rulings 2026-10-10:

- IS1. Gap 1 is an instance gap: give `IrrealisPredicate` the scalar-location
  Complement instance the finite copula has. No new Construction; the licence
  is the copula's frame and `form = Irrealis`.
- IS2. Gap 2: refine the comparative AdjP so a head with `MeasureModifierUse`
  takes its measure Modifier when the *than* Complement is absent. The licence
  reads that declared feature. No test on *greater* or on the numeral.
- IS3. One analysis per string. *2 greater* is measure Modifier + comparative
  head. It must not also read as the scalar value *2* followed by a stray
  Adjective, and "Its power is greater than 2." keeps exactly 1. *power 3 or
  greater* (postpositive bounded property) is a different construction and
  keeps its Readings.
- IS4. If Gap 2 needs more than the refinement in IS2 (for instance an
  understood standard of comparison as a feature), STOP after landing Gap 1
  and report: Gap 1 alone covers no face.
- IS5. Newly covered faces are gains. Reading-count increases are possible on
  covered faces with a measure before a comparative (*{1} less*, *2 more*)
  and on covered *as though … were* faces; none is expected, and each is
  reported with its shape.

## Witnesses

Each is the face's only failing line at `ymqwzour`. Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`; 3 = enumeration capped). Re-probe on the claim parent first. All 0.

- Hotshot Mechanic: "This creature crews Vehicles as though its power were 2
  greater." (0)
- Deathless Pilot, Dynamite Diver, Cloudspire Captain: "This creature saddles
  Mounts and crews Vehicles as though its power were 2 greater." (not probed
  separately; "This creature saddles Mounts and crews Vehicles." 1)
- Prodigy's Prototype: "… create a 1/1 colorless Pilot creature token with
  "This token crews Vehicles as though its power were 2 greater."" (quoted
  form; not probed)

Also 0 today: "This creature can crew Vehicles as though its power were 2
greater."; "Its power is 2 greater than its toughness." (unattested;
localisation only). Controls that must not change: "This creature crews
Vehicles." 1; "Its power is 2." 1; "Its power is greater than 2." 1; "You may
cast it as though it had flash." 2; "This creature attacks as though it
weren't tapped." 2; "Destroy target creature with power 4 or greater." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bwere\b|\bweren't\b` or `\b(greater|less|more|fewer)\b` case-insensitively
plus 300 random covered controls (`cargo xtask english-v3 diff --manifest …
--random 300 --seed 20261010`), `--workers 6 --samples-per-face 0`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost;
IS5 for increases. Full `cargo test -p deckmaste_english_v3`
and `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
once, at the end. A rebuild after a grammar edit takes 2–3 minutes and the six test binaries
about 75 seconds, so batch edits.
Tests assert the role (*were* as irrealis copula with a scalar Complement, the measure as
Modifier of the comparative head, the IS3 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *can be attached only to a creature with power 3 or greater* (Gate
  Smasher, O-Naginata, Konda's Banner): "This Equipment can be attached only
  to a creature." 0, a focused PP Complement of a passive.
- *with power or toughness 4 or greater* (Repel Calamity): coordinated
  property heads.
- Semantics of crewing with modified power.

## Landing record

1. **Census delta** (`ouykkpns` before edits, equivalent to claim parent
   `zyovmtwr`, → `ouykkpns` verified implementation): bounded 4,190 faces;
   covered **3,578 → 3,596**, total Readings **203,239 → 203,495**.
   Official `english-v3 diff --lines`: lost **0**, gained **18**, decreases **0**,
   increases **0**, digest-only **625**. The digest-only changes make the
   existing scalar-predicate and measured-cost schema Categories explicit;
   their Reading counts are unchanged. Complete retained-Reading inspection
   classifies all 625 digest-only faces and all **639** changed top-level lines:
   **544** measured-cost-only faces / **558** lines (`MeasuredScalarLocation`
   at `ScalarLocationComplement`); **80** finite-scalar-only faces / **80**
   lines (`ScalarLocationPredicate` at its original Category); **1** mixed
   face / **1** line, The End's *This spell costs {2} less to cast if your life
   total is 5 or less.* Both schemas occur on that line. These are disjoint
   face and line classes, not Reading-occurrence counts; no unclassified line.
   The parent and implementation retained-Reading inspections agree on every
   class membership. Evidence: `digest-classes.json`, `digest-class-lines.json`,
   `digest-parent-confirmation.json`, `reach-parent.json`, `reach-current.json`.
   Lost faces: none. No gained face reads
   only through an unintended analysis. Selector: every supported face matching
   `\bwere\b|\bweren.t\b|\b(greater|less|more|fewer)\b`, case-insensitive
   (3,914 faces), plus 300 covered controls chosen by the official diff manifest
   writer with seed **20261008**; 24 overlap. Evidence and identity manifest:
   `target/english-v3/english-v3-irrealis-scalar-difference/` (`before.json`,
   `after.json`, `diff.json`, `manifest.json`).
   Post-refresh census on empty `mnonzzlw`, child of `ouykkpns`, preserves all
   18 ticket gains and adds eight trunk gains: covered **3,578 → 3,604**,
   Readings **203,239 → 203,714**; lost **0**, gained **26**, decreases **0**,
   increases **2**, digest-only **625**. The official pre-refresh → refreshed
   diff isolates eight gains and two increases, with no other changes. The
   increases belong to `xswzoyrx` (predeterminer-all), permitted by its PA7
   ruling: Author of Shadows **24 → 48**, adding outer `[all [opponents'
   graveyards]]` beside `[[all opponents]' graveyards]`; Werewolf of Ancient
   Hunger **9 → 16**, adding `[all [players' hands]]` beside `[[all players]'
   hands]` (its affected line doubles **7 → 14**, other lines unchanged).
   The eight trunk gains are Collective Defiance and Demonic Covenant
   (`xswzoyrx`'s *all* predeterminer), and Disorienting Choice, Loamcrafter
   Faun, Nyssa of Traken, Seasoned Cryomancer, Swashbuckler Extraordinaire
   and The Earth King (`xzwlutqp`'s *up to that many*). The supported-corpus
   grep evidence is `trunk-gain-attestation.json`. No lost Reading to restore or justify. Evidence:
   `after-refresh.json`, `diff-refresh.json`, `diff-trunk.json`.
   Review reach extension (2026-10-11): the original finite-scalar instance
   has **46** supported claim-parent users outside the 4,190-face selector,
   each with one retained finite-scalar line. Enumerating every top-level
   Ability Reading of the copula-surface candidates identified all users;
   the claim-parent and implementation user identity sets are equal. The
   loaded lexicon has only `Be`, `BeNegative` and `BeContracted` owners of
   `ScalarLocation` frames; their declared finite surfaces are exhausted by
   supported-corpus grep `\b(is|are|was|were|am|isn.t|aren.t|wasn.t|weren.t)\b|[’\x27](s|re|m)\b`,
   case-insensitive (**8,353** faces, **6,739** outside the original selector).
   No raw Scryfall search was used. The classifier additionally inspects the
   original digest-only faces, for 8,769 inspected candidate faces in all.
   Official `english-v3 diff --lines` on the 46-face identity manifest:
   covered **46 → 46**, Readings **800 → 800**, lost/gained/decreases/increases
   **0/0/0/0**, digest-only **46** faces / **46** lines, issues **0**.
   Both censuses use `--identity-manifest …/reach-manifest.json --workers 6
   --samples-per-face 0` under `/usr/bin/time -v`, with outputs
   `reach-before.json` and `reach-after.json`; the official diff outputs
   `reach-diff.json`. BEFORE temporarily loads the exact refreshed
   `zyovmtwr` grammar and core lexicon, isolating this ticket from inherited
   trunk changes; both report headers retain the workspace change `ltyonqkl`.
   Verified implementation files were restored byte-for-byte before AFTER.
   Classifier/selector evidence stays in `target/`: `copula-reach.json`,
   `reach-input.json`, `outside-parent-users.json`, `outside-after-users.json`,
   `frame-owners.txt`, `frame-owners-after.txt`, and `reach-manifest.json`.
   Authorized integration refresh (2026-10-11), original **4,190** identities
   only: covered **3,578 → 3,615**, Readings **203,239 → 204,124**;
   lost **0**, gained **37** (18 own, 19 inherited), decreases **0**,
   increases **2** (the same `xswzoyrx` PA7 shapes above), digest-only **942**,
   issues **0**. Official reviewed-head → final diff isolates **11** incoming
   gains and **409** digest-only changes, with zero losses/decreases/increases;
   all 18 ticket gains retain their counts. The incoming work is plugin terms
   `ztvzruvp`, keyword choices `zspvqono`, coordinated premodifiers `ptokxtuo`,
   and mana-symbol nominals `qtyxwuyo`. Evidence: `after-integration-refresh.json`,
   `diff-integration-refresh.json`, `diff-integration-incoming.json` and
   `integration-incoming.stat`.
2. **Witnesses** (complete Sentence enumeration on `ouykkpns`): all ticket
   witnesses failed on the claim-parent grammar; none was dropped. Hotshot
   Mechanic **0 → 1**; Deathless Pilot, Dynamite Diver and Cloudspire Captain's
   shared saddles/crews line **0 → 2** each; Prodigy's Prototype's whole trigger
   sentence **0 → 5**, with **1** Reading of the quoted Pilot sentence.
   W9-A controls: the saddles/crews and modal-crew sentences with the existing
   *as though it had flash* clause each have **2**; Prodigy's Prototype with
   the quoted basic crew sentence has **5**. Those independent attachments and
   outer ambiguities survive. Modal crew witness **0 → 2**; *were 2* localization
   **0 → 1**; *Its power is 2 greater.* **0 → 1**. The six stated controls retain
   **1, 1, 1, 2, 2, 2**, respectively. The unattested explicit-standard
   localization *Its power is 2 greater than its toughness.* stays **0 → 0**;
   IS2 covers the bare comparative.
3. **Spot checks** (`ouykkpns`): Hotshot Mechanic, Deathless Pilot, Dynamite
   Diver and Cloudspire Captain select an irrealis copula with a measured
   adjectival predicative Complement in their direct crew/saddle clause.
   Prodigy's Prototype, Shorikai, Genesis Engine, Born to Drive, Back on Track,
   Reckoner Bankbuster and Roadside Assistance select that same analysis inside
   the quoted token ability. All ten retain numeral *2* as the Modifier of
   adjective *greater*, with the declared empty comparative frame. Each
   inspected Reading realizes the full face exactly and has zero issues.
4. **Deviations and STOPs**: no new named Construction or lexical frame;
   both existing Constructions became shared schemas with one additional
   instance each. IS1 retains the existing scalar Subject restriction using
   Scalar Denotation. IS2's measured adjective fills the predicative Complement
   slot directly in the shared schema, preserving its Adjective head and
   Measure Modifier. This fixed a chart-budget regression found by the full
   suite in `all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets`:
   its final fixture uses **11,541** items on `ouykkpns`, versus **11,536** on
   `zyovmtwr`, under the unchanged **12,000** limit. No test was weakened.
   Seed 20261008 follows the binding session instruction rather than the Method's
   older seed. No superseded lexical or grammar route remains; existing live
   cost and bounded-property routes are retained. No STOP or glossary gap.
   Changed-declaration reach inventory (supported faces outside the original
   selector; unchanged form/requirements/exports are stated explicitly):

   - `IrrealisPredicate` Category adds `ScalarDenotation` to `number, person`.
     Outside reach **no**: its existing and new instances require an irrealis
     copula, *were* or *weren't*, both exhaustively selected.
   - `IrrealisHeadAgreement` adds export `ScalarDenotation = No`; its form,
     finiteness, no-clitic, number and person equations are unchanged. This
     changes exports of the existing `SelectedPredicate<IrrealisPredicate,
     IrrealisHeadAgreement>` and `ParticipialAuxiliaryPredicate<IrrealisPredicate,
     ParticipialComplement, IrrealisHeadAgreement>` instances; their forms and
     other requirements/exports are unchanged. Outside reach **no** for the
     policy and both affected instances (lexical *were*/*weren't*). Their
     finite and secondary instances are unchanged.
   - `IrrealisClause` adds the `scalar_subject_licence` requirement. Its Subject,
     Predicate, number and person equations are unchanged. The new table admits
     any Subject for predicate `ScalarDenotation = No`, and only a scalar
     Subject for `Yes`. Outside reach **no** for clause/table: an irrealis
     predicate still requires a selected *were*/*weren't* surface.
   - `ScalarLocationPredicate` becomes a shared schema with the same lexical
     Verb and `selected_frame(head, ScalarLocation)` form. Its entire instance
     list is `(ScalarLocationPredicate, FiniteHeadAgreement)` and
     `(IrrealisPredicate, IrrealisScalarHeadAgreement)`. The existing finite
     instance keeps all original requirements/exports and gains an explicit
     Reading Category field; `FiniteHeadAgreement` itself is unchanged.
     Outside reach **yes** for the shared schema's finite instance: the
     46 users measured in item 1. Outside reach **no** for the new irrealis
     instance. New `IrrealisScalarHeadAgreement` requires Irrealis, Finite,
     no clitic, exports head number/person and `ScalarDenotation = Yes`;
     outside reach **no** (*were*/*weren't*).
   - `MeasuredScalarLocation` becomes a shared schema: its Modifier and
     lexical Head are instance parameters, and `head.MeasureModifierUse = Yes`
     is retained. Its entire instance list is `(ScalarLocationComplement,
     CostSymbols, Determinative, NoFeatures)` and `(PredicativeComplement,
     MeasurePhrase, Adjective, MeasuredComparativeAdjectiveProperties)`.
     The original cost instance's specialized form and requirements/exports
     are unchanged; its Reading gains the explicit Category field. Outside
     reach **no** for either instance/schema: the cost heads are *less/more*,
     while the licensed Adjective head is *greater*, all selected.
   - New `MeasuredComparativeAdjectiveProperties` requires Scalar Measure Kind
     and an Intransitive (empty) head frame, exports Adjectival Predicative Kind
     and `PredicativeAdjunctUse = No`. Outside reach **no**, by its sole
     measured-adjective instance. The comparative AdjP's measure-Modifier form
     is that instance's `[MeasurePhrase, " ", lexical(Adjective)]`;
     `ComparativeAdjectivePhrase` itself, its instances/policies, and the
     existing `MeasurePhrase`/`ScalarMeasurePhrase` declarations are unchanged.
   - The existing `vocab:ScalarDegree/Greater` lexeme adds only
     `MeasureModifierUse = Yes`; its forms, Category and selected frames are
     unchanged. Outside reach **no**: all supported *greater* faces are in
     the selector. No new loader or parser condition was added.

5. **Tests** (`ouykkpns`): full `cargo test -p deckmaste_english_v3` passed
   (1,053 tests), following the initial chart-budget failure and its grammar
   fix; full `cargo test -p deckmaste_lexical_source` passed.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored **0** / re-spelled **4** / ignored **0** / added **5** / removed **0**.
   Review correction: the original tally of three counted edited value sites;
   one is a shared builder used by two tests, so four existing tests are
   affected. Every original assertion and input remains unchanged:

   - `copular_scalar_location.rs::independent_scalar_location_preserves_both_laws_and_traversal_identity`:
     `scalar_location` adds only `category: Category::ScalarLocationPredicate`
     to the expected predicate; the same *its power is 2*, *her power is 1 or
     less*, and *three* values still compare complete parsed sets, realization,
     nodes and Words with the independently constructed values.
   - `cost_scalar_complement.rs::independent_complements_preserve_both_laws_and_traversal_identity`:
     the expected measured value adds only `category: Category::ScalarLocationComplement`;
     *cost {1} less*, *{1} less*, *{0}*, *to cast*, and *costs {1} less to cast*
     retain the same full-set comparisons, including the existing participial
     alternative, and both roundtrip/traversal outcomes.
   - `cost_scalar_complement.rs::ordinary_infinitivals_and_unlicensed_measure_heads_are_rejected`:
     the independently constructed invalid `{1} greater` value adds only
     `category: Category::ScalarLocationComplement`; every original rejected
     fragment still compares its complete parsed set with the empty set, and
     the same invalid values still fail admission and realization.
   - `copular_scalar_location.rs::independent_non_scalar_subject_and_wrong_frame_are_rejected`:
     the shared `scalar_location` builder adds the same predicate Category to
     its two existing invalid values; the wrong Subject and wrong frame, and
     their admission/realization rejection assertions, are unchanged. This
     indirect fourth consumer was omitted from the original tally.

   No value comparison was replaced by a variant/discriminant check; no
   assertion, sentence, alternative, test or blocker was removed. No removal
   to justify.
   Citation checks: zero noncompliant, zero stale; the piped diff audit found
   zero new citation sites, so no new rule registration was needed.
   Post-refresh `cargo build -p xtask` passed. Full `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` passed again for
   `deckmaste_english_v3` (1,067 tests), `deckmaste_lexical_source` (88),
   `xtask` (425), `deckmaste_semantics_v2` (106),
   `deckmaste_construction_core` (474), and `macro_ron` (149).
   The only ignored test is xtask's pre-existing
   `macros::templates::tests::macro_schema_census_count_matches_21`, an
   on-demand live-corpus cross-check; this landing adds no ignore.
   Refresh's sole conflict was three appended `tests/main.rs` module
   registrations; all three were retained exactly once. The successful
   refresh imports `xswzoyrx`, `xzwlutqp`, facts patient-role changes, and
   semantics-v2 counter/macro changes. Logs are `test-refresh-<crate>.log`
   and `clippy-refresh-<crate>.log` in the evidence directory.
   `cargo xtask lean check` passed after the plugin refresh: **245/245**
   cards (237 canon, eight testing); log `lean-refresh.log`. Final citation
   checks again found **0** noncompliant and **0** stale (17,799 sites);
   the implementation's piped `jj diff --git` audit selected zero new sites.
   Final scoped integration checks passed: `cargo build -p xtask`;
   full `cargo test -p deckmaste_english_v3` (**1,084** passed) and
   `cargo test -p deckmaste_lexical_source` (**88** passed); both
   `cargo clippy -p <crate> --tests -- -D warnings` commands passed.
   The named selected-complement-clusters budget test passed with inherited
   CP7 ceilings unchanged. One inherited English ignore names
   `english-v3-mana-symbol-verbless-subject` as its exact blocker:
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`.
   This landing adds no ignore. Integration refresh's registration conflict
   retained both `mana_symbol_nominal` and `irrealis_scalar_difference` once.
   Per the orchestrator's scoped rule, no plugin/semantics checks were repeated:
   this change touches only English grammar/tests and core lexical data.
6. **Timing** (`ouykkpns`): final complete bounded census wall **3 min 28 s**,
   **6 workers**, `--samples-per-face 0`, captured under `/usr/bin/time -v`.
   Refreshed bounded census wall **4 min 40 s**, same manifest and settings;
   `/usr/bin/time -v` captured in `census-refresh.log`.
   Review reach extension: BEFORE **582 ms**, AFTER **499 ms** corpus wall,
   six workers, zero samples; build overhead is separately captured in the
   `/usr/bin/time -v` logs. Final original-manifest census wall **87 s**,
   six workers, zero samples, under `/usr/bin/time -v` in
   `integration-census.log`. No full-corpus census was run.
