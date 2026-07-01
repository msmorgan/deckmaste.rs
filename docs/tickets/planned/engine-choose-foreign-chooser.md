---
needs: []
---
A `Binder::Choose` / `Binder::ChooseOne` nested under `By(actor, ...)` should surface its
`ChooseObjects` decision to the *resolved* actor, not the spell's controller
([CR#608.2d], e.g. [CR#701.21a] "that player sacrifices a creature of their
choice"). The engine carries the v1 simplification of always
choosing `player: frame.controller` (correct for the implicit `By(You, ...)`
that every current card uses; no canon routes a choose binder through a foreign
actor). Fix: thread the enclosing `By` actor and resolve it for the
`ChooseObjects.player`.
