import English.Selection

/-!
Scope-boundary abstraction. A production adapter must preserve ordered anchors and distinguish
transparent hosts from opaque declared-role edges. This is not a second grammatical derivation.
-/

namespace English.Scope

open Selection

/-- Only the right-periphery distinctions needed for these boundary laws. -/
inductive Host where
  | leaf (identity : Nat)
  | coordinate (coordinator : Coordinator) (left right : Host)
  | frame (child : Host)
  | auxiliary (child : Host)
  | role (child : Host)

/-- Paths retain ordered child indices; role edges, not whole frame hosts, are opaque. -/
def Host.sites : Host → List (List Nat)
  | .leaf _ | .role _ => [[]]
  | .coordinate _ _ right => [] :: right.sites.map (1 :: ·)
  | .frame child | .auxiliary child => [] :: child.sites.map (0 :: ·)

/-- Eligibility is declared separately; a deeper eligible site cannot skip an earlier one. -/
inductive FirstEligible {A : Type} (eligible : A → Prop) : List A → A → Prop where
  | here {a rest} : eligible a → FirstEligible eligible (a :: rest) a
  | later {a rest b} : ¬ eligible a → FirstEligible eligible rest b →
      FirstEligible eligible (a :: rest) b

theorem first_eligible_unique {A : Type} {eligible : A → Prop} {sites : List A} {a b : A}
    (ha : FirstEligible eligible sites a) (hb : FirstEligible eligible sites b) : a = b := by
  induction ha with
  | here eligible =>
    cases hb with
    | here _ => rfl
    | later ineligible _ => exact False.elim (ineligible eligible)
  | later ineligible _ ih =>
    cases hb with
    | here eligible => exact False.elim (ineligible eligible)
    | later _ rest => exact ih rest

theorem first_eligible_member {A : Type} {eligible : A → Prop} {sites : List A} {a : A}
    (h : FirstEligible eligible sites a) : a ∈ sites ∧ eligible a := by
  induction h with
  | here h => exact ⟨by simp, h⟩
  | later _ _ ih => exact ⟨by simp [ih.1], ih.2⟩

theorem first_eligible_exists {A : Type} (eligible : A → Prop) (sites : List A)
    (occupied : ∃ a, a ∈ sites ∧ eligible a) : ∃ a, FirstEligible eligible sites a := by
  classical
  induction sites with
  | nil => simp at occupied
  | cons head tail ih =>
    by_cases he : eligible head
    · exact ⟨head, .here he⟩
    · have rest : ∃ a, a ∈ tail ∧ eligible a := by
        obtain ⟨a, mem, valid⟩ := occupied
        simp only [List.mem_cons] at mem
        rcases mem with rfl | mem
        · exact False.elim (he valid)
        · exact ⟨a, mem, valid⟩
      obtain ⟨a, first⟩ := ih rest
      exact ⟨a, .later he first⟩

theorem nonfinal_boundary (coordinator : Coordinator) (left right : Host) (rest : List Nat) :
    (0 :: rest) ∉ (Host.coordinate coordinator left right).sites := by
  simp [Host.sites]

theorem frame_transparent (coordinator : Coordinator) (left : Host) (identity : Nat) :
    [0, 1] ∈ (Host.frame (.coordinate coordinator left (.leaf identity))).sites := by
  simp [Host.sites]

theorem role_edge_opaque (child : Host) (index : Nat) (rest : List Nat) :
    (index :: rest) ∉ (Host.role child).sites := by
  simp [Host.sites]

theorem first_eligible_skips (coordinator : Coordinator) :
    FirstEligible (fun path : List Nat ↦ path ≠ [])
      (Host.coordinate coordinator (.leaf 0) (.frame (.leaf 1))).sites [1] :=
  .later (fun h ↦ h rfl) (.here (by decide))

/-- Ordered anchor topology, including group arity, is preserved by a scope key. -/
inductive Anchors where
  | leaf (identity : Nat)
  | group (children : List Anchors)

def flat : Anchors := .group [.leaf 0, .leaf 1, .leaf 2]
def nested : Anchors := .group [.leaf 0, .group [.leaf 1, .leaf 2]]

/-- Site assignments may vary while every ordered anchor remains part of the class identity. -/
structure AnchorPattern where
  anchors : Anchors
  sites : List (List Nat)

theorem anchor_shape_separate (survivors : AnchorPattern → Prop) (p : Package AnchorPattern Anchors)
    (packed : Packs survivors AnchorPattern.anchors p) (leftSites rightSites : List (List Nat)) :
    ¬ (p.readings ⟨flat, leftSites⟩ ∧ p.readings ⟨nested, rightSites⟩) := by
  apply different_classes_separate packed
  intro h
  cases h

/-- Two mobile choices can be correlated; projection to separate domains loses that relation. -/
def joint (choices : Bool × Bool) : Prop := choices = (false, false) ∨ choices = (true, true)

theorem joint_alternatives_exact :
    ∃ p : Package (Bool × Bool) Unit, Packs joint (fun _ ↦ ()) p ∧
      p.readings (false, false) ∧ p.readings (true, true) ∧
      ¬ p.readings (false, true) ∧ ¬ p.readings (true, false) ∧
      (∃ b, joint (false, b)) ∧ (∃ a, joint (a, true)) := by
  refine ⟨pack joint (fun _ ↦ ()) (), packing_exists _ _ _ ⟨(false, false), Or.inl rfl, rfl⟩,
    ⟨Or.inl rfl, rfl⟩, ⟨Or.inr rfl, rfl⟩, ?_, ?_, ⟨false, Or.inl rfl⟩, ⟨true, Or.inr rfl⟩⟩
  · simp [pack, joint]
  · simp [pack, joint]

end English.Scope
