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

  ||| The placement's two ends, asked of the event-keyed tables the
  ||| retrospective reader asks -- `placementDestOk` and
  ||| `placementOriginOk` are declared in the event vocabulary so that
  ||| this seat and `EventComplement`'s share one answer.
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

  ||| The entry's origin, asked of the same event-keyed table. An entry
  ||| whose origin goes unwritten is the ordinary header, so `Nothing`
  ||| passes.
  public export
  entrySourceOk : {0 bs : Bindings} -> Maybe (EventSource bs) -> Bool
  entrySourceOk Nothing = True
  entrySourceOk (Just src) = lookbackSourceOk Entry src

  public export
  EntrySource : {0 bs : Bindings} -> Maybe (EventSource bs) -> Type
  EntrySource {bs} s = So (entrySourceOk s)

  ||| The result test, "Whenever you roll a 4 or higher": which
  ||| results the header watches for. [CR#706.3a] writes the same three
  ||| forms for a results table's left column -- a single number, a
  ||| two-ended range, a one-ended "N+" -- and the printed headers
  ||| write all three ("a 6", "a 1 or 2", "a 4 or higher"), so the
  ||| ranged test is the quantity vocabulary's literal range and needs
  ||| nothing of its own. The gates are `RollRow`'s: a range whose floor
  ||| tops its ceiling matches no result, and a die is numbered from 1
  ||| [CR#706.1a], so a ceiling of zero matches none either.
  ||| A test is not a third `DiceBatch` arm. The batch is the
  ||| determiner the header writes, and a tested header may write
  ||| either alongside it ("a 4 or higher on a die") or write the range
  ||| in the determiner's place.
  ||| `HighestNatural` is the one test no range spells. [CR#706.2] makes
  ||| the natural result the number on the top face BEFORE any modifier,
  ||| so a modified 20 is a result of 20 and not the natural one; and
  ||| [CR#706.1a] numbers each die from 1 to its own N, so the highest
  ||| natural result is a different number for every die the header may
  ||| watch and is a literal for none of them.
  ||| -- spelling: with `ResultIn`, "a [q]"; with `HighestNatural`,
  ||| "a die's highest natural result".
  public export
  data RollWatch : Bindings -> Type where
    AnyResult : RollWatch bs
    ResultIn : (q : Quantity bs) ->
               {auto 0 nz : NonZeroQ q} ->
               {auto 0 wf : WellFormedQ q} ->
               {auto 0 lt : So (quantLiteral q)} -> RollWatch bs
    HighestNatural : RollWatch bs

  ||| Whether a die-kind narrowing admits the header's result test.
  ||| [CR#706.7] has any effect that refers to a numerical result of a
  ||| die roll -- including one that compares that result to a given
  ||| number -- ignore the rolling of the planar die, and [CR#901.9d]
  ||| repeats it; [CR#901.3a] numbers none of that die's six faces. So a
  ||| planar header has no number to test, and the two tests that read
  ||| one are refused there. The face-valued watch the blank-face line
  ||| wants is not this slot's arm and is not written.
  public export
  watchFitsDie : {0 bs : Bindings} -> RolledDie -> RollWatch bs -> Bool
  watchFitsDie _ AnyResult = True
  watchFitsDie d (ResultIn _) = dieHasResult d
  watchFitsDie d HighestNatural = dieHasResult d

  ||| What a written defender has announced by the time the attackers are
  ||| named. The English writes the defender FIRST -- "attacks you with
  ||| two or more creatures" -- so the attacking group is described in a
  ||| discourse the defender is already in; `agentIntro`'s shape at the
  ||| defender seat.
  public export
  defenderIntro : {bs : Bindings} -> AttackDefender bs -> Bindings
  defenderIntro NoDefender = bs
  defenderIntro (OneDefender m) = nomIntro m

  ||| What an event has announced by the time its second slot is named:
  ||| the acting player when the clause writes one, and nothing when the
  ||| passive leaves it out. `mayCtx`'s shape at the event seat.
  public export
  agentIntro : {bs : Bindings} -> Maybe (Noun bs Player) -> Bindings
  agentIntro Nothing = bs
  agentIntro (Just who) = nomIntro who

  ||| The zone a written patient names, or none where the act writes no
  ||| patient at all. `zoneFits` passes an unwritten zone on either side,
  ||| so the patientless acts and the nouns that name no zone are ungated.
  public export
  patientZone : {bs : Bindings} -> Maybe (Noun bs Object) -> Maybe Zone
  patientZone Nothing = Nothing
  patientZone (Just n) = nounZone n

  ||| The patient a verbed event names, decided by the keyword action's
  ||| own rule: written exactly where that rule performs the act on
  ||| something [CR#701.9a], and left out where it does not
  ||| [CR#701.22a].
  public export
  data VerbPatient : {0 bs : Bindings} -> (v : VerbLabel) ->
                     Maybe (Noun bs Object) -> Type where
    ActOnNothing : {auto 0 np : actPatientOf v = Nothing} ->
                   VerbPatient {bs} v Nothing
    ActOn : {0 n : Noun bs Object} ->
            {auto 0 pk : actPatientOf v = Just Object} ->
            VerbPatient v (Just n)

  ||| A verbed event writes a surface subject: its actor, or -- in the
  ||| passive, which names no actor at all -- its patient. Not both
  ||| dropped: the act would then be announced of no one.
  public export
  data VerbedVoice : {0 bs : Bindings} -> {0 cs : Bindings} ->
                     Maybe (Noun bs Player) -> Maybe (Noun cs Object) ->
                     Type where
    ActiveAct : {0 w : Noun bs Player} -> {0 p : Maybe (Noun cs Object)} ->
                VerbedVoice (Just w) p
    PassiveAct : {0 p : Noun cs Object} -> VerbedVoice Nothing (Just p)

  ||| The event algebra's shape, decided once: composition is carried by
  ||| the slots this vocabulary already has, not by operator constructors.
  ||| The one operator row is `NthOccurrence`. Disjunction is a SEAT slot
  ||| and never a `GameEvent` row: `eventName` is total, so a row would
  ||| leave the classifier naming one of n events, while a seat's only
  ||| name-keyed use (`interceptOk`) distributes over the arms instead.
  ||| `AltEvent` is that slot at the trigger header; `Delayed` and
  ||| `Intercepts` carry the same arm list. It is n-ary because the
  ||| English is, at all three seats. What the arms hand the clause that
  ||| reads them is `sharedCtx`, under that seat's own reader --
  ||| `headerCtx`, `delayedCtx`, `interceptCtx`. Negation is the subject
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
    ||| "Whenever this creature leaves the battlefield", "Whenever one or
    ||| more cards leave your graveyard": the zone left is a SLOT, not the
    ||| battlefield. [CR#603.10a] names the leaves-the-battlefield ability
    ||| and the abilities that trigger when a card leaves a graveyard side
    ||| by side, so the graveyard reading is the rules' own.
    ||| It mirrors `PutInto`'s `from` and takes no zone table of its own:
    ||| [CR#400.1] makes every zone a place objects can be, so there is no
    ||| zone nothing can leave, and the only gate left is that the
    ||| subject's own zone agree with the one written.
    ||| The destination was already unwritten (`eventAfter` moves the
    ||| object nowhere named), so the row now pins neither end.
    ||| The unwritten source is tolerated at its zero: every printed
    ||| header names the zone, and [CR#400.7] makes an object that changed
    ||| zones one that left one.
    ||| -- spelling: "[n] leave(s) [from]"
    Leaves : (n : Noun bs Object) -> (from : Maybe (EventSource bs)) ->
             {auto 0 zn : ZoneFits (nounZone n) (sourceZone from)} -> GameEvent bs
    IsDealtDamage : {k : Kind} -> (to : Noun bs k) ->
                    {auto 0 rk : DamageRecipient to} -> GameEvent bs
    Draws : (who : Noun bs Player) -> GameEvent bs
    LosesGame : (who : Noun bs Player) -> GameEvent bs
    ||| "When this creature enters", "Whenever a creature enters from a
    ||| graveyard", "Whenever a land you control enters from anywhere
    ||| other than your hand": the arrival, with the zone it arrived FROM
    ||| a slot on `PutInto`'s model. [CR#603.6a] gives the event its own
    ||| ability and writes it as putting the permanent ONTO the
    ||| battlefield, so the destination is the event's name and only the
    ||| origin is written; `entryOriginOk` is the table it answers to.
    ||| The subject keeps its battlefield gate, which describes it where
    ||| the event leaves it, and the source is gated on its own.
    ||| -- spelling: "[n] enter(s) [from]"
    Enters : (n : Noun bs Object) -> (from : Maybe (EventSource bs)) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 sk : EntrySource from} -> GameEvent bs
    Attacks : (n : Noun bs Object) ->
              (whom : AttackDefender (nomIntro n)) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              GameEvent bs
    ||| "Whenever you attack with one or more creatures", "Whenever a
    ||| player attacks with five or more creatures": the same attack
    ||| declaration read from the ATTACKING PLAYER's side. [CR#508.3c]
    ||| states this header in the rules' own words and triggers it when a
    ||| creature that player controls is declared as an attacker, so the
    ||| player subject is the rules' own reading and not a paraphrase of
    ||| `Attacks`.
    ||| A second row and not a `Kind` on `Attacks`'s subject: that
    ||| subject carries a battlefield gate, and this one has none.
    ||| The DEFENDER is written here as it is at `Attacks`, and for the
    ||| same rule: [CR#508.1b] has the attacking player announce which
    ||| player, planeswalker or battle each chosen creature is attacking,
    ||| so a header read from that player's side may name it -- "attacks
    ||| YOU with two or more creatures", "attacks A PLANESWALKER YOU
    ||| CONTROL with one or more creatures". One `AttackDefender` and not
    ||| one per attacker: the printed headers name a single defender for
    ||| the whole declaration, and the rule's per-creature announcement is
    ||| what makes that one defender true of each of them.
    ||| It is threaded BEFORE the attackers because the English writes it
    ||| there, so the attacking group is described in a discourse the
    ||| defender is already in.
    ||| It announces those attackers, which the body reads back ("put a
    ||| +1/+1 counter on each of them", "untap up to X lands, where X is
    ||| the greatest power among those creatures").
    ||| -- spelling: "[who] attack(s) [whom] with [attackers]"
    AttacksWith : (who : Noun bs Player) ->
                  (whom : AttackDefender (nomIntro who)) ->
                  (attackers : Noun (defenderIntro whom) Object) ->
                  {auto 0 zn : ZoneFits (nounZone attackers) (Just Battlefield)} ->
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
    ||| "When Fblthp becomes the target of a spell", "Whenever this
    ||| permanent becomes the target of a spell or ability an opponent
    ||| controls": the targeting relation read from the TARGETED side.
    ||| [CR#702.21a] writes this header in the rules' own words as what
    ||| ward means, and [CR#603.2e] makes it a becomes-event -- it happens
    ||| as the target is chosen [CR#115.1] and not while the choice
    ||| stands.
    ||| Same relation as `Predicate`'s `Targets`, spelled from the other
    ||| side, on `BlockerOf`/`BlockedBy`'s model: one pair of gates serves
    ||| both seats, `Targetable` on the side that was targeted [CR#115.1]
    ||| and `Targeter` on the side that did it [CR#115.1a,115.1c,115.1d].
    ||| The subject is kind-polymorphic rather than object-kinded because
    ||| the same rule opens it: the printed headers write "you", "a
    ||| creature you control" and "you or a permanent you control" alike.
    ||| The targeter is announced as well as the subject -- the body reads
    ||| back "that spell's controller" and "put a bounty counter on that
    ||| creature" both -- so the row announces the pair.
    ||| -- spelling: "[n] become(s) the target of [by]"
    BecomesTarget : {k : Kind} -> {kb : Kind} -> (n : Noun bs k) ->
                    (by : Noun (nomIntro n) kb) ->
                    {auto 0 tk : Targetable k} ->
                    {auto 0 tr : Targeter kb} -> GameEvent bs
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
    ||| "If you would flip a coin, instead ...": the flipping ACT as a
    ||| thing that happens, which neither arm of `FlipEvent` names.
    ||| [CR#705.1] makes flipping a coin what an effect instructs, and
    ||| [CR#705.2] reads a winner off that flip only afterwards and only
    ||| where the flipper called it -- so the act happens under both
    ||| readings of the coin, and it is the act a replacement reaches
    ||| [CR#614.1], which no reading of the call can stand in for.
    ||| The subject is a player for [CR#705.2]'s reason, the same one
    ||| `FlipEvent` takes: only the player who flips the coin is
    ||| involved.
    ||| It announces its subject and no coin. [CR#614.6] keeps the
    ||| replaced flip from happening at all, so no coin stands there to
    ||| read, and the replacement writes its own count out ("instead flip
    ||| two coins"). No `DiceBatch` twin: the corpus prints the singular
    ||| determiner alone, where the roll prints both.
    ||| -- spelling: "[who] flip(s) a coin".
    FlipsCoin : (who : Noun bs Player) -> GameEvent bs
    ||| "Whenever you win a coin flip", "Whenever a player wins a coin
    ||| flip": the called reading of a flip as a thing that HAPPENS.
    ||| [CR#705.2] makes it one — the flipper calls the coin and then
    ||| wins or loses the flip — and gives it to that player alone, so
    ||| the subject is a player and the event names no one else. The two
    ||| arms are one row under a `FlipCall` slot lifting through
    ||| `flipEventName`, on `CounterEvent`'s model. It announces its
    ||| subject and no flip: the win is what happened, and the face
    ||| belongs to the effect that instructed the flip.
    ||| -- spelling: "[who] win(s)/lose(s) a coin flip"
    FlipEvent : (who : Noun bs Player) -> (call : FlipCall) -> GameEvent bs
    ||| "Whenever you roll one or more dice", "Whenever you roll a die":
    ||| [CR#706.7] names this event in the rules' own words, so the roll
    ||| is watchable and not merely instructable. It announces the roll's
    ||| number, which the body reads back ("equal to the result") — the
    ||| same mint `RollDice` makes, since [CR#706.2] gives the roll its
    ||| result however the roll was called for.
    ||| The die the header NAMES is its own slot beside the determiner,
    ||| because [CR#706.1] has the rolling instruction specify the kind
    ||| and the count together and a header may repeat either back
    ||| ("one or more planar dice", "one or more six-sided dice", "the
    ||| planar die"). Leaving it out narrows nothing: [CR#706.7] and
    ||| [CR#901.9d] put the planar roll inside the unnarrowed header, so
    ||| `AnyDie` watches every roll there is.
    ||| The planar kind takes no result test, and announces none either
    ||| -- see `watchFitsDie` and `eventAfter`.
    ||| -- spelling: with `ManyDice`, "[who] roll(s) one or more dice";
    ||| with `OneDie`, "[who] roll(s) a die"; the kind, where written,
    ||| sits on the noun ("one or more six-sided dice").
    RollsDice : (who : Noun bs Player) -> (many : DiceBatch) ->
                (die : RolledDie) -> (res : RollWatch bs) ->
                {auto 0 dw : So (watchFitsDie die res)} -> GameEvent bs
    ||| "When a player doesn't pay this enchantment's cumulative upkeep",
    ||| "Whenever you pay this enchantment's cumulative upkeep",
    ||| "Whenever this creature's cumulative upkeep is paid": a stated
    ||| cost's payment as a thing that happens [CR#118.1]. The two
    ||| outcomes are one row under a `PaymentOutcome` slot lifting through
    ||| `paymentEventName`, on `FlipEvent`'s model -- not a general
    ||| negation over events, which the corpus attests at no other event.
    ||| The cost is NAMED, by its keyword and the object that has it,
    ||| never spelled as a `Cost`: a header watches a payment, it does not
    ||| state one. [CR#118.10] applies each payment to one cost, so the
    ||| bearer is singular; its zone is ungated because a keyword ability
    ||| functions from the zones its own rule names [CR#113.6b], and those
    ||| are not always the battlefield.
    ||| The payer is a VOICE and not a second row. The passive names the
    ||| cost as its surface subject and no payer at all, but the same
    ||| event happened: the keyword's own rule fixes who pays it --
    ||| [CR#702.24a] and [CR#702.30a] both say "you" of the permanent's
    ||| controller -- so nothing about the event is left indeterminate by
    ||| the omission. What the omission costs is the announcement: voiced,
    ||| the payer is what the tail reads back ("that player exiles all
    ||| cards from their library"); unvoiced, only the bearer is there to
    ||| read. Both outcomes take either voice; a passive nonpayment is
    ||| unattested and refused by no rule, so it is tolerated at its zero.
    ||| -- spelling: voiced, "[who] pay(s)/doesn't pay [whose]'s
    ||| [keyword]"; unvoiced, "[whose]'s [keyword] is/isn't paid".
    PaysCost : (who : Maybe (Noun bs Player)) -> (out : PaymentOutcome) ->
               (whose : Noun (agentIntro who) Object) -> (kw : KeywordLabel) ->
               {auto 0 kc : KeywordCost kw} ->
               {auto 0 one : nounPlur whose = OneOf} -> GameEvent bs
    ||| "Whenever you pay life" (Font of Agonies): a life payment as a
    ||| thing that happens. Its own row and not a `PaysCost` voice,
    ||| because it names no cost: any payment of life, towards whatever
    ||| cost asked for it, is the event [CR#118.1]. It is also the one
    ||| payment that happens IN a number -- [CR#118.3b] pays life by
    ||| subtracting the indicated amount from a life total and [CR#119.4]
    ||| reads that back as losing that much life -- so the row announces
    ||| that amount for the tail ("put that many blood counters on this
    ||| enchantment"), on `RollsDice`'s model, and the mint is
    ||| [CR#119.4]'s own life loss rather than a second sort naming the
    ||| same number.
    ||| -- spelling: "[who] pay(s) life"
    PaysLife : (who : Noun bs Player) -> GameEvent bs
    ||| "Whenever you discard a card", "Whenever an opponent discards a
    ||| card", "Whenever one or more nonland cards are milled", "Whenever
    ||| you scry": a keyword action as a thing that HAPPENS, named by the
    ||| act rather than by the transition it entails. [CR#701.1] has a
    ||| verb the rules never keyword use its standard English definition,
    ||| so the vocabulary of such acts is open: the label rides the row and
    ||| lifts through `eventName` into `EventName`'s own label-carrying
    ||| arm; what one act needs said about it that another does not is
    ||| said by `verbFacts`, never by a row per verb.
    ||| The patient is the act's own, by that data: [CR#701.9a] discards a
    ||| card and [CR#701.17a] mills cards, while [CR#701.22a] scries a
    ||| NUMBER and carries nothing off, and [CR#701.22b] names the trigger
    ||| on that patientless act outright.
    ||| The actor is a VOICE and not a second row, on `PaysCost`'s model:
    ||| the corpus writes mill passively ("one or more nonland cards are
    ||| milled") and discard actively ("you discard a card"), and no rule
    ||| ties either act to either voice, so both take both. What the
    ||| passive costs is the announcement of who acted.
    ||| The patient is gated to the zone the act's own rule finds it in
    ||| (`actZone`): [CR#701.9a] discards from a hand, [CR#701.8a] and
    ||| [CR#701.21a] take a permanent off the battlefield. That gate is
    ||| what lets this row spell the destruction event outright, so
    ||| DESTROY has no dedicated row beside it: the retired `IsDestroyed`
    ||| said less -- it stamped no participle, and its `Destruction`
    ||| classifier closed the "was destroyed this turn" lookback the
    ||| corpus writes.
    ||| The other two overlaps stand, deliberately. A TAP is two events
    ||| the rules keep apart: [CR#508.1f] has attacking make a creature
    ||| become tapped with no effect performing the act, so `StatusEvent`
    ||| on `Tapped` watches the status [CR#110.5c] and this row watches a
    ||| player's act ("Whenever you tap a land for mana"). A PUT is one
    ||| event two terms reach unequally: `PutInto` writes both ends and
    ||| this row writes neither, since "Put" states no zone of its own --
    ||| so the placement is `PutInto`'s to spell and the verbed reading of
    ||| "Put" is tolerated overgeneration at its printed zero.
    ||| -- spelling: voiced, "[who] [verb](s) [what]"; unvoiced,
    ||| "[what] is/are [participle]".
    VerbedEvent : (who : Maybe (Noun bs Player)) -> (v : VerbLabel) ->
                  (what : Maybe (Noun (agentIntro who) Object)) ->
                  {auto 0 kv : KnownVerb v} ->
                  {auto 0 pt : VerbPatient v what} ->
                  {auto 0 zn : ZoneFits (patientZone what) (actZoneOf v)} ->
                  {auto 0 vc : VerbedVoice who what} -> GameEvent bs
    ||| The ordinal occurrence of an event: "When the fourth plan counter
    ||| is put on this enchantment", "Whenever you cast your first spell
    ||| during each opponent's turn". The ordinal names WHICH occurrence in
    ||| a sequence — a third thing beside `CounterBatch`'s two determiners,
    ||| deliberately not a third batch arm. A wrapper rather than per-event
    ||| twins, so one word serves every countable event; a wrapped wrapper
    ||| ("the third first spell") is tolerated overgeneration. The trigger
    ||| word stays the header's own slot, unconstrained here: an ordinal
    ||| names WHICH occurrence, not how often the header may trigger.
    ||| The RESET is the second slot: "your second card EACH TURN" says
    ||| over what period the occurrences are counted, and the count starts
    ||| again when that period does. It is NOT a `TriggerWindow` and could
    ||| not be one -- a window says WHEN the header may trigger, and
    ||| `windowOk` refuses a bare turn there for exactly the reason this
    ||| slot exists ("during the turn" restricts nothing, because
    ||| [CR#500.1] puts every moment of the game inside some turn). A
    ||| period that bounds a COUNT is a different question from one that
    ||| bounds a moment, and a bare turn answers the first.
    ||| The period is a `TurnPart` and carries no possessor: the printed
    ||| reset is the turn the occurrences fall in, whoever's it is, and a
    ||| header that DOES narrow to some player's turns writes that as its
    ||| window ("your first spell during each opponent's turn"), whose own
    ||| "each" then does the resetting. `Nothing` is the count with no
    ||| stated period, which the plan- and hour-counter headers write:
    ||| [CR#714.2b]'s counters accumulate over the game and the ordinal
    ||| picks one of them absolutely.
    ||| -- spelling: at a counter event, "When the [ord] [kind] counter is
    ||| put on [n]"; at a cast event, "Whenever [who] cast(s) [whose]
    ||| [ord] spell [window]"; the reset, "each [part]".
    NthOccurrence : (ord : Ordinal) -> (per : Maybe TurnPart) ->
                    (ev : GameEvent bs) -> GameEvent bs

  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (Dies _) = Death
  eventName (Leaves _ _) = Departure
  eventName (IsDealtDamage _) = DamageTaken
  eventName (Draws _) = CardDrawn
  eventName (LosesGame _) = GameLoss
  eventName (Enters _ _) = Entry
  eventName (Attacks _ _) = AttackDeclaration
  eventName (AttacksWith _ _ _) = AttackDeclaration
  eventName (Blocks _ _) = BlockDeclaration
  eventName (BecomesBlocked _ _) = BlockedDeclaration
  eventName (DealsCombatDamage _ _) = CombatDamage
  eventName (BeginningOf _ _) = PartBeginning
  eventName (Casts _ _) = SpellCast
  eventName (BecomesTarget _ _) = BecomesTarget
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
  eventName (FlipsCoin _) = CoinFlip
  eventName (FlipEvent _ call) = flipEventName call
  eventName (RollsDice _ _ _ _) = DiceRoll
  eventName (PaysCost _ out _ _) = paymentEventName out
  eventName (PaysLife _) = LifePayment
  eventName (VerbedEvent _ v _) = VerbedAct v
  eventName (NthOccurrence _ _ ev) = eventName ev

  ||| What an event pattern contributes before it happens — its announced
  ||| subject phrase [CR#601.2c]. Read by an interception's replacement,
  ||| whose replaced event never happens [CR#614.6].
  ||| The subject is announced at `selfSubjIntro`, exactly as `eventAfter`
  ||| announces it: a described participant names its referent through its
  ||| own delta, and a DEICTIC one — the source, an attachment's host —
  ||| announces itself, which is what lets "If this creature would be
  ||| destroyed, regenerate IT" (Clergy of the Holy Nimbus) write the
  ||| printed pronoun. [CR#614.6] keeps the event from happening; it does
  ||| not unwrite the phrase the event named.
  public export
  eventIntro : {bs : Bindings} -> GameEvent bs -> Bindings
  eventIntro (Dies n) = selfSubjIntro n
  eventIntro (Leaves n _) = selfSubjIntro n
  eventIntro (IsDealtDamage to) = selfSubjIntro to
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
  eventIntro (DealsCombatDamage n to) = selfSubjIntro to
  eventIntro (BeginningOf _ _) = bs
  eventIntro (Casts _ what) = selfSubjIntro what
  eventIntro (BecomesTarget _ by) = selfSubjIntro by
  eventIntro (StatusEvent n _) = selfSubjIntro n
  eventIntro DayNightShift = bs
  eventIntro (LastCounterRemoved _ n _) = selfSubjIntro n
  eventIntro (PutInto n _ _) = selfSubjIntro n
  eventIntro (CounterEvent _ _ n OneCounter _ _) = selfSubjIntro n
  eventIntro (CounterEvent _ _ n ManyCounters _ _) =
    outcomeB CountersPut :: selfSubjIntro n
  eventIntro (TokensCreated n _ _ _) = selfSubjIntro n
  eventIntro (ChapterMark _) = bs
  eventIntro (Activates _ what) = selfSubjIntro what
  eventIntro (StatBecomes _ _ v) = amtIntro v
  eventIntro (Regenerates n) = selfSubjIntro n
  -- no coin: [CR#614.6] keeps the replaced flip from happening, so the
  -- replacement writes its own count ("instead flip two coins") rather
  -- than reading one back.
  eventIntro (FlipsCoin who) = selfSubjIntro who
  eventIntro (FlipEvent who _) = selfSubjIntro who
  -- the roll it announces is the roll that WOULD happen: a replacement
  -- reads `eventIntro`, and [CR#614.6] keeps the replaced event from
  -- happening at all, so what stands there is the dice the instruction
  -- called for and not any result. That one mention carries both halves
  -- "instead roll that many dice plus one" needs -- the count for
  -- `ThatMuch` and the kind for `ThoseDice` -- because [CR#706.1] has
  -- the roll specify them together. The singular determiner announces
  -- nothing, on `CounterEvent`'s model: "that many" needs a number the
  -- text wrote, and "a die" wrote none.
  -- the announcement is a COUNT of dice and not a result, so the planar
  -- kind announces it like any other: [CR#706.7] withholds the number
  -- from a planar roll, not the dice the instruction called for, and
  -- Ichor Elixir's "that many planar dice plus one" reads exactly this.
  eventIntro (RollsDice who OneDie _ _) = selfSubjIntro who
  eventIntro (RollsDice who ManyDice _ _) = outcomeB DiceRolled :: selfSubjIntro who
  eventIntro (PaysCost _ _ whose _) = selfSubjIntro whose
  eventIntro (PaysLife who) = selfSubjIntro who
  eventIntro (VerbedEvent who _ Nothing) = agentIntro who
  eventIntro (VerbedEvent _ _ (Just what)) = selfSubjIntro what
  eventIntro (NthOccurrence _ _ ev) = eventIntro ev

  ||| The discourse after the event has happened, read by a trigger's
  ||| effect body: it looks for the object in the zone it moved to
  ||| [CR#603.6]. `eventIntro` is read instead by an interception's
  ||| replacement, since the replaced event never happens [CR#614.6].
  public export
  eventAfter : {bs : Bindings} -> GameEvent bs -> Bindings
  eventAfter (Dies n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (Leaves n _) = moveIntro Nothing n Nothing
  eventAfter (IsDealtDamage {k = Object} to) =
    outcomeB DamageDealt :: selfSubjIntro to
  eventAfter (IsDealtDamage to) = outcomeB DamageDealt :: nomIntro to
  eventAfter (Draws who) = nomIntro who
  eventAfter (LosesGame who) = nomIntro who
  eventAfter (Enters n _) = moveIntro Nothing n (Just Battlefield)
  eventAfter (Attacks n NoDefender) = selfSubjIntro n
  eventAfter (Attacks n (OneDefender whom)) = nounDelta whom ++ selfSubjIntro n
  -- the defender stands after it too, on `Attacks`'s model: its mention
  -- is already inside the attackers' own discourse, since the defender
  -- is written first.
  eventAfter (AttacksWith _ _ attackers) = nomIntro attackers
  eventAfter (Blocks n Nothing) = selfSubjIntro n
  eventAfter (Blocks _ (Just what)) = nomIntro what
  eventAfter (BecomesBlocked n Nothing) = selfSubjIntro n
  eventAfter (BecomesBlocked _ (Just by)) = nomIntro by
  eventAfter (DealsCombatDamage n to) = outcomeB DamageDealt :: nomIntro to
  eventAfter (Casts _ what) = nomIntro what
  -- both participants stand after it, on `Attacks`'s model: the tail
  -- reads back the targeter ("that spell's controller loses 5 life") and
  -- the thing targeted ("it phases out") alike.
  eventAfter (BecomesTarget n by) = nounDelta by ++ selfSubjIntro n
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
  -- the flip that DID happen leaves its coin, which the following text
  -- reads by face or by call [CR#705.2].
  eventAfter (FlipsCoin who) = outcomeB CoinFlipped :: nomIntro who
  eventAfter (FlipEvent who _) = nomIntro who
  -- and the roll that DID happen leaves its result, never the dice it
  -- called for: [CR#706.2] makes the number on the die the result of
  -- that roll, and that is the one number a trigger's tail reads back
  -- ("put that many +1/+1 counters on this creature").
  -- except on the planar die, which leaves a face and no number:
  -- [CR#706.7] has every effect referring to a numerical result of a
  -- die roll ignore the planar roll [CR#901.9d], so what stands after
  -- one is the roll itself and nothing to read as a value.
  eventAfter (RollsDice who _ PlanarDie _) = outcomeB PlanarRolled :: nomIntro who
  eventAfter (RollsDice who _ _ _) = outcomeB RollResult :: nomIntro who
  eventAfter (PaysCost _ _ whose _) = nomIntro whose
  eventAfter (PaysLife who) = outcomeB LifeLost :: nomIntro who
  eventAfter (VerbedEvent who _ Nothing) = agentIntro who
  -- the act stamps its patient, so the body may name it back by its
  -- participle ("the milled card"), and leaves it where the act's own
  -- rule puts it [CR#701.9a,701.17a] -- or where it already was, when
  -- that rule moves nothing [CR#701.26a].
  eventAfter (VerbedEvent _ v (Just what)) =
    moveIntro (Just v) what (maybe (nounZone what) Just (actDestOf v))
  eventAfter (NthOccurrence _ _ ev) = eventAfter ev

  public export
  eventSubjectPlur : {bs : Bindings} -> GameEvent bs -> Plurality
  eventSubjectPlur (Dies n) = nounPlur n
  eventSubjectPlur (Leaves n _) = nounPlur n
  eventSubjectPlur (IsDealtDamage to) = nounPlur to
  eventSubjectPlur (Draws who) = nounPlur who
  eventSubjectPlur (LosesGame who) = nounPlur who
  eventSubjectPlur (Enters n _) = nounPlur n
  eventSubjectPlur (Attacks n _) = nounPlur n
  eventSubjectPlur (AttacksWith who _ _) = nounPlur who
  eventSubjectPlur (Blocks n _) = nounPlur n
  eventSubjectPlur (BecomesBlocked n _) = nounPlur n
  eventSubjectPlur (DealsCombatDamage n _) = nounPlur n
  eventSubjectPlur (BeginningOf _ _) = OneOf
  eventSubjectPlur (Casts _ what) = nounPlur what
  eventSubjectPlur (BecomesTarget n _) = nounPlur n
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
  eventSubjectPlur (FlipsCoin who) = nounPlur who
  eventSubjectPlur (FlipEvent who _) = nounPlur who
  eventSubjectPlur (RollsDice who _ _ _) = nounPlur who
  eventSubjectPlur (PaysCost (Just who) _ _ _) = nounPlur who
  -- the passive's surface subject is "[whose]'s [keyword]", whose head
  -- is the already-singular bearer.
  eventSubjectPlur (PaysCost Nothing _ _ _) = OneOf
  eventSubjectPlur (PaysLife who) = nounPlur who
  eventSubjectPlur (VerbedEvent (Just who) _ _) = nounPlur who
  eventSubjectPlur (VerbedEvent Nothing _ (Just what)) = nounPlur what
  -- unreachable: `VerbedVoice` refuses an act with neither actor nor
  -- patient written; folded here rather than left to a catch-all.
  eventSubjectPlur (VerbedEvent Nothing _ Nothing) = OneOf
  eventSubjectPlur (NthOccurrence _ _ ev) = eventSubjectPlur ev

  ||| Whether every arm of a coordination announces exactly what the head
  ||| event announces, under the reader its seat uses.
  public export
  armsAgree : {bs : Bindings} -> (read : GameEvent bs -> Bindings) ->
              Bindings -> List (GameEvent bs) -> Bool
  armsAgree read ds [] = True
  armsAgree read ds (a :: as) = sameBindings ds (read a) && armsAgree read ds as

  ||| What a coordinated event seat hands the clause that reads it. A lone
  ||| event hands its whole discourse. A coordination fires on ANY arm, so
  ||| its reader may take only what the arms say alike: where every arm
  ||| agrees outright with the head that common announcement is the
  ||| context, and where any differs the sentence cannot say which arm
  ||| happened, so the reader takes the outer discourse bare. Whole
  ||| agreement (`sameBindings`), not a meet: under partial agreement the
  ||| sentence still cannot say which arm happened, so the shared prefix
  ||| names nothing determinate.
  ||| The reader is the seat's own: `eventAfter` where the event happened
  ||| [CR#603.6], `eventIntro` where a replacement keeps it from happening
  ||| [CR#614.6].
  public export
  sharedCtx : {bs : Bindings} -> (read : GameEvent bs -> Bindings) ->
              List (GameEvent bs) -> GameEvent bs -> Bindings
  sharedCtx read alts ev =
    if armsAgree read (read ev) alts then read ev else bs

  ||| The context a delayed body reads: the coordination's shared
  ||| after-discourse with the outer clause's targets settled
  ||| [CR#603.7c,603.3d,601.2c].
  public export
  delayedCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  delayedCtx alts ev = settleTargets (sharedCtx eventAfter alts ev)

  ||| The context an interception's replacement reads: the coordination's
  ||| shared announcement, since the replaced event never happens
  ||| [CR#614.6].
  public export
  interceptCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  interceptCtx alts ev = sharedCtx eventIntro alts ev

  public export
  Interceptable : GameEvent bs -> Type
  Interceptable {bs} ev = So (interceptOk (eventName ev))

  ||| The seat's gate distributed over the coordination's arms: the one
  ||| name-keyed reader on any event seat asks each arm the same question
  ||| it asks the head.
  public export
  interceptArmsOk : {0 bs : Bindings} -> List (GameEvent bs) -> Bool
  interceptArmsOk [] = True
  interceptArmsOk (a :: as) = interceptOk (eventName a) && interceptArmsOk as

  public export
  InterceptableArms : {0 bs : Bindings} -> List (GameEvent bs) -> Type
  InterceptableArms as = So (interceptArmsOk as)

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

  ||| The header's coordinated further events: an arm LIST, because the
  ||| English is n-ary -- "attacks, blocks, or becomes the target of a
  ||| spell". Gated arm by arm with the head's own gate, and read back by
  ||| `sharedCtx`. The same slot sits at `Delayed` and at `Intercepts`,
  ||| each carrying the gate that seat puts on its head event; only the
  ||| header's coordination is written with a trigger word.
  public export
  data AltEvent : TriggerWord -> List (GameEvent bs) -> Type where
    NoAlt : AltEvent w []
    MoreAlt : {0 e : GameEvent bs} -> {0 es : List (GameEvent bs)} ->
              {auto 0 hn : HeaderNontarget e} ->
              {auto 0 rest : AltEvent w es} -> AltEvent w (e :: es)

  ||| The discourse a header's intervening clause and effect read: the
  ||| coordination's shared after-discourse. The "blocks or becomes
  ||| blocked" pair announces the one partner phrase from both sides, so
  ||| that common announcement is handed on; Syr Konrad's three arms leave
  ||| the object in three different places, so nothing is.
  public export
  headerCtx : {bs : Bindings} -> List (GameEvent bs) -> GameEvent bs -> Bindings
  headerCtx alts ev = sharedCtx eventAfter alts ev

  public export
  chapterDefaultsOk : {bs : Bindings} -> (ev : GameEvent bs) ->
                      (alts : List (GameEvent bs)) -> Maybe TriggerWindow ->
                      Maybe UsageLimit -> Maybe (Condition (headerCtx alts ev)) -> Bool
  chapterDefaultsOk (ChapterMark _) [] Nothing Nothing Nothing = True
  chapterDefaultsOk (ChapterMark _) _ _ _ _ = False
  chapterDefaultsOk _ _ _ _ _ = True

  public export
  ChapterDefaults : {bs : Bindings} -> (ev : GameEvent bs) -> (alts : List (GameEvent bs)) -> Maybe TriggerWindow -> Maybe UsageLimit -> Maybe (Condition (headerCtx alts ev)) -> Type
  ChapterDefaults {bs} ev alts w l i = So (chapterDefaultsOk ev alts w l i)

  public export
  HeaderNontarget : {bs : Bindings} -> GameEvent bs -> Type
  HeaderNontarget {bs} ev = So (not (anyTargetedAt (eventIntro ev)))
