import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.LinkedChoiceReadback

Gift's first ability is a cost: "As an additional cost to cast this spell, you may choose an
opponent." [CR#702.174a]. Its second ability hands the effect to the chosen player
[CR#702.174b,702.174e], and the two are linked [CR#607.2d,607.2i]. The second reads the choice
back as "the chosen player" (`the isChosenPlayer`), the same words a choice in its own ability
would be read by: the cost publishes the choice to the abilities after it (`Ability.textContext`),
across gift's own body as across a card's text. "The chosen player" requires exactly one player
choice in scope, which holds in gift's body because its cost is the only one there.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.LinkedChoiceReadback

/-- "its gift cost" -/
def gift : LinkSource := .byKeyword (.named "gift")

/-- "As an additional cost to cast this spell, you may choose an opponent." [CR#702.174a] -/
def costPart : Ability := .static (.addedCost (.perform (choose anOpponent)) true .once)

/-- "the chosen player draws a card", the player read back from gift's cost. -/
def chosenDraws : Action := instruct (the isChosenPlayer) (draw 1)

/-- Gift's permanent half: "When this permanent enters, if its gift cost was paid, the chosen
player draws a card." The cost is its intervening "if" clause [CR#702.174b,702.174e,603.4]. -/
def permanentHalf : Ability :=
  when (does thisPermanent enter)
    (if_ := some (costWasPaid gift none .this))
    chosenDraws

/-- Gift's spell half: "If this spell's gift cost was paid, the chosen player draws a card."
[CR#702.174b,113.3a] -/
def spellHalf : Ability :=
  .spell none (doIf (costWasPaid gift none .this) chosenDraws)

/-- Required pin: after gift's cost, "the chosen player" reads the opponent chosen as it was paid
[CR#702.174a,607.2d]. -/
theorem okGiftReadsChosenPlayer :
    NounPhrase.checkAt (some .player) (Ability.textContext (DRS.ofList []) costPart)
      (the isChosenPlayer) = [] := by
  decide

/-- Required pin: with no player choice in scope there is none to read [CR#607.2d]. -/
theorem badUnlinkedChosenPlayer :
    Ability.check (DRS.ofList []) (.spell none (instruct (the isChosenPlayer) (draw 1)))
      = [.choiceRef .theChoice .player 0] := by
  decide

/-- Required pin: two costs each choosing a player leave "the chosen player" ambiguous. -/
theorem badTwoChosenPlayers :
    Ability.checkText (DRS.ofList []) [costPart, costPart, spellHalf] =
      [.choiceRef .theChoice .player 2] := by
  decide

/-- Kicker's cost chooses no player, so there is none to read back [CR#702.33a,607.2d]. -/
theorem badKickerChoosesNoPlayer :
    Ability.checkText (DRS.ofList [])
      [ .static (.addedCost (.mana [generic 1]) true .once),
        .spell none (instruct (the isChosenPlayer) (draw 1)) ] =
      [.choiceRef .theChoice .player 0] := by
  decide

/-- The chosen player is a player: an object context refuses it. -/
theorem badChosenPlayerAsObject :
    NounPhrase.checkAt (some .object) (Ability.textContext (DRS.ofList []) costPart)
      (the isChosenPlayer) = [.kindMismatch .object .player] := by
  decide

/-- Both halves read the chosen player after gift's cost. -/
theorem okBothHalvesReadChosenPlayer :
    Ability.checkText (DRS.ofList []) [costPart, permanentHalf, spellHalf] = [] := by
  decide

/-- Gift's definition: its cost, then its two halves as a `byCardClass` pair. -/
def giftBody : Ability :=
  .keyword (.named "gift") [] [costPart, .byCardClass permanentHalf spellHalf]

/-- Gift's definition holding its cost and both halves is carried on a permanent card. -/
theorem okPermanentClassReadsChosenPlayer :
    Card.check
      (.singleFaced
        { characteristics :=
            { name := some "", cost := some [generic 2], types := [.creature],
              text := [giftBody], power := stat 1, toughness := stat 1 } }) = [] := by
  decide

/-- Gift's definition holding its cost and both halves is carried on an instant card. -/
theorem okSpellClassReadsChosenPlayer :
    Card.check
      (.singleFaced
        { characteristics :=
            { name := some "", cost := some [generic 2], types := [.instant],
              text := [giftBody] } }) = [] := by
  decide

/-- Required pin: inside gift's own body, the halves read its cost's choice
[CR#702.174a,607.2d]; with a second cost choosing a player, neither knows which to read. -/
theorem keywordBodyThreadsItsCostChoice :
    Ability.check (DRS.ofList []) (.keyword (.named "gift") [] [costPart, spellHalf]) = [] ∧
      Ability.check (DRS.ofList []) (.keyword (.named "gift") [] [costPart, costPart, spellHalf]) =
        [.choiceRef .theChoice .player 2] ∧
      Ability.check (DRS.ofList []) (.keyword (.named "gift") [] [spellHalf]) =
        [.choiceRef .theChoice .player 0] := by
  decide

end Semantics.Proofs.LinkedChoiceReadback
