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

## Landing record

Change `rvxsyvvmppyrsxkmmqmrwquqxnvnxpmq`; English lock `covered`: 20,254.

**PROVE:** The root README, guided tour, keyword policy and agent guidance
now name Lean as the active semantics workbench. The succession ADR defines
reference-only Idris maintenance, exact-result theorem pins and the boundary
between the existing workbench build and the pending Rust-to-Lean card gate.
The decision index marks the Idris workbench choice superseded; its broken
README link is repaired. Nine historical ADR bodies are unchanged apart from
a dated succession note. The Lean README describes current checking and
authoring; its migration phrasebook moved verbatim to the new ADR appendix.
Its quoted negative pin exactly matches the existing checked Lean theorem.
All added or replaced local Markdown links resolve, including URL-encoded
paths. The historical declaration ADR retains nine pre-existing links to
tickets at their former planned locations; its original body is preserved. No source, declaration data, proof or test changes: 0 restored,
re-spelled, newly ignored, added or removed tests, and no coverage change
introduced by this documentation landing.

**DISCLOSE:** The keyword-policy update also repairs the active soundness
references in sections 4, 6 and 7 and replaces the stale Idris-constructor
follow-up in section 15; leaving those would contradict the requested
section 14 update. Historical arguments are retained. The separate
`docs-workbench-drift` ticket still owns its two old constructor references;
they are historical in the annotated ADR, and its closure-table edit is
outside this ticket. No new Game Model term or unresolved ruling conflict.

The read-only landing-page audit covered `gh-pages` change
`qtrokvypmkqnknuovmoxvmyztunlwxwm` and the live <https://deckmaste.rs/> page.
Its README and guided-tour links still target the same documents. The page
makes no Idris or Lean workbench-gate claim; its current runtime pipeline and
citation-checking prose remain true. No formal-workbench verification section
was added, so its implementation list required no change. No bookmark or
published page was changed.

**REPORT:** Documentation-only landing; the corpus, selection census,
construction inventory, licensing counts and performance were not remeasured.
The lock remains at the stamped 20,254 identities. Citation checks report
0 noncompliant and 0 stale citations; the two changed citation sites were
read against their rule text. Refresh was a no-op, so existing verification
remained applicable. The historical-note and phrasebook checks compare
against the parent text, rather than relying on a visual diff alone.
