---
needs: [core-bindable-unification]
---
RON/parser + migration-emitter layer of the anaphor mirror (see
[[core-bindable-unification]]). Make the flat RON carry the unified model and the
`That`-by-slot resolution.

## Changes

- **`That` resolves by slot.** A `That` token in a `Reference`-typed field
  deserializes to `Reference::That`; in a `Selection`-typed field to
  `Selection::That`. serde/RON already picks the variant from the field type — the
  job is to ensure both enums carry `That` and that `macro_ron` round-trips it.
  Drop the invented `Those` spelling.
- **`Bindable` (de)serialization** — `With`/`Each`/`Distribute` now take a
  `Bindable`; ensure it parses/renders (`TheRef`/`ChooseOne`/`Choose`/`Existing`/
  `Expanded`).
- **Migration emitter emits the unified shapes** — `Each(Bindable, …)`,
  `With(Bindable, …)`, `Distribute(Bindable, …)`; `It` for iteration elements
  (not `ThatObject`); `That` by slot. Update the cost-side `With` emission too
  (`parsers/cost.rs`). Remove any emitted `Those`/`ThatObject`-for-iteration.

## Done
- `cargo test -p deckmaste_migrations` and the `macro_ron` tests green; new
  round-trips for `Each(Bindable)`/`With(Bindable)` and `That` in both slots.
