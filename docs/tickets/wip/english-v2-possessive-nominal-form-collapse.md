---
needs: []
---
# Collapse the possessive and genitive `nominal_form` splits

**R10 — Group R.** Authority: rewrite ADR "Plan 08 unified-membership
clarification (2026-09-04)" (the re-spell is the unified shape's replacement for
a deleted Category) read together with "Amendment: attachment class is a declared
linguistic property (2026-09-04)" (the value list still has to answer to
English).

Defect. One linguistic frame — possessive determiner plus nominal — is seven
constructions. `possessed_singular_reference`, `possessed_plural_reference` and
`possessed_mass_reference`
(`crates/deckmaste_english_v2/src/constructions.rs:3186, 3201, 3216`) have
**identical forms** (`lex(possessor) nominal`) and identical derive lists,
differing only in a `require nominal.nominal_form in [...]`. The four
`genitive_determiner_*` constructions (`:3230, 3245, 3260, 3274`) are the same
frame split four ways.

`NominalForm` has seven values
(`crates/deckmaste_construction_core/src/feature.rs`: `BareSingularNoun,
ModifiedSingularNoun, SingularCoordination, BarePluralNoun, ModifiedPluralNoun,
PluralCoordination, MassNoun`). Eleven `nominal_form in [...]` requires each
select exactly two of the seven, and the partition is not exhaustive:

- 2026-09-04 — coordinator ruling on this landing's STOP: the possessive
  coordination arm is **struck from this ticket** and deferred to
  `english-v2-underspecified-adjunct-attachment`, which owns the general
  bracketing-preference/attachment device the arm needs (R1
  `english-v2-attachment-class-declared` and R7
  `english-v2-locative-coordination-arms` are parked on the same device).
  `your creature and artifact` still has no derivation.
  was: "the `possessed_*` family has **no coordination arm at all** — `your
  creature and artifact` has no derivation;"
- `genitive_determiner_coordination_reference` requires
  `coordination.number is Singular`, so `target player's creatures and
  artifacts` has none either.

The recorded justification for the excluded values is provenance about a deleted
implementation artifact ("`SingularNominal` produced `BareSingularNoun` and
`ModifiedSingularNoun`… all thirteen former narrow-Category roles carry exactly
those lists"), not an English fact. The one English-facing datum is negative and
site-specific: widening to admit coordination broke `Aquatic Alchemist // Bubble
Up` (`your first [instant or sorcery] spell` became `your [first instant] or
[sorcery spell]`) in the `english-v2-number-feature-unification` landing —
evidence about one role, generalized to thirteen.

Pinned shape. One construction per frame, with the determinative's declared
`nominal_license` doing the licensing through the existing
`determiner_licenses_nominal` checker (`:3310`), which already performs exactly
this job elsewhere in the same file. Where a site genuinely must exclude a value,
state the English fact for that site. Re-examine `demonstrative_possessive_
reference`'s `require possessor.number is Singular` in the same pass: it is a
single construction, so the requirement excludes plural possessors rather than
partitioning a domain — the `english-v2-genitive-determiner-collapse` landing
diagnosed it and minted no ticket; this is that ticket.

Landing this unblocks `english-v2-genitive-determiner-collapse`'s own pinned
single construction, which that landing could not deliver ("Those requirements
are load-bearing, not inert, so the three branches stay distinct").

Fences. Re-spelling a category as a value list without an English reason per
site. Widening every list at once without reading the `Aquatic Alchemist` shape
— report that unit's selected analysis explicitly. A `checked by` naming a
construction.

Glossary: Nominal, Nominal Form, Determinative, Determiner, Genitive,
Possessive, Coordination. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered; 11 `nominal_form in [...]` sites) — re-measure at claim. Standard
constraints apply.

## Landing record

- Measured tree: change `wnplkvro` (the review commit), refreshed onto claim
  parent `qpkvykvz`; coverage-lock `covered` count 17,114, lock delta `+0/-0`
  in `lock_mode=report`, lock file byte-unchanged (`schema_version` 4). Every
  number below is measured on that tree. The implementer's own figures were
  stamped on `krorkkkq` before a refresh moved trunk (the
  `english-v2-form-template-defects` landing raised the corpus by 46 units), so
  they have been re-measured rather than carried forward.
- Safe collapses. The three ordinary possessive-pronoun constructions became
  one `possessed_reference`, and the three ordinary genitive-determiner
  constructions became one `genitive_determiner_reference`. Both read the
  determiner's declared `determiner_number` and `nominal_license` through the
  existing `determiner_licenses_nominal` checker, which `coverage` reports as
  `kind=DeclaredLicenseFeature` — the permitted shape. The five ordinary
  Nominal Forms remain admitted at both sites.
- Which four constructions went, and which stayed. Gone:
  `possessed_singular_reference`, `possessed_plural_reference`,
  `possessed_mass_reference` (three folded into `possessed_reference`) and
  `genitive_determiner_plural_reference`, `genitive_determiner_mass_reference`
  (folded into the surviving `genitive_determiner_reference`) — six
  declarations replaced by two, a net −4 (385 -> 381 construction
  declarations). Stayed, and why: `genitive_determiner_coordination_reference`,
  because a coordinated Noun Phrase has phrase-level agreement (it derives
  `number = Values::Plural`, `concord_class = Values::Other`) distinct from the
  homogeneous Number its Conjuncts carry, which is what an ordinary Nominal
  value would hand to `derive number = nominal.number`; and
  `demonstrative_possessive_reference`, because its demonstrative sits outside
  the possessive and must agree with the possessor, which no other frame does.
  The ticket's "seven constructions, one frame" count therefore lands as three:
  the two folds plus the coordination frame that is a different frame.
- Declared vocabulary property. `PossessiveDeterminerPronoun` declares
  `DeterminerNumber = Both` and `NominalLicense = AnyNominal`; the sole
  `Possessive` construction derives the same two properties. Construction-core
  admits and emits closed-vocabulary helpers for those two already-existing
  feature domains, exactly as it already does for `ModifierLicense`,
  `BareLocativeComplement`, `HomographLicense`, `PrepositionAttachment` and
  `PrepositionComplementKind`; the new emitters are line-for-line the
  `emit_vocab_modifier_license_helper` shape. No new mechanism was invented and
  no checker, requirement or comment names a construction, word, card, noun,
  verb, preposition or other lexeme. `coverage` reports permitted licensing
  checkers 20, forbidden 0 — unchanged from the base tree.
- The remaining Nominal Form requirements in this region, and the English fact
  at each. Three sites now carry the identical five-value list
  `[BareSingularNoun, ModifiedSingularNoun, BarePluralNoun, ModifiedPluralNoun,
  MassNoun]`: `possessed_reference`, `genitive_determiner_reference` and
  `demonstrative_possessive_reference`. The excluded pair is
  `SingularCoordination` and `PluralCoordination`, and the fact is the same at
  all three: a Nominal carrying a coordination derives
  `number = coordination.number`, the homogeneous Number of its Conjuncts, while
  the coordinated Noun Phrase agrees as a phrase; all three constructions derive
  their own `number` from the possessed Nominal, so admitting a coordination
  there would state Singular for `that player's creature and artifact`. That
  sentence is not lost — it derives, uniquely, through
  `genitive_determiner_coordination_reference`, which states the phrase-level
  Plural. No requirement in this region rests on the deleted-Category
  provenance any more.
- STOP — the possessive coordination arm — RULED, 2026-09-04. The implementer
  stopped rather than ship the direct `possessed_*` coordination arm: a probe
  adding it made Aquatic Alchemist // Bubble Up identity
  `eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb` select, by
  specificity, `your [first instant] or [sorcery spell]` in place of the correct
  `your first [instant or sorcery] spell`. Both bracketings are English, and the
  ticket's fences forbid a dominance edge, an exception or a narrowed value list
  to choose between them. Coordinator ruling: the folds land; the arm is an
  explained deferral to `english-v2-underspecified-adjunct-attachment`, which
  owns the general bracketing-preference/attachment device the arm needs (R1
  `english-v2-attachment-class-declared` and R7
  `english-v2-locative-coordination-arms` are parked on the same device). That
  ticket now carries a dated paragraph naming the arm, the two sentences and
  both analyses, and this ticket's letter strikes the arm with a dated `was:`
  line citing the ruling. `your creature and artifact` still has no derivation.
- `genitive_determiner_coordination_reference` no longer requires
  `coordination.number is Singular`, so `target player's creatures and
  artifacts` is licensed. This is a grammar widening with no coverage
  consequence: no corpus unit gains, loses or changes a candidate (see the
  per-unit proof below), so newly covered identities are none and the widening
  is proved by an added test rather than by a coverage number.
- `demonstrative_possessive_reference`. The `require possessor.number is
  Singular` stays, and the English fact is real: the construction's
  demonstrative is a lexically singular `SingularDemonstrative`, so the
  requirement is agreement between the demonstrative and the possessor it
  determines — `that sources' controller` is not English. The requirement
  excludes nothing from the language, because plural demonstrative possessives
  are attested in Oracle (`Those permanents' owners shuffle them into their
  libraries.`, `those opponents' libraries`, `those players' life totals`,
  `those cards' mana costs`, `those stickers' total toughness`) and derive
  through the general path instead: `determined_nominal(those, permanents)` ->
  `possessive_plural_reference` -> `possessive [form plural_s]` ->
  `genitive_determiner_reference`. That was checked with `english_v2 probe` on
  this tree and selects uniquely. What the landing did change there is the
  possessed Nominal: the singular-only Number and Nominal Form requirements are
  gone, so `that source's controllers` is admitted.
- Per-unit selection neutrality, the landing's central claim. `ambiguity --json`
  was run on this tree and on its refreshed parent (a reflink copy with the
  landing's eight source files reverted) and the 17,114 selected rows compared
  by identity: 0 rows only on one side, 0 status changes, 0 resolution-mode
  changes, 0 candidate-count changes, and 0 gained or lost selections. 4,554
  identities changed only the spelling of a Construction name inside the
  selected path; after normalizing the five folded names, 0 selected analyses
  differ. The affected-identity counts, non-disjoint, are 4,220
  `PossessedSingularReference`, 322 `PossessedPluralReference`, 95
  `PossessedMassReference`, 111 `GenitiveDeterminerPluralReference` and 122
  `GenitiveDeterminerMassReference`.
- Coverage. 32,641 total, 17,114 selected, 17,114 covered, 0
  selected-uncovered, 15,527 parse failures, and 0 for every one of unresolved
  ties, internal failures, exception resolutions, exception uses, round-trip
  mismatches, ownership failures, traversal failures, leaf-traversal failures,
  gap spans, overlap spans, synthetic claims and provenance-plan mismatches.
  Newly covered identities: none. Identities that stopped being covered: none.
  Wrong analyses or negative oracles newly covered: none.
- Selection census before and after, identical: 13,468 unique / 3,646
  specificity-resolved / 0 exception-resolved / 0 unresolved ties. The
  specificity share did not move, so no construction pair is named.
- Aquatic Alchemist // Bubble Up, reported explicitly as the ticket requires.
  On this tree identity
  `eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb` is
  `selected`, `resolution = unique`, one candidate, and its analysis is
  `PossessedReference { possessor: Your, nominal: ModifiedSingularNominal {
  first: AttributiveAdjectiveModifier(First), rest:
  [OrSharedHeadModifier(Instant or Sorcery)], head: NounSingularHead(Spell) } }`
  — `your first [instant or sorcery] spell`, the correct bracketing. Bubble Up
  identity
  `8745716e273011362c8bc2e22a5ec671c48281569ce49a35be96805829899ab6` remains
  `selected` by specificity over two candidates, `target [instant or sorcery
  card]`, with its two following possessives as ordinary `PossessedReference`
  readings.
- REPORT — inventories, as named lists, provenance only and never fitted to.
  Licensed vocabulary/lexicon homographs (2): vocab `AttributiveAdjective::Untap`
  beside the Verb declaration keyword action `Untap`; vocab
  `TargetingMarker::Target` beside the Noun lexeme `CommonNoun::Target`.
  Form-literal/vocabulary overlaps (9), each `surface at construction`:
  `additional` at `additional_cost`; `to` at `up_to_quantifying_determiner`;
  `the` and `next` at `definite_next_mass_quantity_reference`; `to` at
  `scalar_less_than_or_equal_to`; `the` at `number_of_scalar_value`; `the` at
  `greatest_scalar_value`; `other` at `other_than_qualified_reference`; `the` at
  `positional_partitive`. Longest form literal 11 bytes. Permitted licensing
  checkers 20 (`bare_preposition_complement_is_licensed`,
  `determinative_is_fused`, `determinative_licenses_plural`,
  `determinative_licenses_singular`, `determiner_licenses_nominal`,
  `full_coordination_is_independent`, `headed_determiner_licenses_nominal`,
  `locative_coordination_has_modifier`,
  `nominal_nonrelational_preposition_is_licensed`,
  `nominal_relational_preposition_is_licensed`,
  `noun_has_distinct_number_surfaces`,
  `object_has_no_selected_source_postmodifier`, `partitive_whole_is_licensed`,
  `predicate_adjunct_is_duration`, `predicate_adjunct_is_nonprepositional`,
  `predicate_adjunct_is_prepositional`, `predicate_preposition_is_licensed`,
  `preposition_complement_is_licensed`, `rightmost_leaf_is::<QuotedBlock>`,
  `temporal_endpoint_denotes_a_time`), forbidden 0. Construction declarations
  385 -> 381.
- Positive gate artifacts, all foreground, after the final `kata refresh` and
  after the review corrections. `cargo fmt --all` clean. Strict all-target
  Clippy at `-D warnings` for `deckmaste_construction_core` and
  `deckmaste_english_v2`: `Finished` with no warning. `cargo test --workspace`
  on the integrated tree: 128 suites `ok`, 0 failed — including `test result:
  ok. 1024 passed; 0 failed` (construction-core), `test result: ok. 76 passed;
  0 failed` (the compiled-consumer fixture), `test result: ok. 29 passed; 0
  failed` (`nominal_grammar`, with
  `possessive_determiners_license_nominals_without_form_partitioning` and
  `shared_head_coordination_remains_selected_under_a_possessive_determiner`
  both `ok`), and `test result: ok. 454 passed; 0 failed; 1 ignored` (`xtask`,
  see *Trunk regression* below).
  `coverage --check` schema 10, the summary above, `lock_mode=report`,
  lock delta `+0/-0`. `ambiguity --require-resolved` 17,114 selected, 13,468
  unique, 3,646 specificity-resolved, 0 unresolved ties. `roundtrip` 17,114
  parse accepted / 17,114 clean / 0 mismatched. No citation changed, so the
  cite gates were not required.
- Trunk regression, inherited, found here and since fixed on trunk. On the tree
  the gates were first run against, `cargo test --workspace` had two failures,
  both in `-p xtask --lib`: `facts::tests::every_stub_has_a_row_or_a_recorded_reason`
  and `facts::tests::keyword_action_destinations_come_from_their_cr_entry`
  (`test result: FAILED. 451 passed; 2 failed; 1 ignored`). They reproduced
  unchanged with this landing's eight source files reverted to the refreshed
  parent, and again with the pre-refresh `crates/xtask/src/facts.rs` restored,
  so they were trunk state, not this landing's. Cause: the
  `workbench-dedup-tables` landing re-spelled `idris/src/Experimental/Words.idr`'s
  keyword-action rows as record updates over `plainAct "<Label>"`, while
  `crates/xtask/src/facts.rs` still scanned for literal `MkActFacts "…"` rows,
  so it read 70 stubs against 0 rows. The final `kata refresh` before integrate
  brought in `workbench-enact-agent-role`, which replaced that scan with an
  `act_rows` reader that understands the bare and the overridden `plainAct`
  spelling; `cargo test -p xtask --lib` is `test result: ok. 454 passed; 0
  failed; 1 ignored` on the integrated tree. The ticket this review had minted
  for it was therefore deleted before integrate rather than filed against
  already-fixed trunk.

- Performance advisory, every corpus command at 8 workers, against the 16.26 s
  quiet-host ceiling (16,260 ms; wall times are given in milliseconds so no
  figure reads as a rule number). `coverage --check` 106,269 ms at 118,302
  ns/B, host load
  7.97/10.19/10.29. `ambiguity --require-resolved` 96,918 ms at 116,512 ns/B,
  host load 4.78/8.38/9.62. `roundtrip` 94,546 ms at 112,859 ns/B, host load
  8.93/8.87/9.66. The base-tree run for the neutrality proof was 110,858 ms at
  129,998 ns/B, host load 6.36/9.30/9.46 — the same corpus on the parent tree,
  so the 4% spread between it and this tree's ambiguity run is contention, not
  a parse-time change. Contention stamp supplied by the coordinator: 1
  concurrent codex executor (`english-v2-granted-ability-coordination`) and 2
  concurrent landing reviews on this host for the whole measurement window,
  plus this review's own base-tree corpus runs. Every figure is a contention
  artifact; none is a parse-time regression from this landing.
- Assurance counts. Restored 0. Re-spelled 4 existing test functions —
  `authentic_nominal_and_selector_sentences_parse` and
  `restricted_postmodifier_paths_ownership_and_ambiguity_are_exact` in
  `tests/nominal_grammar.rs` (two exact selected-path witnesses, assertions
  unchanged) and two `feature_recipe_tests.rs` consumers (constructor rename
  only). Ignored with blockers 0. Added 3 test functions — one compiler
  emission contract in `validate.rs` and two English selection/round-trip
  contracts in `tests/nominal_grammar.rs` — carrying six cases between them
  (possessive singular, possessive plural, plural genitive coordination,
  demonstrative plural possessed Nominal, the demonstrative/possessor
  disagreement rejection, and the demonstrative-over-coordination witness added
  by this review), plus the Aquatic shared-head bracketing. Removed 0; no test
  was deleted, and no value comparison became a `discriminant`/`matches!` check.
- Deviations and additions. One: construction-core closed-vocabulary support
  for the two already-existing determiner feature domains
  (`DeterminerNumber`, `NominalLicense`), which is the declared-property
  plumbing the ticket's pinned shape requires — a closed vocabulary cannot
  otherwise carry the property the checker reads. Beyond the ticket's letter,
  this review added one `require nominal_form in [...]` to
  `demonstrative_possessive_reference` (justified under *The remaining Nominal
  Form requirements*) and one test case witnessing it. The required possessive
  coordination arm is omitted under the ruled STOP above. No dominance edge, no
  exception, no inventory fitting, no word-naming guard.
- `glossary gap:` Nominal Form, Genitive, Possessive — all three are used by
  this ticket and by the feature model, and none is defined in
  `docs/contexts/oracle-english/CONTEXT.md` (which does define Nominal, Noun
  Phrase, Determinative, Determiner, Coordination, Conjunct and Coordinator).

### Review corrections

- MEDIUM — `demonstrative_possessive_reference` had **both** its Nominal Form
  and Number requirements removed, so it alone among the three possessive
  frames admitted `SingularCoordination` and `PluralCoordination` while
  deriving `number = possessed.number` — the exact wrong-agreement the two
  folded constructions exclude by a stated fact. Added the same five-value
  `require possessed.nominal_form in [...]` there, with the same comment. The
  change is provably language-neutral: `Destroy that player's creature and
  artifact.` still selects, uniquely, through
  `genitive_determiner_coordination_reference` (verified by `english_v2 probe`
  and added as a test case), and it is corpus-neutral by construction — the
  base tree's list at that site is a subset of the new one, and the neutrality
  proof shows the wider tree already changed no corpus candidate.
- MEDIUM — every number in the record was stamped on `krorkkkq`, before a
  refresh moved trunk. Re-measured and re-stamped on `wnplkvro`: coverage
  17,068 -> 17,114, parse failures 15,573 -> 15,527, census 13,452/3,616 ->
  13,468/3,646, renamed identities 4,513 -> 4,554 with per-family counts
  re-counted, form-literal/vocabulary overlaps 5 -> 9. The construction count
  (385 -> 381) was re-measured on both trees and is unchanged.
- MEDIUM — the record reported the inventories as bare counts where the
  landing contract asks for named lists. All three are now listed by name.
- MEDIUM — the STOP was written as an open question ("Decision wanted"). It is
  now recorded as ruled, with the ruling's disposition; the deferral paragraph
  was appended to `docs/tickets/planned/english-v2-underspecified-adjunct-
  attachment.md` and the ticket's letter carries the dated `was:` strike.
- MEDIUM — the `demonstrative_possessive_reference` entry asserted the
  singular-possessor agreement fact without checking whether the excluded shape
  exists in Oracle. It does — five plural demonstrative possessives are
  attested — so the entry now names them and shows the general genitive path
  that derives them, which is what makes the requirement an agreement rule
  rather than an exclusion.
- MEDIUM — the record left the ticket's "seven constructions" arithmetic
  unanswered. Added *Which four constructions went, and which stayed*, naming
  the six folded declarations and the English reason each survivor is a
  different frame.
- MEDIUM — the record did not say why the ticket's plural-genitive-coordination
  widening produced no coverage gain. The reconciliation (a widening no corpus
  unit exercises, so witnessed by test rather than by a number) is now stated
  where the widening is disclosed.
- MEDIUM — `glossary gap:` named only Nominal Form; Genitive and Possessive are
  equally undefined. All three are now listed.
- MEDIUM — the workspace suite was red on two `xtask` `facts::` tests. Proved
  inherited from the `workbench-dedup-tables` landing (both failures reproduce
  with this landing's files reverted) and disclosed under *Trunk regression*.
  The final refresh before integrate brought the trunk fix
  (`workbench-enact-agent-role`), so the suite is green on the integrated tree
  and the ticket this review had minted was deleted instead of filed.

- LOW — the compiled-consumer fixture in `crates/deckmaste_construction/tests`
  was not extended for the two new closed-vocabulary feature domains. Left as
  is, deliberately: that fixture covers no closed-vocabulary feature metadata at
  all (`ModifierLicense`, `BareLocativeComplement`, `HomographLicense`,
  `PrepositionAttachment` and `PrepositionComplementKind` are all absent from
  it), the house idiom is the token-level emission test in `validate.rs` that
  this landing followed, and the real compiled consumer is
  `deckmaste_english_v2`, which reads both emitted helpers on every one of the
  17,114 accepted corpus units.
- LOW, observation, no change made. `demonstrative_possessive_reference` looks
  redundant with the general path: `that player's creature` derives both
  through it and through `determined_nominal -> possessive_singular_reference
  -> possessive -> genitive_determiner_reference`. Removing it is a selection
  question well outside this ticket, and the corpus is currently neutral to it.
