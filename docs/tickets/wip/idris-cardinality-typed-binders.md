---
needs: []
design: true
---
**Idris grammar: cardinality-typed binders, so singleton `Each That` read-backs
evaporate.** Deep refactor in `idris/src/Core.idr`, explicitly deferred by the locked
`Each` design ("keep the single-binding read-back for now"). From the 2026-06-26 grammar
census.

A one-element binding (a `(^1)`/`Only`/`Search (^1)` result, or a `With`-bound `That`) is
currently read back with `Each That (…)` — an iteration over a guaranteed singleton, which
is semantically loose (it would attach to *every* element if the quantity were >1) and
duplicates the `enchant` macro's cleaner single-binding form.

Make binders **cardinality-typed**: a one-binder exposes `That : Reference` (a single
object); a many-binder exposes a `Selection` you `Each` over. The singleton-`Each That`
smell at its remaining sites then disappears by construction, and `Each`/`Distribute` can
take a `Bindable` directly instead of forcing the `With (…) (Each That …)` two-step.

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
