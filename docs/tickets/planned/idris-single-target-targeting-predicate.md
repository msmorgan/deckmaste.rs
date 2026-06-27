---
needs: []
---
**Idris grammar: "single target" targeting predicate.** `[design]` a predicate over a
stack object asserting it has exactly one target — e.g. `SingleTarget : Predicate b
AnObject`, or a count-of-targets comparator if the engine exposes that value.

Surfaced while building the stack-object redirect ops (`Copy` / `ChangeTarget` /
`ChooseNewTargets`) for the now-integrated `idris-effects-costs-and-choices` item 14.
The redirect *actions* are expressible, but the targeting *restriction* both their canon
cards carry is not: Spellskite ("Change the target of target spell or ability **with a
single target** to ~") and Bolt Bend ("Change the target of target spell or ability
**with a single target**") may only target a single-target stack object, and there's no
predicate for "has exactly one target" ([CR#115.7a,115.7d]).

Orthogonal to the redirect ops (it's an object predicate, not a stack op) — which is why
it was left out of item 14 rather than folded in. Small; pairs with whatever
count-of-targets value the engine surfaces. Serializes with the other `idris-*` grammar
tickets (they all rewrite `idris/src/Core.idr`, so only one can be in flight at a time).
