---
needs: []
---
# Present-only clause environments for syncretic verbs

Each of *cast*, *put*, *cost* and *bid* has a present and a preterite Word Form
spelled alike. Every clause they head is read twice, once in each tense, unless
something selects the tense. Across the corpus that comes to 25,409 surplus
Readings, measured in `english-v4-byte-identical-duplicate-readings` (*cast*
19,524, *put* 4,983, *cost* 896, *bid* 4). Animar shows the effect.
*Whenever you cast a creature spell, …* has a preterite Reading. *Creature
spells you cast cost {1} less …* has four tense combinations for each
attachment of *for each …*.

In some Oracle environments the clause is always present tense. A preterite
there is a wrong Reading:

- the finite clause after *whenever*, *while*, *as long as*, *until*, plain *as*
  (*as this creature enters*, *as you cast*; not counterfactual *as though*),
  and supplementary *where* (*, where X is …*);
- the clause of a `ChoiceHeader`;
- a non-modal matrix `Declarative` clause, and the clause after a trigger
  *when*.

## Change

- Require `Inflection` `Present` on the finite clause in those environments,
  by Production or Schema, never by a word-specific flag.
- Keep the preterite where it is genuine. *If an opponent cast a red instant
  or sorcery spell this turn* and *if you attacked this turn* read in the
  preterite, and so does a relative clause (*a creature that died this turn*).

## Done when

- In the listed environments, *cast*, *put*, *cost* and *bid* have no
  Preterite Reading. The Census records the drop in Readings.
- Wherever the preterite is genuine, it still reads.
- No face is lost.

Standard constraints apply.

## Record

The corpus narrowed the environments. *Whenever*, *while*, *until* and
supplementary *where* select a present-tense Clause (`PresentClause` frame,
`PresentComplementClause`). The others take genuine preterites and are
ticketed instead: *as long as* and *as* (*as long as it entered this turn*,
*the life paid as it entered*), a non-trigger *when* (*the life you lost when
it entered*) in `english-v4-temporal-adjunct-tense-harmony`, and the matrix
`Declarative` (*To solve — Three or more creatures attacked this turn.*) in
`english-v4-case-solve-condition`, after `english-v4-next-time-replacement-clause`.
`ChoiceHeader` has no preterite Reading on any card, so it gets no constraint.
Coordinated conjuncts in a present-only Complement remain free:
`english-v4-coordinated-clause-tense`.

Census delta (measured on orrqzqyv): faces read 28,709 → 28,709; total Readings 344,330 → 337,377; lost faces: none.
Judgment set: 2,924 enforced at `COMPLETED_STRATUM` 6, all hold; 192 pending.
