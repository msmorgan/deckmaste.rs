import Semantics.Check.Words

/-!
# Semantics.Check.Keywords

The laws over the keyword shapes and registry rows supplied by `Check.Facts`.
-/

namespace Semantics

def distinctKeywordWords : List KeywordFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.word)).elem f.word && distinctKeywordWords fs

def keywordFactsFor (k : KeywordLabel) : Option KeywordFacts := keywordFacts.find? (·.word == k)
def knownKeyword (k : KeywordLabel) : Bool := (keywordFactsFor k).isSome

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
def keywordSchemas (k : KeywordLabel) : List KeywordSchema :=
  (keywordFactsFor k).elim [] (·.argumentSchemas)

def KeywordSchema.shapes (schema : KeywordSchema) : List KeywordParamShape := schema.map (·.shape)

def keywordCosts (k : KeywordLabel) : Bool :=
  (keywordSchemas k).any (fun schema => schema.any (·.shape == .cost))

def KeywordFamily.ok (c : KeywordFamily) : Bool :=
  match keywordFactsFor c.word with
  | none => false
  | some f =>
    match c.sort with
    | none => f.argumentSchemas.any (fun schema => !schema.isEmpty)
    | some _ => f.argumentSchemas.any (fun schema => schema.shapes == [.quality])

def KeywordTerm.known : KeywordTerm → Bool
  | .the k => knownKeyword k
  | .anyIn c => c.ok
  | .theWith k _ => (keywordSchemas k).any (fun schema => schema.shapes == [.number])

def allKnownKeywordTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => k.known && allKnownKeywordTerms ks

def LinkSource.named : LinkSource → Bool
  | .byKeyword kw => (keywordFactsFor kw).elim false (·.paidCost)
  | .byNthKeyword ordinal kw => ordinal.ok && (keywordFactsFor kw).elim false (·.paidCost)
  | .theAlternative => true
  | .theAdditional => true

def AbilityClass.known : AbilityClass → Bool
  | .keyword k => knownKeyword k
  | _ => true

def distinctClasses : List AbilityClass → Bool
  | [] => true
  | c :: cs => !cs.elem c && distinctClasses cs

def CostNamed.nameable : CostNamed → Bool
  | .containing _ => true
  | .ofKeyword kw => keywordCosts kw

end Semantics
