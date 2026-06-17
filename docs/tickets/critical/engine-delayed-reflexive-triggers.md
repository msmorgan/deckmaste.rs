---
needs: [engine-resolve-effects]
---
Delayed ([CR#603.7]) and reflexive ([CR#603.12]) triggered abilities created
during effect resolution. `Effect::Delayed(TriggeredAbility)` and
`Effect::Reflexive(TriggeredAbility)` are defined in core and resolve to a
loud seam today — split out of `engine-resolve-effects` (which built the other
effect frames).

Both need machinery that does not exist yet:

- A **registry on `GameState`** for active delayed/reflexive triggered
  abilities created at resolution — each with its own captured source /
  controller / `TriggerBindings` context (a delayed trigger fires once for an
  event that happens *later*; it is not printed on any permanent, so the live
  `abilities_of_source` scan never sees it).
- **`scan_triggers` integration**: a new watcher source so the scan consults
  the registry alongside live battlefield permanents.
- **Delayed** ([CR#603.7]): a one-shot — fires the next time its event occurs,
  then is removed from the registry. Common shape: "at the beginning of the
  next end step, …", "when ~ leaves the battlefield, …".
- **Reflexive** ([CR#603.12]): "when you do, …" — triggers off an event the
  *same* resolution just produced (an immediate, resolution-scoped window),
  then is gone.

The `Effect::Delayed`/`Effect::Reflexive` resolution arms (currently the
`run_effect` catch-all) just register the ability into the new store; the firing
is the trigger-scan work.
