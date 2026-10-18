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
      power := stat 1, toughness := stat 1, text := [keyword (.named "flying")] } }

private def withDraw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw (.lit 1))] } }

theorem printedDataNeedsNoExpressionMacro : plain.authoring.onlyAuthored = true := plain.onlyAuthored

theorem keywordRecordsMacroAuthorship : withKeyword.authoring.onlyAuthored = true :=
  withKeyword.onlyAuthored

theorem macroDefaultAgentRetainsAuthorship : withDraw.authoring.onlyAuthored = true :=
  withDraw.onlyAuthored

theorem authoredCardStillPassesSemanticCheck : withDraw.card.check = [] := withDraw.ok

private def rawKeywordCard : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [.keyword (.named "flying") [] []] } }

theorem expandedValueIsDefinitionallyIdentical : withKeyword.card = rawKeywordCard := by rfl

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| rawKeywordCard

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (instruct .you (draw (.lit 1)))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (.enact (.action "destroy") (.draw (.lit 1)) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (.enact (.action "destroy") (.move (target creature) (.zones [.zone .battlefield .bare]) (some (.zone .hand .bare)) .arrival none []) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially
        [.enact (.action "destroy") (.sequentially [.move (target creature) (.zones [.zone .battlefield .bare]) (some (.zone .hand .bare)) .arrival none []]) none none])] } }

/-- error: invalid {...} notation, constructor for `Spelled` is marked as private -/
#guard_msgs in
example : Spelled := { withKeyword with card := rawKeywordCard, ok := by decide }

/-- error: Invalid `⟨...⟩` notation: Constructor for `Semantics.Spelled` is marked as private -/
#guard_msgs in
example : Spelled := ⟨rawKeywordCard, .data [], by decide, by decide⟩

opaque hiddenCard : Card := rawKeywordCard

/-- error: Definition Semantics.Proofs.Authoring.hiddenCard has no authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| hiddenCard

/-- error: Semantic parameter card has no authoring evidence -/
#guard_msgs in
example (card : Card) : Spelled := spelled <| card

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <|
  let _discarded : Ability := .keyword (.named "flying") [] []
  withKeyword.card

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| (withKeyword.card, (Ability.keyword (.named "flying") [] [])).1

end Semantics.Proofs.Authoring
