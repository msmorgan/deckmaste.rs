import English.NominalWitnesses

namespace English.NotationWitnesses
open LexicalWitnesses GrammarWitnesses FamilyWitnesses AmbiguityWitnesses

def scalar := word .two (.word .measurePhrase) ["2"]
def plus := word .plus .measure ["plus"]
def sum : Reading Lexeme := .node (.measure plus) [.word scalar .measurePhrase, .word scalar .measurePhrase]
def golem := word .creatureSubtype (.word (.document .subtype)) ["Golem"]
def typeLine : Reading Lexeme := .node (.document .subtypedLine)
  [.node (.document .supertypes) [], .node (.document .types) [control],
    .node (.document .subtypes) [.word golem (.document .subtype)]]

@[simp] theorem scalar_licensed : scalar.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem plus_licensed : plus.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem golem_licensed : golem.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem measure_admitted : Reading.Admitted environment [] sum .measurePhrase ["2", "plus", "2"] := by
  refine ⟨⟨.node (.measure (lexicon := grammar) ⟨plus_licensed, rfl⟩)
    (.cons (.word (lexicon := grammar) .measure ⟨scalar_licensed, Or.inl ⟨_, rfl⟩⟩)
      (.cons (.word (lexicon := grammar) .measure ⟨scalar_licensed, Or.inl ⟨_, rfl⟩⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨scalar_licensed, Or.inl ⟨_, rfl⟩, rfl⟩)
      (.cons (.word (lexicon := grammar) ⟨scalar_licensed, Or.inl ⟨_, rfl⟩, rfl⟩) .nil))
      (.measure (lexicon := grammar) ⟨plus_licensed, rfl⟩)⟩
  · simp [sum, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [sum, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · refine ⟨?_, ?_⟩
    · simp [sum, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem measure_not_a_count_determiner :
    ¬ grammar.word scalar (.cardinalNumeral .plural) := by
  simp [grammar, Lexical.lexicon, scalar, word]

theorem type_line_admitted : Reading.Admitted environment [] typeLine (.document .typeLine)
    ["Artifact", "—", "Golem"] := by
  refine ⟨⟨.node (.document .subtypedLine)
    (.cons (.node (.document (.supertypes (n := 0))) .nil)
      (.cons (.node (.document (.types (n := 0))) (.cons control_admitted.1.1 .nil))
        (.cons (.node (.document (.subtypes (n := 0)))
          (.cons (.word (lexicon := grammar) .subtype ⟨golem_licensed, Or.inl ⟨_, rfl⟩⟩) .nil)) .nil))),
      ?_, ?_, ?_⟩,
    .node (.cons (.node .nil (.document .supertypes))
      (.cons (.node (.cons control_admitted.2 .nil) (.document .types))
        (.cons (.node (.cons (.word (lexicon := grammar) ⟨golem_licensed, Or.inl ⟨_, rfl⟩, rfl⟩) .nil)
          (.document .subtypes)) .nil))) (.document .subtypedLine)⟩
  · simp [typeLine, AmbiguityWitnesses.control, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [typeLine, AmbiguityWitnesses.control, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local,
      Dependencies.exposed, Dependencies.childrenExposed]
  · refine ⟨?_, ?_⟩
    · simp [typeLine, AmbiguityWitnesses.control, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem type_line_requires_types :
    ¬ DocumentProduction .subtypedLine [.document .supertypes, .document .subtypes]
      (.document .typeLine) := by intro h; cases h

/-- `Documents.type_line_order` states the same exclusion in that fixture namespace; see the note
there. -/
theorem type_line_order :
    ¬ DocumentProduction .subtypedLine [.document .subtypes, .document .types, .document .supertypes]
      (.document .typeLine) := by intro h; cases h

def byWord := word .by_ (.preposition .measurePhrase) ["by"]
def measuredPreposition : Reading Lexeme := .node (.preposition byWord .measurePhrase) [sum]

@[simp] theorem by_licensed : byWord.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem measure_preposition_interaction : Reading.Admitted environment [] measuredPreposition
    .prepositionPhrase ["by", "2", "plus", "2"] := by
  refine ⟨⟨.node (.preposition ⟨by_licensed, rfl⟩) (.cons measure_admitted.1.1 .nil), ?_, ?_, ?_⟩,
    .node (.cons measure_admitted.2 .nil) (.preposition (lexicon := grammar) ⟨by_licensed, rfl⟩)⟩
  · simp [measuredPreposition, Features.Conforms, Features.ChildrenConform, Features.Local, CaseAt]
    exact measure_admitted.1.2.1
  · simp [measuredPreposition, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
    exact measure_admitted.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simp [measuredPreposition, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
      exact measure_admitted.1.2.2.2.1
    · decide


end English.NotationWitnesses
