---
needs: []
---
# Inherent triggered abilities of a designation

Standard constraints apply. Found landing `lean-v3-the-monarch-is-unique`.

## Problem

The monarch and the initiative each carry inherent triggered abilities that have no source and are
controlled by the player who held the designation when they triggered, an exception to
[CR#113.8] ([CR#725.2,726.2]). For the monarch: "At the beginning of the monarch's end step, that
player draws a card" and "Whenever a creature deals combat damage to the monarch, its controller
becomes the monarch." `Definition.designation` is only `(label, effectful)`: it has no slot for
abilities, and there is no "no source; controlled by the designee" reading for them.

## Change

Give the designation row a list of inherent abilities, and a way to say that each has no source
and is controlled by the player holding the designation when it triggers. Rules are DSL data (ADR
§0.1 item 4), so these belong on the row, like conferrals on card-type and counter rows. "The
monarch" already names one player (`lean-v3-the-monarch-is-unique`).

## Tests

- Both monarch triggers and the three initiative triggers authored on their rows, each checking
  clean.
- A pin that the abilities name no source.
