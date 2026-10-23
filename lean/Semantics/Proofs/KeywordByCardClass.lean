import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.KeywordByCardClass

A keyword that means one ability on a permanent and another on an instant or sorcery spell
writes its two halves with `byCardClass`, which the card's Card Class selects between
[CR#110.4,110.4a,702.55a,702.131a,702.131b,702.174b] (ruling 2026-10-22/23): gift's entering
trigger or spell ability after its shared cost, haunt's dies trigger or trigger on the spell's
being put into a graveyard during its resolution, and ascend's static ability or spell ability.
No half carries a card-class guard. The checker checks both halves on every card, each in the
context the abilities before it leave, and no ability is checked against the card's class.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordByCardClass

/-- The halves of a keyword declaration's body, a `byCardClass` pair as both of its halves. -/
def halves : Ability → List Ability
  | .keyword _ _ body => body.flatMap fun
    | .byCardClass onPermanent onInstantOrSorcery => [onPermanent, onInstantOrSorcery]
    | ab => [ab]
  | _ => []

/-- A single-faced card with power and toughness on a creature. -/
def card (types : List CardType) (text : List Ability) : Card :=
  .singleFaced
    { characteristics :=
        { name := some "", cost := some [generic 2], types, text,
          power := if types.elem .creature then stat 1 else none,
          toughness := if types.elem .creature then stat 1 else none } }

/-- Gift's, haunt's and ascend's halves are well-formed with no class context. Gift's halves are
checked as one text, since each reads the player its cost chose [CR#607.2d]; haunt's and
ascend's each on its own. -/
theorem okEachHalfChecksClassBlind :
    Ability.checkText (DRS.ofList []) (halves (gift (draw 1))) = [] ∧
      Ability.checkAll (DRS.ofList []) (halves haunt) = [] ∧
      Ability.checkAll (DRS.ofList []) (halves ascend) = [] := by
  decide

/-- Gift, haunt and ascend, each holding both halves, on a permanent card. -/
theorem okBothHalvesOnPermanentCard :
    Card.check (card [.creature] [gift (draw 1)]) = [] ∧
      Card.check (card [.creature] [haunt]) = [] ∧
      Card.check (card [.creature] [ascend]) = [] := by
  decide

/-- Gift, haunt and ascend, each holding both halves, on an instant card. -/
theorem okBothHalvesOnInstantCard :
    Card.check (card [.instant] [gift (draw 1)]) = [] ∧
      Card.check (card [.instant] [haunt]) = [] ∧
      Card.check (card [.instant] [ascend]) = [] := by
  decide

/-- "the chosen player draws a card" -/
def chosenDraws : Ability := spell (instruct (the isChosenPlayer) (draw 1))

/-- `byCardClass` checks both halves: a half that is wrong on its own, reading a chosen player
no ability chose [CR#607.2d], is refused whichever side it is on, though the other half is
fine. -/
theorem badEitherHalfRefused :
    Ability.check (DRS.ofList []) (byCardClass chosenDraws (spell (draw 1))) =
        [.choiceRef .theChoice .player 0] ∧
      Ability.check (DRS.ofList []) (byCardClass (spell (draw 1)) chosenDraws) =
        [.choiceRef .theChoice .player 0] ∧
      Ability.check (DRS.ofList []) (byCardClass (spell (draw 1)) (spell (draw 1))) = [] := by
  decide

/-- Each half is checked in the context the abilities before it leave: after gift's cost, both
read the player it chose [CR#607.2d,702.174a]. -/
theorem okBothHalvesReadTheCostChoice :
    Ability.checkText (DRS.ofList [])
      [ .static (.addedCost (.perform (choose anOpponent)) true .once),
        byCardClass chosenDraws chosenDraws ] = [] := by
  decide

/-- "ten or more permanents and no city's blessing yet" -/
def ascendCondition : Condition :=
  and
    [ .compareAmt (countOf (and [isPermanent, youControl])) .atLeast 10,
      not (is .you (hasDesignation citysBlessing)) ]

/-- "you get the city's blessing for the rest of the game" -/
def getCitysBlessing : Action := gainDesignation .you citysBlessing restOfGame

/-- Ascend's tree: on a permanent, a static ability that gives the city's blessing while you
control ten or more permanents [CR#702.131b]; on an instant or sorcery, a spell ability that gives
it if you do as the spell resolves [CR#702.131a]. Neither half is guarded by its class. -/
theorem ascendTree :
    ascend =
      .keyword (.named "ascend") []
        [ .byCardClass
            (.static (.whileHolds ascendCondition getCitysBlessing))
            (.spell none (doIf ascendCondition getCitysBlessing none)) ] := by
  rfl

end Semantics.Proofs.KeywordByCardClass
