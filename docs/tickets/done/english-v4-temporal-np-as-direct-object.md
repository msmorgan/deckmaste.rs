---
needs: []
---
# Temporal Noun Phrases (*this turn*, *each combat*) read as Direct Objects

Deictic and distributive temporal adjuncts (*this turn*, *each turn*, *each
combat*) project as `TemporalNounPhrase` through `TemporalLocation`
(`english.ron`) and attach to predicates through `TemporalAdjunct`. Because
`Determined` (`NounPhrase -> DeterminerPhrase + Nominal`) and
`NonNumericComplement` (`ComplementNounPhrase -> NounPhrase`, which excludes
`Numeric` and `Manner` but not `Temporal`) also admit the same determiner plus
`NounKind = Temporal` nominal, any preceding ambi-transitive verb (*attack*,
*block*, *draw*, *cast*, *play*, *activate*) or transitive head also reads
*this turn* or *each combat* as its Direct Object:

- *Target creature can't block this turn.* has a Reading in which *this turn*
  is the Direct Object of *block*.
- *This creature attacks each combat if able.* has a Reading in which *each
  combat* is the Direct Object of *attacks*.
- Across Corpus Units with at most 64 derivations alone, this overlap produces
  3,855 surplus Readings across 991 units.

## Change

- Prevent deictic/distributive temporal adjunct phrases (*this turn*, *each
  turn*, *each combat*) from doubling as `ComplementNounPhrase` Direct Objects
  of verbs that do not select a temporal object, while keeping genuine temporal
  complements and subjects where Oracle uses them (*skip your next turn*, *take
  an extra turn after this one*, *end the turn*, *when the game begins*, *until
  the end of the turn*).
- Do this through the grammar's categories and features (`TemporalNounPhrase`,
  `NounKind = Temporal`, `DeterminativeDistribution`), never through a flag
  naming individual verbs.

## Done when

- *Target creature can't block this turn.* and *This creature attacks each
  combat if able.* have no Reading in which *this turn* or *each combat* is the
  Direct Object of *block* or *attack*, covered by `+` Judgments.
- The surplus Direct Object Readings across the corpus are gone.
- No face is lost.

Standard constraints apply.

1. **Census delta** (measured on `mowpuuyx`): faces read 28,709 → 28,709;
   total Readings 344,330 → 341,120; lost faces: none.
2. **Judgment set**: 2,929 enforced at `COMPLETED_STRATUM` 6, all hold; 194
   pending.
