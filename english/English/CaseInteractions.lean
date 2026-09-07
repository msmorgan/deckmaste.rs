import English.FeatureConstraints
import English.Documents

namespace English.CaseInteractions
open Documents (Witness)

inductive Lexeme where
  | he | him | it | they | them | you | see | with_
  deriving DecidableEq

def pronouns : List Lexeme := [.he, .him, .it, .they, .them, .you]

def agreement : Lexeme → Agreement
  | .they | .them => ⟨.third, .plural⟩
  | .you => ⟨.second, .singular⟩
  | _ => ⟨.third, .singular⟩

def text : Lexeme → String
  | .he => "he" | .him => "him" | .it => "it" | .they => "they"
  | .them => "them" | .you => "you" | .see => "see" | .with_ => "with"

def features : Features.Declarations Lexeme where
  nounUse _ _ := False
  determinerUse _ _ := False
  temporalNoun _ := False
  nominalCase
    | .he | .they => .nominative
    | .him | .them => .accusative
    | _ => .nominativeOrAccusative

def object (features : Agreement) : FrameSlot := ⟨.object, .nounPhrase features⟩

def seeText (form : InflectionalForm) : Surface :=
  if form = .plain then ["see"] else ["sees"]

def lexicon : Lexicon Lexeme where
  noun _ _ := False
  adjective _ := False
  nounForm _ _ _ := False
  adjectiveForm _ _ := False
  word head category := head ∈ pronouns ∧ category = .nounPhrase (agreement head)
  wordForm head category surface :=
    head ∈ pronouns ∧ category = .nounPhrase (agreement head) ∧ surface = [Atom.word (text head)]
  verb head form voice frame := head = .see ∧ (form = .plain ∨ form = .thirdSingularPresent) ∧
    voice = .active ∧ ∃ features, frame = [.argument (object features)]
  verbForm head form surface := head = .see ∧ (form = .plain ∨ form = .thirdSingularPresent) ∧
    surface = seeText form
  finite head features form := head = .see ∧ FiniteForm features form
  preposition head category := head = .with_ ∧ ∃ features, category = .nounPhrase features
  markerForm head surface := head = .with_ ∧ surface = (["with"] : Surface)

def pronoun (head : Lexeme) (licensed : head ∈ pronouns) :
    Witness Lexeme lexicon (.nounPhrase (agreement head)) :=
  ⟨.word head (.nounPhrase (agreement head)), [Atom.word (text head)],
   .word .pronoun ⟨licensed, rfl⟩, .word ⟨licensed, rfl, rfl⟩⟩

def see (form : InflectionalForm) (licensed : form = .plain ∨ form = .thirdSingularPresent)
    (head : Lexeme) (nominal : head ∈ pronouns) : Witness Lexeme lexicon (.verbPhrase form) :=
  ⟨.node (.verb .see form [.argument (object (agreement head))]) [(pronoun head nominal).tree],
   seeText form ++ (pronoun head nominal).surface,
   .verb ⟨rfl, licensed, rfl, agreement head, rfl⟩
     (.argument (lexicon := lexicon) (slot := object (agreement head))
       (headGaps := []) (tailGaps := []) (pronoun head nominal).derives .nil),
   .node (.cons (pronoun head nominal).realizes .nil)
     (.verb (v := seeText form) ⟨rfl, licensed, rfl⟩)⟩

def clause (subject complement : Lexeme) (subjectWord : subject ∈ pronouns)
    (objectWord : complement ∈ pronouns) (finite : FiniteForm (agreement subject)
      .thirdSingularPresent) : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node (.finite (agreement subject) .thirdSingularPresent)
     [(pronoun subject subjectWord).tree,
      (see .thirdSingularPresent (Or.inr rfl) complement objectWord).tree],
   (pronoun subject subjectWord).surface ++
     (see .thirdSingularPresent (Or.inr rfl) complement objectWord).surface,
   .finite (pronoun subject subjectWord).derives
     (see .thirdSingularPresent (Or.inr rfl) complement objectWord).derives
     (.verb (lexicon := lexicon) ⟨rfl, finite⟩) rfl,
   .node (.cons (pronoun subject subjectWord).realizes
     (.cons (see .thirdSingularPresent (Or.inr rfl) complement objectWord).realizes .nil))
     .finite⟩

def withPronoun (head : Lexeme) (licensed : head ∈ pronouns) :
    Witness Lexeme lexicon .prepositionPhrase :=
  ⟨.node (.preposition .with_ (.nounPhrase (agreement head))) [(pronoun head licensed).tree],
   (["with"] : Surface) ++ (pronoun head licensed).surface,
   .node (.preposition ⟨rfl, agreement head, rfl⟩) (.cons (pronoun head licensed).derives .nil),
   .node (.cons (pronoun head licensed).realizes .nil)
     (.preposition (m := ["with"]) ⟨rfl, rfl⟩)⟩

def objectGap (head : Lexeme) : Syntax Lexeme :=
  .node (.verb .see .plain [.argument (object (agreement head))])
    [.gap (.nounPhrase (agreement head))]

def raisedSee (head : Lexeme) (licensed : head ∈ pronouns) :
    Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.rightNodeRaising (.verbPhrase .plain) (.nounPhrase (agreement head)))
     [objectGap head, (pronoun head licensed).tree],
   (["see"] : Surface) ++ (pronoun head licensed).surface,
   .rightNodeRaising
     (.verb ⟨rfl, Or.inl rfl, rfl, agreement head, rfl⟩
       (.argument (lexicon := lexicon) (slot := object (agreement head))
         (headGaps := [.nounPhrase (agreement head)]) (tailGaps := []) .gap .nil))
     (pronoun head licensed).derives,
   .node (.cons (.node (.cons .gap .nil)
     (.verb (v := ["see"]) ⟨rfl, Or.inl rfl, rfl⟩))
       (.cons (pronoun head licensed).realizes .nil)) .rightNodeRaising⟩

theorem raised_accusative_object : Features.Admitted lexicon features
    (raisedSee .him (by decide)).tree (.verbPhrase .plain) ["see", "him"] := by
  refine ⟨⟨(raisedSee _ _).derives, (raisedSee .him (by decide)).realizes⟩, ?_⟩
  simp [raisedSee, objectGap, pronoun, Features.Conforms, Features.ChildrenConform,
    Features.Local, Syntax.nominalCase, FrameCases, CaseAt, Relation.casePosition,
    Case.Allows, Case.Argument, features, object]

theorem raised_nominative_is_category_valid : Derives lexicon
    (raisedSee .he (by decide)).tree (.verbPhrase .plain) := (raisedSee _ _).derives

theorem raised_nominative_object_rejected (surface : Surface) :
    ¬ Features.Admitted lexicon features (raisedSee .he (by decide)).tree
      (.verbPhrase .plain) surface := by
  intro admitted
  have checked := admitted.2
  simp [raisedSee, objectGap, pronoun, Features.Conforms, Features.ChildrenConform,
    Features.Local, Syntax.nominalCase, FrameCases, CaseAt, Relation.casePosition,
    Case.Allows, features, object] at checked

/-- A fronted NP supplies the Case checked at each position in the relative body. -/
theorem fronted_case_reaches_body (number : Number) (head body front : Syntax Lexeme)
    (form : RelativeForm) (outerCase : Case)
    (checked : Features.Conforms features (.relative number head body form [front]) outerCase) :
    Features.Conforms features body (front.nominalCase features.nominalCase outerCase) :=
  checked.2.2.1

/-- A fresh unfronted relative binds its own gap independently of an enclosing filler. -/
theorem unfronted_relative_binds_case (number : Number) (head body : Syntax Lexeme)
    (form : RelativeForm) (outerCase : Case)
    (checked : Features.Conforms features (.relative number head body form []) outerCase) :
    Features.Conforms features body .nominativeOrAccusative := checked.2.2.1

theorem nominative_subject_and_accusative_object : Features.Admitted lexicon features
    (clause .he .him (by decide) (by decide) (.singular rfl)).tree
    (.clause .finite) ["he", "sees", "him"] := by
  refine ⟨⟨(clause _ _ _ _ _).derives,
    (clause .he .him (by decide) (by decide) (.singular rfl)).realizes⟩, ?_⟩
  simp [clause, see, pronoun, Features.Conforms, Features.ChildrenConform, Features.Local,
    Syntax.nominalCase, FrameCases, CaseAt, Relation.casePosition, Case.Allows, Case.Argument,
    features, object]

theorem accusative_prepositional_complement : Features.Admitted lexicon features
    (withPronoun .him (by decide)).tree .prepositionPhrase ["with", "him"] := by
  refine ⟨⟨(withPronoun _ _).derives, (withPronoun .him (by decide)).realizes⟩, ?_⟩
  simp [withPronoun, pronoun, Features.Conforms, Features.ChildrenConform, Features.Local,
    Syntax.nominalCase, CaseAt, Relation.casePosition, Case.Allows, Case.Argument, features]

/-- Category and agreement alone admit this tree, so Case must reject the candidate. -/
theorem wrong_subject_is_category_valid : Derives lexicon
    (clause .him .it (by decide) (by decide) (.singular rfl)).tree (.clause .finite) :=
  (clause _ _ _ _ _).derives

theorem accusative_subject_rejected (surface : Surface) : ¬ Features.Admitted lexicon features
    (clause .him .it (by decide) (by decide) (.singular rfl)).tree (.clause .finite) surface := by
  intro admitted
  have checked := admitted.2.1
  simp [Features.Local, clause, pronoun, Syntax.nominalCase, features, Case.Allows] at checked

theorem nominative_object_rejected (surface : Surface) : ¬ Features.Admitted lexicon features
    (see .plain (Or.inl rfl) .he (by decide)).tree (.verbPhrase .plain) surface := by
  intro admitted
  have checked := admitted.2.1
  simp [Features.Local, see, pronoun, FrameCases, CaseAt, object, Syntax.nominalCase,
    features, Case.Allows, Relation.casePosition] at checked

theorem nominative_prepositional_complement_rejected (surface : Surface) :
    ¬ Features.Admitted lexicon features (withPronoun .they (by decide)).tree
      .prepositionPhrase surface := by
  intro admitted
  have checked := admitted.2.1
  simp [Features.Local, withPronoun, pronoun, CaseAt, Syntax.nominalCase,
    features, Case.Allows, Relation.casePosition] at checked

theorem coordination_requires_every_conjunct (coordinator : Coordinator)
    (leftAgreement rightAgreement : Agreement) (left right : Syntax Lexeme)
    (position : CasePosition) :
    ((Syntax.node (.coordinate coordinator (.nounPhrase leftAgreement) (.nounPhrase rightAgreement))
      [left, right]).nominalCase features.nominalCase).Allows position ↔
        (left.nominalCase features.nominalCase).Allows position ∧
        (right.nominalCase features.nominalCase).Allows position :=
  Case.common_allows _ _ _

theorem crossed_case_coordination_rejected (coordinator : Coordinator) :
    ¬ Features.Local features
      (.node (.coordinate coordinator (.nounPhrase (agreement .he)) (.nounPhrase (agreement .them)))
        [(pronoun .he (by decide)).tree, (pronoun .them (by decide)).tree]) := by
  simp [Features.Local, pronoun, Syntax.nominalCase, features, Case.common, Case.Argument]

end English.CaseInteractions
