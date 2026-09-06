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
  | measurePhrase | keywordPhrase
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
  | marked (marker : Lexeme) (complement : Complement)

inductive Coordinator where
  | and_ | or_ | andOr
  deriving DecidableEq

def Coordinator.surface : Coordinator → Surface
  | .and_ => (["and"] : Surface)
  | .or_ => (["or"] : Surface)
  | .andOr => (["and/or"] : Surface)

def Person.join : Person → Person → Person
  | .first, _ | _, .first => .first
  | .second, _ | _, .second => .second
  | .third, .third => .third

/-- Agreement of an additive coordination includes both conjuncts' person features. -/
def Agreement.additive (left right : Agreement) : Agreement :=
  ⟨left.person.join right.person, .plural⟩

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
  | initialAdverbial (dependent : Category)
  | imperative
  | nonfinite (form : InflectionalForm) (voice : Voice := .active)
  | subordinate (marker : Lexeme) (finiteness : Finiteness)
  | adjunct (host dependent : Category) (placement : Placement := .after)
  | coordinate (coordinator : Coordinator) (category : Category) (right : Category := category)

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

/-- Sentence and modal groups share ordinary and colon-prefixed paragraph hosts. -/
def paragraphItem : Category → Bool
  | .document .sentence | .document .modal => true
  | _ => false

/-- Section boundaries remain visible within the flat sequence of a text box. -/
def documentItem : Category → Bool
  | .document .ability | .document .section => true
  | _ => false

/-- Only linguistic constituents coordinate; textual containers have separate list rules. -/
def coordinable : Category → Bool
  | .document _ => false
  | _ => true

inductive InitialAdverbial : Category → Prop where
  | subordinate : InitialAdverbial (.subordinateClause .finite)
  | preposition : InitialAdverbial .prepositionPhrase

/-- Collections consume items directly, never another collection of the same kind. -/
inductive DocumentProduction : DocumentRule → List Category → Category → Prop where
  | sentence : DocumentProduction .sentence [.clause .finite] (.document .sentence)
  | body {first : Category} {rest : List Category} :
      (first :: rest).all paragraphItem = true →
      DocumentProduction .body (first :: rest) (.document .body)
  | ordinary : DocumentProduction .ordinary [.document .body] (.document .ability)
  | document {items : List Category} : items.all documentItem = true →
      DocumentProduction .document items (.document .document)
  | costAction : DocumentProduction .costAction
      [.verbPhrase .plain] (.document .costComponent)
  | costSymbol : DocumentProduction .costSymbol
      [.document .symbol] (.document .costComponent)
  | costs {n : Nat} : DocumentProduction .costs
      (List.replicate (n + 1) (.document .costComponent)) (.document .cost)
  | activated : DocumentProduction .activated
      [.document .cost, .document .body] (.document .ability)
  | keywordLine {n : Nat} : DocumentProduction .keywordLine
      (List.replicate (n + 1) .keywordPhrase) (.document .ability)
  | quote : DocumentProduction .quote [.document .document] (.document .quotedText)
  | mode : DocumentProduction .mode [.document .body] (.document .mode)
  | modeList {n : Nat} : DocumentProduction .modeList
      (List.replicate (n + 1) (.document .mode)) (.document .modes)
  | modes : DocumentProduction .modes
      [.clause .finite, .document .modes] (.document .modal)
  | sentenceModes {n : Nat} : DocumentProduction .sentenceModes
      (List.replicate (n + 1) (.document .sentence) ++ [.document .modes]) (.document .modal)
  | weightedMode : DocumentProduction .weightedMode
      [.document .symbol, .document .body] (.document .mode)
  | label {kind : LabelKind} : DocumentProduction (.label kind)
      [.document (.label kind), .document .ability] (.document .ability)
  | chapter : DocumentProduction .chapter
      [.document .notation, .document .body] (.document .section)
  | classLevel : DocumentProduction .classLevel
      [.document .cost, .document .notation, .document .document] (.document .section)
  | levelBand : DocumentProduction .levelBand
      [.document .notation, .document .notation, .document .document] (.document .section)
  | solve : DocumentProduction .solve [.document .body] (.document .section)
  | solved : DocumentProduction .solved [.document .ability] (.document .section)
  | dieRow : DocumentProduction .dieRow
      [.document .notation, .document .body] (.document .section)
  | dieDashRow : DocumentProduction .dieDashRow
      [.document .notation, .document .body] (.document .section)
  | station : DocumentProduction .station
      [.document .notation, .document .ability] (.document .section)
  | supertypes {n : Nat} : DocumentProduction .supertypes
      (List.replicate n (.document .supertype)) (.document .supertypes)
  | types {n : Nat} : DocumentProduction .types
      (List.replicate (n + 1) (.document .type)) (.document .types)
  | subtypes {n : Nat} : DocumentProduction .subtypes
      (List.replicate (n + 1) (.document .subtype)) (.document .subtypes)
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
      Production lexicon (.keyword head none .free) [] .keywordPhrase
  | keywordParameter {head : Lexeme} {parameter : Category} {placement : KeywordPlacement} :
      lexicon.keyword head (some parameter) placement →
      Production lexicon (.keyword head (some parameter) placement) [parameter] .keywordPhrase
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
  | initialAdverbial {dependent : Category} : InitialAdverbial dependent →
      Production lexicon (.initialAdverbial dependent)
        [dependent, .clause .finite] (.clause .finite)
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
      coordinable category = true →
      Production lexicon (.coordinate coordinator category) [category, category]
        (coordinationResult coordinator category)
  | mixedAdditive {left right : Agreement} : left ≠ right →
      Production lexicon (.coordinate .and_ (.nounPhrase left) (.nounPhrase right))
        [.nounPhrase left, .nounPhrase right] (.nounPhrase (left.additive right))

  | mixedAlternative {left right : Agreement} : left ≠ right →
      Production lexicon (.coordinate .or_ (.nounPhrase left) (.nounPhrase right))
        [.nounPhrase left, .nounPhrase right] (.nounPhrase right)

/-- One recursive tree for phrases, clauses, missing positions and their antecedents. -/
inductive Syntax (Lexeme : Type) where
  | noun (lexeme : Lexeme) (number : Number)
  | adjective (lexeme : Lexeme)
  | modify (modifier head : Syntax Lexeme)
  | marker (lexeme : Lexeme)
  | word (lexeme : Lexeme) (category : Category)
  | node (construction : Construction Lexeme) (children : List (Syntax Lexeme))
  | gap (category : Category)
  | frameCoordination (coordinator : Coordinator) (left right : List (Syntax Lexeme))
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
    | .frameCoordination _ left right => reminderFreeChildren left && reminderFreeChildren right
    | .ellipsis antecedent => antecedent.reminderFree
    | _ => true
  def reminderFreeChildren {Lexeme : Type} : List (Syntax Lexeme) → Bool
    | [] => true
    | first :: rest => first.reminderFree && reminderFreeChildren rest
end

/-- The initial plural nominal coordination uses the general coordination production. -/
def Syntax.coordinate {Lexeme : Type} (left right : Syntax Lexeme) : Syntax Lexeme :=
  .node (.coordinate .and_ (.nominal .plural)) [left, right]

inductive SubjectPosition where
  | beforeVerb | afterVerb
  deriving DecidableEq

/-- Agreement is read at the clause boundary; disjunction consults the nearer conjunct. -/
def subjectAgreement {Lexeme : Type} (position : SubjectPosition) : Syntax Lexeme → Option Agreement
  | .word _ (.nounPhrase agreement) | .gap (.nounPhrase agreement) => some agreement
  | .node (.determine number) _ => some ⟨.third, number⟩
  | .node .barePlural _ => some ⟨.third, .plural⟩
  | .node (.coordinate .and_ (.nounPhrase left) (.nounPhrase right)) _ =>
      some (left.additive right)
  | .node (.coordinate .or_ (.nounPhrase left) (.nounPhrase right)) _ =>
      some (match position with | .beforeVerb => right | .afterVerb => left)
  | .node (.coordinate .andOr (.nounPhrase left) _) _ => some left
  | .node (.adjunct _ _ _) [head, _] => subjectAgreement position head
  | _ => none

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
    | finite {agreement subjectFeatures : Agreement} {form : InflectionalForm} {voice : Voice}
        {subject predicate : Syntax Lexeme}
        {subjectGaps predicateGaps : List Category} :
        Judges lexicon subject (.nounPhrase subjectFeatures) subjectGaps →
        Judges lexicon predicate (.verbPhrase form voice) predicateGaps →
        FiniteLicense lexicon predicate agreement →
        subjectAgreement .beforeVerb subject = some agreement →
        Judges lexicon (.node (.finite agreement form voice) [subject, predicate]) (.clause .finite)
          (subjectGaps ++ predicateGaps)
    | reminder {body : Syntax Lexeme} :
        Judges lexicon body (.document .body) [] → body.reminderFree = true →
        Judges lexicon (.node (.document .reminder) [body]) (.document .ability) []
    | gap {category : Category} : Judges lexicon (.gap category) category [category]
    | sharedCoordination {coordinator : Coordinator} {category gap : Category}
        {left right : Syntax Lexeme} :
        coordinable category = true →
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
    | marked {marker : Lexeme} {head : Syntax Lexeme} {tail : List (Syntax Lexeme)}
        {complement : Complement} {frame : List (FrameItem Lexeme)}
        {headGaps tailGaps : List Category} :
        Judges lexicon head complement.category headGaps → JudgeFrame lexicon tail frame tailGaps →
        JudgeFrame lexicon (.marker marker :: head :: tail) (.marked marker complement :: frame)
          (headGaps ++ tailGaps)
    | coordinate {coordinator : Coordinator} {left right : List (Syntax Lexeme)}
        {first : FrameItem Lexeme} {rest : List (FrameItem Lexeme)}
        {leftGaps rightGaps : List Category} :
        JudgeFrame lexicon left (first :: rest) leftGaps →
        JudgeFrame lexicon right (first :: rest) rightGaps →
        JudgeFrame lexicon [.frameCoordination coordinator left right] (first :: rest)
          (leftGaps ++ rightGaps)

end

/-- Closed grammatical derivation; independent of realization and selection. -/
abbrev Derives {Lexeme : Type} (lexicon : Lexicon Lexeme) (tree : Syntax Lexeme)
    (category : Category) : Prop := Judges lexicon tree category []

/-- Punctuation, capitalization and line boundaries are owned by document productions. -/
inductive DocumentLinearizes : DocumentRule → List Surface → Surface → Prop where
  | sentence {first : Atom} {tail : Surface} : DocumentLinearizes .sentence
      [first :: tail] (Surface.finishSentence (first.capitalize :: tail))
  | body {items : List Surface} : DocumentLinearizes .body items items.flatten
  | ordinary {a : Surface} : DocumentLinearizes .ordinary [a] a
  | document {items : List Surface} :
      DocumentLinearizes .document items (Surface.join [.lineBreak] items)
  | costAction {first : Atom} {tail : Surface} :
      DocumentLinearizes .costAction [first :: tail] (first.capitalize :: tail)
  | costSymbol {a : Surface} : DocumentLinearizes .costSymbol [a] a
  | costs {items : List Surface} :
      DocumentLinearizes .costs items (Surface.join [.closing ","] items)
  | activated {a b : Surface} :
      DocumentLinearizes .activated [a, b] (a ++ ([.closing ":"] : Surface) ++ b)
  | keywordLine {items : List Surface} :
      DocumentLinearizes .keywordLine items
        ((Surface.join [.closing ","] items).capitalize)
  | quote {a : Surface} : DocumentLinearizes .quote [a] a.quote
  | reminder {a : Surface} :
      DocumentLinearizes .reminder [a]
        (([.opening "("] : Surface) ++ a ++ ([.closing ")"] : Surface))
  | mode {a : Surface} : DocumentLinearizes .mode [a] ((["•"] : Surface) ++ a)
  | modeList {items : List Surface} :
      DocumentLinearizes .modeList items (Surface.join [.lineBreak] items)
  | modes {first : Atom} {tail b : Surface} : DocumentLinearizes .modes [first :: tail, b]
      (first.capitalize :: (tail ++ (["—", .lineBreak] : Surface) ++ b))
  | sentenceModes {headers : List Surface} {modes : Surface} :
      DocumentLinearizes .sentenceModes (headers ++ [modes])
        (headers.flatten ++ [.lineBreak] ++ modes)
  | weightedMode {a b : Surface} :
      DocumentLinearizes .weightedMode [a, b] ((["•"] : Surface) ++ a ++ (["—"] : Surface) ++ b)
  | label {kind : LabelKind} {a b : Surface} :
      DocumentLinearizes (.label kind) [a, b] (a ++ (["—"] : Surface) ++ b)
  | chapter {a b : Surface} : DocumentLinearizes .chapter [a, b] (a ++ (["—"] : Surface) ++ b)
  | classLevel {a b c : Surface} : DocumentLinearizes .classLevel [a, b, c]
      (a ++ ([.closing ":", "Level"] : Surface) ++ b ++ Surface.followingLine c)
  | levelBand {a b c : Surface} : DocumentLinearizes .levelBand [a, b, c]
      ((["LEVEL"] : Surface) ++ a ++ [.lineBreak] ++ b ++ Surface.followingLine c)
  | solve {a : Surface} : DocumentLinearizes .solve [a] ((["To", "solve", "—"] : Surface) ++ a)
  | solved {a : Surface} : DocumentLinearizes .solved [a] ((["Solved", "—"] : Surface) ++ a)
  | dieRow {a b : Surface} : DocumentLinearizes .dieRow [a, b] (a ++ (["|"] : Surface) ++ b)
  | dieDashRow {a b : Surface} :
      DocumentLinearizes .dieDashRow [a, b] (a ++ (["—"] : Surface) ++ b)
  | station {a b : Surface} :
      DocumentLinearizes .station [a, b] (a ++ ([.closing "+", "|"] : Surface) ++ b)
  | supertypes {items : List Surface} : DocumentLinearizes .supertypes items items.flatten
  | types {items : List Surface} : DocumentLinearizes .types items items.flatten
  | subtypes {items : List Surface} : DocumentLinearizes .subtypes items items.flatten
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
  | initialAdverbial {dependent : Category} {a b : Surface} :
      Linearizes lexicon (.initialAdverbial dependent) [a, b] (a ++ [.closing ","] ++ b)
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
  | coordinate {coordinator : Coordinator} {category right : Category} {a b : Surface} :
      Linearizes lexicon (.coordinate coordinator category right) [a, b] (a ++ coordinator.surface ++ b)

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
    | frameCoordination {coordinator : Coordinator} {left right : List (Syntax Lexeme)}
        {as bs : List Surface} :
        RealizeChildren lexicon left as → RealizeChildren lexicon right bs →
        Realizes lexicon (.frameCoordination coordinator left right)
          (as.flatten ++ coordinator.surface ++ bs.flatten)
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
