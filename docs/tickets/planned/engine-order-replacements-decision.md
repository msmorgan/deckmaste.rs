---
needs: []
---
**Engine: the `OrderReplacements` decision has no submission handler.**

`crates/deckmaste_engine/src/decide/pending/choice.rs` —
`OrderReplacements::resolve` is unbuilt, and `strategy.rs` has no default for
it either.

[CR#616.1]: when two or more replacement and/or prevention effects would
modify the same event, the affected object's controller (or owner, or the
affected player) chooses one to apply first; the process then repeats over the
remaining applicable effects.

`strategy-evaluator-core` (done) deliberately deferred this as a deep shell —
"none surface in v1 decks". Reachable as soon as a card stacks two applicable
replacement effects on one event, so it should land before the corpus widens.
Pairs with `ChooseReplacement`, which IS wired.

Effort: **M**.
