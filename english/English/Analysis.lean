import English.Dependencies
import English.FrameScope

/-! Reviewed candidates, declaration-backed preference and retained grammatical alternatives. -/
namespace English.Analysis
variable {L : Type}

abbrev Reading (lexicon : Lexicon L) (features : Features.Declarations L)
    (dependencies : Dependencies.Declarations L) (category : Category) (surface : Surface) :=
  {tree : Syntax L // Dependencies.Admitted lexicon features dependencies tree category surface}

/-- Identity provenance is lexical evidence; it conveys no game-reference resolution. -/
inductive IdentityStep (lexicon : Lexicon L) : Syntax L → Syntax L → Prop where
  | word {identity word : L} {category : Category} {surface : Surface} :
      lexicon.identity identity category → lexicon.identityForm identity category surface →
      lexicon.word word category → lexicon.wordForm word category surface →
      IdentityStep lexicon (.identity identity category) (.word word category)

mutual
  def identityCount : Syntax L → Nat
    | .identity _ _ => 1
    | .node _ children => childIdentities children
    | .modify a b | .sharedCoordination _ _ a b => identityCount a + identityCount b
    | .relativeForm _ a b _ front => identityCount a + identityCount b + childIdentities front
    | .frameCoordination _ a b => childIdentities a + childIdentities b
    | _ => 0
  def childIdentities : List (Syntax L) → Nat
    | [] => 0
    | a :: rest => identityCount a + childIdentities rest
end

private theorem identities_append (a b : List (Syntax L)) :
    childIdentities (a ++ b) = childIdentities a + childIdentities b := by
  induction a with
  | nil => simp [childIdentities]
  | cons a as ih => simp [childIdentities,ih,Nat.add_assoc]

private theorem roles_append (a b : List (Syntax L)) :
    RolePreference.roleChildren (a ++ b) =
      RolePreference.roleChildren a + RolePreference.roleChildren b := by
  induction a with
  | nil => simp [RolePreference.roleChildren]
  | cons a as ih => simp [RolePreference.roleChildren,ih,Nat.add_assoc]

def measure (tree : Syntax L) := RolePreference.roleCount tree + identityCount tree

/-- Measures prove termination; only these evidenced local comparisons create preference edges. -/
inductive Prefers (lexicon : Lexicon L) : Syntax L → Syntax L → Prop where
  | role {a b} : RolePreference.RoleStep lexicon a b → Prefers lexicon a b
  | identity {a b} : IdentityStep lexicon a b → Prefers lexicon a b
  | node {a b} (construction : Construction L) (before after : List (Syntax L)) :
      Prefers lexicon a b → Prefers lexicon (.node construction (before ++ [a] ++ after))
        (.node construction (before ++ [b] ++ after))
  | modifyLeft {a b} (head : Syntax L) : Prefers lexicon a b →
      Prefers lexicon (.modify a head) (.modify b head)
  | modifyRight {a b} (modifier : Syntax L) : Prefers lexicon a b →
      Prefers lexicon (.modify modifier a) (.modify modifier b)

theorem preference_increases {lexicon : Lexicon L} {a b : Syntax L} (h : Prefers lexicon a b) :
    measure b < measure a := by
  induction h with
  | role h =>
    cases h
    simp [measure,RolePreference.roleCount,RolePreference.roleChildren,RolePreference.markedCount,
      RolePreference.selectedTree,RolePreference.postmodifierTree,RolePreference.object,
      identityCount,childIdentities]
  | identity h => cases h; simp [measure,RolePreference.roleCount,identityCount]
  | node c before after _ ih =>
    cases c <;> simp [measure,RolePreference.roleCount,roles_append,RolePreference.roleChildren,
      identityCount,identities_append,childIdentities] at * <;> omega
  | modifyLeft head _ ih =>
    simp [measure,RolePreference.roleCount,identityCount] at *; omega
  | modifyRight modifier _ ih =>
    simp [measure,RolePreference.roleCount,identityCount] at *; omega

theorem no_cycle {lexicon : Lexicon L} {tree : Syntax L} :
    ¬ Relation.TransGen (Prefers lexicon) tree tree := by
  have increases {a b} (chain : Relation.TransGen (Prefers lexicon) a b) : measure b < measure a
    := by
    induction chain with
    | single h => exact preference_increases h
    | tail _ h ih => have := preference_increases h; omega
  intro h
  have := increases h
  omega

variable {lexicon : Lexicon L} {features : Features.Declarations L}
  {dependencies : Dependencies.Declarations L} {category : Category} {surface : Surface}
local notation "Candidate" => Reading lexicon features dependencies category surface

def Selected (candidates : List (Candidate)) (tree : Candidate) : Prop :=
  tree ∈ candidates ∧ ∀ other ∈ candidates, ¬ Prefers lexicon other.val tree.val

theorem selected_exists (candidates : List (Candidate))
    (inhabited : ∃ a, a ∈ candidates) : ∃ a, Selected candidates a := by
  obtain ⟨a,mem,_,maximum⟩ := Selection.admitted_maximum (fun _ ↦ True)
    (fun a : Candidate ↦ measure a.val) candidates
    (by obtain ⟨a,h⟩ := inhabited; exact ⟨a,h,trivial⟩)
  refine ⟨a,mem,?_⟩
  intro b member preference
  have := maximum b member trivial
  have := preference_increases preference
  omega

theorem selected_enumeration (xs ys : List (Candidate))
    (same : ∀ a, a ∈ xs ↔ a ∈ ys) (a : Candidate) :
    Selected xs a ↔ Selected ys a := by simp only [Selected,same]

/-- Every intermediate also passes feature and dependency constraints. -/
inductive Related : Candidate →
    Candidate → Prop where
  | refl (a) : Related a a
  | step {a b} : FrameScope.Step a.val b.val → Related a b
  | symm {a b} : Related a b → Related b a
  | trans {a b c} : Related a b → Related b c → Related a c

def setoid : Setoid (Candidate) where
  r := Related
  iseqv := ⟨Related.refl,Related.symm,Related.trans⟩
def key (a : Candidate) := Quotient.mk setoid a

theorem key_exact (a b : Candidate) : key a = key b ↔ Related a b :=
  ⟨Quotient.exact,fun h ↦ Quotient.sound (s := setoid) h⟩

def package (candidates : List (Candidate)) (representative : Candidate) :=
  Selection.pack (Selected candidates) key (key representative)

theorem package_exact (candidates : List (Candidate)) (a b : Candidate) :
    (package candidates a).readings b ↔ Selected candidates b ∧ Related b a :=
  and_congr_right (fun _ ↦ key_exact b a)

theorem no_feature_bypass (candidates : List (Candidate)) (a b : Candidate)
    (_ : (package candidates a).readings b) :
    Features.Conforms features b.val ∧ Dependencies.Safe dependencies b.val :=
      ⟨b.property.1.2,b.property.2⟩

end English.Analysis
