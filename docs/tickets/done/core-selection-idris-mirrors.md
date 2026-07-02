---
needs: []
---
DONE: both constructors mirrored. `Selection::Union(Vec<Selection>)`
(order-preserving concatenation, first-position dedup) and
`Selection::BottomOfLibrary { count, of }` (bottom→up, mirroring
`TopOfLibrary`) landed in core, `eval_selection_set`, and the renderer
(`Union` renders "each X and each Y"; the library windows keep the shared
catch-all like `TopOfLibrary`). The `of`→`whose` spelling for BOTH library
variants stays with [[idris-naming-phase-two]].

---

Original framing: two `Selection` constructors exist in the Idris model with
no Rust mirror. Source: the `Selection` namespace in `idris/src/Core.idr` vs
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
