import English.Grammar

/-! Feature judgments refine category composition before preference or scope packing. -/
namespace English.Features
variable {L : Type}

inductive Countability where
  | count | mass
  deriving DecidableEq

/-- Uses are declarations, so one noun may have both count and mass uses. -/
structure Declarations (L : Type) where
  nounUse : L → Countability → Prop
  determinerUse : L → Countability → Prop
  temporalNoun : L → Prop

inductive NominalUse (features : Declarations L) : Syntax L → Countability → Prop where
  | noun {head : L} {number : Number} {use : Countability} : features.nounUse head use →
      NominalUse features (.noun head number) use
  | modify {modifier head : Syntax L} {use : Countability} : NominalUse features head use →
      NominalUse features (.modify modifier head) use
  | attributive {marker : L} {number : Number} {head : Syntax L} {use : Countability} :
      NominalUse features head use → NominalUse features (.node (.attributive marker number)
        [head]) use
  | targeting {marker : L} {number : Number} {head : Syntax L} {use : Countability} :
      NominalUse features head use → NominalUse features (.node (.targeting marker number) [head])
        use
  | relative {number : Number} {head body : Syntax L} {form : RelativeForm} {front : List (Syntax
    L)}
      {use : Countability} :
      NominalUse features head use → NominalUse features (.relative number head body form front) use
  | adjunct {number : Number} {head dependent : Syntax L} {use : Countability} :
      NominalUse features head use →
      NominalUse features (.node (.adjunct (.nominal number) .prepositionPhrase) [head,dependent])
        use
  | rightNodeRaising {number : Number} {body head : Syntax L} {use : Countability} :
      NominalUse features head use → NominalUse features
        (.node (.rightNodeRaising (.nominal number) (.nominal number)) [body,head]) use
  | coordinate {c : Coordinator} {number : Number} {a b : Syntax L} {use : Countability} :
      NominalUse features a use → NominalUse features b use →
      NominalUse features (.node (.coordinate c (.nominal number)) [a,b]) use

inductive DeterminerUse (features : Declarations L) : Syntax L → Countability → Prop where
  | word {head : L} {number : Number} {use : Countability} : features.determinerUse head use →
      DeterminerUse features (.word head (.determinativePhrase number)) use
  | numeral {number : Number} {numeral : Syntax L} :
      DeterminerUse features (.node (.quantify number) [numeral]) .count
  | genitive {number : Number} {possessor : Syntax L} {use : Countability} :
      DeterminerUse features (.node (.genitive number) [possessor]) use

/-- Targeting precedes descriptive modifiers; adjective ordering otherwise stays open. -/
def containsTarget : Syntax L → Bool
  | .node (.targeting _ _) _ => true
  | .modify _ head | .node (.attributive _ _) [head] | .relativeForm _ head _ _ _ =>
    containsTarget head
  | .node (.coordinate _ _ _) [a,b] => containsTarget a || containsTarget b
  | .node (.adjunct _ _ _) [head,_] => containsTarget head
  | _ => false

inductive Temporal (features : Declarations L) : Syntax L → Prop where
  | noun {head : L} {number : Number} : features.temporalNoun head →
      Temporal features (.noun head number)
  | determine {number : Number} {det head : Syntax L} : Temporal features head →
      Temporal features (.node (.determine number) [det,head])
  | modify {modifier head : Syntax L} : Temporal features head →
      Temporal features (.modify modifier head)
  | coordinate {c : Coordinator} {category : Category} {a b : Syntax L} :
      Temporal features a → Temporal features b →
      Temporal features (.node (.coordinate c category) [a,b])

/-- These checks complement category/number checking; none consult a spelling or card identity. -/
def Local (features : Declarations L) : Syntax L → Prop
  | .node (.determine _) [det,head] =>
      ∃ use, NominalUse features head use ∧ DeterminerUse features det use
  | .node .barePlural [head] => NominalUse features head .count
  | .node .bareMass [head] => NominalUse features head .mass
  | .modify _ head | .node (.attributive _ _) [head] | .node (.targeting _ _) [head] =>
      containsTarget head = false
  | .node (.adjunct (.verbPhrase _ _) (.nounPhrase _) _) [_,dependent] =>
      Temporal features dependent
  | _ => True

mutual
  def Conforms (features : Declarations L) (tree : Syntax L) : Prop :=
    Local features tree ∧ match tree with
      | .relativeForm _ a b _ front =>
          Conforms features a ∧ Conforms features b ∧ ChildrenConform features front
      | .modify a b | .sharedCoordination _ _ a b =>
          Conforms features a ∧ Conforms features b
      | .node _ children => ChildrenConform features children
      | .frameCoordination _ a b => ChildrenConform features a ∧ ChildrenConform features b
      | _ => True
  def ChildrenConform (features : Declarations L) : List (Syntax L) → Prop
    | [] => True
    | a :: rest => Conforms features a ∧ ChildrenConform features rest
end

/-- The reviewed grammar uses this intersection, not raw category derivation, for candidates. -/
def Admitted (lexicon : Lexicon L) (features : Declarations L) (tree : Syntax L)
    (category : Category) (surface : Surface) : Prop :=
  Admissible lexicon tree category surface ∧ Conforms features tree

theorem admitted_base {lexicon : Lexicon L} {features : Declarations L} {tree : Syntax L}
    {category : Category} {surface : Surface} (h : Admitted lexicon features tree category
      surface) :
    Admissible lexicon tree category surface := h.1

theorem numeral_rejects_mass (features : Declarations L) (number : Number)
    (quantity head : Syntax L) (massOnly : ¬ NominalUse features head .count) :
    ¬ Local features (.node (.determine number) [.node (.quantify number) [quantity],head]) := by
  rintro ⟨use,nominal,determiner⟩
  cases determiner
  exact massOnly nominal

theorem target_must_be_outer (features : Declarations L) (marker : L) (number : Number)
    (modifier head : Syntax L) :
    ¬ Local features (.modify modifier (.node (.targeting marker number) [head])) := by
  simp [Local, containsTarget]

theorem temporal_adjunct_requires_feature (features : Declarations L) (form : InflectionalForm)
    (voice : Voice) (agreement : Agreement) (predicate dependent : Syntax L)
    (h : ¬ Temporal features dependent) :
    ¬ Local features
      (.node (.adjunct (.verbPhrase form voice) (.nounPhrase agreement)) [predicate,dependent]) := h

end English.Features
