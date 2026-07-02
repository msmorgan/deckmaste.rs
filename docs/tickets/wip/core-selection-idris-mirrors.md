---
needs: []
---
Two `Selection` constructors exist in the Idris model with no Rust mirror.
Source: the `Selection` namespace in `idris/src/Core.idr` vs
`crates/deckmaste_core/src/selection.rs`.

- **`Union : List (Selection b k) -> Selection b k`** — several selections
  combined as one group. No Rust variant; text like "each creature that
  attacked or blocked this turn" has no direct group encoding today.
- **`BottomOfLibrary`** — Rust has only `TopOfLibrary`. Bottom-of-library
  *destinations* exist, but a bottom-N *selection* (reading those cards as a
  group) has no encoding.

Add the variants to core, wire `eval_selection_set` in
`crates/deckmaste_engine/src/resolve.rs`, and extend the renderer. Adopt when a
corpus card drives them, or as a small fidelity pass alongside
[[idris-naming-phase-two]] (which holds the pure-spelling drift for this
module).
