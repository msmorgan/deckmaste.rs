module Experimental.Macros

import Data.List
import public Experimental

%default total

public export
It : (pl : Plurality) -> {auto 0 ok : countReach Bare pl bs = 1} -> Noun bs Object
It pl = Pro Bare pl Whole

public export
ItVerbed : (v : VerbLabel) -> (pl : Plurality) -> {auto 0 kn : KnownAct v} ->
           {auto 0 ok : countReach (Stamped v) pl bs = 1} -> Noun bs Object
ItVerbed v pl = Pro (Stamped v) pl Whole

public export
They : {auto 0 ok : countReach (Word PlayerW) OneOf bs = 1} -> Noun bs Player
They = Pro (Word PlayerW) OneOf Whole

public export
That : (w : NounWord) -> (pl : Plurality) ->
       {auto 0 ok : countReach (Word w) pl bs = 1} ->
       Noun bs (kindOfW w)
That w pl = Pro (Word w) pl Whole

public export
thatTurn : {auto 0 ok : countReach ThatTurn OneOf bs = 1} -> Noun bs TurnRef
thatTurn = Pro ThatTurn OneOf Whole

public export
TheVerbed : (v : VerbLabel) -> (w : NounWord) -> (marking : VerbedMarking) ->
            (pl : Plurality) ->
            {auto 0 ok : countReach (Verbed v w marking) pl bs = 1} ->
            {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
TheVerbed v w marking pl = Pro (Verbed v w marking) pl Whole

public export
exactly : Nat -> Quantity bs
exactly n = Range (Just n) (Just n)

public export
upTo : Nat -> Quantity bs
upTo n = Range Nothing (Just n)

public export
anyNumber : Quantity bs
anyNumber = Range Nothing Nothing

public export
atLeast : Nat -> Quantity bs
atLeast n = Range (Just n) Nothing

public export
oneThrough : Nat -> Quantity bs
oneThrough n = Range (Just 1) (Just n)

public export
EntersTapped : TokenRider bs
EntersTapped = EntersAs Tapped

public export
MkToken : {0 bs : Bindings} ->
          Maybe (p : Amount bs ** Amount (amtIntro p)) -> List Color ->
          TypeLine -> List Ability -> Maybe String -> TokenChars bs
MkToken {bs} pt cs l abs nm = MkTokenChars {bs} pt cs [] l abs nm []

public export
MkSupertypedToken : {0 bs : Bindings} ->
                    Maybe (p : Amount bs ** Amount (amtIntro p)) ->
                    List Color -> List Supertype ->
                    TypeLine -> List Ability -> Maybe String -> TokenChars bs
MkSupertypedToken {bs} pt cs sups l abs nm = MkTokenChars {bs} pt cs sups l abs nm []

public export
target : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
         {auto 0 tk : Targetable k} -> Noun bs k
target p = Described (TargetDet (Range (Just 1) (Just 1))) p
                     {ph} {ok = (MaxAtLeastOne, Oh, tk)}

public export
each : (p : Predicate bs k) -> {auto ph : Phrasal k} -> Noun bs k
each p = Described EachDet p {ph} {ok = ()}

public export
allOf : (p : Predicate bs k) -> {auto ph : Phrasal k} -> Noun bs k
allOf p = Described AllDet p {ph} {ok = ()}

public export
everyObject : Noun bs Object
everyObject = allOf (And [])

public export
bare : (p : Predicate bs k) -> {auto ph : Phrasal k} -> Noun bs k
bare p = Described BareDet p {ph} {ok = ()}

public export
countOf : {bs : Bindings} -> {k : Kind} -> (p : Predicate bs k) ->
          {auto ph : Phrasal k} ->
          {auto 0 pl : nounPlur (bare p {ph}) = ManyOf} -> Amount bs
countOf p = CountOf (bare p {ph}) {pl} {cg = Oh}

public export
aggregate : {bs : Bindings} -> {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
            (p : Predicate bs k) ->
            {auto ph : Phrasal k} -> {auto 0 sc : projScope ax = k} ->
            {auto 0 pl : nounPlur (bare p {ph}) = ManyOf} -> Amount bs
aggregate op ax p = Aggregate op ax (bare p {ph}) {sc} {pl}

public export
exists : {k : Kind} -> (p : Predicate bs k) -> {auto ph : Phrasal k} -> Condition bs
exists p = Exists (bare p {ph}) {ex = Oh}

public export
the : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
      {auto 0 ok : detOk TheDet p} -> Noun bs k
the p = Described TheDet p {ph} {ok}

public export
counted : (q : Quantity bs) -> (p : Predicate bs k) -> {auto ph : Phrasal k} ->
          {auto 0 ok : detOk (CountDet q Nothing) p} -> Noun bs k
counted q p = Described (CountDet q Nothing) p {ph} {ok}

public export
anyTarget : Predicate bs (Object \/ Player)
anyTarget = Joined (Or [HasType Creature, HasType Planeswalker, HasType Battle])
                   AnyPlayer

public export
youAnd : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youAnd n = Both You n

public export
youOr : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youOr n = EitherOf You n

public export
thatJoin : {auto 0 ok : countReach (Word JoinW) OneOf bs = 1} -> Noun bs (Object \/ Player)
thatJoin = That JoinW OneOf {ok}

public export
chosenPlayer : {auto 0 ok : choiceRefOk TheChoice (countChoice PlayerC bs)} ->
               Predicate bs Player
chosenPlayer = ChosenPlayer TheChoice {ok}

public export
theLastChosenPlayer : {auto 0 ok : choiceRefOk TheLatestChoice (countChoice PlayerC bs)} ->
                      Predicate bs Player
theLastChosenPlayer = ChosenPlayer TheLatestChoice {ok}

public export
ofChosen : (q : QualitySort) ->
           {auto 0 ok : choiceRefOk TheChoice (countChoice (QSort q) bs)} ->
           {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
ofChosen q = OfChosen TheChoice q {ok} {read}

public export
ofTheLastChosen : (q : QualitySort) ->
                  {auto 0 ok : choiceRefOk TheLatestChoice (countChoice (QSort q) bs)} ->
                  {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
ofTheLastChosen q = OfChosen TheLatestChoice q {ok} {read}

public export
thatColor : {auto 0 ok : choiceRefOk TheChoice (countChoice (QSort Color) bs)} ->
            {auto 0 rd : ChosenQualityRead Color} -> ColorTerm bs
thatColor = ThatColor TheChoice {ok} {rd}

public export
theLastChosenColor : {auto 0 ok : choiceRefOk TheLatestChoice (countChoice (QSort Color) bs)} ->
                     {auto 0 rd : ChosenQualityRead Color} -> ColorTerm bs
theLastChosenColor = ThatColor TheLatestChoice {ok} {rd}

public export
chosenNumber : {auto 0 ok : choiceRefOk TheChoice (countChoice (QSort Number) bs)} ->
               Amount bs
chosenNumber = ChosenNumber TheChoice {ok}

public export
theLastChosenNumber : {auto 0 ok : choiceRefOk TheLatestChoice (countChoice (QSort Number) bs)} ->
                      Amount bs
theLastChosenNumber = ChosenNumber TheLatestChoice {ok}

public export
controllerOf : {k : Kind} -> (n : Noun bs k) ->
               {auto 0 ck : So (possessorKind ControllerAx k)} -> Noun bs Player
controllerOf n = PossessorOf ControllerAx n {ck}

public export
ownerOf : {k : Kind} -> (n : Noun bs k) ->
          {auto 0 ck : So (possessorKind OwnerAx k)} -> Noun bs Player
ownerOf n = PossessorOf OwnerAx n {ck}

public export
splitOverPlaneswalker : {bs : Bindings} ->
                        {auto 0 ck : countReach (UnionHalf (TypeW Planeswalker)) OneOf bs = 1} ->
                        {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                        Noun bs Player
splitOverPlaneswalker =
  EitherOf (Pro (UnionHalf PlayerW) OneOf Whole {ok = pk})
           (Macros.controllerOf
             (Pro (UnionHalf (TypeW Planeswalker)) OneOf Whole {ok = ck}))

public export
splitOverPermanent : {bs : Bindings} ->
                     {auto 0 ck : countReach (UnionHalf PermanentW) OneOf bs = 1} ->
                     {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                     Noun bs Player
splitOverPermanent =
  EitherOf (Pro (UnionHalf PlayerW) OneOf Whole {ok = pk})
           (Macros.controllerOf (Pro (UnionHalf PermanentW) OneOf Whole {ok = ck}))


public export
battlefieldZ : ZoneExpr bs
battlefieldZ = ZoneAt Battlefield BareScope

public export
exileZ : ZoneExpr bs
exileZ = ZoneAt Exile BareScope

public export
handZ : ZoneExpr bs
handZ = ZoneAt Hand BareScope

public export
libraryZ : ZoneExpr bs
libraryZ = ZoneAt Library BareScope

public export
commandZ : ZoneExpr bs
commandZ = ZoneAt Command BareScope

public export
spell : Predicate bs Object
spell = IsSpell

public export
frontFace : (name : String) -> (cost : Maybe ManaCost) ->
            (supers : List Supertype) -> (line : TypeLine) ->
            (text : AbilitySeq (costLetters cost)) -> (box : Maybe PrintedBox) ->
            CardFace
frontFace name cost supers line text box =
  MkFace (MkCharacteristics name cost [] supers line text box)

public export
backFace : (name : String) -> (supers : List Supertype) -> (line : TypeLine) ->
           (text : AbilitySeq []) -> (box : Maybe PrintedBox) -> CardFace
backFace name supers line text box =
  MkFace (MkCharacteristics name Nothing [] supers line text box)

||| [CR#710.1] a flip card's alternative characteristics, [CR#715.2] an Adventure's
public export
alternative : (name : String) -> (cost : Maybe ManaCost) ->
              (supers : List Supertype) -> (line : TypeLine) ->
              (text : AbilitySeq (costLetters cost)) -> (box : Maybe PrintedBox) ->
              Characteristics
alternative name cost supers line text box =
  MkCharacteristics name cost [] supers line text box

public export
cardOf : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
         (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
         (box : Maybe PrintedBox) ->
         {auto 0 ln : CardLine line} ->
         {auto 0 sp : CardSupers supers} ->
         {auto 0 tx : CardText line text} ->
         {auto 0 ch : CardChapters line text} ->
         {auto 0 bx : CardBox Front line text box} ->
         {auto 0 mc : CardCost Front line cost} ->
         {auto 0 dr : DoorFrame text} ->
         Card
cardOf name cost supers line text box =
  SingleFaced (MkFace (MkCharacteristics name cost [] supers line text box))
              {fl = MkCharacteristicsLaws {ln} {sp} {tx} {ch} {bx} {mc} {dr}}

public export
printedBox : Maybe (Integer, Integer) -> Maybe PrintedBox
printedBox Nothing = Nothing
printedBox (Just (p, t)) = Just (PtBox (PrintedNum p) (PrintedNum t))

public export
loyaltyBox : Integer -> Maybe PrintedBox
loyaltyBox n = Just (LoyaltyBox (PrintedNum n))

public export
defenseBox : Integer -> Maybe PrintedBox
defenseBox n = Just (DefenseBox (PrintedNum n))

public export
card : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
       (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
       (stats : Maybe (Integer, Integer)) ->
       {auto 0 fl : FaceLaws Front
                    (MkFace (MkCharacteristics name cost [] supers line text
                                               (printedBox stats)))} ->
       Card
card name cost supers line text stats =
  SingleFaced (MkFace (MkCharacteristics name cost [] supers line text
                                         (printedBox stats))) {fl}

public export
levelBand : (range : LevelRange) -> (pow : Integer) -> (tou : Integer) ->
            (text : AbilitySeq []) -> LevelBand
levelBand range pow tou text =
  MkLevelBand range (PtBox (PrintedNum pow) (PrintedNum tou)) text

public export
leveler : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
          (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
          (stats : Maybe (Integer, Integer)) -> (bands : List LevelBand) ->
          {auto 0 nf : FaceLaws Front
                       (MkFace (MkCharacteristics name cost [] supers line text
                                                  (printedBox stats)))} ->
          {auto 0 lv : So (levelerFrameOk line (printedBox stats) bands)} ->
          {auto 0 bl : LevelBandsLaws line bands} ->
          {auto 0 dj : So (bandsDisjoint bands)} ->
          Card
leveler name cost supers line text stats bands =
  Leveler (MkFace (MkCharacteristics name cost [] supers line text
                                     (printedBox stats))) bands
          {nf} {lv} {bl} {dj}

public export
prototypeAlt : (cost : ManaCost) -> (pow : Integer) -> (tou : Integer) ->
               PrototypeAlt
prototypeAlt cost pow tou =
  MkPrototypeAlt cost (PtBox (PrintedNum pow) (PrintedNum tou))

public export
prototype : (name : String) -> (cost : Maybe ManaCost) ->
            (supers : List Supertype) -> (line : TypeLine) ->
            (text : AbilitySeq (costLetters cost)) ->
            (stats : Maybe (Integer, Integer)) -> (alt : PrototypeAlt) ->
            {auto 0 nf : FaceLaws Front
                         (MkFace (MkCharacteristics name cost [] supers line text
                                                    (printedBox stats)))} ->
            {auto 0 pf : So (prototypeFrameOk line (printedBox stats))} ->
            {auto 0 al : PrototypeAltLaws line alt} ->
            Card
prototype name cost supers line text stats alt =
  Prototype (MkFace (MkCharacteristics name cost [] supers line text
                                       (printedBox stats))) alt
            {nf} {pf} {al}

public export
graveyardZ : ZoneExpr bs
graveyardZ = ZoneAt Graveyard BareScope

public export
handOf : (n : Noun bs Player) -> ZoneExpr bs
handOf n = ZoneAt Hand (PossessedBy n {ps = HandIsOwned})

public export
graveyardOf : (n : Noun bs Player) -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (PossessedBy n {ps = GraveyardIsOwned})


public export
thisCreature : Noun bs Object
thisCreature = AsType Creature This Nothing

public export
thisArtifact : Noun bs Object
thisArtifact = AsType Artifact This Nothing

public export
thisEnchantment : Noun bs Object
thisEnchantment = AsType Enchantment This Nothing

public export
thisLand : Noun bs Object
thisLand = AsType Land This Nothing

public export
thisPlaneswalker : Noun bs Object
thisPlaneswalker = AsType Planeswalker This Nothing

public export
thisAura : Noun bs Object
thisAura = AsType Enchantment This (Just (enchantmentType "Aura"))

public export
thisPermanent : Noun bs Object
thisPermanent = AsMarker PermanentMarker This

public export
thisSpell : Noun bs Object
thisSpell = AsMarker SpellMarker This

public export
thisEquipment : Noun bs Object
thisEquipment = AsType Artifact This (Just (artifactType "Equipment"))

public export
thisSaga : Noun bs Object
thisSaga = AsType Enchantment This (Just (enchantmentType "Saga"))

public export
thisClass : Noun bs Object
thisClass = AsType Enchantment This (Just (enchantmentType "Class"))

public export
thisCase : Noun bs Object
thisCase = AsType Enchantment This (Just (enchantmentType "Case"))

public export
thisRoom : Noun bs Object
thisRoom = AsType Enchantment This (Just (enchantmentType "Room"))

public export
thisVehicle : Noun bs Object
thisVehicle = AsType Artifact This (Just (artifactType "Vehicle"))

public export
thisSpacecraft : Noun bs Object
thisSpacecraft = AsType Artifact This (Just (artifactType "Spacecraft"))

public export
exiledWithThisArtifact : Predicate bs Object
exiledWithThisArtifact = ExiledWith thisArtifact

public export
creature : Predicate bs Object
creature = HasType Creature

public export
artifact : Predicate bs Object
artifact = HasType Artifact

public export
source : Predicate bs Object
source = IsSource

public export
enchantment : Predicate bs Object
enchantment = HasType Enchantment

public export
land : Predicate bs Object
land = HasType Land

public export
hasBasePt : (n : Noun bs Object) -> (pow : Amount (selfSubjIntro n)) ->
            (tou : Amount (amtIntro pow)) ->
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> StaticSpec bs
hasBasePt n pow tou = Gets Sets n (PtUp pow) (PtUp tou) {ok}

public export
unblocked : Predicate bs Object
unblocked = Not Blocked

public export
times : (per : Nat) -> (a : Amount bs) ->
        {auto 0 nz : So (amtNonZero (Lit {bs} per))} -> Amount bs
times per a = TimesOf (Lit per) a {nz}

public export
colorsSpentToCast : (n : Noun bs Object) ->
                    {auto 0 sb : PaidSubject n} ->
                    {auto 0 one : nounPlur n = OneOf} -> Amount bs
colorsSpentToCast n = Paid ColorsSpent n {sb} {one}

public export
manaValueSpentToCast : (n : Noun bs Object) ->
                       {auto 0 sb : PaidSubject n} ->
                       {auto 0 one : nounPlur n = OneOf} -> Amount bs
manaValueSpentToCast n = Paid ManaValueSpent n {sb} {one}

public export
timesPaid : (which : PaidCostName) -> (n : Noun bs Object) ->
            {auto 0 nf : PaidFacetNamed (TimesPaid which)} ->
            {auto 0 sb : PaidSubject n} ->
            {auto 0 one : nounPlur n = OneOf} -> Amount bs
timesPaid which n = Paid (TimesPaid which) n {nf} {sb} {one}

public export
paidCostRead : (which : PaidCostName) -> (window : Maybe Lookback) ->
               (n : Noun bs Object) ->
               {auto 0 nf : PaidFacetNamed (PaidCostReadback which window)} ->
               {auto 0 sb : PaidSubject n} ->
               {auto 0 one : nounPlur n = OneOf} -> Amount bs
paidCostRead which window n = Paid (PaidCostReadback which window) n {nf} {sb} {one}

public export
costWasPaid : (which : PaidCostName) -> (window : Maybe Lookback) ->
              (n : Noun bs Object) ->
              {auto 0 nf : PaidFacetNamed (PaidCostReadback which window)} ->
              {auto 0 sb : PaidSubject n} ->
              {auto 0 one : nounPlur n = OneOf} -> Condition bs
costWasPaid which window n =
  CompareAmt (paidCostRead which window n {nf} {sb} {one}) AtLeast (Lit 1)

public export
coloredManaSpentToCast : (n : Noun bs Object) ->
                         {auto 0 sb : PaidSubject n} ->
                         {auto 0 one : nounPlur n = OneOf} -> Condition bs
coloredManaSpentToCast n =
  CompareAmt (colorsSpentToCast n {sb} {one}) AtLeast (Lit 1)

public export
noColoredManaSpentToCast : (n : Noun bs Object) ->
                           {auto 0 sb : PaidSubject n} ->
                           {auto 0 one : nounPlur n = OneOf} -> Condition bs
noColoredManaSpentToCast n =
  CompareAmt (colorsSpentToCast n {sb} {one}) Eq (Lit 0)

public export
noManaSpentToCast : (n : Noun bs Object) ->
                    {auto 0 sb : PaidSubject n} ->
                    {auto 0 one : nounPlur n = OneOf} -> Condition bs
noManaSpentToCast n =
  CompareAmt (manaValueSpentToCast n {sb} {one}) Eq (Lit 0)

public export
theRest : (k : Kind) -> {auto 0 ok : So (theRestOk k bs)} -> Noun bs k
theRest k = TheRest k ManyOf {ok}

public export
theOther : (k : Kind) -> {auto 0 ok : So (theOtherOk k bs)} -> Noun bs k
theOther k = TheRest k OneOf {ok}

public export
castBy : (n : Noun bs Player) -> {auto 0 ps : SoleHolder n} -> Predicate bs Object
castBy n = CastBy n Nothing {ps}

public export
nthCastBy : (ord : Ordinal) -> (n : Noun bs Player) -> (per : RankPeriod) ->
            {auto 0 ps : SoleHolder n} -> Predicate bs Object
nthCastBy ord n per = CastBy n (Just (ord, per)) {ps}

public export
monocolored : Predicate bs Object
monocolored = ColorCount Eq 1

public export
multicolored : Predicate bs Object
multicolored = ColorCount AtLeast 2

public export
instant : Predicate bs Object
instant = HasType Instant

public export
sorcery : Predicate bs Object
sorcery = HasType Sorcery

public export
instantOrSorcery : Predicate bs Object
instantOrSorcery = Or [instant, sorcery]

public export
creatureYouControl : Predicate bs Object
creatureYouControl = And [creature, HasPossessor ControllerAx You]

public export
creatureYouDontControl : Predicate bs Object
creatureYouDontControl = And [creature, Not (HasPossessor ControllerAx You)]

public export
creatureYourOpponentsControl : Predicate bs Object
creatureYourOpponentsControl = And [creature, HasPossessor ControllerAx (PlayerGroup YourOpponents)]

public export
tapped : Predicate bs Object
tapped = HasStatus Tapped

public export
untapped : Predicate bs Object
untapped = HasStatus Untapped

public export
faceDown : Predicate bs Object
faceDown = HasStatus FaceDown

public export
nontoken : Predicate bs Object
nontoken = Not IsToken

public export
emblem : Predicate bs Object
emblem = IsEmblem

public export
copyOfACard : Predicate bs Object
copyOfACard = IsCopyOfACard

public export
cardOnTheStack : Predicate bs Object
cardOnTheStack = And [IsCard, IsSpell]

public export
tokenOnTheBattlefield : Predicate bs Object
tokenOnTheBattlefield = And [IsToken, Permanent]


public export
a : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
    Noun bs k
a p = Described (ADet Unmarked) p {ph} {ok = ()}

public export
aTheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
               {auto 0 ch : countChoosers bs = 1} ->
               Noun bs k
aTheirChoice p = Described (ADet (TheirChoice {ch})) p {ph} {ok = ()}

public export
aYourChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
              Noun bs k
aYourChoice p = Described (ADet YourChoice) p {ph} {ok = ()}

public export
withDifferentNames : (grp : Noun bs Object) ->
                     {auto 0 cm : CountedMention grp} ->
                     {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
withDifferentNames grp = NamesAgree DifferentNames grp {cm} {pl}

public export
withTheSameName : (grp : Noun bs Object) ->
                  {auto 0 cm : CountedMention grp} ->
                  {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
withTheSameName grp = NamesAgree SameName grp {cm} {pl}

public export
aAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            Noun bs k
aAtRandom p = Described (ADet AtRandom) p {ph} {ok = ()}

public export
countedAtRandom : (q : Quantity bs) -> (p : Predicate bs k) ->
                  {auto ph : Phrasal k} ->
                  {auto 0 ok : detOk (CountDet q (Just AtRandom)) p} -> Noun bs k
countedAtRandom q p = Described (CountDet q (Just AtRandom)) p {ph} {ok}

public export
anOpponent : Noun bs Player
anOpponent = a Opponent

public export
anyOtherTarget : {auto 0 ok : anyTargeted (Object \/ Player) bs = True} ->
                 Predicate bs (Object \/ Player)
anyOtherTarget = And [anyTarget, Other {ok = eqToSo ok}] {oa = eqToSo ok}


public export
otherCreature : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                Predicate bs Object
otherCreature n = And [creature, OtherThan n {ca}] {oa = eqToSo ty}

public export
otherCreatureYouControl : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                          {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                          Predicate bs Object
otherCreatureYouControl n = And [creature, HasPossessor ControllerAx You, OtherThan n {ca}] {oa = eqToSo ty}

public export
otherPlayer : Predicate bs Player
otherPlayer = And [AnyPlayer, OtherThan You]


public export
forEach : {bs : Bindings} -> {k : Kind} -> (n : Nat) -> (p : Predicate bs k) ->
          {auto ph : Phrasal k} ->
          {auto 0 pl : nounPlur (bare p {ph}) = ManyOf} ->
          {auto 0 nz : So (amtNonZero (Lit {bs} n))} ->
          Amount bs
forEach n p = times n (countOf p {ph} {pl}) {nz}

public export
move : {k : Kind} -> (what : Noun bs k) -> (to : ZoneExpr (nomIntro what)) ->
       {auto 0 mk : Movable what} ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur what) to} ->
       {auto 0 pl : Placeable (nounTy what) (zoneSort to)} -> Instruction bs
move what to = Move what to [] {mk} {ok} {arr} {pl}

public export
destroy : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
          {auto 0 mk : Movable n} -> Instruction bs
destroy n = Enact Nothing "Destroy" (Move n graveyardZ [] {mk})

public export
exile : {k : Kind} -> (agent : Noun bs Player) -> (n : Noun (agentIntro agent) k) ->
        {auto 0 mk : Movable n} ->
        {auto 0 ke : EnactKeepsOuter (Just agent) (Move n Macros.exileZ [] {mk})} ->
        Instruction bs
exile agent n = Enact (Just agent) "Exile" (Move n exileZ [] {mk}) {ke}

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) -> {auto 0 mk : Movable n} -> Instruction bs
exileWithCounters n amt kind =
  Enact Nothing "Exile"
        (Move n exileZ [WithCounters amt (PrintedKind kind) Fresh] {mk})

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 mk : Movable n} ->
  Instruction bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ [ Under who {one = OneController {one}}
                      , WithCounters amt (PrintedKind kind) Fresh ] {pl} {mk}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     {auto 0 mk : Movable n} ->
                     Instruction bs
putOntoBattlefield n = Move n battlefieldZ [] {pl} {mk}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           {auto 0 mk : Movable n} ->
                           Instruction bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ [EntersTapped] {pl} {mk}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 mk : Movable n} ->
  Instruction bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ [EntersTapped, EntersAttacking NoDefender] {pl} {mk}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 mk : Movable n} ->
  Instruction bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ [Under You] {pl} {mk}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
            {auto 0 mk : Movable n} ->
            {auto 0 ke : EnactKeepsOuter (Just agent) (Move n Macros.graveyardZ [] {mk})} ->
            Instruction bs
sacrifice agent n =
  Enact (Just agent) "Sacrifice" (Move n graveyardZ [] {mk}) {ke}

public export
sacrificeIt : (agent : Noun bs Player) ->
              {auto 0 ok : countReach (AtSlot PermanentSlot) OneOf (agentIntro agent) = 1} ->
              {auto 0 zn : ZoneIs (zoneOfReach (AtSlot PermanentSlot) OneOf (agentIntro agent)) Battlefield} ->
              {auto 0 ke : EnactKeepsOuter (Just agent)
                             (Move (Pro (AtSlot PermanentSlot) OneOf Whole {ok})
                                   Macros.graveyardZ [])} ->
              Instruction bs
sacrificeIt agent =
  sacrifice agent (Pro (AtSlot PermanentSlot) OneOf Whole {ok}) {ok = zn} {ke}

public export
discard : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
          {auto 0 dk : DiscardOk n} ->
          {auto 0 mk : Movable n} ->
          {auto 0 ke : EnactKeepsOuter (Just agent) (Move n Macros.graveyardZ [] {mk})} ->
          Instruction bs
discard agent n =
  Enact (Just agent) "Discard" (Move n graveyardZ [] {mk}) {ke}

public export
tap : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
      Instruction bs
tap n = Enact Nothing "Tap" (SetStatus Tapped n)

public export
untap : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
        Instruction bs
untap n = Enact Nothing "Untap" (SetStatus Untapped n)

public export
transform : (n : Noun bs Object) ->
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> Instruction bs
transform n = Enact Nothing "Transform" (TurnOver n)

public export
meldInto : (n : Noun bs Object) -> (into : String) ->
           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
           {auto 0 mk : Movable n} ->
           Instruction bs
meldInto n into =
  Enact Nothing "Meld"
        (Move n battlefieldZ [EntersMelded into] {arr} {pl} {mk})

public export
returnTo : (n : Noun bs Object) -> (to : ZoneExpr (nomIntro n)) ->
           (riders : List (TokenRider (nomIntro n))) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur n) to} ->
           {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} ->
           {auto 0 mk : Movable n} ->
           Instruction bs
returnTo n to riders = Enact Nothing "Return" (Move n to riders {ok} {arr} {pl} {rf} {mk})

public export
returnToBattlefieldTransformed :
  (n : Noun bs Object) -> (ctrl : Noun (nomIntro n) Player) ->
  {auto 0 one : CtrlOverrideOk ctrl} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 mk : Movable n} ->
  Instruction bs
returnToBattlefieldTransformed n ctrl =
  Enact Nothing "Return"
        (Move n battlefieldZ [EntersTransformed, Under ctrl {one}] {arr} {pl} {mk})

public export
returnToBattlefield : (n : Noun bs Object) ->
                      {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                      {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                      {auto 0 mk : Movable n} ->
                      Instruction bs
returnToBattlefield n = returnTo n battlefieldZ [] {arr} {pl} {mk}

public export
itAsToken : {auto 0 ok : countReach TokenBorn OneOf bs = 1} -> Noun bs Object
itAsToken = Pro TokenBorn OneOf Whole {ok}

public export
ownSubject : {bs : Bindings} -> (n : Noun bs Object) ->
             {auto 0 ok : countReach Bare (nounPlur n)
                            (view (Top (length (selfSubjDelta n ++ nounDelta n)))
                                  (selfSubjIntro n)) = 1} ->
             Noun (selfSubjIntro n) Object
ownSubject n =
  Pro Bare (nounPlur n) (Top (length (selfSubjDelta n ++ nounDelta n))) {ok}

public export
sharedSubject : {bs : Bindings} -> {0 k : Nat} -> (n : Noun bs Object) ->
                (parts : StaticParts k (selfSubjIntro n)) ->
                {auto 0 ne : IsSucc k} ->
                (d : Maybe (Duration (partsIntro parts))) ->
                {auto 0 sp : SpanOk d} ->
                {auto 0 cl : ClauseStatic (AndAlso (Just n) parts {ne})} -> Instruction bs
sharedSubject {bs} n parts d = Continuously {bs} (AndAlso (Just n) parts {ne}) d {sp} {cl}


public export
dealsDivided : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
               (among : Noun (amtIntro amt) k) ->
               {auto 0 gm : GroupMention among} ->
               {auto 0 rk : DamageRecipient among} -> Instruction bs
dealsDivided src amt among =
  Distribute (DividedDamage src) amt among {gm}
             {tk = DamageDivided {rk}}

public export
distributeCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (among : Noun (amtIntro amt) Object) ->
                     {auto 0 gm : GroupMention among} -> Instruction bs
distributeCounters amt kind among =
  Distribute (DistributedCounters kind) amt among {gm}
             {tk = CountersDistributed}


public export
untilEndOfTurn : Duration bs
untilEndOfTurn = Until (EndOf Turn Nothing)

public export
untilYourNextTurn : Duration bs
untilYourNextTurn = Until (StartOf Turn (Just You))

public export
untilEndOfCombat : Duration bs
untilEndOfCombat = Until (EndOf Combat Nothing)

public export
untilYourNextUpkeep : Duration bs
untilYourNextUpkeep = Until (StartOf Upkeep (Just You))

public export
untilYourNextEndStep : Duration bs
untilYourNextEndStep = Until (StartOf EndStep (Just You))

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
               StaticSpec bs
entersTapped n = EntersRider n EntersTapped {zn}

public export
entersWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                     (kind : CounterKind) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticSpec bs
entersWithCounters n amt kind =
  EntersRider n (WithCounters amt (PrintedKind kind) Fresh) {zn}

public export
gets : {bs : Bindings} -> (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
       (tou : PtShift (shiftIntro pow)) ->
       {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
       (d : Maybe (Duration (staticIntro (Gets Adds n pow tou {ok})))) ->
       {auto 0 sp : SpanOk d} -> Instruction bs
gets {bs} n pow tou d = Continuously {bs} (Gets Adds n pow tou {ok}) d {sp}

public export
gains : {bs : Bindings} -> (n : Noun bs Object) -> (a : AbilityAt bs) ->
        {auto 0 ok : GrantSubject a n} ->
        {auto 0 gr : Grantable a} ->
        (d : Maybe (Duration (staticIntro (Gains n a {ok} {gr})))) ->
        {auto 0 sp : SpanOk d} -> Instruction bs
gains {bs} n a d = Continuously {bs} (Gains n a {ok} {gr}) d

public export
gainsHaste : {bs : Bindings} -> (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 ok : GrantSubject (KeywordAbility "Haste" Nothing Nothing) n} ->
             {auto 0 sp : SpanOk d} -> Instruction bs
gainsHaste {bs} n d = gains {bs} n (KeywordAbility "Haste" Nothing Nothing) d

public export
plusOnePlusOne : CounterKind
plusOnePlusOne = BoostCounter (Up 1) (Up 1)

public export
minusOneMinusOne : CounterKind
minusOneMinusOne = BoostCounter (Down 1) (Down 1)

public export
flyingCounter : CounterKind
flyingCounter = KeywordCounter "Flying"


public export
deontic : {k : Kind} -> (n : Noun bs k) -> (c : Compulsion (selfSubjIntro n)) ->
          (deeds : Deeds) -> (role : Role) ->
          (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
          {auto 0 ne : NonEmpty deeds} ->
          {auto 0 dd : So (distinctDeeds deeds)} ->
          {auto 0 kd : KnownActs deeds} ->
          {auto 0 dp : DeedFits deeds role k (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
          {auto 0 pt : So (deonticPatientOk n deeds role patient NoDeonticRider)} ->
          StaticSpec bs
deontic n c deeds role patient =
  Deontic n c deeds role Nothing patient Nothing NoDeonticRider {ne} {dd} {kd} {dp} {pt}

public export
cantMoreThan : (who : Noun bs Player) -> (deed : VerbLabel) -> (k : Nat) ->
               (p : Predicate (nomIntro who) Object) ->
               {auto 0 kd : KnownActs [deed]} ->
               {auto 0 dp : DeedFits [deed] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
               {auto 0 bd : So (deonticBoundOk {bs = nomIntro who} [deed]
                                                (Just (MoreThan (Lit k))))} ->
               {auto 0 pt : So (deonticPatientOk who [deed] Agent
                                  (DeonticCounterpart (allOf p)) NoDeonticRider)} ->
               StaticSpec bs
cantMoreThan who deed k p =
  Deontic who Forbid [deed] Agent (Just (MoreThan (Lit k))) (DeonticCounterpart (allOf p))
          Nothing NoDeonticRider {kd} {dp} {bd} {pt}

public export
mayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                         {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                         {auto 0 dp : DeedFits ["Play"] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
                         {auto 0 bd : So (deonticBoundOk ["Play"] (Just (Additional q)))} ->
                         {auto 0 pt : So (deonticPatientOk who ["Play"] Agent
                                            (DeonticCounterpart (allOf Macros.land))
                                            NoDeonticRider)} ->
                         StaticSpec bs
mayPlayAdditionalLands who q =
  Deontic who Permit ["Play"] Agent (Just (Additional q)) (DeonticCounterpart (allOf Macros.land))
          Nothing NoDeonticRider {dp} {bd} {pt}

public export
mayBlockAdditional : (n : Noun bs Object) -> (q : Quantity (nomIntro n)) ->
                     {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                     {auto 0 dp : DeedFits ["Block"] Agent Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
                     {auto 0 bd : So (deonticBoundOk ["Block"] (Just (Additional q)))} ->
                     {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                        (DeonticCounterpart (allOf Macros.creature))
                                        NoDeonticRider)} ->
                     StaticSpec bs
mayBlockAdditional n q =
  Deontic n Permit ["Block"] Agent (Just (Additional q)) (DeonticCounterpart (allOf Macros.creature))
          Nothing NoDeonticRider {dp} {bd} {pt}

public export
mayVoteAdditional : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                    {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                    {auto 0 dp : DeedFits ["Vote"] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
                    {auto 0 bd : So (deonticBoundOk ["Vote"] (Just (Additional q)))} ->
                    StaticSpec bs
mayVoteAdditional who q =
  Deontic who Permit ["Vote"] Agent (Just (Additional q)) NoDeonticPatient
          Nothing NoDeonticRider {dp} {bd}

public export
doesntUntap : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
              {auto 0 wk : WindowOk UntapStep w} ->
              {auto 0 dp : DeedFits ["Untap"] Patient Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
              StaticSpec bs
doesntUntap n w =
  OnlyDuring UntapStep w
    (Deontic n Forbid ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
mayDeclineUntap : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
                  {auto 0 wk : WindowOk UntapStep w} ->
                  {auto 0 dp : DeedFits ["Untap"] Patient Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
                  StaticSpec bs
mayDeclineUntap n w =
  OnlyDuring UntapStep w
    (Deontic n Permit ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
untapsDuring : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
               {auto 0 wk : WindowOk UntapStep w} ->
               {auto 0 dp : DeedFits ["Untap"] Patient Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
               StaticSpec bs
untapsDuring n w =
  OnlyDuring UntapStep w
    (Deontic n Require ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
cantAttack : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 dp : DeedFits ["Attack"] Agent Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
             {auto 0 sp : SpanOk span} -> Instruction bs
cantAttack {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Attack"] Agent Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
cantBlock : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 dp : DeedFits ["Block"] Agent Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
            {auto 0 sp : SpanOk span} -> Instruction bs
cantBlock {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Block"] Agent Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
canDoAsThough : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
                (p : Predicate (nomIntro n) Object) ->
                {auto 0 kd : KnownActs [deed]} ->
                {auto 0 dp : DeedFits [deed] Agent k (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
                {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro n}) [deed] (Just (AsThoughOf p)))} ->
                StaticSpec bs
canDoAsThough n deed p =
  Deontic n Permit [deed] Agent Nothing NoDeonticPatient (Just (AsThoughOf p)) NoDeonticRider
          {kd} {dp} {at}

public export
maySpendAsThough : (who : Noun bs Player) ->
                   (what : Maybe ColorOrColorless) -> (as : ManaMatch) ->
                   (purpose : Maybe (SpendPurpose (nomIntro who))) ->
                   {auto 0 kd : KnownActs ["Spend"]} ->
                   {auto 0 dp : DeedFits ["Spend"] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
                   {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro who}) ["Spend"]
                                               (Just (AsThoughMana what as purpose)))} ->
                   StaticSpec bs
maySpendAsThough who what as purpose =
  Deontic who Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana what as purpose)) NoDeonticRider
          {dd = Oh} {dp} {bd = Oh} {at} {pt = Oh} {rd = Oh}

public export
playerCant : (deed : VerbLabel) -> (who : Noun bs Player) ->
             {auto 0 kd : KnownActs [deed]} ->
             {auto 0 dp : DeedFits [deed] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
             StaticSpec bs
playerCant deed who = Deontic who Forbid [deed] Agent Nothing NoDeonticPatient Nothing NoDeonticRider
                              {kd} {dp}

public export
objectCant : {k : Kind} -> (deed : VerbLabel) -> (what : Noun bs k) ->
             {auto 0 kd : KnownActs [deed]} ->
             {auto 0 dp : DeedFits [deed] Patient k (nounIsAbility what) (nounHeadTys what) (nounZone what)} ->
             StaticSpec bs
objectCant deed what =
  Deontic what Forbid [deed] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {kd} {dp}

public export
cantDoTo : {k : Kind} -> {kw : Kind} -> (deed : VerbLabel) ->
           (who : Noun bs k) -> (what : Noun (nomIntro who) kw) ->
           {auto 0 kd : KnownActs [deed]} ->
           {auto 0 dp : DeedFits [deed] Agent k (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
           {auto 0 pt : So (deonticPatientOk who [deed] Agent
                              (DeonticCounterpart what)
                              NoDeonticRider)} ->
           StaticSpec bs
cantDoTo deed who what =
  Deontic who Forbid [deed] Agent Nothing (DeonticCounterpart what) Nothing NoDeonticRider
          {kd} {dp} {pt}

public export
cantBeTargetedBy : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                   (by : Noun (nomIntro what) ka) ->
                   {auto 0 tr : Targeter ka} ->
                   {auto 0 dp : DeedFits ["Target"] Patient k (nounIsAbility what) (nounHeadTys what) (nounZone what)} ->
                   StaticSpec bs
cantBeTargetedBy what by =
  Deontic what Forbid ["Target"] Patient Nothing (TargetedBy by {tr}) Nothing NoDeonticRider {dp}

public export
canBeTargetedAsThough : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                        (by : Noun (nomIntro what) ka) ->
                        (p : Predicate (nomIntro what) Object) ->
                        {auto 0 tr : Targeter ka} ->
                        {auto 0 dp : DeedFits ["Target"] Patient k (nounIsAbility what) (nounHeadTys what) (nounZone what)} ->
                        StaticSpec bs
canBeTargetedAsThough what by p =
  Deontic what Permit ["Target"] Patient Nothing (TargetedBy by {tr}) (Just (AsThoughOf p))
          NoDeonticRider {dp}

public export
cantBeBlocked : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 dp : DeedFits ["Block"] Patient Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
                {auto 0 sp : SpanOk span} -> Instruction bs
cantBeBlocked {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Block"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
mustBlockIt : {bs : Bindings} -> (n : Noun bs Object) ->
              (span : Maybe (Duration (selfSubjIntro n))) ->
              {auto 0 dp : DeedFits ["Block"] Agent Object (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
              {auto 0 ok : countReach Bare OneOf
                             (view (Below (length (nounDelta n))) (nomIntro n)) = 1} ->
              {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                 (DeonticCounterpart
                                    (Pro Bare OneOf (Below (length (nounDelta n))) {ok}))
                                 NoDeonticRider)} ->
              {auto 0 sp : SpanOk span} -> Instruction bs
mustBlockIt n span =
  Continuously (Deontic n Require ["Block"] Agent Nothing
                  (DeonticCounterpart
                     (Pro Bare OneOf (Below (length (nounDelta n))) {ok}))
                  Nothing NoDeonticRider {dp} {pt})
               span {sp}


public export
attachToIt : {bs : Bindings} -> (what : Noun bs Object) ->
             {auto 0 zw : ZoneIs (nounZone what) Battlefield} ->
             {auto 0 ok : countReach Bare OneOf
                            (view (Below (length (nounDelta what)))
                                  (nomIntro what)) = 1} -> Instruction bs
attachToIt what =
  AttachTo what (Pro Bare OneOf (Below (length (nounDelta what))) {ok}) {zw}


public export
gainControl : {bs : Bindings} -> (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              {auto 0 zn : ZoneIs (nounZone what) Battlefield} ->
              (d : Maybe (Duration (staticIntro (GainsControl who what {zn})))) ->
              {auto 0 sp : SpanOk d} -> Instruction bs
gainControl {bs} who what d = Continuously {bs} (GainsControl who what {zn}) d {sp}

public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Instruction bs
losesLife who amt = ChangeLife who (LifeDown amt)

public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Instruction bs
gainsLife who amt = ChangeLife who (LifeUp amt)


public export
may : (decider : Noun bs Player) -> Instruction (agentIntro decider) -> Instruction bs
may decider body = May decider body Nothing Nothing


public export
creatureTokOf : (pow : Amount bs) -> (tou : Amount (amtIntro pow)) ->
                List Color -> List Subtype -> TokenChars bs
creatureTokOf pow tou cs ss = MkToken (Just (pow ** tou)) cs (MkTypeLine ss [Creature]) [] Nothing

public export
creatureTok : (pow : Nat) -> (tou : Nat) -> List Color -> List Subtype -> TokenChars bs
creatureTok pow tou cs ss = creatureTokOf (Lit pow) (Lit tou) cs ss

public export
subtypesOnly : List Subtype -> TypeLine
subtypesOnly ss = MkTypeLine ss []

public export
typesOnly : List CardType -> TypeLine
typesOnly ts = MkTypeLine [] ts

public export
create : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
         {auto 0 wf : TokenWellFormed tok} ->
         Instruction bs
create count tok = Create You count (TokenWritten tok {wf}) []

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 wf : TokenWellFormed tok} ->
                        Instruction bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {wf})
         [EntersTapped, EntersAttacking NoDefender]

public export
becomesAs : {bs : Bindings} -> (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 ok : BecomesOk Adds n (Bundle added Nothing)} ->
            {auto 0 sp : SpanOk d} -> Instruction bs
becomesAs {bs} n added d = Continuously {bs} (Becomes n Adds (Bundle added Nothing) {ok}) d {sp}

public export
becomes : {bs : Bindings} -> (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 ok : BecomesOk Adds n (Bundle (MkToken {bs} Nothing [] added [] Nothing) Nothing)} ->
          {auto 0 sp : SpanOk d} -> Instruction bs
becomes {bs} n added d = becomesAs {bs} n (MkToken Nothing [] added [] Nothing) d {ok} {sp}

public export
becomesColor : {bs : Bindings} -> (n : Noun bs Object) -> (cs : ColorSpec) ->
               (d : Maybe (Duration (selfSubjIntro n))) ->
               {auto 0 ok : BecomesOk Sets n (Colored cs)} ->
               {auto 0 sp : SpanOk d} -> Instruction bs
becomesColor {bs} n cs d = Continuously {bs} (Becomes n Sets (Colored cs) {ok}) d {sp}

public export
basicLandLine : (ss : List Subtype) -> {auto 0 bl : BasicLandTypes ss} -> TypeLine
basicLandLine ss = MkTypeLine ss []


public export
noCosts : {0 bs : Bindings} -> List (Instruction bs) -> List (Maybe (Cost bs), Instruction bs)
noCosts = map (\m => (Nothing, m))

public export
chooseModes : (q : Quantity bs) -> (modes : List (Instruction bs)) ->
              {auto 0 nz : NonZeroQ q} ->
              {auto 0 wf : WellFormedQ q} ->
              {auto 0 tw : AtLeastTwo (modeCount (noCosts modes))} ->
              {auto 0 mf : ModesFit q (modeCount (noCosts modes))} -> Instruction bs
chooseModes q modes = Modal q (noCosts modes) {nz} {wf} {tw} {mf}

||| Spree ([CR#702.172a]): "choose one or more modes", each carrying its own
||| additional cost ([CR#700.2h]).
public export
spree : {0 bs : Bindings} -> (modes : List (Maybe (Cost bs), Instruction bs)) ->
        {auto 0 tw : AtLeastTwo (modeCount modes)} ->
        {auto 0 mf : ModesFit (the (Quantity bs) (Macros.atLeast 1)) (modeCount modes)} ->
        {auto 0 ac : AllCosted modes} -> Instruction bs
spree modes = Modal (the (Quantity bs) (Macros.atLeast 1)) modes {tw} {mf}

public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countReach Bare OneOf bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfReach Bare OneOf bs) (seedZone p)} ->
       {auto 0 ah : AttachFits (Macros.It OneOf {ok}) p} ->
       Condition bs
itsA p = Matches (It OneOf {ok}) p {sy} {zc} {ah}

public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countReach Bare OneOf bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfReach Bare OneOf bs) (seedZone p)} ->
          {auto 0 ah : AttachFits (Macros.It OneOf {ok}) p} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc} {ah})

public export
itsACard : (p : Predicate bs Object) ->
           {auto 0 ok : countReach (AtSlot CardSlot) OneOf bs = 1} ->
           {auto 0 sy : PredSays p} ->
           {auto 0 zc : ZoneFits (zoneOfReach (AtSlot CardSlot) OneOf bs) (seedZone p)} ->
           {auto 0 ah : AttachFits (Pro (AtSlot CardSlot) OneOf Whole {ok}) p} ->
           Condition bs
itsACard p = Matches (Pro (AtSlot CardSlot) OneOf Whole {ok}) p {sy} {zc} {ah}

public export
itIsntAnAbility : (p : Predicate bs Object) ->
                  {auto 0 ok : countReach (Word AbilityW) OneOf bs = 1} ->
                  {auto 0 sy : PredSays p} ->
                  {auto 0 bl : TestSubject (Pro (Word AbilityW) OneOf Whole {bs} {ok})} ->
                  {auto 0 zc : ZoneFits (zoneOfReach (Word AbilityW) OneOf bs)
                                              (seedZone p)} ->
                  {auto 0 ah : AttachFits (Pro (Word AbilityW) OneOf Whole {ok}) p} ->
                  Condition bs
itIsntAnAbility p =
  NotCond (Matches (Pro (Word AbilityW) OneOf Whole {ok}) p {sy} {bl} {zc} {ah})


public export
libraryOf : (n : Noun bs Player) -> ZoneExpr bs
libraryOf n = ZoneAt Library (PossessedBy n {ps = LibraryIsOwned})

public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt (OneEnd OnTop) Nothing Nothing BareScope

public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt (OneEnd OnBottom) Nothing Nothing BareScope

public export
onTopIn : (a : Arrangement) ->
          {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnTop) (Just a)} ->
          ZoneExpr bs
onTopIn a = LibraryAt (OneEnd OnTop) (Just a) Nothing {af} {nf = Oh} BareScope

public export
onBottomIn : (a : Arrangement) ->
             {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnBottom) (Just a)} ->
             ZoneExpr bs
onBottomIn a = LibraryAt (OneEnd OnBottom) (Just a) Nothing {af} {nf = Oh} BareScope

public export
nthFromTop : (n : Ordinal) -> ZoneExpr bs
nthFromTop n = LibraryAt (OneEnd OnTop) Nothing (Just n) BareScope

public export
topOrBottomZ : ZoneExpr bs
topOrBottomZ = LibraryAt (EitherEnd Nothing) Nothing Nothing BareScope

public export
choiceOfTopOrBottom : (chooser : Noun bs Player) ->
                      {auto 0 ag : EventAgent (Just chooser)} -> ZoneExpr bs
choiceOfTopOrBottom chooser = LibraryAt (EitherEnd (Just chooser) {ag}) Nothing Nothing BareScope

public export
nthFromTopOrBottomZ : (n : Ordinal) -> ZoneExpr bs
nthFromTopOrBottomZ n = LibraryAt (EitherEnd Nothing) Nothing (Just n) BareScope

public export
shuffleInto : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
              {auto 0 pl : Placeable (nounTy n) Library} ->
              {auto 0 mk : Movable n} ->
              {auto 0 ke : EnactKeepsOuter (Just agent)
                             (Move n (LibraryAt Shuffled Nothing Nothing BareScope)
                                     [] {pl} {mk})} ->
              Instruction bs
shuffleInto agent n =
  Enact (Just agent) "Shuffle"
    (Move n (LibraryAt Shuffled Nothing Nothing BareScope)
            [] {pl} {mk}) {ke}

public export
topSlice : (amt : Amount bs) -> Noun bs Object
topSlice amt = LibrarySlice OnTop amt You

public export
bottomCard : Noun bs Object
bottomCard = LibrarySlice OnBottom (Lit 1) You

public export
someOf : (q : Quantity bs) -> (grp : Noun bs Object) ->
         {auto 0 gm : PartitiveBase grp} ->
         {auto 0 nz : NonZeroQ q} ->
         {auto 0 wf : WellFormedQ q} -> Noun bs Object
someOf q grp = SomeOf (CountedSlice q {nz} {wf}) Nothing grp {gm}

public export
allAmong : (descr : Predicate bs Object) -> (grp : Noun bs Object) ->
           {auto 0 gm : PartitiveBase grp} -> Noun bs Object
allAmong descr grp = SomeOf WholeSlice (Just descr) grp {gm}

public export
among : (grp : Noun bs Object) ->
        {auto 0 gm : PartitiveBase grp} -> Noun bs Object
among grp = SomeOf WholeSlice Nothing grp {gm}

public export
onePile : {auto 0 ok : countReach (Word PileW) ManyOf bs = 1} -> Noun bs Pile
onePile = PileOf (CountedSlice (exactly 1)) Nothing {ok}

public export
pileOfChoice : (by : Noun bs Player) ->
               {auto 0 ok : countReach (Word PileW) ManyOf bs = 1} -> Noun bs Pile
pileOfChoice by = PileOf (CountedSlice (exactly 1)) (Just by) {ok}

public export
fromAmong : (q : Quantity bs) -> (p : Predicate bs Object) ->
            (grp : Noun bs Object) ->
            {auto 0 gm : PartitiveBase grp} ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} -> Noun bs Object
fromAmong q p grp = SomeOf (CountedSlice q {nz} {wf}) (Just p) grp {gm}

public export
allFromAmong : (p : Predicate bs Object) -> (grp : Noun bs Object) ->
               {auto 0 gm : PartitiveBase grp} -> Noun bs Object
allFromAmong p grp = SomeOf WholeSlice (Just p) grp {gm}

public export
lookAt : (n : Noun bs Object) -> Instruction bs
lookAt n = Expose LookAt You (ExposedCards n)

public export
revealCards : (n : Noun bs Object) -> Instruction bs
revealCards n = Expose Reveal You (ExposedCards n)

public export
lookAtHandOf : (n : Noun bs Player) -> Instruction bs
lookAtHandOf n = Expose LookAt You (ExposedZone (handOf n))

public export
foundCard : {auto 0 ok : countReach (Stamped "Search") OneOf bs = 1} -> Noun bs Object
foundCard = ItVerbed "Search" OneOf {ok}

public export
revealsIt : {auto 0 ok : countReach (Stamped "Search") OneOf bs = 1} -> Instruction bs
revealsIt = revealCards (foundCard {ok})

public export
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countReach (Word PlayerW) OneOf (nomIntro who) = 1} -> Instruction bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))



public export
searchLibraryFor : (q : Quantity bs) -> (p : Predicate bs Object) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 zf : ZoneFree p} -> Instruction bs
searchLibraryFor q p = Search You (OneZone yourLibrary) q p {zf}

public export
searchZonesOf : (whose : Noun bs Player) -> (p : Predicate bs Object) ->
                {auto 0 zf : ZoneFree p} -> Instruction bs
searchZonesOf whose p =
  Search You (SomeZones (Just whose) [Graveyard, Hand, Library]) (exactly 1) p {zf}

public export
searchLibraryOrGraveyard : (p : Predicate bs Object) ->
                           {auto 0 zf : ZoneFree p} -> Instruction bs
searchLibraryOrGraveyard p =
  Search You (SomeZones (Just You) [Library, Graveyard]) (exactly 1) p {zf}

public export
puts : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
       (to : ZoneExpr (nomIntro n)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur n) to} ->
       {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
       {auto 0 mk : Movable n} ->
       {auto 0 ke : EnactKeepsOuter (Just agent) (Move n to [] {ok} {arr} {pl} {mk})} ->
       Instruction bs
puts agent n to =
  Enact (Just agent) "Put" (Move n to [] {ok} {arr} {pl} {mk}) {ke}

public export
shuffle : Instruction bs
shuffle = Shuffle You

public export
lifeTotalBecomes : (who : Noun bs Player) -> Amount (nomIntro who) -> Instruction bs
lifeTotalBecomes who a = ChangeLife who (Set a)


public export
ifWouldInstead : {bs : Bindings} -> (ev : GameEvent bs) -> (repl : Instruction (eventIntro ev)) ->
                 (d : Maybe (Duration (eventIntro ev))) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 sp : SpanOk d} -> Instruction bs
ifWouldInstead {bs} ev repl d = Continuously {bs} (Intercepts ev [] Nothing repl Repeatedly Nothing {ok}) d {sp}

public export
nextTimeWouldInstead : {bs : Bindings} -> (ev : GameEvent bs) -> (repl : Instruction (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 sp : SpanOk d} -> Instruction bs
nextTimeWouldInstead {bs} ev repl d =
  Continuously {bs} (Intercepts ev [] Nothing repl NextTimeOnly Nothing {ok}) d {sp}

public export
preventAll : {bs : Bindings} -> (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk d} -> Instruction bs
preventAll {bs} kind scope d =
  Continuously {bs} (DamageRule kind Unattributed scope (Prevent CutAll Nothing) Repeatedly) d {sp}

public export
preventNext : {bs : Bindings} -> (kind : DamageKind) -> (scope : DamageScope bs) ->
              (amt : Amount (scopeIntro scope)) ->
              (d : Maybe (Duration (amtIntro amt))) ->
              {auto 0 sp : SpanOk d} -> Instruction bs
preventNext {bs} kind scope amt d =
  Continuously {bs} (DamageRule kind Unattributed scope (Prevent (Shield amt) Nothing) Repeatedly) d {sp}

public export
preventAllBy : {bs : Bindings} -> (kind : DamageKind) -> (src : Noun bs Object) ->
               (scope : DamageScope (nomIntro src)) ->
               (d : Maybe (Duration (scopeIntro scope))) ->
               {auto 0 sp : SpanOk d} -> Instruction bs
preventAllBy {bs} kind src scope d =
  Continuously {bs} (DamageRule kind (DealtBy src) scope (Prevent CutAll Nothing) Repeatedly) d {sp}

public export
exileUntil : (n : Noun bs Object) ->
             {auto 0 mk : Movable n} ->
             (ev : GameEvent
                     (annIntro
                       (Enact Nothing "Exile"
                         (Move n (Macros.exileZ {bs = nomIntro n})
                               []
                               {mk} {ok = ExileOk} {arr = Oh} {pl = Oh} {rf = Oh})))) ->
             Instruction bs
exileUntil n ev =
  HeldUntil
    (Enact Nothing "Exile"
      (Move n (Macros.exileZ {bs = nomIntro n})
            []
            {mk} {ok = ExileOk} {arr = Oh} {pl = Oh} {rf = Oh}))
            ev {ok = Oh}

public export
phasesOutUntil : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 {auto 0 at : StatusMarkable PhasedOut} ->
                 (ev : GameEvent (annIntro (SetStatus PhasedOut n {ok = zn} {at}))) ->
                 Instruction bs
phasesOutUntil n ev = HeldUntil (SetStatus PhasedOut n {ok = zn} {at}) ev {ok = Oh}


public export
generic : Nat -> ManaSymbol
generic n = Simple (Generic n)

public export
scaledMana : {bs : Bindings} -> (unit : ManaUnit) -> (amt : Amount bs) ->
             {auto 0 fe : ForEachAmount amt} -> Cost bs
scaledMana GenericUnit amt = ScaledCost (Mana [generic 1]) amt {fe}
scaledMana (RunUnit run {wr}) amt = ScaledCost (Mana run {wr}) amt {fe}

public export
pip : Color -> ManaSymbol
pip c = Simple (Specific (OfColor c))

public export
colorlessPip : ManaSymbol
colorlessPip = Simple (Specific Colorless)

public export
hybridPip : (a : Color) -> (b : Color) ->
            {auto 0 ds : HalvesDistinct (Specific (OfColor a)) b} -> ManaSymbol
hybridPip a b = Hybrid (Specific (OfColor a)) b {ds}

public export
phyrexianPip : Color -> ManaSymbol
phyrexianPip c = Phyrexian c Nothing

public export
payLife : (who : Noun bs Player) -> (n : Nat) -> Cost bs
payLife who n = Do (ChangeLife who (LifeDown (Lit n)))

public export
mayWhen : (decider : Noun bs Player) -> (body : Instruction (agentIntro decider)) ->
          Instruction (settleTargets (instrIntro body)) ->
          {auto 0 ok : So (admitsReflexEnclosure (reflexEncloseUse body))} ->
          Instruction bs
mayWhen d body trig =
  Reflexively (May d body Nothing Nothing) trig {en = ok}



public export
mills : (agent : Noun bs Player) -> (amt : Amount (agentIntro agent)) ->
        (whose : Noun (agentIntro agent) Player) ->
        {auto 0 sp : SlicePossessor whose} ->
        {auto 0 ke : EnactKeepsOuter (Just agent)
                       (Move (LibrarySlice OnTop amt whose {sp})
                             Macros.graveyardZ [])} ->
        Instruction bs
mills agent amt whose =
  Enact (Just agent) "Mill"
       (Move (LibrarySlice OnTop amt whose {sp}) graveyardZ
             []) {ke}

public export
lookedTop : (bs : Bindings) -> (amt : Amount bs) -> Bindings
lookedTop bs amt = nomIntro (topSlice {bs} amt)

||| The player an enacted look reads back inside the body. An agent that
||| introduced nothing of its own is re-read as written; anyone else is read
||| through its own window.
public export
OwnRefOk : Bindings -> Bindings -> Plurality -> Type
OwnRefOk [] outer pl = ()
OwnRefOk (d :: ds) outer pl =
  countReach (Word PlayerW) pl
             (view (Top (length (d :: ds))) ((d :: ds) ++ outer)) = 1

public export
AgentRefOk : {bs : Bindings} -> Noun bs Player -> Type
AgentRefOk agent =
  Macros.OwnRefOk (Experimental.Phrase.agentDelta agent) bs
                  (Experimental.Phrase.agentPlur agent)

public export
agentSelfOrOwn : {bs : Bindings} -> (agent : Noun bs Player) ->
                 (ds : Bindings) -> (pl : Plurality) ->
                 (0 ok : Macros.OwnRefOk ds bs pl) -> Noun (ds ++ bs) Player
agentSelfOrOwn agent [] pl ok = agent
agentSelfOrOwn agent (d :: ds) pl ok =
  Pro (Word PlayerW) pl (Top (length (d :: ds))) {ok}

public export
agentRef : {bs : Bindings} -> (agent : Noun bs Player) ->
           (0 ok : Macros.AgentRefOk agent) ->
           Noun (Experimental.Phrase.agentIntro agent) Player
agentRef agent ok =
  Macros.agentSelfOrOwn agent (Experimental.Phrase.agentDelta agent)
                        (Experimental.Phrase.agentPlur agent) ok

||| "the top N cards of that player's library", read in the looker's own
||| context; the looker and the library's owner need not be the same player
||| [CR#701.29a].
public export
lookedSliceOf : {ctx : Bindings} -> (looker : Noun ctx Player) ->
                (0 nd : Experimental.Phrase.nounDelta looker = []) ->
                (whose : Noun ctx Player) ->
                (0 sp : SlicePossessor whose) ->
                (amt : Amount ctx) -> Noun (nomIntro looker) Object
lookedSliceOf looker nd whose sp amt =
  replace {p = \ds => Noun (ds ++ ctx) Object} (sym nd)
          (LibrarySlice OnTop amt whose {sp})

public export
lookedScopeOf : {ctx : Bindings} -> (looker : Noun ctx Player) ->
                (0 nd : Experimental.Phrase.nounDelta looker = []) ->
                (whose : Noun ctx Player) ->
                (0 sp : SlicePossessor whose) ->
                (amt : Amount ctx) -> Bindings
lookedScopeOf looker nd whose sp amt =
  nomIntro (Macros.lookedSliceOf looker nd whose sp amt)

public export
lookedSliceCountOf : {ctx : Bindings} -> (looker : Noun ctx Player) ->
                     (0 nd : Experimental.Phrase.nounDelta looker = []) ->
                     (whose : Noun ctx Player) ->
                     (0 sp : SlicePossessor whose) ->
                     (amt : Amount ctx) -> Nat
lookedSliceCountOf looker nd whose sp amt =
  length (Experimental.Phrase.nounDelta
            (Macros.lookedSliceOf looker nd whose sp amt))

public export
lookedSlicePlurOf : {ctx : Bindings} -> (looker : Noun ctx Player) ->
                    (0 nd : Experimental.Phrase.nounDelta looker = []) ->
                    (whose : Noun ctx Player) ->
                    (0 sp : SlicePossessor whose) ->
                    (amt : Amount ctx) -> Plurality
lookedSlicePlurOf looker nd whose sp amt =
  nounPlur (Macros.lookedSliceOf looker nd whose sp amt)

||| "the top N cards of their library", read in the looker's own context.
public export
lookedSlice : {ctx : Bindings} -> (who : Noun ctx Player) ->
              (0 nd : Experimental.Phrase.nounDelta who = []) ->
              (0 sp : SlicePossessor who) ->
              (amt : Amount ctx) -> Noun (nomIntro who) Object
lookedSlice who nd sp amt = Macros.lookedSliceOf who nd who sp amt

public export
lookedScope : {ctx : Bindings} -> (who : Noun ctx Player) ->
              (0 nd : Experimental.Phrase.nounDelta who = []) ->
              (0 sp : SlicePossessor who) ->
              (amt : Amount ctx) -> Bindings
lookedScope who nd sp amt = Macros.lookedScopeOf who nd who sp amt

public export
lookedSliceCount : {ctx : Bindings} -> (who : Noun ctx Player) ->
                   (0 nd : Experimental.Phrase.nounDelta who = []) ->
                   (0 sp : SlicePossessor who) ->
                   (amt : Amount ctx) -> Nat
lookedSliceCount who nd sp amt = Macros.lookedSliceCountOf who nd who sp amt

public export
lookedSlicePlur : {ctx : Bindings} -> (who : Noun ctx Player) ->
                  (0 nd : Experimental.Phrase.nounDelta who = []) ->
                  (0 sp : SlicePossessor who) ->
                  (amt : Amount ctx) -> Plurality
lookedSlicePlur who nd sp amt = Macros.lookedSlicePlurOf who nd who sp amt

||| "any number of them" [CR#701.22a]: the cards just looked at, read
||| positionally off the look's own delta, one card included.
public export
lookedGroup : (bs : Bindings) -> (n : Nat) -> (pl : Plurality) ->
              (0 mn : countReach Bare pl (view (Top n) bs) = 1) -> Noun bs Object
lookedGroup bs n pl mn =
  SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
         (Pro Bare pl (Top n) {ok = mn}) {gm = Oh}

public export
lookedParted : (bs : Bindings) -> (n : Nat) -> (pl : Plurality) ->
               (0 mn : countReach Bare pl (view (Top n) bs) = 1) -> Bindings
lookedParted bs n pl mn = nomIntro (Macros.lookedGroup bs n pl mn)

public export
lookedSpilled : (bs : Bindings) -> (n : Nat) -> (pl : Plurality) ->
                (0 mn : countReach Bare pl (view (Top n) bs) = 1) ->
                (to : ZoneExpr (Macros.lookedParted bs n pl mn)) -> Bindings
lookedSpilled bs n pl mn to =
  afterMoveTo to
    (moveIntro Nothing (Macros.lookedGroup bs n pl mn) (Just (zoneSort to)))

||| "look at the top N cards of that player's library, then put any number of
||| them <spill> and the rest on top of that library in any order"
||| [CR#701.22a,701.29a]: the looker and the library's owner are separate.
public export
lookAndSortOf :
  {ctx : Bindings} -> (looker : Noun ctx Player) ->
  (0 nd : Experimental.Phrase.nounDelta looker = []) ->
  (whose : Noun ctx Player) ->
  (0 sp : SlicePossessor whose) ->
  (amt : Amount ctx) ->
  (0 mn : countReach Bare (Macros.lookedSlicePlurOf looker nd whose sp amt)
                          (view (Top (Macros.lookedSliceCountOf looker nd whose sp amt))
                                (Macros.lookedScopeOf looker nd whose sp amt)) = 1) ->
  (spill : ZoneExpr (Macros.lookedParted (Macros.lookedScopeOf looker nd whose sp amt)
                                         (Macros.lookedSliceCountOf looker nd whose sp amt)
                                         (Macros.lookedSlicePlurOf looker nd whose sp amt) mn)) ->
  (0 dk : DestOk spill) ->
  (0 sa : ArrangementOk
            (nounPlur (Macros.lookedGroup (Macros.lookedScopeOf looker nd whose sp amt)
                                          (Macros.lookedSliceCountOf looker nd whose sp amt)
                                          (Macros.lookedSlicePlurOf looker nd whose sp amt) mn))
            spill) ->
  (0 ps : Placeable (tyOfReach Bare (Macros.lookedSlicePlurOf looker nd whose sp amt)
                                    (view (Top (Macros.lookedSliceCountOf looker nd whose sp amt))
                                          (Macros.lookedScopeOf looker nd whose sp amt)))
                    (zoneSort spill)) ->
  (0 tr : So (theRestOk Object
                (Macros.lookedSpilled (Macros.lookedScopeOf looker nd whose sp amt)
                                      (Macros.lookedSliceCountOf looker nd whose sp amt)
                                      (Macros.lookedSlicePlurOf looker nd whose sp amt) mn spill))) ->
  (0 pr : Placeable
            (tyOfGroup Object
               (Macros.lookedSpilled (Macros.lookedScopeOf looker nd whose sp amt)
                                     (Macros.lookedSliceCountOf looker nd whose sp amt)
                                     (Macros.lookedSlicePlurOf looker nd whose sp amt) mn spill))
            Library) ->
  Instruction ctx
lookAndSortOf looker nd whose sp amt mn spill dk sa ps tr pr =
  Sequentially
    [ Expose LookAt looker (ExposedCards (Macros.lookedSliceOf looker nd whose sp amt))
    , Move (Macros.lookedGroup (Macros.lookedScopeOf looker nd whose sp amt)
                               (Macros.lookedSliceCountOf looker nd whose sp amt)
                               (Macros.lookedSlicePlurOf looker nd whose sp amt) mn)
           spill [] {mk = ObjectMoves {nb = PayloadIsObject}}
           {ok = dk} {arr = sa} {pl = ps}
    , Move (TheRest Object ManyOf {ok = tr}) (onTopIn AnyOrder {af = Oh}) []
           {mk = ObjectMoves {nb = PayloadIsObject}}
           {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ]

||| "look at the top N cards of their library, then put any number of them
||| <spill> and the rest on top of that library in any order" [CR#701.22a].
public export
lookAndSort :
  {ctx : Bindings} -> (who : Noun ctx Player) ->
  (0 nd : Experimental.Phrase.nounDelta who = []) ->
  (0 sp : SlicePossessor who) ->
  (amt : Amount ctx) ->
  (0 mn : countReach Bare (Macros.lookedSlicePlur who nd sp amt)
                          (view (Top (Macros.lookedSliceCount who nd sp amt))
                                (Macros.lookedScope who nd sp amt)) = 1) ->
  (spill : ZoneExpr (Macros.lookedParted (Macros.lookedScope who nd sp amt)
                                         (Macros.lookedSliceCount who nd sp amt)
                                         (Macros.lookedSlicePlur who nd sp amt) mn)) ->
  (0 dk : DestOk spill) ->
  (0 sa : ArrangementOk
            (nounPlur (Macros.lookedGroup (Macros.lookedScope who nd sp amt)
                                          (Macros.lookedSliceCount who nd sp amt)
                                          (Macros.lookedSlicePlur who nd sp amt) mn))
            spill) ->
  (0 ps : Placeable (tyOfReach Bare (Macros.lookedSlicePlur who nd sp amt)
                                    (view (Top (Macros.lookedSliceCount who nd sp amt))
                                          (Macros.lookedScope who nd sp amt)))
                    (zoneSort spill)) ->
  (0 tr : So (theRestOk Object
                (Macros.lookedSpilled (Macros.lookedScope who nd sp amt)
                                      (Macros.lookedSliceCount who nd sp amt)
                                      (Macros.lookedSlicePlur who nd sp amt) mn spill))) ->
  (0 pr : Placeable
            (tyOfGroup Object
               (Macros.lookedSpilled (Macros.lookedScope who nd sp amt)
                                     (Macros.lookedSliceCount who nd sp amt)
                                     (Macros.lookedSlicePlur who nd sp amt) mn spill))
            Library) ->
  Instruction ctx
lookAndSort who nd sp amt mn spill dk sa ps tr pr =
  Macros.lookAndSortOf who nd who sp amt mn spill dk sa ps tr pr

||| "scry N" [CR#701.22a]
public export
scry : {bs : Bindings} -> (agent : Noun bs Player) ->
       (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
       {auto 0 ar : Macros.AgentRefOk agent} ->
       {auto 0 nd : Experimental.Phrase.nounDelta (Macros.agentRef agent ar) = []} ->
       {auto 0 sp : SlicePossessor (Macros.agentRef agent ar)} ->
       {auto 0 mn : countReach Bare (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                    (view (Top (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)) (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)) = 1} ->
       {auto 0 ps : Placeable (tyOfReach Bare (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                              (view (Top (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)) (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)))
                              Library} ->
       {auto 0 tr : So (theRestOk Object
                          (Macros.lookedSpilled (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)
                                                (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)
                                                (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                                mn (onBottomIn AnyOrder {af = Oh})))} ->
       {auto 0 pr : Placeable
                      (tyOfGroup Object
                         (Macros.lookedSpilled (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)
                                               (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)
                                               (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                               mn (onBottomIn AnyOrder {af = Oh}))) Library} ->
       {auto 0 ke : EnactKeepsOuter (Just agent)
                      (Macros.lookAndSort (Macros.agentRef agent ar) nd sp amt mn
                         (onBottomIn AnyOrder {af = Oh})
                         (LibraryPosOk {af = Oh} {nf = Oh}) Oh ps tr pr)} ->
       Instruction bs
scry agent amt =
  Enact (Just agent) "Scry" {kn = ActInFactsTable}
        (Macros.lookAndSort (Macros.agentRef agent ar) nd sp amt mn
           (onBottomIn AnyOrder {af = Oh})
           (LibraryPosOk {af = Oh} {nf = Oh}) Oh ps tr pr) {ke}

||| "fateseal N" [CR#701.29a]: the agent looks at the top N cards of the
||| named library and sorts them.
public export
fateseal : {bs : Bindings} -> (agent : Noun bs Player) ->
           (whose : Noun (Experimental.Phrase.agentIntro agent) Player) ->
           (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
           {auto 0 ar : Macros.AgentRefOk agent} ->
           {auto 0 nd : Experimental.Phrase.nounDelta (Macros.agentRef agent ar) = []} ->
           {auto 0 sp : SlicePossessor whose} ->
           {auto 0 mn : countReach Bare (Macros.lookedSlicePlurOf (Macros.agentRef agent ar) nd whose sp amt)
                                    (view (Top (Macros.lookedSliceCountOf (Macros.agentRef agent ar) nd whose sp amt)) (Macros.lookedScopeOf (Macros.agentRef agent ar) nd whose sp amt)) = 1} ->
           {auto 0 ps : Placeable (tyOfReach Bare (Macros.lookedSlicePlurOf (Macros.agentRef agent ar) nd whose sp amt)
                                              (view (Top (Macros.lookedSliceCountOf (Macros.agentRef agent ar) nd whose sp amt)) (Macros.lookedScopeOf (Macros.agentRef agent ar) nd whose sp amt)))
                              Library} ->
           {auto 0 tr : So (theRestOk Object
                          (Macros.lookedSpilled (Macros.lookedScopeOf (Macros.agentRef agent ar) nd whose sp amt)
                                                (Macros.lookedSliceCountOf (Macros.agentRef agent ar) nd whose sp amt)
                                                (Macros.lookedSlicePlurOf (Macros.agentRef agent ar) nd whose sp amt)
                                                mn (onBottomIn AnyOrder {af = Oh})))} ->
           {auto 0 pr : Placeable
                      (tyOfGroup Object
                         (Macros.lookedSpilled (Macros.lookedScopeOf (Macros.agentRef agent ar) nd whose sp amt)
                                               (Macros.lookedSliceCountOf (Macros.agentRef agent ar) nd whose sp amt)
                                               (Macros.lookedSlicePlurOf (Macros.agentRef agent ar) nd whose sp amt)
                                               mn (onBottomIn AnyOrder {af = Oh}))) Library} ->
           {auto 0 ke : EnactKeepsOuter (Just agent)
                      (Macros.lookAndSortOf (Macros.agentRef agent ar) nd whose sp amt mn
                         (onBottomIn AnyOrder {af = Oh})
                         (LibraryPosOk {af = Oh} {nf = Oh}) Oh ps tr pr)} ->
           Instruction bs
fateseal agent whose amt =
  Enact (Just agent) "Fateseal" {kn = ActInFactsTable}
        (Macros.lookAndSortOf (Macros.agentRef agent ar) nd whose sp amt mn
           (onBottomIn AnyOrder {af = Oh})
           (LibraryPosOk {af = Oh} {nf = Oh}) Oh ps tr pr) {ke}

||| "surveil N" [CR#701.25a]
public export
surveil : {bs : Bindings} -> (agent : Noun bs Player) ->
          (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
          {auto 0 ar : Macros.AgentRefOk agent} ->
          {auto 0 nd : Experimental.Phrase.nounDelta (Macros.agentRef agent ar) = []} ->
          {auto 0 sp : SlicePossessor (Macros.agentRef agent ar)} ->
          {auto 0 mn : countReach Bare (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                       (view (Top (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)) (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)) = 1} ->
          {auto 0 ps : Placeable (tyOfReach Bare (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                                 (view (Top (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)) (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)))
                                 Graveyard} ->
          {auto 0 tr : So (theRestOk Object
                             (Macros.lookedSpilled (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)
                                                   (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)
                                                   (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                                   mn Macros.graveyardZ))} ->
          {auto 0 pr : Placeable
                         (tyOfGroup Object
                            (Macros.lookedSpilled (Macros.lookedScope (Macros.agentRef agent ar) nd sp amt)
                                                  (Macros.lookedSliceCount (Macros.agentRef agent ar) nd sp amt)
                                                  (Macros.lookedSlicePlur (Macros.agentRef agent ar) nd sp amt)
                                                  mn Macros.graveyardZ)) Library} ->
          {auto 0 ke : EnactKeepsOuter (Just agent)
                         (Macros.lookAndSort (Macros.agentRef agent ar) nd sp amt mn
                            Macros.graveyardZ GraveyardOkBare Oh ps tr pr)} ->
          Instruction bs
surveil agent amt =
  Enact (Just agent) "Surveil" {kn = ActInFactsTable}
        (Macros.lookAndSort (Macros.agentRef agent ar) nd sp amt mn
           graveyardZ GraveyardOkBare Oh ps tr pr) {ke}

public export
playerSearchesTheirLibraryFor : (who : Noun bs Player) ->
                                {auto 0 an : countReach (Word PlayerW) OneOf (nomIntro who) = 1} ->
                                (p : Predicate (nomIntro who) Object) ->
                                {auto 0 zf : ZoneFree p} -> Instruction bs
playerSearchesTheirLibraryFor who p =
  Search who (OneZone (libraryOf (They {ok = an}))) (exactly 1) p {zf}

public export
proliferate : {bs : Bindings} ->
              {auto 0 mj : countReach (Word JoinW) ManyOf
                              (chosenIntro {bs}
                                (counted Macros.anyNumber
                                  (Joined (And [Permanent, HasCounters Nothing])
                                          (Compare {k = Player} [AnyCounterAxis Player] AtLeast (Lit 1))))) = 1} ->
              Instruction bs
proliferate =
  Enact Nothing "Proliferate" {kn = ActInFactsTable}
        (Sequentially [ Choose Nothing Nothing
                          (counted Macros.anyNumber
                            (Joined (And [Permanent, HasCounters Nothing])
                                    (Compare {k = Player} [AnyCounterAxis Player] AtLeast (Lit 1))))
                          Openly
                      , PutCounters (Lit 1) OwnKinds (EachOf (Pro (Word JoinW) ManyOf Whole {ok = mj})) ])

public export
losesAllCounters : (who : Noun bs Player) ->
                   (kind : Maybe (CounterKindSource bs)) ->
                   {auto 0 sc : OptCounterSourceScope kind Player} -> Instruction bs
losesAllCounters who kind = LosesCounters who kind Nothing {sc}

public export
removeCounters : (q : Quantity bs) -> (kind : Maybe (CounterKindSource bs)) ->
                 (from : Noun (quantIntro q) Object) ->
                 {auto 0 wf : WellFormedQ q} ->
                 {auto 0 sc : OptCounterSourceScope kind Object} ->
                 {auto 0 cm : CounterMemory from} -> Instruction bs
removeCounters q kind from =
  RemoveCounters (Just q) kind from
                 {wf = Present {ok = wf}} {sc} {cm}

public export
removeAllCounters : (kind : Maybe (CounterKindSource bs)) ->
                    (from : Noun bs Object) ->
                    {auto 0 sc : OptCounterSourceScope kind Object} ->
                    {auto 0 cm : CounterMemory from} -> Instruction bs
removeAllCounters kind from = RemoveCounters Nothing kind from {sc} {cm}

public export
keyword : {0 bs : Bindings} -> (kw : KeywordLabel) ->
          {auto 0 pf : KeywordParamFits {bs} kw
                         (the (Maybe (KeywordParam bs)) Nothing)} ->
          AbilityAt bs
keyword kw = KeywordAbility kw Nothing Nothing {pf}

public export
keywordSubject : {0 bs : Bindings} -> {k : Kind} -> (kw : KeywordLabel) ->
                 (p : Predicate [] k) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamSubject {bs} p))} ->
                 AbilityAt bs
keywordSubject kw p = KeywordAbility kw (Just (ParamSubject p)) Nothing {pf}

public export
keywordCosting : {0 bs : Bindings} -> (kw : KeywordLabel) -> (c : Cost []) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamCost {bs} c))} ->
                 AbilityAt bs
keywordCosting kw c = KeywordAbility kw (Just (ParamCost c)) Nothing {pf}

||| "level up [cost]" [CR#702.87a]
public export
levelUp : {0 bs : Bindings} -> (c : Cost []) ->
          {auto 0 pf : KeywordParamFits {bs} "LevelUp"
                         (Just (ParamCost {bs} c))} ->
          AbilityAt bs
levelUp c = KeywordAbility "LevelUp" (Just (ParamCost c)) Nothing {pf}

public export
keywordQuality : {k : Kind} -> (kw : KeywordLabel) -> (q : Predicate bs k) ->
                 {auto 0 pk : So (qualityParamKind k)} ->
                 {auto 0 pf : KeywordParamFits kw (Just (ParamQuality q {pk}))} ->
                 AbilityAt bs
keywordQuality kw q = KeywordAbility kw (Just (ParamQuality q {pk})) Nothing {pf}

public export
keywordNumber : {0 bs : Bindings} -> (kw : KeywordLabel) -> (amt : Amount []) ->
                {auto 0 pf : KeywordParamFits {bs} kw
                               (Just (ParamNumber {bs} amt))} ->
                AbilityAt bs
keywordNumber kw amt = KeywordAbility kw (Just (ParamNumber amt)) Nothing {pf}

public export
keywordQualityCosting : {0 bs : Bindings} -> (kw : KeywordLabel) ->
                        (q : Predicate bs Object) -> (c : Cost []) ->
                        {auto 0 pf : KeywordParamFits kw
                                       (Just (ParamQualityCost q c))} ->
                        AbilityAt bs
keywordQualityCosting kw q c = KeywordAbility kw (Just (ParamQualityCost q c)) Nothing {pf}

public export
keywordNumberCosting : {0 bs : Bindings} -> (kw : KeywordLabel) ->
                       (amt : Amount []) -> (c : Cost []) ->
                       {auto 0 pf : KeywordParamFits {bs} kw
                                      (Just (ParamNumberCost {bs} amt c))} ->
                       AbilityAt bs
keywordNumberCosting kw amt c = KeywordAbility kw (Just (ParamNumberCost amt c)) Nothing {pf}

public export
companion : {0 bs : Bindings} -> (dc : DeckCondition) ->
            {auto 0 pf : KeywordParamFits {bs} "Companion"
                           (Just (ParamDeckCondition {bs} dc))} ->
            AbilityAt bs
companion dc = KeywordAbility "Companion" (Just (ParamDeckCondition dc)) Nothing {pf}

public export
abilityWord : {0 bs : Bindings} -> (word : AbilityWordLabel) ->
              (ab : AbilityAt bs) ->
              {auto 0 nw : NotWordHeaded ab} -> AbilityAt bs
abilityWord word ab = ItalicHead (AnAbilityWord word) ab {nw}

public export
flavorWord : {0 bs : Bindings} -> (word : FlavorWordLabel) ->
             (ab : AbilityAt bs) ->
             {auto 0 nw : NotWordHeaded ab} -> AbilityAt bs
flavorWord word ab = ItalicHead (AFlavorWord word) ab {nw}

public export
triggered : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
            (instr : Instruction (eventAfter ev)) ->
            {auto 0 hn : HeaderNontarget ev} ->
            {auto 0 hs : HeaderStatus ev} ->
            {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
            {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing Nothing} ->
            AbilityAt bs
triggered word ev instr =
  Triggered word ev [] Nothing [] Nothing Nothing Nothing instr {hn} {hs} {ae} {cd}

public export
triggeredIf : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (cond : Condition (headerCtx [] ev)) ->
              (instr : Instruction (interveningIntro (Just cond))) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 hs : HeaderStatus ev} ->
              {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
              {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing (Just cond)} ->
              AbilityAt bs
triggeredIf word ev cond instr =
  Triggered word ev [] Nothing [] Nothing Nothing (Just cond) instr {hn} {hs} {ae} {cd}

public export
triggeredOr : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (instr : Instruction (headerCtx alts ev)) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 hs : HeaderStatus ev} ->
              {auto 0 ae : AltEvent word alts} ->
              {auto 0 cd : ChapterDefaults ev alts Nothing [] Nothing Nothing Nothing} ->
              AbilityAt bs
triggeredOr word ev alts instr =
  Triggered word ev alts Nothing [] Nothing Nothing Nothing instr {hn} {hs} {ae} {cd}

public export
triggeredOnlyDuring : {bs : Bindings} -> (word : TriggerWord) ->
                      (ev : GameEvent bs) -> (w : TriggerWindow bs) ->
                      (instr : Instruction (eventAfter ev)) ->
                      {auto 0 hn : HeaderNontarget ev} ->
                      {auto 0 hs : HeaderStatus ev} ->
                      {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                      {auto 0 cd : ChapterDefaults ev [] Nothing [] (Just w) Nothing Nothing} ->
                      AbilityAt bs
triggeredOnlyDuring word ev w instr =
  Triggered word ev [] Nothing [] (Just w) Nothing Nothing instr {hn} {hs} {ae} {cd}

public export
triggeredWhile : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                 (wh : Concurrent (headerCtx (the (List (GameEvent bs)) []) ev)) ->
                 (instr : Instruction (eventAfter ev)) ->
                 {auto 0 hn : HeaderNontarget ev} ->
                 {auto 0 hs : HeaderStatus ev} ->
                 {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                 {auto 0 cd : ChapterDefaults ev [] (Just wh) [] Nothing Nothing Nothing} ->
                 AbilityAt bs
triggeredWhile word ev wh instr =
  Triggered word ev [] (Just wh) [] Nothing Nothing Nothing instr {hn} {hs} {ae} {cd}

public export
triggeredJoined : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                  (joins : List (JoinedHeader bs)) ->
                  (instr : Instruction (joinedCtx joins (headerCtx (the (List (GameEvent bs)) []) ev))) ->
                  {auto 0 hn : HeaderNontarget ev} ->
                  {auto 0 hs : HeaderStatus ev} ->
                  {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                  {auto 0 cd : ChapterDefaults ev [] Nothing joins Nothing Nothing Nothing} ->
                  AbilityAt bs
triggeredJoined word ev joins instr =
  Triggered word ev [] Nothing joins Nothing Nothing Nothing instr {hn} {hs} {ae} {cd}

public export
joinedHead : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
             {auto 0 hn : HeaderNontarget ev} ->
             {auto 0 hs : HeaderStatus ev} ->
             {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
             JoinedHeader bs
joinedHead word ev = MkJoinedHeader word ev [] Nothing Nothing {hn} {hs} {ae}

public export
joinedHeadWhile : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                  (wh : Concurrent (headerCtx (the (List (GameEvent bs)) []) ev)) ->
                  {auto 0 hn : HeaderNontarget ev} ->
                  {auto 0 hs : HeaderStatus ev} ->
                  {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                  JoinedHeader bs
joinedHeadWhile word ev wh = MkJoinedHeader word ev [] (Just wh) Nothing {hn} {hs} {ae}

public export
triggeredOnlyOnce : {bs : Bindings} -> (word : TriggerWord) ->
                    (ev : GameEvent bs) -> (lim : UsageLimit) ->
                    (instr : Instruction (eventAfter ev)) ->
                    {auto 0 hn : HeaderNontarget ev} ->
                    {auto 0 hs : HeaderStatus ev} ->
                    {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                    {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing (Just lim) Nothing} ->
                    AbilityAt bs
triggeredOnlyOnce word ev lim instr =
  Triggered word ev [] Nothing [] Nothing (Just lim) Nothing instr {hn} {hs} {ae} {cd}

public export
activated : (cost : Cost (dropLetter X bs)) ->
            (instr : Instruction (publicOnly (costIntro cost))) ->
            {auto 0 tp : CostTapOnce cost} ->
            {auto 0 py : CostPaidByYou cost} -> AbilityAt bs
activated cost instr = Activated cost instr Nothing Nothing Nothing Nothing {tp} {py}

public export
activatedBy : (cost : Cost (dropLetter X bs)) ->
              (instr : Instruction (publicOnly (costIntro cost))) ->
              (who : Noun bs Player) ->
              {auto 0 tp : CostTapOnce cost} ->
              {auto 0 py : CostPaidByYou cost} -> AbilityAt bs
activatedBy cost instr who =
  Activated cost instr Nothing Nothing Nothing (Just who) {tp} {py}

public export
activatedOnlyDuring : (cost : Cost (dropLetter X bs)) ->
                      (instr : Instruction (publicOnly (costIntro cost))) ->
                      (w : Timing bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyDuring cost instr w = Activated cost instr (Just w) Nothing Nothing Nothing {tp} {py}

public export
activatedOnlyOnce : (cost : Cost (dropLetter X bs)) ->
                    (instr : Instruction (publicOnly (costIntro cost))) ->
                    (lim : UsageLimit) ->
                    {auto 0 tp : CostTapOnce cost} ->
                    {auto 0 py : CostPaidByYou cost} ->
                    {auto 0 ul : So (untriggeredLimitOk (Just lim))} ->
                    AbilityAt bs
activatedOnlyOnce cost instr lim =
  Activated cost instr Nothing (Just lim) Nothing Nothing {tp} {py} {ul}

public export
activatedOnlyIf : (cost : Cost (dropLetter X bs)) ->
                  (instr : Instruction (publicOnly (costIntro cost))) ->
                  (g : Condition bs) ->
                  {auto 0 tp : CostTapOnce cost} ->
                  {auto 0 py : CostPaidByYou cost} ->
                  AbilityAt bs
activatedOnlyIf cost instr g = Activated cost instr Nothing Nothing (Just g) Nothing {tp} {py}

public export
activatedOnlyOnceIf : (cost : Cost (dropLetter X bs)) ->
                      (instr : Instruction (publicOnly (costIntro cost))) ->
                      (lim : UsageLimit) -> (g : Condition bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      {auto 0 ul : So (untriggeredLimitOk (Just lim))} ->
                      AbilityAt bs
activatedOnlyOnceIf cost instr lim g =
  Activated cost instr Nothing (Just lim) (Just g) Nothing {tp} {py} {ul}

public export
mayPlayDeed : (deed : VerbLabel) -> (who : Noun bs Player) ->
              (what : Noun (nomIntro who) Object) ->
              (asThough : Maybe (AsThough (nomIntro who))) ->
              (rider : DeonticRider (nomIntro what)) ->
              {auto 0 kd : KnownActs [deed]} ->
              {auto 0 dd : So (distinctDeeds [deed])} ->
              {auto 0 dp : DeedFits [deed] Agent Player (nounIsAbility who) (nounHeadTys who) (nounZone who)} ->
              {auto 0 pt : So (deonticPatientOk who [deed] Agent
                                                (DeonticCounterpart what)
                                                rider)} ->
              {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro who})
                                           [deed] asThough)} ->
              {auto 0 rd : So (deonticRiderOk [deed] Agent
                                              (Permit {bs = selfSubjIntro who})
                                              (DeonticCounterpart what) (isJust asThough)
                                              rider)} ->
              StaticSpec bs
mayPlayDeed deed who what asThough rider =
  Deontic who Permit [deed] Agent Nothing (DeonticCounterpart what) asThough rider
          {kd} {dd} {dp} {pt} {at} {rd}

public export
leavesBattlefield : {0 bs : Bindings} -> (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    GameEvent bs
leavesBattlefield n = Leaves n (Just (FromZone [battlefieldZ])) {zn}

public export
leavesZone : {0 bs : Bindings} -> (n : Noun bs Object) -> (z : ZoneExpr bs) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just (zoneSort z))} ->
             GameEvent bs
leavesZone n z = Leaves n (Just (FromZone [z])) {zn}

public export
putIntoFrom : (n : Noun bs Object) -> (to : ZoneExpr bs) -> (src : EventSource bs) ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource (Just src)} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone (Just src))} -> GameEvent bs
putIntoFrom n to src = PutInto n to (Just src) {dk} {sk} {zn}

public export
entersWithAdditionalCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                               (kind : CounterKind) ->
                               {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                               StaticSpec bs
entersWithAdditionalCounters n amt kind =
  EntersRider n (WithCounters amt (PrintedKind kind) Additional) {zn}

public export
entersWithFewerCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                          (kind : CounterKind) ->
                          {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                          StaticSpec bs
entersWithFewerCounters n amt kind =
  EntersRider n (WithCounters amt (PrintedKind kind) Fewer) {zn}

public export
attacks : (n : Noun bs Object) ->
          {auto 0 zn : ZoneIs (nounZone n) Battlefield} -> GameEvent bs
attacks n = Attacks n NoDefender {zn}

public export
attacksPlayer : {k : Kind} -> (n : Noun bs Object) -> (whom : Noun (nomIntro n) k) ->
                {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                {auto 0 sg : nounPlur whom = OneOf} ->
                {auto 0 at : Attackable whom} -> GameEvent bs
attacksPlayer n whom = Attacks n (OneDefender whom {sg} {at}) {zn}

public export
dealsCombatDamage : {k : Kind} -> (n : Noun bs Object) ->
                    (to : Noun (nomIntro n) k) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    {auto 0 rk : DamageRecipient to} -> GameEvent bs
dealsCombatDamage n to = DealsDamage CombatOnly n (OnePatient to {rk}) {zn}

public export
flipsCoin : (who : Noun bs Player) -> GameEvent bs
flipsCoin who = FlipsCoin who Nothing

public export
tokensCreated : (n : Noun bs Object) ->
                {auto 0 tk : TokenPhrase n} -> GameEvent bs
tokensCreated n = TokensCreated n False Nothing Nothing {tk}

public export
tokensCreatedUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                     {auto 0 tk : TokenPhrase n} ->
                     {auto 0 vo : So (creationVoiceOk False Nothing (Just under))} ->
                     GameEvent bs
tokensCreatedUnder n under = TokensCreated n False Nothing (Just under) {tk} {vo}

public export
tokensCreatedByEffectUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                             {auto 0 tk : TokenPhrase n} ->
                             {auto 0 vo : So (creationVoiceOk True Nothing
                                                              (Just under))} ->
                             GameEvent bs
tokensCreatedByEffectUnder n under =
  TokensCreated n True Nothing (Just under) {tk} {vo}

public export
counterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
               (many : CounterBatch) -> (n : Noun bs Object) ->
               {auto 0 sc : counterScope kind = Object} ->
               {auto 0 ba : So (counterBatchOk many dir (Just kind))} -> GameEvent bs
counterEvent dir kind many n =
  CounterEvent dir (Just kind) n many Nothing False

public export
bareCounterEvent : {k : Kind} -> (dir : CounterMove) ->
                   (many : CounterBatch) -> (n : Noun bs k) ->
                   {auto 0 ba : So (counterBatchOk many dir Nothing)} ->
                   GameEvent bs
bareCounterEvent dir many n = CounterEvent dir Nothing n many Nothing False

public export
manyCountersPutByEffect : (n : Noun bs Object) -> GameEvent bs
manyCountersPutByEffect n =
  CounterEvent CounterPut Nothing n ManyCounters Nothing True

public export
manyBareCountersPutBy : (who : Noun bs Player) -> (n : Noun bs Object) ->
                        {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
manyBareCountersPutBy who n =
  CounterEvent CounterPut Nothing n ManyCounters (Just who) False {ag}

public export
lastCounterRemoved : (kind : CounterKind) -> (n : Noun bs Object) ->
                     {auto 0 sc : counterScope kind = Object} -> GameEvent bs
lastCounterRemoved kind n =
  CounterEvent CounterTaken (Just kind) n LastCounter Nothing False
               {kn = Present {ok = sc}}

public export
lastCounterRemovedBy : (kind : CounterKind) -> (n : Noun bs Object) ->
                       (who : Noun bs Player) ->
                       {auto 0 sc : counterScope kind = Object} ->
                       {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
lastCounterRemovedBy kind n who =
  CounterEvent CounterTaken (Just kind) n LastCounter (Just who) False
               {kn = Present {ok = sc}} {ag}

public export
0 ExchangeCtx : {bs : Bindings} -> (t : CardType) -> (other : Noun bs Object) ->
                (0 one : nounPlur other = OneOf) -> (0 way : So (ascriptionOk t Nothing)) ->
                Bindings
ExchangeCtx t other one way =
  annIntro (gainControl (controllerOf other) (AsType t This Nothing {way}) Nothing)

public export
exchangeControlOfThis : {bs : Bindings} -> (t : CardType) -> (other : Noun bs Object) ->
                        {auto 0 one : nounPlur other = OneOf} ->
                        {auto 0 way : So (ascriptionOk t Nothing)} ->
                        {auto 0 pw : countReach (Word PermanentW) OneOf
                                       (ExchangeCtx t other one way) = 1} ->
                        {auto 0 zw : So (zoneIsB (zoneOfReach (Word PermanentW) OneOf
                                          (ExchangeCtx t other one way)) Battlefield)} ->
                        Instruction bs
exchangeControlOfThis t other =
  Simultaneously
    [ gainControl (controllerOf other) (AsType t This Nothing {way}) Nothing
    , gainControl (controllerOf (AsType t This Nothing {way}))
                  (That PermanentW OneOf {ok = pw}) Nothing {zn = zw} ]

public export
choose : {k : Kind} -> (n : Noun bs k) ->
         {auto 0 ch : So (choiceClauseOk (the (Maybe (Noun bs Player)) Nothing) n)} ->
         Instruction bs
choose n = Choose Nothing Nothing n Openly {ch}

public export
armyYouControl : {bs : Bindings} -> Predicate bs Object
armyYouControl =
  And [HasSubtype (creatureType "Army"), creature, HasPossessor ControllerAx You]

public export
0 AmassCtx : {bs : Bindings} -> Bindings
AmassCtx {bs} = instrIntro (choose {bs} (Macros.a armyYouControl))

public export
0 AmassAfter : {bs : Bindings} -> (n : Nat) ->
               (0 ac : countReach (Word (TypeW Creature)) OneOf (AmassCtx {bs}) = 1) -> Bindings
AmassAfter n ac =
  instrIntro (PutCounters {bs = AmassCtx {bs}} (Lit n) (PrintedKind plusOnePlusOne)
                        (That (TypeW Creature) OneOf {ok = ac}))

||| "amass [subtype] N" [CR#701.47a]
public export
amass : {bs : Bindings} -> (sub : String) -> (n : Nat) ->
        {auto 0 ac : countReach (Word (TypeW Creature)) OneOf (AmassCtx {bs}) = 1} ->
        {auto 0 ab : countReach Bare OneOf (AmassAfter {bs} n ac) = 1} ->
        Instruction bs
amass sub n =
  Sequentially [ If (NotCond (exists armyYouControl))
                    (create (Lit 1) (creatureTok 0 0 [Black]
                                       [creatureType sub, creatureType "Army"]))
                    Nothing
               , choose (Macros.a armyYouControl)
               , PutCounters (Lit n) (PrintedKind plusOnePlusOne)
                             (That (TypeW Creature) OneOf {ok = ac})
               , If (itIsntA (HasSubtype (creatureType sub)) {ok = ab})
                    (becomes (It OneOf {ok = ab}) (subtypesOnly [creatureType sub]) Nothing)
                    Nothing ]


public export
chooses : {k : Kind} -> (who : Noun bs Player) ->
          (n : Noun (Experimental.Phrase.agentIntro who) k) ->
          {auto 0 ch : So (choiceClauseOk (Just who) n)} -> Instruction bs
chooses who n = Choose Nothing (Just who) n Openly {ch}

public export
secretlyChooses : {k : Kind} -> (who : Noun bs Player) ->
                  (n : Noun (Experimental.Phrase.agentIntro who) k) ->
                  {auto 0 ch : So (choiceClauseOk (Just who) n)} -> Instruction bs
secretlyChooses who n = Choose Nothing (Just who) n Secretly {ch}

public export
itPrior : {bs : Bindings} -> (prev : Instruction bs) ->
          {auto 0 ok : countReach Bare OneOf
                         (view (Top (length (instrDelta prev)))
                               (instrIntro prev)) = 1} ->
          Noun (instrIntro prev) Object
itPrior prev = Pro Bare OneOf (Top (length (instrDelta prev))) {ok}

public export
dealsDamageOwnPower : {bs : Bindings} -> {k : Kind} -> (src : Noun bs Object) ->
                      {auto 0 ok : countReach Bare OneOf
                                     (view (Top (length (nounDelta src)))
                                           (nomIntro src)) = 1} ->
                      {auto 0 ty : So (statHeadTysOk Power
                                        (soleAlt (optCT
                                          (tyOfReach Bare OneOf
                                            (view (Top (length (nounDelta src)))
                                                  (nomIntro src))))))} ->
                      (to : Noun (nounDelta src ++ bs) k) ->
                      {auto 0 pm : PerMember to} ->
                      {auto 0 rk : DamageRecipient to} -> Instruction bs
dealsDamageOwnPower src to =
  DealDamage src
    (StatOf Power (Pro Bare OneOf (Top (length (nounDelta src))) {ok}) {ty})
    to {pm} {rk}

public export
comparesOwnStat : {bs : Bindings} -> {k : Kind} -> (c : Stat) ->
                  (dom : Predicate bs k) -> {auto ph : Phrasal k} ->
                  (r : Comparator) -> (bound : Amount bs) ->
                  {auto 0 ok : countReach Bare OneOf
                                 [Experimental.Phrase.bindFor TheD OneOf ph dom] = 1} ->
                  {auto 0 ty : So (statHeadTysOk c
                                    (nounHeadTys
                                      (Pro Bare OneOf (Top 1)
                                        {bs = Experimental.Phrase.bindFor TheD OneOf ph dom
                                                :: (Experimental.Phrase.predDelta dom ++ bs)}
                                        {ok})))} ->
                  Predicate bs k
comparesOwnStat c dom r bound =
  CompareOver dom (StatOf c (Pro Bare OneOf (Top 1) {ok}) {ty}) r bound

public export
additionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                 (count : Amount bs) ->
                 {auto 0 ad : AddedPart part} ->
                 {auto 0 an : AddedPartWritten anchor} -> Instruction bs
additionalPart part anchor count =
  AdditionalPart Nothing part anchor count Nothing {ad} {an}

public export
getsAdditionalPart : (who : Noun bs Player) -> (part : TurnPart) ->
                     (count : Amount bs) ->
                     {auto 0 ad : AddedPart part} -> Instruction bs
getsAdditionalPart who part count =
  AdditionalPart (Just who) part Nothing count Nothing {ad}

public export
additionalPartThen : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) -> (next : TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten (Just next)} -> Instruction bs
additionalPartThen part anchor count next =
  AdditionalPart Nothing part anchor count (Just next) {ad} {an} {fb}

public export
vote : (voters : Noun bs Player) -> (disc : Disclosure) ->
       (ballot : Ballot (nomIntro voters)) -> Instruction bs
vote voters disc ballot = Vote Nothing voters disc ballot

public export
shiftResult : (amt : Amount bs) ->
              {auto 0 ok : countOutcomes RollResult bs = 1} -> Instruction bs
shiftResult amt = ShiftResult Nothing amt {ok}

public export
chaosEnsues : Instruction bs
chaosEnsues = ChaosEnsues Nothing

public export
delayed : (ev : GameEvent bs) -> (instr : Instruction (delayedCtx [] ev)) -> Instruction bs
delayed ev instr = Delayed ev [] Nothing instr

public export
delayedWithin : (ev : GameEvent bs) -> (span : Duration bs) ->
                (instr : Instruction (delayedCtx [] ev)) ->
                {auto 0 so : DelaySpanOk (Just span)} -> Instruction bs
delayedWithin ev span instr = Delayed ev [] (Just span) instr {so}

public export
quality : (q : QualitySort) -> Predicate bs (Quality q)
quality q = QualityNoun q Nothing

public export
qualityFrom : (q : QualitySort) -> (d : ChoiceDomain (QSort q)) -> Predicate bs (Quality q)
qualityFrom q d = QualityNoun q (Just d)

public export
entersChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 StaticSpec bs
entersChoosing n q = EntersChoice n (QSort q) Nothing Openly {zn}

public export
entersChoosingFrom : (n : Noun bs Object) -> (q : QualitySort) ->
                     (d : ChoiceDomain (QSort q)) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticSpec bs
entersChoosingFrom n q d = EntersChoice n (QSort q) (Just d) Openly {zn}

public export
entersChoosingPlayer : (n : Noun bs Object) ->
                       (d : Maybe (ChoiceDomain PlayerC)) ->
                       {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                       StaticSpec bs
entersChoosingPlayer n d = EntersChoice n PlayerC d Openly {zn}

public export
entersChoosingPlayerSecretly : (n : Noun bs Object) ->
                               (d : Maybe (ChoiceDomain PlayerC)) ->
                               {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                               StaticSpec bs
entersChoosingPlayerSecretly n d = EntersChoice n PlayerC d Secretly {zn}

public export
attachChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 StaticSpec bs
attachChoosing n q = AttachChoice n (QSort q) Nothing {zn}

public export
thisSiege : Noun bs Object
thisSiege = AsType Battle This (Just (battleType "Siege"))

public export
happenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement bs ev k)) Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
happenedTo ev w = HappenedTo (MkLookback ev w Nothing {cw})

public export
happenedToInvolving : {ks : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (w : Lookback) -> (what : Noun bs kc) ->
                      {auto 0 cp : LookbackComplement ev ks kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev ks} -> Predicate bs ks
happenedToInvolving ev w what =
  HappenedTo (MkLookback ev w (Just (Involving what {cp})) {cw} {sb})

public export
happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
           (w : Lookback) ->
           {auto 0 cw : ComplementWritten
                          (the (Maybe (EventComplement (nomIntro who) ev k))
                               Nothing)} ->
           {auto 0 sb : LookbackSubject ev k} -> Condition bs
happened ev who w = Happened who (MkLookback ev w Nothing {cw})

public export
happenedInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                    (who : Noun bs k) -> (w : Lookback) ->
                    (what : Noun (nomIntro who) kc) ->
                    {auto 0 cp : LookbackComplement ev k kc} ->
                    {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                    {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedInvolving ev who w what =
  Happened who (MkLookback ev w (Just (Involving what {cp})) {cw} {sb})

public export
eventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
             (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement (nomIntro who) ev k))
                                 Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCount ev who w = EventTally TallyCount who (MkLookback ev w Nothing {cw})

public export
eventSum : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
           (w : Lookback) ->
           {auto 0 cw : ComplementWritten
                          (the (Maybe (EventComplement (nomIntro who) ev k))
                               Nothing)} ->
           {auto 0 sb : LookbackSubject ev k} ->
           {auto 0 qm : So (tallyOk TallySum ev)} -> Amount bs
eventSum ev who w = EventTally TallySum who (MkLookback ev w Nothing {cw} {sb}) {qm}

public export
eventCountInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (who : Noun bs k) -> (w : Lookback) ->
                      (what : Noun (nomIntro who) kc) ->
                      {auto 0 cp : LookbackComplement ev k kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountInvolving ev who w what =
  EventTally TallyCount who (MkLookback ev w (Just (Involving what {cp})) {cw} {sb})

public export
happenedFrom : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
               (who : Noun bs k) -> (w : Lookback) ->
               (what : Noun (nomIntro who) kc) ->
               (src : EventSource (nomIntro who)) ->
               {auto 0 cp : LookbackComplement ev k kc} ->
               {auto 0 pl : So (complementPlain
                                  (Just (Involving {bs = nomIntro who} what {cp})))} ->
               {auto 0 zo : So (lookbackSourceOk ev src)} ->
               {auto 0 cw : ComplementWritten
                              (Just (FromZones src
                                       (Just (Involving what {cp})) {pl} {ok = zo}))} ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedFrom ev who w what src =
  Happened who
    (MkLookback ev w
       (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb})

public export
eventCountFrom : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                 (who : Noun bs k) -> (w : Lookback) ->
                 (what : Noun (nomIntro who) kc) ->
                 (src : EventSource (nomIntro who)) ->
                 {auto 0 cp : LookbackComplement ev k kc} ->
                 {auto 0 pl : So (complementPlain
                                    (Just (Involving {bs = nomIntro who} what {cp})))} ->
                 {auto 0 zo : So (lookbackSourceOk ev src)} ->
                 {auto 0 cw : ComplementWritten
                                (Just (FromZones src
                                         (Just (Involving what {cp})) {pl} {ok = zo}))} ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountFrom ev who w what src =
  EventTally TallyCount who
    (MkLookback ev w
       (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb})

public export
happenedAt : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
             (w : Lookback) -> (z : ZoneExpr (nomIntro who)) ->
             {auto 0 zo : So (lookbackLocusOk ev (zoneSort z))} ->
             {auto 0 cw : ComplementWritten
                            (Just (AtZone {bs = nomIntro who} {ev} {ks = k}
                                          z {ok = zo}))} ->
             {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedAt ev who w z = Happened who (MkLookback ev w (Just (AtZone z {ok = zo})) {cw} {sb})

public export
thatTurns : {auto 0 ok : countReach ThatTurn OneOf bs = 1} -> HeaderPossessor bs
thatTurns = ByTurn (thatTurn {ok})

public export
beginningOfPossessed : (q : PartQuant) -> (part : TurnPart) ->
                       (poss : Noun bs Player) ->
                       {auto 0 pu : PartTriggerable part (ByPlayer poss)} ->
                       GameEvent bs
beginningOfPossessed q part poss = BeginningOf q part (ByPlayer poss) {pu}

public export
gainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                   (w : GivingWarrant d) ->
                   {auto 0 sc : designationScope d = HeldBy k} ->
                   {auto 0 zn : DesignationHolder d (nounZone n)} -> Instruction bs
gainsDesignation n d w = GainsDesignation n d w Nothing {sc} {zn}

||| "monstrosity N" [CR#701.37a]: this permanent, not only a creature.
public export
monstrosity : {bs : Bindings} -> (amt : Amount bs) ->
              Instruction bs
monstrosity amt =
  If (NotCond (Matches thisPermanent (HasDesignation Monstrous Nothing)))
     (Sequentially [ PutCounters amt (PrintedKind plusOnePlusOne) thisPermanent
                   , GainsDesignation thisPermanent Monstrous
                                      (InExpansionOf MonstrosityW) Nothing ])
     Nothing

public export
getsCitysBlessing : Instruction bs
getsCitysBlessing =
  GainsDesignation You CitysBlessing (InExpansionOf AscendW) (Just RestOfGame)

public export
becomesSaddled : Instruction bs
becomesSaddled =
  GainsDesignation (AsType Artifact This Nothing) Saddled (InExpansionOf SaddleW) (Just untilEndOfTurn)

public export
getsEnduringStory : Instruction bs
getsEnduringStory =
  GainsDesignation You EnduringStory (InExpansionOf StoriedW) (Just RestOfGame)

||| The renown expansion [CR#702.112a].
public export
renownExpansion : (n : Nat) -> AbilityAt []
renownExpansion n =
  triggeredIf When
    (dealsCombatDamage thisCreature (a AnyPlayer))
    (NotCond (Matches thisCreature (HasDesignation Renowned Nothing)))
    (Sequentially [ PutCounters (Lit n) (PrintedKind plusOnePlusOne) thisCreature
                  , GainsDesignation thisCreature Renowned
                                     (InExpansionOf RenownW) Nothing ])

public export
renown : {0 bs : Bindings} -> (n : Nat) -> AbilityAt bs
renown n =
  KeywordAbility "Renown" (Just (ParamNumber (Lit n))) (Just (renownExpansion n))

||| The storm expansion [CR#702.40a].
public export
stormExpansion : AbilityAt []
stormExpansion =
  triggered When (Casts You thisSpell Nothing)
    (Sequentially
       [ Copy FromStack You thisSpell
           (eventCountInvolving SpellCast (a AnyPlayer) EarlierThisTurn
              (a (And [spell, OtherThan thisSpell])))
           []
       , may You (ChooseNewTargets (Pro (Word CopyW) ManyOf Whole)) ])

public export
storm : {0 bs : Bindings} -> AbilityAt bs
storm = KeywordAbility "Storm" Nothing (Just stormExpansion)

||| The cumulative upkeep expansion [CR#702.24a].
public export
cumulativeUpkeepExpansion : (c : Cost []) ->
                            {auto 0 pb : Payable c} ->
                            {auto 0 py : CostPaidByYou c} -> AbilityAt []
cumulativeUpkeepExpansion c =
  triggeredIf At (BeginningOf ThePart Upkeep (ByPlayer You))
    (Matches thisPermanent (InZone battlefieldZ))
    (Sequentially
       [ PutCounters (Lit 1) (PrintedKind (NamedCounter "Age")) thisPermanent
       , May You (Pay You (ScaledCost c (times 1 (CountersOn (NamedCounter "Age") thisPermanent)))
                          PaidOnce {pb} {ag = py})
             Nothing (Just (sacrifice You thisPermanent)) ])

public export
cumulativeUpkeep : {0 bs : Bindings} -> (c : Cost []) ->
                   {auto 0 pb : Payable c} ->
                   {auto 0 py : CostPaidByYou c} -> AbilityAt bs
cumulativeUpkeep c =
  KeywordAbility "CumulativeUpkeep" (Just (ParamCost c))
    (Just (cumulativeUpkeepExpansion c {pb} {py}))

public export
yourCommander : Noun bs Object
yourCommander = Designated CommanderD You

public export
thereIsNo : (d : Designation) ->
            {auto 0 sc : designationScope d = HeldBy Player} ->
            {auto 0 at : So (designationChecked d)} -> Condition bs
thereIsNo d = NoHolder d {sc} {at}

public export
ifThen : (c : Condition bs) -> Instruction (condIntro c) -> Instruction bs
ifThen c e = If c e Nothing

public export
onlyWhile : {bs : Bindings} -> (se : StaticSpec bs) -> (c : Condition (staticIntro se)) -> StaticSpec bs
onlyWhile {bs} se c = Conditionally {bs} se c AsLongAs

public export
onlyUnless : {bs : Bindings} -> (se : StaticSpec bs) -> (c : Condition (staticIntro se)) -> StaticSpec bs
onlyUnless {bs} se c = Conditionally {bs} se (NotCond c) Unless

public export
onlyIfSo : {bs : Bindings} -> (se : StaticSpec bs) -> (c : Condition (staticIntro se)) -> StaticSpec bs
onlyIfSo {bs} se c = Conditionally {bs} se c IfSo

public export
throughout : {bs : Bindings} -> (se : StaticSpec bs) -> (span : Duration (staticIntro se)) ->
             {auto 0 sp : SpanOk (Just span)} ->
             {auto 0 cl : ClauseStatic se} -> Instruction bs
throughout {bs} se span = Continuously {bs} se (Just span) {sp} {cl}

public export
fromTo : Nat -> Nat -> Quantity bs
fromTo lo hi = Range (Just lo) (Just hi)

public export
flipCoins : (who : Noun bs Player) -> (count : Nat) -> Instruction bs
flipCoins who count = FlipCoins who (FlipCount (Lit count))

public export
rollDice : (who : Noun bs Player) -> (count : Nat) -> (sides : Nat) ->
           {auto 0 nz : IsSucc sides} -> Instruction bs
rollDice who count sides = RollDice who (Lit count) (SidesOf sides {nz})

public export
rollRow : (results : Quantity bs) -> (e : Instruction bs) ->
          {auto 0 nz : NonZeroQ results} ->
          {auto 0 wf : WellFormedQ results} ->
          {auto 0 lt : So (quantLiteral results)} -> RollRow bs
rollRow results e = MkRollRow results e {nz} {wf}

public export
resultsTable : (rows : List (RollRow bs)) ->
               {auto 0 ne : IsSucc (rowCount rows)} ->
               {auto 0 ok : countOutcomes RollResult bs = 1} -> Instruction bs
resultsTable rows = ResultsTable rows {ne} {ok}

public export
youRollResultIn : (q : Quantity bs) ->
                  {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} ->
                  {auto 0 lt : So (quantLiteral q)} -> GameEvent bs
youRollResultIn q = RollsDice You OneDie AnyDie (ResultIn q {nz} {wf} {lt})

public export
youRollHighestNatural : GameEvent bs
youRollHighestNatural = RollsDice You OneDie AnyDie HighestNatural

public export
youRollPlanarDice : GameEvent bs
youRollPlanarDice = RollsDice You ManyDice PlanarDie AnyResult

public export
rollThePlanarDie : Instruction bs
rollThePlanarDie = RollPlanarDie You (Lit 1)

public export
preventedThisWay : {auto 0 ok : countOutcomes DamagePrevented bs = 1} -> Amount bs
preventedThisWay = TheOutcome DamagePrevented {ok}

public export
removedThisWay : {auto 0 ok : countOutcomes CountersRemoved bs = 1} -> Amount bs
removedThisWay = TheOutcome CountersRemoved {ok}

public export
shortOfCeiling : {auto 0 ok : countOutcomes CeilingShortfall bs = 1} -> Amount bs
shortOfCeiling = TheOutcome CeilingShortfall {ok}

||| "outlaw": an object with any of the five outlaw creature types [CR#700.12].
public export
outlaw : {0 bs : Bindings} -> Predicate bs Object
outlaw = Or [ HasSubtype (creatureType "Assassin")
            , HasSubtype (creatureType "Mercenary")
            , HasSubtype (creatureType "Pirate")
            , HasSubtype (creatureType "Rogue")
            , HasSubtype (creatureType "Warlock") ]

||| "outlaws you control": outlaw permanents only [CR#700.12a].
public export
outlawYouControl : {0 bs : Bindings} -> Predicate bs Object
outlawYouControl = And [outlaw, HasPossessor ControllerAx You]

||| The four party roles, in rule order [CR#700.8].
public export
partyRoles : {0 bs : Bindings} -> List (Predicate bs Object)
partyRoles = [ HasSubtype (creatureType "Cleric")
             , HasSubtype (creatureType "Rogue")
             , HasSubtype (creatureType "Warrior")
             , HasSubtype (creatureType "Wizard") ]

||| "[a player]'s party": the joint one-each group the game computes over the
||| creatures that player controls [CR#700.8,700.8a].
public export
partyOf : {bs : Bindings} -> (who : Noun bs Player) ->
          {auto 0 sh : SoleHolder who} -> Noun bs Object
partyOf who = OneEachOf partyRoles (allOf (And [creature, HasPossessor ControllerAx who {ps = sh}]))

||| "your party" [CR#700.8].
public export
party : {bs : Bindings} -> Noun bs Object
party = partyOf You

||| "the number of creatures in [a player]'s party" [CR#700.8a].
public export
partySizeOf : {bs : Bindings} -> (who : Noun bs Player) ->
              {auto 0 sh : SoleHolder who} -> Amount bs
partySizeOf who = CountOf (partyOf who {sh})

||| "the number of creatures in your party" [CR#700.8a].
public export
partySize : {bs : Bindings} -> Amount bs
partySize = partySizeOf You

||| "[a player] has a full party": four creatures in that party [CR#700.8c].
public export
fullPartyOf : {bs : Bindings} -> (who : Noun bs Player) ->
              {auto 0 sh : SoleHolder who} -> Condition bs
fullPartyOf who = CompareAmt (partySizeOf who {sh}) Eq (Lit 4)

||| "you have a full party" [CR#700.8c].
public export
fullParty : {bs : Bindings} -> Condition bs
fullParty = fullPartyOf You
