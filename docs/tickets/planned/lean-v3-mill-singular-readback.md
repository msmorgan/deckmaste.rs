---
needs: []
---
# After `mill (.lit 1)`, the singular readback finds nothing

Standard constraints apply. The cause below is a hypothesis to confirm first; the rest is verified.

## Defect (verified 2026-10-21)

After a one-card mill, every singular readback of the milled card is refused, while the plural
forms and the same readbacks after other one-card moves work:

| Action | Result |
|---|---|
| `sequentially [mill (.lit 1), returnToHand (that card)]` | refused: `anaphor (word card) one` |
| `sequentially [mill (.lit 1), returnToHand it]` | refused: `anaphor bare one` |
| `sequentially [mill (.lit 1), returnToHand (the (and [isCard, was (putInto which yourGraveyard) thisWay]))]` | refused: `participialDescription (zoneChange none graveyard)` |
| `sequentially [mill (.lit 1), returnToHand them]` and `(those card)` | clean |
| `sequentially [putInto (topSlice (.lit 1)) graveyard, returnToHand (that card)]` | clean |
| `sequentially [putInto (topSlice (.lit 1)) graveyard, returnToHand (the (and [isCard, was (putInto which yourGraveyard) thisWay]))]` | clean |

This blocks any singular readback after "mill a card" ([CR#701.17a]: to mill a number of
cards, a player puts that many cards from the top of their library into their graveyard).

## Likely cause

`mill` (`Definitions/Actions.lean:159-163`) moves `.slice .top (.inCaller amount) (libraryOf
.actor)`. The plurality rule for a slice (`Check/Phrase.lean:338-343`) is singular only for
`.slice _ (.lit 1) _`; `.inCaller amount` is not `.lit 1`, so the milled slice is a group
whatever the argument. `putInto (topSlice (.lit 1))` writes `.lit 1` directly and is singular.
The first two rows read no Deed or Zone Change stamp, so this predates the move-deeds landing:
the plurality rule and the `mill` body date from 2026-10-14..19 and that landing (2026-10-21) did
not touch them. The third row may have a second cause in how the Participial Description anchors
on a mill's Zone Change; confirm it separately once the first two are fixed.

## Proposed fix

Compute the milled group's plurality from the argument's value after the Definition's parameter
is substituted, so `mill (.lit 1)` is one card and `mill (.lit 3)` or `mill (letter .x)` are
groups. Do not add a singular-mill Definition: one printed verb, one body. Then settle which
referent shape "the card put into your graveyard this way" reads after a mill.

## Tests to add

- The five refused rows above become positive pins (singular after `mill (.lit 1)`); the plural
  rows stay.
- A refusal pin that the singular readback after `mill (.lit 2)` and `mill (letter .x)` is still
  refused.
- A bench item for "Mill a card. Return the card put into your graveyard this way to your hand."
  (*Blood Spatter Analysis* and *The Last Ronin* print "return target creature card from your
  graveyard" instead, so they are not the pin.)

Gates: `./lean/scripts/build`, `cargo xtask lean check`.
