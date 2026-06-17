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
