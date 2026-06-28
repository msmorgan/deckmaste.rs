---
needs: []
---
**Core: `MoveCounters(spec, from, to)` — move counters object→object, including
"all kinds".** From the 2026-06-28 idris↔rust model audit.

Today counters can be put and removed (`PlayerAction::PutCounters` /
`RemoveCounters`, `crates/deckmaste_core/src/action.rs`) but not moved between
objects in one operation, and "all counters" can't be moved atomically.

Idris `Action.MoveCounters (CounterSpec) (from) (to)` with `CounterSpec = Some
CounterKind Count | AllKinds` (`idris/src/Core.idr`) — Power Conduit / Leech
Bonder (`Some`), Ozolith / Fate Transfer (`AllKinds`). `AllKinds` is the one case
the single-kind remove+put can't reach (it quantifies over kinds).

Adoption: add a `MoveCounters` verb taking a counter-spec (a named kind + count,
or all kinds) and a source/destination object.

Verdict: **improvement** (Ozolith/Power Conduit family; `AllKinds` is not
atomically expressible today). Effort: **S–M**. Related: `engine-counters-api`
(done/) is the counter substrate; the Idris-internal de-smell was
`idris-grammar-collapses` (done/) — this is the Rust parity gap.
