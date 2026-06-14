---
needs: []
---
A `Selection::Choose` nested under `By(actor, ...)` should surface its
`ChooseObjects` decision to the *resolved* actor, not the spell's controller
([CR#608.2d], e.g. [CR#701.21a] "that player sacrifices a creature of their
choice"). `engine-resolve-selections` shipped the v1 simplification of always
choosing `player: frame.controller` (correct for the implicit `By(You, ...)`
that every current card uses; no canon routes a `Choose` through a foreign
actor). Fix: thread the enclosing `By` actor out of `unresolved_choice` /
`PendingChoice` and resolve it via `acting_player` for the `ChooseObjects.player`.
