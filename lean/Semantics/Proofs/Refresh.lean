import Semantics.Macros
import Semantics.Check.Card
import Semantics.Proofs.Pin

/-!
# Semantics.Proofs.Refresh

Pins for the exchange laws that arrived with the 2026-09-04 refresh of the Idris reference:
twins of the exchange pins in the Idris `Proofs/Zone` module, kept here in `Pin` form beside
the theorem-form Zone family (`Semantics.Proofs.Zone`). The refresh's Choice, Keyword,
Trigger, Turn, and Mana pins live in their family modules.
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

end Semantics.Proofs.Refresh
