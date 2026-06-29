---
needs: []
---
**Core: unify zone-move destinations — `Destination`/`Anchor`, and ordered group
placement (`MoveArranged`/`Arrangement`).** From the 2026-06-28 idris↔rust model
audit.

Today two unrelated verbs cover relocation (`crates/deckmaste_core/src/action.rs`):
`Action::Move(Selection, Zone)` (no position) and `PlayerAction::PutInLibrary(
Selection, Count)` (a from-top index only). There is no library-*bottom*
placement and no ordered-group arrangement.

Idris models one destination notion (`idris/src/Core.idr`): `Move (Reference)
Destination`, where `Destination = ToZone Zone | ToLibrary Anchor` and `Anchor =
FromTop Count | FromBottom Count`. A simultaneous group landing in an ordered
zone carries an `Arrangement = ChosenOrder | RandomOrder | SameOrder` via a
separate `MoveArranged Selection Arrangement Destination` verb — order is a
property of the placement, not the loop.

Adoption:
1. Introduce a `Destination` (plain zone, or library-at-an-anchor) and route
   `Move`/`PutInLibrary` through it — subsumes both verbs and adds `FromBottom`
   by construction ("graveyard at a position" stays unrepresentable).
2. Add a `MoveArranged` group verb with an `Arrangement`. `RandomOrder` (a
   randomized pile) leans on the RNG seam.

Verdict: **improvement** (collapses two verbs; adds bottom-of-library and
random/preserved group ordering, none expressible today). Effort: **M**.
Related: `engine-randomness` (planned/, the RandomOrder source); `engine-explore`
(planned/) touches `PutInLibrary`'s reveal branch.
