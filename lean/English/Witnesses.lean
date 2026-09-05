import English.Grammar

/-! Synthetic witnesses of the fragment's structure, licensing and surface ambiguity. -/

namespace English.Witnesses

inductive Lexeme where
  | white
  | creature
  | artifact
  deriving DecidableEq

/-- A finite illustrative inventory, not a production vocabulary or licensing rule. -/
def lexicon : Lexicon Lexeme where
  noun lexeme _ := lexeme = .creature ∨ lexeme = .artifact
  adjective lexeme := lexeme = .white
  nounForm lexeme number surface :=
    (lexeme = .creature ∧ number = .singular ∧ surface = ["creature"]) ∨
    (lexeme = .creature ∧ number = .plural ∧ surface = ["creatures"]) ∨
    (lexeme = .artifact ∧ number = .singular ∧ surface = ["artifact"]) ∨
    (lexeme = .artifact ∧ number = .plural ∧ surface = ["artifacts"])
  adjectiveForm lexeme surface := lexeme = .white ∧ surface = ["white"]

def narrow : Syntax Lexeme :=
  .coordinate (.modify (.adjective .white) (.noun .creature .plural)) (.noun .artifact .plural)

def shared : Syntax Lexeme :=
  .modify (.adjective .white) (.coordinate (.noun .creature .plural) (.noun .artifact .plural))

def surface : Surface := ["white", "creatures", "and", "artifacts"]

theorem narrow_derives : Derives lexicon narrow (.nominal .plural) := by
  apply Derives.coordinate
  · apply Derives.modify
    · exact Derives.adjective rfl
    · exact Derives.noun (Or.inl rfl)
  · exact Derives.noun (Or.inr rfl)

theorem shared_derives : Derives lexicon shared (.nominal .plural) := by
  apply Derives.modify
  · exact Derives.adjective rfl
  · exact Derives.coordinate (Derives.noun (Or.inl rfl)) (Derives.noun (Or.inr rfl))

private theorem white_realizes : Realizes lexicon (.adjective .white) ["white"] :=
  .adjective ⟨rfl, rfl⟩

private theorem creatures_realizes : Realizes lexicon (.noun .creature .plural) ["creatures"] :=
  .noun (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))

private theorem artifacts_realizes : Realizes lexicon (.noun .artifact .plural) ["artifacts"] :=
  .noun (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩)))

theorem narrow_realizes : Realizes lexicon narrow surface := by
  exact Realizes.coordinate (Realizes.modify white_realizes creatures_realizes) artifacts_realizes

theorem shared_realizes : Realizes lexicon shared surface := by
  exact Realizes.modify white_realizes (Realizes.coordinate creatures_realizes artifacts_realizes)

/-- Surface identity does not force identity of grammatical analyses. -/
theorem distinct_analyses :
    narrow ≠ shared ∧
    Admissible lexicon narrow (.nominal .plural) surface ∧
    Admissible lexicon shared (.nominal .plural) surface := by
  refine ⟨?_, ⟨narrow_derives, narrow_realizes⟩, ⟨shared_derives, shared_realizes⟩⟩
  intro equality
  cases equality

/-- An adjective's spelling does not grant it a noun use in the illustrative lexicon. -/
theorem adjective_not_noun (number : Number) :
    ¬ Derives lexicon (.noun .white number) (.nominal number) := by
  intro derivation
  cases derivation with
  | noun licensed => cases licensed with
    | inl equality => cases equality
    | inr equality => cases equality

end English.Witnesses
