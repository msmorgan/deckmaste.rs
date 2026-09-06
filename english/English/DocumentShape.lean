import Init

namespace English

inductive LabelKind where
  | abilityWord | flavorWord
  deriving DecidableEq

inductive DocumentCategory where
  | sentence | body | ability | document | cost | keyword | quotedText
  | symbol | label (kind : LabelKind := .abilityWord) | notation | mode | modes
  | supertype | type | subtype | supertypes | types | subtypes | typeLine
  deriving DecidableEq

/-- Structural families, independent of any particular frame or keyword identity. -/
inductive DocumentRule where
  | sentence | body | bodyJoin | ordinary | document | append
  | costAction | costSymbol | costJoin | activated | triggered
  | keywordLine | keywordJoin | quote | reminder | mode | modeJoin | modes | weightedMode
  | label (kind : LabelKind := .abilityWord) | chapter | classLevel | levelBand
  | solve | solved | dieRow | dieDashRow | station
  | supertype | type | subtype | noSupertypes | supertypes | types | subtypes
  | typeLine | subtypedLine
  deriving DecidableEq

inductive KeywordPlacement where
  | free | boundSuffix
  deriving DecidableEq

/-- Each face or door retains its own text box; this is never an inline name separator. -/
inductive FaceLayout where
  | single | room | transforming | modal | split | adventure | aftermath | meld | prepare
  deriving DecidableEq

end English
