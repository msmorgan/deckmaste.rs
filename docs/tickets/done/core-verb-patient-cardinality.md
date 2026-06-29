---
needs: []
---
**Model divergence: effect verbs take a plural `Selection` patient where the
Idris model takes a single `Reference` and reserves plurality for the
`Each`/`Distribute` combinators.** Noticed 2026-06-29 while rendering
`Counter(Selection)`. Sibling of [[idris-naming-reconciliation]] (that one is
renames; this is structural).

**Idris** (`idris/src/Core.idr`) — every action verb's patient is ONE object:

```
1446  Destroy     : Reference b AnObject -> Action b
1447  Counter     : Reference b AnObject -> Action b
1434  DealDamage  : {default This source : Reference b AnObject} -> Reference b k -> Count b -> Action b
1449  Tap         : Reference b AnObject -> Action b
1440  Move        : Reference b AnObject -> Destination b -> ... -> Action b
1473  PutCounters : (c) -> Count b -> Reference b (counterScope c) -> Action b
```

Plurality is a SEPARATE, deliberate effect-level combinator over a `Bindable …
Many` group, binding `It`/`Allotment` per element and running a single-target
body:

```
1626  Each       : Bindable b Many k -> OneShotEffect (bindIt k b) -> OneShotEffect b
1632  Distribute : (amount) -> Bindable b Many k -> OneShotEffect (bindAllot k b) -> OneShotEffect b
1557  Existing   : Selection b k -> Bindable b Many k        -- a group: e.g. Existing (SelectAll pred)
1023  SelectAll  : Predicate b k -> Selection b k            -- "every match (a group)"
```

So "destroy all creatures" = `Each (Existing (SelectAll creature)) (Act (Destroy
It))`; the RON surface for that iterate-over-matches is `OverMatching(pred,
body)` (`idris/src/Ron.idr:511`). A verb NEVER receives a set — you opt into
plurality structurally.

**Rust** (`crates/deckmaste_core/src/action.rs`) — verbs take a plural-capable
`Selection`:

```
88  Destroy(Selection)        95  Counter(Selection)
85  DealDamage(Selection, Count, Reference)   // source stays a Reference
177 Tap(Selection)   175 Exile(Selection)   173 Sacrifice(Selection)
115 Move(Selection, Destination)   206 PutCounters(Selection, _, Count)   ...
```

and `Selection` (`crates/deckmaste_core/src/selection.rs:42`) is a union that
folds the singular case `Ref(Reference)` together with plural group variants
`Each(Filter)`, `Filter(Filter)`, `Choose(Quantity, Filter)`, `Random`,
`Pick`, `TopOfLibrary`, `AmongNoted`, `Those`.

**The real cost:** Rust ALSO kept `Effect::ForEach` / `Effect::DivideAmong`
(the direct ports of Idris `Each` / `Distribute`). So Rust has TWO overlapping
plurality mechanisms — `Destroy(Each(creature))` and
`ForEach(over: creature, Destroy(Ref(ThatObject)))` express the same thing —
where Idris has exactly one. The invariant "a verb acts on exactly one object;
iterate explicitly" is gone, and authors/consumers must handle both spellings.

**Proposed reconciliation (Rust idiom, structure not types):**
- Effect verbs take a single `Reference` patient (as `MoveCounters` from/to,
  `DealDamage` source, and `MayPay`/`MustPay` actor already do).
- The ONLY plurality mechanisms are the effect-level `ForEach` / `DivideAmong`
  (mirroring Idris `Each` / `Distribute`), binding the per-element anaphor the
  verb body reads (`ThatObject` / `Allotment`).
- The genuinely interactive/derived selections (`Choose`, `Random`, `Pick`,
  `TopOfLibrary`, `AmongNoted`) become group binders consumed by
  `With`/`ForEach` (the Idris `Bindable … Many` role), not verb-slot values.
- Drop the plural `Selection` variants from verb slots; `Selection` shrinks to
  the choice/group binders, no longer embedding `Ref` as "the singular escape."

Note the *type-level* half of the Idris design (`Reference b k` / `Selection b
k` indexed by `Cardinality` + `RefKind` with `auto prf` obligations) is NOT
portable — Rust has no dependent types. This ticket recovers the STRUCTURAL
invariant only.

**Scope / impact: large (L).** Touches the core `Action`/`Selection` enums,
every engine consumer (resolve/activate/cast/decide), the renderer
(`fragment::selection` callers), the oracle-text parsers/emitters in
`deckmaste_migrations`, and all card data that puts a plural `Selection` in a
verb slot. Best sequenced WITH or AFTER [[idris-naming-reconciliation]] since
both rewrite the same model surface. Recommend an audit pass first to inventory
the consumer + card-data change sites before committing to the migration.

Severity: **model fidelity / maintainability** (two ways to say one thing).
Effort: **L**.
