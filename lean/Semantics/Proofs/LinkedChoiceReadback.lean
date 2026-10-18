import Semantics.Macros.Declared
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.LinkedChoiceReadback

Gift's first ability is a cost: "As an additional cost to cast this spell, you may choose an
opponent." [CR#702.174a]. Its second ability hands the effect to the chosen player
[CR#702.174b,702.174e], and the two are linked [CR#607.2d,607.2i]. The second reads the choice
back through an explicit paid-cost facet keyed by the cost's name, as kicker's "was kicked" readback is
[CR#702.33e] (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords): `LinkedFacet.chosenPlayer`, read as a noun phrase by `NounPhrase.paid`.

The facet resolves where the named cost chooses a player when it is paid (the keyword row's
`costChoosesPlayer`); a cost that chooses no player leaves the linked ability nothing to read,
which is a reference refusal [CR#607.2d]. The checker requires that explicit facet for the cost link; a bare "the chosen player"
requests a local choice instead.

The definition-fit pins hold a row written here, shaped as gift's, from before gift was bodied;
`Proofs/GiftVariants.lean` checks gift's variants against the registry's own row.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.LinkedChoiceReadback

/-- "its gift cost" -/
def gift : LinkSource := .byKeyword (.named "gift")

/-- "its kicker cost" -/
def kicker : LinkSource := .byKeyword (.named "kicker")

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this permanentCard

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this instantOrSorcery

/-- "the chosen player draws a card", the player read back from gift's cost. -/
def chosenDraws : Action := instruct (chosenAsPaid (.named "gift") .this) (draw (.lit 1))

/-- Gift's permanent half: "When this permanent enters, if this is a permanent card, if its gift
cost was paid, the chosen player draws a card." [CR#702.174b,702.174e] -/
def permanentHalf : Ability :=
  triggeredIf (Declared.enters thisPermanent none) onPermanentCard
    (doIf (costWasPaid gift none .this) chosenDraws)

/-- Gift's spell half: "If this is an instant or sorcery, if this spell's gift cost was paid, the
chosen player draws a card." [CR#702.174b,113.3a] -/
def spellHalf : Ability :=
  .spell none (doIf onInstantOrSorcery (doIf (costWasPaid gift none .this) chosenDraws))

/-- A definition on both classes, as gift's will be [CR#702.174a,702.174b]. -/
def giftShapedRow : KeywordFacts :=
  { word := .named "gift", argumentSchemas := [[]], onInstantOrSorceryCard := true,
    paidCost := true, costChoosesPlayer := true,
    definition := [⟨.triggered, none⟩, ⟨.spell, some .atResolution⟩] }

/-- Required pin: gift's second ability reads the opponent chosen as its gift cost was paid. -/
theorem okGiftReadsChosenPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (chosenAsPaid (.named "gift") .this) = [] := by
  decide

/-- Required pin: this fixture requests a local chosen player without a preceding choice or
an explicit linked facet. Linked abilities can instead read their paired choice [CR#607.2d]. -/
theorem badUnlinkedChosenPlayer :
    Ability.check (DRS.ofList []) (.spell none (instruct (the chosenPlayer) (draw (.lit 1))))
      = [.choiceRef .theChoice .player 0] := by
  decide

/-- Kicker's cost chooses no player, so there is none to read back [CR#702.33a,607.2d]. -/
theorem badKickerChoosesNoPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (chosenAsPaid (.named "kicker") .this)
      = [.linkedChoiceInScope kicker] := by
  decide

/-- Flying has no cost at all: the facet names nothing. -/
theorem badCostlessKeywordChoosesNoPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (chosenAsPaid (.named "flying") .this)
      = [.linkedFacetNamed] := by
  decide

/-- "The additional cost" names no declared choice. -/
theorem badTheAdditionalChoosesNoPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (.paid (.chosenPlayer .theAdditional) .this)
      = [.linkedChoiceInScope .theAdditional] := by
  decide

/-- The chosen player is a player, not an amount; an amount facet is not a player. -/
theorem badFacetReadAsTheOtherKind :
    (Amount.check (DRS.ofList []) (.paid (.chosenPlayer gift) .this),
      NounPhrase.checkAt (some .player) (DRS.ofList []) (.paid (.timesPaid kicker) .this))
      = ([.linkedFacetKind], [.linkedFacetKind]) := by
  decide

/-- The chosen player is a player: an object context refuses it. -/
theorem badChosenPlayerAsObject :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (chosenAsPaid (.named "gift") .this)
      = [.kindMismatch .object .player] := by
  decide

/-- Both guarded halves read the chosen player, each checked with no class context. -/
theorem okBothHalvesReadChosenPlayer :
    Ability.checkAll (DRS.ofList []) [permanentHalf, spellHalf] = [] := by
  decide

/-- Both halves fit a definition declaring a triggered and a spell ability. -/
theorem okBothHalvesFitGiftShapedDefinition :
    [permanentHalf, spellHalf].all giftShapedRow.bodyPartFits = true := by
  decide

/-- On a permanent card the permanent half reads the chosen player under its guard. -/
theorem okPermanentClassReadsChosenPlayer :
    cardTextOk [.creature] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

/-- On an instant card the spell half reads the chosen player under its guard. -/
theorem okSpellClassReadsChosenPlayer :
    cardTextOk [.instant] [.keyword (.named "gift") [] [permanentHalf, spellHalf]] = true := by
  decide

end Semantics.Proofs.LinkedChoiceReadback
