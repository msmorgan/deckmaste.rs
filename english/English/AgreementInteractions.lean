import English.Documents

namespace English.AgreementInteractions
open Documents

inductive Lexeme where
  | pronoun (agreement : Agreement)
  | attack

def pronounText : Agreement → String
  | ⟨.first, .singular⟩ => "I"
  | ⟨.first, .plural⟩ => "we"
  | ⟨.second, _⟩ => "you"
  | ⟨.third, .singular⟩ => "it"
  | ⟨.third, .plural⟩ => "they"

def lexicon : Lexicon Lexeme where
  noun _ _ := False
  adjective _ := False
  nounForm _ _ _ := False
  adjectiveForm _ _ := False
  word head category := ∃ agreement, head = .pronoun agreement ∧ category = .nounPhrase agreement
  wordForm head category surface := ∃ agreement,
    head = .pronoun agreement ∧ category = .nounPhrase agreement ∧
      surface = [Atom.word (pronounText agreement)]
  verb head form voice frame := head = .attack ∧ form = .plain ∧ voice = .active ∧ frame = []
  verbForm head form surface := head = .attack ∧ form = .plain ∧ surface = (["attack"] : Surface)
  finite head agreement form := head = .attack ∧ form = .plain ∧ agreement.concord = .other

def pronoun (agreement : Agreement) : Witness Lexeme lexicon (.nounPhrase agreement) :=
  ⟨.word (.pronoun agreement) (.nounPhrase agreement), [Atom.word (pronounText agreement)],
   .word .pronoun ⟨agreement, rfl, rfl⟩, .word ⟨agreement, rfl, rfl, rfl⟩⟩

def attack : Witness Lexeme lexicon (.verbPhrase .plain) :=
  ⟨.node (.verb .attack .plain []) [], ["attack"], .verb ⟨rfl, rfl, rfl, rfl⟩ .nil,
   .node .nil (.verb (v := ["attack"]) ⟨rfl, rfl, rfl⟩)⟩

def mixed (left right : Agreement) (different : left ≠ right) :
    Witness Lexeme lexicon (.nounPhrase (left.additive right)) :=
  ⟨.node (.coordinate .and_ (.nounPhrase left) (.nounPhrase right))
     [(pronoun left).tree, (pronoun right).tree],
   (pronoun left).surface ++ (["and"] : Surface) ++ (pronoun right).surface,
   .node (.mixedAdditive different) (.cons (pronoun left).derives (.cons (pronoun right).derives
     .nil)),
   .node (.cons (pronoun left).realizes (.cons (pronoun right).realizes .nil)) .coordinate⟩

def mixedClause (left right : Agreement) (different : left ≠ right) :
    Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node (.finite (left.additive right) .plain) [(mixed left right different).tree, attack.tree],
   (mixed left right different).surface ++ attack.surface,
   .finite (mixed left right different).derives attack.derives
     (.verb ⟨rfl, rfl, by simp [Agreement.additive, Agreement.concord]⟩) rfl,
   .node (.cons (mixed left right different).realizes (.cons attack.realizes .nil)) .finite⟩

theorem mixed_number_clause : Written lexicon
    (mixedClause ⟨.third, .singular⟩ ⟨.third, .plural⟩ (by decide)).tree
    (.clause .finite) "it and they attack" :=
  ⟨_, ⟨(mixedClause _ _ _).derives, (mixedClause _ _ _).realizes⟩,
    .cons (.cons (.cons .single))⟩

theorem mixed_person_clause : Written lexicon
    (mixedClause ⟨.second, .singular⟩ ⟨.third, .singular⟩ (by decide)).tree
    (.clause .finite) "you and it attack" :=
  ⟨_, ⟨(mixedClause _ _ _).derives, (mixedClause _ _ _).realizes⟩,
    .cons (.cons (.cons .single))⟩

theorem speaker_person_retained :
    (Agreement.additive ⟨.third, .plural⟩ ⟨.first, .singular⟩) = ⟨.first, .plural⟩ := rfl

theorem addressee_person_retained :
    (Agreement.additive ⟨.second, .singular⟩ ⟨.third, .plural⟩) = ⟨.second, .plural⟩ := rfl

theorem additive_singular_rejected :
    ¬ Production lexicon
      (.coordinate .and_ (.nounPhrase ⟨.third, .singular⟩) (.nounPhrase ⟨.third, .plural⟩))
      [.nounPhrase ⟨.third, .singular⟩, .nounPhrase ⟨.third, .plural⟩]
      (.nounPhrase ⟨.third, .singular⟩) := by
  intro h
  cases h

def alternative : Witness Lexeme lexicon (.nounPhrase ⟨.third, .plural⟩) :=
  ⟨.node (.coordinate .or_ (.nounPhrase ⟨.second, .singular⟩) (.nounPhrase ⟨.third, .plural⟩))
     [(pronoun ⟨.second, .singular⟩).tree, (pronoun ⟨.third, .plural⟩).tree],
   ["you", "or", "they"],
   .node (.mixedAlternative (by decide))
     (.cons (pronoun ⟨.second, .singular⟩).derives
       (.cons (pronoun ⟨.third, .plural⟩).derives .nil)),
   .node (.cons (pronoun ⟨.second, .singular⟩).realizes
     (.cons (pronoun ⟨.third, .plural⟩).realizes .nil)) .coordinate⟩

def alternativeClause : Witness Lexeme lexicon (.clause .finite) :=
  ⟨.node (.finite ⟨.third, .plural⟩ .plain) [alternative.tree, attack.tree],
   ["you", "or", "they", "attack"],
   .finite alternative.derives attack.derives (.verb ⟨rfl, rfl, rfl⟩) rfl,
   .node (.cons alternative.realizes (.cons attack.realizes .nil)) .finite⟩

theorem mixed_alternative_text : Written lexicon alternativeClause.tree
    (.clause .finite) "you or they attack" :=
  ⟨_, ⟨alternativeClause.derives, alternativeClause.realizes⟩,
    .cons (.cons (.cons .single))⟩

theorem proximity_depends_on_position :
    subjectAgreement .beforeVerb alternative.tree = some ⟨.third, .plural⟩ ∧
    subjectAgreement .afterVerb alternative.tree = some ⟨.second, .singular⟩ := ⟨rfl, rfl⟩

/-- The verb admits both feature combinations here; the clause must still choose the nearer one. -/
theorem wrong_proximity_rejected :
    FiniteLicense lexicon attack.tree ⟨.second, .singular⟩ ∧
    ¬ Derives lexicon
      (.node (.finite ⟨.second, .singular⟩ .plain) [alternative.tree, attack.tree])
      (.clause .finite) := by
  refine ⟨.verb ⟨rfl, rfl, rfl⟩, ?_⟩
  have impossible {gaps : List Category}
      (h : Judges lexicon
        (.node (.finite ⟨.second, .singular⟩ .plain) [alternative.tree, attack.tree])
        (.clause .finite) gaps) : False := by
    cases h with
    | node production _ => cases production
    | finite _ _ _ agreement => cases agreement
  exact impossible

end English.AgreementInteractions
