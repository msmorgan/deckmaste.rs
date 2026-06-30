---
needs: []
---
**North star (whole initiative):** mirror the Idris `Bindable`/anaphor model in
Rust so the model is sound by construction and the Idris model can verify cards.
One `Bindable` feeds `With`/`Each`/`Distribute`; anaphors are `It` (iteration /
projection element) and `That` (one-binder, split `Reference`/`Selection` *by
slot*) plus the event roles; the binding environment carries per-slot **kind +
cardinality** with **add/clear** threading; soundness is structural (no separate
checker); the Idris model is the oracle. **Policy:** mirror Idris *concepts*;
keep Rust *spellings* only where the concept is identical; never preserve a
weaker Rust concept behind a matching name. (This reverses the earlier "Rust
naming canonical" reading — see [[idris-oracle-and-naming]].)

This ticket is the **core type layer** of that (deckmaste_core only). Engine,
parser, emitter, and cards follow in [[engine-anaphor-threading]],
[[ron-emitter-bindable]], [[cards-remodel-bindable]].

## Changes (idris/src/Core.idr is the reference)

- **Introduce `Bindable`** — the binder, mirroring Idris `Bindable b card k`.
  Variants: `TheRef(Reference)` (One), `ChooseOne(Filter)` (One),
  `Choose(Quantity, Filter)` (Many), `Existing(Selection)` (Many),
  `Expanded(Expansion<Bindable>)`. Cardinality (One/Many) and `RefKind`
  (object/player) are properties of each variant.
- **Unify the operators on `Bindable`:** `With`, `Each`, and `Distribute`/
  `DivideAmong` all take a single `Bindable` (Idris: `With : Bindable b card k`,
  `Each : Bindable b Many k`, `Distribute : … Bindable b Many k`). Each/Distribute
  fix cardinality = Many. This removes `Each.over: Selection` and the separate
  `With.binder: Binder` — they were two names for the one Idris concept.
- **Anaphors → the Idris set.** Add `Reference::It` (the Each/Distribute/
  projection element). Use **`That` in both `Reference` and `Selection`**
  (one-binder singular vs many-binder group), resolved by slot — drop the invented
  `Selection::Those`. Fold the fragmented `Subject` / `ThatObject` / `ThatPlayer`
  (and the `EventAgent`/`EventActor` aliases) into the Idris-shaped set: `It`,
  `That`, `EventObject`, `EventActor`. **Derive the exact mapping from Core.idr**
  before renaming (Subject is the filter/projection element → `It`; the Each
  element → `It`; trigger roles → `EventObject`/`EventActor`); reconcile, don't
  blind-rename.
- **`Selection` stays** as the group description that `Existing` wraps and that
  names already-bound groups (`Filter`, `Random`, `AmongNoted`, `TopOfLibrary`,
  `Pick`, `GetTargets`, `Expanded`, `That`). It is just no longer the direct
  argument of `Each`.

## Done
- `cargo test -p deckmaste_core` green; round-trip tests for the new shapes
  (`Each(Bindable, …)`, `With(Bindable, …)`, `Reference::It`, `Selection::That`).
- Expect engine/cards/migrations to stop compiling — that is handled by the
  dependent tickets in the shared `core-anaphor-mirror` workspace; do NOT patch
  them here.
