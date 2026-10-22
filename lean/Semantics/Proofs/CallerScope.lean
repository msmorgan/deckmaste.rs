import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions
import Semantics.Check.NumberSlots
import Semantics.Check.LocalShape

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CallerScope

private def actionResult (drs : DRS) (body : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs body).1
private def nounResult (drs : DRS) (body : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false body).1
private def selected (drs : DRS) (body : NounPhrase) : List Referent :=
  let result := stepNounPhrase 64 drs .other false body
  result.2.1.filterMap result.1.findById?
private def callerCreature : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }]
private def callerCreatureSpell : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .stack) none none none }]

private semantic_template drawTwice (amount : Amount) : Action :=
  .sequentially [draw amount, draw amount]
private semantic_template forwardedSubject (subject : NounPhrase) : NounPhrase := subject
private semantic_template subjectPower (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
private semantic_template repeatedValueSum (amount : Amount) : Amount := .arith .plus amount amount
private semantic_template literalEcho (amount : Amount) : Amount := amount
private semantic_template quantityFor (amount : Amount) : Quantity := .exactlyOf amount
private semantic_template graveyardFor (owner : NounPhrase) : Location := .zone .graveyard (some owner)
private semantic_template creatureCondition (subject : NounPhrase) : Condition := .matches subject isCreature
private semantic_template powerEquals (amount : Amount) : Predicate := .compare [.stat .power] .eq amount
private semantic_template lifeCost (amount : Amount) : Cost := .perform (.changeLife (.down amount))
private semantic_template powerBoost (subject : NounPhrase) (amount : Amount) : StaticSpec :=
  Templates.modification subject .power (.up amount)
private semantic_template unusedSubject (_chosen : NounPhrase) : Action := .draw (.exactlyOf 1)
private semantic_template secondSubject (_left right : NounPhrase) : NounPhrase := right
private semantic_template moveSubject (subject : NounPhrase) : Action :=
  .move subject .anyLocation (some graveyard) .arrival none []
private semantic_template onChosen (chosen : NounPhrase) (body : splice (NounPhrase → Action)) : Action :=
  body chosen
private abbrev SubjectBody := NounPhrase → Action
private semantic_template withSubjectBody (subject : NounPhrase) (body : splice SubjectBody) : Action := body subject
private semantic_template configuredDraw (_flag : Bool) : Action := .draw (.exactlyOf 1)
private structure Flag where
  bit : Bool
private def discardedAction (_ : Action) : Flag := ⟨true⟩
private def ordinarySubjectIdentity (subject : NounPhrase) : NounPhrase := subject
private semantic_template ordinaryFunctionConfig (_function : NounPhrase → NounPhrase) : Action := draw 1
private def applyWithForgedContext (body : AuthoringContext → NounPhrase → Action)
    (left right : NounPhrase) : Action := instruct left (body ⟨999⟩ right)
private def discardedSubjectFunction : Bool :=
  let _function : NounPhrase → NounPhrase := fun subject => subject
  true
private def subjectFunctions : List (NounPhrase → NounPhrase) := [fun subject => subject]
private semantic_template functionListConfig (_functions : List (NounPhrase → NounPhrase)) : Action := draw 1

private semantic_definition selectThenTap (subject : NounPhrase) : Action :=
  .ownScope none (.sequentially
    [.setStatus .tapped (.firstMention "select.subject" (.inCaller subject)),
      .setStatus .untapped (.laterMention "select.subject" .any)])
private semantic_definition selectThenMove (subject : NounPhrase) : Action :=
  .ownScope none (.move (.firstMention "select.subject" (.inCaller subject))
    .anyLocation (some graveyard) .arrival none [])
private semantic_definition announceThen (chosen : NounPhrase) (body : Action) : Action :=
  .ownScope none (.sequentially [.setStatus .tapped (.firstMention "announce.subject" (.inCaller chosen)),
    .inCaller body])

/-- "Draw a card. If you control three or more artifacts, <replacement> instead", the
replacement supplied by the caller. -/
private semantic_definition drawUnlessReplaced (replacement : Replacement) : Action :=
  .ownScope none (.replace (draw 1)
    (doIf (compareAmt (countOf isArtifact) .atLeast 3) (.inCaller replacement)))
/-- The same Definition, forgetting that its replacement is its caller's. -/
private semantic_definition drawUnlessReplacedHere (replacement : Replacement) : Action :=
  .ownScope none (.replace (draw 1)
    (doIf (compareAmt (countOf isArtifact) .atLeast 3) replacement))

private def explicitExpansion : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Expansion witness", types := [.sorcery],
      text := [spell (expand (drawTwice 1))] } }
theorem explicitExpansionPreservesAuthorship : explicitExpansion.authoring.onlyAuthored = true := explicitExpansion.onlyAuthored
theorem explicitExpansionChecksItsResult : explicitExpansion.card.check = [] := explicitExpansion.ok
private def explicitBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Body witness", types := [.sorcery],
      text := [spell
      (onChosen (target isCreature) (fun chosen => Primitives.Action.setStatus .tapped chosen))] } }
theorem suppliedBodyCanNameTheTemplateSubject : explicitBodyParameter.card.check = [] := explicitBodyParameter.ok
private def aliasedBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Aliased body witness", types := [.sorcery],
      text := [spell
      (withSubjectBody (target isCreature) (fun subject => Primitives.Action.setStatus .tapped subject))] } }
theorem bodyTypeAliasesRetainParameterEvidence : aliasedBodyParameter.card.check = [] := aliasedBodyParameter.ok
private def nestedBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Nested body witness", types := [.sorcery],
      text := [spell
      (onChosen (target isCreature) (fun subject => moveSubject subject))] } }
theorem parameterizedBodiesForwardSubjectsIntoNestedTemplates : nestedBodyParameter.card.check = [] := nestedBodyParameter.ok

theorem templateArgumentsAreEvaluatedAtTheirTextualUse :
    (unusedSubject it).check (DRS.ofList []) = [] ∧
      (moveSubject (secondSubject (target isArtifact) it)).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide
theorem unusedSubjectDoesNotIntroduceAReferent :
    (actionResult (DRS.ofList []) (unusedSubject (target isCreature))).referents.countP
      (fun ref => ref.det == .target) = 0 := by decide
theorem identicalArgumentsRemainSeparateSelections :
    countOnes .object (actionResult (DRS.ofList [])
      (instruct (target isCreature) (fight (target isCreature)))) = 2 := by decide

theorem namedParameterIsSelectedOnce :
    let after := actionResult (DRS.ofList []) (selectThenTap (target isCreature))
    countOnes .object after = 1 ∧
      (identityBinding "select.subject" after).map (·.id) = some 0 := by decide
theorem nestedTemplatePreservesParameterIdentity :
    let drs := nounResult (DRS.ofList []) (.firstMention "subject" (target isCreature))
    (selected drs (forwardedSubject (.laterMention "subject" .any))).map (·.id) = [0] := by decide
theorem namedParameterCannotReadAnAmountSlot :
    NounPhrase.check (DRS.ofList [letterB .x]) (.laterMention "subject" .any) =
      [.bindingIdentity "subject"] := by decide
theorem movedParameterLosesItsIdentity :
    let after := actionResult (DRS.ofList []) (selectThenMove (target isCreature))
    identityBinding "select.subject" after = none ∧
      (after.visibleReferents.filter (fun r => r.zone == some .graveyard)).length = 1 ∧
      after.findById? 0 = none := by decide

theorem suppliedBodyKeepsItsCallerPronoun :
    (announceThen (target isArtifact) (putInto it graveyard)).check callerCreature = [] := by decide
theorem suppliedBodyUpdatesItsOriginalReferent :
    let after := actionResult callerCreature (announceThen (target isArtifact) (putInto it graveyard))
    (after.visibleReferents.filter (fun r => r.ty == [.creature])).map (·.zone) = [some .graveyard] ∧
      (after.visibleReferents.filter (fun r => r.ty == [.artifact])).map (·.zone) = [some .battlefield] := by decide

theorem forgottenParameterDoesNotSurvivePrivately :
    let drs := nounResult (DRS.ofList [])
      (.firstMention "library.subject" (target (.and [.isCard, .in_ yourLibrary])))
    let after := actionResult drs shuffle
    NounPhrase.check after (.laterMention "library.subject" .any) = [.bindingIdentity "library.subject"] ∧
      countReach .bare .one after = 0 := by decide

theorem quantitiesAcceptTemplateValues : (stepQuantityWith (stepOps 64) (DRS.ofList []) (quantityFor 1)).2 = [] := by decide
theorem templateValuesPreserveLiteralFacts :
    let value := literalEcho 1
    (value.exact, value.plur, value.unit) =
      (some 1, .one, true) := by decide
theorem amountTemplatesCanReuseAValue :
    (repeatedValueSum .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem quantityParameterPreservesOneCardinality :
    ((quantityFor 1).plur) = .one := by decide
theorem zonesAcceptTemplateOwners : (graveyardFor Primitives.NounPhrase.you).check (DRS.ofList []) = [] := by decide
theorem costsAcceptTemplateValues : (lifeCost 1).check (DRS.ofList []) = [] := by decide
theorem staticSpecsAcceptTemplateValues :
    (powerBoost (target isCreature) .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem amountTemplatesAcceptSubjects : (subjectPower (target isCreature)).check (DRS.ofList []) = [] := by decide
theorem amountTemplatesIntroduceSelectionOnce :
    countOnes .object ((stepAmount 64 (DRS.ofList []) (subjectPower (target isCreature))).1) = 1 := by decide
theorem amountTemplatesRejectMissingSubjects : (subjectPower it).check (DRS.ofList []) ≠ [] := by decide
theorem predicateTemplatesAcceptValues :
    Predicate.checkAt .object (DRS.ofList [outcomeB .damageDealt]) (powerEquals .thatMuch) = [] := by decide

theorem conditionsPublishSubjectOnce :
    countOnes .object ((stepCondition 64 (DRS.ofList []) (creatureCondition (target isCreature))).1) = 1 := by decide
theorem conditionsRefineOriginalIdentity :
    let before := DRS.ofList [{
      det := .the, plur := .one,
      payload := .object [] (some .battlefield) none none none}]
    let after := (stepCondition 64 before (creatureCondition it)).1
    (after.findById? 0).map (·.ty) = some [.creature] ∧ after.nextId = before.nextId := by decide

theorem resolvedSubjectRetainsIdentityAndChangesZone :
    let refs := selected callerCreatureSpell (forwardedSubject (.becomes it))
    refs.map (·.id) = [0] ∧ refs.map (·.zone) = [some .battlefield] := by decide
theorem amountReadsPreserveNounContextUpdates :
    (((stepAmount 64 callerCreatureSpell (subjectPower (forwardedSubject (.becomes it)))).1).findById? 0).bind
      (·.zone) = some .battlefield := by decide
theorem nounCoordinationPreservesContextUpdates :
    ((nounResult callerCreatureSpell (.and [forwardedSubject (.becomes it), .you])).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem eventParametersPublishTheirOutcome :
    countOutcomes .damageDealt (GameEvent.extend (DRS.ofList [])
      (.does (.actor thisCreature) (.dealDamage .anyAmount .you))) = 1 := by decide
theorem templateUseRemainsInsideCondition :
    Action.doIf (.compareAmt 1 .eq 1) (moveSubject (target isCreature)) none =
      .doIf (.compareAmt 1 .eq 1)
        (.move (target isCreature) .anyLocation (some graveyard) .arrival none []) none := by rfl
theorem templateUseRemainsInsideRepetition :
    Action.repeat_ (.fixed 2 (moveSubject (target isCreature))) =
      .repeat_ (.fixed 2
        (.move (target isCreature) .anyLocation (some graveyard) .arrival none [])) := by rfl
theorem parameterizedGroupsRemainCountable :
    (Amount.countOf (forwardedSubject (counted (exactly 2) isCreature))).check (DRS.ofList []) = [] := by decide
theorem forwardedYouRemainsThePayer : (forwardedSubject Primitives.NounPhrase.you).isYou = true := by decide
theorem forwardedOpponentIsNotThePayer : (forwardedSubject anOpponent).isYou = false := by decide

theorem firstMentionUsesTextualOrder :
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.laterMention "late" .any),
        .setStatus .untapped (.firstMention "late" (target isCreature))]) = [.bindingIdentity "late"] := by decide
theorem firstMentionMakesLaterMentionAvailable :
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.firstMention "early" (target isCreature)),
        .setStatus .untapped (.laterMention "early" .any)]) = [] := by decide

theorem scopedNumberRegimeIsPreserved :
    (Action.ownScope none (.establish (.letterDefinition .x 2) none)).numberSlots =
      [(.lit 2, .signed)] := rfl

theorem possessorQueryPreservesSubjectUpdate :
    ((nounResult callerCreatureSpell (.possessorOf .controller (.becomes it))).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem sameSelectedSubjectIsReadBackThroughTemplate :
    (Action.exchange (.values (subjectPower (target isCreature)) (powerOf it))).check (DRS.ofList []) = [] := by decide
theorem independentValueSubjectsRemainIndependent :
    (Action.exchange (.values (subjectPower (target isCreature))
      (subjectPower (target isCreature)))).check (DRS.ofList []) = [] := by decide
private semantic_template rereadDamage (amount : Amount) : Action :=
  .sequentially [instruct .this (.dealDamage amount .you), instruct .this (.dealDamage amount .you)]
theorem expressionAmountObservesNewOutcomes :
    (rereadDamage .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [.quantOutcomeInScope 2] := by decide

private semantic_template definedPower (subject : NounPhrase) : StaticSpec :=
  ptDefinition subject .powerAlone 1
theorem scopedStaticDefinitionsRemainVisibleToPrintedBoxes :
    (definedPower thisCreature).definedStats = [.power] := by decide

theorem independentCounterpartDoesNotBecomeSelfByPayloadEquality :
    StaticSpec.check (DRS.ofList [])
      (.must (.actor (target isCreature)) (.block (target isCreature))
        {}) = [] := by decide

theorem containingNounRetainsIndependentSelection :
    let after := nounResult (DRS.ofList []) (.possessorOf .controller (forwardedSubject (target isCreature)))
    (after.visibleReferents.filter (fun ref => ref.ty == [.creature])).length = 1 ∧
      countOnes .player after = 1 ∧
      after.visibleReferents.map (fun ref => (ref.id, ref.zone)) =
        [(1, none), (0, some .battlefield)] := by decide

theorem secondSubjectSelectsOnlyItsTextuallyUsedArgument :
    let after := actionResult (DRS.ofList []) (moveSubject (secondSubject (target isArtifact) (target isCreature)))
    after.visibleReferents.map (·.ty) = [[.creature]] ∧
      after.visibleReferents.map (·.zone) = [some .graveyard] := by decide

theorem nestedForwardingPreservesMoveAndTapBehavior :
    Action.check (DRS.ofList []) (.sequentially
      [moveSubject (forwardedSubject (target isCreature)), .setStatus .tapped (that .permanent)]) =
        [.anaphor (.word .permanent) .one 0] ∧
      Action.check (DRS.ofList []) (.sequentially
        [.setStatus .tapped (forwardedSubject (target isCreature)), .setStatus .untapped (that .permanent)]) = [] := by decide

theorem sourceViewPreservesStaticDefinitionRole :
    StaticSpec.check (DRS.ofList []) (definedPower (forwardedSubject thisCreature)) = [] := by decide

theorem forwardedBareThisRetainsDefinitionZoneExemption :
    (destroy (forwardedSubject .this)).check (DRS.ofList []) = [] := by decide

theorem forwardedLibraryOwnerRetainsOpponentClassification :
    (Action.enact (.action "fateseal")
      (lookAndSortSlice (.slice .top 2 (libraryOf (forwardedSubject anOpponent))) onBottom
        [(.inOrder .anyOrder)]) none none).check
      (DRS.ofList []) = [] := by decide

theorem librarySlicePreservesExplicitOwnerUpdate :
    ((nounResult callerCreatureSpell
      (.slice .top 2 (libraryOf (.possessorOf .controller (.becomes it))))).findById? 0).bind
        (·.zone) = some .battlefield := by decide

theorem forwardedCoordinationRetainsTwoParties :
    (Action.exchange (.lifeTotals (forwardedSubject (.and [.you, anOpponent])))).check
      (DRS.ofList []) = [] := by decide

/-- error: Semantics.Definitions.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forged context witness", types := [.sorcery],
        text := [spell
        (instruct (target isCreature) (@fight ⟨1⟩ (target isCreature)))] } }

/-- error: Semantic parameter outside has no authoring evidence -/
#guard_msgs in
example (outside : NounPhrase) : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unbound callback argument witness", types := [.sorcery],
        text := [spell
        (onChosen (target isCreature) (fun _ => moveSubject outside))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.draw] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Discarded projection witness", types := [.sorcery],
        text := [spell
        (configuredDraw (discardedAction (.draw (.exactlyOf 1))).bit)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.setStatus] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Body witness", types := [.sorcery],
        text := [spell
        (onChosen (target isCreature) (fun chosen => .setStatus .tapped chosen))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.draw] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expansion witness", types := [.sorcery],
        text := [spell (expand (.draw (.exactlyOf 1)))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Expansion witness", types := [.sorcery],
        text := [spell (expand (instruct .you (drawTwice 1)))] } }

/-- error: Semantic parameter subject has no authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unannotated function alias witness", types := [.sorcery],
        text := [spell (ordinaryFunctionConfig ordinarySubjectIdentity)] } }

/-- error: Semantics.Definitions.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Higher order context witness", types := [.sorcery],
        text := [spell
        (applyWithForgedContext (@fight) (target isCreature) (target isCreature))] } }

/-- error: Semantic parameter subject has no authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Discarded function witness", types := [.sorcery],
        text := [spell (configuredDraw discardedSubjectFunction)] } }

/-- error: Semantic parameter subject has no authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Function container witness", types := [.sorcery],
        text := [spell (functionListConfig subjectFunctions)] } }

private semantic_template amountBodyInCaller (_subject : NounPhrase)
    (body : splice Amount) : Amount := body
private semantic_template predicateBodyInCaller (_subject : NounPhrase)
    (body : splice Predicate) : Predicate := body
private semantic_template eventWithSubject (subject : NounPhrase) : GameEvent :=
  .does (.actor subject) (.dealDamage .anyAmount .you)
private semantic_template playerIgnoringSubject (_subject : NounPhrase) : NounPhrase := .you
private semantic_template nestedRereadDamage (amount : Amount) : Action :=
  .sequentially [rereadDamage amount, rereadDamage amount]

theorem amountBodyReadsItsCallerSubject :
    Amount.check callerCreature
      (amountBodyInCaller (target isArtifact) (.statOf (.stat .power) it)) = [] := by decide

theorem amountBodyPreservesLiteralFacts :
    let amount := amountBodyInCaller (target isCreature) 1
    (amount.exact, amount.plur, amount.unit) =
      (some 1, .one, true) := by decide

theorem predicateBodyPreservesItsCallerReference :
    Predicate.checkAt .object callerCreature
      (predicateBodyInCaller (target isCreature) (.otherThan it)) = [] := by decide

theorem eventTemplateAcceptsItsSubject :
    (stepGameEvent 64 (DRS.ofList []) (eventWithSubject (forwardedSubject thisCreature))).2 =
      [] := by decide

theorem eventTemplatePublishesItsOutcome :
    countOutcomes .damageDealt
      (stepGameEvent 64 (DRS.ofList []) (eventWithSubject (forwardedSubject thisCreature))).1 =
      1 := by decide

theorem forwardedSourceRetainsItsSelfDefinitionRole :
    (forwardedSubject thisCreature).selfDefinedOk = true := by decide

theorem callerExitDoesNotResurrectForgottenMentions :
    let caller := nounResult (DRS.ofList [])
      (.firstMention "library.subject" (a (.and [.isCard, .in_ yourLibrary])))
    let inside := enterOwnScope caller
    let restored := leaveCaller inside (afterShuffle (enterCaller inside))
    restored.referents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(0, .player, none)] ∧ identityBinding "library.subject" restored = none ∧
      countReach .bare .one restored = 0 := by decide

theorem possessorQueryPreservesItsWholeContext :
    let after := nounResult callerCreatureSpell
      (.possessorOf .controller (forwardedSubject (.becomes it)))
    after.visibleReferents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(1, .player, none), (0, .object, some .battlefield)] := by decide

/-- The coordination leaves its conjuncts' referents as they were, and publishes their group. -/
theorem nounCoordinationPreservesItsWholeContext :
    let after := nounResult callerCreatureSpell
      (.and [forwardedSubject (.becomes it), .you])
    (after.visibleReferents.filter (·.id < 2)).map (fun ref => (ref.id, ref.kind, ref.zone, ref.partOf)) =
      [(1, .player, none, some 2), (0, .object, some .battlefield, none)] ∧
      (after.findById? 2).map (fun ref => (ref.plur, ref.kind)) =
        some (.many, .join .object .player) := by decide

theorem unusedLibraryOwnerArgumentDoesNotMutateTheCaller :
    let after := nounResult callerCreatureSpell
      (.slice .top 2 (libraryOf (playerIgnoringSubject (.becomes it))))
    after.visibleReferents.map (fun ref => (ref.id, ref.kind, ref.zone)) =
      [(2, .object, some .library), (1, .player, none), (0, .object, some .stack)] := by decide

theorem templatePatientCannotReadItsCoargument :
    Action.check (DRS.ofList []) (.establish
      (.must (.actor (target isCreature)) (.block (forwardedSubject it)) {})
      (some thisTurn)) = [.anaphor .bare .one 0] := by decide

theorem nestedTemplatesRereadTheCurrentOutcome :
    (nestedRereadDamage .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) =
      [.quantOutcomeInScope 2, .quantOutcomeInScope 3, .quantOutcomeInScope 4] := by decide

theorem amountTemplateLetterReadDoesNotIntroduceItsSourceTwice :
    let amount := literalEcho (.letter .x)
    let first := (stepAmount 64 (DRS.ofList []) amount).1
    let second := (stepAmount 64 first amount).1
    second.referents = first.referents ∧ second.nextId = first.nextId ∧
      countLetter .x second = 1 := by decide

theorem unusedSubjectDoesNotRequireABodyUse (subject : NounPhrase) :
    (unusedSubject subject).check (DRS.ofList []) = [] := by rfl

theorem returnedSecondTemplateSubjectChecks :
    (moveSubject (secondSubject (target isCreature) (target isCreature))).check (DRS.ofList []) =
      [] := by decide

theorem targetGroupsRemainCountableThroughTemplate :
    (Amount.countOf (forwardedSubject (.described (.target (exactly 2)) isCreature))).check
      (DRS.ofList []) = [] := by decide

theorem templateBareThisRetainsEnactedZoneExemption :
    (Action.enact (.action "destroy") (moveSubject .this) none none).check (DRS.ofList []) =
      [] := by decide

/-- "Draw a card. If it's a creature, tap it instead.": the replacement's action is a new clause
after its condition, so both can name the same creature. -/
theorem conditionalReplacementActionFollowsItsCondition :
    Action.check callerCreature
      (replace (draw 1) (doIf (is it isCreature) (instead (tap it)))) = [] := by
  decide

/-- A Definition's condition over its caller's replacement reads the caller's action in the
caller's scope: "it" names the caller's creature. -/
theorem conditionOverCallerReplacementReadsItsActionInTheCallersScope :
    Action.check callerCreature (drawUnlessReplaced (instead (tap it))) = [] := by
  decide

/-- It keeps the caller's own condition inside its own, read in the caller's scope too. -/
theorem conditionOverCallerReplacementReadsItsConditionInTheCallersScope :
    Action.check callerCreature
      (drawUnlessReplaced (doIf (is it isCreature) (instead (draw 2)))) = [] := by
  decide

/-- "If you sacrificed a creature this turn": a caller's condition introducing the creature its
action reads. -/
private def sacrificedACreature : Condition :=
  did you (sacrifice (a isCreature)) soFarThisTurn

/-- "… If you sacrificed a creature this turn, return it to its owner's hand instead.", the
replacement the caller's: its condition and action are read in one caller frame, the action a
new clause after the condition, so "it" names the creature the condition introduced. -/
theorem callerConditionIntroducesWhatItsActionReads :
    Action.check (DRS.ofList [])
      (drawUnlessReplaced
        (doIf sacrificedACreature (instead (returnToHand it graveyard)))) = [] := by
  decide

/-- Read in two caller frames instead, one for the condition and one for the action, the
creature the condition introduced is gone by the time the action reads "it". -/
theorem callerConditionReadInItsOwnFrameLosesItsReferent :
    Action.check (DRS.ofList [])
      (.ownScope none (.replace (draw 1)
        (.onlyIf
          (.and [compareAmt (countOf isArtifact) .atLeast 3, .inCaller sacrificedACreature])
          (.instead (.inCaller (returnToHand it graveyard)))))) = [.anaphor .bare .one 0] := by
  decide

/-- Read in the Definition's own scope instead, the caller's action names nothing. -/
theorem callerReplacementActionReadHereNamesNothing :
    Action.check callerCreature (drawUnlessReplacedHere (instead (tap it))) =
      [.anaphor .bare .one 0] := by
  decide

/-- Nor does the caller's condition. -/
theorem callerReplacementConditionReadHereNamesNothing :
    Action.check callerCreature
      (drawUnlessReplacedHere (doIf (is it isCreature) (instead (draw 2)))) =
      [.anaphor .bare .one 0] := by
  decide

end Semantics.Proofs.CallerScope
