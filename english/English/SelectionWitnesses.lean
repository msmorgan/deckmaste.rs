import English.Selection
import English.Witnesses
import English.Composition

/-! Inhabited grammar fragments and counterexamples delimiting the selection laws. -/

namespace English.SelectionWitnesses

open Selection

variable {L : Type}

/-- No preference is asserted between legitimate scope alternatives. -/
def neutral (A : Type) : Policy A Unit := ⟨fun _ ↦ (), fun _ ↦ ⟨false, false⟩⟩

theorem neutral_selected (admitted : L → Prop) (xs : List L) (a : L) :
    Selected admitted (neutral L) xs a ↔ a ∈ xs ∧ admitted a := by
  simp [Selected, Prefers, neutral]

/-- This class is one modifier over two ordered plural nominal conjuncts. -/
def ModifierScope (modifier left right tree : Syntax L) : Prop :=
  tree = .modify modifier (.coordinate left right) ∨
    tree = .coordinate (.modify modifier left) right

theorem modifier_scope_preserves {lexicon : Lexicon L} {m l r tree : Syntax L}
    {ms ls rs : Surface} (modifier : Admissible lexicon m .adjectivePhrase ms)
    (left : Admissible lexicon l (.nominal .plural) ls)
    (right : Admissible lexicon r (.nominal .plural) rs)
    (scope : ModifierScope m l r tree) :
    Admissible lexicon tree (.nominal .plural) (ms ++ ls ++ (["and"] : Surface) ++ rs) := by
  rcases scope with rfl | rfl
  · refine ⟨.modify modifier.1 (.node (.coordinate rfl) (.cons left.1 (.cons right.1 .nil))), ?_⟩
    have realized := Realizes.modify modifier.2
      (Realizes.node (.cons left.2 (.cons right.2 .nil))
        (Linearizes.coordinate (coordinator := .and_) (category := .nominal .plural)))
    simpa [Syntax.coordinate, Coordinator.surface, List.append_assoc] using realized
  · refine ⟨.node (.coordinate rfl) (.cons (.modify modifier.1 left.1) (.cons right.1 .nil)), ?_⟩
    exact .node (.cons (.modify modifier.2 left.2) (.cons right.2 .nil)) .coordinate

/-- Exact packaging is unique for this class, without choosing a reading within it. -/
theorem modifier_scope_unique (m l r : Syntax L) :
    ∃ p : Package (Syntax L) Unit, Packs (ModifierScope m l r) (fun _ ↦ ()) p ∧
      ∀ q, Packs (ModifierScope m l r) (fun _ ↦ ()) q → q = p := by
  let p := pack (ModifierScope m l r) (fun _ ↦ ()) ()
  have hp : Packs (ModifierScope m l r) (fun _ ↦ ()) p :=
    packing_exists _ _ _ ⟨.modify m (.coordinate l r), Or.inl rfl, rfl⟩
  refine ⟨p, hp, ?_⟩
  intro q hq
  exact packing_unique hq hp (by cases q.scope; rfl)

open Witnesses in
/-- Acyclicity leaves two unequal, independently admitted trees on the very same surface. -/
theorem acyclic_tie :
    (∀ t, ¬ PreferenceChain (neutral (Syntax Lexeme)) t t) ∧ narrow ≠ shared ∧
    Selected (fun t ↦ Admissible lexicon t (.nominal .plural) surface)
      (neutral _) [narrow, shared] narrow ∧
    Selected (fun t ↦ Admissible lexicon t (.nominal .plural) surface)
      (neutral _) [narrow, shared] shared := by
  refine ⟨no_preference_cycle _, distinct_analyses.1, ?_, ?_⟩
  · exact (neutral_selected _ _ _).mpr ⟨by simp, distinct_analyses.2.1⟩
  · exact (neutral_selected _ _ _).mpr ⟨by simp, distinct_analyses.2.2⟩

open Witnesses in
/-- One representative retains both readings; it is not evidence for unique interpretation. -/
theorem modifier_package_retains_both :
    ∃ p : Package (Syntax Lexeme) Unit,
      Packs (Selected (fun t ↦ Admissible lexicon t (.nominal .plural) surface)
        (neutral _) [narrow, shared]) (fun _ ↦ ()) p ∧
      p.readings narrow ∧ p.readings shared ∧ narrow ≠ shared := by
  let survivors := Selected (fun t ↦ Admissible lexicon t (.nominal .plural) surface)
    (neutral _) [narrow, shared]
  exact ⟨pack survivors (fun _ ↦ ()) (),
    packing_exists _ _ _ ⟨narrow, acyclic_tie.2.2.1, rfl⟩,
    ⟨acyclic_tie.2.2.1, rfl⟩, ⟨acyclic_tie.2.2.2, rfl⟩, distinct_analyses.1⟩

/-- Two declared lexical identities may have the same form; they are not a scope alternation. -/
def homographs : Lexicon Bool where
  noun _ number := number = .plural
  adjective _ := False
  nounForm _ number surface := number = .plural ∧ surface = (["forms"] : Surface)
  adjectiveForm _ _ := False

theorem homographs_admitted (identity : Bool) :
    Admissible homographs (.noun identity .plural) (.nominal .plural) (["forms"] : Surface) :=
  ⟨.noun rfl, .noun ⟨rfl, rfl⟩⟩

/-- Preserving lexical identity in the key prevents a same-text merge. -/
theorem same_surface_separate
    (p : Package (Syntax Bool) (Syntax Bool))
    (packed : Packs
      (fun t ↦ Admissible homographs t (.nominal .plural) (["forms"] : Surface)) id p) :
    ¬ (p.readings (.noun false .plural) ∧ p.readings (.noun true .plural)) := by
  apply different_classes_separate packed
  intro equal
  cases equal

namespace CrossHost

open Composition

def pair (a b : Syntax Lexeme) : Syntax Lexeme :=
  .node (.coordinate .and_ (.verbPhrase .plain)) [a, b]

def auxiliary (a : Syntax Lexeme) : Syntax Lexeme :=
  .node (.auxiliary .can .plain .plain .positive) [a]

def adjunct (a : Syntax Lexeme) : Syntax Lexeme :=
  .node (.adjunct (.verbPhrase .plain) .prepositionPhrase) [a, duringTurns]

def outside := adjunct (auxiliary (pair attack attack))
def inside := auxiliary (adjunct (pair attack attack))
def finalConjunct := auxiliary (pair attack (adjunct attack))
def surface : Surface := ["can", "attack", "and", "attack", "during", "turns"]
def alternatives (t : Syntax Lexeme) := t = outside ∨ t = inside ∨ t = finalConjunct

private theorem pair_admitted {a b : Syntax Lexeme} {as bs : Surface}
    (ha : Admissible lexicon a (.verbPhrase .plain) as)
    (hb : Admissible lexicon b (.verbPhrase .plain) bs) :
    Admissible lexicon (pair a b) (.verbPhrase .plain) (as ++ (["and"] : Surface) ++ bs) :=
  ⟨.node (.coordinate rfl) (.cons ha.1 (.cons hb.1 .nil)),
    .node (.cons ha.2 (.cons hb.2 .nil)) .coordinate⟩

private theorem auxiliary_admitted {a : Syntax Lexeme} {as : Surface}
    (ha : Admissible lexicon a (.verbPhrase .plain) as) :
    Admissible lexicon (auxiliary a) (.verbPhrase .plain) ((["can"] : Surface) ++ as) :=
  ⟨.node (.auxiliary ⟨rfl, rfl, rfl, rfl, rfl⟩) (.cons ha.1 .nil),
    .node (.cons ha.2 .nil) (.auxiliary (Or.inr (Or.inr (Or.inr ⟨rfl, rfl, rfl⟩))))⟩

private theorem adjunct_admitted {a : Syntax Lexeme} {as : Surface}
    (ha : Admissible lexicon a (.verbPhrase .plain) as) :
    Admissible lexicon (adjunct a) (.verbPhrase .plain) (as ++ (["during", "turns"] : Surface)) :=
  ⟨.node (.adjunct .verbal) (.cons ha.1 (.cons during_derives .nil)),
    .node (.cons ha.2 (.cons during_surface .nil)) .adjunct⟩

theorem cross_host_admitted {t : Syntax Lexeme} (h : alternatives t) :
    Admissible lexicon t (.verbPhrase .plain) surface := by
  have attackValid : Admissible lexicon attack (.verbPhrase .plain) (["attack"] : Surface) :=
    ⟨attack_derives, attack_surface⟩
  rcases h with rfl | rfl | rfl
  · exact adjunct_admitted (auxiliary_admitted (pair_admitted attackValid attackValid))
  · exact auxiliary_admitted (adjunct_admitted (pair_admitted attackValid attackValid))
  · exact auxiliary_admitted (pair_admitted attackValid (adjunct_admitted attackValid))

/-- One overt auxiliary survives all three attachment readings, across both hosts. -/
theorem cross_host_scope :
    ∃ p : Package (Syntax Lexeme) Unit, Packs alternatives (fun _ ↦ ()) p ∧
      p.readings outside ∧ p.readings inside ∧ p.readings finalConjunct ∧
      outside ≠ inside ∧ inside ≠ finalConjunct ∧ outside ≠ finalConjunct ∧
      ∀ t, p.readings t → Admissible lexicon t (.verbPhrase .plain) surface := by
  refine ⟨pack alternatives (fun _ ↦ ()) (),
    packing_exists _ _ _ ⟨outside, Or.inl rfl, rfl⟩,
    ⟨Or.inl rfl, rfl⟩, ⟨Or.inr (Or.inl rfl), rfl⟩,
    ⟨Or.inr (Or.inr rfl), rfl⟩, ?_, ?_, ?_, ?_⟩
  · intro h; cases h
  · intro h; cases h
  · intro h; cases h
  · intro t member
    exact cross_host_admitted member.1

end CrossHost

/-- An inhabited four-feature input checks the two successive preference stages. -/
def declaredPrinciples : Policy Claims Unit := ⟨fun _ ↦ (), id⟩

theorem ordered_principles_winner (claims : Claims) :
    Selected (fun _ ↦ True) declaredPrinciples
      [⟨false, false⟩, ⟨false, true⟩, ⟨true, false⟩, ⟨true, true⟩] claims ↔
      claims = ⟨true, true⟩ := by
  rcases claims with ⟨role, identity⟩
  cases role <;> cases identity <;>
    simp [Selected, Prefers, declaredPrinciples, Claims.rank]

/-- Removing the higher-ranked claim from admissibility leaves the admitted alternative alive. -/
theorem inadmissible_cannot_suppress :
    Selected (fun claims : Claims ↦ claims.frameRole = false) declaredPrinciples
      [⟨true, true⟩, ⟨false, true⟩] ⟨false, true⟩ := by
  simp [Selected, Prefers, declaredPrinciples, Claims.rank]

/-- These dependent categories have no verb-adjunct site in the modeled grammar. -/
theorem qualification_sites_excluded (form : InflectionalForm) (voice : Voice)
    (placement : Placement) :
    ¬ AdjunctLicense (.verbPhrase form voice) .adjectivePhrase placement ∧
    ¬ AdjunctLicense (.verbPhrase form voice) .measurePhrase placement ∧
    ¬ AdjunctLicense (.verbPhrase form voice) (.keywordPhrase) placement ∧
    ¬ AdjunctLicense (.verbPhrase form voice) (.document .quotedText) placement := by
  refine ⟨?_, ?_, ?_, ?_⟩ <;> intro h <;> cases h

/-- Measure Phrases cannot silently acquire the nominal-postmodifier site in this model. -/
theorem measure_postmodifier_excluded (number : Number) (placement : Placement) :
    ¬ AdjunctLicense (.nominal number) .measurePhrase placement := by
  intro h
  cases h

end English.SelectionWitnesses
