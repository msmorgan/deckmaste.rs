import Semantics.Check.Abilities

/-!
# Semantics.Check.AbilityRules

Minimal core ability-level rule checks.
-/

namespace Semantics

def Ability.checkAll (drs : DRS) (abs : List Ability) : List Refusal :=
  (stepAbilityList 64 drs abs).2

private def choiceSortAtKind : Kind → Option ChoiceSort
  | .quality q => some (.quality q)
  | .player => some .player
  | _ => none

private def actionTextChoices (action : Action) (fuel : Nat := 64) : List ChoiceSort :=
  match fuel, action with
  | 0, _ => []
  | n+1, .inCaller body | n+1, .ownScope _ body | n+1, .bracketed body
  | n+1, .instruct _ body | n+1, .limited body _ => actionTextChoices body n
  | _+1, .choose _ (.described (.a _) predicate) _ _ =>
    (predicate.kind?.bind choiceSortAtKind).toList
  | n+1, .sequentially steps => steps.flatMap (fun step => actionTextChoices step n)
  | _+1, _ => []
termination_by structural fuel

/-- Choices made as part of a replaced event, by what a replacement does `also`
("As this enters, choose a color"), which later lines read [CR#614.1c]. -/
private def replacementTextChoices (replacement : Replacement) (fuel : Nat := 64) :
    List ChoiceSort :=
  match fuel, replacement with
  | 0, _ => []
  | n+1, .inCaller body | n+1, .onlyIf _ body => replacementTextChoices body n
  | _+1, .also action => actionTextChoices action
  | _+1, .instead _ | _+1, .withRiders _ => []
termination_by structural fuel

private def publishTextChoices (drs : DRS) (sorts : List ChoiceSort) : DRS :=
  sorts.foldl (fun context sort => context.push sort.binding .other false) drs

private def publicCostResult (ref : Referent) : Bool :=
  !ref.isActorFrame && ref.isPublic &&
    (match ref.payload with
     | .object .. | .pile .. => ref.zone.isSome
     | _ => true)

private def costTextContext (drs : DRS) (cost : Cost) : DRS :=
  let out := (stepCost 64 drs cost).1
  let additions := out.visibleReferents.filter fun ref =>
    ref.id ≥ drs.nextId && publicCostResult ref
  nextClause { drs with
    referents := additions ++ drs.referents,
    nextId := Nat.max drs.nextId out.nextId,
    currentClause := Nat.max drs.currentClause out.currentClause }

private def staticTextContext (drs : DRS) (spec : StaticSpec) (fuel : Nat := 64) : DRS :=
  match fuel, spec with
  | 0, _ => drs
  | n+1, .inCaller body => staticTextContext drs body n
  | n+1, .conjunction _ parts =>
    parts.foldl (fun context part => staticTextContext context part n) drs
  | _+1, .addedCost cost _ _ => costTextContext drs cost
  | _+1, .asEvent _ action | _+1, .whileHolds _ action =>
    publishTextChoices drs (actionTextChoices action)
  | _+1, .replace _ replacement _ _ _ =>
    publishTextChoices drs (replacementTextChoices replacement)
  | _+1, _ => drs
termination_by structural fuel

/-- Choices and public additional-cost results available to later lines of card text. -/
def Ability.textContext (drs : DRS) (ability : Ability) (fuel : Nat := 64) : DRS :=
  match fuel, ability with
  | 0, _ => drs
  | _+1, .activated _ action _ _ _ _ | _+1, .triggered _ _ _ _ _ _ _ action
  | _+1, .spell _ action => publishTextChoices drs (actionTextChoices action)
  | _+1, .static spec => staticTextContext drs spec
  | n+1, .alsoForKeywords body _ | n+1, .italicHead _ body | n+1, .qualified body _ _ _ _ =>
    Ability.textContext drs body n
  | _+1, _ => drs
termination_by structural fuel

/-- Each card-text line can read prior choices and public additional-cost results. -/
def Ability.checkText (drs : DRS) (abilities : List Ability) : List Refusal :=
  (abilities.foldl (fun (context, errors) ability =>
    let errors := errors ++ Ability.check context ability
    (Ability.textContext context ability, errors)) (drs, [])).2

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
    let reach := Reach.verbed (.action "tap") none .attributive
    let actualIds := (resolveCandidates reach .many actual).map (·.id)
    let publishedIds := (resolveCandidates reach .many published).map (·.id)
    actualIds = [8] ∧ publishedIds = actualIds ∧ published.nextId = actual.nextId ∧
      (published.findById? 7).map (·.payload) = some (.quality .color) ∧
      Ability.checkText initial [additionalTapGroup, .spell none (.draw .groupSize)] = [] ∧
      Ability.checkAll initial [additionalTapGroup, .spell none (.draw .groupSize)] =
        [.groupSizeInScope 0] := by decide

private theorem additionalCostDoesNotPublishAHiddenGroup :
    Ability.checkText (DRS.ofList [])
      [additionalHiddenGroup, .spell none (.draw .groupSize)] = [.groupSizeInScope 0] := by decide

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
[CR#404.2], the cards exiled with a source face down [CR#406.4], and a made pile the face it was
given. -/
def Location.face (drs : DRS) (loc : Location) (fuel : Nat := 8) : Option PileFace :=
  match fuel, loc with
  | 0, _ => none
  | n + 1, .fromEnd _ _ inner => inner.face drs n
  | _ + 1, .zone .library _ => some .faceDown
  | _ + 1, .zone .graveyard _ => some .faceUp
  | _ + 1, .exiledWith _ => some .faceDown
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
  .static (.characteristicChange .any [.chosenQuality .adds (.ofChosen .theChoice .color)])

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
       .spell none (.changeControl (.pro .bare .one))] = [.anaphor .bare .one 0] := by decide

private theorem selectedPlayerCanBeReadWithinTheSameAction :
    Action.check (DRS.ofList [])
      (.sequentially
        [.choose none (.described (.a .unmarked) .anyPlayer) .openly none,
         .draw (.devotion (.described .the (.chosenPlayer .theChoice)) (.lit .red) none)]) = [] := by decide

private theorem grantedAbilityReadsChoicesButNotOrdinaryOuterObjects :
    Ability.check (DRS.ofList [qualityB .color])
      (.static (.characteristicChange .any [.addedAbilities
        [.keyword (.named "protection") [.quality (.ofChosen .theChoice .color)] []]])) = [] ∧
      Ability.check (DRS.ofList [kindBinding .object])
        (.static (.characteristicChange .any
          [.addedAbilities [.spell none (.changeControl (.pro .bare .one))]])) =
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
