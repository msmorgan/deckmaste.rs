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
  *produce*. `+` Judgments cover each face. Where that Reading ranks is
  reported, not required (round 9: `docs/decisions/english-v4.md` §4.4,
  item 6).
- *Activate only as a sorcery.* gains no Reading.

## Acceptance (round 9: `docs/decisions/english-v4.md` §4.4, item 9)

- No face is lost.
- Every judgment's positive assertions hold.
- Nothing ungrammatical is admitted.
- `cargo xtask english-v4 strata --check` passes.
- No pathological forest explosion on real cards.

The landing reports the cheapest-tier figures and the judgment-set S/A/P/F,
but they are not gates (item 7). No Attachment Class, Production Cost tweak or
residual ticket is added to win a rank (item 8).

Standard constraints apply.

1. **Census delta** (`mlrkosuz`): faces read 28,801 → 28,802; total Readings 245,781 → 245,784; Surplus 106,817 → 106,819; lost faces: none. Exactly three units change: Virtue of Strength (Line 0) 1 → 2, Nyxbloom Ancient (Line 1) 1 → 2, Mana Reflection (Line 0) 0 → 1 (Unread → Read). Each gains the intended Object Reading, ranked first; every other unit's Derivations and verdict are unchanged. Cheapest tier: singleton in 15,624 of 18,344 ambiguous units → 15,626 of 18,346; summed 21,723 → 21,725; largest 8 → 8. `strata --check` passes.
2. **Judgment set** (`mlrkosuz`): both changed crates' tests and Clippy pass. Added eight judgments in `equative_multiplier.txt`, enforced or pending by their Stratum (the three face entries are above the completed Stratum and so pending): the three face Objects, count and mass multiplier phrases, two Countability controls, and the unchanged single Reading of *Activate only as a sorcery*. No judgments or tests rewritten or retired. Degree *as* and EquativeDeterminer cite CGEL Ch. 13 §§1.2, 4.1.2, 4.3–4.4.1, pp. 1102, 1126, 1130–1131; existing multipliers and partitives supply the surrounding structure. S/A/P/F is unavailable: this tree has no tally tool. Citation checks: no noncompliant entries, 0 stale; no new CR citations.
