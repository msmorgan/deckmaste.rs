import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.LinkedChoiceReadback

Gift's first ability is a cost: "As an additional cost to cast this spell, you may choose an
opponent." [CR#702.174a]. Its second ability hands the effect to the chosen player
[CR#702.174b,702.174e], and the two are linked [CR#607.2d,607.2i]. The second reads the choice
back through a paid-cost facet keyed by the cost's name, as kicker's "was kicked" readback is
[CR#702.33e] (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-06 on gift and the
additional-cost keywords): `PaidFacet.chosenPlayer`, read as a noun phrase by `NounPhrase.paid`.

The facet resolves where the named cost chooses a player when it is paid (the keyword row's
`costChoosesPlayer`); a cost that chooses no player leaves the linked ability nothing to read,
which is a reference refusal [CR#607.2d]. A bare "the chosen player" with no choice in its own
ability is still refused: it does not reach across to a cost.

No registry row declares gift's definition until gift is bodied, so the definition-fit pins hold
a row written here, shaped as gift's will be.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.LinkedChoiceReadback

/-- "its gift cost" -/
def gift : PaidCostName := .byKeyword (.named "gift")

/-- "its kicker cost" -/
def kicker : PaidCostName := .byKeyword (.named "kicker")

/-- "if this is a permanent card" -/
def onPermanentCard : Condition := .matches .this permanentCard

/-- "if this is an instant or sorcery" -/
def onInstantOrSorcery : Condition := .matches .this instantOrSorcery

/-- "the chosen player draws a card", the player read back from gift's cost. -/
def chosenDraws : Instruction := act (chosenAsPaid gift .this) (draw (.lit 1))

/-- Gift's permanent half: "When this permanent enters, if this is a permanent card, if its gift
cost was paid, the chosen player draws a card." [CR#702.174b,702.174e] -/
def permanentHalf : Ability :=
  triggeredIf (Primitives.GameEvent.enters thisPermanent none) onPermanentCard
    (doIf (costWasPaid gift none .this) chosenDraws)

/-- Gift's spell half: "If this is an instant or sorcery, if this spell's gift cost was paid, the
chosen player draws a card." [CR#702.174b,113.3a] -/
def spellHalf : Ability :=
  .spell none (doIf onInstantOrSorcery (doIf (costWasPaid gift none .this) chosenDraws))

/-- A definition on both classes, as gift's will be [CR#702.174a,702.174b]. -/
def giftShapedRow : KeywordFacts :=
  { word := .named "gift", argumentSchemas := [[]], onInstantOrSorceryCard := true,
    paidCost := true, costChoosesPlayer := true, definition := [.triggered, .spell] }

/-- Required pin: gift's second ability reads the opponent chosen as its gift cost was paid. -/
theorem okGiftReadsChosenPlayer :
    NounPhrase.check (some .player) [] (chosenAsPaid gift .this) = [] := by
  decide

/-- Required pin: "the chosen player draws a card" in an ability that chose no player is refused;
a choice made while paying a cost is not in its scope [CR#607.2d]. -/
theorem badUnlinkedChosenPlayer :
    Ability.check [] (.spell none (act (the chosenPlayer) (draw (.lit 1))))
      = [.choiceRef .theChoice .player 0] := by
  decide

/-- Kicker's cost chooses no player, so there is none to read back [CR#702.33a,607.2d]. -/
theorem badKickerChoosesNoPlayer :
    NounPhrase.check (some .player) [] (chosenAsPaid kicker .this)
      = [.paidChoiceInScope kicker] := by
  decide

/-- Flying has no cost at all: the facet names nothing. -/
theorem badCostlessKeywordChoosesNoPlayer :
    NounPhrase.check (some .player) [] (chosenAsPaid (.byKeyword (.named "flying")) .this)
      = [.paidFacetNamed] := by
  decide

/-- "The additional cost" names no declared choice. -/
theorem badTheAdditionalChoosesNoPlayer :
    NounPhrase.check (some .player) [] (chosenAsPaid .theAdditional .this)
      = [.paidChoiceInScope .theAdditional] := by
  decide

/-- The chosen player is a player, not an amount; an amount facet is not a player. -/
theorem badFacetReadAsTheOtherKind :
    (Amount.check [] (.paid (.chosenPlayer gift) .this),
      NounPhrase.check (some .player) [] (.paid (.timesPaid kicker) .this))
      = ([.paidFacetKind], [.paidFacetKind]) := by
  decide

/-- The chosen player is a player: an object context refuses it. -/
theorem badChosenPlayerAsObject :
    NounPhrase.check (some .object) [] (chosenAsPaid gift .this)
      = [.kindMismatch .object .player] := by
  decide

/-- Both guarded halves read the chosen player, each checked with no class context. -/
theorem okBothHalvesReadChosenPlayer :
    Ability.checkAll [] [permanentHalf, spellHalf] = [] := by
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
