---
needs: [core-bindable-unification]
---
Engine layer of the anaphor mirror (see [[core-bindable-unification]] for the
north star). Thread the binding environment the way Idris's `Bindings` index
does, so anaphor reads are determined by structure and the first-of-many class is
unrepresentable.

## Changes (idris/src/Core.idr `Bindings`/`bindIt`/`bindThat` are the reference)

- **`Frame` binding env carries per-slot kind + cardinality**, mirroring Idris
  `Bindings`: `itKind : Maybe RefKind`, `thatKind : Maybe (Cardinality, RefKind)`,
  `targetKinds`, event caps, chosen, `hasAllotment`. (Replaces the untyped
  `frame.those: Option<Vec<ObjectId>>` whose dropped cardinality caused the
  first-of-many bug.)
- **`bindIt` / `bindThat` / `bindAllot` with add-AND-CLEAR.** `bindIt` clears
  `hasAllotment` so a `Distribute` share cannot leak into a nested `Each`
  (Core.idr's allotment-clearing `bindIt`). Threading is add *and* clear, not
  accumulate-only.
- **Reads resolve against the env, singular-vs-group by slot.** `Reference::That`
  requires a `(One, k)` binding; `Selection::That` requires `(Many, k)`; a
  singular read of a `Many` binding is an error, never `.first()`. `It` reads the
  iteration element. Collapse the old `Subject`/`ThatObject`/`ThatPlayer` reads
  into the unified anaphors.
- **`Each`/`Distribute` consume `Bindable`:** resolve the binder to its group,
  iterate, `bindIt` per element (binding `It`); `Distribute` also binds the share.
  (Carry forward the simultaneity-batch / non-`Emit` handling from
  `core-each-batch-workitems` — choice-bearing bodies still schedule per element.)

## Done
- `cargo test -p deckmaste_engine` green. New tests: nested-`Each` clears the
  outer `Distribute` allotment; a many-binder iterated by `Each` acts on ALL
  elements (Brainstorm-shape moves both cards); kind binding for player vs object
  elements.
