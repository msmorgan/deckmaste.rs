import Semantics.Macros
import Semantics.Check.NumberSlots
import Semantics.Check.LocalShape

open Semantics Semantics.Macros

namespace Semantics.Proofs.MacroParameters

private def instructionResult (drs : DRS) (body : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs body).1
private def nounResult (drs : DRS) (body : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false body).1
private def selected (drs : DRS) (body : NounPhrase) : List Referent :=
  let result := stepNounPhrase 64 drs .other false body
  result.2.1.filterMap result.1.findById?
private def callerCreature : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .battlefield) none none none }]
private def callerCreatureSpell : DRS := DRS.ofList
  [{ det := .the, plur := .one, payload := .object [.creature] (some .stack) none none none }]

private semantic_template drawTwice (amount : Amount) : Instruction :=
  .sequentially [draw amount, draw amount]
private semantic_template forwardedSubject (subject : NounPhrase) : NounPhrase := subject
private semantic_template subjectPower (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
private semantic_template repeatedValueSum (amount : Amount) : Amount := .arith .plus amount amount
private semantic_template literalEcho (amount : Amount) : Amount := amount
private semantic_template quantityFor (amount : Amount) : Quantity := .exactlyOf amount
private semantic_template graveyardFor (owner : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy owner)
private semantic_template creatureCondition (subject : NounPhrase) : Condition := .matches subject creature
private semantic_template powerEquals (amount : Amount) : Predicate := .compare [.stat .power] .eq amount
private semantic_template lifeCost (amount : Amount) : Cost := .perform (.changeLife (.down amount))
private semantic_template powerBoost (subject : NounPhrase) (amount : Amount) : StaticSpec :=
  .modification subject .power (.up amount)
private semantic_template unusedSubject (_chosen : NounPhrase) : Instruction := .draw (.lit 1)
private semantic_template secondSubject (_left right : NounPhrase) : NounPhrase := right
private semantic_template moveSubject (subject : NounPhrase) : Instruction :=
  .move subject .anywhere (some graveyard) .arrival none []
private semantic_template onChosen (chosen : NounPhrase) (body : splice (NounPhrase → Instruction)) : Instruction :=
  body chosen
private abbrev SubjectBody := NounPhrase → Instruction
private semantic_template withSubjectBody (subject : NounPhrase) (body : splice SubjectBody) : Instruction := body subject
private semantic_template configuredDraw (_flag : Bool) : Instruction := .draw (.lit 1)
private structure Flag where
  bit : Bool
private def discardedInstruction (_ : Instruction) : Flag := ⟨true⟩
private def ordinarySubjectIdentity (subject : NounPhrase) : NounPhrase := subject
private semantic_template ordinaryFunctionConfig (_function : NounPhrase → NounPhrase) : Instruction := draw (.lit 1)
private def applyWithForgedContext (body : MacroContext → NounPhrase → Instruction)
    (left right : NounPhrase) : Instruction := instruct left (body ⟨999⟩ right)
private def discardedSubjectFunction : Bool :=
  let _function : NounPhrase → NounPhrase := fun subject => subject
  true
private def subjectFunctions : List (NounPhrase → NounPhrase) := [fun subject => subject]
private semantic_template functionListConfig (_functions : List (NounPhrase → NounPhrase)) : Instruction := draw (.lit 1)

private semantic_definition selectThenTap (subject : NounPhrase) : Instruction :=
  .ownScope none (.sequentially
    [.setStatus .tapped (.inCaller (.firstMention "select.subject" subject)),
      .setStatus .untapped (.laterMention "select.subject" .any)])
private semantic_definition selectThenMove (subject : NounPhrase) : Instruction :=
  .ownScope none (.move (.inCaller (.firstMention "select.subject" subject))
    .anywhere (some graveyard) .arrival none [])
private semantic_definition announceThen (chosen : NounPhrase) (body : Instruction) : Instruction :=
  .ownScope none (.sequentially [.setStatus .tapped (.firstMention "announce.subject" (.inCaller chosen)),
    .inCaller body])

private def explicitExpansion : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Macro expansion witness", types := [.sorcery],
      text := [Primitives.Ability.spell none (expand (drawTwice (.lit 1)))] } }
theorem explicitExpansionPreservesAuthorship : explicitExpansion.authoring.onlyMacros = true := explicitExpansion.onlyMacros
theorem explicitExpansionChecksItsResult : explicitExpansion.card.check = [] := explicitExpansion.ok
private def explicitBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Macro body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
      (onChosen (target creature) (fun chosen => Primitives.Instruction.setStatus .tapped chosen))] } }
theorem suppliedBodyCanNameTheMacroSubject : explicitBodyParameter.card.check = [] := explicitBodyParameter.ok
private def aliasedBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Aliased body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
      (withSubjectBody (target creature) (fun subject => Primitives.Instruction.setStatus .tapped subject))] } }
theorem bodyTypeAliasesRetainParameterEvidence : aliasedBodyParameter.card.check = [] := aliasedBodyParameter.ok
private def nestedBodyParameter : Spelled := spelled <| .singleFaced
  { characteristics := {
      name := "Nested body witness", types := [.sorcery],
      text := [Primitives.Ability.spell none
      (onChosen (target creature) (fun subject => moveSubject subject))] } }
theorem parameterizedBodiesForwardSubjectsIntoNestedMacros : nestedBodyParameter.card.check = [] := nestedBodyParameter.ok

theorem templateArgumentsAreEvaluatedAtTheirTextualUse :
    (unusedSubject it).check (DRS.ofList []) = [] ∧
      (moveSubject (secondSubject (target artifact) it)).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide
theorem unusedSubjectDoesNotIntroduceAReferent :
    (instructionResult (DRS.ofList []) (unusedSubject (target creature))).referents.countP
      (fun ref => ref.det == .target) = 0 := by decide
theorem identicalArgumentsRemainSeparateSelections :
    countOnes .object (instructionResult (DRS.ofList [])
      (instruct (target creature) (fight (target creature)))) = 2 := by decide

theorem namedParameterIsSelectedOnce :
    let after := instructionResult (DRS.ofList []) (selectThenTap (target creature))
    countOnes .object after = 1 ∧
      (identityBinding "select.subject" after).map (·.id) = some 0 := by decide
theorem nestedTemplatePreservesParameterIdentity :
    let drs := nounResult (DRS.ofList []) (.firstMention "subject" (target creature))
    (selected drs (forwardedSubject (.laterMention "subject" .any))).map (·.id) = [0] := by decide
theorem namedParameterCannotReadAnAmountSlot :
    NounPhrase.check (DRS.ofList [letterB .x]) (.laterMention "subject" .any) =
      [.bindingIdentity "subject"] := by decide
theorem movedParameterLosesItsIdentity :
    let after := instructionResult (DRS.ofList []) (selectThenMove (target creature))
    identityBinding "select.subject" after = none ∧
      (after.visibleReferents.filter (fun r => r.zone == some .graveyard)).length = 1 ∧
      after.findById? 0 = none := by decide

theorem suppliedBodyKeepsItsCallerPronoun :
    (announceThen (target artifact) (move it graveyard)).check callerCreature = [] := by decide
theorem suppliedBodyUpdatesItsOriginalReferent :
    let after := instructionResult callerCreature (announceThen (target artifact) (move it graveyard))
    (after.visibleReferents.filter (fun r => r.ty == [.creature])).map (·.zone) = [some .graveyard] ∧
      (after.visibleReferents.filter (fun r => r.ty == [.artifact])).map (·.zone) = [some .battlefield] := by decide

theorem forgottenParameterDoesNotSurvivePrivately :
    let drs := nounResult (DRS.ofList [])
      (.firstMention "library.subject" (a (.and [.isCard, .inZone yourLibrary])))
    let after := instructionResult drs shuffle
    NounPhrase.check after (.laterMention "library.subject" .any) = [.bindingIdentity "library.subject"] ∧
      countReach .bare .one after = 0 := by decide

theorem quantitiesAcceptTemplateValues : (stepQuantityWith (stepOps 64) (DRS.ofList []) (quantityFor (.lit 1))).2 = [] := by decide
theorem templateValuesPreserveLiteralFacts :
    let value := literalEcho (.lit 1)
    (value.exact, value.plur, value.unit) = (some 1, .one, true) := by decide
theorem amountTemplatesCanReuseAValue :
    (repeatedValueSum .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem quantityParameterPreservesOneCardinality :
    ((quantityFor (.lit 1)).plur) = .one := by decide
theorem zonesAcceptTemplateOwners : (graveyardFor Primitives.NounPhrase.you).check (DRS.ofList []) = [] := by decide
theorem costsAcceptTemplateValues : (lifeCost (.lit 1)).check (DRS.ofList []) = [] := by decide
theorem staticSpecsAcceptTemplateValues :
    (powerBoost (target creature) .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [] := by decide
theorem amountTemplatesAcceptSubjects : (subjectPower (target creature)).check (DRS.ofList []) = [] := by decide
theorem amountTemplatesIntroduceSelectionOnce :
    countOnes .object ((stepAmount 64 (DRS.ofList []) (subjectPower (target creature))).1) = 1 := by decide
theorem amountTemplatesRejectMissingSubjects : (subjectPower it).check (DRS.ofList []) ≠ [] := by decide
theorem predicateTemplatesAcceptValues :
    Predicate.checkAt .object (DRS.ofList [outcomeB .damageDealt]) (powerEquals .thatMuch) = [] := by decide

theorem conditionsPublishSubjectOnce :
    countOnes .object ((stepCondition 64 (DRS.ofList []) (creatureCondition (target creature))).1) = 1 := by decide
theorem conditionsRefineOriginalIdentity :
    let before := DRS.ofList [{
      det := .the, plur := .one,
      payload := .object [] (some .battlefield) none none none}]
    let after := (stepCondition 64 before (creatureCondition it)).1
    (after.findById? 0).map (·.ty) = some [.creature] ∧ after.nextId = before.nextId := by decide

theorem resolvedSubjectRetainsIdentityAndChangesZone :
    let refs := selected callerCreatureSpell (forwardedSubject (.resolvedPermanent it))
    refs.map (·.id) = [0] ∧ refs.map (·.zone) = [some .battlefield] := by decide
theorem amountReadsPreserveNounContextUpdates :
    (((stepAmount 64 callerCreatureSpell (subjectPower (forwardedSubject (.resolvedPermanent it)))).1).findById? 0).bind
      (·.zone) = some .battlefield := by decide
theorem nounCoordinationPreservesContextUpdates :
    ((nounResult callerCreatureSpell (.and [forwardedSubject (.resolvedPermanent it), .you])).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem eventParametersPublishTheirOutcome :
    countOutcomes .damageDealt (GameEvent.extend (DRS.ofList [])
      (.does (.actor thisCreature) (.dealDamage .anyAmount .you .any))) = 1 := by decide
theorem templateUseRemainsInsideCondition :
    match Instruction.doIf (.compareAmt (.lit 1) .eq (.lit 1)) (selectThenTap (target creature)) none with
    | .doIf _ (.ownScope none _) none => True | _ => False := by trivial
theorem templateUseRemainsInsideRepetition :
    match Instruction.repeat_ (.fixed (.lit 2) (selectThenTap (target creature))) with
    | .repeat_ (.fixed _ (.ownScope none _)) => True | _ => False := by trivial
theorem parameterizedGroupsRemainCountable :
    (Amount.countOf (forwardedSubject (counted (exactly 2) creature))).check (DRS.ofList []) = [] := by decide
theorem forwardedYouRemainsThePayer : (forwardedSubject Primitives.NounPhrase.you).isYou = true := by decide
theorem forwardedOpponentIsNotThePayer : (forwardedSubject anOpponent).isYou = false := by decide

theorem firstMentionUsesTextualOrder :
    Instruction.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.laterMention "late" .any),
        .setStatus .untapped (.firstMention "late" (target creature))]) = [.bindingIdentity "late"] := by decide
theorem firstMentionMakesLaterMentionAvailable :
    Instruction.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (.firstMention "early" (target creature)),
        .setStatus .untapped (.laterMention "early" .any)]) = [] := by decide

theorem scopedNumberRegimeIsPreserved :
    (Instruction.ownScope none (.establish (.letterDefinition .x (.lit 2)) none)).numberSlots =
      [(.lit 2, .clamped)] := rfl

theorem possessorQueryPreservesSubjectUpdate :
    ((nounResult callerCreatureSpell (.possessorOf .controller (.resolvedPermanent it))).findById? 0).bind
      (·.zone) = some .battlefield := by decide

theorem sameSelectedSubjectIsReadBackThroughTemplate :
    (Instruction.exchange (.values (subjectPower (target creature)) (powerOf it))).check (DRS.ofList []) = [] := by decide
theorem independentValueSubjectsRemainIndependent :
    (Instruction.exchange (.values (subjectPower (target creature))
      (subjectPower (target creature)))).check (DRS.ofList []) = [] := by decide
private semantic_template rereadDamage (amount : Amount) : Instruction :=
  .sequentially [instruct .this (.dealDamage amount .you .any), instruct .this (.dealDamage amount .you .any)]
theorem expressionAmountObservesNewOutcomes :
    (rereadDamage .thatMuch).check (DRS.ofList [outcomeB .damageDealt]) = [.quantOutcomeInScope 2] := by decide

private semantic_template definedPower (subject : NounPhrase) : StaticSpec :=
  .ptDefinition subject .powerAlone (.lit 1)
theorem scopedStaticDefinitionsRemainVisibleToPrintedBoxes :
    (definedPower thisCreature).definedSlots = [.powerAlone] := by decide

theorem templatePredicatesDoNotConcealContradictions :
    Predicate.checkAt .object (DRS.ofList []) (.and [.hasStatus .tapped, .hasStatus .untapped]) =
      [.contradictionFree] := by decide

theorem independentCounterpartDoesNotBecomeSelfByPayloadEquality :
    StaticSpec.check (DRS.ofList [])
      (.must (.actor (target creature)) (.block (target creature))
        {rider := .noRider}) = [] := by decide

theorem namedSubjectAndCounterpartStillShareIdentity :
    Instruction.check (DRS.ofList []) (.establish
      (.must (.actor (.firstMention "block.subject" (target creature)))
        (.block (.laterMention "block.subject" .any)) {rider := .noRider})
      (some thisTurn)) = [.deonticPatientOk] := by decide

theorem containingNounRetainsIndependentSelection :
    let after := nounResult (DRS.ofList []) (.possessorOf .controller (forwardedSubject (target creature)))
    (after.visibleReferents.filter (fun ref => ref.ty == [.creature])).length = 1 ∧
      countOnes .player after = 1 := by decide

theorem secondSubjectSelectsOnlyItsTextuallyUsedArgument :
    let after := instructionResult (DRS.ofList []) (moveSubject (secondSubject (target artifact) (target creature)))
    after.visibleReferents.map (·.ty) = [[.creature]] ∧
      after.visibleReferents.map (·.zone) = [some .graveyard] := by decide

theorem nestedForwardingPreservesMoveAndTapBehavior :
    Instruction.check (DRS.ofList []) (.sequentially
      [moveSubject (forwardedSubject (target creature)), .setStatus .tapped (that .permanent)]) =
        [.anaphor (.word .permanent) .one 0] ∧
      Instruction.check (DRS.ofList []) (.sequentially
        [.setStatus .tapped (forwardedSubject (target creature)), .setStatus .untapped (that .permanent)]) = [] := by decide

theorem sourceViewPreservesStaticDefinitionRole :
    StaticSpec.check (DRS.ofList []) (definedPower (forwardedSubject thisCreature)) = [] := by decide

theorem forwardedBareThisRetainsDefinitionZoneExemption :
    (destroy (forwardedSubject .this)).check (DRS.ofList []) = [] := by decide

theorem forwardedLibraryOwnerRetainsOpponentClassification :
    (Instruction.enact (.action "fateseal") (lookAndSort (forwardedSubject anOpponent) (.lit 2)) none none).check
      (DRS.ofList []) = [] := by decide

theorem librarySlicePreservesExplicitOwnerUpdate :
    ((nounResult callerCreatureSpell
      (.slice .top (.lit 2) (library (.possessorOf .controller (.resolvedPermanent it))))).findById? 0).bind
        (·.zone) = some .battlefield := by decide

theorem forwardedCoordinationRetainsTwoParties :
    (Instruction.exchange (.lifeTotals (forwardedSubject (.and [.you, anOpponent])))).check
      (DRS.ofList []) = [] := by decide

/-- error: Macro Semantics.Macros.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Forged context witness", types := [.sorcery],
        text := [Primitives.Ability.spell none
        (instruct (target creature) (@fight ⟨1⟩ (target creature)))] } }

/-- error: Semantic parameter outside has no macro-authoring evidence -/
#guard_msgs in
example (outside : NounPhrase) : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unbound callback argument witness", types := [.sorcery],
        text := [Primitives.Ability.spell none
        (onChosen (target creature) (fun _ => moveSubject outside))] } }

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
        text := [Primitives.Ability.spell none (expand (instruct .you (drawTwice (.lit 1))))] } }

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Unannotated function alias witness", types := [.sorcery],
        text := [Primitives.Ability.spell none (ordinaryFunctionConfig ordinarySubjectIdentity)] } }

/-- error: Macro Semantics.Macros.fight uses a lexical context outside its authoring scope -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Higher order context witness", types := [.sorcery],
        text := [Primitives.Ability.spell none
        (applyWithForgedContext (@fight) (target creature) (target creature))] } }

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Discarded function witness", types := [.sorcery],
        text := [Primitives.Ability.spell none (configuredDraw discardedSubjectFunction)] } }

/-- error: Semantic parameter subject has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Function container witness", types := [.sorcery],
        text := [Primitives.Ability.spell none (functionListConfig subjectFunctions)] } }

end Semantics.Proofs.MacroParameters
