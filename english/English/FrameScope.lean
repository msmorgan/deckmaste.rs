import English.FrameInteractions

/-! Partial-frame boundary variation, with declared frames and actual grammatical readings. -/
namespace English.FrameScope
open GrammaticalScope
variable {L : Type}

def group (c : Coordinator) (cat : Category) (a b : Syntax L) : Syntax L :=
  .node (.coordinate c cat) [a,b]
def pairs (c : Coordinator) (marker : L) (a b d e : Syntax L) : Syntax L :=
  .frameCoordination c [a,.marker marker,b] [d,.marker marker,e]

/-- Each side has three binary anchors, in the same source order. Only their nesting changes. -/
inductive BoundaryMove : Syntax L → Syntax L → Prop where
  | exchange (c : Coordinator) (cat : Category) (marker : L) (a b d e f g : Syntax L) :
      BoundaryMove
        (pairs c marker (group c cat a b) (group c cat d e) f g)
        (pairs c marker (group c cat a b) d (group c cat e f) g)

/-- This relation extends the earlier attachment fragment; it does not redefine its invariants. -/
inductive Step : Syntax L → Syntax L → Prop where
  | attachment {a b} : ScopeStep a b → Step a b
  | boundary {a b} : BoundaryMove a b → Step a b
  | determiner (det a b : Syntax L) :
      Step (.node (.determine .plural) [det,group .and_ (.nominal .plural) a b])
        (group .and_ (.nounPhrase ⟨.third,.plural⟩)
          (.node (.determine .plural) [det,a]) (.node .barePlural [b]))
  | sharedHead (a b head : Syntax L) (number : Number) :
      Step (.modify (group .and_ .adjectivePhrase a b) head)
        (.node (.rightNodeRaising (.nominal number) (.nominal number))
          [.sharedCoordination .and_ (.nominal number)
            (.modify a (.gap (.nominal number))) (.modify b (.gap (.nominal number))),head])
  | node {a b} (construction : Construction L) (before after : List (Syntax L)) :
      Step a b → Step (.node construction (before ++ [a] ++ after))
        (.node construction (before ++ [b] ++ after))

inductive Related {lexicon : Lexicon L} {category : Category} {surface : Surface} :
    Reading lexicon category surface → Reading lexicon category surface → Prop where
  | refl (a) : Related a a
  | step {a b} : Step a.val b.val → Related a b
  | symm {a b} : Related a b → Related b a
  | trans {a b c} : Related a b → Related b c → Related a c

private theorem leaves_append (a b : List (Syntax L)) :
    childLeaves (a ++ b) = childLeaves a ++ childLeaves b := by
  induction a with
  | nil => rfl
  | cons a as ih => simp [childLeaves, ih, List.append_assoc]

theorem boundary_preserves_lexemes {a b : Syntax L} (h : BoundaryMove a b) :
    lexicalLeaves a = lexicalLeaves b := by
  cases h
  simp [pairs, group, lexicalLeaves, childLeaves, constructionLexemes, List.append_assoc]

theorem step_preserves_lexemes {a b : Syntax L} (h : Step a b) :
    lexicalLeaves a = lexicalLeaves b := by
  induction h with
  | attachment h => exact GrammaticalScope.step_preserves_lexemes h
  | boundary h => exact boundary_preserves_lexemes h
  | determiner det a b =>
    simp [group,lexicalLeaves,childLeaves,constructionLexemes,List.append_assoc]
  | sharedHead a b head number =>
    simp [group,lexicalLeaves,childLeaves,constructionLexemes,List.append_assoc]
  | node construction before after _ ih =>
    simp [lexicalLeaves, leaves_append, childLeaves, ih]

theorem related_preserves_lexemes {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : Reading lexicon category surface} (h : Related a b) :
    lexicalLeaves a.val = lexicalLeaves b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_lexemes h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

mutual
  def anchorCount : Syntax L → Nat
    | .node (.coordinate _ _ _) children => childAnchors children + 1
    | .node _ children => childAnchors children
    | .frameCoordination _ a b => childAnchors a + childAnchors b + 1
    | .sharedCoordination _ _ a b => anchorCount a + anchorCount b + 1
    | .relativeForm _ a b _ front => anchorCount a + anchorCount b + childAnchors front
    | .modify a b => anchorCount a + anchorCount b
    | _ => 0
  def childAnchors : List (Syntax L) → Nat
    | [] => 0
    | a :: rest => anchorCount a + childAnchors rest
end
private theorem anchors_append (a b : List (Syntax L)) :
    childAnchors (a ++ b) = childAnchors a + childAnchors b := by
  induction a with
  | nil => simp [childAnchors]
  | cons a as ih => simp [childAnchors, ih, Nat.add_assoc]
private theorem node_anchors (c : Construction L) (a b : List (Syntax L))
    (h : childAnchors a = childAnchors b) :
    anchorCount (.node c a) = anchorCount (.node c b) := by
  cases c <;> simp_all [anchorCount]
private theorem attachment_anchors {a b : Syntax L} (h : ScopeStep a b) :
    anchorCount a = anchorCount b := by
  induction h with
  | direct h =>
    cases h <;> simp [anchorCount, childAnchors, coord, adjunct, Nat.add_assoc,
      Nat.add_comm, Nat.add_left_comm]
    all_goals split <;> simp [anchorCount, childAnchors, Nat.add_assoc,
      Nat.add_comm, Nat.add_left_comm]
  | node c before after _ ih =>
    apply node_anchors
    simp [anchors_append, childAnchors, ih]
  | modifierLeft head _ ih => simp [anchorCount, ih]
  | modifierRight modifier _ ih => simp [anchorCount, ih]
theorem step_preserves_anchors {a b : Syntax L} (h : Step a b) :
    anchorCount a = anchorCount b := by
  induction h with
  | attachment h => exact attachment_anchors h
  | boundary h =>
    cases h
    simp [pairs, group, anchorCount, childAnchors, Nat.add_assoc, Nat.add_comm,
      Nat.add_left_comm]
  | determiner det a b =>
    simp [group,anchorCount,childAnchors,Nat.add_assoc]
  | sharedHead a b head number =>
    simp [group,anchorCount,childAnchors,Nat.add_assoc]
  | node c before after _ ih =>
    apply node_anchors
    simp [anchors_append, childAnchors, ih]
theorem related_preserves_anchors {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : Reading lexicon category surface} (h : Related a b) :
    anchorCount a.val = anchorCount b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_anchors h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

theorem flat_nested_differ (c : Coordinator) (cat : Category) (a b d : Syntax L) :
    anchorCount (.node (.coordinate c cat) [a,b,d]) ≠
      anchorCount (group c cat a (group c cat b d)) := by
  simp [anchorCount, childAnchors, group]

def setoid (lexicon : Lexicon L) (category : Category) (surface : Surface) :
    Setoid (Reading lexicon category surface) where
  r := Related
  iseqv := ⟨Related.refl, Related.symm, Related.trans⟩
def key {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (a : Reading lexicon category surface) := Quotient.mk (setoid lexicon category surface) a

theorem key_exact {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (a b : Reading lexicon category surface) : key a = key b ↔ Related a b :=
  ⟨Quotient.exact, fun h ↦ Quotient.sound (s := setoid lexicon category surface) h⟩

theorem attachment_included {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : Reading lexicon category surface} (h : ScopeRelated a b) : Related a b := by
  induction h with
  | refl => exact .refl _
  | step h => exact .step (.attachment h)
  | symm _ ih => exact .symm ih
  | trans _ _ ih₁ ih₂ => exact .trans ih₁ ih₂

mutual
  /-- The declared role sequence determines which edge is opaque to an enclosing search. -/
  def projectHost : Syntax L → Scope.Host
    | .node (.coordinate c _ _) [a,b] => .coordinate c (projectHost a) (projectHost b)
    | .node (.auxiliary _ _ _ _ _ _) [a] => .auxiliary (projectHost a)
    | .node (.verb _ _ frame _) children => .frame (frameHost children frame)
    | .node (.preposition _ _) [a] => .role (projectHost a)
    | .node (.adjunct _ _ _) [a,_] | .node .barePlural [a]
    | .node (.determine _) [_,a] | .modify _ a => projectHost a
    | _ => .leaf 0
  def frameHost : List (Syntax L) → List (FrameItem L) → Scope.Host
    | [.frameCoordination c a b],frame => .coordinate c (frameHost a frame) (frameHost b frame)
    | [.marker _,a],[.marked _ _] => .role (projectHost a)
    | [a],[.argument _] => projectHost a
    | _ :: rest,.argument _ :: frame => frameHost rest frame
    | .marker _ :: _ :: rest,.marked _ _ :: frame => frameHost rest frame
    | .marker _ :: rest,.fixed _ :: frame => frameHost rest frame
    | _,_ => .leaf 0
end

theorem actual_nonfinal_boundary (c : Coordinator) (cat : Category) (a b : Syntax L)
    (rest : List Nat) :
    (0 :: rest) ∉ (projectHost (group c cat a b)).sites := Scope.nonfinal_boundary _ _ _ _

theorem actual_role_boundary (marker : L) (role : Complement) (a : Syntax L)
    (index : Nat) (rest : List Nat) :
    (index :: rest) ∉ (frameHost [.marker marker,a] [.marked marker role]).sites :=
  Scope.role_edge_opaque _ _ _

theorem fixed_marker_transparent (marker : L) (complement : Complement) (a : Syntax L) :
    frameHost [.marker marker,a] [.fixed marker,.argument complement] =
      frameHost [a] [.argument complement] := rfl

theorem fixed_coordination_transparent (marker : L) (complement : Complement)
    (c : Coordinator) (cat : Category) (a b : Syntax L) :
    [1] ∈ (frameHost [.marker marker,group c cat a b]
      [.fixed marker,.argument complement]).sites := by
  have root : [] ∈ (projectHost b).sites := by
    cases projectHost b <;> simp [Scope.Host.sites]
  simpa [frameHost,group,projectHost,Scope.Host.sites] using root

theorem actual_first_unique (tree : Syntax L) (eligible : List Nat → Prop) (a b : List Nat)
    (ha : Scope.FirstEligible eligible (projectHost tree).sites a)
    (hb : Scope.FirstEligible eligible (projectHost tree).sites b) : a = b :=
  Scope.first_eligible_unique ha hb

namespace Witnesses
open FrameInteractions
open Documents (Witness)

def cluster (a b d e : Witness Lexeme lexicon nominalCategory) :
    Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .put .plain markedFrame) [pairs .and_ .on a.tree b.tree d.tree e.tree],
   (["put"] : Surface) ++ a.surface ++ (["on"] : Surface) ++ b.surface ++ (["and"] : Surface) ++
     d.surface ++ (["on"] : Surface) ++ e.surface,
   .verb ⟨rfl,rfl,rfl,Or.inr rfl⟩ (.coordinate
     (.argument (complement := ⟨.object,nominalCategory⟩) a.derives
       (.marked (complement := role) b.derives .nil))
     (.argument (complement := ⟨.object,nominalCategory⟩) d.derives
       (.marked (complement := role) e.derives .nil))),
   by
     have h := Realizes.node (.cons (Realizes.frameCoordination (coordinator := .and_)
       (.cons a.realizes (.cons (.marker (Or.inl ⟨rfl,rfl⟩)) (.cons b.realizes .nil)))
       (.cons d.realizes (.cons (.marker (Or.inl ⟨rfl,rfl⟩)) (.cons e.realizes .nil)))) .nil)
       (Linearizes.verb (lexicon := lexicon) (head := .put) (form := .plain)
         (frame := markedFrame) (voice := .active) (v := ["put"]) ⟨rfl,rfl,rfl⟩)
     simpa [pairs, Coordinator.surface, List.append_assoc] using h⟩

def left := cluster (pair creatures artifacts) (pair creatures artifacts) creatures artifacts
def right := cluster (pair creatures artifacts) creatures (pair artifacts creatures) artifacts

theorem same_surface : left.surface = right.surface := rfl

theorem both_written :
    Written lexicon left.tree (.verbPhrase .plain)
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" ∧
    Written lexicon right.tree (.verbPhrase .plain)
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" := by
  have spells : Spells left.surface
      "put creatures and artifacts on creatures and artifacts and creatures on artifacts" :=
    .cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))))))
  exact ⟨⟨_,⟨left.derives,left.realizes⟩,spells⟩,
    ⟨_,⟨right.derives,right.realizes⟩,spells⟩⟩

theorem distinct : left.tree ≠ right.tree := by intro h; cases h

def leftReading : Reading lexicon (.verbPhrase .plain) left.surface :=
  ⟨left.tree,left.derives,left.realizes⟩
def rightReading : Reading lexicon (.verbPhrase .plain) left.surface :=
  ⟨right.tree,right.derives,right.realizes⟩

theorem related : Related leftReading rightReading :=
  .step (.node (.verb Lexeme.put .plain markedFrame) [] []
    (.boundary (.exchange .and_ nominalCategory Lexeme.on creatures.tree artifacts.tree
      creatures.tree artifacts.tree creatures.tree artifacts.tree)))

theorem both_packed :
    (Selection.pack (fun a ↦ a = leftReading ∨ a = rightReading) key (key leftReading)).readings
      leftReading ∧
    (Selection.pack (fun a ↦ a = leftReading ∨ a = rightReading) key (key leftReading)).readings
      rightReading :=
  ⟨⟨Or.inl rfl,rfl⟩,⟨Or.inr rfl,(key_exact _ _).mpr (.symm related)⟩⟩

theorem same_role_count : RolePreference.roleCount left.tree =
    RolePreference.roleCount right.tree := rfl

theorem no_boundary_preference :
    ¬ RolePreference.Prefers lexicon left.tree right.tree ∧
    ¬ RolePreference.Prefers lexicon right.tree left.tree :=
  ⟨RolePreference.equal_role_counts_cannot_prefer same_role_count,
   RolePreference.equal_role_counts_cannot_prefer same_role_count.symm⟩

end Witnesses
end English.FrameScope
