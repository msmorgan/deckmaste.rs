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
- one engine system: the exact dependency-ordering function and its focused
  test in `crates/deckmaste_engine/src/layer.rs` — citation density,
  invariants, and the layer fixpoint without asking someone to browse a
  multi-thousand-line file;
- the proof layer: the exact `failing` blocks in `idris/Spec.idr` — invalid
  authored states rejected at typecheck, with pinned error messages;
- `cargo run` — the screencast, live.

Render paths as clickable relative Markdown links and name the symbol or test
the reviewer should read; code-font paths alone are not a trail. Keep it to
~10 lines. The deep version of this orientation is
`docs-grammar-guided-tour` (maybe/) — this section is the trailhead, not
the tour.
