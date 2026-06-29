---
needs: []
---
**Core: divided distribution — a distribution effect with a per-element
`Allotment` anaphor (divided damage / counters).** From the 2026-06-28 idris↔rust
model audit.

Today damage and counters go to one recipient with one amount
(`Action::DealDamage(Selection, Count)`,
`PlayerAction::PutCounters(Selection, CounterRef, Count)` in
`crates/deckmaste_core/src/action.rs`); "N damage divided as you choose among
[a group]" and "distribute N +1/+1 counters among ..." are inexpressible.

Idris `OneShotEffect.Distribute (amount : Count) (Bindable Many k) body`
(`idris/src/Core.idr`) binds each element of the group as `It` with its
`Allotment` (the split is resolution-time, ≥1 each summing to `amount`); the body
is general — `Act (DealDamage It Allotment)`, `Act (PutCounters c Allotment It)`
— so one primitive subsumes divided damage AND divided counters (it replaced the
bespoke divided-damage verb).

Adoption: add a divided-distribution effect plus an `Allotment` value form read
inside its body.

**Name collision:** Rust already has an unrelated `PlayerAction::Distribute {
group, bins, name }` — the scry/surveil partition. Rename that (e.g. to
`Partition`) before taking the divided-distribution name, or pick a distinct name
(e.g. `DivideAmong`).

Verdict: **improvement** (a real card family with no current home; generality
via the allotment anaphor). Effort: **M**. Related: NONE.
