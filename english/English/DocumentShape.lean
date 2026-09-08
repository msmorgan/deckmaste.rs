import Init

namespace English

inductive LabelKind where
  | abilityWord | flavorWord
  deriving DecidableEq

/-- Notation constituents differ in kind, and the document rules that consume one read the kind,
never its spelling: a Saga chapter line takes a roman-numeral chapter notation (`I —`, `II —`), a
die row takes a die-result notation (`2 —`, `1–9 |`). Without the distinction the chapter and
die-dash-row productions are byte-identical in children, result and linearization, and every
`<notation> — <body>` section derives as two trees. -/
inductive NotationKind where
  | plain | chapter | dieResult
  deriving DecidableEq

inductive DocumentCategory where
  | sentence | body | ability | document | cost | quotedText
  | section | modal | parenthetical | costComponent | symbol
  | label (kind : LabelKind := .abilityWord) | notation (kind : NotationKind := .plain)
  | mode | modes
  | supertype | type | subtype | supertypes | types | subtypes | typeLine
  deriving DecidableEq

inductive KeywordSeparator where
  | comma | semicolon
  deriving DecidableEq

/-- Structural families, independent of any particular frame or keyword identity. -/
inductive DocumentRule where
  | sentence | body | ordinary | document
  | costAction | costSymbol | costs | activated
  | keywordLine (separator : KeywordSeparator := .comma)
  | quote | quoteClause | quoteKeyword (period : Bool := false) | reminder | mode | modeList | modes | sentenceModes | weightedMode
  | label (kind : LabelKind := .abilityWord) | chapter | classLevel | levelBand
  | solve | solved | dieRow | dieDashRow | station
  | supertypes | types | subtypes | typeLine | subtypedLine
  deriving DecidableEq

inductive KeywordPlacement where
  | free | boundSuffix
  deriving DecidableEq

/-- Each face or door retains its own text box; this is never an inline name separator. -/
inductive FaceLayout where
  | single | room | transforming | modal | split | adventure | aftermath | meld | prepare
  deriving DecidableEq

end English
