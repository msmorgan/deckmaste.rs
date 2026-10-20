import Semantics
import Semantics.CallerScope

/-!
# Semantics.Primitives

The authoring boundary's primitive layer: the characteristic inputs that quality wording
expands through, and the generated constructor wrappers (`declare_semantic_primitives`, under
`Semantics.Primitives.<Type>`). Worded phrasings over a primitive are templates
(`Semantics.Templates`).
-/

namespace Semantics

/-- Convenience input for established quality wording, `op` written once over it. Expansion
produces typed edits; this input is not stored in the checked static or copy syntax. -/
inductive QualityPayload where
  /-- `characteristics`, each written with `op`. A `retained` type ("It's still a land") keeps
  every prior type, supertype and subtype [CR#205.1b], so the type line is added, whatever `op`
  writes the rest; the type it names is wording only. -/
  | bundle (characteristics : CharacteristicBundle) (retained : Option CardType)
  | quality (quality : WrittenQuality)
  | colored (colors : ColorSpec)

attribute [semantic_expression] QualityPayload

private def nonempty {α : Type} (xs : List α) : Option (List α) :=
  if xs.isEmpty then none else some xs

def Characteristics.typeChanges (c : Characteristics) : TypeLineChanges :=
  ⟨nonempty c.supertypes, nonempty c.types, nonempty c.subtypes⟩

/-- The value characteristics `c` writes, each with `op`: an addition sets a written value,
as "becomes a 2/2 creature in addition to its other types" does, and a removal removes it. -/
def Characteristics.valueEdits (c : Characteristics) (op : QualityOp) : List CharacteristicEdit :=
  let valueOp := if op == .loses then .loses else .sets
  c.name.toList.map (.name valueOp) ++
    c.cost.toList.map (fun cost => .manaCost (if op == .loses then none else some cost)) ++
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
    .typeLine (if retained.isSome then .adds else op) b.characteristics.typeChanges ::
      b.characteristics.valueEdits op ++ b.qualities.map (.quality op)
  | .quality q => [.quality op q]
  | .colored colors => [.colors op colors]

/-- A ballot read in the caller's Reference Scope: its candidates, when it names them. -/
def Ballot.inCaller : Ballot → Ballot
  | .byCandidate candidates => .byCandidate (.inCaller candidates)
  | ballot => ballot

/-- A play payment read in the caller's Reference Scope: its cost, when it names one. -/
def PlayPayment.inCaller : PlayPayment → PlayPayment
  | .payingInstead cost => .payingInstead (.inCaller cost)
  | .faceDown spec cost => .faceDown spec (.inCaller cost)
  | payment => payment

end Semantics

/-! Named wrappers for exposed primitives. Generated wrappers preserve constructor signatures;
the authoring boundary records their names. -/

declare_semantic_primitives
