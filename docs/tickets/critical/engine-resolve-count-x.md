---
needs: [engine-deontic-polarities, engine-trigger-events]
---
`Count::X`: announced at cast/activate [CR#107.3a], stored on the stack entry,
read back in `eval_count`. The announce slot lives in cast.rs (from
engine-deontic-polarities). Also covers smaller leftover seams: Loyalty/Defense
`StatOf` reads (counter machinery) and the trigger-bound `ThatMuch` magnitude
(needs a `TriggerBindings` slot in trigger.rs).
