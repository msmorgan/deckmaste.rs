import Semantics.Macros.Declared

open Semantics Semantics.Macros

namespace Semantics.Proofs.DrtCards

private def feralContest : Instruction := .sequentially [
  .putCounters (.lit 1) (.printed p1p1Counter) (target creatureYouControl) .one true,
  .establish
    (.must (.actor (target (.and [creature, .other]))) (.block it) { rider := .noRider })
    (some thisTurn)]

private def monstrousStep : Instruction := .sequentially [
  get (target creature) (.up (.lit 7)) (.up (.lit 7)) (some untilEndOfTurn),
  .establish
    (.must (.actor (.described (.target (upTo 1)) (.and [creature, .other])))
      (.block it) { rider := .noRider }) (some thisTurn)]

private def cloudAttach : Ability :=
  when (.does .any (putOntoBattlefield thisCreature))
    (Primitives.Instruction.attachTo
      (.described (.target (upTo 1))
        (.and [.hasSubtype (artifactType "Equipment"), .hasPossessor .controller .you])) it)

theorem feralContestUsesOrdinaryPronoun :
    Instruction.check (DRS.ofList []) feralContest = [] := by decide

theorem monstrousStepUsesOrdinaryPronoun :
    Instruction.check (DRS.ofList []) monstrousStep = [] := by decide

theorem cloudUsesOrdinaryPronoun :
    Ability.check (DRS.ofList []) cloudAttach = [] := by decide


theorem feralContestBlocksFirstCreature :
    (stepInstruction 64 .postEvent .obj (DRS.ofList []) feralContest).1.resolvedIds = [1] := by
  decide

theorem monstrousStepBlocksFirstCreature :
    (stepInstruction 64 .postEvent .obj (DRS.ofList []) monstrousStep).1.resolvedIds = [0] := by
  decide

theorem airbendSingleTargetUsesPerCardLoop :
    Instruction.check (DRS.ofList []) (Declared.airbend (target creature)) = [] := by decide

theorem airbendMultipleTargetsUsePerCardLoop :
    Instruction.check (DRS.ofList [])
      (Declared.airbend (.described (.target (upTo 2)) creature)) = [] := by decide

theorem selfCounterProhibitionIsValidInstantText :
    cardTextOk [.instant] [.static (objectCant (.action "counter") thisSpell)] = true := by decide

theorem otherSpellCounterProhibitionIsNotInstantStaticText :
    cardTextOk [.instant] [.static (objectCant (.action "counter") (allOf spell))] = false := by decide

theorem battlefieldProhibitionIsNotInstantStaticText :
    cardTextOk [.instant] [.static (objectCant (.action "untap") thisCreature)] = false := by decide

theorem counterAlternativesUseTheirOwnHolderKinds :
    Predicate.checkAt (.join .object .player) (DRS.ofList [])
      (.or [.and [.inZone battlefield, .hasCounters none],
        .compare [.anyCounter .player] .atLeast (.lit 1)]) = [] := by decide

theorem mixedCounterAxesStillRejectOneHolder :
    axesAt .object [.anyCounter .object, .anyCounter .player] = false ∧
      axesAt .player [.anyCounter .object, .anyCounter .player] = false := by decide

theorem anyDieResultReadsOneRollBatch :
    Condition.check (DRS.ofList [outcomeB .diceRolled])
      (.anyResultIs .atLeast (.lit 10)) = [] := by decide

theorem anyDieResultRequiresARoll :
    Condition.check (DRS.ofList []) (.anyResultIs .atLeast (.lit 10)) =
      [.outcomeInScope .rollResult 0] := by decide

theorem anyDieResultRejectsAmbiguousBatches :
    Condition.check (DRS.ofList [outcomeB .diceRolled, outcomeB .diceRolled])
      (.anyResultIs .atLeast (.lit 10)) = [.outcomeInScope .rollResult 2] := by decide

theorem distributedLifeLossKeepsOneScalarLetter :
    let out := (stepInstruction 64 .postEvent .obj (DRS.ofList [])
      (instruct (each .opponent) (loseLife (.letter .x)))).1
    countLetter .x out = 1 ∧ anyOpenLetter .x out = true := by decide

theorem alternativeConditionsCannotBorrowOtherBranchParticipants :
    Condition.check (DRS.ofList [])
      (.or [.did ⟨.any, destroy (a creature), .duration soFarThisTurn none⟩,
        .matches it creature]) =
      [.anaphor .bare .one 0] := by decide

theorem powerDefinitionPublishesItsNumber :
    let out := (stepStaticSpec 64 (DRS.ofList [])
      (.ptDefinition thisCreature .powerAlone (.lit 2))).1
    countOutcomes .namedNumber out = 1 := by decide

theorem grantedKeywordPublishesItsScalarParameter :
    let out := (stepStaticSpec 64 (DRS.ofList [])
      (.abilityGrant thisCreature (keywordNumber (.named "bushido") (.letter .x)))).1
    anyOpenLetter .x out = true ∧ countLetter .x out = 1 := by decide

theorem typedPlayerViewOwnsAHandWithoutChangingTheJoinedReferent :
    let joinedRef : Referent := {
      id := 17, det := .target, plur := .one,
      payload := .join (.object [.planeswalker] (some .battlefield) none none none none)
        (.player false) }
    let initial := nextClause (DRS.ofList [joinedRef])
    let out := stepZoneScopeWith (stepOps 12) initial (.possessedBy they)
    out.2 = [] ∧ (out.1.findById? 17).map (·.payload) =
      (initial.findById? 17).map (·.payload) := by decide

theorem distributedDamagePublishesItsRecipientGroup :
    let out := (stepInstruction 12 .postEvent .obj (DRS.ofList [])
      (.dealDamage (.lit 1) (each .opponent) .noncombatOnly)).1
    (resolveCandidates (.word .player) .many (nextClause out)).map (·.id) = [0] := by decide

theorem conditionalModificationKeepsItsAnnouncedTarget :
    let out := stepInstruction 12 .postEvent .obj (DRS.ofList [])
      (.doOnlyIf (get (target creature) (.up (.lit 2)) (.up (.lit 2)) none)
        (.compareAmt (.statOf (.stat .power) it) .eq (.lit 2)) none)
    out.2 = [] ∧ (resolveCandidates .bare .one (nextClause out.1)).map (·.id) = [0] := by decide

theorem conditionalModificationDoesNotExportAnIndefiniteSubject :
    let out := stepInstruction 12 .postEvent .obj (DRS.ofList [])
      (.doOnlyIf (get (a creature) (.up (.lit 2)) (.up (.lit 2)) none)
        (.compareAmt (.statOf (.stat .power) it) .eq (.lit 2)) none)
    out.2 = [] ∧ (resolveCandidates .bare .one (nextClause out.1)).isEmpty = true := by decide

end Semantics.Proofs.DrtCards
