---
needs: []
---
Pin down the per-verb facet contract of the `Act` composite lane. The shipped
dual-facet `Composite(KeywordAction, body)` gives each verb its own implicit
semantics — destroy/draw make `Act` the pre-commit replaceable moment, while
mill/scry/surveil/fateseal emit `Act` post-commit — but the shared surfaces
still advertise ONE uniform contract, and several correctness holes fall out
of the mismatch (found in the 2026-07-16 post-integration review):

1. **Post-commit verbs are not replaceable, but claim to be.**
   `replace_registry.rs` `replaceable()` marks every `GameEvent::Act`
   replaceable, and the `EventFilter::Act` docs (core `event.rs`, engine
   `event.rs`) call `Act` "both the guardable/replaceable moment and the
   trigger fact". For mill (batch committed first, `resolve/action.rs`) an
   authored `Instead(would: Act(Mill(..)), ..)` — the exact shape
   `Regenerate.ron` uses for destroy — fires after the cards already moved:
   the `instead` effect runs IN ADDITION to the mill and only the trigger
   fact is suppressed. Fix: make `replaceable()` verb-aware (pre-commit lanes
   only) and correct both docs; decide where a mill/scry replacement is
   supposed to bite (the per-card `ZoneWillChange`s today).

2. **Degenerate destroy falls into the reorder branch.** When the patient is
   gone, zoneless, or already in the destination zone, `move_shape` is `None`
   (`resolve/action.rs` `from != to` gate) and a destroy composite runs its
   stored `Move` body as a plain UNGUARDED move AND emits `Act(Destroy)`
   anyway (`composite_body_acts` defaults true for an `Act`-headed body) —
   spurious graveyard-remint `ZoneChanged` facts plus a shapeless
   `Act(Destroy)` that `Act(Destroy(Any))` triggers match, for an action that
   should do nothing [CR#701.8a]. A destroy whose `move_shape` is `None` must
   fizzle: no body run, no `Act`.

3. **No atom↔body coherence gate.** The raw `Composite(atom, body)` spelling
   is accepted plain-RON grammar and nothing validates the two facets agree
   (`validate.rs` has no check). `Composite(Mill(You,3), MoveGroup(top-5 …))`
   mills five while the `Act` fact reports the atom;
   `Composite(Destroy(x), Move(y, Exile))` commits y→Exile tagged
   `Cause::destroy` (the cause mapping is move-shape-, not verb-, keyed) — a
   silent wrong-fact commit instead of a fizzle. Add a load-time validation
   that the body is the atom's canonical expansion (or derive one facet from
   the other), and key the cause on the verb.

4. **`Act(Mill)` outlives a fully-suppressed batch.** The aggregate mill
   `Act` is scheduled once the schedule-time batch is non-empty; if an
   apply-time cant (e.g. `CantHappen(ZoneChange(from: Library))`) suppresses
   every per-card move, zero cards move yet the mill fact + "whenever you
   mill" triggers still fire [CR#614.17]. The act emit needs to be
   conditioned on the batch actually committing.

5. **Draw with a non-player `who` leaves phantom facts.** `who: None`
   resolves gracefully, but the lane still emits N `Act(Draw)` singles that
   fall through the pure-fact arm in `step.rs` — N trigger-visible "a player
   drew" facts with no card moved. The lane should fizzle before emitting
   when `who` is unresolvable.

6. **No count channel for multi-draw/mill replacement.** [CR#121.2a] requires
   count-level modification of a multi-draw before the individual draws
   (Alhammarret's Archive; Bruvac for mill), but draw N is decomposed into N
   `Act(Draw)` at schedule time and `Act`/`KeywordActionPattern` carry no
   count. Design where "draws twice that many" intercepts — this is a forward
   gap baked into the current surface, not a regression.

The `Composite` variant doc in core `action.rs` ("resolving it runs `body`")
also mis-describes the destroy/draw lanes — align it with whatever contract
this ticket settles. Items 1-5 are behavior bugs; 6 is design. All six move
together because the fix is one contract, not five patches.
