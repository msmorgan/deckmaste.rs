---
needs: []
---
# "When you lose control of <object>" has no event

Standard constraints apply.

## Problem

`GameEvent` names a control change only from the gaining side (`changeControl`, "gains control
of … from you"). Nothing names a player's losing control of a permanent, which is not the same
event: control returns to the owner when a "gain control" effect ends, and other effects can take
it. *Stolen Uniform*'s last sentence, a delayed triggered ability its resolution creates
[CR#603.7a], needs it: "When you lose control of that Equipment this turn, if it's attached to a
creature you control, unattach it."

Other Vintage-legal cards in `data/derived/cards.jsonl` that print "lose control of": *Duplicity*,
*Khârn the Betrayer*, *Krovikan Vampire*, *Ogre Geargrabber*, *Ray of Command*, *Seraph*.

## Tests

- Extend `stolenUniformOpening` (`Cards/Choice.lean`) to the whole card.
