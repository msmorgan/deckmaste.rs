---
needs: []
---
# Tense of a *when* or *as* clause follows the clause it modifies

`english-v4-clause-tense-selection` gave *whenever*, *while*, *until* and
supplementary *where* a present-tense Clause Complement (`PresentClause`).
*When* and plain *as* could not join them: a trigger's clause is always
present (*When you cast this spell, …*; *As this creature enters, …*), but
the same prepositions take a genuine preterite after a preterite or
past-participial host:

- *you gain life equal to the life you lost when it entered* (Soulgorger Orgg);
- *the life paid as it entered* (Minion of the Wastes, Nameless Race,
  Phyrexian Processor);
- *if you were the monarch as the turn began* (Knights of the Black Rose).

Giving *when* the present-only frame removed about 3,700 Readings and lost
Soulgorger Orgg. So the rule is not lexical: a temporal *when*/*as* clause
takes the tense of the clause or Verb Phrase it modifies (present or plain
host: present; preterite or past-participial host: preterite).

## Change

- Express that agreement between a temporal adjunct's Clause and its host
  through features (the Preposition Phrase carrying its Clause's Inflection,
  constrained by the Adjunct Schema), never by naming verbs.
- *Sengir, the Dark Baron*'s *that player's life total as the turn began*
  currently parses only by attaching *as the turn began* to present-tense
  *gain* (because *as* has `hosts: Predicate` only); allow a preterite *as*
  temporal PP to postmodify a nominal (as in *that player's life total as the
  turn began* and *cards in your hand as the turn began* on Jalum Grimoire) so
  it is not lost when predicate-attached *as* requires tense harmony.

## Done when

- A trigger *when*/*as* clause has no preterite *cast*/*put*/*cost*/*bid*
  Reading; the faces above still read with their preterite.
- No face is lost.

Standard constraints apply.
