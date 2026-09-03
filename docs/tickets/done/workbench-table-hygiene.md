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

## As landed

- F19: `Words.paidCostKeyword` is gone; `KeywordFacts` gained a `paidCost :
  Bool` column, True on every row `keywordCosts` already admitted
  (`CostParam`/`CompoundParam`) and on the nineteen folded names, which are
  new rows (Disturb was already one). `paidCostNamed (ByKeyword w)` and
  `(ByNthKeyword _ w)` read the column. New rows' cells: `paramShape` from
  each keyword's printed form in its Comprehensive Rules definition
  (`Impending N—[cost]`/`Awaken N—[cost]` are `CompoundParam NumberHead`,
  `Casualty N` is `NumberParam`, `Gift a [something]` is `SubjectParam` — the
  one shape choice with no exact analogue); `regime` is `Just AtCasting` where
  the rule says the ability functions while the spell is on the stack and
  `Nothing` for the graveyard/hand/any-zone keywords (Escape, Foretell,
  Bestow, Disguise, Evoke, Harmonize); `counterEligible` False for all by
  [CR#122.1b]; the card-kind cells from the oracle corpus
  (`data/derived/cards.jsonl`: Overload, Cleave, Harmonize, Awaken, Buyback,
  Replicate are spell-only; Escape, Foretell, Casualty, Gift both).
- F19: `Events.deedCounterfactualOk` deleted (no reader in the tree).
- Pin: `Words.paidCostAgrees` — every table word is `knownKeyword` and its
  `paidCostNamed (ByKeyword w)` answer equals its `paidCost` cell. Probed
  non-vacuous by re-pointing `paidCostNamed (ByKeyword kw)` at
  `keywordCosts kw`: the proof fails with `Mismatch between: True and False`
  (Casualty and Gift are paid costs that are not cost parameters).
- F15: `sameEventName` was already `eventIx` + 3 clauses (facts-tables round);
  untouched. `Eq CounterKind` is `counterIx` + 3 clauses. `Eq QualitySort` is
  `qualityIx` (SubtypeQ t = 5 + `cardTypeIx` t); `sameQEq` is the
  `sameCardTypeEq` round-trip idiom over `qualityAt`/`qualityAtIx`, and
  `sameQRefl` is one clause. `kindLteTrans` untouched.
- Docs: `mtg-dev.ipkg` header names the Proofs* set without counts and points
  the full gate at `./scripts/build`; closure tables §3 anchors are
  `Module.decl` names, the third heading names the modules split from
  `Experimental.idr`, and where the decl is gone or the slot has since opened
  the anchor says so (`Effect.AsThough`, `Phrase.OfLastChosen`, `Macros.youAnd`,
  `Effect.CantMoreThan`, `Effect.ChosenQuality`, the retired `AnyTarget`,
  `capSubjectOk`, `TagBody`, `partUse`, `badBottomMill`) — the slot reasoning
  is otherwise as written; `VERIFY.md` gained "The `Experimental.*`
  workbench" (modules, what a pin is, the non-vacuity probe, the two builds,
  the cite gates).
- Undone: nothing. `cite check --list-noncompliant` reports one pre-existing
  string in `docs/tickets/done/workbench-facts-tables.md:95`, outside this
  round's diff.

## Landing record

- Construction count: unchanged (no constructor added or removed);
  `keywordFacts` +19 rows, +1 column; `cr-citations.lock` unchanged.
- Assurance: restored 0; re-spelled 0; ignored 0; added 1
  (`paidCostAgrees`); removed 0.
- Positive artifacts: `./scripts/build` 23/23, 0 Error, 0 Warning; `cite
  check` 0 stale; diff audit 1 site read ([CR#120.7] on the `IsSource` row,
  anchor-only change).
- Deviations and additions: the §3 anchors carry a short "since opened" or
  "retired" note where the decl no longer matches the entry; Gift's
  `SubjectParam`.
- STOPs: none.
