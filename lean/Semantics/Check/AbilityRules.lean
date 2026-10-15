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

private def instructionTextChoices (instruction : Instruction) (fuel : Nat := 64) : List ChoiceSort :=
  match fuel, instruction with
  | 0, _ => []
  | n+1, .inCaller body | n+1, .ownScope _ body | n+1, .bracketed body
  | n+1, .instruct _ body | n+1, .limited body _ => instructionTextChoices body n
  | _+1, .choose _ (.described (.a _) predicate) _ _ =>
    (predicate.kind?.bind choiceSortAtKind).toList
  | n+1, .sequentially steps => steps.flatMap (fun step => instructionTextChoices step n)
  | _+1, _ => []
termination_by structural fuel

private def costTextChoices (drs : DRS) (cost : Cost) : List ChoiceSort :=
  let out := (stepCost 64 drs cost).1
  (out.referents.filter (fun ref => ref.id ≥ drs.nextId)).reverse.filterMap fun ref =>
    match ref.payload with
    | .quality q => some (.quality q)
    | .player true => some .player
    | _ => none

private def staticTextChoices (drs : DRS) (spec : StaticSpec) (fuel : Nat := 64) : List ChoiceSort :=
  match fuel, spec with
  | 0, _ => []
  | n+1, .inCaller body => staticTextChoices drs body n
  | _+1, .choice _ _ sort _ _ => [sort]
  | n+1, .conjunction _ parts => parts.flatMap (fun part => staticTextChoices drs part n)
  | _+1, .addedCost cost _ _ => costTextChoices drs cost
  | _+1, .asEvent _ action | _+1, .whileHolds _ action => instructionTextChoices action
  | _+1, _ => []
termination_by structural fuel

private def abilityTextChoices (drs : DRS) (ability : Ability) (fuel : Nat := 64) : List ChoiceSort :=
  match fuel, ability with
  | 0, _ => []
  | _+1, .activated _ action _ _ _ _ | _+1, .triggered _ _ _ _ _ _ _ action
  | _+1, .spell _ action => instructionTextChoices action
  | _+1, .static spec => staticTextChoices drs spec
  | n+1, .alsoForKeywords body _ | n+1, .italicHead _ body | n+1, .qualified body _ _ _ _ =>
    abilityTextChoices drs body n
  | _+1, _ => []
termination_by structural fuel

/-- Each card-text line can read the choices announced by earlier lines. -/
def Ability.checkText (drs : DRS) (abilities : List Ability) : List Refusal :=
  (abilities.foldl (fun (context, errors) ability =>
    let errors := errors ++ Ability.check context ability
    let context := (abilityTextChoices context ability).foldl
      (fun context sort => context.push sort.binding .other false) context
    (context, errors)) (drs, [])).2

def CopyExcept.check (drs : DRS) (exception : CopyExcept) : List Refusal :=
  (stepCopyExceptWith (stepOps 64) drs exception).2

def CopyExcept.checkAll (drs : DRS) (exceptions : List CopyExcept) : List Refusal :=
  (stepCopyExceptListWith (stepOps 64) drs exceptions).2

def ZoneExpr.check (drs : DRS) (z : ZoneExpr) : List Refusal :=
  (stepZoneExprWith (stepOps 64) drs z).2

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

def PilePlacement.check (drs : DRS) (placement : PilePlacement) : List Refusal :=
  (stepPilePlacementWith (stepOps 64) drs placement).2

def MoveRider.checkAll (drs : DRS) (allowed : Option RiderKind)
    (riders : List MoveRider) : List Refusal :=
  (riders.foldl (fun (d, errors) rider =>
    let (out, errs) := stepMoveRiderWith (stepOps 64) d rider
    let found := match rider with | .entry _ => RiderKind.entry | .placement _ => .placement
    let shape := allowed.toList.flatMap (fun kind => refuse (kind == found) (.riderKind kind found))
    (out, errors ++ errs ++ shape)) (drs, [])).2

def Pile.face (drs : DRS) : Pile → Option PileFace
  | .library _ => some .faceDown
  | .graveyard _ => some .faceUp
  | .exiledWith _ => some .faceDown
  | .that => faceOfReach (.word .pile) .one drs

def UsageLimit.check (drs : DRS) (limit : UsageLimit) : List Refusal :=
  (stepUsageLimitWith (stepOps 64) drs limit).2

private def announcedColor : Ability :=
  .static (.choice .entry .any (.quality .color) none .openly)
private def readAnnouncedColor : Ability :=
  .static (.characteristicChange .any [.chosenQuality .adds (.ofChosen .theChoice .color)])

private theorem cardTextThreadsChoicesButIndependentAbilitiesDoNot :
    Ability.checkText (DRS.ofList []) [announcedColor, readAnnouncedColor] = [] ∧
      Ability.checkAll (DRS.ofList []) [announcedColor, readAnnouncedColor] =
        [.choiceRef .theChoice (.quality .color) 0] ∧
      Ability.checkText (DRS.ofList []) [readAnnouncedColor, announcedColor] =
        [.choiceRef .theChoice (.quality .color) 0] := by decide

private theorem choiceNeedsAnIndefiniteOrSelectingPhrase :
    Instruction.check (DRS.ofList []) (.choose none .you .openly none) = [.choiceClause] ∧
      Instruction.check (DRS.ofList [])
        (.choose none (.described .the .anyPlayer) .openly none) = [.choiceClause] := by decide

end Semantics
