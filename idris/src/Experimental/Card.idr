||| The card and face layer: the printed record built over the ability
||| and effect vocabulary.
module Experimental.Card

import public Experimental.Effect

%default total

public export
supersDistinct : List Supertype -> Bool
supersDistinct [] = True
supersDistinct (s :: ss) = not (elem s ss) && supersDistinct ss

public export
CardSupers : List Supertype -> Type
CardSupers ss = So (supersDistinct ss)

||| The three card frames the rules distinguish: a card that can be put
||| onto the battlefield [CR#110.4a], one that is cast and resolves off the
||| stack [CR#112.1], and one that stays in the command zone and is neither
||| [CR#309.2c,311.2,312.2,313.2,314.2,315.3].
public export
data CardClass = PermanentCard | SpellCard | CommandZoneCard

||| `typesCombinable` refuses a permanent type beside a spell type but
||| admits a command-zone type beside either, no rule refusing that, so the
||| order here decides a mixed line: the command-zone type's rule is the one
||| that says where the card stays, and it wins.
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

||| Overgenerated at its zero: a command-zone type beside any other type
||| passes, because no rule refuses it. [CR#300.2] admits more than one card
||| type without excluding these six, and each per-type rule prohibits the
||| card's movement rather than its type line, which [CR#101.2] already
||| resolves. No printed card writes such a line.
public export
typesCombinable : List CardType -> Bool
typesCombinable tys =
  not (anyPermanentType tys && anySpellType tys)
    && (not (elem Kindred tys) || hasNonKindredType tys)

||| Which card class may print a keyword on itself. This is not
||| `keywordStackRegime`'s question re-asked: that field says from which
||| zone a keyword's ability functions, this pair says whether the word
||| can sit on the card at all, and the two would disagree even if every
||| row agreed today. Flash is the case that used to look like a
||| disagreement: [CR#702.8a] says only that flash functions in any zone
||| the card could be played from, and restricts the word to no card
||| type, so an instant or sorcery may carry it. On an instant the word
||| grants what the card already has — redundant, which is not the same
||| as meaningless — and a sorcery carrying it is not redundant at all.
||| The words a card class may not print are each some rule's:
||| [CR#702.84a] and [CR#702.49a] put the card onto the battlefield,
||| which [CR#110.4] denies an instant or sorcery card, so unearth and
||| ninjutsu sit on no spell card, and echo is the same shape --
||| [CR#702.30a] speaks of “this permanent”. [CR#702.34a] permits the
||| flashback cast only if the resulting spell is an instant or sorcery,
||| so flashback is the one word no permanent card prints.
public export
keywordCardOk : CardClass -> KeywordLabel -> Bool
keywordCardOk PermanentCard k = maybe False onPermanentCard (keywordFactsFor k)
keywordCardOk SpellCard k = maybe False onSpellCard (keywordFactsFor k)
keywordCardOk CommandZoneCard k = maybe False onCommandZoneCard (keywordFactsFor k)

public export
staticOnSpellCardOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
-- [CR#113.6g] licenses a "can't be countered" static on any object and
-- says nothing about how broadly its subject may be described.
staticOnSpellCardOk (Deontic _ Forbid ["Counter"] Patient _ _) = True
staticOnSpellCardOk (Deontic _ Forbid ["Copy"] Patient _ _) = True
staticOnSpellCardOk (AltCost _) = True
-- [CR#113.6e]: "An object's ability that restricts or modifies how that
-- particular object can be played or cast functions in any zone from
-- which it could be played or cast and also on the stack." The cast
-- WINDOW is that ability -- 47 supported lines print "Cast this spell
-- only during ..." -- and it is the windowed permission and not a bare
-- one, since what the line restricts is when the only permission there
-- is holds.
staticOnSpellCardOk (OnlyDuring _ _ (Deontic _ Permit ["Cast"] Patient _ _)) = True
staticOnSpellCardOk (OnlyDuring _ _ se) = staticOnSpellCardOk se
staticOnSpellCardOk (Conditionally _ se _) = staticOnSpellCardOk se
staticOnSpellCardOk (OnlyWhile se _ _) = staticOnSpellCardOk se
staticOnSpellCardOk _ = False

public export
classAbilityOk : {0 bs : Bindings} -> CardClass -> AbilityAt bs -> Bool
classAbilityOk PermanentCard (KeywordAbility k _) = keywordCardOk PermanentCard k
classAbilityOk PermanentCard (Activated _ _ _ _ _) = True
classAbilityOk PermanentCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk PermanentCard (Static _) = True
classAbilityOk PermanentCard (AlsoForKeywords ab _) = classAbilityOk PermanentCard ab
classAbilityOk PermanentCard (Spell _) = False
classAbilityOk PermanentCard (AbilityWord _ ab) = classAbilityOk PermanentCard ab
classAbilityOk SpellCard (KeywordAbility k _) = keywordCardOk SpellCard k
classAbilityOk SpellCard (Activated c _ _ _ _) = costOffBattlefield c
classAbilityOk SpellCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk SpellCard (Static se) = staticOnSpellCardOk se
classAbilityOk SpellCard (AlsoForKeywords ab _) = classAbilityOk SpellCard ab
classAbilityOk SpellCard (Spell _) = True
classAbilityOk SpellCard (AbilityWord _ ab) = classAbilityOk SpellCard ab
-- A command-zone card is never a permanent and is never cast
-- [CR#309.2c,311.2,312.2,313.2,314.2,315.3], so it prints no spell
-- ability [CR#113.3a] and its activated ability's cost is read off the
-- battlefield for the same reason a spell card's is [CR#113.6j].
-- [CR#311.4,313.4,314.4] give these cards static, triggered and activated
-- abilities from the command zone. That is the SHARED frame; where one
-- type's own rule reads a narrower list, `commandZoneTypeAbilityOk` below
-- says so, and `cardAbilityOk` asks both.
classAbilityOk CommandZoneCard (KeywordAbility k _) = keywordCardOk CommandZoneCard k
classAbilityOk CommandZoneCard (Activated c _ _ _ _) = costOffBattlefield c
classAbilityOk CommandZoneCard (Triggered _ _ _ _ _ _ _ _ _) = True
classAbilityOk CommandZoneCard (Static _) = True
classAbilityOk CommandZoneCard (AlsoForKeywords ab _) = classAbilityOk CommandZoneCard ab
classAbilityOk CommandZoneCard (Spell _) = False
classAbilityOk CommandZoneCard (AbilityWord _ ab) = classAbilityOk CommandZoneCard ab

||| What ONE command-zone type's own rule licenses, where that rule reads
||| a narrower list than the shared frame. Three of the six read the frame
||| exactly -- [CR#311.4], [CR#313.4] and [CR#314.4] each give a plane, a
||| vanguard and a scheme card "any number of static, triggered, and/or
||| activated abilities" -- so those three have nothing to say here and
||| fall to the catch-all, as does every type that is not a command-zone
||| type at all.
|||
||| Two read narrower. [CR#315.5] gives a conspiracy card static or
||| triggered abilities and stops: no activated ability is licensed from
||| the command zone, where the plane, vanguard and scheme rules each name
||| one. [CR#309.4c] is narrower still -- a dungeon card's abilities are
||| its ROOMS' triggered abilities, whose full text the rule writes out,
||| and it licenses their triggering and nothing else.
|||
||| Phenomena are left OPEN, and deliberately: [CR#312.5] states that each
||| phenomenon card has the encounter trigger, which is a fact about what
||| such cards carry and not a list of what they may carry. It is the only
||| rule about a phenomenon's abilities, and it neither licenses nor
||| refuses a static or an activated one -- and stating no licence is not
||| refusing one, so this cell stands open rather than shut on a rule that
||| does not say it.
|||
||| Measured at ZERO throughout: of the 443 cards in the corpus carrying
||| one of these six types (29 conspiracies, 21 phenomena, 184 planes, 102
||| schemes, 107 vanguards; no dungeon is a card there at all), not one is
||| supported. So both refusals refuse a line no vintage-playable card
||| writes. They are taken anyway because a rule REFUSES them, which is
||| the line the tolerated-overgeneration doctrine draws: an overgeneration
||| no rule refuses is recorded at its zero, and one a rule refuses is
||| closed whatever the count.
public export
commandZoneTypeAbilityOk : {0 bs : Bindings} -> CardType -> AbilityAt bs -> Bool
commandZoneTypeAbilityOk t (AlsoForKeywords ab _) = commandZoneTypeAbilityOk t ab
commandZoneTypeAbilityOk t (AbilityWord _ ab) = commandZoneTypeAbilityOk t ab
commandZoneTypeAbilityOk Conspiracy (Activated _ _ _ _ _) = False
commandZoneTypeAbilityOk Dungeon (KeywordAbility _ _) = False
commandZoneTypeAbilityOk Dungeon (Activated _ _ _ _ _) = False
commandZoneTypeAbilityOk Dungeon (Static _) = False
commandZoneTypeAbilityOk _ _ = True

||| ...over the whole printed line, because [CR#300.2] lets a card name
||| more than one type and each named type's own rule binds the card.
public export
commandZoneTypesAbilityOk : {0 bs : Bindings} -> List CardType -> AbilityAt bs -> Bool
commandZoneTypesAbilityOk [] a = True
commandZoneTypesAbilityOk (t :: ts) a =
  commandZoneTypeAbilityOk t a && commandZoneTypesAbilityOk ts a

||| An ability a card may print: its FRAME's licence and its own TYPES'.
||| The frame is the shared one three of the command-zone rules state
||| verbatim; the types are where the other two narrow it.
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
chapterLineOk subs (AbilityWord _ ab) = chapterLineOk subs ab
chapterLineOk subs (AlsoForKeywords ab _) = chapterLineOk subs ab
chapterLineOk _ _ = True

public export
chapterFrameOk : {0 bs : Bindings} -> List Subtype -> AbilitySeq bs -> Bool
chapterFrameOk subs [] = True
chapterFrameOk subs (a :: as) = chapterLineOk subs a && chapterFrameOk subs as

public export
data PrintedStat = PrintedNum Integer | PrintedStar | PrintedStarPlus Nat
                 | PrintedMinusStar Nat

public export
starred : PrintedStat -> Bool
starred (PrintedNum _) = False
starred PrintedStar = True
starred (PrintedStarPlus _) = True
starred (PrintedMinusStar _) = True

||| The one box in a face's lower right corner. [CR#208.1] prints a creature
||| card's power and toughness there, [CR#209.1] a planeswalker card's
||| starting loyalty, and [CR#210.1] a battle card's defense. [CR#200.1]
||| lists the three as separate parts of a card and all three rules name the
||| same corner, so a face prints at most one of them and its type line
||| decides which.
public export
data PrintedBox : Type where
  PtBox : (pow : PrintedStat) -> (tou : PrintedStat) -> PrintedBox
  LoyaltyBox : (start : PrintedStat) -> PrintedBox
  DefenseBox : (def : PrintedStat) -> PrintedBox

||| The power/toughness pair a corner box writes, if that is what it writes.
||| Only [CR#208.1]'s box has slots a characteristic-defining line can star.
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

||| [CR#207.2c] gives the ability word no rules meaning, so a characteristic-
||| defining line is still one when a word prefixes it: the starred-print gate
||| reads through the wrapper.
public export
abDefinesPt : {0 bs : Bindings} -> AbilityAt bs -> Maybe DefinedSlots
abDefinesPt (Static se) = staticDefinesPt se
abDefinesPt (AbilityWord _ ab) = abDefinesPt ab
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

||| Which corner box a card type demands. [CR#208.1] has a creature card
||| write its two numbers, [CR#209.1] a planeswalker card its loyalty number,
||| and [CR#210.1] a battle card its defense number; a type that writes no
||| number in that corner leaves the box to the rest of the line. A line that
||| names two of the three demands two numbers in one corner and so has no
||| box that fits it.
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
CardText : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs -> Type
CardText l as = So (cardTextOk l.tys as)

public export
CardChapters : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs -> Type
CardChapters l as = So (chapterFrameOk l.subs as)

public export
data CardBox : {0 bs : Bindings} -> TypeLine -> AbilitySeq bs ->
               Maybe PrintedBox -> Type where
  MkCardBox : {0 box : Maybe PrintedBox} ->
              {auto 0 ok : So (cardBoxOk l.tys as box)} -> CardBox l as box

||| Which corner box a card type demands at a face that is not a card of its
||| own. The creature demand stands: [CR#710.1b] lists power and toughness
||| among what a flip card's alternative half prints, and a nonmodal creature
||| back face prints them too. The loyalty demand does not: [CR#209.1] puts the
||| number on each planeswalker *card*, and [CR#712.8a] reads a double-faced
||| card's characteristics off its front face in every zone but the battlefield
||| and the stack, so a planeswalker back face has a loyalty to read without
||| printing one. Both spellings are printed, so the box stays optional there.
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

||| A printed face whose mana cost, where it prints one, is its own rather
||| than another face's. Four layouts print such a face: the single face of a
||| one-faced card, either face of a modal double-faced card [CR#712.3],
||| either half of a split card [CR#709.4b], and both parts of an adventurer
||| card's frame [CR#715.2]. The cost stays a `Maybe` because a land face
||| writes none; what separates these faces from `AltFace` is having a cost
||| slot at all.
|||
||| Its text is typed at ITS OWN cost's letters and nothing else, so no
||| face's words read a binding another face introduced. That is not a
||| convenience: a face's characteristics exist only while that face is
||| the one in play [CR#712.8f,709.3b,715.3b], so there is no moment at
||| which one face's clause could resolve against the other's antecedent.
||| The telescope is `costLetters cost` rather than `[]` because
||| [CR#107.3i] makes every instance of X on the object one value, and the
||| value the caster announced [CR#107.3a] lives in the cost -- so
||| Prosperity's printed "{X}" and its text's "X" are one binding rather
||| than two spellings, which is what the empty telescope could not say.
public export
record CardFace where
  constructor MkFace
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq (costLetters cost)
  box : Maybe PrintedBox

||| A printed face that writes no mana cost at all. Two layouts print one:
||| the back face of a nonmodal double-faced card, whose mana value is read
||| off the front face precisely because the back has no cost of its own
||| [CR#202.3a,202.3b], and a flip card's upside-down half, which shares the
||| card's single printed cost [CR#710.1c]. [CR#710.1b] lists what such a half
||| does print — a name, a text box, a type line, and its power and toughness
||| — and a mana cost is not among them, so this record has no field for one
||| rather than a law refusing one.
public export
record AltFace where
  constructor MkAltFace
  name : String
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq []
  box : Maybe PrintedBox

||| Every card-level law, re-stated at one full printed face. [CR#712.8] gives
||| each face of a double-faced card its own set of characteristics, [CR#709.4c]
||| reads each split half's types and text on its own, and [CR#715.2] does the
||| same for an adventurer card's two frames: the line, supertype, text,
||| chapter, corner-box and cost laws are all face laws, and not one of them is
||| a whole-card law that a second face could escape.
public export
data FaceLaws : CardFace -> Type where
  MkFaceLaws : {0 f : CardFace} ->
               {auto 0 ln : CardLine f.line} ->
               {auto 0 sp : CardSupers f.supers} ->
               {auto 0 tx : CardText f.line f.text} ->
               {auto 0 ch : CardChapters f.line f.text} ->
               {auto 0 bx : CardBox f.line f.text f.box} ->
               {auto 0 mc : CardCost f.line f.cost} ->
               FaceLaws f

||| The same laws at a costless face, with two of them restated for it.
||| `CardCost` is the one card-level law with nothing left to say: [CR#202.3a]
||| and [CR#710.1c] leave the face without a mana cost, so the land-cost gate
||| has no cost to read. The corner-box law is `AltCardBox`, not `CardBox`,
||| because a face is not a card and [CR#209.1] speaks of cards.
public export
data AltFaceLaws : AltFace -> Type where
  MkAltFaceLaws : {0 f : AltFace} ->
                  {auto 0 ln : CardLine f.line} ->
                  {auto 0 sp : CardSupers f.supers} ->
                  {auto 0 tx : CardText f.line f.text} ->
                  {auto 0 ch : CardChapters f.line f.text} ->
                  {auto 0 bx : AltCardBox f.line f.text f.box} ->
                  AltFaceLaws f

||| An adventurer card's inset frame [CR#715.1]. A player chooses to play the
||| card "as an Adventure" [CR#715.3], and Adventure is a spell type
||| [CR#205.3k], so the inset names that spell type. That its line is then an
||| instant or a sorcery is not restated here: `CardLine`'s `subsFitLine`
||| already fits a spell type to no other card type, and a second conjunct
||| saying so would be unreachable.
public export
adventureInsetOk : TypeLine -> Bool
adventureInsetOk l = elem (spellType "Adventure") l.subs

public export
AdventureInset : TypeLine -> Type
AdventureInset l = So (adventureInsetOk l)

||| Either half of a flip card [CR#710.1]. [CR#710.2] applies the alternative
||| characteristics only once the permanent is flipped, and only on the
||| battlefield, so each half names a permanent type — the six [CR#110.4]
||| lists, which is what `anyPermanentType` reads. Excluding spell types is
||| not restated here: `CardLine`'s `typesCombinable` already refuses a line
||| mixing the two.
public export
flipHalfOk : TypeLine -> Bool
flipHalfOk l = anyPermanentType l.tys

public export
FlipHalf : TypeLine -> Type
FlipHalf l = So (flipHalfOk l)

||| One card: its faces, and the rule its layout answers.
|||
||| Five layouts, five constructors — not one record with a layout tag. The
||| layouts disagree about which boxes a face prints, and a uniform face record
||| would state that disagreement away: it would let a flip card's upside-down
||| half [CR#710.1c] or a nonmodal back face [CR#202.3a] carry a mana cost
||| neither prints, and would give a costed face to a layout that has none to
||| give. What the layouts do share — a full face with a cost of its own — is
||| `CardFace`; a costless half is `AltFace`; and every card-level law is
||| re-stated at each face by `FaceLaws` and `AltFaceLaws`, so no law silently
||| applies to one face of two.
|||
||| Meld [CR#712.4] is not among them. A meld pair's combined back face belongs
||| to two cards at once [CR#712.4b], so it is not a second face of one card
||| and does not fit this shape.
public export
data Card : Type where
  ||| A card with a single face; the other side is the normal Magic card back.
  SingleFaced : (face : CardFace) ->
                {auto 0 fl : FaceLaws face} -> Card

  ||| A nonmodal double-faced card [CR#712.2]: abilities on one or both faces
  ||| turn it over. [CR#712.8] gives each face its own characteristics, and
  ||| [CR#202.3a,202.3b] leave the back face costless, reading its mana value
  ||| off the front — so the back is an `AltFace`.
  Transforming : (front : CardFace) -> (back : AltFace) ->
                 {auto 0 ff : FaceLaws front} ->
                 {auto 0 bf : AltFaceLaws back} -> Card

  ||| A modal double-faced card [CR#712.3]: two Magic card faces whose
  ||| characteristics are usually independent of one another. Each is a full
  ||| face — [CR#712.11b] has the caster choose which of them they are casting
  ||| and [CR#712.12] which land face enters — so each face's cost is its own,
  ||| never read off the other the way a nonmodal back face's is [CR#202.3b].
  ||| Two land faces write no cost at all.
  ModalDfc : (front : CardFace) -> (back : CardFace) ->
             {auto 0 ff : FaceLaws front} ->
             {auto 0 bf : FaceLaws back} -> Card

  ||| A split card [CR#709.1]: two faces on one card whose other side is the
  ||| normal card back. [CR#709.4b] gives each half its own mana cost and
  ||| [CR#709.4c] its own card types and text box.
  SplitCard : (left : CardFace) -> (right : CardFace) ->
              {auto 0 lf : FaceLaws left} ->
              {auto 0 rf : FaceLaws right} -> Card

  ||| An adventurer card [CR#715.1]: the normal face, printed as usual, and the
  ||| inset frame whose alternative characteristics the object has while it is
  ||| a spell [CR#715.2].
  Adventurer : (normal : CardFace) -> (inset : CardFace) ->
               {auto 0 nf : FaceLaws normal} ->
               {auto 0 sf : FaceLaws inset} ->
               {auto 0 ai : AdventureInset inset.line} -> Card

  ||| A flip card [CR#710.1]: the right-side-up half writes the card's normal
  ||| characteristics and the upside-down half the alternative ones. [CR#710.1c]
  ||| leaves the one printed mana cost with the card however it is turned, so
  ||| the alternative half is an `AltFace`. The transition itself is the landed
  ||| `Flipped` status, one-way by [CR#710.4]; this constructor adds no second
  ||| verb for it.
  FlipCard : (normal : CardFace) -> (alternative : AltFace) ->
             {auto 0 nf : FaceLaws normal} ->
             {auto 0 af : AltFaceLaws alternative} ->
             {auto 0 nh : FlipHalf normal.line} ->
             {auto 0 ah : FlipHalf alternative.line} -> Card
