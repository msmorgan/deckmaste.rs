import Semantics
import Semantics.Check.Phrase

/-!
# Semantics.Primitives

The authoring boundary's primitive layer: the characteristic inputs that quality wording
expands through, the generated constructor wrappers (`declare_semantic_primitives`, under
`Semantics.Primitives.<Type>`), and a few worded primitive templates over them.
-/

namespace Semantics

/-- Convenience input for established quality wording. Expansion produces typed edits;
this input is not stored in the checked static or copy syntax. -/
inductive QualityPayload where
  | bundle (characteristics : CharacteristicBundle) (retained : Option CardType)
  | everyTypeOf (space : SubtypeSpace)
  | chosenQuality (quality : Predicate)
  | colored (colors : ColorSpec)

attribute [semantic_expression] QualityPayload

private def nonempty {α : Type} (xs : List α) : Option (List α) :=
  if xs.isEmpty then none else some xs

def Characteristics.typeChanges (c : Characteristics) (retained : Option CardType := none) :
    TypeLineChanges :=
  ⟨nonempty c.supertypes, nonempty c.types, nonempty c.subtypes, retained⟩

def Characteristics.valueEdits (c : Characteristics) (op : QualityOp) : List CharacteristicEdit :=
  let valueOp := if op == .loses then .loses else .sets
  c.name.toList.map (.name op) ++
    c.cost.toList.map (fun cost => .manaCost (some cost)) ++
    (if c.colors.isEmpty then [] else [.colors valueOp (.some c.colors)]) ++
    c.power.toList.map (.stat valueOp .power) ++
    c.toughness.toList.map (.stat valueOp .toughness) ++
    c.loyalty.toList.map (.stat valueOp .loyalty) ++
    c.defense.toList.map (.stat valueOp .defense) ++
    (if c.text.isEmpty then []
     else if op == .loses then [.removedAbilities (.specified (c.text.map .written))]
     else [.addedAbilities c.text])

def QualityPayload.edits (op : QualityOp) : QualityPayload → List CharacteristicEdit
  | .bundle b retained =>
    .typeLine op (b.characteristics.typeChanges retained) ::
      b.characteristics.valueEdits op ++ b.qualities.map (fun quality =>
        match quality with
        | .withEveryType space => .everyTypeOf op space
        | .withQuality q => .chosenQuality op q)
  | .everyTypeOf space => [.everyTypeOf op space]
  | .chosenQuality quality => [.chosenQuality op quality]
  | .colored colors => [.colors op colors]

/-- A play payment read in the caller's Reference Scope: its cost, when it names one. -/
def PlayPayment.inCaller : PlayPayment → PlayPayment
  | .payingInstead cost => .payingInstead (.inCaller cost)
  | .faceDown spec cost => .faceDown spec (.inCaller cost)
  | payment => payment

end Semantics

/-! Named wrappers for exposed primitives and fixed wording. Generated wrappers preserve
constructor signatures; the worded forms below expand into shared operations. The authoring
boundary records their names. -/

declare_semantic_primitives

namespace Semantics.Primitives.Action

/-- A numeric definition at instruction position uses the shared static definition. -/
semantic_template define (letter : Letter) (amount : Amount) : Semantics.Action :=
  .establish (.letterDefinition letter amount) none

semantic_template offer (body : Semantics.Action) (ifDid ifNot : Option Semantics.Action) :
    Semantics.Action :=
  .withContinuation .optional body ifDid ifNot

semantic_template doIfDone (body : Semantics.Action) (ifDid ifNot : Option Semantics.Action) :
    Semantics.Action := .withContinuation .required body ifDid ifNot

semantic_template repeatTimes (times : Amount) (body : Semantics.Action) : Semantics.Action :=
  .repeat_ (.fixed times body)

end Semantics.Primitives.Action

namespace Semantics.Primitives.NounPhrase

semantic_template both (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .and [left, right]
semantic_template eitherOf (left right : Semantics.NounPhrase) : Semantics.NounPhrase := .or [left, right]

end Semantics.Primitives.NounPhrase

namespace Semantics.Primitives.Cost

semantic_template either (left right : Semantics.Cost) : Semantics.Cost := .or [left, right]

end Semantics.Primitives.Cost

namespace Semantics.Primitives.Predicate

semantic_template isAttached (word : Option AttachWord) : Semantics.Predicate :=
  .attachment .host word none

semantic_template attachedBy (word : Option AttachWord) (by_ : NounPhrase) : Semantics.Predicate :=
  .attachment .host word (some by_)

semantic_template attachedTo (host : NounPhrase) : Semantics.Predicate :=
  .attachment .attachment none (some host)

/-- A transformed permanent is the whole double-faced permanent with its back face up. -/
semantic_template isTransformed : Semantics.Predicate := .currentFace .back

end Semantics.Primitives.Predicate

namespace Semantics.Primitives.StaticSpec

semantic_template qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    Semantics.StaticSpec := .characteristicChange subject (payload.edits op)

semantic_template abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : Semantics.StaticSpec :=
  .characteristicChange subject [.removedAbilities (.specified abilities)]

semantic_template allAbilityLoss (subject : NounPhrase) (except : Option Semantics.Predicate) :
    Semantics.StaticSpec := .characteristicChange subject [.removedAbilities (.allExcept except)]

end Semantics.Primitives.StaticSpec

namespace Semantics.Primitives.CopyExcept

semantic_template types (types : List CardType) (subtypes : List Subtype) : Semantics.CopyExcept :=
  .edits [.typeLine .adds (({ types := types, subtypes := subtypes } :
    Characteristics).typeChanges)]

semantic_template name (name : String) : Semantics.CopyExcept := .edits [.name .sets name]
semantic_template pt (power toughness : Amount) : Semantics.CopyExcept :=
  .edits [.stat .sets .power power, .stat .sets .toughness toughness]
semantic_template nonlegendary : Semantics.CopyExcept :=
  .edits [.typeLine .loses { supertypes := some [.legendary] }]
semantic_template color (color : Color) : Semantics.CopyExcept := .edits [.colors .sets (.some [color])]

end Semantics.Primitives.CopyExcept

namespace Semantics.Primitives.Action

semantic_template removeFromCombat (subject : NounPhrase) : Semantics.Action :=
  .combat subject (.participation .outsideCombat)

semantic_template becomeAttacking (subject : NounPhrase) (defender : Option NounPhrase) : Semantics.Action :=
  .combat subject (.participation (.attacking defender))

semantic_template becomeBlocking (subject blocked : NounPhrase) : Semantics.Action :=
  .combat subject (.blocking .attached blocked)

semantic_template stopBlocking (subject blocked : NounPhrase) : Semantics.Action :=
  .combat subject (.blocking .unattached blocked)

semantic_template unattach (subject : NounPhrase) : Semantics.Action :=
  .attachment .unattached subject none

end Semantics.Primitives.Action

namespace Semantics.Primitives.Action

semantic_template getEmblem (abilities : List Ability) : Semantics.Action :=
  .createObject (.lit 1) (.emblem none abilities) true none

end Semantics.Primitives.Action

namespace Semantics.Primitives.StaticSpec

semantic_template entryChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain)
    (disclosure : Disclosure) : Semantics.StaticSpec :=
  .choice .entry subject sort domain disclosure

semantic_template attachmentChoice (subject : NounPhrase) (sort : ChoiceSort) (domain : Option ChoiceDomain) :
    Semantics.StaticSpec := .choice .attachment subject sort domain .openly

end Semantics.Primitives.StaticSpec

namespace Semantics.Primitives.StaticSpec

semantic_template partScope (part : TurnPart) (whose : Option NounPhrase) (spec : Semantics.StaticSpec) :
    Semantics.StaticSpec := .conditional spec (.duringPart part whose) .asLongAs

end Semantics.Primitives.StaticSpec
