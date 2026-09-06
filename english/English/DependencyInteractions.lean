import English.Dependencies
import English.Composition

namespace English.DependencyInteractions
open Dependencies

inductive Lexeme where
  | creature | artifact | controller | you | which | whose | control | attack | put | into
  deriving DecidableEq

def plural : Agreement := ⟨.third,.plural⟩
def singular : Agreement := ⟨.third,.singular⟩
def second : Agreement := ⟨.second,.singular⟩
def object : FrameItem Lexeme := .argument ⟨.object,.nounPhrase plural⟩
def destination : FrameItem Lexeme := .argument ⟨.complement,.prepositionPhrase⟩
def lexicon : Lexicon Lexeme where
  noun head number := ((head = .creature ∨ head = .artifact) ∧ number = .plural) ∨
    (head = .controller ∧ number = .singular)
  adjective _ := False
  nounForm head number surface :=
    (head = .creature ∧ number = .plural ∧ surface = (["creatures"] : Surface)) ∨
    (head = .artifact ∧ number = .plural ∧ surface = (["artifacts"] : Surface)) ∨
    (head = .controller ∧ number = .singular ∧ surface = (["controller"] : Surface))
  adjectiveForm _ _ := False
  word head category := (head = .you ∧ category = .nounPhrase second) ∨
    (head = .which ∧ category = .nounPhrase plural) ∨
    (head = .whose ∧ category = .determinativePhrase .singular)
  wordForm head category surface :=
    (head = .you ∧ category = .nounPhrase second ∧ surface = (["you"] : Surface)) ∨
    (head = .which ∧ category = .nounPhrase plural ∧ surface = (["which"] : Surface)) ∨
    (head = .whose ∧ category = .determinativePhrase .singular ∧ surface = (["whose"] : Surface))
  verb head form voice frame := form = .plain ∧ voice = .active ∧
    ((head = .control ∧ frame = [object]) ∨ (head = .attack ∧ frame = []) ∨
      (head = .put ∧ frame = [object,destination]))
  verbForm head form surface := form = .plain ∧
    ((head = .control ∧ surface = (["control"] : Surface)) ∨
      (head = .attack ∧ surface = (["attacks"] : Surface)) ∨
      (head = .put ∧ surface = (["put"] : Surface)))
  finite head agreement form := form = .plain ∧
    ((head = .attack ∧ agreement = singular) ∨
     ((head = .control ∨ head = .put) ∧ agreement = second))
  preposition head category := head = .into ∧ category = .nounPhrase plural
  markerForm head surface := head = .into ∧ surface = (["into"] : Surface)
def features : Features.Declarations Lexeme where
  nounUse _ use := use = .count
  determinerUse _ use := use = .count
  temporalNoun _ := False
def dependencies : Declarations Lexeme where
  relativePronoun head := head = .which
  relativeDeterminer head := head = .whose

def creatures : Syntax Lexeme := .noun .creature .plural
def you : Syntax Lexeme := .word .you (.nounPhrase second)
def which : Syntax Lexeme := .word .which (.nounPhrase plural)
def controlGap : Syntax Lexeme := .node (.verb .control .plain [object]) [.gap (.nounPhrase plural)]
def objectBody : Syntax Lexeme := .node (.finite second .plain) [you,controlGap]

theorem object_body : Judges lexicon objectBody (.clause .finite) [.nounPhrase plural] :=
  .finite (.word .pronoun (Or.inl ⟨rfl,rfl⟩))
    (.verb ⟨rfl,rfl,Or.inl ⟨rfl,rfl⟩⟩ (.argument (complement := ⟨.object,.nounPhrase plural⟩)
      .gap .nil)) (.verb ⟨rfl,Or.inr ⟨Or.inl rfl,rfl⟩⟩) rfl

theorem object_form : Realizes lexicon objectBody ["you","control"] :=
  .node (.cons (.word (Or.inl ⟨rfl,rfl,rfl⟩)) (.cons
    (.node (.cons .gap .nil) (.verb (v := ["control"]) ⟨rfl,Or.inl ⟨rfl,rfl⟩⟩)) .nil)) .finite

theorem object_accessible : exposed objectBody = [⟨.nounPhrase plural,.object⟩] := rfl

theorem object_body_safe : Safe dependencies objectBody := by
  simp [Safe,ChildrenSafe,Local,objectBody,you,controlGap,exposed]

theorem zero_relative_admitted : Admitted lexicon features dependencies
    (.relative .plural creatures objectBody .zero) (.nominal .plural)
      ["creatures","you","control"] := by
  refine ⟨⟨⟨.zeroRelative (.noun (Or.inl ⟨Or.inl rfl,rfl⟩)) object_body,
    .zeroRelative (.noun (Or.inl ⟨rfl,rfl,rfl⟩)) object_form⟩,?features⟩,?safe⟩
  case features =>
    simp [Features.Conforms, Features.ChildrenConform, Features.Local, creatures,
      objectBody,you,controlGap]
  case safe =>
    exact ⟨⟨plural,.object,by decide,rfl⟩,⟨⟨trivial,trivial⟩,object_body_safe,trivial⟩⟩

theorem which_relative_admitted : Admitted lexicon features dependencies
    (.relative .plural creatures objectBody .fronted [which]) (.nominal .plural)
    ["creatures","which","you","control"] := by
  refine ⟨⟨⟨.frontedRelative (Or.inl rfl) (.noun (Or.inl ⟨Or.inl rfl,rfl⟩))
    (.word .pronoun (Or.inr (Or.inl ⟨rfl,rfl⟩))) object_body,
    .frontedRelative (.noun (Or.inl ⟨rfl,rfl,rfl⟩))
      (.word (Or.inr (Or.inl ⟨rfl,rfl,rfl⟩))) object_form⟩,?features⟩,?safe⟩
  case features =>
    simp [Features.Conforms,Features.ChildrenConform,Features.Local,creatures,
      objectBody,you,controlGap,which]
  case safe =>
    exact ⟨⟨.nounPhrase plural,.object,.pronoun rfl,rfl⟩,
      ⟨⟨trivial,trivial⟩,object_body_safe,⟨⟨trivial,trivial⟩,trivial⟩⟩⟩

theorem subject_zero_rejected :
    ¬ RelativeLicense (L := Composition.Lexeme) ⟨fun _ ↦ False,fun _ ↦ False⟩ .zero []
      Composition.relativeBody :=
  zero_subject_excluded _ _ Composition.plural rfl

def whoseController : Syntax Lexeme := .node (.determine .singular)
  [.word .whose (.determinativePhrase .singular),.noun .controller .singular]
def attack : Syntax Lexeme := .node (.verb .attack .plain []) []
def subjectBody : Syntax Lexeme := .node (.finite singular .plain) [.gap (.nounPhrase
  singular),attack]

theorem whose_fronted_phrase : RelativePhrase dependencies whoseController (.nounPhrase singular) :=
  .possessive rfl

theorem whose_agreement_independent :
    Derives lexicon (.relative .plural creatures subjectBody .fronted [whoseController])
      (.nominal .plural) ∧
    RelativeLicense dependencies .fronted [whoseController] subjectBody := by
  have front : Derives lexicon whoseController (.nounPhrase singular) :=
    .node .determine (.cons (.word .determinative (Or.inr (Or.inr ⟨rfl,rfl⟩)))
      (.cons (.noun (Or.inr ⟨rfl,rfl⟩)) .nil))
  have body : Judges lexicon subjectBody (.clause .finite) [.nounPhrase singular] :=
    .finite .gap (.verb ⟨rfl,rfl,Or.inr (Or.inl ⟨rfl,rfl⟩)⟩ .nil)
      (.verb ⟨rfl,Or.inl ⟨rfl,rfl⟩⟩) rfl
  exact ⟨.frontedRelative (Or.inl rfl) (.noun (Or.inl ⟨Or.inl rfl,rfl⟩)) front body,
    ⟨.nounPhrase singular,.subject,whose_fronted_phrase,rfl⟩⟩

def intoWhich : Syntax Lexeme := .node (.preposition .into (.nounPhrase plural)) [which]
def artifacts : Syntax Lexeme := .node .barePlural [.noun .artifact .plural]
def destinationBody : Syntax Lexeme := .node (.finite second .plain)
  [you,.node (.verb .put .plain [object,destination]) [artifacts,.gap .prepositionPhrase]]

theorem pied_piping :
    Derives lexicon (.relative .plural creatures destinationBody .fronted [intoWhich])
      (.nominal .plural) ∧
    RelativeLicense dependencies .fronted [intoWhich] destinationBody := by
  have front : Derives lexicon intoWhich .prepositionPhrase :=
    .node (.preposition ⟨rfl,rfl⟩)
      (.cons (.word .pronoun (Or.inr (Or.inl ⟨rfl,rfl⟩))) .nil)
  have obj : Derives lexicon artifacts (.nounPhrase plural) :=
    .node .barePlural (.cons (.noun (Or.inl ⟨Or.inr rfl,rfl⟩)) .nil)
  have body : Judges lexicon destinationBody (.clause .finite) [.prepositionPhrase] :=
    .finite (.word .pronoun (Or.inl ⟨rfl,rfl⟩))
      (.verb ⟨rfl,rfl,Or.inr (Or.inr ⟨rfl,rfl⟩)⟩
        (.argument (complement := ⟨.object,.nounPhrase plural⟩) obj
          (.argument (complement := ⟨.complement,.prepositionPhrase⟩) .gap .nil)))
      (.verb ⟨rfl,Or.inr ⟨Or.inr rfl,rfl⟩⟩) rfl
  exact ⟨.frontedRelative (Or.inl rfl) (.noun (Or.inl ⟨Or.inl rfl,rfl⟩)) front body,
    ⟨.prepositionPhrase,.complement,.preposition (.pronoun rfl),rfl⟩⟩

theorem pied_piping_cannot_use_bare_which :
    ¬ RelativeLicense dependencies .fronted [which] destinationBody := by
  rintro ⟨category,relation,front,equation⟩
  change ([⟨.prepositionPhrase,.complement⟩] : List GapUse) = [⟨category,relation⟩] at equation
  cases equation
  exact wh_category_must_match _ _ _ front

theorem modifier_gap_rejected :
    ¬ Safe dependencies (.node (.adjunct (.verbPhrase .plain) .prepositionPhrase)
      [attack,.node (.preposition .into (.nounPhrase plural)) [.gap (.nounPhrase plural)]]) :=
  adjunct_island _ _ _ _ _ ⟨.nounPhrase plural,.complement⟩ rfl

end English.DependencyInteractions
