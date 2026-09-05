import Semantics.Check.Card

/-!
# Semantics.Proofs.Pin

The shape of a pin suite. A `Witness` is a term with the sentence it spells; a `Pin` is a
witness the checker admits beside the same sentence mis-stated in exactly one place, with the
one refusal that mis-statement earns. Both verdicts are closed by `decide` at the definition,
so a pin that does not hold does not define, as an Idris pin that did not elaborate did not.

The sentence is data, not a docstring: a printed sentence names its card, and a synthetic one
says so, so a later renderer can be held to `render ok.term = ok.sentence`.
-/

namespace Semantics.Proofs

/-- What a witness spells: oracle text as printed on a named card, or a rules-meaningful
sentence written for the pin. -/
inductive Sentence where
  | printed (card text : String)
  | synthetic (text : String)
  deriving Repr, BEq

def Sentence.text : Sentence → String
  | .printed _ t => t
  | .synthetic t => t

/-- A term and the sentence it spells. -/
structure Witness (α : Type) where
  sentence : Sentence
  term : α

/-- A spelling the checker admits, standing alone: a twin with no pin beside it yet. -/
structure Spelling {α : Type} (check : α → List Refusal) where
  witness : Witness α
  ok : check witness.term = []

/-- A refusal, non-vacuous: `ok` is admitted, `bad` is the same sentence mis-stated in exactly
one place, and `reason` is the obligation the mis-statement fails. `also` lists the refusals a
single mis-statement cascades into (a failed anaphor leaves a zone unknown); it is empty for
nearly every pin, and a pin declares the whole list either way. -/
structure Pin {α : Type} (check : α → List Refusal) where
  ok : Witness α
  bad : Witness α
  reason : Refusal
  also : List Refusal := []
  hOk : check ok.term = []
  hBad : check bad.term = reason :: also

/-- Build a `Spelling`, closing the verdict by `decide`. -/
def spelling {α : Type} {check : α → List Refusal} (witness : Witness α)
    (ok : check witness.term = [] := by decide) : Spelling check :=
  ⟨witness, ok⟩

/-- Build a `Pin`, closing both verdicts by `decide`. -/
def pin {α : Type} {check : α → List Refusal} (ok bad : Witness α) (reason : Refusal)
    (also : List Refusal := []) (hOk : check ok.term = [] := by decide)
    (hBad : check bad.term = reason :: also := by decide) : Pin check :=
  ⟨ok, bad, reason, also, hOk, hBad⟩

/-- A synthetic witness. -/
def says {α : Type} (text : String) (term : α) : Witness α := ⟨.synthetic text, term⟩

/-- A witness quoting a printed card. -/
def prints {α : Type} (card text : String) (term : α) : Witness α := ⟨.printed card text, term⟩

end Semantics.Proofs
