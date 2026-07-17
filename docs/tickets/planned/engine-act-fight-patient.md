---
needs: []
---
## STATUS: NOT addressed by this feature — an open design fork remains

The `Act` facet contract shipped, but the `Fight` second-combatant gap it
inherited was left as-is (no canon card in this feature fights, and no test
exercises a second-fighter fight trigger, so it stayed latent). Recorded here
accurately so it is not read as done.

Carry the second fighter on the `Act(Fight)` fact. The engine still decomposes
only the FIRST fighter onto the event: `resolve/action.rs`'s `Fight` arm emits
`future("Fight", None, Some(fighter_1), …)`, which the `FactView`
(`eval.rs`) records on `object` with `actor`/`patient` both `None` for an `Act`
fact. But the `EventFilter::Act` matcher for `verb == "Fight"` reads BOTH
`fact.object` (the pattern's `who` slot) AND `fact.patient` (its `on` slot) —
and a `None` part matches only `Predicate::Any`. Net: `Fight(Ref(This), Any)`
("whenever this creature fights") silently misses when the creature is the
SECOND fighter, and any narrowed second slot (`Fight(Any, Type("Zombie"))`)
never matches at all. The `eval.rs` matcher comment already claims Fight
"carries its two combatants on `object`/`patient`", but the emitter only ever
populates `object` — a half-implemented shape.

Because "fights" is symmetric ([CR#701.14a]: each creature deals damage to the
other), a "whenever ~ fights" trigger should fire on EITHER participant; today
it fires only when ~ is the first-named fighter. The fix is a design fork the
`Act` contract did NOT settle:

- **(a) Emit the second fighter symmetrically** — carry it on the fact (a new
  patient facet for `Fight`, or a second emitted `Act`) and match either
  participant, so the trigger fires on both combatants ([CR#701.14a]). Larger:
  touches the `GameEvent::Act` shape, the emitter, and the matcher.
- **(b) Drop the second slot from the `Fight` pattern type** until a card needs
  it — removes the dead-narrowing grammar but does NOT fix the symmetry miss on
  its own.

Today's grammar can still express `Fight` patterns that can never fire, which
the no-dead-grammar philosophy exists to prevent. Picking (a) vs (b) is a design
decision, not a mechanical edit — it should be made before this ticket is closed
as done.
