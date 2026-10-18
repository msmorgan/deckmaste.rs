import Semantics.Phrase

/-!
# Semantics.Triggers

Usage limits and trigger headers (`CountBound`, `UsageLimit`, `JoinedHeader`).

Because `DeonticMods` and `Action.limited` reference `CountBound` and `UsageLimit` while
`GameEvent.does` and `DidClause` reference `Action`, these declarations live in the unified
`mutual` block of `Semantics.Abilities` and are re-exported here.
-/
