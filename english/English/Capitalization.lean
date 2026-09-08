import English.Lexical

namespace English.Casing

/-- Capabilities of a constituent in initial and ordinary interior positions. -/
structure Summary where
  empty : Bool := true
  initial : Bool := true
  interior : Bool := true
  deriving DecidableEq

@[simp] def append (a b : Summary) : Summary :=
  if a.empty then b else if b.empty then a else
    ⟨false, a.initial && b.interior, a.interior && b.interior⟩

@[simp] def boundary (a : Summary) : Summary :=
  ⟨a.empty, a.initial, a.initial⟩

@[simp] def word {L : Type} (w : WordForm L) : Summary :=
  ⟨false, w.capitalization == .initial || decide (w.spelling.capitalize = w.spelling),
    w.capitalization == .declared⟩

mutual
  /-- Composition follows declared surface order, without changing retained lexical values. -/
  @[simp] def summary {L : Type} : Syntax (WordForm L) → Summary
    | .noun w _ | .adjective w | .marker w | .word w _ | .identity w _ => word w
    | .modify a b | .sharedCoordination _ _ a b => append (summary a) (summary b)
    | .relativeForm _ head body _ front =>
        append (append (summary head) (children front)) (summary body)
    | .frameCoordination _ a b => append (children a) (children b)
    | .node (.keyword w _ .boundSuffix) xs => append (children xs) (word w)
    | .node (.keyword w _ .free) xs | .node (.attributive w _) xs
    | .node (.targeting w _) xs | .node (.compare w) xs | .node (.namePredicate w) xs
    | .node (.participialAttributive w _) xs | .node (.preposition w _) xs
    | .node (.verb w _ _ _) xs | .node (.auxiliary w _ _ _ _ _) xs
    | .node (.subordinate w _) xs => append (word w) (children xs)
    | .node (.nominalComplement w marker _) xs =>
        append (append (word w) (word marker)) (children xs)
    | .node (.measure marker) [a, b] => append (append (summary a) (word marker)) (summary b)
    | .node (.adjunct _ _ .before) [a, b] => append (summary b) (summary a)
    | .node (.document .sentence) xs | .node (.document .costAction) xs
    | .node (.document (.keywordLine _)) xs | .node (.document .modes) xs =>
        boundary (children xs)
    | .node (.document .quoteClause) xs | .node (.document (.quoteKeyword _)) xs =>
        ⟨(children xs).empty, (children xs).interior, (children xs).interior⟩
    | .node _ xs => children xs
    | .gap _ | .ellipsis _ _ => {}
  @[simp] def children {L : Type} : List (Syntax (WordForm L)) → Summary
    | [] => {}
    | x :: xs => append (summary x) (children xs)
end

@[simp] def valid {L : Type} (tree : Syntax (WordForm L)) : Prop :=
  (summary tree).initial = true ∨ (summary tree).interior = true

instance {L : Type} (tree : Syntax (WordForm L)) : Decidable (valid tree) :=
  inferInstanceAs (Decidable ((summary tree).initial = true ∨ (summary tree).interior = true))

theorem word_valid {L : Type} (w : WordForm L) :
    (word w).initial = true ∨ (word w).interior = true := by
  cases h : w.capitalization <;> simp_all

theorem append_valid (a b : Summary)
    (left : a.initial = true ∨ a.interior = true) (right : b.interior = true) :
    (append a b).initial = true ∨ (append a b).interior = true := by
  cases a with | mk ae ai ar =>
    cases b with | mk be bi br =>
      cases ae <;> cases be <;> simp_all

theorem joined_initial_requires_interior (a b : Summary)
    (left : a.empty = false) (right : b.empty = false)
    (initial : (append a b).initial = true) : b.interior = true := by
  simp [append, left, right] at initial
  exact initial.2

end English.Casing
