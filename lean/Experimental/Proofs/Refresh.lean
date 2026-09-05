import Experimental.Macros
import Experimental.Check.Card

/-!
# Experimental.Proofs.Refresh

Pins for the laws that arrived with the 2026-09-04 refresh of the Idris reference: twins of
pins in the Idris `Proofs/{Zone,Choice,Keyword,Trigger,Turn,Mana}` modules, whose families
are not yet ported. Each moves to its family module when that module is.
-/

open Mtg Mtg.Macros

namespace Mtg.ProofsRefresh

/-! ## Zone: exchanges -/

/-- "Exchange your life total with this creature's toughness." (Tree of Redemption): two
settable values [CR#701.12g]. -/
theorem okExchangeTwoValues :
    Instruction.check [] (.exchange (.values (lifeTotalOf .you) (toughnessOf thisCreature))) = [] := by
  decide

/-- "Exchange this creature's power with this creature's power.": each value would become
equal to its own previous value [CR#701.12g]. -/
theorem badExchangeValueWithItself :
    Instruction.check [] (.exchange (.values (powerOf thisCreature) (powerOf thisCreature)))
      = [.selfExchanged] := by
  decide

/-- "Exchange this creature's power with three.": a literal is not a value the game can set
[CR#701.12g]. -/
theorem badExchangeLiteralValue :
    Instruction.check [] (.exchange (.values (powerOf thisCreature) (.lit 3))) = [.settableValue] := by
  decide

/-- "Exchange the text boxes of this creature and another creature." [CR#701.12h] -/
theorem okExchangeTextBoxes :
    Instruction.check []
      (.exchange (.textBoxes thisCreature (a (.and [creature, .otherThan .this])))) = [] := by
  decide

/-- "Exchange the text boxes of this creature and a creature card in your graveyard.": a text
box is exchanged between permanents on the battlefield. -/
theorem badExchangeTextBoxInGraveyard :
    Instruction.check []
      (.exchange (.textBoxes thisCreature (a (.and [creature, .inZone (graveyardOf .you)]))))
      = [.zoneIs .battlefield] := by
  decide

/-! ## Trigger and Turn: who attacks -/

/-- "Whenever you attack, …": a player attacks [CR#508.1], and the ability triggers on the
creatures they control being declared [CR#508.3d]. -/
theorem okPlayerAttacksHeader : GameEvent.check [] (attacks .you) = [] := by decide

/-- "Whenever target creature card in your graveyard attacks, …" -/
theorem badGraveyardAttacker :
    GameEvent.check [] (attacks (target (.and [creature, .inZone (graveyardOf .you)])))
      = [.attacker] := by
  decide

/-- "player an opponent is attacking": the attacker of an attacked player may be a player
[CR#506.2]. -/
theorem okAttackedByPlayer :
    Predicate.check .player [] (.inCombat .attackedBy (some anOpponent)) = [] := by decide

/-- "player attacked by target creature card in your graveyard" -/
theorem badAttackedByGraveyardRelatum :
    Predicate.check .player []
      (.inCombat .attackedBy (some (target (.and [creature, .inZone (graveyardOf .you)]))))
      = [.combatRelOk] := by
  decide

/-! ## Choice -/

/-- "Choose target opponent with more life than you as you activate this ability.": the rider
times the announcement, not the comparison. -/
theorem okChoiceWithActivationRider :
    Instruction.check []
      (chooseWhile
        (target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
        (.whileDoing (.activates .you thisAbility))) = [] := by
  decide

/-- The same choice ridered on a death, which is not an event a choice can be made during. -/
theorem badChoiceRiderNotUnderway :
    Instruction.check []
      (chooseWhile
        (target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
        (.whileDoing (.dies thisCreature))) = [.eventUnderway] := by
  decide

/-- "Each player secretly chooses a number. Then those numbers are revealed. Each player who
chose the highest number loses that much life." (Menacing Ogre): not a vote [CR#701.38c]; the
gate is the plural choice. -/
theorem okChoseExtremeAfterNumbers :
    Instruction.check []
      (.sequentially
        [ secretlyChooses (each .anyPlayer) (a (quality .number)),
          .choicesRevealed .numbers,
          losesLife (each (.and [.anyPlayer, .choseExtreme .max])) .thatMuch ]) = [] := by
  decide

/-- The same read with no number chosen anywhere in the text. -/
theorem badChoseExtremeWithoutChoice :
    Instruction.check [] (losesLife (each (.and [.anyPlayer, .choseExtreme .max])) (.lit 1))
      = [.numberChoiceInScope] := by
  decide

/-! ## Keyword: that ability -/

/-- "Choose flying or trample. This creature gains that ability until end of turn." -/
theorem okThatAbilityAfterChoice :
    Instruction.check []
      (.sequentially
        [ choose (a (qualityFrom .ability (.abilitiesAmong [.the "Flying", .the "Trample"]))),
          gains thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]) = [] := by
  decide

/-- "This creature gains that ability until end of turn", with no ability chosen anywhere in
the text. -/
theorem badThatAbilityWithoutChoice :
    Instruction.check [] (gains thisCreature (.thatAbility .theChoice) (some untilEndOfTurn))
      = [.choiceRef .theChoice (.quality .ability) 0] := by
  decide

/-! ## Mana: tapped for mana of a type -/

/-- "As this artifact enters, choose a color. Whenever a basic land is tapped for mana of the
chosen color, draw a card." -/
theorem okTapForChosenColorMana :
    Card.check (.singleFaced
      { name := "", types := [.artifact],
        text :=
          [ .static (.entersChoice thisArtifact (.quality .color) none .openly),
            triggered .whenever
              (.tappedForMana none (a (.and [land, .hasSupertype .basic]))
                (some (.ofColor thatColor)))
              (draw .you (.lit 1)) ] }) = [] := by
  decide

/-- "As this artifact enters, choose a creature type. Whenever a basic land is tapped for mana
of the chosen type, draw a card.": the six mana types are the five colors and colorless
[CR#106.1b], so `ofColor` reads a chosen color alone. -/
theorem badTapForChosenNonManaType :
    Card.check (.singleFaced
      { name := "", types := [.artifact],
        text :=
          [ .static (.entersChoice thisArtifact (.quality (.subtype .creature)) none .openly),
            triggered .whenever
              (.tappedForMana none (a (.and [land, .hasSupertype .basic]))
                (some (.ofColor thatColor)))
              (draw .you (.lit 1)) ] }) = [.choiceRef .theChoice (.quality .color) 0] := by
  decide

end Mtg.ProofsRefresh
