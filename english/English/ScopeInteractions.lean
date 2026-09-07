import English.GrammaticalScope
import English.Interactions

namespace English.GrammaticalScope

open Witnesses in
theorem independent_modifier_regions :
    ∃ initial : SchemaWitness lexicon (.nominal .plural)
        (surface ++ (["and"] : Surface) ++ surface),
      ∃ final : SchemaWitness lexicon (.nominal .plural)
        (surface ++ (["and"] : Surface) ++ surface),
      initial.val = .coordinate shared shared ∧ final.val = .coordinate narrow narrow ∧
      ScopeRelated initial final := by
  have pairValid {a b : Syntax Lexeme}
      (ha : Admissible lexicon a (.nominal .plural) surface)
      (hb : Admissible lexicon b (.nominal .plural) surface) :
      Admissible lexicon (.coordinate a b) (.nominal .plural)
        (surface ++ (["and"] : Surface) ++ surface) :=
    ⟨.node (.coordinate rfl) (.cons ha.1 (.cons hb.1 .nil)),
     .node (.cons ha.2 (.cons hb.2 .nil)) .coordinate⟩
  have move : ScopeStep shared narrow :=
    .direct (.modifier .and_ .plural (.adjective Lexeme.white)
      (.noun Lexeme.creature .plural) (.noun Lexeme.artifact .plural))
  have initial := pairValid distinct_analyses.2.2 distinct_analyses.2.2
  have middle := pairValid distinct_analyses.2.1 distinct_analyses.2.2
  have final := pairValid distinct_analyses.2.1 distinct_analyses.2.1
  exact ⟨⟨_,initial⟩, ⟨_,final⟩, rfl, rfl,
    two_regions (.coordinate .and_ (.nominal .plural)) move move initial middle final⟩
open Composition Interactions.NestedScope

private def wideReading : SchemaWitness lexicon (.verbPhrase .plain) nestedSurface :=
  ⟨outerWide innerWide, (nested_valid inner_valid.1).1⟩

private theorem outer_move (inner : Syntax Lexeme) :
    ScopeStep (outerWide inner) (outerNarrow inner) :=
  .direct (.postmodifier .and_ (.verbPhrase .plain) attack attack (during inner) rfl)

private theorem inner_move : ScopeStep (outerWide innerWide) (outerWide innerNarrow) :=
  .node (.adjunct (.verbPhrase .plain) .prepositionPhrase) [PreferenceWitnesses.CrossHost.pair
    attack attack] []
    (.node (.preposition Composition.Lexeme.during (.nounPhrase plural)) [] []
      (.direct (.postmodifier .and_ (.nounPhrase plural) creatures artifacts withTurns rfl)))

theorem nested_scope_related (tree : SchemaWitness lexicon (.verbPhrase .plain) nestedSurface)
    (member : tree.val ∈ alternatives) : ScopeRelated wideReading tree := by
  rcases tree with ⟨tree, valid⟩
  simp only [alternatives, List.mem_cons, List.not_mem_nil, or_false] at member
  rcases member with rfl | rfl | rfl | rfl
  · exact .refl _
  · exact .step inner_move
  · exact .step (outer_move innerWide)
  · exact .trans (b := ⟨outerWide innerNarrow, (nested_valid inner_valid.2).1⟩)
      (.step inner_move) (.step (outer_move innerNarrow))

/-- The grammar-derived class retains every complete nested-mobile alternative. -/
theorem nested_package_exact (tree : SchemaWitness lexicon (.verbPhrase .plain) nestedSurface) :
    (package (fun t ↦ t.val ∈ alternatives) wideReading).readings tree ↔
      tree.val ∈ alternatives := by
  rw [package_exact]
  exact ⟨And.left, fun member ↦ ⟨member, .symm (nested_scope_related tree member)⟩⟩

private theorem quoted_step {a b : Syntax Lexeme} (h : ScopeStep a b) :
    ScopeStep (quotedTree a) (quotedTree b) :=
  .node (.document .quote) [] [] (.node (.document .document) [] []
    (.node (.document .ordinary) [] [] (.node (.document .body) [] []
      (.node (.document .sentence) [] [] (.node .imperative [] [] h)))))

theorem quoted_scope_related (tree : SchemaWitness lexicon (.verbPhrase .plain) nestedSurface)
    (member : tree.val ∈ alternatives) :
    ScopeRelated ⟨quotedTree wideReading.val, quoted_admitted wideReading.property⟩
      ⟨quotedTree tree.val, quoted_admitted tree.property⟩ :=
  map_related quotedTree quoted_step (fun a ↦ quoted_admitted a.property)
    (nested_scope_related tree member)

/-- `Interactions.NestedScope.all_four_packed` re-keyed on the grammatical scope class. The
`Unit` key of that statement cannot tell anything apart, so its packing half held for want of a
key; `scopeClass` does separate classes (`homographs_separate`, `association_separate` below), and
the four readings are retained here because they are scope-related, not by default. -/
theorem nested_alternatives_packed :
    ∃ p : Preference.Package (SchemaWitness lexicon (.verbPhrase .plain) nestedSurface)
        (Quotient (scopeSetoid lexicon (.verbPhrase .plain) nestedSurface)),
      Preference.Packs (fun t ↦ t.val ∈ alternatives) scopeClass p ∧
      ∀ t, p.readings t ↔ t.val ∈ alternatives :=
  ⟨package (fun t ↦ t.val ∈ alternatives) wideReading,
    Preference.packing_exists _ _ _ ⟨wideReading, by simp [alternatives, wideReading], rfl⟩,
    nested_package_exact⟩

private def quotedWide : SchemaWitness lexicon (.document .quotedText) quotedSurface :=
  ⟨quotedTree wideReading.val, quoted_admitted wideReading.property⟩

private def quotedAlternatives
    (tree : SchemaWitness lexicon (.document .quotedText) quotedSurface) : Prop :=
  ∃ a, a ∈ alternatives ∧ tree.val = quotedTree a

/-- `Interactions.NestedScope.quoted_packing_retains_all` re-keyed on the grammatical scope class:
lifting the class through a quotation retains every reading under a key that distinguishes
classes, not under a key that cannot distinguish anything. -/
theorem quoted_alternatives_packed :
    ∃ p : Preference.Package (SchemaWitness lexicon (.document .quotedText) quotedSurface)
        (Quotient (scopeSetoid lexicon (.document .quotedText) quotedSurface)),
      Preference.Packs quotedAlternatives scopeClass p ∧
      ∀ a, ∀ member : a ∈ alternatives,
        p.readings ⟨quotedTree a, quoted_admitted (alternatives_admitted member)⟩ := by
  refine ⟨package quotedAlternatives quotedWide,
    Preference.packing_exists _ _ _
      ⟨quotedWide, ⟨_, by simp [alternatives, wideReading], rfl⟩, rfl⟩, ?_⟩
  intro a member
  refine (package_exact _ _ _).mpr ⟨⟨a, member, rfl⟩, ?_⟩
  exact .symm (quoted_scope_related ⟨a, alternatives_admitted member⟩ member)

/-- Homographic noun lexemes cannot collapse merely because their text is identical. -/
theorem homographs_separate :
    scopeClass ⟨.noun false .plural, PreferenceWitnesses.homographs_admitted false⟩ ≠
      scopeClass ⟨.noun true .plural, PreferenceWitnesses.homographs_admitted true⟩ := by
  intro same
  have leaves := related_preserves_lexemes ((same_class_iff _ _).mp same)
  cases leaves

private theorem pair_valid {a b : Syntax Lexeme} {as bs : Surface}
    (ha : Admissible lexicon a (.verbPhrase .plain) as)
    (hb : Admissible lexicon b (.verbPhrase .plain) bs) :
    Admissible lexicon (PreferenceWitnesses.CrossHost.pair a b) (.verbPhrase .plain)
      (as ++ (["and"] : Surface) ++ bs) :=
  ⟨.node (.coordinate rfl) (.cons ha.1 (.cons hb.1 .nil)),
   .node (.cons ha.2 (.cons hb.2 .nil)) .coordinate⟩

/-- Both association trees are grammatical, but scope does not reassociate their hosts. -/
theorem association_separate :
    ∃ a b : SchemaWitness lexicon (.verbPhrase .plain) ["attack", "and", "attack", "and", "attack"],
      a.val = PreferenceWitnesses.CrossHost.pair
        (PreferenceWitnesses.CrossHost.pair attack attack) attack ∧
      b.val = PreferenceWitnesses.CrossHost.pair attack
        (PreferenceWitnesses.CrossHost.pair attack attack) ∧ scopeClass a ≠ scopeClass b := by
  have av : Admissible lexicon attack (.verbPhrase .plain) ["attack"] :=
    ⟨attack_derives, attack_surface⟩
  refine ⟨⟨_, pair_valid (pair_valid av av) av⟩,
    ⟨_, pair_valid av (pair_valid av av)⟩, rfl, rfl, ?_⟩
  intro same
  have hosts := related_preserves_hosts ((same_class_iff _ _).mp same)
  cases hosts

namespace CrossHost
open PreferenceWitnesses.CrossHost

/-- The `ScopeMove.auxiliary` constructor's first inhabitant. `outside` and `inside` are the two
trees it relates: the temporal adjunct outside the auxiliary, and inside it. -/
theorem auxiliary_scope_move : ScopeStep outside inside :=
  .direct (.auxiliary Lexeme.can .plain .plain .positive .active .active
    (pair attack attack) duringTurns)

def outsideWitness : SchemaWitness lexicon (.verbPhrase .plain) surface :=
  ⟨outside, cross_host_admitted (Or.inl rfl)⟩
def insideWitness : SchemaWitness lexicon (.verbPhrase .plain) surface :=
  ⟨inside, cross_host_admitted (Or.inr (Or.inl rfl))⟩

theorem auxiliary_scope_related : ScopeRelated outsideWitness insideWitness :=
  .step auxiliary_scope_move

/-- Nonvacuity: the move is a scope alternation, not a different analysis — the two trees carry
the same host structure, so `related_preserves_hosts` has content here rather than relating a
tree to itself. The trees themselves are distinct. -/
theorem auxiliary_move_keeps_hosts :
    outside ≠ inside ∧ hostStructure outside = hostStructure inside :=
  ⟨(fun h ↦ by cases h), related_preserves_hosts auxiliary_scope_related⟩

end CrossHost

end English.GrammaticalScope
