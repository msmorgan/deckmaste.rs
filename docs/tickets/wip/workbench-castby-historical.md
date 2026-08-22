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

## As landed (2026-08-22)

`seedZone (CastBy _)` now returns `Nothing` instead of `Just Stack`
(`idris/src/Experimental.idr`). Casting is history, not a location:
[CR#400.7d] lets a permanent's ability reference the spell it was cast as, and
[CR#702.40a] counts spells cast before it this turn, which have long left the
stack. `CastBy` therefore composes with any `InZone`, and defaults nothing on
its own; every existing `CastBy` site in `Cards.idr`/`ProofsE.idr` pairs it
with `Macros.spell` (= `InZone stackZ`), so no Stack seed was lost.

`badCastInGraveyard` deleted from `ProofsD.idr`. `badCastFromBattlefield`
(`ProofsG.idr`) untouched — a different constructor (`CastFrom`) and a
different question.

Bench: `cycleOfLife` (`Cards.idr`) — Cycle of Life, "Target creature you
cast this turn has base power and toughness 0/1 until your next upkeep." The
subject is a battlefield permanent, so this printed line is the off-stack
`CastBy` witness. It is the only corpus hit under
`corpus --match "cast this turn|you cast this turn|was cast"` with an
off-stack subject *and* a named caster; the other off-stack hit,
"protection from spells and from permanents that were cast this turn", names
no caster and so cannot use `CastBy`'s required `Noun bs Player`.

No corpus line was found for the graveyard reading itself; the widening is
carried by the rules, not by a count.

Gates: build 18/18 exit 0; Cards.idr implicits 0; `cite check` 0 stale,
0 non-compliant ([CR#400.7d] newly blessed, audit read).
