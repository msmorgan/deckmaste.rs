import Semantics.Check.NumberSlots
import Semantics.Macros
import Semantics.Check.Card
import Semantics.Cards.Keyword
import Semantics.Cards.Static
import Semantics.Cards.Deontic

open Semantics Semantics.Macros

namespace Semantics.Proofs.GetsBothDeltas

private def instructionResult (drs : DRS) (instruction : Instruction) : DRS :=
  (stepInstruction 64 .postEvent .obj drs instruction).1

theorem getsPtIsOneNode (subject : NounPhrase) (power toughness : Delta Amount) :
    getsPt subject power toughness = .ptModification subject power toughness := rfl

theorem okTargetGetsIntroducesOneTarget :
    (instructionResult (DRS.ofList []) (get (target creature) (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))).referents.countP
      (fun b => b.det == .target) = 1 := by
  decide

theorem subjectWrittenTwiceIntroducesTwoTargets :
    (instructionResult (DRS.ofList [])
      (.establish
        (.conjunction none
          [ .modification (target creature) .power (.up (.lit 2)),
            .modification (target creature) .toughness (.up (.lit 2)) ])
        (some untilEndOfTurn))).referents.countP (fun b => b.det == .target) = 2 := by
  decide

theorem ptModificationSlots :
    (StaticSpec.ptModification thisCreature (.up (.lit 2)) (.set (.lit (-1)))).numberSlots =
      [(.lit 2, .clamped), (.lit (-1), .signed)] := by
  rfl

theorem okDeltasReadTheirSubject :
    Instruction.check (DRS.ofList [])
      (get (target creature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
        (some untilEndOfTurn)) = [] := by
  decide

theorem okGetsInsideCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [getsPt (target creature) (.up (.lit 1)) (.up (.lit 1)), .abilityGrant it (keyword (.named "flying"))])
      = [] := by
  decide

private def selectedSubjectContext : DRS :=
  instructionResult (DRS.ofList []) (.sequentially
    [.setStatus .tapped (target artifact),
      get (.firstMention "pump.subject" (target creature))
        (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)])

theorem selectedPtSubjectHasStableIdentity :
    (identityBinding "pump.subject" selectedSubjectContext).map (·.id) = some 1 ∧
      (firstReach .bare .one selectedSubjectContext).map (·.id) = some 1 ∧
      (selectedSubjectContext.findById? 1).map (·.ty) = some [.creature] := by decide

theorem deltasReadSelectedSubjectAfterUnrelatedObject :
    Instruction.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (target artifact),
        get (.firstMention "pump.subject" (target creature))
          (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)]) = [] := by decide

theorem pluralPtSubjectIsIntroducedOnce :
    let after := instructionResult (DRS.ofList [])
      (get (counted (exactly 2) creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
    after.visibleReferents.length = 1 ∧
      (firstReach .bare .many after).map (·.id) = some 0 ∧
      countReach .bare .one after = 0 := by decide

theorem ambiguousSingularPtSubjectIsRefused :
    let before := (stepNounPhrase 64 (DRS.ofList []) .subj true
      (.and [target creature, target artifact])).1
    StaticSpec.check (nextClause before)
      (.ptModification it (.up (.lit 1)) (.up (.lit 1))) =
      [.anaphor .bare .one 2] := by decide

theorem nestedModernCoordinationIsRefused :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [.conjunction none
          [.ptModification thisCreature (.up (.lit 1)) (.up (.lit 1)),
            .abilityGrant thisCreature (keyword (.named "flying"))],
          .abilityGrant thisCreature (keyword (.named "trample"))]) = [.notCoord] := by decide

private def abilityTrace (context : DRS) (ability : Ability) : DRS × List Refusal :=
  match ability with
  | .static spec => stepStaticSpec 32 context spec
  | .spell none body => stepInstruction 32 .postEvent .obj context body
  | .triggered event alternatives while_ [] none none guard body =>
    let (header, headerErrors) := stepTriggerHeaderWith (stepOps 32) context
      event alternatives while_ none
    let (condition, conditionErrors) := match guard with
      | some condition => stepCondition 32 (nextClause header) condition
      | none => (header, [])
    let (out, bodyErrors) := stepInstruction 32 .postEvent .obj (nextClause condition) body
    (out, headerErrors ++ conditionErrors ++ bodyErrors)
  | _ => (context, [.recursionLimit])

private def singleFaceText (card : Card) : List Ability :=
  match card with
  | .singleFaced face => face.characteristics.text
  | _ => []

private def firstLineTrace (card : Spelled) : DRS × List Refusal :=
  match singleFaceText card.card with
  | first :: _ => abilityTrace (DRS.ofList []) first
  | [] => (DRS.ofList [], [.nonEmpty])

private def cardLineTrace (card : Spelled) (index : Nat) : DRS × List Refusal :=
  let text := singleFaceText card.card
  let context := (text.take index).foldl Ability.textContext (DRS.ofList [])
  match text[index]? with
  | some line => abilityTrace context line
  | none => (context, [.nonEmpty])

private def creatureIdentity (context : DRS) (id : ReferentId) : Bool :=
  (context.visibleReferents.filter (fun ref => ref.ty.contains .creature)).map (·.id) == [id]

theorem somberwaldAlphaPumpsItsBlockedCreature :
    let result := abilityTrace (DRS.ofList []) Cards.somberwaldAlpha
    result.2 = [] ∧ result.1.resolvedIds = [1] ∧ creatureIdentity result.1 1 = true := by decide

theorem deeprootWarriorPumpsItsOwnBlockedSource :
    let result := abilityTrace (DRS.ofList []) Cards.deeprootWarrior
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem jeskaiAscendancyUntapsItsPumpedGroup :
    let result := abilityTrace (DRS.ofList []) Cards.jeskaiAscendancyPump
    result.2 = [] ∧ creatureIdentity result.1 5 = true ∧
      (result.1.findById? 5).map (·.plur) = some .many ∧
      (identityBinding "untap.subject" result.1).map (·.id) = some 5 := by decide

theorem diminishKeepsOneTargetForBothSetDeltas :
    let result := firstLineTrace Cards.diminish
    singleFaceText Cards.diminish.card =
      [.spell none (get (target creature) (.set (.lit 1)) (.set (.lit 1))
        (some untilEndOfTurn))] ∧
      result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by
  constructor
  · rfl
  · decide

theorem cycleOfLifeKeepsItsCastCreatureTarget :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList []) Cards.cycleOfLife
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target ∧
      Cards.cycleOfLife = get (target (.and [creature, castBy .you]))
        (.set (.lit 0)) (.set (.lit 1))
        (some (untilEvent (.beginningOf .upkeep (some .you) (.next none)))) := by
  refine ⟨by decide, by decide, by decide, rfl⟩

theorem bondsOfFaithConditionReadsItsEnchantedSubject :
    let result := abilityTrace (DRS.ofList []) Cards.bondsOfFaithPump
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem builtToSmashConditionAndTrampleReadThePumpedTarget :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList []) Cards.builtToSmash
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.ty) = some [.creature, .artifact] := by decide

theorem earthenGooBothCounterDeltasReadItsOwnSubject :
    let result := cardLineTrace Cards.earthenGoo 2
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.pronounResolved) = some true := by decide

theorem nyxathidBothDeltasReadItsUniqueAnnouncedPlayer :
    let result := cardLineTrace Cards.nyxathid 1
    result.2 = [] ∧ countChoice .player result.1 = 1 ∧
      (result.1.findById? 0).map (·.payload) = some (.player true) ∧
      creatureIdentity result.1 1 = true ∧
      (singleFaceText Cards.nyxathid.card)[1]? = some (.static
        (getsPt thisCreature
          (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))])))
          (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))]))))) := by
  refine ⟨by decide, by decide, by decide, by decide, rfl⟩

theorem archpriestOfIonaGivesFlyingToItsPumpedTarget :
    let result := abilityTrace (DRS.ofList []) Cards.archpriestOfIonaFullParty
    result.2 = [] ∧ result.1.resolvedIds = [5] ∧
      (result.1.findById? 5).map (·.det) = some .target ∧
      (result.1.findById? 5).map (·.ty) = some [.creature] := by decide

theorem nightmarishEndKeepsItsTargetAndDefinesBothDeltaLetters :
    let result := firstLineTrace Cards.nightmarishEnd
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target ∧
      anyOpenLetter .x result.1 = false := by decide

theorem distortionStrikeProhibitionReadsItsPumpedTarget :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList []) Cards.distortionStrikeLine
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target ∧
      (match Cards.distortionStrikeLine with
      | .sequentially [pump, .establish (.cant .any (.block patient) _) _] =>
        let pumped := (stepInstruction 32 .postEvent .obj (DRS.ofList []) pump).1
        (stepNounPhrase 32 (nextClause pumped) .obj true patient).2.1 = [0]
      | _ => False) := by
  refine ⟨by decide, by decide, by decide, ?_⟩
  change (stepNounPhrase 32
    (nextClause ((stepInstruction 32 .postEvent .obj (DRS.ofList [])
      (get (target creature) (.up (.lit 1)) (.up (.lit 0))
        (some untilEndOfTurn))).1)) .obj true (that (.type .creature))).2.1 = [0]
  decide

theorem awakenTheBearGrantsTrampleToItsPumpedTarget :
    let result := firstLineTrace Cards.awakenTheBear
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by decide

theorem spidersilkArmorGrantsReachToItsPumpedGroup :
    let result := firstLineTrace Cards.spidersilkArmor
    result.2 = [] ∧ result.1.resolvedIds = [1] ∧ creatureIdentity result.1 1 = true ∧
      (result.1.findById? 1).map (·.plur) = some .many := by decide

theorem enchantedVanillaChangesOnlyItsAttachmentHost :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (Cards.enchantedBecomesVanilla [.blue] "Frog" 1 1)
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem equippedPumpWritesItsHostOnce :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 1)))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem enchantedPenaltyWritesItsHostOnce :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (getsPt (.attachHost .enchanted (.type .creature)) (.down (.lit 3)) (.down (.lit 0)))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem otherCreaturesAnthemKeepsSourceAndPumpedGroupDistinct :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (getsPt (allOf (otherCreatureYouControl thisCreature)) (.up (.lit 1)) (.up (.lit 1)))
    result.2 = [] ∧ (result.1.findById? 0).map (·.plur) = some .one ∧
      (result.1.findById? 2).map (·.plur) = some .many ∧
      (result.1.findById? 2).map (·.ty) = some [.creature] := by decide

theorem eachCreaturePumpKeepsItsDistributedSubject :
    let ability := whenever (.does (.actor .you) (.procedure .cast (a spell) .itsOwnCost none none none))
      (.sequentially [get (each creatureYouControl) (.up (.lit 1)) (.up (.lit 0))
        (some untilEndOfTurn), scry (.lit 1)])
    let result := abilityTrace (DRS.ofList []) ability
    result.2 = [] ∧ (result.1.visibleReferents.filter
      (fun ref => ref.ty.contains .creature && ref.det == .each)).length = 1 := by decide

theorem deltasReadTheirNewlyIntroducedTarget :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList [])
      (get (target creature) (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn))
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem deltaSubjectSurvivesAnEarlierArtifactDestruction :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList [])
      (.sequentially [destroy (target artifact),
        get (target creature) (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)])
    result.2 = [] ∧ result.1.resolvedIds = [2] ∧
      creatureIdentity result.1 2 = true ∧
      (result.1.findById? 1).map (·.zone) = some (some .graveyard) := by decide

theorem nextUpkeepPumpKeepsItsCreatureTarget :
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList [])
      (get (target creature) (.up (.lit 3)) (.up (.lit 3))
        (some (untilEvent (.beginningOf .upkeep (some .you) (.next none)))))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by decide

theorem modernFlatCoordinationKeepsOneTarget :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (target creature) (.up (.lit 1)) (.up (.lit 1)),
        .abilityGrant it (keyword (.named "flying")),
        .abilityGrant it (keyword (.named "trample"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem modernPluralCoordinationKeepsItsPumpedGroup :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)),
        .abilityGrant them (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [1] ∧ creatureIdentity result.1 1 = true ∧
      (result.1.findById? 1).map (·.plur) = some .many := by decide

theorem modernEnchantedSubjectCannotBeReadAsPlural :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .creature)) (.up (.lit 1)) (.up (.lit 1)),
        .abilityGrant them (keyword (.named "flying"))]) = [.anaphor .bare .many 0] := by decide

theorem modernStaticSelfRemainsReadableAsThatCreature :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.lit 1)) (.up (.lit 1)),
        .abilityGrant (that (.type .creature)) (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem modernLandHostProhibitionKeepsItsPumpedHost :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .land)) (.up (.lit 1)) (.up (.lit 1)),
        .cant (.actor it) (.block .any) { rider := .noRider }])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.land] ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide


theorem equippedLandPumpIsRefused :
    StaticSpec.check (DRS.ofList [])
      (getsPt (.attachHost .equipped (.type .land)) (.up (.lit 1)) (.up (.lit 1))) =
        [.contradictionFree] := by decide

theorem fortifiedCreaturePumpIsRefused :
    StaticSpec.check (DRS.ofList [])
      (getsPt (.attachHost .fortified (.type .creature)) (.up (.lit 1)) (.up (.lit 1))) =
        [.contradictionFree] := by decide


theorem modernStaticLetterDefinitionClosesItsPumpedSubjectLetter :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up (.lit 0)),
        .letterDefinition .x (countOf creatureYouControl)])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.creature] ∧
      anyOpenLetter .x result.1 = false := by decide

theorem modernStaticLetterCannotBeDefinedTwice :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up (.lit 0)),
        .letterDefinition .x (countOf creatureYouControl),
        .letterDefinition .x (countOf creature)]) = [.openLetter .x] := by decide

theorem modernSharedSubjectSurvivesAnEarlierExile :
    let body := Instruction.sequentially [exile (target artifact),
      establishFor (.firstMention "audit.shared" (target creature))
        [getsPt (.laterMention "audit.shared" it) (.up (.lit 1)) (.up (.lit 1)),
          .abilityGrant (.laterMention "audit.shared" it) (keyword (.named "flying"))]
        (some untilEndOfTurn)]
    let result := stepInstruction 32 .postEvent .obj (DRS.ofList []) body
    result.2 = [] ∧ (identityBinding "audit.shared" result.1).map (·.id) = some 2 ∧
      (result.1.findById? 2).map (·.ty) = some [.creature] ∧
      (result.1.findById? 1).map (·.zone) = some (some .exile) := by decide

end Semantics.Proofs.GetsBothDeltas
