import Semantics
import Semantics.Check.Phrase

/-!
# Semantics.Primitives

The authoring boundary's primitive layer: the characteristic inputs that quality wording
expands through, and the generated constructor wrappers (`declare_semantic_primitives`, under
`Semantics.Primitives.<Type>`). Worded phrasings over a primitive are templates
(`Semantics.Templates`).
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

/-! Named wrappers for exposed primitives. Generated wrappers preserve constructor signatures;
the authoring boundary records their names. -/

declare_semantic_primitives
