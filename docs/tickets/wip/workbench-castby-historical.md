---
needs: []
---
# `CastBy` as a historical relation

Ruling 2026-08-22. `badCastInGraveyard` refuses reading "you cast it" of a
card no longer on the stack. Admit it: `CastBy` is a historical relation (a
card in a graveyard that was cast this turn is meaningful; lookback of this
kind exists at [CR#603.10]) rather than a stack-seeding predicate. Widen the
gate, delete the pin, bench a printed line if one exists (`corpus --match
"cast this turn"`). `badCastFromBattlefield` stays: a permanent is an object
[CR#110.1], and casting moves a card "from where it is" [CR#601.2a] — every
other zone (hand, graveyard, exile, library, command) is already an admitted
cast source.

## Consumption boundary
`idris/src/Experimental.idr`, `Words.idr`, `Proofs*.idr`, `Cards.idr`.

## Acceptance
The gate admits; pin gone; build PASS; cites 0/0. Standard constraints apply.
