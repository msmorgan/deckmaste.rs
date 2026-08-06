---
needs: []
---
**Engine: `Binder::Produce` only produces-and-captures for `Move`.**

`Binder::Produce(action)` runs an action and binds what it produced as `That`
([CR#400.7j] — an effect that moves an object to a public zone can find that
object again). `OneShotEffect::With` implements it, and only for
`Action::Move`, riding the same-resolution move record `engine-find-moved-object`
built. Two other consumers are consequently loud:

- `crates/deckmaste_engine/src/activate.rs` — `with_cost_feasible` can't
  decide payability ([CR#601.2h]) for a cost whose binder is `Produce`. This
  is a real hole: [CR#400.7j]'s second sentence is explicit that a *cost* may
  move an object to a public zone and the spell's effects can then find it.
- `crates/deckmaste_engine/src/resolve/effect.rs` — `resolve_binder`, the
  read-only (`&self`) spine `With`/`Each`/`Distribute` share, has no
  run-an-action fallback. Unreachable today, since `Each`/`Distribute` take
  many-binders and `Produce` is a one-binder; this arm is a guard, not a gap.

Fix: lift produce-and-capture out of `With`'s bespoke arm into a primitive the
cost-feasibility path can also call. Then decide which non-`Move` producer
actions the corpus actually needs — `deckmaste_core`'s `Binder` doc calls them
a labeled seam, and no corpus card drives one today. Run that corpus check
first, not last: if nothing needs a non-`Move` producer, the work is only the
cost-path hole and the ticket is much smaller.

Effort: **M**.
