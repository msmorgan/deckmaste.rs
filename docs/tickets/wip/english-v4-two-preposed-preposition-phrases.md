---
needs: [english-v4-comma-adjunct-island]
---
# Two preposed Preposition Phrases set off by one comma

Witness, Cyclopean Tomb:

> When this artifact is put into a graveyard from the battlefield, at the
> beginning of each of your upkeeps for the rest of the game, remove all mire
> counters from a land that a mire counter was put onto with this artifact but
> that a mire counter has not been removed from with this artifact.

## The defect

The intended analysis has no Production.
- The trigger creates a delayed triggered ability [CR#603.1,603.7] whose
  timing phrase begins its Clause (`docs/oracle-style-guide.md` §8, Delayed
  and reflexive triggers: "A timing clause may also begin the sentence").
- *At the beginning of each of your upkeeps* and *for the rest of the game*
  are both preposed Adjuncts of *remove*, set off by one comma. *For the rest
  of the game* is the game-long duration (§11, Continuous and bounded
  durations).
- Nothing in the grammar fronts two Preposition Phrases before one comma.

On `pzvrlpou`, the line has 336 Derivations, none of them the intended
analysis. Under the `english-v4-comma-adjunct-island` candidate it has 0, so
the face goes unread when that ticket lands (round 3, item 3.2). If
`deferred.txt` names the unit, this landing removes that entry.

## The trap

A general [PrepositionPhrase, PrepositionPhrase, `,`, Clause] Production would
also split fixed phrases into two Adjuncts:
- *At the beginning of combat on your turn,* (342 Oracle records; §11 calls it
  a fixed phrase);
- similar phrases with the same shape.

That raises hundreds of units with a spurious Reading. Do not land it.

## Candidates

The first Preposition Phrase must not end where the second could attach
inside it, by the same right-edge closure as candidate 2 in
`english-v4-fused-genitive-head-split`'s `PreposedPrepositionAndTemporal`
section:
- *for + NP* is Predicate-hosted only (`docs/decisions/english-v4.md` §4.2;
  `prepositions.ron`). It cannot postmodify *upkeeps*, so the two-Adjunct
  analysis is the only one.
- *on + NP* may postmodify a Nominal (`NominalAndPredicateConjunct`), so
  *on your turn* attaches inside *combat*, and the closure blocks the split.

If no closure avoids the blow-up without a new feature, STOP and return the
options to the owner.

## Done when

- Cyclopean Tomb reads, with a `+` Judgment: both Preposition Phrases are
  Adjuncts of *remove*.
- A `+` Judgment on *At the beginning of combat on your turn, put a +1/+1
  counter on target creature.* has
  `!PrepositionPhrase "on your turn" as Adjunct of put`.
- No new feature in `english.ron`. If the fix needs one, STOP and return to
  the owner.
- No face is lost, and no unit other than the witness gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
