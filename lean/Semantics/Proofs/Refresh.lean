import Semantics.Macros
import Semantics.Check.Card
import Semantics.Proofs.Pin

/-!
# Semantics.Proofs.Refresh

Pins for the laws that arrived with the 2026-09-04 refresh of the Idris reference: twins of
pins in the Idris `Proofs/{Zone,Choice,Keyword,Trigger,Turn,Mana}` modules, whose families
are not yet ported. Each moves to its family module when that module is.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Refresh

/-! ## Zone: exchanges -/

/-- Each value would become equal to its own previous value [CR#701.12g]. -/
def exchangeValueWithItself : Pin (Instruction.check []) :=
  pin (prints "Tree of Redemption" "Exchange your life total with this creature's toughness."
        (.exchange (.values (lifeTotalOf .you) (toughnessOf thisCreature))))
      (says "Exchange this creature's power with this creature's power."
        (.exchange (.values (powerOf thisCreature) (powerOf thisCreature))))
      .selfExchanged

/-- A literal is not a value the game can set [CR#701.12g]. -/
def exchangeLiteralValue : Pin (Instruction.check []) :=
  pin (prints "Tree of Redemption" "Exchange your life total with this creature's toughness."
        (.exchange (.values (lifeTotalOf .you) (toughnessOf thisCreature))))
      (says "Exchange this creature's power with three."
        (.exchange (.values (powerOf thisCreature) (.lit 3))))
      .settableValue

/-- [CR#701.12h] A text box is exchanged between permanents on the battlefield. -/
def exchangeTextBoxInGraveyard : Pin (Instruction.check []) :=
  pin (says "Exchange the text boxes of this creature and another creature."
        (.exchange (.textBoxes thisCreature (a (.and [creature, .otherThan .this])))))
      (says "Exchange the text boxes of this creature and a creature card in your graveyard."
        (.exchange (.textBoxes thisCreature (a (.and [creature, .inZone (graveyardOf .you)])))))
      (.zoneIs .battlefield)

/-! ## Trigger and Turn: who attacks -/

/-- A player attacks [CR#508.1], and the ability triggers on the creatures they control being
declared [CR#508.3d]. -/
def graveyardAttacker : Pin (GameEvent.check []) :=
  pin (says "Whenever you attack, …" (attacks .you))
      (says "Whenever target creature card in your graveyard attacks, …"
        (attacks (target (.and [creature, .inZone (graveyardOf .you)]))))
      .attacker

/-- The attacker of an attacked player may be a player [CR#506.2]. -/
def attackedByGraveyardRelatum : Pin (Predicate.check .player []) :=
  pin (says "player an opponent is attacking" (.inCombat .attackedBy (some anOpponent)))
      (says "player attacked by target creature card in your graveyard"
        (.inCombat .attackedBy (some (target (.and [creature, .inZone (graveyardOf .you)])))))
      .combatRelOk

/-! ## Choice -/

/-- The same choice ridered on a death, which is not an event a choice can be made during. -/
def choiceRiderNotUnderway : Pin (Instruction.check []) :=
  pin (says "Choose target opponent with more life than you as you activate this ability."
        (chooseWhile
          (target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
          (.whileDoing (.activates .you thisAbility))))
      (says "Choose target opponent with more life than you as this creature dies."
        (chooseWhile
          (target (.and [.opponent, .compare [.playerStat .lifeTotal] .greater (lifeTotalOf .you)]))
          (.whileDoing (.dies thisCreature))))
      .eventUnderway

/-- Not a vote [CR#701.38c]; the gate is the plural choice. The same read with no number
chosen anywhere in the text. -/
def choseExtremeWithoutChoice : Pin (Instruction.check []) :=
  pin (prints "Menacing Ogre"
        "Each player secretly chooses a number. Then those numbers are revealed. Each player who \
          chose the highest number loses that much life."
        (.sequentially
          [ secretlyChooses (each .anyPlayer) (a (quality .number)),
            .choicesRevealed .numbers,
            losesLife (each (.and [.anyPlayer, .choseExtreme .max])) .thatMuch ]))
      (says "Each player who chose the highest number loses 1 life."
        (losesLife (each (.and [.anyPlayer, .choseExtreme .max])) (.lit 1)))
      .numberChoiceInScope

/-! ## Keyword: that ability -/

/-- The same gain with no ability chosen anywhere in the text. -/
def thatAbilityWithoutChoice : Pin (Instruction.check []) :=
  pin (says "Choose flying or trample. This creature gains that ability until end of turn."
        (.sequentially
          [ choose (a (qualityFrom .ability (.abilitiesAmong [.the "Flying", .the "Trample"]))),
            gains thisCreature (.thatAbility .theChoice) (some untilEndOfTurn) ]))
      (says "This creature gains that ability until end of turn."
        (gains thisCreature (.thatAbility .theChoice) (some untilEndOfTurn)))
      (.choiceRef .theChoice (.quality .ability) 0)

/-! ## Mana: tapped for mana of a type -/

/-- The six mana types are the five colors and colorless [CR#106.1b], so `ofColor` reads a
chosen color alone. -/
def tapForChosenNonManaType : Pin Card.check :=
  pin (says
        "As this artifact enters, choose a color. Whenever a basic land is tapped for mana of the \
          chosen color, draw a card."
        (.singleFaced
          { name := "", types := [.artifact],
            text :=
              [ .static (.entersChoice thisArtifact (.quality .color) none .openly),
                triggered .whenever
                  (.tappedForMana none (a (.and [land, .hasSupertype .basic]))
                    (some (.ofColor thatColor)))
                  (draw .you (.lit 1)) ] }))
      (says
        "As this artifact enters, choose a creature type. Whenever a basic land is tapped for \
          mana of the chosen type, draw a card."
        (.singleFaced
          { name := "", types := [.artifact],
            text :=
              [ .static (.entersChoice thisArtifact (.quality (.subtype .creature)) none .openly),
                triggered .whenever
                  (.tappedForMana none (a (.and [land, .hasSupertype .basic]))
                    (some (.ofColor thatColor)))
                  (draw .you (.lit 1)) ] }))
      (.choiceRef .theChoice (.quality .color) 0)

end Semantics.Proofs.Refresh
