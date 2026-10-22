import Semantics.Check.Abilities

/-!
# Semantics.Check.AbilityRules

Minimal core ability-level rule checks.
-/

namespace Semantics

def Ability.checkAll (drs : DRS) (abs : List Ability) : List Refusal :=
  (stepAbilityList 64 drs abs).2

/-- Choices, cost results, added targets, and the objects a printed triggered or activated ability
acted on, available to later lines of card text [CR#607.2a]. -/
def Ability.textContext (drs : DRS) (ability : Ability) : DRS :=
  Ability.textContextWith (stepOps 64) drs ability

/-- Each card-text line can read prior choices and results of the lines before it. -/
def Ability.checkText (drs : DRS) (abilities : List Ability) : List Refusal :=
  checkTextWith (stepOps 64) drs abilities

private def additionalTapGroupCost : Cost :=
  .perform (.ownScope (some (.action "tap"))
    (.enact (.action "tap")
      (.setStatus .tapped (.described (.count (.range (some 0) none) none)
        (.and [.hasType .creature, .in_ (.zone .battlefield none)]))) none none))

private def additionalTapGroup : Ability :=
  .static (.addedCost additionalTapGroupCost false .once)

private def additionalHiddenGroup : Ability :=
  .static (.addedCost
    (.perform (.move
      (.described (.count (.range (some 0) none) none)
        (.and [.isCard, .in_ (.zone .hand none)]))
      (.zone .hand none) (some (.zone .library none)) .arrival none []))
    false .once)

private theorem additionalCostPublishesTheActualGroupIdentity :
    let initial := DRS.ofList [{ det := .the, plur := .one, payload := .quality .color, id := 7 }]
    let actual := (stepCost 64 initial additionalTapGroupCost).1
    let published := Ability.textContext initial additionalTapGroup
    let actualIds := (participialCandidates (.deed (.action "tap")) (some .many) none false actual).map (·.id)
    let publishedIds :=
      (participialCandidates (.deed (.action "tap")) (some .many) none false published).map (·.id)
    actualIds = [8] ∧ publishedIds = actualIds ∧ published.nextId = actual.nextId ∧
      (published.findById? 7).map (·.payload) = some (.quality .color) ∧
      Ability.checkText initial [additionalTapGroup, .spell none (.draw (.exactlyOf .groupSize))] = [] ∧
      Ability.checkAll initial [additionalTapGroup, .spell none (.draw (.exactlyOf .groupSize))] =
        [.groupSizeInScope 0] := by decide

/-- The group an additional cost puts from a hand into a library is published like any other: a
hidden destination is the engine's concern, not the text's (`semantics-v3.md` §0.1 item 4). -/
private theorem additionalCostPublishesAGroupPutIntoALibrary :
    Ability.checkText (DRS.ofList [])
      [additionalHiddenGroup, .spell none (.draw (.exactlyOf .groupSize))] = [] := by decide

def CopyExcept.check (drs : DRS) (exception : CopyExcept) : List Refusal :=
  (stepCopyExceptWith (stepOps 64) drs exception).2

def CopyExcept.checkAll (drs : DRS) (exceptions : List CopyExcept) : List Refusal :=
  (stepCopyExceptListWith (stepOps 64) drs exceptions).2

def Location.check (drs : DRS) (z : Location) : List Refusal :=
  (stepLocationWith (stepOps 64) drs z).2

def PlayPayment.check (drs : DRS) (p : PlayPayment) : List Refusal :=
  (stepPlayPaymentWith (stepOps 64) drs p).2

def ProducedMana.check (drs : DRS) (p : ProducedMana) : List Refusal :=
  (stepProducedManaWith (stepOps 64) drs p).2

def Duration.check (drs : DRS) (d : Duration) : List Refusal :=
  (stepDurationWith (stepOps 64) drs d).2

def TurnRef.check (drs : DRS) (turn : TurnRef) : List Refusal :=
  (stepTurnRefWith (stepOps 64) drs turn).2

def CharacteristicBundle.check (drs : DRS) (bundle : CharacteristicBundle) : List Refusal :=
  (stepCharacteristicBundleWith (stepOps 64) drs bundle).2

def TextChange.check (drs : DRS) (change : TextChange) : List Refusal :=
  (stepTextChangeWith (stepOps 64) drs change).2

def Timing.check (drs : DRS) (timing : Timing) : List Refusal :=
  (stepTimingWith (stepOps 64) drs timing).2

def CounterKind.check (kind : CounterKind) : List Refusal := refuse kind.known .knownCounter

def Ballot.check (drs : DRS) (ballot : Ballot) : List Refusal :=
  (stepBallotWith (stepOps 64) drs ballot).2

def sortedDomainCheck (drs : DRS) (sort : ChoiceSort)
    (domain : Option ChoiceDomain) : List Refusal :=
  (sortedDomainCheckWith (stepOps 64) drs sort domain).2

def MoveRider.check (drs : DRS) (placement : MoveRider) : List Refusal :=
  (stepMoveRiderWith (stepOps 64) drs placement).2

def MoveRider.checkAll (drs : DRS) (riders : List MoveRider) : List Refusal :=
  (riders.foldl (fun (d, errors) rider =>
    let (out, errs) := stepMoveRiderWith (stepOps 64) d rider
    (out, errors ++ errs)) (drs, [])).2

/-- The face of the pile a location names: a library face down [CR#401.2], a graveyard face up
[CR#404.2], and a made pile the face it was given. The cards exiled with a source have no face of
their own: exiled cards are kept face up by default [CR#406.3], and those exiled with a source
are face down only if its linked ability exiled them so [CR#406.6,607.2a]. -/
def Location.face (drs : DRS) (loc : Location) (fuel : Nat := 8) : Option PileFace :=
  match fuel, loc with
  | 0, _ => none
  | n + 1, .fromEnd _ _ inner => inner.face drs n
  | _ + 1, .zone .library _ => some .faceDown
  | _ + 1, .zone .graveyard _ => some .faceUp
  | _ + 1, .pile _ => faceOfReach (.word .pile) .one drs
  | _ + 1, _ => none
termination_by structural fuel

def UsageLimit.check (drs : DRS) (limit : UsageLimit) : List Refusal :=
  (stepUsageLimitWith (stepOps 64) drs limit).2

private def announcedColor : Ability :=
  .static (.replace (.would .any (.move .this .anyLocation (some (.zone .battlefield none)) .arrival none []))
    (.also (.choose none (.described (.a .unmarked) (.qualityNoun .color none)) .openly none))
    none .repeatedly none)
private def readAnnouncedColor : Ability :=
  .static (.characteristicChange .any [.quality .adds (.chosen (.ofChosen .theChoice .color))])

private theorem cardTextThreadsChoicesButIndependentAbilitiesDoNot :
    Ability.checkText (DRS.ofList []) [announcedColor, readAnnouncedColor] = [] ∧
      Ability.checkAll (DRS.ofList []) [announcedColor, readAnnouncedColor] =
        [.choiceRef .theChoice (.quality .color) 0] ∧
      Ability.checkText (DRS.ofList []) [readAnnouncedColor, announcedColor] =
        [.choiceRef .theChoice (.quality .color) 0] := by decide

private theorem choiceNeedsAnIndefiniteOrSelectingPhrase :
    Action.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] ∧
      Action.check (DRS.ofList [])
        (.choose none (.described .the (.superlative .max (.playerStat .lifeTotal) .anyPlayer)) .openly none) = [.choiceClause] := by decide

private theorem cardTextDoesNotExportOrdinaryObjectMentions :
    Ability.checkText (DRS.ofList [])
      [.spell none (.choose none (.described (.a .unmarked) (.hasType .creature)) .openly none),
       .spell none (.establish (.controlGrant .actor (.pro .bare .one)) none)] = [.anaphor .bare .one 0] := by decide

private theorem selectedPlayerCanBeReadWithinTheSameAction :
    Action.check (DRS.ofList [])
      (.sequentially
        [.choose none (.described (.a .unmarked) .anyPlayer) .openly none,
         .draw (.exactlyOf (.devotion (.described .the (.chosenPlayer .theChoice)) (.lit .red) none))]) = [] := by decide

private theorem grantedAbilityReadsChoicesButNotOrdinaryOuterObjects :
    Ability.check (DRS.ofList [qualityB .color])
      (.static (.characteristicChange .any [.addedAbilities
        [.keyword (.named "protection") [.quality (.ofChosen .theChoice .color)] []]])) = [] ∧
      Ability.check (DRS.ofList [kindBinding .object])
        (.static (.characteristicChange .any
          [.addedAbilities [.spell none (.establish (.controlGrant .actor (.pro .bare .one)) none)]])) =
          [.anaphor .bare .one 0] := by decide

private def chooserFrameContext (kind : Kind) (ids : List ReferentId) : DRS :=
  DRS.ofList [
    { id := 30, det := .the, plur := .one, payload := .actor {
      kind, isYou := false, ascribable := false, refIds := ids } },
    { id := 24, det := .the, plur := .one, payload := .player false },
    { id := 23, det := .each, plur := .one, payload := .player false },
    { id := 22, det := .the, plur := .many, payload := .player false } ]

private theorem theirChoiceUsesTheActivePlayerIdentity :
    choiceModeChecks (chooserFrameContext .player [23]) .theirChoice = [] ∧
      choiceModeChecks (chooserFrameContext .player [23, 24]) .theirChoice = [.chooserInScope 2] ∧
      choiceModeChecks (chooserFrameContext .object [23]) .theirChoice = [.chooserInScope 0] := by decide

private theorem eachPlayerChoosesLocallyAmongUnrelatedPlayers :
    Action.check (DRS.ofList [kindBinding .player, kindBinding .player])
      (.instruct (.described .each .anyPlayer)
        (.choose none (.described (.a .theirChoice) (.hasType .creature)) .openly none)) = [] := by decide

end Semantics
