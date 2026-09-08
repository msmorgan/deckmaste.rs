import English.Documents
import English.DependencyInteractions
import English.Readings

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

/-! Ordinary-word payload boundaries. The condition these three theorems test is
`English.Atom.WellFormed`, a side condition of `LexicalAnalysis` (`English/Lexical.lean`), and no
longer a standalone predicate that nothing consults. Same asserted outcomes as the retired
`wordPayload` trio: a two-word string is not one word atom, a lexical apostrophe is, a newline is
not. -/

theorem source_space_is_not_a_word : ¬ Atom.WellFormed (.word "two words") := by
  intro h
  have := (h.2 ' ' (by decide)).1
  contradiction

theorem lexical_apostrophe_allowed : Atom.WellFormed (.word "player's") := by decide

theorem source_newline_is_not_a_word : ¬ Atom.WellFormed (.word "two\nwords") := by
  intro h
  have := (h.2 '\n' (by decide)).1
  contradiction

/-- A quote delimiter is the surface's own bracketing atom, never a word's payload. -/
theorem quote_delimiter_is_not_a_word : ¬ Atom.WellFormed (.word "\"quoted\"") := by
  intro h
  have := (h.2 '"' (by decide)).2
  contradiction

end English.BoundaryInteractions

/-! The payload condition reaches the grammar: a declared spelling that owns a source space is not
licensed, and the same declaration's single-word spelling is. The contrast is exhibited against
`AnalysisWithoutPayload`, which is `LexicalAnalysis` with exactly that one conjunct dropped. -/
namespace English.BoundaryInteractions.Payload

/-- `LexicalAnalysis` with the payload side condition dropped — the weakened premise. Every other
conjunct is `LexicalAnalysis`'s, verbatim. -/
def AnalysisWithoutPayload {L : Type} (environment : LexicalEnvironment L)
    (surface : Surface) (word : WordForm L) : Prop :=
  ∃ declaration, environment word.lexeme = some declaration ∧
    word.provenance = declaration.provenance ∧ word.bundle ∈ declaration.bundles ∧
    word.spelling ∈ declaration.morphology.forms word.bundle ∧
    (word.capitalization = .declared ∨ word.spelling.capitalize ≠ word.spelling) ∧
    surface = word.surface

/-- One declaration, one bundle, two declared spellings: a single word and a spelling that owns a
source space. Nothing but the payload distinguishes the two alternatives. -/
def declaration : LexemeDeclaration Unit where
  lemma := ["counters"]
  provenance := "synthetic-payload"
  bundles := [.noun .plural .count]
  morphology := {
    defaultForm := fun _ ↦ ["counters"]
    overrides := fun _ ↦ some [["counters"], [.word "two words"]] }

def environment : LexicalEnvironment Unit := fun _ ↦ some declaration

def word (spaced : Bool) : WordForm Unit :=
  ⟨(), .noun .plural .count, if spaced then [.word "two words"] else ["counters"], .declared,
    "synthetic-payload"⟩

def leaf (spaced : Bool) : Reading Unit := .noun (word spaced) .plural

/-- Weakened premise: with the payload conjunct dropped, both alternatives analyse. -/
theorem both_analysed_without_payload (spaced : Bool) :
    AnalysisWithoutPayload environment (word spaced).surface (word spaced) := by
  cases spaced <;>
    exact ⟨declaration, rfl, rfl, by decide, by decide, Or.inl rfl, rfl⟩

theorem plain_licensed : (word false).Licensed environment :=
  ⟨declaration, rfl, rfl, by decide, by decide, Or.inl rfl, rfl, by decide⟩

/-- Restored premise: the spaced alternative is not licensed, and the payload conjunct is the only
thing that changed. -/
theorem spaced_not_licensed : ¬ (word true).Licensed environment := by
  rintro ⟨_, _, _, _, _, _, _, payload⟩
  exact (source_space_is_not_a_word (payload (.word "two words") (by decide))).elim

theorem plain_admitted :
    Reading.Admitted environment [] (leaf false) (.nominal .plural) ["counters"] := by
  refine ⟨⟨.noun ⟨plain_licensed, .count, rfl⟩, ?_, ?_, ?_⟩,
    .noun ⟨plain_licensed, ⟨.count, rfl⟩, rfl⟩⟩
  · simp [leaf, Features.Conforms, Features.Local]
  · simp [leaf, Dependencies.Safe, Dependencies.Local]
  · simp [leaf, word, Reading.GrammarConforms, Reading.LocalGrammar]

theorem spaced_not_admitted (surface : Surface) :
    ¬ Reading.Admitted environment [] (leaf true) (.nominal .plural) surface := by
  rintro ⟨⟨derives, _⟩, _⟩
  cases derives with
  | noun capable => exact spaced_not_licensed capable.1

end English.BoundaryInteractions.Payload

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
      (.argument (slot := ⟨.object,.nounPhrase plural⟩) npD .nil)
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
