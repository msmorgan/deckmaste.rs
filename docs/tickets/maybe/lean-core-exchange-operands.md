---
needs: []
---
# Fold `Exchanged.lifeTotals` into `Exchanged.values` (`[CR#701.12c,701.12g]`)

In [`Semantics/Abilities.lean:518-525`](../../../lean/Semantics/Abilities.lean),
`Exchanged` has five constructors:
- `lifeTotals (parties : NounPhrase)` (`[CR#701.12c]`)
- `controlOf (left right : NounPhrase)` (`[CR#701.12b]`)
- `cardsAcross (left right : NounPhrase)`
- `zones (left right : Location)` (`[CR#701.12d]`)
- `values (left right : Amount)` (`[CR#701.12g]`: life totals, powers,
  toughnesses, or rolled results)

Under `[CR#701.12c,701.12g]`, exchanging life totals is exchanging two numerical
values, and *Tree of Redemption* ("Exchange your life total with Tree of
Redemption's toughness") already uses `values (lifeOf you) (toughnessOf thisCreature)`.
Two-party life-total exchanges with distinct NPs (*Mirror Universe*: `"you"` and
`"target opponent"`) likewise write `values (lifeOf you) (lifeOf (target isOpponent))`
directly.

## Design

- Check whether plural two-party life exchanges (*Soul Conduit*: `"Two target
  players exchange life totals"`, currently `lifeTotals (targets (exactly 2) isPlayer)`)
  can be expressed cleanly through `values` (or a pairwise `valuesOf (axis : ProjAxis) (parties : NounPhrase)`)
  without losing the single plural target group (`targets (exactly 2) isPlayer`),
  retiring `Exchanged.lifeTotals`.
