import English.Grammar

/-! Nominal Case licences are independent of NP agreement and grammatical relation. -/
namespace English

inductive Case where
  | noCommonCase | nominative | accusative | genitive | nominativeOrAccusative
  deriving DecidableEq

inductive CasePosition where
  | nominative | accusative | genitive
  deriving DecidableEq

def Case.Allows : Case → CasePosition → Prop
  | .nominative, .nominative | .accusative, .accusative | .genitive, .genitive
  | .nominativeOrAccusative, .nominative | .nominativeOrAccusative, .accusative => True
  | _, _ => False

instance (value : Case) (position : CasePosition) : Decidable (value.Allows position) := by
  cases value <;> cases position <;> unfold Case.Allows <;> infer_instance

def Case.common : Case → Case → Case
  | .nominative, .nominative | .nominative, .nominativeOrAccusative
  | .nominativeOrAccusative, .nominative => .nominative
  | .accusative, .accusative | .accusative, .nominativeOrAccusative
  | .nominativeOrAccusative, .accusative => .accusative
  | .genitive, .genitive => .genitive
  | .nominativeOrAccusative, .nominativeOrAccusative => .nominativeOrAccusative
  | _, _ => .noCommonCase

theorem Case.common_allows (left right : Case) (position : CasePosition) :
    (left.common right).Allows position ↔ left.Allows position ∧ right.Allows position := by
  cases left <;> cases right <;> cases position <;> simp [common, Allows]

theorem Case.common_associative (first second third : Case) :
    (first.common second).common third = first.common (second.common third) := by
  cases first <;> cases second <;> cases third <;> rfl

def Case.Argument : Case → Prop
  | .nominative | .accusative | .nominativeOrAccusative => True
  | .noCommonCase | .genitive => False

def Relation.casePosition : Relation → CasePosition
  | .subject => .nominative
  | .object | .complement => .accusative

mutual
  /-- Ordinary headed NPs are Case-invariant; lexical NPs declare their own licences. -/
  def Syntax.nominalCase {L : Type} (wordCase : L → Case) (tree : Syntax L)
      (gapCase : Case := .nominativeOrAccusative) : Case :=
    match tree with
    | .word head (.nounPhrase _) | .identity head (.nounPhrase _) => wordCase head
    | .gap (.nounPhrase _) => gapCase
    | .node (.determine _) _ | .node .barePlural _ | .node .bareMass _ => .nominativeOrAccusative
    | .node (.coordinate _ (.nounPhrase _) (.nounPhrase _)) [left, right]
    | .sharedCoordination _ (.nounPhrase _) left right =>
        (left.nominalCase wordCase gapCase).common (right.nominalCase wordCase gapCase)
    | .node (.serialCoordinate _ (.nounPhrase _)) children =>
        childrenNominalCase wordCase children gapCase
    | .node (.adjunct (.nounPhrase _) _ _) [head, _] => head.nominalCase wordCase gapCase
    | .node (.rightNodeRaising (.nounPhrase _) _) [body, head] =>
        body.nominalCase wordCase (head.nominalCase wordCase gapCase)
    | _ => .noCommonCase
  /-- Every coordinand of a flat serial coordination licenses the same Case, as in the binary
  rule; an empty coordination licenses none. -/
  def childrenNominalCase {L : Type} (wordCase : L → Case) (trees : List (Syntax L))
      (gapCase : Case := .nominativeOrAccusative) : Case :=
    match trees with
    | [] => .noCommonCase
    | [only] => only.nominalCase wordCase gapCase
    | first :: rest =>
        (first.nominalCase wordCase gapCase).common (childrenNominalCase wordCase rest gapCase)
end

def CaseAt {L : Type} (wordCase : L → Case) (relation : Relation)
    (category : Category) (tree : Syntax L)
    (gapCase : Case := .nominativeOrAccusative) : Prop :=
  match category with
  | .nounPhrase _ => (tree.nominalCase wordCase gapCase).Allows relation.casePosition
  | _ => True

/-- The category judgment checks schema identity; this judgment checks nominal role licences. -/
def FrameCases {L : Type} (wordCase : L → Case)
    (children : List (Syntax L)) (frame : List (FrameItem L))
    (gapCase : Case := .nominativeOrAccusative) : Prop :=
  match children, frame with
  | [], [] => True
  | [.frameCoordination _ left right], _ :: _ =>
      FrameCases wordCase left frame gapCase ∧ FrameCases wordCase right frame gapCase
  | head :: rest, .argument complement :: tail =>
      CaseAt wordCase complement.relation complement.category head gapCase ∧
        FrameCases wordCase rest tail gapCase
  | _ :: rest, .fixed _ :: tail => FrameCases wordCase rest tail gapCase
  | _ :: head :: rest, .marked _ complement :: tail =>
      CaseAt wordCase complement.relation complement.category head gapCase ∧
        FrameCases wordCase rest tail gapCase
  | _, _ => False
termination_by sizeOf children

end English
