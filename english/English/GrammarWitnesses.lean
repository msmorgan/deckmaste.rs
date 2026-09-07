import English.LexicalWitnesses

/-! Complete admissions use the lexical environment, composition, feature and dependency judgments. -/
namespace English.GrammarWitnesses
open LexicalWitnesses

abbrev grammar := Lexical.lexicon environment

def exiledTree : Reading Lexeme := .node (.verb exiled .pastParticiple [] .passive) []
def passive (auxiliary : WordForm Lexeme) : Reading Lexeme :=
  .node (.auxiliary auxiliary .preterite .pastParticiple .positive .passive .passive) [exiledTree]
def clause (subject auxiliary : WordForm Lexeme) (agreement : Agreement) : Reading Lexeme :=
  .node (.finite agreement .preterite .passive)
    [.word subject (.nounPhrase agreement), passive auxiliary]

theorem exiled_derives : Derives grammar exiledTree (.verbPhrase .pastParticiple .passive) := by
  apply Judges.verb
  · exact ⟨exiled_licensed, ⟨none, rfl⟩, declaration .exile, rfl, Or.inl ⟨rfl, rfl, rfl, rfl⟩⟩
  · exact .nil

theorem passive_derives (auxiliary : WordForm Lexeme) (agreement : Agreement)
    (licensed : auxiliary.Licensed environment) (head : auxiliary.lexeme = .be)
    (bundle : auxiliary.bundle = .verb .preterite (some (.past, agreement))) :
    Derives grammar (passive auxiliary) (.verbPhrase .preterite .passive) := by
  refine .node (.auxiliary ⟨licensed, ⟨_, bundle⟩, ?_⟩) (.cons exiled_derives .nil)
  exact ⟨declaration .be, by simp [environment, head], rfl, rfl, rfl, rfl, rfl⟩

theorem clause_derives (subject auxiliary : WordForm Lexeme) (agreement : Agreement)
    (subjectLicensed : subject.Licensed environment)
    (subjectBundle : subject.bundle = .word (.nounPhrase agreement))
    (auxiliaryLicensed : auxiliary.Licensed environment) (head : auxiliary.lexeme = .be)
    (bundle : auxiliary.bundle = .verb .preterite (some (.past, agreement))) :
    Derives grammar (clause subject auxiliary agreement) (.clause .finite) :=
  JudgesIn.finite (lexicon := grammar)
    (.word .pronoun ⟨subjectLicensed, Or.inl ⟨_, subjectBundle⟩⟩)
    (passive_derives auxiliary agreement auxiliaryLicensed head bundle)
    (.auxiliary ⟨auxiliaryLicensed, .past, bundle⟩) rfl

theorem clause_valid (subject auxiliary : WordForm Lexeme) (agreement : Agreement)
    (subjectLicensed : subject.Licensed environment)
    (subjectBundle : subject.bundle = .word (.nounPhrase agreement))
    (auxiliaryLicensed : auxiliary.Licensed environment) (head : auxiliary.lexeme = .be)
    (bundle : auxiliary.bundle = .verb .preterite (some (.past, agreement))) :
    Reading.Valid environment [] (clause subject auxiliary agreement) (.clause .finite) := by
  refine ⟨clause_derives subject auxiliary agreement subjectLicensed subjectBundle
    auxiliaryLicensed head bundle, ?_, ?_, ?_⟩
  · simp [clause, passive, exiledTree, Features.Conforms, Features.ChildrenConform, Features.Local,
      Syntax.nominalCase, Lexical.features, subjectBundle, Case.Allows, Case.Argument, FrameCases]
  · simp [clause, passive, exiledTree, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.exposed]
  · simp only [clause, passive, exiledTree, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar, and_true, true_and]
    exact .verb rfl

theorem clause_realizes (subject auxiliary : WordForm Lexeme) (agreement : Agreement)
    (subjectLicensed : subject.Licensed environment)
    (subjectBundle : subject.bundle = .word (.nounPhrase agreement))
    (auxiliaryLicensed : auxiliary.Licensed environment)
    (bundle : auxiliary.bundle = .verb .preterite (some (.past, agreement))) :
    Reading.Realizes grammar (clause subject auxiliary agreement)
      (subject.surface ++ (auxiliary.surface ++ (exiled.surface ++ []))) :=
  .node (.cons (.word ⟨subjectLicensed, Or.inl ⟨_, subjectBundle⟩, rfl⟩)
    (.cons (.node (.cons (.node .nil (.verb ⟨exiled_licensed, ⟨none, rfl⟩, rfl⟩)) .nil)
      (.auxiliary ⟨auxiliaryLicensed, ⟨_, bundle⟩, rfl⟩)) .nil)) .finite

theorem singular_was : Reading.Admitted environment [] (clause it was singular)
    (.clause .finite) ["it", "was", "exiled"] :=
  ⟨clause_valid _ _ _ it_licensed rfl was_licensed rfl rfl,
    clause_realizes _ _ _ it_licensed rfl was_licensed rfl⟩

theorem plural_were : Reading.Admitted environment [] (clause they were plural)
    (.clause .finite) ["they", "were", "exiled"] :=
  ⟨clause_valid _ _ _ they_licensed rfl were_licensed rfl rfl,
    clause_realizes _ _ _ they_licensed rfl were_licensed rfl⟩

theorem addressee_were : Reading.Admitted environment [] (clause you wereYou addressee)
    (.clause .finite) ["you", "were", "exiled"] :=
  ⟨clause_valid _ _ _ you_licensed rfl wereYou_licensed rfl rfl,
    clause_realizes _ _ _ you_licensed rfl wereYou_licensed rfl⟩

theorem finite_tense_is_past : Reading.HasTense (clause it was singular) .past :=
  .clause (.auxiliary rfl)

/-- A nonfinite lexical use carries no Tense at all: the exclusion `HasTense` was missing. -/
theorem no_tense_on_nonfinite (t : Tense) : ¬ Reading.HasTense exiledTree t := by
  intro h
  cases h with
  | verb bundle => simp [exiled, word] at bundle

/-- Finiteness is what the auxiliary's complement position reads, not Word Form or Tense. -/
theorem finite_is_not_nonfinite_use : ¬ Reading.NonfiniteUse (passive was) := by
  intro h
  cases h with
  | auxiliary bundle => simp [was, word] at bundle

/-- One Word Form and one Inflectional Form, two Finiteness values: the dimensions separate. -/
theorem plain_form_both_finite_and_nonfinite :
    attack.bundle = .verb .plain none ∧
    attackPlural.bundle = .verb .plain (some (.present, plural)) ∧
    attack.Licensed environment ∧ attackPlural.Licensed environment ∧
    attack.spelling = attackPlural.spelling :=
  ⟨rfl, rfl, attack_licensed, attackPlural_licensed, rfl⟩

/-- The preterite Inflectional Form correlates with past Tense but is a distinct field: the same
admitted tree satisfies `HasTense … .past` and refutes `HasTense … .present`. -/
theorem preterite_is_not_tense :
    was.bundle = .verb .preterite (some (.past, singular)) ∧
    Reading.HasTense (clause it was singular) .past ∧
    ¬ Reading.HasTense (clause it was singular) .present := by
  refine ⟨rfl, finite_tense_is_past, ?_⟩
  intro h
  cases h with
  | clause inner => cases inner with
    | auxiliary bundle => simp [was, word, singular] at bundle

theorem wrong_plural_was_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause they was plural) (.clause .finite) surface := by
  intro admitted
  have derivation := admitted.1.1
  change JudgesIn grammar [] _ _ [] at derivation
  generalize gapEq : ([] : List Category) = gaps at derivation
  cases derivation with
  | finite _ _ finite agreement =>
    cases agreement
    cases finite with
    | auxiliary license =>
      obtain ⟨_, tense, wrong⟩ := license
      simp [was, word, singular, plural] at wrong
  | node production _ => cases production

theorem wrong_singular_were_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause it were singular) (.clause .finite) surface := by
  intro admitted
  have derivation := admitted.1.1
  change JudgesIn grammar [] _ _ [] at derivation
  generalize gapEq : ([] : List Category) = gaps at derivation
  cases derivation with
  | finite _ _ finite agreement =>
    cases agreement
    cases finite with
    | auxiliary license =>
      obtain ⟨_, tense, wrong⟩ := license
      simp [were, word, singular, plural] at wrong
  | node production _ => cases production

theorem wrong_addressee_was_excluded (surface : Surface) :
    ¬ Reading.Admitted environment [] (clause you was addressee) (.clause .finite) surface := by
  intro admitted
  have derivation := admitted.1.1
  change JudgesIn grammar [] _ _ [] at derivation
  generalize gapEq : ([] : List Category) = gaps at derivation
  cases derivation with
  | finite _ _ finite agreement =>
    cases agreement
    cases finite with
    | auxiliary license =>
      obtain ⟨_, tense, wrong⟩ := license
      simp [was, word, singular, addressee] at wrong
  | node production _ => cases production

end English.GrammarWitnesses
