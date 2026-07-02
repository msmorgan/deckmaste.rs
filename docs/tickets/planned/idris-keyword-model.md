---
needs: []
---
Continue the keyword-ability probe sketched in `idris/src/Experimental.idr`
(committed with the ledger-reconciliation claim): a dependently-typed model of
keywords and keyword abilities.

What the sketch has so far:

- `Keyword : {intrinsic : Bool} -> {params : Bool} -> Type` — each keyword
  declares by type index whether it is an engine intrinsic and whether it is
  parameterized.
- `KeywordSpec b` — `Bare`, `WithCost (Cost b)`, `WithPredicate (Predicate b
  k)` — the keyword-plus-rider shapes ("Flying", "Ward {2}", "Hexproof from
  blue").
- `KeywordAbility b` — the ability-level wrapper (`Intrinsic` so far).

Goal: let the type system pin the taxonomy `docs/rules-taxonomy.md` §10
derives empirically (9 intrinsic abilities + 16 intrinsic actions vs ~215
composites) *before* the Rust composite-keyword layer (census §6, 157 rows)
commits to shapes — which keywords are parameterized, which carry
costs/predicates, and what a composite expansion is entitled to reference.
Same probe discipline as the anaphor model: closed enums refined by total
type-functions; a keyword shape that will not typecheck marks a real taxonomy
error.

Open questions the model should answer:

- Is `intrinsic` a property of the keyword or of the ability bearing it?
- Do parameterized keywords (`params=True`) unify with
  `WithCost`/`WithPredicate` or stay a distinct axis?
- Where do keyword *actions* (scry, mill, …) sit relative to keyword
  abilities?
