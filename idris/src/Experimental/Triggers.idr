||| The trigger layer: the event-pattern algebra and trigger-header
||| machinery built over it.
module Experimental.Triggers

import public Experimental.Phrase

%default total

mutual
  public export
  data Duration : Bindings -> Type where
    ThisTurn : Duration bs
    ||| [CR#702.131a] and [CR#702.195a] both write "for the rest of the
    ||| game", which no turn-part endpoint spells.
    RestOfGame : Duration bs
    Until : DurationEnd -> Duration bs
    ForAsLongAs : Condition bs -> Duration bs
    UntilEvent : GameEvent bs -> Duration bs
    ||| "During [who]'s next turn": a named player's next turn entire.
    ||| Every other next-turn span the corpus writes is an ENDPOINT --
    ||| "until your next turn", "until the end of your next turn" --
    ||| which `Until` already spells against `StartOf`/`EndOf`; this one
    ||| is neither endpoint but the turn between them.
    ||| The possessor is a noun and not a `Whose` because the surface asks
    ||| for one: beside "your" and "that player's" the printed phrasings
    ||| include "target opponent's", "target player's", "its controller's"
    ||| and "the first/second player's". One of those announces a target,
    ||| which the governed statement reads back ("creatures THAT PLAYER
    ||| controls"), and only the leading twin `Throughout` is placed to
    ||| let it. A turn has one active player [CR#102.1], so the possessor
    ||| names one.
    DuringNextTurnOf : (who : Noun bs Player) ->
                       {auto 0 one : nounPlur who = OneOf} -> Duration bs

  ||| The defender an attack names, or none written. A creature attacks
  ||| one defender [CR#508.1b] and [CR#506.3] closes what that may be, so
  ||| the slot is kind-polymorphic under a gate rather than player-kinded:
  ||| "that player or planeswalker" is an ordinary joined-kind noun.
  ||| The unwritten row carries no kind at all.
  public export
  data AttackDefender : Bindings -> Type where
    NoDefender : AttackDefender bs
    OneDefender : {k : Kind} -> (m : Noun bs k) ->
                  {auto 0 sg : nounPlur m = OneOf} ->
                  {auto 0 at : Attackable m} -> AttackDefender bs

  public export
  data BlockPartner : {0 bs : Bindings} -> Maybe (Noun bs Object) -> Type where
    NoPartner : BlockPartner Nothing
    OnePartner : {0 m : Noun bs Object} ->
                 {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} -> BlockPartner (Just m)

  public export
  data CreationVoice : {0 bs : Bindings} -> Maybe Causer ->
                       Maybe (Noun bs Player) -> Maybe (Noun bs Player) -> Type where
    CreatedPlain : CreationVoice Nothing Nothing Nothing
    CreatedBy : {0 w : Noun bs Player} ->
                {auto 0 bl : Bindingless w} -> CreationVoice Nothing (Just w) Nothing
    CreatedUnder : {0 u : Noun bs Player} ->
                   {auto 0 one : nounPlur u = OneOf} ->
                   {auto 0 bl : Bindingless u} -> CreationVoice Nothing Nothing (Just u)
    ||| [CR#111.2] makes the creator the token's controller, so naming
    ||| both says one thing twice — redundant, not meaningless.
    CreatedByUnder : {0 w, u : Noun bs Player} ->
                     {auto 0 one : nounPlur u = OneOf} ->
                     {auto 0 bw : Bindingless w} ->
                     {auto 0 bu : Bindingless u} ->
                     CreationVoice Nothing (Just w) (Just u)
    CreatedByCauser : {0 c : Causer} -> {0 u : Noun bs Player} ->
                      {auto 0 one : nounPlur u = OneOf} ->
                      {auto 0 bl : Bindingless u} ->
                      CreationVoice (Just c) Nothing (Just u)
    ||| A causer with no stated controller: [CR#111.2] supplies one.
    CreatedByCauserPlain : {0 c : Causer} ->
                           CreationVoice (Just c) Nothing Nothing

  public export
  data CausedBy : {0 bs : Bindings} ->
                  Maybe Causer -> Maybe (Noun bs Player) -> Type where
    NotCaused : {0 w : Maybe (Noun bs Player)} -> CausedBy Nothing w
    CausedByEffect : {0 c : Causer} -> CausedBy (Just c) Nothing

  public export
  data EventSource : Bindings -> Type where
    FromAnywhere : EventSource bs
    FromZone : (z : ZoneExpr bs) -> EventSource bs

  public export
  sourceZone : {0 bs : Bindings} -> Maybe (EventSource bs) -> Maybe Zone
  sourceZone Nothing = Nothing
  sourceZone (Just FromAnywhere) = Nothing
  sourceZone (Just (FromZone z)) = Just (zoneSort z)

  ||| Which zone a `PutInto` event may describe a card as arriving in.
  ||| A different table from the move instruction's `DestOk`: this event
  ||| DESCRIBES where a card landed rather than instructing a move, so
  ||| its scope over possessed zones is free where `DestOk`'s is gated
  ||| [CR#400.3]. The battlefield is excluded because English writes
  ||| "put ONTO the battlefield", never "into" it [CR#603.6a].
  public export
  putDestZoneOk : Zone -> Bool
  putDestZoneOk Graveyard = True
  putDestZoneOk Exile = True
  putDestZoneOk Library = True
  putDestZoneOk Hand = True
  putDestZoneOk Battlefield = True
  putDestZoneOk Stack = False

  public export
  putDestOk : {0 bs : Bindings} -> ZoneExpr bs -> Bool
  putDestOk z = putDestZoneOk (zoneSort z)

  public export
  PutDest : {0 bs : Bindings} -> ZoneExpr bs -> Type
  PutDest {bs} z = So (putDestOk z)

  public export
  putSourceZoneOk : Zone -> Bool
  putSourceZoneOk Battlefield = True
  putSourceZoneOk Graveyard = True
  putSourceZoneOk Library = True
  putSourceZoneOk Exile = True
  putSourceZoneOk Hand = False
  putSourceZoneOk Stack = False

  public export
  putSourceOk : {0 bs : Bindings} -> Maybe (EventSource bs) -> Bool
  putSourceOk Nothing = True
  putSourceOk (Just FromAnywhere) = True
  putSourceOk (Just (FromZone z)) = putSourceZoneOk (zoneSort z)

  public export
  PutSource : {0 bs : Bindings} -> Maybe (EventSource bs) -> Type
  PutSource {bs} s = So (putSourceOk s)

  ||| The event algebra's shape, decided once: composition is carried by
  ||| the slots this vocabulary already has, not by operator constructors.
  ||| The one operator row is `NthOccurrence`. Disjunction lives at the
  ||| trigger header (`AltEvent`), where `headerCtx` reads back the
  ||| announcement the two arms share; the header's `alt` slot is the one
  ||| seat that never consults `eventName`, and a general row would leave
  ||| that classifier naming one of two events. Negation is the subject
  ||| predicate's `Not`, or `NotCond` over `Happened`; a conditioned event
  ||| is the header's intervening slot [CR#603.4]; a window is the
  ||| reader's `Lookback` or the header's `TriggerWindow`. Cause is never
  ||| an agency channel: a printed cause is a different VERB (its own row
  ||| here or its own `EventName`), a per-row causer slot (`CounterEvent`,
  ||| `TokensCreated`), or the by-source agent phrase, which waits at its
  ||| ledger tag.
  public export
  data GameEvent : Bindings -> Type where
    Dies : (n : Noun bs Object) ->
           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
    Leaves : (n : Noun bs Object) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
    IsDestroyed : (n : Noun bs Object) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
    IsDealtDamage : {k : Kind} -> (to : Noun bs k) ->
                    {auto 0 rk : DamageRecipient to} -> GameEvent bs
    Draws : (who : Noun bs Player) -> GameEvent bs
    LosesGame : (who : Noun bs Player) -> GameEvent bs
    Enters : (n : Noun bs Object) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} -> GameEvent bs
    Attacks : (n : Noun bs Object) ->
              (whom : AttackDefender (nomIntro n)) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              GameEvent bs
    Blocks : (n : Noun bs Object) ->
             (what : Maybe (Noun (nomIntro n) Object)) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 bp : BlockPartner what} -> GameEvent bs
    BecomesBlocked : (n : Noun bs Object) ->
                     (by : Maybe (Noun (nomIntro n) Object)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 bp : BlockPartner by} -> GameEvent bs
    DealsCombatDamage : {k : Kind} -> (n : Noun bs Object) ->
                        (to : Noun (nomIntro n) k) ->
                        {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                        {auto 0 rk : DamageRecipient to} -> GameEvent bs
    BeginningOf : (part : TurnPart) -> (whose : HeaderPossessor bs) ->
                  {auto 0 pu : PartTriggerable part whose} ->
                  {auto 0 td : TurnDeixis (possessorWord whose) bs} -> GameEvent bs
    Casts : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
            {auto 0 zn : OnStack (nounZone what)} ->
            {auto 0 one : nounPlur what = OneOf} ->
            {auto 0 nt : Nontarget what} -> GameEvent bs
    StatusEvent : {c : StatusCat} -> (n : Noun bs Object) ->
                  (v : StatusVal c) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  {auto 0 at : StatusEventVal v} -> GameEvent bs
    DayNightShift : GameEvent bs
    LastCounterRemoved : (kind : CounterKind) -> (n : Noun bs Object) ->
                         (by : Maybe (Noun bs Player)) ->
                         {auto 0 sc : counterScope kind = Object} ->
                         {auto 0 ag : EventAgent by} -> GameEvent bs
    PutInto : (n : Noun bs Object) -> (to : ZoneExpr bs) ->
              (from : Maybe (EventSource bs)) ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource from} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone from)} -> GameEvent bs
    ||| A counter arriving on or leaving its holder. The holder is
    ||| kind-polymorphic because [CR#122.1] places a counter "on an
    ||| object or player", and the kind gate keeps each counter word on
    ||| its own side of that line. The player side spells the same event
    ||| with the `get` verb -- Winding Constrictor's "if you would get one
    ||| or more counters".
    CounterEvent : {k : Kind} ->
                   (dir : CounterMove) -> (kind : Maybe CounterKind) ->
                   (n : Noun bs k) ->
                   (many : CounterBatch) ->
                   (by : Maybe (Noun bs Player)) ->
                   (cause : Maybe Causer) ->
                   {auto 0 kn : CounterKindNamed k kind} ->
                   {auto 0 ag : EventAgent by} ->
                   {auto 0 cz : CausedBy cause by} -> GameEvent bs
    TokensCreated : (n : Noun bs Object) ->
                    (cause : Maybe Causer) ->
                    (by : Maybe (Noun bs Player)) ->
                    (under : Maybe (Noun bs Player)) ->
                    {auto 0 tk : TokenPhrase n} ->
                    {auto 0 vo : CreationVoice cause by under} -> GameEvent bs
    ChapterMark : (ns : List ChapterNumber) ->
                  {auto 0 cm : ChapterMarks ns} -> GameEvent bs
    Activates : (who : Noun bs Player) ->
                (what : Noun (nomIntro who) Ability) ->
                {auto 0 one : nounPlur what = OneOf} ->
                {auto 0 nt : Nontarget what} -> GameEvent bs
    ||| "[its] power becomes 20": [CR#603.2e] licenses a "becomes" event,
    ||| which happens only as the value is reached and not while it holds.
    StatBecomes : (n : Noun bs Object) -> (c : Characteristic) ->
                  (v : Amount (nomIntro n)) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  GameEvent bs
    ||| "[it] regenerates": [CR#701.19a]'s shield applying, which is not
    ||| the same event as creating it [CR#701.19c].
    Regenerates : (n : Noun bs Object) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  GameEvent bs
    ||| The ordinal occurrence of an event: "When the fourth plan counter
    ||| is put on this enchantment", "Whenever you cast your first spell
    ||| during each opponent's turn". The ordinal names WHICH occurrence in
    ||| a sequence — a third thing beside `CounterBatch`'s two determiners,
    ||| deliberately not a third batch arm. A wrapper rather than per-event
    ||| twins, so one word serves every countable event; a wrapped wrapper
    ||| ("the third first spell") is tolerated overgeneration. The trigger
    ||| word stays the header's own slot, unconstrained here: an ordinal
    ||| names WHICH occurrence, not how often the header may trigger.
    ||| -- spelling: at a counter event, "When the [ord] [kind] counter is
    ||| put on [n]"; at a cast event, "Whenever [who] cast(s) [whose]
    ||| [ord] spell [window]".
    NthOccurrence : (ord : Ordinal) -> (ev : GameEvent bs) -> GameEvent bs

  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (Dies _) = Death
  eventName (Leaves _) = Departure
  eventName (IsDestroyed _) = Destruction
  eventName (IsDealtDamage _) = DamageTaken
  eventName (Draws _) = CardDrawn
  eventName (LosesGame _) = GameLoss
  eventName (Enters _) = Entry
  eventName (Attacks _ _) = AttackDeclaration
  eventName (Blocks _ _) = BlockDeclaration
  eventName (BecomesBlocked _ _) = BlockedDeclaration
  eventName (DealsCombatDamage _ _) = CombatDamage
  eventName (BeginningOf _ _) = PartBeginning
  eventName (Casts _ _) = SpellCast
  eventName (StatusEvent {c} _ _) = statusEventName c
  eventName DayNightShift = TimeShift
  eventName (LastCounterRemoved _ _ _) = LastCounterRemoval
  eventName (PutInto _ _ _) = Placement
  eventName (CounterEvent dir _ _ _ _ _) = counterEventName dir
  eventName (TokensCreated _ _ _ _) = TokenCreation
  eventName (ChapterMark _) = ChapterArrival
  eventName (Activates _ _) = AbilityActivation
  eventName (StatBecomes _ _ _) = StatValueChange
  eventName (Regenerates _) = Regeneration
  eventName (NthOccurrence _ ev) = eventName ev

  ||| What an event pattern contributes before it happens — its announced
  ||| subject phrase [CR#601.2c]. Read by an interception's replacement,
  ||| whose replaced event never happens [CR#614.6].
  public export
  eventIntro : {bs : Bindings} -> GameEvent bs -> Bindings
  eventIntro (Dies n) = nomIntro n
  eventIntro (Leaves n) = nomIntro n
  eventIntro (IsDestroyed n) = nomIntro n
  eventIntro (IsDealtDamage to) = nomIntro to
  eventIntro (Draws who) = nomIntro who
  eventIntro (LosesGame who) = nomIntro who
  eventIntro (Enters n) = nomIntro n
  eventIntro (Attacks n NoDefender) = nomIntro n
  eventIntro (Attacks _ (OneDefender whom)) = nomIntro whom
  eventIntro (Blocks n Nothing) = nomIntro n
  eventIntro (Blocks _ (Just what)) = nomIntro what
  eventIntro (BecomesBlocked n Nothing) = nomIntro n
  eventIntro (BecomesBlocked _ (Just by)) = nomIntro by
  eventIntro (DealsCombatDamage n to) = nomIntro to
  eventIntro (BeginningOf _ _) = bs
  eventIntro (Casts _ what) = nomIntro what
  eventIntro (StatusEvent n _) = nomIntro n
  eventIntro DayNightShift = bs
  eventIntro (LastCounterRemoved _ n _) = nomIntro n
  eventIntro (PutInto n _ _) = nomIntro n
  eventIntro (CounterEvent _ _ n OneCounter _ _) = nomIntro n
  eventIntro (CounterEvent _ _ n ManyCounters _ _) =
    outcomeB CountersPut :: nomIntro n
  eventIntro (TokensCreated n _ _ _) = nomIntro n
  eventIntro (ChapterMark _) = bs
  eventIntro (Activates _ what) = nomIntro what
  eventIntro (StatBecomes _ _ v) = amtIntro v
  eventIntro (Regenerates n) = nomIntro n
  eventIntro (NthOccurrence _ ev) = eventIntro ev

  ||| The discourse after the event has happened, read by a trigger's
  ||| effect body: it looks for the object in the zone it moved to
  ||| [CR#603.6]. `eventIntro` is read instead by an interception's
  ||| replacement, since the replaced event never happens [CR#614.6].
  public export
  eventAfter : {bs : Bindings} -> GameEvent bs -> Bindings
  eventAfter (Dies n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (Leaves n) = moveIntro Nothing n Nothing
  eventAfter (IsDestroyed n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (IsDealtDamage {k = Object} to) =
    outcomeB DamageDealt :: selfSubjIntro to
  eventAfter (IsDealtDamage to) = outcomeB DamageDealt :: nomIntro to
  eventAfter (Draws who) = nomIntro who
  eventAfter (LosesGame who) = nomIntro who
  eventAfter (Enters n) = moveIntro Nothing n (Just Battlefield)
  eventAfter (Attacks n NoDefender) = selfSubjIntro n
  eventAfter (Attacks n (OneDefender whom)) = nounDelta whom ++ selfSubjIntro n
  eventAfter (Blocks n Nothing) = selfSubjIntro n
  eventAfter (Blocks _ (Just what)) = nomIntro what
  eventAfter (BecomesBlocked n Nothing) = selfSubjIntro n
  eventAfter (BecomesBlocked _ (Just by)) = nomIntro by
  eventAfter (DealsCombatDamage n to) = outcomeB DamageDealt :: nomIntro to
  eventAfter (Casts _ what) = nomIntro what
  eventAfter (BeginningOf _ whose) = possessorIntro whose
  eventAfter (StatusEvent n _) = selfSubjIntro n
  eventAfter DayNightShift = bs
  eventAfter (LastCounterRemoved _ n _) = selfSubjIntro n
  eventAfter (PutInto n to _) = moveIntro Nothing n (Just (zoneSort to))
  eventAfter (CounterEvent _ _ n OneCounter _ _) = selfSubjIntro n
  eventAfter (CounterEvent _ _ n ManyCounters _ _) =
    outcomeB CountersPut :: selfSubjIntro n
  eventAfter (TokensCreated n _ _ _) = nomIntro n
  eventAfter (ChapterMark _) = bs
  eventAfter (Activates _ what) = nomIntro what
  eventAfter (StatBecomes n _ v) = amtDelta v ++ selfSubjIntro n
  eventAfter (Regenerates n) = selfSubjIntro n
  eventAfter (NthOccurrence _ ev) = eventAfter ev

  public export
  eventSubjectPlur : {bs : Bindings} -> GameEvent bs -> Plurality
  eventSubjectPlur (Dies n) = nounPlur n
  eventSubjectPlur (Leaves n) = nounPlur n
  eventSubjectPlur (IsDestroyed n) = nounPlur n
  eventSubjectPlur (IsDealtDamage to) = nounPlur to
  eventSubjectPlur (Draws who) = nounPlur who
  eventSubjectPlur (LosesGame who) = nounPlur who
  eventSubjectPlur (Enters n) = nounPlur n
  eventSubjectPlur (Attacks n _) = nounPlur n
  eventSubjectPlur (Blocks n _) = nounPlur n
  eventSubjectPlur (BecomesBlocked n _) = nounPlur n
  eventSubjectPlur (DealsCombatDamage n _) = nounPlur n
  eventSubjectPlur (BeginningOf _ _) = OneOf
  eventSubjectPlur (Casts _ what) = nounPlur what
  eventSubjectPlur (StatusEvent n _) = nounPlur n
  eventSubjectPlur DayNightShift = OneOf
  eventSubjectPlur (LastCounterRemoved _ n _) = nounPlur n
  eventSubjectPlur (PutInto n _ _) = nounPlur n
  eventSubjectPlur (CounterEvent _ _ n _ _ _) = nounPlur n
  eventSubjectPlur (TokensCreated n _ _ _) = nounPlur n
  eventSubjectPlur (ChapterMark _) = OneOf
  eventSubjectPlur (Activates who _) = nounPlur who
  eventSubjectPlur (StatBecomes n _ _) = nounPlur n
  eventSubjectPlur (Regenerates n) = nounPlur n
  eventSubjectPlur (NthOccurrence _ ev) = eventSubjectPlur ev

  ||| The context a delayed body reads: the event's own after-discourse
  ||| with the outer clause's targets settled [CR#603.7c,603.3d,601.2c].
  public export
  delayedCtx : {bs : Bindings} -> GameEvent bs -> Bindings
  delayedCtx ev = settleTargets (eventAfter ev)

  public export
  Interceptable : GameEvent bs -> Type
  Interceptable {bs} ev = So (interceptOk (eventName ev))

  ||| The nouns a trigger header writes as a possessive: the attachment
  ||| anaphor the Curses print ("enchanted player's upkeep"), and no other.
  public export
  data PossessorNoun : {0 bs : Bindings} -> Noun bs Player -> Type where
    AttachedPossessor : {0 bs : Bindings} -> {0 w : AttachWord} ->
                        {auto 0 ok : AttachHeadOk w PlayerW} ->
                        PossessorNoun (AttachHost w PlayerW {ok})

  ||| A header carries one possessor: a quantifier word, an anaphoric noun,
  ||| or none.
  public export
  data HeaderPossessor : Bindings -> Type where
    NoPossessor : HeaderPossessor bs
    ByWord : (w : Owner) -> HeaderPossessor bs
    ByNoun : (n : Noun bs Player) ->
             {auto 0 pn : PossessorNoun n} -> HeaderPossessor bs

  public export
  possessorWord : {0 bs : Bindings} -> HeaderPossessor bs -> Maybe Owner
  possessorWord NoPossessor = Nothing
  possessorWord (ByWord w) = Just w
  possessorWord (ByNoun _) = Nothing

  public export
  possessorIntro : {bs : Bindings} -> HeaderPossessor bs -> Bindings
  possessorIntro NoPossessor = bs
  possessorIntro (ByWord w) = possessorB (Just w) ++ bs
  possessorIntro (ByNoun n) = MkBinding TheD Player OneOf PlayerP :: nomIntro n

  public export
  PartTriggerable : {0 bs : Bindings} -> TurnPart -> HeaderPossessor bs -> Type
  PartTriggerable {bs} p h = So (partTriggerOk p)

  public export
  AddedPart : TurnPart -> Type
  AddedPart p = So (partAddable p)

  public export
  data FollowerPart : Maybe TurnPart -> Type where
    NoFollower : FollowerPart Nothing
    MkFollowerPart : {auto 0 ok : So (partAddable p)} ->
                     FollowerPart (Just p)

  public export
  data AnchorPart : Maybe TurnPart -> Type where
    BareAnchor : AnchorPart Nothing
    MkAnchorPart : {auto 0 ok : So (partAddable p)} ->
                   AnchorPart (Just p)

  public export
  data TurnDeixis : Maybe Owner -> Bindings -> Type where
    NoTurnDeixis : {auto 0 ok : So (not (isTurnDeictic w))} -> TurnDeixis w bs
    TurnInScope : {auto 0 ok : countOnes TurnRef bs = 1} ->
                  TurnDeixis (Just ThatTurns) bs

  public export
  isTurnDeictic : Maybe Owner -> Bool
  isTurnDeictic Nothing = False
  isTurnDeictic (Just Yours) = False
  isTurnDeictic (Just ThatPlayers) = False
  isTurnDeictic (Just EachPlayers) = False
  isTurnDeictic (Just EachOpponents) = False
  isTurnDeictic (Just EachYours) = False
  isTurnDeictic (Just AnOpponents) = False
  isTurnDeictic (Just ThatTurns) = True

  ||| Which endpoint words a duration adverbial spells. [CR#611.2a]
  ||| gives a clause any stated duration, so the only refusal is a
  ||| second spelling of an endpoint the adverbial already has.
  public export
  durationOk : {0 bs : Bindings} -> Duration bs -> Bool
  durationOk (UntilEvent ev) = spanEventOk (eventName ev)
  durationOk _ = True

  ||| What a span announces for the statement it governs to read. Only a
  ||| span written BEFORE its statement is in a position to be read, so
  ||| `Throughout` is the one clause row that threads this; `Continuously`
  ||| sits its span at `staticIntro` and announces into nothing.
  public export
  spanIntro : {bs : Bindings} -> Duration bs -> Bindings
  spanIntro ThisTurn = bs
  spanIntro RestOfGame = bs
  spanIntro (Until _) = bs
  spanIntro (ForAsLongAs c) = condIntro c
  spanIntro (UntilEvent ev) = eventIntro ev
  spanIntro (DuringNextTurnOf who) = nomIntro who

  ||| A resolving clause's duration slot: [CR#611.2a] gives a stated
  ||| duration its meaning and gives an unstated one the end of the game.
  public export
  data SpanOk : StaticKind -> Maybe (Duration bs) -> Type where
    SpanUnstated : SpanOk k Nothing
    SpanStated : {auto 0 ok : So (durationOk d)} -> SpanOk k (Just d)

  ||| [CR#603.7b] fires a delayed trigger once "unless it has a stated
  ||| duration", naming "this turn" as an example and not as the list.
  public export
  data DelaySpanOk : Maybe (Duration bs) -> Type where
    DelayOnce : DelaySpanOk Nothing
    DelayFor : {auto 0 ok : So (durationOk d)} -> DelaySpanOk (Just d)

  public export
  data TriggerWindow : Type where
    DuringWindow : (p : TurnPart) -> (w : Maybe Owner) ->
                   {auto 0 hw : WindowOk p w} -> TriggerWindow

  public export
  data Timing : Type where
    AsSorcery : Timing
    AsInstant : Timing
    DuringPart : (p : TurnPart) -> (w : Maybe Owner) ->
                 {auto 0 wk : WindowOk p w} -> Timing
    BeforePoint : (pt : TurnPoint) -> (w : Maybe Owner) ->
                  {auto 0 pk : PointWindowOk pt w} -> Timing

  public export
  data UsageLimit = OncePerTurn | OncePerGame

  public export
  data AltEvent : TriggerWord -> Maybe (GameEvent bs) -> Type where
    NoAlt : AltEvent w Nothing
    OneAlt : {0 e : GameEvent bs} ->
             {auto 0 hn : HeaderNontarget e} -> AltEvent w (Just e)

  ||| The discourse a header's intervening clause and effect read. A lone
  ||| event hands its whole after-discourse (`eventAfter`). A coordination
  ||| fires on EITHER arm, so its tail may read only what the arms say
  ||| alike: where the two after-discourses agree outright — the "blocks
  ||| or becomes blocked" pair announces the one partner phrase from both
  ||| sides — that common announcement is the context, and where they
  ||| differ the sentence cannot say which arm happened, so the tail reads
  ||| the outer discourse bare. Whole agreement (`sameBindings`), not a
  ||| meet: under partial agreement the sentence still cannot say which
  ||| arm happened, so the shared prefix names nothing determinate.
  public export
  headerCtx : {bs : Bindings} -> Maybe (GameEvent bs) -> GameEvent bs -> Bindings
  headerCtx Nothing ev = eventAfter ev
  headerCtx (Just alt) ev =
    if sameBindings (eventAfter ev) (eventAfter alt) then eventAfter ev else bs

  public export
  chapterDefaultsOk : {bs : Bindings} -> (ev : GameEvent bs) ->
                      (alt : Maybe (GameEvent bs)) -> Maybe TriggerWindow ->
                      Maybe UsageLimit -> Maybe (Condition (headerCtx alt ev)) -> Bool
  chapterDefaultsOk (ChapterMark _) Nothing Nothing Nothing Nothing = True
  chapterDefaultsOk (ChapterMark _) _ _ _ _ = False
  chapterDefaultsOk _ _ _ _ _ = True

  public export
  ChapterDefaults : {bs : Bindings} -> (ev : GameEvent bs) -> (alt : Maybe (GameEvent bs)) -> Maybe TriggerWindow -> Maybe UsageLimit -> Maybe (Condition (headerCtx alt ev)) -> Type
  ChapterDefaults {bs} ev alt w l i = So (chapterDefaultsOk ev alt w l i)

  public export
  HeaderNontarget : {bs : Bindings} -> GameEvent bs -> Type
  HeaderNontarget {bs} ev = So (not (anyTargetedAt (eventIntro ev)))
