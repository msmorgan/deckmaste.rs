---
needs: []
---
**Engine: `ForThisEvent` riders other than `Cant(Regenerate)` are unbuilt.**

`crates/deckmaste_engine/src/resolve/effect.rs` wires exactly one rider shape:
`Cant(Regenerate(on: <bare object Ref>))` ([CR#701.19c] — "can't be
regenerated"). Three adjacent arms stay loud:

- a `ForThisEvent` rider part that isn't `Cant`;
- a `Cant` rider whose action isn't `Regenerate`;
- `Cant(Regenerate(on:))` whose subject isn't a bare object `Ref`.

Documented residue of `engine-durations-grants` (done), which deliberately
left them per-kind loud rather than guessing. Widen when a card in the corpus
needs one; each new rider kind wants its own test.

Effort: **S** per rider kind.
