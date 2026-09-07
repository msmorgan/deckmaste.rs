import English.Dependencies

/-! Declared lexical alternatives, independent of the surrounding construction.
The generic grammar is instantiated at Word Forms, so all parent checks see the same bundle. -/
namespace English

inductive Tense where
  | present | past
  deriving DecidableEq

inductive Capitalization where
  | declared | initial
  deriving DecidableEq

def Capitalization.apply : Capitalization → Surface → Surface
  | .declared, surface => surface
  | .initial, surface => surface.capitalize

/-- Inapplicable features have no field. Finite agreement and tense are one correlated use. -/
inductive FeatureBundle where
  | noun (number : Number) (countability : Features.Countability)
  | verb (form : InflectionalForm) (finite : Option (Tense × Agreement))
  | adjective
  | determinative (number : Number) (countability : Features.Countability)
  | word (category : Category) (grammaticalCase : Case := .nominativeOrAccusative)
  | identity (category : Category) (grammaticalCase : Case := .nominativeOrAccusative)
  | attributive | targeting
  | preposition (complement : Category)
  | subordinator (finiteness : Finiteness)
  | comparison | measure | fixed
  | keyword (parameter : Option Category) (placement : KeywordPlacement)
  deriving DecidableEq

/-- A default is used once per licensed bundle; an override replaces it, including alternatives. -/
structure Morphology where
  defaultForm : FeatureBundle → Surface
  overrides : FeatureBundle → Option (List Surface) := fun _ ↦ none

def Morphology.forms (morphology : Morphology) (bundle : FeatureBundle) : List Surface :=
  (morphology.overrides bundle).getD [morphology.defaultForm bundle]

theorem Morphology.override_replaces (m : Morphology) (b : FeatureBundle) (forms : List Surface)
    (override : m.overrides b = some forms) : m.forms b = forms := by
  simp [Morphology.forms, override]

/-- Frames and lexical properties belong to declarations, never to surface-sensitive guards. -/
structure LexemeDeclaration (L : Type) where
  lemma : Surface
  provenance : String
  bundles : List FeatureBundle
  morphology : Morphology
  verbFrame : InflectionalForm → Voice → List (FrameItem L) → Prop := fun _ _ _ ↦ False
  auxiliary : InflectionalForm → InflectionalForm → Voice → Voice → Prop := fun _ _ _ _ ↦ False
  temporal : Prop := False
  relativePronoun : Prop := False
  relativeDeterminer : Prop := False

abbrev LexicalEnvironment (L : Type) := L → Option (LexemeDeclaration L)

/-- Spelling and case choices are grammatical value data; locations are not lexical identity. -/
structure WordForm (L : Type) where
  lexeme : L
  bundle : FeatureBundle
  spelling : Surface
  capitalization : Capitalization := .declared
  provenance : String

def WordForm.surface {L : Type} (word : WordForm L) : Surface :=
  word.capitalization.apply word.spelling

/-- There is no category, frame or parse context input to lexical analysis. -/
def LexicalAnalysis {L : Type} (environment : LexicalEnvironment L)
    (surface : Surface) (word : WordForm L) : Prop :=
  ∃ declaration, environment word.lexeme = some declaration ∧
    word.provenance = declaration.provenance ∧ word.bundle ∈ declaration.bundles ∧
    word.spelling ∈ declaration.morphology.forms word.bundle ∧
    (word.capitalization = .declared ∨ word.spelling.capitalize ≠ word.spelling) ∧
    surface = word.surface

def WordForm.Licensed {L : Type} (environment : LexicalEnvironment L) (word : WordForm L) : Prop :=
  LexicalAnalysis environment word.surface word

def WordForm.HasDeclaration {L : Type} (environment : LexicalEnvironment L) (word : WordForm L)
    (property : LexemeDeclaration L → Prop) : Prop :=
  ∃ declaration, environment word.lexeme = some declaration ∧ property declaration

/-- Erase only the chosen marker spelling when checking the declaration's lexical frame identity. -/
def FrameItem.lexemes {L : Type} : FrameItem (WordForm L) → FrameItem L
  | .argument complement => .argument complement
  | .fixed marker => .fixed marker.lexeme
  | .marked marker complement => .marked marker.lexeme complement

namespace Lexical
variable {L : Type}

/-- Every lexical capability requires a whole licensed alternative, not independent projections. -/
def lexicon (environment : LexicalEnvironment L) : Lexicon (WordForm L) where
  noun w n := w.Licensed environment ∧ ∃ use, w.bundle = .noun n use
  adjective w := w.Licensed environment ∧ w.bundle = .adjective
  nounForm w n s := w.Licensed environment ∧ (∃ use, w.bundle = .noun n use) ∧ s = w.surface
  adjectiveForm w s := w.Licensed environment ∧ w.bundle = .adjective ∧ s = w.surface
  word w c := w.Licensed environment ∧
    ((∃ k, w.bundle = .word c k) ∨
      ∃ n use, w.bundle = .determinative n use ∧ c = .determinativePhrase n)
  wordForm w c s := w.Licensed environment ∧
    ((∃ k, w.bundle = .word c k) ∨
      ∃ n use, w.bundle = .determinative n use ∧ c = .determinativePhrase n) ∧ s = w.surface
  identity w c := w.Licensed environment ∧ ∃ k, w.bundle = .identity c k
  identityForm w c s := w.Licensed environment ∧ (∃ k, w.bundle = .identity c k) ∧ s = w.surface
  verb w f v frame := w.Licensed environment ∧ (∃ finite, w.bundle = .verb f finite) ∧
    w.HasDeclaration environment (fun d ↦ d.verbFrame f v (frame.map FrameItem.lexemes))
  verbForm w f s := w.Licensed environment ∧ (∃ finite, w.bundle = .verb f finite) ∧ s = w.surface
  finite w agreement f := w.Licensed environment ∧ ∃ tense,
    w.bundle = .verb f (some (tense, agreement))
  auxiliary w f selected v selectedVoice := w.Licensed environment ∧
    (∃ finite, w.bundle = .verb f finite) ∧
    w.HasDeclaration environment (fun d ↦ d.auxiliary f selected v selectedVoice)
  attributive w := w.Licensed environment ∧ w.bundle = .attributive
  targeting w := w.Licensed environment ∧ w.bundle = .targeting
  preposition w c := w.Licensed environment ∧ w.bundle = .preposition c
  subordinator w f := w.Licensed environment ∧ w.bundle = .subordinator f
  comparison w := w.Licensed environment ∧ w.bundle = .comparison
  measure w := w.Licensed environment ∧ w.bundle = .measure
  markerForm w s := w.Licensed environment ∧ s = w.surface
  keyword w p placement := w.Licensed environment ∧ w.bundle = .keyword p placement
  keywordForm w s := w.Licensed environment ∧ s = w.surface
  keywordSuffix w suffix := w.Licensed environment ∧ w.surface = [.closing suffix]

def features (environment : LexicalEnvironment L) : Features.Declarations (WordForm L) where
  nounUse w use := ∃ n, w.bundle = .noun n use
  determinerUse w use := ∃ n, w.bundle = .determinative n use
  temporalNoun w := w.HasDeclaration environment (·.temporal)
  nominalCase w := match w.bundle with
    | .word _ k | .identity _ k => k
    | _ => .nominativeOrAccusative

def dependencies (environment : LexicalEnvironment L) : Dependencies.Declarations (WordForm L) where
  relativePronoun w := w.HasDeclaration environment (·.relativePronoun)
  relativeDeterminer w := w.HasDeclaration environment (·.relativeDeterminer)

theorem unknown_excluded (environment : LexicalEnvironment L) (word : WordForm L)
    (unknown : environment word.lexeme = none) (surface : Surface) :
    ¬ LexicalAnalysis environment surface word := by
  rintro ⟨declaration, declared, _⟩
  rw [unknown] at declared
  cases declared

theorem unlicensed_bundle_excluded (environment : LexicalEnvironment L) (word : WordForm L)
    (declaration : LexemeDeclaration L) (declared : environment word.lexeme = some declaration)
    (absent : word.bundle ∉ declaration.bundles) (surface : Surface) :
    ¬ LexicalAnalysis environment surface word := by
  rintro ⟨other, found, _, bundle, _⟩
  rw [declared] at found
  cases found
  exact absent bundle

theorem finite_uses_selected_bundle (environment : LexicalEnvironment L) (word : WordForm L)
    (agreement : Agreement) (form : InflectionalForm)
    (admitted : (lexicon environment).finite word agreement form) :
    ∃ tense, word.bundle = .verb form (some (tense, agreement)) := admitted.2

end Lexical
end English
