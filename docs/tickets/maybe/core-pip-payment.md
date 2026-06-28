---
needs: []
design: true
---
[design] **Core: per-pip alternative payment (`PayPips`) for convoke / delve /
improvise — distinct from cost *reduction*.** From the 2026-06-28 idris↔rust
model audit; speculative because it competes with a planned shape.

The planned cost-modification work (`engine-cost-modification`) models convoke /
delve / improvise as `CostModifier` statics — i.e. cost *reductions*
(`CostChange`). The Idris model deliberately splits these out: convoke/delve/
improvise *pay* individual pips of the total cost, they do not *reduce* it — an
observable distinction (the total cost itself; Trinisphere; "its mana cost was
paid" / "mana spent" triggers).

Idris `StaticEffect.PayPips PipClass PayAct` (`idris/src/Core.idr`), with
`PipClass = GenericPip | ColorPip` and `PayAct = TapToPay Predicate | ExileToPay
Predicate`: "for each [pip class] of this spell's total cost, you may [tap a
matching permanent / exile a matching card] rather than pay that mana."

Adoption (design question): introduce a per-pip alternative-payment static
distinct from `CostChange` reduction, and **reconcile** with
`engine-cost-modification`, which currently plans to lump convoke/improvise/delve
in with reducers.

Verdict: **improvement / rules-correctness**, but it competes with a planned
shape — hence `[design]`, in `maybe/`. Effort: **M–L**. Related: tension with
`engine-cost-modification` (planned/); `engine-alt-costs` / `core-alt-costs` are
base-swaps, not per-pip payment.
