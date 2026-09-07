import English.AgreementInteractions

/-! Word-form agreement requirements and counterexamples to the current surface-independent model.
The lexical license below is evidence for a future lexical interface; `Admissible` does not yet
consume it. The counterexamples deliberately retain that distinction. -/
namespace English.WordFormInteractions
open Documents (Witness)

inductive Lexeme where
  | pronoun (agreement : Agreement)
  | be | exile
  deriving DecidableEq

def pastBe (agreement : Agreement) (surface : Surface) : Prop :=
  (agreement.person ≠ .second ∧ agreement.number = .singular ∧ surface = (["was"] : Surface)) ∨
  ((agreement.person = .second ∨ agreement.number = .plural) ∧ surface = (["were"] : Surface))

def lexicon : Lexicon Lexeme where
  noun _ _ := False
  adjective _ := False
  nounForm _ _ _ := False
  adjectiveForm _ _ := False
  word head category := ∃ agreement, head = .pronoun agreement ∧ category = .nounPhrase agreement
  wordForm head category surface := ∃ agreement,
    head = .pronoun agreement ∧ category = .nounPhrase agreement ∧
      surface = [Atom.word (AgreementInteractions.pronounText agreement)]
  verb head form voice frame :=
    head = .exile ∧ form = .pastParticiple ∧ voice = .passive ∧ frame = []
  verbForm head form surface :=
    (head = .be ∧ form = .preterite ∧
      (surface = (["was"] : Surface) ∨ surface = (["were"] : Surface))) ∨
    (head = .exile ∧ form = .pastParticiple ∧ surface = (["exiled"] : Surface))
  finite head _ form := head = .be ∧ form = .preterite
  auxiliary head form selected voice selectedVoice :=
    head = .be ∧ form = .preterite ∧ selected = .pastParticiple ∧
      voice = .passive ∧ selectedVoice = .passive

def pronoun (agreement : Agreement) : Witness Lexeme lexicon (.nounPhrase agreement) :=
  ⟨.word (.pronoun agreement) (.nounPhrase agreement),
   [Atom.word (AgreementInteractions.pronounText agreement)],
   .word .pronoun ⟨agreement, rfl, rfl⟩, .word ⟨agreement, rfl, rfl, rfl⟩⟩

def exiled : Witness Lexeme lexicon (.verbPhrase .pastParticiple .passive) :=
  ⟨.node (.verb .exile .pastParticiple [] .passive) [], ["exiled"],
   .verb ⟨rfl, rfl, rfl, rfl⟩ .nil,
   .node .nil (.verb (v := ["exiled"]) (Or.inr ⟨rfl, rfl, rfl⟩))⟩

def passive : Syntax Lexeme :=
  .node (.auxiliary .be .preterite .pastParticiple .positive .passive .passive) [exiled.tree]

def clause (agreement : Agreement) : Syntax Lexeme :=
  .node (.finite agreement .preterite .passive) [(pronoun agreement).tree, passive]

theorem clause_derives (agreement : Agreement) : Derives lexicon (clause agreement)
    (.clause .finite) :=
  .finite (pronoun agreement).derives
    (.node (.auxiliary ⟨rfl, rfl, rfl, rfl, rfl⟩) (.cons exiled.derives .nil))
    (.auxiliary ⟨rfl, rfl⟩) rfl

theorem finite_clause (agreement : Agreement) (words : Surface)
    (spelling : words = (["was"] : Surface) ∨ words = (["were"] : Surface)) :
    Admissible lexicon (clause agreement) (.clause .finite)
      ((pronoun agreement).surface ++ words ++ (["exiled"] : Surface)) :=
  ⟨clause_derives agreement,
    .node (.cons (pronoun agreement).realizes
      (.cons (.node (.cons exiled.realizes .nil)
        (.auxiliary (v := words) (Or.inl ⟨rfl, rfl, spelling⟩))) .nil)) .finite⟩

theorem singular_was : Admissible lexicon (clause ⟨.third, .singular⟩) (.clause .finite)
    ["it", "was", "exiled"] := finite_clause ⟨.third, .singular⟩ _ (Or.inl rfl)

theorem plural_were : Admissible lexicon (clause ⟨.third, .plural⟩) (.clause .finite)
    ["they", "were", "exiled"] := finite_clause _ _ (Or.inr rfl)

theorem addressee_were : Admissible lexicon (clause ⟨.second, .singular⟩) (.clause .finite)
    ["you", "were", "exiled"] := finite_clause _ _ (Or.inr rfl)

theorem expected_word_form_licenses :
    pastBe ⟨.third, .singular⟩ ["was"] ∧
    pastBe ⟨.third, .plural⟩ ["were"] ∧
    pastBe ⟨.second, .singular⟩ ["were"] := by
  simp [pastBe]

/-- These trees and surfaces are admitted today despite violating the lexical agreement license. -/
theorem wrong_addressee_was_admitted :
    Admissible lexicon (clause ⟨.second, .singular⟩) (.clause .finite)
      ["you", "was", "exiled"] ∧ ¬ pastBe ⟨.second, .singular⟩ ["was"] :=
  ⟨finite_clause _ _ (Or.inl rfl), by simp [pastBe]⟩

theorem wrong_singular_were_admitted :
    Admissible lexicon (clause ⟨.third, .singular⟩) (.clause .finite)
      ["it", "were", "exiled"] ∧ ¬ pastBe ⟨.third, .singular⟩ ["were"] :=
  ⟨finite_clause _ _ (Or.inr rfl), by simp [pastBe]⟩

theorem wrong_plural_was_admitted :
    Admissible lexicon (clause ⟨.third, .plural⟩) (.clause .finite)
      ["they", "was", "exiled"] ∧ ¬ pastBe ⟨.third, .plural⟩ ["was"] :=
  ⟨finite_clause _ _ (Or.inl rfl), by simp [pastBe]⟩

/-- A shared Concord Class does not imply a shared lexical agreement license. -/
theorem concord_does_not_determine_word_form :
    (⟨.first, .singular⟩ : Agreement).concord = (⟨.second, .singular⟩ : Agreement).concord ∧
    pastBe ⟨.first, .singular⟩ ["was"] ∧ ¬ pastBe ⟨.second, .singular⟩ ["was"] := by
  simp [pastBe, Agreement.concord]

/-- Both spellings realize the same VP tree; exact finite-host selection remains unmodeled. -/
theorem both_preterite_spellings : Realizes lexicon passive ["was", "exiled"] ∧
    Realizes lexicon passive ["were", "exiled"] :=
  ⟨.node (.cons exiled.realizes .nil)
      (.auxiliary (v := ["was"]) (Or.inl ⟨rfl, rfl, Or.inl rfl⟩)),
   .node (.cons exiled.realizes .nil)
      (.auxiliary (v := ["were"]) (Or.inl ⟨rfl, rfl, Or.inr rfl⟩))⟩

end English.WordFormInteractions
