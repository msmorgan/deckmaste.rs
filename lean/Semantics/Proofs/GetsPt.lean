import Semantics.Check.NumberSlots
import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Keyword
import Semantics.Cards.Static
import Semantics.Cards.Deontic
import Semantics.Cards.Cost
import Semantics.Cards.Faces
import Semantics.Cards.Description
import Semantics.Cards.Turn
import Semantics.Proofs.Helpers

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.GetsPt

/-- The stat edits of the characteristic change battle cry's trigger establishes. -/
private def battleCryStatEdits : List CharacteristicEdit :=
  match battleCry with
  | .keyword _ _ [.triggered _ _ _ _ _ _ _ (.establish (.characteristicChange _ edits) _)] => edits
  | _ => []

/-- Whether an edit raises `stat` by the literal `n`. -/
private def raisesBy (stat : CharacteristicStat) (n : Int) : CharacteristicEdit → Bool
  | .stat .adds stat' (.lit m) => stat' == stat && m == n
  | _ => false

/-- "Each other attacking creature gets +1/+0" [CR#702.91a]: one change carrying both printed
deltas, its "+0" toughness edit included; these modify power and toughness [CR#613.4c]. -/
theorem battleCryGetsBothPrintedDeltas :
    battleCryStatEdits.any (raisesBy .power 1) = true ∧
      battleCryStatEdits.any (raisesBy .toughness 0) = true := by
  decide

theorem okTargetGetsIntroducesOneTarget :
    (actionResult (DRS.ofList []) (get (target isCreature) (.up 2) (.up 2) (some untilEndOfTurn))).referents.countP
      (fun b => b.det == .target) = 1 := by
  decide

theorem subjectWrittenTwiceIntroducesTwoTargets :
    (actionResult (DRS.ofList [])
      (.establish
        (.conjunction none
          [ Templates.modification (target isCreature) .power (.up 2),
            Templates.modification (target isCreature) .toughness (.up 2) ])
        (some untilEndOfTurn))).referents.countP (fun b => b.det == .target) = 2 := by
  decide

/-- "Gets +2 power and has base toughness −1": the raise is clamped, the set stands [CR#107.1b]. -/
theorem getsPtSlots :
    (getsPt thisCreature (.up 2) (.set (.lit (-1)))).numberSlots =
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
        [getsPt (target isCreature) (.up 1) (.up 1), Templates.abilityGrant it (keyword (.named "flying"))])
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
      (get (counted (exactly 2) isCreature) (.up 1) (.up 1) (some untilEndOfTurn))
    after.visibleReferents.length = 1 ∧
      (firstReach .bare .many after).map (·.id) = some 0 ∧
      countReach .bare .one after = 0 := by decide

theorem ambiguousSingularPtSubjectIsRefused :
    let before := (stepNounPhrase 64 (DRS.ofList []) .subj true
      (.and [target isCreature, target isArtifact])).1
    StaticSpec.check (nextClause before)
      (getsPt it (.up 1) (.up 1)) =
      [.anaphor .bare .one 2] := by decide

theorem nestedModernCoordinationIsRefused :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [.conjunction none
          [getsPt thisCreature (.up 1) (.up 1),
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

/-- The pumped group is 3, renumbered from 5 because the two "you"s are one referent
(`mentionYou`) and a handoff's performer takes no id (`DRS.performers`). -/
theorem jeskaiAscendancyUntapsItsPumpedGroup :
    let result := abilityTrace (DRS.ofList []) Cards.jeskaiAscendancyPump
    result.2 = [] ∧ creatureIdentity result.1 3 = true ∧
      (result.1.findById? 3).map (·.plur) = some .many ∧
      (identityBinding "untap.subject" result.1).map (·.id) = some 3 := by decide

/-- Diminish sets one target creature's base power and toughness to 1/1 [CR#613.4b]. -/
theorem diminishKeepsOneTargetForBothSetDeltas :
    let result := firstLineTrace Cards.diminish
    singleFaceText Cards.diminish.card =
      [.spell none (get (target isCreature) (.set 1) (.set 1)
        (some untilEndOfTurn))] ∧
      result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by
  constructor
  · rfl
  · decide

/-- The lookback "you cast this turn" steps its own referents, the caster "you" among them, before
the target it restricts, so the target is referent 2. -/
theorem cycleOfLifeKeepsItsCastCreatureTarget :
    let result := stepAction 32 .postEvent .obj (DRS.ofList []) Cards.cycleOfLife
    result.2 = [] ∧ creatureIdentity result.1 2 = true ∧
      (result.1.findById? 2).map (·.det) = some .target ∧
      Cards.cycleOfLife = get (target (.and [isCreature, did you (cast which) soFarThisTurn]))
        (.set 0) (.set 1)
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

/-- The pumped target is 4, renumbered from 5 because the two "you"s are one referent
(`mentionYou`). -/
theorem archpriestOfIonaGivesFlyingToItsPumpedTarget :
    let result := abilityTrace (DRS.ofList []) Cards.archpriestOfIonaFullParty
    result.2 = [] ∧ result.1.resolvedIds = [4] ∧
      (result.1.findById? 4).map (·.det) = some .target ∧
      (result.1.findById? 4).map (·.ty) = some [.creature] := by decide

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
      (get (target isCreature) (.up 1) (.up 0)
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
      (getsPt (.attachHost .equipped (.type .creature)) (.up 2) (.up 1))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem enchantedPenaltyWritesItsHostOnce :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (getsPt (.attachHost .enchanted (.type .creature)) (.down 3) (.down 0))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

/-- "Creatures you control other than this creature get +1/+1." -/
theorem otherCreaturesAnthemKeepsSourceAndPumpedGroupDistinct :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (getsPt (allOf ((and [isCreature, youControl, otherThan thisCreature]))) (.up 1) (.up 1))
    result.2 = [] ∧ (result.1.findById? 0).map (·.plur) = some .one ∧
      (result.1.findById? 2).map (·.plur) = some .many ∧
      (result.1.findById? 2).map (·.ty) = some [.creature] := by decide

theorem eachCreaturePumpKeepsItsDistributedSubject :
    let ability := whenever (.does (.actor .you) (.procedure .cast (a isSpell) .itsOwnCost none none none))
      (.sequentially [get (each (and [isCreature, youControl])) (.up 1) (.up 0)
        (some untilEndOfTurn), scry 1])
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
      (get (target isCreature) (.up 3) (.up 3)
        (some (untilEvent (.beginningOf .upkeep (some .you) (.next none)))))
    result.2 = [] ∧ creatureIdentity result.1 0 = true ∧
      (result.1.findById? 0).map (·.det) = some .target := by decide

theorem modernFlatCoordinationKeepsOneTarget :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (target isCreature) (.up 1) (.up 1),
        Templates.abilityGrant it (keyword (.named "flying")),
        Templates.abilityGrant it (keyword (.named "trample"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true := by decide

theorem modernPluralCoordinationKeepsItsPumpedGroup :
    let result := stepStaticSpec 32 (DRS.ofList [])
      (.conjunction none [getsPt (allOf (and [isCreature, youControl])) (.up 1) (.up 1),
        Templates.abilityGrant them (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [1] ∧ creatureIdentity result.1 1 = true ∧
      (result.1.findById? 1).map (·.plur) = some .many := by decide

theorem modernEnchantedSubjectCannotBeReadAsPlural :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .creature)) (.up 1) (.up 1),
        Templates.abilityGrant them (keyword (.named "flying"))]) = [.anaphor .bare .many 0] := by decide

/-- "This creature gets +1/+1 and has flying.": the shared subject is the Source, re-mentioned
by a second "this creature" (`mentionThis`) and never read by "that creature", a self-referent
(`Determiner.self`; owner ruling 2026-10-24). -/
theorem modernStaticSelfRemainsReadableAsThisCreature :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up 1) (.up 1),
        Templates.abilityGrant thisCreature (keyword (.named "flying"))])
    result.2 = [] ∧ result.1.resolvedIds = [0] ∧ creatureIdentity result.1 0 = true ∧
      (stepStaticSpec 32 (DRS.ofList []) (.conjunction none
        [getsPt thisCreature (.up 1) (.up 1),
          Templates.abilityGrant (that (.type .creature)) (keyword (.named "flying"))])).2 =
        [.anaphor (.word (.type .creature)) .one 0] := by decide

theorem modernLandHostProhibitionKeepsItsPumpedHost :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt (.attachHost .enchanted (.type .land)) (.up 1) (.up 1),
        .cant (.actor it) (.block .any) {}])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.land] ∧
      (result.1.findById? 0).map (·.attachmentHost) = some true := by decide

theorem modernStaticLetterDefinitionClosesItsPumpedSubjectLetter :
    let result := stepStaticSpec 32 (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up 0),
        .letterDefinition .x (countOf (and [isCreature, youControl]))])
    result.2 = [] ∧ (result.1.findById? 0).map (·.ty) = some [.creature] ∧
      anyOpenLetter .x result.1 = false := by decide

theorem modernStaticLetterCannotBeDefinedTwice :
    StaticSpec.check (DRS.ofList []) (.conjunction none
      [getsPt thisCreature (.up (.letter .x)) (.up 0),
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

/-- "This creature has reinforce X—{2}, where X is …": the letter is introduced whichever of the
keyword's parameters writes it, here the first of two [CR#702.77a], not only a sole one. -/
theorem grantedKeywordLetterIsIntroducedBesideACost :
    let grant := Templates.abilityGrant thisCreature (reinforce (.letter .x) [generic 2])
    anyOpenLetter .x (stepStaticSpec 32 (DRS.ofList []) grant).1 = true ∧
      (stepStaticSpec 32 (DRS.ofList []) (.conjunction none
        [grant, .letterDefinition .x (countOf (and [isCreature, youControl]))])).2 = [] := by
  decide

theorem modernSharedSubjectSurvivesAnEarlierExile :
    let body := Action.sequentially [exile (target isArtifact),
      establishFor (.firstMention "audit.shared" (target isCreature))
        [getsPt (.laterMention "audit.shared" it) (.up 1) (.up 1),
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

/-! ## Gets and gains

"<subject> gets +X/+Y and gains <ability> until end of turn" is one sentence with one written
subject and coordinated predicates: one `establish` of a `conjunction` that shares the subject,
its parts reading it back as "it" or "them". The power/toughness change [CR#613.4c] and the
ability grant [CR#613.1f], or a type change [CR#613.1d] ("gain all creature types", "lose all
creature types"), apply in their own layers from that one effect [CR#611.2a]. -/

private def faceText (face : CardFace) : List Ability := face.characteristics.text

private def frontText : Card → List Ability
  | .singleFaced face => faceText face
  | .transforming front _ => faceText front
  | _ => []

private def backText : Card → List Ability
  | .transforming _ back => faceText back
  | _ => []

private def abilityBody : Ability → Option Action
  | .spell _ body | .activated _ body _ _ _ _ | .triggered _ _ _ _ _ _ _ body
  | .italicHead _ (.activated _ body _ _ _ _) => some body
  | _ => none

/-- The body of each gets-and-gains ability: *Ghor-Clan Rampager*, *Archpriest of Iona*,
*Elspeth's Talent*, *Chittering Host*, *Awaken the Bear*, *Aim High*, *Volatile Claws* ("get
+2/+0 and gain all creature types"), both faces of *Arlinn Kord*, *Berserk* ("gains trample
and gets +X/+0"), *Garruk, the Veil-Cursed*'s −3 ("gain trample and get +X/+X"), and *Nameless
Inversion* and *Ego Erasure* ("get … and lose all creature types"). -/
private def getsAndGainsBodies : List (Option Action) :=
  [ abilityBody Cards.ghorClanRampager,
    abilityBody Cards.archpriestOfIonaFullParty,
    (frontText Cards.elspethsTalent.card)[2]? >>= abilityBody,
    (faceText Cards.chitteringHost)[2]? >>= abilityBody,
    (frontText Cards.awakenTheBear.card)[0]? >>= abilityBody,
    (frontText Cards.aimHigh.card)[0]? >>= abilityBody,
    (frontText Cards.volatileClaws.card)[0]? >>= abilityBody,
    (frontText Cards.arlinnKord.card)[0]? >>= abilityBody,
    (backText Cards.arlinnKord.card)[0]? >>= abilityBody,
    (frontText Cards.berserk.card)[0]? >>= abilityBody,
    (backText Cards.garrukRelentless.card)[2]? >>= abilityBody,
    some Cards.namelessInversionBody,
    some Cards.egoErasureBody ]

private def isEstablish : Action → Bool
  | .establish _ _ => true
  | _ => false

/-- The `establish` a body writes, alone or among a sequence, and the context it is stepped in:
after the members before it ("Untap target creature. It gets …"). -/
private def establishInContext : Action → Option (DRS × Action)
  | .sequentially members =>
    let before := members.takeWhile (!isEstablish ·)
    let context :=
      if before.isEmpty then DRS.ofList []
      else nextClause (stepAction 32 .postEvent .obj (DRS.ofList []) (.sequentially before)).1
    (members.find? isEstablish).map (context, ·)
  | action => some (DRS.ofList [], action)

private def isCharacteristicChange : StaticSpec → Bool
  | .characteristicChange _ _ => true
  | _ => false

private def readsSharedSubject : StaticSpec → Bool
  | .characteristicChange (.pro .bare _) _ => true
  | _ => false

/-- One `establish` for a stated duration of a shared-subject `conjunction` whose characteristic
changes (at least two) read the subject back rather than writing it again. -/
private def sharesOneSubject : Action → Bool
  | .establish (.conjunction (some _) parts) (some _) =>
    (parts.filter isCharacteristicChange).length ≥ 2 &&
      (parts.filter isCharacteristicChange).all readsSharedSubject
  | _ => false

/-- A targeted or universally quantified referent. -/
private def isSelected (ref : Referent) : Bool := ref.det == .target || ref.det == .all

/-- Whether stepping the `establish` leaves as many targeted or quantified referents as stepping
its shared subject alone: the parts add none, so the subject is stepped once. -/
private def stepsSubjectOnce : DRS × Action → Bool
  | (context, action@(.establish (.conjunction (some subject) _) _)) =>
    (stepAction 32 .postEvent .obj context action).1.referents.countP isSelected ==
      (stepNounPhrase 32 context .subj true subject).1.referents.countP isSelected
  | _ => false

/-- Required pin: every gets-and-gains card writes one `establish` of one shared-subject
`conjunction`. -/
theorem getsAndGainsShareOneShape :
    getsAndGainsBodies.all
      (fun body => (body >>= establishInContext).any (sharesOneSubject ·.2)) = true := by
  decide

/-- Required pin: each steps its subject once; its parts introduce no referent of their own. -/
theorem getsAndGainsStepOneSubject :
    getsAndGainsBodies.all (fun body => (body >>= establishInContext).any stepsSubjectOnce) =
      true := by
  decide

/-- The subject written again in each part is a new referent each time. -/
theorem subjectRepeatedInEachPartIsSteppedEachTime :
    let repeated := establishFor (target isCreature)
      [ getsPt (target isCreature) (.up 4) (.up 4),
        Templates.abilityGrant (target isCreature) (keyword (.named "trample")) ]
      (some untilEndOfTurn)
    sharesOneSubject repeated = false ∧ stepsSubjectOnce (DRS.ofList [], repeated) = false ∧
      (stepAction 32 .postEvent .obj (DRS.ofList []) repeated).1.referents.countP isSelected =
        3 := by
  decide

end Semantics.Proofs.GetsPt
