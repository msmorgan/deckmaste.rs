import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.GiftVariants

Gift is one definition holding both its abilities [CR#702.174a,702.174b]: the additional cost
"you may choose an opponent", an enters trigger guarded by the permanent class and a spell
ability guarded by the instant-or-sorcery class, each handing the effect to "the chosen player",
the player chosen as the gift cost was paid [CR#607.2d,607.2i]. Bare gift has no spelling; each
spelled variant is gift with its effect (`giftACard`, `giftATappedFish`, … in
`Semantics.Definitions`). These pins hold the two variants printed on both card classes, "Gift a
card" [CR#702.174e] and "Gift a tapped Fish" [CR#702.174f], on a permanent card and on an
instant.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.GiftVariants

/-- "its gift cost" -/
def giftCost : LinkSource := .byKeyword (.named "gift")

/-- "if this has a permanent type": read off the types, so it holds of a token with gift as
well as a card [CR#110.4]. -/
def onPermanent : Condition := .matches .this isPermanentType

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this isInstantOrSorcery

/-- "if its gift cost was paid, the chosen player <effect>" [CR#702.174b] -/
def handed (effect : Action) : Action :=
  doIf (costWasPaid giftCost none .this) (instruct (the isChosenPlayer) effect)

/-- "As an additional cost to cast this spell, you may choose an opponent." [CR#702.174a] -/
def costPart : Ability := .static (.addedCost (.perform (choose anOpponent)) true .once)

/-- Gift's term with its effect, as the declaration builds it [CR#702.174a,702.174b]. On a
permanent, "if its gift cost was paid" is the trigger's intervening "if" clause, so the ability
does not trigger when no gift was promised [CR#603.4]. -/
def gift (effect : Action) : Ability :=
  .keyword (.named "gift") []
    [ costPart,
      when (does thisPermanent enter)
        (if_ := some (.and [onPermanent, costWasPaid giftCost none .this]))
        (instruct (the isChosenPlayer) effect),
      .spell none (doIf onInstantOrSorcery (handed effect)) ]

/-- "Gift a card": the chosen player draws a card [CR#702.174e]. -/
def giftACard : Ability := gift (draw 1)

/-- "Gift a tapped Fish": the chosen player creates a tapped 1/1 blue Fish creature token
[CR#702.174f]. -/
def giftATappedFish : Ability :=
  gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))

/-- Both variants check with no class context. -/
theorem okVariantsCheck :
    (Ability.check (DRS.ofList []) giftACard, Ability.check (DRS.ofList []) giftATappedFish) = ([], []) := by
  decide

/-- "Gift a card" on a permanent card (Scrapshooter) and on an instant (Long River's Pull). -/
theorem okGiftACardOnBothClasses :
    (cardTextOk [.creature] [giftACard], cardTextOk [.instant] [giftACard]) = (true, true) := by
  decide

/-- "Gift a tapped Fish" on a permanent card (Starforged Sword) and on an instant (Into the
Flood Maw). -/
theorem okGiftATappedFishOnBothClasses :
    (cardTextOk [.artifact] [giftATappedFish], cardTextOk [.instant] [giftATappedFish])
      = (true, true) := by
  decide

/-- The intervening "if" clause of a keyword body's triggered ability, if it has one. -/
def triggerCondition? : Ability → Option (Option Condition)
  | .keyword _ _ body => body.findSome? fun
    | .triggered _ _ _ _ _ _ condition _ => some condition
    | _ => none
  | _ => none

/-- The declaration's permanent half triggers only if this has a permanent type and its gift
cost was paid: both are its intervening "if" clause, so with no gift promised it does not
trigger at all [CR#702.174b,603.4]. -/
theorem declaredGiftTriggerHasInterveningIf :
    triggerCondition? (Semantics.Definitions.gift (draw 1))
      = some (some (and [is this_ isPermanentType, keywordCostPaid (.named "gift") this_])) := by
  rfl

end Semantics.Proofs.GiftVariants
