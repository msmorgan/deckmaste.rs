import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Cost
import Semantics.Cards.Counters
import Semantics.Cards.Turn

/-!
# Semantics.Proofs.Keyword

Port of `idris/src/Experimental/Proofs/Keyword.idr`: the pins of the Keyword family, in
theorem form, each closed by `decide`. The names and the sentences are the Idris ones.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Keyword

private def nounResult (drs : DRS) (noun : NounPhrase) : DRS :=
  (stepNounPhrase 64 drs .other false noun).1

/-- An activated ability with no timing, limit, guard, or activator. -/
def activatedAbility (cost : Cost) (action : Action) : Ability :=
  .activated cost action none none none none

/-- "Sacrifice a creature: Draw a card." -/
theorem okSacrificeAsCost :
    Ability.check (DRS.ofList []) (activatedAbility (.perform (sacrifice (a isCreature))) (draw 1)) = [] := by
  decide

/-- "Target creature gains flying." -/
theorem okGainsKeyword :
    Action.check (DRS.ofList []) (gain (target isCreature) (keyword (.named "flying")) none) = [] := by decide

/-- "Creature spells you cast cost {1} less to cast." -/
theorem okCostSubjectOnStack :
    StaticSpec.check (DRS.ofList []) (.costShift (allOf (.and [isCreature, isSpell])) (.generic (.down 1) none)) = [] :=
        by
  decide

/-- "Creatures you control have flying." -/
theorem okBattlefieldFlying :
    StaticSpec.check (DRS.ofList []) (Templates.abilityGrant (allOf (and [isCreature, youControl])) (keyword (.named "flying"))) = [] := by
        decide

/-- "Target creature gets +1/+1, gains flying, and gains trample." -/
theorem okFlatCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ Templates.modification (target isCreature) .power (.up 1),
          Templates.modification it .toughness (.up 1),
          Templates.abilityGrant it (keyword (.named "flying")),
          Templates.abilityGrant it (keyword (.named "trample")) ]) = [] := by
  decide

/-- "Target creature gets +1/+1 and gains flying and gains trample." -/
theorem badNestedCoordination :
    StaticSpec.check (DRS.ofList [])
      (.conjunction none
        [ .conjunction none
            [ Templates.modification (target isCreature) .power (.up 1),
              Templates.modification it .toughness (.up 1),
              Templates.abilityGrant it (keyword (.named "flying")) ],
          Templates.abilityGrant it (keyword (.named "trample")) ]) = [.notCoord] := by
  decide

/-- "Whenever a creature enters, destroy that creature." -/
theorem okThatCreatureAfterAntecedent :
    Ability.check (DRS.ofList [])
      (whenever (does (a isCreature) enter) (destroy (that (.type .creature))))
      = [] := by
  decide

/-- "This creature gets +1/+1 and it has flying.": "it" reads the Source; "that creature" never
does, the Source being a self-referent (`Determiner.self`; owner ruling 2026-10-24). -/
theorem staticSelfRemainsReadableByIt :
    let spec := fun (third : NounPhrase) => StaticSpec.conjunction none
      [Templates.modification thisCreature .power (.up 1),
        Templates.modification it .toughness (.up 1),
        Templates.abilityGrant third (keyword (.named "flying"))]
    let after := (stepStaticSpec 64 (DRS.ofList []) (spec it)).1
    Ability.check (DRS.ofList []) (.static (spec it)) = [] ∧
      Ability.check (DRS.ofList []) (.static (spec (that (.type .creature)))) =
        [.anaphor (.word (.type .creature)) .one 0] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 0 ∧
      (after.findById? 0).map (·.ty) = some [.creature] := by decide

/-- "Creatures you control get +1/+1 and they have flying." -/
theorem okCoordinatedPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ Templates.modification (allOf (and [isCreature, youControl])) .power (.up 1),
            Templates.modification them .toughness (.up 1),
            Templates.abilityGrant them (keyword (.named "flying")) ])) = [] := by
  decide

/-- "Enchanted creature gets +1/+1 and they have flying." -/
theorem badCoordinatedHostPlural :
    Ability.check (DRS.ofList [])
      (.static
        (.conjunction none
          [ Templates.modification (.attachHost .enchanted (.type .creature)) .power (.up 1),
            Templates.modification it .toughness (.up 1),
            Templates.abilityGrant them (keyword (.named "flying")) ])) = [.anaphor .bare .many 0] := by
  decide

/-- "Create a 1/1 white Soldier creature token with flying." -/
theorem okTokenKeywordAbility :
    Action.check (DRS.ofList [])
      (create 1
        (.written { characteristics :=
          { colors := [.white], types := [.creature], subtypes := [soldier],
            power := some 1, toughness := some 1, text := [keyword (.named "flying")] } }))
      = [] := by
  decide

/-- "Creatures you control have '{T}: Draw a card.'" -/
theorem okQuotedGrantOnPermanent :
    StaticSpec.check (DRS.ofList []) (Templates.abilityGrant (allOf (and [isCreature, youControl])) (activatedAbility .tapSymbol (draw 1)))
      = [] := by
  decide

/-- "of the creature type of your choice" -/
theorem okYourChoiceCreatureType :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice (.subtype .creature) none) = [] := by decide

/-- "of the number of your choice" -/
theorem badYourChoiceNumber :
    Predicate.checkAt .object (DRS.ofList []) (.ofYourChoice .number none) = [.chosenQualityRead .number] := by
  decide

/-- An enchantment with the given text. -/
def enchantmentWith (text : List Ability) : Card :=
  .singleFaced { characteristics := { name := "", types := [.enchantment], text } }

/-- "As this enchantment enters, choose a creature type." -/
def choosesCreatureType : Ability :=
  .static (entersChoosing thisEnchantment (.subtype .creature))

/-- "Creatures of the chosen type get +1/+1." -/
def chosenTypeGets : Ability :=
  .static
    (getsPt (allOf (.and [isCreature, ofChosen (.subtype .creature)])) (.up 1) (.up 1))

/-- "As this enchantment enters, choose a creature type. Creatures of the chosen type get
+1/+1." -/
theorem okReaderAfterChooser :
    Card.check (enchantmentWith [choosesCreatureType, chosenTypeGets]) = [] := by decide

theorem badReaderBeforeChooser :
    Card.check (enchantmentWith [chosenTypeGets, choosesCreatureType])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badTwoChoosersOneSortRead :
    Card.check (enchantmentWith [choosesCreatureType, choosesCreatureType, chosenTypeGets])
      = [.choiceRef .theChoice (.quality (.subtype .creature)) 2] := by
  decide

theorem badChosenReadWrongSort :
    Card.check
      (enchantmentWith
        [ .static (entersChoosing thisEnchantment .color),
          chosenTypeGets ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

theorem badChosenProtectionBeforeChoice :
    Card.check
      (.singleFaced
        { characteristics :=
          { name := "", types := [.creature], subtypes := [angel],
            power := stat 2, toughness := stat 2,
            text :=
              [ .static
                  (Templates.abilityGrant thisCreature
                    (.keyword (.named "protection") [.quality (ofChosen .color)] [])),
                .static (entersChoosing thisCreature .color) ] } })
      = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

theorem badAscribedQualityBeforeChoice :
    Card.check
      (enchantmentWith
        [ .static
            (qualityChange (allOf (.and [isCreature, .hasPossessor .controller .you])) .adds
              (.quality (.chosen (ofChosen (.subtype .creature))))),
          choosesCreatureType ]) = [.choiceRef .theChoice (.quality (.subtype .creature)) 0] := by
  decide

/-- "{T}: Draw a card. Activate only once each turn." -/
theorem okActivatedTurnLimit :
    Ability.check (DRS.ofList []) (.activated .tapSymbol (draw 1) none (some oncePerTurn)
        none none)
      = [] := by
  decide

/-- "As long as a card exiled with this creature has <k>, this creature has <k>." -/
def exiledHas (k : KeywordTerm) : StaticSpec :=
  .conditional (keywordGrant thisCreature k (.and [.in_ (.exiledWith thisCreature), .hasKeyword k]))
    (exists_ (.and [.in_ (.exiledWith thisCreature), .hasKeyword k])) .asLongAs

/-- `exiledHas` as the ability a line prints. -/
def whileExiledHas (k : KeywordTerm) : Ability := .static (exiledHas k)

/-- "As long as a card exiled with this creature has flying, this creature has flying." -/
def flyingWhileExiledFlying : Ability := whileExiledHas (.the (.named "flying"))

/-- "As long as a card exiled with this creature has flying, this creature has flying. The same is
true for menace and trample.": one static ability whose parts are the three grants
[CR#113.2c], checked clean. -/
theorem okKeywordListOverGrant :
    alsoForKeywords whileExiledHas
        [.the (.named "flying"), .the (.named "menace"), .the (.named "trample")]
      = .static (.conjunction none [exiledHas (.the (.named "flying")),
          exiledHas (.the (.named "menace")), exiledHas (.the (.named "trample"))]) ∧
    Ability.check (DRS.ofList []) (alsoForKeywords whileExiledHas
        [.the (.named "flying"), .the (.named "menace"), .the (.named "trample")]) = [] :=
  ⟨rfl, by decide⟩

/-- An empty keyword list joins nothing: the expansion's empty `conjunction` is refused
`nonEmpty`. -/
theorem badEmptyKeywordList :
    Ability.check (DRS.ofList []) (alsoForKeywords whileExiledHas []) = [.nonEmpty] := by decide

/-- A body that is static for one keyword and of another kind for the next has no one ability to
join into: the expansion is an empty `conjunction`, refused `nonEmpty`. -/
theorem badMixedKeywordBodies :
    Ability.check (DRS.ofList [])
      (alsoForKeywords
        (fun k => if k == .the (.named "flying") then whileExiledHas k else .mayBeginOnBattlefield)
        [.the (.named "flying"), .the (.named "menace")]) = [.nonEmpty] := by
  decide

/-- "the number of basic land types among lands you control" -/
theorem okBasicLandTypeAxis :
    Amount.check (DRS.ofList [])
      (.distinctCount (.subtype .land .basicOnly)
        (allOf (.and [isLand, .hasPossessor .controller .you]))) = [] := by
  decide

/-- "the number of basic creature types among creatures you control" -/
theorem badBasicCreatureTypeAxis :
    Amount.check (DRS.ofList []) (.distinctCount (.subtype .creature .basicOnly) (allOf (and [isCreature, youControl])))
      = [.kindAxisSort] := by
  decide

/-- "Echo {2}{W}" -/
theorem okParameterisedEcho :
    Ability.check (DRS.ofList []) (keywordCosting (.named "echo") (.mana [generic 2, pip .white])) = [] := by decide

/-- "{T}: This deals X damage to any target, where X is 3." -/
theorem okActivatedOpensItsOwnLetter :
    Ability.check (DRS.ofList [])
      (activated .tapSymbol
        (.sequentially
          [(Action.instruct .this (.dealDamage (.letter .x) (target isAnyTarget))), define .x 3])) = [] := by
  decide

/-- "{T}: … , where X is 3" -/
theorem badActivatedClosesCardLetter :
    Ability.check (costLetters (some [.variable])) (activated .tapSymbol (define .x 3))
      = [.openLetter .x] := by
  decide

theorem activatedManaCostIntroducesItsOwnX :
    Ability.check (costLetters (some [.variable]))
      (activated (.mana [.variable]) (Templates.instruct .you (draw (.letter .x)))) = [] := by
  decide

theorem sharedSubjectSurvivesSecondSingular :
    Action.check (DRS.ofList [])
      (.sequentially
        [ exile (target isArtifact),
          establishFor (.firstMention "sharedSubject.subject" (target isCreature))
            [ Templates.modification (.laterMention "sharedSubject.subject" (target isCreature)) .power (.up 1),
              Templates.modification (.laterMention "sharedSubject.subject" (target isCreature)) .toughness (.up 1),
              Templates.abilityGrant (.laterMention "sharedSubject.subject" (target isCreature)) (keyword (.named "flying")) ]
            (some untilEndOfTurn) ]) = [] := by
  decide

/-- "target creature": the subject a shared-subject clause re-reads -/
theorem okSharedSubjectDelta :
    NounPhrase.checkAt (some .object) (nounResult (DRS.ofList []) (.firstMention "sharedSubject.subject" (target isCreature)))
      (.laterMention "sharedSubject.subject" (target isCreature)) = [] := by
  decide

theorem badSharedSubjectEmptyDelta :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.laterMention "sharedSubject.subject" it)
      = [.bindingIdentity "sharedSubject.subject"] := by
  decide

/-- "Pay 2 life: Draw a card." -/
theorem okOwnPayerCost : Ability.check (DRS.ofList []) (activatedAbility (payLife 2) (draw 1)) =
    [] := by
  decide

/-- "Counter target spell." -/
theorem okCounterSpell : Action.check (DRS.ofList []) (counter (target isSpell)) = [] := by decide

/-- "Counter target creature or player." -/
theorem badCounterJoinedPlayer :
    Action.check (DRS.ofList []) (counter (target isAnyTarget))
      = [.kindLte (.join .object .player) (.join .object .pile)] := by
  decide

/-- "Companion — Each permanent card in your starting deck has mana value 2 or less." -/
theorem okCompanionCharacteristicRead :
    Ability.check (DRS.ofList [])
      (companion
        (.everyCardIs isPermanentCard
          (.aCharacteristic (.compare [.stat .manaValue] .atMost 2)))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a card type." -/
theorem okCompanionSharedCardType :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .cardType .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Companion — Each nonland card in your starting deck shares a color.": color is a
characteristic [CR#109.3] and the deck-building condition is checked against the starting deck
[CR#702.139a]. -/
theorem okCompanionSharedColor :
    Ability.check (DRS.ofList []) (companion (.groupAgrees (.agree .color .share (allOf (.and [.not isLand, .isCard]))))) = [] := by
  decide

/-- "Each player scries 1.": one scry clause over a distributed player reference
[CR#701.22a,701.22c]. -/
theorem okEachPlayerScriesOne :
    Action.check (DRS.ofList []) (Templates.instruct (each .anyPlayer) (scry 1)) = [] := by decide

/-- "Fateseal 2.": the sorted library is an opponent's [CR#701.29a]. -/
theorem okFatesealAnOpponent :
    Action.check (DRS.ofList []) (fateseal 2) = [] := by decide

/-- "Choose flying or trample. This creature gains that ability until end of turn." -/
theorem okThatAbilityAfterChoice :
    Action.check (DRS.ofList [])
      (.sequentially
        [ choose (a (qualityFrom .ability (.abilitiesAmong [.the (.named "flying"), .the (.named "trample")]))),
          gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "This creature gains that ability until end of turn", with no ability chosen anywhere in
the text. -/
theorem badThatAbilityWithoutChoice :
    Action.check (DRS.ofList []) (gain thisCreature (.thatAbility .theChoice) (some untilEndOfTurn))
      = [.choiceRef .theChoice (.quality .ability) 0] := by
  decide

/-- "Hexproof from black": a hexproof ability with a quality [CR#702.11d]. -/
theorem okHexproofFrom : Ability.check (DRS.ofList []) (hexproofFrom (hasColor .black)) = [] := by
  decide

/-- A keyword ability reads as the keyword it is: "with flying" names flying's keyword, as the
written label would. -/
theorem hasKeywordOfAbility :
    hasKeyword flying = Predicate.hasKeyword (.the (.named "flying")) := by rfl

/-- So does each member of a keyword list: "flying, first strike, or trample". -/
theorem keywordListOfAbilities :
    (abilitiesAmong [flying, firstStrike, trample] : ChoiceDomain) =
      .abilitiesAmong [.the (.named "flying"), .the (.named "firstStrike"), .the (.named "trample")] := by
  rfl

/-- Madness {1}{R}: discarding this card exiles it instead, and its owner may cast it from exile
[CR#702.35a]. -/
theorem okMadness : Ability.check (DRS.ofList []) (madness (mana [generic 1, pip .red])) = [] := by
  decide

/-- Madness's replacement body, "that player discards it, but exiles it instead of putting it
into their graveyard" [CR#702.35a]: the discard frame nests the exile Definition, so the card is
both exiled, read by "the exiled card" and by the trigger's "when this card is exiled this way"
[CR#702.35a], and discarded, as effects referencing the discarded card find it [CR#702.35c]. -/
theorem madnessCardIsDiscardedAndExiled :
    let body : Action := .ownScope (some discard.deed) (exileFrom (.inCaller thisCard) hand)
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) body).1
    (participialCandidates (.deed exile.deed) (some .one) none false after).length = 1 ∧
      (participialCandidates (.deed discard.deed) (some .one) none false after).length = 1 ∧
      Action.check (DRS.ofList [])
        (sequentially [body, putInto (the (was (exile which))) graveyard]) = [] ∧
      (exileFrom thisCard hand).namedDeed = some exile.deed := by
  decide

/-- Mayhem {1}{R}: castable from your graveyard as long as you discarded this card this turn
[CR#702.187b]. -/
theorem okMayhem : Ability.check (DRS.ofList []) (mayhem (mana [generic 1, pip .red])) = [] := by
  decide

/-- Umbra armor: enchanted permanent's destruction becomes the removal of its damage and this
Aura's destruction [CR#702.89a]. -/
theorem okUmbraArmor : Ability.check (DRS.ofList []) umbraArmor = [] := by decide

/-- Split second: players can't cast other spells or activate abilities that aren't mana
abilities [CR#702.61a]. -/
theorem okSplitSecond : Ability.check (DRS.ofList []) splitSecond = [] := by decide

/-- The Ring tempts you, its emblem counting the times it has [CR#701.54a,701.54c]. -/
theorem okTheRingTemptsYou : Action.check (DRS.ofList []) theRingTemptsYou = [] := by decide

/-- An ability that is no keyword coerces to no keyword: "with Landfall — flying" is refused as
an unknown keyword term rather than read as a keyword. -/
theorem badNonKeywordAbilityAsKeyword :
    Predicate.check (DRS.ofList []) (hasKeyword (landfall flying)) = [.knownKeywordTerm] := by
  decide

/-- The same in a keyword list. -/
theorem badNonKeywordAbilityInKeywordList :
    Action.check (DRS.ofList [])
        (choose (a (qualityFrom .ability (abilitiesAmong [flying, landfall flying])))) =
      [.knownKeywordTerm] := by
  decide

/-! ### "The same is true for …" on the printed cards -/

/-- A card's line `i`. -/
private def lineOf (card : Spelled) (i : Nat) : Option Ability :=
  match card.card with
  | .singleFaced face => face.characteristics.text[i]?
  | _ => none

/-- The characteristic edits of each part a "the same is true for" line joins: each static
part's, or each `sequentially` step's. -/
private def partEdits : Ability → List (List CharacteristicEdit)
  | .static (.conjunction _ parts) => parts.map fun
    | .conditional (.characteristicChange _ edits) _ _ => edits
    | _ => []
  | .triggered _ _ _ _ _ _ _ (.sequentially steps) => steps.map fun
    | .doIf _ (.establish (.characteristicChange _ edits) _) _ _ => edits
    | _ => []
  | _ => []

/-- How a part grants its keyword: `some (some t)` for the family term `t`, through
`addedSelectionOf` over all of the objects a predicate describes; `some none` for one bare keyword,
through `addedAbilities`; `none` for anything else, a family term inside `addedAbilities`
among it. -/
private def grantForm : List CharacteristicEdit → Option (Option KeywordTerm)
  | [.addedSelectionOf (.described .all _) (.specified [.term (.anyIn family)])] =>
    some (some (.anyIn family))
  | [.addedAbilities [.keyword _ [] []]] => some none
  | _ => none

/-- The grant form of each part of a card's line `i`. -/
private def grantForms (card : Spelled) (i : Nat) : List (Option (Option KeywordTerm)) :=
  ((lineOf card i).map partEdits).getD [] |>.map grantForm

/-- The parts of a card's static line `i`. -/
private def staticParts (card : Spelled) (i : Nat) : List StaticSpec :=
  match lineOf card i with
  | some (.static (.conjunction _ parts)) => parts
  | _ => []

/-- Required pin: *Death-Mask Duplicant* and *Cairn Wanderer* gain "all landwalk and protection
abilities" of the cards their conditions find (their rulings of 2004-12-01 and 2007-10-01): each
family term is granted through `addedSelectionOf` over those objects with a `.term (anyIn …)`
selection, each other keyword is granted bare, and no `addedAbilities` holds a family term
[CR#702.1c]. Death-Mask Duplicant's landwalk part reads off exactly the objects its condition
finds. -/
theorem familyTermsGrantThroughAddedSelectionOf :
    grantForms Cards.deathMaskDuplicant 1 =
        [ some none, some none, some none, some none, some none,
          some (some Cards.landwalkAbilities), some (some Cards.protectionAbilities), some none ] ∧
      grantForms Cards.cairnWanderer 1 =
        [ some none, some none, some none, some none, some none, some none,
          some (some Cards.landwalkAbilities), some none, some (some Cards.protectionAbilities),
          some none, some none, some none, some none ] ∧
      (staticParts Cards.deathMaskDuplicant 1)[5]? =
        some (onlyWhile
          (abilityGrantSelected thisCreature
            (allOf (and [in_ (exiledWith thisCreature), hasKeyword Cards.landwalkAbilities]))
            (.specified [.term Cards.landwalkAbilities]))
          (exists_ (and [in_ (exiledWith thisCreature), hasKeyword Cards.landwalkAbilities]))) :=
  ⟨by decide, by decide, rfl⟩

/-- Required pin: hexproof is a family term on *Odric, Lunarch Marshal*, *Bleeding Effect* and
*Urborg Scavengers*: "hexproof from [quality]" is a hexproof ability [CR#702.11d], so each grants
the hexproof variants the objects its condition finds have [CR#702.1c] (Odric's ruling of
2020-01-24), and every other keyword bare. -/
theorem odricGrantsHexproofVariants :
    (grantForms Cards.odricLunarchMarshal 0).all Option.isSome ∧
      (grantForms Cards.odricLunarchMarshal 0).filterMap (·.bind id) = [Cards.hexproofAbilities] ∧
      (grantForms Cards.bleedingEffect 0).all Option.isSome ∧
      (grantForms Cards.bleedingEffect 0).filterMap (·.bind id) = [Cards.hexproofAbilities] ∧
      (grantForms Cards.urborgScavengers 1).all Option.isSome ∧
      (grantForms Cards.urborgScavengers 1).filterMap (·.bind id) = [Cards.hexproofAbilities] := by
  decide

/-- A keyword wants a modal spell when its own Definition writes a Mode Change: entwine,
escalate, spree and tiered [CR#702.42a,702.120a,702.172a,702.183a]. -/
theorem okEntwineEscalateSpreeTieredWantModes :
    [ entwine (.mana [generic 2]), escalate (.mana [generic 2]), spree, tiered ].all
      Ability.keywordWantsModes = true := by decide

/-- A keyword whose Definition writes no Mode Change wants none, a cost-bearing one included. -/
theorem kickerWantsNoModes : (kicker (.mana [generic 2])).keywordWantsModes = false := by decide

/-- Bloodthirst asks whether an opponent was dealt damage this turn [CR#702.54a]: the opponent is
the damage's recipient, and the agentless passive leaves the source that dealt it [CR#120.1]
unnamed. -/
theorem bloodthirstReadsTheOpponentAsTheDamagedPlayer :
    (match bloodthirst 1 with
      | .keyword _ _ [.static (.replace _ (.onlyIf c _) _ _ _)] => some c
      | _ => none) =
      some (Condition.was (dealDamage anyAmount anOpponent) soFarThisTurn) := rfl

/-- Reconfigure's second ability unattaches this permanent [CR#702.151a,701.3d]: the `unattach`
Template, stamped with no attach deed, since unattaching is not attaching. -/
theorem reconfigureUnattachesWithoutTheAttachDeed :
    (match reconfigure (.mana [generic 2]) with
      | .keyword _ _ [_, .activated _ effect _ _ _ _] => some effect
      | _ => none) = some (unattach thisPermanent) ∧
      (unattach thisPermanent).ownDeed? = none := ⟨rfl, rfl⟩

/-- Shadow's second half binds this creature as the blocker [CR#702.28b]: creatures without
shadow can't be blocked by it, not by every creature with shadow. -/
theorem shadowSecondHalfIsAboutThisCreature :
    (match shadow with
      | .keyword _ _ [_, .static (.cant subject action _)] => some (subject, action)
      | _ => none) =
      some (.by thisCreature,
        .block (allOf (and [isCreature, not (.hasKeyword (.the (.named "shadow")))]))) := rfl

/-- Melee counts opponents attacked with a creature this combat, not attack events [CR#702.121a]:
each axis gets one per such opponent. -/
theorem meleeCountsAttackedOpponents :
    let perOpponent : Amount := forEach 1 (and [isOpponent,
      did you (.attackWith (a isCreature) (some .which)) thisCombat])
    (match melee with
      | .keyword _ _ [.triggered _ _ _ _ _ _ _
          (.establish (.characteristicChange _ [.stat _ _ power, .stat _ _ toughness]) _)] =>
        some (power, toughness)
      | _ => none) = some (perOpponent, perOpponent) := rfl

/-- Time travel's exiled cards are suspended ones [CR#701.56a]: a suspended card has suspend as
well as a time counter [CR#702.62b]. -/
theorem timeTravelReadsSuspendedCards :
    (match timeTravel with
      | .ownScope _ (.sequentially (.choose _ (.described _ (.coord _ [_, exiled]) ) _ _ :: _)) =>
        some exiled
      | _ => none) =
      some (and [.isCard, .in_ exileZone, ownedBy .actor,
        .hasKeyword (.the (.named "suspend")), .hasCounters timeCounter]) := rfl

/-- The last step of a keyword's last ability's effect, when it is a sequence: where the copy
keywords offer new targets. -/
private def copyKeywordLastStep (keyword : Ability) : Option Action :=
  let last : Action → Option Action
    | .sequentially steps => steps.getLast?
    | _ => none
  match keyword with
  | .keyword _ _ abilities =>
    match abilities.getLast? with
    | some (.triggered _ _ _ _ _ _ _ effect) => last effect
    | some (.spell _ (.delay _ _ _ effect)) => last effect
    | _ => none
  | _ => none

/-- "If the spell has any targets, you may choose new targets for" the copy or copies
[CR#702.40a,702.56a,702.69a,702.153a,702.78a,702.50a]: each copy keyword guards its new-target
choice on the spell's targeting. -/
theorem copyKeywordsGuardNewTargetsOnTheSpellsTargets :
    let guarded (copies : NounPhrase) : Action :=
      doIf (is thisSpell (targeting .any .someTarget)) (may (.chooseNewTargets copies))
    copyKeywordLastStep storm = some (guarded (.pro (.word .copy) .many)) ∧
      copyKeywordLastStep (replicate (.mana [generic 1])) = some (guarded (those .copy)) ∧
      copyKeywordLastStep gravestorm = some (guarded (those .copy)) ∧
      copyKeywordLastStep (casualty 2) = some (guarded (that .copy)) ∧
      copyKeywordLastStep conspire = some (guarded (that .copy)) ∧
      copyKeywordLastStep epic = some (guarded (that .copy)) := ⟨rfl, rfl, rfl, rfl, rfl, rfl⟩

end Semantics.Proofs.Keyword
