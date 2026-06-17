---
needs: []
---
**Risk spike (do early).** Validate that the engine's existing evaluators work
when driven by a *synthesized* `Frame` outside effect resolution — the linchpin
of data-driven strategies. The strategy evaluator must REUSE, not reimplement,
the engine's condition/count/reference logic:

- `GameState::eval_count(&Count, &Frame) -> Uint` (`crates/deckmaste_engine/src/resolve.rs`)
- `GameState::condition_holds(&Condition, &Frame) -> bool` (`crates/deckmaste_engine/src/condition.rs`)
- `GameState::eval_reference(&Reference, &Frame) -> ObjectId` (`crates/deckmaste_engine/src/resolve.rs`)

Build a `Frame` (`crates/deckmaste_engine/src/stack.rs`) binding `You` → the
deciding player's seat and `This`/`ThatObject` → a candidate option, then confirm
the three evaluate correctly against a live `GameState` with NO resolving effect
present. Deliverable: a helper that constructs such a Frame + tests pinning a
handful of Conditions/Counts (e.g. `Compare(CountOf(...), Less, Literal(...))`,
`StatOf(This, Power)`) against a hand-built game state.

If any evaluator assumes a live resolving effect, document the gap and the
thinner adaptation needed — this is the highest-risk unknown and gates
`strategy-evaluator-core`. Open sub-question: does `Frame` already expose clean
bindable "you" and "this/subject" slots, or does binding the candidate require a
small `Frame` addition?

## Status: DONE — R1 resolved, no adaptation needed

`crates/deckmaste_engine/src/strategy.rs` (2 spike tests; engine suite + clippy
green):

- **R1 resolved.** `eval_count`, `condition_holds`, and `eval_reference` are
  purely frame-driven over live `GameState` — they make **no assumption of a
  live resolving effect**. `eval_reference` reads `You`→`player(frame.controller)`
  and `This`→`frame.source` (a plain id lookup, not a stack object);
  `eval_count`'s `CountOf`/`StatOf` iterate the live object store / read derived
  stats off `layers()`; `condition_holds`'s `Exists`/`Is`/`Compare`/`AllOf`…/
  `YourTurn`/`DuringPhase`/`Happened` all read `GameState` + `Frame` only. No
  thinner adaptation is required; the strategy evaluator reuses them verbatim.
- **R2 answered.** `Frame` already has clean bindable slots — `controller`
  (→`Reference::You`) and `source` (→`Reference::This`/`~` when `bindings` is
  `None`). **No `Frame` addition is needed**, and no new `Reference::Candidate`
  variant in core: bind the candidate into `source`.
- **Deliverable.** `eval_frame(state, seat, candidate: Option<ObjectId>) ->
  Frame` builds the sensing frame (`You`→seat, `This`→candidate, or seat's
  player proxy when there is no candidate; no targets/bindings/choice/X). Tests
  pin: `eval_reference(You/This)`, `eval_count(StatOf(This, Power))` = 2 on a
  battlefield Grizzly Bears, and `condition_holds(Compare(StatOf(This, Power),
  AtLeast, 2))`. Carries `#[allow(dead_code)]` until `strategy-evaluator-core`
  (next ticket) consumes it.
