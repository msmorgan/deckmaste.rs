---
needs: []
---
**Collapse the constructors the 2026-09-06 load-bearing audit found, restore
two laws the Idris port dropped, and record the inert-vocabulary ruling.**
One round, before `semantics-v2-crate` mirrors these types into Rust. The
audit classified every constructor of the six syntax layers on four checks:
a distinct `Check/*` read, a card or pin witness, macro-expressibility, and a
twin. Standard constraints apply; every affected pin is re-spelled, never
deleted.

Collapse (each has a byte-identical or absent checker read; file refs are
where the shared arm lives):

- `CharacteristicStat` duplicates `Words.Stat`; the axis is `_` at all 47
  sites (`Check/AbilityRules.lean`, `Check/Abilities.lean`). Add `defense` to
  `Stat`, retype `CharacteristicEdit.stat`, delete the type.
- `StatusCat` and `Status.category`: zero uses outside their declaration;
  `Status.clash` enumerates pairs directly. Delete, unless the status-clash
  law below wants the category back.
- `OutcomeVerb`: `Instruction.conclude` never projects it; `CoreDeed.winGame`
  and `loseGame` are read. Make `conclude` take a `CoreDeed`, delete the type.
- `Card.split` and `Card.modalDfc` have the same checker body. Either one is a
  macro over the other or the law separating the frames is missing; decide.
- `Lookback`: seven constructors, 38 witnesses, `LookbackClause.window` is
  defined and never called, `PaidFacet.readback` discards it. Give it a law
  or shrink it; `thisCombat` has no witness either way.
- `ManaUnit` belongs in `Macros.lean`: no constructor field mentions it and
  `scaledMana` eliminates it immediately.
- Pairs with one shared arm: `drawGame`/`restartGame`; `Predicate.wasCast` vs
  `castBy none`; `Repetition.again`/`againExcludingChosen`;
  `ProducedMana.asPrintedCost`/`lastNoted` and `couldProduce`/`amongColorsOf`;
  `RollWatch.anyResult` vs `Option RollWatch`; `TokenRider.entersMelded`'s
  unread `into`; `LoyaltyCost.{up,down,zero}` as `Delta Nat`.
- Unused, no witness and no read: `ChoiceDomain.nonbasicTypesOnly`,
  `Condition.gameIs`, `Condition.flipFace` (twin of `Predicate.coinCameUp`
  through `Condition.matches`), `Causing.anEffect`, `Repetition.untilCond`,
  `SpecialAction.{putCompanionIntoHand,foretell,unlockDoor}`,
  `SubtypeScope.nonbasicOnly`, `HiddenSort.choices`, `ColorFreedom.eachColor`,
  `LockState.unlocked`. `TurnPart.firstStrikeCombatDamage` is corpus-attested
  [CR#510.4] and wants a witness instead.

Checker gaps found on the way, fix rather than remove:

- `Predicate.hasSupertype` is read only in the 13-way arm at
  `Check/PhraseRules.lean` and absent from `writtenTypes`, `seedType`,
  `hasHead`, so "legendary" contributes nothing to type-line reasoning.
- `Quantity.upToOf` binds no shortfall outcome while `Amount.upTo` binds
  `outcomeB .ceilingShortfall` (`Check/Phrase.lean`).
- Two Idris laws did not survive the port: the status-clash family
  (`sameStatusVal`, `statusClash`, `statusWordOk`, `statusMarkable` in
  `idris/src/Experimental/Words.idr`) and the face laws on `FaceSide`
  (`boxSuitsType`, `cardBoxOk`, `cardCostOk` in `Experimental/Card.idr`).
  Restore each as a Lean law with its pins, or record in the landing why the
  Lean shape makes it unnecessary.
- `SharedLineHalf.name` is subject to no law while a face's name is.

Ruling to record in `docs/decisions/lean-is-the-workbench.md` (user,
2026-09-06): the 35 `Words.lean` and `Events.lean` enums no checker function
reads (`Disclosure`, `CoinFace`, `Arrangement`, `RoundMode`, `Parity`,
`ArithOp`, `SpecialAction`, `PayTimes`, …) are engine-facing vocabulary the
semantics layer carries opaquely. They were carried vocabulary in Idris too:
of the 35, three were read by an Idris law, seven only by their own derived
`Eq`, and eight by nothing. They owe no law and are not twins to prune; the
as-written rule keeps each printed word its own constructor. Four have no
introducing ticket (`LibraryEnd`, `CombatRole`, `ScaleFactor`,
`OutcomeVerb`); give each a doc comment naming its printed word.

Also fold in `workbench-last-shapes` if its five shapes are cheap here;
otherwise leave it.
