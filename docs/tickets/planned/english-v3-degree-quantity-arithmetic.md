---
needs: []
---
# Read addition and subtraction on a degree-quantified Noun Phrase: that many plus one +1/+1 counters are put on it instead

## Why

*that many +1/+1 counters* and *twice that many +1/+1 counters* read; *that
many plus one +1/+1 counters* and *that many cards plus one* do not. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): keys
`at:many plus` 15 and failing token `plus` 38 sole-blocker faces.
`cargo xtask english-v3 grep -i` counts, all unread:

| Shape | Pattern | Faces | Sole failing line |
|---|---|---|---|
| inside the Determiner: *that many plus one +1/+1 counters* | `that many plus (one\|two\|[0-9])` | 19 | 15 |
| delayed: *that many cards plus one*, *that many dice plus one*, *that many cards minus one* | `that many [a-z0-9/+ ]+ (plus\|minus) (one\|two\|[a-z]+)\b` | 21 | 15 |

Second causes inside those: 5 of the 19 continue *… plus one of each of
those kinds of counters* (the partitive is unread without the arithmetic
too); 4 of the 21 start *Discard all the cards in your hand*
(`english-v3-predeterminer-all`); the *roll that many dice plus one and
ignore the lowest roll* lines have other gaps. The clean set is about 14 +
7.

## Goal

A degree Determinative Phrase (*that many*) takes an addition or subtraction
phrase (*plus one*, *minus one*, *plus four*) either inside the Determiner,
before the noun, or delayed after the head noun, and the whole still
quantifies that noun, with byte-exact realisation and plural agreement.

## Analysis

CGEL Ch. 5 §11(f), p. 433, "PPs: condition, exception, proportion, and
addition/subtraction", [8iv]: "*There's room for* [*ninety thousand minus a
few hundred*] *spectators*"; "Here we have PPs in post-head modifier
function. There are alternants in which the PP is delayed, so that it is not
a constituent of the DP: … *There's room for ninety thousand spectators
minus a few hundred*." That backs both attested orders: the *plus* / *minus*
PP as post-head Modifier in the Determinative Phrase, and its delayed
alternant after the noun. CGEL's head there is a numeral; a degree
Determinative head (*that many*) and a variable are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`DegreeDeterminativePhrase` (`QuantitativeDeterminer`, `[modifier:
lexical(Determinative), " ", head: lexical(Determinative)]`, `DegreeUse`
Modifier / Head) reads *that many*; `PrepositionDeterminer`
(`[value: QuantitativePrepositionPhrase]`) is the precedent for a PP inside
a quantitative Determiner; `ArithmeticMeasure` (`[left: ArithmeticOperand,
" ", operator: lexical(Preposition), " ", right: ArithmeticOperand]`,
`require operator.MeasureOperator = Yes`) reads *2 plus the number of …*;
*plus* and *minus* are `vocab:Preposition/Plus` / `Minus` with
`MeasureOperator: Yes`. Glossary: **Degree Modifier**, **Degree Use**,
**Arithmetic Operand**, **Measure Phrase**.

Orchestrator rulings 2026-10-10:

- DQ1. Inside the Determiner: refine `DegreeDeterminativePhrase` (or the
  `QuantitativeDeterminer` layer above it) with an optional post-head
  Modifier: a `MeasureOperator` Preposition with a cardinal Complement
  (*plus one*). The result is still a `QuantitativeDeterminer` with the
  degree head's features and plural agreement. No new Construction.
- DQ2. Delayed: the same PP after the head noun (*that many cards plus one*)
  is a post-head dependent of the Noun Phrase, admitted only when the NP's
  Determiner is a degree Determinative Phrase that could have hosted it
  (read from the Determiner's exported features, e.g. its `DegreeUse`-
  derived kind), so it is the DQ1 Modifier in its delayed position, not a
  general NP postmodifier. One form on an existing NP Construction; if it
  needs a new Construction, land DQ1 alone and STOP with the count.
- DQ3. The licence reads declared features: `MeasureOperator` on the
  Preposition, the degree head on the Determiner, the cardinal's
  `numeral_kind`. No test on *that*, *many*, *plus* or *one*. The
  Complement is a cardinal (spelled or digit) as attested; do not add a
  check to block longer operands.
- DQ4. One analysis per string. The twins to avoid: (a) *that many plus one
  +1/+1 counters* is not `ArithmeticMeasure` over an NP operand, and not
  *that many* (fused head) + a *plus* PP whose Complement is the NP *one
  +1/+1 counters*; (b) *that many cards plus one* does not attach *plus one*
  to the verb or Clause; (c) *2 plus the number of Swamps your opponents
  control* keeps its `ArithmeticMeasure` Readings (2 today); (d) *twice that
  many +1/+1 counters* keeps its Predeterminer Reading.
- DQ5. *the top X plus one cards*, *X minus one 2/2 … tokens* (3 faces): the
  same DQ1 Modifier on a variable Determiner. In scope if the refinement
  sits on the shared `QuantitativeDeterminer` layer; otherwise list them.
- DQ6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face matches either pattern. An increase on a
  covered *that many* face (`(?i)\bthat (many|much)\b`: 590 faces) or a
  covered *plus* / *minus* face is a DQ4 defect; report it with its shape
  before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- Hardened Scales: "If one or more +1/+1 counters would be put on a creature
  you control, that many plus one +1/+1 counters are put on it instead."
  (controls: without "plus one" 2; with "twice that many" 2)
- Conclave Mentor: "If one or more +1/+1 counters would be put on a creature
  you control, that many plus one +1/+1 counters are put on that creature
  instead."
- Kenessos, Priest of Thassa: "If you would scry a number of cards, scry
  that many cards plus one instead."
- Colossus of the Blood Age: "When this creature dies, discard any number of
  cards, then draw that many cards plus one."
- The Water Crystal: "If an opponent would mill one or more cards, they mill
  that many cards plus four instead."

Also 0 today: "that many plus one +1/+1 counters" (`noun-phrase`); "Draw
that many cards plus one." (control "Draw that many cards." 1). Controls
that must not change: "that many +1/+1 counters" (`noun-phrase`) 1; "Its
power is 2 plus the number of Swamps your opponents control." 2. The
without-arithmetic controls for Kenessos, Colossus and The Water Crystal,
and The Water Crystal's witness itself (sole failing line per the blocker
report), were not probed (the trunk moved under the prebuilt binary); probe
them on the claim parent.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bthat (many|much)\b|\b(plus|minus)\b` (748 faces) plus
300 random covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; DQ6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests --
-D warnings`, once, at the end: a rebuild after a grammar edit takes about
2–3 minutes and the six test binaries about 75 seconds, so batch edits.
Tests assert the role (the *plus* PP as Modifier within the Determinative
Phrase; its delayed position as a dependent tied to that Determiner; the
noun still quantified and plural; the DQ4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only. Glossary: extend **Degree Modifier** / add the addition PP (CGEL Ch. 5
§11(f), p. 433) in the landing.

## Out of scope

- *that many plus one of each of those kinds of counters* (5 faces): the
  partitive after a degree Determinative Phrase is unread without the
  arithmetic.
- *one plus the number of …*, *X plus one*
  (`english-v3-spelled-cardinal-scalar-measure`).
- *that many times plus an additional time* (Twinning Staff); *plus an
  additional {1} for each …*; *those tokens plus a … token*.
- *exactly that many colors plus one* (Evolving Door).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
