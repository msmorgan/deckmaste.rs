import English.Grammar

/-! Cross-capability witnesses use synthetic wording, not claims about named Oracle cards. -/

namespace English.Composition

inductive Lexeme where
  | creature | artifact | turn | attack | destroy | can | during | with_ | each | two | one | plus
  | be | have | to | greaterThan | if_
  deriving DecidableEq

def plural : Agreement := ⟨.third, .plural⟩
def singular : Agreement := ⟨.third, .singular⟩
def objectFrame : List (FrameItem Lexeme) := [.argument ⟨.object, .nounPhrase plural⟩]

def lexicon : Lexicon Lexeme where
  noun lexeme _ := lexeme = .creature ∨ lexeme = .artifact ∨ lexeme = .turn
  adjective _ := False
  nounForm lexeme number surface :=
    (lexeme = .creature ∧ number = .plural ∧ surface = (["creatures"] : Surface)) ∨
    (lexeme = .artifact ∧ number = .plural ∧ surface = (["artifacts"] : Surface)) ∨
    (lexeme = .turn ∧ number = .plural ∧ surface = (["turns"] : Surface))
  adjectiveForm _ _ := False
  word lexeme category :=
    (lexeme = .each ∧ category = .determinativePhrase .singular) ∨
    (lexeme = .two ∧ category = .cardinalNumeral .plural) ∨
    ((lexeme = .one ∨ lexeme = .two) ∧ category = .measurePhrase)
  wordForm lexeme category surface :=
    (lexeme = .two ∧ category = .cardinalNumeral .plural ∧ surface = (["two"] : Surface)) ∨
    (lexeme = .one ∧ category = .measurePhrase ∧ surface = (["1"] : Surface)) ∨
    (lexeme = .two ∧ category = .measurePhrase ∧ surface = (["2"] : Surface))
  verb lexeme form voice frame := voice = .active ∧ (
    (lexeme = .attack ∧ (form = .plain ∨ form = .thirdSingularPresent) ∧ frame = []) ∨
    (lexeme = .destroy ∧ form = .plain ∧ frame = objectFrame))
  verbForm lexeme form surface :=
    (lexeme = .attack ∧ form = .plain ∧ surface = (["attack"] : Surface)) ∨
    (lexeme = .attack ∧ form = .thirdSingularPresent ∧ surface = (["attacks"] : Surface)) ∨
    (lexeme = .destroy ∧ form = .plain ∧ surface = (["destroy"] : Surface)) ∨
    (lexeme = .can ∧ form = .plain ∧ surface = (["can"] : Surface))
  finite lexeme agreement form :=
    (lexeme = .attack ∧ FiniteForm agreement form) ∨ (lexeme = .can ∧ form = .plain)
  auxiliary lexeme form selected voice selectedVoice :=
    lexeme = .can ∧ form = .plain ∧ selected = .plain ∧
      voice = .active ∧ selectedVoice = .active
  preposition lexeme category :=
    (lexeme = .during ∨ lexeme = .with_) ∧ category = .nounPhrase plural
  markerForm lexeme surface :=
    (lexeme = .during ∧ surface = (["during"] : Surface)) ∨
    (lexeme = .with_ ∧ surface = (["with"] : Surface)) ∨
    (lexeme = .plus ∧ surface = (["plus"] : Surface))
  measure lexeme := lexeme = .plus

def creatures : Syntax Lexeme := .node .barePlural [.noun .creature .plural]
def artifacts : Syntax Lexeme := .node .barePlural [.noun .artifact .plural]
def turns : Syntax Lexeme := .node .barePlural [.noun .turn .plural]
def attack : Syntax Lexeme := .node (.verb .attack .plain []) []
def duringTurns : Syntax Lexeme := .node (.preposition .during (.nounPhrase plural)) [turns]
def canAttack : Syntax Lexeme := .node (.auxiliary .can .plain .plain .positive) [attack]
def relativeBody : Syntax Lexeme :=
  .node (.finite plural .plain)
    [.gap (.nounPhrase plural),
     .node (.adjunct (.verbPhrase .plain) .prepositionPhrase) [canAttack, duringTurns]]
def relativeNominal : Syntax Lexeme := .relative .plural (.noun .creature .plural) relativeBody

theorem creatures_derives : Derives lexicon creatures (.nounPhrase plural) :=
  .node .barePlural (.cons (.noun (Or.inl rfl)) .nil)

theorem artifacts_derives : Derives lexicon artifacts (.nounPhrase plural) :=
  .node .barePlural (.cons (.noun (Or.inr (Or.inl rfl))) .nil)

theorem turns_derives : Derives lexicon turns (.nounPhrase plural) :=
  .node .barePlural (.cons (.noun (Or.inr (Or.inr rfl))) .nil)

theorem attack_derives : Derives lexicon attack (.verbPhrase .plain) :=
  .verb ⟨rfl, Or.inl ⟨rfl, Or.inl rfl, rfl⟩⟩ .nil

theorem during_derives : Derives lexicon duringTurns .prepositionPhrase :=
  .node (.preposition ⟨Or.inl rfl, rfl⟩) (.cons turns_derives .nil)

theorem can_attack_derives : Derives lexicon canAttack (.verbPhrase .plain) :=
  .node (.auxiliary ⟨rfl, rfl, rfl, rfl, rfl⟩) (.cons attack_derives .nil)

/-- The subject gap survives both auxiliary composition and a postposed adjunct. -/
theorem relative_body_gap :
    Judges lexicon relativeBody (.clause .finite) [.nounPhrase plural] :=
  .finite .gap
    (.node (.adjunct .verbal) (.cons can_attack_derives (.cons during_derives .nil)))
    (.adjunct (.auxiliary (Or.inr ⟨rfl, rfl⟩))) rfl

theorem relative_derives : Derives lexicon relativeNominal (.nominal .plural) :=
  .relative (.noun (Or.inl rfl)) relative_body_gap

def coordinatedObjects : Syntax Lexeme :=
  .node (.coordinate .and_ (.nounPhrase plural)) [creatures, artifacts]
def destroyObjects : Syntax Lexeme := .node (.verb .destroy .plain objectFrame) [coordinatedObjects]

theorem coordinated_complement : Derives lexicon destroyObjects (.verbPhrase .plain) :=
  .verb ⟨rfl, Or.inr ⟨rfl, rfl, rfl⟩⟩
    (.argument (slot := ⟨.object, .nounPhrase plural⟩) (.closedNode (.coordinate rfl) (.cons
      creatures_derives (.cons artifacts_derives .nil))) .nil)

/-- The identical NP tree fills Subject, Object and a preposition’s Complement. -/
theorem noun_phrase_relations :
    Derives lexicon (.node (.finite plural .plain) [creatures, attack]) (.clause .finite) ∧
    Derives lexicon (.node (.verb .destroy .plain objectFrame) [creatures]) (.verbPhrase .plain) ∧
    Derives lexicon (.node (.preposition .with_ (.nounPhrase plural)) [creatures])
      .prepositionPhrase :=
  ⟨.finite creatures_derives attack_derives (.verb (Or.inl ⟨rfl, .other rfl⟩)) rfl,
   .verb ⟨rfl, Or.inr ⟨rfl, rfl, rfl⟩⟩ 
     (.argument (slot := ⟨.object, .nounPhrase plural⟩) creatures_derives .nil),
   .node (.preposition ⟨Or.inr rfl, rfl⟩) (.cons creatures_derives .nil)⟩

theorem creatures_surface : Realizes lexicon creatures (["creatures"] : Surface) :=
  .node (.cons (.noun (Or.inl ⟨rfl, rfl, rfl⟩)) .nil) .barePlural

theorem artifacts_surface : Realizes lexicon artifacts (["artifacts"] : Surface) :=
  .node (.cons (.noun (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))) .nil) .barePlural

theorem turns_surface : Realizes lexicon turns (["turns"] : Surface) :=
  .node (.cons (.noun (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))) .nil) .barePlural

theorem attack_surface : Realizes lexicon attack (["attack"] : Surface) :=
  .node .nil (.verb (v := (["attack"] : Surface)) (Or.inl ⟨rfl, rfl, rfl⟩))

theorem can_attack_surface : Realizes lexicon canAttack (["can", "attack"] : Surface) :=
  .node (.cons attack_surface .nil)
    (.auxiliary (v := (["can"] : Surface)) (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))))

theorem during_surface : Realizes lexicon duringTurns (["during", "turns"] : Surface) :=
  .node (.cons turns_surface .nil) (.preposition (m := (["during"] : Surface)) (Or.inl ⟨rfl, rfl⟩))

theorem relative_surface : Realizes lexicon relativeNominal
    (["creatures", "that", "can", "attack", "during", "turns"] : Surface) :=
  .relative (.noun (surface := (["creatures"] : Surface)) (Or.inl ⟨rfl, rfl, rfl⟩))
    (.node (.cons .gap (.cons
      (.node (.cons can_attack_surface (.cons during_surface .nil)) .adjunct) .nil)) .finite)

theorem coordinated_surface : Realizes lexicon destroyObjects
    (["destroy", "creatures", "and", "artifacts"] : Surface) :=
  .node (.cons (.node (.cons creatures_surface (.cons artifacts_surface .nil)) .coordinate) .nil)
    (.verb (v := (["destroy"] : Surface)) (Or.inr (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩))))

/-- Negative polarity follows a licensed auxiliary. -/
theorem negative_auxiliary :
    Admissible lexicon (.node (.auxiliary .can .plain .plain .negative) [attack])
      (.verbPhrase .plain) (["can", "not", "attack"] : Surface) :=
  ⟨.node (.auxiliary ⟨rfl, rfl, rfl, rfl, rfl⟩) (.cons attack_derives .nil),
   .node (.cons attack_surface .nil)
     (.negativeAuxiliary (v := (["can"] : Surface)) (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))))⟩

theorem quantity_np :
    Admissible lexicon (.node (.determine .plural)
      [.node (.quantify .plural) [.word .two (.cardinalNumeral .plural)], .noun .creature .plural])
      (.nounPhrase plural) (["two", "creatures"] : Surface) :=
  ⟨.node .determine (.cons
      (.node .quantify (.cons (.word .quantity (Or.inr (Or.inl ⟨rfl, rfl⟩))) .nil))
      (.cons (.noun (Or.inl rfl)) .nil)),
   .node (.cons (.node (.cons (.word (Or.inl ⟨rfl, rfl, rfl⟩)) .nil) .quantify)
     (.cons (.noun (Or.inl ⟨rfl, rfl, rfl⟩)) .nil)) .determine⟩

/-- Local exclusions have explicit positive twins, without an empty admissibility domain. -/
theorem singular_agreement : FiniteForm singular .thirdSingularPresent := .singular rfl

theorem singular_rejects_plain : ¬ FiniteForm singular .plain := by
  intro derivation
  cases derivation with
  | other equality => cases equality

theorem gap_not_closed (category : Category) :
    ¬ Derives lexicon (.gap category) category := by
  intro derivation
  cases derivation

private theorem extra_complement (gaps : List Category) :
    ¬ JudgeChildren lexicon [creatures, artifacts] [Category.nounPhrase plural] gaps := by
  intro derivation
  cases derivation with
  | cons _ tail => cases tail

theorem frame_rejects_extra_complement :
    ¬ JudgeChildren lexicon [creatures, artifacts] [Category.nounPhrase plural] [] :=
  extra_complement []

theorem elliptic_fragment :
    Derives lexicon (.ellipsis .plain) (.verbPhrase .plain) := .ellipsis

theorem elliptic_surface : Realizes lexicon (.ellipsis .plain) [] := .ellipsis

/-- Invariant modals use declared agreement, including a singular Subject. -/
theorem modal_singular : FiniteLicense lexicon canAttack singular :=
  .auxiliary (Or.inr ⟨rfl, rfl⟩)

def voiceLexicon : Lexicon Lexeme := { lexicon with
  verb := fun head form voice frame => head = .destroy ∧ form = .pastParticiple ∧
    ((voice = .active ∧ frame = objectFrame) ∨ (voice = .passive ∧ frame = []))
  auxiliary := fun head form selected voice selectedVoice =>
    form = .thirdSingularPresent ∧ selected = .pastParticiple ∧
    ((head = .be ∧ voice = .passive ∧ selectedVoice = .passive) ∨
     (head = .have ∧ voice = .active ∧ selectedVoice = .active))
  verbForm := fun head form surface =>
    (head = .destroy ∧ form = .pastParticiple ∧ surface = (["destroyed"] : Surface)) ∨
    (head = .be ∧ form = .thirdSingularPresent ∧ surface = (["is"] : Surface)) }

def passive : Syntax Lexeme :=
  .node (.auxiliary .be .thirdSingularPresent .pastParticiple .positive .passive .passive)
    [.node (.verb .destroy .pastParticiple [] .passive) []]

theorem passive_derives :
    Derives voiceLexicon passive (.verbPhrase .thirdSingularPresent .passive) :=
  .node (.auxiliary ⟨rfl, rfl, Or.inl ⟨rfl, rfl, rfl⟩⟩)
    (.cons (.verb (lexicon := voiceLexicon) (head := .destroy) (form := .pastParticiple)
      (voice := .passive) (frame := []) ⟨rfl, rfl, Or.inr ⟨rfl, rfl⟩⟩ .nil) .nil)

theorem passive_surface : Realizes voiceLexicon passive (["is", "destroyed"] : Surface) :=
  .node (.cons (.node .nil (.verb (v := (["destroyed"] : Surface)) (Or.inl ⟨rfl, rfl, rfl⟩))) .nil)
    (.auxiliary (v := (["is"] : Surface)) (Or.inr ⟨rfl, rfl, rfl⟩))

/-- A perfect auxiliary cannot select a passive frame by morphology alone. -/
theorem perfect_rejects_passive :
    ¬ voiceLexicon.auxiliary .have .thirdSingularPresent .pastParticiple .active .passive := by
  intro licensed
  rcases licensed with ⟨_, _, wrongHead | wrongVoice⟩
  · cases wrongHead.1
  · cases wrongVoice.2.2

theorem perfect_active_twin :
    voiceLexicon.auxiliary .have .thirdSingularPresent .pastParticiple .active .active :=
  ⟨rfl, rfl, Or.inr ⟨rfl, rfl, rfl⟩⟩

theorem fixed_marker_frame :
    JudgeFrame lexicon [.marker .to, creatures]
      [.fixed .to, .argument ⟨.complement, .nounPhrase plural⟩] [] :=
  .fixed (.argument (slot := ⟨.complement, .nounPhrase plural⟩) creatures_derives .nil)

theorem wrong_fixed_marker :
    ¬ JudgeFrame lexicon [.marker .during] [.fixed .to] [] := by
  intro derivation
  cases derivation

/-- Both conjuncts share one missing NP; ordinary composition would retain two resources. -/
theorem shared_relative_gap :
    Judges lexicon (.sharedCoordination .and_ (.clause .finite) relativeBody relativeBody)
      (.clause .finite) [.nounPhrase plural] :=
  .sharedCoordination rfl relative_body_gap relative_body_gap

theorem arithmetic : Admissible lexicon
    (.node (.measure .plus) [.word .one .measurePhrase, .word .two .measurePhrase])
    .measurePhrase (["1", "plus", "2"] : Surface) :=
  ⟨.node (.measure rfl)
    (.cons (.word .measure (Or.inr (Or.inr ⟨Or.inl rfl, rfl⟩)))
      (.cons (.word .measure (Or.inr (Or.inr ⟨Or.inr rfl, rfl⟩))) .nil)),
   .node (.cons (.word (Or.inr (Or.inl ⟨rfl, rfl, rfl⟩)))
      (.cons (.word (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))) .nil))
    (.measure (m := (["plus"] : Surface)) (Or.inr (Or.inr ⟨rfl, rfl⟩)))⟩

def comparisonLexicon : Lexicon Lexeme := { lexicon with
  comparison := fun head => head = .greaterThan
  markerForm := fun head surface =>
    head = .greaterThan ∧ surface = (["greater", "than"] : Surface) }

theorem comparison : Admissible comparisonLexicon
    (.node (.compare .greaterThan) [.word .two .measurePhrase])
    .adjectivePhrase (["greater", "than", "2"] : Surface) :=
  ⟨.node (.compare rfl)
    (.cons (.word .measure (Or.inr (Or.inr ⟨Or.inr rfl, rfl⟩))) .nil),
   .node (.cons (.word (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))) .nil)
     (.compare (m := (["greater", "than"] : Surface)) ⟨rfl, rfl⟩)⟩

theorem nonfinite_clause : Derives lexicon
    (.node (.nonfinite .plain) [attack]) (.clause .nonfinite) :=
  .node (.nonfinite (Or.inl rfl)) (.cons attack_derives .nil)


end English.Composition
