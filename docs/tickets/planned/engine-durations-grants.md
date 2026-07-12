---
needs: []
design: true
---
Close the two P0.W1 seams in `resolve.rs`'s `OneShotEffect::Continuously` arm
(the one-shot that mints a continuous-effect instance, [CR#611.2c]).

**Durations** (`resolve.rs` `todo!("P0.W1: duration …")`): only
`FixedUntil(EndOfTurn)` (cleanup sweep, [CR#514.2]) and `EndOfGame` may mint
instances today — any other duration would silently last forever, so the seam
is loud. Unbuilt sweeps/tracking: `FixedUntil(EndOfCombat)` ([CR#511.2]),
`FixedUntil(YourNextTurn)` ("until your next turn"), `UntilEvent(EventFilter)`
(engine pairs the undo one-shot, [CR#610.3]), `ForAsLongAs(Condition)` (tracked
predicate with the never-started / once-stopped-never-resumes `started` latch,
[CR#611.2b]), `ForThisEvent` (instruction-scoped rider, e.g. DestroyNoRegen).

**Grants** (`todo!("P0.W1: Continuously({…}) — non-Modify grants unbuilt")`):
only `Modify(reference, change)` (locked scope, [CR#613.6]) and
`Each(SelectAll(f), Modify(It, change))` (floating anthem/distributor shape)
are wired. A granted `Deontic` row ("target creature can't block this turn"),
`CostModifier` row, or `Each` over a non-`SelectAll` selection would be
silently inert — loud instead. Building grants means the minted instance can
carry non-Modification static rows and the legality/cost readers must see them
(the derived-view side: `legal.rs` deontic evaluation is its own ticket,
core-casting-restrictions / engine-combat-requirements — this ticket covers
instance creation + scope/duration bookkeeping so those rows EXIST in the
view).

Both seams share the `ContinuousEffect` instance record (timestamp, scope,
duration, controller) — build together. Sim strategy / decide arms are
unaffected (no new decision kinds).
