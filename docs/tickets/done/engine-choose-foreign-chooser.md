---
needs: []
---
DONE (with [[core-binder-choose-by]]): a `Binder::ChooseOne`/`Choose` now
surfaces its `ChooseObjects` decision to the *resolved chooser*, not
unconditionally to the spell's controller ([CR#608.2d], e.g. [CR#701.21a]
"that player sacrifices a creature of their choice"). The chooser rides the
binder itself — the new `by: Reference` field (default `You`), mirroring the
Idris `{default You by}` — rather than the enclosing `By(actor, …)` this
ticket originally sketched: `binder_choice` resolves `by` through
`acting_player` and the `Each`/`With`/`DivideAmong` surfacing sites use it as
`ChooseObjects.player`. `By(You, …)` behavior is unchanged
(`acting_player(You) = frame.controller`); the foreign route is covered by
`foreign_by_routes_choice_to_that_player` in resolve.rs.
