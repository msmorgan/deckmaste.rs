import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions

open Semantics Semantics.Templates Semantics.Definitions

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

theorem printedDataNeedsNoExpressionTemplate : plain.authoring.onlyAuthored = true := plain.onlyAuthored

theorem keywordRecordsTemplateAuthorship : withKeyword.authoring.onlyAuthored = true :=
  withKeyword.onlyAuthored

theorem templateDefaultAgentRetainsAuthorship : withDraw.authoring.onlyAuthored = true :=
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
      text := [Primitives.Ability.spell none (.enact (.action "destroy") (.draw (.exactlyOf (.lit 1))) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (.enact (.action "destroy") (.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival none []) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially
        [.enact (.action "destroy") (.sequentially [.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival none []]) none none])] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Replacement.instead] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (replace (draw (.lit 1)) (.instead (draw (.lit 2))))] } }

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

-- `Subtype` is a semantic expression and `internal_expansion`: a card names a catalog entry
-- (`Semantics.Definitions.Subtypes`), no primitive wrapper spells one from a string, and a raw
-- constructor in a card is refused. *Mishra's Factory* is an Assembly-Worker [CR#205.3m], not the
-- "AssemblyWorker" a raw string let it write. The raw subtypes below are deliberate: each pins a
-- refusal. The accepted direction is the corpus: *Mishra's Factory* (`Cards/Mana.lean`) and the
-- Adventure card in `Cards/Faces.lean` are `spelled` with catalog subtypes.

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Subtype.of] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.artifact, .creature],
      subtypes := [.of .creature "AssemblyWorker"], cost := some [generic 1], power := stat 2,
      toughness := stat 2 } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Subtype.spell] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.instant], subtypes := [.spell "Adventure"],
      cost := some [generic 1], text := [Primitives.Ability.spell none (draw (.lit 1))] } }

/-- error: Unknown identifier `Semantics.Primitives.Subtype.of` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.of .creature "Dragon"

/-- error: Unknown identifier `Semantics.Primitives.Subtype.spell` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.spell "Adventure"

end Semantics.Proofs.Authoring
