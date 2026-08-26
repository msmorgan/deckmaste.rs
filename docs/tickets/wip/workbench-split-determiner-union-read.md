---
needs: [workbench-join-shape-flat-or-pair]
---
# The split-determiner union read — 25 occurrences over 24 cards

"That player or that planeswalker's controller" (Blightning, Searing Blaze,
Rakdos's Return, Chandra Nalaar, Bonfire of the Damned, Lavalanche, Soul of
Shandalar ×2, and sixteen more) names **each half** of a union antecedent
instead of echoing it whole. Two cards write both spellings in one text (Searing
Blaze, Quenchable Fire), which is what says the whole-echo and the split read
are one phenomenon rather than two.

This entry was carried in the workbench's round queue and it is **not** closed by
the round that landed the union anaphor. It survives the direction call, and the
direction changes its shape: what it needs is exactly the pair the join may or
may not carry.

Authority for the layer split:
[The kind index joins; union marking is spelling](../../decisions/kind-index-joins-union-marking-is-spelling.md).

The neighbouring measurement, which this entry sits beside: of the 48 supported
anaphor readbacks, 33 echo their antecedent's kind pair exactly and 15 with no
pair write the generic "permanent or player"; `Payload.UnionP` carries nothing,
so no half is reachable today.

## What it needs

1. **The halves' own word access** — a demonstrative reaching one half of a
   union mention, which today's payload deliberately refuses for want of
   evidence (no card reads a half alone). Keeping "that planeswalker's
   controller" from following a permanent-headed union wants the class half
   recorded, which is the pair-carrying question `workbench-join-shape-flat-or-pair`
   settles; take that decision as given, do not re-open it.
2. **A same-kind noun disjunction of two player nouns** — a fourth coordination
   beside the three the queue already records, and the only genuinely new
   construction here.

Measured shape, and the two halves take **different** gates: the class half
echoes its antecedent (planeswalker 21, permanent 4); the player half does not
(24 write "player", 1 "opponent", and Rakdos's Return writes "that player" over
an antecedent that said "target opponent"). The controller reader is not a
blocker — it takes an ordinary object noun and composes the moment (1) lands.

## Consumption boundary

`idris/src/Experimental/Words.idr` (the payload and the demonstrative's words),
`idris/src/Experimental.idr` (the noun coordination), evidence bench
`idris/src/Experimental/Cards.idr`.

## Acceptance

- All 25 occurrences spell, with the two halves' asymmetry reproduced rather
  than averaged into one gate.
- The two cards writing both spellings elaborate under one construction.
- `idris/scripts/build` PASS, no witness lost, no pin silently passing.

Standard constraints apply.
