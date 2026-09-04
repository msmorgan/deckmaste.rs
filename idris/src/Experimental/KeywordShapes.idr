module Experimental.KeywordShapes

import public Data.So

%default total

public export
KeywordLabel : Type
KeywordLabel = String

public export
data CompoundHead = QualityHead | NumberHead

public export
compoundHeadIx : CompoundHead -> Nat
compoundHeadIx QualityHead = 0
compoundHeadIx NumberHead = 1

public export
Eq CompoundHead where
  (==) a b = compoundHeadIx a == compoundHeadIx b

public export
compoundHeadOptional : CompoundHead -> Bool
compoundHeadOptional QualityHead = True
compoundHeadOptional NumberHead = False

||| `QualityParam` covers "partner with [name]" [CR#702.124j], whose slot is a
||| card name.
public export
data KeywordParamShape = NoParam | CostParam | QualityParam | SubjectParam
                       | NumberParam | AbilityParam
                       | CompoundParam CompoundHead
                       | DeckConditionParam

public export
keywordParamShapeIx : KeywordParamShape -> Nat
keywordParamShapeIx NoParam = 0
keywordParamShapeIx CostParam = 1
keywordParamShapeIx QualityParam = 2
keywordParamShapeIx SubjectParam = 3
keywordParamShapeIx NumberParam = 4
keywordParamShapeIx AbilityParam = 5
keywordParamShapeIx (CompoundParam _) = 6
keywordParamShapeIx DeckConditionParam = 7

public export
sameKeywordParam : KeywordParamShape -> KeywordParamShape -> Bool
sameKeywordParam (CompoundParam a) (CompoundParam b) = a == b
sameKeywordParam _ _ = True

public export
Eq KeywordParamShape where
  (==) a b = keywordParamShapeIx a == keywordParamShapeIx b && sameKeywordParam a b

public export
paramShapeFits : KeywordParamShape -> KeywordParamShape -> Bool
paramShapeFits (CompoundParam h) CostParam = compoundHeadOptional h
paramShapeFits want got = want == got

||| A written parameter fits a keyword when any of the row's admitted shapes
||| takes it, so a keyword's CR-defined variants live on one row.
public export
paramShapesFit : List KeywordParamShape -> KeywordParamShape -> Bool
paramShapesFit wants got = any (\want => paramShapeFits want got) wants

public export
data StackRegime = AtCasting | AtResolution

public export
stackRegimeIx : StackRegime -> Nat
stackRegimeIx AtCasting = 0
stackRegimeIx AtResolution = 1

public export
Eq StackRegime where
  (==) a b = stackRegimeIx a == stackRegimeIx b

public export
record KeywordFacts where
  constructor MkKeywordFacts
  word : KeywordLabel
  ||| Every parameter shape the CR admits for this keyword [CR#702].
  paramShapes : List KeywordParamShape
  counterEligible : Bool
  regime : Maybe StackRegime
  ||| True where the ability is the spell's own, so no permanent holds it [CR#113.6].
  functionsOnStack : Bool
  onPermanentCard : Bool
  onSpellCard : Bool
  paidCost : Bool
  ||| Defined by the CR as a triggered ability with a quoted expansion [CR#702.21a,702.24a,702.30a,702.40a,702.45a,702.86a,702.112a,702.135a].
  bodied : Bool
  wantsModes : Bool

public export
defaultKeywordFacts : KeywordFacts
defaultKeywordFacts = MkKeywordFacts "" [] False Nothing False True False False False False
