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
target : (p : Predicate bs k) -> {auto tk : Targetable k} ->
         {auto 0 hd : Headed p} ->
         {auto 0 af : AnyTargetAtCount (exactly 1) p} -> Noun bs k
target p = TargetGroup (exactly 1) p {tk} {hd} {af}


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
handOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
handOf n = ZoneAt Hand (OwnedBy n {ps = HandIsOwned} {pn})

public export
graveyardOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (OwnedBy n {ps = GraveyardIsOwned} {pn})


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
    {auto 0 hd : Headed p} ->
    {auto 0 af : AnyTargetFree p} -> Noun bs k
a p = Indefinite Unmarked p {ph} {hd} {af}

public export
aTheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
               {auto 0 ch : countChoosers bs = 1} ->
               {auto 0 hd : Headed p} ->
               {auto 0 af : AnyTargetFree p} -> Noun bs k
aTheirChoice p = Indefinite (TheirChoice {ch}) p {ph} {hd} {af}

public export
aYourChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
              {auto 0 hd : Headed p} ->
              {auto 0 af : AnyTargetFree p} -> Noun bs k
aYourChoice p = Indefinite YourChoice p {ph} {hd} {af}

public export
aAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            {auto 0 hd : Headed p} ->
            {auto 0 af : AnyTargetFree p} -> Noun bs k
aAtRandom p = Indefinite AtRandom p {ph} {hd} {af}

public export
anOpponent : Noun bs Player
anOpponent = a Opponent

public export
anyOtherTarget : {auto 0 ok : anyTargeted Object bs = True} -> Predicate bs Object
anyOtherTarget = And [AnyTarget, Other {ok = eqToSo ok}] {oa = eqToSo ok}


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
           {auto 0 hd : Headed p} -> {auto 0 nz : IsSucc n} ->
           {auto 0 af : AnyTargetFree p} -> Amount bs
nForEach n p = Times n (CountOf p {hd} {af}) {nz}

public export
forEach : {k : Kind} -> (p : Predicate bs k) ->
          {auto 0 hd : Headed p} ->
          {auto 0 af : AnyTargetFree p} -> Amount bs
forEach p = nForEach 1 p {hd} {af}

public export
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
          {auto 0 na : NotPlayerSpanning n} -> Effect bs
destroy n = Composite Destroy (Move n graveyardZ {na}) {ok = DestroyB {z = ok} {na}}

public export
exile : (n : Noun bs Object) -> {auto 0 na : NotPlayerSpanning n} -> Effect bs
exile n = Composite Exile (Move n exileZ {na}) {ok = ExileB {na}}

public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) ->
                    {auto 0 wc : WrittenCount amt} ->
                    {auto 0 na : NotPlayerSpanning n} -> Effect bs
exileWithCounters n amt kind =
  Composite Exile
            (Move n exileZ
                  {riders = MkMoveRiders [] Nothing
                                         {counters = Just (MkCounterRider amt kind {wc})}})
            {ok = ExileWithCountersB}

public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 wc : WrittenCount amt} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotPlayerSpanning n} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ {pl} {na}
       {riders = MkMoveRiders [] (Just who) {one = OneController {one}}
                              {counters = Just (MkCounterRider amt kind {wc})}}

public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     {auto 0 na : NotPlayerSpanning n} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ {pl} {na}

public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           {auto 0 na : NotPlayerSpanning n} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [EntersTapped] Nothing}

public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotPlayerSpanning n} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [EntersTapped, EntersAttacking] Nothing}

public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotPlayerSpanning n} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [] (Just You)}

public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} ->
            {auto 0 na : NotPlayerSpanning n} -> Effect bs
sacrifice agent n =
  Does agent Sacrifice (Move n graveyardZ {na}) {tb = SacrificeB {z = ok} {na}}

public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} ->
           {auto 0 na : NotPlayerSpanning n} -> Effect bs
discards agent n =
  Does agent Discard (Move n graveyardZ {na}) {tb = DiscardB {d = dk} {na}}

public export
discardsACard : (agent : Noun bs Player) -> Effect bs
-- sort-only: an owned-hand expansion needs a subject-read noun the
-- vocabulary doesn't have yet.
discardsACard agent = discards agent (a (InZone handZ))


public export
dealsDivided : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
               (among : Noun (amtIntro amt) k) ->
               {auto 0 ds : DamageSource src} ->
               {auto 0 wc : WrittenCount amt} ->
               {auto 0 pl : nounPlur among = ManyOf} ->
               {auto 0 gm : GroupMention among} ->
               {auto 0 rk : DamageRecipient among} -> Effect bs
dealsDivided src amt among =
  Distribute (DividedDamage src {ds}) amt among {wc} {pl} {gm}
             {tk = DamageDivided {rk}}

public export
distributeCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (among : Noun (amtIntro amt) Object) ->
                     {auto 0 wc : WrittenCount amt} ->
                     {auto 0 pl : nounPlur among = ManyOf} ->
                     {auto 0 gm : GroupMention among} ->
                     {auto 0 zn : OnBattlefield (nounZone among)} -> Effect bs
distributeCounters amt kind among =
  Distribute (DistributedCounters kind) amt among {wc} {pl} {gm}
             {tk = CountersDistributed {zn}}


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
           {auto 0 ng : CondNegatable c} ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
unlessSo c se = Conditionally (NotCond c) se {marking = Unless} {nn}

public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

public export
entersWithCounters : (n : Noun bs Object) -> (k : Nat) -> (kind : CounterKind) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 wc : WrittenCount {bs} (Lit k)} ->
                     StaticEffect bs
entersWithCounters n k kind = EntersWithCounters n (Lit k) kind {zn}

public export
gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
       (tou : PtShift (nomIntro n)) ->
       (d : Maybe (Duration (selfSubjIntro n))) ->
       {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
       {auto 0 ps : PumpSigns pow tou} ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou {ps}) d

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
cantAttack n span = Continuously (Deontic n Forbid Attack Agent Nothing {zn} {dp}) span {sp}

public export
cantBlock : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
            {auto 0 dp : DeedParticipant Block Agent (nounTy n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span = Continuously (Deontic n Forbid Block Agent Nothing {zn} {dp}) span {sp}

public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe (Duration (selfSubjIntro n))) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 dp : DeedParticipant Block Patient (nounTy n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span = Continuously (Deontic n Forbid Block Patient Nothing {zn} {dp}) span {sp}


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
         {auto 0 wc : WrittenCount count} -> Effect bs
create count tok = Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc}) [] {wc}
                          {rr = Oh}

public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} ->
                        {auto 0 ta : TokenAbilities tok} ->
                        {auto 0 tc : TokenCanonical tok} ->
                        {auto 0 wc : WrittenCount count} -> Effect bs
createTappedAttacking count tok =
  Create You count (TokenWritten tok {tt} {tp} {sf} {ta} {tc})
         [EntersTapped, EntersAttacking] {wc}
         {rr = Oh}

public export
becomesAs : (n : Noun bs Object) -> (added : TokenChars bs) ->
            (d : Maybe (Duration (selfSubjIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
            {auto 0 ne : LineNonEmpty added.line} ->
            {auto 0 nw : AddsSomething (nounTy n) added.line} ->
            {auto 0 af : AddedFits (nounTy n) added.line} ->
            {auto 0 cd : ColorsDistinct added.colors} ->
            {auto 0 ta : TokenAbilities added} ->
            {auto 0 un : AdditionUnnamed added} ->
            {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomesAs n added d =
  Continuously (BecomesAlso n added {zn} {ne} {nw} {af} {cd} {ta} {un}) d {sp}

public export
becomes : (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (selfSubjIntro n))) ->
          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
          {auto 0 ne : LineNonEmpty added} ->
          {auto 0 nw : AddsSomething (nounTy n) added} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = becomesAs n (MkToken Nothing [] added [] Nothing) d {zn} {ne} {nw} {af} {sp}

public export
basicLandLine : (ss : List Subtype) -> {auto 0 bl : BasicLandTypes ss} -> TypeLine
basicLandLine ss = MkTypeLine ss []


public export
drawACard : Effect bs
drawACard = Draw You (Lit 1)

public export
drawCards : (n : Nat) -> {auto 0 wc : WrittenCount {bs} (Lit n)} -> Effect bs
drawCards n = Draw You (Lit n) {wc}

public export
drawsACard : (who : Noun bs Player) -> Effect bs
drawsACard who = Draw who (Lit 1)


public export
chooseOne : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly 1) (modeCount modes)} ->
            {auto 0 mh : ModalHead (exactly 1) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf} {mh} {dm = eqToSo dm}

public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly 2) (modeCount modes)} ->
            {auto 0 mh : ModalHead (exactly 2) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf} {mh} {dm = eqToSo dm}

public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.oneOrBoth (modeCount modes)} ->
                  {auto 0 mh : ModalHead Macros.oneOrBoth (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrBoth modes = Modal Macros.oneOrBoth modes {tw} {mf} {mh} {dm = eqToSo dm}

public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast 1) (modeCount modes)} ->
                  {auto 0 mh : ModalHead (atLeast 1) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf} {mh} {dm = eqToSo dm}

public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.anyNumber (modeCount modes)} ->
                  {auto 0 mh : ModalHead Macros.anyNumber (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {wf = Oh} {tw} {mf} {mh} {dm = eqToSo dm}

public export
notSo : (c : Condition bs) -> {auto 0 ng : CondNegatable c} -> Condition bs
notSo c = NotCond c {ng}

public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
       {auto 0 af : AnyTargetFree p} -> Condition bs
itsA p = Matches (It {ok}) p {sy} {zc} {af}

public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
          {auto 0 af : AnyTargetFree p} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc} {af}) {ng = eqToSo nf}


public export
libraryOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
libraryOf n = ZoneAt Library (OwnedBy n {ps = LibraryIsOwned} {pn})

public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt OnTop Nothing Bare

public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt OnBottom Nothing Bare

public export
onTopIn : (a : Arrangement) -> {auto 0 af : ArrangementFits OnTop (Just a)} ->
          ZoneExpr bs
onTopIn a = LibraryAt OnTop (Just a) {af} Bare

public export
onBottomIn : (a : Arrangement) -> {auto 0 af : ArrangementFits OnBottom (Just a)} ->
             ZoneExpr bs
onBottomIn a = LibraryAt OnBottom (Just a) {af} Bare

public export
nthFromTop : (n : LibOrdinal) -> ZoneExpr bs
nthFromTop n = LibraryAt OnTop Nothing {off = Just n} Bare

public export
topCards : (n : Nat) -> {auto 0 wc : WrittenCount {bs} (Lit n)} -> Noun bs Object
topCards n = LibrarySlice OnTop (Lit n) You {wc}

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
lookAtHandOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> Effect bs
lookAtHandOf n = Expose LookAt You (ExposedZone (handOf n {pn}))

public export
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countOnes Player (nomIntro who) = 1} -> Effect bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))



public export
searchLibraryFor : (p : Predicate bs Object) ->
                   {auto 0 hd : Headed p} ->
                   {auto 0 af : AnyTargetFree p} ->
                   {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryFor p = Search You yourLibrary p {hd} {af} {zf}

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
                 {auto 0 uo : ReplUseOk ev Repeatedly} ->
                 {auto 0 sp : SpanOk Replacement d} -> Effect bs
ifWouldInstead ev repl d = Continuously (Intercepts ev repl Repeatedly {ok} {uo}) d {sp}

public export
nextTimeWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 uo : ReplUseOk ev NextTimeOnly} ->
                       {auto 0 sp : SpanOk Replacement d} -> Effect bs
nextTimeWouldInstead ev repl d =
  Continuously (Intercepts ev repl NextTimeOnly {ok} {uo}) d {sp}

public export
preventAll : (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAll kind scope d = Continuously (Prevents kind AllOfIt scope Nothing Nothing) d {sp}

public export
preventNext : (kind : DamageKind) -> (amt : Amount bs) ->
              (scope : DamageScope (amtIntro amt)) ->
              (d : Maybe (Duration (scopeIntro scope))) ->
              {auto 0 wc : WrittenCount amt} ->
              {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNext kind amt scope d =
  Continuously (Prevents kind (TheNext amt {wc}) scope Nothing Nothing) d {sp}

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
                {auto 0 wc : WrittenCount amt} ->
                {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNextBy kind amt scope src d =
  Continuously (Prevents kind (TheNext amt {wc}) scope (Just src) Nothing) d {sp}

public export
shieldingIt : {k : Kind} -> (n : Noun bs k) ->
              {auto 0 rk : DamageRecipient n} -> DamageScope bs
shieldingIt n = ToRecipient n {rk}

public export
exileUntil : (n : Noun bs Object) -> {auto 0 na : NotPlayerSpanning n} ->
             (ev : GameEvent (preIntro (exile n))) ->
             {auto 0 hd : Holdable ev} -> Effect bs
exileUntil n ev = HeldUntil (exile n) ev {ok = Oh} {hd}

public export
insteadOf : (replaced : Effect bs) -> (repl : Effect (annIntro replaced)) ->
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
