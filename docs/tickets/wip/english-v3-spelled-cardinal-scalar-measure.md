---
needs: []
---
# Read a spelled cardinal as a scalar operand and comparison standard: where X is one plus the number of Foods you control

## Why

A numeral written in digits is a scalar Measure Phrase; the same numeral
spelled out is not, so arithmetic and comparison fail on it. Blocker report
at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): keys `at:one
plus` 10 and `at:is less` 21 sole-blocker faces. Probes: "Its power is less
than 5." 1, "Its power is less than five." 0; "… where X is 1 plus the number
of opponents who control an artifact." 1, "… one plus …" 0;
`arithmetic-operand` "1" 1, "one" 0; `scalar-measure-phrase` "5" 1, "five" 0.
The spelled cardinal does read alone as a value ("Its power is five." 1,
"Its power is five or greater." 1).

`cargo xtask english-v3 grep -i` counts, all unread:

| Shape | Pattern | Faces | Sole failing line |
|---|---|---|---|
| *one plus the number of …* | `\b(one\|two\|three\|four\|five) (plus\|minus)\b` | 17 | 16 |
| *is less than five / seven* | `\b(is\|are) (less\|greater) than (one\|…\|ten)\b` | 22 | 21 |
| *to be greater than four* | (Clockwork Steed, Clockwork Avian) | 2 | 2 |
| *<scalar NP> plus / minus one* | (Sea Gate Restoration, Kyren Toy, Proft's Eidetic Memory) | 3 | 3 |

The 22 *is less than five* faces are the devotion Gods and also need
`english-v3-devotion-to-colour`; this ticket alone is expected to cover about
20 faces.

## Goal

A spelled cardinal fills the two scalar positions a digit numeral fills:
Arithmetic Operand (*one plus the number of …*, *… plus one*) and the
standard of a scalar comparison (*less than five*, *greater than four*),
with byte-exact realisation.

## Analysis

No CGEL citation is relied on: that a cardinal numeral denotes a number is
not in dispute, and which positions Oracle English fills with the spelled
form is a project ruling. (CGEL Ch. 7 §5.1, p. 635, [1i] lists *plus* and
*minus* among the prepositions; that is already the grammar's analysis.)

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`UngroupedScalarNumeral` (`MeasurePhrase`, `require head.numeral_kind =
Arabic`) feeds `ScalarMeasurePhrase`, which is what
`ScalarComparativeComplement` (`[value: ScalarMeasurePhrase]`) and
`ScalarMeasureOperand` (`ArithmeticOperand`, `[value: MeasurePhrase]`) take;
`ArithmeticMeasure` joins two operands with a `MeasureOperator` Preposition.
The spelled cardinal (`numeral_kind = Cardinal`) reads through
`ScalarCardinalValue` (`ScalarValue`), `CardinalAmount` and the `Cardinal`
category. The house pattern for "digit or spelled" is a sibling pair:
`ScalarExtentMeasure` / `CardinalExtentMeasure`, `ScalarAmount` /
`CardinalAmount`. Glossary: **Arithmetic Operand**, **Comparative
Complement**, **Scalar Value**.

Orchestrator rulings 2026-10-10:

- SC1. Follow the sibling pattern at exactly two sites: `ArithmeticOperand`
  and `ComparativeComplement` each admit a spelled cardinal
  (`numeral_kind = Cardinal`). Prefer a second form on the existing
  Construction (`ScalarMeasureOperand`, `ScalarComparativeComplement`); a
  sibling Construction per site, as `CardinalExtentMeasure`, is the
  permitted fallback. Do not widen `UngroupedScalarNumeral` or
  `MeasurePhrase` itself: that would put a spelled cardinal wherever a
  measure reads (*three damage*, *five life*) and twin every cardinal
  Determiner.
- SC2. The licence is the Numeral's declared `numeral_kind`; no list of
  number words.
- SC3. One analysis per string. The twins to avoid: (a) *Its power is five.*
  and *five or greater* keep exactly their Readings (`ScalarCardinalValue`
  and `AdjectivalScalarCoordination` stay the only routes; the new operand
  is not a `ScalarValue` on its own); (b) *more than one creature*, *fewer
  than seven cards* keep the Comparative Quantity Determiner Reading only
  (no comparative Adjective Phrase + noun twin); (c) *one plus the number of
  Foods you control* has the cardinal as left operand, not a Determiner of a
  fused head followed by a *plus* PP.
- SC4. Both operand orders are in scope (*one plus N*, *N plus one*, *N
  minus one*) when the other operand already reads as an Arithmetic Operand.
  Arithmetic inside a Determiner (*that many plus one counters*, *the top X
  plus one cards*) is `english-v3-degree-quantity-arithmetic`.
- SC5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected; an increase on a face with a spelled cardinal after
  *than* means SC3(b) leaked. Report any increase with its shape before
  landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Journey On: "Create X Map tokens, where X is one plus the number of
  opponents who control an artifact." (control with "1" 1; without "one
  plus" 1)
- Welcome to Sweettooth: "III — Put X +1/+1 counters on target creature you
  control, where X is one plus the number of Foods you control." (sentence
  without "one plus" 2)
- Boss's Chauffeur: "This creature enters with a number of +1/+1 counters on
  it equal to one plus the number of other creatures you control." (without
  "one plus" 3+)
- Mind Burst: "Target player discards X cards, where X is one plus the
  number of cards named Mind Burst in all graveyards." (without "one plus"
  3+)
- Clockwork Steed: "… This ability can't cause the total number of +1/+0
  counters on this creature to be greater than four. …" (may have a second
  cause; report)

Joint with the devotion ticket, 0 today: "Its power is less than five."
(control "… less than 5." 1); "Your life total is less than seven."
(control "Your life total is less than 5." 1).
Controls that must not change: "Its power is five." 1; "Its power is five or
greater." 1; "Your life total is seven or greater." 1; "If you have fewer
than seven cards in hand, draw a card." 2; "You can't cast more than one
spell each turn." 2; "Its power is 2 plus the number of Swamps your
opponents control." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(plus|minus)\b|\b(less|greater) than\b` (485 faces) or
`(?i)\b(than|plus|minus|equal to) (one|two|three|four|five|six|seven|eight|nine|ten)\b`
(part of 189 faces), plus 300 random covered controls (`--random 300 --seed
20261010`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; SC5 for increases.
Full `cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings`, once, at the end: a rebuild
after a grammar edit takes about 2–3 minutes and the six test binaries about
75 seconds, so batch edits. Tests assert the role (the spelled cardinal as
Arithmetic Operand; as the Complement of the comparative governor; the SC3
negatives), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *devotion to <colour>* (`english-v3-devotion-to-colour`).
- *that many plus one counters*, *that many cards plus one*, *X plus one
  cards* (`english-v3-degree-quantity-arithmetic`).
- *less than three mana*, *less than one mana* (Trinisphere, Convergence of
  Dominion): a spelled measured amount after *than*.
- *plus an additional …*, *those tokens plus a … token* (about 12 faces).

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
