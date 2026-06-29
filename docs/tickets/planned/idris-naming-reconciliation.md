---
needs: []
---
**Idris reference model: adopt the canonical Rust constructor names (naming
reconciliation).** From the 2026-06-29 code review: the idris→rust port spelled
new types in existing Rust conventions, so the two models diverged on several
constructor names. Decision (with the author): the **Rust idiom is canonical**;
bring the Idris model (`idris/src/*.idr`) in line so future ports don't
reintroduce drift.

Rename in the Idris model (Idris → Rust canonical):

- `Each` → `ForEach` (iteration over a matched set)
- `Distribute` → `DivideAmong` (divided distribution effect)
- `EventObject` → `EventAgent` (event source reference; also reads consistently
  with the existing `Role = Agent | Patient`)
- `Equal` / `GreaterEq` / `LessEq` → `Eq` / `AtLeast` / `AtMost` (`Cmp`;
  `Greater`/`Less` already match)
- `IsCard` / `IsEmblem` / `IsSpell` / `IsToken` / `IsAbility` →
  `Card` / `Emblem` / `Spell` / `Token` / `Ability` (`ObjectKind` — drop the
  `Is` prefix; the enum name supplies the context)
- `Opponent` / `Teammate` → `OpponentOf` / `TeammateOf` (relations — the `…Of`
  suffix, matching `ControllerOf`/`OwnerOf`)
- `GenericPip` / `ColorPip` → `Generic` / `Colored` (`PipClass`)
- `It` / `That` → `ThatObject` / `ThatPlayer` — confirm the exact anaphor mapping
  during implementation (Rust keeps these as aliases of `EventAgent`/`EventActor`);
  reconcile rather than blind-rename.

Already consistent across both models (no change): `AdditionalCost`, `MayPay`,
`MustPay`, `Unless`, `PayPips`, `TapTotal`, `MoveCounters`, `Destination`,
`Anchor`, `FromTop`/`FromBottom`, `ManaCostOf`, `CountDistinct`, `Allotment`,
`TurnOf`, and the `Quantity → Range` + named-macro approach.

Out of scope (an addition, not a rename): Rust `Count::Plus` has no Idris
counterpart — only adopt it into the Idris `Count` if you also want the operator
there.

Scope: a pure, mechanical rename across `idris/src/{Core,Spec,Cards,Macros,Ron}.idr`
(and `Experimental.idr` if kept) — no semantic change — keeping the Idris model
building. The `.idr` files are cite-scanned, so re-run `cargo xtask cite check`
after.

Severity: **improvement / consistency** (keeps the reference model and Rust in
lockstep). Effort: **M** (mechanical rename across the Idris sources + rebuild).
