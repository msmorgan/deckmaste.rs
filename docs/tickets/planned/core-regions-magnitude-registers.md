---
needs: []
---
**A magnitude produced at runtime has no register, so nothing can read
it.** Routed from `core-regions-test-restoration`'s landing, 2026-09-02;
two restored tests are `#[ignore]`d on it. Standard constraints apply.

The discourse stage deleted the state-global magnitude register and its
occurrence-amount writer, and lowering pins a magnitude to a definition
only when the amount is statically known. So a count produced while
resolving reaches no destination: a coin-flip or die-roll tally in
`crates/deckmaste_engine/src/resolve/player_action.rs`, and a zone-change
batch's card count in `crates/deckmaste_engine/src/step/mod.rs`, which
also lost the entailment table's amount column.

Give a runtime-produced magnitude a definition the way a runtime-produced
object already gets one: the instruction that fixes the amount writes a
number register, and the later read is an ordinary register read. This is
the number half of the same move the discourse stage made for objects.

## Gates

Both ignored tests un-ignore and pass. A card that flips coins and then
reads the number of heads resolves correctly. Standard suites green.
