---
needs: []
---
**Shrink `GameState`'s ~705-method surface by moving component-local methods
onto the component types they actually touch.** `GameState` (see
`crates/deckmaste_engine/src/state.rs`) is a struct-of-components — `Zones`,
`ObjectStore`, `TurnState`, `CombatState`, `PlayerState`, `Cards`,
`DesignationStore`, … — but nearly all behavior hangs off `impl GameState`
blocks spread across 19 files. A large share of those methods read/write
exactly one component and belong on that component's own `impl`:

- a method touching only `self.zones` → `impl Zones`
- only `self.combat` → `impl CombatState`
- only `self.objects` → `impl ObjectStore`
- pure helpers over one component's data → module-local free fns taking
  narrow params

Payoffs: the god object's method list shrinks toward genuinely cross-cutting
orchestration; each component's API documents what its subsystem actually
needs; split borrows get *easier* (borrowck can see disjoint fields), removing
`let x = self.f(); self.g(x)` dances. Explicitly **not traits** — behavior
does not vary by implementor here; this is plain method placement
(ruling settled: traits only where a second implementor exists, e.g.
`StrategyEvaluator`).

## Plan

1. Inventory: for each `impl GameState` method, compute the set of fields it
   touches (a quick script over `self.<field>` occurrences per method body is
   enough for triage; verify by hand before moving).
2. Move the single-component methods, file by file (subsystem by subsystem —
   this can also happen opportunistically inside [[split-trigger-rs]] /
   [[split-step-rs]] carves). Call sites change from `self.foo(...)` to
   `self.zones.foo(...)` — mechanical.
3. Methods touching one component plus a cheap read of another often want the
   read hoisted to the caller and passed as a param; don't force it where the
   result reads worse.
4. No behavior change anywhere. No new `pub` — component methods get the
   narrowest visibility that compiles (most `pub(crate)`).

Gates: engine suite green, workspace clippy clean, nightly fmt,
`cargo xtask cite check` (moved methods carry CR-cited comments).
