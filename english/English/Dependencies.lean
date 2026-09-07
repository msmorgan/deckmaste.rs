import English.FeatureConstraints

/-! Structural extraction constraints; grammatical antecedents never denote game entities here. -/
namespace English.Dependencies
variable {L : Type}

structure Declarations (L : Type) where
  relativePronoun : L → Prop
  relativeDeterminer : L → Prop

structure GapUse where
  category : Category
  relation : Relation
  deriving DecidableEq

mutual
  def exposed (tree : Syntax L) (relation : Relation := .complement) : List GapUse :=
    match tree with
    | .gap category => [⟨category,relation⟩]
    | .relativeForm _ _ _ _ _ | .node (.rightNodeRaising _ _) _ => []
    | .node (.finite _ _ _) [subject,predicate] =>
        exposed subject .subject ++ exposed predicate .complement
    | .node (.verb _ _ frame _) children => frameExposed children frame
    | .node _ children => childrenExposed children relation
    | .modify a b => exposed a relation ++ exposed b relation
    | .sharedCoordination _ _ a _ => exposed a relation
    | .frameCoordination _ a b => childrenExposed a relation ++ childrenExposed b relation
    | _ => []
  def childrenExposed (trees : List (Syntax L)) (relation : Relation) : List GapUse :=
    match trees with
    | [] => []
    | a :: rest => exposed a relation ++ childrenExposed rest relation
  def frameExposed (trees : List (Syntax L)) (frame : List (FrameItem L)) : List GapUse :=
    match trees,frame with
    | [.frameCoordination _ a b],frame => frameExposed a frame ++ frameExposed b frame
    | a :: rest,.argument complement :: frame =>
        exposed a complement.relation ++ frameExposed rest frame
    | .marker _ :: a :: rest,.marked _ complement :: frame =>
        exposed a complement.relation ++ frameExposed rest frame
    | .marker _ :: rest,.fixed _ :: frame => frameExposed rest frame
    | _,_ => []
end

inductive RelativePhrase (features : Declarations L) : Syntax L → Category → Prop where
  | pronoun {head : L} {agreement : Agreement} : features.relativePronoun head →
      RelativePhrase features (.word head (.nounPhrase agreement)) (.nounPhrase agreement)
  | possessive {head : L} {number : Number} {nominal : Syntax L} :
      features.relativeDeterminer head →
      RelativePhrase features
        (.node (.determine number) [.word head (.determinativePhrase number),nominal])
        (.nounPhrase ⟨.third,number⟩)
  | preposition {marker : L} {category : Category} {front : Syntax L} :
      RelativePhrase features front category →
      RelativePhrase features (.node (.preposition marker category) [front]) .prepositionPhrase

/-- The zero form cannot discharge a Subject, and fronted relatives require a declared wh phrase. -/
def RelativeLicense (features : Declarations L) (form : RelativeForm)
    (front : List (Syntax L)) (body : Syntax L) : Prop :=
  match form,front with
  | .that_,[] => ∃ agreement relation, exposed body = [⟨.nounPhrase agreement,relation⟩]
  | .zero,[] => ∃ agreement relation, relation ≠ .subject ∧
      exposed body = [⟨.nounPhrase agreement,relation⟩]
  | .fronted,[phrase] | .supplementary,[phrase] =>
      ∃ category relation, RelativePhrase features phrase category ∧
        exposed body = [⟨category,relation⟩]
  | _,_ => False

def Local (features : Declarations L) : Syntax L → Prop
  | .node (.adjunct _ _ _) [_,dependent] | .node (.initialAdverbial _) [dependent,_] =>
      exposed dependent = []
  | .modify modifier _ => exposed modifier = []
  | .node (.finite _ _ _) [subject,_] =>
      (∃ category, subject = .gap category) ∨ exposed subject = []
  | .node (.document _) children => childrenExposed children .complement = []
  | .node (.coordinate _ _ _) children | .node (.serialCoordinate _ _) children =>
      childrenExposed children .complement = []
  | .frameCoordination _ a b =>
      childrenExposed a .complement = [] ∧ childrenExposed b .complement = []
  | .sharedCoordination _ _ a b => exposed a = exposed b
  | .node (.rightNodeRaising _ filler) [body,head] =>
      (∃ relation, exposed body = [⟨filler,relation⟩]) ∧ exposed head = []
  | .relativeForm _ _ body form front => RelativeLicense features form front body
  | _ => True

mutual
  def Safe (features : Declarations L) (tree : Syntax L) : Prop :=
    Local features tree ∧ match tree with
      | .node _ children => ChildrenSafe features children
      | .modify a b | .sharedCoordination _ _ a b => Safe features a ∧ Safe features b
      | .relativeForm _ head body _ front =>
          Safe features head ∧ Safe features body ∧ ChildrenSafe features front
      | .frameCoordination _ a b => ChildrenSafe features a ∧ ChildrenSafe features b
      | _ => True
  def ChildrenSafe (features : Declarations L) : List (Syntax L) → Prop
    | [] => True
    | a :: rest => Safe features a ∧ ChildrenSafe features rest
end

/-- Final grammatical candidates satisfy composition, lexical features and dependency constraints.
  -/
def Admitted (lexicon : Lexicon L) (features : Features.Declarations L)
    (dependencies : Declarations L) (tree : Syntax L) (category : Category) (surface : Surface) :
      Prop :=
  Features.Admitted lexicon features tree category surface ∧ Safe dependencies tree

theorem zero_subject_excluded (features : Declarations L) (body : Syntax L) (agreement : Agreement)
    (subject : exposed body = [⟨.nounPhrase agreement,.subject⟩]) :
    ¬ RelativeLicense features .zero [] body := by
  rintro ⟨a,r,notSubject,equation⟩
  rw [subject] at equation
  cases equation
  exact notSubject rfl

theorem adjunct_island (features : Declarations L) (host dependent : Category)
    (head body : Syntax L) (gap : GapUse) (gapped : exposed body = [gap]) :
    ¬ Safe features (.node (.adjunct host dependent) [head,body]) := by
  intro h
  have empty := h.1
  change exposed body = [] at empty
  rw [gapped] at empty
  cases empty

theorem ordinary_coordination_cannot_share (features : Declarations L) (c : Coordinator)
    (category : Category) (a b : Syntax L) (gapped : exposed a ≠ []) :
    ¬ Safe features (.node (.coordinate c category) [a,b]) := by
  intro h
  have empty := h.1
  change exposed a ++ (exposed b ++ []) = [] at empty
  exact gapped (List.append_eq_nil_iff.mp empty).1

theorem wh_category_must_match (features : Declarations L) (head : L) (agreement : Agreement) :
    ¬ RelativePhrase features (.word head (.nounPhrase agreement)) .prepositionPhrase := by
  intro h
  cases h

end English.Dependencies
