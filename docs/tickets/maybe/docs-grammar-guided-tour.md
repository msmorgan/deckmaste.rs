---
needs: []
---
**Write a guided tour of the card-grammar / semantics surface.** A single
walkthrough doc that takes a reader from a printed card to its RON encoding to
the core effect tree to the Idris north-star check — the orientation that is
currently spread across ticket bodies and code comments.

Carved out of the original "regen wizards, full build + test, write guided
tour" checkpoint. The first three are **routine and effectively continuous**
(`cargo xtask generate plugins/wizards`, `cargo build/test --workspace`,
`cargo xtask idris-check` all run constantly); the residual deliverable is the
tour itself.

## Scope

- End-to-end for a handful of representative cards (a vanilla creature, a
  keyword-action card e.g. scry, a targeted spell, a triggered ability, a modal
  card): printed text → `plugins/canon/cards/*.ron` → `Effect`/`Action` tree →
  render-back-to-oracle → Idris re-emit + typecheck.
- The macro layer: how keyword actions expand over the ~6 primitives, and how
  `parse-via-macros` keeps render ⇄ parse one truth.
- The soundness story: what idris-check gates and why nonsense no-ops rather
  than crashes.

## Notes

- Likely partly stale — confirm the checkpoint's build/test/regen intent isn't
  already satisfied before treating this as net-new work; if so, this is purely
  the doc.
- A tour is documentation, not a spec/plan/review, so it may live as a committed
  reference (unlike process notes, which stay local).
