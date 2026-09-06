---
needs: [ci-lean-gate]
---
**Deferred by the user (2026-09-06): `semantics_v2` must exist before this
emitter is implemented.** Completing `ci-lean-gate` alone does not make this
work ready. The emitter must consume the actual v2 card representation and
macro expansions. Do not build an adapter from the current expanded RON or
invent a Lean/Rust crosswalk to work around the missing prerequisite. This
ticket remains unimplemented; the existing Idris gate stays in place meanwhile.

**The Lean workbench, not the Idris mirror, is the soundness gate for card
data.** Today `cargo xtask idris-check <plugin>` re-emits every expanded
RON card as an `idris/src/Semantics.idr` term through
`crates/deckmaste_plugin/src/idris_emit.rs` (4,507 lines) and typechecks it
with `idris2`; a per-plugin `idris-check-baseline.ron` ratchets pass/gap, and
`--differential` cross-checks that verdict against `deckmaste_lowering`. The
ADR `docs/decisions/idris-is-a-soundness-gate.md` is what makes the CI
`idris` job load-bearing. Replace it:

- an emitter from the expanded card to a Lean `Card` value of
  `lean/Semantics/Card.lean` (a `.lean` file per plugin, or one generated
  module), and a gate that `Card.check` returns `[]` for each, evaluated by
  `decide` exactly as the pin suites are;
- the baseline ratchet and the differential mode re-homed on the Lean verdict;
- `idris_emit.rs`, `idris_check.rs` and `plugins/*/idris-check-baseline.ron`
  deleted once the Lean gate covers every card the Idris gate did, with the
  landing record listing every card whose verdict differs and why.

Decisions already made: the Lean model is the semantics the engine consumes
(numbers are `Int` with declared regimes; core deeds are a closed taxonomy;
conferrals come from the registries); a law reads a declared feature, never a
lexeme. `plugin-rider-split` proposed extracting `idris_emit.rs` into a crate;
this ticket supersedes that half of it.
