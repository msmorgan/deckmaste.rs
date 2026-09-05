---
needs: []
---
**No live ticket gates on or delivers into the Idris workbench.** The
2026-09-05 inventory found 43 tickets outside `done/` mentioning Idris. Sweep
them in one landing:

- Idris-specific deliverables, moved to `done/` with a closing note that the
  Lean workbench superseded them, or re-pointed at the Lean equivalent where
  the underlying model gap is real: `idris-mirror-enum-gaps`,
  `idris-coalesce-payer`, `idris-copy-grammar-tail`,
  `idris-distinct-position-proof`, `idris-emittables-drift-control` (closed
  by `entailments-table-without-idris`), `maybe/idris-composite-card-types`,
  `maybe/idris-emit-twoface-card`, `maybe/idris-event-model-coverage-gaps`,
  `maybe/workbench-obligation-cycle-totality`. For each re-pointed one the
  ticket text names the Lean file and construct; a moved one records what,
  if anything, the Lean already covers.
- Passing mentions whose gate line reads "`cargo xtask idris-check
  plugins/canon` no regressions" or "no existing Idris shape" (about thirty,
  listed in the inventory: `target-sugar-elaboration`,
  `core-do-or-die-divide-and-choose`, `core-location-outside-the-game`,
  `core-remove-default-args`, `cards-untap-skip-authoring`,
  `portfolio-polish` and the rest): rewrite the gate line to the Lean gate
  (`lean/scripts/build`, and `lean-card-soundness-gate` once it lands) and
  drop `needs:` edges on done Idris tickets that no longer bind.
- `plugin-rider-split`: strike the `idris_emit.rs` extraction; that code is
  deleted by `lean-card-soundness-gate`.
- `census.md` and `docs/tickets/README.md`: no Idris column or gate wording.

`kata kanban board` before and after go in the landing record; every moved
ticket is named.
