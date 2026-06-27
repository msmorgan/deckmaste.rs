---
needs: []
---
**Idris grammar: the residual duplicate-concept collapses + one binder soundness check.**
Mostly light factorings in `idris/src/Core.idr`; item 5 is a soundness fix. From the
2026-06-26 grammar census.

1. **`While` (StaticEffect) vs `ForAsLongAs` (Duration).** The same condition-gated
   continuous effect lives in two categories. Collapse to one.
2. **`Condition` vs `Facet` timing-atom duplication.** `[design]` Timing atoms (`During`,
   turn-of) and `And`/`Or`/`Not` are duplicated across `Condition` and `Facet`. Collapse
   via a `Facet.Whenever : Condition -> Facet` bridge so a `Condition` is reusable as an
   event facet instead of re-encoding the atoms.
3. **`Search.from : List Zone` vs `MayCastFor.from : Zone`.** Same field, inconsistent
   arity — unify (probably both `List Zone`).
4. **`ExileUntil _ Forever` ≡ `Move … Exile`** overlap, against the file's own "no dedicated
   bounce verb" rationale. Reconcile the two exile paths.
5. **Soundness — `bindIt` and `hasAllotment`.** `bindIt` preserves `hasAllotment`, so a
   `Modify`/`Each` nested inside a `Distribute` body can rebind `It` while a stale *outer*
   `Allotment` share stays in scope. Source now carries an allotment-clearing twin of
   `bindIt`; **confirm `Distribute`'s inner rebind routes through it**, and if not, have
   `bindIt` reset `hasAllotment := False`. Verify-or-close.

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
