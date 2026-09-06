import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.Numbers

The pins of the number ruling [CR#107.1]: the only numbers the game uses are integers, so
`Amount.lit` carries an `Int` and a printed negative face is spellable; and every position
that consumes an `Amount` declares which regime of [CR#107.1b] it is in, `clamped` where a
result below zero reads as zero and `signed` where the negative stands.

There is no Idris original for this suite. The face pins close by `decide`, as every other
suite does. The slot pins close by `rfl`: `Amount` derives no `DecidableEq` (it sits in the
nested mutual block of `Phrase`, see the README), so an equation over
`List (Amount × NumberRegime)` has no `Decidable` instance to run — `rfl` asks the kernel for
the same evaluation without one.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.Numbers

/-! ## A printed negative face -/

/-- Spinal Parasite {5} — Artifact Creature — Insect, printed −1/−1. A game value such as a
creature's power may be less than zero [CR#107.1b], so the face is admitted. -/
theorem okPrintedNegativePowerAndToughness :
    Card.check (.singleFaced
      { characteristics :=
        { name := "Spinal Parasite", cost := some [generic 5],
          types := [.artifact, .creature], subtypes := [.of .creature "Insect"],
          power := some (.lit (-1)), toughness := some (.lit (-1)) } }) = [] := by
  decide

/-- Char-Rumbler {2}{R}{R} — Creature — Elemental, printed −1/3: a negative power beside a
positive toughness [CR#107.1b]. -/
theorem okPrintedNegativePower :
    Card.check (.singleFaced
      { characteristics :=
        { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red],
          types := [.creature], subtypes := [.of .creature "Elemental"],
          power := some (.lit (-1)), toughness := some (.lit 3) } }) = [] := by
  decide

/-! ## Clamped slots: an effect's result reads zero -/

/-- "You gain X life": you can't gain negative life, so a calculation below zero reads as
zero [CR#107.1b]. -/
theorem okGainsLifeClamped :
    (gainLife (.letter .x) (agent := .you)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "You lose X life" [CR#107.1b]. -/
theorem okLosesLifeClamped :
    (loseLife (.letter .x) (agent := .you)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "Draw X cards": a count of cards [CR#107.1b]. -/
theorem okDrawClamped :
    (draw (.letter .x) (agent := .you)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "This spell deals X damage to target creature": you can't deal negative damage
[CR#107.1b]. -/
theorem okDealDamageClamped :
    (Instruction.dealDamage .this (.letter .x) (target creature)).numberSlots
      = [(.letter .x, .clamped)] := by rfl

/-- "X is the number of creatures you control": the letter X, defined by the text, is a count
[CR#107.1b,107.3c]. -/
theorem okDefineClamped :
    (Instruction.define .x (.countOf (.described .all creature))).numberSlots
      = [(.countOf (.described .all creature), .clamped)] := by rfl

/-- The same letter defined by a static ability [CR#107.1b,107.3c]. -/
theorem okDefinesLetterClamped :
    (StaticSpec.letterDefinition .x (.countOf (.described .all creature))).numberSlots
      = [(.countOf (.described .all creature), .clamped)] := by rfl

/-- "This creature gets +2/+0": the sign is the constructor, and the magnitude beneath it is a
number the game does not take below zero [CR#107.1b]. -/
theorem okModifyUpClamped :
    (StaticSpec.modification thisCreature .power (.up (.lit 2))).numberSlots
      = [(.lit 2, .clamped)] := by rfl

/-! ## Signed slots: the negative stands -/

/-- "Target creature's base power and toughness become 0/2": an effect that sets a power and
toughness is the exception [CR#107.1b] names. -/
theorem okDefinesPtSigned :
    (StaticSpec.ptDefinition (target creature) .bothEach (.lit 0)).numberSlots
      = [(.lit 0, .signed)] := by rfl

/-- "Whenever this creature's power becomes 3": the header watches a game value, which may be
less than zero [CR#107.1b]. -/
theorem okStatBecomesSigned :
    (GameEvent.statBecomes thisCreature .power (.lit 3)).numberSlots
      = [(.lit 3, .signed)] := by rfl

/-- "This creature's power becomes -1": a set of a power, the exception [CR#107.1b] names,
so the printed negative stands where the `up` half of the same `Delta` would clamp. -/
theorem okModifySetSigned :
    (StaticSpec.modification thisCreature .power (.set (.lit (-1)))).numberSlots
      = [(.lit (-1), .signed)] := by rfl

/-- "Your life total becomes twice your life total": doubling a life total is written as a
set, and a set of a life total is the other half of the [CR#107.1b] exception. -/
theorem okLifeTotalBecomesSigned :
    (Instruction.changeLife (.set (.arith .times (lifeTotalOf .you) (.lit 2))) (agent :=
        .you)).numberSlots
      = [(.arith .times (lifeTotalOf .you) (.lit 2), .signed)] := by rfl

/-- "Exchange life totals" and the other numerical exchanges [CR#701.12g] set each side to the
other's value, so both stand below zero [CR#107.1b]. -/
theorem okExchangeValuesSigned :
    (Instruction.exchange
      (.values (lifeTotalOf .you) (lifeTotalOf (target .opponent)))).numberSlots
      = [(lifeTotalOf .you, .signed), (lifeTotalOf (target .opponent), .signed)] := by rfl

/-! ## Doubling damage -/

/-- "If a source would deal damage to a permanent or player, it deals double that damage
instead": [CR#107.1b] exempts doubling from clamping, but the factor is a `ScaleFactor` and
not an `Amount`, so the rule declares no number slot. The exemption is visible instead where
doubling is written as a set, above. -/
theorem okMultipliedDamageDeclaresNoSlot :
    (StaticSpec.damageRule .any .unattributed .everywhere
      (.scale (.multiplied .doubled)) .repeatedly).numberSlots = [] := by rfl

/-- "...it deals that much damage plus 1 instead": the shift is a magnitude with its direction
for a sign, and damage is never negative [CR#107.1b]. -/
theorem okShiftedDamageClamped :
    (StaticSpec.damageRule .any .unattributed .everywhere
      (.scale (.shifted .up (.lit 1))) .repeatedly).numberSlots = [(.lit 1, .clamped)] := by rfl

/-! ## Printed slots and effect-written expressions -/

theorem badPrintedPowerLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], toughness := some (.lit 1),
        power := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedPowerLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], toughness := some (.lit 1),
        power := some (.lit 1) } }) = [] := by decide

theorem badPrintedToughnessLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], power := some (.lit 1),
        toughness := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedToughnessLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], power := some (.lit 1),
        toughness := some (.lit 1) } }) = [] := by decide

theorem badPrintedLoyaltyLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.planeswalker],
        loyalty := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedLoyaltyLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.planeswalker],
        loyalty := some (.lit 1) } }) = [] := by decide

theorem badPrintedDefenseLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.battle],
        defense := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedDefenseLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.battle],
        defense := some (.lit 1) } }) = [] := by decide

/-- A valid expression still cannot occupy a printed literal slot. -/
theorem badPrintedArithmetic :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature],
        power := some (.arith .plus (.lit 1) (.lit 1)), toughness := some (.lit 1) } })
      = [.cardBox] := by decide

theorem okPrintedArithmeticLiteralTwin :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature],
        power := some (.lit 2), toughness := some (.lit 1) } }) = [] := by decide

/-- Even a letter bound by the mana cost remains an expression in a printed slot. -/
theorem badPrintedBoundLetter :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", cost := some [.variable], types := [.creature],
        power := some (.letter .x), toughness := some (.lit 1) } }) = [.cardBox] := by decide

theorem okPrintedBoundLetterLiteralTwin :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", cost := some [.variable], types := [.creature],
        power := some (.lit 0), toughness := some (.lit 1) } }) = [] := by decide

theorem badSharedLinePowerLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], toughness := some (.lit 1), power := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLinePowerLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], toughness := some (.lit 1), power := some (.lit 1) } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineToughnessLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], power := some (.lit 1), toughness := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineToughnessLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], power := some (.lit 1), toughness := some (.lit 1) } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineLoyaltyLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.planeswalker], loyalty := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineLoyaltyLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.planeswalker], loyalty := some (.lit 1) } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineDefenseLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.battle], defense := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineDefenseLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.battle], defense := some (.lit 1) } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badLevelBandPowerLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      [{ power := some .thatMuch, toughness := some (.lit 1), range := .atLeast 1 }])
      = [.cardBox] := by decide

theorem okLevelBandPowerLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      [{ power := some (.lit (-1)), toughness := some (.lit 1), range := .atLeast 1 }])
      = [] := by decide

theorem badLevelBandToughnessLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      [{ toughness := some .thatMuch, power := some (.lit 1), range := .atLeast 1 }])
      = [.cardBox] := by decide

theorem okLevelBandToughnessLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      [{ toughness := some (.lit (-1)), power := some (.lit 1), range := .atLeast 1 }])
      = [] := by decide

theorem badPrototypePowerLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      { power := some .thatMuch, toughness := some (.lit 1), cost := some [generic 1] })
      = [.cardBox] := by decide

theorem okPrototypePowerLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      { power := some (.lit (-1)), toughness := some (.lit 1), cost := some [generic 1] })
      = [] := by decide

theorem badPrototypeToughnessLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      { toughness := some .thatMuch, power := some (.lit 1), cost := some [generic 1] })
      = [.cardBox] := by decide

theorem okPrototypeToughnessLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some (.lit 1), toughness := some (.lit 1) } }
      { toughness := some (.lit (-1)), power := some (.lit 1), cost := some [generic 1] })
      = [] := by decide

theorem badDynamicPowerContext :
    CharacteristicBundle.check []
      { characteristics := { power := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicPowerContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { power := some .thatMuch } } = [] := by decide

theorem badDynamicToughnessContext :
    CharacteristicBundle.check []
      { characteristics := { toughness := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicToughnessContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { toughness := some .thatMuch } } = [] := by decide

theorem badDynamicLoyaltyContext :
    CharacteristicBundle.check []
      { characteristics := { loyalty := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicLoyaltyContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { loyalty := some .thatMuch } } = [] := by decide

theorem badDynamicDefenseContext :
    CharacteristicBundle.check []
      { characteristics := { defense := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicDefenseContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { defense := some .thatMuch } } = [] := by decide

theorem okDynamicArithmetic :
    CharacteristicBundle.check []
      { characteristics := { power := some (.arith .plus (.lit 1) (.lit 1)) } } = [] := by decide

end Semantics.Proofs.Numbers
