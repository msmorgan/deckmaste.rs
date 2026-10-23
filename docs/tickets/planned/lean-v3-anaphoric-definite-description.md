---
needs: [lean-v3-choose-over-coordination]
---
# A restricted "the <description>" reads back a mention in scope

Standard constraints apply.

## Problem (probed 2026-10-23)

`the p` (`.described (.the _) p`) is always a new selection, and the checker refuses it with
`uniquifying` unless `p` alone makes it unique (`Predicate.uniquifies`,
`Check/PredicateShape.lean`; the refusal is in the description arm of `stepNounPhrase`,
`Check/Phrase.lean`). Oracle text also writes a definite description as an Anaphor: after
"Choose target creature you control and target creature you don't control", "the creature you
control" is the first target, picked out of the two by its restriction. The targets were chosen
as the spell was cast [CR#601.2c]; the description chooses nothing.

Probe (scratch, on `lean-v3-choose-over-coordination`):

```lean
sequentially [choose (both (target creatureYouControl) (target creatureYouDontControl)),
  get (the creatureYouControl) (up 2) (up 1) untilEndOfTurn]       -- [uniquifying]
sequentially [choose (target creatureYouControl), tap (the creatureYouControl)]  -- [uniquifying]
```

The second line shows that the gap does not depend on coordination.

## Witnesses

- "the creature you control": *Blizzard Brawl*, *Joust*, *Tail Swipe*, *Malamet Battle Glyph*,
  *Beastie Beatdown*.
- "the creature an opponent controls": *Beastie Beatdown* ("The creature you control deals
  damage equal to its power to the creature an opponent controls").

*Stolen Uniform* writes "the chosen creature", which is `lean-v3-chosen-participial-anchor`.

## Design (proposed)

A `the` description whose predicate does not make it unique reads back the one compatible
mention in its Reference Scope that the predicate admits, as a Participial Description requires
a unique compatible antecedent. With no such mention, or more than one, it is refused as today.
The open question is which of the two readings a `the` description gets when both are possible.

## Tests

- *Joust*, *Tail Swipe* and *Beastie Beatdown* bench items through every sentence that does not
  need `lean-v3-reciprocal-each-other`, extending
  `chooseCreatureYouControlAndCreatureYouDont` in `Cards/Choice.lean`.
- A refusal pin: "the creature you control" with two compatible mentions in scope.
