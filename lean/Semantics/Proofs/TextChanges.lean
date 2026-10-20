import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.TextChanges

A Text Change is a specified rewrite of an object's rules text, applied in layer 3
[CR#612.1,613.1c]; the card is written once, its bracketed words in place as the transparent
markers `Predicate.bracketed` and `Action.bracketed` (`docs/decisions/semantics-v2.md` §7,
ruling 2026-10-09). The checker checks a marker's contents and nothing else, and checks a Text
Change by kinds only: the rewritten text is not checked again.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.TextChanges

/-- "{1}{U}{U}" -/
def cleaveCost : Cost := .mana [generic 1, pip .blue, pip .blue]

/-- "target [creature]" read where a player is wanted: the brackets change nothing, so the
mismatch is refused as it is without them. -/
theorem bracketedKindMismatchStillRefused :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (target (.bracketed isCreature))
      = NounPhrase.checkAt (some .player) (DRS.ofList []) (target isCreature) := by decide

/-- The refusal the bracketed mismatch shares is a real one. -/
theorem bracketedKindMismatchIsRefused :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (target (.bracketed isCreature)) ≠ [] := by decide

/-- "Counter target spell [that wasn't cast from a hand]." -/
theorem okBracketedDescription :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isSpell, .bracketed (.not (.castFrom (.zone .hand none)))])) = [] := by decide

/-- "[Draw a card.]": an action checks as its contents. -/
theorem okBracketedAction : Action.check (DRS.ofList []) (.bracketed (draw (.lit 1))) = [] := by
  decide

/-- "[This spell deals 2 damage to target creature.]" read as dealt to a player: refused inside
the brackets as outside. -/
theorem bracketedActionMismatchStillRefused :
    Action.check (DRS.ofList []) (.bracketed (Action.instruct thisSpell (.dealDamage (.lit 2) (target isCreature) .any)))
      = Action.check (DRS.ofList []) (Action.instruct thisSpell (.dealDamage (.lit 2) (target isCreature) .any)) := by decide

/-- Cleave's Text Change on a spell that prints no brackets removes nothing and still checks. -/
theorem okRemoveBracketedWithoutBrackets :
    Ability.check (DRS.ofList []) (.static (textChange thisSpell removeBracketed)) = [] := by decide

/-- "Change the text of target permanent by replacing all instances of one basic land type with
another." [CR#612.2] -/
theorem okReplaceLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent) (replaceWord [.subtype .land])) none) = [] := by
  decide

/-- "… one color word with Swamp": a replaced word of another sort than the row's is refused. -/
theorem badReplaceWordSort :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent)
        (replaceWord [.color] none (some (.basicLandType (swamp))))) none)
      = [.kindMismatch (.quality .color) (.quality (.subtype .land))] := by decide

/-- "Change the text of target spell or permanent by replacing all instances of one color word
with another or one basic land type with another." [CR#612.2] -/
theorem okReplaceColorOrLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target (.or [isSpell, isPermanent]))
        (replaceWord [.color, .subtype .land])) none) = [] := by decide

/-- "… one color word with Swamp": a word replaced by a word of another kind is refused. -/
theorem badReplaceAcrossKinds :
    TextChange.check (DRS.ofList []) (.replaceWord [.color, .subtype .land] (some (.color .black))
      (some (.basicLandType (swamp))))
      = [.kindMismatch (.quality .color) (.quality (.subtype .land))] := by decide

/-- A name is not a word a Text Change replaces [CR#612.2]. -/
theorem badReplaceWordCardName :
    TextChange.check (DRS.ofList []) (.replaceWord [.cardName] none none)
      = [.kindMismatch (.quality .color) (.quality .cardName)] := by decide

/-- Cleave's definition: the alternative cost, and the Text Change under its paid cost
[CR#702.148a]. -/
theorem okCleaveDefinition : Ability.check (DRS.ofList []) (cleave cleaveCost) = [] := by decide

/-- Overload's definition: the alternative cost, and "target" to "each" under its paid cost
[CR#702.96a]. -/
theorem okOverloadDefinition : Ability.check (DRS.ofList []) (overload cleaveCost) = [] := by decide

/-- Splice onto Arcane's definition: as you cast an Arcane spell, you may reveal this card; if you
do, that spell gains its text and you pay the cost as an additional cost [CR#702.47a]. -/
theorem okSpliceDefinition :
    Ability.check (DRS.ofList []) (splice_ (.hasSubtype (.spell "Arcane")) cleaveCost) = [] := by
  decide

end Semantics.Proofs.TextChanges
