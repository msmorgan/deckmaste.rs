---
needs: []
---
**The docs name the Lean workbench, and an ADR records the succession.**
The 2026-09-05 inventory: `README.md` has a section "The Idris gate" and no
mention of `lean/`; `CLAUDE.md` says "the Idris workbench" in the terminology
rule and recommends `cargo xtask map idris`; `docs/guided_tour.md` sends
readers to `idris/src/Spec.idr`; `docs/keyword-policy.md` §14 requires an
Idris `KeywordActionSpec` constructor per keyword action;
`docs/decisions/idris-is-a-soundness-gate.md` links `idris/README.md`, which
does not exist; `docs/decisions/README.md` indexes it. Deliver:

- `docs/decisions/lean-is-the-workbench.md`: the decision that Lean succeeds
  Idris, what "reference only" permits (reread, never extend), the
  theorem-form pin convention (`okX`/`badX` by `decide`, Idris names kept),
  and that `idris-is-a-soundness-gate` is superseded by
  `lean-card-soundness-gate` when that lands; index it in
  `docs/decisions/README.md` and mark the Idris ADR superseded there.
- `README.md`: replace "The Idris gate" with a Lean workbench section (what it
  is, `lean/scripts/build`, the pin suites, the facts tables) and fix the
  roadmap line about "reattaching the Idris mirror".
- `CLAUDE.md`: terminology rule names the Lean workbench; the Bearings line
  points at the Lean constructor map once `map lean` exists (until then, at
  `lean/Semantics/Words.lean` and `Abilities.lean` directly).
- `docs/guided_tour.md` and `docs/keyword-policy.md` §14: the Lean checker and
  its declared-feature laws are the soundness reference; the Idris sentences
  go.
- `lean/README.md`: drop the "port of `../idris/...`" framing and the
  Idris→Lean phrasebook into the ADR's appendix, so the README describes the
  workbench as it is.

- The `gh-pages` bookmark is out of tree and easy to forget: its
  `index.html` (the live deckmaste.rs landing page) links the README and the
  guided tour and describes the pipeline in its own words. Audit it in the
  same landing: today it names neither workbench (checked 2026-09-05, bookmark
  and live site identical), so the work is to keep its prose and links true
  once those two docs change, and to add the Lean workbench to its
  "what's implemented" list if the page is to mention verification at all.
  The bookmark is hand-authored; edit it on its own line, never from a
  feature workspace.

Historical ADRs (`semantics-spelling-lowering`, `semantics-v2`,
`oracle-text-is-forward-anaphoric`, `workbench-ron-shaped-and-label-rulings`
and the rest that cite Idris files as evidence) are not rewritten; each gets
one dated note at the top that the evidence now lives in `lean/` and the
Idris paths are historical. `docs-workbench-drift` (existing) fixes the two
stale constructor references and can land first or with this.
