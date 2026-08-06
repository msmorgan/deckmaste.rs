---
needs: []
---
# engine-unbound-eventobject-panics — unbound `EventObject` into `PutCounters` panics instead of fizzling

An unbound `Reference::EventObject` ("it" with no "it" bound) flowing into
`PlayerAction::PutCounters` panics at the object store's live-id assert
instead of fizzling — violating the engine's never-crash-on-invalid-semantics
principle ("invalid semantic input fizzles, never panics").

## The chain

1. `Reference::EventObject` resolves in `crates/deckmaste_engine/src/resolve/query.rs:484-498`.
   When neither `frame.anaphora.that_object` nor `that_patient` is set, the
   `None` arm at query.rs:495 calls `Self::unbound_ref(reference, "EventObject
   outside a trigger")` (defined query.rs:582-587), which only `eprintln!`s a
   warning and returns `ObjectId::null()` — a "fizzle" in name only: the null
   id is handed back to the caller as if it were live.
2. `PlayerAction::PutCounters(sel, kind, count)` resolves at
   `crates/deckmaste_engine/src/resolve/player_action.rs:385-415`. Unlike a
   `Selection`-based action (whose unbound case fizzles the WHOLE group to
   `Vec::new()` via `unbound_group`, query.rs:593-597), `PutCounters`'s first
   field is a single `Reference` (`deckmaste_core::action.rs:386`), resolved
   via `eval_reference_set` (query.rs:355-357), which just wraps one
   `eval_reference` call in a `Vec` — so an unbound `EventObject` produces
   `vec![ObjectId::null()]`, not an empty vec. `events` is therefore
   non-empty, and a `GameEvent::CounterPlaced { object: ObjectId::null(), .. }`
   is queued as a `WorkItem::Emit`.
3. That event applies later at `crates/deckmaste_engine/src/step/mod.rs:382`
   (`GameEvent::CounterPlaced(e) => e.apply(self)`), dispatching to
   `impl EventApply for CounterPlaced` in
   `crates/deckmaste_engine/src/step/player.rs:92-100`. Line 95,
   `g.objects.obj_mut(self.object)`, calls
   `crates/deckmaste_engine/src/object.rs:395-397`:

   ```rust
   pub fn obj_mut(&mut self, id: ObjectId) -> &mut GameObject {
       self.objects.get_mut(id).expect("live ObjectId")
   }
   ```

   `ObjectId::null()` is not a live key in the slotmap, so this **panics**.

`PlayerAction::RemoveCounters` (`player_action.rs:416-441`) has the identical
shape (a bare `Reference`, no null check before emitting `CounterRemoved`) and
shares the same latent gap — a fix should cover both.

## Fix direction

Check `.is_null()` on the resolved `Reference` before emitting the event (or
have `eval_reference_set` fizzle a null single-`Reference` to the empty vec,
mirroring `unbound_group`'s treatment of an unbound `Selection`), so an
unbound `EventObject`/other reference degrades to a true no-op fizzle instead
of reaching `obj`/`obj_mut`'s "live ObjectId" assert.

## Status: latent, out of scope here

No card in this feature's path exercises it: `engine-act-fight-patient`'s
fight triggers now bind "it" (`EventObject`) per firing (`trigger.rs`'s
uniform `Act` arm), so the fight trigger path never reaches this unbound
state. Found while grounding that work, not caused by it.
