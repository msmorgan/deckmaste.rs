---
needs: []
---
Make one-shot effect verbs FIZZLE on an unbound reference instead of panicking, as
required by [Invalid authoring fizzles](../../decisions/invalid-authoring-fizzles.md).
An unbound `Reference::It`/`That`
(the anaphora slot is `None` and no lone-target fallback applies) correctly degrades to a
sentinel `ObjectId::null()` in `GameState::eval_reference`
(`crates/deckmaste_engine/src/resolve/query.rs:582-587`, tested by
`unbound_reference_degrades_to_null_not_panic`). But the singular wrapper
`eval_reference_set` (`query.rs:355-357`) returns `vec![null]` — a ONE-element vector holding
the sentinel — whereas the group twin `unbound_group` (`query.rs:593-598`) returns an EMPTY
vector. So `Selection`-based callers fizzle at the set level, while every one-shot `Act` verb
built on `eval_reference_set` never sees the "empty means unresolved" signal and packs the
null id straight into a store-mutating event with no liveness filter.

Confirmed panic path: `PlayerAction::PutCounters` (`resolve/player_action.rs:398-428`, only
guard is `count == 0`) builds `CounterPlaced { object: null, … }` →
`apply_occurrence`/`apply` → `impl EventApply for CounterPlaced`
(`crates/deckmaste_engine/src/step/player.rs:95`) calls `g.objects.obj_mut(null)` →
`object.rs:445 .expect("live ObjectId")` PANICS. `Action::DealDamage`
(`resolve/action.rs:58-75`) has the identical shape — this is the general pattern for every
`Act`-lowered one-shot verb, not a `PutCounters` quirk. The `Continuously(Modify(…))` path
(`resolve/effect.rs:484-488`) is safe only by accident: `apply_effect_in_layer`'s
`working.contains_key(&null)` membership check (`layer.rs:1608-1616`) is always false, so the
null target silently no-ops — no deliberate null guard at resolution time.

Fix at the boundary: have `eval_reference_set` drop null sentinels (returning an empty vector,
matching `unbound_group`) so all one-shot callers fizzle uniformly, OR filter null ids in the
event-construction step of each `Act`/`PlayerAction` one-shot verb before scheduling
`WorkItem::Emit`. Prefer the former (one fix, symmetric with the group path) unless a caller
legitimately needs the null slot. Add a behavioral test: an effect body `PutCounters(It, …,
<nonzero>)` in a frame with `anaphora.it == None` must fizzle (no counter placed), not panic.
Pre-existing (`DealDamage` shares it); surfaced by the `core-copy-grammar` Amass review, where
a `With(ChooseOne)`-bound `It` reference (now fixed to `That`) would otherwise have hit this.
