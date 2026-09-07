import English.FeatureConstraints
import English.Documents

namespace English.FeatureInteractions
open Features
open Documents (Witness)

inductive Lexeme where
  | creature | damage | turn | white | target | much | many | this_ | destroy
  deriving DecidableEq

def features : Declarations Lexeme where
  nounUse head use :=
    ((head = .creature ∨ head = .turn) ∧ use = .count) ∨ (head = .damage ∧ use = .mass)
  determinerUse head use :=
    (head = .much ∧ use = .mass) ∨ (head = .many ∧ use = .count) ∨ head = .this_
  temporalNoun head := head = .turn

def lexicon : Lexicon Lexeme where
  noun head number := (head = .creature ∧ number = .plural) ∨
    ((head = .damage ∨ head = .turn) ∧ number = .singular)
  adjective head := head = .white
  nounForm head number surface :=
    (head = .creature ∧ number = .plural ∧ surface = (["creatures"] : Surface)) ∨
    (head = .damage ∧ number = .singular ∧ surface = (["damage"] : Surface)) ∨
    (head = .turn ∧ number = .singular ∧ surface = (["turn"] : Surface))
  adjectiveForm head surface := head = .white ∧ surface = (["white"] : Surface)
  word head category :=
    (head = .much ∧ category = .determinativePhrase .singular) ∨
    (head = .many ∧ category = .determinativePhrase .plural) ∨
    (head = .this_ ∧ category = .determinativePhrase .singular)
  wordForm head category surface :=
    (head = .much ∧ category = .determinativePhrase .singular ∧ surface = (["much"] : Surface)) ∨
    (head = .many ∧ category = .determinativePhrase .plural ∧ surface = (["many"] : Surface)) ∨
    (head = .this_ ∧ category = .determinativePhrase .singular ∧ surface = (["this"] : Surface))
  targeting head := head = .target
  markerForm head surface := head = .target ∧ surface = (["target"] : Surface)
  verb head form voice frame :=
    head = .destroy ∧ form = .pastParticiple ∧ voice = .passive ∧ frame = []
  verbForm head form surface :=
    head = .destroy ∧ form = .pastParticiple ∧ surface = (["destroyed"] : Surface)

def damage : Syntax Lexeme := .noun .damage .singular
def much : Syntax Lexeme := .word .much (.determinativePhrase .singular)
def muchDamage : Syntax Lexeme := .node (.determine .singular) [much,damage]

theorem mass_quantity : Admitted lexicon features muchDamage (.nounPhrase ⟨.third,.singular⟩)
    ["much","damage"] := by
  refine ⟨⟨.node .determine (.cons (.word .determinative (Or.inl ⟨rfl,rfl⟩))
    (.cons (.noun (Or.inr ⟨Or.inl rfl,rfl⟩)) .nil)),
    .node (.cons (.word (Or.inl ⟨rfl,rfl,rfl⟩))
      (.cons (.noun (Or.inr (Or.inl ⟨rfl,rfl,rfl⟩))) .nil)) .determine⟩,?basic⟩
  case basic =>
    exact ⟨Or.inl ⟨.mass,.noun (Or.inr ⟨rfl,rfl⟩),.word (Or.inl ⟨rfl,rfl⟩)⟩,
      ⟨⟨trivial,trivial⟩,⟨⟨trivial,trivial⟩,trivial⟩⟩⟩

theorem numerical_mass_excluded (q : Syntax Lexeme) :
    ¬ Local features (.node (.determine .singular) [.node (.quantify .singular) [q],damage]) := by
  apply numeral_rejects_mass
  intro h
  cases h with
  | noun h => simp [features] at h

def creatures : Syntax Lexeme := .noun .creature .plural
def whiteCreatures : Syntax Lexeme := .modify (.adjective .white) creatures
def targetWhite : Syntax Lexeme := .node (.targeting .target .plural) [whiteCreatures]
def targetPhrase : Syntax Lexeme := .node .barePlural [targetWhite]

theorem target_and_adjective : Admitted lexicon features targetPhrase (.nounPhrase ⟨.third,.plural⟩)
    ["target","white","creatures"] := by
  refine ⟨⟨.node .barePlural (.cons (.node (.targeting rfl)
    (.cons (.modify (.adjective rfl) (.noun (Or.inl ⟨rfl,rfl⟩))) .nil)) .nil),
    .node (.cons (.node (.cons (.modify (.adjective ⟨rfl,rfl⟩)
      (.noun (Or.inl ⟨rfl,rfl,rfl⟩))) .nil) (.targeting (m := ["target"]) ⟨rfl,rfl⟩))
      .nil) .barePlural⟩,?basic⟩
  case basic =>
    refine ⟨.targeting (.modify (.noun (Or.inl ⟨Or.inl rfl,rfl⟩))),?children⟩
    case children =>
      simp [ChildrenConform, Conforms, Local, containsTarget, targetWhite, whiteCreatures,
        creatures]

theorem reversed_target_excluded :
    ¬ Conforms features (.modify (.adjective .white)
      (.node (.targeting .target .plural) [creatures])) := by
  intro h
  exact target_must_be_outer features _ _ _ _ h.1

def thisTurn : Syntax Lexeme := .node (.determine .singular)
  [.word .this_ (.determinativePhrase .singular),.noun .turn .singular]
def destroyed : Syntax Lexeme := .node (.verb .destroy .pastParticiple [] .passive) []
def passiveTemporal : Syntax Lexeme :=
  .node (.adjunct (.verbPhrase .pastParticiple .passive) (.nounPhrase ⟨.third,.singular⟩))
    [destroyed,thisTurn]

theorem passive_temporal : Admitted lexicon features passiveTemporal
    (.verbPhrase .pastParticiple .passive) ["destroyed","this","turn"] := by
  have turnD : Derives lexicon thisTurn (.nounPhrase ⟨.third,.singular⟩) :=
    .node .determine (.cons (.word .determinative (Or.inr (Or.inr ⟨rfl,rfl⟩)))
      (.cons (.noun (Or.inr ⟨Or.inr rfl,rfl⟩)) .nil))
  have turnR : Realizes lexicon thisTurn ["this","turn"] :=
    .node (.cons (.word (Or.inr (Or.inr ⟨rfl,rfl,rfl⟩)))
      (.cons (.noun (Or.inr (Or.inr ⟨rfl,rfl,rfl⟩))) .nil)) .determine
  have turnC : Conforms features thisTurn :=
    ⟨Or.inl ⟨.count,.noun (Or.inl ⟨Or.inr rfl,rfl⟩),.word (Or.inr (Or.inr rfl))⟩,
      ⟨⟨trivial,trivial⟩,⟨⟨trivial,trivial⟩,trivial⟩⟩⟩
  exact ⟨⟨.node (.adjunct .temporal)
    (.cons (.verb (lexicon := lexicon) ⟨rfl,rfl,rfl,rfl⟩ .nil) (.cons turnD .nil)),
    .node (.cons (.node .nil (.verb (v := ["destroyed"]) ⟨rfl,rfl,rfl⟩))
      (.cons turnR .nil)) .adjunct⟩,
    ⟨.determine (.noun rfl),
      ⟨⟨by simp [Local, destroyed, FrameCases], trivial⟩, ⟨turnC, trivial⟩⟩⟩⟩

theorem ordinary_np_not_temporal : ¬ Temporal features muchDamage := by
  intro h
  cases h with
  | determine h => cases h with
    | noun h => cases h

theorem passive_frame_does_not_acquire_object :
    ¬ JudgeFrame lexicon [thisTurn] [] [] := by intro h; cases h

/-- Countability is head-owned: a genitive determiner passes the head's use through (mass and count
alike) and declares none of its own, so it cannot supply a use the head does not declare. Replaces
`genitive_preserves_countability`, whose wildcard `DeterminerUse.genitive` proved a mass claim and a
count claim about the same shape from one constructor and so discriminated nothing. The third
conjunct is the weakened-premise counterexample: it is exactly what fails if `Transparent` is
weakened back to a `DeterminerUse` constructor holding for every countability. -/
theorem genitive_determiner_is_transparent (possessor : Syntax Lexeme) :
    Local features
      (.node (.determine .singular) [.node (.genitive .singular) [possessor], damage]) ∧
    Local features (.node (.determine .plural)
      [.node (.genitive .plural) [possessor], .noun .creature .plural]) ∧
    (∀ use, ¬ DeterminerUse features (.node (.genitive .singular) [possessor]) use) ∧
    ¬ Local features (.node (.determine .singular)
      [.node (.genitive .singular) [possessor], .noun .white .singular]) := by
  have undeclared : ∀ use, ¬ NominalUse features (.noun .white .singular) use := by
    rintro use nominal
    cases nominal with
    | noun declared => simp [features] at declared
  refine ⟨Or.inr ⟨.genitive, .mass, .noun (Or.inr ⟨rfl,rfl⟩)⟩,
    Or.inr ⟨.genitive, .count, .noun (Or.inl ⟨Or.inl rfl,rfl⟩)⟩, ?_, ?_⟩
  · intro use determiner
    cases determiner
  · rintro (⟨use,nominal,_⟩ | ⟨_,use,nominal⟩) <;> exact undeclared use nominal

end English.FeatureInteractions
