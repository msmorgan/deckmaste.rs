---
needs: []
---
# `costActionOk`: admit or cite every row

Ruling 2026-08-22. `costActionOk` (`idris/src/Experimental.idr`) still has ~35
`False` rows with no pin and no rule, left by
`workbench-pins-refuse-rules-impossibility-only` (its gap 10). The five
structural rows — `Continuously`, `InsteadOf`, `Delayed`, `HeldUntil`,
`Reflexively` are not actions a player takes [CR#118.1] — are pinned already.
For each remaining row: admit it (a cost is any action a player can take
[CR#118.1]; destroying, exiling, milling, drawing, revealing, putting counters,
dealing damage as a cost are all rules-meaningful), or refuse it with the rule
that makes it meaningless and a biting pin. Bench one printed card per newly
admitted action where one exists (`corpus`/`card`). Doctrine:
`docs/memory/rulings/measurements-live-in-pins.md`.

## Consumption boundary
`idris/src/Experimental.idr`, `idris/src/Experimental/Proofs*.idr`,
`idris/src/Experimental/Cards.idr`, `idris/src/Experimental/Macros.idr`.

## Acceptance
No `costActionOk` row is `False` without a rule and a pin; build PASS;
`Cards.idr` binds no implicits; cites 0/0. Standard constraints apply.
