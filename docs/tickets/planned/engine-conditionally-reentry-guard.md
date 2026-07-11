---
needs: [engine-combatant-capability]
---
Guard against unbounded recursion when a `Conditionally` static's condition re-enters
the deontic-collector walk. `engine-combatant-capability` wired
`StaticEffect::Conditionally` into `for_each_static`/`walk_abilities` (so a conditional
`Cant` contributes only when its condition holds), which means the collector now calls
`GameState::condition_holds` while gathering an object's statics. Most conditions are
inert here, but one condition — `Condition::LegallyAttached` — evaluates via
`attachment_legal` → `may_attach_rows`/`cant_attach_rows` → `for_each_static`, i.e. back
into the collector walk. A permanent authored with
`Conditionally(LegallyAttached(This), <an Attach deontic>)` would recurse without bound →
stack overflow.

This was impossible before the feature (the walk looked *through* `Conditionally` without
evaluating its condition). It violates [[engine-never-crashes-on-authoring-mistakes]]: an
authoring mistake must fizzle/no-op, never crash the engine. Reachability today is ZERO —
no canon or planned card authors this shape, and the combatant feature's own confers use
only `Matches(This, SummoningSick)` / `Not(Matches(This, Has(Haste)))` (neither re-enters a
collector) — so it ships no live bug, but the latent stack-overflow should be closed.

**Fix options:** (a) a bounded re-entry/depth guard on the confer-fold + attach-legality
condition evaluation (a small recursion-depth counter or a visited-set on the frame,
returning a conservative default — "condition false" / "not legally attached" — when the
limit is hit, so the walk terminates and the pathological static simply drops); (b)
statically reject / lint conditions that can reach a collector at authoring-load, per the
soundness-gate philosophy. (a) is the robustness floor (never crash at runtime); (b) is the
authoring-surface complement. Prefer (a) as the guarantee, (b) as an optional early error.
