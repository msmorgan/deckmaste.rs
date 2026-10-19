import Semantics.Check.Words

/-!
# Semantics.Check.Keywords

The laws over the keyword shapes and registry rows supplied by `Check.Facts`.
-/

namespace Semantics

/-- A card is a permanent card or an instant or sorcery card by its types [CR#110.4a]. -/
inductive CardClass where
  | permanentCard | instantOrSorceryCard
  deriving DecidableEq, Repr

def distinctKeywordWords : List KeywordFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.word)).elem f.word && distinctKeywordWords fs

def keywordFactsFor (k : KeywordLabel) : Option KeywordFacts := keywordFacts.find? (·.word == k)
def knownKeyword (k : KeywordLabel) : Bool := (keywordFactsFor k).isSome
def keywordSchemas (k : KeywordLabel) : List KeywordSchema :=
  (keywordFactsFor k).elim [] (·.argumentSchemas)

def KeywordSchema.shapes (schema : KeywordSchema) : List KeywordParamShape := schema.map (·.shape)

def keywordCosts (k : KeywordLabel) : Bool :=
  (keywordSchemas k).any (fun schema => schema.any (·.shape == .cost))
def keywordParamless (k : KeywordLabel) : Bool :=
  (keywordSchemas k).any List.isEmpty

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

def KeywordTerm.bare : KeywordTerm → Bool
  | .the k => keywordParamless k
  | .anyIn c => c.ok
  | .theWith _ _ => false

def allKnownKeywordTerms : List KeywordTerm → Bool
  | [] => true
  | k :: ks => k.known && allKnownKeywordTerms ks

def LinkSource.named : LinkSource → Bool
  | .byKeyword kw => (keywordFactsFor kw).elim false (·.paidCost)
  | .byNthKeyword ordinal kw => ordinal.ok && (keywordFactsFor kw).elim false (·.paidCost)
  | .theAlternative => true
  | .theAdditional => true
  | .byDeed deed => knownAct deed

/-- Whether paying the named cost chooses a player for the linked ability to read back
[CR#607.2d,702.174a]. Only a keyword's cost declares one; "the additional cost" and "the
alternative cost" name no declared choice. -/
def LinkSource.choosesPlayer : LinkSource → Bool
  | .byKeyword kw | .byNthKeyword _ kw => (keywordFactsFor kw).elim false (·.costChoosesPlayer)
  | .theAlternative | .theAdditional | .byDeed _ => false

/-- The deed an `objects` facet reads is one its Link Source performs: a printed ability named
by its deed performs that deed, and a keyword ability's own deed ("creatures it devoured"
[CR#702.82b]) is performed by that keyword's source alone. A deed the source never performs
leaves the facet nothing to read [CR#607.2a]. -/
def LinkSource.performs (which : LinkSource) (deed : Deed) : Bool :=
  match which, deed with
  | .byDeed d, _ => d == deed
  | .byKeyword kw, .ofAbility k | .byNthKeyword _ kw, .ofAbility k => kw == k
  | _, .ofAbility _ => false
  | _, _ => true

def AbilityClass.known : AbilityClass → Bool
  | .keyword k => knownKeyword k
  | _ => true

def distinctClasses : List AbilityClass → Bool
  | [] => true
  | c :: cs => !cs.elem c && distinctClasses cs

def CostNamed.nameable : CostNamed → Bool
  | .containing _ => true
  | .ofKeyword kw => keywordCosts kw

/-- Is the counter kind one the tables admit? One registry lookup: a kind is a counter
declaration's name, and the counter facts table has a row for every declaration [CR#122.1], so
a +X/+Y counter [CR#122.1a] and a keyword counter [CR#122.1b] are known exactly when they are
declared. -/
def CounterKind.known (c : CounterKind) : Bool := knownCounter c.name

end Semantics
