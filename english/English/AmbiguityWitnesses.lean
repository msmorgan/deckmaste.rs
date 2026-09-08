import English.FamilyWitnesses

namespace English.AmbiguityWitnesses
open LexicalWitnesses GrammarWitnesses

def speakerWord := word .i (.word (.nounPhrase speaker) .nominative) ["I"]
def saw := word .see (.verb .preterite (some (.past, speaker))) ["saw"]
def possessive := word .herPossessive (.determinative .singular .count) ["her"]
def objectWord := word .herObject (.word (.nounPhrase singular) .accusative) ["her"]
def duckNoun := word .duckNoun (.noun .singular .count) ["duck"]
def duckVerb := word .duckVerb (.verb .plain none) ["duck"]
def nominalFrame : List (FrameItem (WordForm Lexeme)) := [.argument ⟨.object, .nounPhrase singular⟩]
def perceptionFrame : List (FrameItem (WordForm Lexeme)) :=
  [.argument ⟨.object, .nounPhrase singular⟩, .argument ⟨.complement, .verbPhrase .plain⟩]
def subject : Reading Lexeme := .word speakerWord (.nounPhrase speaker)
def herDuck : Reading Lexeme := .node (.determine .singular)
  [.word possessive (.determinativePhrase .singular), .noun duckNoun .singular]
def ducking : Reading Lexeme := .node (.verb duckVerb .plain []) []
def nounPredicate : Reading Lexeme := .node (.verb saw .preterite nominalFrame) [herDuck]
def verbPredicate : Reading Lexeme := .node (.verb saw .preterite perceptionFrame)
  [.word objectWord (.nounPhrase singular), ducking]
def nounReading : Reading Lexeme := .node (.finite speaker .preterite) [subject, nounPredicate]
def verbReading : Reading Lexeme := .node (.finite speaker .preterite) [subject, verbPredicate]

@[simp] theorem speaker_licensed : speakerWord.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem saw_licensed : saw.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem possessive_licensed : possessive.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem object_licensed : objectWord.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem duck_noun_licensed : duckNoun.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
@[simp] theorem duck_verb_licensed : duckVerb.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem subject_derives : Derives grammar subject (.nounPhrase speaker) :=
  .word .pronoun ⟨speaker_licensed, Or.inl ⟨_, rfl⟩⟩
theorem her_duck_derives : Derives grammar herDuck (.nounPhrase singular) :=
  .node .determine
    (.cons (.word (lexicon := grammar) .determinative ⟨possessive_licensed, Or.inr ⟨_, _, rfl, rfl⟩⟩)
      (.cons (.noun (lexicon := grammar) ⟨duck_noun_licensed, .count, rfl⟩) .nil))
theorem ducking_derives : Derives grammar ducking (.verbPhrase .plain) :=
  .verb ⟨duck_verb_licensed, ⟨_, rfl⟩, declaration .duckVerb, rfl,
    Or.inr (Or.inr (Or.inr (Or.inl ⟨rfl, rfl, rfl, rfl⟩)))⟩ .nil

theorem noun_reading_derives : Derives grammar nounReading (.clause .finite) :=
  .finite subject_derives
    (.verb ⟨saw_licensed, ⟨_, rfl⟩, declaration .see, rfl,
      Or.inr (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl, Or.inl rfl⟩)))⟩
      (.argument (slot := ⟨.object, .nounPhrase singular⟩) her_duck_derives .nil))
    (.verb ⟨saw_licensed, .past, rfl⟩) rfl

theorem verb_reading_derives : Derives grammar verbReading (.clause .finite) :=
  .finite subject_derives
    (.verb ⟨saw_licensed, ⟨_, rfl⟩, declaration .see, rfl,
      Or.inr (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl, Or.inr rfl⟩)))⟩
      (.argument (slot := ⟨.object, .nounPhrase singular⟩)
        (.word (lexicon := grammar) .pronoun ⟨object_licensed, Or.inl ⟨_, rfl⟩⟩)
        (.argument (slot := ⟨.complement, .verbPhrase .plain⟩) ducking_derives .nil)))
    (.verb ⟨saw_licensed, .past, rfl⟩) rfl

theorem noun_reading_realizes : Reading.Realizes grammar nounReading ["I", "saw", "her", "duck"] :=
  .node (.cons (.word (lexicon := grammar) ⟨speaker_licensed, Or.inl ⟨_, rfl⟩, rfl⟩)
    (.cons (.node (.cons (.node
      (.cons (.word (lexicon := grammar) ⟨possessive_licensed, Or.inr ⟨_, _, rfl, rfl⟩, rfl⟩)
        (.cons (.noun (lexicon := grammar) ⟨duck_noun_licensed, ⟨.count, rfl⟩, rfl⟩) .nil)) .determine)
      .nil) (.verb (lexicon := grammar) ⟨saw_licensed, ⟨_, rfl⟩, rfl⟩)) .nil)) .finite

theorem verb_reading_realizes : Reading.Realizes grammar verbReading ["I", "saw", "her", "duck"] :=
  .node (.cons (.word (lexicon := grammar) ⟨speaker_licensed, Or.inl ⟨_, rfl⟩, rfl⟩)
    (.cons (.node
      (.cons (.word (lexicon := grammar) ⟨object_licensed, Or.inl ⟨_, rfl⟩, rfl⟩)
        (.cons (.node .nil (.verb (lexicon := grammar) ⟨duck_verb_licensed, ⟨_, rfl⟩, rfl⟩)) .nil))
      (.verb (lexicon := grammar) ⟨saw_licensed, ⟨_, rfl⟩, rfl⟩)) .nil)) .finite

theorem noun_admitted : Reading.Admitted environment [] nounReading (.clause .finite)
    ["I", "saw", "her", "duck"] := by
  refine ⟨⟨noun_reading_derives, ?_, ?_, ?_⟩, noun_reading_realizes⟩
  · simp [nounReading, subject, nounPredicate, herDuck, nominalFrame, Features.Conforms,
      Features.ChildrenConform, Features.Local, Syntax.nominalCase, FrameCases, CaseAt,
      Relation.casePosition, Lexical.features, speakerWord, word, Case.Allows, Case.Argument]
    exact Or.inl ⟨.count, .noun ⟨.singular, rfl⟩, .word ⟨.singular, rfl⟩⟩
  · simp [nounReading, subject, nounPredicate, herDuck, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.exposed]
  · refine ⟨?_, ?_⟩
    · simp [nounReading, subject, nounPredicate, herDuck, Reading.GrammarConforms,
        Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem verb_admitted : Reading.Admitted environment [] verbReading (.clause .finite)
    ["I", "saw", "her", "duck"] := by
  refine ⟨⟨verb_reading_derives, ?_, ?_, ?_⟩, verb_reading_realizes⟩
  · simp [verbReading, subject, verbPredicate, ducking, perceptionFrame, Features.Conforms,
      Features.ChildrenConform, Features.Local, Syntax.nominalCase, FrameCases, CaseAt,
      Relation.casePosition, Lexical.features, speakerWord, objectWord, word, Case.Allows, Case.Argument]
  · simp [verbReading, subject, verbPredicate, ducking, Dependencies.Safe, Dependencies.ChildrenSafe,
      Dependencies.Local, Dependencies.exposed]
  · refine ⟨?_, ?_⟩
    · simp [verbReading, subject, verbPredicate, ducking, Reading.GrammarConforms,
        Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem unrelated_readings_retained :
    Reading.Ambiguous environment [] (.clause .finite) ["I", "saw", "her", "duck"] :=
  ⟨nounReading, verbReading, noun_admitted, verb_admitted, by intro h; cases h⟩

/-- An illustrative external preference leaves both grammatical analyses available. -/
def nounPreference : Reading.Preference environment [] (.clause .finite) ["I", "saw", "her", "duck"] where
  prefers a b := a = nounReading ∧ b = verbReading
  admitted := by rintro a b ⟨rfl, rfl⟩; exact ⟨noun_admitted, verb_admitted⟩

theorem preferred_and_nonpreferred_remain :
    Reading.readings environment [] (.clause .finite) ["I", "saw", "her", "duck"] nounReading ∧
    Reading.readings environment [] (.clause .finite) ["I", "saw", "her", "duck"] verbReading :=
  Reading.preference_retains_both nounPreference ⟨rfl, rfl⟩

def artifact := word .artifactType (.word (.document .type)) ["Artifact"]
def control : Reading Lexeme := .word artifact (.document .type)

@[simp] theorem artifact_licensed : artifact.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem control_admitted : Reading.Admitted environment [] control (.document .type) ["Artifact"] :=
  ⟨⟨.word .type ⟨artifact_licensed, Or.inl ⟨_, rfl⟩⟩, ⟨trivial, trivial⟩, ⟨trivial, trivial⟩,
    ⟨⟨trivial, trivial⟩, by decide⟩⟩, .word ⟨artifact_licensed, Or.inl ⟨_, rfl⟩, rfl⟩⟩

private theorem type_word_unique (w : WordForm Lexeme) (capable : grammar.word w (.document .type)) :
    w = artifact := by
  rcases capable with ⟨licensed, ⟨k, bundle⟩ | ⟨n, use, _, impossible⟩⟩
  · cases w with
    | mk head features spelling capitalization provenance =>
      simp only at bundle
      subst features
      cases head <;>
        simp [WordForm.Licensed, LexicalAnalysis, environment, declaration, rows,
          Morphology.forms, singular, plural, addressee] at licensed
      rcases licensed with ⟨rfl, rfl, ⟨_, rfl⟩, casing⟩
      have same : capitalization = .declared := casing.1.resolve_right (by decide)
      cases same
      rfl
  · cases impossible

private theorem no_type_identity (w : WordForm Lexeme) : ¬ grammar.identity w (.document .type) := by
  rintro ⟨licensed, k, bundle⟩
  cases w with
  | mk head features spelling capitalization provenance =>
    simp only at bundle
    subst features
    cases head <;>
      simp [WordForm.Licensed, LexicalAnalysis, environment, declaration, rows,
        Morphology.forms, singular, plural, addressee] at licensed

private theorem coordination_not_type (coordinator : Coordinator) (category : Category)
    (licensed : coordinable category = true) :
    coordinationResult coordinator category ≠ .document .type := by
  cases coordinator <;> cases category <;> simp_all [coordinationResult, coordinable]

theorem control_unique (tree : Reading Lexeme)
    (admitted : Reading.Admitted environment [] tree (.document .type) ["Artifact"]) : tree = control := by
  have derivation := admitted.1.1
  change JudgesIn grammar [] tree (.document .type) [] at derivation
  generalize resultEq : (Category.document .type) = category at derivation
  generalize gapEq : ([] : List Category) = gaps at derivation
  cases derivation <;> try solve | cases resultEq | cases gapEq
  case word wordCategory capable =>
    cases resultEq
    rw [type_word_unique _ capable]
    rfl
  case identity wordCategory capable =>
    cases resultEq
    exact False.elim (no_type_identity _ capable)
  case node production children =>
    cases production <;> try solve | cases resultEq
    case document rule => cases rule <;> cases resultEq
    case adjunct license => cases license <;> cases resultEq
    case coordinate licensed => exact False.elim (coordination_not_type _ _ licensed resultEq.symm)
    case serialCoordinate licensed =>
      exact False.elim (coordination_not_type _ _ licensed resultEq.symm)
  case rightNodeRaising body filler =>
    cases resultEq
    have localCheck := admitted.1.2.2.2.1.1
    change coordinable (.document .type) = true ∧ _ at localCheck
    cases localCheck.1

theorem control_unambiguous : ¬ Reading.Ambiguous environment [] (.document .type) ["Artifact"] := by
  rintro ⟨a, b, ha, hb, different⟩
  exact different ((control_unique a ha).trans (control_unique b hb).symm)

/-- Nonvacuity for `Reading.duplicate_derivations`, and the "one rule twice ≠ ambiguity"
distinction stated on real trees. The hypotheses range over a tree that is actually admitted
(`control_admitted` inhabits them), and the three conjuncts separate the two levels: proofs of one
value collapse to one element of the reading subtype; that surface carries exactly one reading;
and a genuinely ambiguous surface carries two distinct *values*, which no amount of proof
collapsing removes. Derivation multiplicity is invisible inside `Prop`, so the multiplicity that
mattered was structural — the paragraph body's two judgment routes — and it is excluded by
`EllipsisInteractions.paragraph_is_the_only_route`. -/
theorem duplicate_derivations_witnessed
    (first second : Reading.Admitted environment [] control (.document .type) ["Artifact"]) :
    (⟨control, first⟩ : {t // Reading.Admitted environment [] t (.document .type) ["Artifact"]})
        = ⟨control, second⟩ ∧
    ¬ Reading.Ambiguous environment [] (.document .type) ["Artifact"] ∧
    Reading.Ambiguous environment [] (.clause .finite) ["I", "saw", "her", "duck"] :=
  ⟨Reading.duplicate_derivations first second, control_unambiguous, unrelated_readings_retained⟩

/-- The premise above is inhabited. -/
theorem duplicate_derivations_premise_inhabited :
    Reading.Admitted environment [] control (.document .type) ["Artifact"] := control_admitted

/-- The analysis roundtrip obligation has teeth: any operation satisfying it is pinned to the
exact surface of every admitted reading, including both readings of one ambiguous surface. It
cannot invent, drop or reorder atoms, and it cannot answer the two readings differently. -/
theorem analysis_roundtrip_has_teeth (render : Reading Lexeme → Option Surface)
    (roundtrip : Reading.AnalysisRoundtrip environment render) :
    render nounReading = some (["I", "saw", "her", "duck"] : Surface) ∧
    render verbReading = some (["I", "saw", "her", "duck"] : Surface) ∧
    render control = some (["Artifact"] : Surface) :=
  ⟨roundtrip [] nounReading (.clause .finite) _ noun_admitted,
    roundtrip [] verbReading (.clause .finite) _ verb_admitted,
    roundtrip [] control (.document .type) _ control_admitted⟩

/-- A concrete wrong operation the obligation refutes: the renderer that returns nothing. -/
theorem analysis_roundtrip_rejects_silence :
    ¬ Reading.AnalysisRoundtrip environment (fun _ ↦ none) := by
  intro roundtrip
  cases (analysis_roundtrip_has_teeth _ roundtrip).1

/-- A second concrete wrong operation: a renderer that answers one fixed surface for every
reading. It agrees with both readings of "I saw her duck" and is still refuted, by an admitted
reading whose surface differs. -/
theorem analysis_roundtrip_rejects_constant :
    ¬ Reading.AnalysisRoundtrip environment
      (fun _ ↦ some (["I", "saw", "her", "duck"] : Surface)) := by
  intro roundtrip
  have wrong := (analysis_roundtrip_has_teeth _ roundtrip).2.2
  simp at wrong

end English.AmbiguityWitnesses
