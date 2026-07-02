---
needs: []
---
DONE (this claim): the probe reworked `Experimental.idr` from the two-Bool
sketch into a **parameter-shape-indexed** model and answered all three open
questions, each backed by a positive or a `failing` negative that the package
build enforces:

1. **Intrinsic is a property of the keyword**, not of the ability bearing it
   — a total `isIntrinsic : Keyword p -> Bool` over the closed constructors,
   grounded in taxonomy §10's nine (an intrinsic can still be parameterized:
   mutate carries a cost).
2. **Parameterization unifies**: the earlier sketch's free-floating
   `WithCost`/`WithPredicate` (which let "Deathtouch {2}" typecheck) collapse
   into one applicator `KA : Keyword shape -> Args shape b -> KeywordUse b`
   where `ParamShape` (None/Counted/Costed/CountedCost/Predicated/Named — the
   shapes taxonomy §10 observes) is an index the KEYWORD declares and
   `Args : ParamShape -> Endophora -> Type` is the refining type-function.
   Misparameterization has no term.
3. **Keyword actions are a separate family** (`KeywordAction shape` +
   `ActionUse`, effect position, confer nothing); `KA Scry …` is a type
   error.

Extra finding: keyword **names** are an open set (taxonomy: "data-driven,
never a closed Rust enum") — the model closes the *shape vocabulary* and the
*intrinsic set* instead, with `Custom name shape` carrying the open tail,
shape still pinned in the type. Core-adoption note: Core's flat `KeywordSpec`
carries parameters ad-hoc and cannot carry costs at all ("`KeywordSpec`
precedes `Cost`" — Morph/Flashback are bare tags); the shape-indexed spec
removes both weaknesses and is the recommended Rust-mirror direction for the
composite-keyword layer (census §6).

---

Original framing: continue the keyword-ability probe sketched in
`idris/src/Experimental.idr` (committed with the ledger-reconciliation
claim): a dependently-typed model of keywords and keyword abilities.

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
