import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.GiftVariants

Gift is one definition holding both its abilities [CR#702.174a,702.174b]: the additional cost
"you may choose an opponent", an enters trigger guarded by the permanent class and a spell
ability guarded by the instant-or-sorcery class, each handing the effect to the player chosen as
the gift cost was paid [CR#607.2d,607.2i]. Bare gift has no spelling; each spelled variant is gift
with its effect (`giftACard`, `giftATappedFish`, … in `Semantics.Definitions`). These pins hold the
two variants printed on both card classes, "Gift a card" [CR#702.174e] and "Gift a tapped Fish"
[CR#702.174f], against the registry's own gift row, on a permanent card and on an instant, with
twins that must stay refused.
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

/-- "if its gift cost was paid, the chosen player <effect>" -/
def handed (effect : Action) : Action :=
  doIf (costWasPaid giftCost none .this) (instruct (chosenAsPaid (.named "gift") .this) effect)

/-- "As an additional cost to cast this spell, you may choose an opponent." [CR#702.174a] -/
def costPart : Ability := .static (.addedCost (.perform (choose anOpponent)) true .once)

/-- Gift's term with its effect, as the declaration builds it [CR#702.174a,702.174b]. -/
def gift (effect : Action) : Ability :=
  .keyword (.named "gift") []
    [ costPart,
      triggeredIf (does thisPermanent enter) onPermanent (handed effect),
      .spell none (doIf onInstantOrSorcery (handed effect)) ]

/-- "Gift a card": the chosen player draws a card [CR#702.174e]. -/
def giftACard : Ability := gift (draw (.lit 1))

/-- "Gift a tapped Fish": the chosen player creates a tapped 1/1 blue Fish creature token
[CR#702.174f]. -/
def giftATappedFish : Ability :=
  gift (createTappedToken (creatureToken 1 1 [.blue] [creatureType "Fish"]))

/-- Both variants fit the registry's gift row and check with no class context. -/
theorem okVariantsFitGiftRow :
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

/-- Twin: the spell half guarded by the permanent class does not fit gift's row, which puts a
spell ability under the instant-or-sorcery guard [CR#113.3a,702.174b], and is refused on a
permanent card. -/
def giftACardSpellHalfOnPermanent : Ability :=
  .keyword (.named "gift") []
    [ costPart,
      triggeredIf (does thisPermanent enter) onPermanent
        (handed (draw (.lit 1))),
      .spell none (doIf onPermanent (handed (draw (.lit 1)))) ]

theorem badSpellHalfUnderPermanentGuard :
    (Ability.check (DRS.ofList []) giftACardSpellHalfOnPermanent,
      cardTextOk [.creature] [giftACardSpellHalfOnPermanent])
      = ([.keywordBodyFits (.named "gift")], false) := by
  decide

/-- Twin: "the chosen player" read in the effect's own scope does not reach the opponent chosen
as the gift cost was paid [CR#607.2d]. -/
def giftACardUnlinked : Ability :=
  .keyword (.named "gift") []
    [ costPart,
      .spell none
        (doIf onInstantOrSorcery
          (doIf (costWasPaid giftCost none .this) (instruct (the isChosenPlayer) (draw (.lit 1))))) ]

theorem badUnlinkedChosenPlayer :
    Ability.check (DRS.ofList []) giftACardUnlinked = [.choiceRef .theChoice .player 0] := by
  decide

end Semantics.Proofs.GiftVariants
