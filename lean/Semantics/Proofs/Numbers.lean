import Semantics.Check.NumberSlots
import Semantics.Definitions
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

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Numbers

/-! ## A printed negative face -/

/-- Spinal Parasite {5} — Artifact Creature — Insect, printed −1/−1. A game value such as a
creature's power may be less than zero [CR#107.1b], so the face is admitted. -/
theorem okPrintedNegativePowerAndToughness :
    Card.check (.singleFaced
      { characteristics :=
        { name := "Spinal Parasite", cost := some [generic 5],
          types := [.artifact, .creature], subtypes := [insect],
          power := some (.lit (-1)), toughness := some (.lit (-1)) } }) = [] := by
  decide

/-- Char-Rumbler {2}{R}{R} — Creature — Elemental, printed −1/3: a negative power beside a
positive toughness [CR#107.1b]. -/
theorem okPrintedNegativePower :
    Card.check (.singleFaced
      { characteristics :=
        { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red],
          types := [.creature], subtypes := [elemental],
          power := some (.lit (-1)), toughness := some 3 } }) = [] := by
  decide

/-! ## Clamped slots: an effect's result reads zero -/

/-- "You gain X life": you can't gain negative life, so a calculation below zero reads as
zero [CR#107.1b]. -/
theorem okGainsLifeClamped :
    (gainLife (.letter .x)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "You lose X life" [CR#107.1b]. -/
theorem okLosesLifeClamped :
    (loseLife (.letter .x)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "This creature enters with X +1/+1 counters on it": the riders a replacement adds to a move
are its own slots, a count of counters [CR#107.1b]. -/
theorem okEnteringRidersClamped :
    (withRiders [.withCounters (.letter .x) (.printed p1p1Counter) .fresh]).numberSlots =
      [(.letter .x, .clamped)] := by rfl

/-- "Draw X cards": a count of cards [CR#107.1b]. -/
theorem okDrawClamped :
    (draw (.letter .x)).numberSlots = [(.letter .x, .clamped)] := by rfl

/-- "This spell deals X damage to target creature": you can't deal negative damage
[CR#107.1b]. -/
theorem okDealDamageClamped :
    (Action.instruct .this (.dealDamage (.letter .x) (target isCreature))).numberSlots
      = [(.letter .x, .clamped)] := by rfl

/-- "This creature gets +2/+0": the sign is the constructor, and the magnitude beneath it is a
number the game does not take below zero [CR#107.1b]. -/
theorem okModifyUpClamped :
    (Templates.modification thisCreature .power (.up 2)).numberSlots
      = [(.lit 2, .clamped)] := by rfl

/-! ## Signed slots: the negative stands -/

/-- "X is the number of creatures you control": the letter X, defined by the text [CR#107.3c],
is a calculation and not an effect's result, so the definition itself stands below zero
[CR#107.1b]; the slot that consumes X decides whether to clamp it. -/
theorem okDefineSigned :
    (define .x (.countOf (.described .all isCreature))).numberSlots
      = [(.countOf (.described .all isCreature), .signed)] := by rfl

/-- The same letter defined by a static ability [CR#107.1b,107.3c]. -/
theorem okDefinesLetterSigned :
    (StaticSpec.letterDefinition .x (.countOf (.described .all isCreature))).numberSlots
      = [(.countOf (.described .all isCreature), .signed)] := by rfl

/-- Guardian Scalelord: "return target nonland permanent card with mana value X or less from
your graveyard to the battlefield, where X is this creature's power." With power -1, X is -1 and
the comparison uses it [CR#107.1b], so no card qualifies, mana value 0 included. The pin is on
the definition: it stands below zero, a negative X included. -/
theorem okNegativeLetterInComparison :
    (define .x (powerOf thisCreature)).numberSlots = [(powerOf thisCreature, .signed)] ∧
      (define .x (.lit (-1))).numberSlots = [(.lit (-1), .signed)] :=
  ⟨rfl, rfl⟩

/-- "Target creature's base power and toughness become 0/2": an effect that sets a power and
toughness is the exception [CR#107.1b] names. -/
theorem okDefinesPtSigned :
    (StaticSpec.characteristicChange (target isCreature)
      [.stat .sets .power 0, .stat .sets .toughness 2]).numberSlots
      = [(.lit 0, .signed), (.lit 2, .signed)] := by rfl

/-- "This creature's power and toughness are each equal to …": a Characteristic-Defining
Ability sets them, so a negative it reads stands [CR#107.1b,604.3a]. -/
theorem okCharacteristicDefiningPtSigned :
    (ptDefinitions thisCreature 0 2).numberSlots
      = [(.lit 0, .signed), (.lit 2, .signed)] := by rfl

/-- "Whenever this creature's power becomes 3": the header watches a game state [CR#603.8], a
comparison of a game value that may be less than zero [CR#107.1b]; it is no effect's result, so
it has no clamped number slot. -/
theorem okStateHeaderHasNoNumberSlot :
    (stateHolds (compareAmt (powerOf thisCreature) .eq 3)).numberSlots = [] := by rfl

/-- "This creature's power becomes -1": a set of a power, the exception [CR#107.1b] names,
so the printed negative stands where the `up` half of the same `Delta` would clamp. -/
theorem okModifySetSigned :
    (Templates.modification thisCreature .power (.set (.lit (-1)))).numberSlots
      = [(.lit (-1), .signed)] := by rfl

/-- "Your life total becomes twice your life total": doubling a life total is written as a
set, and a set of a life total is the other half of the [CR#107.1b] exception. -/
theorem okLifeTotalBecomesSigned :
    (Action.changeLife (.set (.arith .times (lifeTotalOf .you) 2))).numberSlots
      = [(.arith .times (lifeTotalOf .you) 2, .signed)] := by rfl

/-- "Exchange life totals" and the other numerical exchanges [CR#701.12g] set each side to the
other's value, so both stand below zero [CR#107.1b]. -/
theorem okExchangeValuesSigned :
    (Action.exchange
      (.values (lifeTotalOf .you) (lifeTotalOf (target (.isOpponentOf .you))))).numberSlots
      = [(lifeTotalOf .you, .signed), (lifeTotalOf (target (.isOpponentOf .you)), .signed)] := by rfl

/-! ## Doubling damage -/

/-- "If a source would deal damage to a permanent or player, it deals double that damage
instead": [CR#107.1b] exempts doubling from clamping, but the factor is a `ScaleFactor` and
not an `Amount`, so the rule declares no number slot. The exemption is visible instead where
doubling is written as a set, above. -/
theorem okMultipliedDamageDeclaresNoSlot :
    (StaticSpec.damageRule .unattributed .everywhere
      (.scale (.multiplied .doubled)) .repeatedly).numberSlots = [] := by rfl

/-- "...it deals that much damage plus 1 instead": the shift is a magnitude with its direction
for a sign, and damage is never negative [CR#107.1b]. -/
theorem okShiftedDamageClamped :
    (StaticSpec.damageRule .unattributed .everywhere
      (.scale (.shifted (.up 1))) .repeatedly).numberSlots = [(.lit 1, .clamped)] := by rfl

/-! ## Printed slots and effect-written expressions -/

theorem badPrintedPowerLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], toughness := some 1,
        power := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedPowerLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], toughness := some 1,
        power := some 1 } }) = [] := by decide

theorem badPrintedToughnessLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], power := some 1,
        toughness := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedToughnessLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature], power := some 1,
        toughness := some 1 } }) = [] := by decide

theorem badPrintedLoyaltyLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.planeswalker],
        loyalty := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedLoyaltyLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.planeswalker],
        loyalty := some 1 } }) = [] := by decide

theorem badPrintedDefenseLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.battle],
        defense := some .thatMuch } }) = [.cardBox] := by decide

theorem okPrintedDefenseLiteral :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.battle],
        defense := some 1 } }) = [] := by decide

/-- A valid expression still cannot occupy a printed literal slot. -/
theorem badPrintedArithmetic :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature],
        power := some (.arith .plus 1 1), toughness := some 1 } })
      = [.cardBox] := by decide

theorem okPrintedArithmeticLiteralTwin :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", types := [.creature],
        power := some 2, toughness := some 1 } }) = [] := by decide

/-- Even a letter bound by the mana cost remains an expression in a printed slot. -/
theorem badPrintedBoundLetter :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", cost := some [.variable], types := [.creature],
        power := some (.letter .x), toughness := some 1 } }) = [.cardBox] := by decide

theorem okPrintedBoundLetterLiteralTwin :
    Card.check (.singleFaced { characteristics :=
      { name := "Stat witness", cost := some [.variable], types := [.creature],
        power := some 0, toughness := some 1 } }) = [] := by decide

theorem badSharedLinePowerLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], toughness := some 1, power := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLinePowerLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], toughness := some 1, power := some 1 } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineToughnessLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], power := some 1, toughness := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineToughnessLiteral :
    Card.check (.sharedLineSplit
      { characteristics :=
        { types := [.creature], power := some 1, toughness := some 1 } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineLoyaltyLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.planeswalker], loyalty := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineLoyaltyLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.planeswalker], loyalty := some 1 } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badSharedLineDefenseLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.battle], defense := some .thatMuch } }
      { name := "Left" } { name := "Right" }) = [.cardBox, .cardBox] := by decide

theorem okSharedLineDefenseLiteral :
    Card.check (.sharedLineSplit
      { characteristics := { types := [.battle], defense := some 1 } }
      { name := "Left" } { name := "Right" }) = [] := by decide

theorem badLevelBandPowerLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      [{ power := some .thatMuch, toughness := some 1, range := .plus 1 }])
      = [.cardBox] := by decide

theorem okLevelBandPowerLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      [{ power := some (.lit (-1)), toughness := some 1, range := .plus 1 }])
      = [] := by decide

theorem badLevelBandToughnessLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      [{ toughness := some .thatMuch, power := some 1, range := .plus 1 }])
      = [.cardBox] := by decide

theorem okLevelBandToughnessLiteral :
    Card.check (.leveler
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      [{ toughness := some (.lit (-1)), power := some 1, range := .plus 1 }])
      = [] := by decide

theorem badPrototypePowerLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      { power := some .thatMuch, toughness := some 1, cost := some [generic 1] })
      = [.cardBox] := by decide

theorem okPrototypePowerLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      { power := some (.lit (-1)), toughness := some 1, cost := some [generic 1] })
      = [] := by decide

theorem badPrototypeToughnessLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      { toughness := some .thatMuch, power := some 1, cost := some [generic 1] })
      = [.cardBox] := by decide

theorem okPrototypeToughnessLiteral :
    Card.check (.prototype
      { characteristics :=
        { name := "Stat witness", types := [.creature],
          power := some 1, toughness := some 1 } }
      { toughness := some (.lit (-1)), power := some 1, cost := some [generic 1] })
      = [] := by decide

theorem badDynamicPowerContext :
    CharacteristicBundle.check (DRS.ofList [])
      { characteristics := { power := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicPowerContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { power := some .thatMuch } } = [] := by decide

theorem badDynamicToughnessContext :
    CharacteristicBundle.check (DRS.ofList [])
      { characteristics := { toughness := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicToughnessContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { toughness := some .thatMuch } } = [] := by decide

theorem badDynamicLoyaltyContext :
    CharacteristicBundle.check (DRS.ofList [])
      { characteristics := { loyalty := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicLoyaltyContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { loyalty := some .thatMuch } } = [] := by decide

theorem badDynamicDefenseContext :
    CharacteristicBundle.check (DRS.ofList [])
      { characteristics := { defense := some .thatMuch } } = [.quantOutcomeInScope 0] := by decide

theorem okDynamicDefenseContext :
    CharacteristicBundle.check [outcomeB .damageDealt]
      { characteristics := { defense := some .thatMuch } } = [] := by decide

theorem okDynamicArithmetic :
    CharacteristicBundle.check (DRS.ofList [])
      { characteristics := { power := some (.arith .plus 1 1) } } = [] := by decide

end Semantics.Proofs.Numbers
