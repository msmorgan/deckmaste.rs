module Experimental.Macros

import Data.List
import public Experimental

%default total

public export
It : {auto 0 ok : countReach Bare OneOf bs = 1} -> Noun bs Object
It = Pro Bare OneOf

public export
ItAbility : {auto 0 ok : countReach (Word AbilityW) OneOf bs = 1} -> Noun bs Ability
ItAbility = Pro (Word AbilityW) OneOf

public export
ItAt : (sl : SlotCarrier) -> {auto 0 ok : countReach (AtSlot sl) OneOf bs = 1} ->
       Noun bs Object
ItAt sl = Pro (AtSlot sl) OneOf

public export
ItVerbed : (v : VerbLabel) -> {auto 0 kn : KnownAct v} ->
           {auto 0 ok : countReach (Stamped v) OneOf bs = 1} -> Noun bs Object
ItVerbed v = Pro (Stamped v) OneOf

public export
ItToken : {auto 0 ok : countReach TokenBorn OneOf bs = 1} -> Noun bs Object
ItToken = Pro TokenBorn OneOf

public export
They : {auto 0 ok : countReach (Word PlayerW) OneOf bs = 1} -> Noun bs Player
They = Pro (Word PlayerW) OneOf

public export
Them : {auto 0 ok : countReach Bare ManyOf bs = 1} -> Noun bs Object
Them = Pro Bare ManyOf

public export
ThemVerbed : (v : VerbLabel) -> {auto 0 kn : KnownAct v} ->
             {auto 0 ok : countReach (Stamped v) ManyOf bs = 1} -> Noun bs Object
ThemVerbed v = Pro (Stamped v) ManyOf

public export
Those : (w : NounWord) -> {auto 0 ok : countReach (Word w) ManyOf bs = 1} ->
        Noun bs (kindOfW w)
Those w = Pro (Word w) ManyOf

public export
That : (w : NounWord) -> {auto 0 ok : countReach (Word w) OneOf bs = 1} ->
       Noun bs (kindOfW w)
That w = Pro (Word w) OneOf

public export
thatTurn : {auto 0 ok : countReach ThatTurn OneOf bs = 1} -> Noun bs TurnRef
thatTurn = Pro ThatTurn OneOf

public export
ThatHalf : (w : NounWord) ->
           {auto 0 ok : countReach (UnionHalf w) OneOf bs = 1} ->
           Noun bs (kindOfW w)
ThatHalf w = Pro (UnionHalf w) OneOf

public export
TheVerbed : (v : VerbLabel) -> (w : NounWord) -> (marking : VerbedMarking) ->
            {auto 0 ok : countReach (Verbed v w marking) OneOf bs = 1} ->
            {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
TheVerbed v w marking = Pro (Verbed v w marking) OneOf

public export
ThoseVerbed : (v : VerbLabel) -> (w : NounWord) -> (marking : VerbedMarking) ->
              {auto 0 ok : countReach (Verbed v w marking) ManyOf bs = 1} ->
              {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
ThoseVerbed v w marking = Pro (Verbed v w marking) ManyOf


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
target : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
         {auto 0 tk : Targetable k} -> Noun bs k
target p = Described (TargetDet (exactly 1)) p {ph} {ok = (MaxAtLeastOne, Oh, tk)}

public export
targets : (q : Quantity bs) -> (p : Predicate bs k) -> {auto ph : Phrasal k} ->
          {auto 0 ok : detOk (TargetDet q) p} -> Noun bs k
targets q p = Described (TargetDet q) p {ph} {ok}

public export
each : (p : Predicate bs k) -> {auto ph : Phrasal k} -> Noun bs k
each p = Described EachDet p {ph} {ok = ()}

public export
allOf : (p : Predicate bs k) -> {auto ph : Phrasal k} -> Noun bs k
allOf p = Described AllDet p {ph} {ok = ()}

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
kindJoin : (who : Predicate bs Player) -> (what : Predicate bs Object) ->
           Predicate bs (Object \/ Player)
kindJoin who what = Joined what who

public export
youAnd : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youAnd n = Both You n

public export
youOr : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youOr n = EitherOf You n

public export
thatJoin : {auto 0 ok : countReach (Word JoinW) OneOf bs = 1} -> Noun bs (Object \/ Player)
thatJoin = That JoinW {ok}

public export
controllerOf : {k : Kind} -> (n : Noun bs k) ->
               {auto 0 one : nounPlur n = OneOf} ->
               {auto 0 ck : So (possessorKind ControllerAx k)} -> Noun bs Player
controllerOf n = PossessorOf ControllerAx n {one} {ck}

public export
ownerOf : {k : Kind} -> (n : Noun bs k) ->
          {auto 0 one : nounPlur n = OneOf} ->
          {auto 0 ck : So (possessorKind OwnerAx k)} -> Noun bs Player
ownerOf n = PossessorOf OwnerAx n {one} {ck}

public export
thatSplitController : (cls : Noun bs Object) ->
                      {auto 0 one : nounPlur cls = OneOf} ->
                      {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                      Noun bs Player
thatSplitController cls = EitherOf (ThatHalf PlayerW {ok = pk}) (Macros.controllerOf cls {one})

public export
splitOverPlaneswalker : {bs : Bindings} ->
                        {auto 0 ck : countReach (UnionHalf (TypeW Planeswalker)) OneOf bs = 1} ->
                        {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                        Noun bs Player
splitOverPlaneswalker = thatSplitController (ThatHalf (TypeW Planeswalker) {ok = ck}) {pk}

public export
splitOverPermanent : {bs : Bindings} ->
                     {auto 0 ck : countReach (UnionHalf PermanentW) OneOf bs = 1} ->
                     {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                     Noun bs Player
splitOverPermanent = thatSplitController (ThatHalf PermanentW {ok = ck}) {pk}


public export
battlefieldZ : ZoneExpr bs
battlefieldZ = ZoneAt Battlefield Bare

public export
exileZ : ZoneExpr bs
exileZ = ZoneAt Exile Bare

public export
handZ : ZoneExpr bs
handZ = ZoneAt Hand Bare

public export
stackZ : ZoneExpr bs
stackZ = ZoneAt Stack Bare

public export
commandZ : ZoneExpr bs
commandZ = ZoneAt Command Bare

public export
spell : Predicate bs Object
spell = InZone stackZ

public export
frontFace : (name : String) -> (cost : Maybe ManaCost) ->
            (supers : List Supertype) -> (line : TypeLine) ->
            (text : AbilitySeq (costLetters cost)) -> (box : Maybe PrintedBox) ->
            CardFace
frontFace name cost supers line text box = MkFace name cost [] supers line text box

public export
backFace : (name : String) -> (supers : List Supertype) -> (line : TypeLine) ->
           (text : AbilitySeq []) -> (box : Maybe PrintedBox) -> CardFace
backFace name supers line text box = MkFace name Nothing [] supers line text box

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
  SingleFaced (MkFace name cost [] supers line text box)
              {fl = MkFaceLaws {ln} {sp} {tx} {ch} {bx} {mc} {dr}}

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
                    (MkFace name cost [] supers line text (printedBox stats))} ->
       Card
card name cost supers line text stats =
  SingleFaced (MkFace name cost [] supers line text (printedBox stats)) {fl}

public export
jointCard : (choices : List QualitySort) ->
            (name : String) -> (cost : Maybe ManaCost) ->
            (supers : List Supertype) -> (line : TypeLine) ->
            (text : AbilitySeq (jointBindings choices (costLetters cost))) ->
            (stats : Maybe (Integer, Integer)) ->
            {auto 0 ln : CardLine line} ->
            {auto 0 sp : CardSupers supers} ->
            {auto 0 tx : CardText line text} ->
            {auto 0 ch : CardChapters line text} ->
            {auto 0 bx : CardBox Front line text (printedBox stats)} ->
            {auto 0 mc : CardCost Front line cost} ->
            {auto 0 dr : DoorFrame text} ->
            {auto 0 jc : JointChoices choices text} -> Card
jointCard choices name cost supers line text stats =
  SingleFaced
    (MkFace name cost choices supers line text (printedBox stats))
    {fl = MkFaceLaws {ln} {sp} {tx} {ch} {bx} {mc} {dr} {jc}}

public export
counterSpell : {k : Kind} -> (n : Noun bs k) ->
               {auto 0 ct : Counterable n} -> Effect bs
counterSpell n = CounterSpell n {ct}

public export
graveyardZ : ZoneExpr bs
graveyardZ = ZoneAt Graveyard Bare

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
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> StaticEffect bs
hasBasePt n pow tou = Gets Sets n (PtUp pow) (PtUp tou) {ok}

public export
unblocked : Predicate bs Object
unblocked = Not Blocked

public export
times : (per : Nat) -> (a : Amount bs) ->
        {auto 0 nz : So (amtNonZero (Lit {bs} per))} -> Amount bs
times per a = TimesOf (Lit per) a {nz}

public export
theRest : {auto 0 ok : So (theRestOk bs)} -> Noun bs Object
theRest = TheRest ManyOf {ok}

public export
theOther : {auto 0 ok : So (theOtherOk bs)} -> Noun bs Object
theOther = TheRest OneOf {ok}

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
powerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
powerOf n = StatOf Power n {one}

public export
toughnessOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
toughnessOf n = StatOf Toughness n {one}

public export
manaValueOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
manaValueOf n = StatOf ManaValue n {one}


public export
nForEach : {bs : Bindings} -> {k : Kind} -> (n : Nat) -> (p : Predicate bs k) ->
           {auto ph : Phrasal k} ->
           {auto 0 pl : nounPlur (bare p {ph}) = ManyOf} ->
           {auto 0 nz : So (amtNonZero (Lit {bs} n))} ->
           Amount bs
nForEach n p = times n (countOf p {ph} {pl}) {nz}

public export
forEach : {bs : Bindings} -> {k : Kind} -> (p : Predicate bs k) ->
          {auto ph : Phrasal k} ->
          {auto 0 pl : nounPlur (bare p {ph}) = ManyOf} ->
          Amount bs
forEach p = nForEach 1 p {ph} {pl}

public export
move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur what) to} ->
       {auto 0 pl : Placeable (nounTy what) (zoneSort to)} -> Effect bs
move what to = Move what to [] {ok} {arr} {pl}

public export
destroy : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
          Effect bs
destroy n = Enact Nothing "Destroy" (Move n graveyardZ [])

public export
exile : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
        Effect bs
exile agent n = Enact (Just agent) "Exile" (Move n exileZ [])

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) -> Effect bs
exileWithCounters n amt kind =
  Enact Nothing "Exile"
        (Move n exileZ [WithCounters amt (PrintedKind kind) Fresh])

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ [ Under who {one = OneController {one}}
                      , WithCounters amt (PrintedKind kind) Fresh ] {pl}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ [] {pl}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ [EntersTapped] {pl}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ [EntersTapped, EntersAttacking NoDefender] {pl}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ [Under You] {pl}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> Effect bs
sacrifice agent n =
  Enact (Just agent) "Sacrifice" (Move n graveyardZ [])

public export
sacrificeIt : (agent : Noun bs Player) ->
              {auto 0 ok : countReach (AtSlot PermanentSlot) OneOf (agentIntro agent) = 1} ->
              {auto 0 zn : ZoneIs (zoneOfReach (AtSlot PermanentSlot) OneOf (agentIntro agent)) Battlefield} ->
              Effect bs
sacrificeIt agent = sacrifice agent (ItAt PermanentSlot {ok}) {ok = zn}

public export
discard : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
          {auto 0 dk : DiscardOk n} -> Effect bs
discard agent n =
  Enact (Just agent) "Discard" (Move n graveyardZ [])

public export
tap : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
      Effect bs
tap n = Enact Nothing "Tap" (SetStatus Tapped n)

public export
untap : (n : Noun bs Object) -> {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
        Effect bs
untap n = Enact Nothing "Untap" (SetStatus Untapped n)

public export
transform : (n : Noun bs Object) ->
            {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> Effect bs
transform n = Enact Nothing "Transform" (TurnOver n)

public export
meldInto : (n : Noun bs Object) -> (into : String) ->
           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
           Effect bs
meldInto n into =
  Enact Nothing "Meld"
        (Move n battlefieldZ [EntersMelded into] {arr} {pl})

public export
returnTo : (n : Noun bs Object) -> (to : ZoneExpr (nomIntro n)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur n) to} ->
           {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
           Effect bs
returnTo n to = Enact Nothing "Return" (Move n to [] {ok} {arr} {pl})

public export
returnToBattlefieldTransformed :
  (n : Noun bs Object) -> (ctrl : Noun (nomIntro n) Player) ->
  {auto 0 one : CtrlOverrideOk ctrl} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldTransformed n ctrl =
  Enact Nothing "Return"
        (Move n battlefieldZ [EntersTransformed, Under ctrl {one}] {arr} {pl})

public export
returnToBattlefield : (n : Noun bs Object) ->
                      {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                      {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                      Effect bs
returnToBattlefield n = returnTo n battlefieldZ {arr} {pl}

public export
itAsToken : {auto 0 ok : countReach TokenBorn OneOf bs = 1} -> Noun bs Object
itAsToken = ItToken {ok}

public export
ownSubject : {bs : Bindings} -> (n : Noun bs Object) ->
             {auto 0 ok : countReach Bare (nounPlur n) (selfSubjDelta n ++ nounDelta n) = 1} ->
             Noun (selfSubjIntro n) Object
ownSubject n =
  Own (nounPlur n) (selfSubjDelta n ++ nounDelta n) bs
      {sp = appendAssociative (selfSubjDelta n) (nounDelta n) bs} {ok}

public export
sharedSubject : {bs : Bindings} -> {0 k : Nat} -> (n : Noun bs Object) ->
                (parts : StaticParts k (selfSubjIntro n)) ->
                {auto 0 ne : IsSucc k} ->
                (d : Maybe (Duration (partsIntro parts))) ->
                {auto 0 sp : SpanOk d} ->
                {auto 0 cl : ClauseStatic (AndAlso (Just n) parts {ne})} -> Effect bs
sharedSubject {bs} n parts d = Continuously {bs} (AndAlso (Just n) parts {ne}) d {sp} {cl}


public export
dealsDivided : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
               (among : Noun (amtIntro amt) k) ->
               {auto 0 gm : GroupMention among} ->
               {auto 0 rk : DamageRecipient among} -> Effect bs
dealsDivided src amt among =
  Distribute (DividedDamage src) amt among {gm}
             {tk = DamageDivided {rk}}

public export
distributeCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (among : Noun (amtIntro amt) Object) ->
                     {auto 0 gm : GroupMention among} -> Effect bs
distributeCounters amt kind among =
  Distribute (DistributedCounters kind) amt among {gm}
             {tk = CountersDistributed}


public export
thisTurn : Duration bs
thisTurn = ThisTurn

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
asLongAs : {bs : Bindings} -> (c : Condition bs) -> (se : StaticEffect (condIntro c)) -> StaticEffect bs
asLongAs {bs} c se = Conditionally {bs} c se AsLongAs

public export
unlessSo : {bs : Bindings} -> (c : Condition bs) -> (se : StaticEffect (condIntro (NotCond c))) -> StaticEffect bs
unlessSo {bs} c se = Conditionally {bs} (NotCond c) se Unless

public export
ifSo : {bs : Bindings} -> (c : Condition bs) -> (se : StaticEffect (condIntro c)) -> StaticEffect bs
ifSo {bs} c se = Conditionally {bs} c se IfSo

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

public export
entersWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                     (kind : CounterKind) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticEffect bs
entersWithCounters n amt kind =
  EntersRider n (WithCounters amt (PrintedKind kind) Fresh) {zn}

public export
gets : {bs : Bindings} -> (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
       (tou : PtShift (shiftIntro pow)) ->
       {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
       (d : Maybe (Duration (staticIntro (Gets Adds n pow tou {ok})))) ->
       {auto 0 sp : SpanOk d} -> Effect bs
gets {bs} n pow tou d = Continuously {bs} (Gets Adds n pow tou {ok}) d {sp}

public export
gains : {bs : Bindings} -> (n : Noun bs Object) -> (a : AbilityAt bs) ->
        {auto 0 ok : GrantSubject a n} ->
        {auto 0 gr : Grantable a} ->
        (d : Maybe (Duration (staticIntro (Gains n a {ok} {gr})))) ->
        {auto 0 sp : SpanOk d} -> Effect bs
gains {bs} n a d = Continuously {bs} (Gains n a {ok} {gr}) d

public export
gainsHaste : {bs : Bindings} -> (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 ok : GrantSubject (KeywordAbility "Haste" Nothing Nothing) n} ->
             {auto 0 sp : SpanOk d} -> Effect bs
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
          {auto 0 dp : DeedFits deeds role k (nounHeadTys n) (nounZone n)} ->
          {auto 0 pt : So (deonticPatientOk n deeds role patient NoDeonticRider)} ->
          StaticEffect bs
deontic n c deeds role patient =
  Deontic n c deeds role Nothing patient Nothing NoDeonticRider {ne} {dd} {kd} {dp} {pt}

public export
cantMoreThan : (who : Noun bs Player) -> (deed : VerbLabel) -> (k : Nat) ->
               (p : Predicate (nomIntro who) Object) ->
               {auto 0 kd : KnownActs [deed]} ->
               {auto 0 dp : DeedFits [deed] Agent Player (nounHeadTys who) (nounZone who)} ->
               {auto 0 bd : So (deonticBoundOk {bs = nomIntro who} [deed]
                                                (Just (MoreThan (Lit k))))} ->
               {auto 0 pt : So (deonticPatientOk who [deed] Agent
                                  (DeonticCounterpart (allOf p)) NoDeonticRider)} ->
               StaticEffect bs
cantMoreThan who deed k p =
  Deontic who Forbid [deed] Agent (Just (MoreThan (Lit k))) (DeonticCounterpart (allOf p))
          Nothing NoDeonticRider {kd} {dp} {bd} {pt}

public export
mayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                         {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                         {auto 0 dp : DeedFits ["Play"] Agent Player (nounHeadTys who) (nounZone who)} ->
                         {auto 0 bd : So (deonticBoundOk ["Play"] (Just (Additional q)))} ->
                         {auto 0 pt : So (deonticPatientOk who ["Play"] Agent
                                            (DeonticCounterpart (allOf Macros.land))
                                            NoDeonticRider)} ->
                         StaticEffect bs
mayPlayAdditionalLands who q =
  Deontic who Permit ["Play"] Agent (Just (Additional q)) (DeonticCounterpart (allOf Macros.land))
          Nothing NoDeonticRider {dp} {bd} {pt}

public export
mayBlockAdditional : (n : Noun bs Object) -> (q : Quantity (nomIntro n)) ->
                     {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                     {auto 0 dp : DeedFits ["Block"] Agent Object (nounHeadTys n) (nounZone n)} ->
                     {auto 0 bd : So (deonticBoundOk ["Block"] (Just (Additional q)))} ->
                     {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                        (DeonticCounterpart (allOf Macros.creature))
                                        NoDeonticRider)} ->
                     StaticEffect bs
mayBlockAdditional n q =
  Deontic n Permit ["Block"] Agent (Just (Additional q)) (DeonticCounterpart (allOf Macros.creature))
          Nothing NoDeonticRider {dp} {bd} {pt}

public export
mayVoteAdditional : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                    {auto 0 nz : NonZeroQ q} -> {auto 0 wf : WellFormedQ q} ->
                    {auto 0 dp : DeedFits ["Vote"] Agent Player (nounHeadTys who) (nounZone who)} ->
                    {auto 0 bd : So (deonticBoundOk ["Vote"] (Just (Additional q)))} ->
                    StaticEffect bs
mayVoteAdditional who q =
  Deontic who Permit ["Vote"] Agent (Just (Additional q)) NoDeonticPatient
          Nothing NoDeonticRider {dp} {bd}

public export
doesntUntap : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
              {auto 0 wk : WindowOk UntapStep w} ->
              {auto 0 dp : DeedFits ["Untap"] Patient Object (nounHeadTys n) (nounZone n)} ->
              StaticEffect bs
doesntUntap n w =
  OnlyDuring UntapStep w
    (Deontic n Forbid ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
mayDeclineUntap : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
                  {auto 0 wk : WindowOk UntapStep w} ->
                  {auto 0 dp : DeedFits ["Untap"] Patient Object (nounHeadTys n) (nounZone n)} ->
                  StaticEffect bs
mayDeclineUntap n w =
  OnlyDuring UntapStep w
    (Deontic n Permit ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
untapsDuring : (n : Noun bs Object) -> (w : Maybe (Noun bs Player)) ->
               {auto 0 wk : WindowOk UntapStep w} ->
               {auto 0 dp : DeedFits ["Untap"] Patient Object (nounHeadTys n) (nounZone n)} ->
               StaticEffect bs
untapsDuring n w =
  OnlyDuring UntapStep w
    (Deontic n Require ["Untap"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp})
    {wk}

public export
cantAttack : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 dp : DeedFits ["Attack"] Agent Object (nounHeadTys n) (nounZone n)} ->
             {auto 0 sp : SpanOk span} -> Effect bs
cantAttack {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Attack"] Agent Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
cantBlock : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 dp : DeedFits ["Block"] Agent Object (nounHeadTys n) (nounZone n)} ->
            {auto 0 sp : SpanOk span} -> Effect bs
cantBlock {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Block"] Agent Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
canDoAsThough : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
                (p : Predicate (nomIntro n) Object) ->
                {auto 0 kd : KnownActs [deed]} ->
                {auto 0 dp : DeedFits [deed] Agent k (nounHeadTys n) (nounZone n)} ->
                {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro n}) [deed] (Just (AsThoughOf p)))} ->
                StaticEffect bs
canDoAsThough n deed p =
  Deontic n Permit [deed] Agent Nothing NoDeonticPatient (Just (AsThoughOf p)) NoDeonticRider
          {kd} {dp} {at}

public export
maySpendAsThough : (who : Noun bs Player) ->
                   (what : Maybe ColorOrColorless) -> (as : ManaMatch) ->
                   (purpose : Maybe (SpendPurpose (nomIntro who))) ->
                   {auto 0 kd : KnownActs ["Spend"]} ->
                   {auto 0 dp : DeedFits ["Spend"] Agent Player (nounHeadTys who) (nounZone who)} ->
                   {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro who}) ["Spend"]
                                               (Just (AsThoughMana what as purpose)))} ->
                   StaticEffect bs
maySpendAsThough who what as purpose =
  Deontic who Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana what as purpose)) NoDeonticRider
          {dd = Oh} {dp} {bd = Oh} {at} {pt = Oh} {rd = Oh}

public export
playerCant : (deed : VerbLabel) -> (who : Noun bs Player) ->
             {auto 0 kd : KnownActs [deed]} ->
             {auto 0 dp : DeedFits [deed] Agent Player (nounHeadTys who) (nounZone who)} ->
             StaticEffect bs
playerCant deed who = Deontic who Forbid [deed] Agent Nothing NoDeonticPatient Nothing NoDeonticRider
                              {kd} {dp}

public export
objectCant : {k : Kind} -> (deed : VerbLabel) -> (what : Noun bs k) ->
             {auto 0 kd : KnownActs [deed]} ->
             {auto 0 dp : DeedFits [deed] Patient k (nounHeadTys what) (nounZone what)} ->
             StaticEffect bs
objectCant deed what =
  Deontic what Forbid [deed] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {kd} {dp}

public export
cantDoTo : {k : Kind} -> {kw : Kind} -> (deed : VerbLabel) ->
           (who : Noun bs k) -> (what : Noun (nomIntro who) kw) ->
           {auto 0 kd : KnownActs [deed]} ->
           {auto 0 dp : DeedFits [deed] Agent k (nounHeadTys who) (nounZone who)} ->
           {auto 0 pt : So (deonticPatientOk who [deed] Agent
                              (DeonticCounterpart what)
                              NoDeonticRider)} ->
           StaticEffect bs
cantDoTo deed who what =
  Deontic who Forbid [deed] Agent Nothing (DeonticCounterpart what) Nothing NoDeonticRider
          {kd} {dp} {pt}

public export
cantBeTargetedBy : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                   (by : Noun (nomIntro what) ka) ->
                   {auto 0 tr : Targeter ka} ->
                   {auto 0 dp : DeedFits ["Target"] Patient k (nounHeadTys what) (nounZone what)} ->
                   StaticEffect bs
cantBeTargetedBy what by =
  Deontic what Forbid ["Target"] Patient Nothing (TargetedBy by {tr}) Nothing NoDeonticRider {dp}

public export
canBeTargetedAsThough : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                        (by : Noun (nomIntro what) ka) ->
                        (p : Predicate (nomIntro what) Object) ->
                        {auto 0 tr : Targeter ka} ->
                        {auto 0 dp : DeedFits ["Target"] Patient k (nounHeadTys what) (nounZone what)} ->
                        StaticEffect bs
canBeTargetedAsThough what by p =
  Deontic what Permit ["Target"] Patient Nothing (TargetedBy by {tr}) (Just (AsThoughOf p))
          NoDeonticRider {dp}

public export
cantBeBlocked : {bs : Bindings} -> (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 dp : DeedFits ["Block"] Patient Object (nounHeadTys n) (nounZone n)} ->
                {auto 0 sp : SpanOk span} -> Effect bs
cantBeBlocked {bs} n span =
  Continuously {bs} (Deontic n Forbid ["Block"] Patient Nothing NoDeonticPatient Nothing NoDeonticRider {dp}) span {sp}

public export
mustBlockIt : {bs : Bindings} -> (n : Noun bs Object) ->
              (span : Maybe (Duration (selfSubjIntro n))) ->
              {auto 0 dp : DeedFits ["Block"] Agent Object (nounHeadTys n) (nounZone n)} ->
              {auto 0 ok : countOnes Object bs = 1} ->
              {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                 (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok}))
                                 NoDeonticRider)} ->
              {auto 0 sp : SpanOk span} -> Effect bs
mustBlockIt n span =
  Continuously (Deontic n Require ["Block"] Agent Nothing
                  (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok}))
                  Nothing NoDeonticRider {dp} {pt})
               span {sp}


public export
attachToIt : {bs : Bindings} -> (what : Noun bs Object) ->
             {auto 0 zw : ZoneIs (nounZone what) Battlefield} ->
             {auto 0 ok : countOnes Object bs = 1} -> Effect bs
attachToIt what = AttachTo what (ItOtherThan (nounDelta what) bs {ok}) {zw}


public export
gainControl : {bs : Bindings} -> (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              {auto 0 zn : ZoneIs (nounZone what) Battlefield} ->
              (d : Maybe (Duration (staticIntro (GainsControl who what {zn})))) ->
              {auto 0 sp : SpanOk d} -> Effect bs
gainControl {bs} who what d = Continuously {bs} (GainsControl who what {zn}) d {sp}

public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)


public export
may : (decider : Noun bs Player) -> Effect (agentIntro decider) -> Effect bs
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
         Effect bs
create count tok = Create You count (TokenWritten tok {wf}) []

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 wf : TokenWellFormed tok} ->
                        Effect bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {wf})
         [EntersTapped, EntersAttacking NoDefender]

public export
becomesAs : {bs : Bindings} -> (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 ok : BecomesOk Adds n (Bundle added Nothing)} ->
            {auto 0 sp : SpanOk d} -> Effect bs
becomesAs {bs} n added d = Continuously {bs} (Becomes n Adds (Bundle added Nothing) {ok}) d {sp}

public export
becomes : {bs : Bindings} -> (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 ok : BecomesOk Adds n (Bundle (MkToken {bs} Nothing [] added [] Nothing) Nothing)} ->
          {auto 0 sp : SpanOk d} -> Effect bs
becomes {bs} n added d = becomesAs {bs} n (MkToken Nothing [] added [] Nothing) d {ok} {sp}

public export
becomesColor : {bs : Bindings} -> (n : Noun bs Object) -> (cs : ColorSpec) ->
               (d : Maybe (Duration (selfSubjIntro n))) ->
               {auto 0 ok : BecomesOk Sets n (Colored cs)} ->
               {auto 0 sp : SpanOk d} -> Effect bs
becomesColor {bs} n cs d = Continuously {bs} (Becomes n Sets (Colored cs) {ok}) d {sp}

public export
basicLandLine : (ss : List Subtype) -> {auto 0 bl : BasicLandTypes ss} -> TypeLine
basicLandLine ss = MkTypeLine ss []


public export
draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) -> Effect bs
draw who amt = Draw who amt


public export
chooseOne : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 1) (modeCount modes)} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf}

public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 2) (modeCount modes)} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf}

public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Range (Just 1) (Just 2) {bs}) (modeCount modes)} -> Effect bs
chooseOneOrBoth modes =
  Modal (Range (Just 1) (Just 2)) modes {tw} {mf}

public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast {bs} 1) (modeCount modes)} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf}

public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Macros.anyNumber {bs}) (modeCount modes)} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {wf = Oh} {tw} {mf}

public export
notSo : (c : Condition bs) -> Condition bs
notSo c = NotCond c

public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countReach Bare OneOf bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfReach Bare OneOf bs) (seedZone p)} ->
       Condition bs
itsA p = Matches (It {ok}) p {sy} {zc}

public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countReach Bare OneOf bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfReach Bare OneOf bs) (seedZone p)} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc})

public export
itsACard : (p : Predicate bs Object) ->
           {auto 0 ok : countReach (AtSlot CardSlot) OneOf bs = 1} ->
           {auto 0 sy : PredSays p} ->
           {auto 0 zc : ZoneFits (zoneOfReach (AtSlot CardSlot) OneOf bs) (seedZone p)} ->
           Condition bs
itsACard p = Matches (ItAt CardSlot {ok}) p {sy} {zc}

public export
itIsntAnAbility : (p : Predicate bs Ability) ->
                  {auto 0 ok : countReach (Word AbilityW) OneOf bs = 1} ->
                  {auto 0 sy : PredSays p} ->
                  {auto 0 bl : TestSubject (ItAbility {bs} {ok})} ->
                  {auto 0 zc : ZoneFits (zoneOfReach (Word AbilityW) OneOf bs)
                                              (seedZone p)} ->
                  Condition bs
itIsntAnAbility p =
  NotCond (Matches (ItAbility {ok}) p {sy} {bl} {zc})


public export
libraryOf : (n : Noun bs Player) -> ZoneExpr bs
libraryOf n = ZoneAt Library (PossessedBy n {ps = LibraryIsOwned})

public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt (OneEnd OnTop) Nothing Nothing Bare

public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt (OneEnd OnBottom) Nothing Nothing Bare

public export
onTopIn : (a : Arrangement) ->
          {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnTop) (Just a)} ->
          ZoneExpr bs
onTopIn a = LibraryAt (OneEnd OnTop) (Just a) Nothing {af} {nf = Oh} Bare

public export
onBottomIn : (a : Arrangement) ->
             {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnBottom) (Just a)} ->
             ZoneExpr bs
onBottomIn a = LibraryAt (OneEnd OnBottom) (Just a) Nothing {af} {nf = Oh} Bare

public export
nthFromTop : (n : LibOrdinal) -> ZoneExpr bs
nthFromTop n = LibraryAt (OneEnd OnTop) Nothing (Just n) Bare

public export
topOrBottomZ : ZoneExpr bs
topOrBottomZ = LibraryAt (EitherEnd Nothing) Nothing Nothing Bare

public export
choiceOfTopOrBottom : (chooser : Noun bs Player) ->
                      {auto 0 ag : EventAgent (Just chooser)} -> ZoneExpr bs
choiceOfTopOrBottom chooser = LibraryAt (EitherEnd (Just chooser) {ag}) Nothing Nothing Bare

public export
nthFromTopOrBottomZ : (n : LibOrdinal) -> ZoneExpr bs
nthFromTopOrBottomZ n = LibraryAt (EitherEnd Nothing) Nothing (Just n) Bare

public export
shuffleInto : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
              {auto 0 pl : Placeable (nounTy n) Library} -> Effect bs
shuffleInto agent n =
  Enact (Just agent) "Shuffle"
    (Move n (LibraryAt Shuffled Nothing Nothing Bare)
            [] {pl})

public export
topSlice : (amt : Amount bs) -> Noun bs Object
topSlice amt = LibrarySlice OnTop amt You

public export
topCards : (n : Nat) -> Noun bs Object
topCards n = topSlice (Lit n)

public export
topCard : Noun bs Object
topCard = topCards 1

public export
bottomCard : Noun bs Object
bottomCard = LibrarySlice OnBottom (Lit 1) You

public export
oneOf : (grp : Noun bs Object) -> {auto 0 gm : PartitiveBase grp} -> Noun bs Object
oneOf grp = SomeOf (CountedSlice (exactly 1)) Nothing grp {gm}

public export
onePile : {auto 0 ok : countReach (Word PileW) ManyOf bs = 1} -> Noun bs Object
onePile = PileOf (CountedSlice (exactly 1)) Nothing {ok}

public export
pileOfChoice : (by : Noun bs Player) ->
               {auto 0 ok : countReach (Word PileW) ManyOf bs = 1} -> Noun bs Object
pileOfChoice by = PileOf (CountedSlice (exactly 1)) (Just by) {ok}

public export
someOf : (n : Nat) -> (grp : Noun bs Object) -> {auto 0 gm : PartitiveBase grp} ->
         {auto 0 nz : NonZeroQ (exactly {bs} n)} ->
         {auto 0 wf : WellFormedQ (exactly {bs} n)} -> Noun bs Object
someOf n grp = SomeOf (CountedSlice (exactly n) {nz} {wf}) Nothing grp {gm}

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
oneFromAmong : (p : Predicate bs Object) -> (grp : Noun bs Object) ->
               {auto 0 gm : PartitiveBase grp} -> Noun bs Object
oneFromAmong p grp = SomeOf (CountedSlice (exactly 1)) (Just p) grp {gm}


public export
lookAt : (n : Noun bs Object) -> Effect bs
lookAt n = Expose LookAt You (ExposedCards n)

public export
revealCards : (n : Noun bs Object) -> Effect bs
revealCards n = Expose Reveal You (ExposedCards n)

public export
lookAtHandOf : (n : Noun bs Player) -> Effect bs
lookAtHandOf n = Expose LookAt You (ExposedZone (handOf n))

public export
foundCard : {auto 0 ok : countReach (Stamped "Search") OneOf bs = 1} -> Noun bs Object
foundCard = ItVerbed "Search" {ok}

public export
revealsIt : {auto 0 ok : countReach (Stamped "Search") OneOf bs = 1} -> Effect bs
revealsIt = revealCards (foundCard {ok})

public export
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countReach (Word PlayerW) OneOf (nomIntro who) = 1} -> Effect bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))



public export
searchLibraryFor : (p : Predicate bs Object) ->
                   {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryFor p = Search You (OneZone yourLibrary) (exactly 1) p {zf}

public export
searchLibraryForCount : (q : Quantity bs) -> (p : Predicate bs Object) ->
                        {auto 0 nz : NonZeroQ q} ->
                        {auto 0 wf : WellFormedQ q} ->
                        {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryForCount q p = Search You (OneZone yourLibrary) q p {zf}

public export
searchZonesOf : (whose : Noun bs Player) -> (p : Predicate bs Object) ->
                {auto 0 zf : ZoneFree p} -> Effect bs
searchZonesOf whose p =
  Search You (SomeZones (Just whose) [Graveyard, Hand, Library]) (exactly 1) p {zf}

public export
searchLibraryOrGraveyard : (p : Predicate bs Object) ->
                           {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryOrGraveyard p =
  Search You (SomeZones (Just You) [Library, Graveyard]) (exactly 1) p {zf}

public export
puts : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
       (to : ZoneExpr (nomIntro n)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur n) to} ->
       {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
       Effect bs
puts agent n to =
  Enact (Just agent) "Put" (Move n to [] {ok} {arr} {pl})

public export
shuffle : Effect bs
shuffle = Shuffle You

public export
lifeTotalBecomes : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
lifeTotalBecomes who a = ChangeLife who (Set a)


public export
ifWouldInstead : {bs : Bindings} -> (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                 (d : Maybe (Duration (eventIntro ev))) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 sp : SpanOk d} -> Effect bs
ifWouldInstead {bs} ev repl d = Continuously {bs} (Intercepts ev [] Nothing repl Repeatedly Nothing {ok}) d {sp}

public export
nextTimeWouldInstead : {bs : Bindings} -> (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 sp : SpanOk d} -> Effect bs
nextTimeWouldInstead {bs} ev repl d =
  Continuously {bs} (Intercepts ev [] Nothing repl NextTimeOnly Nothing {ok}) d {sp}

public export
preventAll : {bs : Bindings} -> (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk d} -> Effect bs
preventAll {bs} kind scope d =
  Continuously {bs} (DamageRule kind Unattributed scope (Prevent CutAll Nothing) Repeatedly) d {sp}

public export
preventNext : {bs : Bindings} -> (kind : DamageKind) -> (scope : DamageScope bs) ->
              (amt : Amount (scopeIntro scope)) ->
              (d : Maybe (Duration (amtIntro amt))) ->
              {auto 0 sp : SpanOk d} -> Effect bs
preventNext {bs} kind scope amt d =
  Continuously {bs} (DamageRule kind Unattributed scope (Prevent (Shield amt) Nothing) Repeatedly) d {sp}

public export
preventAllBy : {bs : Bindings} -> (kind : DamageKind) -> (src : Noun bs Object) ->
               (scope : DamageScope (nomIntro src)) ->
               (d : Maybe (Duration (scopeIntro scope))) ->
               {auto 0 sp : SpanOk d} -> Effect bs
preventAllBy {bs} kind src scope d =
  Continuously {bs} (DamageRule kind (DealtBy src) scope (Prevent CutAll Nothing) Repeatedly) d {sp}

public export
shieldingIt : {k : Kind} -> (n : Noun bs k) ->
              {auto 0 rk : DamageRecipient n} -> DamageScope bs
shieldingIt n = ToRecipient n {rk}

public export
exileUntil : (n : Noun bs Object) ->
             (ev : GameEvent
                     (preIntro
                       (Enact Nothing "Exile"
                         (Move n (Macros.exileZ {bs = nomIntro n})
                               []
                               {ok = ExileOk} {arr = Oh} {pl = Oh} {rf = Oh})))) ->
             Effect bs
exileUntil n ev =
  HeldUntil
    (Enact Nothing "Exile"
      (Move n (Macros.exileZ {bs = nomIntro n})
            []
            {ok = ExileOk} {arr = Oh} {pl = Oh} {rf = Oh}))
            ev {ok = Oh}

public export
phasesOutUntil : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 {auto 0 at : StatusEffectVal PhasedOut} ->
                 (ev : GameEvent (annIntro (SetStatus PhasedOut n {ok = zn} {at}))) ->
                 Effect bs
phasesOutUntil n ev = HeldUntil (SetStatus PhasedOut n {ok = zn} {at}) ev {ok = Oh}

public export
insteadOf : (replaced : Effect bs) -> (repl : Effect (replacedCtx replaced)) ->
            {auto 0 na : NotInstead replaced} ->
            {auto 0 nb : NotInstead repl} -> Effect bs
insteadOf replaced repl = InsteadOf replaced repl {na} {nb}


public export
generic : Nat -> ManaSymbol
generic n = Simple (Generic n)

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
payLife who n = Do (ChangeLife who (Down (Lit n)))

public export
mayWhen : (decider : Noun bs Player) -> (body : Effect (agentIntro decider)) ->
          Effect (settleTargets (effIntro body)) ->
          {auto 0 ok : So (admitsReflexEnclosure (reflexEncloseUse body))} ->
          Effect bs
mayWhen d body trig =
  Reflexively (May d body Nothing Nothing) trig {en = ok}



public export
mills : (agent : Noun bs Player) -> (amt : Amount (agentIntro agent)) ->
        (whose : Noun (agentIntro agent) Player) ->
        {auto 0 sp : SlicePossessor whose} ->
        Effect bs
mills agent amt whose =
  Enact (Just agent) "Mill"
       (Move (LibrarySlice OnTop amt whose {sp}) graveyardZ
             [])

public export
lookedTop : (bs : Bindings) -> (amt : Amount bs) -> Bindings
lookedTop bs amt = nomIntro (topSlice {bs} amt)

public export %inline
agentLookedTop : (bs : Bindings) -> (0 an : countReach (Word PlayerW) OneOf bs = 1) ->
                 (amt : Amount bs) -> Bindings
agentLookedTop bs an amt =
  nomIntro (LibrarySlice OnTop amt (They {ok = an}) {sp = Oh})

public export %inline
agentMovedRest : (bs : Bindings) -> (0 mn : countReach Bare ManyOf bs = 1) ->
                 (z : Zone) -> Bindings
agentMovedRest bs mn z =
  moveIntro {bs} Nothing
    (SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
            (Them {ok = mn}) {gm = Oh})
    (Just z)

public export %inline
lookAtAgentsTop : {bs : Bindings} -> (amt : Amount bs) ->
                  {auto 0 an : countReach (Word PlayerW) OneOf bs = 1} -> Effect bs
lookAtAgentsTop amt =
  Expose LookAt (They {ok = an})
    (ExposedCards (LibrarySlice OnTop amt (They {ok = an}) {sp = Oh}))

public export %inline
oneCardAmount : Amount bs -> Bool
oneCardAmount (Lit 1) = True
oneCardAmount _ = False

public export
data LookReq : {bs : Bindings} -> (agent : Noun bs Player) ->
               Amount (Experimental.Phrase.agentIntro agent) -> Zone -> Type where
  YourOneLookReq :
    {auto 0 ay : agent = You} ->
    {auto 0 am : amt = Lit 1} ->
    {auto 0 iw : countReach (Word CardW) OneOf
                    (lookedTop (Experimental.Phrase.agentIntro agent) (Lit 1)) = 1} ->
    {auto 0 pi : Placeable
                    (tyOfReach (Word CardW) OneOf
                      (lookedTop (Experimental.Phrase.agentIntro agent) (Lit 1))) z} ->
    LookReq agent amt z
  YourManyLookReq :
    {auto 0 ay : agent = You} ->
    {auto 0 no : So (not (oneCardAmount amt))} ->
    {auto 0 mn : countReach Bare ManyOf
                    (lookedTop (Experimental.Phrase.agentIntro agent) amt) = 1} ->
    {auto 0 ps : Placeable
                    (tyOfReach Bare ManyOf (lookedTop (Experimental.Phrase.agentIntro agent) amt)) z} ->
    {auto 0 tr : So (theRestOk
                      (agentMovedRest
                        (lookedTop (Experimental.Phrase.agentIntro agent) amt) mn z))} ->
    {auto 0 pr : Placeable
                    (tyOfGroup
                      (agentMovedRest
                        (lookedTop (Experimental.Phrase.agentIntro agent) amt) mn z)) Library} ->
    LookReq agent amt z
  TheirOneLookReq :
    {auto 0 ny : So (not (nounIsYou agent))} ->
    {auto 0 am : amt = Lit 1} ->
    {auto 0 an : countReach (Word PlayerW) OneOf
                    (Experimental.Phrase.agentIntro agent) = 1} ->
    {auto 0 ap : countReach (Word PlayerW) OneOf
                    (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
    {auto 0 iw : countReach (Word CardW) OneOf
                    (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
    {auto 0 pi : Placeable
                    (tyOfReach (Word CardW) OneOf
                      (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1))) z} ->
    LookReq agent amt z
  TheirManyLookReq :
    {auto 0 ny : So (not (nounIsYou agent))} ->
    {auto 0 no : So (not (oneCardAmount amt))} ->
    {auto 0 an : countReach (Word PlayerW) OneOf
                    (Experimental.Phrase.agentIntro agent) = 1} ->
    {auto 0 mn : countReach Bare ManyOf
                    (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt) = 1} ->
    {auto 0 ps : Placeable
                    (tyOfReach Bare ManyOf (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt)) z} ->
    {auto 0 tr : So (theRestOk
                      (agentMovedRest
                        (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt) mn z))} ->
    {auto 0 pr : Placeable
                    (tyOfGroup
                      (agentMovedRest
                        (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt) mn z))
                    Library} ->
    LookReq agent amt z

scryTheirOne : {bs : Bindings} -> (agent : Noun bs Player) ->
               {0 an : countReach (Word PlayerW) OneOf (Experimental.Phrase.agentIntro agent) = 1} ->
               {0 ap : countReach (Word PlayerW) OneOf
                         (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
               {0 iw : countReach (Word CardW) OneOf
                         (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
               {0 pi : Placeable
                         (tyOfReach (Word CardW) OneOf
                           (agentLookedTop
                             (Experimental.Phrase.agentIntro agent) an (Lit 1))) Library} ->
               Effect bs
scryTheirOne {bs} agent {an} {ap} {iw} {pi} =
  Enact (Just agent) "Scry" {kn = Oh}
       (Sequentially
          [ lookAtAgentsTop (Lit 1) {an}
          , may (They {ok = ap})
                (move (That CardW {ok = iw}) onBottomZ
                      {ok = LibraryPosOk {af = Oh} {nf = Oh}}
                      {arr = Oh} {pl = pi}) ])

scryTheirMany : {bs : Bindings} -> (agent : Noun bs Player) ->
                (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
                {0 an : countReach (Word PlayerW) OneOf (Experimental.Phrase.agentIntro agent) = 1} ->
                {0 mn : countReach Bare ManyOf
                          (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt) = 1} ->
                {0 ps : Placeable
                          (tyOfReach Bare ManyOf
                            (agentLookedTop
                              (Experimental.Phrase.agentIntro agent) an amt)) Library} ->
                {0 tr : So (theRestOk
                             (agentMovedRest
                               (agentLookedTop
                                 (Experimental.Phrase.agentIntro agent) an amt) mn Library))} ->
                {0 pr : Placeable
                          (tyOfGroup
                            (agentMovedRest
                              (agentLookedTop
                                (Experimental.Phrase.agentIntro agent) an amt) mn Library)) Library} ->
                Effect bs
scryTheirMany {bs} agent amt {an} {mn} {ps} {tr} {pr} =
  Enact (Just agent) "Scry" {kn = Oh}
       (Sequentially [ lookAtAgentsTop amt {an}
                     , move (SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
                                    (Them {ok = mn}) {gm = Oh})
                            (onBottomIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (theRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
scry : {bs : Bindings} -> (agent : Noun bs Player) ->
       (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
       {auto req : LookReq agent amt Library} -> Effect bs
scry {bs} _ _ {req = YourOneLookReq {ay = Refl} {iw} {pi}} =
  Enact (Just You) "Scry" {kn = Oh}
       (Sequentially
          [ lookAt topCard
          , may You
                (move (That CardW {ok = iw}) onBottomZ
                      {ok = LibraryPosOk {af = Oh} {nf = Oh}}
                      {arr = Oh} {pl = pi}) ])
scry {bs} _ amt {req = YourManyLookReq {ay = Refl} {mn} {ps} {tr} {pr}} =
  Enact (Just You) "Scry" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
                                    (Them {ok = mn}) {gm = Oh})
                            (onBottomIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (theRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])
scry agent@(Pro (Word PlayerW) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = scryTheirOne agent {an} {ap} {iw} {pi}
scry agent@(Pro (UnionHalf PlayerW) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = scryTheirOne agent {an} {ap} {iw} {pi}
scry agent@(Pro (Verbed _ PlayerW _) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = scryTheirOne agent {an} {ap} {iw} {pi}
scry agent@(Pro (Word PlayerW) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = scryTheirMany agent amt {an} {mn} {ps} {tr} {pr}
scry agent@(Pro (UnionHalf PlayerW) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = scryTheirMany agent amt {an} {mn} {ps} {tr} {pr}
scry agent@(Pro (Verbed _ PlayerW _) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = scryTheirMany agent amt {an} {mn} {ps} {tr} {pr}
scry agent@(AttachHost _ PlayerW) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = scryTheirOne agent {an} {ap} {iw} {pi}
scry agent@(AttachHost _ PlayerW) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = scryTheirMany agent amt {an} {mn} {ps} {tr} {pr}
scry agent _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} =
  scryTheirOne agent {an} {ap} {iw} {pi}
scry agent amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} =
  scryTheirMany agent amt {an} {mn} {ps} {tr} {pr}

||| "fateseal N" [CR#701.29a]
public export
fateseal : {bs : Bindings} ->
           (amt : Amount (Experimental.Phrase.agentIntro (anOpponent {bs}))) ->
           {auto 0 an : countReach (Word PlayerW) OneOf
                     (Experimental.Phrase.agentIntro (anOpponent {bs})) = 1} ->
           {auto 0 mn : countReach Bare ManyOf
                     (agentLookedTop (Experimental.Phrase.agentIntro (anOpponent {bs})) an amt) = 1} ->
           {auto 0 ps : Placeable
                     (tyOfReach Bare ManyOf
                       (agentLookedTop
                         (Experimental.Phrase.agentIntro (anOpponent {bs})) an amt)) Library} ->
           {auto 0 tr : So (theRestOk
                        (agentMovedRest
                          (agentLookedTop
                            (Experimental.Phrase.agentIntro (anOpponent {bs})) an amt) mn Library))} ->
           {auto 0 pr : Placeable
                     (tyOfGroup
                       (agentMovedRest
                         (agentLookedTop
                           (Experimental.Phrase.agentIntro (anOpponent {bs})) an amt) mn Library)) Library} ->
           Effect bs
fateseal amt = scryTheirMany anOpponent amt {an} {mn} {ps} {tr} {pr}

surveilTheirOne : {bs : Bindings} -> (agent : Noun bs Player) ->
                  {0 an : countReach (Word PlayerW) OneOf (Experimental.Phrase.agentIntro agent) = 1} ->
                  {0 ap : countReach (Word PlayerW) OneOf
                            (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
                  {0 iw : countReach (Word CardW) OneOf
                            (agentLookedTop (Experimental.Phrase.agentIntro agent) an (Lit 1)) = 1} ->
                  {0 pi : Placeable
                            (tyOfReach (Word CardW) OneOf
                              (agentLookedTop
                                (Experimental.Phrase.agentIntro agent) an (Lit 1))) Graveyard} ->
                  Effect bs
surveilTheirOne {bs} agent {an} {ap} {iw} {pi} =
  Enact (Just agent) "Surveil" {kn = Oh}
       (Sequentially
          [ lookAtAgentsTop (Lit 1) {an}
          , may (They {ok = ap})
                (move (That CardW {ok = iw}) graveyardZ
                      {ok = GraveyardOkBare} {arr = Oh} {pl = pi}) ])

surveilTheirMany : {bs : Bindings} -> (agent : Noun bs Player) ->
                   (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
                   {0 an : countReach (Word PlayerW) OneOf (Experimental.Phrase.agentIntro agent) = 1} ->
                   {0 mn : countReach Bare ManyOf
                             (agentLookedTop (Experimental.Phrase.agentIntro agent) an amt) = 1} ->
                   {0 ps : Placeable
                             (tyOfReach Bare ManyOf
                               (agentLookedTop
                                 (Experimental.Phrase.agentIntro agent) an amt)) Graveyard} ->
                   {0 tr : So (theRestOk
                                (agentMovedRest
                                  (agentLookedTop
                                    (Experimental.Phrase.agentIntro agent) an amt) mn Graveyard))} ->
                   {0 pr : Placeable
                             (tyOfGroup
                               (agentMovedRest
                                 (agentLookedTop
                                   (Experimental.Phrase.agentIntro agent) an amt) mn Graveyard)) Library} ->
                   Effect bs
surveilTheirMany {bs} agent amt {an} {mn} {ps} {tr} {pr} =
  Enact (Just agent) "Surveil" {kn = Oh}
       (Sequentially [ lookAtAgentsTop amt {an}
                     , move (SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
                                    (Them {ok = mn}) {gm = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (theRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
surveil : {bs : Bindings} -> (agent : Noun bs Player) ->
          (amt : Amount (Experimental.Phrase.agentIntro agent)) ->
          {auto req : LookReq agent amt Graveyard} -> Effect bs
surveil {bs} _ _ {req = YourOneLookReq {ay = Refl} {iw} {pi}} =
  Enact (Just You) "Surveil" {kn = Oh}
       (Sequentially
          [ lookAt topCard
          , may You
                (move (That CardW {ok = iw}) graveyardZ
                      {ok = GraveyardOkBare} {arr = Oh} {pl = pi}) ])
surveil {bs} _ amt {req = YourManyLookReq {ay = Refl} {mn} {ps} {tr} {pr}} =
  Enact (Just You) "Surveil" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf (CountedSlice Macros.anyNumber {wf = Oh}) Nothing
                                    (Them {ok = mn}) {gm = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (theRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh})
                            {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])
surveil agent@(Pro (Word PlayerW) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = surveilTheirOne agent {an} {ap} {iw} {pi}
surveil agent@(Pro (UnionHalf PlayerW) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = surveilTheirOne agent {an} {ap} {iw} {pi}
surveil agent@(Pro (Verbed _ PlayerW _) _) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = surveilTheirOne agent {an} {ap} {iw} {pi}
surveil agent@(Pro (Word PlayerW) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = surveilTheirMany agent amt {an} {mn} {ps} {tr} {pr}
surveil agent@(Pro (UnionHalf PlayerW) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = surveilTheirMany agent amt {an} {mn} {ps} {tr} {pr}
surveil agent@(Pro (Verbed _ PlayerW _) _) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = surveilTheirMany agent amt {an} {mn} {ps} {tr} {pr}
surveil agent@(AttachHost _ PlayerW) _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} = surveilTheirOne agent {an} {ap} {iw} {pi}
surveil agent@(AttachHost _ PlayerW) amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} = surveilTheirMany agent amt {an} {mn} {ps} {tr} {pr}
surveil agent _ {req = TheirOneLookReq {an} {ap} {iw} {pi}} =
  surveilTheirOne agent {an} {ap} {iw} {pi}
surveil agent amt {req = TheirManyLookReq {an} {mn} {ps} {tr} {pr}} =
  surveilTheirMany agent amt {an} {mn} {ps} {tr} {pr}

public export
playerSearchesTheirLibraryFor : (who : Noun bs Player) ->
                                {auto 0 an : countReach (Word PlayerW) OneOf (nomIntro who) = 1} ->
                                (p : Predicate (nomIntro who) Object) ->
                                {auto 0 zf : ZoneFree p} -> Effect bs
playerSearchesTheirLibraryFor who p =
  Search who (OneZone (libraryOf (They {ok = an}))) (exactly 1) p {zf}

public export
proliferate : {bs : Bindings} ->
              {auto 0 mj : countReach (Word JoinW) ManyOf
                              (chosenIntro {bs}
                                (counted Macros.anyNumber
                                  (kindJoin (Compare [AnyCounterAxis Player] AtLeast (Lit 1))
                                            (And [Permanent, HasCounters Nothing])))) = 1} ->
              Effect bs
proliferate =
  Enact Nothing "Proliferate" {kn = Oh}
        (Sequentially [ Choose
                          (counted Macros.anyNumber
                            (kindJoin (Compare [AnyCounterAxis Player] AtLeast (Lit 1))
                                      (And [Permanent, HasCounters Nothing])))
                          Nothing Openly
                      , PutCounters (Lit 1) OwnKinds (EachOf (Those JoinW {ok = mj})) ])

public export
losesAllCounters : (who : Noun bs Player) ->
                   (kind : Maybe (CounterKindSource bs)) ->
                   {auto 0 sc : OptCounterSourceScope kind Player} -> Effect bs
losesAllCounters who kind = LosesCounters who kind Nothing {sc}

public export
removeCounters : (q : Quantity bs) -> (kind : Maybe (CounterKindSource bs)) ->
                 (from : Noun (quantIntro q) Object) ->
                 {auto 0 wf : WellFormedQ q} ->
                 {auto 0 sc : OptCounterSourceScope kind Object} ->
                 {auto 0 cm : CounterMemory from} -> Effect bs
removeCounters q kind from =
  RemoveCounters (Just q) kind from
                 {wf = Present {ok = wf}} {sc} {cm}

public export
removeAllCounters : (kind : Maybe (CounterKindSource bs)) ->
                    (from : Noun bs Object) ->
                    {auto 0 sc : OptCounterSourceScope kind Object} ->
                    {auto 0 cm : CounterMemory from} -> Effect bs
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
            (eff : Effect (eventAfter ev)) ->
            {auto 0 hn : HeaderNontarget ev} ->
            {auto 0 hs : HeaderStatus ev} ->
            {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
            {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing Nothing} ->
            AbilityAt bs
triggered word ev eff =
  Triggered word ev [] Nothing [] Nothing Nothing Nothing eff {hn} {hs} {ae} {cd}

public export
triggeredIf : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (cond : Condition (headerCtx [] ev)) ->
              (eff : Effect (interveningIntro (Just cond))) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 hs : HeaderStatus ev} ->
              {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
              {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing Nothing (Just cond)} ->
              AbilityAt bs
triggeredIf word ev cond eff =
  Triggered word ev [] Nothing [] Nothing Nothing (Just cond) eff {hn} {hs} {ae} {cd}

public export
triggeredOr : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (eff : Effect (headerCtx alts ev)) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 hs : HeaderStatus ev} ->
              {auto 0 ae : AltEvent word alts} ->
              {auto 0 cd : ChapterDefaults ev alts Nothing [] Nothing Nothing Nothing} ->
              AbilityAt bs
triggeredOr word ev alts eff =
  Triggered word ev alts Nothing [] Nothing Nothing Nothing eff {hn} {hs} {ae} {cd}

public export
triggeredOnlyDuring : {bs : Bindings} -> (word : TriggerWord) ->
                      (ev : GameEvent bs) -> (w : TriggerWindow bs) ->
                      (eff : Effect (eventAfter ev)) ->
                      {auto 0 hn : HeaderNontarget ev} ->
                      {auto 0 hs : HeaderStatus ev} ->
                      {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                      {auto 0 cd : ChapterDefaults ev [] Nothing [] (Just w) Nothing Nothing} ->
                      AbilityAt bs
triggeredOnlyDuring word ev w eff =
  Triggered word ev [] Nothing [] (Just w) Nothing Nothing eff {hn} {hs} {ae} {cd}

public export
triggeredWhile : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                 (wh : Concurrent (headerCtx (the (List (GameEvent bs)) []) ev)) ->
                 (eff : Effect (eventAfter ev)) ->
                 {auto 0 hn : HeaderNontarget ev} ->
                 {auto 0 hs : HeaderStatus ev} ->
                 {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                 {auto 0 cd : ChapterDefaults ev [] (Just wh) [] Nothing Nothing Nothing} ->
                 AbilityAt bs
triggeredWhile word ev wh eff =
  Triggered word ev [] (Just wh) [] Nothing Nothing Nothing eff {hn} {hs} {ae} {cd}

public export
whileState : {0 bs : Bindings} -> Condition bs -> Concurrent bs
whileState c = WhileTrue c

public export
whenState : {0 bs : Bindings} -> Condition bs -> GameEvent bs
whenState c = StateHolds c

public export
triggeredJoined : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
                  (joins : List (JoinedHeader bs)) ->
                  (eff : Effect (joinedCtx joins (headerCtx (the (List (GameEvent bs)) []) ev))) ->
                  {auto 0 hn : HeaderNontarget ev} ->
                  {auto 0 hs : HeaderStatus ev} ->
                  {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                  {auto 0 cd : ChapterDefaults ev [] Nothing joins Nothing Nothing Nothing} ->
                  AbilityAt bs
triggeredJoined word ev joins eff =
  Triggered word ev [] Nothing joins Nothing Nothing Nothing eff {hn} {hs} {ae} {cd}

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
                    (eff : Effect (eventAfter ev)) ->
                    {auto 0 hn : HeaderNontarget ev} ->
                    {auto 0 hs : HeaderStatus ev} ->
                    {auto 0 ae : AltEvent word (the (List (GameEvent bs)) [])} ->
                    {auto 0 cd : ChapterDefaults ev [] Nothing [] Nothing (Just lim) Nothing} ->
                    AbilityAt bs
triggeredOnlyOnce word ev lim eff =
  Triggered word ev [] Nothing [] Nothing (Just lim) Nothing eff {hn} {hs} {ae} {cd}

public export
activated : (cost : Cost (dropLetter X bs)) ->
            (eff : Effect (publicOnly (costIntro cost))) ->
            {auto 0 tp : CostTapOnce cost} ->
            {auto 0 py : CostPaidByYou cost} -> AbilityAt bs
activated cost eff = Activated cost eff Nothing Nothing Nothing Nothing {tp} {py}

public export
activatedBy : (cost : Cost (dropLetter X bs)) ->
              (eff : Effect (publicOnly (costIntro cost))) ->
              (who : Noun bs Player) ->
              {auto 0 tp : CostTapOnce cost} ->
              {auto 0 py : CostPaidByYou cost} -> AbilityAt bs
activatedBy cost eff who =
  Activated cost eff Nothing Nothing Nothing (Just who) {tp} {py}

public export
activatedOnlyDuring : (cost : Cost (dropLetter X bs)) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (w : Timing bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyDuring cost eff w = Activated cost eff (Just w) Nothing Nothing Nothing {tp} {py}

public export
activatedOnlyOnce : (cost : Cost (dropLetter X bs)) ->
                    (eff : Effect (publicOnly (costIntro cost))) ->
                    (lim : UsageLimit) ->
                    {auto 0 tp : CostTapOnce cost} ->
                    {auto 0 py : CostPaidByYou cost} ->
                    {auto 0 ul : So (untriggeredLimitOk (Just lim))} ->
                    AbilityAt bs
activatedOnlyOnce cost eff lim =
  Activated cost eff Nothing (Just lim) Nothing Nothing {tp} {py} {ul}

public export
activatedOnlyIf : (cost : Cost (dropLetter X bs)) ->
                  (eff : Effect (publicOnly (costIntro cost))) ->
                  (g : Condition bs) ->
                  {auto 0 tp : CostTapOnce cost} ->
                  {auto 0 py : CostPaidByYou cost} ->
                  AbilityAt bs
activatedOnlyIf cost eff g = Activated cost eff Nothing Nothing (Just g) Nothing {tp} {py}

public export
activatedOnlyOnceIf : (cost : Cost (dropLetter X bs)) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (lim : UsageLimit) -> (g : Condition bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      {auto 0 ul : So (untriggeredLimitOk (Just lim))} ->
                      AbilityAt bs
activatedOnlyOnceIf cost eff lim g =
  Activated cost eff Nothing (Just lim) (Just g) Nothing {tp} {py} {ul}

public export
fromZ : ZoneExpr bs -> Maybe (ZoneExpr bs)
fromZ = Just

public export
free : PlayPayment bs
free = WithoutPaying

public export
paying : (c : Cost bs) -> {auto 0 ok : So (costOffBattlefield c)} -> PlayPayment bs
paying c = PayingInstead c {ok}

public export
onceEachYourTurn : Maybe PlayLimit
onceEachYourTurn = Just OnceEachYourTurn

public export
duringEachYourTurn : Maybe PlayWindow
duringEachYourTurn = Just DuringEachOfYourTurns

public export
whileSearching : Maybe PlayWindow
whileSearching = Just WhileSearchingLibrary

public export
mayPlayDeed : (deed : VerbLabel) -> (who : Noun bs Player) ->
              (what : Noun (nomIntro who) Object) ->
              (rider : DeonticRider (nomIntro what)) ->
              {auto 0 kd : KnownActs [deed]} ->
              {auto 0 dd : So (distinctDeeds [deed])} ->
              {auto 0 dp : DeedFits [deed] Agent Player (nounHeadTys who) (nounZone who)} ->
              {auto 0 pt : So (deonticPatientOk who [deed] Agent
                                                (DeonticCounterpart what)
                                                rider)} ->
              {auto 0 rd : So (deonticRiderOk [deed] Agent
                                              (Permit {bs = selfSubjIntro who})
                                              (DeonticCounterpart what) False
                                              rider)} ->
              StaticEffect bs
mayPlayDeed deed who what rider =
  Deontic who Permit [deed] Agent Nothing (DeonticCounterpart what) Nothing rider
          {kd} {dd} {dp} {pt} {rd}

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
                               StaticEffect bs
entersWithAdditionalCounters n amt kind =
  EntersRider n (WithCounters amt (PrintedKind kind) Additional) {zn}

public export
entersWithFewerCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                          (kind : CounterKind) ->
                          {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                          StaticEffect bs
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
manyCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                   (n : Noun bs Object) ->
                   {auto 0 sc : counterScope kind = Object} -> GameEvent bs
manyCounterEvent dir kind n =
  CounterEvent dir (Just kind) n ManyCounters Nothing False

public export
singleCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                     (n : Noun bs Object) ->
                     {auto 0 sc : counterScope kind = Object} -> GameEvent bs
singleCounterEvent dir kind n =
  CounterEvent dir (Just kind) n OneCounter Nothing False

public export
bareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                   GameEvent bs
bareCounterEvent dir n = CounterEvent dir Nothing n OneCounter Nothing False

public export
manyBareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                       GameEvent bs
manyBareCounterEvent dir n =
  CounterEvent dir Nothing n ManyCounters Nothing False

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
  annIntro (gainControl (controllerOf other {one}) (AsType t This Nothing {way}) Nothing)

public export
exchangeControlOfThis : {bs : Bindings} -> (t : CardType) -> (other : Noun bs Object) ->
                        {auto 0 one : nounPlur other = OneOf} ->
                        {auto 0 way : So (ascriptionOk t Nothing)} ->
                        {auto 0 pw : countReach (Word PermanentW) OneOf
                                       (ExchangeCtx t other one way) = 1} ->
                        {auto 0 zw : So (zoneIsB (zoneOfReach (Word PermanentW) OneOf
                                          (ExchangeCtx t other one way)) Battlefield)} ->
                        Effect bs
exchangeControlOfThis t other =
  Simultaneously
    [ gainControl (controllerOf other {one}) (AsType t This Nothing {way}) Nothing
    , gainControl (controllerOf (AsType t This Nothing {way}))
                  (That PermanentW {ok = pw}) Nothing {zn = zw} ]

public export
choose : {k : Kind} -> (n : Noun bs k) ->
         {auto 0 ch : ChoiceClause (the (Maybe (Noun bs Player)) Nothing) n} ->
         Effect bs
choose n = Choose n Nothing Openly {ch}

public export
armyYouControl : {bs : Bindings} -> Predicate bs Object
armyYouControl =
  And [HasSubtype (creatureType "Army"), creature, HasPossessor ControllerAx You]

public export
0 AmassCtx : {bs : Bindings} -> Bindings
AmassCtx {bs} = effIntro (choose {bs} (Macros.a armyYouControl))

public export
0 AmassAfter : {bs : Bindings} -> (n : Nat) ->
               (0 ac : countReach (Word (TypeW Creature)) OneOf (AmassCtx {bs}) = 1) -> Bindings
AmassAfter n ac =
  effIntro (PutCounters {bs = AmassCtx {bs}} (Lit n) (PrintedKind plusOnePlusOne)
                        (That (TypeW Creature) {ok = ac}))

||| "amass [subtype] N" [CR#701.47a]
public export
amass : {bs : Bindings} -> (sub : String) -> (n : Nat) ->
        {auto 0 ac : countReach (Word (TypeW Creature)) OneOf (AmassCtx {bs}) = 1} ->
        {auto 0 ab : countReach Bare OneOf (AmassAfter {bs} n ac) = 1} ->
        Effect bs
amass sub n =
  Sequentially [ If (notSo (exists armyYouControl))
                    (create (Lit 1) (creatureTok 0 0 [Black]
                                       [creatureType sub, creatureType "Army"]))
                    Nothing
               , choose (Macros.a armyYouControl)
               , PutCounters (Lit n) (PrintedKind plusOnePlusOne)
                             (That (TypeW Creature) {ok = ac})
               , If (itIsntA (HasSubtype (creatureType sub)) {ok = ab})
                    (becomes (It {ok = ab}) (subtypesOnly [creatureType sub]) Nothing)
                    Nothing ]


public export
chooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
          {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
chooses who n = Choose n (Just who) Openly {ch}

public export
secretlyChooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
                  {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
secretlyChooses who n = Choose n (Just who) Secretly {ch}

public export
itVerbed : (v : VerbLabel) -> {auto 0 kn : KnownAct v} ->
           {auto 0 ok : countReach (Stamped v) OneOf bs = 1} -> Noun bs Object
itVerbed v = ItVerbed v {kn} {ok}

public export
themVerbed : (v : VerbLabel) -> {auto 0 kn : KnownAct v} ->
             {auto 0 ok : countReach (Stamped v) ManyOf bs = 1} -> Noun bs Object
themVerbed v = ThemVerbed v {kn} {ok}

takeDropAppend : {0 elem : Type} -> (n : Nat) -> (xs : List elem) ->
                 xs = take n xs ++ drop n xs
takeDropAppend Z xs = Refl
takeDropAppend (S n) [] = Refl
takeDropAppend (S n) (x :: xs) = cong (x ::) (takeDropAppend n xs)

public export
itPrior : {bs : Bindings} -> (prev : Effect bs) ->
          {auto 0 ok : countReach Bare OneOf (effDelta prev) = 1} ->
          Noun (effIntro prev) Object
itPrior {bs} prev =
  Own OneOf (effDelta prev)
    (drop (length (effIntro prev) `minus` length bs) (effIntro prev))
    {sp = takeDropAppend (length (effIntro prev) `minus` length bs)
                         (effIntro prev)}
    {ok}

public export
dealsDamageOwnPower : {bs : Bindings} -> {k : Kind} -> (src : Noun bs Object) ->
                      {auto 0 ok : countReach Bare OneOf (nounDelta src) = 1} ->
                      (to : Noun (nounDelta src ++ bs) k) ->
                      {auto 0 pm : PerMember to} ->
                      {auto 0 rk : DamageRecipient to} -> Effect bs
dealsDamageOwnPower src to =
  DealDamage src (StatOf Power (Own OneOf (nounDelta src) bs {sp = Refl} {ok})) to {pm} {rk}

public export
theVerbed : (v : VerbLabel) -> (w : NounWord) ->
            {auto 0 ok : countReach (Verbed v w Attributive) OneOf bs = 1} ->
            {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
theVerbed v w = TheVerbed v w Attributive {ok} {mk}

public export
theVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                   {auto 0 ok : countReach (Verbed v w ThisWay) OneOf bs = 1} ->
                   {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
theVerbedThisWay v w = TheVerbed v w ThisWay {ok} {mk}

public export
thoseVerbed : (v : VerbLabel) -> (w : NounWord) ->
              {auto 0 ok : countReach (Verbed v w Attributive) ManyOf bs = 1} ->
              {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
thoseVerbed v w = ThoseVerbed v w Attributive {ok} {mk}

public export
thoseVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                     {auto 0 ok : countReach (Verbed v w ThisWay) ManyOf bs = 1} ->
                     {auto 0 mk : ActNamesParticiple v} -> Noun bs (kindOfW w)
thoseVerbedThisWay v w = ThoseVerbed v w ThisWay {ok} {mk}

public export
additionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                 (count : Amount bs) ->
                 {auto 0 ad : AddedPart part} ->
                 {auto 0 an : AddedPartWritten anchor} -> Effect bs
additionalPart part anchor count =
  AdditionalPart Nothing part anchor count Nothing {ad} {an}

public export
getsAdditionalPart : (who : Noun bs Player) -> (part : TurnPart) ->
                     (count : Amount bs) ->
                     {auto 0 ad : AddedPart part} -> Effect bs
getsAdditionalPart who part count =
  AdditionalPart (Just who) part Nothing count Nothing {ad}

public export
additionalPartThen : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) -> (next : TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten (Just next)} -> Effect bs
additionalPartThen part anchor count next =
  AdditionalPart Nothing part anchor count (Just next) {ad} {an} {fb}

public export
vote : (voters : Noun bs Player) -> (disc : Disclosure) ->
       (ballot : Ballot (nomIntro voters)) -> Effect bs
vote voters disc ballot = Vote Nothing voters disc ballot

public export
shiftResult : (amt : Amount bs) ->
              {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
shiftResult amt = ShiftResult Nothing amt {ok}

public export
chaosEnsues : Effect bs
chaosEnsues = ChaosEnsues Nothing

public export
delayed : (ev : GameEvent bs) -> (eff : Effect (delayedCtx [] ev)) -> Effect bs
delayed ev eff = Delayed ev [] Nothing eff

public export
delayedWithin : (ev : GameEvent bs) -> (span : Duration bs) ->
                (eff : Effect (delayedCtx [] ev)) ->
                {auto 0 so : DelaySpanOk (Just span)} -> Effect bs
delayedWithin ev span eff = Delayed ev [] (Just span) eff {so}

public export
quality : (q : QualitySort) -> Predicate bs (Quality q)
quality q = QualityNoun q Nothing

public export
qualityFrom : (q : QualitySort) -> (d : ChoiceDomain (QSort q)) -> Predicate bs (Quality q)
qualityFrom q d = QualityNoun q (Just d)

public export
entersChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 StaticEffect bs
entersChoosing n q = EntersChoice n (QSort q) Nothing Openly {zn}

public export
entersChoosingFrom : (n : Noun bs Object) -> (q : QualitySort) ->
                     (d : ChoiceDomain (QSort q)) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticEffect bs
entersChoosingFrom n q d = EntersChoice n (QSort q) (Just d) Openly {zn}

public export
entersChoosingPlayer : (n : Noun bs Object) ->
                       (d : Maybe (ChoiceDomain PlayerC)) ->
                       {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                       StaticEffect bs
entersChoosingPlayer n d = EntersChoice n PlayerC d Openly {zn}

public export
entersChoosingPlayerSecretly : (n : Noun bs Object) ->
                               (d : Maybe (ChoiceDomain PlayerC)) ->
                               {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                               StaticEffect bs
entersChoosingPlayerSecretly n d = EntersChoice n PlayerC d Secretly {zn}

public export
attachChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 StaticEffect bs
attachChoosing n q = AttachChoice n (QSort q) Nothing {zn}

public export
thisSiege : Noun bs Object
thisSiege = AsType Battle This (Just (battleType "Siege"))

public export
happenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement bs ev k)) Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
happenedTo ev w = HappenedTo ev w Nothing {cw}

public export
happenedToInvolving : {ks : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (w : Lookback) -> (what : Noun bs kc) ->
                      {auto 0 cp : LookbackComplement ev ks kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev ks} -> Predicate bs ks
happenedToInvolving ev w what =
  HappenedTo ev w (Just (Involving what {cp})) {cw} {sb}

public export
happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
           (w : Lookback) ->
           {auto 0 cw : ComplementWritten
                          (the (Maybe (EventComplement (nomIntro who) ev k))
                               Nothing)} ->
           {auto 0 sb : LookbackSubject ev k} -> Condition bs
happened ev who w = Happened ev who w Nothing {cw}

public export
happenedInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                    (who : Noun bs k) -> (w : Lookback) ->
                    (what : Noun (nomIntro who) kc) ->
                    {auto 0 cp : LookbackComplement ev k kc} ->
                    {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                    {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedInvolving ev who w what =
  Happened ev who w (Just (Involving what {cp})) {cw} {sb}

public export
eventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
             (w : Lookback) ->
             {auto 0 cw : ComplementWritten
                            (the (Maybe (EventComplement (nomIntro who) ev k))
                                 Nothing)} ->
             {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCount ev who w = EventTally TallyCount ev who w Nothing {cw}

public export
eventSum : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
           (w : Lookback) ->
           {auto 0 cw : ComplementWritten
                          (the (Maybe (EventComplement (nomIntro who) ev k))
                               Nothing)} ->
           {auto 0 sb : LookbackSubject ev k} ->
           {auto 0 qm : So (tallyOk TallySum ev)} -> Amount bs
eventSum ev who w = EventTally TallySum ev who w Nothing {cw} {sb} {qm}

public export
eventCountInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (who : Noun bs k) -> (w : Lookback) ->
                      (what : Noun (nomIntro who) kc) ->
                      {auto 0 cp : LookbackComplement ev k kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountInvolving ev who w what =
  EventTally TallyCount ev who w (Just (Involving what {cp})) {cw} {sb}

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
  Happened ev who w
    (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb}

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
  EventTally TallyCount ev who w
    (Just (FromZones src (Just (Involving what {cp})) {pl} {ok = zo})) {cw} {sb}

public export
happenedAt : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
             (w : Lookback) -> (z : ZoneExpr (nomIntro who)) ->
             {auto 0 zo : So (lookbackLocusOk ev (zoneSort z))} ->
             {auto 0 cw : ComplementWritten
                            (Just (AtZone {bs = nomIntro who} {ev} {ks = k}
                                          z {ok = zo}))} ->
             {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedAt ev who w z = Happened ev who w (Just (AtZone z {ok = zo})) {cw} {sb}

public export
yours : HeaderPossessor bs
yours = ByPlayer You

public export
eachPlayers : HeaderPossessor bs
eachPlayers = ByPlayer (each AnyPlayer)

public export
eachOpponents : HeaderPossessor bs
eachOpponents = ByPlayer (each Opponent)

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
                   {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
gainsDesignation n d w = GainsDesignation n d w Nothing {sc} {zn}

public export
monstrosity : {bs : Bindings} -> (amt : Amount bs) ->
              Effect bs
monstrosity amt =
  If (notSo (Matches This (HasDesignation Monstrous)))
     (Sequentially [ PutCounters amt (PrintedKind plusOnePlusOne) thisCreature
                   , GainsDesignation thisCreature Monstrous
                                      (InExpansionOf MonstrosityW) Nothing ])
     Nothing

public export
getsCitysBlessing : Effect bs
getsCitysBlessing =
  GainsDesignation You CitysBlessing (InExpansionOf AscendW) (Just RestOfGame)

public export
becomesSaddled : Effect bs
becomesSaddled =
  GainsDesignation (AsType Artifact This Nothing) Saddled (InExpansionOf SaddleW) (Just untilEndOfTurn)

public export
becomesRenowned : {bs : Bindings} -> (amt : Amount bs) -> Effect bs
becomesRenowned amt =
  Sequentially [ PutCounters amt (PrintedKind plusOnePlusOne) thisCreature
               , GainsDesignation thisCreature Renowned
                                  (InExpansionOf RenownW) Nothing ]

public export
getsEnduringStory : Effect bs
getsEnduringStory =
  GainsDesignation You EnduringStory (InExpansionOf StoriedW) (Just RestOfGame)

||| The renown expansion [CR#702.112a].
public export
renownExpansion : (n : Nat) -> AbilityAt []
renownExpansion n =
  triggeredIf When
    (dealsCombatDamage thisCreature (a AnyPlayer))
    (notSo (Matches thisCreature (HasDesignation Renowned)))
    (becomesRenowned (Lit n))

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
           (eventCountInvolving SpellCast (a AnyPlayer) ThisTurn
              (a (And [spell, OtherThan thisSpell])))
           []
       , may You (ChooseNewTargets (Those CopyW)) ])

public export
storm : {0 bs : Bindings} -> AbilityAt bs
storm = KeywordAbility "Storm" Nothing (Just stormExpansion)

||| The cumulative upkeep expansion [CR#702.24a].
public export
cumulativeUpkeepExpansion : (c : Cost []) ->
                            {auto 0 pb : Payable c} ->
                            {auto 0 py : CostPaidByYou c} -> AbilityAt []
cumulativeUpkeepExpansion c =
  triggeredIf At (BeginningOf ThePart Upkeep yours)
    (Matches thisPermanent (InZone battlefieldZ))
    (Sequentially
       [ PutCounters (Lit 1) (PrintedKind (Named "Age")) thisPermanent
       , May You (Pay You (ScaledCost c (times 1 (CountersOn (Named "Age") thisPermanent)))
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
ifThen : (c : Condition bs) -> Effect (condIntro c) -> Effect bs
ifThen c e = If c e Nothing

public export
onlyWhile : {bs : Bindings} -> (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyWhile {bs} se c = Conditionally {bs} c se AsLongAs {st = Static.StaticFirstDone}

public export
onlyUnless : {bs : Bindings} -> (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyUnless {bs} se c = Conditionally {bs} (NotCond c) se Unless {st = Static.StaticFirstDone}

public export
onlyIfSo : {bs : Bindings} -> (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyIfSo {bs} se c = Conditionally {bs} c se IfSo {st = Static.StaticFirstDone}

public export
throughout : {bs : Bindings} -> (span : Duration bs) -> (se : StaticEffect (spanIntro span)) ->
             {auto 0 sp : SpanOk (Just span)} ->
             {auto 0 cl : ClauseStatic se} -> Effect bs
throughout {bs} span se = Continuously {bs} se (Just span) {ts = SpanFirstDone} {sp} {cl}

public export
fromTo : Nat -> Nat -> Quantity bs
fromTo lo hi = Range (Just lo) (Just hi)

public export
flipCoins : (who : Noun bs Player) -> (count : Nat) -> Effect bs
flipCoins who count = FlipCoins who (FlipCount (Lit count))

public export
rollDice : (who : Noun bs Player) -> (count : Nat) -> (sides : Nat) ->
           {auto 0 nz : IsSucc sides} -> Effect bs
rollDice who count sides = RollDice who (Lit count) (SidesOf sides {nz})

public export
theResult : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theResult = TheOutcome RollResult {ok}

public export
rollRow : (results : Quantity bs) -> (e : Effect bs) ->
          {auto 0 nz : NonZeroQ results} ->
          {auto 0 wf : WellFormedQ results} ->
          {auto 0 lt : So (quantLiteral results)} -> RollRow bs
rollRow results e = MkRollRow results e {nz} {wf}

public export
resultsTable : (rows : List (RollRow bs)) ->
               {auto 0 ne : IsSucc (rowCount rows)} ->
               {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
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
rollThePlanarDie : Effect bs
rollThePlanarDie = RollPlanarDie You (Lit 1)

public export
orHigher : Nat -> Quantity bs
orHigher lo = Range (Just lo) Nothing

public export
theTotal : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theTotal = TheOutcome RollResult {ok}

public export
preventedThisWay : {auto 0 ok : countOutcomes DamagePrevented bs = 1} -> Amount bs
preventedThisWay = TheOutcome DamagePrevented {ok}

public export
removedThisWay : {auto 0 ok : countOutcomes CountersRemoved bs = 1} -> Amount bs
removedThisWay = TheOutcome CountersRemoved {ok}

public export
shortOfCeiling : {auto 0 ok : countOutcomes CeilingShortfall bs = 1} -> Amount bs
shortOfCeiling = TheOutcome CeilingShortfall {ok}

public export
coinsThatCameUp : (face : CoinFace) ->
                  {auto 0 fl : So (coinFlipInScope bs)} -> Amount bs
coinsThatCameUp face = CoinsShowing face {fl}
