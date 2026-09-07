import English.GrammarWitnesses

/-! One connected environment exercises the grammar families and their composition. -/
namespace English.FamilyWitnesses
open LexicalWitnesses GrammarWitnesses

private theorem declared_word (head : Lexeme) (bundle : FeatureBundle) (surface : Surface)
    (known : head ≠ .unknown) (member : (bundle, surface) ∈ rows head) :
    (word head bundle surface).Licensed environment := row_licensed _ _ _ _ (Or.inl rfl) known member

def noun : Reading Lexeme := .noun creatures .plural
def objects : Reading Lexeme := .node .barePlural [noun]
def white := word .white .adjective ["white"]
def adjective : Reading Lexeme := .adjective white
def modified : Reading Lexeme := .modify adjective noun
def turns := word .turn (.noun .plural .count) ["turns"]
def temporal : Reading Lexeme := .node .barePlural [.noun turns .plural]
def during := word .during (.preposition (.nounPhrase plural)) ["during"]
def preposition : Reading Lexeme := .node (.preposition during (.nounPhrase plural)) [temporal]
def predicate : Reading Lexeme := .node (.verb attackPlural .plain []) []
def instruction : Reading Lexeme := .node .imperative [.node (.verb attack .plain []) []]
def finite : Reading Lexeme := .node (.finite plural .plain) [objects, predicate]
def conditioned := word .if_ (.subordinator .finite) ["if"] .initial
def subordinate : Reading Lexeme := .node (.subordinate conditioned .finite) [finite]
def conditional : Reading Lexeme := .node (.initialAdverbial (.subordinateClause .finite))
  [subordinate, instruction]
def sentence : Reading Lexeme := .node (.document .sentence) [conditional]
def document : Reading Lexeme := .node (.document .document)
  [.node (.document .ordinary) [.node (.document .body) [sentence]]]

@[simp] theorem white_licensed : white.Licensed environment :=
  declared_word _ _ _ (by decide) (by simp [rows])
@[simp] theorem turns_licensed : turns.Licensed environment :=
  declared_word _ _ _ (by decide) (by simp [rows])
@[simp] theorem during_licensed : during.Licensed environment :=
  declared_word _ _ _ (by decide) (by simp [rows])
@[simp] theorem conditioned_licensed : conditioned.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

@[simp] theorem noun_use : Features.NominalUse (Lexical.features environment) noun .count :=
  .noun ⟨.plural, rfl⟩
@[simp] theorem temporal_use :
    Features.NominalUse (Lexical.features environment) (.noun turns .plural) .count :=
  .noun ⟨.plural, rfl⟩

theorem noun_derives : Derives grammar noun (.nominal .plural) :=
  .noun ⟨creatures_licensed, .count, rfl⟩
theorem objects_derives : Derives grammar objects (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) .barePlural (.cons noun_derives .nil)
theorem adjective_derives : Derives grammar adjective .adjectivePhrase :=
  .adjective ⟨white_licensed, rfl⟩
theorem modified_derives : Derives grammar modified (.nominal .plural) :=
  .modify adjective_derives noun_derives
theorem temporal_derives : Derives grammar temporal (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) .barePlural (.cons (.noun (lexicon := grammar) ⟨turns_licensed, .count, rfl⟩) .nil)
theorem preposition_derives : Derives grammar preposition .prepositionPhrase :=
  .node (.preposition (lexicon := grammar) ⟨during_licensed, rfl⟩) (.cons temporal_derives .nil)
theorem predicate_derives : Derives grammar predicate (.verbPhrase .plain) :=
  .verb ⟨attackPlural_licensed, ⟨_, rfl⟩,
    declaration .attack, rfl, Or.inr (Or.inl ⟨rfl, rfl, rfl⟩)⟩ .nil

theorem instruction_derives : Derives grammar instruction (.clause .finite) :=
  JudgesIn.node (lexicon := grammar) .imperative (.cons (.verb (lexicon := grammar) ⟨attack_licensed, ⟨_, rfl⟩,
    declaration .attack, rfl, Or.inr (Or.inl ⟨rfl, rfl, rfl⟩)⟩ .nil) .nil)

theorem finite_derives : Derives grammar finite (.clause .finite) :=
  .finite objects_derives predicate_derives (.verb (lexicon := grammar) ⟨attackPlural_licensed, .present, rfl⟩) rfl

theorem subordinate_derives : Derives grammar subordinate (.subordinateClause .finite) :=
  .node (.subordinate (lexicon := grammar) ⟨conditioned_licensed, rfl⟩) (.cons finite_derives .nil)
theorem conditional_derives : Derives grammar conditional (.clause .finite) :=
  .node (.initialAdverbial .subordinate) (.cons subordinate_derives (.cons instruction_derives .nil))
theorem document_derives : Derives grammar document (.document .document) :=
  .node (.document (.document rfl)) (.cons (.node (.document .ordinary)
    (.cons (.node (.document (.body rfl))
      (.cons (.node (.document .sentence) (.cons conditional_derives .nil)) .nil)) .nil)) .nil)

theorem noun_realizes : Reading.Realizes grammar noun ["creatures"] :=
  .noun ⟨creatures_licensed, ⟨.count, rfl⟩, rfl⟩
theorem objects_realizes : Reading.Realizes grammar objects ["creatures"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons noun_realizes .nil) .barePlural
theorem adjective_realizes : Reading.Realizes grammar adjective ["white"] :=
  .adjective ⟨white_licensed, rfl, rfl⟩
theorem temporal_realizes : Reading.Realizes grammar temporal ["turns"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons (.noun (lexicon := grammar) ⟨turns_licensed, ⟨.count, rfl⟩, rfl⟩) .nil) .barePlural
theorem preposition_realizes : Reading.Realizes grammar preposition ["during", "turns"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons temporal_realizes .nil) (.preposition (lexicon := grammar) ⟨during_licensed, rfl⟩)
theorem predicate_realizes : Reading.Realizes grammar predicate ["attack"] :=
  Reading.Realizes.node (lexicon := grammar) .nil (.verb (lexicon := grammar) ⟨attackPlural_licensed, ⟨_, rfl⟩, rfl⟩)
theorem instruction_realizes : Reading.Realizes grammar instruction ["attack"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons (.node .nil (.verb (lexicon := grammar) ⟨attack_licensed, ⟨_, rfl⟩, rfl⟩)) .nil) .imperative
theorem finite_realizes : Reading.Realizes grammar finite ["creatures", "attack"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons objects_realizes (.cons predicate_realizes .nil)) .finite
theorem subordinate_realizes : Reading.Realizes grammar subordinate ["If", "creatures", "attack"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons finite_realizes .nil) (.subordinate (lexicon := grammar) ⟨conditioned_licensed, rfl⟩)
theorem conditional_realizes :
    Reading.Realizes grammar conditional ["If", "creatures", "attack", .closing ",", "attack"] :=
  Reading.Realizes.node (lexicon := grammar) (.cons subordinate_realizes (.cons instruction_realizes .nil)) .initialAdverbial

theorem document_realizes : Reading.Realizes grammar document
    ["If", "creatures", "attack", .closing ",", "attack", .closing "."] :=
  Reading.Realizes.node (lexicon := grammar) (.cons (.node (.cons (.node (.cons (.node (.cons conditional_realizes .nil)
    (.document (.sentence rfl))) .nil) (.document .body)) .nil) (.document .ordinary)) .nil)
    (.document .document)

theorem document_admitted : Reading.Admitted environment [] document (.document .document)
    ["If", "creatures", "attack", .closing ",", "attack", .closing "."] := by
  refine ⟨⟨document_derives, ?_, ?_, ?_⟩, document_realizes⟩
  · simp [document, sentence, conditional, subordinate, finite, objects, instruction, predicate,
      Features.Conforms, Features.ChildrenConform, Features.Local, noun,
      Syntax.nominalCase, FrameCases, Case.Allows]
    exact noun_use
  · simp [document, sentence, conditional, subordinate, finite, objects, instruction, predicate, noun,
      Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local, Dependencies.exposed,
      Dependencies.childrenExposed, Dependencies.frameExposed]
  · simp [document, sentence, conditional, subordinate, finite, objects, instruction, predicate, noun,
      Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    exact .verb rfl

/-- The same independently declared adjective participates in a nominal used as a verb Object. -/
theorem modified_admitted : Reading.Admitted environment [] modified (.nominal .plural)
    ["white", "creatures"] := by
  refine ⟨⟨modified_derives, ?_, ?_, ?_⟩, .modify adjective_realizes noun_realizes⟩
  · simp [modified, adjective, noun, Features.Conforms, Features.Local,
      Features.containsTarget]
  · simp [modified, adjective, noun, Dependencies.Safe, Dependencies.Local,
      Dependencies.exposed]
  · simp [modified, adjective, noun, Reading.GrammarConforms, Reading.LocalGrammar]

theorem preposition_admitted : Reading.Admitted environment [] preposition .prepositionPhrase
    ["during", "turns"] := by
  refine ⟨⟨preposition_derives, ?_, ?_, ?_⟩, preposition_realizes⟩
  · simp [preposition, temporal, Features.Conforms, Features.ChildrenConform, Features.Local,
      temporal_use, CaseAt, Syntax.nominalCase, Case.Allows, Relation.casePosition]
  · simp [preposition, temporal, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [preposition, temporal, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]

theorem wrong_preposition_complement : ¬ grammar.preposition during (.clause .finite) := by
  simp [grammar, Lexical.lexicon, during, word]

theorem wrong_subordinate_finiteness : ¬ grammar.subordinator conditioned .nonfinite := by
  simp [grammar, Lexical.lexicon, conditioned, word]

theorem document_rejects_bare_nominal :
    ¬ DocumentProduction .document [.nominal .plural] (.document .document) := by
  intro h
  cases h with
  | document h => cases h

end English.FamilyWitnesses
