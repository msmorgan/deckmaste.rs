import English.SurfaceRelations

/-! Admission is independent of preference, and grammatical identity is independent of derivations. -/
namespace English

/-- The tree retains one correlated lexical alternative at each occurrence. -/
abbrev Reading (L : Type) := Syntax (WordForm L)

namespace Reading
variable {L : Type}

/-- The inflection selected by an auxiliary or nonfinite host must have a nonfinite lexical use. -/
inductive NonfiniteUse : Reading L → Prop where
  | verb {word : WordForm L} {form : InflectionalForm} {voice : Voice}
      {frame : List (FrameItem (WordForm L))} {children : List (Reading L)} :
      word.bundle = .verb form none → NonfiniteUse (.node (.verb word form frame voice) children)
  | auxiliary {word : WordForm L} {form selected : InflectionalForm} {voice selectedVoice : Voice}
      {polarity : Polarity} {child : Reading L} : word.bundle = .verb form none →
      NonfiniteUse (.node (.auxiliary word form selected polarity voice selectedVoice) [child])
  | adjunct {host dependent : Category} {placement : Placement} {head child : Reading L} :
      NonfiniteUse head → NonfiniteUse (.node (.adjunct host dependent placement) [head, child])
  | coordinate {coordinator : Coordinator} {category : Category} {left right : Reading L} :
      NonfiniteUse left → NonfiniteUse right →
      NonfiniteUse (.node (.coordinate coordinator category) [left, right])
  | ellipsis {form : InflectionalForm} {voice : Voice} : NonfiniteUse (.ellipsis form voice)

/-- Tense is available to parents without confusing it with finiteness or morphological form. -/
inductive HasTense : Reading L → Tense → Prop where
  | verb {word : WordForm L} {form : InflectionalForm} {voice : Voice} {tense : Tense}
      {agreement : Agreement} {frame : List (FrameItem (WordForm L))} {children : List (Reading L)} :
      word.bundle = .verb form (some (tense, agreement)) →
      HasTense (.node (.verb word form frame voice) children) tense
  | auxiliary {word : WordForm L} {form selected : InflectionalForm} {voice selectedVoice : Voice}
      {polarity : Polarity} {child : Reading L} {tense : Tense} {agreement : Agreement} :
      word.bundle = .verb form (some (tense, agreement)) →
      HasTense (.node (.auxiliary word form selected polarity voice selectedVoice) [child]) tense
  | adjunct {host dependent : Category} {placement : Placement} {head child : Reading L}
      {tense : Tense} : HasTense head tense →
      HasTense (.node (.adjunct host dependent placement) [head, child]) tense
  | clause {agreement : Agreement} {form : InflectionalForm} {voice : Voice}
      {subject predicate : Reading L} {tense : Tense} : HasTense predicate tense →
      HasTense (.node (.finite agreement form voice) [subject, predicate]) tense

mutual
  /-- Count syntactic gap occurrences before shared discharge, retaining multiplicity. -/
  def gapOccurrences : Reading L → Nat
    | .gap _ => 1
    | .node _ children => childGaps children
    | .modify a b | .sharedCoordination _ _ a b => gapOccurrences a + gapOccurrences b
    | .relativeForm _ head _ _ _ => gapOccurrences head
    | .frameCoordination _ a b => childGaps a + childGaps b
    | _ => 0
  def childGaps : List (Reading L) → Nat
    | [] => 0
    | first :: rest => gapOccurrences first + childGaps rest
end

/-- Additional structural constraints on shared gaps and nonfinite complements. -/
def LocalGrammar : Reading L → Prop
  | .node (.rightNodeRaising result _) [body, _] =>
      coordinable result = true ∧ 2 ≤ gapOccurrences body
  | .node (.auxiliary _ _ _ _ _ _) [child] => NonfiniteUse child
  | .node .imperative [child] | .node (.nonfinite _ _) [child] => NonfiniteUse child
  | _ => True

mutual
  def GrammarConforms (tree : Reading L) : Prop :=
    LocalGrammar tree ∧ match tree with
      | .node _ children => ChildrenConform children
      | .modify a b | .sharedCoordination _ _ a b => GrammarConforms a ∧ GrammarConforms b
      | .relativeForm _ head body _ front =>
          GrammarConforms head ∧ GrammarConforms body ∧ ChildrenConform front
      | .frameCoordination _ a b => ChildrenConform a ∧ ChildrenConform b
      | _ => True
  def ChildrenConform : List (Reading L) → Prop
    | [] => True
    | first :: rest => GrammarConforms first ∧ ChildrenConform rest
end

/-- Checked construction needs no source text and does not invoke an analysis procedure. -/
def Valid (environment : LexicalEnvironment L) (context : List Category)
    (tree : Reading L) (category : Category) : Prop :=
  DerivesIn (Lexical.lexicon environment) context tree category ∧
    Features.Conforms (Lexical.features environment) tree ∧
    Dependencies.Safe (Lexical.dependencies environment) tree ∧ GrammarConforms tree

/-- Relational analysis combines independent lexical recognition with grammatical admission. -/
def Admitted (environment : LexicalEnvironment L) (context : List Category)
    (tree : Reading L) (category : Category) (surface : Surface) : Prop :=
  Valid environment context tree category ∧ Realizes (Lexical.lexicon environment) tree surface

/-- A set of values, not a list of proof paths or a quotient by attachment/preference. -/
def readings (environment : LexicalEnvironment L) (context : List Category)
    (category : Category) (surface : Surface) : Reading L → Prop :=
  fun tree ↦ Admitted environment context tree category surface

def Ambiguous (environment : LexicalEnvironment L) (context : List Category)
    (category : Category) (surface : Surface) : Prop :=
  ∃ a b, readings environment context category surface a ∧
    readings environment context category surface b ∧ a ≠ b

/-- Preference annotates admitted alternatives. It cannot establish or revoke admission. -/
structure Preference (environment : LexicalEnvironment L) (context : List Category)
    (category : Category) (surface : Surface) where
  prefers : Reading L → Reading L → Prop
  admitted : ∀ {a b}, prefers a b →
    readings environment context category surface a ∧ readings environment context category surface b

theorem preference_retains_both {environment : LexicalEnvironment L} {context : List Category}
    {category : Category} {surface : Surface} (preference : Preference environment context category surface)
    {a b : Reading L} (preferred : preference.prefers a b) :
    readings environment context category surface a ∧ readings environment context category surface b :=
  preference.admitted preferred

theorem duplicate_derivations {environment : LexicalEnvironment L} {context : List Category}
    {category : Category} {surface : Surface} {tree : Reading L}
    (first second : Admitted environment context tree category surface) :
    (⟨tree, first⟩ : {t // Admitted environment context t category surface}) = ⟨tree, second⟩ := rfl

/-- Obligation for an independently specified realization operation; no implementation is assumed. -/
def AnalysisRoundtrip (environment : LexicalEnvironment L) (render : Reading L → Option Surface) : Prop :=
  ∀ context tree category surface, Admitted environment context tree category surface →
    render tree = some surface

/-- Independent construction must be grammatically sound and preserve the exact value on realization.
`constructs` describes source-independent construction; it must not be defined by parsing output.
These are obligations on supplied operations, not definitions of a parser or renderer. -/
def ValueRoundtrip (environment : LexicalEnvironment L)
    (constructs : List Category → Reading L → Category → Prop)
    (render : Reading L → Option Surface) : Prop :=
  ∀ context tree category, constructs context tree category →
    Valid environment context tree category ∧
      ∃ surface, render tree = some surface ∧ readings environment context category surface tree

end Reading
end English
