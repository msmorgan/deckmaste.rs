---
needs: []
---
# *N times / twice as much (of) X*: the equative multiplier

The grammar has no equative *as much* or *as many*, so it cannot read a
multiplier over one:

- Nyxbloom Ancient and Virtue of Strength: *If you tap a permanent (a basic
  land) for mana, it produces three times as much of that mana instead.*
- Mana Reflection: *If you tap a permanent for mana, it produces twice as much
  of that mana instead.*

Mana Reflection is unread. Nyxbloom Ancient and Virtue of Strength are counted
as read, but only through one wrong Reading. In it, *three times* is the
Object of *produce* (`Determined`, a `Numerical` determiner over *times*), and
*as much of that mana* is a separate predicative *as* Adjunct.

## Authority

- CGEL, Ch. 5 §12(c), p. 434: multipliers (*twice*, *three times*) are
  predeterminer modifiers. `english-v3-multiplier-predeterminer` (done) relied
  on the same passage.
- The equative *as much* / *as many*, with its omissible *as* Complement, is
  CGEL's comparison of equality (Ch. 13). Cite the exact section and page at
  design time, never from memory.
- `docs/decisions/english-v4.md` §4.4: this is new coverage, so it may add
  the Productions, Categories or values the construction needs. Each must be
  justified by citation and stated generally.

## Warning

`english-v4-stratum-1-cleanup` (done) found that a degree *as* must never
combine with an arbitrary Noun Phrase through `AdverbModifiedNounPhrase`.
Otherwise *as a sorcery* becomes the Object of *activate*. The equative
*as* belongs to the determinative *much* / *many* it grades, not to Noun
Phrases in general.

## Change

- Analyze *three times / twice as much of that mana* as a multiplier over the
  equative determinative *as much*, with its partitive oblique and its *as*
  Complement omitted. The whole Noun Phrase is the Object of *produce*.
- Declare any vocabulary this needs in `deckmaste_lexical_source_v4`. No word
  enumerated by the CR goes in `english.ron`.

## Done when

- Nyxbloom Ancient, Virtue of Strength and Mana Reflection each have a Reading
  in which the multiplier over *as much of that mana* is the Object of
  *produce*, and that Reading is in the cheapest tier. `+` Judgments cover
  each face.
- *Activate only as a sorcery.* gains no Reading.
- No face is lost.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
