import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.TextChanges

A Text Change is a specified rewrite of an object's rules text, applied in layer 3
[CR#612.1,613.1c]; cleave's Text Change removes all text found within square brackets
[CR#702.148a,702.148b].
The brackets are a fact about the wording, not about the meaning: the card is written once with its
bracketed words inside the identity template `bracketed`, which Core reduces away. The authoring
tree keeps the call, where lowering reads it to drive the text change. A Text Change is checked by
kinds only: the rewritten text is not checked again.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.TextChanges

/-- "{1}{U}{U}" -/
def cleaveCost : Cost := .mana [generic 1, pip .blue, pip .blue]

mutual
  /-- The Template and Definition calls a Form names, in order. -/
  private def formCalls : Authoring.Form → List Lean.Name
    | .call name arguments => name :: formsCalls arguments
    | .data children | .constructor _ children => formsCalls children
    | .binder _ body => formCalls body
    | .literal _ | .parameter _ | .raw _ => []

  private def formsCalls : List Authoring.Form → List Lean.Name
    | [] => []
    | form :: forms => formCalls form ++ formsCalls forms
end

/-- The refusal a bracketed mismatch shares with its bare form is a real one. -/
theorem bracketedKindMismatchIsRefused :
    NounPhrase.checkAt (some .player) (DRS.ofList []) (target (bracketed isCreature)) =
        [.kindMismatch .player .object] ∧
      NounPhrase.checkAt (some .player) (DRS.ofList []) (target isCreature) =
        [.kindMismatch .player .object] := by
  decide

/-- "Counter target spell [that wasn't cast from a hand]." -/
theorem okBracketedDescription :
    NounPhrase.checkAt (some .object) (DRS.ofList [])
      (target (.and [isSpell,
        bracketed (.not (did any (cast which (from_ := some (.zone .hand none))) thisObject))])) =
      [] := by
  decide

/-- "[Draw a card.]": an action checks as its contents. -/
theorem okBracketedAction : Action.check (DRS.ofList []) (bracketed (draw 1)) = [] := by
  decide

/-- "Cleave {1}{U}{U}. Draw a card [for each opponent]." in miniature: a spell with cleave and a
bracketed action. -/
private def bracketedSpell : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Bracket witness", types := [.sorcery], cost := some [generic 2],
      text := [cleave [generic 1, pip .blue, pip .blue], Primitives.Ability.spell none (bracketed (draw 1))] } }

/-- The authoring tree of a bracketed card records the `bracketed` call, for lowering to read. -/
theorem bracketedCardRecordsTheCall :
    (formCalls bracketedSpell.authoring).count ``Templates.bracketed = 1 := by decide

/-- The card with its brackets still checks, and a cleave card still checks. -/
theorem bracketedCleaveCardChecks : bracketedSpell.card.check = [] := bracketedSpell.ok

/-- The card's Core is the unbracketed card: nothing in Core marks the brackets. -/
theorem bracketedCardCoreIsUnbracketed :
    bracketedSpell.card = (spelled <| .singleFaced
      { characteristics :=
        { name := "Bracket witness", types := [.sorcery], cost := some [generic 2],
          text := [cleave [generic 1, pip .blue, pip .blue], Primitives.Ability.spell none (draw 1)] } }).card := by
  rfl

/-- Cleave's Text Change on a spell that prints no brackets removes nothing and still checks. -/
theorem okRemoveBracketedWithoutBrackets :
    Ability.check (DRS.ofList []) (.static (textChange thisSpell removeBracketed)) = [] := by decide

/-- "Change the text of target permanent by replacing all instances of one basic land type with
another." [CR#612.2] -/
theorem okReplaceLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent) (replaceWord [.subtype .land])) none) = [] := by
  decide

/-- Artificial Evolution: "Change the text of target spell or permanent by replacing all instances
of one creature type with another." A creature type word used as a creature type is replaced
like a color word or a land type [CR#612.2]. -/
theorem okReplaceCreatureTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target (.or [isSpell, isPermanent]))
        (replaceWord [.subtype .creature])) none) = [] := by
  decide

/-- New Blood: "Change the text of that creature by replacing all instances of one creature type
with Vampire." [CR#612.2] -/
theorem okReplaceCreatureTypeWithVampire :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isCreature)
        (replaceWord [.subtype .creature] none (some (.subtype vampire)))) none) = [] := by
  decide

/-- "… one creature type with Swamp": a land type is not a creature type. -/
theorem badReplaceCreatureTypeWithLandType :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isCreature)
        (replaceWord [.subtype .creature] none (some (.subtype swamp)))) none)
      = [.textWordSort [.subtype .creature] (.subtype .land)] := by decide

/-- "… one color word with Swamp": a replaced word of another sort than the row's is refused. -/
theorem badReplaceWordSort :
    Action.check (DRS.ofList [])
      (.establish (textChange (target isPermanent)
        (replaceWord [.color] none (some (.subtype swamp)))) none)
      = [.textWordSort [.color] (.subtype .land)] := by decide

/-- "Change the text of target spell or permanent by replacing all instances of one color word
with another or one basic land type with another." [CR#612.2] -/
theorem okReplaceColorOrLandTypeWord :
    Action.check (DRS.ofList [])
      (.establish (textChange (target (.or [isSpell, isPermanent]))
        (replaceWord [.color, .subtype .land])) none) = [] := by decide

/-- "… one color word with Swamp": a word replaced by a word of another kind is refused. -/
theorem badReplaceAcrossKinds :
    TextChange.check (DRS.ofList []) (.replaceWord [.color, .subtype .land] (some (.color .black))
      (some (.subtype swamp)))
      = [.kindMismatch (.quality .color) (.quality (.subtype .land))] := by decide

/-- A name is not a word a Text Change replaces [CR#612.2]. -/
theorem badReplaceWordCardName :
    TextChange.check (DRS.ofList []) (.replaceWord [.cardName] none none)
      = [.textChangeSort .cardName] := by decide

/-- Cleave's definition: the alternative cost, and the Text Change under its paid cost
[CR#702.148a]. -/
theorem okCleaveDefinition : Ability.check (DRS.ofList []) (cleave cleaveCost) = [] := by decide

/-- Overload's definition: the alternative cost, and "target" to "each" under its paid cost
[CR#702.96a]. -/
theorem okOverloadDefinition : Ability.check (DRS.ofList []) (overload cleaveCost) = [] := by decide

/-- Splice onto Arcane's definition: as you cast an Arcane spell, you may reveal this card; if you
do, that spell gains its text and you pay the cost as an additional cost [CR#702.47a]. -/
theorem okSpliceDefinition :
    Ability.check (DRS.ofList []) (Ability.splice (.hasSubtype arcane) cleaveCost) = [] := by
  decide

end Semantics.Proofs.TextChanges
