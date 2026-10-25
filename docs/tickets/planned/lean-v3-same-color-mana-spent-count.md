---
needs: [lean-v3-colored-mana-spent-count]
---
# "Three mana of the same color was spent to cast this spell" (proposed)

Standard constraints apply. Split from `lean-v3-colored-mana-spent-count` (owner ruling
2026-10-24), which covers a stated type of mana through `LinkedFacet.manaSpent (type :
ManaTypeTerm)` [CR#106.1b]. This ticket is proposed, not ruled.

## Problem

*Clockwork Servant* and *Henge Walker* print adamant-style conditions on "three mana of the same
color". No color is stated, so `manaSpent` with a `ManaTypeTerm` cannot name it. The condition
quantifies over colors: some color has at least three mana spent of it. Adamant is an ability
word with no rules meaning of its own [CR#207.2c].

## Change

Decide the form: a condition quantifying over colors on top of `manaSpent`, or a dedicated
facet. Spell both witnesses in full once decided. Whether colorless counts as a "color" here is
for the design to settle; [CR#105.1] lists the colors and [CR#106.1b] separates colorless as a
mana type.

## Tests

- *Clockwork Servant* and *Henge Walker* spelled in full.
- A pin that a stated-color count does not satisfy the same-color form and vice versa.
