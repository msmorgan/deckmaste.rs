import English.GrammaticalScope
import English.Scope

namespace English.RolePreference
variable {L : Type}
def object (agreement : Agreement) : FrameItem L := .argument ⟨.object, .nounPhrase agreement⟩
def selectedTree (head marker : L) (form : InflectionalForm) (voice : Voice)
    (agreement : Agreement) (role : Complement) (obj value : Syntax L) : Syntax L :=
  .node (.verb head form [object agreement, .marked marker role] voice)
    [obj,.marker marker,value]
def postmodifierTree (head marker : L) (form : InflectionalForm) (voice : Voice)
    (agreement : Agreement) (role : Complement) (obj value : Syntax L) : Syntax L :=
  .node (.verb head form [object agreement] voice)
    [.node (.adjunct (.nounPhrase agreement) .prepositionPhrase)
      [obj,.node (.preposition marker role.category) [value]]]
/-- PP occurrences on governed material's right periphery, in source order.
A PP's own complement is not another occurrence on its enclosing noun's edge. -/
def postmodifiers : Syntax L → List (Syntax L)
  | .node (.adjunct _ .prepositionPhrase .after) [head, mobile] => postmodifiers head ++ [mobile]
  | .node (.coordinate _ _ _) [_, right] => postmodifiers right
  | .node .barePlural [head] | .node (.determine _) [_, head] => postmodifiers head
  | .modify _ head => postmodifiers head
  | _ => []

def matchesRole (marker : L) (role : Complement) : Syntax L → Prop
  | .node (.preposition head category) [_] => head = marker ∧ category = role.category
  | _ => False

/-- The immediately following candidate cannot skip an earlier eligible occurrence. -/
def FirstRole (marker : L) (role : Complement) (obj : Syntax L) : Prop :=
  ∀ candidate ∈ postmodifiers obj, ¬ matchesRole marker role candidate

inductive RoleStep (lexicon : Lexicon L) : Syntax L → Syntax L → Prop where
  | immediate {head marker : L} {form : InflectionalForm} {voice : Voice}
      {agreement : Agreement} {role : Complement} {obj value : Syntax L} :
      lexicon.verb head form voice [object agreement, .marked marker role] →
      lexicon.verb head form voice [object agreement] →
      lexicon.preposition marker role.category →
      FirstRole marker role obj →
      Derives lexicon obj (.nounPhrase agreement) →
      Derives lexicon value role.category →
      RoleStep lexicon (selectedTree head marker form voice agreement role obj value)
        (postmodifierTree head marker form voice agreement role obj value)
theorem role_step_derives {lexicon : Lexicon L} {better worse : Syntax L}
    (step : RoleStep lexicon better worse) :
    ∃ form voice, Derives lexicon better (.verbPhrase form voice) ∧
      Derives lexicon worse (.verbPhrase form voice) := by
  cases step with
  | @immediate head marker form voice agreement role obj value selected base prep _ objD valueD =>
    exact ⟨_,_, .verb selected
      (.argument (complement := ⟨.object, .nounPhrase agreement⟩) objD
        (.marked valueD .nil)),
      .verb base (.argument (complement := ⟨.object, .nounPhrase agreement⟩)
        (.node (context := []) (.adjunct .nounPhrase)
          (.cons objD (.cons (.node (.preposition prep) (.cons valueD .nil)) .nil))) .nil)⟩
theorem immediate_same_surface {lexicon : Lexicon L}
    {head marker : L} {form : InflectionalForm} {voice : Voice}
    {agreement : Agreement} {role : Complement} {obj value : Syntax L}
    {v m os vs : Surface}
    (verbForm : lexicon.verbForm head form v) (markerForm : lexicon.markerForm marker m)
    (objectForm : Realizes lexicon obj os) (valueForm : Realizes lexicon value vs) :
    Realizes lexicon (selectedTree head marker form voice agreement role obj value)
      (v ++ os ++ m ++ vs) ∧
    Realizes lexicon (postmodifierTree head marker form voice agreement role obj value)
      (v ++ os ++ m ++ vs) := by
  constructor
  · simpa [selectedTree, List.append_assoc] using
      (Realizes.node (.cons objectForm (.cons (.marker markerForm) (.cons valueForm .nil)))
        (Linearizes.verb (head := head) (frame := [object agreement, .marked marker role])
          (voice := voice) verbForm))
  · have pp := Realizes.node (.cons valueForm .nil)
      (Linearizes.preposition (category := role.category) markerForm)
    have post := Realizes.node (.cons objectForm (.cons pp .nil))
      (Linearizes.adjunct (host := .nounPhrase agreement) (dependent := .prepositionPhrase))
    simpa [postmodifierTree, List.append_assoc] using
      (Realizes.node (.cons post .nil)
        (Linearizes.verb (head := head) (frame := [object agreement]) (voice := voice) verbForm))
theorem nonfinal_occurrence_excluded (coordinator : Coordinator) (category : Category)
    (left right : Syntax L) :
    postmodifiers (.node (.coordinate coordinator category) [left, right]) =
      postmodifiers right := rfl

theorem occurrence_order (category : Category) (obj first second : Syntax L) :
    postmodifiers (.node (.adjunct category .prepositionPhrase)
      [.node (.adjunct category .prepositionPhrase) [obj, first], second]) =
      postmodifiers obj ++ [first, second] := by
  simp [postmodifiers, List.append_assoc]

theorem role_candidate_cannot_skip {marker : L} {role : Complement} {obj first : Syntax L}
    {category : Category} (eligible : matchesRole marker role first) :
    ¬ FirstRole marker role (.node (.adjunct category .prepositionPhrase) [obj, first]) := by
  intro h
  exact h first (by simp [postmodifiers]) eligible

def markedCount : List (FrameItem L) → Nat
  | [] => 0
  | .marked _ _ :: rest => markedCount rest + 1
  | _ :: rest => markedCount rest

mutual
  def roleCount : Syntax L → Nat
    | .node (.verb _ _ frame _) children => markedCount frame + roleChildren children
    | .node _ children => roleChildren children
    | .relativeForm _ a b _ front => roleCount a + roleCount b + roleChildren front
    | .modify a b | .sharedCoordination _ _ a b => roleCount a + roleCount b
    | .frameCoordination _ a b => roleChildren a + roleChildren b
    | .ellipsis _ _ => 0
    | _ => 0
  def roleChildren : List (Syntax L) → Nat
    | [] => 0
    | first :: rest => roleCount first + roleChildren rest
end

private theorem roleChildren_append (a b : List (Syntax L)) :
    roleChildren (a ++ b) = roleChildren a + roleChildren b := by
  induction a with
  | nil => simp [roleChildren]
  | cons first rest ih => simp [roleChildren, ih, Nat.add_assoc]

theorem role_step_increases {lexicon : Lexicon L} {better worse : Syntax L}
    (h : RoleStep lexicon better worse) : roleCount worse < roleCount better := by
  cases h
  simp [roleCount, roleChildren, selectedTree, postmodifierTree, object, markedCount]

/-- A declaration-backed local improvement in unchanged syntactic surroundings. -/
inductive Prefers (lexicon : Lexicon L) : Syntax L → Syntax L → Prop where
  | role {better worse} : RoleStep lexicon better worse → Prefers lexicon better worse
  | node {better worse} (construction : Construction L) (before after : List (Syntax L)) :
      Prefers lexicon better worse →
      Prefers lexicon (.node construction (before ++ [better] ++ after))
        (.node construction (before ++ [worse] ++ after))
  | modifierLeft {better worse} (head : Syntax L) : Prefers lexicon better worse →
      Prefers lexicon (.modify better head) (.modify worse head)
  | modifierRight {better worse} (modifier : Syntax L) : Prefers lexicon better worse →
      Prefers lexicon (.modify modifier better) (.modify modifier worse)

theorem preference_increases {lexicon : Lexicon L} {better worse : Syntax L}
    (h : Prefers lexicon better worse) : roleCount worse < roleCount better := by
  induction h with
  | role h => exact role_step_increases h
  | node construction before after _ ih =>
    cases construction <;> simp [roleCount, roleChildren_append, roleChildren] <;> omega
  | modifierLeft head _ ih => simp [roleCount]; omega
  | modifierRight modifier _ ih => simp [roleCount]; omega

theorem no_role_preference_cycle {lexicon : Lexicon L} {tree : Syntax L} :
    ¬ Relation.TransGen (Prefers lexicon) tree tree := by
  intro h
  have rises {a b : Syntax L} (chain : Relation.TransGen (Prefers lexicon) a b) :
      roleCount b < roleCount a := by
    induction chain with
    | single h => exact preference_increases h
    | tail _ h ih => have := preference_increases h; omega
  have := rises h
  omega

def Selected {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (candidates : List (GrammaticalScope.Reading lexicon category surface))
    (tree : GrammaticalScope.Reading lexicon category surface) : Prop :=
  tree ∈ candidates ∧ ∀ other ∈ candidates, ¬ Prefers lexicon other.val tree.val

theorem role_worse_excluded {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {candidates : List (GrammaticalScope.Reading lexicon category surface)}
    {better worse : GrammaticalScope.Reading lexicon category surface}
    (step : RoleStep lexicon better.val worse.val) (member : better ∈ candidates) :
    ¬ Selected candidates worse := by
  intro h
  exact h.2 better member (.role step)

theorem selected_exists {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (candidates : List (GrammaticalScope.Reading lexicon category surface))
    (inhabited : ∃ tree, tree ∈ candidates) : ∃ tree, Selected candidates tree := by
  obtain ⟨tree, member, _, maximal⟩ := Selection.admitted_maximum (fun _ ↦ True)
    (fun tree : GrammaticalScope.Reading lexicon category surface ↦ roleCount tree.val)
    candidates (by obtain ⟨t, h⟩ := inhabited; exact ⟨t, h, trivial⟩)
  refine ⟨tree, member, ?_⟩
  intro other mem preference
  have := maximal other mem trivial
  have := preference_increases preference
  omega

theorem selected_enumeration {lexicon : Lexicon L} {category : Category} {surface : Surface}
    {xs ys : List (GrammaticalScope.Reading lexicon category surface)}
    (same : ∀ tree, tree ∈ xs ↔ tree ∈ ys)
    (tree : GrammaticalScope.Reading lexicon category surface) :
    Selected xs tree ↔ Selected ys tree := by
  simp only [Selected, same]

theorem role_pair_selected {lexicon : Lexicon L} {category : Category} {surface : Surface}
    (better worse : GrammaticalScope.Reading lexicon category surface)
    (step : RoleStep lexicon better.val worse.val) :
    Selected [better, worse] better ∧ ¬ Selected [better, worse] worse := by
  refine ⟨⟨by simp, ?_⟩, role_worse_excluded step (by simp)⟩
  intro other member preference
  have improvement := role_step_increases step
  have competing := preference_increases preference
  simp only [List.mem_cons, List.not_mem_nil, or_false] at member
  rcases member with rfl | rfl <;> omega

theorem equal_role_counts_cannot_prefer {lexicon : Lexicon L} {a b : Syntax L}
    (same : roleCount a = roleCount b) : ¬ Prefers lexicon a b := by
  intro h
  have := preference_increases h
  omega

end English.RolePreference
