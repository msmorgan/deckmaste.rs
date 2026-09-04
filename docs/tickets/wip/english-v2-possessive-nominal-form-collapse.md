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

- the `possessed_*` family has **no coordination arm at all** — `your creature
  and artifact` has no derivation;
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

- STOPPED, with the safe portion committed for review. Measured tree: change
  `krorkkkq`, refreshed onto claim parent `qpkvykvz`; coverage-lock `covered`
  count 17,068. The report-mode bless left the schema-10 lock byte-unchanged.
  The required possessive-pronoun coordination arm is not present, so this
  record does not claim the whole ticket is complete.
- Safe collapses: the three ordinary possessive-pronoun constructions became
  `possessed_reference`, and the three ordinary genitive-determiner
  constructions became `genitive_determiner_reference`. Both read the
  determiner's declared `determiner_number` and `nominal_license` through the
  existing `determiner_licenses_nominal` checker. The five ordinary Nominal
  Forms remain in these constructions. Coordination remains outside them
  because a coordinated Noun Phrase has phrase-level concord distinct from the
  homogeneous Number of its Conjuncts; it therefore needs the separate
  coordination construction described by the ticket rather than admission as
  an ordinary Nominal value.
- Declared vocabulary property: `PossessiveDeterminerPronoun` declares
  `DeterminerNumber = Both` and `NominalLicense = AnyNominal`; `Possessive`
  derives the same determiner properties. Construction-core now admits and
  emits closed-vocabulary helpers for those two existing feature domains. The
  checker remains general: no checker or requirement names a construction,
  word, card, noun, verb, preposition, or other lexeme.
- The existing `genitive_determiner_coordination_reference` no longer requires
  singular coordination, so `target player's creatures and artifacts` is
  licensed. `demonstrative_possessive_reference` no longer restricts the
  possessed Nominal to singular ordinary forms; the singular possessor
  requirement remains because a singular demonstrative agrees with the
  possessor it determines. Thus `that source's controllers` is admitted while
  `that sources' controller` is rejected for the stated English agreement
  fact.
- STOP — required possessive-pronoun coordination arm. A probe adding the
  direct arm needed by `your creature and artifact` made Aquatic Alchemist //
  Bubble Up identity
  `eabbc2128de743c6555fa404e3102827062681275d373db6f3adbed32980abbb`
  select, by specificity, the split analysis
  `PossessedCoordinationReference -> OrNominalCoordination ->
  ModifiedSingularCoordinationMember(first instant) +
  ModifiedSingularCoordinationMember(sorcery spell)`. The correct surviving
  analysis is
  `PossessedReference -> ModifiedSingularNominal ->
  AttributiveAdjectiveModifier(first) -> OrSharedHeadModifier(instant or
  sorcery) -> SingularHead(spell)`, i.e. `your first [instant or sorcery]
  spell`. Both attachments are syntactically available in English; there is no
  true general Nominal Form or determiner-license fact at this site that admits
  `your creature and artifact` but excludes the split reading. The ticket
  forbids a dominance edge, exception, or narrowed form to choose between
  them, so the probe arm was removed and work stopped with both analyses
  disclosed. Decision wanted: authorize a general, English-justified selection
  principle for this attachment, or revise the ticket's arm/fence combination.
- Construction count: 385 on the refreshed parent / 381 on this tree. The
  exact four-construction reduction is the two three-to-one ordinary folds
  above. No count is claimed for the required arm that was probed and removed.
- Coverage-lock delta, as printed in report mode: 17,068 -> 17,068 (`+0/-0`).
  Coverage summary before and after: 32,641 total, 17,068 selected and covered,
  15,573 parse failures, 0 selected-uncovered, unresolved ties, internal
  failures, exception resolutions, exception uses, round-trip mismatches,
  ownership failures, traversal failures, leaf-traversal failures, gap spans,
  overlap spans, synthetic claims, or provenance-plan mismatches. Newly
  covered identities: none. Lost identities: none. Wrong analyses or negative
  oracles newly covered: none.
- Selection census before and after: 13,452 unique / 3,616
  specificity-resolved / 0 exception-resolved / 0 unresolved ties. The full
  refreshed-parent and feature `ambiguity --json` rows were compared by id:
  0 status changes, 0 gained or lost selections, 0 resolution-mode changes,
  and 0 candidate-count changes. 4,513 identities changed only a raw selected
  Construction-path spelling; after normalizing the five folded names, 0
  selected analyses differ. Non-disjoint affected-identity counts are 4,181
  `PossessedSingularReference`, 321 `PossessedPluralReference`, 93
  `PossessedMassReference`, 111 `GenitiveDeterminerPluralReference`, and 122
  `GenitiveDeterminerMassReference`. No identity's selected analysis changed,
  so there is no changed-selection identity list beyond the explicit Aquatic
  probe disclosure above.
- Aquatic final selection: the identity above is unique, not
  specificity-resolved, and keeps `your first [instant or sorcery] spell` with
  the shared head `spell`. Bubble Up identity
  `8745716e273011362c8bc2e22a5ec671c48281569ce49a35be96805829899ab6`
  remains specificity-selected as `target [instant or sorcery card]`; its two
  following possessives remain ordinary `PossessedReference` readings.
- Inventory pins and ceilings: 2 licensed vocabulary/lexicon homographs, 5
  form-literal/vocabulary overlaps, 20 permitted licensing checkers, 0
  forbidden licensing checkers, longest form literal 11 bytes. These were
  reported, not fitted.
- Positive gate artifacts, all after refresh and in the foreground: `cargo fmt
  --all` completed; strict all-target Clippy for
  `deckmaste_construction_core` and `deckmaste_english_v2` finished without a
  warning; `cargo test --workspace` finished green. Representative artifacts:
  `test result: ok. 39 passed; 0 failed`, `test result: ok. 1024 passed; 0
  failed`, `test result: ok. 76 passed; 0 failed`, and `test result: ok. 453
  passed; 0 failed; 1 ignored`. `coverage --check` reported schema 10 and the
  summary above with `lock_mode=report`; `coverage --bless` wrote the same
  covered set. `ambiguity --require-resolved` reported 17,068 selected,
  13,452 unique, 3,616 specificity-resolved, and 0 unresolved ties. Roundtrip
  reported 17,068 parse accepted / 17,068 clean / 0 mismatched. No citation
  changed, so cite gates were not required.
- Performance advisory, every corpus command at 8 workers: final coverage
  check 100.388 s at 113,575 ns/B, host load 10.44/11.34/12.08; ambiguity
  95.806 s at 115,107 ns/B, host load 5.86/9.06/11.06; roundtrip 92.082 s at
  116,364 ns/B, host load 6.64/8.77/10.77. All exceed the 16.26-second
  quiet-host ceiling and are advisory. The declared concurrent workload was
  three executors including this one (`english-v2-copular-complement-sum` and
  `english-v2-granted-ability-coordination` beside it), with additional
  integration/review activity not visible from this sandbox.
- Assurance counts: restored 0; re-spelled 4 existing test functions (two
  exact selected-path witnesses and two feature-recipe consumers) with their
  assertions unchanged; ignored with blockers 0; added 3 test functions (one
  compiler emission contract and two English selection/round-trip contracts);
  removed 0. The new English cases cover possessive singular/plural,
  plural genitive coordination, demonstrative plural possessed Nominal,
  demonstrative/possessor disagreement rejection, and the Aquatic shared-head
  bracketing.
- Deviations and additions: construction-core vocabulary support for the two
  existing determiner feature domains is the necessary declared-property
  plumbing requested by the ticket. The required possessive coordination arm
  is omitted under the STOP above. No dominance edge, exception, inventory
  fitting, or word-naming guard was added. `glossary gap: Nominal Form` is used
  by the ticket and feature model but is not defined in the Oracle English
  glossary.
