---
needs: []
---
**Give the README a fast reviewer path.** The repo's strongest artifacts
are invisible inside 250k+ lines: a drive-by reviewer gets one scroll of
prose and no pointer into the code. Add a short "Reviewing this in ten
minutes?" section — after the demo screencast, before Design — naming one
representative artifact per claim, each with a half-line of why:

- a hand-authored card: `plugins/canon/cards/Elesh Norn, Grand
  Cenobite.ron` — two statics that read exactly like the printed card;
- one engine system: `crates/deckmaste_engine/src/layer.rs` — the
  continuous-effects system: citation density, invariant comments, the
  dependency-ordering fixpoint;
- the proof layer: `idris/Spec.idr`'s `failing` blocks — invalid authored
  states rejected at typecheck, with pinned error messages;
- `cargo run` — the screencast, live.

Keep it to ~10 lines. The deep version of this orientation is
`docs-grammar-guided-tour` (maybe/) — this section is the trailhead, not
the tour.
