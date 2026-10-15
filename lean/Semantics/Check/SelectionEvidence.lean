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

private theorem closedBoxesDoNotExportChoiceEvidence :
    (choiceContext afterOptionalCreature).choiceWasPossible .object = false ∧
      (leaveRelative (DRS.ofList []) afterOptionalCreature).choiceWasPossible .object = false ∧
      Ability.checkText (DRS.ofList [])
        [.spell none optionalCreature,
         .spell none (.dealDamage (.lit 1)
           (.described .each (.and [.hasType .creature, .notChosen])) .noncombatOnly)] =
        [.choiceInScope .object] ∧
      (stepInstruction 64 .postEvent .obj (DRS.ofList [])
        (.doForEach (.described .each .anyPlayer) optionalCreature)).1.choiceWasPossible .object = false := by decide

end Semantics
