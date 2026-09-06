import Init

namespace English

inductive LabelKind where
  | abilityWord | flavorWord
  deriving DecidableEq

inductive DocumentCategory where
  | sentence | body | ability | document | cost | quotedText
  | section | modal | costComponent | symbol | label (kind : LabelKind := .abilityWord) | notation | mode | modes
  | supertype | type | subtype | supertypes | types | subtypes | typeLine
  deriving DecidableEq

/-- Structural families, independent of any particular frame or keyword identity. -/
inductive DocumentRule where
  | sentence | body | ordinary | document
  | costAction | costSymbol | costs | activated
  | keywordLine | quote | reminder | mode | modeList | modes | sentenceModes | weightedMode
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
