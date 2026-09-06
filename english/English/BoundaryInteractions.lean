import English.Documents
import English.DependencyInteractions

namespace English.BoundaryInteractions
open Documents

def semicolonLine := binary ward landwalk (.keywordSeparated (n := 0)) .keywordSeparated

theorem semicolon_text : Written lexicon semicolonLine.tree (.document .ability)
    "Ward {2}; landwalk" :=
  ⟨_,⟨semicolonLine.derives,semicolonLine.realizes⟩,
    .cons (.cons (.cons (.cons .single)))⟩

theorem singleton_has_no_separator :
    ¬ DocumentProduction (.keywordLine .semicolon) [.keywordPhrase] (.document .ability) := by
  intro h
  cases h

/-- Lexical words own internal lexical characters, but no source whitespace or quote delimiters. -/
def wordPayload (text : String) : Prop :=
  text ≠ "" ∧ ∀ c ∈ text.toList, c.isWhitespace = false ∧ c ≠ '"'

theorem source_space_is_not_a_word : ¬ wordPayload "two words" := by
  intro h
  have := (h.2 ' ' (by decide)).1
  contradiction

theorem lexical_apostrophe_allowed : wordPayload "player's" := by unfold wordPayload; decide

theorem source_newline_is_not_a_word : ¬ wordPayload "two\nwords" := by
  intro h
  have := (h.2 '\n' (by decide)).1
  contradiction

end English.BoundaryInteractions

namespace English.BoundaryInteractions.Supplements
open DependencyInteractions

def nominal : Syntax Lexeme := .relative .plural creatures objectBody .supplementary [which]
def np : Syntax Lexeme := .node .barePlural [nominal]
def predicate : Syntax Lexeme := .node (.verb .control .plain [object]) [np]
def clause : Syntax Lexeme := .node (.finite second .plain) [you,predicate]
def sentence : Syntax Lexeme := .node (.document .sentence) [clause]

theorem at_sentence_end : Written lexicon sentence (.document .sentence)
    "You control creatures, which you control." := by
  have nominalD : Derives lexicon nominal (.nominal .plural) :=
    .frontedRelative (Or.inr rfl) (.noun (Or.inl ⟨Or.inl rfl,rfl⟩))
      (.word .pronoun (Or.inr (Or.inl ⟨rfl,rfl⟩))) object_body
  have npD : Derives lexicon np (.nounPhrase plural) := .node .barePlural (.cons nominalD .nil)
  have predicateD : Derives lexicon predicate (.verbPhrase .plain) :=
    .verb ⟨rfl,rfl,Or.inl ⟨rfl,rfl⟩⟩
      (.argument (complement := ⟨.object,.nounPhrase plural⟩) npD .nil)
  have clauseD : Derives lexicon clause (.clause .finite) :=
    .finite (.word .pronoun (Or.inl ⟨rfl,rfl⟩)) predicateD
      (.verb ⟨rfl,Or.inr ⟨Or.inl rfl,rfl⟩⟩) rfl
  have nominalR : Realizes lexicon nominal
      ["creatures",.closing ",","which","you","control",.closing ","] :=
    .supplementaryRelative (.noun (surface := ["creatures"]) (Or.inl ⟨rfl,rfl,rfl⟩))
      (.word (surface := ["which"]) (Or.inr (Or.inl ⟨rfl,rfl,rfl⟩))) object_form
  have npR := Realizes.node (.cons nominalR .nil) Linearizes.barePlural
  have predicateR : Realizes lexicon predicate
      ["control","creatures",.closing ",","which","you","control",.closing ","] :=
    .node (.cons npR .nil) (.verb (v := ["control"]) ⟨rfl,Or.inl ⟨rfl,rfl⟩⟩)
  have clauseR : Realizes lexicon clause
      ["you","control","creatures",.closing ",","which","you","control",.closing ","] :=
    .node (.cons (.word (Or.inl ⟨rfl,rfl,rfl⟩)) (.cons predicateR .nil)) .finite
  exact ⟨_,⟨.node (.document .sentence) (.cons clauseD .nil),
    .node (.cons clauseR .nil) (.document .sentence)⟩,
    .cons (.cons (.cons (.cons (.cons (.cons (.cons .single))))))⟩

end English.BoundaryInteractions.Supplements
