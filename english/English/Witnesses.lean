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
    (lexeme = .creature ∧ number = .singular ∧ surface = (["creature"] : Surface)) ∨
    (lexeme = .creature ∧ number = .plural ∧ surface = (["creatures"] : Surface)) ∨
    (lexeme = .artifact ∧ number = .singular ∧ surface = (["artifact"] : Surface)) ∨
    (lexeme = .artifact ∧ number = .plural ∧ surface = (["artifacts"] : Surface))
  adjectiveForm lexeme surface := lexeme = .white ∧ surface = (["white"] : Surface)

def narrow : Syntax Lexeme :=
  .coordinate (.modify (.adjective .white) (.noun .creature .plural)) (.noun .artifact .plural)

def shared : Syntax Lexeme :=
  .modify (.adjective .white) (.coordinate (.noun .creature .plural) (.noun .artifact .plural))

def surface : Surface := (["white", "creatures", "and", "artifacts"] : Surface)

theorem narrow_derives : Derives lexicon narrow (.nominal .plural) := by
  apply Judges.node (Production.coordinate rfl)
  apply JudgeChildren.cons (headGaps := []) (tailGaps := [])
  · apply Judges.modify
    · exact Judges.adjective rfl
    · exact Judges.noun (Or.inl rfl)
  · exact .cons (.noun (Or.inr rfl)) .nil

theorem shared_derives : Derives lexicon shared (.nominal .plural) := by
  apply Judges.modify
  · exact Judges.adjective rfl
  · exact .node (.coordinate rfl) (.cons (.noun (Or.inl rfl)) (.cons (.noun (Or.inr rfl)) .nil))

private theorem white_realizes : Realizes lexicon (.adjective .white) (["white"] : Surface) :=
  .adjective ⟨rfl, rfl⟩

private theorem creatures_realizes :
    Realizes lexicon (.noun .creature .plural) (["creatures"] : Surface) :=
  .noun (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))

private theorem artifacts_realizes :
    Realizes lexicon (.noun .artifact .plural) (["artifacts"] : Surface) :=
  .noun (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩)))

theorem narrow_realizes : Realizes lexicon narrow surface := by
  exact .node (.cons (.modify white_realizes creatures_realizes)
    (.cons artifacts_realizes .nil)) .coordinate

theorem shared_realizes : Realizes lexicon shared surface := by
  exact .modify white_realizes (.node (.cons creatures_realizes
    (.cons artifacts_realizes .nil)) .coordinate)

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
