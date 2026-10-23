---
needs: []
---
# "The monarch" names exactly one player

Standard constraints apply. Found landing `lean-v3-damage-doesnt-cause-life-loss`.

## Problem

`the (and [isPlayer, hasDesignation monarch])` is refused `uniquifying`, but only one player can
be the monarch at a time [CR#725.3]. The general monarch triggers [CR#725.2] say "the monarch", so
they cannot be authored today. *Archon of Coronation*'s witness avoided it by using its reminder
sentence, which says "you".

## Change

Let a designation that only one player can hold at a time make a `the` description unique, in
`Predicate.uniquifies` (`Check/PredicateShape.lean`). Decide where that fact lives: the
designation's own declaration (`Definitions/Designations.lean`, now self-contained) is the natural
home, as a field or a `Definition.designation` row. The initiative [CR#726.3] is the other such
designation; check the CR for any more. If no monarch exists, a static reading "the monarch" does
nothing [CR#725.5]; that is the engine's, not the checker's.

## Tests

- `okTheMonarch`: "the monarch draws a card" checks clean.
- A designation that several players can hold at once still leaves `the` refused.
- Author the two [CR#725.2] triggers if they fit a Definition row.
