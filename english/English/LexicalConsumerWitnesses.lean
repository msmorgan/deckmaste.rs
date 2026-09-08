import English.MeasureWitnesses

namespace English.LexicalConsumerWitnesses
open LexicalWitnesses

abbrev grammar := Lexical.lexicon environment

def named := word .nameVerb (.verb .pastParticiple none) ["named"]
def cardName := word .cardName (.identity .name) ["Powerstone", "Shard"]
def defending := word .defend (.verb .gerundParticiple none) ["defending"]

theorem named_licensed : named.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
theorem cardName_licensed : cardName.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
theorem defending_licensed : defending.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

def nameComplement : Reading Lexeme := .node (.namePredicate named) [.identity cardName .name]
def namedCreature : Reading Lexeme :=
  .node (.namedModifier .singular) [.noun creature .singular, nameComplement]
def defendingCreature : Reading Lexeme :=
  .node (.participialAttributive defending .singular) [.noun creature .singular]

theorem named_frame :
    grammar.verb named .pastParticiple .passive [.argument ⟨.complement, .name⟩] :=
  ⟨named_licensed, ⟨none, rfl⟩, declaration .nameVerb, rfl, rfl, rfl, rfl⟩

theorem defending_frame : grammar.verb defending .gerundParticiple .active [] :=
  ⟨defending_licensed, ⟨none, rfl⟩, declaration .defend, rfl, rfl, rfl, rfl⟩

theorem defending_attributive : grammar.participialAttributive defending :=
  ⟨defending_licensed, declaration .defend, rfl, rfl⟩

theorem name_complement_admitted : Reading.Admitted environment [] nameComplement .namePredicate
    ["named", "Powerstone", "Shard"] := by
  refine ⟨⟨.node (.namePredicate named_frame)
    (.cons (.identity (lexicon := grammar) .name ⟨cardName_licensed, _, rfl⟩) .nil),
    ?_, ?_, ?_⟩,
    .node (.cons (.identity (lexicon := grammar) ⟨cardName_licensed, ⟨_, rfl⟩, rfl⟩) .nil)
      (.namePredicate (lexicon := grammar) (head := named)
        ⟨named_licensed, ⟨none, rfl⟩, rfl⟩)⟩
  · simp [nameComplement, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [nameComplement, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · refine ⟨?_, ?_⟩
    · simp [nameComplement, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem named_nominal_admitted : Reading.Admitted environment [] namedCreature (.nominal .singular)
    ["creature", "named", "Powerstone", "Shard"] := by
  refine ⟨⟨.node .namedModifier
    (.cons (.noun (lexicon := grammar) ⟨creature_licensed, .count, rfl⟩)
      (.cons name_complement_admitted.1.1 .nil)), ?_, ?_, ?_⟩,
    .node (.cons (.noun (lexicon := grammar) ⟨creature_licensed, ⟨.count, rfl⟩, rfl⟩)
      (.cons name_complement_admitted.2 .nil)) .namedModifier⟩
  · simpa [namedCreature, Features.Conforms, Features.ChildrenConform, Features.Local] using
      name_complement_admitted.1.2.1
  · simpa [namedCreature, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local] using
      name_complement_admitted.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [namedCreature, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
        using name_complement_admitted.1.2.2.2.1
    · decide

theorem defending_nominal_admitted : Reading.Admitted environment [] defendingCreature
    (.nominal .singular) ["defending", "creature"] := by
  refine ⟨⟨.node (.participialAttributive defending_attributive defending_frame)
    (.cons (.noun (lexicon := grammar) ⟨creature_licensed, .count, rfl⟩) .nil), ?_, ?_, ?_⟩,
    .node (.cons (.noun (lexicon := grammar) ⟨creature_licensed, ⟨.count, rfl⟩, rfl⟩) .nil)
      (.participialAttributive (lexicon := grammar) (head := defending)
        ⟨defending_licensed, ⟨none, rfl⟩, rfl⟩)⟩
  · simp [defendingCreature, Features.Conforms, Features.ChildrenConform, Features.Local,
      Features.containsTarget]
  · simp [defendingCreature, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · refine ⟨?_, ?_⟩
    · simp [defendingCreature, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem name_is_not_an_ordinary_object (lexicon : Lexicon Lexeme) (head : Lexeme) :
    ¬ Production lexicon (.namePredicate head) [.nounPhrase singular] .namePredicate := by
  intro h
  cases h

theorem past_participle_not_licensed_attributively : ¬ grammar.participialAttributive named := by
  rintro ⟨_, d, found, licensed⟩
  change some (declaration .nameVerb) = some d at found
  cases found
  cases licensed

def amount := word .amount (.noun .singular .count) ["amount"]
def ofMarker := word .of_ .fixed ["of"]
def greenSymbol := word .greenSymbol (.word (.document .symbol)) [.symbol "{G}"]
def symbols : Reading Lexeme :=
  .node .symbolSequence [.word greenSymbol (.document .symbol)]
def measuredNominal : Reading Lexeme :=
  .node (.nominalComplement amount ofMarker .singular) [symbols]

theorem amount_licensed : amount.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
theorem of_licensed : ofMarker.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])
theorem symbol_licensed : greenSymbol.Licensed environment :=
  row_licensed _ _ _ _ (by decide) (by decide) (by simp [rows])

theorem amount_frame : grammar.nounComplement amount ofMarker .symbolSequence :=
  ⟨amount_licensed, of_licensed, declaration .amount, rfl, rfl, rfl, rfl⟩

theorem symbols_admitted : Reading.Admitted environment [] symbols .symbolSequence
    [.symbol "{G}"] := by
  refine ⟨⟨.node (.symbolSequence (n := 0))
    (.cons (.word (lexicon := grammar) .symbol ⟨symbol_licensed, Or.inl ⟨_, rfl⟩⟩) .nil),
    ?_, ?_, ?_⟩,
    .node (.cons (.word (lexicon := grammar) ⟨symbol_licensed, Or.inl ⟨_, rfl⟩, rfl⟩) .nil)
      .symbolSequence⟩
  · simp [symbols, Features.Conforms, Features.ChildrenConform, Features.Local]
  · simp [symbols, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local]
  · refine ⟨?_, ?_⟩
    · simp [symbols, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
    · decide

theorem amount_complement_admitted : Reading.Admitted environment [] measuredNominal
    (.nominal .singular) ["amount", "of", .symbol "{G}"] := by
  refine ⟨⟨.node (.nominalComplement ⟨amount_licensed, .count, rfl⟩ amount_frame)
    (.cons symbols_admitted.1.1 .nil), ?_, ?_, ?_⟩,
    .node (.cons symbols_admitted.2 .nil)
      (.nominalComplement (lexicon := grammar) (head := amount) (marker := ofMarker)
        ⟨amount_licensed, ⟨.count, rfl⟩, rfl⟩ ⟨of_licensed, rfl⟩)⟩
  · simpa [measuredNominal, Features.Conforms, Features.ChildrenConform, Features.Local] using
      symbols_admitted.1.2.1
  · simpa [measuredNominal, Dependencies.Safe, Dependencies.ChildrenSafe, Dependencies.Local] using
      symbols_admitted.1.2.2.1
  · refine ⟨?_, ?_⟩
    · simpa [measuredNominal, Reading.GrammarConforms, Reading.ChildrenConform, Reading.LocalGrammar]
        using symbols_admitted.1.2.2.2.1
    · decide

theorem ordinary_noun_does_not_select_symbols :
    ¬ grammar.nounComplement creature ofMarker .symbolSequence := by
  rintro ⟨_, _, d, found, licensed⟩
  change some (declaration .creature) = some d at found
  cases found
  cases licensed.1

theorem symbol_complement_is_nonempty (lexicon : Lexicon Lexeme) :
    ¬ Production lexicon .symbolSequence [] .symbolSequence := by
  intro h
  cases h

theorem amount_complement_keeps_countability :
    Features.NominalUse (Lexical.features environment) measuredNominal .count :=
  .nominalComplement ⟨.singular, rfl⟩

end English.LexicalConsumerWitnesses
