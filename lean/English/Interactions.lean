import English.SelectionWitnesses
import English.Documents

/-! Synthetic interaction witnesses for nested scope and document embedding. -/

open English English.Selection English.SelectionWitnesses
namespace English.Interactions.NestedScope
open Composition
def pairNP (a b : Syntax Lexeme) :=
  Syntax.node (.coordinate .and_ (.nounPhrase plural)) [a, b]
def npAdj (a b : Syntax Lexeme) :=
  Syntax.node (.adjunct (.nounPhrase plural) .prepositionPhrase) [a, b]
def during (a : Syntax Lexeme) := Syntax.node (.preposition .during (.nounPhrase plural)) [a]
def withTurns : Syntax Lexeme := .node (.preposition .with_ (.nounPhrase plural)) [turns]
def innerWide := npAdj (pairNP creatures artifacts) withTurns
def innerNarrow := pairNP creatures (npAdj artifacts withTurns)
def vpAdj (a b : Syntax Lexeme) :=
  Syntax.node (.adjunct (.verbPhrase .plain) .prepositionPhrase) [a, b]
def outerWide (inner : Syntax Lexeme) := vpAdj (CrossHost.pair attack attack) (during inner)
def outerNarrow (inner : Syntax Lexeme) := CrossHost.pair attack (vpAdj attack (during inner))
def nestedSurface : Surface :=
  ["attack", "and", "attack", "during", "creatures", "and", "artifacts", "with", "turns"]
theorem pairNP_valid {a b : Syntax Lexeme} {as bs : Surface}
    (ha : Admissible lexicon a (.nounPhrase plural) as)
    (hb : Admissible lexicon b (.nounPhrase plural) bs) :
    Admissible lexicon (pairNP a b) (.nounPhrase plural) (as ++ (["and"] : Surface) ++ bs) :=
  ⟨.node .coordinate (.cons ha.1 (.cons hb.1 .nil)),
   .node (.cons ha.2 (.cons hb.2 .nil)) .coordinate⟩
theorem npAdj_valid {a b : Syntax Lexeme} {as bs : Surface}
    (ha : Admissible lexicon a (.nounPhrase plural) as)
    (hb : Admissible lexicon b .prepositionPhrase bs) :
    Admissible lexicon (npAdj a b) (.nounPhrase plural) (as ++ bs) :=
  ⟨.node (.adjunct .nounPhrase) (.cons ha.1 (.cons hb.1 .nil)),
   .node (.cons ha.2 (.cons hb.2 .nil)) .adjunct⟩
theorem inner_valid :
    Admissible lexicon innerWide (.nounPhrase plural)
      (["creatures", "and", "artifacts", "with", "turns"] : Surface) ∧
    Admissible lexicon innerNarrow (.nounPhrase plural)
      (["creatures", "and", "artifacts", "with", "turns"] : Surface) := by
  have creaturesValid : Admissible lexicon creatures (.nounPhrase plural) ["creatures"] :=
    ⟨creatures_derives, creatures_surface⟩
  have artifactsValid : Admissible lexicon artifacts (.nounPhrase plural) ["artifacts"] :=
    ⟨artifacts_derives, artifacts_surface⟩
  have withValid : Admissible lexicon withTurns .prepositionPhrase ["with", "turns"] :=
    ⟨.node (.preposition ⟨Or.inr rfl, rfl⟩) (.cons turns_derives .nil),
      .node (.cons turns_surface .nil)
        (.preposition (m := (["with"] : Surface)) (Or.inr (Or.inl ⟨rfl, rfl⟩)))⟩
  exact ⟨npAdj_valid (pairNP_valid creaturesValid artifactsValid) withValid,
    pairNP_valid creaturesValid (npAdj_valid artifactsValid withValid)⟩
theorem nested_valid {inner : Syntax Lexeme}
    (h : Admissible lexicon inner (.nounPhrase plural)
      (["creatures", "and", "artifacts", "with", "turns"] : Surface)) :
    Admissible lexicon (outerWide inner) (.verbPhrase .plain) nestedSurface ∧
    Admissible lexicon (outerNarrow inner) (.verbPhrase .plain) nestedSurface := by
  have attackValid : Admissible lexicon attack (.verbPhrase .plain) ["attack"] :=
    ⟨attack_derives, attack_surface⟩
  have duringValid : Admissible lexicon (during inner) .prepositionPhrase
      ["during", "creatures", "and", "artifacts", "with", "turns"] :=
    ⟨.node (.preposition ⟨Or.inl rfl, rfl⟩) (.cons h.1 .nil),
      .node (.cons h.2 .nil) (.preposition (m := (["during"] : Surface)) (Or.inl ⟨rfl, rfl⟩))⟩
  have pairValid {a b : Syntax Lexeme} {as bs : Surface}
      (ha : Admissible lexicon a (.verbPhrase .plain) as)
      (hb : Admissible lexicon b (.verbPhrase .plain) bs) :
      Admissible lexicon (CrossHost.pair a b) (.verbPhrase .plain)
        (as ++ (["and"] : Surface) ++ bs) :=
    ⟨.node .coordinate (.cons ha.1 (.cons hb.1 .nil)),
     .node (.cons ha.2 (.cons hb.2 .nil)) .coordinate⟩
  have adjunctValid {a b : Syntax Lexeme} {as bs : Surface}
      (ha : Admissible lexicon a (.verbPhrase .plain) as)
      (hb : Admissible lexicon b .prepositionPhrase bs) :
      Admissible lexicon (vpAdj a b) (.verbPhrase .plain) (as ++ bs) :=
    ⟨.node (.adjunct .verbal) (.cons ha.1 (.cons hb.1 .nil)),
     .node (.cons ha.2 (.cons hb.2 .nil)) .adjunct⟩
  exact ⟨adjunctValid (pairValid attackValid attackValid) duringValid,
    pairValid attackValid (adjunctValid attackValid duringValid)⟩

theorem four_distinct :
    outerWide innerWide ≠ outerWide innerNarrow ∧
    outerWide innerWide ≠ outerNarrow innerWide ∧
    outerWide innerWide ≠ outerNarrow innerNarrow ∧
    outerWide innerNarrow ≠ outerNarrow innerWide ∧
    outerWide innerNarrow ≠ outerNarrow innerNarrow ∧
    outerNarrow innerWide ≠ outerNarrow innerNarrow := by
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩ <;> intro h <;> cases h
end English.Interactions.NestedScope


namespace English.Interactions.NestedScope

open Selection SelectionWitnesses Composition

def alternatives : List (Syntax Lexeme) :=
  [outerWide innerWide, outerWide innerNarrow, outerNarrow innerWide, outerNarrow innerNarrow]

theorem alternatives_admitted {tree : Syntax Lexeme} (member : tree ∈ alternatives) :
    Admissible lexicon tree (.verbPhrase .plain) nestedSurface := by
  simp only [alternatives, List.mem_cons, List.not_mem_nil, or_false] at member
  rcases member with rfl | rfl | rfl | rfl
  · exact (nested_valid inner_valid.1).1
  · exact (nested_valid inner_valid.2).1
  · exact (nested_valid inner_valid.1).2
  · exact (nested_valid inner_valid.2).2

theorem all_four_survive {tree : Syntax Lexeme} (member : tree ∈ alternatives) :
    Selected (fun t ↦ Admissible lexicon t (.verbPhrase .plain) nestedSurface)
      (neutral _) alternatives tree :=
  (neutral_selected _ _ _).mpr ⟨member, alternatives_admitted member⟩

/-- This local class has four readings; the abstract correlation example cannot prune it. -/
theorem all_four_packed :
    ∃ p : Package (Syntax Lexeme) Unit,
      Packs (Selected (fun t ↦ Admissible lexicon t (.verbPhrase .plain) nestedSurface)
        (neutral _) alternatives) (fun _ ↦ ()) p ∧
      ∀ t, p.readings t ↔ t ∈ alternatives := by
  let survivors := Selected (fun t ↦ Admissible lexicon t (.verbPhrase .plain) nestedSurface)
    (neutral _) alternatives
  refine ⟨pack survivors (fun _ ↦ ()) (),
    packing_exists _ _ _ ⟨outerWide innerWide, all_four_survive (by simp [alternatives]), rfl⟩, ?_⟩
  intro t
  constructor
  · exact fun h ↦ h.1.1
  · exact fun h ↦ ⟨all_four_survive h, rfl⟩

def quotedTree (tree : Syntax Lexeme) : Syntax Lexeme :=
  .node (.document .quote) [.node (.document .document) [.node (.document .ordinary)
    [.node (.document .body) [.node (.document .sentence) [.node .imperative [tree]]]]]]

def quotedSurface : Surface :=
  [.opening "\"", "Attack", "and", "attack", "during", "creatures", "and", "artifacts",
   "with", "turns", .closing ".", .closing "\""]

theorem quoted_admitted {tree : Syntax Lexeme}
    (h : Admissible lexicon tree (.verbPhrase .plain) nestedSurface) :
    Admissible lexicon (quotedTree tree) (.document .quotedText) quotedSurface := by
  let instruction : Documents.Witness Lexeme lexicon (.clause .finite) :=
    ⟨.node .imperative [tree], nestedSurface,
      .node .imperative (.cons h.1 .nil), .node (.cons h.2 .nil) .imperative⟩
  let sentence := Documents.unary instruction .sentence .sentence
  let body := Documents.unary sentence .body .body
  let ability := Documents.unary body .ordinary .ordinary
  let document := Documents.unary ability .document .document
  let quoted := Documents.unary document .quote .quote
  exact ⟨quoted.derives, quoted.realizes⟩

theorem quoted_distinction {left right : Syntax Lexeme} (distinct : left ≠ right) :
    quotedTree left ≠ quotedTree right := by
  intro h
  apply distinct
  simpa [quotedTree] using h

/-- Lifting this local scope class through a common quotation retains every complete reading. -/
theorem quoted_packing_retains_all :
    ∃ p : Package (Syntax Lexeme) Unit,
      Packs (Selected (fun t ↦ Admissible lexicon t (.document .quotedText) quotedSurface)
        (neutral _) (alternatives.map quotedTree)) (fun _ ↦ ()) p ∧
      ∀ t, t ∈ alternatives → p.readings (quotedTree t) := by
  have survives {t : Syntax Lexeme} (mem : t ∈ alternatives) :
      Selected (fun t ↦ Admissible lexicon t (.document .quotedText) quotedSurface)
        (neutral _) (alternatives.map quotedTree) (quotedTree t) :=
    (neutral_selected _ _ _).mpr
      ⟨by simp [List.mem_map]; exact ⟨t, mem, rfl⟩, quoted_admitted (alternatives_admitted mem)⟩
  let survivors := Selected (fun t ↦ Admissible lexicon t (.document .quotedText) quotedSurface)
    (neutral _) (alternatives.map quotedTree)
  exact ⟨pack survivors (fun _ ↦ ()) (),
    packing_exists _ _ _ ⟨quotedTree (outerWide innerWide), survives (by simp [alternatives]), rfl⟩,
    fun _ mem ↦ ⟨survives mem, rfl⟩⟩

end English.Interactions.NestedScope

namespace English.Interactions.Quotation

open Documents

def gainNested : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .gain .plain quotedFrame) [nestedQuote.tree],
   (["gain"] : Surface) ++ [nestedQuote.surface].flatten,
   .verb ⟨rfl, rfl, Or.inr ⟨rfl, Or.inr rfl⟩⟩
     (.argument (complement := ⟨.object, .document .quotedText⟩) nestedQuote.derives .nil),
   .node (.cons nestedQuote.realizes .nil)
     (.verb (lexicon := lexicon) (head := .gain) (form := .plain)
       (v := ["gain"]) ⟨rfl, Or.inr ⟨rfl, rfl⟩⟩)⟩

def instruction : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node .imperative [gainNested.tree], gainNested.surface,
   .node .imperative (.cons gainNested.derives .nil),
   .node (.cons gainNested.realizes .nil) .imperative⟩

def sentence := unary instruction .sentence .sentence

/-- Extending the earlier nested-quote fixture with another sentence adds no second period. -/
theorem deep_quote_written :
    Written lexicon sentence.tree (.document .sentence) "Gain \"Gain 'Attack.'\"" := by
  refine ⟨sentence.surface, ⟨sentence.derives, sentence.realizes⟩, ?_⟩
  exact .cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))

end English.Interactions.Quotation
