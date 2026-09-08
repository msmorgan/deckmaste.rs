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
  · refine ⟨?_, ?_⟩
    · simp [tree, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · cases variant <;> cases capitalization <;> decide

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

/-- A source-independent construction relation over this fixture: the three declared values at
their one category. It is not defined by, and does not consult, any parsing output. -/
def constructs (context : List Category) (value : Reading Unit) (category : Category) : Prop :=
  context = [] ∧ category = .nounPhrase ⟨.third, .plural⟩ ∧
    (value = tree false ∨ value = tree true ∨ value = tree true .initial)

private theorem realizes_surface {variant : Bool} {capitalization : Capitalization}
    {surface : Surface}
    (realized : Reading.Realizes (Lexical.lexicon environment) (tree variant capitalization)
      surface) : surface = (word variant capitalization).surface := by
  cases realized with
  | node realizeChildren linearization =>
    cases linearization with
    | barePlural =>
      cases realizeChildren with
      | cons head _ => cases head with
        | noun form => exact form.2.2

/-- The value roundtrip obligation forces variant and capitalization retention. Any operation
satisfying it on this fixture answers each declared value with that value's own exact surface, so
the three values cannot be collapsed onto one rendering. -/
theorem value_roundtrip_forces_variants (render : Reading Unit → Option Surface)
    (roundtrip : Reading.ValueRoundtrip environment constructs render) :
    render (tree false) = some (["indexes"] : Surface) ∧
    render (tree true) = some (["indices"] : Surface) ∧
    render (tree true .initial) = some (["Indices"] : Surface) := by
  have pinned (variant : Bool) (capitalization : Capitalization)
      (declared : constructs [] (tree variant capitalization) (.nounPhrase ⟨.third, .plural⟩)) :
      render (tree variant capitalization) = some (word variant capitalization).surface := by
    obtain ⟨_, surface, rendered, admitted⟩ :=
      roundtrip [] (tree variant capitalization) (.nounPhrase ⟨.third, .plural⟩) declared
    rw [rendered, realizes_surface admitted.2]
  exact ⟨pinned false .declared ⟨rfl, rfl, Or.inl rfl⟩,
    pinned true .declared ⟨rfl, rfl, Or.inr (Or.inl rfl)⟩,
    pinned true .initial ⟨rfl, rfl, Or.inr (Or.inr rfl)⟩⟩

/-- A concrete wrong operation the obligation refutes: one that answers the declaration's default
spelling for every value, dropping the selected variant. -/
theorem value_roundtrip_rejects_variant_blind :
    ¬ Reading.ValueRoundtrip environment constructs (fun _ ↦ some (["indexes"] : Surface)) := by
  intro roundtrip
  have wrong := (value_roundtrip_forces_variants _ roundtrip).2.1
  simp at wrong

/-- An operation that reads the value's selected spelling but drops its selected capitalization.
It answers correctly for both declared-case values and is still refuted, so capitalization is
carried by the law and not only by the variant. -/
def spellingOnly : Reading Unit → Option Surface
  | .node .barePlural [.noun w _] => some w.spelling
  | _ => none

theorem value_roundtrip_rejects_case_blind :
    ¬ Reading.ValueRoundtrip environment constructs spellingOnly := by
  intro roundtrip
  have wrong := (value_roundtrip_forces_variants _ roundtrip).2.2
  simp [spellingOnly, tree, word] at wrong

def overridden : Morphology := {
  defaultForm := fun _ ↦ ["indexes"]
  overrides := fun _ ↦ some [["indices"]] }

theorem override_does_not_add_default :
    (["indexes"] : Surface) ∉ overridden.forms (.noun .plural .count) := by decide

end English.SpellingWitnesses
