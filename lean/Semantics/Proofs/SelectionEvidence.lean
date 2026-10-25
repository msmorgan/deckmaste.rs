import Semantics.Check

/-! Selection evidence: the referents a selection leaves for a later remainder, choice or loop
read. -/

namespace Semantics.Proofs.SelectionEvidence

open Semantics

private def optionalCreature : Action :=
  .withContinuation .optional
    (.choose none (.described (.a .unmarked) (.hasType .creature)) .openly none) none none

private def afterOptionalCreature : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) optionalCreature).1

theorem optionalChoiceKeepsEvidenceWithoutTheCreature :
    Action.check (DRS.ofList []) optionalCreature = [] ∧
      Predicate.checkAt .object afterOptionalCreature .notChosen = [] ∧
      NounPhrase.check afterOptionalCreature (.pro .bare .one) = [.anaphor .bare .one 0] := by decide

private def fiveCards : DRS :=
  (stepNounPhrase 64 (DRS.ofList []) .obj true
    (.slice .top 5 (.zone .library none))).1

private def optionalSubset : Action :=
  .withContinuation .optional
    (.ownScope none (.expose .reveal (.cards (.inCaller
      (.someOf (.counted (.range (some 1) (some 1))) none (.pro .bare .many)))))) none none

private def afterOptionalSubset : DRS :=
  (stepAction 64 .postEvent .obj fiveCards optionalSubset).1

theorem optionalSubsetKeepsItsGroupWithoutTheSelectedCard :
    Action.check fiveCards optionalSubset = [] ∧
      NounPhrase.check afterOptionalSubset (.theRest .object .many) = [] ∧
      countParts .object afterOptionalSubset = 0 ∧
      countOnes .object afterOptionalSubset = 0 ∧
      Predicate.checkAt .object afterOptionalSubset .notChosen = [.choiceInScope .object] := by decide

theorem missingSelectionIsStillRefused :
    Predicate.checkAt .object (DRS.ofList []) .notChosen = [.choiceInScope .object] ∧
      NounPhrase.check fiveCards (.theRest .object .many) = [.theRestFits .object] := by decide

theorem unrelatedOrClosedGroupCannotSupplyTheRemainder :
    let context := fiveCards.recordSelection .object (some 999) false
    NounPhrase.check context (.theRest .object .many) = [.theRestFits .object] ∧
      NounPhrase.check
        (stepAction 64 .postEvent .obj (DRS.ofList [])
          (.withContinuation .optional
            (.expose .reveal (.cards (.someOf (.counted (.range (some 1) (some 1))) none
              (.slice .top 5 (.zone .library none)))) ) none none)).1
        (.theRest .object .many) = [.theRestFits .object] := by decide

theorem optionalSubsetCannotPromiseOneRemainingCard :
    theOtherOk .object afterOptionalSubset = false ∧
      NounPhrase.check afterOptionalSubset (.theRest .object .one) = [.theRestFits .object] := by decide

theorem selectionEvidenceFollowsExplicitCallerScope :
    let chosen := (DRS.ofList []).recordSelection .object none true
    let inner := enterOwnScope chosen
    let caller := enterCaller inner
    inner.choiceWasPossible .object = false ∧
      caller.choiceWasPossible .object = true ∧
      (enterOwnScope caller).choiceWasPossible .object = false ∧
      (leaveCaller inner caller).choiceWasPossible .object = false ∧
      (leaveOwnScope (leaveCaller inner caller)).choiceWasPossible .object = true ∧
      (leaveOwnScope ((enterOwnScope (DRS.ofList [])).recordSelection .object none true)).choiceWasPossible .object = false := by decide

theorem selectionEvidenceTracksTheMovedGroupIdentity :
    let group := fiveCards.recordSelection .object (some 0) false
    let moved := applyZoneMove group [0] none (some .graveyard)
    moved.subsetWasPossible .object = true ∧
      moved.selectionEvidence.head?.bind (·.sourceGroup) = some 1 := by decide

theorem possibleChoiceHistoryDoesNotExportLoopReferences :
    (choiceContext afterOptionalCreature).choiceWasPossible .object = false ∧
      (leaveRelative (DRS.ofList []) afterOptionalCreature).choiceWasPossible .object = false ∧
      Ability.checkText (DRS.ofList [])
        [.spell none optionalCreature,
         .spell none (.dealDamage 1
           (.described .each (.and [.hasType .creature, .notChosen])))] =
        [.choiceInScope .object] ∧
      let out := (stepAction 64 .postEvent .obj (DRS.ofList [])
        (.doForEach (.described .each .isPlayer) optionalCreature)).1
      out.choiceWasPossible .object = true ∧ countParts .object out = 0 ∧
        countOnes .object out = 0 ∧ countManys .object out = 0 := by decide

private def loopTarget (predicate : Predicate) : Action :=
  .choose none (.described (.target (.range (some 1) (some 1))) predicate) .openly none

private def targetForEachPlayer : Action :=
  .doForEach (.described .each .isPlayer)
    (loopTarget (.and [.in_ (.zone .battlefield none),
      .hasPossessor .controller (.pro (.word .player) .one)]))

private def afterTargetLoop : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList []) targetForEachPlayer).1

theorem targetLoopPublishesAFreshPluralResult :
    Action.check (DRS.ofList []) targetForEachPlayer = [] ∧
      afterTargetLoop.referents.map (·.id) = [3, 0] ∧
      countOnes .object afterTargetLoop = 0 ∧
      countManys .object afterTargetLoop = 1 ∧
      countOnes .player afterTargetLoop = 0 ∧
      countManys .player afterTargetLoop = 1 ∧
      NounPhrase.check afterTargetLoop (.pro (.word .permanent) .many) = [] ∧
      NounPhrase.check afterTargetLoop (.pro (.word .permanent) .one) =
        [.anaphor (.word .permanent) .one 0] ∧
      NounPhrase.check afterTargetLoop (.pro .bare .one) = [.anaphor .bare .one 0] := by decide

theorem playersCanSacrificeTheCollectedTargets :
    Action.check (DRS.ofList []) (.sequentially [targetForEachPlayer,
      .instruct (.pro (.word .player) .many)
        (.ownScope (some (.action "sacrifice")) (.move (.inCaller (.pro (.word .permanent) .many))
            (.zone .battlefield none) (some (.zone .graveyard none))
            .departure []))]) = [] := by decide

private def afterPlayerLoop (body : Action) : DRS :=
  (stepAction 64 .postEvent .obj (DRS.ofList [])
    (.doForEach (.described .each .isPlayer) body)).1

theorem ordinaryLocalsAndOptionalTargetsRemainClosed :
    countManys .object (afterPlayerLoop
      (.choose none (.described (.a .unmarked) (.hasType .creature)) .openly none)) = 0 ∧
      countManys .object (afterPlayerLoop
        (.expose .reveal (.cards (.described (.target (.range (some 1) (some 1)))
          (.hasType .creature))))) = 0 ∧
      countManys .object (afterPlayerLoop
        (.withContinuation .optional (loopTarget (.hasType .creature)) none none)) = 0 := by decide

theorem separateTargetAnnouncementsStaySeparate :
    let out := afterPlayerLoop (.sequentially [loopTarget (.hasType .creature), loopTarget (.hasType .artifact)])
    countReach (.word (.type .creature)) .many out = 1 ∧
      countReach (.word (.type .artifact)) .many out = 1 ∧
      countManys .object out = 2 := by decide

theorem targetGroupUsesTheMovedSuccessorWithoutLocalIdentity :
    let out := afterPlayerLoop (.sequentially [
      .choose none (.firstMention "loop.target"
        (.described (.target (.range (some 1) (some 1))) (.hasType .creature))) .openly none,
      .move (.laterMention "loop.target" .any) .anyLocation
        (some (.zone .graveyard none)) .arrival []])
    out.referents.map (·.id) = [4, 0] ∧
      (out.findById? 4).bind (·.zone) = some .graveyard ∧
      (out.findById? 4).map (·.idents) = some [] ∧
      (out.findById? 2) = none ∧ (out.findById? 3) = none ∧
      countOnes .object out = 0 := by decide

theorem preexistingTargetsAreNotReaggregated :
    let outer : DRS := DRS.ofNumbered [{
      id := 10, det := .target, plur := .one,
      payload := .object [.artifact] (some .battlefield) none none none,
      announcedTargetResult := true }]
    let out := (stepAction 64 .postEvent .obj outer
      (.doForEach (.described .each .isPlayer) (.draw (.exactlyOf 1)))).1
    (out.findById? 10).map (·.plur) = some .one ∧
      countManys .object out = 0 ∧ countOnes .object out = 1 := by decide

private def deniedTarget : StaticSpec :=
  .cant (.actor (.firstMention "outside.target"
    (.described (.target (.range (some 1) (some 1))) (.hasType .creature))))
    (.block .any) {}

theorem prohibitionKeepsTheAnnouncedTargetIdentity :
    StaticSpec.check (DRS.ofList []) deniedTarget = [] ∧
      let out := (stepStaticSpec 64 (DRS.ofList []) deniedTarget).1
      out.referents.map (·.id) = [0] ∧
        NounPhrase.check out (.laterMention "outside.target" .any) = [] ∧
        NounPhrase.check out (.pro (.word (.type .creature)) .one) = [] := by decide

/-- A prohibition closes the hypothetical indefinite it introduces ("A creature can't block") and
publishes a universal domain ("All creatures can't block"), which a later "those creatures" reads
(`closeModalBox`; owner ruling 2026-10-24). -/
theorem prohibitionClosesItsIndefiniteAndPublishesItsUniversal :
    let indefinite := (stepStaticSpec 64 (DRS.ofList [])
      (.cant (.actor (.described (.a .unmarked) (.hasType .creature)))
        (.block .any) {})).1
    let universal := (stepStaticSpec 64 (DRS.ofList [])
      (.cant (.actor (.described .all (.hasType .creature)))
        (.block .any) {})).1
    countOnes .object indefinite = 0 ∧ countManys .object universal = 1 := by decide

theorem attachmentSourceRemainsAvailableAfterProhibition :
    let source := StaticSpec.cant (.actor (.attachHost .enchanted (.type .creature)))
      (.block .any) {}
    let out := (stepStaticSpec 64 (DRS.ofList []) source).1
    out.referents.map (·.id) = [0] ∧
      NounPhrase.check out (.pro .bare .one) = [] := by decide

private def tollCost : Cost := .scaled (.mana [.simple (.generic 1)]) (.arith .times 1 .groupSize)

/-- A toll reads the actor group after the action: a coordinated actor ("creatures and
artifacts") is the one group the coordination publishes. -/
theorem tollReadsTheActorGroupAfterTheAction :
    StaticSpec.check (DRS.ofList [])
      (.gatedBy tollCost (.actor (.described .all (.hasType .creature)))
        (.attack (some (.or [.you, .described .all (.hasType .planeswalker)])))
        {}) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy tollCost (.actor (.headed (.type .creature) .this)) (.attack none)
          {}) = [.groupSizeInScope 0] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy tollCost
          (.actor (.and [.described .all (.hasType .creature), .described .all (.hasType .artifact)]))
          (.attack none) {}) = [] := by decide

theorem postposedTollSeesItsAttachmentPatient :
    StaticSpec.check (DRS.ofList [])
      (.gatedBy (.perform (.clearDamage (.pro .bare .one))) .any
        (.block (.attachHost .enchanted (.type .creature))) {}) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy .anyCost (.actor (.described .all (.hasType .creature)))
          (.attack none) {}) = [.concreteCost] := by decide

theorem linkedProhibitionStartsANewClauseAfterItsAction :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .ownScope (some (.action "destroy")) (.move (.described .all (.hasType .creature))
              (.zone .battlefield none) (some (.zone .graveyard none))
              .departure []),
          .establish (.cant .any
            (.ownScope (some (.action "regenerate")) (.clearDamage (.inCaller (.pro .bare .many)))) {})
            (some (.between none (some (.endOf .turn none .current)))) ]) = [] := by decide

theorem addingAbilitiesRetainsTheChangedSubjectButIsolatesTheirBodies :
    let change := StaticSpec.characteristicChange (.headed (.type .planeswalker) .this)
      [.addedAbilities [.keyword (.named "indestructible") [] []]]
    let out := (stepStaticSpec 64 (DRS.ofList []) change).1
    StaticSpec.check (DRS.ofList []) change = [] ∧
      out.referents.map (·.id) = [0] ∧
      NounPhrase.check out (.pro .bare .one) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.characteristicChange (.headed (.type .planeswalker) .this)
          [.addedAbilities [.spell none (.clearDamage (.pro .bare .one))]]) =
        [.anaphor .bare .one 0] := by decide

theorem targetAgentsKeepTheirActualSpellOrAbilityIdentity :
    let spellClause := StaticSpec.cant (.by (.headed .spell .this))
      (.target (.described (.a .unmarked) (.hasType .creature))) {}
    let abilityClause := StaticSpec.cant (.by (.headed .ability .this))
      (.target (.described (.a .unmarked) (.hasType .creature))) {}
    StaticSpec.check (DRS.ofList []) spellClause = [] ∧
      StaticSpec.check (DRS.ofList []) abilityClause = [] ∧
      (stepStaticSpec 64 (DRS.ofList []) spellClause).1.referents.map (·.id) = [1] ∧
      (stepStaticSpec 64 (DRS.ofList []) abilityClause).1.referents.map (·.id) = [1] := by decide

theorem ordinaryGroupSizeIsNotAnchoredToThePerformer :
    let group := DRS.ofList [{
      det := .all, plur := .many,
      payload := .object [.creature] (some .battlefield) none none none }]
    Action.check group (.instruct .you (.draw (.exactlyOf .groupSize))) = [] ∧
      StaticSpec.check group
        (.gatedBy tollCost (.actor (.headed (.type .creature) .this))
          (.attack none) {}) = [.groupSizeInScope 0] ∧
      (stepStaticSpec 64 group
        (.gatedBy tollCost (.actor (.described .all (.hasType .creature)))
          (.attack none) {})).1.groupSizeScope = none := by decide

theorem eachModificationHasAMemberViewAndAPluralResult :
    let subject := NounPhrase.described .each (.hasType .creature)
    let modification := StaticSpec.characteristicChange subject
      [.stat .adds .power (.statOf (.stat .power) (.pro .bare .one)), .stat .adds .toughness 1]
    let out := (stepStaticSpec 64 (DRS.ofList []) modification).1
    StaticSpec.check (DRS.ofList []) modification = [] ∧
      countManys .object out = 1 ∧ countOnes .object out = 0 ∧
      NounPhrase.check (nextClause out) (.pro (.word (.type .creature)) .many) = [] ∧
      NounPhrase.check (nextClause out) (.pro (.word (.type .creature)) .one) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      Action.check (DRS.ofList []) (.sequentially [
        .establish (.characteristicChange subject
          [.stat .adds .power (.letter .x), .stat .adds .toughness (.letter .x)]) none,
        .establish (.letterDefinition .x (.distinctCount .color
          (.pro (.word (.type .creature)) .many))) none]) = [] := by decide

private def repeatedManaPayment : Action :=
  .pay (.mana [.simple (.generic 2), .simple (.specific (.of .black))]) .anyNumberOfTimes .paid none none

theorem repeatedPaymentCarriesOnlyItsCountThroughAnOffer :
    let offered := Action.withContinuation .optional repeatedManaPayment none none
    let out := (stepAction 64 .postEvent .obj (DRS.ofList []) offered).1
    Action.check (DRS.ofList []) offered = [] ∧
      countOutcomes .repeatCount out = 1 ∧ countOnes .object out = 0 ∧
      Amount.check out .thatMuch = [] ∧
      Action.check (DRS.ofList [])
        (.triggerReflexively offered none
          (.putCounters .thatMuch (.printed (.named "p1p1Counter"))
            (.headed (.type .creature) .this))) = [] := by decide

theorem ordinaryUnpaidAndProspectivePaymentsDoNotInventRepeatCounts :
    let mana := Cost.mana [.simple (.generic 1)]
    let once := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (.pay mana .once .paid none none)).1
    let unpaid := (stepAction 64 .postEvent .obj (DRS.ofList [])
      (.pay mana .anyNumberOfTimes .unpaid none none)).1
    let prospective := (stepAction 64 .preEvent .obj (DRS.ofList []) repeatedManaPayment).1
    Amount.check once .thatMuch = [.quantOutcomeInScope 0] ∧
      Amount.check unpaid .thatMuch = [.quantOutcomeInScope 0] ∧
      Amount.check prospective .thatMuch = [.quantOutcomeInScope 0] ∧
      countOutcomes .repeatCount
        (stepAction 64 .postEvent .obj (DRS.ofList [])
          (.withContinuation .optional (.repeat_ (.fixed 2 (.draw (.exactlyOf 1)))) none none)).1 = 0 := by decide

theorem oneDiceBatchHasOneNumericResultRead :
    let batch := DRS.ofList [outcomeB .diceRolled]
    let out := (stepAmount 64 batch (.theOutcome .rollResult)).1
    Amount.check batch (.theOutcome .rollResult) = [] ∧
      countOutcomes .diceRolled out = 1 ∧ countOutcomes .rollResult out = 0 ∧
      countAmountOutcomes out = 1 ∧ Amount.check out .thatMuch = [] ∧
      Amount.check batch (.theOutcome .repeatCount) = [.outcomeInScope .repeatCount 0] := by decide

theorem numericDiceResultRequiresOneUnambiguousRoll :
    Amount.check (DRS.ofList []) (.theOutcome .rollResult) = [.outcomeInScope .rollResult 0] ∧
    Amount.check (DRS.ofList [outcomeB .diceRolled, outcomeB .diceRolled])
      (.theOutcome .rollResult) = [.outcomeInScope .rollResult 2] ∧
    Amount.check (DRS.ofList [outcomeB .rollResult, outcomeB .rollResult])
      (.theOutcome .rollResult) = [.outcomeInScope .rollResult 2] ∧
    Amount.check (DRS.ofList [outcomeB .diceRolled, outcomeB .rollResult])
      (.theOutcome .rollResult) = [] ∧
    Amount.check (DRS.ofList [outcomeB .rollResult]) (.theOutcome .rollResult) = [] ∧
    countOutcomes .diceRolled (DRS.ofList [outcomeB .rollResult]) = 0 := by decide

theorem performedReplacementResultTakesPriorityOverItsDiceCount :
    let prospective := DRS.ofList [outcomeB .diceRolled]
    let performed := (stepAction 64 .postEvent .obj prospective
      (.rollDice .thatMuch .thoseDice .anyResult)).1
    Action.check prospective (.rollDice .thatMuch .thoseDice .anyResult) = [] ∧
      countOutcomes .diceRolled performed = 1 ∧
      countOutcomes .rollResult performed = 1 ∧
      Amount.check performed (.theOutcome .rollResult) = [] ∧
      Condition.check performed (.anyResultIs .atLeast 1) = [] := by decide

end Semantics.Proofs.SelectionEvidence
