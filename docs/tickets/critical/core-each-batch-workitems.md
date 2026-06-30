---
needs: []
---
Fallout from the core-verb-patient-cardinality refactor: the `Effect::Each`
simultaneity-batch fast path silently drops every non-`Emit` work item, plus a
few cheap correctness/clarity wins surfaced alongside it.

## #1 (correctness) — Each batch path drops non-`Emit` work items
`crates/deckmaste_engine/src/resolve.rs` (the `Effect::Each` arm, single-`Act`
batch branch). The branch collects only `WorkItem::Emit`:
```rust
for item in self.action_items(action, &next) {
    if let WorkItem::Emit(occ) = item { /* … */ }   // everything else dropped
}
```
The batch gate (`Effect::Act(action) if !CreateReplacement`) is too coarse — it
also matches `By(player, PlayerAction)` bodies, whose `action_items` return
non-`Emit` items (`DiscardCards`, `ChooseManaColor`, `OpenDistribute`). So
`Each(over: <players>, By(Subject, Discard{count:1}))` — "each player discards a
card", and likewise "each player sacrifices …" — **silently no-ops per element**:
no panic, no warning. Latent only because shipped Each cards (Pyroclasm/Flame
Rift) deal damage (pure `Emit`) and the lone Each test covers `DealDamage`.

**Fix:** narrow the batch gate to bodies known to be pure-event verbs, OR in the
loop fall through to the per-element `RunEffect` path when a non-`Emit` item
appears (don't drop it). Fix the false "a verb emits only `Emit` work items"
comment. Add an Each-over-`Discard` (or `AddMana(AnyColor)`) resolution test
asserting each element's choice-bearing work item is actually scheduled.

## Cheap wins (bundled)
- **`eval_selection` exhaustiveness** — `resolve.rs` `eval_selection` is now a bare
  `todo!()` reached via a single-variant wildcard in `eval_selection_set`
  (clippy `match_wildcard_for_single_variant`, `used_underscore_binding`). Today
  the wildcard routes `Selection::AmongNoted` (grammar-valid per
  `docs/conformance.md`) into the `todo!`, and it defeats exhaustiveness (a new
  `Selection` variant compiles clean, panics at runtime). Handle `AmongNoted`
  explicitly (real arm or a labelled not-yet-supported error), then delete the
  `eval_selection` shim + wildcard so new variants force a compile error.
- **`Frame.those` doc** (`stack.rs`) — the doc-comment describes `those` only as
  the plural group read by `Selection::Those`; the refactor also stores the
  singleton `Reference::That` there. Document the dual use (the principled
  separate-field fix lives in `core-many-binder-group-move`).
- **`That` render marker** (`render/fragment.rs`) — `Reference::That =>
  ctx.that.unwrap_or("it")` silently renders "it" for a mis-encoded top-level
  `That` (no enclosing `With`); emit the crate's `[unrendered:…]` marker when
  `ctx.that` is `None`, matching the rest of the renderer's error contract.
