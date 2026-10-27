---
needs: [lean-v3-spell-abilities-share-targets, plugins-v3-builtin-primitives-and-templates]
---
# "If at least three [color] mana was spent to cast this spell"

Standard constraints apply.

## Problem

Adamant is an ability word [CR#207.2c]. It has no rules meaning of its own: the condition is
written out in each ability it heads, "if at least three [color] mana was spent to cast this
spell". The checker has no amount for how much mana of one color was spent. `LinkedFacet`
(`lean/Semantics/Abilities.lean`) has only `colorsSpent` (how many colors of mana were spent,
`colorsSpentToCast`) and `manaValueSpent` (how much mana was spent in total,
`manaValueSpentToCast`). Neither counts the mana of one stated color.

## Witnesses

- *Searing Barrage* {4}{R}: "Searing Barrage deals 5 damage to target creature. / Adamant — If at
  least three red mana was spent to cast this spell, Searing Barrage deals 3 damage to that
  creature's controller."
- *Rally for the Throne* {2}{W}: "Create two 1/1 white Human creature tokens. / Adamant — If at
  least three white mana was spent to cast this spell, you gain 1 life for each creature you
  control."
- *Unexplained Vision* {4}{U}: "Draw three cards. / Adamant — If at least three blue mana was spent
  to cast this spell, scry 3."

## Ruling (owner, 2026-10-24)

Use `ManaTypeTerm` [CR#106.1b] (`Abilities.lean:347-350`: `.colorless | .ofColor (color :
ColorTerm)`, verified).

## Change

- Add `LinkedFacet.manaSpent (type : ManaTypeTerm)` (`Abilities.lean:207`), the amount of mana of
  that type spent to cast the object. Templates beside `colorsSpentToCast` (`Templates.lean:1230`):
  `manaOfColorSpentToCast` and `colorlessManaSpentToCast`. Adamant then reads
  `compareAmt (manaOfColorSpentToCast .red thisSpell) .atLeast 3`.
- The same facet covers *Desecrate Reality*'s "three colorless mana".
- "Three mana of the same color" (*Clockwork Servant*, *Henge Walker*) is not covered; if it
  needs quantification over colors it is the separate ticket
  [lean-v3-same-color-mana-spent-count](lean-v3-same-color-mana-spent-count.md) (proposed).

## Tests

- Each witness spelled in full under `adamant`.
- *Desecrate Reality*'s "three colorless mana" via `colorlessManaSpentToCast`.
