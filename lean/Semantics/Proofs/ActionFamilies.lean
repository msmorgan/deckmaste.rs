import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.ActionFamilies

private def nounZone (drs : DRS) (noun : NounPhrase) : Option Zone :=
  let result := stepNounPhrase 64 drs .other false noun
  result.2.1.head? >>= fun id => (result.1.findById? id).bind (·.zone)

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

private def outcomeReferents (drs : DRS) : List Referent :=
  drs.referents.filter fun ref => match ref.payload with | .outcome _ => true | _ => false

theorem okBlockednessWithoutChangingBlockingRelations :
    Action.check (DRS.ofList []) (.combat thisCreature (.participation (.blocked false))) = [] := by decide

theorem okRemoveAttackedPlaneswalkerFromCombat :
    Action.check (DRS.ofList []) (.combat (.headed (.type .planeswalker) .this)
      (.participation .outsideCombat)) = [] := by decide

theorem okRemoveAttackedBattleFromCombat :
    Action.check (DRS.ofList []) (.combat (.headed (.type .battle) .this)
      (.participation .outsideCombat)) = [] := by decide

theorem okAttachToPlayer :
    Action.check (DRS.ofList []) (.attachment .attached thisAura (some .you)) = [] := by decide

theorem okDetachWithoutKnowingOldHost :
    Action.check (DRS.ofList []) (.attachment .unattached thisArtifact none) = [] := by decide

theorem okExtraTurnPublishesTurnReference :
    TurnRef.check (actionResult (DRS.ofList []) (.addTurnPart .turn none (.lit 1) none)) .that = [] := by
  decide

/-- "Target creature takes an extra turn after this one." An extra turn is taken by a player
[CR#500.7]: the turn's taker is the player the action is handed to, and a permanent is
not one. -/
theorem badExtraTurnWithoutPlayer :
    Action.check (DRS.ofList []) (instruct (target isCreature) (.addTurnPart .turn none (.lit 1) none))
      = [.kindMismatch .player .object] := by decide

def emblemAbilities : List Ability := [.static (Templates.abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "haste")))]

theorem okCreateSeveralEmblems :
    Action.check (DRS.ofList []) (.createObject (.lit 2) (.emblem none emblemAbilities) true none) = [] := by decide

theorem okEmblemPublication :
    (actionResult (DRS.ofList [])
      (.createObject (.lit 1) (.emblem none emblemAbilities) true none)).sameReferents
      (DRS.ofList [({ det := .a, plur := .one, payload := .object [] (some .command) none none none } : Referent)]) = true := by decide

theorem copiedTokenUsesItsSelectedSourceRatherThanANeighbor :
    let source := nounResult (DRS.ofList []) (.firstMention "token.source" (target isArtifact))
    let before := nounResult source (target isCreature)
    let after := actionResult before (.createObject (.lit 1)
      (.token (.copyOf (.laterMention "token.source" .any) []) []) true none)
    (after.findById? 0).map (·.ty) = some [.artifact] ∧
      (after.findById? 1).map (·.ty) = some [.creature] ∧
      (after.findById? before.nextId).map (·.payload) =
        some (.object [.artifact] (some .battlefield) none (some .token) none) := by decide


private def exiledAbility : DRS :=
  actionResult (DRS.ofList []) (.move (target (.abilityHead .anyActivated)) .anyLocation (some exileZone) .arrival none [])

private def graveyardAbility : DRS :=
  actionResult (DRS.ofList []) (.move (target (.abilityHead .anyActivated)) .anyLocation (some graveyard) .arrival none [])

theorem exiledAbilityKeepsItsLocation :
    nounZone exiledAbility (that .ability) = some .exile := by decide

theorem exiledAbilityRemainsAnAbilityReference :
    NounPhrase.checkAt none exiledAbility (that .ability) = [] := by decide

theorem exiledAbilityIsNotAStackReference :
    NounPhrase.checkAt none exiledAbility (that .stack) = [.anaphor (.word .stack) .one 0] := by decide

theorem exiledAbilityIsNotASpellOrCard :
    NounPhrase.checkAt none exiledAbility (that .spell) = [.anaphor (.word .spell) .one 0] ∧
    NounPhrase.checkAt none exiledAbility (that .card) = [.anaphor (.word .card) .one 0] := by decide

theorem movementPublishesItsRequiredDestination :
    nounZone graveyardAbility (that .ability) = some .graveyard := by decide

theorem movementAcceptsAnAbilitySubject :
    Action.check (DRS.ofList []) (.move (target (.abilityHead .anyActivated)) .anyLocation (some graveyard) .arrival none []) = [] := by decide

theorem graveyardDestinationAcceptsCounterRiders :
    Action.check (DRS.ofList []) (.move (target (.abilityHead .anyActivated)) .anyLocation (some graveyard) .arrival none [(.withCounters (.lit 1) (.printed p1p1Counter) .fresh)]) = [] := by decide

theorem pronounMovePreservesCopyOriginAndOuterBindings :
    (actionResult (DRS.ofList [
      { det := .the, plur := .one, payload := .ability (some .copy) },
      { det := .the, plur := .one, payload := .player false }])
      (.move (that .ability) .anyLocation (some graveyard) .arrival none [])).sameReferents
      (DRS.ofList [
        { det := .the, plur := .one, payload := .ability (some .copy) (some .graveyard) },
        { det := .the, plur := .one, payload := .player false }]) = true := by decide

theorem abilityUnionRetainsCommonLocation :
    (unionPayload (.ability none (some .exile)) (.ability none (some .exile))).map
      (fun pair => pair.2.zone) = some (some .exile) := by decide

theorem abilityUnionDoesNotInventACommonLocation :
    (unionPayload (.ability none (some .exile)) (.ability none (some .stack))).map
      (fun pair => pair.2.zone) = some none := by decide

theorem mixedUnionRetainsCommonLocation :
    (unionPayload (.ability none (some .exile))
      (.object [] (some .exile) none none none)).map
      (fun pair => pair.2.zone) = some (some .exile) := by decide

theorem abilityElementRetainsItsLocation :
    nounZone (DRS.ofList []) (.eachOf (allOf (.and [.abilityHead .anyActivated, .in_ exileZone]))) = some .exile := by decide


/-- Marked damage remains even when a permanent stops being a creature [CR#120.6]. -/
theorem clearDamageAdmitsALand :
    Action.check (DRS.ofList []) (.clearDamage (target isLand)) = [] := by decide

theorem clearDamageRejectsAPlayer :
    Action.check (DRS.ofList []) (.clearDamage .you) =
      [.kindMismatch .object .player] := by decide

theorem clearDamagePublishesItsSubject :
    NounPhrase.checkAt (some .object) (actionResult (DRS.ofList []) (.clearDamage (target isCreature))) it =
      [] := by decide


theorem clearDamageDoesNotPublishDamageDealt :
    outcomeReferents (actionResult (DRS.ofList []) (.clearDamage (target isCreature))) = [] := by decide


theorem movingThisAbilityAllocatesFreshIdentity :
    let after := actionResult (DRS.ofList [])
      (.move thisAbility .anyLocation (some exileZone) .arrival none [])
    after.findById? 0 = none ∧
      after.visibleReferents.map (fun r => (r.id, r.det, r.plur, r.payload, r.idents)) =
        [(1, .the, .one, .ability none (some .exile), [])] := by decide


private def operandKey (i : Nat) : BindingId := "operand." ++ toString i

private def operand (i : Nat) : NounPhrase := .laterMention (operandKey i) .any

private def operandContext (drs : DRS) (operands : List NounPhrase) : DRS :=
  operands.zipIdx.foldl (fun context (noun, i) =>
    (stepNounPhrase 64 context .other false (.firstMention (operandKey i) noun)).1) drs

private def operandIdentity (drs : DRS) (i : Nat) : Option ReferentId :=
  (drs.visibleReferents.find? (fun ref => ref.idents.contains (operandKey i))).map (·.id)

private def movedOperand (drs : DRS) (subject : NounPhrase) (to : Option Zone) : DRS :=
  (stepAction 64 .postEvent .obj drs
    (.move subject .anyLocation (to.map fun zone => .zone zone none) .arrival none [])).1

/-- Equal-looking target phrases name two objects, rather than one shared payload. -/
theorem distinctOperandsHaveDistinctIdentities :
    let bs := operandContext (DRS.ofList []) [target isCreature, target isCreature]
    (operandIdentity bs 0, operandIdentity bs 1) = (some 0, some 1) := by decide

theorem capturesDoNotDuplicateOrdinaryPronounCandidates :
    countReach .bare .one (operandContext (DRS.ofList []) [target isCreature]) = 1 := by decide

theorem captureUpdatesTheOriginalReference :
    let bs := operandContext (DRS.ofList []) [target isCreature, target isCreature]
    let moved := movedOperand bs (operand 0) (some .exile)
    (firstReach (.word .card) .one moved).bind (·.zone) = some .exile ∧
      (moved.findById? 1).bind (·.zone) = some .battlefield ∧
      (moved.visibleReferents.filter (fun ref => ref.zone == some .exile)).length = 1 ∧
      operandIdentity moved 0 = none := by decide

theorem laterMentionsDoNotShiftCapturedOperands :
    let bs := operandContext (DRS.ofList []) [target isCreature]
    let more := nounResult bs (target isLand)
    (((more.findById? 0).map (·.ty)).getD [], nounZone more (operand 0)) = ([.creature], some .battlefield) := by decide

/-- X already exists: a later target's X does not create a second numeric definition. -/
theorem captureUsesActualNumericIntroductions :
    let bs : DRS := [letterB .x, ({ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none, idents := [] } : Referent)]
    let old : NounPhrase := .pro .bare .one 
    let second := target (.and [isCreature, .compare [.stat .power] .eq (.letter .x)])
    let bound := operandContext bs [old, second]
    (operandIdentity bound 0, operandIdentity bound 1) = (some 0, some 2) := by decide

theorem nestedCaptureUpdatesTheOuterOperand :
    let outer := operandContext (DRS.ofList []) [target isCreature]
    let inner := operandContext outer [operand 0]
    let moved := movedOperand inner (operand 0) (some .exile)
    moved.visibleReferents.map (·.zone) = [some .exile] ∧
      operandIdentity moved 0 = none := by decide

theorem nestedCaptureUpdatesAnInlineOuterOperand :
    let outer := operandContext (DRS.ofList []) [.headed .ability .this]
    let inner := operandContext outer [operand 0]
    let moved := movedOperand inner (operand 0) none
    moved.visibleReferents.map (·.zone) = [none] ∧
      operandIdentity moved 0 = none := by decide

theorem fightAcceptsAnExistingReference :
    Action.check (nounResult (DRS.ofList []) (target isCreature))
      (instruct it (fight (target isCreature))) = [] := by decide

theorem fightAcceptsItsOwnSource :
    Action.check (DRS.ofList []) (instruct thisCreature (fight thisCreature)) = [] := by decide

theorem regenerationApplicationChecks :
    Action.check (DRS.ofList []) (regenerationApplication thisCreature) = [] := by decide

theorem regenerationInstallationPublishesNoImmediateOutcome :
    outcomeReferents (actionResult (DRS.ofList []) (regenerate thisCreature)) = [] := by decide

theorem counterDoesNotLeaveAnAbilityOnTheStack :
    nounZone (actionResult (DRS.ofList [])
      (counter (target (.abilityHead .anyActivated))))
      (that .ability) = some .graveyard := by decide

theorem counterPreservesTheAbilityCategory :
    NounPhrase.checkAt none (actionResult (DRS.ofList [])
      (counter (target (.abilityHead .anyActivated))))
      (that .ability) = [] := by decide

theorem counterIsSingleEnactedMove (subject : NounPhrase) :
    counter subject =
      .ownScope (some (.action "counter"))
        (.enact (.action "counter")
          (.move (.inCaller (.firstMention "counter.subject" subject))
            (.zone .stack none) (some graveyard) .arrival none []) none none) := by rfl

theorem losingCountersPublishesTheSharedRemovalOutcome :
    countOutcomes .countersRemoved (actionResult (DRS.ofList [])
      (loseCounters (some (.printed (.named "poison")))
        (some (.lit 1)) .you)) = 1 := by decide

theorem ownPowerCanReadAnExistingReference :
    Action.check (nounResult (DRS.ofList []) (target isCreature)) (dealDamageOwnPower it .you) = [] := by decide


theorem forgottenOperandDoesNotRedirectToNeighbor :
    let bound := operandContext (DRS.ofList [])
      [a (.and [.isCard, .in_ yourLibrary]), target isCreature]
    let neighbor := operandIdentity bound 1
    let shuffled := afterShuffle bound
    let moved := movedOperand shuffled (operand 0) (some .exile)
    NounPhrase.checkAt none shuffled (operand 0) = [.bindingIdentity (operandKey 0)] ∧
      neighbor.isSome = true ∧
      operandIdentity shuffled 1 = neighbor ∧
      (neighbor.bind moved.findById?).bind (·.zone) = some .battlefield ∧
      moved.sameReferents shuffled = true := by decide

theorem postposedCounterConditionReadsTheUnmovedSpell :
    Action.check (DRS.ofList []) (.doOnlyIf (counter (target isSpell))
      (costWasPaid (.byKeyword (.named "kicker")) none it) none) = [] := by decide


theorem losingCountersAmountReadsItsPlayer :
    Action.check (DRS.ofList []) (loseCounters
      (some (.printed (.named "poison"))) (some (lifeTotalOf they)) (a (.isOpponentOf .you))) = [] := by decide

theorem regenerationCanProtectANoncreaturePermanent :
    Action.check (DRS.ofList []) (regenerate (target isArtifact)) = [] := by decide


theorem alternativeDoesNotObserveThePrimaryBranchesMove :
    Action.check (DRS.ofList []) (.doOnlyIf (putInto (target isSpell) exileZone)
      (.matches it (.colorIs .red)) (some (counter it))) = [] := by decide


theorem forgottenAliasesAreBothRefused :
    let bound := operandContext (DRS.ofList []) [a (.and [.isCard, .in_ yourLibrary]), it]
    let shuffled := afterShuffle bound
    NounPhrase.checkAt none shuffled (operand 0) = [.bindingIdentity (operandKey 0)] ∧
      NounPhrase.checkAt none shuffled (operand 1) = [.bindingIdentity (operandKey 1)] := by decide

theorem forgottenNestedAliasIsRefused :
    let outer := operandContext (DRS.ofList []) [a (.and [.isCard, .in_ yourLibrary])]
    let inner := operandContext outer [operand 0]
    NounPhrase.checkAt none (afterShuffle inner) (operand 0) =
      [.bindingIdentity (operandKey 0)] := by decide

theorem conditionalShuffleForgetsPrivateOperand :
    Action.check (DRS.ofList []) (.ownScope none (.sequentially [
      .expose .lookAt (.cards (.firstMention (operandKey 0) (a (.and [.isCard, .in_ yourLibrary])))),
      .doIf (.matches .you .anyPlayer) (.shuffle (libraryOf .actor)) none,
      .move (operand 0) .anyLocation (some graveyard) .arrival none []])) =
      [.bindingIdentity (operandKey 0)] := by decide

theorem capturePreservesTheResolvedPermanentView :
    Action.check (DRS.ofList []) (instruct
      (.resolvedPermanent (target (.and [isSpell, isCreature]))) (fight (target isCreature))) = [] := by decide


theorem alternativeReadsTheAmountWithoutObservingTheUntakenDamage :
    let context := (fun action => (stepAction 64 .preEvent .obj (DRS.ofList []) action).1) (Action.instruct .this (.dealDamage (.lit 3) .you .any))
    Amount.check context .thatMuch = [] ∧
      Condition.check context (.dealtThisWay .anyPlayer) = [.damageDealtInScope] := by decide

end Semantics.Proofs.ActionFamilies
