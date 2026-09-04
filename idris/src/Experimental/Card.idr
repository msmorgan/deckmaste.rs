module Experimental.Card

import public Experimental.Effect

%default total

public export
CardSupers : List Supertype -> Type
CardSupers ss = So (supersDistinct ss)

public export
data CardClass = PermanentCard | SpellCard

public export
cardClassOf : List CardType -> CardClass
cardClassOf [] = PermanentCard
cardClassOf (t :: ts) =
  if spellCardType t then SpellCard
  else cardClassOf ts

public export
anyPermanentType : List CardType -> Bool
anyPermanentType [] = False
anyPermanentType (t :: ts) = permanentType t || anyPermanentType ts

public export
anySpellType : List CardType -> Bool
anySpellType [] = False
anySpellType (t :: ts) = spellCardType t || anySpellType ts

public export
hasNonKindredType : List CardType -> Bool
hasNonKindredType [] = False
hasNonKindredType (t :: ts) = not (t == Kindred) || hasNonKindredType ts

public export
typesCombinable : List CardType -> Bool
typesCombinable tys =
  not (anyPermanentType tys && anySpellType tys)
    && (not (elem Kindred tys) || hasNonKindredType tys)

public export
keywordCardOk : CardClass -> KeywordLabel -> Bool
keywordCardOk PermanentCard k = maybe False onPermanentCard (keywordFactsFor k)
keywordCardOk SpellCard k = maybe False onSpellCard (keywordFactsFor k)

public export
staticOnSpellCardOk : {0 bs : Bindings} -> StaticSpec bs -> Bool
staticOnSpellCardOk (Deontic _ Forbid deeds Patient _ _ _ _) =
  all (\deed => deedZoneOf deed Patient == Just Stack) deeds
staticOnSpellCardOk (AltCost This _) = True
staticOnSpellCardOk (CostsToCast This _) = True
staticOnSpellCardOk (AddedCost _ _) = True
staticOnSpellCardOk (OnlyDuring _ _ (Deontic _ Permit deeds Patient _ _ _ _)) =
  all (\deed => deedZoneOf deed Patient == Just Stack) deeds
staticOnSpellCardOk (OnlyDuring _ _ se) = staticOnSpellCardOk se
staticOnSpellCardOk (Conditionally se _ _) = staticOnSpellCardOk se
staticOnSpellCardOk _ = False

public export
classAbilityOk : {0 bs : Bindings} -> CardClass -> AbilityAt bs -> Bool
classAbilityOk PermanentCard (KeywordAbility k _ _) = keywordCardOk PermanentCard k
classAbilityOk PermanentCard (Activated _ _ _ _ _ _) = True
classAbilityOk PermanentCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk PermanentCard (Static _) = True
classAbilityOk PermanentCard (AlsoForKeywords ab _) = classAbilityOk PermanentCard ab
classAbilityOk PermanentCard (Spell _ _) = False
classAbilityOk PermanentCard (ItalicHead _ ab) = classAbilityOk PermanentCard ab
classAbilityOk PermanentCard MayBeginOnBattlefield = True
classAbilityOk SpellCard (KeywordAbility k _ _) = keywordCardOk SpellCard k
classAbilityOk SpellCard (Activated c _ _ _ _ _) = costOffBattlefield c
classAbilityOk SpellCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk SpellCard (Static se) = staticOnSpellCardOk se
classAbilityOk SpellCard (AlsoForKeywords ab _) = classAbilityOk SpellCard ab
classAbilityOk SpellCard (Spell _ _) = True
classAbilityOk SpellCard (ItalicHead _ ab) = classAbilityOk SpellCard ab
classAbilityOk SpellCard MayBeginOnBattlefield = False

public export
cardAbilityOk : {0 bs : Bindings} -> List CardType -> AbilityAt bs -> Bool
cardAbilityOk tys a = classAbilityOk (cardClassOf tys) a

public export
cardTextOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs -> Bool
cardTextOk tys [] = True
cardTextOk tys (a :: as) = cardAbilityOk tys a && cardTextOk tys as

public export
chapterLineOk : {0 bs : Bindings} -> List Subtype -> AbilityAt bs -> Bool
chapterLineOk subs (Triggered _ (ChapterMark _) _ _ _ _ _ _ _) = elem (enchantmentType "Saga") subs
chapterLineOk subs (ItalicHead _ ab) = chapterLineOk subs ab
chapterLineOk subs (AlsoForKeywords ab _) = chapterLineOk subs ab
chapterLineOk _ _ = True

public export
chapterFrameOk : {0 bs : Bindings} -> List Subtype -> AbilitySeq bs -> Bool
chapterFrameOk subs [] = True
chapterFrameOk subs (a :: as) = chapterLineOk subs a && chapterFrameOk subs as

public export
doorFrameOk : {0 bs : Bindings} -> AbilitySeq bs -> Bool
doorFrameOk [] = True
doorFrameOk (a :: as) = not (abilityNamesThisDoor a) && doorFrameOk as

public export
DoorFrame : {0 bs : Bindings} -> AbilitySeq bs -> Type
DoorFrame as = So (doorFrameOk as)

public export
data PrintedStat = PrintedNum Integer | PrintedStar | PrintedStarPlus Nat
                 | PrintedMinusStar Nat

public export
starred : PrintedStat -> Bool
starred (PrintedNum _) = False
starred PrintedStar = True
starred (PrintedStarPlus _) = True
starred (PrintedMinusStar _) = True

public export
data PrintedBox : Type where
  PtBox : (pow : PrintedStat) -> (tou : PrintedStat) -> PrintedBox
  LoyaltyBox : (start : PrintedStat) -> PrintedBox
  DefenseBox : (def : PrintedStat) -> PrintedBox

public export
boxPt : Maybe PrintedBox -> Maybe (PrintedStat, PrintedStat)
boxPt (Just (PtBox p t)) = Just (p, t)
boxPt _ = Nothing

public export
staticDefinesPt : {0 bs : Bindings} -> StaticSpec bs -> Maybe DefinedSlots
staticDefinesPt (DefinesPt _ sl _) = Just sl
staticDefinesPt (Conditionally se _ _) = staticDefinesPt se
staticDefinesPt _ = Nothing

public export
abDefinesPt : {0 bs : Bindings} -> AbilityAt bs -> Maybe DefinedSlots
abDefinesPt (Static se) = staticDefinesPt se
abDefinesPt (ItalicHead _ ab) = abDefinesPt ab
abDefinesPt _ = Nothing

public export
textDefines : {0 bs : Bindings} -> (DefinedSlots -> Bool) -> AbilitySeq bs -> Bool
textDefines f [] = False
textDefines f (a :: as) =
  (case abDefinesPt a of
     Just sl => f sl
     Nothing => False) || textDefines f as

public export
definedSlotsStarred : Maybe (PrintedStat, PrintedStat) -> Bool -> Bool -> Bool
definedSlotsStarred Nothing dp dt = not dp && not dt
definedSlotsStarred (Just (p, t)) dp dt =
  (not dp || starred p) && (not dt || starred t)

public export
data FaceSide = Front | Back

public export
boxSuitsType : FaceSide -> CardType -> Maybe PrintedBox -> Bool
boxSuitsType _ Creature (Just (PtBox _ _)) = True
boxSuitsType _ Creature _ = False
boxSuitsType _ Planeswalker (Just (LoyaltyBox _)) = True
boxSuitsType Back Planeswalker Nothing = True
boxSuitsType _ Planeswalker _ = False
boxSuitsType _ Battle (Just (DefenseBox _)) = True
boxSuitsType _ Battle _ = False
boxSuitsType _ _ _ = True

public export
boxFitsLine : FaceSide -> List CardType -> Maybe PrintedBox -> Bool
boxFitsLine side [] box = True
boxFitsLine side (t :: ts) box =
  boxSuitsType side t box && boxFitsLine side ts box

public export
cardBoxOk : {0 bs : Bindings} -> FaceSide -> List CardType -> AbilitySeq bs ->
            Maybe PrintedBox -> Bool
cardBoxOk side tys text box =
  boxFitsLine side tys box &&
  definedSlotsStarred (boxPt box) (textDefines definesPower text)
                                  (textDefines definesToughness text)

public export
cardCostOk : FaceSide -> List CardType -> Maybe ManaCost -> Bool
cardCostOk _ _ Nothing = True
cardCostOk Back _ (Just _) = False
cardCostOk Front tys (Just _) = not (elem Land tys)

public export
data CardLine : TypeLine -> Type where
  MkCardLine : {auto 0 ne : So (lineNonEmpty l)} ->
               {auto 0 dst : So (typesDistinct l.tys)} ->
               {auto 0 cmb : So (typesCombinable l.tys)} ->
               {auto 0 sf : So (subsFitLine l.subs l.tys)} -> CardLine l

public export
keywordWantsModes : {0 bs : Bindings} -> AbilityAt bs -> Bool
keywordWantsModes (KeywordAbility k _ _) = maybe False wantsModes (keywordFactsFor k)
keywordWantsModes (ItalicHead _ ab) = keywordWantsModes ab
keywordWantsModes (AlsoForKeywords ab _) = keywordWantsModes ab
keywordWantsModes _ = False

public export
abilityWritesModes : {0 bs : Bindings} -> AbilityAt bs -> Bool
abilityWritesModes (Spell _ (Modal _ _)) = True
abilityWritesModes (ItalicHead _ ab) = abilityWritesModes ab
abilityWritesModes (AlsoForKeywords ab _) = abilityWritesModes ab
abilityWritesModes _ = False

public export
anyWantsModes : {0 bs : Bindings} -> AbilitySeq bs -> Bool
anyWantsModes [] = False
anyWantsModes (a :: as) = keywordWantsModes a || anyWantsModes as

public export
anyWritesModes : {0 bs : Bindings} -> AbilitySeq bs -> Bool
anyWritesModes [] = False
anyWritesModes (a :: as) = abilityWritesModes a || anyWritesModes as

public export
modalFrameOk : {0 bs : Bindings} -> AbilitySeq bs -> Bool
modalFrameOk as = not (anyWantsModes as) || anyWritesModes as

public export
CardText : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs -> Type
CardText l as = So (cardTextOk l.tys as && modalFrameOk as)

public export
CardChapters : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs -> Type
CardChapters l as = So (chapterFrameOk l.subs as)

public export
data CardBox : {0 bs : Bindings} -> FaceSide -> TypeLine -> AbilitySeq bs ->
               Maybe PrintedBox -> Type where
  MkCardBox : {0 box : Maybe PrintedBox} ->
              {auto 0 ok : So (cardBoxOk side l.tys as box)} ->
              CardBox side l as box

public export
CardCost : FaceSide -> TypeLine -> Maybe ManaCost -> Type
CardCost side l c = So (cardCostOk side l.tys c)

public export
jointBindings : List QualitySort -> Bindings -> Bindings
jointBindings [] bs = bs
jointBindings (q :: qs) bs = qualityB q :: jointBindings qs bs

||| [CR#109.3] the characteristic set, printed on a face or supplied by a rules-defined alternative
public export
record Characteristics where
  constructor MkCharacteristics
  name : String
  cost : Maybe ManaCost
  choices : List QualitySort
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq (jointBindings choices (costLetters cost))
  box : Maybe PrintedBox

public export
record CardFace where
  constructor MkFace
  characteristics : Characteristics

public export
abilityChoiceDelta : {bs : Bindings} -> AbilityAt bs -> List Binding
abilityChoiceDelta (Activated _ instr _ _ _ _) = instrChoiceDelta instr
abilityChoiceDelta (Triggered _ _ _ _ _ _ _ _ instr) = instrChoiceDelta instr
abilityChoiceDelta (Static se) = staticChoiceDelta se
abilityChoiceDelta (AlsoForKeywords ab _) = abilityChoiceDelta ab
abilityChoiceDelta (ItalicHead _ ab) = abilityChoiceDelta ab
abilityChoiceDelta (Spell _ instr) = instrChoiceDelta instr
abilityChoiceDelta _ = []

public export
textChoiceDelta : {bs : Bindings} -> AbilitySeq bs -> List Binding
textChoiceDelta [] = []
textChoiceDelta (ab :: abs) = abilityChoiceDelta ab ++ textChoiceDelta abs

public export
jointChoicesOk : List QualitySort -> List Binding -> Bool
jointChoicesOk [] _ = True
jointChoicesOk (q :: qs) made =
  not (countChoice (QSort q) made == 0) && jointChoicesOk qs made

public export
JointChoices : {bs : Bindings} -> List QualitySort -> AbilitySeq bs -> Type
JointChoices qs text = So (jointChoicesOk qs (textChoiceDelta text))

public export
data CharacteristicsLaws : FaceSide -> Characteristics -> Type where
  MkCharacteristicsLaws : {0 side : FaceSide} -> {0 c : Characteristics} ->
                          {auto 0 ln : CardLine c.line} ->
                          {auto 0 sp : CardSupers c.supers} ->
                          {auto 0 tx : CardText c.line c.text} ->
                          {auto 0 ch : CardChapters c.line c.text} ->
                          {auto 0 bx : CardBox side c.line c.text c.box} ->
                          {auto 0 mc : CardCost side c.line c.cost} ->
                          {auto 0 dr : DoorFrame c.text} ->
                          {auto 0 jc : JointChoices c.choices c.text} ->
                          CharacteristicsLaws side c

public export
FaceLaws : FaceSide -> CardFace -> Type
FaceLaws side f = CharacteristicsLaws side f.characteristics

public export
record SharedLineHalf where
  constructor MkSharedHalf
  name : String
  cost : Maybe ManaCost
  text : AbilitySeq (costLetters cost)

public export
data SharedLineHalfLaws : (l : TypeLine) -> Maybe PrintedBox ->
                          SharedLineHalf -> Type where
  MkSharedLineHalfLaws : {0 l : TypeLine} -> {0 box : Maybe PrintedBox} ->
                         {0 h : SharedLineHalf} ->
                         {auto 0 tx : CardText l h.text} ->
                         {auto 0 ch : CardChapters l h.text} ->
                         {auto 0 bx : CardBox Front l h.text box} ->
                         {auto 0 mc : CardCost Front l h.cost} ->
                         SharedLineHalfLaws l box h

public export
adventureInsetOk : TypeLine -> Bool
adventureInsetOk l = elem (spellType "Adventure") l.subs

public export
AdventureInset : TypeLine -> Type
AdventureInset l = So (adventureInsetOk l)

public export
flipHalfOk : TypeLine -> Bool
flipHalfOk l = anyPermanentType l.tys

public export
FlipHalf : TypeLine -> Type
FlipHalf l = So (flipHalfOk l)

||| [CR#711.2a] a closed band, [CR#711.2b] the open last band
public export
data LevelRange : Type where
  LevelBetween : (from : Nat) -> (to : Nat) -> LevelRange
  LevelAtLeast : (from : Nat) -> LevelRange

public export
levelRangeOk : LevelRange -> Bool
levelRangeOk (LevelBetween from to) = from <= to
levelRangeOk (LevelAtLeast _) = True

public export
rangeLow : LevelRange -> Nat
rangeLow (LevelBetween from _) = from
rangeLow (LevelAtLeast from) = from

public export
rangeHigh : LevelRange -> Maybe Nat
rangeHigh (LevelBetween _ to) = Just to
rangeHigh (LevelAtLeast _) = Nothing

public export
rangesOverlap : LevelRange -> LevelRange -> Bool
rangesOverlap a b =
  case (rangeHigh a, rangeHigh b) of
    (Nothing, Nothing) => True
    (Just x, Nothing) => max (rangeLow a) (rangeLow b) <= x
    (Nothing, Just y) => max (rangeLow a) (rangeLow b) <= y
    (Just x, Just y) => max (rangeLow a) (rangeLow b) <= min x y

public export
record LevelBand where
  constructor MkLevelBand
  range : LevelRange
  box : PrintedBox
  text : AbilitySeq []

public export
bandDisjointFrom : LevelRange -> List LevelBand -> Bool
bandDisjointFrom r [] = True
bandDisjointFrom r (c :: cs) = not (rangesOverlap r c.range) && bandDisjointFrom r cs

public export
bandsDisjoint : List LevelBand -> Bool
bandsDisjoint [] = True
bandsDisjoint (b :: bs) = bandDisjointFrom b.range bs && bandsDisjoint bs

public export
levelerPtBox : Maybe PrintedBox -> Bool
levelerPtBox (Just (PtBox _ _)) = True
levelerPtBox _ = False

public export
hasBands : List LevelBand -> Bool
hasBands [] = False
hasBands (_ :: _) = True

public export
levelerFrameOk : TypeLine -> Maybe PrintedBox -> List LevelBand -> Bool
levelerFrameOk l box bands =
  elem Creature l.tys && levelerPtBox box && hasBands bands

public export
data LevelBandLaws : (l : TypeLine) -> LevelBand -> Type where
  MkLevelBandLaws : {0 l : TypeLine} -> {0 b : LevelBand} ->
                    {auto 0 rg : So (levelRangeOk b.range)} ->
                    {auto 0 tx : CardText l b.text} ->
                    {auto 0 ch : CardChapters l b.text} ->
                    {auto 0 bx : CardBox Front l b.text (Just b.box)} ->
                    {auto 0 dr : DoorFrame b.text} ->
                    LevelBandLaws l b

public export
data LevelBandsLaws : (l : TypeLine) -> List LevelBand -> Type where
  NoBands : {0 l : TypeLine} -> LevelBandsLaws l []
  AndBand : {0 l : TypeLine} -> {0 b : LevelBand} -> {0 bs : List LevelBand} ->
            {auto 0 hd : LevelBandLaws l b} ->
            {auto 0 tl : LevelBandsLaws l bs} ->
            LevelBandsLaws l (b :: bs)

||| [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box
public export
record PrototypeAlt where
  constructor MkPrototypeAlt
  cost : ManaCost
  box : PrintedBox

public export
prototypeFrameOk : TypeLine -> Maybe PrintedBox -> Bool
prototypeFrameOk l (Just (PtBox _ _)) = elem Creature l.tys
prototypeFrameOk _ _ = False

public export
data PrototypeAltLaws : (l : TypeLine) -> PrototypeAlt -> Type where
  MkPrototypeAltLaws : {0 l : TypeLine} -> {0 a : PrototypeAlt} ->
                       {auto 0 mc : ManaRun a.cost} ->
                       {auto 0 bx : CardBox {bs = []} Front l [] (Just a.box)} ->
                       PrototypeAltLaws l a

public export
data Card : Type where
  SingleFaced : (face : CardFace) ->
                {auto 0 fl : FaceLaws Front face} -> Card

  Transforming : (front : CardFace) -> (back : CardFace) ->
                 {auto 0 ff : FaceLaws Front front} ->
                 {auto 0 bf : FaceLaws Back back} -> Card

  ModalDfc : (front : CardFace) -> (back : CardFace) ->
             {auto 0 ff : FaceLaws Front front} ->
             {auto 0 bf : FaceLaws Front back} -> Card

  SplitCard : (left : CardFace) -> (right : CardFace) ->
              {auto 0 lf : FaceLaws Front left} ->
              {auto 0 rf : FaceLaws Front right} -> Card

  SharedLineSplit : (line : TypeLine) -> (supers : List Supertype) ->
                    (box : Maybe PrintedBox) ->
                    (left : SharedLineHalf) -> (right : SharedLineHalf) ->
                    {auto 0 ln : CardLine line} ->
                    {auto 0 sp : CardSupers supers} ->
                    {auto 0 pc : So (anyPermanentType line.tys)} ->
                    {auto 0 lh : SharedLineHalfLaws line box left} ->
                    {auto 0 rh : SharedLineHalfLaws line box right} -> Card

  Adventurer : (normal : CardFace) -> (adventure : Characteristics) ->
               {auto 0 nf : FaceLaws Front normal} ->
               {auto 0 sf : CharacteristicsLaws Front adventure} ->
               {auto 0 ai : AdventureInset adventure.line} -> Card

  FlipCard : (normal : CardFace) -> (alternative : Characteristics) ->
             {auto 0 nf : FaceLaws Front normal} ->
             {auto 0 af : CharacteristicsLaws Back alternative} ->
             {auto 0 nh : FlipHalf normal.characteristics.line} ->
             {auto 0 ah : FlipHalf alternative.line} -> Card

  Leveler : (inner : CardFace) -> (bands : List LevelBand) ->
            {auto 0 nf : FaceLaws Front inner} ->
            {auto 0 lv : So (levelerFrameOk inner.characteristics.line
                                            inner.characteristics.box bands)} ->
            {auto 0 bl : LevelBandsLaws inner.characteristics.line bands} ->
            {auto 0 dj : So (bandsDisjoint bands)} -> Card

  Prototype : (inner : CardFace) -> (alt : PrototypeAlt) ->
              {auto 0 nf : FaceLaws Front inner} ->
              {auto 0 pf : So (prototypeFrameOk inner.characteristics.line
                                                inner.characteristics.box)} ->
              {auto 0 al : PrototypeAltLaws inner.characteristics.line alt} -> Card
