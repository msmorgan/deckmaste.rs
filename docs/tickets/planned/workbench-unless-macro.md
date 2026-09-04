---
needs: []
---
**Retire `Effect.Unless`; spell "unless" as a macro over the deontic offer.**
Ruling 2026-09-04 (cleanroom review 3, D-Q9).

- `Effect.Unless` and `Effect.May offer (Pay …) Nothing (Just e)` are two core
  spellings of the same "unless a player pays" sentence — `Cards/Choice.idr`
  writes one, `Cards/Deontic.idr` the other. The offer is the row: `Unless`
  becomes a macro over `May` with a cost, and the constructor goes.
- `Unless` is one of the constructors that consumes `annIntro`; the macro
  threads the context the offer already threads, so nothing new is introduced.
- Re-spell both families of witnesses through the macro, as printed, and
  re-spell the pins that refuted through the constructor.

Size: S. Done when: `grep` finds no `Effect.Unless`; every "unless" witness
reads through the macro; the re-spelled pins probe non-vacuous; build at its
module count. Standard constraints apply, including the RON-shaped constraint.
