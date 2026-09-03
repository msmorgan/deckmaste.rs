---
needs: []
---
**Fix the drifted table cells and the stale workbench docs, and give the big
enums index-based `Eq`.** 2026-09-02 workbench audit (F15, F19, §5).

## Dead and drifted table cells (F19, S)

`Words.paidCostKeyword:2740` (Escape, Foretell, Bestow, Disguise, Mutate, …) is
a keyword vocabulary that `keywordFacts:2554` does not contain, so `PaidCost
(ByKeyword "Escape")` passes `PaidCostNamed` while `KeywordAbility "Escape"`
fails `KnownKeyword` — one word, two answers. `Events.deedCounterfactualOk:653`
has no consumers.

Fix: fold the paid-cost names into `keywordFacts` rows as a `paidCost : Bool`
column, delete `paidCostKeyword` and `deedCounterfactualOk`.

## Index-based `Eq` for the big enums (F15, S)

`Events.sameEventName:96` (81 clauses), `Words.Eq CounterKind` (~50),
`Words.sameQEq:322` (37) are hand-rolled. `toIx : T -> Nat` (one clause per
constructor) plus `a == b = toIx a == toIx b` halves them.
`Words.kindLteTrans:463` (73 clauses) is a proof and stays as it is.

## Documentation drift (§5)

- `idris/mtg-dev.ipkg:4-5` says "five Proofs* modules (~4,500 lines)"; there are
  eight (6,371 lines), and its "must be 16/16" gate line is stale — the full
  package is 23 modules.
- `docs/idris-workbench-closure-tables.md §3` cites line numbers into a single
  `Experimental.idr` (`:643`, `:1298`, `:6177`, `:13885`, `:16698`); the
  workbench has since been split into six modules and every anchor in that
  section is stale. Its *content* — which slots are fixed and why — is still
  the authority for `workbench-singleton-sorts`, so re-anchor it, do not
  rewrite it.
- `idris/VERIFY.md` describes the old `Semantics.idr` oracle loop (`Targeted
  [Target …]`, `That Card`) and covers nothing in `Experimental.*`, which is
  the workbench every recent ticket grew. Either add an `Experimental.*`
  section describing the real gate (`idris2 --build mtg-dev.ipkg` inner loop,
  `mtg.ipkg` full gate, the `Proofs*` pin discipline, `Unspellable`
  negative pins) or say plainly which document does.

Size: S.

Done when: build is 23/23; `paidCostKeyword` and `deedCounterfactualOk` are
gone and `keywordFacts` carries the `paidCost` column; a pin shows `PaidCost
(ByKeyword w)` and `KeywordAbility w` now agree for every keyword in the table,
and it is non-vacuous; `sameEventName`/`Eq CounterKind`/`sameQEq` are index
comparisons; the three documents above name the module set and anchors that
actually exist. Standard constraints apply.
