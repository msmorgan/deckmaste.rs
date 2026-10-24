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

## Design

A `the` description whose predicate does not make it unique reads back the one compatible
mention in its Reference Scope that the predicate admits, as a Participial Description requires
a unique compatible antecedent. With no such mention, or more than one, it is refused as today.

## Owner ruling (2026-10-24)

- Approved: a `the` description can read back an earlier mention, but only when its predicate
  does not make it unique by itself. `Predicate.uniquifies` (`Check/PredicateShape.lean`) is
  checked first.
- When `pred.uniquifies` holds, it stays a fresh description that makes itself unique, as
  before: superlatives ("the creature with the greatest power"), "the monarch" (a sole
  designation), "the player being attacked", exiled-with, and so on. An earlier creature mention
  never takes over "the creature with the greatest power".
- When it does not ("the creature you control"), the description reads back the one compatible
  mention in its Reference Scope. It is refused when no mention matches or two or more do.
- No new Core constructor: this is checker behaviour on the existing `.described (.the _) p`.

Resolution: a mention matches when its recorded facts say each conjunct of the predicate: a Card
Type its types include, "you control", "you don't control", or "an opponent controls" (which
"you don't control" does not say). A predicate with any other conjunct matches no mention.
Neither does a moved object [CR#400.7], the Source or "you", a hypothetical, or a linked
referent. With no match, or two or more, the refusal is the same `uniquifying` as before.

## Tests

- *Blizzard Brawl*, *Tail Swipe* and *Malamet Battle Glyph* bench items through every sentence
  that does not need `lean-v3-reciprocal-each-other`, beside
  `chooseCreatureYouControlAndCreatureYouDont` in `Cards/Choice.lean`.
- Pins in `Proofs/Description.lean`: the readback after the coordinated choice and after one
  target, the refusal with no matching mention and with two, and a superlative that describes
  afresh with a creature mention in scope.

Carried elsewhere:

- *Joust*: its postposed "if it's a Knight" cannot read the branch's subject yet
  (`lean-v3-postposed-doif-condition`).
- *Beastie Beatdown*: its later paragraphs are separate spell abilities [CR#113.2c] that cannot
  read the first paragraph's targets yet (`lean-v3-spell-abilities-share-targets`).
