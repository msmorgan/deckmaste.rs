import English.GrammarWitnesses
import English.FrameScope

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
    (.cons (.paragraph (.body rfl)
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
  · refine ⟨?_, ?_⟩
    · simp [document, sentence, conditional, subordinate, finite, objects, instruction, predicate, noun,
        Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
      exact ⟨.verb rfl, rfl⟩
    · decide

/-- The same independently declared adjective participates in a nominal used as a verb Object. -/
theorem modified_admitted : Reading.Admitted environment [] modified (.nominal .plural)
    ["white", "creatures"] := by
  refine ⟨⟨modified_derives, ?_, ?_, ?_⟩, .modify adjective_realizes noun_realizes⟩
  · simp [modified, adjective, noun, Features.Conforms, Features.Local,
      Features.containsTarget]
  · simp [modified, adjective, noun, Dependencies.Safe, Dependencies.Local,
      Dependencies.exposed]
  · refine ⟨?_, ?_⟩
    · simp [modified, adjective, noun, Reading.GrammarConforms, Reading.LocalGrammar]
    · decide

theorem preposition_admitted : Reading.Admitted environment [] preposition .prepositionPhrase
    ["during", "turns"] := by
  refine ⟨⟨preposition_derives, ?_, ?_, ?_⟩, preposition_realizes⟩
  · simp [preposition, temporal, Features.Conforms, Features.ChildrenConform, Features.Local,
      temporal_use, CaseAt, Syntax.nominalCase, Case.Allows, Relation.casePosition]
  · simp [preposition, temporal, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · refine ⟨?_, ?_⟩
    · simp [preposition, temporal, Reading.GrammarConforms, Reading.ChildrenConform,
        Reading.LocalGrammar]
    · decide

theorem wrong_preposition_complement : ¬ grammar.preposition during (.clause .finite) := by
  simp [grammar, Lexical.lexicon, during, word]

theorem wrong_subordinate_finiteness : ¬ grammar.subordinator conditioned .nonfinite := by
  simp [grammar, Lexical.lexicon, conditioned, word]

theorem document_rejects_bare_nominal :
    ¬ DocumentProduction .document [.nominal .plural] (.document .document) := by
  intro h
  cases h with
  | document h => cases h

/-! Flat serial-comma Coordination: three coordinands under one node, whose surface carries the
serial commas, beside the nested binary bracketing of the same three coordinands. -/

def whitened : Reading Lexeme := .node .barePlural [modified]
def serialThree : Reading Lexeme :=
  FrameScope.serial .and_ (.nounPhrase plural) [objects, temporal, whitened]
def nestedThree : Reading Lexeme :=
  FrameScope.group .and_ (.nounPhrase plural) objects
    (FrameScope.group .and_ (.nounPhrase plural) temporal whitened)
def pairTwo : Reading Lexeme := FrameScope.group .and_ (.nounPhrase plural) objects temporal

/-- The serial surface: the Oxford comma before the coordinator is part of the realization. -/
def serialSurface : Surface :=
  ["creatures", .closing ",", "turns", .closing ",", "and", "white", "creatures"]

theorem whitened_derives : Derives grammar whitened (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) .barePlural (.cons modified_derives .nil)
theorem whitened_realizes : Reading.Realizes grammar whitened ["white", "creatures"] :=
  Reading.Realizes.node (lexicon := grammar)
    (.cons (.modify adjective_realizes noun_realizes) .nil) .barePlural

theorem serialThree_derives : Derives grammar serialThree (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) (Production.serialCoordinate (n := 0) rfl)
    (.cons objects_derives (.cons temporal_derives (.cons whitened_derives .nil)))
theorem serialThree_realizes : Reading.Realizes grammar serialThree serialSurface :=
  Reading.Realizes.node (lexicon := grammar)
    (.cons objects_realizes (.cons temporal_realizes (.cons whitened_realizes .nil)))
    (.serialCoordinate (lexicon := grammar))

theorem nestedThree_derives : Derives grammar nestedThree (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) (Production.coordinate rfl)
    (.cons objects_derives
      (.cons (JudgesIn.node (lexicon := grammar) (Production.coordinate rfl)
        (.cons temporal_derives (.cons whitened_derives .nil))) .nil))
theorem nestedThree_realizes : Reading.Realizes grammar nestedThree
    ["creatures", "and", "turns", "and", "white", "creatures"] :=
  Reading.Realizes.node (lexicon := grammar)
    (.cons objects_realizes
      (.cons (Reading.Realizes.node (lexicon := grammar)
        (.cons temporal_realizes (.cons whitened_realizes .nil)) .coordinate) .nil))
    .coordinate

theorem pairTwo_derives : Derives grammar pairTwo (.nounPhrase plural) :=
  JudgesIn.node (lexicon := grammar) (Production.coordinate rfl)
    (.cons objects_derives (.cons temporal_derives .nil))

theorem serial_admitted :
    Reading.Admitted environment [] serialThree (.nounPhrase plural) serialSurface := by
  refine ⟨⟨serialThree_derives, ?_, ?_, ?_⟩, serialThree_realizes⟩
  · simp [serialThree, FrameScope.serial, objects, temporal, whitened, modified, adjective, noun,
      Features.Conforms, Features.ChildrenConform, Features.Local, childrenNominalCase,
      Syntax.nominalCase, Case.common, Case.Argument, Features.containsTarget]
    exact ⟨noun_use, .modify noun_use⟩
  · simp [serialThree, FrameScope.serial, objects, temporal, whitened, modified, adjective, noun,
      Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local, Dependencies.exposed,
      Dependencies.childrenExposed]
  · refine ⟨?_, ?_⟩
    · simp [serialThree, FrameScope.serial, objects, temporal, whitened, modified, adjective, noun,
        Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

private theorem objects_surface {surface : Surface}
    (realized : Reading.Realizes grammar objects surface) :
    surface = (["creatures"] : Surface) := by
  cases realized with
  | node children linearizes =>
    cases linearizes
    cases children with
    | cons head _ =>
      cases head with
      | noun declared => exact declared.2.2

private theorem nested_surface {surface : Surface}
    (realized : Reading.Realizes grammar nestedThree surface) :
    ∃ rest, surface = (["creatures", "and"] : Surface) ++ rest := by
  cases realized with
  | node children linearizes =>
    cases linearizes with
    | coordinate =>
      cases children with
      | cons head _ => exact ⟨_, by rw [objects_surface head]; rfl⟩

/-- Under the declared linearizations the nested binary bracketing has no realization on the
serial-comma surface: a binary coordination emits its coordinator between its two coordinands and
introduces no comma. This is a property of this model's declared linearizations, not evidence
that the nested analysis is wrong for Oracle English. -/
theorem nested_rejects_serial_surface :
    ¬ Reading.Admitted environment [] nestedThree (.nounPhrase plural) serialSurface := by
  rintro ⟨_, realized⟩
  obtain ⟨rest, equation⟩ := nested_surface realized
  simp [serialSurface] at equation

/-- Weakened premise: drop the comma atoms from the surface and the same nested bracketing is
admitted, so the exclusion above rests on the declared linearizations, not on the bracketing. -/
theorem nested_admitted_without_commas :
    Reading.Admitted environment [] nestedThree (.nounPhrase plural)
      ["creatures", "and", "turns", "and", "white", "creatures"] := by
  refine ⟨⟨nestedThree_derives, ?_, ?_, ?_⟩, nestedThree_realizes⟩
  · simp [nestedThree, FrameScope.group, objects, temporal, whitened, modified, adjective, noun,
      Features.Conforms, Features.ChildrenConform, Features.Local,
      Syntax.nominalCase, Case.common, Case.Argument, Features.containsTarget]
    exact ⟨noun_use, .modify noun_use⟩
  · simp [nestedThree, FrameScope.group, objects, temporal, whitened, modified, adjective, noun,
      Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local, Dependencies.exposed,
      Dependencies.childrenExposed]
  · refine ⟨?_, ?_⟩
    · simp [nestedThree, FrameScope.group, objects, temporal, whitened, modified, adjective, noun,
        Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

/-- Both bracketings of the same three coordinands derive, and their anchor cardinality differs. -/
theorem serial_flat_nested_differ :
    ∃ flatTree nestedTree : Reading Lexeme,
      Derives grammar flatTree (.nounPhrase plural) ∧
      Derives grammar nestedTree (.nounPhrase plural) ∧
        FrameScope.anchorCount flatTree ≠ FrameScope.anchorCount nestedTree :=
  FrameScope.flat_nested_differ serialThree_derives nestedThree_derives

/-- The two derivable bracketings project onto the two anchor shapes the boundary abstraction
names, and a binary coordination projects onto a two-anchor group. -/
theorem serial_anchor_shapes :
    FrameScope.projectAnchors serialThree = Scope.flat ∧
    FrameScope.projectAnchors nestedThree = Scope.nested ∧
    FrameScope.projectAnchors pairTwo = .group [.leaf 0, .leaf 1] := ⟨rfl, rfl, rfl⟩

theorem serial_anchor_shape_separate (survivors : Scope.AnchorPattern → Prop)
    (p : Preference.Package Scope.AnchorPattern Scope.Anchors)
    (packed : Preference.Packs survivors Scope.AnchorPattern.anchors p)
    (flatSites nestedSites : List (List Nat)) :
    ∃ x y : Reading Lexeme, Derives grammar x (.nounPhrase plural) ∧
      Derives grammar y (.nounPhrase plural) ∧
      ¬ (p.readings ⟨FrameScope.projectAnchors x, flatSites⟩ ∧
          p.readings ⟨FrameScope.projectAnchors y, nestedSites⟩) :=
  FrameScope.anchor_shape_separate survivors p packed serialThree_derives nestedThree_derives
    rfl rfl flatSites nestedSites

/-- The cardinality alone would merge a binary coordination with a flat three-item one: same
anchor count, different ordered anchor topology. -/
theorem serial_anchor_count_coarser :
    ∃ x y : Reading Lexeme, Derives grammar x (.nounPhrase plural) ∧
      Derives grammar y (.nounPhrase plural) ∧
      FrameScope.anchorCount x = FrameScope.anchorCount y ∧
      FrameScope.projectAnchors x ≠ FrameScope.projectAnchors y :=
  FrameScope.projectAnchors_flat_ne_nested pairTwo_derives serialThree_derives rfl rfl rfl

end English.FamilyWitnesses
