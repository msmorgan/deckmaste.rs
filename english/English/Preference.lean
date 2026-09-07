import English.Grammar

/-! Preference over independent admissibility, followed by exact scope-class packing. -/

namespace English.Preference

variable {A K : Type}

/-- Claims read from declared grammatical features in one comparison region. -/
structure Claims where
  frameRole : Bool
  identity : Bool

/-- The four values encode the ordered principles, not weights learned from a corpus. -/
def Claims.rank (claims : Claims) : Nat :=
  (if claims.frameRole then 2 else 0) + (if claims.identity then 1 else 0)

/-- Supplying coherent features and comparison regions is a separate adapter obligation. -/
structure Policy (A K : Type) where
  region : A → K
  claims : A → Claims

def Prefers (policy : Policy A K) (better worse : A) : Prop :=
  policy.region better = policy.region worse ∧
    (policy.claims worse).rank < (policy.claims better).rank

def Selected (admitted : A → Prop) (policy : Policy A K) (candidates : List A)
    (analysis : A) : Prop :=
  analysis ∈ candidates ∧ admitted analysis ∧
    ∀ other, other ∈ candidates → admitted other → ¬ Prefers policy other analysis

theorem selected_sound {admitted : A → Prop} {policy : Policy A K} {candidates : List A}
    {analysis : A} (selected : Selected admitted policy candidates analysis) : admitted analysis :=
  selected.2.1

theorem selected_enumeration (admitted : A → Prop) (policy : Policy A K)
    {xs ys : List A} (same : ∀ a, a ∈ xs ↔ a ∈ ys) (a : A) :
    Selected admitted policy xs a ↔ Selected admitted policy ys a := by
  simp only [Selected, same]

theorem selected_permutation (admitted : A → Prop) (policy : Policy A K)
    {xs ys : List A} (perm : xs.Perm ys) (a : A) :
    Selected admitted policy xs a ↔ Selected admitted policy ys a :=
  selected_enumeration admitted policy (fun _ ↦ perm.mem_iff) a

theorem frame_role_preempts (policy : Policy A K) (better worse : A)
    (region : policy.region better = policy.region worse)
    (betterRole : (policy.claims better).frameRole = true)
    (worseRole : (policy.claims worse).frameRole = false) : Prefers policy better worse := by
  refine ⟨region, ?_⟩
  simp only [Claims.rank, betterRole, worseRole, Bool.false_eq_true, ↓reduceIte]
  split <;> split <;> omega

theorem identity_preempts (policy : Policy A K) (better worse : A)
    (region : policy.region better = policy.region worse)
    (sameRole : (policy.claims better).frameRole = (policy.claims worse).frameRole)
    (betterIdentity : (policy.claims better).identity = true)
    (worseIdentity : (policy.claims worse).identity = false) : Prefers policy better worse := by
  refine ⟨region, ?_⟩
  simp only [Claims.rank, sameRole, betterIdentity, worseIdentity, Bool.false_eq_true, ↓reduceIte]
  omega

theorem preference_decreases {policy : Policy A K} {a b : A}
    (h : Prefers policy a b) : (policy.claims b).rank < (policy.claims a).rank := h.2

theorem preference_asymmetric {policy : Policy A K} {a b : A}
    (h : Prefers policy a b) : ¬ Prefers policy b a := by
  intro reverse
  have := h.2
  have := reverse.2
  omega

theorem preference_transitive {policy : Policy A K} {a b c : A}
    (ab : Prefers policy a b) (bc : Prefers policy b c) : Prefers policy a c := by
  refine ⟨ab.1.trans bc.1, ?_⟩
  have := ab.2
  have := bc.2
  omega

/-- Nonempty paths; an empty path is not a preference cycle. -/
inductive PreferenceChain (policy : Policy A K) : A → A → Prop where
  | step {a b} : Prefers policy a b → PreferenceChain policy a b
  | cons {a b c} : Prefers policy a b → PreferenceChain policy b c → PreferenceChain policy a c

theorem no_preference_cycle (policy : Policy A K) (a : A) : ¬ PreferenceChain policy a a := by
  have chain : ∀ {x y}, PreferenceChain policy x y → Prefers policy x y := by
    intro x y h
    induction h with
    | step h => exact h
    | cons h _ ih => exact preference_transitive h ih
  intro h
  exact preference_asymmetric (chain h) (chain h)

theorem admitted_maximum (admitted : A → Prop) (rank : A → Nat) (xs : List A)
    (inhabited : ∃ a, a ∈ xs ∧ admitted a) :
    ∃ a, a ∈ xs ∧ admitted a ∧ ∀ b, b ∈ xs → admitted b → rank b ≤ rank a := by
  classical
  induction xs with
  | nil => simp at inhabited
  | cons head tail ih =>
    by_cases tailInhabited : ∃ a, a ∈ tail ∧ admitted a
    · obtain ⟨best, member, valid, maximal⟩ := ih tailInhabited
      by_cases headBetter : admitted head ∧ rank best < rank head
      · refine ⟨head, by simp, headBetter.1, ?_⟩
        intro b mem adm
        simp only [List.mem_cons] at mem
        rcases mem with rfl | mem
        · omega
        · have := maximal b mem adm
          have := headBetter.2
          omega
      · refine ⟨best, by simp [member], valid, ?_⟩
        intro b mem adm
        simp only [List.mem_cons] at mem
        rcases mem with rfl | mem
        · have : ¬ rank best < rank b := fun h ↦ headBetter ⟨adm, h⟩
          omega
        · exact maximal b mem adm
    · obtain ⟨a, mem, valid⟩ := inhabited
      have aHead : a = head := by
        simp only [List.mem_cons] at mem
        rcases mem with eq | mem
        · exact eq
        · exact False.elim (tailInhabited ⟨a, mem, valid⟩)
      subst a
      refine ⟨head, by simp, valid, ?_⟩
      intro b mem adm
      simp only [List.mem_cons] at mem
      rcases mem with rfl | mem
      · omega
      · exact False.elim (tailInhabited ⟨b, mem, adm⟩)

theorem selected_exists (admitted : A → Prop) (policy : Policy A K) (xs : List A)
    (inhabited : ∃ a, a ∈ xs ∧ admitted a) : ∃ a, Selected admitted policy xs a := by
  obtain ⟨a, member, valid, maximal⟩ :=
    admitted_maximum admitted (fun a ↦ (policy.claims a).rank) xs inhabited
  refine ⟨a, member, valid, ?_⟩
  intro b mem adm preferred
  have := maximal b mem adm
  have := preferred.2
  omega

/-- Readings are complete analyses; no independent site-choice product is taken. -/
structure Package (A K : Type) where
  scope : K
  readings : A → Prop

/-- Exactness and nonemptiness are obligations on a proposed package. -/
def Packs (survivors : A → Prop) (key : A → K) (package : Package A K) : Prop :=
  (∃ a, survivors a ∧ key a = package.scope) ∧
    ∀ a, package.readings a ↔ survivors a ∧ key a = package.scope

def pack (survivors : A → Prop) (key : A → K) (scope : K) : Package A K :=
  ⟨scope, fun a ↦ survivors a ∧ key a = scope⟩

theorem packing_exists (survivors : A → Prop) (key : A → K) (scope : K)
    (occupied : ∃ a, survivors a ∧ key a = scope) :
    Packs survivors key (pack survivors key scope) :=
  ⟨occupied, fun _ ↦ Iff.rfl⟩

theorem packing_exact {survivors : A → Prop} {key : A → K} {p : Package A K}
    (h : Packs survivors key p) (a : A) : p.readings a ↔ survivors a ∧ key a = p.scope := h.2 a

theorem packing_unique {survivors : A → Prop} {key : A → K} {p q : Package A K}
    (hp : Packs survivors key p) (hq : Packs survivors key q)
    (same : p.scope = q.scope) : p = q := by
  cases p with
  | mk pk pr =>
    cases q with
    | mk qk qr =>
      simp only at same
      subst qk
      have readings : pr = qr := by
        funext a
        exact propext ((hp.2 a).trans (hq.2 a).symm)
      cases readings
      rfl

theorem packing_enumeration (admitted : A → Prop) (policy : Policy A K) (key : A → K)
    {xs ys : List A} (same : ∀ a, a ∈ xs ↔ a ∈ ys) (p : Package A K) :
    Packs (Selected admitted policy xs) key p ↔ Packs (Selected admitted policy ys) key p := by
  have selected := selected_enumeration admitted policy same
  simp only [Packs, selected]

theorem different_classes_separate {survivors : A → Prop} {key : A → K} {p : Package A K}
    (h : Packs survivors key p) {a b : A} (different : key a ≠ key b) :
    ¬ (p.readings a ∧ p.readings b) := by
  intro both
  exact different (((h.2 a).mp both.1).2.trans ((h.2 b).mp both.2).2.symm)

/-- A normal form cannot acquire an analysis that was absent from all input survivors. -/
theorem unpack_exact (survivors : A → Prop) (key : A → K) (a : A) :
    (∃ p, Packs survivors key p ∧ p.readings a) ↔ survivors a := by
  constructor
  · rintro ⟨p, hp, member⟩
    exact ((hp.2 a).mp member).1
  · intro member
    exact ⟨pack survivors key (key a), packing_exists _ _ _ ⟨a, member, rfl⟩, member, rfl⟩

/-- A package of selected grammatical analyses retains grammatical admissibility. -/
theorem packed_admissible {L : Type} {lexicon : Lexicon L} {category : Category}
    {surface : Surface} {policy : Policy (Syntax L) K} {xs : List (Syntax L)}
    {key : Syntax L → K} {p : Package (Syntax L) K} {tree : Syntax L}
    (packed : Packs (Selected (fun t ↦ Admissible lexicon t category surface) policy xs) key p)
    (member : p.readings tree) : Admissible lexicon tree category surface :=
  selected_sound (admitted := fun t ↦ Admissible lexicon t category surface)
    ((packed.2 tree).mp member).1

end English.Preference
