import English.DependencyWitnesses

namespace English.NominalWitnesses
open LexicalWitnesses GrammarWitnesses FamilyWitnesses AmbiguityWitnesses

def mana := word .mana (.noun .singular .mass) ["mana"]
def much := word .much (.determinative .singular .mass) ["much"]
def one := word .oneDet (.determinative .singular .count) ["one"]
def mass : Reading Lexeme := .node .bareMass [.noun mana .singular]
def counted : Reading Lexeme := .node (.determine .singular)
  [.word one (.determinativePhrase .singular), .noun creature .singular]
def quantifiedMass : Reading Lexeme := .node (.determine .singular)
  [.word much (.determinativePhrase .singular), .noun mana .singular]

@[simp] theorem mana_licensed : mana.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem much_licensed : much.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem one_licensed : one.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

@[simp] theorem mana_use :
    Features.NominalUse (Lexical.features environment) (.noun mana .singular) .mass :=
  .noun ⟨.singular, rfl⟩
@[simp] theorem creature_use :
    Features.NominalUse (Lexical.features environment) (.noun creature .singular) .count :=
  .noun ⟨.singular, rfl⟩
@[simp] theorem much_use : Features.DeterminerUse (Lexical.features environment)
    (.word much (.determinativePhrase .singular)) .mass := .word ⟨.singular, rfl⟩
@[simp] theorem one_use : Features.DeterminerUse (Lexical.features environment)
    (.word one (.determinativePhrase .singular)) .count := .word ⟨.singular, rfl⟩

theorem mass_admitted : Reading.Admitted environment [] mass (.nounPhrase singular) ["mana"] := by
  refine ⟨⟨.node .bareMass (.cons (.noun (lexicon := grammar) ⟨mana_licensed, .mass, rfl⟩) .nil),
      ?_, ?_, ?_⟩, .node (.cons (.noun (lexicon := grammar) ⟨mana_licensed, ⟨.mass, rfl⟩, rfl⟩) .nil)
    .bareMass⟩
  · simp [mass, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [mass, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [mass, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem count_admitted : Reading.Admitted environment [] counted (.nounPhrase singular)
    ["one", "creature"] := by
  refine ⟨⟨.node .determine
    (.cons (.word (lexicon := grammar) .determinative ⟨one_licensed, Or.inr ⟨_, _, rfl, rfl⟩⟩)
      (.cons (.noun (lexicon := grammar) ⟨creature_licensed, .count, rfl⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨one_licensed, Or.inr ⟨_, _, rfl, rfl⟩, rfl⟩)
      (.cons (.noun (lexicon := grammar) ⟨creature_licensed, ⟨.count, rfl⟩, rfl⟩) .nil)) .determine⟩
  · simp [counted, Features.Conforms, Features.ChildrenConform, Features.Local]
    exact Or.inl ⟨.count, creature_use, one_use⟩
  · simp [counted, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [counted, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem quantified_mass_admitted : Reading.Admitted environment [] quantifiedMass (.nounPhrase singular)
    ["much", "mana"] := by
  refine ⟨⟨.node .determine
    (.cons (.word (lexicon := grammar) .determinative ⟨much_licensed, Or.inr ⟨_, _, rfl, rfl⟩⟩)
      (.cons (.noun (lexicon := grammar) ⟨mana_licensed, .mass, rfl⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨much_licensed, Or.inr ⟨_, _, rfl, rfl⟩, rfl⟩)
      (.cons (.noun (lexicon := grammar) ⟨mana_licensed, ⟨.mass, rfl⟩, rfl⟩) .nil)) .determine⟩
  · simp [quantifiedMass, Features.Conforms, Features.ChildrenConform, Features.Local]
    exact Or.inl ⟨.mass, mana_use, much_use⟩
  · simp [quantifiedMass, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [quantifiedMass, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem count_determiner_rejects_mass (surface : Surface) :
    ¬ Reading.Admitted environment []
      (.node (.determine .singular) [.word one (.determinativePhrase .singular), .noun mana .singular])
      (.nounPhrase singular) surface := by
  intro admitted
  rcases admitted.1.2.1.1 with ⟨use, nominal, determiner⟩ | ⟨transparent, _⟩
  · cases nominal with
    | noun nounUse =>
      cases determiner with
      | word detUse =>
        rcases nounUse with ⟨n, h⟩
        rcases detUse with ⟨m, k⟩
        simp [mana, one, word] at h k
        exact h.2.trans k.2.symm |> Features.Countability.noConfusion
  · cases transparent

/-- A genitive determiner built over an ordinary noun phrase; the possessed head owns the
countability. -/
def genitiveDeterminer : Reading Lexeme := .node (.genitive .singular) [counted]
def possessedMana : Reading Lexeme :=
  .node (.determine .singular) [genitiveDeterminer, .noun mana .singular]

/-- The first `Derives`/`Realizes` witness for `Construction.genitive`: without one, the genitive
production and its linearization were unreachable and the transparency law ranged over no derivable
tree. The head here is mass, so a determiner that declared its own countability could not license
it. -/
theorem genitive_mass_admitted : Reading.Admitted environment [] possessedMana
    (.nounPhrase singular) ["one", "creature", .closing "'s", "mana"] := by
  refine ⟨⟨.node .determine
    (.cons (.node .genitive (.cons count_admitted.1.1 .nil))
      (.cons (.noun (lexicon := grammar) ⟨mana_licensed, .mass, rfl⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.node (.cons count_admitted.2 .nil) .genitive)
      (.cons (.noun (lexicon := grammar) ⟨mana_licensed, ⟨.mass, rfl⟩, rfl⟩) .nil)) .determine⟩
  · refine ⟨Or.inr ⟨.genitive, .mass, mana_use⟩, ⟨⟨trivial, ?_⟩, ?_⟩⟩
    · exact ⟨count_admitted.1.2.1, trivial⟩
    · simp [Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [possessedMana, genitiveDeterminer, counted, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local]
  · simp [possessedMana, genitiveDeterminer, counted, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]

/-- Head-owned countability, at the level of admission: the genitive licenses the mass head above
and the count head here, and neither admission goes through a countability claim of its own. -/
theorem genitive_count_admitted : Reading.Admitted environment []
    (.node (.determine .singular) [genitiveDeterminer, .noun creature .singular])
    (.nounPhrase singular) ["one", "creature", .closing "'s", "creature"] := by
  refine ⟨⟨.node .determine
    (.cons (.node .genitive (.cons count_admitted.1.1 .nil))
      (.cons (.noun (lexicon := grammar) ⟨creature_licensed, .count, rfl⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.node (.cons count_admitted.2 .nil) .genitive)
      (.cons (.noun (lexicon := grammar) ⟨creature_licensed, ⟨.count, rfl⟩, rfl⟩) .nil)) .determine⟩
  · refine ⟨Or.inr ⟨.genitive, .count, creature_use⟩, ⟨⟨trivial, ?_⟩, ?_⟩⟩
    · exact ⟨count_admitted.1.2.1, trivial⟩
    · simp [Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [genitiveDeterminer, counted, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [genitiveDeterminer, counted, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]

def targetMarker := word .targetMarker .targeting ["target"]
def targetVerb := word .targetVerb (.verb .plain none) ["target"]
def targetFinite := word .targetVerb (.verb .plain (some (.present, plural))) ["target"]
def targets : Reading Lexeme := .node .barePlural [.node (.targeting targetMarker .plural) [noun]]
def targetFrame : List (FrameItem (WordForm Lexeme)) := [.argument ⟨.object, .nounPhrase plural⟩]
def targetingAction : Reading Lexeme := .node .imperative
  [.node (.verb targetVerb .plain targetFrame) [objects]]
def targetingBody : Reading Lexeme := .node (.finite plural .plain)
  [.gap (.nounPhrase plural), .node (.verb targetFinite .plain targetFrame) [objects]]
def targetingRelative : Reading Lexeme := .relative .plural noun targetingBody

@[simp] theorem target_marker_licensed : targetMarker.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem target_verb_licensed : targetVerb.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem target_finite_licensed : targetFinite.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem targeting_marker_admitted : Reading.Admitted environment [] targets (.nounPhrase plural)
    ["target", "creatures"] := by
  refine ⟨⟨.node .barePlural (.cons (.node (.targeting (lexicon := grammar) ⟨target_marker_licensed, rfl⟩)
    (.cons noun_derives .nil)) .nil), ?_, ?_, ?_⟩,
    .node (.cons (.node (.cons noun_realizes .nil)
      (.targeting (lexicon := grammar) ⟨target_marker_licensed, rfl⟩)) .nil) .barePlural⟩
  · simp [targets, noun, Features.Conforms, Features.ChildrenConform, Features.Local,
      Features.containsTarget]
    exact .targeting noun_use
  · simp [targets, noun, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [targets, noun, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem target_action_admitted : Reading.Admitted environment [] targetingAction (.clause .finite)
    ["target", "creatures"] := by
  have frame : grammar.verb targetVerb .plain .active targetFrame :=
    ⟨target_verb_licensed, ⟨_, rfl⟩, declaration .targetVerb, rfl,
      Or.inr (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))⟩
  refine ⟨⟨.node .imperative (.cons (.verb frame
    (.argument (slot := ⟨.object, .nounPhrase plural⟩) objects_derives .nil)) .nil), ?_, ?_, ?_⟩,
    .node (.cons (.node (.cons objects_realizes .nil)
      (.verb (lexicon := grammar) ⟨target_verb_licensed, ⟨_, rfl⟩, rfl⟩)) .nil) .imperative⟩
  · simp [targetingAction, targetFrame, objects, Features.Conforms, Features.ChildrenConform,
      Features.Local, noun, FrameCases, CaseAt, Relation.casePosition, Syntax.nominalCase,
      Case.Allows]
    exact noun_use
  · simp [targetingAction, objects, noun, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [targetingAction, objects, noun, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]
    exact .verb rfl

theorem target_relative_admitted : Reading.Admitted environment [] targetingRelative (.nominal .plural)
    ["creatures", "that", "target", "creatures"] := by
  have predicate : Derives grammar (.node (.verb targetFinite .plain targetFrame) [objects])
      (.verbPhrase .plain) :=
    .verb ⟨target_finite_licensed, ⟨_, rfl⟩, declaration .targetVerb, rfl,
      Or.inr (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))⟩
      (.argument (slot := ⟨.object, .nounPhrase plural⟩) objects_derives .nil)
  have body : Judges grammar targetingBody (.clause .finite) [.nounPhrase plural] :=
    .finite .gap predicate (.verb ⟨target_finite_licensed, .present, rfl⟩) rfl
  refine ⟨⟨.relative noun_derives body, ?_, ?_, ?_⟩,
    .relative noun_realizes (.node (.cons .gap (.cons (.node (.cons objects_realizes .nil)
      (.verb (lexicon := grammar) ⟨target_finite_licensed, ⟨_, rfl⟩, rfl⟩)) .nil)) .finite)⟩
  · simp [targetingRelative, targetingBody, targetFrame, objects, noun, Features.Conforms,
      Features.ChildrenConform, Features.Local, FrameCases, CaseAt, Relation.casePosition,
      Syntax.nominalCase, Case.Allows]
    exact noun_use
  · simp [targetingRelative, targetingBody, targetFrame, objects, noun, Dependencies.Safe,
      Dependencies.ChildrenSafe, Dependencies.Local, Dependencies.RelativeLicense, Dependencies.exposed,
      Dependencies.frameExposed, Dependencies.childrenExposed]
  · simp [targetingRelative, targetingBody, objects, noun, Reading.GrammarConforms,
      Reading.ChildrenConform, Reading.LocalGrammar]

theorem marker_is_not_a_verb : ¬ grammar.verb targetMarker .plain .active targetFrame := by
  simp [grammar, Lexical.lexicon, targetMarker, word]

theorem target_rivalry :
    Reading.Admitted environment [] targets (.nounPhrase plural) ["target", "creatures"] ∧
    Reading.Admitted environment [] targetingAction (.clause .finite) ["target", "creatures"] ∧
    targets ≠ targetingAction := ⟨targeting_marker_admitted, target_action_admitted, by intro h; cases h⟩

/-! Previously unwitnessed cases: the `attributive` construction and the `.before` placement.
Both were declared and reachable by no derivation. -/

def nontoken := word .nontoken .attributive ["nontoken"]
def attributed : Reading Lexeme :=
  .node .barePlural [.node (.attributive nontoken .plural) [noun]]

@[simp] theorem nontoken_licensed : nontoken.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

/-- The first derivation of `Construction.attributive`. -/
theorem attributive_admitted : Reading.Admitted environment [] attributed (.nounPhrase plural)
    ["nontoken", "creatures"] := by
  refine ⟨⟨.node .barePlural (.cons (.node
    (.attributive (lexicon := grammar) ⟨nontoken_licensed, rfl⟩) (.cons noun_derives .nil)) .nil),
    ?_, ?_, ?_⟩,
    .node (.cons (.node (.cons noun_realizes .nil)
      (.attributive (lexicon := grammar) ⟨nontoken_licensed, rfl⟩)) .nil) .barePlural⟩
  · simp [attributed, noun, Features.Conforms, Features.ChildrenConform, Features.Local,
      Features.containsTarget]
    exact .attributive noun_use
  · simp [attributed, noun, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [attributed, noun, Reading.GrammarConforms, Reading.ChildrenConform,
      Reading.LocalGrammar]

/-- The ordering exclusion that pairs with it: an attributive layer does not hide a target from
the outer-modifier check. Instance of `Features.attributive_does_not_hide_target`. -/
theorem attributive_cannot_hide_target (modifier : Reading Lexeme) :
    ¬ Features.Local (Lexical.features environment)
      (.modify modifier (.node (.attributive nontoken .plural)
        [.node (.targeting targetMarker .plural) [noun]])) :=
  Features.attributive_does_not_hide_target _ _ _ _ _ _

def veryWord := word .degreeAdverb (.word .adverbPhrase) ["very"]
def degreeAdverb : Reading Lexeme := .word veryWord .adverbPhrase
/-- `AdjunctLicense.degree` is the only `.before` licence in the fragment. -/
def degreeAdjective : Reading Lexeme :=
  .node (.adjunct .adjectivePhrase .adverbPhrase .before) [adjective, degreeAdverb]

@[simp] theorem very_licensed : veryWord.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem adverb_derives : Derives grammar degreeAdverb .adverbPhrase :=
  .word .adverb ⟨very_licensed, Or.inl ⟨_, rfl⟩⟩
theorem adverb_realizes : Reading.Realizes grammar degreeAdverb ["very"] :=
  .word ⟨very_licensed, Or.inl ⟨_, rfl⟩, rfl⟩

/-- The first derivation at `Placement.before`: the dependent is realized *ahead* of its host. -/
theorem degree_adjunct_admitted : Reading.Admitted environment [] degreeAdjective
    .adjectivePhrase ["very", "white"] := by
  refine ⟨⟨.node (.adjunct .degree) (.cons adjective_derives (.cons adverb_derives .nil)),
    ?_, ?_, ?_⟩,
    .node (.cons adjective_realizes (.cons adverb_realizes .nil)) .preposedAdjunct⟩
  · simp [degreeAdjective, adjective, degreeAdverb, Features.Conforms, Features.ChildrenConform,
      Features.Local]
  · simp [degreeAdjective, adjective, degreeAdverb, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.exposed]
  · simp [degreeAdjective, adjective, degreeAdverb, Reading.GrammarConforms,
      Reading.ChildrenConform, Reading.LocalGrammar]

/-- Weakened premise: drop `.before` from the same pair and nothing licenses the adjunct, so the
placement component of `AdjunctLicense` is load-bearing rather than decorative. -/
theorem degree_adjunct_rejects_after :
    ¬ Derives grammar (.node (.adjunct .adjectivePhrase .adverbPhrase .after)
      [adjective, degreeAdverb]) .adjectivePhrase := by
  intro h
  cases h with
  | node production _ => cases production with | adjunct licence => cases licence

end English.NominalWitnesses
