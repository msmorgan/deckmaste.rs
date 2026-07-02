---
needs: [engine-deontic-polarities, engine-trigger-events]
---
DONE (headline): `Count::X` — announced at cast/activate [CR#107.3a], stored on
the stack entry, read back in `eval_count` — landed with engine-x-costs
(`Frame.x` threaded announce→`StackEntry`→`Frame`; `Count::X => frame.x`).

DONE (residue): Loyalty/Defense reads come off the counter map everywhere the
seams stood — the target-filter `derived_stat` (target.rs), the LKI
`snapshot_stat` (trigger.rs; snapshots carry counters), and activation
payability (activate.rs) — all mirroring `eval_count` [CR#122.1e,122.1g]. The
trigger-bound `ThatMuch` magnitude rides a new `TriggerBindings.that_much`,
captured at fire time from the amount-carrying events the apply funnel fixes
(DamageDealt/LifeLost/LifeGained) and seeded into the resolution register by
`resolve_object`; a `ThatMuch` with no antecedent magnitude stays a loud
authoring-error panic. Placement of a planeswalker's/battle's printed
loyalty/defense counters remains the planeswalker/battle modeling work
([[engine-planeswalkers]], [[engine-battles]]).
