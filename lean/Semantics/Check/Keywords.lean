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
def keywordSchemas (k : KeywordLabel) : List KeywordSchema :=
  (keywordFactsFor k).elim [] (·.argumentSchemas)

def KeywordSchema.shapes (schema : KeywordSchema) : List KeywordParamShape := schema.map (·.shape)

def keywordCosts (k : KeywordLabel) : Bool :=
  (keywordSchemas k).any (fun schema => schema.any (·.shape == .cost))
/-- The ability categories the keyword's definition may be written in [CR#702.1]. -/
def keywordDefinition (k : KeywordLabel) : List AbilityCategory :=
  (keywordFactsFor k).elim [] (·.definition)
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

def keywordCounterOk (k : KeywordLabel) : Bool := (keywordFactsFor k).elim false (·.counterEligible)
def keywordStackRegime (k : KeywordLabel) : Option StackRegime := keywordFactsFor k >>= (·.regime)
def keywordFunctionsOnStack (k : KeywordLabel) : Bool :=
  (keywordFactsFor k).elim false (·.functionsOnStack)

def PaidCostName.named : PaidCostName → Bool
  | .byKeyword kw => (keywordFactsFor kw).elim false (·.paidCost)
  | .byNthKeyword ordinal kw => ordinal.ok && (keywordFactsFor kw).elim false (·.paidCost)
  | .theAlternative => true
  | .theAdditional => true

def PaidFacet.named : PaidFacet → Bool
  | .colorsSpent => true
  | .manaValueSpent => true
  | .timesPaid which => which.named
  | .readback which _ => which.named

def AbilityClass.known : AbilityClass → Bool
  | .keyword k => knownKeyword k
  | _ => true

def distinctClasses : List AbilityClass → Bool
  | [] => true
  | c :: cs => !cs.elem c && distinctClasses cs

def CostNamed.nameable : CostNamed → Bool
  | .containing _ => true
  | .ofKeyword kw => keywordCosts kw
  | .ofSpecialAction _ => true

/-- Idris `KeywordCounterEligible`, `knownCounter`, `counterShift` folded: is the counter kind
one the tables admit? -/
def CounterKind.known : CounterKind → Bool
  | .boost p t => counterShift p && counterShift t
  | .keyword k => keywordCounterOk k
  | .named l => knownCounter l

end Semantics
