import Semantics.Check.Words

/-!
# Semantics.Check.Keywords

The laws over keyword terms and the ability words [CR#207.2c].
-/

namespace Semantics

/-- The ability words, as [CR#207.2c] lists and spells them. -/
def abilityWordNames : List String :=
  [ "adamant",
    "addendum",
    "alliance",
    "battalion",
    "bloodrush",
    "celebration",
    "channel",
    "chroma",
    "cohort",
    "constellation",
    "converge",
    "council’s dilemma",
    "coven",
    "delirium",
    "descend 4",
    "descend 8",
    "disappear",
    "domain",
    "eerie",
    "eminence",
    "enrage",
    "fateful hour",
    "fathomless descent",
    "ferocious",
    "flurry",
    "formidable",
    "grandeur",
    "hellbent",
    "heroic",
    "imprint",
    "infusion",
    "inspired",
    "join forces",
    "kinship",
    "landfall",
    "lieutenant",
    "magecraft",
    "metalcraft",
    "morbid",
    "opus",
    "pack tactics",
    "paradox",
    "parley",
    "radiance",
    "raid",
    "rally",
    "renew",
    "repartee",
    "revolt",
    "secret council",
    "spell mastery",
    "strive",
    "survival",
    "sweep",
    "tempting offer",
    "threshold",
    "undergrowth",
    "valiant",
    "vivid",
    "void",
    "will of the council" ]

/-- Whether an ability word is one [CR#207.2c] lists. -/
def AbilityWordLabel.known : AbilityWordLabel → Bool
  | .named name => abilityWordNames.contains name

/-- Whether a label names a keyword. Every label a card can write is the authored constant of a
keyword Definition (`Authoring.keywordLabelConstant`); `keywordOf` gives an ability that is no
keyword the empty label. -/
def KeywordLabel.namesKeyword : KeywordLabel → Bool
  | .named name => !name.isEmpty

def KeywordTerm.known : KeywordTerm → Bool
  | .the k | .theWith k _ => k.namesKeyword
  | .anyIn c => c.word.namesKeyword

def allKnownKeywordTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => k.known && allKnownKeywordTerms ks

def LinkSource.named : LinkSource → Bool
  | .byNthKeyword ordinal _ => ordinal.ok
  | .byKeyword _ | .theAlternative | .theAdditional => true

def distinctClasses : List AbilityClass → Bool
  | [] => true
  | c :: cs => !cs.elem c && distinctClasses cs

end Semantics
