---
needs: []
---
# Read *devotion* with its *to* Complement: You gain life equal to your devotion to green.

## Why

*your devotion* reads as a Noun Phrase; *your devotion to black* does not, so
almost no devotion card reads. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:is less` 21 sole-blocker
faces (the Theros Gods) is one corner of it. `cargo xtask english-v3 grep -i
'devotion to'` finds **61** faces, **58** unread; 58 failing lines match and
**57** are the only failing line of their face. By shape (sole-line faces):
*equal to your devotion to C* 19; *where X is your devotion to C* 16; *as
long as your devotion to C [and C] is less than five/seven* 21; *if your
devotion to C and C is seven or greater* 1.

`english-v3-data-batch-2` deferred this ("*Devotion to* needs a nominal
Complement marker beyond the currently declared None/Of"). The 21 God lines
also need a spelled cardinal after *less than*
(`english-v3-spelled-cardinal-scalar-measure`); this ticket alone is expected
to cover about 35 faces.

## Goal

*devotion* takes a *to* Complement naming one colour or a coordination of two
(*your devotion to black*, *your devotion to white and black*), and the
resulting Noun Phrase has Scalar Denotation wherever *your life total* does:
after *equal to*, as the value of *X is …*, and as the Subject of a scalar
comparison.

## Analysis

CGEL Ch. 5 §14.1(a), p. 440: "With PP complements the head noun determines
the choice of preposition or the range of permitted choices. The default
preposition is *of*", and nouns such as *injury* "permit either default *of*
or a more specific preposition (… *an injury to*/*of the wrist*)"; "*address*
takes *to*". That backs a noun licensing a *to* Complement. That *devotion*
does, and what the Complement may be, are project rulings.

Existing declarations: `lexeme:CommonNoun/Devotion` (`lexicon/core.ron`,
`countability: [Mass]`, no features, no frames); feature
`NominalComplementMarker { None, Of }` and table `nominal_postmodifier_licence`
(`crates/deckmaste_english_v3/src/declarations.rs`), which license a noun's
*of* Complement; `vocab:Preposition/To` with `PrepositionFunctionLicence`;
the colour words `vocab:ColorWord/Black` etc., which read today as Adjectives
and as Keyword Quality (*protection from black*) but not as a Noun Phrase
("black" as `noun-phrase` 0; "to black" as `preposition-phrase` 0).
`ScalarNominalOperand` and the *equal to* route read
`NounPhrase.ScalarDenotation`.

Orchestrator rulings 2026-10-10:

- DV1. Extend the existing noun-Complement licence: `NominalComplementMarker`
  gains `To`, *devotion* declares it (data), and *to* declares
  `NounComplement` in its Preposition Function Licence if it does not
  already. The existing Construction that reads a noun's *of* Complement
  reads the *to* Complement through the same table; no new Construction.
- DV2. The Complement of *to* here is a colour word. Route it through the
  declared class the colour words already carry (the one Keyword Quality
  reads); if a colour word must head a Noun Phrase for this, that is a
  declared nominal use on the colour-word class, not a new Lexeme per colour
  and not a test on the five words. Two colours coordinate with *and* by the
  ordinary coordination of that category.
- DV3. *devotion* declares Scalar Denotation as *life total* does (data).
  The licence for *equal to …*, *X is …* and *… is less than N* is that
  feature; no test on the noun.
- DV4. One analysis per string. The twins to avoid: (a) *to black* attaching
  as a clause Adjunct or as a Complement of *equal* / *is* instead of
  *devotion* (*equal to your devotion to green* has one *to* per head);
  (b) *black and green* coordinating NP-high (*[your devotion to black] and
  [green]*); (c) the colour word read as an Adjective with an elided noun.
- DV5. *your devotion to that color* (Nykthos, Shrine to Nyx; Nyx Lotus) and
  *Your devotion to each color and each combination of colors* (Altar of
  the Pantheon) are the 3 covered faces. They read today although *devotion*
  licenses no Complement, so their *to* PP attaches somewhere else. With DV1
  the Complement of *to* is whatever NP the Preposition takes, so these gain
  the Complement Reading. Inspect their current Readings on the claim
  parent: a Reading that attaches *to that color* to the verb or the Clause
  is a wrong analysis to retire if a declared feature separates it, and to
  report with its shape if not. No named guard.
- DV6. Newly covered faces are gains. Reading-count changes are expected on
  exactly those 3 faces, of the DV5 shape. Every other covered face,
  including every covered face with a colour word, keeps its count; an
  increase from colour words newly heading NPs elsewhere (*becomes black*,
  *protection from black*) is a DV2 defect. Report every change with its
  shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Nylea's Disciple: "When this creature enters, you gain life equal to your
  devotion to green." (control "… equal to your life total." 1)
- Tymaret, Chosen from Death: "Tymaret's toughness is equal to your devotion
  to black." (control "… equal to your life total." 1)
- Gray Merchant of Asphodel: "When this creature enters, each opponent loses
  X life, where X is your devotion to black. You gain life equal to the life
  lost this way."
- Master of Waves: "When this creature enters, create a number of 1/0 blue
  Elemental creature tokens equal to your devotion to blue."

Also 0 today: "your devotion to black" and "your devotion to black and green"
(`noun-phrase`); "Your devotion to black is 5." (sentence). Controls that
must not change: "your devotion" (`noun-phrase`) 1; "Its power is 5." 1;
"Target creature gets +X/+0 until end of turn, where X is your life total."
1; "You gain life equal to your life total." 1; "Your life total is seven or
greater." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bdevotion\b|\b(black|blue|green|red|white)\b` (3,295
faces; the colour-word half guards DV2, which changes what a colour word can
be) plus 300 random covered controls (`--random 300 --seed 20261010`),
`--workers 6 --samples-per-face 0`, compared with `cargo xtask english-v3
diff` against the claim parent. Zero lost; DV6 for increases. Full `cargo
test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_english_v3` and `deckmaste_lexical_source`, once, at the end: a
rebuild after a grammar edit takes about 2–3 minutes and the six test
binaries about 75 seconds; lexicon data loads at runtime. Tests assert the
roles (the *to* PP as Complement of *devotion*, the colour coordination
inside it, the NP's Scalar Denotation, the DV4 negatives), never an
incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- *is less than five* / *less than seven* (21 sole-line faces): the spelled
  cardinal, `english-v3-spelled-cardinal-scalar-measure`. Those faces need
  both tickets.
- Any further gap on the 3 covered faces beyond DV5.
- The game meaning of devotion.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
