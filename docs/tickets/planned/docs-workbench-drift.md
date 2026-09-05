---
needs: []
---
**Fix the two documentation drifts the terminology audit recorded.**
Residue of `terminology-context-adoption` (2026-09-04):

- `docs/decisions/workbench-ron-shaped-and-label-rulings.md` and
  `docs/idris-workbench-closure-tables.md` still reference a workbench
  `OnlyWhile` constructor that no longer exists (the clause-order collapse
  made it the `onlyWhile` order macro); rewrite each reference to the
  surviving spelling.
- `docs/idris-workbench-closure-tables.md` §2.5 states a drifted
  `Effect.Conditionally` signature; restate it from
  `cargo xtask map idris idris/src/Experimental/Effect.idr`.

Size: S (docs only). Done when: `grep OnlyWhile docs/` finds only history;
§2.5 matches the map output; `cargo xtask cite check` clean. Standard
constraints apply.
