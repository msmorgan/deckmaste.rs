---
needs: []
---
DONE: decided for the policy default — **mirror Idris**. `Binder::ChooseOne`
and `Binder::Choose` are now struct variants carrying `by: Reference`
(default `You`, omitted on write), matching the search binders' shape inside
the same enum and the Idris `{default You by}`. The engine reads the binder's
own `by` (resolved via `acting_player`) as the `ChooseObjects` chooser — the
runtime half is [[engine-choose-foreign-chooser]]. RON data (Brainstorm,
Enchant) and the oracle-text emitters moved to the named-field form
(`ChooseOne(filter: …)`, `Choose(quantity: …, filter: …)`). The rejected
alternative (derive the chooser from the enclosing `By(actor, …)` scope) is
recorded below for the design history.

---

Original framing: reconcile "who chooses" on the choose binders with the
Idris model — a design decision, then a small core change.

Idris carries the chooser directly on the binder: `ChooseOne : {default You by
: Reference b APlayer} -> Predicate b k -> Bindable b One k`, and likewise
`Choose` (`idris/src/Core.idr`) — exactly as `SearchOne`/`Search` do in *both*
models (Rust's search binders already carry `by`/`whose`/`from`;
`crates/deckmaste_core/src/binder.rs`). Rust's `ChooseOne(Filter)` /
`Choose(Quantity, Filter)` carry no chooser: the engine derives it from the
enclosing agent context (`Action::By(actor, …)`, implicit `By(You, …)`) and
today always resolves it to the spell's controller — the v1 simplification
[[engine-choose-foreign-chooser]] exists to fix.

Decide which shape is canonical and align:

- **Mirror Idris** (the naming policy's default — the Idris model is the
  canonical concept source): add `by: Reference` (defaulting to `You`) to
  `Binder::ChooseOne` / `Binder::Choose`, matching the search binders' shape
  inside the same enum; the engine reads the binder's own `by` instead of
  threading the enclosing `By` actor.
- **Or argue the enclosing-`By` shape is the genuinely-identical concept**, and
  move `by` *out* of the Idris choose binders instead — then
  [[engine-choose-foreign-chooser]] implements the actor threading.

Either way, `engine-choose-foreign-chooser` (routing `ChooseObjects.player` to
the resolved actor at runtime) is the companion engine half; keep the two
consistent.
