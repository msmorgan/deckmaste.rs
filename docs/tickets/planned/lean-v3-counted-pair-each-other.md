---
needs: [lean-v3-reciprocal-each-other]
---
# "Two target creatures … fight each other": a counted pair

Standard constraints apply.

*Rivals' Duel*: "Choose two target creatures that share no creature types. Those creatures fight
each other." `DRS.distributivePair?` (`Check/Words.lean`) finds a pair only in a coordination's
group, whose members are one referent per mention. `targets (exactly 2) isCreature` steps to one
plural referent with no recorded count (`Payload.size` is `none`), so
`sequentially [choose (targets (exactly 2) isCreature), fightEachOther (those creature)]` is
refused `theRestFits .object`.

Change: record the exact count on a counted selection's referent (or otherwise let
`distributivePair?` see that a counted group has exactly two members), then spell *Rivals' Duel*
(its "share no creature types" is `agree (.subtype .creature) .differ …`). Two targets of one
"target" word are distinct objects [CR#115.3], so no self-fight [CR#701.14c] arises here.
Check what the count does to other readers of `Payload.size` (`countedGroupSize`, `theOtherOk`).
