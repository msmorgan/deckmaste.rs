---
needs: []
---
**Core: `Destination` flatten overlaps the dedicated `Library(Anchor)` form.**
Found in the 2026-06-29 code review.

`Destination` is `Zone(#[macro_ron(flatten)] Zone)` + `Library(Anchor)`, but the
flattened `Zone` still contains `Library` (`crates/deckmaste_core/src/zone.rs`),
so a library destination is expressible two ways — bare `Library`
(position-less, underspecified; a library destination needs a position) and
`Library(FromTop(0))`. Two representations of one concept (the exact smell the
model audit targets), inviting corpus drift between `Move(sel, Library)` and
`Move(sel, Library(FromTop(0)))`.

Fix: exclude `Library` from the flattened zone set used by `Destination` (or
validate that bare `Library` is forbidden as a `Move` destination, the anchored
form being canonical).

Severity: **medium** (model dual-representation). Effort: **S**.
