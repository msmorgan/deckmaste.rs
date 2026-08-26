---
needs: []
---

# Split Experimental.idr into layered modules

`idris/src/Experimental.idr` (6047 lines: one `mutual` block + a post-mutual
card layer) splits into four layer modules plus a pure umbrella, so agents and
rebuilds touch one layer instead of the whole core. Pure relocation — zero
semantic change; every declaration, doc comment, and citation moves verbatim.

Decisions already made (full symbol-by-symbol assignment lives in the dispatch
brief for this ticket):

- Idris 2 has no cross-module mutual recursion, so the split follows the real
  seams inside the mutual block: `Experimental.Phrase`
  (zone-expr/Predicate/Noun/Amount/Quantity/Condition + their delta/intro/gate
  families) → `Experimental.Triggers` (GameEvent + eventIntro/eventAfter,
  trigger-header machinery, Duration) → `Experimental.Effect` (the genuinely
  mutual clause core: Cost/Effect/StaticEffect/AbilityAt/TokenChars + total
  tables and telescopes) → `Experimental.Card` (post-mutual face/card laws).
- `Experimental.idr` becomes a pure umbrella of `import public` lines, so
  Macros/Cards/Proofs*/ProofsAnaphora need zero edits.
- Landed bottom-up (Phrase, then Triggers, then Effect+Card+umbrella as one
  stage), `mtg-dev.ipkg` gate per stage, full `mtg.ipkg` + cite check at the
  end. Umbrella goes last in the ipkg module lists.
- Fix rule for a mis-assigned symbol: move it down a layer; never duplicate,
  forward-declare, or weaken a gate. Never split the clause-core types apart.
