---
needs: []
---
# *The next time …, instead …*: *instead* before the clause it modifies

## Witnesses

- **Ring of Ma'rûf:** *{5}, {T}, Exile this artifact: The next time you would
  draw a card this turn, instead put a card you own from outside the game into
  your hand.* (*instead* before an imperative)
- **Mangara's Tome:** *{2}: The next time you would draw a card this turn,
  instead put the top card of the exiled pile into its owner's hand.*
  (*instead* before an imperative)
- **Eye for an Eye:** *The next time a source of your choice would deal damage
  to you this turn, instead that source deals that much damage to you and Eye
  for an Eye deals that much damage to that source's controller.* (*instead*
  before a finite declarative, ahead of its Subject)

All three are Stratum 7 and have no Reading on `lxzvuwqu`
(`english-v4-bare-relative-fronted-adjunct`). Before it, every Reading of each
was wrong: the comma-set *, instead* closed the adverbial relative (*time [you
would draw a card this turn, instead]*), and the whole temporal Noun Phrase was
then the Subject of a preterite *put*, or ran into *that source deals*. A bare
relative's Clause now opens with its Subject and admits no comma-set
post-adjunct (CGEL, Ch. 12 §3.5.5, p. 1055; Ch. 12 §4, p. 1058), so those
Readings are gone. Even *The next time you draw a card this turn, instead put
a card into your hand.* has no Reading.

The two kinds are one problem, so one ticket: *instead* is an unpunctuated
preposed Adjunct of the Clause after the preposed temporal Noun Phrase, whether
that Clause is an imperative or a declarative with its Subject.

## Intended analysis

[*The next time …*] is a preposed temporal Noun Phrase Adjunct, with its
adverbial relative clause, set off by the comma. *Instead*, an intransitive
Preposition Phrase, is a preposed Adjunct of the following Clause without a
comma: *instead [put …]*, *instead [that source deals …]*.

## Change

- Admit an intransitive Preposition Phrase as an unpunctuated preposed Adjunct
  of a Clause, so that it covers both the imperative and the declarative.
  - Distinguish it by its existing `ComplementKind` (`Intransitive`), not by
    the word.
  - A pre-`VerbPhrase` site would miss Eye for an Eye, where *instead* comes
    before the Subject.
- Two items of the earlier plan are no longer needed:
  - Excluding intransitive prepositions from the comma-set post-`VerbPhrase`
    Adjunct. That schema is gone (`english-v4-comma-adjunct-island`), and the
    adverbial relative no longer takes *, instead*.
  - The `NominalPostmodifier` exclusion through `RelativeKind`. That feature
    does not exist, and adding it would break the no-new-features rule. The
    wrong Subject Readings it targeted are gone.
- Measure what else the new site admits. Each of its Readings on real text must
  be endorsed.

## Done when

- Ring of Ma'rûf, Mangara's Tome's activated ability and Eye for an Eye read,
  with the temporal Noun Phrase as an Adjunct of the main Clause and
  `PrepositionPhrase "instead"` as an Adjunct of *put* or *deal*. `+` judgments
  assert it.
- `english-v4-case-solve-condition` can exclude the preterite main clause
  without losing them.
- **No new features in `english.ron`.** If the fix needs one, STOP and return
  to the owner.
- No face is lost, and no unit gains a Reading other than the witnesses'.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
