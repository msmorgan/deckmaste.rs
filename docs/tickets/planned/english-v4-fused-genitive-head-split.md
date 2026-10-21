---
needs: []
---
# A fused-head genitive cannot be followed by the nominal it determines

Owner ruling (cost-audit smaller defect S8): mint this as a defect.

## The defect

Two genitive productions share one right-hand side (`Possessor` + `'s`):

- `GenitivePossessor` (CGEL Type I, a subject-determiner);
- `IndependentGenitivePhrase` (CGEL Type III, a fused subject-determiner-head).

Because they share it, a genitive directly followed by its own head nominal
gets a second analysis:

- the genitive is a complete fused-head Noun Phrase;
- the head nominal becomes a separate Temporal Noun Phrase Adjunct.

Examples:

- *that creature doesn't untap during [its controller's] [next untap step]*:
  *during* takes *its controller's* as its Complement, and *next untap step* is
  an Adjunct of *untap* or *don't*. (A Segment of the corpus sample; the
  template recurs across the corpus.)
- *At [this turn's] [next end of combat], tap each creature …*: *At* takes
  *this turn's*, and *next end of combat* becomes a separate Adjunct of *tap*.

## Authority

- CGEL, Ch. 5 §16.3, pp. 467–468:
  - In Type I (*Kim's father*), the genitive is a subject-determiner inside a
    matrix Noun Phrase.
  - In Type III (*Max's attempt wasn't as good as Kim's*), the genitive fuses
    with the head, which is understood.
- CGEL, Ch. 17 §6.1, pp. 1516–1517:
  - The head of a fused-head Noun Phrase is recovered from an antecedent
    nominal or from the context.
  - The antecedent cannot be discontinuous.
  - When the nominal stands immediately after the genitive, it is not an
    antecedent. It is the head, which is the Type I structure.
- `docs/decisions/english-lexical-analysis.md` §2: "a newly admitted wrong
  Reading is a defect".

## Change

- The fused-head genitive (`IndependentGenitivePhrase`) may not end a
  Constituent that is immediately followed by a Nominal that the genitive could
  determine as a Type I subject-determiner. Express this structurally:
  - either through the `unextended` mechanism against a following `Nominal`;
  - or through an equivalent closure the engine derives.
- Never name the words.
- Glossary: CGEL Ch. 5 §16.3 makes the glossary's *Independent Genitive* the
  pronoun forms (*mine*), and the glossary entry agrees. A noun genitive in
  fused-head use is CGEL's "fused subject-determiner-head". If this landing
  touches the production's name, add that term to
  `docs/contexts/oracle-english/CONTEXT.md` (`domain-modeling` skill) and
  rename the production to match.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:** the 2 Segments above lose 68 Readings, out of a combined
  Surplus of 78.
- **Judgment set:**
  - 2 entries lose 6 Readings, out of a Surplus of 8.
  - No entry loses its endorsed Reading.
- **Corpus reach:**
  - 478 ambiguous units contain a genitive directly before a turn-part or step
    nominal (*'s next untap step*, *this turn's*, *'s upkeep*, …).
  - Their combined count is 3,850 Readings.
  - Expected reduction: most of those units' Surplus that comes from this
    split. The landing measures it.
- **Must keep:** genuine fused-head genitives with an antecedent, for example
  *power less than Mirko's* (the production's own example; its comment cites CGEL Ch. 5 §16.5, p. 471, but p. 471 is §16.4, "Genitive pronouns" — correct the comment if the production is touched).

## Done when

- *during its controller's next untap step* and *At this turn's next end of
  combat* have no Reading with the genitive as a complete Noun Phrase.
- Each has a `+` Judgment on real text.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
