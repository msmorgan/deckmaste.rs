import English.FrameInteractions
import English.Dependencies

/-! Correlated alternatives that are not a Cartesian product.

One shared gap, two declared argument slots of the same Category. The category judgment compares
only the gap's Category, so it licenses all four combinations of "which slot the gap occupies" in
the two coordinands. The dependency judgment compares the recorded Relation, so exactly the two
correlated combinations survive. Nothing here reads a spelling: the discriminating datum is the
Relation declared by the verb's frame. -/
namespace English.JointInteractions

open FrameInteractions RolePreference Preference Scope

/-- Countability declarations for the `put` fixture; both nouns are count nouns. -/
def features : Features.Declarations Lexeme where
  nounUse _ use := use = .count
  determinerUse _ _ := False
  temporalNoun _ := False

/-- This fragment declares no relative vocabulary; the gap is licensed by right node raising. -/
def dependencies : Dependencies.Declarations Lexeme where
  relativePronoun _ := False
  relativeDeterminer _ := False

/-- The Relation each slot declares: the Object slot, or the marked declared-role slot. -/
def gapRelation : Bool → Relation
  | false => .object
  | true => .complement

/-- One `put NP on NP` verb phrase whose gap sits in the Object slot (`false`) or in the
declared-role slot (`true`). Both slots take `nominalCategory`, so the two differ only in the
Relation their frame item declares. -/
def conjunct : Bool → Syntax Lexeme
  | false =>
      .node (.verb .put .plain markedFrame) [.gap nominalCategory, .marker .on, artifacts.tree]
  | true =>
      .node (.verb .put .plain markedFrame) [creatures.tree, .marker .on, .gap nominalCategory]

/-- Two coordinands sharing one gap. The choices are independent here; the constraint is below. -/
def body (left right : Bool) : Syntax Lexeme :=
  .sharedCoordination .and_ (.verbPhrase .plain) (conjunct left) (conjunct right)

/-- The shared gap discharged by a raised right node, so every row is a closed derivation. -/
def row (left right : Bool) : Syntax Lexeme :=
  .node (.rightNodeRaising (.verbPhrase .plain) nominalCategory) [body left right, creatures.tree]

def conjunctSurface : Bool → Surface
  | false => ["put", "on", "artifacts"]
  | true => ["put", "creatures", "on"]

def bodySurface (left right : Bool) : Surface :=
  conjunctSurface left ++ (["and"] : Surface) ++ conjunctSurface right

def rowSurface (left right : Bool) : Surface :=
  bodySurface left right ++ (["creatures"] : Surface)

/-! ### The category layer sees a full product -/

theorem conjunct_judges (marked : Bool) :
    Judges lexicon (conjunct marked) (.verbPhrase .plain) [nominalCategory] := by
  cases marked
  · exact .verb ⟨rfl, rfl, rfl, Or.inr rfl⟩
      (.argument (slot := ⟨.object, nominalCategory⟩) .gap
        (.marked (marker := Lexeme.on) (slot := role) artifacts.derives .nil))
  · exact .verb ⟨rfl, rfl, rfl, Or.inr rfl⟩
      (.argument (slot := ⟨.object, nominalCategory⟩) creatures.derives
        (.marked (marker := Lexeme.on) (slot := role) .gap .nil))

theorem body_judges (left right : Bool) :
    Judges lexicon (body left right) (.verbPhrase .plain) [nominalCategory] :=
  .sharedCoordination rfl (conjunct_judges left) (conjunct_judges right)

/-- All four combinations are closed derivations at one Category. -/
theorem row_derives (left right : Bool) : Derives lexicon (row left right) (.verbPhrase .plain) :=
  .rightNodeRaising (body_judges left right) creatures.derives

theorem conjunct_realizes (marked : Bool) :
    Realizes lexicon (conjunct marked) (conjunctSurface marked) := by
  cases marked
  · simpa [conjunct, conjunctSurface, artifacts] using Realizes.node (lexicon := lexicon)
      (.cons .gap (.cons (.marker (Or.inl ⟨rfl, rfl⟩)) (.cons artifacts.realizes .nil)))
      (Linearizes.verb (head := Lexeme.put) (form := .plain) (frame := markedFrame)
        (voice := .active) ⟨rfl, rfl, rfl⟩)
  · simpa [conjunct, conjunctSurface, creatures] using Realizes.node (lexicon := lexicon)
      (.cons creatures.realizes (.cons (.marker (Or.inl ⟨rfl, rfl⟩)) (.cons .gap .nil)))
      (Linearizes.verb (head := Lexeme.put) (form := .plain) (frame := markedFrame)
        (voice := .active) ⟨rfl, rfl, rfl⟩)

theorem body_realizes (left right : Bool) :
    Realizes lexicon (body left right) (bodySurface left right) :=
  .sharedCoordination (conjunct_realizes left) (conjunct_realizes right)

theorem row_realizes (left right : Bool) :
    Realizes lexicon (row left right) (rowSurface left right) :=
  .node (.cons (body_realizes left right) (.cons creatures.realizes .nil)) .rightNodeRaising

theorem row_admissible (left right : Bool) :
    Admissible lexicon (row left right) (.verbPhrase .plain) (rowSurface left right) :=
  ⟨row_derives left right, row_realizes left right⟩

/-! ### The feature layer sees the same full product -/

/-- A bare plural of a declared count noun conforms; the head owns the countability. -/
theorem bare_conforms (head : Lexeme) (number : Number) :
    Features.Conforms features (.node .barePlural [.noun head number]) :=
  ⟨.noun rfl, ⟨trivial, trivial⟩, trivial⟩

theorem conjunct_conforms (marked : Bool) : Features.Conforms features (conjunct marked) := by
  cases marked
  · refine ⟨?_, ⟨trivial, trivial⟩, ⟨trivial, trivial⟩, bare_conforms .artifact .plural, trivial⟩
    simp [conjunct, Features.Local, FrameCases, markedFrame, object, role, CaseAt,
      Relation.casePosition, Syntax.nominalCase, Case.Allows, artifacts, nominalCategory]
  · refine ⟨?_, bare_conforms .creature .plural, ⟨trivial, trivial⟩, ⟨trivial, trivial⟩, trivial⟩
    simp [conjunct, Features.Local, FrameCases, markedFrame, object, role, CaseAt,
      Relation.casePosition, Syntax.nominalCase, Case.Allows, creatures, nominalCategory]

theorem row_conforms (left right : Bool) : Features.Conforms features (row left right) :=
  ⟨trivial, ⟨trivial, conjunct_conforms left, conjunct_conforms right⟩,
    bare_conforms .creature .plural⟩

/-- Weakened premise: with the dependency check dropped, every one of the four combinations is
admitted. This is the counterexample the correlation law needs — the product is really there
until `Dependencies.Safe` removes half of it. -/
theorem all_four_admitted_without_dependencies (left right : Bool) :
    Features.Admitted lexicon features (row left right) (.verbPhrase .plain)
      (rowSurface left right) :=
  ⟨row_admissible left right, row_conforms left right⟩

/-! ### The dependency layer records the Relation, and only the correlated rows survive -/

theorem conjunct_exposed (marked : Bool) (relation : Relation) :
    Dependencies.exposed (conjunct marked) relation = [⟨nominalCategory, gapRelation marked⟩] := by
  cases marked <;> rfl

theorem conjunct_exposed_determines {left right : Bool}
    (same : Dependencies.exposed (conjunct left) = Dependencies.exposed (conjunct right)) :
    left = right := by
  cases left <;> cases right
  · rfl
  · exact absurd (((conjunct_exposed false .complement).symm.trans same).trans
      (conjunct_exposed true .complement)) (by decide)
  · exact absurd (((conjunct_exposed true .complement).symm.trans same).trans
      (conjunct_exposed false .complement)) (by decide)
  · rfl

/-- A bare plural exposes no gap, so it is dependency-safe wherever it stands. -/
theorem bare_safe (head : Lexeme) (number : Number) :
    Dependencies.Safe dependencies (.node .barePlural [.noun head number]) :=
  ⟨trivial, ⟨trivial, trivial⟩, trivial⟩

theorem conjunct_safe (marked : Bool) : Dependencies.Safe dependencies (conjunct marked) := by
  cases marked
  · exact ⟨trivial, ⟨trivial, trivial⟩, ⟨trivial, trivial⟩, bare_safe .artifact .plural, trivial⟩
  · exact ⟨trivial, bare_safe .creature .plural, ⟨trivial, trivial⟩, ⟨trivial, trivial⟩, trivial⟩

theorem body_exposed (left right : Bool) (relation : Relation) :
    Dependencies.exposed (body left right) relation =
      [⟨nominalCategory, gapRelation left⟩] :=
  conjunct_exposed left relation

theorem body_safe_iff (left right : Bool) :
    Dependencies.Safe dependencies (body left right) ↔ left = right := by
  constructor
  · intro h
    exact conjunct_exposed_determines h.1
  · rintro rfl
    exact ⟨rfl, conjunct_safe left, conjunct_safe left⟩

theorem row_exposed (left right : Bool) (relation : Relation) :
    Dependencies.exposed (row left right) relation = [] := rfl

/-- Exactly two of the four combinations are dependency-safe: the gap's Relation must agree
across the coordinands, which is a correlation and not a per-slot restriction. -/
theorem row_safe_iff (left right : Bool) :
    Dependencies.Safe dependencies (row left right) ↔ left = right := by
  constructor
  · intro h
    exact (body_safe_iff left right).mp h.2.1
  · rintro rfl
    exact ⟨⟨⟨gapRelation left, body_exposed left left .complement⟩, rfl⟩,
      (body_safe_iff left left).mpr rfl, bare_safe .creature .plural, trivial⟩

theorem row_admitted_iff (left right : Bool) :
    Dependencies.Admitted lexicon features dependencies (row left right) (.verbPhrase .plain)
      (rowSurface left right) ↔ left = right :=
  ⟨fun h ↦ (row_safe_iff left right).mp h.2,
    fun h ↦ ⟨all_four_admitted_without_dependencies left right, (row_safe_iff left right).mpr h⟩⟩

/-! ### The packing keyed on the discriminating field -/

/-- The candidate analyses: the four closed rows and the two shared-gap bodies whose coordinands
already agree. The bodies are dependency-safe, so keeping them out of a closed row's package is
work only the key can do. -/
def candidate (tree : Syntax Lexeme) : Prop :=
  (∃ left right, tree = row left right) ∨ (∃ marked, tree = body marked marked)

def surviving (tree : Syntax Lexeme) : Prop :=
  candidate tree ∧ Dependencies.Safe dependencies tree

theorem row_surviving_iff (left right : Bool) : surviving (row left right) ↔ left = right :=
  ⟨fun h ↦ (row_safe_iff left right).mp h.2,
    fun h ↦ ⟨Or.inl ⟨left, right, rfl⟩, (row_safe_iff left right).mpr h⟩⟩

theorem body_surviving (marked : Bool) : surviving (body marked marked) :=
  ⟨Or.inr ⟨marked, rfl⟩, (body_safe_iff marked marked).mpr rfl⟩

/-- The grammatical instance of `Scope.joint_alternatives_exact`. Two derivable alternatives are
retained under one `Dependencies.exposed` scope; the two uncorrelated combinations are excluded by
the dependency correlation, and the two open shared-gap bodies — which do survive — are excluded by
the key. Neither slot choice is restricted on its own: the last two conjuncts give a surviving row
for each coordinate value, so the retained set is not the product of its projections. -/
theorem joint_alternatives_grammatical :
    ∃ p : Package (Syntax Lexeme) (List Dependencies.GapUse),
      Packs surviving (fun tree ↦ Dependencies.exposed tree) p ∧
      p.readings (row false false) ∧ p.readings (row true true) ∧
      ¬ p.readings (row false true) ∧ ¬ p.readings (row true false) ∧
      ¬ p.readings (body false false) ∧ ¬ p.readings (body true true) ∧
      (∃ b, surviving (row false b)) ∧ (∃ a, surviving (row a true)) := by
  obtain ⟨p, packed, scope, left, right, outLeft, outRight⟩ :=
    joint_correlated_pack surviving (fun tree ↦ Dependencies.exposed tree) []
      (retainedLeft := row false false) (retainedRight := row true true)
      (excludedLeft := row false true) (excludedRight := row true false)
      ((row_surviving_iff false false).mpr rfl) ((row_surviving_iff true true).mpr rfl)
      (row_exposed false false .complement) (row_exposed true true .complement)
      (fun h ↦ by cases (row_surviving_iff false true).mp h.1)
      (fun h ↦ by cases (row_surviving_iff true false).mp h.1)
  have keyed (marked : Bool) : ¬ p.readings (body marked marked) := by
    intro member
    have key := ((packing_exact packed _).mp member).2
    rw [scope] at key
    exact absurd ((body_exposed marked marked .complement).symm.trans key) (by simp)
  exact ⟨p, packed, left, right, outLeft, outRight, keyed false, keyed true,
    ⟨false, (row_surviving_iff false false).mpr rfl⟩,
    ⟨true, (row_surviving_iff true true).mpr rfl⟩⟩

end English.JointInteractions
