import English.LexicalWitnesses

namespace English.MeasureWitnesses
open LexicalWitnesses

abbrev grammar := Lexical.lexicon environment

def three := word .three (.word .unsignedScalar) [.symbol "3"]
def scalarVariable := word .scalarVariable (.word .unsignedScalar) [.symbol "X"]

theorem three_licensed : three.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem variable_licensed : scalarVariable.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

def unsigned (head : WordForm Lexeme) : Reading Lexeme :=
  .node .unsignedScalar [.word head .unsignedScalar]

def signed (sign : ScalarSign) (head : WordForm Lexeme) : Reading Lexeme :=
  .node (.signedScalar sign) [.word head .unsignedScalar]

def pair (left right : Reading Lexeme) : Reading Lexeme := .node .slashPair [left, right]

theorem unsigned_admitted (head : WordForm Lexeme) (licensed : head.Licensed environment)
    (bundle : head.bundle = .word .unsignedScalar) :
    Reading.Admitted environment [] (unsigned head) .scalarComponent head.surface := by
  refine ⟨⟨.node .unsignedScalar
    (.cons (.word (lexicon := grammar) .unsignedScalar ⟨licensed, Or.inl ⟨_, bundle⟩⟩)
      .nil), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨licensed, Or.inl ⟨_, bundle⟩, rfl⟩)
      .nil) .unsignedScalar⟩
  · simp [unsigned, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [unsigned, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simpa [unsigned, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar] using Casing.word_valid head

theorem signed_admitted (sign : ScalarSign) (head : WordForm Lexeme)
    (licensed : head.Licensed environment) (bundle : head.bundle = .word .unsignedScalar) :
    Reading.Admitted environment [] (signed sign head) .scalarComponent
      (sign.surface ++ head.surface) := by
  refine ⟨⟨.node .signedScalar
    (.cons (.word (lexicon := grammar) .unsignedScalar ⟨licensed, Or.inl ⟨_, bundle⟩⟩)
      .nil), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨licensed, Or.inl ⟨_, bundle⟩, rfl⟩)
      .nil) .signedScalar⟩
  · simp [signed, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [signed, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simpa [signed, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar] using Casing.word_valid head

theorem pair_admitted {left right : Reading Lexeme} {a b : Surface}
    (l : Reading.Admitted environment [] left .scalarComponent a)
    (r : Reading.Admitted environment [] right .scalarComponent b)
    (interior : (Casing.summary right).interior = true) :
    Reading.Admitted environment [] (pair left right) .slashPair
      (a ++ [.symbol "/"] ++ b) := by
  refine ⟨⟨.node .slashPair (.cons l.1.1 (.cons r.1.1 .nil)), ?_, ?_, ?_⟩,
    .node (.cons l.2 (.cons r.2 .nil)) .slashPair⟩
  · simpa [pair, Features.Conforms, Features.ChildrenConform, Features.Local] using
      And.intro l.1.2.1 r.1.2.1
  · simpa [pair, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local] using
      And.intro l.1.2.2.1 r.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [pair, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar] using
        And.intro l.1.2.2.2.1 r.1.2.2.2.1
    · apply Casing.append_valid _ _ l.1.2.2.2.2
      simp only [Casing.children, Casing.append]
      split <;> simp_all

theorem signed_pair : Reading.Admitted environment []
    (pair (signed .plus three) (signed .minus three)) .slashPair
    [.symbol "+", .symbol "3", .symbol "/", .symbol "-", .symbol "3"] :=
  pair_admitted (signed_admitted .plus three three_licensed rfl)
    (signed_admitted .minus three three_licensed rfl) rfl

theorem variable_pair : Reading.Admitted environment []
    (pair (unsigned scalarVariable) (unsigned scalarVariable)) .slashPair
    [.symbol "X", .symbol "/", .symbol "X"] :=
  pair_admitted (unsigned_admitted scalarVariable variable_licensed rfl)
    (unsigned_admitted scalarVariable variable_licensed rfl) rfl

theorem signed_pair_spells :
    Spells [.symbol "+", .symbol "3", .symbol "/", .symbol "-", .symbol "3"] "+3/-3" :=
  .cons (.cons (.cons (.cons .single)))

theorem signs_and_order_survive :
    pair (signed .plus three) (signed .minus three) ≠
      pair (signed .minus three) (signed .plus three) := by
  intro h
  cases h

theorem no_third_component (lexicon : Lexicon Lexeme) :
    ¬ Production lexicon .slashPair [.scalarComponent, .scalarComponent, .scalarComponent]
      .slashPair := by intro h; cases h

theorem no_nested_pair (lexicon : Lexicon Lexeme) :
    ¬ Production lexicon .slashPair [.slashPair, .scalarComponent] .slashPair := by
  intro h; cases h

theorem no_double_sign (lexicon : Lexicon Lexeme) (sign : ScalarSign) :
    ¬ Production lexicon (.signedScalar sign) [.scalarComponent] .scalarComponent := by
  intro h; cases h

theorem pair_is_not_a_count (lexicon : Lexicon Lexeme) :
    ¬ Production lexicon .slashMeasure [.slashPair] (.cardinalNumeral .plural) := by
  intro h; cases h

def signedMeasure : Reading Lexeme :=
  .node .slashMeasure [pair (signed .plus three) (signed .minus three)]
def modifiedCreatures : Reading Lexeme :=
  .node (.slashModifier .plural)
    [pair (unsigned scalarVariable) (unsigned scalarVariable), .noun creatures .plural]
def two := word .two (.word (.cardinalNumeral .plural)) ["two"]
def count : Reading Lexeme := .node (.quantify .plural) [.word two (.cardinalNumeral .plural)]
def countedCreatures : Reading Lexeme :=
  .node (.determine .plural) [count, modifiedCreatures]
def get := word .get (.verb .plain none) ["get"]
def adjustment : Reading Lexeme :=
  .node (.verb get .plain [.argument ⟨.complement, .measurePhrase .pair⟩]) [signedMeasure]

theorem get_licensed : get.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
theorem two_licensed : two.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem signed_measure_admitted : Reading.Admitted environment [] signedMeasure
    (.measurePhrase .pair) [.symbol "+", .symbol "3", .symbol "/", .symbol "-", .symbol "3"] := by
  refine ⟨⟨.node .slashMeasure (.cons signed_pair.1.1 .nil), ?_, ?_, ?_⟩,
    .node (.cons signed_pair.2 .nil) .slashMeasure⟩
  · simpa [signedMeasure, Features.Conforms, Features.ChildrenConform, Features.Local] using
      signed_pair.1.2.1
  · simpa [signedMeasure, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local] using
      signed_pair.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [signedMeasure, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
        using signed_pair.1.2.2.2.1
    · decide

theorem modified_creatures_admitted : Reading.Admitted environment [] modifiedCreatures
    (.nominal .plural) [.symbol "X", .symbol "/", .symbol "X", "creatures"] := by
  refine ⟨⟨.node .slashModifier (.cons variable_pair.1.1
    (.cons (.noun (lexicon := grammar) ⟨creatures_licensed, .count, rfl⟩) .nil)), ?_, ?_, ?_⟩,
    .node (.cons variable_pair.2
      (.cons (.noun (lexicon := grammar) ⟨creatures_licensed, ⟨.count, rfl⟩, rfl⟩) .nil))
      .slashModifier⟩
  · simpa [modifiedCreatures, Features.Conforms, Features.ChildrenConform, Features.Local,
      Features.containsTarget] using
      And.intro (Features.NominalUse.noun (features := Lexical.features environment)
        (head := creatures) (number := .plural) ⟨.plural, rfl⟩) variable_pair.1.2.1
  · simpa [modifiedCreatures, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
      using variable_pair.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [modifiedCreatures, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
        using variable_pair.1.2.2.2.1
    · decide

theorem count_admitted : Reading.Admitted environment [] count (.determinativePhrase .plural)
    ["two"] := by
  refine ⟨⟨.node .quantify (.cons
    (.word (lexicon := grammar) .quantity ⟨two_licensed, Or.inl ⟨_, rfl⟩⟩) .nil), ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨two_licensed, Or.inl ⟨_, rfl⟩, rfl⟩) .nil) .quantify⟩
  · simp [count, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [count, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · simp [count, two, word, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]

theorem counted_creatures_admitted : Reading.Admitted environment [] countedCreatures
    (.nounPhrase plural) ["two", .symbol "X", .symbol "/", .symbol "X", "creatures"] := by
  refine ⟨⟨.node .determine (.cons count_admitted.1.1
    (.cons modified_creatures_admitted.1.1 .nil)), ?_, ?_, ?_⟩,
    .node (.cons count_admitted.2 (.cons modified_creatures_admitted.2 .nil)) .determine⟩
  · refine ⟨Or.inl ⟨.count, .slashModifier (.noun ⟨.plural, rfl⟩), .numeral⟩,
      count_admitted.1.2.1, modified_creatures_admitted.1.2.1, trivial⟩
  · simpa [countedCreatures, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
      using And.intro count_admitted.1.2.2.1 modified_creatures_admitted.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [countedCreatures, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
        using And.intro count_admitted.1.2.2.2.1 modified_creatures_admitted.1.2.2.2.1
    · decide

theorem adjustment_admitted : Reading.Admitted environment [] adjustment (.verbPhrase .plain)
    ["get", .symbol "+", .symbol "3", .symbol "/", .symbol "-", .symbol "3"] := by
  refine ⟨⟨.verb
    ⟨get_licensed, ⟨none, rfl⟩, declaration .get, rfl, rfl, rfl, rfl⟩
    (.argument (headGaps := []) (tailGaps := [])
      (slot := ⟨.complement, .measurePhrase .pair⟩) signed_measure_admitted.1.1 .nil), ?_, ?_, ?_⟩,
    .node (.cons signed_measure_admitted.2 .nil)
      (.verb (lexicon := grammar) (head := get) ⟨get_licensed, ⟨none, rfl⟩, rfl⟩)⟩
  · refine ⟨?_, signed_measure_admitted.1.2.1, trivial⟩
    simp [adjustment, Features.Local, signedMeasure, FrameCases, CaseAt]
  · simpa [adjustment, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local] using
      signed_measure_admitted.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [adjustment, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar] using
      signed_measure_admitted.1.2.2.2.1
    · decide

end English.MeasureWitnesses
