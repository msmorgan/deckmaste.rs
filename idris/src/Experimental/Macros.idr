module Experimental.Macros

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
ItVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
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
ThemVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
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
ThatHalf : (w : NounWord) ->
           {auto 0 ok : countReach (UnionHalf w) OneOf bs = 1} ->
           Noun bs (kindOfW w)
ThatHalf w = Pro (UnionHalf w) OneOf

public export
TheVerbed : (v : VerbLabel) -> (w : NounWord) -> (marking : VerbedMarking) ->
            {auto 0 ok : countReach (Verbed v w marking) OneOf bs = 1} ->
            {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
TheVerbed v w marking = Pro (Verbed v w marking) OneOf

public export
ThoseVerbed : (v : VerbLabel) -> (w : NounWord) -> (marking : VerbedMarking) ->
              {auto 0 ok : countReach (Verbed v w marking) ManyOf bs = 1} ->
              {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
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
oneOrBoth : Quantity bs
oneOrBoth = Range (Just 1) (Just 2)

public export
oneThrough : Nat -> Quantity bs
oneThrough n = Range (Just 1) (Just n)

public export
target : (p : Predicate bs k) -> {auto tk : Targetable k} -> Noun bs k
target p = TargetGroup (exactly 1) p {tk}

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
thatJoin : {auto 0 ok : countReach (Word JoinW) OneOf bs = 1} -> Noun bs (Object \/ Player)
thatJoin = That JoinW {ok}

public export
thatSplitController : (cls : Noun bs Object) ->
                      {auto 0 one : nounPlur cls = OneOf} ->
                      {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
                      Noun bs Player
thatSplitController cls = EitherOf (ThatHalf PlayerW {ok = pk}) (ControllerOf cls {one})

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
cardOf : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
         (line : TypeLine) -> (text : AbilitySeq (costLetters cost)) ->
         (box : Maybe PrintedBox) ->
         {auto 0 ln : CardLine line} ->
         {auto 0 sp : CardSupers supers} ->
         {auto 0 tx : CardText line text} ->
         {auto 0 ch : CardChapters line text} ->
         {auto 0 bx : CardBox line text box} ->
         {auto 0 mc : CardCost line cost} ->
         {auto 0 dr : DoorFrame text} ->
         Card
cardOf name cost supers line text box =
  SingleFaced (MkFace name cost supers line text box)
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
       {auto 0 ln : CardLine line} ->
       {auto 0 sp : CardSupers supers} ->
       {auto 0 tx : CardText line text} ->
       {auto 0 ch : CardChapters line text} ->
       {auto 0 bx : CardBox line text (printedBox stats)} ->
       {auto 0 mc : CardCost line cost} ->
       {auto 0 dr : DoorFrame text} ->
       Card
card name cost supers line text stats =
  cardOf name cost supers line text (printedBox stats) {ln} {sp} {tx} {ch} {bx} {mc} {dr}

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
            {auto 0 bx : CardBox line text (printedBox stats)} ->
            {auto 0 mc : CardCost line cost} ->
            {auto 0 dr : DoorFrame text} ->
            {auto 0 jc : JointChoices choices text} -> Card
jointCard choices name cost supers line text stats =
  JointSingleFaced
    (MkJointFace choices name cost supers line text (printedBox stats))
    {fl = MkJointFaceLaws {ln} {sp} {tx} {ch} {bx} {mc} {dr} {jc}}

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
creatureYouControl = And [creature, ControlledBy You]

public export
creatureYouDontControl : Predicate bs Object
creatureYouDontControl = And [creature, Not (ControlledBy You)]

public export
creatureYourOpponentsControl : Predicate bs Object
creatureYourOpponentsControl = And [creature, ControlledBy (PlayerGroup YourOpponents)]

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
a p = Indefinite Unmarked p {ph}

public export
aTheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
               {auto 0 ch : countChoosers bs = 1} ->
               Noun bs k
aTheirChoice p = Indefinite (TheirChoice {ch}) p {ph}

public export
aYourChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
              Noun bs k
aYourChoice p = Indefinite YourChoice p {ph}

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
aAtRandom p = Indefinite AtRandom p {ph}

public export
countedAtRandom : (q : Quantity bs) -> (p : Predicate bs k) ->
                  {auto ph : Phrasal k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} -> Noun bs k
countedAtRandom q p = CountedGroup q (Just AtRandom) p {ph} {nz} {wf}

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
otherCreatureYouControl n = And [creature, ControlledBy You, OtherThan n {ca}] {oa = eqToSo ty}

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
nForEach : {k : Kind} -> (n : Nat) -> (p : Predicate bs k) ->
           {auto 0 nz : IsSucc n} ->
           Amount bs
nForEach n p = Times n (CountOf p) {nz}

public export
forEach : {k : Kind} -> (p : Predicate bs k) ->
          Amount bs
forEach p = nForEach 1 p

public export
noRiders : MoveRiders bs
noRiders = MkMoveRiders [] Nothing Nothing

public export
move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur what) to} ->
       {auto 0 pl : Placeable (nounTy what) (zoneSort to)} -> Effect bs
move what to = Move what to noRiders {ok} {arr} {pl}

public export
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
          Effect bs
destroy n = Enact "Destroy" (Move n graveyardZ noRiders)

public export
exile : (n : Noun bs Object) -> Effect bs
exile n = Enact "Exile" (Move n exileZ noRiders)

public export
exiles : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
         Effect bs
exiles agent n = Does agent "Exile" (Move n exileZ noRiders)

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) -> Effect bs
exileWithCounters n amt kind =
  Enact "Exile"
        (Move n exileZ (MkMoveRiders [] Nothing (Just (MkCounterRider amt kind))))

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ (MkMoveRiders [] (Just who) (Just (MkCounterRider amt kind)) {one = OneController {one}}) {pl}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ noRiders {pl}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ (MkMoveRiders [EntersTapped] Nothing Nothing) {pl}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ (MkMoveRiders [EntersTapped, EntersAttacking NoDefender] Nothing Nothing) {pl}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ (MkMoveRiders [] (Just You) Nothing) {pl}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
sacrifice agent n =
  Does agent "Sacrifice" (Move n graveyardZ noRiders)

public export
itAsPermanent : {auto 0 ok : countReach (AtSlot PermanentSlot) OneOf bs = 1} -> Noun bs Object
itAsPermanent = ItAt PermanentSlot {ok}

public export
itAsCard : {auto 0 ok : countReach (AtSlot CardSlot) OneOf bs = 1} -> Noun bs Object
itAsCard = ItAt CardSlot {ok}

public export
sacrificeIt : (agent : Noun bs Player) ->
              {auto 0 ok : countReach (AtSlot PermanentSlot) OneOf (agentIntro agent) = 1} ->
              {auto 0 zn : OnBattlefield (zoneOfReach (AtSlot PermanentSlot) OneOf (agentIntro agent))} ->
              Effect bs
sacrificeIt agent = sacrifice agent (itAsPermanent {ok}) {ok = zn}

public export
discards : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n =
  Does agent "Discard" (Move n graveyardZ noRiders)

public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (a (InZone handZ))

public export
discardsACardAtRandom : (agent : Noun bs Player) -> Effect bs
discardsACardAtRandom agent = discards agent (aAtRandom (InZone handZ))

public export
discard : (n : Noun bs Object) -> {auto 0 dk : DiscardOk n} -> Effect bs
discard n = Enact "Discard" (Move n graveyardZ noRiders)

public export
tap : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
      Effect bs
tap n = Enact "Tap" (SetStatus Tapped n)

public export
untap : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
        Effect bs
untap n = Enact "Untap" (SetStatus Untapped n)

public export
transform : (n : Noun bs Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
transform n = Enact "Transform" (TurnOver n)

public export
convert : (n : Noun bs Object) ->
          {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
convert n = Enact "Convert" (TurnOver n)

public export
transforms : (n : Noun bs Object) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             GameEvent bs
transforms n = VerbedEvent Nothing "Transform" (Just n) Nothing False {zn}

public export
transformsInto : (n : Noun bs Object) -> (into : Predicate bs Object) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 {auto 0 sy : PredSays into} ->
                 GameEvent bs
transformsInto n into =
  VerbedEvent Nothing "Transform" (Just n) (Just into) False {zn}

public export
meldInto : (n : Noun bs Object) -> (into : String) ->
           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
           Effect bs
meldInto n into =
  Enact "Meld"
        (Move n battlefieldZ (MkMoveRiders [EntersMelded into] Nothing Nothing) {arr} {pl})

public export
returnTo : (n : Noun bs Object) -> (to : ZoneExpr (nomIntro n)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur n) to} ->
           {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
           Effect bs
returnTo n to = Enact "Return" (Move n to noRiders {ok} {arr} {pl})

public export
returnToBattlefieldTransformed :
  (n : Noun bs Object) -> (ctrl : Maybe (Noun (nomIntro n) Player)) ->
  {auto 0 one : CtrlOverrideOk ctrl} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldTransformed n ctrl =
  Enact "Return"
        (Move n battlefieldZ (MkMoveRiders [EntersTransformed] ctrl Nothing {one})
              {arr} {pl})

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
sharedSubject : {0 k : Nat} -> (n : Noun bs Object) ->
                (vps : SubjectVPs k (selfSubjIntro n)) ->
                {auto 0 ne : IsSucc k} ->
                {auto 0 ok : So (vpsOk (nounZone n) (nounRegime n) (nounHeadTys n) vps)} ->
                (d : Maybe (Duration (vpsIntro vps))) ->
                {auto 0 sp : SpanOk Coordination d} -> Effect bs
sharedSubject n vps d = Continuously (OfSubject n vps {ne} {ok}) d {sp}


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
untilYourNextTurn = Until (StartOf Turn (Just Yours))

public export
untilEndOfCombat : Duration bs
untilEndOfCombat = Until (EndOf Combat Nothing)

public export
untilYourNextUpkeep : Duration bs
untilYourNextUpkeep = Until (StartOf Upkeep (Just Yours))

public export
untilYourNextEndStep : Duration bs
untilYourNextEndStep = Until (StartOf EndStep (Just Yours))

public export
asLongAs : (c : Condition bs) -> (se : StaticEffect (condIntro c)) -> StaticEffect bs
asLongAs c se = Conditionally c se AsLongAs

public export
unlessSo : (c : Condition bs) -> (se : StaticEffect (condIntro (NotCond c))) -> StaticEffect bs
unlessSo c se = Conditionally (NotCond c) se Unless

public export
ifSo : (c : Condition bs) -> (se : StaticEffect (condIntro c)) -> StaticEffect bs
ifSo c se = Conditionally c se IfSo

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

public export
entersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                     (kind : CounterKind) -> StaticEffect bs
entersWithCounters n amt kind = EntersWithCounters n amt (PrintedKind kind) Fresh

public export
gets : (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
       (tou : PtShift (shiftIntro pow)) ->
       {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
       (d : Maybe (Duration (staticIntro (Gets n pow tou {ok})))) ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou {ok}) d {sp}

public export
gains : (n : Noun bs Object) -> (a : AbilityAt bs) ->
        {auto 0 ok : GrantSubject a n} ->
        {auto 0 gr : Grantable a} ->
        (d : Maybe (Duration (staticIntro (Gains n a {ok} {gr})))) ->
        {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gains n a d = Continuously (Gains n a {ok} {gr}) d

public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 ok : GrantSubject (KeywordAbility "Haste" Nothing) n} ->
             {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gainsHaste n d = gains n (KeywordAbility "Haste" Nothing) d

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
          {auto 0 kd : KnownDeeds deeds} ->
          {auto 0 zn : ZoneFits (nounZone n) (deedsZone deeds role)} ->
          {auto 0 dp : DeedParticipant deeds role k (nounHeadTys n)} ->
          {auto 0 pt : So (deonticPatientOk n deeds role patient NoDeonticRider)} ->
          StaticEffect bs
deontic n c deeds role patient =
  Deontic n c deeds role patient Nothing NoDeonticRider {ne} {dd} {kd} {zn} {dp} {pt}

public export
cantAttack : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Attack"] Agent)} ->
             {auto 0 dp : DeedParticipant ["Attack"] Agent Object (nounHeadTys n)} ->
             {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttack n span =
  Continuously (Deontic n Forbid ["Attack"] Agent NoDeonticPatient Nothing NoDeonticRider {zn} {dp}) span {sp}

public export
cantBlock : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Agent)} ->
            {auto 0 dp : DeedParticipant ["Block"] Agent Object (nounHeadTys n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span =
  Continuously (Deontic n Forbid ["Block"] Agent NoDeonticPatient Nothing NoDeonticRider {zn} {dp}) span {sp}

public export
cantAttackOrBlock : (n : Noun bs Object) ->
                    (span : Maybe (Duration (selfSubjIntro n))) ->
                    {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Attack", "Block"] Agent)} ->
                    {auto 0 dp : DeedParticipant ["Attack", "Block"] Agent Object (nounHeadTys n)} ->
                    {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttackOrBlock n span =
  Continuously (Deontic n Forbid ["Attack", "Block"] Agent NoDeonticPatient Nothing
                  NoDeonticRider {zn} {dp}) span {sp}

public export
canDo : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
        {auto 0 kd : KnownDeeds [deed]} ->
        {auto 0 zn : ZoneFits (nounZone n) (deedsZone [deed] Agent)} ->
        {auto 0 dp : DeedParticipant [deed] Agent k (nounHeadTys n)} ->
        StaticEffect bs
canDo n deed = Deontic n Permit [deed] Agent NoDeonticPatient Nothing NoDeonticRider {kd} {zn} {dp}

public export
canDoAsThough : {k : Kind} -> (n : Noun bs k) -> (deed : VerbLabel) ->
                (p : Predicate (nomIntro n) Object) ->
                {auto 0 kd : KnownDeeds [deed]} ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone [deed] Agent)} ->
                {auto 0 dp : DeedParticipant [deed] Agent k (nounHeadTys n)} ->
                {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro n}) [deed] (Just (AsThoughOf p)))} ->
                StaticEffect bs
canDoAsThough n deed p =
  Deontic n Permit [deed] Agent NoDeonticPatient (Just (AsThoughOf p)) NoDeonticRider
          {kd} {zn} {dp} {at}

public export
maySpendAsThough : (who : Noun bs Player) ->
                   (what : Maybe ColorOrColorless) -> (as : ManaMatch) ->
                   (purpose : Maybe (SpendPurpose (nomIntro who))) ->
                   {auto 0 kd : KnownDeeds ["Spend"]} ->
                   {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Spend"] Agent)} ->
                   {auto 0 dp : DeedParticipant ["Spend"] Agent Player (nounHeadTys who)} ->
                   {auto 0 at : So (asThoughOk (Permit {bs = selfSubjIntro who}) ["Spend"]
                                               (Just (AsThoughMana what as purpose)))} ->
                   StaticEffect bs
maySpendAsThough who what as purpose =
  Deontic who Permit ["Spend"] Agent NoDeonticPatient
          (Just (AsThoughMana what as purpose)) NoDeonticRider
          {kd = Oh} {dd = Oh} {zn} {dp} {at} {pt = Oh} {rd = Oh}

public export
playerCant : (deed : VerbLabel) -> (who : Noun bs Player) ->
             {auto 0 kd : KnownDeeds [deed]} ->
             {auto 0 zn : ZoneFits (nounZone who) (deedsZone [deed] Agent)} ->
             {auto 0 dp : DeedParticipant [deed] Agent Player (nounHeadTys who)} ->
             StaticEffect bs
playerCant deed who = Deontic who Forbid [deed] Agent NoDeonticPatient Nothing NoDeonticRider
                              {kd} {zn} {dp}

public export
objectCant : {k : Kind} -> (deed : VerbLabel) -> (what : Noun bs k) ->
             {auto 0 kd : KnownDeeds [deed]} ->
             {auto 0 zn : ZoneFits (nounZone what) (deedsZone [deed] Patient)} ->
             {auto 0 dp : DeedParticipant [deed] Patient k (nounHeadTys what)} ->
             StaticEffect bs
objectCant deed what =
  Deontic what Forbid [deed] Patient NoDeonticPatient Nothing NoDeonticRider {kd} {zn} {dp}

public export
cantDoTo : {k : Kind} -> {kw : Kind} -> (deed : VerbLabel) ->
           (who : Noun bs k) -> (what : Noun (nomIntro who) kw) ->
           {auto 0 kd : KnownDeeds [deed]} ->
           {auto 0 zn : ZoneFits (nounZone who) (deedsZone [deed] Agent)} ->
           {auto 0 dp : DeedParticipant [deed] Agent k (nounHeadTys who)} ->
           {auto 0 pt : So (deonticPatientOk who [deed] Agent
                              (DeonticCounterpart what)
                              NoDeonticRider)} ->
           StaticEffect bs
cantDoTo deed who what =
  Deontic who Forbid [deed] Agent (DeonticCounterpart what) Nothing NoDeonticRider
          {kd} {zn} {dp} {pt}

public export
cantBeTargetedBy : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                   (by : Noun (nomIntro what) ka) ->
                   {auto 0 tr : Targeter ka} ->
                   {auto 0 zn : ZoneFits (nounZone what) (deedsZone ["Target"] Patient)} ->
                   {auto 0 dp : DeedParticipant ["Target"] Patient k (nounHeadTys what)} ->
                   StaticEffect bs
cantBeTargetedBy what by =
  Deontic what Forbid ["Target"] Patient (TargetedBy by {tr}) Nothing NoDeonticRider {zn} {dp}

public export
canBeTargetedAsThough : {k : Kind} -> {ka : Kind} -> (what : Noun bs k) ->
                        (by : Noun (nomIntro what) ka) ->
                        (p : Predicate (nomIntro what) Object) ->
                        {auto 0 tr : Targeter ka} ->
                        {auto 0 zn : ZoneFits (nounZone what) (deedsZone ["Target"] Patient)} ->
                        {auto 0 dp : DeedParticipant ["Target"] Patient k (nounHeadTys what)} ->
                        StaticEffect bs
canBeTargetedAsThough what by p =
  Deontic what Permit ["Target"] Patient (TargetedBy by {tr}) (Just (AsThoughOf p))
          NoDeonticRider {zn} {dp}

public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Patient)} ->
                {auto 0 dp : DeedParticipant ["Block"] Patient Object (nounHeadTys n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span =
  Continuously (Deontic n Forbid ["Block"] Patient NoDeonticPatient Nothing NoDeonticRider {zn} {dp}) span {sp}

public export
mustBlockIt : {bs : Bindings} -> (n : Noun bs Object) ->
              (span : Maybe (Duration (selfSubjIntro n))) ->
              {auto 0 zn : ZoneFits (nounZone n) (deedsZone ["Block"] Agent)} ->
              {auto 0 dp : DeedParticipant ["Block"] Agent Object (nounHeadTys n)} ->
              {auto 0 ok : countOnes Object bs = 1} ->
              {auto 0 pt : So (deonticPatientOk n ["Block"] Agent
                                 (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok}))
                                 NoDeonticRider)} ->
              {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
mustBlockIt n span =
  Continuously (Deontic n Require ["Block"] Agent
                  (DeonticCounterpart (ItOtherThan (nounDelta n) bs {ok}))
                  Nothing NoDeonticRider {zn} {dp} {pt})
               span {sp}


public export
attachToIt : {bs : Bindings} -> (what : Noun bs Object) ->
             {auto 0 zw : OnBattlefield (nounZone what)} ->
             {auto 0 ok : countOnes Object bs = 1} -> Effect bs
attachToIt what = AttachTo what (ItOtherThan (nounDelta what) bs {ok}) {zw}


public export
gainControl : (n : Noun bs Object) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              (d : Maybe (Duration (staticIntro (GainsControl You n {zn})))) ->
              {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainControl n d = Continuously (GainsControl You n {zn}) d {sp}

public export
gainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
               {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} ->
               (d : Maybe (Duration (staticIntro (GainsControl who what {zn})))) ->
               {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainsControl who what d = Continuously (GainsControl who what {zn}) d {sp}

public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)


public export
may : (decider : Noun bs Player) -> Effect (agentIntro decider) -> Effect bs
may d body = May d body Nothing Nothing

public export
mayThen : (decider : Noun bs Player) -> (body : Effect (agentIntro decider)) ->
          Effect (effIntro body) -> Effect bs
mayThen d body did = May d body (Just did) Nothing

public export
mayElse : (decider : Noun bs Player) -> (body : Effect (agentIntro decider)) ->
          Effect (agentIntro decider) -> Effect bs
mayElse d body notd = May d body Nothing (Just notd)

public export
mayThenElse : (decider : Noun bs Player) ->
              (body : Effect (agentIntro decider)) ->
              Effect (effIntro body) -> Effect (agentIntro decider) -> Effect bs
mayThenElse d body did notd = May d body (Just did) (Just notd)


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
         {auto 0 tt : TokenTyped tok} ->
         {auto 0 tp : TokenPt tok} ->
         {auto 0 sf : SubtypesFit tok} ->
         {auto 0 ta : TokenAbilities tok} ->
         {auto 0 tc : TokenCanonical tok} ->
         {auto 0 qf : TokenQualsFit tok} ->
         Effect bs
create count tok = Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc} {qf}) []

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} ->
                        {auto 0 ta : TokenAbilities tok} ->
                        {auto 0 tc : TokenCanonical tok} ->
                        {auto 0 qf : TokenQualsFit tok} ->
                        Effect bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc} {qf})
         [EntersTapped, EntersAttacking NoDefender]

public export
becomesAs : (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 sw : AdditionSaysSomething (nounTy n) added} ->
            {auto 0 af : AddedFits (nounTy n) added.line} ->
            {auto 0 tc : TokenCanonical added} ->
            {auto 0 ta : TokenAbilities added} ->
            {auto 0 qf : TokenQualsFit added} ->
            {auto 0 un : AdditionUnnamed added} ->
            {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomesAs n added d =
  Continuously (BecomesAlso n added {sw} {af} {tc} {ta} {qf} {un}) d {sp}

public export
becomes : (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 sw : AdditionSaysSomething (nounTy n) (MkToken {bs} Nothing [] added [] Nothing)} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 tc : TokenCanonical (MkToken {bs} Nothing [] added [] Nothing)} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = becomesAs n (MkToken Nothing [] added [] Nothing) d {sw} {af} {tc} {sp}

public export
becomesColor : (n : Noun bs Object) -> (cs : ColorSpec) ->
               (d : Maybe (Duration (selfSubjIntro n))) ->
               {auto 0 cd : ColorSpecOk cs} ->
               {auto 0 sp : SpanOk ColorSet d} -> Effect bs
becomesColor n cs d = Continuously (SetsColor n cs {cd}) d {sp}

public export
basicLandLine : (ss : List Subtype) -> {auto 0 bl : BasicLandTypes ss} -> TypeLine
basicLandLine ss = MkTypeLine ss []


public export
drawACard : Effect bs
drawACard = Draw You (Lit 1)

public export
drawCards : (n : Nat) -> Effect bs
drawCards n = Draw You (Lit n)

public export
drawsACard : (who : Noun bs Player) -> Effect bs
drawsACard who = Draw who (Lit 1)


public export
chooseOne : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 1) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly {bs} 2) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Macros.oneOrBoth {bs}) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrBoth modes = Modal Macros.oneOrBoth modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast {bs} 1) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (Macros.anyNumber {bs}) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {wf = Oh} {tw} {mf} {dm = eqToSo dm}

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
itsACard p = Matches (itAsCard {ok}) p {sy} {zc}

public export
itsAnAbility : (p : Predicate bs Ability) ->
               {auto 0 ok : countReach (Word AbilityW) OneOf bs = 1} ->
               {auto 0 sy : PredSays p} ->
               {auto 0 bl : TestSubject (ItAbility {bs} {ok})} ->
               {auto 0 zc : ZoneFits (zoneOfReach (Word AbilityW) OneOf bs)
                                           (seedZone p)} ->
               Condition bs
itsAnAbility p = Matches (ItAbility {ok}) p {sy} {bl} {zc}

public export
itIsntAnAbility : (p : Predicate bs Ability) ->
                  {auto 0 ok : countReach (Word AbilityW) OneOf bs = 1} ->
                  {auto 0 sy : PredSays p} ->
                  {auto 0 bl : TestSubject (ItAbility {bs} {ok})} ->
                  {auto 0 zc : ZoneFits (zoneOfReach (Word AbilityW) OneOf bs)
                                              (seedZone p)} ->
                  Condition bs
itIsntAnAbility p = NotCond (itsAnAbility p {ok} {sy} {bl} {zc})


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
shuffledIntoZ : ZoneExpr bs
shuffledIntoZ = LibraryAt Shuffled Nothing Nothing Bare

public export
shuffleInto : (n : Noun bs Object) ->
              {auto 0 pl : Placeable (nounTy n) Library} -> Effect bs
shuffleInto n = Enact "Shuffle" (Move n shuffledIntoZ noRiders {pl})

public export
shufflesInto : (agent : Noun bs Player) -> (n : Noun (agentIntro agent) Object) ->
               {auto 0 pl : Placeable (nounTy n) Library} -> Effect bs
shufflesInto agent n = Does agent "Shuffle" (Move n shuffledIntoZ noRiders {pl})

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
puts agent n to = Does agent "Put" (Move n to noRiders {ok} {arr} {pl})

public export
shuffle : Effect bs
shuffle = Shuffle You

public export
lifeTotalBecomes : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
lifeTotalBecomes who a = ChangeLife who (Set a)


public export
ifWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                 (d : Maybe (Duration (eventIntro ev))) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 sp : SpanOk Replacement d} -> Effect bs
ifWouldInstead ev repl d = Continuously (Intercepts ev [] Nothing repl Repeatedly Nothing {ok}) d {sp}

public export
nextTimeWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 sp : SpanOk Replacement d} -> Effect bs
nextTimeWouldInstead ev repl d =
  Continuously (Intercepts ev [] Nothing repl NextTimeOnly Nothing {ok}) d {sp}

public export
preventAll : (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAll kind scope d = Continuously (Prevents kind AllOfIt scope Nothing Nothing) d {sp}

public export
preventNext : (kind : DamageKind) -> (amt : Amount bs) ->
              (scope : DamageScope (amtIntro amt)) ->
              (d : Maybe (Duration (scopeIntro scope))) ->
              {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNext kind amt scope d =
  Continuously (Prevents kind (TheNext amt) scope Nothing Nothing) d {sp}

public export
preventAllBy : (kind : DamageKind) -> (scope : DamageScope bs) ->
               (src : Noun (scopeIntro scope) Object) ->
               (d : Maybe (Duration (nomIntro src))) ->
               {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAllBy kind scope src d =
  Continuously (Prevents kind AllOfIt scope (Just src) Nothing) d {sp}

public export
preventNextBy : (kind : DamageKind) -> (amt : Amount bs) ->
                (scope : DamageScope (amtIntro amt)) ->
                (src : Noun (scopeIntro scope) Object) ->
                (d : Maybe (Duration (nomIntro src))) ->
                {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNextBy kind amt scope src d =
  Continuously (Prevents kind (TheNext amt) scope (Just src) Nothing) d {sp}

public export
shieldingIt : {k : Kind} -> (n : Noun bs k) ->
              {auto 0 rk : DamageRecipient n} -> DamageScope bs
shieldingIt n = ToRecipient n {rk}

public export
exileUntil : (n : Noun bs Object) -> (ev : GameEvent (preIntro (exile n))) ->
             Effect bs
exileUntil n ev = HeldUntil (exile n) ev {ok = Oh}

public export
phasesOutUntil : (n : Noun bs Object) ->
                 {auto 0 zn : OnBattlefield (nounZone n)} ->
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
monoHybridPip : Nat -> Color -> ManaSymbol
monoHybridPip n c = Hybrid (Generic n) c

public export
phyrexianPip : Color -> ManaSymbol
phyrexianPip c = Phyrexian c Nothing

public export
payLife : (who : Noun bs Player) -> (n : Nat) -> Cost bs
payLife who n = Do (ChangeLife who (Down (Lit n)))

public export
doThen : (body : Effect bs) -> Effect (effIntro body) ->
         {auto 0 en : ReflexEnclosure body} -> Effect bs
doThen body did = IfDone body (Just did) Nothing {en}

public export
doElse : (body : Effect bs) -> Effect bs ->
         {auto 0 en : ReflexEnclosure body} -> Effect bs
doElse body notd = IfDone body Nothing (Just notd) {en}

public export
doThenElse : (body : Effect bs) -> Effect (effIntro body) -> Effect bs ->
             {auto 0 en : ReflexEnclosure body} -> Effect bs
doThenElse body did notd = IfDone body (Just did) (Just notd) {en}

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
  Does agent "Mill"
       (Move (LibrarySlice OnTop amt whose {sp}) graveyardZ noRiders)

public export
lookedTop : (bs : Bindings) -> (amt : Amount bs) -> Bindings
lookedTop bs amt = nomIntro (topSlice {bs} amt)

public export
lookedRest : (bs : Bindings) -> (0 mn : countReach Bare ManyOf bs = 1) ->
             (z : Zone) -> Bindings
lookedRest bs mn z = moveIntro {bs} Nothing (SomeOf (CountedSlice anyNumber {wf = Oh}) Nothing (Them {ok = mn}) {gm = Oh}) (Just z)

public export
theyLookAtTop : {bs : Bindings} -> (amt : Amount bs) ->
                {auto 0 an : countReach (Word PlayerW) OneOf bs = 1} -> Effect bs
theyLookAtTop amt =
  Expose LookAt (They {ok = an})
         (ExposedCards (LibrarySlice OnTop amt (They {ok = an})))

public export
scry : {bs : Bindings} -> (amt : Amount bs) ->
       {auto 0 mn : countReach Bare ManyOf (lookedTop bs amt) = 1} ->
       {auto 0 ps : Placeable (tyOfReach Bare ManyOf (lookedTop bs amt)) Library} ->
       {auto 0 tr : So (theRestOk (lookedRest (lookedTop bs amt) mn Library))} ->
       {auto 0 pr : Placeable (tyOfGroup (lookedRest (lookedTop bs amt) mn Library)) Library} ->
       Effect bs
scry amt =
  Does You "Scry" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf (CountedSlice anyNumber {wf = Oh}) Nothing (Them {ok = mn}) {gm = Oh})
                            (onBottomIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
surveil : {bs : Bindings} -> (amt : Amount bs) ->
          {auto 0 mn : countReach Bare ManyOf (lookedTop bs amt) = 1} ->
          {auto 0 ps : Placeable (tyOfReach Bare ManyOf (lookedTop bs amt)) Graveyard} ->
          {auto 0 tr : So (theRestOk (lookedRest (lookedTop bs amt) mn Graveyard))} ->
          {auto 0 pr : Placeable (tyOfGroup (lookedRest (lookedTop bs amt) mn Graveyard)) Library} ->
          Effect bs
surveil amt =
  Does You "Surveil" {kn = Oh}
       (Sequentially [ lookAt (topSlice amt)
                     , move (SomeOf (CountedSlice anyNumber {wf = Oh}) Nothing (Them {ok = mn}) {gm = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
scryOne : {bs : Bindings} ->
          {auto 0 iw : countReach (Word CardW) OneOf (lookedTop bs (Lit 1)) = 1} ->
          {auto 0 pi : Placeable (tyOfReach (Word CardW) OneOf (lookedTop bs (Lit 1))) Library} ->
          Effect bs
scryOne =
  Does You "Scry" {kn = Oh}
       (Sequentially [ lookAt topCard
                     , may You (move (That CardW {ok = iw}) onBottomZ {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pi}) ])

public export
surveilOne : {bs : Bindings} ->
             {auto 0 iw : countReach (Word CardW) OneOf (lookedTop bs (Lit 1)) = 1} ->
             {auto 0 pi : Placeable (tyOfReach (Word CardW) OneOf (lookedTop bs (Lit 1))) Graveyard} ->
             Effect bs
surveilOne =
  Does You "Surveil" {kn = Oh}
       (Sequentially [ lookAt topCard
                     , may You (move (That CardW {ok = iw}) graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = pi}) ])

public export
playerScries : {bs : Bindings} -> (agent : Noun bs Player) ->
               (amt : Amount (agentIntro agent)) ->
               {auto 0 an : countReach (Word PlayerW) OneOf (agentIntro agent) = 1} ->
               {auto 0 mn : countReach Bare ManyOf (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) = 1} ->
               {auto 0 ps : Placeable (tyOfReach Bare ManyOf (nomIntro (LibrarySlice OnTop amt (They {ok = an})))) Library} ->
               {auto 0 tr : So (theRestOk (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Library))} ->
               {auto 0 pr : Placeable (tyOfGroup (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Library)) Library} ->
               Effect bs
playerScries agent amt =
  Does agent "Scry" {kn = Oh}
       (Sequentially [ theyLookAtTop amt {an}
                     , move (SomeOf (CountedSlice anyNumber {wf = Oh}) Nothing (Them {ok = mn}) {gm = Oh})
                            (onBottomIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
playerSurveils : {bs : Bindings} -> (agent : Noun bs Player) ->
                 (amt : Amount (agentIntro agent)) ->
                 {auto 0 an : countReach (Word PlayerW) OneOf (agentIntro agent) = 1} ->
                 {auto 0 mn : countReach Bare ManyOf (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) = 1} ->
                 {auto 0 ps : Placeable (tyOfReach Bare ManyOf (nomIntro (LibrarySlice OnTop amt (They {ok = an})))) Graveyard} ->
                 {auto 0 tr : So (theRestOk (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Graveyard))} ->
                 {auto 0 pr : Placeable (tyOfGroup (lookedRest (nomIntro (LibrarySlice OnTop amt (They {ok = an}))) mn Graveyard)) Library} ->
                 Effect bs
playerSurveils agent amt =
  Does agent "Surveil" {kn = Oh}
       (Sequentially [ theyLookAtTop amt {an}
                     , move (SomeOf (CountedSlice anyNumber {wf = Oh}) Nothing (Them {ok = mn}) {gm = Oh})
                            graveyardZ {ok = GraveyardOkBare} {arr = Oh} {pl = ps}
                     , move (TheRest {ok = tr})
                            (onTopIn AnyOrder {af = Oh}) {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pr} ])

public export
theirTopCard : (bs : Bindings) -> (0 an : countReach (Word PlayerW) OneOf bs = 1) -> Bindings
theirTopCard bs an = nomIntro (LibrarySlice {bs} OnTop (Lit 1) (They {ok = an}))

public export
playerScriesOne : {bs : Bindings} -> (agent : Noun bs Player) ->
                  {auto 0 an : countReach (Word PlayerW) OneOf (agentIntro agent) = 1} ->
                  {auto 0 ap : countReach (Word PlayerW) OneOf (Macros.theirTopCard (agentIntro agent) an) = 1} ->
                  {auto 0 iw : countReach (Word CardW) OneOf (Macros.theirTopCard (agentIntro agent) an) = 1} ->
                  {auto 0 pi : Placeable (tyOfReach (Word CardW) OneOf (Macros.theirTopCard (agentIntro agent) an)) Library} ->
                  Effect bs
playerScriesOne agent =
  Does agent "Scry" {kn = Oh}
       (Sequentially [ theyLookAtTop (Lit 1) {an}
                     , may (They {ok = ap})
                           (move (That CardW {ok = iw}) onBottomZ
                                 {ok = LibraryPosOk {af = Oh} {nf = Oh}} {arr = Oh} {pl = pi}) ])

public export
playerSurveilsOne : {bs : Bindings} -> (agent : Noun bs Player) ->
                    {auto 0 an : countReach (Word PlayerW) OneOf (agentIntro agent) = 1} ->
                    {auto 0 ap : countReach (Word PlayerW) OneOf (Macros.theirTopCard (agentIntro agent) an) = 1} ->
                    {auto 0 iw : countReach (Word CardW) OneOf (Macros.theirTopCard (agentIntro agent) an) = 1} ->
                    {auto 0 pi : Placeable (tyOfReach (Word CardW) OneOf (Macros.theirTopCard (agentIntro agent) an)) Graveyard} ->
                    Effect bs
playerSurveilsOne agent =
  Does agent "Surveil" {kn = Oh}
       (Sequentially [ theyLookAtTop (Lit 1) {an}
                     , may (They {ok = ap})
                           (move (That CardW {ok = iw}) graveyardZ
                                 {ok = GraveyardOkBare} {arr = Oh} {pl = pi}) ])

public export
playerSearchesTheirLibraryFor : (who : Noun bs Player) ->
                                {auto 0 an : countReach (Word PlayerW) OneOf (nomIntro who) = 1} ->
                                (p : Predicate (nomIntro who) Object) ->
                                {auto 0 zf : ZoneFree p} -> Effect bs
playerSearchesTheirLibraryFor who p =
  Search who (OneZone (libraryOf (They {ok = an}))) (exactly 1) p {zf}

public export
proliferable : Predicate bs (Object \/ Player)
proliferable = kindJoin (CounterCompare Nothing AtLeast (Lit 1))
                        (And [Permanent, HasCounters Nothing])

public export
proliferated : (bs : Bindings) -> Bindings
proliferated bs = chosenIntro {bs} (CountedGroup Macros.anyNumber Nothing Macros.proliferable)

public export
proliferate : {bs : Bindings} ->
              {auto 0 mj : countReach (Word JoinW) ManyOf (Macros.proliferated bs) = 1} ->
              Effect bs
proliferate =
  Enact "Proliferate" {kn = Oh}
        (Sequentially [ Choose (CountedGroup Macros.anyNumber Nothing Macros.proliferable) Nothing Openly
                      , GiveCountersOfOwnKinds (EachOf (Those JoinW {ok = mj})) ])

public export
losesCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                (kind : Maybe CounterKind) ->
                {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesCounters who amt kind = LosesCounters who kind (Just amt) {pk}

public export
losesAllCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                   {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesAllCounters who kind = LosesCounters who kind Nothing {pk}

public export
removeCounters : (q : Quantity bs) -> (kind : Maybe CounterKind) ->
                 (from : Noun (quantIntro q) Object) ->
                 {auto 0 wf : WellFormedQ q} ->
                 {auto 0 kn : CounterKindNamed Object kind} ->
                 {auto 0 cm : CounterMemory from} -> Effect bs
removeCounters q kind from = RemoveCounters (Just q) kind from {wf} {kn} {cm}

public export
removeAllCounters : (kind : Maybe CounterKind) -> (from : Noun bs Object) ->
                    {auto 0 kn : CounterKindNamed Object kind} ->
                    {auto 0 cm : CounterMemory from} -> Effect bs
removeAllCounters kind from = RemoveCounters Nothing kind from {kn} {cm}

public export
keyword : {0 bs : Bindings} -> (kw : KeywordLabel) ->
          {auto 0 pf : KeywordParamFits {bs} kw
                         (the (Maybe (KeywordParam bs)) Nothing)} ->
          AbilityAt bs
keyword kw = KeywordAbility kw Nothing {pf}

public export
keywordSubject : {0 bs : Bindings} -> {k : Kind} -> (kw : KeywordLabel) ->
                 (p : Predicate [] k) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamSubject {bs} p))} ->
                 AbilityAt bs
keywordSubject kw p = KeywordAbility kw (Just (ParamSubject p)) {pf}

public export
keywordCosting : {0 bs : Bindings} -> (kw : KeywordLabel) -> (c : Cost []) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamCost {bs} c))} ->
                 AbilityAt bs
keywordCosting kw c = KeywordAbility kw (Just (ParamCost c)) {pf}

public export
keywordQuality : {k : Kind} -> (kw : KeywordLabel) -> (q : Predicate bs k) ->
                 {auto 0 pk : So (qualityParamKind k)} ->
                 {auto 0 pf : KeywordParamFits kw (Just (ParamQuality q {pk}))} ->
                 AbilityAt bs
keywordQuality kw q = KeywordAbility kw (Just (ParamQuality q {pk})) {pf}

public export
keywordNumber : {0 bs : Bindings} -> (kw : KeywordLabel) -> (amt : Amount []) ->
                {auto 0 pf : KeywordParamFits {bs} kw
                               (Just (ParamNumber {bs} amt))} ->
                AbilityAt bs
keywordNumber kw amt = KeywordAbility kw (Just (ParamNumber amt)) {pf}

public export
keywordQualityCosting : {0 bs : Bindings} -> (kw : KeywordLabel) ->
                        (q : Predicate bs Object) -> (c : Cost []) ->
                        {auto 0 pf : KeywordParamFits kw
                                       (Just (ParamQualityCost q c))} ->
                        AbilityAt bs
keywordQualityCosting kw q c = KeywordAbility kw (Just (ParamQualityCost q c)) {pf}

public export
keywordNumberCosting : {0 bs : Bindings} -> (kw : KeywordLabel) ->
                       (amt : Amount []) -> (c : Cost []) ->
                       {auto 0 pf : KeywordParamFits {bs} kw
                                      (Just (ParamNumberCost {bs} amt c))} ->
                       AbilityAt bs
keywordNumberCosting kw amt c = KeywordAbility kw (Just (ParamNumberCost amt c)) {pf}

public export
abilityWord : {0 bs : Bindings} -> (word : AbilityWordName) ->
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
                      (ev : GameEvent bs) -> (w : TriggerWindow) ->
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
                      (w : Timing) ->
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
mayPlayDeed : (deed : VerbLabel) -> (who : Noun bs Player) ->
              (what : Noun (nomIntro who) Object) ->
              (rider : DeonticRider (nomIntro what)) ->
              {auto 0 kd : KnownDeeds [deed]} ->
              {auto 0 dd : So (distinctDeeds [deed])} ->
              {auto 0 zn : ZoneFits (nounZone who) (deedsZone [deed] Agent)} ->
              {auto 0 dp : DeedParticipant [deed] Agent Player (nounHeadTys who)} ->
              {auto 0 pt : So (deonticPatientOk who [deed] Agent
                                                (DeonticCounterpart what)
                                                rider)} ->
              {auto 0 rd : So (deonticRiderOk [deed] Agent
                                              (Permit {bs = selfSubjIntro who})
                                              (DeonticCounterpart what) False
                                              rider)} ->
              StaticEffect bs
mayPlayDeed deed who what rider =
  Deontic who Permit [deed] Agent (DeonticCounterpart what) Nothing rider
          {kd} {dd} {zn} {dp} {pt} {rd}

public export
mayPlay : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
          {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Play"] Agent)} ->
          {auto 0 dp : DeedParticipant ["Play"] Agent Player (nounHeadTys who)} ->
          {auto 0 pt : So (deonticPatientOk who ["Play"] Agent
                                            (DeonticCounterpart what)
                                            (PlayRider Nothing Nothing Nothing False ItsOwnCost))} ->
          {auto 0 rd : So (deonticRiderOk ["Play"] Agent
                                          (Permit {bs = selfSubjIntro who})
                                          (DeonticCounterpart what) False
                                          (PlayRider Nothing Nothing Nothing
                                                     False ItsOwnCost))} ->
          StaticEffect bs
mayPlay who what =
  mayPlayDeed "Play" who what (PlayRider Nothing Nothing Nothing False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
              {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
              {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                (DeonticCounterpart what)
                                                (PlayRider (Just from) Nothing Nothing False ItsOwnCost))} ->
              {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                              (Permit {bs = selfSubjIntro who})
                                              (DeonticCounterpart what) False
                                              (PlayRider (Just from) Nothing Nothing
                                                         False ItsOwnCost))} ->
              StaticEffect bs
mayCastFrom who what from =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing Nothing False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastFromPaying : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                    (from : ZoneExpr (nomIntro what)) ->
                    (c : Cost (nomIntro what)) ->
                    {auto 0 cf : So (costOffBattlefield c)} ->
                    {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                    {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                    {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                      (DeonticCounterpart what)
                                                      (PlayRider (Just from) Nothing Nothing False
                                                                 (PayingInstead c {ok = cf})))} ->
                    {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                    (Permit {bs = selfSubjIntro who})
                                                    (DeonticCounterpart what) False
                                                    (PlayRider (Just from) Nothing Nothing False
                                                               (PayingInstead c {ok = cf})))} ->
                    StaticEffect bs
mayCastFromPaying who what from c =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing Nothing False (PayingInstead c {ok = cf}))
              {zn} {dp} {pt} {rd}

public export
mayPlayFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Play"] Agent)} ->
              {auto 0 dp : DeedParticipant ["Play"] Agent Player (nounHeadTys who)} ->
              {auto 0 pt : So (deonticPatientOk who ["Play"] Agent
                                                (DeonticCounterpart what)
                                                (PlayRider (Just from) Nothing Nothing False ItsOwnCost))} ->
              {auto 0 rd : So (deonticRiderOk ["Play"] Agent
                                              (Permit {bs = selfSubjIntro who})
                                              (DeonticCounterpart what) False
                                              (PlayRider (Just from) Nothing Nothing
                                                         False ItsOwnCost))} ->
              StaticEffect bs
mayPlayFrom who what from =
  mayPlayDeed "Play" who what
              (PlayRider (Just from) Nothing Nothing False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastFromLimited : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     (from : ZoneExpr (nomIntro what)) -> (lim : PlayLimit) ->
                     {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                     {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                     {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                       (DeonticCounterpart what)
                                                       (PlayRider (Just from) (Just lim) Nothing False ItsOwnCost))} ->
                     {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                     (Permit {bs = selfSubjIntro who})
                                                     (DeonticCounterpart what) False
                                                     (PlayRider (Just from) (Just lim)
                                                                Nothing False ItsOwnCost))} ->
                     StaticEffect bs
mayCastFromLimited who what from lim =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) (Just lim) Nothing False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastFromOnly : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                  (from : ZoneExpr (nomIntro what)) ->
                  {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                  {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                  {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                    (DeonticCounterpart what)
                                                    (PlayRider (Just from) Nothing Nothing True ItsOwnCost))} ->
                  {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                  (Permit {bs = selfSubjIntro who})
                                                  (DeonticCounterpart what) False
                                                  (PlayRider (Just from) Nothing Nothing
                                                             True ItsOwnCost))} ->
                  StaticEffect bs
mayCastFromOnly who what from =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing Nothing True ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayPlayFromEachYourTurn : (who : Noun bs Player) ->
                          (what : Noun (nomIntro who) Object) ->
                          (from : ZoneExpr (nomIntro what)) ->
                          {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Play"] Agent)} ->
                          {auto 0 dp : DeedParticipant ["Play"] Agent Player (nounHeadTys who)} ->
                          {auto 0 pt : So (deonticPatientOk who ["Play"] Agent
                                                            (DeonticCounterpart what)
                                                            (PlayRider (Just from) Nothing (Just DuringEachOfYourTurns) False ItsOwnCost))} ->
                          {auto 0 rd : So (deonticRiderOk ["Play"] Agent
                                                          (Permit {bs = selfSubjIntro who})
                                                          (DeonticCounterpart what) False
                                                          (PlayRider (Just from) Nothing
                                                                     (Just DuringEachOfYourTurns)
                                                                     False ItsOwnCost))} ->
                          StaticEffect bs
mayPlayFromEachYourTurn who what from =
  mayPlayDeed "Play" who what
              (PlayRider (Just from) Nothing (Just DuringEachOfYourTurns) False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastFromFree : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                  (from : ZoneExpr (nomIntro what)) ->
                  {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                  {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                  {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                    (DeonticCounterpart what)
                                                    (PlayRider (Just from) Nothing Nothing False WithoutPaying))} ->
                  {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                  (Permit {bs = selfSubjIntro who})
                                                  (DeonticCounterpart what) False
                                                  (PlayRider (Just from) Nothing Nothing
                                                             False WithoutPaying))} ->
                  StaticEffect bs
mayCastFromFree who what from =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing Nothing False WithoutPaying)
              {zn} {dp} {pt} {rd}

public export
mayCastFromEachYourTurn : (who : Noun bs Player) ->
                          (what : Noun (nomIntro who) Object) ->
                          (from : ZoneExpr (nomIntro what)) ->
                          {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                          {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                          {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                            (DeonticCounterpart what)
                                                            (PlayRider (Just from) Nothing (Just DuringEachOfYourTurns) False ItsOwnCost))} ->
                          {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                          (Permit {bs = selfSubjIntro who})
                                                          (DeonticCounterpart what) False
                                                          (PlayRider (Just from) Nothing
                                                                     (Just DuringEachOfYourTurns)
                                                                     False ItsOwnCost))} ->
                          StaticEffect bs
mayCastFromEachYourTurn who what from =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing (Just DuringEachOfYourTurns) False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
mayCastAsThough : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                  {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                  {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                  {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                    (DeonticCounterpart what)
                                                    (PlayRider Nothing Nothing Nothing False ItsOwnCost))} ->
                  {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                  (Permit {bs = selfSubjIntro who})
                                                  (DeonticCounterpart what) True
                                                  (PlayRider Nothing Nothing Nothing
                                                             False ItsOwnCost))} ->
                  StaticEffect bs
mayCastAsThough who what =
  Deontic who Permit ["Cast"] Agent (DeonticCounterpart what)
          (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
          (PlayRider Nothing Nothing Nothing False ItsOwnCost)
          {zn} {dp} {pt} {rd}

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
entersWithAdditionalCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                               (kind : CounterKind) -> StaticEffect bs
entersWithAdditionalCounters n amt kind =
  EntersWithCounters n amt (PrintedKind kind) Additional

public export
entersWithFewerCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                          (kind : CounterKind) -> StaticEffect bs
entersWithFewerCounters n amt kind = EntersWithCounters n amt (PrintedKind kind) Fewer

public export
attacks : (n : Noun bs Object) ->
          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
attacks n = Attacks n NoDefender {zn}

public export
attacksPlayer : {k : Kind} -> (n : Noun bs Object) -> (whom : Noun (nomIntro n) k) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 sg : nounPlur whom = OneOf} ->
                {auto 0 at : Attackable whom} -> GameEvent bs
attacksPlayer n whom = Attacks n (OneDefender whom {sg} {at}) {zn}

public export
tokensCreated : (n : Noun bs Object) ->
                {auto 0 tk : TokenPhrase n} -> GameEvent bs
tokensCreated n = TokensCreated n Nothing Nothing Nothing {tk}

public export
tokensCreatedUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                     {auto 0 tk : TokenPhrase n} ->
                     {auto 0 vo : CreationVoice Nothing Nothing (Just under)} ->
                     GameEvent bs
tokensCreatedUnder n under = TokensCreated n Nothing Nothing (Just under) {tk} {vo}

public export
tokensCreatedByEffectUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                             {auto 0 tk : TokenPhrase n} ->
                             {auto 0 vo : CreationVoice (Just AnEffect) Nothing
                                                        (Just under)} ->
                             GameEvent bs
tokensCreatedByEffectUnder n under =
  TokensCreated n (Just AnEffect) Nothing (Just under) {tk} {vo}

public export
manyCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                   (n : Noun bs Object) ->
                   {auto 0 sc : counterScope kind = Object} -> GameEvent bs
manyCounterEvent dir kind n =
  CounterEvent dir (Just kind) n ManyCounters Nothing Nothing

public export
singleCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                     (n : Noun bs Object) ->
                     {auto 0 sc : counterScope kind = Object} -> GameEvent bs
singleCounterEvent dir kind n =
  CounterEvent dir (Just kind) n OneCounter Nothing Nothing

public export
bareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                   GameEvent bs
bareCounterEvent dir n = CounterEvent dir Nothing n OneCounter Nothing Nothing

public export
manyBareCounterEvent : {k : Kind} -> (dir : CounterMove) -> (n : Noun bs k) ->
                       GameEvent bs
manyBareCounterEvent dir n =
  CounterEvent dir Nothing n ManyCounters Nothing Nothing

public export
manyCountersPutByEffect : (n : Noun bs Object) -> GameEvent bs
manyCountersPutByEffect n =
  CounterEvent CounterPut Nothing n ManyCounters Nothing (Just AnEffect)

public export
manyBareCountersPutBy : (who : Noun bs Player) -> (n : Noun bs Object) ->
                        {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
manyBareCountersPutBy who n =
  CounterEvent CounterPut Nothing n ManyCounters (Just who) Nothing {ag}

public export
lastCounterRemovedBy : (kind : CounterKind) -> (n : Noun bs Object) ->
                       (who : Noun bs Player) ->
                       {auto 0 sc : counterScope kind = Object} ->
                       {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
lastCounterRemovedBy kind n who =
  LastCounterRemoved kind n (Just who) {sc} {ag}

public export
choose : {k : Kind} -> (n : Noun bs k) ->
         {auto 0 ch : ChoiceClause (the (Maybe (Noun bs Player)) Nothing) n} ->
         Effect bs
choose n = Choose n Nothing Openly {ch}

public export
chooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
          {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
chooses who n = Choose n (Just who) Openly {ch}

public export
secretlyChooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
                  {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
secretlyChooses who n = Choose n (Just who) Secretly {ch}

public export
secretlyChoose : {k : Kind} -> (n : Noun bs k) ->
                 {auto 0 ch : ChoiceClause (the (Maybe (Noun bs Player)) Nothing) n} ->
                 Effect bs
secretlyChoose n = Choose n Nothing Secretly {ch}

public export
aCardInHand : Noun bs Object
aCardInHand = a (InZone handZ)

public export
handPick : (bs : Bindings) -> Bindings
handPick bs = nomIntro {bs} (aCardInHand {bs})

public export
discardN : (amt : Amount bs) ->
           {auto 0 pk : countReach (Word CardW) OneOf (handPick (amtIntro amt)) = 1} ->
           {auto 0 dz : zoneOfReach (Word CardW) OneOf (handPick (amtIntro amt)) = Just Hand} ->
           Effect bs
discardN amt =
  Repeated amt (Sequentially [ choose (aCardInHand)
                             , discard (That CardW {ok = pk})
                                       {dk = DiscardTracked {z = dz}} ])

public export
itVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
           {auto 0 ok : countReach (Stamped v) OneOf bs = 1} -> Noun bs Object
itVerbed v = ItVerbed v {kn} {ok}

public export
themVerbed : (v : VerbLabel) -> {auto 0 kn : KnownVerb v} ->
             {auto 0 ok : countReach (Stamped v) ManyOf bs = 1} -> Noun bs Object
themVerbed v = ThemVerbed v {kn} {ok}

public export
itPrior : {bs : Bindings} -> (prev : Effect bs) ->
          {auto 0 sp : effIntro prev = effDelta prev ++ bs} ->
          {auto 0 ok : countOnes Object (effDelta prev) = 1} ->
          Noun (effIntro prev) Object
itPrior prev = ItPrior (effDelta prev) bs {sp} {ok}

public export
dealsDamageOwnPower : {bs : Bindings} -> {k : Kind} -> (src : Noun bs Object) ->
                      {auto 0 ok : countReach Bare OneOf (nounDelta src) = 1} ->
                      (to : Noun (nounDelta src ++ bs) k) ->
                      {auto 0 pm : PerMember to} ->
                      {auto 0 rk : DamageRecipient to} -> Effect bs
dealsDamageOwnPower src to =
  DealDamage src (StatOf Power (Own (nounDelta src) bs {sp = Refl} {ok})) to {pm} {rk}

public export
theVerbed : (v : VerbLabel) -> (w : NounWord) ->
            {auto 0 ok : countReach (Verbed v w Attributive) OneOf bs = 1} ->
            {auto 0 mk : VerbedMarkingOk v Attributive} -> Noun bs (kindOfW w)
theVerbed v w = TheVerbed v w Attributive {ok} {mk}

public export
theVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                   {auto 0 ok : countReach (Verbed v w ThisWay) OneOf bs = 1} ->
                   {auto 0 mk : VerbedMarkingOk v ThisWay} -> Noun bs (kindOfW w)
theVerbedThisWay v w = TheVerbed v w ThisWay {ok} {mk}

public export
thoseVerbed : (v : VerbLabel) -> (w : NounWord) ->
              {auto 0 ok : countReach (Verbed v w Attributive) ManyOf bs = 1} ->
              {auto 0 mk : VerbedMarkingOk v Attributive} -> Noun bs (kindOfW w)
thoseVerbed v w = ThoseVerbed v w Attributive {ok} {mk}

public export
thoseVerbedThisWay : (v : VerbLabel) -> (w : NounWord) ->
                     {auto 0 ok : countReach (Verbed v w ThisWay) ManyOf bs = 1} ->
                     {auto 0 mk : VerbedMarkingOk v ThisWay} -> Noun bs (kindOfW w)
thoseVerbedThisWay v w = ThoseVerbed v w ThisWay {ok} {mk}

public export
additionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                 (count : Amount bs) ->
                 {auto 0 ad : AddedPart part} ->
                 {auto 0 an : AddedPartWritten anchor} -> Effect bs
additionalPart part anchor count = AdditionalPart part anchor count Nothing {ad} {an}

public export
additionalPartThen : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) -> (next : TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten (Just next)} -> Effect bs
additionalPartThen part anchor count next =
  AdditionalPart part anchor count (Just next) {ad} {an} {fb}

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
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 StaticEffect bs
entersChoosing n q = EntersChoice n (QSort q) Nothing Openly {zn}

public export
entersChoosingFrom : (n : Noun bs Object) -> (q : QualitySort) ->
                     (d : ChoiceDomain (QSort q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
entersChoosingFrom n q d = EntersChoice n (QSort q) (Just d) Openly {zn}

public export
entersChoosingPlayer : (n : Noun bs Object) ->
                       (d : Maybe (ChoiceDomain PlayerC)) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       StaticEffect bs
entersChoosingPlayer n d = EntersChoice n PlayerC d Openly {zn}

public export
entersChoosingPlayerSecretly : (n : Noun bs Object) ->
                               (d : Maybe (ChoiceDomain PlayerC)) ->
                               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                               StaticEffect bs
entersChoosingPlayerSecretly n d = EntersChoice n PlayerC d Secretly {zn}

public export
attachChoosing : (n : Noun bs Object) -> (q : QualitySort) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
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
eventCount ev who w = EventCount ev who w Nothing {cw}

public export
eventCountInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (who : Noun bs k) -> (w : Lookback) ->
                      (what : Noun (nomIntro who) kc) ->
                      {auto 0 cp : LookbackComplement ev k kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountInvolving ev who w what =
  EventCount ev who w (Just (Involving what {cp})) {cw} {sb}

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
  EventCount ev who w
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
happenedToAt : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
               (z : ZoneExpr bs) ->
               {auto 0 zo : So (lookbackLocusOk ev (zoneSort z))} ->
               {auto 0 cw : ComplementWritten
                              (Just (AtZone {bs} {ev} {ks = k} z {ok = zo}))} ->
               {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
happenedToAt ev w z = HappenedTo ev w (Just (AtZone z {ok = zo})) {cw} {sb}

public export
beginningOfPossessed : (part : TurnPart) -> (poss : Noun bs Player) ->
                       {auto 0 pn : PossessorNoun poss} ->
                       {auto 0 pu : PartTriggerable part (ByNoun poss {pn})} ->
                       GameEvent bs
beginningOfPossessed part poss = BeginningOf part (ByNoun poss {pn}) {pu}

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
renown : {bs : Bindings} -> (amt : Amount bs) -> Effect bs
renown amt =
  Sequentially [ PutCounters amt (PrintedKind plusOnePlusOne) thisCreature
               , GainsDesignation thisCreature Renowned
                                  (InExpansionOf RenownW) Nothing ]

public export
getsEnduringStory : Effect bs
getsEnduringStory =
  GainsDesignation You EnduringStory (InExpansionOf StoriedW) (Just RestOfGame)

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
onlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyWhile se c = OnlyWhile se c AsLongAs

public export
onlyUnless : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyUnless se c = OnlyWhile se (NotCond c) Unless

public export
onlyIfSo : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) -> StaticEffect bs
onlyIfSo se c = OnlyWhile se c IfSo

public export
mayCastFromWhileSearching : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                            (from : ZoneExpr (nomIntro what)) ->
                            {auto 0 zn : ZoneFits (nounZone who) (deedsZone ["Cast"] Agent)} ->
                            {auto 0 dp : DeedParticipant ["Cast"] Agent Player (nounHeadTys who)} ->
                            {auto 0 pt : So (deonticPatientOk who ["Cast"] Agent
                                                              (DeonticCounterpart what)
                                                              (PlayRider (Just from) Nothing (Just WhileSearchingLibrary) False ItsOwnCost))} ->
                            {auto 0 rd : So (deonticRiderOk ["Cast"] Agent
                                                            (Permit {bs = selfSubjIntro who})
                                                            (DeonticCounterpart what) False
                                                            (PlayRider (Just from) Nothing
                                                                       (Just WhileSearchingLibrary)
                                                                       False ItsOwnCost))} ->
                            StaticEffect bs
mayCastFromWhileSearching who what from =
  mayPlayDeed "Cast" who what
              (PlayRider (Just from) Nothing (Just WhileSearchingLibrary) False ItsOwnCost)
              {zn} {dp} {pt} {rd}

public export
fromTo : Nat -> Nat -> Quantity bs
fromTo lo hi = Range (Just lo) (Just hi)

public export
flipACoin : Effect bs
flipACoin = FlipCoins You (FlipCount (Lit 1))

public export
flipCoins : (n : Nat) -> Effect bs
flipCoins n = FlipCoins You (FlipCount (Lit n))

public export
youWinTheFlip : {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
youWinTheFlip = FlipCalled You WinsFlip {fl}

public export
youLoseTheFlip : {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
youLoseTheFlip = FlipCalled You LosesFlip {fl}

public export
comesUp : (face : CoinFace) -> {auto 0 fl : So (coinFlipInScope bs)} ->
          Condition bs
comesUp face = FlipFace face {fl}

public export
rollADie : (sides : Nat) -> {auto 0 nz : IsSucc sides} -> Effect bs
rollADie sides = RollDice You (Lit 1) (SidesOf sides {nz})

public export
rollDice : (count : Nat) -> (sides : Nat) -> {auto 0 nz : IsSucc sides} ->
           Effect bs
rollDice count sides = RollDice You (Lit count) (SidesOf sides {nz})

public export
theResult : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theResult = TheResult {ok}

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
youWinACoinFlip : GameEvent bs
youWinACoinFlip = FlipEvent You WinsFlip

public export
youLoseACoinFlip : GameEvent bs
youLoseACoinFlip = FlipEvent You LosesFlip

public export
youRollDice : GameEvent bs
youRollDice = RollsDice You ManyDice AnyDie AnyResult

public export
youRollADie : GameEvent bs
youRollADie = RollsDice You OneDie AnyDie AnyResult

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
youFlipACoin : GameEvent bs
youFlipACoin = FlipsCoin You

public export
rollThePlanarDie : Effect bs
rollThePlanarDie = RollPlanarDie You (Lit 1)

public export
flipACoinFor : {k : Kind} -> (each : Noun bs k) ->
               {auto 0 pl : nounPlur each = ManyOf} ->
               {auto 0 rk : So (kindLte k (Object \/ Player))} -> Effect bs
flipACoinFor each = FlipCoins You (FlipPer each {pl} {rk})

public export
orHigher : Nat -> Quantity bs
orHigher lo = Range (Just lo) Nothing

public export
theTotal : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
theTotal = TheTotal {ok}

public export
coinsThatCameUp : (face : CoinFace) ->
                  {auto 0 fl : So (coinFlipInScope bs)} -> Amount bs
coinsThatCameUp face = CoinsShowing face {fl}
