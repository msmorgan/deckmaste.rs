---
needs: []
---
**Narrow `GameState`'s field visibility so the engine-runner boundary is
enforced by the compiler, not by convention.** Today every field on
`GameState` (23 of them — `zones`, `objects`, `stack`, `turn`, `combat`,
`rng`, `agenda`, `shields`, …; see `crates/deckmaste_engine/src/state.rs`) is
fully `pub`. Any consumer crate can mutate `state.zones` or `state.stack`
directly, bypassing the zone-change pipeline (LKI snapshots, replacement
shields, trigger noting, SBA scheduling) and silently invalidating every
invariant the engine maintains. The ruling that the runner owns only
convenience/filtering/redaction while the engine owns all state transitions is
currently unenforceable.

A first-pass grep (receivers named `state`/`game`/`gs` outside
`crates/deckmaste_engine`) finds roughly 60 external field-access sites:
`zones` ~24, `objects` ~9, `stack` ~8, `turn` ~7, `combat` ~7, `players` ~4,
`pending` 1. That heuristic undercounts; step 1 below re-audits properly.

## Plan

1. **Audit**: flip all 23 fields to `pub(crate)` in a scratch build and let
   the compiler enumerate every external access site (tui, xtask, plugins,
   benches, integration tests). Classify each: read-only display (tui), test
   setup, or a genuine mutation from outside the engine (a boundary
   violation to be rerouted through an engine API).
2. **Reads**: add narrow `pub` read accessors (or extend existing view types)
   for the legitimate read-only uses. Do NOT expose `&mut` accessors.
3. **Writes**: reroute external mutations through existing engine entry
   points (`step`, `submit_decision`, the sim harness) or add a deliberate
   API where one is genuinely missing. Test-only construction can live behind
   `#[doc(hidden)]` helpers or `test_support`-style fixtures rather than raw
   field pokes.
4. **Land**: fields at `pub(crate)` (tighter where a field is only touched by
   one module and can go private). No behavior change anywhere — this is a
   visibility-only refactor.

Within-crate visibility (everything `pub(crate)` to all 19 `impl GameState`
files) stays as-is; carving intra-crate boundaries is out of scope here.

Gates: full engine suite green, workspace clippy clean, no new `pub` items
beyond the deliberate accessor surface.
