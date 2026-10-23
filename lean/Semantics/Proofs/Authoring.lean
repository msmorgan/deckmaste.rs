import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions
import Semantics.Cards.Choice
import Semantics.Cards.Cost

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Authoring

private def plain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1 } }

private def withKeyword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [reach] } }

private def withDraw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (draw 1)] } }

theorem printedDataNeedsNoExpressionTemplate : plain.authoring.onlyAuthored = true := plain.onlyAuthored

theorem keywordRecordsDefinitionAuthorship : withKeyword.authoring.onlyAuthored = true :=
  withKeyword.onlyAuthored

theorem templateDefaultAgentRetainsAuthorship : withDraw.authoring.onlyAuthored = true :=
  withDraw.onlyAuthored

theorem authoredCardStillPassesSemanticCheck : withDraw.card.check = [] := withDraw.ok

private def rawKeywordCard : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [.keyword (.named "reach") [] []] } }

theorem expandedValueIsDefinitionallyIdentical : withKeyword.card = rawKeywordCard := by rfl

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| rawKeywordCard

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (instruct .you (draw 1))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (.enact (.action "destroy") (.draw (.exactlyOf 1)) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none
        (.enact (.action "destroy") (.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival []) none none)] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.enact] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (Primitives.Action.sequentially
        [.enact (.action "destroy") (.sequentially [.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival []]) none none])] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Replacement.instead] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (replace (draw 1) (.instead (draw 2)))] } }

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
      cost := some [generic 1], text := [Primitives.Ability.spell none (draw 1)] } }

/-- error: Unknown identifier `Semantics.Primitives.Subtype.of` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.of .creature "Dragon"

/-- error: Unknown identifier `Semantics.Primitives.Subtype.spell` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.spell "Adventure"

-- `CardType` is a catalog item in the same way (`Semantics.Definitions.CardTypes`): a structure
-- carrying its own fields, tagged `semantic_expression` and `internal_expansion`. A card names a
-- declared card type (`.creature`); a raw structure in a card is refused, here one that would
-- make an "Artifact" that is no permanent type [CR#110.4].

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.CardType.mk] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [{ name := "Artifact", permanent := false }],
      cost := some [generic 1] } }

/-- error: Unknown identifier `Semantics.Primitives.CardType.mk` -/
#guard_msgs in
example : CardType := Semantics.Primitives.CardType.mk "Artifact" true [] false

-- `CounterKind`, `DesignationLabel` and `RelationLabel` are catalog items in the same way
-- (`Semantics.Definitions.Counters`, `.Designations`, `.Relations`): each value carries its own
-- columns, a counter's holder and a designation's scope, and is tagged `semantic_expression` and
-- `internal_expansion`. A card names a declared item; a raw constructor in a card is refused, so
-- an undeclared counter, designation or relation never reaches the checker, and a misspelled
-- catalog name is no declaration at all. The raw items below are deliberate: each pins a
-- refusal. The accepted direction is the witnesses that follow and the corpus.

private def sorceryWith (action : Action) : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none action] } }

private def withCatalogItems : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [Primitives.Ability.spell none (putCounters 1 p1p1Counter (target isCreature)),
        Primitives.Ability.spell none (gainDesignation you monarch),
        Primitives.Ability.spell none (relate this_ haunting (target isCreature))] } }

theorem catalogItemsRecordAuthorship : withCatalogItems.authoring.onlyAuthored = true :=
  withCatalogItems.onlyAuthored

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.CounterKind.named] -/
#guard_msgs in
example : Spelled := spelled <|
  sorceryWith (putCounters 1 (CounterKind.named "typoCounter") (target isCreature))

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.DesignationLabel.named] -/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (gainDesignation you (.named "dusk" .heldByGame))

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.RelationLabel.named] -/
#guard_msgs in
example : Spelled := spelled <|
  sorceryWith (relate this_ (.named "unrelated") (target isCreature))

/--
error: Unknown identifier `p1p1Countr`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (putCounters 1 p1p1Countr (target isCreature))

/--
error: Unknown identifier `monarc`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (gainDesignation you monarc)

/--
error: Unknown identifier `hauntng`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (relate this_ hauntng (target isCreature))

/-- error: Unknown identifier `Semantics.Primitives.CounterKind.named` -/
#guard_msgs in
example : CounterKind := Semantics.Primitives.CounterKind.named "typoCounter"

/-- error: Unknown identifier `Semantics.Primitives.DesignationLabel.named` -/
#guard_msgs in
example : DesignationLabel := Semantics.Primitives.DesignationLabel.named "dusk" .heldByGame

/-- error: Unknown identifier `Semantics.Primitives.RelationLabel.named` -/
#guard_msgs in
example : RelationLabel := Semantics.Primitives.RelationLabel.named "unrelated"

-- A `KeywordLabel` is a catalog item in the same way: each keyword ability Definition generates
-- its authored label constant (`kicker.label`), which a card writes by the Definition's name
-- (`keywordCostPaid kicker it`), and `KeywordLabel.named` is an internal expansion. So a card
-- names only a declared keyword: a raw label is refused and a misspelled one is no identifier.
-- `Ability.keyword` and the by-label helpers that spell it (`keywordCosting`, …) are internal
-- too, so a card writes a keyword ability through its typed Definition, whose signature fixes
-- its parameters. The accepted direction is `withKeyword` above and the corpus (*Ertai's
-- Trickery*, *Merfolk Falconer*).

private def instantWith (action : Action) : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.instant], cost := some [generic 1],
      text := [spell action] } }

theorem keywordLabelIsGenerated : kicker.label = .named "kicker" ∧
    multikicker.label = .named "kicker" := by decide

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.KeywordLabel.named] -/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (doOnlyIf (counter (target isSpell)) (keywordCostPaid (.named "flyign") it))

/--
error: Unknown identifier `kickre`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (doOnlyIf (counter (target isSpell)) (keywordCostPaid kickre it))

/-- error: Unknown identifier `Semantics.Primitives.KeywordLabel.named` -/
#guard_msgs in
example : KeywordLabel := Semantics.Primitives.KeywordLabel.named "flyign"

/-- error: Unknown identifier `Semantics.Primitives.Ability.keyword` -/
#guard_msgs in
example : Ability := Semantics.Primitives.Ability.keyword (.named "flying") [] []

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [keywordCosting ward [generic 2]] } }

-- A keyword action's `Deed` is a catalog item the same way: each Action Definition generates its
-- authored deed constant (`exile.deed`), which a card writes by the Definition's name
-- (`enact exile …`), and `Deed.action` is an internal expansion. So a card names only a declared
-- keyword action: a raw label is refused and a misspelled one is no identifier.

theorem deedIsGenerated : exile.deed = .action "exile" ∧ searchZonesOf.deed = search.deed ∧
    explore.deed = .action "explore" true := by decide

private def withEnactedDeed : Spelled := spelled <|
  instantWith (enact destroy (destroy (target isCreature)))

theorem enactedDeedRecordsDefinitionAuthorship : withEnactedDeed.authoring.onlyAuthored = true :=
  withEnactedDeed.onlyAuthored

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Deed.action] -/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (enact (.action "descry") (destroy (target isCreature)))

/--
error: Unknown identifier `destory`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (enact destory (destroy (target isCreature)))

/-! ## Coercions and the card envelope

`spelled` accepts the authoring coercions: core's `α → Option α`, a written mana cost and an action
as a `Cost`, a subtype as a `Predicate`, and a numeral as an `Amount`. A coercion is expanded by
the elaborator before `Authoring.inspect` reads the term, so it reaches `inspect` as the Template
call it unfolds to. A numeral stays an `OfNat.ofNat` term that `inspect` reads through to its
`Amount.lit` leaf. `singleFacedCard` frames a single-faced card's characteristics. Each pin below
restates a corpus card as it was written before the sweep and compares the two. -/

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

mutual
  /-- The numeric literals a Form names, in order. -/
  private def formNumerals : Authoring.Form → List Nat
    | .literal (.natVal value) => [value]
    | .call _ children | .data children | .constructor _ children => formsNumerals children
    | .binder _ body => formNumerals body
    | .literal (.strVal _) | .parameter _ | .raw _ => []

  private def formsNumerals : List Authoring.Form → List Nat
    | [] => []
    | form :: forms => formNumerals form ++ formsNumerals forms
end

/-- A numeral is retained more than once along the path `inspect` reads it through, so literals
are compared as runs: repeats of one literal in a row count once. -/
private def runs : List Nat → List Nat
  | a :: b :: rest => if a = b then runs (b :: rest) else a :: runs (b :: rest)
  | other => other

/-- The characteristics of a single-faced card's Form, under either envelope. -/
private def faceOf : Authoring.Form → Authoring.Form
  | .call `Semantics.Templates.singleFacedCard [characteristics] => characteristics
  | .constructor `Semantics.Card.singleFaced
      [.constructor `Semantics.CardFace.mk [characteristics]] => characteristics
  | form => form

/-- *Cold Storage* before the sweep: `some`, `mana [...]` and `perform (...)` written out. -/
private def coldStorageWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [generic 4], types := [.artifact],
      text :=
        [ activated (mana [generic 3]) (exile (target (and [isCreature, youControl]))),
          activated (perform (sacrifice thisArtifact))
            (returnToBattlefield (each (and [isCreature, exiledWithThisArtifact]))
              [(under you)]) ] } }

theorem costCoercionsKeepTheCard : Cards.coldStorage.card = coldStorageWritten.card := rfl

theorem costCoercionsKeepTheAuthoring :
    faceOf Cards.coldStorage.authoring = faceOf coldStorageWritten.authoring := rfl

theorem envelopeIsTheSingleFacedCardTemplate :
    (formCalls Cards.coldStorage.authoring).head? = some ``Templates.singleFacedCard := rfl

/-- *Korlessa, Scale Singer* before the sweep: `hasSubtype dragon` written out. -/
private def korlessaWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [pip .green, pip .blue],
      supertypes := [.legendary], types := [.creature],
      subtypes := [dragon, bard],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static (Cards.castFromTop (and [isSpell, hasSubtype dragon])) ],
      power := stat 1, toughness := stat 4 } }

theorem subtypeCoercionKeepsTheCard : Cards.korlessaScaleSinger.card = korlessaWritten.card := rfl

theorem subtypeCoercionKeepsTheAuthoring :
    faceOf Cards.korlessaScaleSinger.authoring = faceOf korlessaWritten.authoring := rfl

/-- *Char-Rumbler* before the sweep: `.lit 1`, `.lit 0` and `some untilEndOfTurn` written out. -/
private def charRumblerWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [generic 2, pip .red, pip .red], types := [.creature],
      subtypes := [elemental],
      text :=
        [ doubleStrike,
          activated (mana [pip .red])
            (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (.lit (-1)), toughness := stat 3 } }

theorem numeralsKeepTheCard : Cards.charRumbler.card = charRumblerWritten.card := rfl

theorem numeralsKeepTheTemplates :
    formCalls (faceOf Cards.charRumbler.authoring) = formCalls (faceOf charRumblerWritten.authoring) := rfl

theorem numeralsKeepTheLiterals :
    runs (formNumerals (faceOf Cards.charRumbler.authoring)) =
      runs (formNumerals (faceOf charRumblerWritten.authoring)) := rfl

-- A leading-dot name resolves against the expected type before any coercion applies, so an
-- optional kind keeps its `some` (lean-v3-deal-damage-kind) and a mana cost of leading-dot
-- symbols alone keeps its `mana`.

/--
error: Unknown constant `Option.combat`

Note: Inferred this name from the expected resulting type of `.combat`:
  Option DamageKind
-/
#guard_msgs in
example : Action := dealDamage (to := a isPlayer) (kind := .combat)

/--
error: Invalid dotted identifier notation: The expected type of `.snow` could not be determined

Hint: Using one of these would be unambiguous:
  [apply] `ManaSymbol.snow`
  [apply] `Supertype.snow`
-/
#guard_msgs in
example : Ability := activated [.snow] (draw 1)

end Semantics.Proofs.Authoring
