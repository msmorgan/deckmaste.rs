---
needs: []
---
**Fold the constructor pairs that differ by one optional cell or an axis that
already exists, finish the possessor/combat relation axes, and fold the face
records.** 2026-09-02 workbench audit (F10, F11, F12).

## Maybe-slot and existing-axis folds (F10, S)

- `AdditionalPart part anchor count followedBy:1374` / `GetsAdditionalPart who
  part count:1380` / `GetsAdditionalPartAfter who part count anchor:1383` →
  one row with `Maybe who`, `Maybe anchor`.
- `ShiftResult amt:1209` / `ShiftResultOneWay (rises : Bool) amt:1211` (ignores
  the existing `Words.ShiftDir:2385`) → `ShiftResult (dir : Maybe ShiftDir) amt`.
- `Vote voters disc ballot:1151` / `VoteStarting first voters ballot:1153`
  (drops the `disc` slot `Vote` has) → `Vote (first : Maybe) voters disc ballot`.
- `ChaosEnsues` / `ChaosEnsuesFor what:1215-1216` → `Maybe what`.
- `Does subj v e:1295` / `DoesGroup:1298` (differ by `nounPlur subj = ManyOf`
  and the `effIntro` branch) → branch on `nounPlur` inside `effIntro`.
- `Triggers.DealsCombatDamage:229` (15) / `DealsDamage n (to : DamagePatient)
  :233` (1): the `DamageKind` axis (`Events:456`) is already a slot on
  `IsDealtDamage:196` → `DealsDamage kind n to`.
- `Triggers.FlipsCoin who:291` / `FlipEvent who call:292` → `FlipsCoin who
  (Maybe FlipCall)`.
- `Phrase.CounterCompare kind r bound:323` (3) is `Compare` with a counter axis
  → `ProjAxis += CounterAxis (Maybe CounterKind)`.

## Possessor axis and combat relations (F11, S)

`Words.PossessorAxis:740` (`OwnerAx | ControllerAx`) drives only the plural
`PossessorsOf ax grp:1572`; the singulars `ControllerOf n:1568` (38) / `OwnerOf
n:1571` (5) and the predicates `ControlledBy n:249` (256) / `OwnedBy n:252` (5)
are pairs over that same axis. Fix: `PossessorOf ax n`, `PossessedBy ax n`,
with the `controlKind` gate keyed on `ax`; macros keep `controllerOf` /
`ownerOf`. Likewise `BlockerOf`/`BlockedBy`/`AttackedBy`/`AttackerOf`/
`CouldBlock`/`CouldBeBlockedBy:263-282` → `CombatRel (r : CombatRelation) (m :
Noun)` with one `Attackable`/zone gate table.

## Fold the face records (F12, S)

`Card.CardFace:301`, `AltFace:311` (= CardFace minus `cost`), `JointFace:350`
(= CardFace plus `choices`), `SharedLineHalf:397`, with four matching law
bundles `FaceLaws:361`, `AltFaceLaws:373`, `JointFaceLaws:384`,
`SharedLineHalfLaws:404`. `cardCostOk tys Nothing` (`:213`) is `True` and
`jointChoicesOk [] _` (`:341`) is `True`, so one `CardFace {cost : Maybe
ManaCost, choices : List QualitySort}` + one `FaceLaws` subsumes three of the
four. `Card.JointSingleFaced:435` folds into `SingleFaced`. Meld stays as
ruled: the reverse face is duplicated per card.

Size: S each; one round together.

Also in scope (audit §2c): the enumerated-presence `data` wrappers that spell
the presence pattern of `Maybe` slots as constructors — `Triggers.CreationVoice:71-89`
(six constructors over three `Maybe` slots), `CausedBy:92`, `VerbedVoice:177`,
`VerbPatient:159`, `VerbBecomes:168`, `BlockPartner:44`. Each becomes the
`Maybe` slots it enumerates, with the presence law as a gate where one is
needed.

Also (own-read landing, 2026-09-02): `Own` and `ItPrior` in `Phrase.Noun` are
structurally identical leading-segment reads (`countReach Bare OneOf` vs
`countOnes Object`) differing only in what the macro fills the segment with;
fold them into one segment read if the count gate can be shared.

Done when: build is 23/23; every constructor named above as a fold source is
gone from the tree; the six presence wrappers are gone; the three folded face records and their three law bundles
are gone; `PossessorAxis` and `CombatRelation` are the sole spelling of those
relations; every bench witness of a folded row is re-spelled and still
typechecks; the face-law and combat pins in `Proofs*` are re-spelled and remain
non-vacuous. Standard constraints apply.

## As landed

- F10 `AdditionalPart (who : Maybe) part (anchor : Maybe) count (followedBy : Maybe)` with `ad`/`an`/`fb`; `GetsAdditionalPart`/`GetsAdditionalPartAfter` gone; `Macros.getsAdditionalPart` added, `additionalPart`/`additionalPartThen` keep their surfaces. Pin `badAdditionalTurn` re-spelled, probed; witness `namedAdditionalPartAnchor` re-spelled (the anchor now passes `AddedPartWritten`, a tighter gate than the old bare slot).
- F10 `ShiftResult (dir : Maybe ShiftDir) amt`; `ShiftResultOneWay` gone; `Macros.shiftResult`; witness `oneWayResultShift` re-spelled.
- F10 `Vote (first : Maybe) voters disc ballot`; `VoteStarting` gone (the folded row gains the disclosure it lacked); `Macros.vote`; witness `voteStartingWithSpecifiedPlayer` re-spelled.
- F10 `ChaosEnsues (what : Maybe)`; `ChaosEnsuesFor` gone; `Macros.chaosEnsues`; witness `objectScopedChaos` re-spelled.
- F10 `DoesGroup` gone; `Does`' `effIntro`/`preIntro`/`annIntro`/`riderIntro` branch on `nounPlur subj` (`ManyOf` takes the old group intro). `nounPlur (Each _) = ManyOf`, so fourteen bench `Does (Each …)` sites (Innocent Blood, Syphon Mind, Crackling Doom, …) now take the group intro; all still typecheck. `costPaidByYou` answers `nounIsYou subj` for both pluralities (the old group row fell through to `True`). Witness `distributiveGroupSurvives` re-spelled; pin `badDistributedMillSingular` re-probed.
- F10 `DealsDamage (kind : DamageKind) n (to : DamagePatient)` with `ZoneFits (nounZone n) (damageSourceZone kind)` (battlefield only for `CombatOnly`); `DealsCombatDamage` gone; `Macros.dealsCombatDamage`.
- F10 `FlipsCoin who (call : Maybe FlipCall)`; `FlipEvent` gone; `Macros.flipsCoin`.
- F10 `CounterCompare` gone. Deviation: `ProjAxis` gains two members, `CounterAxis CounterKind` (scope `counterScope c`) and `AnyCounterAxis Kind` (scope `k`), not one `Maybe` member — `Macros.proliferate` reads the unnamed comparison at `Player` through `kindJoin`, which a `Maybe` axis with a fixed scope cannot spell. The axis block moved below `CounterKind` in Words.idr (byte-identical apart from the new members) so it can name it.
- §2c the six presence wrappers are gone; each is its `Maybe` slots plus one `So` gate under the old gate name: `bp` (`ZoneFits (patientZone what) …`), `vo` (`creationVoiceOk`), `cz` (`causedByOk`), `pt` (`verbPatientOk`), `bc` (`verbBecomesOk`), `vc` (`verbedVoiceOk`). Pins re-spelled and probed: `badCausedCounterWithAgent`, `badScryPatient`, `badVoicelessAct`, `badPassiveWithoutParticiple`, `badBecomesWithoutIntransitive`. New pins (probed): `badTokensCreatedByCauserAndPlayer`, `badTokensCreatedUnderPlural` [CR#111.2], `badTokensCreatedByMintingNoun`.
- F11 `PossessorOf ax n` and `HasPossessor ax n` (the ticket's `PossessedBy` name is taken by the zone possessor) with `possessorKind ax k` (`controlKind` / `kindLte k Object`); `ControllerOf`/`OwnerOf`/`ControlledBy`/`OwnedBy` gone; `Macros.controllerOf`/`ownerOf` added; predicates spelled `HasPossessor ControllerAx` in the bench. `Eq PossessorAxis` added for `predEq`.
- F11 `CombatRel (r : CombatRelation) m` over `combatRelOk r k km zone tys`, one table; the six predicate constructors are gone and their names are `CombatRelation`'s. `predEq` compares same-`r` `Object` relata; `AttackerOf` stays never-equal. Pins `badBlockingGraveyardRelatum`, `badCouldBlockGraveyardRelatum` re-spelled, probed.
- Own read: `ItPrior` gone; `Own OneOf` is the segment read, `Macros.itPrior` keeps its `takeDropAppend` proof under `countReach Bare OneOf`; `itPriorReadsOnlyPrefix`/`itPriorResolvesInPrefix` re-spelled.
- F12 one `CardFace {cost : Maybe ManaCost, choices : List QualitySort, …}` and one `FaceLaws (side : FaceSide) face` (`Front | Back`); `AltFace`, `JointFace`, `AltFaceLaws`, `JointFaceLaws`, `JointSingleFaced`, `AltCardBox` gone. The side carries the two laws `AltFace` held by shape: `cardCostOk Back` refuses a cost, and `boxSuitsType Back Planeswalker Nothing` admits the loyalty-less back (`planeswalkerBackWithoutLoyalty` re-spelled as its witness). `SharedLineHalf` stays: its line is the card's, not its own, so it is not a face. `Macros.frontFace`/`backFace` added; `jointCard` kept. New pin `badTransformingBackWithCost` [CR#202.3a,202.3b] (probed, `MkFaceLaws impossible`); `badSpellFlipHalf`, `badDoorHeaderOffSharedLine` re-spelled and re-probed.
- Undone: nothing in the letter. `PossessorsOf` (plural) is untouched — not named as a fold source.
