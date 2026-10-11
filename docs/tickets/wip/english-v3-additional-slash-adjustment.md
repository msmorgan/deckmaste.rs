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

## Landing record
