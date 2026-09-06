import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.MacroParameters

private semantic_macro drawTwice (amount : Amount)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .sequentially [draw amount agent, draw amount agent]

private def explicitExpansion : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Macro expansion witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (expand (drawTwice (.lit 1)))] } }

theorem explicitExpansionPreservesAuthorship : explicitExpansion.authoring.onlyMacros = true :=
  explicitExpansion.onlyMacros

theorem explicitExpansionChecksItsResult : explicitExpansion.card.check = [] :=
  explicitExpansion.ok

private semantic_macro capturedDamage (amount : capture Amount) : Instruction :=
  .sequentially [.dealDamage .this amount .you, .dealDamage .this amount .you]

private semantic_macro rereadDamage (amount : Amount) : Instruction :=
  .sequentially [.dealDamage .this amount .you, .dealDamage .this amount .you]

theorem capturedAmountSurvivesNewOutcomes :
    (capturedDamage .thatMuch).check [outcomeB .damageDealt] = [] := by decide

theorem expressionAmountObservesNewOutcomes :
    (rereadDamage .thatMuch).check [outcomeB .damageDealt] = [.quantOutcomeInScope 2] := by decide

theorem valueCaptureDoesNotIntroduceItsSourceTwice :
    let bound := captureBindings [] 3 [.value (.letter .x)]
    let value := Amount.parameter 3 0 (Amount.letter .x).shape
    value.intro (value.intro bound) = bound := by rfl

theorem valueReferenceChecksItsShape :
    let bound := captureBindings [] 3 [.value (.lit 1)]
    (Amount.parameter 3 0 (Amount.lit 2).shape).check bound = [.amountParameter 3 0] := by decide

theorem valueReferenceCannotReadASubjectSlot :
    let bound := captureBindings [] 3 [.subject .you]
    (Amount.parameter 3 0 (Amount.lit 1).shape).check bound = [.amountParameter 3 0] := by decide

theorem capturedOneRetainsCardinality :
    let value := MacroCapture.read 3 0 (Amount.lit 1)
    (value.exact, value.plur, value.unit) = (some 1, .one, true) := by decide

private semantic_macro nestedDamage (amount : capture Amount) : Instruction :=
  .sequentially [capturedDamage amount, capturedDamage amount]

theorem nestedMacrosForwardTheCapturedValue :
    (nestedDamage .thatMuch).check [outcomeB .damageDealt] = [] := by decide

theorem identicalArgumentsRemainSeparateSelections :
    countOnes .object ((fight (target creature) (target creature)).intro []) = 2 := by decide

private semantic_macro unusedSubject (chosen : capture NounPhrase) : Instruction :=
  .draw (.lit 1)

theorem aCaptureDoesNotRequireABodyUse :
    (unusedSubject (target creature)).check [] = [] := by decide

theorem unusedCapturedSyntaxStillNeedsAReferent :
    (unusedSubject it).check [] ≠ [] := by decide

private semantic_macro announceThen (chosen : capture NounPhrase) (body : splice Instruction) :
    Instruction := .sequentially [.setStatus .tapped chosen, body]

private def callerCreature : Bindings :=
  [⟨.the, .one, .object [.creature] (some .battlefield) none none none⟩]

theorem suppliedBodyKeepsItsCallerPronoun :
    (announceThen (target artifact) (.move it graveyard [])).check callerCreature = [] := by decide

theorem suppliedBodyUpdatesItsOriginalReferent :
    let after := (announceThen (target artifact) (.move it graveyard [])).intro callerCreature
    (after.find? (fun b => b.ty == [.creature])).map Binding.zone = some (some .graveyard) := by decide

private semantic_macro onChosen (chosen : capture NounPhrase)
    (body : splice (NounPhrase → Instruction)) : Instruction := body chosen

private def explicitBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Macro body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (onChosen (target creature) (fun chosen => Primitives.Instruction.setStatus .tapped chosen))] } }

theorem suppliedBodyCanNameTheMacroSubject : explicitBodyParameter.card.check = [] :=
  explicitBodyParameter.ok

private semantic_macro capturedLifeCost (amount : capture Amount) : Cost :=
  .perform (.changeLife (.down amount))

theorem costsAcceptTypedCaptures : (capturedLifeCost (.lit 1)).check [] = [] := by decide

theorem capturedCostsPreservePaymentClassifiers :
    let cost := capturedLifeCost (.lit 1)
    (cost.paidByYou, cost.tapOnce, cost.payable) = (true, true, true) := by decide

private semantic_macro capturedBoost (subject : capture NounPhrase) (amount : capture Amount) :
    StaticSpec := .modification subject .power (.up amount)

theorem staticSpecsAcceptTypedCaptures :
    (capturedBoost (target creature) .thatMuch).check [outcomeB .damageDealt] = [] := by decide

theorem callerExitDoesNotResurrectForgottenMentions :
    let before := captureBindings [] 0 [.subject (target (.inZone yourLibrary))]
    let restored := leaveCaller before (afterShuffle (enterCaller 0 before))
    countReach .bare .one restored = 0 := by decide

theorem callerExitPreservesThePrivateAlias :
    let before := captureBindings [] 0 [.subject (target (.inZone yourLibrary))]
    let restored := leaveCaller before (afterShuffle (enterCaller 0 before))
    (NounPhrase.pro .bare .one (.parameter 0 0)).zone restored = some .library := by decide

private semantic_macro capturedPower (subject : capture NounPhrase) : Amount :=
  .statOf (.stat .power) subject

private semantic_macro capturedSum (amount : capture Amount) : Amount :=
  .arith .plus amount amount

private semantic_macro announceThenRead (subject : capture NounPhrase) (body : splice Amount) :
    Amount := body

theorem amountMacrosAcceptCapturedSubjects :
    (capturedPower (target creature)).check [] = [] := by decide

theorem amountMacrosIntroduceTheirSelectionOnce :
    countOnes .object ((capturedPower (target creature)).intro []) = 1 := by decide

theorem amountMacrosRejectMalformedCapturedSubjects :
    (capturedPower it).check [] ≠ [] := by decide

theorem amountMacrosCanReturnCapturedValues :
    (capturedSum .thatMuch).check [outcomeB .damageDealt] = [] := by decide

theorem amountBodiesReadTheirCallerSubject :
    (announceThenRead (target artifact) (.statOf (.stat .power) it)).check callerCreature = [] :=
  by decide

theorem amountScopesPreserveLiteralFacts :
    let amount := announceThenRead (target creature) (.lit 1)
    (amount.exact, amount.plur, amount.unit, amount.deckBound) = (some 1, .one, true, true) :=
  by decide

private semantic_macro powerEquals (amount : capture Amount) : Predicate :=
  .compare [.stat .power] .eq amount

private semantic_macro announceThenDescribe (subject : capture NounPhrase)
    (body : splice Predicate) : Predicate := body

theorem predicatesAcceptCapturedValues :
    (powerEquals .thatMuch).check .object [outcomeB .damageDealt] = [] := by decide

theorem predicateBodiesPreserveCallerReferences :
    (announceThenDescribe (target creature) (.otherThan it)).check .object callerCreature = [] :=
  by decide

theorem predicateScopesDoNotConcealContradictions :
    (Predicate.and [.hasStatus .tapped,
      announceThenDescribe (target creature) (.hasStatus .untapped)]).check .object [] =
        [.contradictionFree] := by decide

theorem predicateScopesDoNotConcealNegatedZones :
    (Predicate.and [.inZone battlefield,
      .not (announceThenDescribe (target creature) (.inZone battlefield))]).check .object [] =
        [.zoneCoherent] := by decide

private semantic_macro secondSubject (left right : capture NounPhrase) : NounPhrase := right

private semantic_macro moveSubject (subject : capture NounPhrase) : Instruction :=
  .move subject graveyard []

theorem nounMacrosCanReturnASecondCapturedSubject :
    (moveSubject (secondSubject (target creature) (target creature))).check [] = [] := by decide

theorem nounMacroResultsRetainDistinctSelectionIdentity :
    let after := (moveSubject (secondSubject (target creature) (target creature))).intro []
    (after.map Binding.zone, countOnes .object after) =
      ([some .graveyard, some .battlefield], 2) := by decide

theorem movingAScopedNounUpdatesItsReturnedSubject :
    let after := moveIntro [] none (secondSubject (target creature) (target creature)) (some .graveyard)
    after.map Binding.zone = [some .graveyard, some .battlefield] := by decide

private semantic_macro forwardedSubject (subject : capture NounPhrase) : NounPhrase := subject

private semantic_macro moveThroughNestedAlias (subject : capture NounPhrase) : Instruction :=
  .sequentially [.move (forwardedSubject subject) graveyard [], .setStatus .tapped subject]

theorem nestedNounResultsKeepOuterInlineAliasesLive :
    (moveThroughNestedAlias (.asType .creature .this none)).check [] = [.zoneIs .battlefield] :=
  by decide

theorem closingAFrameRelocatesAddressesPastForgottenRows :
    let rows : Bindings := [⟨.the, .one, .parameterFrame 0 0 []⟩,
      ⟨.the, .one, .hidden (.object [.artifact] (some .library) none none none)⟩,
      ⟨.the, .one, .object [.creature] (some .battlefield) none none none⟩]
    closeOperandAddress rows [2] = some [0] := by decide

private semantic_macro capturedQuantity (amount : capture Amount) : Quantity := .exactlyOf amount

private semantic_macro capturedZone (owner : capture NounPhrase) : ZoneExpr :=
  .zone .graveyard (.possessedBy owner)

private semantic_macro capturedCondition (subject : capture NounPhrase) : Condition :=
  .matches subject creature

theorem quantitiesAcceptCapturedValues :
    (capturedQuantity (.lit 1)).check [] = [] := by decide

theorem zonesAcceptCapturedOwners : (capturedZone Primitives.NounPhrase.you).check [] = [] := by decide

theorem conditionsPublishCapturedSubjectsOnce :
    countOnes .object ((capturedCondition (target creature)).intro []) = 1 := by decide

theorem conditionsRetainRefinementsOfCapturedCallerSubjects :
    let caller : Bindings := [⟨.the, .one, .object [] (some .battlefield) none none none⟩]
    ((capturedCondition it).intro caller).map Binding.ty = [[.creature]] := by decide

private semantic_macro resolvedSubject (subject : splice NounPhrase) : NounPhrase :=
  .resolvedPermanent subject

private def callerCreatureSpell : Bindings :=
  [⟨.the, .one, .object [.creature] (some .stack) none none none⟩]

theorem scopedNounResultsPreserveResolvedViews :
    (MacroCapture.input (forwardedSubject (resolvedSubject it))).bind callerCreatureSpell |>.2.2.zone =
      some .battlefield := by decide

theorem amountReadsPreserveNounContextUpdates :
    let noun := forwardedSubject (.resolvedPermanent it)
    ((Amount.statOf (.stat .power) noun).intro callerCreatureSpell).map Binding.zone =
      [some .battlefield] := by decide

theorem nounCoordinationPreservesContextUpdates :
    let noun := forwardedSubject (.resolvedPermanent it)
    (nomIntro callerCreatureSpell (.and [noun, .you])).map Binding.zone = [some .battlefield] :=
  by decide

theorem conditionsRefineScopedNounResults :
    let caller : Bindings := [⟨.the, .one, .object [] (some .battlefield) none none none⟩]
    ((Condition.matches (forwardedSubject it) creature).intro caller).map Binding.ty =
      [[.creature]] := by decide

private semantic_macro capturedDamageEvent (subject : capture NounPhrase) : GameEvent :=
  .damage .any (some subject) (some .you)

theorem eventsAcceptCapturedSubjects :
    (capturedDamageEvent (.asType .creature .this none)).check [] = [] := by decide

theorem scopedEventsPublishTheirOutcome :
    countOutcomes .damageDealt ((capturedDamageEvent (.asType .creature .this none)).after []) = 1 :=
  by decide

theorem capturesStayInsideTheirCondition :
    match Instruction.doIf (.compareAmt (.lit 1) .eq (.lit 1))
        (moveSubject (target creature)) none with
    | .doIf _ (.withBindings _ [.subject _] _) none => True
    | _ => False := by trivial

theorem capturesStayInsideTheirRepetition :
    match Instruction.repeat_ (.fixed (.lit 2) (moveSubject (target creature))) with
    | .repeat_ (.fixed _ (.withBindings _ [.subject _] _)) => True
    | _ => False := by trivial

private semantic_macro capturedCount (subject : capture NounPhrase) : Amount := .countOf subject

private semantic_macro capturedPayment (payer : capture NounPhrase) : Cost :=
  .perform (.changeLife (.down (.lit 1)) payer)

theorem capturedGroupsRemainCountable :
    (capturedCount (.described (.target (exactly 2)) creature)).check [] = [] := by decide

theorem capturedYouRemainsThePayer :
    (capturedPayment Primitives.NounPhrase.you).paidByYou = true := by decide

theorem capturedSourceRetainsItsSelfDefinitionRole :
    (MacroCapture.read 0 0 (NounPhrase.asType .creature .this none)).selfDefinedOk = true := by decide

private abbrev SubjectBody := NounPhrase → Instruction

private semantic_macro withSubjectBody (subject : capture NounPhrase) (body : splice SubjectBody) :
    Instruction := body subject

private def aliasedBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Aliased body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (withSubjectBody (target creature)
          (fun subject => Primitives.Instruction.setStatus .tapped subject))] } }

theorem bodyTypeAliasesRetainParameterEvidence : aliasedBodyParameter.card.check = [] :=
  aliasedBodyParameter.ok

private structure Flag where
  bit : Bool

private def discardedInstruction (_ : Instruction) : Flag := ⟨true⟩

private semantic_macro configuredDraw (_flag : Bool) : Instruction := .draw (.lit 1)

private semantic_macro dependentSubjects (first second : capture NounPhrase) : Instruction :=
  .sequentially [.move second graveyard [], .setStatus .tapped first]

theorem dependentCapturesFollowDeclarationOrderRatherThanBodyUse :
    (dependentSubjects (target creature) it).check [] = [.zoneIs .battlefield] := by decide

private def nestedBodyCapture : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Nested body capture witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (onChosen (target creature) (fun subject => moveSubject subject))] } }

theorem parameterizedBodiesForwardSubjectsIntoNestedMacros : nestedBodyCapture.card.check = [] :=
  nestedBodyCapture.ok

/-- error: Macro Semantics.Macros.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forged context witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (@fight ⟨1⟩ (target creature) (target creature))] } }

/-- error: Semantic parameter outside has no macro-authoring evidence -/
#guard_msgs in
example (outside : NounPhrase) : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unbound callback argument witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (onChosen (target creature) (fun _ => moveSubject outside))] } }

private semantic_macro capturedLifePayment (payer : capture NounPhrase) : Instruction :=
  .changeLife (.down (.lit 1)) payer

theorem instructionScopesDoNotHideAnIncorrectPayer :
    (Cost.perform (capturedLifePayment anOpponent)).paidByYou = false := by decide

private semantic_macro definedPower (subject : capture NounPhrase) : StaticSpec :=
  .ptDefinition subject .powerAlone (.lit 1)

theorem scopedStaticDefinitionsRemainVisibleToPrintedBoxes :
    (definedPower thisCreature).definedSlots = [.powerAlone] := by decide

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.draw] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Discarded projection witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (configuredDraw (discardedInstruction (.draw (.lit 1))).bit)] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.setStatus] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Macro body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (onChosen (target creature) (fun chosen => .setStatus .tapped chosen))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.draw] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Macro expansion witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (expand (.draw (.lit 1)))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Macro expansion witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (expand (drawTwice (.lit 1) .you))] } }

private semantic_macro scopedMove (subject : capture NounPhrase) : Instruction :=
  .move subject graveyard []

theorem scopedEnactedMoveRetainsSourceZoneCheck :
    (Instruction.enact (.action "Destroy")
      (scopedMove (target (.and [creature, .inZone graveyard])))).check [] = [.zoneFits] := by
  decide

theorem capturedBareThisRetainsEnactedZoneExemption :
    (Instruction.enact (.action "Destroy") (scopedMove .this)).check [] = [] := by decide

theorem scopedLookRetainsOpponentRequirement :
    (Instruction.enact (.action "Fateseal")
      (.withBindings 5 [] (lookAndSort .you (.lit 2))) (some .you)).check [] =
        [.opponentsLibrary] := by decide

private semantic_macro capturedLook (whose : capture NounPhrase) : Instruction :=
  lookAndSort whose (.lit 2)

theorem capturedLibraryOwnerRetainsOpponentClassification :
    (Instruction.enact (.action "Fateseal") (capturedLook anOpponent) (some .you)).check [] =
      [] := by decide

theorem nestedReplacementCannotHideInScope :
    (Instruction.replace
      (.withBindings 5 [] (.replace (draw (.lit 1)) (draw (.lit 2))))
      (draw (.lit 3))).check [] = [.notInstead] := by decide

theorem scopedCoordinationRetainsTwoParties :
    twoPartiesOk (.withBindings 5 [] (.and [.you, anOpponent])) = true := by decide

theorem capturedCoordinationRetainsTwoParties :
    twoPartiesOk (MacroCapture.read 5 0 (NounPhrase.and [.you, anOpponent])) = true := by decide

theorem scopedAmountCannotConcealSelfExchange :
    (Instruction.exchange (.values (.withBindings 5 [] (powerOf thisCreature))
      (powerOf thisCreature))).check [] = [.selfExchanged] := by decide

theorem scopedDefinitionRemainsAdmissibleAsCost :
    (Instruction.establish (.withBindings 5 [] (.letterDefinition .x (.lit 2))) none).costActionOk =
      true := by decide

theorem scopedDefinitionRetainsNumberRegime :
    (Instruction.establish (.withBindings 5 [] (.letterDefinition .x (.lit 2))) none).numberSlots =
      [(.lit 2, .clamped)] := by rfl

theorem scopedCounterpartCannotConcealSelfReference :
    (Instruction.establish (deontic (target creature) .require [.core .block] .agent
      (.counterpart (.withBindings 5 [] it))) (some .thisTurn)).check [] =
        [.deonticPatientOk] := by decide

private semantic_macro capturedCounterpart (subject : capture NounPhrase) : NounPhrase := subject

theorem capturedCounterpartCannotConcealSelfReference :
    (Instruction.establish (deontic (target creature) .require [.core .block] .agent
      (.counterpart (capturedCounterpart it))) (some .thisTurn)).check [] =
        [.deonticPatientOk] := by decide

theorem independentCounterpartDoesNotBecomeSelfByPayloadEquality :
    counterpartNotSelf [] (target creature) (capturedCounterpart (target creature)) = true := by
  decide

private def ordinarySubjectIdentity (subject : NounPhrase) : NounPhrase := subject

private semantic_macro ordinaryFunctionConfig (_function : NounPhrase → NounPhrase) : Instruction :=
  draw (.lit 1)

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unannotated function alias witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (ordinaryFunctionConfig ordinarySubjectIdentity)] } }

private def applyWithForgedContext
    (body : MacroContext → NounPhrase → NounPhrase → Instruction)
    (left right : NounPhrase) : Instruction := body ⟨999⟩ left right

/-- error: Macro Semantics.Macros.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Higher order context witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
        (applyWithForgedContext (@fight) (target creature) (target creature))] } }

private def discardedSubjectFunction : Bool :=
  let _function : NounPhrase → NounPhrase := fun subject => subject
  true

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Discarded function witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (configuredDraw discardedSubjectFunction)] } }

private def subjectFunctions : List (NounPhrase → NounPhrase) := [fun subject => subject]

private semantic_macro functionListConfig (_functions : List (NounPhrase → NounPhrase)) : Instruction :=
  draw (.lit 1)

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Function container witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (functionListConfig subjectFunctions)] } }

theorem capturedAmountSubjectCannotConcealSelfExchange :
    (Instruction.exchange (.values (capturedPower thisCreature)
      (powerOf thisCreature))).check [] = [.selfExchanged] := by decide

theorem sameSelectedSubjectCannotConcealSelfExchange :
    (Instruction.exchange (.values (capturedPower (target creature))
      (powerOf it))).check [] = [.selfExchanged] := by decide

theorem independentValueSubjectsRemainIndependent :
    (Instruction.exchange (.values (capturedPower (target creature))
      (capturedPower (target creature)))).check [] = [] := by decide

theorem scopedLibrarySliceRetainsItsOwner :
    enactLibraryOwnerOk (.action "Fateseal")
      (.expose .lookAt (.cards (.withBindings 5 [] (.librarySlice .top (.lit 2) .you))) .you) =
        false := by decide

theorem callerCounterpartCannotConcealSelfReference :
    (Instruction.establish (deontic (target creature) .require [.core .block] .agent
      (.counterpart (.withBindings 5 [.subject (target creature)] (.inCaller 5 it))))
      (some .thisTurn)).check [] = [.deonticPatientOk] := by decide

private semantic_macro selfBlocking (subject : capture NounPhrase) : Instruction :=
  .establish (deontic subject .require [.core .block] .agent (.counterpart subject)) (some .thisTurn)

theorem capturedSubjectAndCounterpartStillShareIdentity :
    (selfBlocking (target creature)).check [] = [.deonticPatientOk] := by decide

theorem scopedDefinitionCannotConcealInvalidCostCapture :
    (Instruction.establish
      (.withBindings 5 [.subject (.and [.you, anOpponent])] (.letterDefinition .x (.lit 2)))
      none).costActionOk = false := by decide

theorem possessorQueriesPreserveScopedSubjectUpdates :
    (nomIntro callerCreatureSpell (.possessorOf .controller (resolvedSubject it))).map Binding.zone =
      [none, some .battlefield] := by decide

private semantic_macro resolvingPlayer (subject : capture NounPhrase) : NounPhrase :=
  Primitives.NounPhrase.you

theorem librarySlicesPreserveScopedOwnerUpdates :
    (nomIntro callerCreatureSpell
      (.librarySlice .top (.lit 2) (resolvingPlayer (.resolvedPermanent it)))).map Binding.zone =
        [some .library, some .battlefield] := by decide

theorem containingNounsRetainIndependentCaptureIntroductions :
    (nomIntro [] (.possessorOf .controller (capturedCounterpart (target creature)))).map Binding.zone =
      [none, some .battlefield] := by decide

end Semantics.Proofs.MacroParameters
