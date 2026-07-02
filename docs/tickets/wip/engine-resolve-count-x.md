---
needs: [engine-deontic-polarities, engine-trigger-events]
---
DONE (headline): `Count::X` — announced at cast/activate [CR#107.3a], stored on
the stack entry, read back in `eval_count` — landed with engine-x-costs
(`Frame.x` threaded announce→`StackEntry`→`Frame`; `Count::X => frame.x`).

REMAINING seams only:
- Loyalty/Defense `StatOf` reads (counter machinery).
- The trigger-bound `ThatMuch` magnitude (needs a `TriggerBindings` slot in
  trigger.rs; the resolve.rs `Count::ThatMuch` todo covers the trigger-bound
  cases — apply-funnel `ThatMuch` already works).
