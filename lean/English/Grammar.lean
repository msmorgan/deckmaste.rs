import Init

/-!
The initial Oracle English grammar fragment. See `docs/english-grammar-design.md`
for intended scope and the decisions left to composition and document modeling.
-/

namespace English

inductive Number where
  | singular
  | plural
  deriving DecidableEq

/-- Categories implemented in this fragment, not the whole intended grammar. -/
inductive Category where
  | nominal (number : Number)
  | adjectivePhrase
  deriving DecidableEq

/-- Ordered surface atoms; this does not yet specify spelling or byte boundaries. -/
abbrev Surface := List String

/-- Lexical licensing and word forms are separate assumptions of the model. -/
structure Lexicon (Lexeme : Type) where
  noun : Lexeme → Number → Prop
  adjective : Lexeme → Prop
  nounForm : Lexeme → Number → Surface → Prop
  adjectiveForm : Lexeme → Surface → Prop

/-- Candidate structures; a value is grammatical only with a derivation. -/
inductive Syntax (Lexeme : Type) where
  | noun (lexeme : Lexeme) (number : Number)
  | adjective (lexeme : Lexeme)
  | modify (modifier head : Syntax Lexeme)
  | coordinate (left right : Syntax Lexeme)

/-- Structural licensing, independent of surface forms and candidate selection. -/
inductive Derives {Lexeme : Type} (lexicon : Lexicon Lexeme) :
    Syntax Lexeme → Category → Prop where
  | noun {lexeme : Lexeme} {number : Number} :
      lexicon.noun lexeme number → Derives lexicon (.noun lexeme number) (.nominal number)
  | adjective {lexeme : Lexeme} :
      lexicon.adjective lexeme → Derives lexicon (.adjective lexeme) .adjectivePhrase
  | modify {modifier head : Syntax Lexeme} {number : Number} :
      Derives lexicon modifier .adjectivePhrase →
      Derives lexicon head (.nominal number) →
      Derives lexicon (.modify modifier head) (.nominal number)
  | coordinate {left right : Syntax Lexeme} :
      Derives lexicon left (.nominal .plural) →
      Derives lexicon right (.nominal .plural) →
      Derives lexicon (.coordinate left right) (.nominal .plural)

/-- Relational linearization; grammatical licensing is required separately. -/
inductive Realizes {Lexeme : Type} (lexicon : Lexicon Lexeme) :
    Syntax Lexeme → Surface → Prop where
  | noun {lexeme : Lexeme} {number : Number} {surface : Surface} :
      lexicon.nounForm lexeme number surface →
      Realizes lexicon (.noun lexeme number) surface
  | adjective {lexeme : Lexeme} {surface : Surface} :
      lexicon.adjectiveForm lexeme surface → Realizes lexicon (.adjective lexeme) surface
  | modify {modifier head : Syntax Lexeme} {modifierSurface headSurface : Surface} :
      Realizes lexicon modifier modifierSurface →
      Realizes lexicon head headSurface →
      Realizes lexicon (.modify modifier head) (modifierSurface ++ headSurface)
  | coordinate {left right : Syntax Lexeme} {leftSurface rightSurface : Surface} :
      Realizes lexicon left leftSurface →
      Realizes lexicon right rightSurface →
      Realizes lexicon (.coordinate left right) (leftSurface ++ ["and"] ++ rightSurface)

/-- A grammatical analysis with its surface, without any preference or packing policy. -/
def Admissible {Lexeme : Type} (lexicon : Lexicon Lexeme) (tree : Syntax Lexeme)
    (category : Category) (surface : Surface) : Prop :=
  Derives lexicon tree category ∧ Realizes lexicon tree surface

end English
