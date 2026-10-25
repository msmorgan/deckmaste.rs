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

## Ruling (owner, 2026-10-24)

Approved.

## Change

- Record `size := some 2`, or the exact count `n` when the `Quantity` is `range (some n) (some n)`
  (`Abilities.lean:449`), on the plural referent introduced by `DetPhrase.target` /
  `DetPhrase.count` (`stepDetPhraseWith`, `Check/Phrase.lean:821-830`).
- That way `targets (exactly 2) isCreature` is recognized by `distributivePair?`
  (`Check/Words.lean:876`) and `theOtherOk` (`:891`) for `fightEachOther`
  (`Definitions/Actions.lean:222`).
- Then spell *Rivals' Duel* (its "share no creature types" is `agree (.subtype .creature) .differ
  …`). Two targets of one "target" word are distinct objects [CR#115.3], so no self-fight
  [CR#701.14c] arises here.
- Check what the count does to other readers of `Payload.size` (`countedGroupSize`,
  `Check/Words.lean:865`, `theOtherOk`).

## Tests

- *Rivals' Duel* spelled in full, checking clean.
- Pin that `targets (exactly 2) isCreature` followed by `fightEachOther (those creature)` is
  accepted, and that a count other than two (`exactly 3`) is still refused for the pair.
