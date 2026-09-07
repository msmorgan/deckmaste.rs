import English.Lexical

/-! V3 surface recognition preserves the selected lexical casing. Document boundaries require
initial capitalization; they do not rewrite the lexical analysis beneath them.

`English.Linearizes` / `English.DocumentLinearizes` in `English/Grammar.lean` are the parallel
schema-level copies of these relations. They diverge in exactly one place — the schema copy
rewrites sentence and cost-action case, this one requires it — so a `Spells` claim proved
against the schema copy is weaker than it reads. Any new constructor must be added to **both**
copies or the v3 model silently loses it. -/
namespace English.Reading

inductive DocumentLinearizes : DocumentRule → List Surface → Surface → Prop where
  | sentence {first : Atom} {tail : Surface} : first.capitalize = first → DocumentLinearizes .sentence
      [first :: tail] (Surface.finishSentence (first :: tail))
  | body {items : List Surface} : DocumentLinearizes .body items items.flatten
  | ordinary {a : Surface} : DocumentLinearizes .ordinary [a] a
  | document {items : List Surface} :
      DocumentLinearizes .document items (Surface.join [.lineBreak] items)
  | costAction {first : Atom} {tail : Surface} :
      first.capitalize = first → DocumentLinearizes .costAction [first :: tail] (first :: tail)
  | costSymbol {a : Surface} : DocumentLinearizes .costSymbol [a] a
  | costs {items : List Surface} :
      DocumentLinearizes .costs items (Surface.join [.closing ","] items)
  | activated {a b : Surface} :
      DocumentLinearizes .activated [a, b] (a ++ ([.closing ":"] : Surface) ++ b)
  | keywordLine {items : List Surface} : (Surface.join [.closing ","] items).capitalize = Surface.join [.closing ","] items →
      DocumentLinearizes .keywordLine items (Surface.join [.closing ","] items)
  | keywordSeparated {items : List Surface} :
      (Surface.join [.closing ";"] items).capitalize = Surface.join [.closing ";"] items →
      DocumentLinearizes (.keywordLine .semicolon) items (Surface.join [.closing ";"] items)
  | quote {a : Surface} : DocumentLinearizes .quote [a] a.quote
  | reminder {a : Surface} :
      DocumentLinearizes .reminder [a]
        (([.opening "("] : Surface) ++ a ++ ([.closing ")"] : Surface))
  | mode {a : Surface} : DocumentLinearizes .mode [a] ((["•"] : Surface) ++ a)
  | modeList {items : List Surface} :
      DocumentLinearizes .modeList items (Surface.join [.lineBreak] items)
  | modes {first : Atom} {tail b : Surface} : first.capitalize = first → DocumentLinearizes .modes [first :: tail, b]
      (first :: (tail ++ (["—", .lineBreak] : Surface) ++ b))
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
  | bareMass {a : Surface} : Linearizes lexicon .bareMass [a] a
  | attributive {head : Lexeme} {number : Number} {a m : Surface} :
      lexicon.markerForm head m → Linearizes lexicon (.attributive head number) [a] (m ++ a)
  | targeting {head : Lexeme} {number : Number} {a m : Surface} :
      lexicon.markerForm head m → Linearizes lexicon (.targeting head number) [a] (m ++ a)
  | rightNodeRaising {result filler : Category} {a b : Surface} :
      Linearizes lexicon (.rightNodeRaising result filler) [a,b] (a ++ b)
  | genitive {number : Number} {a : Surface} :
      Linearizes lexicon (.genitive number) [a] (a ++ [.closing "'s"])
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
      Linearizes lexicon (.coordinate coordinator category right) [a, b] (a ++ coordinator.surface
        ++ b)
  | serialCoordinate {coordinator : Coordinator} {category : Category} {items : List Surface} :
      Linearizes lexicon (.serialCoordinate coordinator category) items
        (Surface.serial coordinator.surface items)

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
    | identity {head : Lexeme} {category : Category} {surface : Surface} :
        lexicon.identityForm head category surface → Realizes lexicon (.identity head category)
          surface
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
    | zeroRelative {number : Number} {head body : Syntax Lexeme} {a b : Surface} :
        Realizes lexicon head a → Realizes lexicon body b →
        Realizes lexicon (.relative number head body .zero) (a ++ b)
    | frontedRelative {number : Number} {head body front : Syntax Lexeme} {a b f : Surface} :
        Realizes lexicon head a → Realizes lexicon front f → Realizes lexicon body b →
        Realizes lexicon (.relative number head body .fronted [front]) (a ++ f ++ b)
    | supplementaryRelative {number : Number} {head body front : Syntax Lexeme} {a b f : Surface} :
        Realizes lexicon head a → Realizes lexicon front f → Realizes lexicon body b →
        Realizes lexicon (.relative number head body .supplementary [front])
          (a ++ [.closing ","] ++ f ++ b ++ [.closing ","])
    | ellipsis {form : InflectionalForm} {voice : Voice} :
        Realizes lexicon (.ellipsis form voice) []
  inductive RealizeChildren {Lexeme : Type} (lexicon : Lexicon Lexeme) :
      List (Syntax Lexeme) → List Surface → Prop where
    | nil : RealizeChildren lexicon [] []
    | cons {head : Syntax Lexeme} {tail : List (Syntax Lexeme)} {a : Surface} {b : List Surface} :
        Realizes lexicon head a → RealizeChildren lexicon tail b →
        RealizeChildren lexicon (head :: tail) (a :: b)
end


end English.Reading
