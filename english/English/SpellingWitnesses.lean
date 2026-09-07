import English.NotationWitnesses

namespace English.SpellingWitnesses

def declaration : LexemeDeclaration Unit where
  lemma := ["index"]
  provenance := "synthetic-spelling-variants"
  bundles := [.noun .plural .count]
  morphology := {
    defaultForm := fun _ ↦ ["indexes"]
    overrides := fun _ ↦ some [["indexes"], ["indices"]] }

def environment : LexicalEnvironment Unit := fun _ ↦ some declaration

def word (variant : Bool) (capitalization : Capitalization := .declared) : WordForm Unit :=
  ⟨(), .noun .plural .count, if variant then ["indices"] else ["indexes"], capitalization,
    "synthetic-spelling-variants"⟩

theorem licensed (variant : Bool) (capitalization : Capitalization) :
    (word variant capitalization).Licensed environment := by
  cases variant <;> cases capitalization <;>
    simp [WordForm.Licensed, LexicalAnalysis, word, environment, declaration, Morphology.forms]
  all_goals decide

def tree (variant : Bool) (capitalization : Capitalization := .declared) : Reading Unit :=
  .node .barePlural [.noun (word variant capitalization) .plural]

theorem admitted (variant : Bool) (capitalization : Capitalization) :
    Reading.Admitted environment [] (tree variant capitalization) (.nounPhrase ⟨.third, .plural⟩)
      (word variant capitalization).surface := by
  refine ⟨⟨.node .barePlural (.cons (.noun (lexicon := Lexical.lexicon environment)
    ⟨licensed variant capitalization, .count, rfl⟩) .nil), ?_, ?_, ?_⟩,
    .node (.cons (.noun (lexicon := Lexical.lexicon environment)
      ⟨licensed variant capitalization, ⟨.count, rfl⟩, rfl⟩) .nil) .barePlural⟩
  · simp [tree, Features.Conforms, Features.ChildrenConform, Features.Local]
    exact .noun ⟨.plural, rfl⟩
  · simp [tree, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [tree, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem variants_preserved :
    Reading.Realizes (Lexical.lexicon environment) (tree false) ["indexes"] ∧
    Reading.Realizes (Lexical.lexicon environment) (tree true) ["indices"] ∧
    Reading.Realizes (Lexical.lexicon environment) (tree true .initial) ["Indices"] :=
  ⟨(admitted false .declared).2, (admitted true .declared).2, (admitted true .initial).2⟩

theorem variant_values_distinct : tree false ≠ tree true := by
  simp [tree, word]

theorem capitalization_values_distinct : tree true .declared ≠ tree true .initial := by
  intro h
  cases h

def overridden : Morphology := {
  defaultForm := fun _ ↦ ["indexes"]
  overrides := fun _ ↦ some [["indices"]] }

theorem override_does_not_add_default :
    (["indexes"] : Surface) ∉ overridden.forms (.noun .plural .count) := by decide

end English.SpellingWitnesses
