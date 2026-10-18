import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Faces

Port of `idris/src/Experimental/Proofs/Faces.idr`: the pins of the Faces family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.

Not ported: `badFaceAsFlipAlternative`, an Idris type error (a flip card's other half is a
characteristics set, not a face [CR#710.1]); here a flip card's other half is a `CardFace`
too, so the distinction is not a type's to make.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Faces

/-- An activated ability with no limit, guard, or activator. -/
def activatedAbility (cost : Cost) (instruction : Action) (timing : Option Timing := none) : Ability :=
  .activated cost instruction timing none none none

/-- A single-faced card. -/
def one (face : Characteristics) : Card := .singleFaced { characteristics := face }

/-- "Snow-Covered Forest — Basic Snow Land — Forest" -/
theorem okSingleSnow :
    Card.check
      (one
        { name := some "Snow-Covered Forest", supertypes := [.basic, .snow], types := [.land],
          subtypes := [landType "Forest"] }) = [] := by
  decide

/-- "Snow Snow Land — Forest" -/
theorem badDuplicateSnow :
    Card.check
      (one
        { name := some "", supertypes := [.snow, .snow], types := [.land],
          subtypes := [landType "Forest"] }) = [.distinct] := by
  decide

/-- "Draw a card." printed on an instant -/
theorem okSpellAbilityOnInstant :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.instant],
          text := [.spell none (draw (.lit 1))] }) = [] := by
  decide

/-- "Draw a card." -/
theorem badSpellAbilityOnPermanent :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.creature],
          text := [.spell none (draw (.lit 1))], power := stat 1, toughness := stat
              1 })
      = [.cardText] := by
  decide

/-- "Creatures you control get +1/+1." -/
theorem badStaticOnSorcery :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.sorcery],
          text := [.static (getsPt (allOf (and [isCreature, youControl])) (.up (.lit 1)) (.up (.lit 1)))] })
      = [.cardText] := by
  decide

/-- "Cast this spell only during the declare attackers step. Draw a card." -/
theorem okCastWindowOnSpell :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.instant],
          text := [.spell (some (duringPart .declareAttackers none)) (draw (.lit 1))] })
      = [] := by
  decide

/-- The same sentence spelled as a static permitting the cast. A cast window is stated by the
spell [CR#506.8], which is `okCastWindowOnSpell`'s window slot, not a permission a spell
card's static ability grants. -/
theorem badCastWindowAsDeonticStatic :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.instant],
          text :=
            [ .static
                (Primitives.StaticSpec.partScope .declareAttackers none
                  (.can .any (.procedure .cast .this .itsOwnCost none none none) {})),
              .spell none (draw (.lit 1)) ] }) = [.cardText] := by
  decide

/-- "Flying" -/
theorem badKeywordOnInstant :
    Card.check
      (one
        { name := some "", cost := some [pip .red], types := [.instant],
          text := [keyword (.named "flying")] }) = [.cardText] := by
  decide

/-- "{T}: Draw a card." -/
theorem badTapSorcery :
    Card.check
      (one
        { name := some "Impossible Tap Sorcery", cost := some [pip .blue], types := [.sorcery],
          text := [activatedAbility .tapSymbol (draw (.lit 1))] }) = [.cardText] := by
  decide

/-- a creature card printed with no power or toughness -/
theorem badCreatureCardNoPt :
    Card.check (one { name := some "", cost := some [pip .green], types := [.creature] })
      = [.cardBox] := by
  decide

/-- a land card printed with "{1}" -/
theorem badLandWithManaCost :
    Card.check (one { name := some "", cost := some [generic 1], types := [.land] })
      = [.cardCost] := by
  decide

/-- "Legendary Legendary Creature" -/
theorem badDuplicateSupertype :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], supertypes := [.legendary, .legendary],
          types := [.creature], power := stat 1, toughness := stat 1 }) = [.distinct] := by
  decide

/-- "Land Creature Instant" -/
theorem badMixedPermanentSpellLine :
    Card.check
      (one
        { name := some "", types := [.land, .creature, .instant], power := stat 1,
          toughness := stat 1 }) = [.cardLine] := by
  decide

/-- a card printed with an empty type line -/
theorem badCardNoTypes :
    Card.check (one { name := some "", cost := some [generic 1] }) = [.cardLine] := by decide

/-- "Creature Creature" -/
theorem badCardDuplicateType :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.creature, .creature],
          power := stat 2, toughness := stat 2 }) = [.cardLine] := by
  decide

theorem turnedFaceDownHeader :
    Ability.check (DRS.ofList [])
      (whenever (.does .any (.setStatus .faceDown (a isPermanent))) (draw (.lit 1))) = [] := by
  decide

/-- "Target creature doesn't untap during its controller's next untap step." -/
theorem okDoesntUntapNextOnBattlefield :
    Action.check (DRS.ofList []) (.skipUntap (target isCreature) (.lit 1)) = [] := by decide

/-- "Turn target creature face down." -/
theorem okTurnFaceDownOnBattlefield :
    Action.check (DRS.ofList []) (.setStatus .faceDown (target isCreature)) = [] := by decide

/-- a white creature face reading "Protection from red" -/
theorem okProtectionOnCreature :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.creature],
          text := [keywordQuality (.named "protection") (.colorIs .red)], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

theorem badStarlessDefinedPt :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.creature],
          text := [.static (.ptDefinition thisCreature .bothEach (countOf (and [isCreature, youControl])))],
          power := stat 2, toughness := stat 2 }) = [.cardBox] := by
  decide

/-- "each creature with flying" -/
theorem okKnownKeywordPredicate :
    Predicate.checkAt .object (DRS.ofList []) (.hasKeyword (.the (.named "flying"))) = [] := by decide

/-- "each creature with flyign" -/
theorem badUnknownKeywordPredicate :
    Predicate.checkAt .object (DRS.ofList []) (.hasKeyword (.the (.named "flyign"))) = [.knownKeywordTerm] := by decide

/-- "Protection from red" on a creature card -/
theorem okProtectionOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.creature],
          subtypes := [creatureType "Soldier"],
          text := [keywordQuality (.named "protection") (.colorIs .red)], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

/-- "Protection from red" -/
theorem badProtectionOnInstant :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.instant],
          text := [keywordQuality (.named "protection") (.colorIs .red)] }) = [.cardText] := by
  decide

/-- "Protection from red" -/
theorem okProtectionFromAColor :
    Ability.check (DRS.ofList []) (keywordQuality (.named "protection") (.colorIs .red)) = [] := by decide

/-- "Protection from player" -/
theorem badProtectionFromPlayerRestriction :
    Ability.check (DRS.ofList []) (.keyword (.named "protection") [.subject .anyPlayer] [])
      = [.keywordParamFits (.named "protection")] := by
  decide

/-- "Equip {2}" on an Equipment card -/
theorem okEquipOnEquipment :
    Card.check
      (one
        { name := some "", cost := some [generic 1], types := [.artifact],
          subtypes := [artifactType "Equipment"],
          text := [keywordCosting (.named "equip") (.mana [generic 2])] }) = [] := by
  decide

/-- "Equip {2}" -/
theorem badEquipOnSorcery :
    Card.check
      (one
        { name := some "", cost := some [generic 1], types := [.sorcery],
          text := [keywordCosting (.named "equip") (.mana [generic 2])] }) = [.cardText] := by
  decide

/-- "each of up to two target creatures" -/
theorem okEachOfATargetGroup :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (.described (.target (upTo 2)) isCreature))
      = [] := by
  decide

/-- "each of one or more creatures" -/
theorem badEachOfCountedGroup :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.eachOf (counted (atLeast 1) isCreature))
      = [.groupMention] := by
  decide

/-- "enchanted player" -/
theorem okEnchantedPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (.attachHost .enchanted .player) = [] := by decide

/-- "equipped player" -/
theorem badEquippedPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (.attachHost .equipped .player) =
      [.kindLte .player .object] := by
  decide

/-- "Target creature gains indestructible." -/
theorem okGainsIndestructible :
    StaticSpec.check (DRS.ofList []) (.abilityGrant (target isCreature) (keyword (.named "indestructible"))) = [] := by
        decide

/-- a "Kindred Enchantment — Merfolk" card -/
theorem okKindredWithAnotherType :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred, .enchantment],
          subtypes := [creatureType "Merfolk"], text := [keyword (.named "flying")] }) = [] := by
  decide

/-- a "Kindred Enchantment — Siege" card -/
theorem badSiegeWithoutBattle :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred, .enchantment],
          subtypes := [.of .battle "Siege"], text := [keyword (.named "flying")] }) = [.cardLine] := by
  decide

/-- a "Kindred — Merfolk" card naming no other card type -/
theorem badKindredAlone :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred],
          subtypes := [creatureType "Merfolk"], text := [keyword (.named "flying")] }) = [.cardLine] := by
  decide

/-- "Enchanted creature can't attack." -/
theorem okEnchantedCreatureCantAttack :
    Ability.check (DRS.ofList [])
      (.static
        (.cant (.actor (.attachHost .enchanted (.type .creature))) (.attack none) {}))
      = [] := by
  decide

/-- "Enchanted planeswalker can't attack.": a planeswalker can become a creature while
retaining its planeswalker type [CR#205.1b]. This fixture keeps the restriction available
when attackers are chosen [CR#508.1a]. -/
theorem okPlaneswalkerAttacks :
    Ability.check (DRS.ofList [])
      (.static
        (.cant (.actor (.attachHost .enchanted (.type .planeswalker))) (.attack none) {})) = [] := by
  decide

/-- "you pay {2}" -/
theorem okPayMana : Action.check (DRS.ofList []) (.pay (.mana [generic 2]) .once .paid none none) = [] := by
    decide

/-- A legendary Jace planeswalker face with no text or loyalty yet. -/
def jace : Characteristics :=
  { name := some "", cost := some [pip .blue], supertypes := [.legendary], types := [.planeswalker],
    subtypes := [.of .planeswalker "Jace"] }

/-- "[+1]: Draw a card." on a planeswalker card -/
theorem okLoyaltyOnPlaneswalker :
    Card.check
      (one
        { jace with text := [activatedAbility (.loyaltySymbol (.up 1)) (draw (.lit 1))],
                    loyalty := stat 3 }) = [] := by
  decide

/-- "[+1]: Draw a card." -/
theorem badLoyaltySorcery :
    Card.check
      (one
        { name := some "Impossible Loyalty Sorcery", cost := some [pip .blue], types := [.sorcery],
          text := [activatedAbility (.loyaltySymbol (.up 1)) (draw (.lit 1))] }) = [.cardText] :=
              by
  decide

/-- "a token that's a copy of target creature, except it's an artifact" -/
theorem okCopyTypeException :
    Action.check (DRS.ofList [])
      (Action.create (.lit 1) (.copyOf (target isCreature) [Primitives.CopyExcept.types [.artifact] []]) []) = []
          := by
  decide

theorem badEmptyCopyTypeException :
    Action.check (DRS.ofList []) (Action.create (.lit 1) (.copyOf (target isCreature) [Primitives.CopyExcept.types [] []]) [])
      = [.lineNonEmpty] := by
  decide

/-- "Copy target instant or sorcery spell." -/
theorem okCopyStackSpell :
    Action.check (DRS.ofList [])
      (.copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 1) []) = []
          := by
  decide

/-- "Create a token that's a copy of target creature. Untap that token." -/
theorem okSetStatusOnBattlefield :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Action.create (.lit 1) (.copyOf (target isCreature) []) [],
          .setStatus .untapped (that .token) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. Untap that token." The Idris pin refutes the
anaphor. -/
theorem badStackCopyAsToken :
    Action.check (DRS.ofList [])
      (.sequentially
        [ .copy .fromStack (target (.and [isInstantOrSorcery, isSpell])) (.lit 1) [],
          .setStatus .untapped (that .token) ])
      = [.anaphor (.word .token) .one 0] := by
  decide

/-- "Create a token that's a copy of target creature. Untap that copy." -/
theorem badTokenCopyAsCopyMention :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Action.create (.lit 1) (.copyOf (target isCreature) []) [],
          .setStatus .untapped (that .copy) ])
      = [.anaphor (.word .copy) .one 0] := by
  decide

/-- "Saga — I, II, III — Draw a card." -/
theorem okChapterOnSaga :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.enchantment],
          subtypes := [enchantmentType "Saga"],
          text :=
            [ when (.chapterMark [1]) (draw (.lit 1)),
              when (.chapterMark [2]) (draw (.lit 1)),
              when (.chapterMark [3]) (draw (.lit 1)) ] }) = [] := by
  decide

theorem badChapterOnNonSaga :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.enchantment],
          text := [when (.chapterMark [1]) (draw (.lit 1))] })
      = [.chapterFrame] := by
  decide

/-- "You may sacrifice a Mountain rather than pay this spell's mana cost." -/
theorem okAltCostSacrifice :
    StaticSpec.check (DRS.ofList [])
      (.altCost .this
        (some (.perform (sacrifice (a (.and [isLand, .hasSubtype (landType "Mountain")]))))))
      = [] := by
  decide

/-- "Escalate {2}. Choose one or both —" -/
theorem okEscalateWithModes :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text :=
            [ keywordCosting (.named "escalate") (.mana [generic 2]),
              .spell none
                (chooseModes (.range (some 1) (some 2))
                  [ get (target isCreature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
                    get (target isCreature) (.down (.lit 1)) (.down (.lit 1))
                      (some untilEndOfTurn) ]) ] }) = [] := by
  decide

/-- "Escalate {2}" -/
theorem badEscalateWithoutModes :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text := [keywordCosting (.named "escalate") (.mana [generic 2])] }) = [.modalFrame] := by
  decide

/-- "Entwine {2}" -/
theorem badEntwineWithoutModes :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.sorcery],
          text := [keywordCosting (.named "entwine") (.mana [generic 2])] }) = [.modalFrame] := by
  decide

/-- "You may choose the same mode more than once", with no modes to change: a Mode Change
needs a modal ability on the same text. -/
theorem badModeChoiceWithoutModes :
    Card.check
      (one
        { name := some "", cost := some [pip .red], types := [.sorcery],
          text := [.static (.modeChoice .repeatable)] }) = [.modalFrame] := by
  decide

/-- "Whenever you draw a card, choose one that hasn't been chosen this turn — Draw a card; or
destroy target artifact.": a Mode Change over a triggered ability's modes [CR#700.2b]. -/
theorem okModeChoiceOnTriggeredModes :
    Card.check
      (one
        { name := some "", cost := some [generic 3], types := [.artifact],
          text :=
            [ .triggered (.does (.actor .you) (.draw .anyAmount)) [] none [] none none none
                (chooseModes (exactly 1) [draw (.lit 1), destroy (target isArtifact)]),
              .static (.modeChoice (.notChosenBefore soFarThisTurn)) ] }) = [] := by
  decide

theorem jointCrossAbilityChoice :
    Card.check
      (.singleFaced
        { characteristics :=
            { name := some "Joint choice witness", types := [.creature],
              subtypes := [creatureType "Shapeshifter"],
              text :=
                [ .static (.abilityGrant thisCreature (keywordQuality (.named "protection") (ofChosen
                    .color))),
                  .static (entersChoosing thisCreature .color) ],
              power := stat 1, toughness := stat 1 },
          choices := [.color] }) = [] := by
  decide

/-- "Cumulative upkeep {2}" on an enchantment card -/
theorem okCumulativeUpkeepOnPermanent :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.enchantment],
          text := [keywordCosting (.named "cumulativeUpkeep") (.mana [generic 2])] }) = [] := by
  decide

/-- "Cumulative upkeep {2}" -/
theorem badCumulativeUpkeepOnSpell :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.sorcery],
          text := [keywordCosting (.named "cumulativeUpkeep") (.mana [generic 2])] }) = [.cardText] := by
  decide

/-- "Renown 1 (When this creature deals combat damage to a player, …)" -/
theorem okRenownWithRenownExpansion :
    Ability.check (DRS.ofList []) (.keyword (.named "renown") [.number (.lit 1)] [renownExpansion 1])
      = [] := by
  decide

/-- "Flying (When this creature deals combat damage to a player, …)": flying's definition is a
static ability [CR#702.9a,509.1b], so a triggered body is the wrong category. Re-spelled from
`badBodyOnBodilessKeyword`, which asserted the same refusal when no keyword took a static body. -/
theorem badTriggeredBodyOnStaticKeyword :
    Ability.check (DRS.ofList []) (.keyword (.named "flying") [] [renownExpansion 1])
      = [.keywordBodyFits (.named "flying")] := by
  decide

/-- "Flying (This creature can't be blocked except by creatures with flying and/or reach.)" -/
theorem okFlyingWithFlyingDefinition : Ability.check (DRS.ofList []) flying = [] := by decide

/-- "Renown 1 (This creature can't be blocked except by creatures with flying and/or reach.)":
renown's definition is a triggered ability [CR#702.112a]. -/
theorem badStaticBodyOnTriggeredKeyword :
    Ability.check (DRS.ofList []) (.keyword (.named "renown") [.number (.lit 1)] [flyingExpansion])
      = [.keywordBodyFits (.named "renown")] := by
  decide

/-- "Bushido 2 (Whenever this creature blocks or becomes blocked, it gets +2/+2 until end of
turn.)" -/
theorem okBushidoWithBushidoDefinition : Ability.check (DRS.ofList []) (bushido 2) = [] := by decide

/-- "Cycling {2} ({2}, Discard this card: Draw a card.)" -/
theorem okCyclingWithCyclingDefinition :
    Ability.check (DRS.ofList []) (cycling (.mana [generic 2])) = [] := by decide

/-- "Flying ({2}, Discard this card: Draw a card.)": an activated body on a keyword whose
definition is static [CR#702.9a,509.1b]. -/
theorem badActivatedBodyOnStaticKeyword :
    Ability.check (DRS.ofList []) (.keyword (.named "flying") [] [cyclingExpansion (.mana [generic 2])])
      = [.keywordBodyFits (.named "flying")] := by
  decide

/-- "Bushido 2 ({2}, Discard this card: Draw a card.)": an activated body on a keyword whose
definition is triggered [CR#702.45a]. -/
theorem badActivatedBodyOnTriggeredKeyword :
    Ability.check (DRS.ofList []) (.keyword (.named "bushido") [.number (.lit 2)]
        [cyclingExpansion (.mana [generic 2])])
      = [.keywordBodyFits (.named "bushido")] := by
  decide

/-- "Modular 1 (This permanent enters with a +1/+1 counter on it. When it dies, you may put a
+1/+1 counter on target artifact creature for each +1/+1 counter on this permanent.)": a
definition of two abilities in two categories [CR#702.43a]. -/
theorem okModularWithModularDefinition : Ability.check (DRS.ofList []) (modular 1) = [] := by decide

/-- "Modular 1 (… {2}, Discard this card: Draw a card.)": one element of the definition list is
of a category modular does not declare [CR#702.43a]. -/
theorem badUndeclaredCategoryInDefinitionList :
    Ability.check (DRS.ofList []) (.keyword (.named "modular") [.number (.lit 1)]
        (modularExpansion 1 ++ [cyclingExpansion (.mana [generic 2])]))
      = [.keywordBodyFits (.named "modular")] := by
  decide

/-- "Renown 1 (When you cast this spell, copy it …)" -/
theorem badRenownWithStormExpansion :
    Ability.check (DRS.ofList []) (.keyword (.named "renown") [.number (.lit 1)] [stormExpansion])
      = [.keywordBodyFits (.named "renown")] := by
  decide

/-- "your commander" -/
theorem okPossessedCommander :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.designated (.named "commander") .you) = [] := by decide

/-- "your monarch" -/
theorem badPossessedMonarch :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.designated (.named "monarch") .you)
      = [.designationScope (.named "monarch")] := by
  decide

/-- "Unearth {B}" -/
theorem okCostedUnearth :
    Ability.check (DRS.ofList []) (keywordCosting (.named "unearth") (.mana [pip .black])) = [] := by decide

/-- "Unearth" -/
theorem badBareUnearth :
    Ability.check (DRS.ofList []) (.keyword (.named "unearth") [] []) = [.keywordParamFits (.named "unearth")] := by decide

/-- "Retrace {1}" -/
theorem badCostedRetrace :
    Ability.check (DRS.ofList []) (keywordCosting (.named "retrace") (.mana [generic 1]))
      = [.keywordParamFits (.named "retrace")] := by
  decide

/-- "Unearth {B}" on a creature card -/
theorem okUnearthOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.creature],
          subtypes := [creatureType "Zombie"],
          text := [keywordCosting (.named "unearth") (.mana [pip .black])], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

/-- "Unearth {B}" -/
theorem badUnearthOnSpellCard :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text := [keywordCosting (.named "unearth") (.mana [pip .black])] }) = [.cardText] := by
  decide

/-- "Flashback {2}{U}" -/
theorem badFlashbackOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [generic 2, pip .blue], types := [.artifact],
          text := [keywordCosting (.named "flashback") (.mana [generic 2, pip .blue])] })
      = [.cardText] := by
  decide

/-- "for each attacking creature you control. You gain that much life." -/
theorem okThatMuchAfterQuantity :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Templates.instruct (target (.isOpponentOf .you)) (loseLife
            (forEach 1 (.and [isAttacking, isCreature, .hasPossessor .controller .you]))),
          gainLife .thatMuch ]) = [] := by
  decide

/-- "Flip a coin. Draw that many cards." -/
theorem badThatMuchAfterFlip :
    Action.check (DRS.ofList []) (.sequentially [flipCoins 1, draw .thatMuch])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Flip a coin. If you win the flip, draw a card." -/
theorem okFlipArmAfterFlip :
    Action.check (DRS.ofList [])
      (.sequentially [flipCoins 1, doIf (.flipCalled .you .wins) (draw (.lit 1))])
      = [] := by
  decide

/-- "If you win the flip, draw a card." -/
theorem badFlipArmWithoutFlip :
    Action.check (DRS.ofList []) (doIf (.flipCalled .you .wins) (draw (.lit 1)))
      = [.coinFlipInScope] := by
  decide

/-- "Roll a d6." -/
theorem okSixSidedDie : Action.check (DRS.ofList []) (.rollDice (.lit 1) (.sides 6) .one .anyResult) = [] :=
    by decide

/-- "Roll a d0." -/
theorem badNoughtSidedDie :
    Action.check (DRS.ofList []) (.rollDice (.lit 1) (.sides 0) .one .anyResult) = [.nonZeroQ] := by decide

/-- a planeswalker card printing its starting loyalty -/
theorem okPlaneswalkerLoyaltyBox : Card.check (one { jace with loyalty := stat 3 }) = [] := by
  decide

/-- a planeswalker card printed with no starting loyalty -/
theorem badPlaneswalkerNoLoyalty : Card.check (one jace) = [.cardBox] := by decide

/-- a planeswalker card printing "3/3" where its loyalty number goes -/
theorem badPlaneswalkerPtBox :
    Card.check (one { jace with power := stat 3, toughness := stat 3 }) = [.cardBox] := by decide

/-- a battle card printed with no defense -/
theorem badBattleNoDefense :
    Card.check
      (one
        { name := some "", cost := some [generic 2, pip .white], types := [.battle],
          subtypes := [.of .battle "Siege"] }) = [.cardBox] := by
  decide

/-- A blue 1/1 creature front face. -/
def blueCreature : CardFace :=
  { characteristics :=
    { name := some "", cost := some [pip .blue], types := [.creature], power := stat 1,
      toughness := stat 1 } }

/-- an adventurer card whose inset frame is a named Adventure sorcery -/
theorem okNamedAdventure :
    Card.check
      (.adventurer blueCreature
        { characteristics :=
          { name := some "", cost := some [pip .blue], types := [.sorcery],
            subtypes := [spellType "Adventure"] } }) = [] := by
  decide

/-- an adventurer card whose inset frame is a plain instant, naming no Adventure -/
theorem badUnnamedAdventure :
    Card.check
      (.adventurer blueCreature
        { characteristics :=
          { name := some "", cost := some [pip .blue], types := [.instant] } })
      = [.adventureInset] := by
  decide

/-- A green 1/1 creature front face. -/
def greenCreature : CardFace :=
  { characteristics :=
    { name := some "", cost := some [pip .green], types := [.creature], power := stat 1,
      toughness := stat 1 } }

/-- a flip card whose upside-down half is a creature -/
theorem okCreatureFlipHalf :
    Card.check
      (.flip greenCreature
        { characteristics :=
          { name := some "", types := [.creature], power := stat 2, toughness := stat 2 } })
      = [] := by
  decide

/-- a flip card whose upside-down half is an instant -/
theorem badSpellFlipHalf :
    Card.check
      (.flip greenCreature { characteristics := { name := some "", types := [.instant] } })
      = [.flipHalf] := by
  decide

/-- a transforming card whose back face prints no mana cost -/
theorem okTransformingBackWithoutCost :
    Card.check
      (.transforming greenCreature
        { characteristics :=
          { name := some "", types := [.creature], power := stat 1, toughness := stat 1 } })
      = [] := by
  decide

/-- a transforming card whose back face prints a mana cost of its own -/
theorem badTransformingBackWithCost :
    Card.check
      (.transforming greenCreature
        { characteristics :=
          { name := some "", cost := some [pip .green], types := [.creature], power := stat 1,
            toughness := stat 1 } }) = [.cardCost] := by
  decide

/-- "your devotion to black" -/
theorem okSingularDevotion : Amount.check (DRS.ofList []) (.devotion .you (.lit .black) none) = [] := by decide

/-- "your opponents' devotion to black" -/
theorem badPluralDevotion :
    Amount.check (DRS.ofList []) (.devotion (.described .all (.isOpponentOf .you)) (.lit .black) none)
      = [.singular] := by
  decide

/-- "Flip a coin. Take an extra turn for each coin that comes up heads." -/
theorem okCoinsShowingAfterFlip :
    Action.check (DRS.ofList []) (.sequentially [flipCoins 1, extraTurn (.coinsShowing .heads)])
      = [] := by
  decide

/-- "Take an extra turn for each coin that comes up heads." -/
theorem badCoinsShowingWithoutFlip :
    Action.check (DRS.ofList []) (extraTurn (.coinsShowing .heads)) = [.coinFlipInScope] :=
        by
  decide

/-- "Transform target creature." -/
theorem okTurnOverOnField : Action.check (DRS.ofList []) (.turnOver (target isCreature)) = [] := by decide

/-- "Room" -/
def room : Predicate := .hasSubtype (enchantmentType "Room")

/-- "unlock a locked door of a Room you control" -/
theorem okUnlockRoomDoor :
    Action.check (DRS.ofList [])
      (.unlock
        (.doorOf (some .locked)
          (.described (.target (upTo 1)) (.and [room, .hasPossessor .controller .you]))))
      = [] := by
  decide

/-- "When you unlock this door, this Room deals N damage to each opponent." -/
def doorDamage (amount : Nat) : Ability :=
  when (.does (.actor .you) (.unlock .thisDoor))
    (Action.instruct thisRoom (.dealDamage (.lit amount) (each (.isOpponentOf .you)) .any))

/-- "When you unlock this door, this Room deals 1 damage to each opponent.": a door header
belongs to a Room's shared line. -/
theorem okRoomDoorHeaderOnSharedLine :
    Card.check
      (.sharedLineSplit
        { characteristics :=
          { types := [.enchantment], subtypes := [enchantmentType "Room"] } }
        ⟨"", some [pip .red], [doorDamage 1]⟩ ⟨"", some [generic 3, pip .red], [doorDamage 2]⟩)
      = [] := by
  decide

/-- "When you unlock this door, this Room deals 1 damage to each opponent." -/
theorem badDoorHeaderOffSharedLine :
    Card.check
      (one
        { name := some "", cost := some [pip .red], types := [.enchantment],
          subtypes := [enchantmentType "Room"], text := [doorDamage 1] }) = [.doorFrame] := by
  decide

theorem generalManaSymbolMatcher :
    Predicate.checkAt .object (DRS.ofList []) (.manaCostHas (.simple (.specific (.of .red)))) = [] := by decide

theorem playerItRead :
    NounPhrase.checkAt (some .player) [({ det := .a, plur := .one, payload := .player false, idents := [] } : Referent)] they = [] := by decide

theorem delayedDoorTraversal :
    Action.namesThisDoor (delay (.does (.actor .you) (.unlock .thisDoor)) (draw (.lit 1)))
      = true := by
  decide

theorem distributiveGroupSurvives :
    Action.check (DRS.ofList [])
      (.sequentially
        [ Templates.instruct (each (.isOpponentOf .you)) (.enact (.action "shuffle") (.shuffle (libraryOf .actor)) none none),
          Templates.instruct (those .player) (.changeLife (.down (.lit 1))) ]) = [] := by
  decide

theorem redundantSameClauseShuffleHandoffIsRefused :
    Action.check (DRS.ofList [])
      (Templates.instruct (each (.isOpponentOf .you))
        (.enact (.action "shuffle")
          (Templates.instruct they (.shuffle (libraryOf .actor))) none none)) =
      [.anaphor (.word .player) .one 0] := by
  decide

theorem secondChooserDevotionRead :
    Amount.check [qualityB .color, qualityB .color] (.devotion .you theLastChosenColor none)
      = [] := by
  decide

theorem emblemGrantorRead : NounPhrase.checkAt (some .object) (DRS.ofList []) (.theGrantor .emblem) = [] := by
  decide

/-- The alternative-cost keywords whose payment reads back. -/
def alternativeCostKeywords : List KeywordLabel :=
  [ (.named "escape"), (.named "foretell"), (.named "bestow"), (.named "disguise"), (.named "mutate"), (.named "overload"), (.named "disturb"), (.named "dash"), (.named "evoke"),
    (.named "blitz"), (.named "cleave"), (.named "harmonize"), (.named "impending"), (.named "awaken"), (.named "buyback"), (.named "casualty"), (.named "squad"),
    (.named "offspring"), (.named "gift"), (.named "replicate") ]

theorem alternativeCostReadbacks :
    alternativeCostKeywords.all
      (fun k => (Amount.check (DRS.ofList []) (paidCostRead (.byKeyword k) none .this)).isEmpty) = true := by
  decide

theorem modalCostReadbacks :
    Amount.check (DRS.ofList []) (paidCostRead (.byKeyword (.named "entwine")) none .this) ++
      Amount.check (DRS.ofList []) (timesPaid (.byKeyword (.named "escalate")) .this) = [] := by
  decide

/-- "It doesn't untap during its controller's next untap step." -/
theorem okUntapNextSingleIt :
    Action.check (DRS.ofList [])
      (.sequentially [.setStatus .tapped (target isCreature), .skipUntap it (.lit 1)])
      = [] := by
  decide

theorem recentTappedArtifactIsSelectedForUntapSkip :
    let taps := .sequentially [.setStatus .tapped (target isCreature), .setStatus .tapped (target isArtifact)]
    let after := (stepAction 64 .postEvent .obj (DRS.ofList []) taps).1
    Action.check (DRS.ofList []) (.sequentially
      [.setStatus .tapped (target isCreature), .setStatus .tapped (target isArtifact), .skipUntap it (.lit 1)]) = [] ∧
      (firstReach .bare .one (nextClause after)).map (·.id) = some 1 ∧
      (after.findById? 1).map (·.ty) = some [.artifact] := by decide

/-- "I — Draw a card." -/
theorem okChapterMark :
    Ability.check (DRS.ofList []) (when (.chapterMark [1]) (draw (.lit 1))) = [] := by decide

/-- "— Draw a card." -/
theorem badEmptyChapterMark :
    Ability.check (DRS.ofList []) (when (.chapterMark []) (draw (.lit 1)))
      = [.chapterMarks] := by
  decide

/-- "II, II — Draw a card." -/
theorem badRepeatedChapterMark :
    Ability.check (DRS.ofList []) (when (.chapterMark [2, 2]) (draw (.lit 1)))
      = [.chapterMarks] := by
  decide

/-- "Roll a d20. 1—9 | Draw a card." -/
theorem okResultsTableAfterRoll :
    Action.check (DRS.ofList [])
      (.sequentially [rollDice 1 20, .applyResultsTable [⟨fromTo 1 9, draw (.lit 1)⟩]])
      = [] := by
  decide

/-- "1—9 | Draw a card." -/
theorem badTableWithoutRoll :
    Action.check (DRS.ofList []) (.applyResultsTable [⟨fromTo 1 9, draw (.lit 1)⟩])
      = [.outcomeInScope .rollResult 0] := by
  decide

/-- A red 2/2 creature face for the leveler pins. -/
def redCreature : CardFace :=
  { characteristics :=
    { name := some "", cost := some [pip .red], types := [.creature], power := stat 2,
      toughness := stat 2 } }

/-- "LEVEL <range> [P/T]" with no text. -/
def band (range : LevelRange) (power toughness : Nat) : LevelBand :=
  { range := range, power := stat power, toughness := stat toughness }

/-- "LEVEL 1-2 [2/3]" beside "LEVEL 3+ [2/4]" on a 2/2 creature -/
theorem okLevelerBands :
    Card.check (.leveler redCreature [band (.between 1 2) 2 3, band (.atLeast 3) 2 4]) = [] := by
  decide

/-- "LEVEL 4-2 [2/3]": no number of level counters is at least 4 and at most 2 [CR#711.2a] -/
theorem badEmptyLevelRange :
    Card.check (.leveler redCreature [band (.between 4 2) 2 3]) = [.levelRange] := by decide

/-- "LEVEL 1-4 [2/3]" beside "LEVEL 3+ [2/4]": level 3 falls in both, each setting base power
and toughness [CR#711.2a,711.2b] -/
theorem badOverlappingLevelBands :
    Card.check (.leveler redCreature [band (.between 1 4) 2 3, band (.atLeast 3) 2 4])
      = [.bandsDisjoint] := by
  decide

/-- "LEVEL 1+ [2/2]" printed on a sorcery: no striated text box to level [CR#711.1] -/
theorem badLevelBandOffLevelerFrame :
    Card.check
      (.leveler
        { characteristics := { name := some "", cost := some [pip .red], types := [.sorcery] } }
        [band (.atLeast 1) 2 2]) = [.levelerFrame] := by
  decide

/-- A 2/2 artifact creature face for the prototype pins. -/
def artifactCreature : CardFace :=
  { characteristics :=
    { name := some "", cost := some [generic 2], types := [.artifact, .creature], power := stat 2,
      toughness := stat 2 } }

/-- "Prototype {1}{R} — 1/1" on a 2/2 artifact creature -/
theorem okPrototypeAlt :
    Card.check
      (.prototype artifactCreature
        { cost := some [generic 1, pip .red], power := stat 1, toughness := stat 1 }) = [] := by
  decide

/-- "Prototype — 1/1": the inset frame's second set includes a mana cost, and a prototyped
spell is cast using only that one [CR#702.160a,718.3a] -/
theorem badPrototypeWithoutAltCost :
    Card.check
      (.prototype artifactCreature { cost := some [], power := stat 1, toughness := stat 1 })
      = [.nonEmpty] := by
  decide

/-- "Prototype {1}{R} — loyalty 3": the inset frame's second set is power and toughness
[CR#702.160a,718.1]. The Idris pin refutes the box; the frame law fails on the same set. -/
theorem badPrototypeAltLoyaltyBox :
    Card.check
      (.prototype artifactCreature { cost := some [generic 1, pip .red] })
      = [.prototypeFrame, .cardBox] := by
  decide

/-- "Prototype {1}{R} — 1/1" printed on a noncreature artifact: no first set of power and
toughness for the inset frame to give a second [CR#718.1,718.2] -/
theorem badPrototypeOffCreatureFrame :
    Card.check
      (.prototype
        { characteristics := { name := some "", cost := some [generic 2], types := [.artifact] } }
        { cost := some [generic 1, pip .red], power := stat 1, toughness := stat 1 })
      = [.prototypeFrame] := by
  decide

/-- "When you cast this spell, copy it for each other spell that was cast before it this turn."
Storm counts the spells cast earlier in the turn [CR#702.40a], not every spell cast during
it. -/
theorem stormCountsEarlierThisTurn :
    stormExpansion =
      when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
        (.sequentially
          [ .copy .fromStack thisSpell
              (eventCount (.procedure .cast (a (.and [isSpell, .otherThan thisSpell])) .itsOwnCost none none none) (.actor (a .anyPlayer)) earlierThisTurn)
              [],
            offer (.chooseNewTargets (.pro (.word .copy) .many)) ]) := by
  rfl

end Semantics.Proofs.Faces
