module Experimental.Triggers

import public Experimental.Phrase

%default total

||| The mana type a "tapped for mana of …" trigger specifies [CR#106.12a]:
||| colorless, or a color written or chosen — the six types of [CR#106.1b].
public export
data ManaTypeTerm : Bindings -> Type where
  ColorlessMana : ManaTypeTerm bs
  ManaOfColor : ColorTerm bs -> ManaTypeTerm bs

mutual
  public export
  data Duration : Bindings -> Type where
    ThisTurn : Duration bs
    RestOfGame : Duration bs
    Until : DurationEnd bs -> Duration bs
    ForAsLongAs : Condition bs -> Duration bs
    UntilEvent : GameEvent bs -> Duration bs
    DuringNextTurnOf : (who : Noun bs Player) ->
                       {auto 0 one : nounPlur who = OneOf} -> Duration bs

  public export
  data AttackDefender : Bindings -> Type where
    NoDefender : AttackDefender bs
    OneDefender : {k : Kind} -> (m : Noun bs k) ->
                  {auto 0 sg : nounPlur m = OneOf} ->
                  {auto 0 at : Attackable m} -> AttackDefender bs

  public export
  data DamagePatient : Bindings -> Type where
    NoPatient : DamagePatient bs
    OnePatient : {k : Kind} -> (m : Noun bs k) ->
                 {auto 0 rk : DamageRecipient m} -> DamagePatient bs

  public export
  data Door : Bindings -> Type where
    ThisDoor : Door bs
    DoorOf : (state : Maybe LockState) -> (room : Noun bs Object) ->
             {auto 0 zn : ZoneIs (nounZone room) Battlefield} ->
             Door bs

  public export
  doorIntro : {bs : Bindings} -> Door bs -> Bindings
  doorIntro ThisDoor = bs
  doorIntro (DoorOf _ room) = nomIntro room

  public export
  doorNamesHost : {0 bs : Bindings} -> Door bs -> Bool
  doorNamesHost ThisDoor = False
  doorNamesHost (DoorOf _ _) = True

  public export
  DoorNamesHost : {0 bs : Bindings} -> Door bs -> Type
  DoorNamesHost {bs} d = So (doorNamesHost d)

  public export
  creationVoiceOk : {bs : Bindings} -> Bool ->
                    Maybe (Noun bs Player) -> Maybe (Noun bs Player) -> Bool
  creationVoiceOk byEffect by under =
    (case by of
       Nothing => True
       Just w => isNil (nounDelta w)) &&
    (case under of
       Nothing => True
       Just u => isOne (nounPlur u) && isNil (nounDelta u)) &&
    not (byEffect && isJust by)

  public export
  causedByOk : {0 bs : Bindings} ->
               Bool -> Maybe (Noun bs Player) -> Bool
  causedByOk byEffect by = not (byEffect && isJust by)

  public export
  putDestOk : {0 bs : Bindings} -> ZoneExpr bs -> Bool
  putDestOk z = lookbackDestOk Placement (zoneSort z)

  public export
  PutDest : {0 bs : Bindings} -> ZoneExpr bs -> Type
  PutDest {bs} z = So (putDestOk z)

  public export
  putSourceOk : {0 bs : Bindings} -> Maybe (EventSource bs) -> Bool
  putSourceOk Nothing = True
  putSourceOk (Just src) = lookbackSourceOk Placement src

  public export
  PutSource : {0 bs : Bindings} -> Maybe (EventSource bs) -> Type
  PutSource {bs} s = So (putSourceOk s)

  public export
  entrySourceOk : {0 bs : Bindings} -> Maybe (EventSource bs) -> Bool
  entrySourceOk Nothing = True
  entrySourceOk (Just src) = lookbackSourceOk Entry src

  public export
  EntrySource : {0 bs : Bindings} -> Maybe (EventSource bs) -> Type
  EntrySource {bs} s = So (entrySourceOk s)

  public export
  data RollWatch : Bindings -> Type where
    AnyResult : RollWatch bs
    ResultIn : (q : Quantity bs) ->
               {auto 0 nz : NonZeroQ q} ->
               {auto 0 wf : WellFormedQ q} ->
               {auto 0 lt : So (quantLiteral q)} -> RollWatch bs
    HighestNatural : RollWatch bs

  public export
  watchFitsDie : {0 bs : Bindings} -> RolledDie -> RollWatch bs -> Bool
  watchFitsDie _ AnyResult = True
  watchFitsDie d (ResultIn _) = dieHasResult d
  watchFitsDie d HighestNatural = dieHasResult d

  public export
  defenderIntro : {bs : Bindings} -> AttackDefender bs -> Bindings
  defenderIntro NoDefender = bs
  defenderIntro (OneDefender m) = nomIntro m

  public export
  patientIntro : {bs : Bindings} -> DamagePatient bs -> Bindings
  patientIntro NoPatient = bs
  patientIntro (OnePatient m) = nomIntro m

  public export
  agentIntro : {bs : Bindings} -> Maybe (Noun bs Player) -> Bindings
  agentIntro Nothing = bs
  agentIntro (Just who) = nomIntro who

  public export
  patientZone : {bs : Bindings} -> Maybe (Noun bs Object) -> Maybe Zone
  patientZone Nothing = Nothing
  patientZone (Just n) = nounZone n

  public export
  patientZoneIsB : {bs : Bindings} -> Maybe (Noun bs Object) -> Zone -> Bool
  patientZoneIsB Nothing _ = True
  patientZoneIsB (Just n) z = zoneIsB (nounZone n) z

  public export
  PatientZoneIs : {bs : Bindings} -> Maybe (Noun bs Object) -> Zone -> Type
  PatientZoneIs what z = So (patientZoneIsB what z)

  public export
  verbPatientOk : {0 bs : Bindings} -> (v : VerbLabel) ->
                  Maybe (Noun bs Object) -> Bool
  verbPatientOk v Nothing = null (actPatientKindsOf v)
  verbPatientOk v (Just _) = elem Object (actPatientKindsOf v)

  public export
  verbBecomesOk : {0 bs : Bindings} -> (v : VerbLabel) ->
                  Maybe (Predicate bs Object) -> Bool
  verbBecomesOk v Nothing = True
  verbBecomesOk v (Just p) = actIntransitiveOf v && predSays p

  public export
  verbedVoiceOk : {0 bs : Bindings} -> {0 cs : Bindings} -> (v : VerbLabel) ->
                  Maybe (Noun bs Player) -> Maybe (Noun cs Object) -> Bool
  verbedVoiceOk v (Just _) _ = True
  verbedVoiceOk v Nothing what =
    isJust what && (actNamesParticiple v || actIntransitiveOf v)

  ||| A crime is one player's action against an opponent [CR#700.13], so the
  ||| committing player is a single player: the whole player set has no
  ||| opponent, hence no opponent-owned object a crime could target.
  public export
  data CrimeSubject : {0 bs : Bindings} -> Noun bs Player -> Type where
    OneCriminal : {0 bs : Bindings} -> {0 n : Noun bs Player} ->
                  {auto 0 ok : So (isOne (nounPlur n))} -> CrimeSubject n

  public export
  damageSourceZone : DamageKind -> Maybe Zone
  damageSourceZone AnyDamage = Nothing
  damageSourceZone CombatOnly = Just Battlefield
  damageSourceZone NoncombatOnly = Nothing

  public export
  data GameEvent : Bindings -> Type where
    Dies : (n : Noun bs Object) ->
           {auto 0 zn : ZoneIs (nounZone n) Battlefield} -> GameEvent bs
    Leaves : (n : Noun bs Object) -> (from : Maybe (EventSource bs)) ->
             {auto 0 zn : ZoneFits (nounZone n) (sourceZone from)} -> GameEvent bs
    IsDealtDamage : {k : Kind} -> (kind : DamageKind) -> (to : Noun bs k) ->
                    {auto 0 rk : DamageRecipient to} -> GameEvent bs
    Draws : (who : Noun bs Player) -> GameEvent bs
    LosesGame : (who : Noun bs Player) -> GameEvent bs
    Enters : (n : Noun bs Object) -> (from : Maybe (EventSource bs)) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 sk : EntrySource from} -> GameEvent bs
    Attacks : (n : Noun bs Object) ->
              (whom : AttackDefender (nomIntro n)) ->
              {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
              GameEvent bs
    AttacksWith : (who : Noun bs Player) ->
                  (whom : AttackDefender (nomIntro who)) ->
                  (attackers : Noun (defenderIntro whom) Object) ->
                  {auto 0 zn : ZoneIs (nounZone attackers) Battlefield} ->
                  GameEvent bs
    Blocks : (n : Noun bs Object) ->
             (what : Maybe (Noun (nomIntro n) Object)) ->
             {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
             {auto 0 bp : PatientZoneIs what Battlefield} ->
             GameEvent bs
    BecomesBlocked : (n : Noun bs Object) ->
                     (by : Maybe (Noun (nomIntro n) Object)) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     {auto 0 bp : PatientZoneIs by Battlefield} ->
                     GameEvent bs
    BecomesAttached : {k : Kind} -> (n : Noun bs Object) ->
                      (host : Noun (nomIntro n) k) ->
                      {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                      {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                      GameEvent bs
    BecomesUnattached : (n : Noun bs Object) ->
                        (host : Noun (nomIntro n) Object) ->
                        {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                        GameEvent bs
    DealsDamage : (kind : DamageKind) -> (n : Noun bs Object) ->
                  (to : DamagePatient (nomIntro n)) ->
                  {auto 0 zn : ZoneFits (nounZone n) (damageSourceZone kind)} ->
                  GameEvent bs
    BeginningOf : (q : PartQuant) -> (part : TurnPart) ->
                  (whose : HeaderPossessor bs) ->
                  {auto 0 pu : PartTriggerable part whose} -> GameEvent bs
    Casts : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
            (from : Maybe (ZoneExpr (nomIntro what))) ->
            {auto 0 zn : ZoneIs (nounZone what) Stack} ->
            {auto 0 one : nounPlur what = OneOf} ->
            {auto 0 nt : Nontarget what} ->
            {auto 0 pf : PlayableFrom (map Phrase.zoneSort from)} -> GameEvent bs
    BecomesTarget : {k : Kind} -> {kb : Kind} -> (n : Noun bs k) ->
                    (by : Noun (nomIntro n) kb) ->
                    {auto 0 tk : Targetable k} ->
                    {auto 0 tr : Targeter kb} -> GameEvent bs
    StatusEvent : {c : StatusCat} -> (n : Noun bs Object) ->
                  (v : StatusVal c) ->
                  {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                  {auto 0 at : StatusMarkable v} -> GameEvent bs
    DayNightShift : GameEvent bs
    StateHolds : (c : Condition bs) -> GameEvent bs
    PutInto : (n : Noun bs Object) -> (to : ZoneExpr bs) ->
              (from : Maybe (EventSource bs)) ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource from} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone from)} -> GameEvent bs
    CounterEvent : {k : Kind} ->
                   (dir : CounterMove) -> (kind : Maybe CounterKind) ->
                   (n : Noun bs k) ->
                   (many : CounterBatch) ->
                   (by : Maybe (Noun bs Player)) ->
                   (byEffect : Bool) ->
                   {auto 0 kn : CounterKindNamed k kind} ->
                   {auto 0 ag : EventAgent by} ->
                   {auto 0 lb : So (counterBatchOk many dir kind)} ->
                   {auto 0 cz : So (causedByOk byEffect by)} -> GameEvent bs
    TokensCreated : (n : Noun bs Object) ->
                    (byEffect : Bool) ->
                    (by : Maybe (Noun bs Player)) ->
                    (under : Maybe (Noun bs Player)) ->
                    {auto 0 tk : TokenPhrase n} ->
                    {auto 0 vo : So (creationVoiceOk byEffect by under)} ->
                    GameEvent bs
    ChapterMark : (ns : List ChapterNumber) ->
                  {auto 0 cm : ChapterMarks ns} -> GameEvent bs
    Activates : (who : Noun bs Player) ->
                (what : Noun (nomIntro who) Object) ->
                {auto 0 one : nounPlur what = OneOf} ->
                {auto 0 nt : Nontarget what} -> GameEvent bs
    StatBecomes : (n : Noun bs Object) -> (c : Stat) ->
                  (v : Amount (nomIntro n)) ->
                  {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                  GameEvent bs
    Regenerates : (n : Noun bs Object) ->
                  {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                  GameEvent bs
    FlipsCoin : (who : Noun bs Player) -> (call : Maybe FlipCall) -> GameEvent bs
    RollsDice : (who : Noun bs Player) -> (many : DiceBatch) ->
                (die : RolledDie) -> (res : RollWatch bs) ->
                {auto 0 dw : So (watchFitsDie die res)} -> GameEvent bs
    PaysCost : (who : Maybe (Noun bs Player)) -> (out : PaymentOutcome) ->
               (whose : Noun (agentIntro who) Object) -> (kw : KeywordLabel) ->
               {auto 0 kc : KeywordCost kw} ->
               {auto 0 one : nounPlur whose = OneOf} -> GameEvent bs
    PaysLife : (who : Noun bs Player) -> GameEvent bs
    LifeChanges : (who : Noun bs Player) -> (dir : LifeMove) -> GameEvent bs
    VerbedEvent : (who : Maybe (Noun bs Player)) -> (v : VerbLabel) ->
                  (what : Maybe (Noun (agentIntro who) Object)) ->
                  (becomes : Maybe (Predicate (agentIntro who) Object)) ->
                  {auto 0 kv : KnownAct v} ->
                  {auto 0 pt : So (verbPatientOk v what)} ->
                  {auto 0 zn : ZoneFits (patientZone what) (actZoneOf v)} ->
                  {auto 0 vc : So (verbedVoiceOk v who what)} ->
                  {auto 0 bc : So (verbBecomesOk v becomes)} ->
                  GameEvent bs
    ||| A mana ability with {T} in its cost resolving and producing mana
    ||| [CR#106.12a].
    TappedForMana : (who : Maybe (Noun bs Player)) ->
                    (what : Noun (agentIntro who) Object) ->
                    (ty : Maybe (ManaTypeTerm (agentIntro who))) ->
                    {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} ->
                    GameEvent bs
    UnlocksDoor : (who : Noun bs Player) -> (door : Door (nomIntro who)) ->
                  GameEvent bs
    NthOccurrence : (ord : Ordinal) -> (per : Maybe TurnPart) ->
                    (ev : GameEvent bs) -> GameEvent bs
    Triggers : (what : Noun bs Object) ->
               {auto 0 one : nounPlur what = OneOf} ->
               {auto 0 nt : Nontarget what} -> GameEvent bs
    CommitsCrime : (who : Noun bs Player) ->
                   {auto 0 sc : CrimeSubject who} -> GameEvent bs
    Causes : (by : Causing bs) -> (what : GameEvent (causingIntro by)) ->
             GameEvent bs

  public export
  data Causing : Bindings -> Type where
    CausedBySource : {k : Kind} -> (src : Noun bs k) -> Causing bs
    CausedByEvent : (ev : GameEvent bs) -> Causing bs
    CausedByAnEffect : Causing bs

  public export
  causingIntro : {bs : Bindings} -> Causing bs -> Bindings
  causingIntro (CausedBySource src) = nomIntro src
  causingIntro (CausedByEvent ev) = eventAfter ev
  causingIntro CausedByAnEffect = bs

  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (Dies _) = Death
  eventName (Leaves _ _) = Departure
  eventName (IsDealtDamage _ _) = DamageTaken
  eventName (Draws _) = CardDrawn
  eventName (LosesGame _) = GameLoss
  eventName (Enters _ _) = Entry
  eventName (Attacks _ _) = AttackDeclaration
  eventName (AttacksWith _ _ _) = AttackDeclaration
  eventName (Blocks _ _) = BlockDeclaration
  eventName (BecomesBlocked _ _) = BlockedDeclaration
  eventName (BecomesAttached _ _) = Attachment
  eventName (BecomesUnattached _ _) = Unattachment
  eventName (DealsDamage CombatOnly _ _) = CombatDamage
  eventName (DealsDamage _ _ _) = DamageDealing
  eventName (BeginningOf _ _ _) = PartBeginning
  eventName (Casts _ _ _) = SpellCast
  eventName (BecomesTarget _ _) = BecomesTarget
  eventName (StatusEvent {c} _ _) = statusEventName c
  eventName DayNightShift = TimeShift
  eventName (StateHolds _) = StateMatch
  eventName (PutInto _ _ _) = Placement
  eventName (CounterEvent dir _ _ many _ _) = counterEventName dir many
  eventName (TokensCreated _ _ _ _) = TokenCreation
  eventName (ChapterMark _) = ChapterArrival
  eventName (Activates _ _) = AbilityActivation
  eventName (StatBecomes _ _ _) = StatValueChange
  eventName (Regenerates _) = Regeneration
  eventName (FlipsCoin _ Nothing) = CoinFlip
  eventName (FlipsCoin _ (Just call)) = flipEventName call
  eventName (RollsDice _ _ _ _) = DiceRoll
  eventName (PaysCost _ out _ _) = paymentEventName out
  eventName (PaysLife _) = LifePayment
  eventName (LifeChanges _ dir) = lifeEventName dir
  eventName (VerbedEvent _ v _ _) = VerbedAct v
  eventName (TappedForMana _ _ _) = TappedForMana
  eventName (UnlocksDoor _ _) = VerbedAct "Unlock"
  eventName (NthOccurrence _ _ ev) = eventName ev
  eventName (Triggers _) = AbilityTrigger
  eventName (CommitsCrime _) = CrimeCommission
  eventName (Causes _ what) = eventName what

  public export
  eventIntro : {bs : Bindings} -> GameEvent bs -> Bindings
  eventIntro (Dies n) = selfSubjIntro n
  eventIntro (Leaves n _) = selfSubjIntro n
  eventIntro (IsDealtDamage _ to) = outcomeB DamageDealt :: selfSubjIntro to
  eventIntro (Draws who) = selfSubjIntro who
  eventIntro (LosesGame who) = selfSubjIntro who
  eventIntro (Enters n _) = selfSubjIntro n
  eventIntro (Attacks n NoDefender) = selfSubjIntro n
  eventIntro (Attacks _ (OneDefender whom)) = selfSubjIntro whom
  eventIntro (AttacksWith _ _ attackers) = selfSubjIntro attackers
  eventIntro (Blocks n Nothing) = selfSubjIntro n
  eventIntro (Blocks _ (Just what)) = selfSubjIntro what
  eventIntro (BecomesBlocked n Nothing) = selfSubjIntro n
  eventIntro (BecomesBlocked _ (Just by)) = selfSubjIntro by
  eventIntro (BecomesAttached _ host) = selfSubjIntro host
  eventIntro (BecomesUnattached _ host) = selfSubjIntro host
  eventIntro (DealsDamage _ n NoPatient) = outcomeB DamageDealt :: selfSubjIntro n
  eventIntro (DealsDamage _ _ (OnePatient m)) =
    outcomeB DamageDealt :: selfSubjIntro m
  eventIntro (BeginningOf _ _ _) = bs
  eventIntro (Casts _ what _) = selfSubjIntro what
  eventIntro (BecomesTarget _ by) = selfSubjIntro by
  eventIntro (StatusEvent n _) = selfSubjIntro n
  eventIntro DayNightShift = bs
  eventIntro (StateHolds _) = bs
  eventIntro (PutInto n _ _) = selfSubjIntro n
  eventIntro (CounterEvent _ _ n OneCounter _ _) = selfSubjIntro n
  eventIntro (CounterEvent _ _ n ManyCounters _ _) =
    outcomeB CountersPut :: selfSubjIntro n
  eventIntro (CounterEvent _ _ n LastCounter _ _) = selfSubjIntro n
  eventIntro (TokensCreated n _ _ _) = selfSubjIntro n
  eventIntro (ChapterMark _) = bs
  eventIntro (Activates _ what) = selfSubjIntro what
  eventIntro (StatBecomes _ _ v) = amtIntro v
  eventIntro (Regenerates n) = selfSubjIntro n
  eventIntro (FlipsCoin who _) = selfSubjIntro who
  eventIntro (RollsDice who OneDie _ _) = selfSubjIntro who
  eventIntro (RollsDice who ManyDice _ _) = outcomeB DiceRolled :: selfSubjIntro who
  eventIntro (PaysCost _ _ whose _) = selfSubjIntro whose
  eventIntro (PaysLife who) = selfSubjIntro who
  eventIntro (LifeChanges who dir) =
    outcomeB (lifeMoveOutcome dir) :: selfSubjIntro who
  eventIntro (VerbedEvent who _ Nothing _) = agentIntro who
  eventIntro (VerbedEvent _ _ (Just what) _) = selfSubjIntro what
  eventIntro (TappedForMana _ what _) = selfSubjIntro what
  eventIntro (UnlocksDoor who door) = doorIntro door
  eventIntro (NthOccurrence _ _ ev) = eventIntro ev
  eventIntro (Triggers what) = selfSubjIntro what
  eventIntro (CommitsCrime who) = selfSubjIntro who
  eventIntro (Causes _ what) = eventIntro what

  public export
  eventAfter : {bs : Bindings} -> GameEvent bs -> Bindings
  eventAfter (Dies n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (Leaves n _) = moveIntro Nothing n Nothing
  eventAfter (IsDealtDamage _ to) = outcomeB DamageDealt :: selfSubjIntro to
  eventAfter (Draws who) = nomIntro who
  eventAfter (LosesGame who) = nomIntro who
  eventAfter (Enters n _) = moveIntro Nothing n (Just Battlefield)
  eventAfter (Attacks n NoDefender) = selfSubjIntro n
  eventAfter (Attacks n (OneDefender whom)) = nounDelta whom ++ selfSubjIntro n
  eventAfter (AttacksWith _ _ attackers) = nomIntro attackers
  eventAfter (Blocks n Nothing) = selfSubjIntro n
  eventAfter (Blocks _ (Just what)) = nomIntro what
  eventAfter (BecomesBlocked n Nothing) = selfSubjIntro n
  eventAfter (BecomesBlocked _ (Just by)) = nomIntro by
  eventAfter (BecomesAttached _ host) = nomIntro host
  eventAfter (BecomesUnattached _ host) = nomIntro host
  eventAfter (DealsDamage _ n NoPatient) = outcomeB DamageDealt :: selfSubjIntro n
  eventAfter (DealsDamage _ _ (OnePatient m)) = outcomeB DamageDealt :: nomIntro m
  eventAfter (Casts _ what _) = nomIntro what
  eventAfter (BecomesTarget n by) = nounDelta by ++ selfSubjIntro n
  eventAfter (BeginningOf _ _ whose) = possessorIntro whose
  eventAfter (StatusEvent n _) = selfSubjIntro n
  eventAfter DayNightShift = bs
  eventAfter (StateHolds _) = bs
  eventAfter (PutInto n to _) = moveIntro Nothing n (Just (zoneSort to))
  eventAfter (CounterEvent _ _ n OneCounter _ _) = selfSubjIntro n
  eventAfter (CounterEvent _ _ n ManyCounters _ _) =
    outcomeB CountersPut :: selfSubjIntro n
  eventAfter (CounterEvent _ _ n LastCounter _ _) = selfSubjIntro n
  eventAfter (TokensCreated n _ _ _) = nomIntro n
  eventAfter (ChapterMark _) = bs
  eventAfter (Activates _ what) = nomIntro what
  eventAfter (StatBecomes n _ v) = amtDelta v ++ selfSubjIntro n
  eventAfter (Regenerates n) = selfSubjIntro n
  eventAfter (FlipsCoin who Nothing) = outcomeB CoinFlipped :: nomIntro who
  eventAfter (FlipsCoin who (Just _)) = nomIntro who
  eventAfter (RollsDice who _ PlanarDie _) = outcomeB PlanarRolled :: nomIntro who
  eventAfter (RollsDice who _ _ _) = outcomeB RollResult :: nomIntro who
  eventAfter (PaysCost _ _ whose _) = nomIntro whose
  eventAfter (PaysLife who) = outcomeB LifeLost :: nomIntro who
  eventAfter (LifeChanges who dir) = outcomeB (lifeMoveOutcome dir) :: nomIntro who
  eventAfter (VerbedEvent who _ Nothing _) = agentIntro who
  eventAfter (VerbedEvent _ v (Just what) _) =
    moveIntro (Just v) what (maybe (nounZone what) Just (actDestOf v))
  eventAfter (TappedForMana _ what _) =
    outcomeB ManaProduced :: stampIntro (Just "Tap") what
  eventAfter (UnlocksDoor who door) = doorIntro door
  eventAfter (NthOccurrence _ _ ev) = eventAfter ev
  eventAfter (Triggers what) = nomIntro what
  eventAfter (CommitsCrime who) = nomIntro who
  eventAfter (Causes _ what) = eventAfter what

  public export
  eventSubjectPlur : {bs : Bindings} -> GameEvent bs -> Plurality
  eventSubjectPlur (Dies n) = nounPlur n
  eventSubjectPlur (Leaves n _) = nounPlur n
  eventSubjectPlur (IsDealtDamage _ to) = nounPlur to
  eventSubjectPlur (Draws who) = nounPlur who
  eventSubjectPlur (LosesGame who) = nounPlur who
  eventSubjectPlur (Enters n _) = nounPlur n
  eventSubjectPlur (Attacks n _) = nounPlur n
  eventSubjectPlur (AttacksWith who _ _) = nounPlur who
  eventSubjectPlur (Blocks n _) = nounPlur n
  eventSubjectPlur (BecomesBlocked n _) = nounPlur n
  eventSubjectPlur (BecomesAttached n _) = nounPlur n
  eventSubjectPlur (BecomesUnattached n _) = nounPlur n
  eventSubjectPlur (DealsDamage _ n _) = nounPlur n
  eventSubjectPlur (BeginningOf _ _ _) = OneOf
  eventSubjectPlur (Casts _ what _) = nounPlur what
  eventSubjectPlur (BecomesTarget n _) = nounPlur n
  eventSubjectPlur (StatusEvent n _) = nounPlur n
  eventSubjectPlur DayNightShift = OneOf
  eventSubjectPlur (StateHolds _) = OneOf
  eventSubjectPlur (PutInto n _ _) = nounPlur n
  eventSubjectPlur (CounterEvent _ _ n _ _ _) = nounPlur n
  eventSubjectPlur (TokensCreated n _ _ _) = nounPlur n
  eventSubjectPlur (ChapterMark _) = OneOf
  eventSubjectPlur (Activates who _) = nounPlur who
  eventSubjectPlur (StatBecomes n _ _) = nounPlur n
  eventSubjectPlur (Regenerates n) = nounPlur n
  eventSubjectPlur (FlipsCoin who _) = nounPlur who
  eventSubjectPlur (RollsDice who _ _ _) = nounPlur who
  eventSubjectPlur (PaysCost (Just who) _ _ _) = nounPlur who
  eventSubjectPlur (PaysCost Nothing _ _ _) = OneOf
  eventSubjectPlur (PaysLife who) = nounPlur who
  eventSubjectPlur (LifeChanges who _) = nounPlur who
  eventSubjectPlur (VerbedEvent (Just who) _ _ _) = nounPlur who
  eventSubjectPlur (VerbedEvent Nothing _ (Just what) _) = nounPlur what
  eventSubjectPlur (VerbedEvent Nothing _ Nothing _) = OneOf
  eventSubjectPlur (TappedForMana (Just who) _ _) = nounPlur who
  eventSubjectPlur (TappedForMana Nothing what _) = nounPlur what
  eventSubjectPlur (UnlocksDoor who _) = nounPlur who
  eventSubjectPlur (NthOccurrence _ _ ev) = eventSubjectPlur ev
  eventSubjectPlur (Triggers what) = nounPlur what
  eventSubjectPlur (CommitsCrime who) = nounPlur who
  eventSubjectPlur (Causes _ what) = eventSubjectPlur what

  public export
  eventNamesThisDoor : {0 bs : Bindings} -> GameEvent bs -> Bool
  eventNamesThisDoor (UnlocksDoor _ ThisDoor) = True
  eventNamesThisDoor (NthOccurrence _ _ ev) = eventNamesThisDoor ev
  eventNamesThisDoor (Causes _ what) = eventNamesThisDoor what
  eventNamesThisDoor _ = False

  public export
  anyEventNamesThisDoor : {0 bs : Bindings} -> List (GameEvent bs) -> Bool
  anyEventNamesThisDoor [] = False
  anyEventNamesThisDoor (e :: es) =
    eventNamesThisDoor e || anyEventNamesThisDoor es

  public export
  armsAgree : {bs : Bindings} -> (read : GameEvent bs -> Bindings) ->
              Bindings -> List (GameEvent bs) -> Bool
  armsAgree read ds [] = True
  armsAgree read ds (a :: as) = sameBindings ds (read a) && armsAgree read ds as

  public export
  unionArms : {bs : Bindings} -> (read : GameEvent bs -> Bindings) ->
              Bindings -> List (GameEvent bs) -> Maybe Bindings
  unionArms read ds [] = Just ds
  unionArms read ds (a :: as) =
    case unionBindings ds (read a) of
      Just u => unionArms read u as
      Nothing => Nothing

  public export
  sharedCtx : {bs : Bindings} -> (read : GameEvent bs -> Bindings) ->
              List (GameEvent bs) -> GameEvent bs -> Bindings
  sharedCtx read alts ev =
    if armsAgree read (read ev) alts then read ev
    else case unionArms read (read ev) alts of
           Just u => u
           Nothing => bs

  public export
  delayedCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  delayedCtx alts ev = settleTargets (sharedCtx eventAfter alts ev)

  public export
  interceptCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  interceptCtx alts ev = sharedCtx eventIntro alts ev

  public export
  Interceptable : GameEvent bs -> Type
  Interceptable {bs} ev = So (interceptOk (eventName ev))

  public export
  interceptArmsOk : {0 bs : Bindings} -> List (GameEvent bs) -> Bool
  interceptArmsOk [] = True
  interceptArmsOk (a :: as) = interceptOk (eventName a) && interceptArmsOk as

  public export
  InterceptableArms : {0 bs : Bindings} -> List (GameEvent bs) -> Type
  InterceptableArms as = So (interceptArmsOk as)

  public export
  data HeaderPossessor : Bindings -> Type where
    NoPossessor : HeaderPossessor bs
    ByPlayer : (n : Noun bs Player) -> HeaderPossessor bs
    ByTurn : (n : Noun bs TurnRef) -> HeaderPossessor bs

  public export
  possessorIntro : {bs : Bindings} -> HeaderPossessor bs -> Bindings
  possessorIntro NoPossessor = bs
  possessorIntro (ByPlayer n) = selfSubjDelta n ++ Phrase.agentIntro n
  possessorIntro (ByTurn _) = bs

  public export
  headerPossessorOk : {bs : Bindings} -> HeaderPossessor bs -> Bool
  headerPossessorOk NoPossessor = True
  headerPossessorOk (ByPlayer n) = partPossessorOk (Just n)
  headerPossessorOk (ByTurn _) = True

  public export
  PartTriggerable : {bs : Bindings} -> TurnPart -> HeaderPossessor bs -> Type
  PartTriggerable {bs} p h = So (properTurnPart p && headerPossessorOk h)

  public export
  AddedPart : TurnPart -> Type
  AddedPart p = So (properTurnPart p)

  public export
  AddedPartWritten : Maybe TurnPart -> Type
  AddedPartWritten = OptOk AddedPart

  public export
  durationOk : {0 bs : Bindings} -> Duration bs -> Bool
  durationOk (UntilEvent ev) = spanEventOk (eventName ev)
  durationOk _ = True

  public export
  spanIntro : {bs : Bindings} -> Duration bs -> Bindings
  spanIntro ThisTurn = bs
  spanIntro RestOfGame = bs
  spanIntro (Until _) = bs
  spanIntro (ForAsLongAs c) = condIntro c
  spanIntro (UntilEvent ev) = eventIntro ev
  spanIntro (DuringNextTurnOf who) = nomIntro who

  public export
  SpanOk : Maybe (Duration bs) -> Type
  SpanOk = OptOk (\d => So (durationOk d))

  public export
  DelaySpanOk : Maybe (Duration bs) -> Type
  DelaySpanOk = SpanOk

  public export
  data TriggerWindow : Bindings -> Type where
    DuringWindow : (p : TurnPart) -> (w : Maybe (Noun bs Player)) ->
                   {auto 0 hw : WindowOk p w} -> TriggerWindow bs

  public export
  data Timing : Bindings -> Type where
    AsSorcery : Timing bs
    AsInstant : Timing bs
    DuringPart : (p : TurnPart) -> (w : Maybe (Noun bs Player)) ->
                 {auto 0 wk : WindowOk p w} -> Timing bs
    BeforePart : (p : TurnPart) -> (w : Maybe (Noun bs Player)) ->
                 {auto 0 bp : So (properTurnPart p)} ->
                 {auto 0 pk : PointWindowOk w} -> Timing bs

  public export
  data UsageLimit = OncePerTurn | OncePerGame | ActionOncePerTurn

  public export
  untriggeredLimitOk : Maybe UsageLimit -> Bool
  untriggeredLimitOk Nothing = True
  untriggeredLimitOk (Just OncePerTurn) = True
  untriggeredLimitOk (Just OncePerGame) = True
  untriggeredLimitOk (Just ActionOncePerTurn) = False

  public export
  data AltEvent : TriggerWord -> List (GameEvent bs) -> Type where
    NoAlt : AltEvent w []
    MoreAlt : {0 e : GameEvent bs} -> {0 es : List (GameEvent bs)} ->
              {auto 0 hn : HeaderNontarget e} ->
              {auto 0 hs : HeaderStatus e} ->
              {auto 0 rest : AltEvent w es} -> AltEvent w (e :: es)

  public export
  headerCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  headerCtx alts ev = sharedCtx eventAfter alts ev

  public export
  data Concurrent : Bindings -> Type where
    WhileTrue : (c : Condition bs) -> Concurrent bs
    WhileDoing : (ev : GameEvent bs) ->
                 {auto 0 up : So (eventUnderwayOk (eventName ev))} ->
                 Concurrent bs

  public export
  data JoinedHeader : Bindings -> Type where
    MkJoinedHeader : (word : TriggerWord) -> (ev : GameEvent bs) ->
                     (alts : List (GameEvent bs)) ->
                     (while : Maybe (Concurrent (headerCtx alts ev))) ->
                     (window : Maybe (TriggerWindow bs)) ->
                     {auto 0 hn : HeaderNontarget ev} ->
                     {auto 0 hs : HeaderStatus ev} ->
                     {auto 0 ae : AltEvent word alts} ->
                     JoinedHeader bs

  public export
  joinedAfter : {bs : Bindings} -> JoinedHeader bs -> Bindings
  joinedAfter (MkJoinedHeader _ ev alts _ _) = headerCtx alts ev

  public export
  joinedCtx : {bs : Bindings} -> List (JoinedHeader bs) -> Bindings -> Bindings
  joinedCtx [] ds = ds
  joinedCtx (j :: js) ds =
    if sameBindings ds (joinedAfter j) then joinedCtx js ds else bs

  public export
  concurrentNamesThisDoor : {0 bs : Bindings} -> Maybe (Concurrent bs) -> Bool
  concurrentNamesThisDoor Nothing = False
  concurrentNamesThisDoor (Just (WhileTrue _)) = False
  concurrentNamesThisDoor (Just (WhileDoing ev)) = eventNamesThisDoor ev

  public export
  joinedNamesThisDoor : {0 bs : Bindings} -> JoinedHeader bs -> Bool
  joinedNamesThisDoor (MkJoinedHeader _ ev alts while _) =
    eventNamesThisDoor ev || anyEventNamesThisDoor alts ||
      concurrentNamesThisDoor while

  public export
  joinsNameThisDoor : {0 bs : Bindings} -> List (JoinedHeader bs) -> Bool
  joinsNameThisDoor [] = False
  joinsNameThisDoor (j :: js) = joinedNamesThisDoor j || joinsNameThisDoor js

  public export
  chapterDefaultsOk : {bs : Bindings} ->
                      (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
                      Maybe (Concurrent (headerCtx alts ev)) ->
                      (joins : List (JoinedHeader bs)) ->
                      Maybe (TriggerWindow bs) -> Maybe UsageLimit ->
                      Maybe (Condition (joinedCtx joins (headerCtx alts ev))) -> Bool
  chapterDefaultsOk (ChapterMark _) [] Nothing [] Nothing Nothing Nothing = True
  chapterDefaultsOk (ChapterMark _) _ _ _ _ _ _ = False
  chapterDefaultsOk _ _ _ _ _ _ _ = True

  public export
  ChapterDefaults : {bs : Bindings} -> (ev : GameEvent bs) -> (alts : List (GameEvent bs)) -> Maybe (Concurrent (headerCtx alts ev)) -> (joins : List (JoinedHeader bs)) -> Maybe (TriggerWindow bs) -> Maybe UsageLimit -> Maybe (Condition (joinedCtx joins (headerCtx alts ev))) -> Type
  ChapterDefaults {bs} ev alts wh js w l i = So (chapterDefaultsOk ev alts wh js w l i)

  public export
  HeaderNontarget : {bs : Bindings} -> GameEvent bs -> Type
  HeaderNontarget {bs} ev = So (not (anyTargetedAt (eventIntro ev)))

  public export
  headerStatusOk : {0 bs : Bindings} -> GameEvent bs -> Bool
  headerStatusOk (StatusEvent _ v) = statusMarkable v
  headerStatusOk _ = True

  public export
  HeaderStatus : {bs : Bindings} -> GameEvent bs -> Type
  HeaderStatus {bs} ev = So (headerStatusOk ev)
