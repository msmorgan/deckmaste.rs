||| The macro layer: one definition per English phrase shape, mirroring
||| Experimental's Macros-over-Semantics split.
module Experimental.Macros

import public Experimental

%default total


public export
exactly : Nat -> Quantity
exactly n = Range (Just n) (Just n)

public export
upTo : Nat -> Quantity
upTo n = Range Nothing (Just n)

public export
anyNumber : Quantity
anyNumber = Range Nothing Nothing

public export
atLeast : Nat -> Quantity
atLeast n = Range (Just n) Nothing

public export
oneOrBoth : Quantity
oneOrBoth = Range (Just 1) (Just 2)

public export
oneThrough : Nat -> Quantity
oneThrough n = Range (Just 1) (Just n)

public export
target : (p : Predicate bs k) -> {auto tk : Targetable k} -> Noun bs k
target p = TargetGroup (exactly 1) p {tk}

||| "any target" [CR#115.4]: the class word, written out as the rule's own
||| list — a creature, a player, a planeswalker or a battle. A function, not
||| a row: the marking is spelling, and the expansion is what the rule says.
||| -- spelling: "any target" — the whole target phrase in two words, never
||| the four nouns coordinated.
public export
anyTarget : Predicate bs (Object \/ Player)
anyTarget = Joined (Or [HasType Creature, HasType Planeswalker, HasType Battle])
                   AnyPlayer

||| "target creature or player", "target permanent or player", "target
||| player or planeswalker": the cross-kind head, over any two descriptions.
||| [CR#115.1] is the rule that admits it — a spell's targets are objects
||| and/or players — not [CR#115.4], which covers the class word written
||| INSTEAD of "target [something]". The arguments are player-then-object, as the retired row
||| wrote them; the printed order of the two halves is the spelling layer's.
||| -- spelling: the two halves joined by "or", in whichever order the card
||| prints them.
public export
kindJoin : (who : Predicate bs Player) -> (what : Predicate bs Object) ->
           Predicate bs (Object \/ Player)
kindJoin who what = Joined what who

||| "you and permanents you control" [CR#109.5]: the mixed group. It binds
||| nothing jointly — see `Both` — so no clause reads the pair back.
||| -- spelling: "you and [phrase]", the player half first.
public export
youAnd : (n : Noun bs Object) -> Noun bs (Player \/ Object)
youAnd n = Both You n

||| "that creature or player", "that permanent or player": the demonstrative
||| that reads a joined mention back. The word is `JoinW`, whose kind is now
||| the join, so the read lands at the antecedent's own kind.
||| -- spelling: "that " then the two halves as the antecedent spelled them.
public export
thatJoin : {auto 0 ok : countWord JoinW bs = 1} -> Noun bs (Object \/ Player)
thatJoin = That JoinW {ok}


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
spell : Predicate bs Object
spell = InZone stackZ

public export
cardOf : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
         (line : TypeLine) -> (text : AbilitySeq []) ->
         (stats : Maybe (PrintedStat, PrintedStat)) ->
         {auto 0 ln : CardLine line} ->
         {auto 0 sp : CardSupers supers} ->
         {auto 0 tx : CardText line text} ->
         {auto 0 ch : CardChapters line text} ->
         {auto 0 pts : CardPt line text stats} ->
         {auto 0 mc : CardCost line cost} ->
         Card
cardOf name cost supers line text stats = MkCard name cost supers line text stats

public export
printedBox : Maybe (Integer, Integer) -> Maybe (PrintedStat, PrintedStat)
printedBox Nothing = Nothing
printedBox (Just (p, t)) = Just (PrintedNum p, PrintedNum t)

public export
card : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
       (line : TypeLine) -> (text : AbilitySeq []) ->
       (stats : Maybe (Integer, Integer)) ->
       {auto 0 ln : CardLine line} ->
       {auto 0 sp : CardSupers supers} ->
       {auto 0 tx : CardText line text} ->
       {auto 0 ch : CardChapters line text} ->
       {auto 0 pts : CardPt line text (printedBox stats)} ->
       {auto 0 mc : CardCost line cost} ->
       Card
card name cost supers line text stats =
  cardOf name cost supers line text (printedBox stats) {ln} {sp} {tx} {ch} {pts} {mc}

public export
counterSpell : (n : Noun bs Object) ->
               {auto 0 zn : OnStack (nounZone n)} -> Effect bs
counterSpell n = CounterSpell n {zn}

public export
graveyardZ : ZoneExpr bs
graveyardZ = ZoneAt Graveyard Bare

public export
handOf : (n : Noun bs Player) -> ZoneExpr bs
handOf n = ZoneAt Hand (OwnedBy n {ps = HandIsOwned})

public export
graveyardOf : (n : Noun bs Player) -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (OwnedBy n {ps = GraveyardIsOwned})


public export
thisCreature : Noun bs Object
thisCreature = AsType Creature This

public export
thisArtifact : Noun bs Object
thisArtifact = AsType Artifact This

public export
thisEnchantment : Noun bs Object
thisEnchantment = AsType Enchantment This

public export
thisLand : Noun bs Object
thisLand = AsType Land This


public export
thisAura : Noun bs Object
thisAura = AsType Enchantment This {sub = Just Aura}

public export
thisEquipment : Noun bs Object
thisEquipment = AsType Artifact This {sub = Just Equipment}

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
aAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            Noun bs k
aAtRandom p = Indefinite AtRandom p {ph}

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
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
          Effect bs
destroy n = Composite Destroy (Move n graveyardZ) {ok = DestroyB {z = ok}}

public export
exile : (n : Noun bs Object) -> Effect bs
exile n = Composite Exile (Move n exileZ) {ok = ExileB}

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) -> Effect bs
exileWithCounters n amt kind =
  Composite Exile
            (Move n exileZ
                  {riders = MkMoveRiders [] Nothing
                                         {counters = Just (MkCounterRider amt kind)}})
            {ok = ExileWithCountersB}

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ {pl}
       {riders = MkMoveRiders [] (Just who) {one = OneController {one}}
                              {counters = Just (MkCounterRider amt kind)}}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ {pl}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ {pl} {riders = MkMoveRiders [EntersTapped] Nothing}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ {pl} {riders = MkMoveRiders [EntersTapped, EntersAttacking] Nothing}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ {pl} {riders = MkMoveRiders [] (Just You)}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
sacrifice agent n =
  Does agent Sacrifice (Move n graveyardZ) {tb = SacrificeB {z = ok}}

public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n =
  Does agent Discard (Move n graveyardZ) {tb = DiscardB {d = dk}}

public export
discardsACard : (agent : Noun bs Player) -> Effect bs
-- sort-only: an owned-hand expansion needs a subject-read noun the
-- vocabulary doesn't have yet.
discardsACard agent = discards agent (a (InZone handZ))


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
asLongAs : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
asLongAs c se = Conditionally c se {nn}

public export
unlessSo : (c : Condition bs) -> (se : StaticEffect bs) ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
unlessSo c se = Conditionally (NotCond c) se {marking = Unless} {nn}

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

public export
entersWithCounters : (n : Noun bs Object) -> (k : Nat) -> (kind : CounterKind) ->
                     StaticEffect bs
entersWithCounters n k kind = EntersWithCounters n (Lit k) kind

public export
gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
       (tou : PtShift (nomIntro n)) ->
       (d : Maybe (Duration (selfSubjIntro n))) ->
       {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou) d

public export
gains : (n : Noun bs Object) -> (a : AbilityAt bs) ->
        (d : Maybe (Duration (selfSubjIntro n))) ->
        {auto 0 ok : GrantSubject a n} ->
        {auto 0 gr : Grantable a} ->
        {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gains n a d = Continuously (Gains n a) d

public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 ok : GrantSubject (KeywordAbility Haste) n} ->
             {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gainsHaste n d = gains n (KeywordAbility Haste) d

public export
plusOnePlusOne : CounterKind
plusOnePlusOne = BoostCounter (Up 1) (Up 1)

public export
minusOneMinusOne : CounterKind
minusOneMinusOne = BoostCounter (Down 1) (Down 1)

public export
flyingCounter : CounterKind
flyingCounter = KeywordCounter Flying


public export
cantAttack : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 dp : DeedParticipant Attack Agent (nounTy n)} ->
             {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttack n span = Continuously (Deontic n Forbid Attack Agent NoDeonticPatient {zn} {dp}) span {sp}

public export
cantBlock : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
            {auto 0 dp : DeedParticipant Block Agent (nounTy n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span = Continuously (Deontic n Forbid Block Agent NoDeonticPatient {zn} {dp}) span {sp}

public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 dp : DeedParticipant Block Patient (nounTy n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span = Continuously (Deontic n Forbid Block Patient NoDeonticPatient {zn} {dp}) span {sp}


public export
gainControl : (n : Noun bs Object) -> (d : Maybe (Duration (selfSubjIntro n))) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainControl n d = Continuously (GainsControl You n {zn}) d {sp}

public export
gainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
               (d : Maybe (Duration (selfSubjIntro what))) ->
               {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} ->
               {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainsControl who what d = Continuously (GainsControl who what {zn}) d {sp}

public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)


public export
may : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
may d body = May (Just d) body Nothing Nothing

public export
mayThen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (effIntro body) -> Effect bs
mayThen d body did = May (Just d) body (Just did) Nothing

public export
mayElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (nomIntro decider) -> Effect bs
mayElse d body notd = May (Just d) body Nothing (Just notd)

public export
-- the did-branch reads everything the body introduced (it only runs if
-- the body did); the didn't-branch reads only what preceded the offer,
-- since the body never ran.
mayThenElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
              Effect (effIntro body) -> Effect (nomIntro decider) -> Effect bs
mayThenElse d body did notd = May (Just d) body (Just did) (Just notd)


public export
creatureTokOf : (pow : Amount bs) -> (tou : Amount bs) -> List Color -> List Subtype ->
                TokenChars bs
creatureTokOf pow tou cs ss = MkToken (Just (pow, tou)) cs (MkTypeLine ss [Creature]) [] Nothing

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
         Effect bs
create count tok = Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc}) []

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} ->
                        {auto 0 ta : TokenAbilities tok} ->
                        {auto 0 tc : TokenCanonical tok} ->
                        Effect bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc})
         [EntersTapped, EntersAttacking]

public export
becomesAs : (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 ne : LineNonEmpty added.line} ->
            {auto 0 nw : AddsSomething (nounTy n) added.line} ->
            {auto 0 af : AddedFits (nounTy n) added.line} ->
            {auto 0 cd : ColorsDistinct added.colors} ->
            {auto 0 ta : TokenAbilities added} ->
            {auto 0 un : AdditionUnnamed added} ->
            {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomesAs n added d =
  Continuously (BecomesAlso n added {ne} {nw} {af} {cd} {ta} {un}) d {sp}

public export
becomes : (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 ne : LineNonEmpty added} ->
          {auto 0 nw : AddsSomething (nounTy n) added} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = becomesAs n (MkToken Nothing [] added [] Nothing) d {ne} {nw} {af} {sp}

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
            {auto 0 mf : ModesFit (exactly 1) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly 2) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.oneOrBoth (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrBoth modes = Modal Macros.oneOrBoth modes {tw} {mf} {dm = eqToSo dm}

public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast 1) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf} {dm = eqToSo dm}

public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.anyNumber (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {wf = Oh} {tw} {mf} {dm = eqToSo dm}

public export
notSo : (c : Condition bs) -> Condition bs
notSo c = NotCond c

public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
       Condition bs
itsA p = Matches (It {ok}) p {sy} {zc}

public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc})


public export
libraryOf : (n : Noun bs Player) -> ZoneExpr bs
libraryOf n = ZoneAt Library (OwnedBy n {ps = LibraryIsOwned})

public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt (OneEnd OnTop) Nothing Bare

public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt (OneEnd OnBottom) Nothing Bare

public export
onTopIn : (a : Arrangement) ->
          {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnTop) (Just a)} ->
          ZoneExpr bs
onTopIn a = LibraryAt (OneEnd OnTop) (Just a) {af} Bare

public export
onBottomIn : (a : Arrangement) ->
             {auto 0 af : PlaceArrangementFits (OneEnd {bs} OnBottom) (Just a)} ->
             ZoneExpr bs
onBottomIn a = LibraryAt (OneEnd OnBottom) (Just a) {af} Bare

public export
nthFromTop : (n : LibOrdinal) -> ZoneExpr bs
nthFromTop n = LibraryAt (OneEnd OnTop) Nothing {off = Just n} Bare

||| "on the top or bottom of <a> library": the bare position disjunction,
||| no chooser named (Write into Being).
public export
topOrBottomZ : ZoneExpr bs
topOrBottomZ = LibraryAt (EitherEnd Nothing) Nothing Bare

||| "on <player>'s choice of the top or bottom of <a> library": the
||| separable chooser slot over the same disjunction.
public export
choiceOfTopOrBottom : (chooser : Noun bs Player) ->
                      {auto 0 ag : EventAgent (Just chooser)} -> ZoneExpr bs
choiceOfTopOrBottom chooser = LibraryAt (EitherEnd (Just chooser) {ag}) Nothing Bare

||| "into <a> library Nth from the top or on the bottom": the offset
||| spelling, whose ordinal rides the top alternative [CR#401.7].
public export
nthFromTopOrBottomZ : (n : LibOrdinal) -> ZoneExpr bs
nthFromTopOrBottomZ n = LibraryAt (EitherEnd Nothing) Nothing {off = Just n} Bare

public export
topCards : (n : Nat) -> Noun bs Object
topCards n = LibrarySlice OnTop (Lit n) You

public export
topCard : Noun bs Object
topCard = topCards 1

public export
bottomCard : Noun bs Object
bottomCard = LibrarySlice OnBottom (Lit 1) You

public export
oneOf : (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} -> Noun bs Object
oneOf grp = SomeOf (exactly 1) grp {gm}

public export
someOf : (n : Nat) -> (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} ->
         {auto 0 nz : NonZeroQ (exactly n)} ->
         {auto 0 wf : WellFormedQ (exactly n)} -> Noun bs Object
someOf n grp = SomeOf (exactly n) grp {gm} {nz} {wf}


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
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countOnes Player (nomIntro who) = 1} -> Effect bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))



public export
searchLibraryFor : (p : Predicate bs Object) ->
                   {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryFor p = Search You (OneZone yourLibrary) p {zf}

||| "Search <player>'s graveyard, hand, and library for …": the three-zone
||| sweep, possessor-anchored [CR#701.23a].
public export
searchZonesOf : (whose : Noun bs Player) -> (p : Predicate bs Object) ->
                {auto 0 zf : ZoneFree p} -> Effect bs
searchZonesOf whose p = Search You (GraveyardHandLibraryOf whose) p {zf}

||| "<player> puts <it> into/onto <zone>": the agentive placement clause.
public export
puts : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
       (to : ZoneExpr (nomIntro n)) ->
       {auto 0 ok : DestOk to} ->
       {auto 0 arr : ArrangementOk (nounPlur n) to} ->
       {auto 0 pl : Placeable (nounTy n) (zoneSort to)} ->
       Effect bs
puts agent n to = Does agent Put (Move n to {ok} {arr} {pl}) {tb = PutB}

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
ifWouldInstead ev repl d = Continuously (Intercepts ev repl Repeatedly {ok}) d {sp}

public export
nextTimeWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 sp : SpanOk Replacement d} -> Effect bs
nextTimeWouldInstead ev repl d =
  Continuously (Intercepts ev repl NextTimeOnly {ok}) d {sp}

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

||| "<permanent> phases out until [event]": [CR#610.4]'s rider, the
||| second one-shot the CR hangs "until" on beside [CR#610.3]'s zone
||| change.
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
doThen : (body : Effect bs) -> Effect (effIntro body) -> Effect bs
doThen body did = May Nothing body (Just did) Nothing

public export
doElse : (body : Effect bs) -> Effect bs -> Effect bs
doElse body notd = May Nothing body Nothing (Just notd)

public export
-- a reflexive trigger waits on the stack rather than continuing in the
-- same resolution as mayThen's arm [CR#603.3], so its bindings are
-- settled here first.
mayWhen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (settleTargets (effIntro body)) ->
          {auto 0 ok : So (admitsReflexEnclosure (reflexEncloseUse body))} ->
          Effect bs
mayWhen d body trig =
  Reflexively (May (Just d) body Nothing Nothing) trig {en = ok}


-- Wrapping macros for the optional and proof-carrying slots, so a card
-- never binds an implicit: every slot below is reached by a positional
-- argument here instead.

||| "Target player mills N cards."
public export
mills : (agent : Noun bs Player) -> (amt : Amount (nomIntro agent)) ->
        (whose : Noun (nomIntro agent) Player) ->
        {auto 0 sp : SlicePossessor whose} ->
        Effect bs
mills agent amt whose =
  Does agent Mill
       (Move (LibrarySlice OnTop amt whose {sp}) graveyardZ)
       {tb = MillB {sp}}

||| "Target player scries N." / "Target player surveils N."
||| [CR#701.22a] and [CR#701.25a] name one player and read that player's
||| own library, so the subject is written once and the slice reads it
||| back as the anaphor.
public export
playerScries : (agent : Noun bs Player) -> (amt : Amount (nomIntro agent)) ->
               {auto 0 an : countOnes Player (nomIntro agent) = 1} ->
               Effect bs
playerScries agent amt =
  Does agent Scry
       (Expose LookAt (They {ok = an})
               (ExposedCards (LibrarySlice OnTop amt (They {ok = an}))))

public export
playerSurveils : (agent : Noun bs Player) -> (amt : Amount (nomIntro agent)) ->
                 {auto 0 an : countOnes Player (nomIntro agent) = 1} ->
                 Effect bs
playerSurveils agent amt =
  Does agent Surveil
       (Expose LookAt (They {ok = an})
               (ExposedCards (LibrarySlice OnTop amt (They {ok = an}))))

||| "<player> loses N <kind> counters": the counted removal beside the
||| bare "all" spelling `LosesCounters` writes with the slot unfilled.
public export
losesCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                (kind : Maybe CounterKind) ->
                {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesCounters who amt kind = LosesCounters who kind {amt = Just amt} {pk}

||| "<player> loses all <kind> counters": the bare removal, which the
||| unwritten amount slot spells.
public export
losesAllCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                   {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
losesAllCounters who kind = LosesCounters who kind {pk}

||| "Enchant creature": a keyword whose parameter is a subject phrase.
public export
keywordSubject : {0 bs : Bindings} -> {k : Kind} -> (kw : Keyword) ->
                 (p : Predicate [] k) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamSubject {bs} p))} ->
                 AbilityAt bs
keywordSubject kw p = KeywordAbility kw {param = Just (ParamSubject p)} {pf}

||| "Equip {2}", "Ward {2}": a keyword whose parameter is a cost.
public export
keywordCosting : {0 bs : Bindings} -> (kw : Keyword) -> (c : Cost []) ->
                 {auto 0 pf : KeywordParamFits {bs} kw
                                (Just (ParamCost {bs} c))} ->
                 AbilityAt bs
keywordCosting kw c = KeywordAbility kw {param = Just (ParamCost c)} {pf}

||| "Protection from red": a keyword whose parameter is a quality.
public export
keywordQuality : (kw : Keyword) -> (q : Predicate bs Object) ->
                 {auto 0 pf : KeywordParamFits kw (Just (ParamQuality q))} ->
                 AbilityAt bs
keywordQuality kw q = KeywordAbility kw {param = Just (ParamQuality q)} {pf}

||| "Renown 1": a keyword whose parameter is a written number.
public export
keywordNumber : {0 bs : Bindings} -> (kw : Keyword) -> (amt : Amount []) ->
                {auto 0 pf : KeywordParamFits {bs} kw
                               (Just (ParamNumber {bs} amt))} ->
                AbilityAt bs
keywordNumber kw amt = KeywordAbility kw {param = Just (ParamNumber amt)} {pf}

||| "Whenever …, if <condition>, …": a trigger with an intervening-if clause.
public export
triggeredIf : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (cond : Condition (headerCtx Nothing ev)) ->
              (eff : Effect (interveningIntro (Just cond))) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 ae : AltEvent word (the (Maybe (GameEvent bs)) Nothing)} ->
              {auto 0 cd : ChapterDefaults ev Nothing Nothing Nothing (Just cond)} ->
              AbilityAt bs
triggeredIf word ev cond eff =
  Triggered word ev {intervening = Just cond} eff {hn} {ae} {cd}

||| "Whenever X or Y, …": a trigger with an alternative event.
public export
triggeredOr : {bs : Bindings} -> (word : TriggerWord) -> (ev : GameEvent bs) ->
              (alt : GameEvent bs) -> (eff : Effect bs) ->
              {auto 0 hn : HeaderNontarget ev} ->
              {auto 0 ae : AltEvent word (Just alt)} ->
              {auto 0 cd : ChapterDefaults ev (Just alt) Nothing Nothing Nothing} ->
              AbilityAt bs
triggeredOr word ev alt eff =
  Triggered word ev {alt = Just alt} eff {hn} {ae} {cd}

||| "Whenever …, during <window>, …": a trigger confined to a window.
public export
triggeredOnlyDuring : {bs : Bindings} -> (word : TriggerWord) ->
                      (ev : GameEvent bs) -> (w : TriggerWindow) ->
                      (eff : Effect (eventAfter ev)) ->
                      {auto 0 hn : HeaderNontarget ev} ->
                      {auto 0 ae : AltEvent word (the (Maybe (GameEvent bs)) Nothing)} ->
                      {auto 0 cd : ChapterDefaults ev Nothing (Just w) Nothing Nothing} ->
                      AbilityAt bs
triggeredOnlyDuring word ev w eff =
  Triggered word ev {window = Just w} eff {hn} {ae} {cd}

||| "Whenever …, … . This triggers only once each turn."
public export
triggeredOnlyOnce : {bs : Bindings} -> (word : TriggerWord) ->
                    (ev : GameEvent bs) -> (lim : UsageLimit) ->
                    (eff : Effect (eventAfter ev)) ->
                    {auto 0 hn : HeaderNontarget ev} ->
                    {auto 0 ae : AltEvent word (the (Maybe (GameEvent bs)) Nothing)} ->
                    {auto 0 cd : ChapterDefaults ev Nothing Nothing (Just lim) Nothing} ->
                    AbilityAt bs
triggeredOnlyOnce word ev lim eff =
  Triggered word ev {limit = Just lim} eff {hn} {ae} {cd}

||| "Activate only as a sorcery" / "only during your upkeep".
public export
activatedOnlyDuring : (cost : Cost bs) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (w : Timing) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyDuring cost eff w = Activated cost eff {window = Just w} {tp} {py}

||| "Activate only once each turn" (or once each game).
public export
activatedOnlyOnce : (cost : Cost bs) ->
                    (eff : Effect (publicOnly (costIntro cost))) ->
                    (lim : UsageLimit) ->
                    {auto 0 tp : CostTapOnce cost} ->
                    {auto 0 py : CostPaidByYou cost} ->
                    AbilityAt bs
activatedOnlyOnce cost eff lim = Activated cost eff {limit = Just lim} {tp} {py}

||| "Activate only if <condition>."
public export
activatedOnlyIf : (cost : Cost bs) ->
                  (eff : Effect (publicOnly (costIntro cost))) ->
                  (g : Condition bs) ->
                  {auto 0 tp : CostTapOnce cost} ->
                  {auto 0 py : CostPaidByYou cost} ->
                  AbilityAt bs
activatedOnlyIf cost eff g = Activated cost eff {guard = Just g} {tp} {py}

||| "Activate only once each turn and only if <condition>."
public export
activatedOnlyOnceIf : (cost : Cost bs) ->
                      (eff : Effect (publicOnly (costIntro cost))) ->
                      (lim : UsageLimit) -> (g : Condition bs) ->
                      {auto 0 tp : CostTapOnce cost} ->
                      {auto 0 py : CostPaidByYou cost} ->
                      AbilityAt bs
activatedOnlyOnceIf cost eff lim g =
  Activated cost eff {limit = Just lim} {guard = Just g} {tp} {py}

||| "You may cast <what> from <zone>."
public export
mayCastFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 pz : PlaySource (nounZone what) (Just from) Nothing} ->
              {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFrom who what from =
  MayPlay who what {verb = Cast} {from = Just from} {pz} {cv}

||| "You may play <what> from <zone>."
public export
mayPlayFrom : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              (from : ZoneExpr (nomIntro what)) ->
              {auto 0 pz : PlaySource (nounZone what) (Just from) Nothing} ->
              {auto 0 cv : CastableTy Play (nounTy what)} -> StaticEffect bs
mayPlayFrom who what from =
  MayPlay who what {verb = Play} {from = Just from} {pz} {cv}

||| "You may cast <what> from <zone>", under a play limit.
public export
mayCastFromLimited : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     (from : ZoneExpr (nomIntro what)) -> (lim : PlayLimit) ->
                     {auto 0 pz : PlaySource (nounZone what) (Just from) Nothing} ->
                     {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFromLimited who what from lim =
  MayPlay who what {verb = Cast} {from = Just from} {limit = Just lim} {pz} {cv}

||| "You may cast <what> as though it had flash."
public export
mayCastAsThough : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                  (asThough : PlayAsThough) ->
                  {auto 0 pz : PlaySource (nounZone what)
                                          (the (Maybe (ZoneExpr (nomIntro what))) Nothing)
                                          (Just asThough)} ->
                  {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastAsThough who what asThough =
  MayPlay who what {verb = Cast} {asThough = Just asThough} {pz} {cv}

||| "… is put into <zone> from <source>."
public export
putIntoFrom : (n : Noun bs Object) -> (to : ZoneExpr bs) -> (src : EventSource bs) ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource (Just src)} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone (Just src))} -> GameEvent bs
putIntoFrom n to src = PutInto n to {from = Just src} {dk} {sk} {zn}

||| "… enters with an additional counter on it."
public export
entersWithAdditionalCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                               (kind : CounterKind) -> StaticEffect bs
entersWithAdditionalCounters n amt kind =
  EntersWithCounters n amt kind {mark = Additional}

||| "Whenever <creature> attacks <player>."
public export
attacksPlayer : (n : Noun bs Object) -> (whom : Noun (nomIntro n) Player) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 df : AttackDefender (Just whom)} -> GameEvent bs
attacksPlayer n whom = Attacks n {whom = Just whom} {zn} {df}

||| "Whenever one or more tokens are created under <player>'s control."
public export
tokensCreatedUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                     {auto 0 tk : TokenPhrase n} ->
                     {auto 0 vo : CreationVoice Nothing Nothing (Just under)} ->
                     GameEvent bs
tokensCreatedUnder n under = TokensCreated n {under = Just under} {tk} {vo}

||| "Whenever an effect creates tokens under <player>'s control."
public export
tokensCreatedByEffectUnder : (n : Noun bs Object) -> (under : Noun bs Player) ->
                             {auto 0 tk : TokenPhrase n} ->
                             {auto 0 vo : CreationVoice (Just AnEffect) Nothing
                                                        (Just under)} ->
                             GameEvent bs
tokensCreatedByEffectUnder n under =
  TokensCreated n {cause = Just AnEffect} {under = Just under} {tk} {vo}

||| "Whenever a counter is put on …": the one-counter reading.
public export
singleCounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                     (n : Noun bs Object) ->
                     {auto 0 sc : counterScope kind = Object} -> GameEvent bs
singleCounterEvent dir kind n =
  CounterEvent dir kind n {many = OneCounter} {sc}

||| "When the last <kind> counter is removed from … by <player>."
public export
lastCounterRemovedBy : (kind : CounterKind) -> (n : Noun bs Object) ->
                       (who : Noun bs Player) ->
                       {auto 0 sc : counterScope kind = Object} ->
                       {auto 0 ag : EventAgent (Just who)} -> GameEvent bs
lastCounterRemovedBy kind n who =
  LastCounterRemoved kind n {by = Just who} {sc} {ag}

||| "<player> chooses …": a choice made by someone other than you.
public export
chooses : {k : Kind} -> (who : Noun bs Player) -> (n : Noun bs k) ->
          {auto 0 ch : ChoiceClause (Just who) n} -> Effect bs
chooses who n = Choose n {by = Just who} {ch}

||| "those cards destroyed this way": the marked plural anaphor.
public export
thoseVerbedThisWay : (v : VerbName) -> (w : NounWord) ->
                     {auto 0 ok : countManyVerbed v w bs = 1} ->
                     {auto 0 mk : VerbedMarkingOk v ThisWay} -> Noun bs (kindOfW w)
thoseVerbedThisWay v w = ThoseVerbed v w {marking = ThisWay} {ok} {mk}

||| "… followed by <part>": an added turn part with a successor.
public export
additionalPartThen : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) -> (next : TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AnchorPart anchor} ->
                     {auto 0 fb : FollowerPart (Just next)} -> Effect bs
additionalPartThen part anchor count next =
  AdditionalPart part anchor count {followedBy = Just next} {ad} {an} {fb}

||| "When <event> this turn, …": a delayed trigger with an explicit span.
public export
delayedWithin : (ev : GameEvent bs) -> (span : Duration bs) ->
                (eff : Effect (delayedCtx ev)) ->
                {auto 0 so : DelaySpanOk (Just span)} -> Effect bs
delayedWithin ev span eff = Delayed ev {span = Just span} eff {so}

||| "a creature type other than Wall": a quality noun with a choice domain.
public export
qualityFrom : (q : QualitySort) -> (d : ChoiceDomain q) -> Predicate bs (Quality q)
qualityFrom q d = QualityNoun q {dom = Just d}

||| "As … enters, choose a color other than red."
public export
entersChoosingFrom : (n : Noun bs Object) -> (q : QualitySort) ->
                     (d : ChoiceDomain q) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
entersChoosingFrom n q d = EntersChoice n q {dom = Just d} {zn}

||| "this Siege": the self-reference read at a subtype.
public export
thisSiege : Noun bs Object
thisSiege = AsType Battle This {sub = Just Siege}

||| "… that was dealt damage by <noun> this turn": the lookback complement.
public export
happenedToInvolving : {ks : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (w : Lookback) -> (what : Noun bs kc) ->
                      {auto 0 cp : LookbackComplement ev ks kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev ks} -> Predicate bs ks
happenedToInvolving ev w what =
  HappenedTo ev w {what = Just (Involving what {cp})} {cw} {sb}

||| "if you cast <noun> this turn": a lookback condition with a complement.
public export
happenedInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                    (who : Noun bs k) -> (w : Lookback) ->
                    (what : Noun (nomIntro who) kc) ->
                    {auto 0 cp : LookbackComplement ev k kc} ->
                    {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                    {auto 0 sb : LookbackSubject ev k} -> Condition bs
happenedInvolving ev who w what =
  Happened ev who w {what = Just (Involving what {cp})} {cw} {sb}

||| "the number of <noun> you cast this turn": a counted lookback.
public export
eventCountInvolving : {k : Kind} -> {kc : Kind} -> (ev : EventName) ->
                      (who : Noun bs k) -> (w : Lookback) ->
                      (what : Noun (nomIntro who) kc) ->
                      {auto 0 cp : LookbackComplement ev k kc} ->
                      {auto 0 cw : ComplementWritten (Just (Involving what {cp}))} ->
                      {auto 0 sb : LookbackSubject ev k} -> Amount bs
eventCountInvolving ev who w what =
  EventCount ev who w {what = Just (Involving what {cp})} {cw} {sb}

||| "At the beginning of enchanted player's upkeep, …": a turn part
||| possessed by a noun rather than by a quantifier word.
public export
beginningOfPossessed : (part : TurnPart) -> (poss : Noun bs Player) ->
                       {auto 0 pn : PossessorNoun poss} ->
                       {auto 0 pu : PartTriggerable part (ByNoun poss {pn})} ->
                       GameEvent bs
beginningOfPossessed part poss = BeginningOf part (ByNoun poss {pn}) {pu}

||| "Monstrosity N" [CR#701.37a]: the keyword action spells its own
||| expansion body, and that body is the only place `Monstrous` is
||| conferred.
public export
monstrosity : {bs : Bindings} -> (amt : Amount bs) ->
              Effect bs
monstrosity amt =
  If (notSo (Matches thisCreature (HasDesignation Monstrous)))
     (Sequentially [ PutCounters amt plusOnePlusOne thisCreature
                   , GainsDesignation thisCreature Monstrous
                                      (InExpansionOf MonstrosityW) ])
     Nothing

||| Ascend's expansion body [CR#702.131a]: "you get the city's blessing
||| for the rest of the game."
public export
getsCitysBlessing : Effect bs
getsCitysBlessing =
  GainsDesignation You CitysBlessing (InExpansionOf AscendW)
                   {span = Just RestOfGame}

||| Saddle's expansion body [CR#702.171a]: "This permanent becomes
||| saddled until end of turn."
public export
becomesSaddled : Effect bs
becomesSaddled =
  GainsDesignation (AsType Artifact This) Saddled (InExpansionOf SaddleW)
                   {span = Just untilEndOfTurn}

||| "your commander" [CR#903.3]: the card-scope designation read as a
||| possessed noun.
public export
yourCommander : Noun bs Object
yourCommander = Designated CommanderD You

||| "there is no monarch" [CR#725.1]: the designation's absence check.
public export
thereIsNo : (d : Designation) ->
            {auto 0 sc : designationScope d = HeldBy Player} ->
            {auto 0 at : So (designationChecked d)} -> Condition bs
thereIsNo d = NoHolder d {sc} {at}

||| "If [c], [e]."
public export
ifThen : (c : Condition bs) -> Effect (condIntro c) -> Effect bs
ifThen c e = If c e Nothing

||| "If [c], [e]. Otherwise, [o]."
public export
ifThenElse : (c : Condition bs) -> (e : Effect (condIntro c)) ->
             Effect (otherwiseCtx e) -> Effect bs
ifThenElse c e o = If c e (Just o)

||| "[e] if [c]."
public export
onlyIf : (e : Effect bs) -> Condition (preIntro e) -> Effect bs
onlyIf e c = OnlyIf e c Nothing

||| "[e] unless [c]." — the conditional unless, beside the cost arm `Unless`.
public export
onlyIfNot : (e : Effect bs) -> Condition (preIntro e) -> Effect bs
onlyIfNot e c = OnlyIf e (NotCond c) Nothing

||| "[se] as long as [c]."
public export
onlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
            {auto 0 nn : NotConditional se} -> StaticEffect bs
onlyWhile se c = OnlyWhile se c {nn}

||| "[se] unless [c]."
public export
onlyUnless : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
             {auto 0 nn : NotConditional se} -> StaticEffect bs
onlyUnless se c = OnlyWhile se (NotCond c) {marking = Unless} {nn}

||| "[body], where [w] is [def]": the letter's definition written after the
||| clause it scopes over, as English postposes it. The letter is a name the
||| ability defines once [CR#107.3], so the core keeps the binder first and
||| this macro restores the English order.
public export
whereLetter : (w : LetterWord) -> (body : Effect (Experimental.Words.letterB w :: bs)) ->
              (def : Amount bs) -> Effect bs
whereLetter w body def = WhereLetter w def body

||| "[se], where [w] is [def]": the static twin.
public export
whereLetterStatic : (w : LetterWord) ->
                    (se : StaticEffect (Experimental.Words.letterB w :: bs)) ->
                    (def : Amount bs) -> StaticEffect bs
whereLetterStatic w se def = WhereLetterStatic w def se

||| "While you're searching your library, you may cast <what> from <zone>."
public export
mayCastFromWhileSearching : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                            (from : ZoneExpr (nomIntro what)) ->
                            {auto 0 pz : PlaySource (nounZone what) (Just from) Nothing} ->
                            {auto 0 cv : CastableTy Cast (nounTy what)} -> StaticEffect bs
mayCastFromWhileSearching who what from =
  MayPlay who what {verb = Cast} {from = Just from}
          {window = Just WhileSearchingLibrary} {pz} {cv}
