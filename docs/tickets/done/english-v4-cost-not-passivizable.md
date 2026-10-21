---
needs: []
---
# *Cost* and *have* do not passivize

For every frame whose first slot is an NP, `compile.rs` (around lines
1634–1690) emits `…/passive` and `…/passive+by` valence productions, plus
Adjectival Passives.
- *cost* (`verbs.ron`: `Monotransitive`,
  `TransitiveWithComplement(HollowToInf)`) therefore gets a passive
  participle.
- That participle postmodifies nouns. Magus Lucea Kane's *its activation
  cost* also parses as *its activation [cost (this turn)]*.
- Lexical *have* (`closed_class.ron`: `Monotransitive`,
  `Transitive(KeywordPhrase)`, …) gets the same unwanted passive.

## Owner's ruling

*Cost* and *have* are non-passivizable transitive verbs: CGEL, Ch. 16
§10.1.2(a), p. 1432, example [17] (*A packet of cigarettes costs around seven
dollars*). The round-1 ruling cited §5.1; the passage is §10.1.2.

Declare this as a frame property in the lexicon, so that `compile.rs` emits
no `/passive` production from those frames.

**No new features in `english.ron`.** If the fix cannot meet that, STOP and
return to the owner.

## Change

- Add an active-only property to the frame declaration in the lexicon. It
  sits beside the frames of *cost* in `verbs.ron` and of *have* in
  `closed_class.ron`, where *have* is declared.
- `compile.rs` skips the passive, passive-with-*by*, prepositional-passive and
  Adjectival Passive derivations for such a frame.
- The perfect auxiliary *have* is a grammar production
  (`Perfect`/`PassivePerfect`) and is unaffected.
- No word is named in the grammar or the engine.

## Expected reductions

About 1,400 Surplus over 13 units, none emptied:
- Magus Lucea Kane 1,192;
- Protective Sphere 146;
- Unbound Flourishing 32;
- Artifact Possession 9;
- Heirloom Epic 6.

*Have*'s share was not simulated; the landing measures it.

## Done when

- No Reading has a past participle of the verbs *cost* or *have* other than
  the perfect auxiliary.
- *mana cost*, *activation cost*, *costs {1} less to cast*, *has flying* and
  *have target player …* keep their Readings.
- Add a judgment for Magus Lucea Kane's sentence asserting
  `!VerbPhrase "cost"`.
- No new feature in `english.ron`.
- 0 faces lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta** (measured on `mxukolkx`): faces read 28,709 → 28,709;
   total Readings 327,929 → 325,187 (Surplus 175,304 → 173,822, over 19
   units, none rising); lost faces: none.
2. **Judgment set**: 2,977 enforced at `COMPLETED_STRATUM` 6, all hold; 195
   pending.
