---
needs: []
---
# *Cost* has no passive

`verbs.ron` gives *cost* the frames `Monotransitive` and
`TransitiveWithComplement(HollowToInf)`. The engine derives a passive from
every frame, so past-participial *cost* postmodifies nouns. In Magus Lucea
Kane, *its activation cost* also parses as *its activation [cost (this
turn)]*, where *cost* is a passive participle of the verb.

## Owner's ruling

*Cost* is an active-only verb and does not passivize. CGEL, Ch. 16 §10.1.2(a),
p. 1432, example [17]: *A packet of cigarettes costs around seven dollars*,
among verbs that "do not occur in" the passive.

The ruling cited Ch. 16 §5.1. The passage is in §10.1.2 ("Complements
externalised from the passive VP"), on p. 1432 as ruled.

## Change

- Let a verb frame in the lexicon declare that it is active-only. The engine
  then derives no passive, gapped passive or adjectival-passive productions
  from it.
- Declare that on *cost*'s frames in `verbs.ron`.
- No word is named in the grammar or the engine.
- Other active-only verbs in CGEL's list are out of scope until the corpus
  needs them.

## Expected reductions

About 1,400 Surplus over 13 units, none emptied:
- Magus Lucea Kane 1,192;
- Protective Sphere 146;
- Unbound Flourishing 32;
- Artifact Possession 9;
- Heirloom Epic 6.

## Done when

- No Reading has *cost* as a past participle of the verb.
- *mana cost*, *activation cost* and *costs {1} less to cast* keep their
  Readings.
- Add a judgment for Magus Lucea Kane's sentence asserting
  `!VerbPhrase "cost"`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
