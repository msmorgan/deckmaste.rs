import English.GrammarWitnesses

/-! The former crossed-word-form counterexamples are repaired at actual grammatical admission. -/
namespace English.WordFormInteractions
open LexicalWitnesses GrammarWitnesses

def pastBe (agreement : Agreement) (surface : Surface) : Prop :=
  (agreement.person ≠ .second ∧ agreement.number = .singular ∧ surface = (["was"] : Surface)) ∨
  ((agreement.person = .second ∨ agreement.number = .plural) ∧ surface = (["were"] : Surface))

theorem singular_was : Reading.Admitted environment [] (clause it was singular)
    (.clause .finite) ["it", "was", "exiled"] := GrammarWitnesses.singular_was

theorem plural_were : Reading.Admitted environment [] (clause they were plural)
    (.clause .finite) ["they", "were", "exiled"] := GrammarWitnesses.plural_were

theorem addressee_were : Reading.Admitted environment [] (clause you wereYou addressee)
    (.clause .finite) ["you", "were", "exiled"] := GrammarWitnesses.addressee_were

theorem expected_word_form_licenses :
    pastBe ⟨.third, .singular⟩ ["was"] ∧ pastBe ⟨.third, .plural⟩ ["were"] ∧
    pastBe ⟨.second, .singular⟩ ["were"] := by simp [pastBe]

theorem wrong_addressee_was_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause you was addressee) (.clause .finite) surface :=
  GrammarWitnesses.wrong_addressee_was_excluded surface

theorem wrong_singular_were_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause it were singular) (.clause .finite) surface :=
  GrammarWitnesses.wrong_singular_were_excluded surface

theorem wrong_plural_was_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause they was plural) (.clause .finite) surface :=
  GrammarWitnesses.wrong_plural_was_excluded surface

theorem concord_does_not_determine_word_form :
    (⟨.first, .singular⟩ : Agreement).concord = (⟨.second, .singular⟩ : Agreement).concord ∧
    pastBe ⟨.first, .singular⟩ ["was"] ∧ ¬ pastBe ⟨.second, .singular⟩ ["was"] := by
  simp [pastBe, Agreement.concord]

/-- Different selected finite Word Forms require different grammatical values. -/
theorem distinct_passive_word_forms : passive was ≠ passive were := by
  intro h
  cases h

end English.WordFormInteractions
