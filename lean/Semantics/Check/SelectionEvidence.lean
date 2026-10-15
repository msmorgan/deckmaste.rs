import Semantics.Check.AbilityRules

namespace Semantics

private def optionalCreature : Instruction :=
  .withContinuation .optional
    (.choose none (.described (.a .unmarked) (.hasType .creature)) .openly none) none none

private def afterOptionalCreature : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) optionalCreature).1

private theorem optionalChoiceKeepsEvidenceWithoutTheCreature :
    Instruction.check (DRS.ofList []) optionalCreature = [] ∧
      Predicate.checkAt .object afterOptionalCreature .notChosen = [] ∧
      NounPhrase.check afterOptionalCreature (.pro .bare .one) = [.anaphor .bare .one 0] := by decide

private def fiveCards : DRS :=
  (stepNounPhrase 64 (DRS.ofList []) .obj true
    (.slice .top (.lit 5) (.library .bare))).1

private def optionalSubset : Instruction :=
  .withContinuation .optional
    (.ownScope none (.expose .reveal (.cards (.inCaller
      (.someOf (.counted (.range (some 1) (some 1))) none (.pro .bare .many)))))) none none

private def afterOptionalSubset : DRS :=
  (stepInstruction 64 .postEvent .obj fiveCards optionalSubset).1

private theorem optionalSubsetKeepsItsGroupWithoutTheSelectedCard :
    Instruction.check fiveCards optionalSubset = [] ∧
      NounPhrase.check afterOptionalSubset (.theRest .object .many) = [] ∧
      countParts .object afterOptionalSubset = 0 ∧
      countOnes .object afterOptionalSubset = 0 ∧
      Predicate.checkAt .object afterOptionalSubset .notChosen = [.choiceInScope .object] := by decide

private theorem missingSelectionIsStillRefused :
    Predicate.checkAt .object (DRS.ofList []) .notChosen = [.choiceInScope .object] ∧
      NounPhrase.check fiveCards (.theRest .object .many) = [.theRestFits .object] := by decide

private theorem unrelatedOrClosedGroupCannotSupplyTheRemainder :
    let context := fiveCards.recordSelection .object (some 999) false
    NounPhrase.check context (.theRest .object .many) = [.theRestFits .object] ∧
      NounPhrase.check
        (stepInstruction 64 .postEvent .obj (DRS.ofList [])
          (.withContinuation .optional
            (.expose .reveal (.cards (.someOf (.counted (.range (some 1) (some 1))) none
              (.slice .top (.lit 5) (.library .bare)))) ) none none)).1
        (.theRest .object .many) = [.theRestFits .object] := by decide

private theorem optionalSubsetCannotPromiseOneRemainingCard :
    theOtherOk .object afterOptionalSubset = false ∧
      NounPhrase.check afterOptionalSubset (.theRest .object .one) = [.theRestFits .object] := by decide

private theorem selectionEvidenceFollowsExplicitCallerScope :
    let chosen := (DRS.ofList []).recordSelection .object none true
    let inner := enterOwnScope chosen
    let caller := enterCaller inner
    inner.choiceWasPossible .object = false ∧
      caller.choiceWasPossible .object = true ∧
      (enterOwnScope caller).choiceWasPossible .object = false ∧
      (leaveCaller inner caller).choiceWasPossible .object = false ∧
      (leaveOwnScope (leaveCaller inner caller)).choiceWasPossible .object = true ∧
      (leaveOwnScope ((enterOwnScope (DRS.ofList [])).recordSelection .object none true)).choiceWasPossible .object = false := by decide

private theorem selectionEvidenceTracksTheMovedGroupIdentity :
    let group := fiveCards.recordSelection .object (some 0) false
    let moved := applyZoneMove group [0] none (some .graveyard)
    moved.subsetWasPossible .object = true ∧
      moved.selectionEvidence.head?.bind (·.sourceGroup) = some 1 := by decide

private theorem possibleChoiceHistoryDoesNotExportLoopReferences :
    (choiceContext afterOptionalCreature).choiceWasPossible .object = false ∧
      (leaveRelative (DRS.ofList []) afterOptionalCreature).choiceWasPossible .object = false ∧
      Ability.checkText (DRS.ofList [])
        [.spell none optionalCreature,
         .spell none (.dealDamage (.lit 1)
           (.described .each (.and [.hasType .creature, .notChosen])) .noncombatOnly)] =
        [.choiceInScope .object] ∧
      let out := (stepInstruction 64 .postEvent .obj (DRS.ofList [])
        (.doForEach (.described .each .anyPlayer) optionalCreature)).1
      out.choiceWasPossible .object = true ∧ countParts .object out = 0 ∧
        countOnes .object out = 0 ∧ countManys .object out = 0 := by decide

private def loopTarget (predicate : Predicate) : Instruction :=
  .choose none (.described (.target (.range (some 1) (some 1))) predicate) .openly none

private def targetForEachPlayer : Instruction :=
  .doForEach (.described .each .anyPlayer)
    (loopTarget (.and [.inZone (.zone .battlefield .bare),
      .hasPossessor .controller (.pro (.word .player) .one)]))

private def afterTargetLoop : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList []) targetForEachPlayer).1

private theorem targetLoopPublishesAFreshPluralResult :
    Instruction.check (DRS.ofList []) targetForEachPlayer = [] ∧
      afterTargetLoop.referents.map (·.id) = [3, 0] ∧
      countOnes .object afterTargetLoop = 0 ∧
      countManys .object afterTargetLoop = 1 ∧
      countOnes .player afterTargetLoop = 0 ∧
      countManys .player afterTargetLoop = 1 ∧
      NounPhrase.check afterTargetLoop (.pro (.word .permanent) .many) = [] ∧
      NounPhrase.check afterTargetLoop (.pro (.word .permanent) .one) =
        [.anaphor (.word .permanent) .one 0] ∧
      NounPhrase.check afterTargetLoop (.pro .bare .one) = [.anaphor .bare .one 0] := by decide

private theorem playersCanSacrificeTheCollectedTargets :
    Instruction.check (DRS.ofList []) (.sequentially [targetForEachPlayer,
      .instruct (.pro (.word .player) .many)
        (.enact (.action "sacrifice")
          (.move (.pro (.word .permanent) .many)
            (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare))
            .departure none []) none none)]) = [] := by decide

private def afterPlayerLoop (body : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj (DRS.ofList [])
    (.doForEach (.described .each .anyPlayer) body)).1

private theorem ordinaryLocalsAndOptionalTargetsRemainClosed :
    countManys .object (afterPlayerLoop
      (.choose none (.described (.a .unmarked) (.hasType .creature)) .openly none)) = 0 ∧
      countManys .object (afterPlayerLoop
        (.expose .reveal (.cards (.described (.target (.range (some 1) (some 1)))
          (.hasType .creature))))) = 0 ∧
      countManys .object (afterPlayerLoop
        (.withContinuation .optional (loopTarget (.hasType .creature)) none none)) = 0 := by decide

private theorem separateTargetAnnouncementsStaySeparate :
    let out := afterPlayerLoop (.sequentially [loopTarget (.hasType .creature), loopTarget (.hasType .artifact)])
    countReach (.word (.type .creature)) .many out = 1 ∧
      countReach (.word (.type .artifact)) .many out = 1 ∧
      countManys .object out = 2 := by decide

private theorem targetGroupUsesTheMovedSuccessorWithoutLocalIdentity :
    let out := afterPlayerLoop (.sequentially [
      .choose none (.firstMention "loop.target"
        (.described (.target (.range (some 1) (some 1))) (.hasType .creature))) .openly none,
      .move (.laterMention "loop.target" .any) .anywhere
        (some (.zone .graveyard .bare)) .arrival none []])
    out.referents.map (·.id) = [4, 0] ∧
      (out.findById? 4).bind (·.zone) = some .graveyard ∧
      (out.findById? 4).map (·.idents) = some [] ∧
      (out.findById? 2) = none ∧ (out.findById? 3) = none ∧
      countOnes .object out = 0 := by decide

private theorem preexistingTargetsAreNotReaggregated :
    let outer : DRS := DRS.ofList [{
      id := 10, det := .target, plur := .one,
      payload := .object [.artifact] (some .battlefield) none none none none,
      announcedTargetResult := true }]
    let out := (stepInstruction 64 .postEvent .obj outer
      (.doForEach (.described .each .anyPlayer) (.draw (.lit 1)))).1
    (out.findById? 10).map (·.plur) = some .one ∧
      countManys .object out = 0 ∧ countOnes .object out = 1 := by decide

private def deniedTarget : StaticSpec :=
  .cant (.actor (.firstMention "outside.target"
    (.described (.target (.range (some 1) (some 1))) (.hasType .creature))))
    (.block .any) { rider := .noRider }

private theorem prohibitionKeepsTheAnnouncedTargetIdentity :
    StaticSpec.check (DRS.ofList []) deniedTarget = [] ∧
      let out := (stepStaticSpec 64 (DRS.ofList []) deniedTarget).1
      out.referents.map (·.id) = [0] ∧
        NounPhrase.check out (.laterMention "outside.target" .any) = [] ∧
        NounPhrase.check out (.pro (.word (.type .creature)) .one) = [] := by decide

private theorem hypotheticalProhibitionParticipantsStayClosed :
    let indefinite := (stepStaticSpec 64 (DRS.ofList [])
      (.cant (.actor (.described (.a .unmarked) (.hasType .creature)))
        (.block .any) { rider := .noRider })).1
    let universal := (stepStaticSpec 64 (DRS.ofList [])
      (.cant (.actor (.described .all (.hasType .creature)))
        (.block .any) { rider := .noRider })).1
    countOnes .object indefinite = 0 ∧ countManys .object universal = 0 := by decide

private theorem attachmentSourceRemainsAvailableAfterProhibition :
    let source := StaticSpec.cant (.actor (.attachHost .enchanted (.type .creature)))
      (.block .any) { rider := .noRider }
    let out := (stepStaticSpec 64 (DRS.ofList []) source).1
    out.referents.map (·.id) = [0] ∧
      NounPhrase.check out (.pro .bare .one) = [] := by decide

private theorem explicitByPhraseIsStillAnAgent :
    let patient := NounPhrase.described (.a .unmarked) (.hasType .creature)
    StaticSpec.check (DRS.ofList [])
      (.cant (.by .you) (.block patient) { rider := .noRider }) = [.deedFits] ∧
      StaticSpec.check (DRS.ofList [])
        (.cant (.by (.attachHost .enchanted (.type .creature)))
          (.block patient) { rider := .noRider }) = [] := by decide

private def tollCost : Cost := .scaled (.mana [.simple (.generic 1)]) (.arith .times (.lit 1) .groupSize)

private theorem tollReadsTheActorGroupAfterTheAction :
    StaticSpec.check (DRS.ofList [])
      (.gatedBy tollCost (.actor (.described .all (.hasType .creature)))
        (.attack (some (.or [.you, .described .all (.hasType .planeswalker)])))
        { rider := .noRider }) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy tollCost (.actor (.asType .creature .this none)) (.attack none)
          { rider := .noRider }) = [.groupSizeInScope 0] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy tollCost
          (.actor (.and [.described .all (.hasType .creature), .described .all (.hasType .artifact)]))
          (.attack none) { rider := .noRider }) = [.groupSizeInScope 2] := by decide

private theorem postposedTollSeesItsAttachmentPatient :
    StaticSpec.check (DRS.ofList [])
      (.gatedBy (.perform (.clearDamage (.pro .bare .one))) .any
        (.block (.attachHost .enchanted (.type .creature))) { rider := .noRider }) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.gatedBy .anyCost (.actor (.described .all (.hasType .creature)))
          (.attack none) { rider := .noRider }) = [.concreteCost] := by decide

private theorem linkedProhibitionStartsANewClauseAfterItsAction :
    Instruction.check (DRS.ofList [])
      (.doAndForbid
        (.enact (.action "destroy")
          (.move (.described .all (.hasType .creature))
            (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare))
            .departure none []) none none)
        (.action "regenerate") (.pro .bare .many)) = [] := by decide

private theorem addingAbilitiesRetainsTheChangedSubjectButIsolatesTheirBodies :
    let change := StaticSpec.characteristicChange (.asType .planeswalker .this none)
      [.addedAbilities [.keyword (.named "indestructible") [] []]]
    let out := (stepStaticSpec 64 (DRS.ofList []) change).1
    StaticSpec.check (DRS.ofList []) change = [] ∧
      out.referents.map (·.id) = [0] ∧
      NounPhrase.check out (.pro .bare .one) = [] ∧
      StaticSpec.check (DRS.ofList [])
        (.characteristicChange (.asType .planeswalker .this none)
          [.addedAbilities [.spell none (.clearDamage (.pro .bare .one))]]) =
        [.anaphor .bare .one 0] := by decide

private theorem targetAgentsKeepTheirActualSpellOrAbilityIdentity :
    let spellClause := StaticSpec.cant (.by (.asMarker .spell .this))
      (.target (.described (.a .unmarked) (.hasType .creature))) { rider := .noRider }
    let abilityClause := StaticSpec.cant (.by (.asMarker .ability .this))
      (.target (.described (.a .unmarked) (.hasType .creature))) { rider := .noRider }
    StaticSpec.check (DRS.ofList []) spellClause = [] ∧
      StaticSpec.check (DRS.ofList []) abilityClause = [] ∧
      (stepStaticSpec 64 (DRS.ofList []) spellClause).1.referents.map (·.id) = [1] ∧
      (stepStaticSpec 64 (DRS.ofList []) abilityClause).1.referents.map (·.id) = [1] ∧
      StaticSpec.check (DRS.ofList [])
        (.cant (.by .you) (.target (.described (.a .unmarked) (.hasType .creature)))
          { rider := .noRider }) = [.deedFits] ∧
      StaticSpec.check (DRS.ofList [])
        (.cant (.by (.asType .land .this none))
          (.target (.described (.a .unmarked) (.hasType .creature)))
          { rider := .noRider }) = [.deedFits] := by decide

private theorem ordinaryGroupSizeIsNotAnchoredToThePerformer :
    let group := DRS.ofList [{
      det := .all, plur := .many,
      payload := .object [.creature] (some .battlefield) none none none none }]
    Instruction.check group (.instruct .you (.draw .groupSize)) = [] ∧
      StaticSpec.check group
        (.gatedBy tollCost (.actor (.asType .creature .this none))
          (.attack none) { rider := .noRider }) = [.groupSizeInScope 0] ∧
      (stepStaticSpec 64 group
        (.gatedBy tollCost (.actor (.described .all (.hasType .creature)))
          (.attack none) { rider := .noRider })).1.groupSizeScope = none := by decide

end Semantics
