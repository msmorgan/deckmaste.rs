---
needs: []
design: true
---

# macro-frames-2 — close the pilot divergences, then the corpus-wide matcher

Round 2 of macro-frames (round 1 integrated at `xplortom`; gates and
carry-forwards in
`docs/superpowers/research/2026-07-30-macro-frames/round-1-ledger.md`).

Scope, in dependency order:

1. **Catalog-entry `body:` patterns** — one mechanism for both G4
   divergence families: the `#[macro_ron(embed)]` sugar family
   (`GainLife(N)` ≡ `By(You, GainLife(N))`, so flat two-param recovery
   does not even parse) and the announce-list family (Lightning Bolt is
   authored `Targeted(targets: [AnyTarget], effect: Act(DealDamage(This,
   N, Target(0))))`, not flat `DealDamage`). An entry's RON side becomes
   an optional macro_ron body term with `Param` leaves (absent ⇒ today's
   positional application), and G4 compares recovered *values* at the
   expanded level instead of spelled strings.
2. **`You` and `AnyTarget` pro-form entries** (parallel to the existing
   `This`) plus the `GainLife`/`Targeted` body entries — G4 to 0 diverged
   / 0 unspellable, `COVERAGE_FLOOR` raised in the same commits.
3. **`kind:`-scoped entry registration** (design-authority ruling,
   2026-07-31): required `kind:` on `ConstructorFrames`, mirror enum in
   macro_ron, register each entry at its declared kind only — kills the
   4–5× cross-category registration blowup before the catalog grows.
4. **Round-1's recorded integrity gaps** (ledger "ROUND-2 PLANNER
   NOTES"): spec D8's *build-time* uniqueness check (unimplemented; only
   the render-time half shipped), deterministic `macro inspect`
   resolution, and the render re-parse cross-check against
   `CompiledFrame::tree`.
5. **Corpus-wide matcher + residual census** (spec D11) — tile every
   canon line against the full lexicon; residuals emerge pre-typed and
   ranked; draft entries emitted to the research dir. The discovery
   surface.
6. **Hygiene:** strip round-1 process-narration comments from plugins RON
   (`Flying.ron`, `constructors.ron`, and siblings) — task/gate/review
   narration and citations of gitignored research docs go; CR-cited
   semantics stay. Genre rules per the `comment-discipline-sweep`
   ticket's decision-doc draft; that ticket's crates/ sweep stays
   separate.

Plan: `docs/superpowers/plans/2026-07-31-macro-frames-round-2.md`.
Execute via a fresh **Opus orchestrator** session named `macro-frames-2`
(Sonnet/Opus workers per task). Round parks un-integrated; integration is
the user's call.
