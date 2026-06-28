---
needs: []
---
**Core: `ManaCostOf(Reference)` — pay mana equal to a referenced object's mana
cost.** From the 2026-06-28 idris↔rust model audit.

Today `CostComponent::Mana(ManaCost)` (`crates/deckmaste_core/src/cost.rs`) is a
literal symbol list; a cost "equal to its mana cost" (Snapcaster Mage's granted
flashback) can't be spelled.

Idris `Cost.ManaCostOf (Reference …AnObject)` (`idris/src/Core.idr`) pays the
full colored cost of an object — the cost-language twin of the numeric mana-value
read.

Adoption: add a `ManaCostOf` cost component referencing an object.

Verdict: **improvement** (granted-flashback "cost equal to its mana cost"; niche
but clean). Effort: **S**. Related: NONE.
