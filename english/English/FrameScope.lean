import English.RolePreference

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
    SchemaWitness lexicon category surface → SchemaWitness lexicon category surface → Prop where
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
    {a b : SchemaWitness lexicon category surface} (h : Related a b) :
    lexicalLeaves a.val = lexicalLeaves b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_lexemes h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

mutual
  def anchorCount : Syntax L → Nat
    | .node (.coordinate _ _ _) children | .node (.serialCoordinate _ _) children =>
        childAnchors children + 1
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
    {a b : SchemaWitness lexicon category surface} (h : Related a b) :
    anchorCount a.val = anchorCount b.val := by
  induction h with
  | refl => rfl
  | step h => exact step_preserves_anchors h
  | symm _ ih => exact ih.symm
  | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

/-- The host invariant belongs to `ScopeStep`, not to `Step`. `Step.determiner`,
`Step.sharedHead` and `Step.boundary` are host-restructuring moves by construction — a
determiner distributed across two conjuncts, a head raised out of a coordination, and a
regrouped frame boundary each rewrite the ordered host — so no analogue of
`GrammaticalScope.related_preserves_hosts` holds over `Related`. This exhibits the failure
rather than asserting it: the distribution step changes the host at every instance.
`Related` retains `related_preserves_lexemes` and `related_preserves_anchors`; the host
invariant is `GrammaticalScope.related_preserves_hosts` over `ScopeRelated`. -/
theorem step_can_change_hosts (det a b : Syntax L) :
    ∃ x y : Syntax L, Step x y ∧ hostStructure x ≠ hostStructure y :=
  ⟨_,_,.determiner det a b, by simp [hostStructure, hostChildren, hostNode, group]⟩

def serial (c : Coordinator) (cat : Category) (children : List (Syntax L)) : Syntax L :=
  .node (.serialCoordinate c cat) children

mutual
  /-- Ordered anchors a tree contributes to its host; a non-coordination host is one anchor. -/
  def anchorWidth : Syntax L → Nat
    | .node (.coordinate _ _ _) children | .node (.serialCoordinate _ _) children =>
        childWidth children
    | .sharedCoordination _ _ a b => anchorWidth a + anchorWidth b
    | .frameCoordination _ a b => childWidth a + childWidth b
    | _ => 1
  def childWidth : List (Syntax L) → Nat
    | [] => 0
    | first :: rest => anchorWidth first + childWidth rest
end

mutual
  /-- Ordered anchor topology of a tree, anchors identified by source position. Group arity is
  retained, so a flat coordination and a nested one of the same coordinands differ here even
  where their cardinality agrees. -/
  def anchorsFrom : Nat → Syntax L → Scope.Anchors
    | start,.node (.coordinate _ _ _) children
    | start,.node (.serialCoordinate _ _) children => .group (childAnchorsFrom start children)
    | start,.sharedCoordination _ _ a b =>
        .group [anchorsFrom start a,anchorsFrom (start + anchorWidth a) b]
    | start,.frameCoordination _ a b =>
        .group (childAnchorsFrom start a ++ childAnchorsFrom (start + childWidth a) b)
    | start,_ => .leaf start
  def childAnchorsFrom : Nat → List (Syntax L) → List Scope.Anchors
    | _,[] => []
    | start,first :: rest =>
        anchorsFrom start first :: childAnchorsFrom (start + anchorWidth first) rest
end

/-- The anchor projection the boundary abstraction was written against. -/
def projectAnchors (tree : Syntax L) : Scope.Anchors := anchorsFrom 0 tree

mutual
  /-- The ordered projection never invents an anchor: its group count is bounded by the
  cardinality. Equality fails exactly where a coordination sits under a transparent host, which
  is why the count alone cannot support an anchor-shape law. -/
  theorem anchorCount_anchorsFrom (start : Nat) (tree : Syntax L) :
      (anchorsFrom start tree).groups ≤ anchorCount tree := by
    match tree with
    | .node (.coordinate c cat right) children =>
        have inner := anchorCount_childAnchorsFrom start children
        simp only [anchorsFrom, anchorCount, Scope.Anchors.groups]
        omega
    | .node (.serialCoordinate c cat) children =>
        have inner := anchorCount_childAnchorsFrom start children
        simp only [anchorsFrom, anchorCount, Scope.Anchors.groups]
        omega
    | .sharedCoordination c cat a b =>
        have left := anchorCount_anchorsFrom start a
        have right := anchorCount_anchorsFrom (start + anchorWidth a) b
        simp only [anchorsFrom, anchorCount, Scope.Anchors.groups, Scope.Anchors.groupList]
        omega
    | .frameCoordination c a b =>
        have left := anchorCount_childAnchorsFrom start a
        have right := anchorCount_childAnchorsFrom (start + childWidth a) b
        simp only [anchorsFrom, anchorCount, Scope.Anchors.groups,
          Scope.Anchors.groupList_append]
        omega
    | .noun _ _ | .adjective _ | .marker _ | .word _ _ | .identity _ _ | .gap _
    | .ellipsis _ _ | .modify _ _ | .relativeForm _ _ _ _ _
    | .node (.document _) _ | .node (.keyword _ _ _) _ | .node (.determine _) _
    | .node .barePlural _ | .node .bareMass _ | .node (.attributive _ _) _
    | .node (.targeting _ _) _ | .node (.rightNodeRaising _ _) _ | .node (.genitive _) _
    | .node (.quantify _) _ | .node (.compare _) _ | .node (.measure _) _
    | .node (.preposition _ _) _ | .node (.verb _ _ _ _) _ | .node (.auxiliary _ _ _ _ _ _) _
    | .node (.finite _ _ _) _ | .node (.initialAdverbial _) _ | .node .imperative _
    | .node (.nonfinite _ _) _ | .node (.subordinate _ _) _ | .node (.adjunct _ _ _) _ =>
        simp [anchorsFrom, Scope.Anchors.groups]
  theorem anchorCount_childAnchorsFrom (start : Nat) (children : List (Syntax L)) :
      Scope.Anchors.groupList (childAnchorsFrom start children) ≤ childAnchors children := by
    match children with
    | [] => simp [childAnchorsFrom, childAnchors, Scope.Anchors.groupList]
    | first :: rest =>
        have head := anchorCount_anchorsFrom start first
        have tail := anchorCount_childAnchorsFrom (start + anchorWidth first) rest
        simp only [childAnchorsFrom, childAnchors, Scope.Anchors.groupList]
        omega
end

/-- The arithmetic the flat/nested separation rests on, independent of any derivation. -/
private theorem flat_nested_anchors (c : Coordinator) (cat : Category) (a b d : Syntax L) :
    anchorCount (serial c cat [a,b,d]) ≠ anchorCount (group c cat a (group c cat b d)) := by
  simp [serial, group, anchorCount, childAnchors]

/-- Flat serial coordination and the nested binary bracketing of the same three coordinands are
both derivable, and the anchor cardinality separates them. This is a property of the declared
productions and linearizations, not a claim that the nested analysis is wrong for Oracle
English. -/
theorem flat_nested_differ {lexicon : Lexicon L} {category : Category} {c : Coordinator}
    {cat : Category} {a b d : Syntax L}
    (flat : Derives lexicon (serial c cat [a,b,d]) category)
    (nested : Derives lexicon (group c cat a (group c cat b d)) category) :
    ∃ flatTree nestedTree : Syntax L, Derives lexicon flatTree category ∧
      Derives lexicon nestedTree category ∧ anchorCount flatTree ≠ anchorCount nestedTree :=
  ⟨_,_,flat,nested,flat_nested_anchors c cat a b d⟩

/-- Cardinality alone would merge shapes the ordered projection separates: a binary and a flat
three-item coordination have the same anchor count and different anchor topologies. -/
theorem projectAnchors_flat_ne_nested {lexicon : Lexicon L} {category : Category}
    {pair triple : Syntax L}
    (pairDerives : Derives lexicon pair category) (tripleDerives : Derives lexicon triple category)
    (sameCount : anchorCount pair = anchorCount triple)
    (pairShape : projectAnchors pair = .group [.leaf 0,.leaf 1])
    (tripleShape : projectAnchors triple = Scope.flat) :
    ∃ x y : Syntax L, Derives lexicon x category ∧ Derives lexicon y category ∧
      anchorCount x = anchorCount y ∧ projectAnchors x ≠ projectAnchors y := by
  refine ⟨pair,triple,pairDerives,tripleDerives,sameCount,?_⟩
  rw [pairShape,tripleShape]
  simp [Scope.flat]

/-- The anchor-shape law of `Scope.anchor_shape_separate`, over derivable trees whose ordered
projections are the two shapes it names. -/
theorem anchor_shape_separate {lexicon : Lexicon L} {category : Category}
    (survivors : Scope.AnchorPattern → Prop)
    (p : Preference.Package Scope.AnchorPattern Scope.Anchors)
    (packed : Preference.Packs survivors Scope.AnchorPattern.anchors p)
    {flatTree nestedTree : Syntax L}
    (flatDerives : Derives lexicon flatTree category)
    (nestedDerives : Derives lexicon nestedTree category)
    (flatShape : projectAnchors flatTree = Scope.flat)
    (nestedShape : projectAnchors nestedTree = Scope.nested)
    (flatSites nestedSites : List (List Nat)) :
    ∃ x y : Syntax L, Derives lexicon x category ∧ Derives lexicon y category ∧
      ¬ (p.readings ⟨projectAnchors x,flatSites⟩ ∧ p.readings ⟨projectAnchors y,nestedSites⟩) := by
  refine ⟨flatTree,nestedTree,flatDerives,nestedDerives,?_⟩
  rw [flatShape,nestedShape]
  exact Scope.anchor_shape_separate survivors p packed flatSites nestedSites

def setoid (lexicon : Lexicon L) (category : Category) (surface : Surface) :
    Setoid (SchemaWitness lexicon category surface) where
  r := Related
  iseqv := ⟨Related.refl, Related.symm, Related.trans⟩
def key {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (a : SchemaWitness lexicon category surface) := Quotient.mk (setoid lexicon category surface) a

theorem key_exact {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (a b : SchemaWitness lexicon category surface) : key a = key b ↔ Related a b :=
  ⟨Quotient.exact, fun h ↦ Quotient.sound (s := setoid lexicon category surface) h⟩

theorem attachment_included {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {a b : SchemaWitness lexicon category surface} (h : ScopeRelated a b) : Related a b := by
  induction h with
  | refl => exact .refl _
  | step h => exact .step (.attachment h)
  | symm _ ih => exact .symm ih
  | trans _ _ ih₁ ih₂ => exact .trans ih₁ ih₂

mutual
  /-- The declared role sequence determines which edge is opaque to an enclosing search. -/
  def projectHost : Syntax L → Scope.Host
    | .node (.coordinate c _ _) [a,b] => .coordinate c (projectHost a) (projectHost b)
    | .node (.serialCoordinate c _) children => serialHost c children
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
  /-- A flat serial coordination has the same right-periphery topology as the binary rule
  applied to its coordinands in order: only the last one stays reachable. -/
  def serialHost : Coordinator → List (Syntax L) → Scope.Host
    | _,[] => .leaf 0
    | _,[only] => projectHost only
    | c,first :: rest => .coordinate c (projectHost first) (serialHost c rest)
end

theorem actual_nonfinal_boundary (c : Coordinator) (cat : Category) (a b : Syntax L)
    (rest : List Nat) :
    (0 :: rest) ∉ (projectHost (group c cat a b)).sites := Scope.nonfinal_boundary _ _ _ _

theorem actual_role_boundary (marker : L) (role : FrameSlot) (a : Syntax L)
    (index : Nat) (rest : List Nat) :
    (index :: rest) ∉ (frameHost [.marker marker,a] [.marked marker role]).sites :=
  Scope.role_edge_opaque _ _ _

theorem fixed_marker_transparent (marker : L) (slot : FrameSlot) (a : Syntax L) :
    frameHost [.marker marker,a] [.fixed marker,.argument slot] =
      frameHost [a] [.argument slot] := rfl

theorem fixed_coordination_transparent (marker : L) (slot : FrameSlot)
    (c : Coordinator) (cat : Category) (a b : Syntax L) :
    [1] ∈ (frameHost [.marker marker,group c cat a b]
      [.fixed marker,.argument slot]).sites := by
  have root : [] ∈ (projectHost b).sites := by
    cases projectHost b <;> simp [Scope.Host.sites]
  simpa [frameHost,group,projectHost,Scope.Host.sites] using root

theorem actual_first_unique (tree : Syntax L) (eligible : List Nat → Prop) (a b : List Nat)
    (ha : Scope.FirstEligible eligible (projectHost tree).sites a)
    (hb : Scope.FirstEligible eligible (projectHost tree).sites b) : a = b :=
  Scope.first_eligible_unique ha hb

end English.FrameScope
