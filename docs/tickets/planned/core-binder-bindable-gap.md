---
needs: [core-bindable-unification]
---
Close the `Binder` ⊊ `Bindable` gap surfaced during
[[core-bindable-unification]]: the Idris `Bindable b card k` has **7**
constructors — `Produce`, `ChooseOne`, `SearchOne`, `TheRef`, `Existing`,
`Choose`, `Search` — but Rust `Binder` has only **5** (`TheRef`/`ChooseOne`/
`Choose`/`Existing`/`Expanded`), missing the producer/search binders.

The initial mirror left them out because the current corpus doesn't need them,
but they are real Idris binders (e.g. Cavern of Souls' `With (Produce (Move It
(ToZone Exile)))`; tutor/search effects bind a searched card). Closing the gap:

- Add `Produce` (a mana-production binder), `Search` (Many) and `SearchOne` (One)
  to Rust `Binder`, matching the Idris cardinality/kind discipline.
- Wire them through engine eval (resolve the binder → its binding), the migration
  emitter, and serde, with round-trips.
- Re-model any corpus cards that need them (e.g. tutors, Cavern-style produce-and-
  bind) onto the new variants.

Out of scope for the initial anaphor mirror; pick up once a card needs it or as a
fidelity pass.
