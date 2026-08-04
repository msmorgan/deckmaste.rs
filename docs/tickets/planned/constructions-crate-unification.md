---
needs: [english-construction-compiler]
---
**Collapse `deckmaste_construction_compiler` and `deckmaste_constructions_macro`
into one crate per purpose.** The declaration compiler is split across two
crates for a reason that no longer holds cleanly: `deckmaste_construction_compiler`
is a plain library so each stage stays directly testable, and
`deckmaste_constructions_macro` is a thin `proc-macro = true` facade over it.
The cost is two test suites and two `trybuild` trees for one pipeline, split on
a boundary that tracks testability rather than purpose. Only the
`deckmaste_features` option below drops the crate *count*; the other two leave
two crates, each with one job.

## The blocker, and why the fold is not just a `mv`

A `proc-macro = true` crate cannot export ordinary items, so everything the
*emitted* code needs must live somewhere else. `runtime` is exactly that:
`deckmaste_english` depends on `deckmaste_construction_compiler` as a normal
dependency and imports `runtime::{GroupData, ConstructionData, AtomData,
FieldKindData, WitnessClassData, DeclarationViolation, …}` directly, and
`emit.rs` writes 21 absolute `::deckmaste_construction_compiler::runtime::…`
paths into generated code. Folding the compile-time machinery into the
proc-macro crate therefore requires giving `runtime` a home reachable by
normal consumers first.

## Scope

- Move the compile-time stages — `model`, `parse`, `validate`, `emit`, plus
  `diag` and `render` — into `deckmaste_constructions_macro` as private
  modules, along with their unit suites, the `goldens/` tree, and the
  `compile_fail/` `trybuild` cases. One crate, one purpose.
- Give `runtime` a home. Verified options, none pre-decided:
  - **Slim the existing crate to runtime-only.** Smallest diff, keeps every
    emitted path and every `deckmaste_english` import valid unchanged, but
    leaves a crate whose name no longer describes it (rename is then its own
    churn).
  - **Relocate the runtime types into `deckmaste_features`.** Both crates
    already depend on it, so it removes a crate outright — at the cost of
    putting grammar-shaped data in the shared substrate crate.
  - **Mint a small runtime crate** (e.g. `deckmaste_constructions_runtime`).
    Cleanest naming, one more crate in the workspace.
- Whatever split survives, the in-repo `macro_ron` / `macro_ron_derive` pair is
  the shape to compare against: the normal library faces consumers, the
  `proc-macro = true` sibling holds only the macro machinery. Two limits on
  copying it wholesale. `macro_ron` re-exports its derive behind an optional
  `derive` feature — an unconditional re-export drags `syn`/`quote` into every
  consumer's build. And the re-export direction inverts under the
  `deckmaste_features` option: the folded macro crate needs `deckmaste_features`
  for the compile-time stages, so having `deckmaste_features` re-export the
  macro closes a Cargo dependency cycle. Keep the direction acyclic whichever
  option wins.

## Gotchas

- The emitted absolute paths in `emit.rs` move only if the runtime crate's name
  does — it does under the `deckmaste_features` and new-crate options, not
  under slimming in place. Where it moves, the checked-in golden
  `tests/goldens/fixture_coordination.rs` spells those paths out ~44 times:
  regenerate, do not hand-patch, and re-bless any `.stderr` carrying a crate
  path.
- The compiler crate's two `compile_fail` fixtures are the sharp case. They
  prove `validate::ValidatedGroup`'s private field cannot be forged, and they
  do it by importing `deckmaste_construction_compiler::{model, validate}` from
  *outside* the crate — which is exactly what a `proc-macro = true` crate can
  never offer. Moving `model` and `validate` into the macro crate leaves that
  seal with no external vantage point, and an in-crate test is no substitute
  (`trybuild` needs a separate compilation unit). Decide what replaces it
  before the move, not after.
- The suites merge into one crate, so keep `#[test]` coverage at parity and
  keep the two `trybuild` trees distinguishable after the merge.

## Gates

Standard constraints apply. No behavior change: workspace suites green with
`#[test]`-count parity, both `trybuild` trees passing, and the generated
`deckmaste_english` grammar byte-identical apart from any runtime path rename.
