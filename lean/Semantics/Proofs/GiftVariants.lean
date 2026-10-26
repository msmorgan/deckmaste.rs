import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.GiftVariants

Gift is one definition holding both its abilities [CR#702.174a,702.174b]: the additional cost
"you may choose an opponent", an enters trigger and a spell ability, each handing the effect to
"the chosen player", the player chosen as the gift cost was paid [CR#607.2d,607.2i]. The trigger
and the spell ability are a `byCardClass` pair, the card's Card Class selecting between them
[CR#110.4,110.4a,702.174b]; neither carries a card-class guard. Bare gift has no spelling; each
spelled variant is gift with its effect (`giftACard`, `giftATappedFish`, … in
`Semantics.Definitions`). These pins hold the two variants printed on both card classes, "Gift a
card" [CR#702.174e] and "Gift a tapped Fish" [CR#702.174f], on a permanent card and on an
instant.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.GiftVariants

/-- Both variants check with no class context. -/
theorem okVariantsCheck :
    (Ability.check (DRS.ofList []) giftACard, Ability.check (DRS.ofList []) giftATappedFish) = ([], []) := by
  decide

/-- A single-faced card with power and toughness on a creature. -/
def card (types : List CardType) (text : List Ability) : Card :=
  .singleFaced
    { characteristics :=
        { name := some "", cost := some [generic 2], types, text,
          power := if types.elem .creature then stat 1 else none,
          toughness := if types.elem .creature then stat 1 else none } }

/-- "Gift a card" on a permanent card (Scrapshooter) and on an instant (Long River's Pull). -/
theorem okGiftACardOnBothClasses :
    (Card.check (card [.creature] [giftACard]), Card.check (card [.instant] [giftACard]))
      = ([], []) := by
  decide

/-- "Gift a tapped Fish" on a permanent card (Starforged Sword) and on an instant (Into the
Flood Maw). -/
theorem okGiftATappedFishOnBothClasses :
    (Card.check (card [.artifact] [giftATappedFish]),
        Card.check (card [.instant] [giftATappedFish]))
      = ([], []) := by
  decide

/-- The intervening "if" clause of a keyword body's triggered ability, if it has one, a
`byCardClass` half included. -/
def triggerCondition? : Ability → Option (Option Condition)
  | .keyword _ _ body => body.findSome? fun
    | .triggered _ _ _ _ _ _ condition _
    | .byCardClass (.triggered _ _ _ _ _ _ condition _) _ => some condition
    | _ => none
  | _ => none

/-- The declaration's permanent half triggers only if its gift cost was paid: that alone is its
intervening "if" clause, so with no gift promised it does not trigger at all
[CR#702.174b,603.4]. -/
theorem declaredGiftTriggerHasInterveningIf :
    triggerCondition? (gift (draw 1))
      = some (some (keywordCostPaid (.named "gift") this_)) := by
  rfl

end Semantics.Proofs.GiftVariants
