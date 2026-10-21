import Semantics.Check.NumberSlots
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Keyword
import Semantics.Cards.Static
import Semantics.Cards.Deontic

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.GetsBothDeltas

private def actionResult (drs : DRS) (action : Action) : DRS :=
  (stepAction 64 .postEvent .obj drs action).1

/-- "<subject> gets +P/+T" is one characteristic change, the subject written once, with one
stat edit per delta: a change by an amount raises or lowers, a set sets [CR#613.4b,613.4c]. -/
theorem getsPtIsOneCharacteristicChange (subject : NounPhrase) (power toughness : Amount) :
    getsPt subject (.up power) (.set toughness) =
      .characteristicChange subject [.stat .adds .power power, .stat .sets .toughness toughness] ∧
    getsPt subject (.down power) (.up toughness) =
      .characteristicChange subject [.stat .loses .power power, .stat .adds .toughness toughness] :=
  ⟨rfl, rfl⟩

theorem okTargetGetsIntroducesOneTarget :
    (actionResult (DRS.ofList []) (get (target isCreature) (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))).referents.countP
      (fun b => b.det == .target) = 1 := by
  decide

theorem subjectWrittenTwiceIntroducesTwoTargets :
    (actionResult (DRS.ofList [])
      (.establish
        (.conjunction none
          [ Templates.modification (target isCreature) .power (.up (.lit 2)),
            Templates.modification (target isCreature) .toughness (.up (.lit 2)) ])
        (some untilEndOfTurn))).referents.countP (fun b => b.det == .target) = 2 := by
  decide

/-- "Gets +2 power and has base toughness −1": the raise is clamped, the set stands [CR#107.1b]. -/
theorem getsPtSlots :
    (getsPt thisCreature (.up (.lit 2)) (.set (.lit (-1)))).numberSlots =
      [(.lit 2, .clamped), (.lit (-1), .signed)] := by
  rfl

theorem okDeltasReadTheirSubject :
    Action.check (DRS.ofList [])
      (get (target isCreature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
        (some untilEndOfTurn)) = [] := by
  decide

theorem okGetsInsideCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [getsPt (target isCreature) (.up (.lit 1)) (.up (.lit 1)), Templates.abilityGrant it (keyword (.named "flying"))])
      = [] := by
  decide

private def selectedSubjectContext : DRS :=
  actionResult (DRS.ofList []) (.sequentially
    [.setStatus .tapped (target isArtifact),
      get (.firstMention "pump.subject" (target isCreature))
        (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)])

theorem selectedPtSubjectHasStableIdentity :
    (identityBinding "pump.subject" selectedSubjectContext).map (·.id) = some 1 ∧
      (firstReach .bare .one selectedSubjectContext).map (·.id) = some 1 ∧
      (selectedSubjectContext.findById? 1).map (·.ty) = some [.creature] := by decide

theorem deltasReadSelectedSubjectAfterUnrelatedObject :
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (target isArtifact),
        get (.firstMention "pump.subject" (target isCreature))
          (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)]) = [] := by decide

theorem pluralPtSubjectIsIntroducedOnce :
    let after := actionResult (DRS.ofList [])
      (get (counted (exactly 2) isCreature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
    after.visibleReferents.length = 1 ∧
      (firstReach .bare .many after).map (·.id) = some 0 ∧
      countReach .bare .one after = 0 := by decide

theorem ambiguousSingularPtSubjectIsRefused :
    let before := (stepNounPhrase 64 (DRS.ofList []) .subj true
      (.and [target isCreature, target isArtifact])).1
    StaticSpec.check (nextClause before)
      (getsPt it (.up (.lit 1)) (.up (.lit 1))) =
      [.anaphor .bare .one 2] := by decide

theorem nestedModernCoordinationIsRefused :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [.conjunction none
          [getsPt thisCreature (.up (.lit 1)) (.up (.lit 1)),
            Templates.abilityGrant thisCreature (keyword (.named "flying"))],
          Templates.abilityGrant thisCreature (keyword (.named "trample"))]) = [.notCoord] := by decide

private def abilityTrace (context : DRS) (ability : Ability) : DRS × List Refusal :=
  match ability with
  | .static spec => stepStaticSpec 32 context spec
  | .spell none body => stepAction 32 .postEvent .obj context body
  | .triggered event alternatives while_ [] none none guard body =>
    let (header, headerErrors) := stepTriggerHeaderWith (stepOps 32) context
      event alternatives while_ none
    let (condition, conditionErrors) := match guard with
      | some condition => stepCondition 32 (nextClause header) condition
      | none => (header, [])
    let (out, bodyErrors) := stepAction 32 .postEvent .obj (nextClause condition) body
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
      [.spell none (get (target isCreature) (.set (.lit 1)) (.set (.lit 1))
        (some untilEndOfTurn))] ∧
      result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by
  constructor
  · rfl
  · decide

theorem cycleOfLifeKeepsItsCastCreatureTarget :
    let result := stepAction 32 .postEvent .obj (DRS.ofList []) Cards.cycleOfLife
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target ∧
      Cards.cycleOfLife = get (target (.and [isCreature, castBy .you]))
        (.set (.lit 0)) (.set (.lit 1))
        (some (untilEvent (.beginningOf .upkeep (some .you) (.next none)))) := by
  refine ⟨by decide, by decide, by decide, rfl⟩

theorem bondsOfFaithConditionReadsItsEnchantedSubject :
    let result := abilityTrace (DRS.ofList []) Cards.bondsOfFaithPump
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem builtToSmashConditionAndTrampleReadThePumpedTarget :
    let result := stepAction 32 .postEvent .obj (DRS.ofList []) Cards.builtToSmash
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.ty) = some [.creature, .artifact] := by decide

/-- Both deltas read the one creature, its own subject, by one bare pronoun resolution. The
sacrifice cumulative upkeep may make is published to this line for a linked readback, and is no
pronoun's antecedent. -/
theorem earthenGooBothCounterDeltasReadItsOwnSubject :
    let result := cardLineTrace Cards.earthenGoo 2
    let subject := result.1.resolvedIds.headD 0
    result.2 = [] ∧ result.1.resolvedIds.length = 1 ∧ creatureIdentity result.1 subject = true ∧
      (result.1.findById? subject).map (·.pronounResolved) = some true := by decide

theorem nyxathidBothDeltasReadItsUniqueAnnouncedPlayer :
    let result := cardLineTrace Cards.nyxathid 1
    result.2 = [] ∧ countChoice .player result.1 = 1 ∧
      (result.1.findById? 0).map (·.payload) = some (.player true) ∧
      creatureIdentity result.1 1 = true ∧
      (singleFaceText Cards.nyxathid.card)[1]? = some (.static
        (getsPt thisCreature
          (.down (countOf (.and [.isCard, .in_ (handOf (the isChosenPlayer))])))
          (.down (countOf (.and [.isCard, .in_ (handOf (the isChosenPlayer))]))))) := by
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
    let result := stepAction 32 .postEvent .obj (DRS.ofList []) Cards.distortionStrikeLine
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target ∧
      (match Cards.distortionStrikeLine with
      | .sequentially [pump, .establish (.cant .any (.block patient) _) _] =>
        let pumped := (stepAction 32 .postEvent .obj (DRS.ofList []) pump).1
        (stepNounPhrase 32 (nextClause pumped) .obj true patient).2.1 = [0]
      | _ => False) := by
  refine ⟨by decide, by decide, by decide, ?_⟩
  change (stepNounPhrase 32
    (nextClause ((stepAction 32 .postEvent .obj (DRS.ofList [])
      (get (target isCreature) (.up (.lit 1)) (.up (.lit 0))
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
      (Cards.enchantedBecomesVanilla [.blue] Semantics.Definitions.frog 1 1)
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
      (getsPt (allOf ((and [isCreature, youControl, otherThan thisCreature]))) (.up (.lit 1)) (.up (.lit 1)))
    result.2 = [] ∧ (result.1.findById? 0).map (·.plur) = some .one ∧
      (result.1.findById? 2).map (·.plur) = some .many ∧
      (result.1.findById? 2).map (·.ty) = some [.creature] := by decide

theorem eachCreaturePumpKeepsItsDistributedSubject :
    let ability := whenever (.does (.actor .you) (.procedure .cast (a isSpell) .itsOwnCost none none none))
      (.sequentially [get (each (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 0))
        (some untilEndOfTurn), scry (.lit 1)])
    let result := abilityTrace (DRS.ofList []) ability
    result.2 = [] ∧ (result.1.visibleReferents.filter
      (fun ref => ref.ty.contains .creature && ref.det == .each)).length = 1 := by decide

theorem deltasReadTheirNewlyIntroducedTarget :
    let result := stepAction 32 .postEvent .obj (DRS.ofList [])
      (get (target isCreature) (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn))
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem deltaSubjectSurvivesAnEarlierArtifactDestruction :
    let result := stepAction 32 .postEvent .obj (DRS.ofList [])
      (.sequentially [destroy (target isArtifact),
        get (target isCreature) (.up (powerOf it)) (.up (powerOf it)) (some untilEndOfTurn)])
    result.2 = [] ∧ result.1.resolvedIds = [2] ∧
      creatureIdentity result.1 2 = true ∧
      (result.1.findById? 1).map (·.zone) = some (some .graveyard) := by decide

theorem nextUpkeepPumpKeepsItsCreatureTarget :
    let result := stepAction 32 .postEvent .obj (DRS.ofList [])
      (get (target isCreature) (.up (.lit 3)) (.up (.lit 3))
        (some (untilEvent (.beginningOf .upkeep (some .you) (.next none)))))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by decide

theorem modernFlatCoordinationKeepsOneTarget :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (target isCreature) (.up (.lit 1)) (.up (.lit 1)),
        Templates.abilityGrant it (keyword (.named "flying")),
        Templates.abilityGrant it (keyword (.named "trample"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem modernPluralCoordinationKeepsItsPumpedGroup :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (allOf (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1)),
        Templates.abilityGrant them (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [1] ∧ creatureIdentity result.1 1 = true ∧
      (result.1.findById? 1).map (·.plur) = some .many := by decide

theorem modernEnchantedSubjectCannotBeReadAsPlural :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .creature)) (.up (.lit 1)) (.up (.lit 1)),
        Templates.abilityGrant them (keyword (.named "flying"))]) = [.anaphor .bare .many 0] := by decide

theorem modernStaticSelfRemainsReadableAsThatCreature :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.lit 1)) (.up (.lit 1)),
        Templates.abilityGrant (that (.type .creature)) (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem modernLandHostProhibitionKeepsItsPumpedHost :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .land)) (.up (.lit 1)) (.up (.lit 1)),
        .cant (.actor it) (.block .any) {}])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.land] ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem modernStaticLetterDefinitionClosesItsPumpedSubjectLetter :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up (.lit 0)),
        .letterDefinition .x (countOf (and [isCreature, youControl]))])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.creature] ∧
      anyOpenLetter .x result.1 = false := by decide

theorem modernStaticLetterCannotBeDefinedTwice :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up (.lit 0)),
        .letterDefinition .x (countOf (and [isCreature, youControl])),
        .letterDefinition .x (countOf isCreature)]) = [.openLetter .x] := by decide

/-- "This creature has toxic X, where X is the number of creatures you control.": a granted
keyword whose number is a letter introduces that letter, so the text's definition closes it. -/
theorem grantedKeywordLetterIsIntroducedAndClosed :
    let grant :=
      Templates.abilityGrant thisCreature (.keyword (.named "toxic") [.number (.letter .x)] [])
    anyOpenLetter .x (stepStaticSpec 32 (DRS.ofList []) grant).1 = true ∧
      let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
        [grant, .letterDefinition .x (countOf (and [isCreature, youControl]))])
      result.2 = [] ∧ anyOpenLetter .x result.1 = false := by decide

theorem modernSharedSubjectSurvivesAnEarlierExile :
    let body := Action.sequentially [exile (target isArtifact),
      establishFor (.firstMention "audit.shared" (target isCreature))
        [getsPt (.laterMention "audit.shared" it) (.up (.lit 1)) (.up (.lit 1)),
          Templates.abilityGrant (.laterMention "audit.shared" it) (keyword (.named "flying"))]
        (some untilEndOfTurn)]
    let result := stepAction 32 .postEvent .obj (DRS.ofList []) body
    result.2 = [] ∧ (identityBinding "audit.shared" result.1).map (·.id) = some 2 ∧
      (result.1.findById? 2).map (·.ty) = some [.creature] ∧
      (result.1.findById? 1).map (·.zone) = some (some .exile) := by decide

theorem enchantedPlayerDoesNotPermitAnObjectTypeRefinement :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (.attachHost .enchanted .player) = [] ∧
      NounPhrase.checkAt (some .player) (DRS.ofList [])
        (.attachHost .enchanted (.ofType .player .creature)) = [.kindLte .player .object] := by
    decide

end Semantics.Proofs.GetsBothDeltas
