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
