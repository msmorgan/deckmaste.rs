---
needs: []
---
**Idris grammar: a de-smell pass collapsing same-shape constructors and dropping
a redundant choice path.** From the idris↔rust structure audit (2026-06-26). These
are independent, low-risk factorings in `idris/src/Core.idr`.

1. **`Gate` + `Toll` → one `Priced`.** Both have the identical signature
   `Cost -> Deed -> StaticEffect` and differ only by *when* the cost is paid
   (declaration-time vs downstream punish — ward, [CR#702.21a]). That difference
   belongs in a parameter: `Priced : TollTiming -> Cost -> Deed -> StaticEffect`
   with `TollTiming = AtDeclaration | Downstream`.

2. **Parameterized `RemoveCounters`.** `PutCounters c Count ref` is parameterized,
   but removal is bespoke and asymmetric: `RemoveAllCounters c ref` (no count) and
   `MoveAllCounters from to` (all kinds; its own comment hedges the one-kind case
   "can't reach"). Add `RemoveCounters : CounterKind -> Count b -> Reference b
   (counterCarrier c) -> Action b` mirroring `PutCounters`; `RemoveAllCounters`
   becomes a thin convenience and `MoveAllCounters` (genuinely all-kinds, Ozolith)
   stands alone with its scope made explicit.

3. **Drop the unconstrained choice domains.** `ChooseDomain.APlayerChoice` /
   `AnObjectChoice` are unconstrained — Clone's "choose **a creature**" loses its
   filter — even though `Bindable.Choose` already carries a `Predicate`. Route
   object/player choice through the filtered `Choose`, keeping the
   `ChosenObject`/`ChosenPlayer` readback anaphors. (The bare `AColor`/`AName`/
   `ANumber` domains are correctly unconstrained — leave them.)

4. *(Optional, organizational)* Namespace `Predicate` into Characteristic / State /
   Relation sub-groups for readability. **Do NOT** add a `Where : Condition ->
   Predicate` bridge or a `Subject` anaphor: predicates are deliberately
   candidate-implicit (`Core.idr`, `It`/`Modify` comments — *"a `Predicate`'s
   candidate is already implicit"*), and a bridge would re-introduce a second
   test-language plus a binder. The flat predicate vocabulary is the unification,
   not a smell; its constructor count is the accepted first-order cost.
