# Idris is a soundness gate

> Superseded for workbench choice (2026-09-05) by
> [Lean is the workbench](lean-is-the-workbench.md), and for the executable
> card gate over `plugins_v2/` (2026-09-06) by
> [lean-card-soundness-gate](../tickets/done/lean-card-soundness-gate.md):
> `cargo xtask lean-check` emits each expanded v2 card as a Lean term and
> proves `Card.check = []` by `decide`. `cargo xtask idris-check` remains the
> gate for the v1 `plugins/canon` corpus, which has no v2 counterpart until
> `plugins-v2-canon` lands; the emitter, checker and baselines retire with it.
> The original rationale below is historical.

## Decision

The Idris model is a design probe and soundness gate for expanded card data, not
a parallel game engine. It should reject invalid semantic states when the model
can express the constraint without duplicating intrinsic or already-modeled
state.

## Rationale

Dependent types can make malformed references and incompatible shapes
unrepresentable before play. Keeping runtime semantics in one engine avoids two
implementations drifting while still letting the stronger model guide the Rust
grammar.

## Consequences

Validation operates on expanded semantic data and reports translation gaps
explicitly. Prefer structural types to redundant runtime flags, but do not add
an Idris representation solely to mirror engine state or rules that are
intrinsic to engine execution.

## Tracked references

- [Current Lean workbench](../../lean/README.md)
- [Keyword policy](../keyword-policy.md)
- [Rules taxonomy](../rules-taxonomy.md)
