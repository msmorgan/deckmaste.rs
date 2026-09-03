module Experimental.Card

import public Experimental.Effect

%default total

public export
CardSupers : List Supertype -> Type
CardSupers ss = So (supersDistinct ss)

public export
data CardClass = PermanentCard | SpellCard | CommandZoneCard

public export
cardClassOf : List CardType -> CardClass
cardClassOf [] = PermanentCard
cardClassOf (t :: ts) =
  if commandZoneType t then CommandZoneCard
  else if spellCardType t then SpellCard
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
anyCommandZoneType : List CardType -> Bool
anyCommandZoneType [] = False
anyCommandZoneType (t :: ts) = commandZoneType t || anyCommandZoneType ts

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
keywordCardOk CommandZoneCard k = maybe False onCommandZoneCard (keywordFactsFor k)

public export
staticOnSpellCardOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
staticOnSpellCardOk (Deontic _ Forbid ["Counter"] Patient _ _ _) = True
staticOnSpellCardOk (Deontic _ Forbid ["Copy"] Patient _ _ _) = True
staticOnSpellCardOk (AltCost This _) = True
staticOnSpellCardOk (CostsToCast This _) = True
staticOnSpellCardOk (AddedCost _ _) = True
staticOnSpellCardOk (OnlyDuring _ _ (Deontic _ Permit ["Cast"] Patient _ _ _)) = True
staticOnSpellCardOk (OnlyDuring _ _ se) = staticOnSpellCardOk se
staticOnSpellCardOk (Conditionally _ se _) = staticOnSpellCardOk se
staticOnSpellCardOk (OnlyWhile se _ _) = staticOnSpellCardOk se
staticOnSpellCardOk _ = False

public export
classAbilityOk : {0 bs : Bindings} -> CardClass -> AbilityAt bs -> Bool
classAbilityOk PermanentCard (KeywordAbility k _) = keywordCardOk PermanentCard k
classAbilityOk PermanentCard (Activated _ _ _ _ _ _) = True
classAbilityOk PermanentCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk PermanentCard (Static _) = True
classAbilityOk PermanentCard (AlsoForKeywords ab _) = classAbilityOk PermanentCard ab
classAbilityOk PermanentCard (Spell _) = False
classAbilityOk PermanentCard (ItalicHead _ ab) = classAbilityOk PermanentCard ab
classAbilityOk PermanentCard MayBeginOnBattlefield = True
classAbilityOk SpellCard (KeywordAbility k _) = keywordCardOk SpellCard k
classAbilityOk SpellCard (Activated c _ _ _ _ _) = costOffBattlefield c
classAbilityOk SpellCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk SpellCard (Static se) = staticOnSpellCardOk se
classAbilityOk SpellCard (AlsoForKeywords ab _) = classAbilityOk SpellCard ab
classAbilityOk SpellCard (Spell _) = True
classAbilityOk SpellCard (ItalicHead _ ab) = classAbilityOk SpellCard ab
classAbilityOk SpellCard MayBeginOnBattlefield = False
classAbilityOk CommandZoneCard (KeywordAbility k _) = keywordCardOk CommandZoneCard k
classAbilityOk CommandZoneCard (Activated c _ _ _ _ _) = costOffBattlefield c
classAbilityOk CommandZoneCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk CommandZoneCard (Static _) = True
classAbilityOk CommandZoneCard (AlsoForKeywords ab _) = classAbilityOk CommandZoneCard ab
classAbilityOk CommandZoneCard (Spell _) = False
classAbilityOk CommandZoneCard (ItalicHead _ ab) = classAbilityOk CommandZoneCard ab
classAbilityOk CommandZoneCard MayBeginOnBattlefield = False

public export
commandZoneTypeAbilityOk : {0 bs : Bindings} -> CardType -> AbilityAt bs -> Bool
commandZoneTypeAbilityOk t (AlsoForKeywords ab _) = commandZoneTypeAbilityOk t ab
commandZoneTypeAbilityOk t (ItalicHead _ ab) = commandZoneTypeAbilityOk t ab
commandZoneTypeAbilityOk Conspiracy (Activated _ _ _ _ _ _) = False
commandZoneTypeAbilityOk Dungeon (KeywordAbility _ _) = False
commandZoneTypeAbilityOk Dungeon (Activated _ _ _ _ _ _) = False
commandZoneTypeAbilityOk Dungeon (Static _) = False
commandZoneTypeAbilityOk _ _ = True

public export
commandZoneTypesAbilityOk : {0 bs : Bindings} -> List CardType -> AbilityAt bs -> Bool
commandZoneTypesAbilityOk [] a = True
commandZoneTypesAbilityOk (t :: ts) a =
  commandZoneTypeAbilityOk t a && commandZoneTypesAbilityOk ts a

public export
cardAbilityOk : {0 bs : Bindings} -> List CardType -> AbilityAt bs -> Bool
cardAbilityOk tys a = classAbilityOk (cardClassOf tys) a && commandZoneTypesAbilityOk tys a

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
staticDefinesPt : {0 bs : Bindings} -> StaticEffect bs -> Maybe DefinedSlots
staticDefinesPt (DefinesPt _ sl _) = Just sl
staticDefinesPt (Conditionally _ se _) = staticDefinesPt se
staticDefinesPt (OnlyWhile se _ _) = staticDefinesPt se
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
boxSuitsType : CardType -> Maybe PrintedBox -> Bool
boxSuitsType Creature (Just (PtBox _ _)) = True
boxSuitsType Creature _ = False
boxSuitsType Planeswalker (Just (LoyaltyBox _)) = True
boxSuitsType Planeswalker _ = False
boxSuitsType Battle (Just (DefenseBox _)) = True
boxSuitsType Battle _ = False
boxSuitsType _ _ = True

public export
boxFitsLine : List CardType -> Maybe PrintedBox -> Bool
boxFitsLine [] box = True
boxFitsLine (t :: ts) box = boxSuitsType t box && boxFitsLine ts box

public export
cardBoxOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs ->
            Maybe PrintedBox -> Bool
cardBoxOk tys text box =
  boxFitsLine tys box &&
  definedSlotsStarred (boxPt box) (textDefines definesPower text)
                                  (textDefines definesToughness text)

public export
cardCostOk : List CardType -> Maybe ManaCost -> Bool
cardCostOk tys cost = case (elem Land tys, cost) of
  (True, Just _) => False
  _ => True

public export
data CardLine : TypeLine -> Type where
  MkCardLine : {auto 0 ne : So (lineNonEmpty l)} ->
               {auto 0 dst : So (typesDistinct l.tys)} ->
               {auto 0 cmb : So (typesCombinable l.tys)} ->
               {auto 0 sf : So (subsFitLine l.subs l.tys)} -> CardLine l

public export
keywordWantsModes : {0 bs : Bindings} -> AbilityAt bs -> Bool
keywordWantsModes (KeywordAbility k _) = k == "Entwine" || k == "Escalate"
keywordWantsModes (ItalicHead _ ab) = keywordWantsModes ab
keywordWantsModes (AlsoForKeywords ab _) = keywordWantsModes ab
keywordWantsModes _ = False

public export
abilityWritesModes : {0 bs : Bindings} -> AbilityAt bs -> Bool
abilityWritesModes (Spell (Modal _ _)) = True
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
data CardBox : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs ->
               Maybe PrintedBox -> Type where
  MkCardBox : {0 box : Maybe PrintedBox} ->
              {auto 0 ok : So (cardBoxOk l.tys as box)} -> CardBox l as box

public export
altBoxSuitsType : CardType -> Maybe PrintedBox -> Bool
altBoxSuitsType Creature (Just (PtBox _ _)) = True
altBoxSuitsType Creature _ = False
altBoxSuitsType Planeswalker (Just (LoyaltyBox _)) = True
altBoxSuitsType Planeswalker Nothing = True
altBoxSuitsType Planeswalker _ = False
altBoxSuitsType Battle (Just (DefenseBox _)) = True
altBoxSuitsType Battle _ = False
altBoxSuitsType _ _ = True

public export
altBoxFitsLine : List CardType -> Maybe PrintedBox -> Bool
altBoxFitsLine [] box = True
altBoxFitsLine (t :: ts) box = altBoxSuitsType t box && altBoxFitsLine ts box

public export
altBoxOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs ->
           Maybe PrintedBox -> Bool
altBoxOk tys text box =
  altBoxFitsLine tys box &&
  definedSlotsStarred (boxPt box) (textDefines definesPower text)
                                  (textDefines definesToughness text)

public export
data AltCardBox : TypeLine -> AbilitySeq [] -> Maybe PrintedBox -> Type where
  MkAltCardBox : {0 box : Maybe PrintedBox} ->
                 {auto 0 ok : So (altBoxOk l.tys as box)} -> AltCardBox l as box

public export
CardCost : TypeLine -> Maybe ManaCost -> Type
CardCost l c = So (cardCostOk l.tys c)

public export
record CardFace where
  constructor MkFace
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq (costLetters cost)
  box : Maybe PrintedBox

public export
record AltFace where
  constructor MkAltFace
  name : String
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq []
  box : Maybe PrintedBox

public export
jointBindings : List QualitySort -> Bindings -> Bindings
jointBindings [] bs = bs
jointBindings (q :: qs) bs = qualityB q :: jointBindings qs bs

public export
abilityChoiceDelta : {bs : Bindings} -> AbilityAt bs -> List Binding
abilityChoiceDelta (Activated _ eff _ _ _ _) = effChoiceDelta eff
abilityChoiceDelta (Triggered _ _ _ _ _ _ _ _ eff) = effChoiceDelta eff
abilityChoiceDelta (Static se) = staticChoiceDelta se
abilityChoiceDelta (AlsoForKeywords ab _) = abilityChoiceDelta ab
abilityChoiceDelta (ItalicHead _ ab) = abilityChoiceDelta ab
abilityChoiceDelta (Spell eff) = effChoiceDelta eff
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
record JointFace where
  constructor MkJointFace
  choices : List QualitySort
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq (jointBindings choices (costLetters cost))
  box : Maybe PrintedBox

public export
data FaceLaws : CardFace -> Type where
  MkFaceLaws : {0 f : CardFace} ->
               {auto 0 ln : CardLine f.line} ->
               {auto 0 sp : CardSupers f.supers} ->
               {auto 0 tx : CardText f.line f.text} ->
               {auto 0 ch : CardChapters f.line f.text} ->
               {auto 0 bx : CardBox f.line f.text f.box} ->
               {auto 0 mc : CardCost f.line f.cost} ->
               {auto 0 dr : DoorFrame f.text} ->
               FaceLaws f

public export
data AltFaceLaws : AltFace -> Type where
  MkAltFaceLaws : {0 f : AltFace} ->
                  {auto 0 ln : CardLine f.line} ->
                  {auto 0 sp : CardSupers f.supers} ->
                  {auto 0 tx : CardText f.line f.text} ->
                  {auto 0 ch : CardChapters f.line f.text} ->
                  {auto 0 bx : AltCardBox f.line f.text f.box} ->
                  {auto 0 dr : DoorFrame f.text} ->
                  AltFaceLaws f

public export
data JointFaceLaws : JointFace -> Type where
  MkJointFaceLaws : {0 f : JointFace} ->
                    {auto 0 ln : CardLine f.line} ->
                    {auto 0 sp : CardSupers f.supers} ->
                    {auto 0 tx : CardText f.line f.text} ->
                    {auto 0 ch : CardChapters f.line f.text} ->
                    {auto 0 bx : CardBox f.line f.text f.box} ->
                    {auto 0 mc : CardCost f.line f.cost} ->
                    {auto 0 dr : DoorFrame f.text} ->
                    {auto 0 jc : JointChoices f.choices f.text} ->
                    JointFaceLaws f

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
                         {auto 0 bx : CardBox l h.text box} ->
                         {auto 0 mc : CardCost l h.cost} ->
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

public export
data Card : Type where
  SingleFaced : (face : CardFace) ->
                {auto 0 fl : FaceLaws face} -> Card

  JointSingleFaced : (face : JointFace) ->
                     {auto 0 fl : JointFaceLaws face} -> Card

  Transforming : (front : CardFace) -> (back : AltFace) ->
                 {auto 0 ff : FaceLaws front} ->
                 {auto 0 bf : AltFaceLaws back} -> Card

  ModalDfc : (front : CardFace) -> (back : CardFace) ->
             {auto 0 ff : FaceLaws front} ->
             {auto 0 bf : FaceLaws back} -> Card

  SplitCard : (left : CardFace) -> (right : CardFace) ->
              {auto 0 lf : FaceLaws left} ->
              {auto 0 rf : FaceLaws right} -> Card

  SharedLineSplit : (line : TypeLine) -> (supers : List Supertype) ->
                    (box : Maybe PrintedBox) ->
                    (left : SharedLineHalf) -> (right : SharedLineHalf) ->
                    {auto 0 ln : CardLine line} ->
                    {auto 0 sp : CardSupers supers} ->
                    {auto 0 pc : So (anyPermanentType line.tys)} ->
                    {auto 0 lh : SharedLineHalfLaws line box left} ->
                    {auto 0 rh : SharedLineHalfLaws line box right} -> Card

  Adventurer : (normal : CardFace) -> (inset : CardFace) ->
               {auto 0 nf : FaceLaws normal} ->
               {auto 0 sf : FaceLaws inset} ->
               {auto 0 ai : AdventureInset inset.line} -> Card

  FlipCard : (normal : CardFace) -> (alternative : AltFace) ->
             {auto 0 nf : FaceLaws normal} ->
             {auto 0 af : AltFaceLaws alternative} ->
             {auto 0 nh : FlipHalf normal.line} ->
             {auto 0 ah : FlipHalf alternative.line} -> Card
