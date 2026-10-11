---
needs: []
---
# Read *an additional* before a power/toughness adjustment: it gets an additional +1/+0.

## Why

A power/toughness adjustment reads as the Complement of *get* ("It gets
+1/+0." 1), and *an additional* reads before a measured noun ("It deals an
additional 2 damage." 1; "It enters with an additional +1/+1 counter on it."
3). The two together do not: "It gets an additional +1/+0." 0. Blocker report
at trunk `qzprwuxz` (`target/english-v3/blockers-wave9.json`): `at:0 .`,
`at:1 .`, and the mid-sentence keys `at:2 until`, `at:2 and`, `at:0 as`
(the parser stops right after the pair). `cargo xtask english-v3 grep -i 'an
additional [+-](\d+|X)/[+-](\d+|X)([.,]| (until|and|as|for|unless|if)\b)'`
finds **26** faces, all unread; 26 failing lines match, **25** the only
failing line of their face. Continuations among the sole lines: *until end of
turn* 11, *and has/gains …* 6, *as long as …* 4, sentence end 4.

## Goal

*an additional* + a signed power/toughness pair reads as the Complement of
the verbs that take the bare pair, with every dependent the bare pair's
clause takes (duration, *for each*, *as long as*, coordination with *and
has*), byte-exact.

## Analysis

No CGEL passage is relied on. The adjustment is a measure expression with no
head noun (`PowerToughnessAdjustment`), so *an additional +1/+0* is not an
ordinary NP with a premodifier: the analysis is a project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): frame
`SlashMeasure = Predicate(Complement(PowerToughnessAdjustment))` with
`frame_category PowerToughnessAdjustment = SlashPair`, and the shared
Complement instances over `SlashPair` (`FinitePredicate` /
`SecondaryVerbPhrase`); `SlashPair` and `SlashModifiedNominal` (`[modifier:
SlashPair, " ", head: Nominal]`, *a +1/+1 counter*); *additional* is an
Adjective Lexeme read in `PremodifiedNominal` over `CardinalMeasuredNominal`
(*an additional 2 damage*) under `IndefiniteNounPhrase`.

Orchestrator rulings 2026-10-11:

- AD1. Refine the adjustment Complement, not the NP family: the
  `PowerToughnessAdjustment` Complement gains an optional leading field for
  the additive modifier (the compiler's optional lexical field with a
  trailing separator, R-budget: `optional(…, " ")`), or a second form of the
  Construction that builds the Complement. No new Construction. Absent
  fields are omitted from Debug, so covered digests must not move; if they do,
  report by class (R-reach).
- AD2. The licence reads a declared feature of the modifier Lexeme (the
  additive class *additional* belongs to; find the feature that already lets
  it stand before a cardinal measure and reuse it), plus the article's
  ordinary agreement with it. No test on *additional*, on the sign or on
  *get*.
- AD3 (R-class). Every Lexeme of that declared additive class gets the
  position, and every verb whose frame is `SlashMeasure` (*get*, and any
  other that declares it). Do not mint *another +1/+0* or *extra*; "It gets
  another +1/+0." is 0 and unattested: no work, and no check to block it if
  the class admits it.
- AD4 (R-one). One analysis per string. The twins to avoid: (a) *an
  additional +1/+0* as an `IndefiniteNounPhrase` whose head is the pair
  (the pair is not a Nominal); (b) a fused-head NP *an additional* followed
  by a stray pair; (c) *an additional +1/+1 counter* gaining a second
  Reading through the new field (control below stays 3).
- AD5. *an additional +X/+0* (variable pair) follows from `SlashPair`; in
  scope if it falls out.
- AD6. Newly covered faces are gains. No Reading-count change is expected on
  covered faces. The witness must read exactly as many ways as its control
  without *an additional*.

- AD7 (orchestrator, 2026-10-11). AD2 assumed an absent feature. Declare one
  Additive Modifier Use feature at class level on existing additive Lexemes
  whose Category can take the position; mint none. Amend the Oracle English
  glossary as a project term. First inspect the existing measured-noun and
  ordinary-noun analyses; reuse their licence only if it already expresses
  the additive class. AD1 and AD3–AD6 stand.

## Witnesses

Each is a sentence of the face's only failing line. Probed 2026-10-11 with the
trunk binary (`--category sentence --readings 8`): all 0.

- Silver-Inlaid Dagger: "As long as equipped creature is a Human, it gets an
  additional +1/+0."
- Wild Might: "That creature gets an additional +4/+4 until end of turn
  unless any player pays {2}."
- Taste for Mayhem: "Enchanted creature gets an additional +2/+0 as long as
  you have no cards in hand."
- Desert's Due: "It gets an additional -1/-1 until end of turn for each
  Desert you control."
- Tenza, Godo's Maul: "As long as it's legendary, it gets an additional
  +2/+2."

Controls that must not change: "As long as equipped creature is a Human, it
gets +1/+0." 1; "It gets -1/-1 until end of turn for each Desert you
control." 1; "It deals an additional 2 damage." 1; "It enters with an
additional +1/+1 counter on it." 3; "Draw an additional card." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\badditional\b` (686 faces, 115 unread) plus 300 random
covered controls (`--random 300 --seed 20261008`); if the Complement
Construction itself changes form (AD1), add every face matching
`[+-](\d+|X)/[+-](\d+|X)`; `--workers 6 --samples-per-face 0`, compared with
`cargo xtask english-v3 diff` against the claim parent. Zero lost; AD6 for
changes. Full `cargo test -p deckmaste_english_v3` and clippy `--tests -- -D
warnings` (plus `-p deckmaste_lexical_source` if a feature is declared), once,
at the end (a grammar rebuild takes 2–3 minutes, the six test binaries about
75 seconds; lexicon data loads at runtime). Tests assert the role (the
modifier is a dependent inside the adjustment Complement; AD4 negatives),
never an incidental count or a Debug-printed tree. Attestation by `cargo
xtask english-v3 grep` only.

## Out of scope

- *an additional time* (*investigate an additional time*, 9 sole; *triggers
  an additional time* reads): frequency NP, see
  `english-v3-quantified-times-frequency`.
- *can block an additional seven creatures each combat* (2 faces).
- *gets an additional upkeep step after this step* (Paradox Haze); *you get
  an additional vote* (Brago's Representative).
- *create those tokens plus an additional Treasure token* (4 faces).

## Claim-parent check and STOP (2026-10-11)

Measured on `zquvqlrm`, empty child of claim `nxlymxxu`, before any
implementation: `cargo xtask english-v3 probe --category sentence --readings
64 --json` reports all five witnesses at 0, with complete enumeration.
Their controls with *an additional* removed, in witness order, report
1 / 1 / 2 / 1 / 1. The remaining controls (damage / counter / card) report
1 / 3 / 1. No witness is already covered. Probe evidence is under
`target/english-v3/english-v3-additional-slash-adjustment/*-before.json`.

STOP: AD2 asks to reuse an existing additive-class feature, but none is
present on this claim parent. `Additional` belongs to `AttributiveAdjective`
in `crates/deckmaste_lexical_source/lexicon/vocabulary.rs`, with the general
`HomographLicense = Unlicensed` and `ModifierLicense = Unrestricted`, shared
with *starting*, *opening*, *base*, and the rest of that class.
`PremodifiedNominal` admits its ordinary `AdjectivePhrase` before
`CardinalMeasuredNominal`; it does not test an additive licence. The existing
`MeasureModifierUse`, `MeasureOperator` and `CardinalModifierUse` features
are not declared on *additional* and govern different positions.
Resolved by AD7: declare the additive adjective class licence. The existing
`AdjectivePremodifierUse` licence is general, not additive; both inspected
controls use `IndefiniteNounPhrase` over `PremodifiedNominal` with `Adjective`,
over `CardinalMeasuredNominal` for damage and `Noun` for card. That licence
would admit unrelated adjectives before a pair and cannot be reused.

## Landing record

1. **Census delta — `zquvqlrm`.** Bounded 10,783 faces: covered
   10,006 → 10,031; total Readings 228,917 → 229,109; 25 gains, none
   through an unintended new analysis. Lost faces: none. Decreases / increases
   on covered faces / digest-only classes: 0 / 0 / none (0 lines); 0 issues.
   Selector: corpus grep `(?i)\badditional\b|[+-]?(\d+|[XYZ])/[+-]?(\d+|[XYZ])`
   (10,582 faces), union 300 random covered faces, seed 20261008; 201 controls
   outside the grep set. This includes unsigned and Y/Z pairs and every consumer of
   the changed declarations (all 7,081 baseline `SlashPair` users are included).
   BEFORE uses the captured claim-parent `nxlymxxu` executable with the parent
   grammar and lexical data restored; the verified implementation was then
   restored byte-for-byte. AFTER and `diff --lines` use the new executable.
   Evidence: `target/english-v3/english-v3-additional-slash-adjustment/`.
   Final authorized integration refresh: the ORIGINAL 10,783-face manifest
   gives 37 gains (25 this landing; 12 from trunk `onymknyk`, mana-symbol
   passive subjects), covered 10,006 → 10,043 and Readings 228,917 → 229,297.
   Lost / decreases / increases / digest-only / issues remain 0. The built-in
   `refresh-diff.json` isolates those 12 trunk gains (21 materialized lines);
   no covered-face Reading shapes changed.
2. **Witnesses — `zquvqlrm`.** Silver-Inlaid Dagger 1 / control 1; Wild
   Might 1 / 1; Taste for Mayhem 2 / 2; Desert's Due 1 / 1; Tenza, Godo's
   Maul 1 / 1. Each was 0 before. W9-A preserves Taste for Mayhem's independent
   ambiguity. Damage / counter / card controls remain 1 / 3 / 1. The variable
   pair *an additional +X/+0* reads 1, byte-exact.
3. **Spot checks — `zquvqlrm`.** Bride's Gown, Desert's Due, Devouring
   Rage, Fistful of Force, Glistening Deluge, Growth Cycle, Heavy Mattock,
   Motivated Pony, Silver-Inlaid Dagger, Wild Might: each retains *additional*
   inside the `SlashPair` argument of `SelectedPredicate`. Document Readings
   / controls: 2/2, 1/1, 1/1, 4/4, 1/1, 2/2, 1/1, 8/8, 1/1, 1/1.
4. **Deviations and STOPs — `zquvqlrm`.** AD2 STOP resolved by AD7;
   Additive Modifier Use is a new project term and one class-level feature
   on the existing adjectives *additional* / *extra*. No Lexeme, Construction
   or frame added or deleted. The inspected noun controls use
   `IndefiniteNounPhrase(PremodifiedNominal(Adjective, CardinalMeasuredNominal
   / Noun))`; their general `AdjectivePremodifierUse` licence cannot express
   the additive class and would admit unrelated adjectives before a pair.
   The article is a paired optional lexical field, retaining its ordinary
   Onset agreement and identity. The three non-adjustment consumers
   (`CostPowerToughnessKeyword`, `SlashMeasure`, `SlashModifiedNominal`) read
   the declared feature to reject modified pairs; no old route is retired.
   The compiler Selected Literal observation is recorded as hygiene C5.
   The final refresh independently corrected the two inherited timing
   values in `english-v3-predicative-gap-relative`; the documentation
   conflict was resolved to trunk’s equivalent wording, leaving no change
   to that ticket in this landing.
5. **Tests — `zquvqlrm`.** `cargo test -p deckmaste_english_v3`: passes, 1,137 passed and one
   inherited ignore: `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`
   (MS4 needs a governed Mana Phrase Subject route keeping bare {X}/{2}
   outside ManaPhrase; `english-v3-mana-symbol-verbless-subject`).
   `cargo test -p deckmaste_lexical_source`: passes after re-spelling its exact
   feature-map expectation for *extra*; initial run found that mismatch.
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests --
   -D warnings`: passes. Citation checks: 0 noncompliant, 0 stale; diff audit:
   0 new citation sites. Restored / re-spelled / newly ignored / added /
   removed: 0 / 6 / 0 / 4 / 0. No removals. Re-spelled tests:
   `grammar::slash_pairs_preserve_ordered_components_and_explicit_signs`,
   `grammar::independent_slash_consumers_preserve_count_components_and_leaf_order`,
   `coordinated_property_values::independent_ordered_pair_and_premodification_preserve_both_laws`,
   `each_of_partitive::fused_head_controls_singular_agreement_independently_of_plural_oblique`,
   `keyword_payloads::authentic_amount_cost_quality_cost_and_size_payloads_keep_separators`
   add absent article/modifier fields to the same expected pair values;
   `next_lexical::single_and_extra_are_invariant_adjectives_with_declared_default_license`
   adds `AdditiveModifierUse = Yes` to *extra*'s exact expected feature map.
   Authorized post-refresh checks all pass: `cargo build -p xtask`;
   `cargo test -p deckmaste_english_v3` (1,143 passed, one inherited ignore);
   `cargo test -p deckmaste_lexical_source` (88 passed);
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests --
   -D warnings`. The designated selected-complement chart-budget test
   passes without a ceiling change. Final citation checks remain clean.
6. **Timing — `zquvqlrm`.** Bounded AFTER census: 4 min 4 s 620 ms wall
   under `/usr/bin/time -v`, 6 workers, samples per face 0. No full-corpus
   census was run. Final refreshed census: 2 min 39 s 441 ms corpus wall;
   `/usr/bin/time -v` wall 6 min 7 s 790 ms including Cargo lock wait.
   Scoped `cargo build -p xtask` passed (4 min 14 s).
