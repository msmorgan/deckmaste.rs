import Semantics.Macros

open Semantics Semantics.Macros

namespace Semantics.Proofs.Authoring

private def plain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1 } }

private def withKeyword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [keyword "Flying"] } }

private def withDraw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw (.lit 1))] } }

theorem printedDataNeedsNoExpressionMacro : plain.authoring.onlyMacros = true := plain.onlyMacros

theorem keywordRecordsMacroAuthorship : withKeyword.authoring.onlyMacros = true :=
  withKeyword.onlyMacros

theorem macroDefaultAgentRetainsAuthorship : withDraw.authoring.onlyMacros = true :=
  withDraw.onlyMacros

theorem authoredCardStillPassesSemanticCheck : withDraw.card.check = [] := withDraw.ok

private def rawKeywordCard : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [.keyword "Flying" none none] } }

theorem expandedValueIsDefinitionallyIdentical : withKeyword.card = rawKeywordCard := by rfl

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| rawKeywordCard

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw (.lit 1) (agent := .you))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (.enact (.action "Destroy") (.draw (.lit 1) (agent := .you)))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (.enact (.action "Destroy") (.move (target creature) (.zone .hand .bare) []))] } }

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Instruction.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (Primitives.Instruction.sequence
        [.enact (.action "Destroy") (.sequence [.move (target creature) (.zone .hand .bare) []])])] } }

/-- error: invalid {...} notation, constructor for `Spelled` is marked as private -/
#guard_msgs in
example : Spelled := { withKeyword with card := rawKeywordCard, ok := by decide }

/-- error: Invalid `⟨...⟩` notation: Constructor for `Semantics.Spelled` is marked as private -/
#guard_msgs in
example : Spelled := ⟨rawKeywordCard, .data [], by decide, by decide⟩

opaque hiddenCard : Card := rawKeywordCard

/-- error: Definition Semantics.Proofs.Authoring.hiddenCard has no macro-authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| hiddenCard

/-- error: Semantic parameter card has no macro-authoring evidence -/
#guard_msgs in
example (card : Card) : Spelled := spelled <| card

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <|
  let _discarded : Ability := .keyword "Flying" none none
  withKeyword.card

/-- error: Card definitions must use semantic macros; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| (withKeyword.card, (Ability.keyword "Flying" none none)).1

end Semantics.Proofs.Authoring
