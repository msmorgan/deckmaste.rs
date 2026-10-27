---
needs: []
---
**No live ticket gates on or delivers into the Idris workbench.** The
2026-09-05 inventory found 43 tickets outside `done/` mentioning Idris. Sweep
them in one landing:

- Idris-specific and chopping-block deliverables: deleted 35 obsolete tickets
  targeting deletion-bound crates and directories (Idris workbench, legacy v1
  semantics/lowering, legacy v1 macro/plugin system, legacy v1 renderers, and
  legacy regex migration parsers), per owner instruction to delete tickets
  targeting chopping-block components rather than carrying forward maintenance debt.
- Repointed `docs/tickets/maybe/workbench-paper.md` to center on the Lean 4
  semantics workbench (`lean/Semantics/`).
- Passing mentions and deliverables scrubbed across 15 tickets to point to Lean gates
  (`lean/scripts/build`), Rust core/engine tests, or remove Idris mirror requirements:
  `balance-choose-complement-player-selection`, `core-do-or-die-divide-and-choose`,
  `engine-base-characteristics`, `core-location-outside-the-game`, `portfolio-polish`,
  `spinal-parasite-negative-pt`, `engine-crossed-channel-scope`, `engine-loyalty-limit-intrinsic`,
  `core-use-limit-general-window`, `core-designation-card-scope`,
  `core-retire-planechase-surface`, `tokens-predefined-registry`,
  `post-reshape-comment-rot`, `workbench-corpus-frame-gate`, plus `workbench-paper`.
- Checked `census.md` and `docs/tickets/README.md`: confirmed no live Idris columns or gate wording.

### Deleted tickets (35)
- **Critical (3):**
  - `critical/frames-catalog-merge.md` (retiring `plugins/builtin/frames/constructors.ron` into v1 defs)
  - `critical/macro-collision-diagnostic.md` (collision diagnostic for v1 macros in `deckmaste_semantics` / `macro_ron`)
  - `critical/target-sugar-elaboration.md` (indexed-target-slot sugar elaboration for v1 `deckmaste_semantics`)
- **Planned — Idris workbench (6):**
  - `planned/core-idris-prose-sweep.md` (sweeps comments to point to v1 `deckmaste_semantics`, gates on Idris build)
  - `planned/core-remove-default-args.md` (strips default args in `idris/src/Semantics.idr` and `idris_emit.rs`)
  - `planned/idris-coalesce-payer.md` (coalescing/payer typing derivation in `idris/src/`)
  - `planned/idris-copy-grammar-tail.md` (copy-grammar parity in `idris/src/Semantics.idr`)
  - `planned/idris-mirror-enum-gaps.md` (enum parity between v1 `deckmaste_semantics` and `Semantics.idr`)
  - `planned/plugin-rider-split.md` (splits `idris_emit.rs` and `validate.rs` out of `deckmaste_plugin`)
- **Planned — Legacy v1 macro & plugin system (5):**
  - `planned/canon-slice.md` (growing v1 `plugins/canon/`)
  - `planned/cards-counter-ref-validation.md` (counter ref validation in `deckmaste_plugin` for `idris_emit.rs`)
  - `planned/cards-render-cast-impulse.md` (render arms in legacy render for Chandra in `plugins/canon`)
  - `planned/macro-keyword-templates.md` (keyword macro templates in `plugins/builtin/macros/keyword/`)
  - `planned/memoize-macro-invocations.md` (macro memoization in `deckmaste_plugin`)
- **Planned — Legacy regex migration parsers & v1 card generation (13):**
  - `planned/parse-additional-cost.md` (legacy parser additional cost for `plugins/wizards`)
  - `planned/parse-ascend-fold-structured.md` (legacy parser ascend gate against `plugins/builtin`)
  - `planned/parse-cross-sentence-anaphora.md` (legacy parser cross-sentence anaphora, gates on `idris-check`)
  - `planned/parse-look-at-top-more-tails.md` (legacy parser look-at-top targeting `plugins/builtin/macros/effect/`)
  - `planned/parse-macro-slot-readers.md` (legacy parser macro slot readers in `deckmaste_plugin`)
  - `planned/parse-mana-cost-comma-spacing.md` (legacy parser mana cost comma spacing for `plugins/wizards`)
  - `planned/parse-reveal-hand-choose.md` (legacy parser reveal-hand choose, gates on `idris-check`)
  - `planned/parse-self-cost-mod.md` (legacy parser self cost mod for `plugins/wizards`)
  - `planned/parse-static-conditional-channel.md` (legacy parser static conditional channel)
  - `planned/parse-subject-filter-stringly-channels.md` (legacy parser subject filter stringly channels)
  - `planned/parse-target-hoisted-effect-macros.md` (legacy parser target hoisted effect macros)
  - `planned/parse-targeted-wrapper-dedup.md` (legacy parser targeted wrapper dedup)
  - `planned/spell-filter-subtype-validation.md` (legacy parser subtype validation against `deckmaste_plugin` `validate.rs`)
- **Planned — Legacy v1 renderers (6):**
  - `planned/render-battlefield-scope-noun.md` (battlefield scope render in `deckmaste_legacy_render`)
  - `planned/render-copy-retain-possessive.md` (copy retain possessive render in `deckmaste_plugin/src/render/`)
  - `planned/render-duration-clause-mid-insert.md` (duration clause placement in `deckmaste_plugin/src/render/`)
  - `planned/render-multi-element-fidelity-gaps.md` (multi-element render in `deckmaste_plugin/src/render/`)
  - `planned/render-opponents-control-suffix.md` (opponents control suffix in `deckmaste_plugin/src/render/`)
  - `planned/render-type-subtype-templates.md` (type/subtype template render in `deckmaste_plugin/src/render/`)
- **Maybe — Legacy v1 semantics & lowering (2):**
  - `maybe/characteristics-atoms-crate.md` (shared crate below v1 `deckmaste_semantics` and `deckmaste_core`)
  - `maybe/lowering-mirror-drift-gate.md` (mirror drift gate between v1 `deckmaste_semantics` and `deckmaste_core`)
