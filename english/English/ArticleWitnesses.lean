import English.Readings

namespace English.ArticleWitnesses

inductive Lexeme where
  | article | creature | artifact | unusual | uniform
  deriving DecidableEq

def spelling : Lexeme → Surface
  | .article => ["a"]
  | .creature => ["creature"]
  | .artifact => ["artifact"]
  | .unusual => ["unusual"]
  | .uniform => ["uniform"]

def bundle : Lexeme → FeatureBundle
  | .article => .determinative .singular .count
  | .creature | .artifact => .noun .singular .count
  | .unusual | .uniform => .adjective

def environment : LexicalEnvironment Lexeme := fun head ↦ some {
  lemma := spelling head
  provenance := "article-witness"
  bundles := [bundle head]
  morphology := {
    defaultForm := fun _ ↦ spelling head
    overrides := fun _ ↦ if head = .article then some [["a"], ["an"]] else none }
  onset := fun _ ↦ some (match head with
    | .artifact | .unusual => .vowel
    | _ => .consonant)
  articleOnset := fun surface ↦ if head = .article then
    if surface = (["a"] : Surface) then some .consonant else some .vowel else none
}

def word (head : Lexeme) : WordForm Lexeme :=
  ⟨head, bundle head, spelling head, .declared, "article-witness"⟩

def article (vowel : Bool) : WordForm Lexeme :=
  { word .article with spelling := if vowel then ["an"] else ["a"] }

def phrase (vowel : Bool) : Reading Lexeme :=
  .node (.determine .singular)
    [.word (article vowel) (.determinativePhrase .singular),
     .noun (word (if vowel then .artifact else .creature)) .singular]

theorem article_licensed (vowel : Bool) : (article vowel).Licensed environment := by
  cases vowel <;> simp [WordForm.Licensed, LexicalAnalysis, environment, article, word,
    spelling, bundle, Morphology.forms, WordForm.surface, Capitalization.apply,
    Surface.WellFormed, Atom.WellFormed] <;> decide

theorem noun_licensed (vowel : Bool) :
    (word (if vowel then .artifact else .creature)).Licensed environment := by
  cases vowel <;> simp [WordForm.Licensed, LexicalAnalysis, environment, word,
    spelling, bundle, Morphology.forms, WordForm.surface, Capitalization.apply,
    Surface.WellFormed, Atom.WellFormed] <;> decide

theorem phrase_admitted (vowel : Bool) :
    Reading.Admitted environment [] (phrase vowel) (.nounPhrase ⟨.third, .singular⟩)
      (if vowel then ["an", "artifact"] else ["a", "creature"]) := by
  have det := article_licensed vowel
  have noun := noun_licensed vowel
  have nounBundle : (word (if vowel then .artifact else .creature)).bundle =
      .noun .singular .count := by cases vowel <;> rfl
  refine ⟨⟨.node .determine (.cons
      (.word (lexicon := Lexical.lexicon environment) .determinative ⟨det, Or.inr ⟨_, _, rfl, rfl⟩⟩)
      (.cons (.noun (lexicon := Lexical.lexicon environment) ⟨noun, .count, nounBundle⟩) .nil)), ?_, ?_, ?_⟩, ?_⟩
  · have nounUse : Features.NominalUse (Lexical.features environment)
        (.noun (word (if vowel then .artifact else .creature)) .singular) .count :=
      .noun ⟨_, nounBundle⟩
    have detUse : Features.DeterminerUse (Lexical.features environment)
        (.word (article vowel) (.determinativePhrase .singular)) .count := .word ⟨_, rfl⟩
    cases vowel <;> simpa [phrase, Features.Conforms, Features.ChildrenConform, Features.Local,
      Features.requiredOnset, Features.leadingOnset, Lexical.features, environment,
      article, word, spelling] using And.intro nounUse detUse
  · simp [phrase, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [phrase, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar, article, word]
  · have realized := Reading.Realizes.node
      (.cons (Reading.Realizes.word (lexicon := Lexical.lexicon environment) ⟨det, Or.inr ⟨_, _, rfl, rfl⟩, rfl⟩)
        (.cons (Reading.Realizes.noun (lexicon := Lexical.lexicon environment) ⟨noun, ⟨.count, nounBundle⟩, rfl⟩) .nil))
      (Reading.Linearizes.determine (number := .singular))
    cases vowel <;> exact realized

theorem wrong_onset_excluded (vowel : Bool) (surface : Surface) :
    ¬ Reading.Admitted environment []
      (.node (.determine .singular)
        [.word (article vowel) (.determinativePhrase .singular),
         .noun (word (if vowel then .creature else .artifact)) .singular])
      (.nounPhrase ⟨.third, .singular⟩) surface := by
  intro admitted
  have hlocal := admitted.1.2.1.1
  cases vowel <;> simp [Features.Local, Features.requiredOnset, Features.leadingOnset,
    Lexical.features, environment, article, word, spelling] at hlocal

theorem first_modifier_controls_onset :
    Features.leadingOnset (Lexical.features environment)
      (.modify (.adjective (word .unusual)) (.noun (word .creature) .singular)) = some .vowel ∧
    Features.leadingOnset (Lexical.features environment)
      (.modify (.adjective (word .uniform)) (.noun (word .artifact) .singular)) = some .consonant := by
  decide

theorem quoted_keyword_case_is_independent_of_period (period : Bool) :
    Casing.valid (.node (.document (.quoteKeyword period))
      [.word (word .creature) .keywordPhrase]) ∧
    ¬ Casing.valid (.node (.document (.quoteKeyword period))
      [.word { word .creature with capitalization := .initial } .keywordPhrase]) := by
  simp [word, spelling]

theorem sentence_requires_initial :
    Casing.valid (.node (.document .sentence)
      [.word { word .creature with capitalization := .initial } .keywordPhrase]) ∧
    ¬ Casing.valid (.node (.document .sentence)
      [.word (word .creature) .keywordPhrase]) := by
  decide

end English.ArticleWitnesses
