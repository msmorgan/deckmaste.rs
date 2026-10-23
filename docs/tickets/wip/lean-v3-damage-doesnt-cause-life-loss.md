---
needs: []
---
# "Damage doesn't cause you to lose life" has no form

Standard constraints apply. Found landing `lean-v3-damage-composition`, which left the card
unauthored.

*Archon of Coronation*: "As long as you're the monarch, damage doesn't cause you to lose life."

## Problem

Damage dealt to a player normally causes that player to lose that much life [CR#119.2,120.3a]; the
life loss is a result of the damage, processed after the damage is dealt [CR#120.4c]. The card
stops that result and nothing else: the damage is still dealt, so combat damage still makes its
creature's controller the monarch, damage still triggers "whenever you're dealt damage", and
lifelink, infect and toxic results of the same damage still happen [CR#120.3b,120.3f,120.3g]. Only
the loss of life goes.

Neither form the damage-composition ruling provides expresses that today:

- `replace` needs an action to put in place of the event: `instead (X)` [CR#614.1a], or `skip`
  [CR#614.1b]. Replacing the loss with nothing is `skip`-shaped, which is wrong for a result that
  is simply not caused. A replacement with no action is a form that does not exist.
- `cant` over `loseLife` would stop every loss of life, not only one damage causes: "you can't lose
  life" is a different card and a different rule [CR#119.8]. A prohibition qualified by cause,
  `cant you (loseLife …)` restricted to a loss that damage causes, does not exist. `causedBy`
  exists on `replace`'s would-clause, not on `cant`'s action.

## Candidate forms (proposed, not ruled)

1. A cause-qualified prohibition: `cant`'s action takes the same `causedBy` a would-clause does, so
   the card reads `static (cant you (loseLife anyAmount) (causedBy (damage …)))` under the
   monarch condition. Changes `Action.cant` or its action argument; smallest change to the
   composition ruling's vocabulary.
2. A replacement whose replacement is no action: `WouldClause` `loseLife`, `causedBy` damage, and
   an `instead` slot that may be empty. Changes `replace`; needs a ruling on whether it differs
   from `skip` [CR#614.1b] and whether a replacement with no action can be written for other cards.
3. The result as a thing damage does: the rows for damage results (`asEvent`) gain a negative form
   (damage does not cause this result). Largest; reopens the composition ruling.

Decide which, state it in Core, author the card, and add a witness that also checks the damage's
other results (the monarch trigger, lifelink) are untouched. *Soul-Scar Mage* is only a neighbour:
it replaces noncombat damage to an opponent with counters, a replacement with an action, and is
not this ticket's.

## Ruling (owner, 2026-10-23)

Option 1: a cause-qualified prohibition. `cant you (loseLife anyAmount)`, restricted to a loss
that damage causes, reusing the existing `causedBy` [CR#119.8,120.3a]. The prohibition stops only
the result; the damage is still dealt, and its other results are untouched [CR#120.3b,120.3f,120.3g].

Not a replacement: the text has no "instead", and a replacement would compete with other
replacement and prevention effects under [CR#616.1]. Options 2 and 3 are rejected.

## Change

- `cant`'s action takes `causedBy` as a would-clause does, so *Archon of Coronation* reads
  `static (cant you (loseLife anyAmount) (causedBy (damage …)))` under the monarch condition.
- Author *Archon of Coronation* in `lean/Semantics/Cards/Damage.lean`.
- Update the closure tables and `lean/CONTRACTS.md` for the new `cant` argument.

## Tests

- `okArchonOfCoronation`: the card checks clean.
- A witness that the damage's other results are untouched: the monarch trigger from combat damage
  and lifelink still read their damage; only the loss of life is prohibited.
- A negative: `cant you (loseLife anyAmount)` with no cause still denotes "you can't lose life"
  [CR#119.8], a different card.
