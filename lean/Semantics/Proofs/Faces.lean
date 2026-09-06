import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Faces

Port of `idris/src/Experimental/Proofs/Faces.idr`: the pins of the Faces family, in theorem
form, each closed by `decide`. The names and the sentences are the Idris ones.

Not ported: `badFaceAsFlipAlternative`, an Idris type error (a flip card's other half is a
characteristics set, not a face [CR#710.1]); here a flip card's other half is a `CardFace`
too, so the distinction is not a type's to make.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Faces

/-- An activated ability with no limit, guard, or activator. -/
def act (cost : Cost) (instruction : Instruction) (timing : Option Timing := none) : Ability :=
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
          text := [.spell none (draw (.lit 1) (agent := .you))] }) = [] := by
  decide

/-- "Draw a card." -/
theorem badSpellAbilityOnPermanent :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.creature],
          text := [.spell none (draw (.lit 1) (agent := .you))], power := stat 1, toughness := stat
              1 })
      = [.cardText] := by
  decide

/-- "Creatures you control get +1/+1." -/
theorem badStaticOnSorcery :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.sorcery],
          text := [.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))] })
      = [.cardText] := by
  decide

/-- "Cast this spell only during the declare attackers step. Draw a card." -/
theorem okCastWindowOnSpell :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.instant],
          text := [.spell (some (.duringPart .declareAttackers none)) (draw (.lit 1) (agent :=
              .you))] })
      = [] := by
  decide

/-- The same sentence spelled as a static permitting the cast. A cast window is stated by the
spell [CR#506.7], which is `okCastWindowOnSpell`'s window slot, not a permission a spell
card's static ability grants. -/
theorem badCastWindowAsDeonticStatic :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.instant],
          text :=
            [ .static
                (Primitives.StaticSpec.partScope .declareAttackers none
                  (deontic .this .permit [.action "Cast"] .patient .noPatient)),
              .spell none (draw (.lit 1) (agent := .you)) ] }) = [.cardText] := by
  decide

/-- "Flying" -/
theorem badKeywordOnInstant :
    Card.check
      (one
        { name := some "", cost := some [pip .red], types := [.instant],
          text := [keyword "Flying"] }) = [.cardText] := by
  decide

/-- "{T}: Draw a card." -/
theorem badTapSorcery :
    Card.check
      (one
        { name := some "Impossible Tap Sorcery", cost := some [pip .blue], types := [.sorcery],
          text := [act .tapSymbol (draw (.lit 1) (agent := .you))] }) = [.cardText] := by
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
    Ability.check []
      (whenever (.statusEvent (a permanent) .faceDown) (draw (.lit 1) (agent := .you))) = [] := by
  decide

/-- "Target creature doesn't untap during its controller's next untap step." -/
theorem okDoesntUntapNextOnBattlefield :
    Instruction.check [] (.skipUntap (target creature) (.lit 1)) = [] := by decide

theorem badUntapNextGraveyard :
    Instruction.check []
      (.skipUntap (target (.and [creature, .inZone (graveyardOf .you)])) (.lit 1))
      = [.zoneIs .battlefield] := by
  decide

/-- "Turn target creature face down." -/
theorem okTurnFaceDownOnBattlefield :
    Instruction.check [] (.setStatus .faceDown (target creature)) = [] := by decide

/-- "Turn target creature card in your graveyard face down." -/
theorem badTurnFaceDownGraveyard :
    Instruction.check []
      (.setStatus .faceDown (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- "Target creature card in your hand phases out." -/
theorem badPhasesOutInHand :
    Instruction.check [] (.setStatus .phasedOut (target (.and [creature, .inZone (handOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- a white creature face reading "Protection from red" -/
theorem okProtectionOnCreature :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.creature],
          text := [keywordQuality "Protection" (.colorIs .red)], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

theorem badStarlessDefinedPt :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.creature],
          text := [.static (.ptDefinition thisCreature .bothEach (countOf creatureYouControl))],
          power := stat 2, toughness := stat 2 }) = [.cardBox] := by
  decide

/-- "each creature with flying" -/
theorem okKnownKeywordPredicate :
    Predicate.check .object [] (.hasKeyword (.the "Flying")) = [] := by decide

/-- "each creature with flyign" -/
theorem badUnknownKeywordPredicate :
    Predicate.check .object [] (.hasKeyword (.the "Flyign")) = [.knownKeywordTerm] := by decide

/-- "Protection from red" on a creature card -/
theorem okProtectionOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.creature],
          subtypes := [creatureType "Soldier"],
          text := [keywordQuality "Protection" (.colorIs .red)], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

/-- "Protection from red" -/
theorem badProtectionOnInstant :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.instant],
          text := [keywordQuality "Protection" (.colorIs .red)] }) = [.cardText] := by
  decide

/-- "Protection from red" -/
theorem okProtectionFromAColor :
    Ability.check [] (keywordQuality "Protection" (.colorIs .red)) = [] := by decide

/-- "Protection from player" -/
theorem badProtectionFromPlayerRestriction :
    Ability.check [] (.keyword "Protection" [.subject .anyPlayer] none)
      = [.keywordParamFits "Protection"] := by
  decide

/-- "Equip {2}" on an Equipment card -/
theorem okEquipOnEquipment :
    Card.check
      (one
        { name := some "", cost := some [generic 1], types := [.artifact],
          subtypes := [artifactType "Equipment"],
          text := [keywordCosting "Equip" (.mana [generic 2])] }) = [] := by
  decide

/-- "Equip {2}" -/
theorem badEquipOnSorcery :
    Card.check
      (one
        { name := some "", cost := some [generic 1], types := [.sorcery],
          text := [keywordCosting "Equip" (.mana [generic 2])] }) = [.cardText] := by
  decide

/-- "each of up to two target creatures" -/
theorem okEachOfATargetGroup :
    NounPhrase.check (some .object) [] (.eachOf (.described (.target (upTo 2)) creature))
      = [] := by
  decide

/-- "each of one or more creatures" -/
theorem badEachOfCountedGroup :
    NounPhrase.check (some .object) [] (.eachOf (counted (atLeast 1) creature))
      = [.groupMention] := by
  decide

/-- "enchanted player" -/
theorem okEnchantedPlayer :
    NounPhrase.check (some .player) [] (.attachHost .enchanted .player) = [] := by decide

/-- "equipped player" -/
theorem badEquippedPlayer :
    NounPhrase.check (some .player) [] (.attachHost .equipped .player) = [.attachHeadOk] := by
  decide

/-- "fortified creature" -/
theorem badFortifiedCreatureNoun :
    NounPhrase.check (some .object) [] (.attachHost .fortified (.type .creature))
      = [.attachHeadOk] := by
  decide

/-- "Target creature gains indestructible." -/
theorem okGainsIndestructible :
    StaticSpec.check [] (.abilityGrant (target creature) (keyword "Indestructible")) = [] := by
        decide

/-- "Target creature gains flash." -/
theorem badBattlefieldFlash :
    StaticSpec.check [] (.abilityGrant (target creature) (keyword "Flash")) = [.grantSubject] := by
  decide

/-- "Target spell gains indestructible." -/
theorem badSpellIndestructible :
    StaticSpec.check [] (.abilityGrant (target spell) (keyword "Indestructible")) = [.grantSubject]
        := by
  decide

/-- a "Kindred Enchantment — Merfolk" card -/
theorem okKindredWithAnotherType :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred, .enchantment],
          subtypes := [creatureType "Merfolk"], text := [keyword "Flying"] }) = [] := by
  decide

/-- a "Kindred Enchantment — Siege" card -/
theorem badSiegeWithoutBattle :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred, .enchantment],
          subtypes := [.of .battle "Siege"], text := [keyword "Flying"] }) = [.cardLine] := by
  decide

/-- a "Kindred — Merfolk" card naming no other card type -/
theorem badKindredAlone :
    Card.check
      (one
        { name := some "", cost := some [generic 2], types := [.kindred],
          subtypes := [creatureType "Merfolk"], text := [keyword "Flying"] }) = [.cardLine] := by
  decide

/-- "Enchanted creature can't attack." -/
theorem okEnchantedCreatureCantAttack :
    Ability.check []
      (.static
        (deontic (.attachHost .enchanted (.type .creature)) .forbid [.core .attack] .agent
          .noPatient))
      = [] := by
  decide

/-- "Enchanted planeswalker can't attack.": a planeswalker an effect has made a creature can
attack [CR#205.1b], and the restriction is created even while it is not one [CR#208.3a]. -/
theorem okPlaneswalkerAttacks :
    Ability.check []
      (.static
        (deontic (.attachHost .enchanted (.type .planeswalker)) .forbid [.core .attack] .agent
          .noPatient)) = [] := by
  decide

/-- "you pay {2}" -/
theorem okPayMana : Instruction.check [] (.pay (.mana [generic 2]) .once (agent := .you)) = [] := by
    decide

/-- "you pay [+1]" -/
theorem badPayLoyalty :
    Instruction.check [] (.pay (.loyaltySymbol (.up 1)) .once (agent := .you)) = [.payable] := by
        decide

/-- A legendary Jace planeswalker face with no text or loyalty yet. -/
def jace : Characteristics :=
  { name := some "", cost := some [pip .blue], supertypes := [.legendary], types := [.planeswalker],
    subtypes := [.of .planeswalker "Jace"] }

/-- "[+1]: Draw a card." on a planeswalker card -/
theorem okLoyaltyOnPlaneswalker :
    Card.check
      (one
        { jace with text := [act (.loyaltySymbol (.up 1)) (draw (.lit 1) (agent := .you))],
                    loyalty := stat 3 }) = [] := by
  decide

/-- "[+1]: Draw a card." -/
theorem badLoyaltySorcery :
    Card.check
      (one
        { name := some "Impossible Loyalty Sorcery", cost := some [pip .blue], types := [.sorcery],
          text := [act (.loyaltySymbol (.up 1)) (draw (.lit 1) (agent := .you))] }) = [.cardText] :=
              by
  decide

/-- "a token that's a copy of target creature, except it's an artifact" -/
theorem okCopyTypeException :
    Instruction.check []
      (Primitives.Instruction.create (.lit 1) (.copyOf (target creature) [Primitives.CopyExcept.types [.artifact] []]) [] (agent := .you)) = []
          := by
  decide

theorem badEmptyCopyTypeException :
    Instruction.check [] (Primitives.Instruction.create (.lit 1) (.copyOf (target creature) [Primitives.CopyExcept.types [] []]) [] (agent :=
        .you))
      = [.lineNonEmpty] := by
  decide

/-- "Copy target instant or sorcery spell." -/
theorem okCopyStackSpell :
    Instruction.check []
      (.copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [] (agent := .you)) = []
          := by
  decide

/-- "Copy target creature." -/
theorem badCopyPermanent :
    Instruction.check [] (.copy .fromStack (target creature) (.lit 1) [] (agent := .you))
      = [.copySourceOk] := by
  decide

/-- "Create a token that's a copy of target creature. Untap that token." -/
theorem okSetStatusOnBattlefield :
    Instruction.check []
      (.sequentially
        [ Primitives.Instruction.create (.lit 1) (.copyOf (target creature) []) [] (agent := .you),
          .setStatus .untapped (that .token) ]) = [] := by
  decide

/-- "Copy target instant or sorcery spell. Untap that token." The Idris pin refutes the
anaphor; the unresolved "that token" has no zone either. -/
theorem badStackCopyAsToken :
    Instruction.check []
      (.sequentially
        [ .copy .fromStack (target (.and [instantOrSorcery, spell])) (.lit 1) [] (agent := .you),
          .setStatus .untapped (that .token) ])
      = [.anaphor (.word .token) .one 0, .zoneIs .battlefield] := by
  decide

/-- "Create a token that's a copy of target creature. Untap that copy." -/
theorem badTokenCopyAsCopyMention :
    Instruction.check []
      (.sequentially
        [ Primitives.Instruction.create (.lit 1) (.copyOf (target creature) []) [] (agent := .you),
          .setStatus .untapped (that .copy) ])
      = [.anaphor (.word .copy) .one 0, .zoneIs .battlefield] := by
  decide

/-- "Saga — I, II, III — Draw a card." -/
theorem okChapterOnSaga :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.enchantment],
          subtypes := [enchantmentType "Saga"],
          text :=
            [ when (.chapterMark [1]) (draw (.lit 1) (agent := .you)),
              when (.chapterMark [2]) (draw (.lit 1) (agent := .you)),
              when (.chapterMark [3]) (draw (.lit 1) (agent := .you)) ] }) = [] := by
  decide

theorem badChapterOnNonSaga :
    Card.check
      (one
        { name := some "", cost := some [pip .white], types := [.enchantment],
          text := [when (.chapterMark [1]) (draw (.lit 1) (agent := .you))] })
      = [.chapterFrame] := by
  decide

/-- "You may sacrifice a Mountain rather than pay this spell's mana cost." -/
theorem okAltCostSacrifice :
    StaticSpec.check []
      (.altCost .this
        (some (.perform (sacrifice (a (.and [land, .hasSubtype (landType "Mountain")])) (agent :=
            .you)))))
      = [] := by
  decide

/-- "You may {T} rather than pay this spell's mana cost." -/
theorem badAltCostTapSymbol :
    StaticSpec.check [] (.altCost .this (some .tapSymbol)) = [.altPayment] := by decide

/-- "You may [+1] rather than pay this spell's mana cost." -/
theorem badAltCostLoyaltySymbol :
    StaticSpec.check [] (.altCost .this (some (.loyaltySymbol (.up 1)))) = [.altPayment] := by
  decide

/-- "Escalate {2}. Choose one or both —" -/
theorem okEscalateWithModes :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text :=
            [ keywordCosting "Escalate" (.mana [generic 2]),
              .spell none
                (chooseModes (.range (some 1) (some 2))
                  [ get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
                    get (target creature) (.down (.lit 1)) (.down (.lit 1))
                      (some untilEndOfTurn) ]) ] }) = [] := by
  decide

/-- "Escalate {2}" -/
theorem badEscalateWithoutModes :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text := [keywordCosting "Escalate" (.mana [generic 2])] }) = [.modalFrame] := by
  decide

/-- "Entwine {2}" -/
theorem badEntwineWithoutModes :
    Card.check
      (one
        { name := some "", cost := some [pip .green], types := [.sorcery],
          text := [keywordCosting "Entwine" (.mana [generic 2])] }) = [.modalFrame] := by
  decide

theorem jointCrossAbilityChoice :
    Card.check
      (.singleFaced
        { characteristics :=
            { name := some "Joint choice witness", types := [.creature],
              subtypes := [creatureType "Shapeshifter"],
              text :=
                [ .static (.abilityGrant thisCreature (keywordQuality "Protection" (ofChosen
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
          text := [keywordCosting "CumulativeUpkeep" (.mana [generic 2])] }) = [] := by
  decide

/-- "Cumulative upkeep {2}" -/
theorem badCumulativeUpkeepOnSpell :
    Card.check
      (one
        { name := some "", cost := some [pip .blue], types := [.sorcery],
          text := [keywordCosting "CumulativeUpkeep" (.mana [generic 2])] }) = [.cardText] := by
  decide

/-- "Renown 1 (When this creature deals combat damage to a player, …)" -/
theorem okRenownWithRenownExpansion :
    Ability.check [] (.keyword "Renown" [.number (.lit 1)] (some (renownExpansion 1)))
      = [] := by
  decide

/-- "Flying (When this creature deals combat damage to a player, …)" -/
theorem badBodyOnBodilessKeyword :
    Ability.check [] (.keyword "Flying" [] (some (renownExpansion 1)))
      = [.keywordBodyFits "Flying"] := by
  decide

/-- "Renown 1 (When you cast this spell, copy it …)" -/
theorem badRenownWithStormExpansion :
    Ability.check [] (.keyword "Renown" [.number (.lit 1)] (some stormExpansion))
      = [.keywordBodyFits "Renown"] := by
  decide

/-- "your commander" -/
theorem okPossessedCommander :
    NounPhrase.check (some .object) [] (.designated "commander" .you) = [] := by decide

/-- "your monarch" -/
theorem badPossessedMonarch :
    NounPhrase.check (some .object) [] (.designated "the monarch" .you)
      = [.designationScope "the monarch"] := by
  decide

/-- "Unearth {B}" -/
theorem okCostedUnearth :
    Ability.check [] (keywordCosting "Unearth" (.mana [pip .black])) = [] := by decide

/-- "Unearth" -/
theorem badBareUnearth :
    Ability.check [] (.keyword "Unearth" [] none) = [.keywordParamFits "Unearth"] := by decide

/-- "Retrace {1}" -/
theorem badCostedRetrace :
    Ability.check [] (keywordCosting "Retrace" (.mana [generic 1]))
      = [.keywordParamFits "Retrace"] := by
  decide

/-- "Unearth {B}" on a creature card -/
theorem okUnearthOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.creature],
          subtypes := [creatureType "Zombie"],
          text := [keywordCosting "Unearth" (.mana [pip .black])], power := stat 2,
          toughness := stat 2 }) = [] := by
  decide

/-- "Unearth {B}" -/
theorem badUnearthOnSpellCard :
    Card.check
      (one
        { name := some "", cost := some [pip .black], types := [.instant],
          text := [keywordCosting "Unearth" (.mana [pip .black])] }) = [.cardText] := by
  decide

/-- "Flashback {2}{U}" -/
theorem badFlashbackOnPermanentCard :
    Card.check
      (one
        { name := some "", cost := some [generic 2, pip .blue], types := [.artifact],
          text := [keywordCosting "Flashback" (.mana [generic 2, pip .blue])] })
      = [.cardText] := by
  decide

/-- "for each attacking creature you control. You gain that much life." -/
theorem okThatMuchAfterQuantity :
    Instruction.check []
      (.sequentially
        [ loseLife
            (forEach 1 (.and [attacking, creature, .hasPossessor .controller .you])) (agent :=
                (target .opponent)),
          gainLife .thatMuch (agent := .you) ]) = [] := by
  decide

/-- "Flip a coin. Draw that many cards." -/
theorem badThatMuchAfterFlip :
    Instruction.check [] (.sequentially [flipCoins 1 (agent := .you), draw .thatMuch (agent := .you)])
      = [.quantOutcomeInScope 0] := by
  decide

/-- "Flip a coin. If you win the flip, draw a card." -/
theorem okFlipArmAfterFlip :
    Instruction.check []
      (.sequentially [flipCoins 1 (agent := .you), doIf (.flipCalled .you .wins) (draw (.lit 1) (agent
          := .you))])
      = [] := by
  decide

/-- "If you win the flip, draw a card." -/
theorem badFlipArmWithoutFlip :
    Instruction.check [] (doIf (.flipCalled .you .wins) (draw (.lit 1) (agent := .you)))
      = [.coinFlipInScope] := by
  decide

/-- "Roll a d6." -/
theorem okSixSidedDie : Instruction.check [] (.rollDice (.lit 1) (.sides 6) (agent := .you)) = [] :=
    by decide

/-- "Roll a d0." -/
theorem badNoughtSidedDie :
    Instruction.check [] (.rollDice (.lit 1) (.sides 0) (agent := .you)) = [.nonZeroQ] := by decide

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
theorem okSingularDevotion : Amount.check [] (.devotion .you (.lit .black) none) = [] := by decide

/-- "your opponents' devotion to black" -/
theorem badPluralDevotion :
    Amount.check [] (.devotion (.playerGroup .yourOpponents) (.lit .black) none)
      = [.singular] := by
  decide

/-- "Flip a coin. Take an extra turn for each coin that comes up heads." -/
theorem okCoinsShowingAfterFlip :
    Instruction.check [] (.sequentially [flipCoins 1 (agent := .you), Primitives.Instruction.addTurn (.coinsShowing .heads)
        (agent := .you)])
      = [] := by
  decide

/-- "Take an extra turn for each coin that comes up heads." -/
theorem badCoinsShowingWithoutFlip :
    Instruction.check [] (Primitives.Instruction.addTurn (.coinsShowing .heads) (agent := .you)) = [.coinFlipInScope] :=
        by
  decide

/-- "Transform target creature." -/
theorem okTurnOverOnField : Instruction.check [] (.turnOver (target creature)) = [] := by decide

/-- "Transform target creature card in your graveyard." -/
theorem badTurnOverOffField :
    Instruction.check [] (.turnOver (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.zoneIs .battlefield] := by
  decide

/-- "Room" -/
def room : Predicate := .hasSubtype (enchantmentType "Room")

/-- "unlock a locked door of a Room you control" -/
theorem okUnlockRoomDoor :
    Instruction.check []
      (.unlock
        (.doorOf (some .locked)
          (.described (.target (upTo 1)) (.and [room, .hasPossessor .controller .you]))))
      = [] := by
  decide

/-- "unlock this door" -/
theorem badUnlockThisDoor : Instruction.check [] (.unlock .thisDoor) = [.doorNamesHost] := by
  decide

/-- "unlock a locked door of a Room card in your graveyard" -/
theorem badUnlockDoorOffBattlefield :
    Instruction.check [] (.unlock (.doorOf (some .locked) (a (.and [room, .inZone graveyard]))))
      = [.zoneIs .battlefield] := by
  decide

theorem badDoorOfBareThis :
    Instruction.check [] (.unlock (.doorOf (some .locked) .this)) = [.zoneIs .battlefield] := by
  decide

/-- "When you unlock this door, this Room deals N damage to each opponent." -/
def doorDamage (amount : Nat) : Ability :=
  when (.unlocksDoor .you .thisDoor)
    (.dealDamage thisRoom (.lit amount) (each .opponent))

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
    Predicate.check .object [] (.manaCostHas (.simple (.specific (.of .red)))) = [] := by decide

theorem playerItRead :
    NounPhrase.check (some .player) [⟨.a, .one, .player false⟩] they = [] := by decide

theorem delayedDoorTraversal :
    Instruction.namesThisDoor (delay (.unlocksDoor .you .thisDoor) (draw (.lit 1) (agent := .you)))
      = true := by
  decide

theorem distributiveGroupSurvives :
    Instruction.check []
      (.sequentially
        [ .enact (.action "Shuffle") (.shuffle (agent := they)) (agent := (some (each .opponent))),
          .changeLife (.down (.lit 1)) (agent := (those .player)) ]) = [] := by
  decide

theorem secondChooserDevotionRead :
    Amount.check [qualityB .color, qualityB .color] (.devotion .you theLastChosenColor none)
      = [] := by
  decide

theorem emblemGrantorRead : NounPhrase.check (some .object) [] (.theGrantor .emblem) = [] := by
  decide

/-- The alternative-cost keywords whose payment reads back. -/
def alternativeCostKeywords : List KeywordLabel :=
  [ "Escape", "Foretell", "Bestow", "Disguise", "Mutate", "Overload", "Disturb", "Dash", "Evoke",
    "Blitz", "Cleave", "Harmonize", "Impending", "Awaken", "Buyback", "Casualty", "Squad",
    "Offspring", "Gift", "Replicate" ]

theorem alternativeCostReadbacks :
    alternativeCostKeywords.all
      (fun k => (Amount.check [] (paidCostRead (.byKeyword k) none .this)).isEmpty) = true := by
  decide

theorem modalCostReadbacks :
    Amount.check [] (paidCostRead (.byKeyword "Entwine") none .this) ++
      Amount.check [] (timesPaid (.byKeyword "Escalate") .this) = [] := by
  decide

/-- "It doesn't untap during its controller's next untap step." -/
theorem okUntapNextSingleIt :
    Instruction.check []
      (.sequentially [.setStatus .tapped (target creature), .skipUntap it (.lit 1)])
      = [] := by
  decide

theorem badUntapNextAmbiguousIt :
    Instruction.check []
      (.sequentially
        [ .setStatus .tapped (target creature),
          .setStatus .tapped (target artifact),
          .skipUntap it (.lit 1) ]) = [.anaphor .bare .one 2] := by
  decide

/-- "I — Draw a card." -/
theorem okChapterMark :
    Ability.check [] (when (.chapterMark [1]) (draw (.lit 1) (agent := .you))) = [] := by decide

/-- "— Draw a card." -/
theorem badEmptyChapterMark :
    Ability.check [] (when (.chapterMark []) (draw (.lit 1) (agent := .you)))
      = [.chapterMarks] := by
  decide

/-- "II, II — Draw a card." -/
theorem badRepeatedChapterMark :
    Ability.check [] (when (.chapterMark [2, 2]) (draw (.lit 1) (agent := .you)))
      = [.chapterMarks] := by
  decide

/-- "Roll a d20. 1—9 | Draw a card." -/
theorem okResultsTableAfterRoll :
    Instruction.check []
      (.sequentially [rollDice 1 20 (agent := .you), .applyResultsTable [⟨fromTo 1 9, draw (.lit 1)
          (agent := .you)⟩]])
      = [] := by
  decide

/-- "1—9 | Draw a card." -/
theorem badTableWithoutRoll :
    Instruction.check [] (.applyResultsTable [⟨fromTo 1 9, draw (.lit 1) (agent := .you)⟩])
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
      = [.manaRun] := by
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
      when (.casts .you (some thisSpell) none)
        (.sequentially
          [ .copy .fromStack thisSpell
              (eventCount (.casts (.gap .player) (some (a (.and [spell, .otherThan thisSpell])))
                none) (a .anyPlayer) .earlierThisTurn)
              [] (agent := .you),
            offer (.chooseNewTargets (.pro (.word .copy) .many .whole)) (agent := .you) ]) := by
  rfl

end Semantics.Proofs.Faces
