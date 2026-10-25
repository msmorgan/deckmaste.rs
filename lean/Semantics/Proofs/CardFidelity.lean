import Semantics.Cards
import Semantics.Check.Card

/-!
# Semantics.Proofs.CardFidelity

Pins on authored card trees whose printed text an earlier encoding dropped or misread
(`lean-v3-rules-and-card-fidelity` §§3–5): each states the corrected tree, or the part of it the
correction is about, and why the Oracle text and the rules want it.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CardFidelity

/-- The text of a single-faced card. -/
def faceText : Card → List Ability
  | .singleFaced face => face.characteristics.text
  | _ => []

/-- The effect of a spell ability. -/
def spellEffect : Ability → Option Action
  | .spell _ effect => some effect
  | _ => none

/-- Memory Plunder: "You may cast target instant or sorcery card from an opponent's graveyard
without paying its mana cost." The cast is a one-shot optional instruction carried out during
resolution [CR#608.2g], not a durationless permission, which would last for the rest of the game
[CR#611.2a]. -/
theorem memoryPlunderCastsDuringResolution :
    ((faceText Cards.memoryPlunder.card).filterMap spellEffect) =
      [ may (cast (target (and [isInstantOrSorcery, isCard]))
          (from_ := graveyardOf anOpponent) (payment := withoutPaying)) ] := rfl

/-- Flawless Forgery and Arcane Proxy: "You may cast the copy without paying its mana cost," a
one-shot cast during resolution [CR#608.2g]. -/
theorem copiedCardIsCastDuringResolution :
    Cards.flawlessForgeryLine =
      sequentially
        [ exile (target (and [isInstantOrSorcery, in_ (graveyardOf anOpponent)])),
          copy .fromCardZone (that card) 1 [],
          may (cast (that .copy) (payment := withoutPaying)) ] := rfl

/-- Madness: the owner "may cast it by paying [cost] rather than paying its mana cost. If that
player doesn't, they put this card into their graveyard" [CR#702.35a]: a cast now, during the
trigger's resolution, or the graveyard. -/
theorem madnessCastsDuringTheTriggersResolution :
    (match madness (.mana [generic 1]) with
      | .keyword _ _ [_, .triggered _ _ _ _ _ _ _ (.instruct _ (.withContinuation _ arm _ _))] =>
        some arm
      | _ => none) =
      some (cast thisCard (from_ := exileZone)
        (payment := .payingInstead (.inCaller (.mana [generic 1])))) := rfl

end Semantics.Proofs.CardFidelity
