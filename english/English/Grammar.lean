import English.Surface
import English.DocumentShape

/-! Oracle English composition; assumptions and scope are in `docs/english-grammar-design.md`. -/

namespace English

inductive Number where
  | singular | plural
  deriving DecidableEq

inductive Person where
  | first | second | third
  deriving DecidableEq

structure Agreement where
  person : Person
  number : Number
  deriving DecidableEq

inductive ConcordClass where
  | thirdSingular | other
  deriving DecidableEq

def Agreement.concord : Agreement → ConcordClass
  | ⟨.third, .singular⟩ => .thirdSingular
  | _ => .other

inductive InflectionalForm where
  | plain | thirdSingularPresent | nonThirdSingularPresent | firstSingularPresent
  | preterite | gerundParticiple | pastParticiple
  deriving DecidableEq

inductive Voice where
  | active | passive
  deriving DecidableEq

inductive Placement where
  | before | after
  deriving DecidableEq

inductive Polarity where
  | positive | negative
  deriving DecidableEq

inductive Finiteness where
  | finite | nonfinite
  deriving DecidableEq

/-- Regular finite inflection; irregular and invariant heads declare their own licensing. -/
inductive FiniteForm : Agreement → InflectionalForm → Prop where
  | singular {agreement : Agreement} : agreement.concord = .thirdSingular →
      FiniteForm agreement .thirdSingularPresent
  | other {agreement : Agreement} : agreement.concord = .other → FiniteForm agreement .plain
  | past {agreement : Agreement} : FiniteForm agreement .preterite

inductive Category where
  | nominal (number : Number)
  | nounPhrase (agreement : Agreement)
  | determinativePhrase (number : Number)
  | adjectivePhrase | adverbPhrase | prepositionPhrase
  | cardinalNumeral (number : Number)
  | measurePhrase
  | verbPhrase (form : InflectionalForm) (voice : Voice := .active)
  | clause (finiteness : Finiteness)
  | subordinateClause (finiteness : Finiteness)
  | document (category : DocumentCategory)
  deriving DecidableEq

inductive Relation where
  | subject | object | complement
  deriving DecidableEq

/-- A relation is attached to a selected category, not used as a category itself. -/
structure Complement where
  relation : Relation
  category : Category
  deriving DecidableEq

inductive FrameItem (Lexeme : Type) where
  | argument (complement : Complement)
  | fixed (marker : Lexeme)

inductive Coordinator where
  | and_ | or_ | andOr
  deriving DecidableEq

def Coordinator.surface : Coordinator → Surface
  | .and_ => (["and"] : Surface)
  | .or_ => (["or"] : Surface)
  | .andOr => (["and/or"] : Surface)

/-- Local productions. Recursive arguments live in Syntax, never in lexical frame schemas. -/
inductive Construction (Lexeme : Type) where
  | document (rule : DocumentRule)
  | keyword (head : Lexeme) (parameter : Option Category) (placement : KeywordPlacement)
  | determine (number : Number)
  | barePlural
  | quantify (number : Number)
  | compare (marker : Lexeme)
  | measure (marker : Lexeme)
  | preposition (head : Lexeme) (complement : Category)
  | verb (head : Lexeme) (form : InflectionalForm) (frame : List (FrameItem Lexeme))
      (voice : Voice := .active)
  | auxiliary (head : Lexeme) (form selected : InflectionalForm) (polarity : Polarity)
      (voice selectedVoice : Voice := .active)
  | finite (agreement : Agreement) (form : InflectionalForm) (voice : Voice := .active)
  | imperative
  | nonfinite (form : InflectionalForm) (voice : Voice := .active)
  | subordinate (marker : Lexeme) (finiteness : Finiteness)
  | adjunct (host dependent : Category) (placement : Placement := .after)
  | coordinate (coordinator : Coordinator) (category : Category)

/-- Lexical licensing and word forms remain independent assumptions. -/
structure Lexicon (Lexeme : Type) where
  noun : Lexeme → Number → Prop
  adjective : Lexeme → Prop
  nounForm : Lexeme → Number → Surface → Prop
  adjectiveForm : Lexeme → Surface → Prop
  keyword : Lexeme → Option Category → KeywordPlacement → Prop := fun _ _ _ => False
  keywordForm : Lexeme → Surface → Prop := fun _ _ => False
  keywordSuffix : Lexeme → String → Prop := fun _ _ => False
  word : Lexeme → Category → Prop := fun _ _ => False
  wordForm : Lexeme → Category → Surface → Prop := fun _ _ _ => False
  verb : Lexeme → InflectionalForm → Voice → List (FrameItem Lexeme) → Prop := fun _ _ _ _ => False
  verbForm : Lexeme → InflectionalForm → Surface → Prop := fun _ _ _ => False
  finite : Lexeme → Agreement → InflectionalForm → Prop := fun _ _ _ => False
  auxiliary : Lexeme → InflectionalForm → InflectionalForm → Voice → Voice → Prop :=
    fun _ _ _ _ _ => False
  preposition : Lexeme → Category → Prop := fun _ _ => False
  subordinator : Lexeme → Finiteness → Prop := fun _ _ => False
  comparison : Lexeme → Prop := fun _ => False
  measure : Lexeme → Prop := fun _ => False
  markerForm : Lexeme → Surface → Prop := fun _ _ => False

/-- Only these phrase projections can be supplied as lexical words in this fragment. -/
inductive WordCategory : Category → Prop where
  | pronoun {agreement : Agreement} : WordCategory (.nounPhrase agreement)
  | determinative {number : Number} : WordCategory (.determinativePhrase number)
  | adverb : WordCategory .adverbPhrase
  | quantity {number : Number} : WordCategory (.cardinalNumeral number)
  | measure : WordCategory .measurePhrase
  | symbol : WordCategory (.document .symbol)
  | label {kind : LabelKind} : WordCategory (.document (.label kind))
  | notation : WordCategory (.document .notation)
  | supertype : WordCategory (.document .supertype)
  | type : WordCategory (.document .type)
  | subtype : WordCategory (.document .subtype)

/-- Adjunct distribution is independent of lexical complement selection. -/
inductive AdjunctLicense : Category → Category → Placement → Prop where
  | nominal {number : Number} : AdjunctLicense (.nominal number) .prepositionPhrase .after
  | nounPhrase {agreement : Agreement} :
      AdjunctLicense (.nounPhrase agreement) .prepositionPhrase .after
  | verbal {form : InflectionalForm} {voice : Voice} :
      AdjunctLicense (.verbPhrase form voice) .prepositionPhrase .after
  | adverbial {form : InflectionalForm} {voice : Voice} :
      AdjunctLicense (.verbPhrase form voice) .adverbPhrase .after
  | clausal {finiteness : Finiteness} :
      AdjunctLicense (.clause finiteness) .prepositionPhrase .after
  | conditional {finiteness : Finiteness} :
      AdjunctLicense (.clause finiteness) (.subordinateClause .finite) .after
  | degree : AdjunctLicense .adjectivePhrase .adverbPhrase .before

def coordinationResult (coordinator : Coordinator) (category : Category) : Category :=
  match coordinator, category with
  | .and_, .nounPhrase agreement => .nounPhrase { agreement with number := .plural }
  | _, _ => category

/-- Document categories compose over the same clause and phrase categories. -/
inductive DocumentProduction : DocumentRule → List Category → Category → Prop where
  | sentence : DocumentProduction .sentence
      [.clause .finite] (.document .sentence)
  | body : DocumentProduction .body
      [.document .sentence] (.document .body)
  | bodyJoin : DocumentProduction .bodyJoin
      [.document .body, .document .sentence] (.document .body)
  | ordinary : DocumentProduction .ordinary
      [.document .body] (.document .ability)
  | document : DocumentProduction .document
      [.document .ability] (.document .document)
  | append : DocumentProduction .append
      [.document .document, .document .document] (.document .document)
  | costAction : DocumentProduction .costAction
      [.verbPhrase .plain] (.document .cost)
  | costSymbol : DocumentProduction .costSymbol
      [.document .symbol] (.document .cost)
  | costJoin : DocumentProduction .costJoin
      [.document .cost, .document .cost] (.document .cost)
  | activated : DocumentProduction .activated
      [.document .cost, .document .body] (.document .ability)
  | triggered : DocumentProduction .triggered
      [.subordinateClause .finite, .clause .finite] (.document .ability)
  | keywordLine : DocumentProduction .keywordLine
      [.document .keyword] (.document .ability)
  | keywordJoin : DocumentProduction .keywordJoin
      [.document .keyword, .document .keyword] (.document .keyword)
  | quote : DocumentProduction .quote
      [.document .document] (.document .quotedText)
  | mode : DocumentProduction .mode
      [.document .body] (.document .mode)
  | modeJoin : DocumentProduction .modeJoin
      [.document .mode, .document .mode] (.document .modes)
  | modes : DocumentProduction .modes
      [.clause .finite, .document .modes] (.document .ability)
  | weightedMode : DocumentProduction .weightedMode
      [.document .symbol, .document .body] (.document .mode)
  | label {kind : LabelKind} : DocumentProduction (.label kind)
      [.document (.label kind), .document .ability] (.document .ability)
  | chapter : DocumentProduction .chapter
      [.document .notation, .document .body] (.document .document)
  | classLevel : DocumentProduction .classLevel
      [.document .cost, .document .notation, .document .document] (.document .document)
  | levelBand : DocumentProduction .levelBand
      [.document .notation, .document .notation, .document .document] (.document .document)
  | solve : DocumentProduction .solve
      [.document .body] (.document .document)
  | solved : DocumentProduction .solved
      [.document .ability] (.document .document)
  | dieRow : DocumentProduction .dieRow
      [.document .notation, .document .body] (.document .document)
  | dieDashRow : DocumentProduction .dieDashRow
      [.document .notation, .document .body] (.document .document)
  | station : DocumentProduction .station
      [.document .notation, .document .ability] (.document .document)
  | supertype : DocumentProduction .supertype
      [.document .supertype] (.document .supertypes)
  | type : DocumentProduction .type
      [.document .type] (.document .types)
  | subtype : DocumentProduction .subtype
      [.document .subtype] (.document .subtypes)
  | noSupertypes : DocumentProduction .noSupertypes
      [] (.document .supertypes)
  | supertypes : DocumentProduction .supertypes
      [.document .supertypes, .document .supertype] (.document .supertypes)
  | types : DocumentProduction .types
      [.document .types, .document .type] (.document .types)
  | subtypes : DocumentProduction .subtypes
      [.document .subtypes, .document .subtype] (.document .subtypes)
  | typeLine : DocumentProduction .typeLine
      [.document .supertypes, .document .types] (.document .typeLine)
  | subtypedLine : DocumentProduction .subtypedLine
      [.document .supertypes, .document .types, .document .subtypes] (.document .typeLine)

/-- Ordered child categories are supplied by grammar rules, or by a declared lexical frame. -/
inductive Production {Lexeme : Type} (lexicon : Lexicon Lexeme) :
    Construction Lexeme → List Category → Category → Prop where
  | document {rule : DocumentRule} {children : List Category} {category : Category} :
      DocumentProduction rule children category →
      Production lexicon (.document rule) children category
  | keywordBare {head : Lexeme} : lexicon.keyword head none .free →
      Production lexicon (.keyword head none .free) [] (.document .keyword)
  | keywordParameter {head : Lexeme} {parameter : Category} {placement : KeywordPlacement} :
      lexicon.keyword head (some parameter) placement →
      Production lexicon (.keyword head (some parameter) placement) [parameter] (.document .keyword)
  | determine {number : Number} : Production lexicon (.determine number)
      [.determinativePhrase number, .nominal number] (.nounPhrase ⟨.third, number⟩)
  | barePlural : Production lexicon .barePlural [.nominal .plural]
      (.nounPhrase ⟨.third, .plural⟩)
  | quantify {number : Number} : Production lexicon (.quantify number)
      [.cardinalNumeral number] (.determinativePhrase number)
  | compare {marker : Lexeme} : lexicon.comparison marker →
      Production lexicon (.compare marker) [.measurePhrase] .adjectivePhrase
  | measure {marker : Lexeme} : lexicon.measure marker →
      Production lexicon (.measure marker) [.measurePhrase, .measurePhrase] .measurePhrase
  | preposition {head : Lexeme} {category : Category} : lexicon.preposition head category →
      Production lexicon (.preposition head category) [category] .prepositionPhrase
  | auxiliary {head : Lexeme} {form selected : InflectionalForm} {polarity : Polarity}
      {voice selectedVoice : Voice} :
      lexicon.auxiliary head form selected voice selectedVoice →
      Production lexicon (.auxiliary head form selected polarity voice selectedVoice)
        [.verbPhrase selected selectedVoice] (.verbPhrase form voice)
  | imperative : Production lexicon .imperative [.verbPhrase .plain] (.clause .finite)
  | nonfinite {form : InflectionalForm} {voice : Voice} :
      (form = .plain ∨ form = .gerundParticiple ∨ form = .pastParticiple) →
      Production lexicon (.nonfinite form voice) [.verbPhrase form voice] (.clause .nonfinite)
  | subordinate {marker : Lexeme} {finiteness : Finiteness} :
      lexicon.subordinator marker finiteness → Production lexicon (.subordinate marker finiteness)
        [.clause finiteness] (.subordinateClause finiteness)
  | adjunct {host dependent : Category} {placement : Placement} :
      AdjunctLicense host dependent placement →
      Production lexicon (.adjunct host dependent placement) [host, dependent] host
  | coordinate {coordinator : Coordinator} {category : Category} :
      Production lexicon (.coordinate coordinator category) [category, category]
        (coordinationResult coordinator category)

/-- One recursive tree for phrases, clauses, missing positions and their antecedents. -/
inductive Syntax (Lexeme : Type) where
  | noun (lexeme : Lexeme) (number : Number)
  | adjective (lexeme : Lexeme)
  | modify (modifier head : Syntax Lexeme)
  | marker (lexeme : Lexeme)
  | word (lexeme : Lexeme) (category : Category)
  | node (construction : Construction Lexeme) (children : List (Syntax Lexeme))
  | gap (category : Category)
  | sharedCoordination (coordinator : Coordinator) (category : Category)
      (left right : Syntax Lexeme)
  | relative (number : Number) (head body : Syntax Lexeme)
  | ellipsis (antecedent : Syntax Lexeme)

mutual
  /-- Structural reminder nesting check; lexical spellings are assumed to respect atom ownership. -/
  def Syntax.reminderFree {Lexeme : Type} : Syntax Lexeme → Bool
    | .node (.document .reminder) _ => false
    | .node _ children => reminderFreeChildren children
    | .modify left right | .relative _ left right | .sharedCoordination _ _ left right =>
        left.reminderFree && right.reminderFree
    | .ellipsis antecedent => antecedent.reminderFree
    | _ => true
  def reminderFreeChildren {Lexeme : Type} : List (Syntax Lexeme) → Bool
    | [] => true
    | first :: rest => first.reminderFree && reminderFreeChildren rest
end

/-- The initial plural nominal coordination uses the general coordination production. -/
def Syntax.coordinate {Lexeme : Type} (left right : Syntax Lexeme) : Syntax Lexeme :=
  .node (.coordinate .and_ (.nominal .plural)) [left, right]

/-- Finite agreement is licensed by the overt head, including irregular auxiliaries. -/
inductive FiniteLicense {Lexeme : Type} (lexicon : Lexicon Lexeme) :
    Syntax Lexeme → Agreement → Prop where
  | verb {head : Lexeme} {form : InflectionalForm} {frame : List (FrameItem Lexeme)} {voice : Voice}
      {children : List (Syntax Lexeme)} {agreement : Agreement} :
      lexicon.finite head agreement form →
      FiniteLicense lexicon (.node (.verb head form frame voice) children) agreement
  | auxiliary {head : Lexeme} {form selected : InflectionalForm} {polarity : Polarity}
      {voice selectedVoice : Voice} {child : Syntax Lexeme} {agreement : Agreement} :
      lexicon.finite head agreement form →
      FiniteLicense lexicon
        (.node (.auxiliary head form selected polarity voice selectedVoice) [child])
        agreement
  | adjunct {host dependent : Category} {placement : Placement} 
      {head child : Syntax Lexeme} {agreement : Agreement} :
      FiniteLicense lexicon head agreement →
      FiniteLicense lexicon (.node (.adjunct host dependent placement) [head, child]) agreement
  | coordinate {coordinator : Coordinator} {category : Category} {left right : Syntax Lexeme}
      {agreement : Agreement} : FiniteLicense lexicon left agreement →
      FiniteLicense lexicon right agreement →
      FiniteLicense lexicon (.node (.coordinate coordinator category) [left, right]) agreement
  | ellipsis {antecedent : Syntax Lexeme} {agreement : Agreement} :
      FiniteLicense lexicon antecedent agreement →
      FiniteLicense lexicon (.ellipsis antecedent) agreement

mutual
  /-- Gaps are ordered resources. Ordinary composition concatenates, never deletes them. -/
  inductive Judges {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      Syntax Lexeme → Category → List Category → Prop where
    | noun {lexeme : Lexeme} {number : Number} : lexicon.noun lexeme number →
        Judges lexicon (.noun lexeme number) (.nominal number) []
    | adjective {lexeme : Lexeme} : lexicon.adjective lexeme →
        Judges lexicon (.adjective lexeme) .adjectivePhrase []
    | modify {modifier head : Syntax Lexeme} {number : Number} {gaps : List Category} :
        Judges lexicon modifier .adjectivePhrase [] → Judges lexicon head (.nominal number) gaps →
        Judges lexicon (.modify modifier head) (.nominal number) gaps
    | word {lexeme : Lexeme} {category : Category} : WordCategory category →
        lexicon.word lexeme category → Judges lexicon (.word lexeme category) category []
    | node {construction : Construction Lexeme} {children : List (Syntax Lexeme)}
        {categories gaps : List Category} {category : Category} :
        Production lexicon construction categories category →
        JudgeChildren lexicon children categories gaps →
        Judges lexicon (.node construction children) category gaps
    | verb {head : Lexeme} {form : InflectionalForm}
        {frame : List (FrameItem Lexeme)} {voice : Voice}
        {children : List (Syntax Lexeme)} {gaps : List Category} :
        lexicon.verb head form voice frame → JudgeFrame lexicon children frame gaps →
        Judges lexicon (.node (.verb head form frame voice) children) (.verbPhrase form voice) gaps
    | finite {agreement : Agreement} {form : InflectionalForm} {voice : Voice}
        {subject predicate : Syntax Lexeme}
        {subjectGaps predicateGaps : List Category} :
        Judges lexicon subject (.nounPhrase agreement) subjectGaps →
        Judges lexicon predicate (.verbPhrase form voice) predicateGaps →
        FiniteLicense lexicon predicate agreement →
        Judges lexicon (.node (.finite agreement form voice) [subject, predicate]) (.clause .finite)
          (subjectGaps ++ predicateGaps)
    | reminder {body : Syntax Lexeme} :
        Judges lexicon body (.document .body) [] → body.reminderFree = true →
        Judges lexicon (.node (.document .reminder) [body]) (.document .ability) []
    | gap {category : Category} : Judges lexicon (.gap category) category [category]
    | sharedCoordination {coordinator : Coordinator} {category gap : Category}
        {left right : Syntax Lexeme} :
        Judges lexicon left category [gap] → Judges lexicon right category [gap] →
        Judges lexicon (.sharedCoordination coordinator category left right)
          (coordinationResult coordinator category) [gap]
    | relative {number : Number} {head body : Syntax Lexeme} :
        Judges lexicon head (.nominal number) [] →
        Judges lexicon body (.clause .finite) [.nounPhrase ⟨.third, number⟩] →
        Judges lexicon (.relative number head body) (.nominal number) []
    | ellipsis {antecedent : Syntax Lexeme} {form : InflectionalForm} {voice : Voice} :
        Judges lexicon antecedent (.verbPhrase form voice) [] →
        Judges lexicon (.ellipsis antecedent) (.verbPhrase form voice) []
  inductive JudgeChildren {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      List (Syntax Lexeme) → List Category → List Category → Prop where
    | nil : JudgeChildren lexicon [] [] []
    | cons {head : Syntax Lexeme} {tail : List (Syntax Lexeme)} {category : Category}
        {categories headGaps tailGaps : List Category} :
        Judges lexicon head category headGaps → JudgeChildren lexicon tail categories tailGaps →
        JudgeChildren lexicon (head :: tail) (category :: categories) (headGaps ++ tailGaps)
  /-- Fixed markers are matched to their declared frame positions, without spelling guards. -/
  inductive JudgeFrame {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      List (Syntax Lexeme) → List (FrameItem Lexeme) → List Category → Prop where
    | nil : JudgeFrame lexicon [] [] []
    | argument {head : Syntax Lexeme} {tail : List (Syntax Lexeme)} {complement : Complement}
        {frame : List (FrameItem Lexeme)} {headGaps tailGaps : List Category} :
        Judges lexicon head complement.category headGaps → JudgeFrame lexicon tail frame tailGaps →
        JudgeFrame lexicon (head :: tail) (.argument complement :: frame) (headGaps ++ tailGaps)
    | fixed {marker : Lexeme} {tail : List (Syntax Lexeme)} {frame : List (FrameItem Lexeme)}
        {gaps : List Category} : JudgeFrame lexicon tail frame gaps →
        JudgeFrame lexicon (.marker marker :: tail) (.fixed marker :: frame) gaps

end

/-- Closed grammatical derivation; independent of realization and selection. -/
abbrev Derives {Lexeme : Type} (lexicon : Lexicon Lexeme) (tree : Syntax Lexeme)
    (category : Category) : Prop := Judges lexicon tree category []

/-- Punctuation, capitalization and line boundaries are owned by document productions. -/
inductive DocumentLinearizes : DocumentRule → List Surface → Surface → Prop where
  | sentence {first : Atom} {tail : Surface} : DocumentLinearizes .sentence
      [first :: tail] (Surface.finishSentence (first.capitalize :: tail))
  | body {a : Surface} : DocumentLinearizes .body [a] a
  | bodyJoin {a b : Surface} : DocumentLinearizes .bodyJoin [a, b] (a ++ b)
  | ordinary {a : Surface} : DocumentLinearizes .ordinary [a] a
  | document {a : Surface} : DocumentLinearizes .document [a] a
  | append {a b : Surface} : DocumentLinearizes .append [a, b] (a ++ [.lineBreak] ++ b)
  | costAction {first : Atom} {tail : Surface} :
      DocumentLinearizes .costAction [first :: tail] (first.capitalize :: tail)
  | costSymbol {a : Surface} : DocumentLinearizes .costSymbol [a] a
  | costJoin {a b : Surface} :
      DocumentLinearizes .costJoin [a, b] (a ++ ([.closing ","] : Surface) ++ b)
  | activated {a b : Surface} :
      DocumentLinearizes .activated [a, b] (a ++ ([.closing ":"] : Surface) ++ b)
  | triggered {first : Atom} {tail b : Surface} : DocumentLinearizes .triggered [first :: tail, b]
      (Surface.finishSentence (first.capitalize :: (tail ++ ([.closing ","] : Surface) ++ b)))
  | keywordLine {first : Atom} {tail : Surface} :
      DocumentLinearizes .keywordLine [first :: tail] (first.capitalize :: tail)
  | keywordJoin {a b : Surface} :
      DocumentLinearizes .keywordJoin [a, b] (a ++ ([.closing ","] : Surface) ++ b)
  | quote {a : Surface} : DocumentLinearizes .quote [a] a.quote
  | reminder {a : Surface} :
      DocumentLinearizes .reminder [a]
        (([.opening "("] : Surface) ++ a ++ ([.closing ")"] : Surface))
  | mode {a : Surface} : DocumentLinearizes .mode [a] ((["•"] : Surface) ++ a)
  | modeJoin {a b : Surface} : DocumentLinearizes .modeJoin [a, b] (a ++ [.lineBreak] ++ b)
  | modes {first : Atom} {tail b : Surface} : DocumentLinearizes .modes [first :: tail, b]
      (first.capitalize :: (tail ++ (["—", .lineBreak] : Surface) ++ b))
  | weightedMode {a b : Surface} :
      DocumentLinearizes .weightedMode [a, b] ((["•"] : Surface) ++ a ++ (["—"] : Surface) ++ b)
  | label {kind : LabelKind} {a b : Surface} :
      DocumentLinearizes (.label kind) [a, b] (a ++ (["—"] : Surface) ++ b)
  | chapter {a b : Surface} : DocumentLinearizes .chapter [a, b] (a ++ (["—"] : Surface) ++ b)
  | classLevel {a b c : Surface} : DocumentLinearizes .classLevel [a, b, c]
      (a ++ ([.closing ":", "Level"] : Surface) ++ b ++ [.lineBreak] ++ c)
  | levelBand {a b c : Surface} : DocumentLinearizes .levelBand [a, b, c]
      ((["LEVEL"] : Surface) ++ a ++ [.lineBreak] ++ b ++ [.lineBreak] ++ c)
  | solve {a : Surface} : DocumentLinearizes .solve [a] ((["To", "solve", "—"] : Surface) ++ a)
  | solved {a : Surface} : DocumentLinearizes .solved [a] ((["Solved", "—"] : Surface) ++ a)
  | dieRow {a b : Surface} : DocumentLinearizes .dieRow [a, b] (a ++ (["|"] : Surface) ++ b)
  | dieDashRow {a b : Surface} :
      DocumentLinearizes .dieDashRow [a, b] (a ++ (["—"] : Surface) ++ b)
  | station {a b : Surface} :
      DocumentLinearizes .station [a, b] (a ++ ([.closing "+", "|"] : Surface) ++ b)
  | supertype {a : Surface} : DocumentLinearizes .supertype [a] a
  | type {a : Surface} : DocumentLinearizes .type [a] a
  | subtype {a : Surface} : DocumentLinearizes .subtype [a] a
  | noSupertypes : DocumentLinearizes .noSupertypes [] []
  | supertypes {a b : Surface} : DocumentLinearizes .supertypes [a, b] (a ++ b)
  | types {a b : Surface} : DocumentLinearizes .types [a, b] (a ++ b)
  | subtypes {a b : Surface} : DocumentLinearizes .subtypes [a, b] (a ++ b)
  | typeLine {a b : Surface} : DocumentLinearizes .typeLine [a, b] (a ++ b)
  | subtypedLine {a b c : Surface} :
      DocumentLinearizes .subtypedLine [a, b, c] (a ++ b ++ (["—"] : Surface) ++ c)

/-- Surface order for local productions; fixed markers remain lexically declared. -/
inductive Linearizes {Lexeme : Type} (lexicon : Lexicon Lexeme) :
    Construction Lexeme → List Surface → Surface → Prop where
  | document {rule : DocumentRule} {children : List Surface} {surface : Surface} :
      DocumentLinearizes rule children surface →
      Linearizes lexicon (.document rule) children surface
  | keywordBare {head : Lexeme} {surface : Surface} : lexicon.keywordForm head surface →
      Linearizes lexicon (.keyword head none .free) [] surface
  | keywordParameter {head : Lexeme} {parameter : Category} {a keywordSurface : Surface} :
      lexicon.keywordForm head keywordSurface →
      Linearizes lexicon (.keyword head (some parameter) .free) [a] (keywordSurface ++ a)
  | keywordBound {head : Lexeme} {parameter : Category} {a : Surface} {suffix : String} :
      lexicon.keywordSuffix head suffix →
      Linearizes lexicon (.keyword head (some parameter) .boundSuffix) [a] (a ++ [.closing suffix])
  | determine {number : Number} {a b : Surface} :
      Linearizes lexicon (.determine number) [a, b] (a ++ b)
  | barePlural {a : Surface} : Linearizes lexicon .barePlural [a] a
  | quantify {number : Number} {a : Surface} : Linearizes lexicon (.quantify number) [a] a
  | compare {marker : Lexeme} {a m : Surface} : lexicon.markerForm marker m →
      Linearizes lexicon (.compare marker) [a] (m ++ a)
  | measure {marker : Lexeme} {a b m : Surface} : lexicon.markerForm marker m →
      Linearizes lexicon (.measure marker) [a, b] (a ++ m ++ b)
  | preposition {head : Lexeme} {category : Category} {a m : Surface} :
      lexicon.markerForm head m → Linearizes lexicon (.preposition head category) [a] (m ++ a)
  | verb {head : Lexeme} {form : InflectionalForm} {frame : List (FrameItem Lexeme)} {voice : Voice}
      {args : List Surface} {v : Surface} : lexicon.verbForm head form v →
      Linearizes lexicon (.verb head form frame voice) args (v ++ args.flatten)
  | auxiliary {head : Lexeme} {form selected : InflectionalForm}
      {voice selectedVoice : Voice} {a v : Surface} :
      lexicon.verbForm head form v →
      Linearizes lexicon (.auxiliary head form selected .positive voice selectedVoice) [a] (v ++ a)
  | negativeAuxiliary {head : Lexeme} {form selected : InflectionalForm}
      {voice selectedVoice : Voice} {a v : Surface} :
      lexicon.verbForm head form v →
      Linearizes lexicon (.auxiliary head form selected .negative voice selectedVoice) [a]
        (v ++ (["not"] : Surface) ++ a)
  | finite {agreement : Agreement} {form : InflectionalForm} {voice : Voice} {a b : Surface} :
      Linearizes lexicon (.finite agreement form voice) [a, b] (a ++ b)
  | imperative {a : Surface} : Linearizes lexicon .imperative [a] a
  | nonfinite {form : InflectionalForm} {voice : Voice} {a : Surface} :
      Linearizes lexicon (.nonfinite form voice) [a] a
  | subordinate {marker : Lexeme} {finiteness : Finiteness} {a m : Surface} :
      lexicon.markerForm marker m →
      Linearizes lexicon (.subordinate marker finiteness) [a] (m ++ a)
  | adjunct {host dependent : Category} {a b : Surface} :
      Linearizes lexicon (.adjunct host dependent) [a, b] (a ++ b)
  | preposedAdjunct {host dependent : Category} {a b : Surface} :
      Linearizes lexicon (.adjunct host dependent .before) [a, b] (b ++ a)
  | coordinate {coordinator : Coordinator} {category : Category} {a b : Surface} :
      Linearizes lexicon (.coordinate coordinator category) [a, b] (a ++ coordinator.surface ++ b)

mutual
  /-- Realization does not choose a reading or certify grammatical licensing. -/
  inductive Realizes {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      Syntax Lexeme → Surface → Prop where
    | noun {lexeme : Lexeme} {number : Number} {surface : Surface} :
        lexicon.nounForm lexeme number surface → Realizes lexicon (.noun lexeme number) surface
    | adjective {lexeme : Lexeme} {surface : Surface} :
        lexicon.adjectiveForm lexeme surface → Realizes lexicon (.adjective lexeme) surface
    | modify {modifier head : Syntax Lexeme} {a b : Surface} :
        Realizes lexicon modifier a → Realizes lexicon head b →
        Realizes lexicon (.modify modifier head) (a ++ b)
    | marker {lexeme : Lexeme} {surface : Surface} : lexicon.markerForm lexeme surface →
        Realizes lexicon (.marker lexeme) surface
    | word {lexeme : Lexeme} {category : Category} {surface : Surface} :
        lexicon.wordForm lexeme category surface → Realizes lexicon (.word lexeme category) surface
    | node {construction : Construction Lexeme} {children : List (Syntax Lexeme)}
        {surfaces : List Surface} {surface : Surface} :
        RealizeChildren lexicon children surfaces →
        Linearizes lexicon construction surfaces surface →
        Realizes lexicon (.node construction children) surface
    | gap {category : Category} : Realizes lexicon (.gap category) []
    | sharedCoordination {coordinator : Coordinator} {category : Category}
        {left right : Syntax Lexeme} {a b : Surface} :
        Realizes lexicon left a → Realizes lexicon right b →
        Realizes lexicon (.sharedCoordination coordinator category left right)
          (a ++ coordinator.surface ++ b)
    | relative {number : Number} {head body : Syntax Lexeme} {a b : Surface} :
        Realizes lexicon head a → Realizes lexicon body b →
        Realizes lexicon (.relative number head body) (a ++ (["that"] : Surface) ++ b)
    | ellipsis {antecedent : Syntax Lexeme} : Realizes lexicon (.ellipsis antecedent) []
  inductive RealizeChildren {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      List (Syntax Lexeme) → List Surface → Prop where
    | nil : RealizeChildren lexicon [] []
    | cons {head : Syntax Lexeme} {tail : List (Syntax Lexeme)} {a : Surface} {b : List Surface} :
        Realizes lexicon head a → RealizeChildren lexicon tail b →
        RealizeChildren lexicon (head :: tail) (a :: b)
end

/-- A grammatical analysis with its surface, without any preference or packing policy. -/
def Admissible {Lexeme : Type} (lexicon : Lexicon Lexeme) (tree : Syntax Lexeme)
    (category : Category) (surface : Surface) : Prop :=
  Derives lexicon tree category ∧ Realizes lexicon tree surface

/-- Textual admissibility retains the annotated surface as evidence, independently of selection. -/
def Written {Lexeme : Type} (lexicon : Lexicon Lexeme) (tree : Syntax Lexeme)
    (category : Category) (text : String) : Prop :=
  ∃ surface, Admissible lexicon tree category surface ∧ Spells surface text

end English
