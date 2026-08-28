||| The clause core: cost, effect, static-effect, and ability layers built
||| over the trigger algebra.
module Experimental.Effect

import public Experimental.Triggers

%default total

mutual
  public export
  record TokenChars (bs : Bindings) where
    constructor MkToken
    pt : Maybe (p : Amount bs ** Amount (amtIntro p))
    colors : List Color
    line : TypeLine
    abilities : List (AbilityAt [])
    name : Maybe String

  public export
  tokenTyped : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenTyped t = lineNonEmpty (MkTypeLine [] t.line.tys)

  public export
  ptWritten : {0 bs : Bindings} -> Maybe (p : Amount bs ** Amount (amtIntro p)) -> Bool
  ptWritten Nothing = False
  ptWritten (Just _) = True

  public export
  tokenPtOk : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenPtOk t = not (elem Creature t.line.tys) || ptWritten t.pt

  public export
  TokenTyped : TokenChars bs -> Type
  TokenTyped {bs} t = So (tokenTyped t)

  public export
  TokenPt : TokenChars bs -> Type
  TokenPt {bs} t = So (tokenPtOk t)

  public export
  SubtypesFit : TokenChars bs -> Type
  SubtypesFit {bs} t = So (subsFitLine t.line.subs t.line.tys)

  public export
  tokenCanonical : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenCanonical t = colorsDistinct t.colors && typesDistinct t.line.tys

  public export
  TokenCanonical : TokenChars bs -> Type
  TokenCanonical {bs} t = So (tokenCanonical t)

  public export
  additionUnnamed : {0 bs : Bindings} -> TokenChars bs -> Bool
  additionUnnamed t = isNothing t.name

  public export
  AdditionUnnamed : TokenChars bs -> Type
  AdditionUnnamed {bs} t = So (additionUnnamed t)

  public export
  someWritten : {0 a : Type} -> List a -> Bool
  someWritten [] = False
  someWritten (_ :: _) = True

  ||| What an addition line has to SAY, asked of the whole bundle where
  ||| `LineNonEmpty` and `AddsSomething` asked the type line alone.
  ||| [CR#613.1d] and [CR#613.1e] are different layers, so "becomes blue
  ||| in addition to its other colors" (Indigo Faerie) adds at layer 5
  ||| and names no type at all. Neither line-level refusal is dropped: a
  ||| line that is WRITTEN must still add a type the subject does not
  ||| already have, and the only bundle admitted without one is a bundle
  ||| that writes a colour. A P/T or a with-clause ability alone is still
  ||| refused -- [CR#613.4b]'s base P/T is `HasBasePt`'s statement and a
  ||| granted ability is `Gains`', and neither is printed without a type
  ||| word in an addition sentence.
  public export
  additionSaysSomething : {0 bs : Bindings} -> Maybe CardType ->
                          TokenChars bs -> Bool
  additionSaysSomething subj t =
    addsSomething subj t.line ||
      (not (lineNonEmpty t.line) && someWritten t.colors)

  public export
  AdditionSaysSomething : Maybe CardType -> TokenChars bs -> Type
  AdditionSaysSomething {bs} subj t = So (additionSaysSomething subj t)


  public export
  tokenHeadTy : {0 bs : Bindings} -> TokenChars bs -> Maybe CardType
  tokenHeadTy t = lastType t.line.tys

  public export
  data TokenSpec : Bindings -> Type where
    TokenWritten : (t : TokenChars bs) ->
                   {auto 0 tt : TokenTyped t} ->
                   {auto 0 tp : TokenPt t} ->
                   {auto 0 sf : SubtypesFit t} ->
                   {auto 0 ta : TokenAbilities t} ->
                   {auto 0 tc : TokenCanonical t} -> TokenSpec bs
    ||| The anaphoric specification: the create clause reads back a
    ||| definition of characteristics an earlier clause wrote [CR#111.3],
    ||| never the objects that definition made. NOT gated on the
    ||| antecedent's plurality -- one token carries a definition as a
    ||| batch does, and English spells the read to agree with what it
    ||| found ("those tokens" after a batch, "that token" after one),
    ||| which is spelling and not a second constructor.
    ||| -- spelling: "[count] of those tokens"; after a singular
    ||| antecedent, "that token".
    TokenAsThose : {auto 0 ok : countTokenSpecs bs = 1} -> TokenSpec bs
    TokenCopyOf : (src : Noun bs Object) -> (exc : List (CopyExcept bs)) ->
                  {auto 0 pm : PerMember src} -> TokenSpec bs

  public export
  specHeadTy : {bs : Bindings} -> TokenSpec bs -> Maybe CardType
  specHeadTy (TokenWritten t) = tokenHeadTy t
  specHeadTy TokenAsThose = tyOfThoseAny TokenW bs
  specHeadTy (TokenCopyOf src _) = nounTy src

  ||| What a written token's P/T amounts mention; a copy's source is left
  ||| out, since the copy clause already names it and a second mention
  ||| would double the pronoun's antecedents.
  public export
  ptDelta : {bs : Bindings} -> Maybe (p : Amount bs ** Amount (amtIntro p)) ->
            List Binding
  ptDelta Nothing = []
  ptDelta (Just (p ** t)) = amtDelta t ++ amtDelta p

  public export
  specDelta : {bs : Bindings} -> TokenSpec bs -> List Binding
  specDelta (TokenWritten t) = ptDelta t.pt
  specDelta TokenAsThose = []
  specDelta (TokenCopyOf _ _) = []

  public export
  data PtShift : Bindings -> Type where
    PtUp : (amt : Amount bs) -> PtShift bs
    PtDown : (amt : Amount bs) -> PtShift bs

  public export
  shiftAmount : {0 bs : Bindings} -> PtShift bs -> Amount bs
  shiftAmount (PtUp a) = a
  shiftAmount (PtDown a) = a

  public export
  shiftRises : {0 bs : Bindings} -> PtShift bs -> Bool
  shiftRises (PtUp _) = True
  shiftRises (PtDown _) = False

  public export
  shiftDelta : {bs : Bindings} -> PtShift bs -> List Binding
  shiftDelta s = amtDelta (shiftAmount s)

  public export
  shiftIntro : {bs : Bindings} -> PtShift bs -> Bindings
  shiftIntro s = amtIntro (shiftAmount s)

  public export
  writtenZero : {0 bs : Bindings} -> Amount bs -> Bool
  writtenZero (Lit Z) = True
  writtenZero _ = False

  ||| Direction agreement, not structural equality: no `Eq` instance.
  public export
  sameDirection : {0 bs : Bindings} -> PtShift bs -> PtShift bs -> Bool
  sameDirection p t = if shiftRises p then shiftRises t else not (shiftRises t)

  public export
  data CostShift : Bindings -> Type where
    CostLess : (amt : Amount bs) -> CostShift bs
    CostMore : (amt : Amount bs) -> CostShift bs

  public export
  costAmount : {0 bs : Bindings} -> CostShift bs -> Amount bs
  costAmount (CostLess a) = a
  costAmount (CostMore a) = a

  namespace Static
    public export
    data StaticEffect : Bindings -> Type where
      ||| The shift amounts sit at `selfSubjIntro n`, not `nomIntro n`:
      ||| the subject is written before them and a deictic one announces
      ||| itself, so "gets +2/+2 for each Aura attached to IT"
      ||| (Auramancer's Guise) reads the enchanted creature its own
      ||| statement named. That is the context `staticIntro` has always
      ||| exported for this row; the slots were the drift.
      Gets : (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
             (tou : PtShift (shiftIntro pow)) ->
             {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
             StaticEffect bs
      ||| `Gets`' move, at the defining amount: "becomes an artifact
      ||| creature with power and toughness each equal to ITS mana value"
      ||| (Titania's Song).
      DefinesPt : (n : Noun bs Object) -> (sl : DefinedSlots) ->
                  (amt : Amount (selfSubjIntro n)) ->
                  {auto 0 sd : SelfDefined n} ->
                  StaticEffect bs
      HasBasePt : (n : Noun bs Object) -> (pow : Amount bs) ->
                  (tou : Amount (amtIntro pow)) -> StaticEffect bs
      SwitchesPt : (n : Noun bs Object) ->
                   {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                   StaticEffect bs
      CostsToCast : {k : Kind} -> (n : Noun bs k) -> (sh : CostShift bs) ->
                    {auto 0 cs : CostSubject n} ->
                    StaticEffect bs
      AltCost : (c : Maybe (Cost bs)) ->
                {auto 0 ap : AltPayment c} -> StaticEffect bs
      ||| The static twin of the clause-level `Define`: "[se], where [l] is
      ||| [amt]" as one member of an `AndAlso` after the statement that used
      ||| the letter.
      ||| -- spelling: as `Define`.
      Define : (l : Letter) -> (amt : Amount bs) ->
               {auto 0 ok : So (anyOpenLetter l bs)} -> StaticEffect bs
      Gains : (n : Noun bs Object) -> (ab : AbilityAt bs) ->
              {auto 0 ok : GrantSubject ab n} ->
              {auto 0 gr : Grantable ab} -> StaticEffect bs
      Deontic : (n : Noun bs Object) -> (c : Compulsion bs) ->
                (deed : Deed) -> (role : Role) ->
                (patient : DeonticPatient {bs = nomIntro n} deed role) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 dp : DeedParticipant deed role (nounTy n)} -> StaticEffect bs
      MayDeclineUntap : (n : Noun bs Object) ->
                        {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                        StaticEffect bs
      OutcomeGate : (k : OutcomeGateKind) -> (who : Noun bs Player) ->
                    StaticEffect bs
      PlayerCant : (act : PlayerAct) -> (who : Noun bs Player) ->
                   StaticEffect bs
      ObjectCant : {k : Kind} -> (act : ObjectAct) -> (what : Noun bs k) ->
                   {auto 0 sub : ActSubject act what} -> StaticEffect bs
      DoesntUntap : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      CantUntapMoreThan : (who : Noun bs Player) -> (k : Nat) ->
                          (p : Predicate bs Object) ->
                          {auto 0 zn : ZoneFits (seedZone p) (Just Battlefield)} ->
                          StaticEffect bs
      Skips : (who : Noun bs Player) -> (part : TurnPart) -> StaticEffect bs
      ||| The type ADDITION [CR#205.1b] at [CR#613.1d]'s layer 4, and the
      ||| colour addition [CR#613.1e] rides the same sentence: what the
      ||| bundle says is asked of the bundle (`AdditionSaysSomething`)
      ||| rather than of its type line, so the one printed colour-only
      ||| addition writes. `TokenCanonical` stands where
      ||| `ColorsDistinct` did: the distinctness the SETTING row demands
      ||| is the addition's too. Naming a card type twice in one line is
      ||| not a second addition -- [CR#205.1b] retains the prior types
      ||| and this line names what is added, so the repeat adds nothing
      ||| the first mention did not, while `tokenHeadTy` reads the head
      ||| off the LAST type and so reads one word twice. Measured
      ||| zero: no supported addition line repeats a card type or a
      ||| colour, and no ORDERING demand exists anywhere in the grammar
      ||| to be asked here or at the setting.
      ||| -- spelling: "[n] is/becomes [added] in addition to its other
      ||| types" (and "… other colors", where the bundle is a colour).
      BecomesAlso : (n : Noun bs Object) -> (added : TokenChars bs) ->
                    {auto 0 sw : AdditionSaysSomething (nounTy n) added} ->
                    {auto 0 af : AddedFits (nounTy n) added.line} ->
                    {auto 0 tc : TokenCanonical added} ->
                    {auto 0 ta : TokenAbilities added} ->
                    {auto 0 un : AdditionUnnamed added} -> StaticEffect bs
      AddsEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      {auto 0 sh : SpaceHosted space (nounTy n)} ->
                      StaticEffect bs
      ||| The quantifier's negative pole: "loses all creature types"
      ||| (4 lines), "loses all land types" (3). [CR#613.1d]'s layer 4
      ||| again, and the same `TypeSpace` payload and host gate as the
      ||| positive pole: [CR#205.1a]'s last sentence -- removing a
      ||| subtype "doesn't affect its card types at all" -- is why the
      ||| host gate survives the loss, the subject still having the card
      ||| type the emptied space belongs to.
      ||| The ability loss printed beside it on all three land lines is
      ||| NOT a rider here: [CR#613.1f] applies ability removal at layer
      ||| 6 where this applies at layer 4, so "loses all land types and
      ||| abilities" is two statements coordinated by `AndAlso`, the
      ||| second of them `LosesAllAbilities`' own row.
      ||| -- spelling: "[n] lose(s) all [space] types".
      LosesEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       {auto 0 sh : SpaceHosted space (nounTy n)} ->
                       StaticEffect bs
      ||| The literal colour change, [CR#613.1e]'s layer 5 under the
      ||| "becomes"/copular verb with no type word in the sentence ("that
      ||| creature becomes green", "All creatures are black"). Its OWN
      ||| row rather than an empty type line on `SetsType`, and the
      ||| asymmetry with the addition above is the operation's own: a
      ||| SETTING that named no type would say the subject's types are
      ||| replaced by none, where an ADDITION of no type adds none and
      ||| the sentence still means what it says. Modeled on
      ||| `SetsChosenQuality`, which writes this same sentence over a
      ||| chosen colour [CR#607.2d]. The payload is the token bundle's
      ||| own colour list, so the empty list is "colorless" [CR#105.2c]
      ||| exactly as it is on a written token. Multi-colour settings are
      ||| a measured zero colour-only ("becomes a blue and red Dragon"
      ||| writes a type line) and are admitted with the list, and the
      ||| printed quantifier "all colors" is `ColorSpec`'s other arm.
      ||| NO subject-zone demand, and here the printed evidence is not
      ||| one line but eight: the lace cycle and Ersatz Gnomes set the
      ||| colour of a SPELL, which is on the stack. Same ground as the
      ||| chosen-quality rows below -- [CR#613.1] applies the layers to
      ||| an object's characteristics and [CR#109.1] makes a spell an
      ||| object.
      ||| -- spelling: "[n] becomes [color]", "[n] is/are [color]",
      ||| "[n] is all colors".
      SetsColor : (n : Noun bs Object) -> (cs : ColorSpec) ->
                  {auto 0 cd : ColorSpecOk cs} ->
                  StaticEffect bs
      SetsType : (n : Noun bs Object) -> (t : TokenChars bs) ->
                 (ret : Maybe CardType) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 {auto 0 ne : LineNonEmpty t.line} ->
                 {auto 0 af : AddedFits (nounTy n) t.line} ->
                 {auto 0 ta : TokenAbilities t} ->
                 {auto 0 tc : TokenCanonical t} ->
                 {auto 0 ro : RetentionOk t.line ret} -> StaticEffect bs
      ||| The two chosen-quality ascriptions carry NO subject-zone
      ||| demand, where the type-line rows beside them do. Decided on
      ||| Ashes of the Fallen, "Each creature card in your graveyard has
      ||| the chosen creature type in addition to its other types": the
      ||| layers apply to an OBJECT's characteristics [CR#613.1] and
      ||| [CR#109.1] makes a card an object, naming no zone, so nothing
      ||| makes a graveyard card's type unchangeable and the demand
      ||| refused printed text. What carries the meaning is `HostedRead` --
      ||| [CR#205.3d] refuses a subtype corresponding to none of the
      ||| object's types -- and it is asked of the subject's TYPE, which
      ||| a zone does not decide. Recorded overgeneration: a library or
      ||| hand subject, neither printed. The type-line rows keep their
      ||| demand, no printed line asking them to drop it.
      AddsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 hr : HostedRead q n} ->
                       StaticEffect bs
      SetsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 hr : HostedRead q n} ->
                       StaticEffect bs
      AlsoOffBattlefield : (se : StaticEffect bs) ->
                           {auto 0 nx : NotExtended se} -> StaticEffect bs
      BecomesCopy : (n : Noun bs Object) -> (src : Noun (nomIntro n) Object) ->
                    (exc : List (CopyExcept (nomIntro src))) ->
                    {auto 0 pm : PerMember src} -> StaticEffect bs
      LosesAllAbilities : (n : Noun bs Object) ->
                          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                          StaticEffect bs
      GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} -> StaticEffect bs
      ||| The cap slot is the trigger rider's word reused, not a third
      ||| `ReplUse` ending: [CR#614.3]'s two endings say how long the
      ||| replacement STANDS, where "the first time ... each turn" says how
      ||| often it may apply while it stands, and the two are written
      ||| together ("The first time you would draw a card each turn ... you
      ||| draw four cards instead"). Redundant beside `NextTimeOnly` rather
      ||| than meaningless, so it carries no gate, on `CreatedByUnder`'s
      ||| model. `OncePerGame` is measured at zero here and admitted with
      ||| the rest of the word.
      ||| -- spelling: "the first time [ev] each turn, [repl] instead".
      ||| The WINDOW slot: "If this creature would untap DURING YOUR UNTAP
      ||| STEP, remove a +1/+1 counter from it instead" (Bewitching
      ||| Leechcraft). [CR#614.1] has a replacement watch for a particular
      ||| event that would happen, and a printed window narrows WHICH
      ||| occurrences of it are watched -- the same narrowing a trigger
      ||| header writes with the same words, so the seat takes
      ||| `TriggerWindow` itself rather than a second window vocabulary,
      ||| and `windowOk`'s refusals carry over unchanged (the bare turn
      ||| included).
      Intercepts : (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
                   (window : Maybe TriggerWindow) ->
                   (repl : Effect (interceptCtx alts ev)) ->
                   (use : ReplUse) ->
                   (limit : Maybe UsageLimit) ->
                   {auto 0 ok : Interceptable ev} ->
                   {auto 0 oks : InterceptableArms alts} -> StaticEffect bs
      Prevents : (kind : DamageKind) -> (size : Shield bs) ->
                 (scope : DamageScope (shieldIntro size)) ->
                 (by : Maybe (Noun (scopeIntro scope) Object)) ->
                 (also : Maybe (Effect (outcomeB DamagePrevented :: byIntro by))) ->
                 StaticEffect bs
      PreventsFrom : (kind : DamageKind) ->
                     (src : DamageAgent bs) ->
                     (scope : DamageScope (agentIntro src)) ->
                     (cut : PreventCut (scopeIntro scope)) ->
                     (use : ReplUse) ->
                     (also : Maybe (Effect (outcomeB DamagePrevented :: cutIntro cut))) ->
                     StaticEffect bs
      Redirects : {k : Kind} -> (kind : DamageKind) -> (size : Shield bs) ->
                  (scope : DamageScope (shieldIntro size)) ->
                  (by : Maybe (Noun (scopeIntro scope) Object)) ->
                  (to : Noun (byIntro by) k) ->
                  {auto 0 rk : DamageRecipient to} ->
                  {auto 0 one : SingleRecipient to} -> StaticEffect bs
      RedirectsFrom : {k : Kind} -> (kind : DamageKind) ->
                      (src : DamageAgent bs) ->
                      (scope : DamageScope (agentIntro src)) ->
                      (to : Noun (scopeIntro scope) k) ->
                      (use : ReplUse) ->
                      {auto 0 rk : DamageRecipient to} ->
                      {auto 0 one : SingleRecipient to} -> StaticEffect bs
      Scales : (kind : DamageKind) -> (src : Noun bs Object) ->
               (scope : DamageScope (nomIntro src)) ->
               (op : DamageScale (scopeIntro scope)) ->
               (use : ReplUse) -> StaticEffect bs
      ||| "If [n] would enter the battlefield under an opponent's
      ||| control, it enters under [who]'s control instead" (Gather
      ||| Specimens): the entry with its CONTROLLER replaced. A dedicated
      ||| row whose body is fixed, on `Redirects`' model, and not an
      ||| entry written into `Intercepts`' body slot: [CR#614.1d] makes a
      ||| line reading "[objects] enter [the battlefield] ..." a
      ||| replacement effect and [CR#614.12] has such an effect modify
      ||| HOW the permanent enters, so what changes is a parameter of the
      ||| one entry rather than a second instruction to put the permanent
      ||| anywhere. The `Effect` vocabulary states no entry for that
      ||| reason -- entering is not an act a player is instructed to
      ||| take, which is why [CR#614.1c,614.1d]'s other entry riders
      ||| (`EntersRider`, `EntersWithCounters`, `EntersChoice`) are
      ||| static rows too.
      ||| The event's own side is written on the subject: the permanent
      ||| that would enter carries the control it would enter under,
      ||| which [CR#614.12] checks as it would exist on the battlefield.
      ||| `Replacement` and not `EntryRider`: those three are a
      ||| permanent's own printed entry riders, standing for as long as
      ||| the permanent does, where this is [CR#614.1a]'s "instead" and
      ||| takes a duration ("this turn").
      ||| -- spelling: "[n] enters under [who]'s control instead".
      EntersUnderInstead : (n : Noun bs Object) ->
                           (who : Noun (nomIntro n) Player) ->
                           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                           {auto 0 ps : SoleHolder who} -> StaticEffect bs
      CantPrevent : (kind : DamageKind) -> (scope : DamageScope bs) ->
                    (by : Maybe (Noun (scopeIntro scope) Object)) -> StaticEffect bs
      Conditionally : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
                      (marking : CondMarking) ->
                      {auto 0 nn : NotConditional se} ->
                      {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      ||| The postposed static conditional, "[se] as long as [c]" / "[se]
      ||| unless [c]": the condition is written after the statement and reads
      ||| the statement's own subject ("has hexproof as long as IT's
      ||| untapped"), so it sits at `staticIntro se`. `Conditionally` is the
      ||| leading twin, whose statement reads the condition; neither is a
      ||| macro over the other and neither reads forward.
      ||| -- spelling: "[se] as long as [c]"; under `NotCond` with `Unless`,
      ||| "[se] unless [c]".
      OnlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
                  (marking : CondMarking) ->
                  {auto 0 nn : NotConditional se} ->
                  {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      MayPlay : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                (verb : PlayVerb) ->
                (from : Maybe (ZoneExpr (nomIntro what))) ->
                (asThough : Maybe PlayAsThough) ->
                (limit : Maybe PlayLimit) ->
                (window : Maybe PlayWindow) ->
                {auto 0 pz : PlaySource (nounZone what) from asThough} ->
                {auto 0 cv : CastableTy verb (nounTy what)} -> StaticEffect bs
      Visibility : (v : ExposeVerb) -> (who : Noun bs Player) ->
                   (what : VisibleThing) ->
                   {auto 0 vo : VisibilityOk v what} -> StaticEffect bs
      ||| The land allowance keeps a literal bound: no printed line writes
      ||| "up to [amt] additional lands", and the statement introduces no
      ||| mention of its own, so an amount bound written here would be
      ||| announced nowhere. Widening waits on a printed line.
      MayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity bs) ->
                               {auto 0 nz : NonZeroQ q} ->
                               {auto 0 wf : WellFormedQ q} ->
                               {auto 0 lt : So (quantLiteral q)} ->
                               StaticEffect bs
      ||| [CR#506.3a] and [CR#508.4d] both say what happens when a
      ||| permanent enters attacking, so either rider is a real entry.
      EntersRider : (n : Noun bs Object) -> (rider : TokenRider) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      ||| "[n] enters with [amt] [kind] counter(s) on it". The kind slot
      ||| takes a printed word or a printed menu alike: [CR#614.12a] has
      ||| the pick made before the permanent enters, so a menu here is
      ||| the same replacement effect [CR#614.1c] spelled with its range
      ||| instead of its word.
      ||| -- spelling: "enters with your choice of a [k1], [k2], or [k3]
      ||| counter on it" (Denry Klin).
      EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                           (kind : CounterKindSource bs) ->
                           (mark : EntryCounterMark) ->
                           StaticEffect bs
      EntersChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      ||| "As [n] becomes attached to [host], choose [q]" -- the choice
      ||| an ATTACHMENT replacement makes. A distinct row beside
      ||| `EntersChoice` and never a relaxation of its gate: [CR#614.1c]
      ||| names the entering event, while [CR#301.5b] says Equipment
      ||| "enter the battlefield like other artifacts. They don't enter
      ||| the battlefield attached to a creature", and [CR#701.3a]'s
      ||| attaching takes the permanent "from where it currently is" and
      ||| puts it onto its host. So the two riders are both replacement
      ||| effects under [CR#614.1]'s general definition -- effects that
      ||| "watch for a particular event that would happen" -- watching
      ||| DIFFERENT events, which is also why this one re-fires on every
      ||| re-equip where the other fires once per entry.
      ||| The host phrase the line prints ("to a creature") is not
      ||| carried: [CR#301.5] fixes an Equipment's host and [CR#301.6] a
      ||| Fortification's outright, and [CR#303.4] hands an Aura's to its
      ||| own enchant ability, so the phrase restates what the subject
      ||| already carries and no line reads it back. No gate asks whether
      ||| the subject CAN attach either:
      ||| that is the `Subtype` catalog's vocabulary, and all three
      ||| supported carriers write an Equipment ascription.
      ||| -- spelling: "As [n] becomes attached to [host], choose [q]"
      ||| (Sanctuary Blade, Psychic Paper).
      AttachChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      AndAlso : {0 n : Nat} -> StaticParts n bs ->
                {auto 0 ne : IsSucc n} -> StaticEffect bs
      ||| "[n] gets +4/+4 and gains trample", "equipped creature gets
      ||| +1/+1 and has flying": VERB PHRASE coordination over ONE
      ||| subject, which English writes by eliding the second verb
      ||| phrase's subject rather than by pronominalising it. The subject
      ||| is written once and every part is predicated of it, so the
      ||| printed line contains no pronoun and this row's slots contain
      ||| no anaphor -- where `AndAlso` coordinates whole STATEMENTS,
      ||| each naming its own subject, and a second statement wanting the
      ||| first one's subject has to read it back.
      |||
      ||| That read is what this row retires. It is not a widening of any
      ||| gate: the shared subject is this construction's own earlier
      ||| argument, which is the forward binder contract's clause 4
      ||| rather than clause 3, and nothing here counts anything
      ||| (`docs/decisions/oracle-text-is-forward-anaphoric.md`).
      ||| 922 supported faces write the two-part form measured
      ||| 2026-08-27 -- 435 "gets [pt] and gains [ab]", 487 "gets [pt]
      ||| and has [ab]" -- and the two spellings are one row, since
      ||| `Gains` already spells both.
      |||
      ||| The parts thread left to right on `StaticParts`' model
      ||| [CR#608.2c], each typed in the previous part's output, and the
      ||| subject's own announcement is made once at the head. The
      ||| per-part obligations are the rows' own, asked of the shared
      ||| subject: a P/T shift wants a battlefield subject, a grant wants
      ||| a subject the ability may be granted to [CR#113.6e].
      ||| -- spelling: "[n] [vp1] and [vp2]", the second verb phrase
      ||| written without a subject.
      OfSubject : {0 k : Nat} -> (n : Noun bs Object) ->
                  (vps : SubjectVPs k (selfSubjIntro n)) ->
                  {auto 0 ne : IsSucc k} ->
                  {auto 0 ok : So (vpsOk (nounZone n) (nounRegime n) vps)} ->
                  StaticEffect bs

  public export
  data Compulsion : Bindings -> Type where
    Forbid : Compulsion bs
    Require : Compulsion bs
    GatedBy : (c : Cost bs) -> Compulsion bs

  ||| The deed's other participant, written or left out. [CR#506.3]
  ||| fixes who that may be: only a player, a planeswalker or a battle
  ||| is attacked, and only a creature attacks or blocks.
  public export
  data DeonticPatient : {0 bs : Bindings} -> Deed -> Role -> Type where
    NoDeonticPatient : DeonticPatient {bs} d r
    ||| What an attack is aimed at [CR#506.3] -- a player, a planeswalker
    ||| or a battle, and under a joined kind a phrase naming either half.
    DefendingPlayer : {k : Kind} -> (m : Noun bs k) ->
                      {auto 0 at : Attackable m} ->
                      DeonticPatient {bs} Attack Agent
    DeonticCounterpart : (m : Noun bs Object) ->
                         {auto 0 dp : DeedParticipant d (counterRole r) (nounTy m)} ->
                         {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                         DeonticPatient {bs} d r

  ||| The gate on both static conditionals: a conditioned statement is not
  ||| conditioned again. It is a NARROWING and not a pin, and it is
  ||| asserted by nothing on purpose. A nested conditional is
  ||| rules-meaningful — [CR#603.4] leaves "if" its normal English meaning
  ||| everywhere it is not an intervening clause — so a proof refusing one
  ||| would refuse no rules impossibility, which is the pin `Effect.If`'s
  ||| round retired for exactly that reason. Nor does the gate merely
  ||| prefer a canonical form: `AndCond` is not the same term, because it
  ||| holds every conjunct at `bs` where a nesting would type the inner
  ||| condition at `condIntro` of the outer and let it read what the outer
  ||| announced. So the gate withholds real width, and no supported line
  ||| pays for it — the eight "as long as … as long as" lines are
  ||| independent statements. It stays until one does.
  public export
  notConditional : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notConditional (Conditionally _ _ _) = False
  notConditional (OnlyWhile _ _ _) = False
  notConditional _ = True


  public export
  NotConditional : StaticEffect bs -> Type
  NotConditional {bs} se = So (notConditional se)

  public export
  data Shield : Bindings -> Type where
    AllOfIt : Shield bs
    TheNext : (amt : Amount bs) -> Shield bs

  public export
  shieldIntro : {bs : Bindings} -> Shield bs -> Bindings
  shieldIntro AllOfIt = bs
  shieldIntro (TheNext amt) = amtIntro amt

  public export
  data DamageScope : Bindings -> Type where
    Everywhere : DamageScope bs
    ToRecipient : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 rk : DamageRecipient n} -> DamageScope bs

  public export
  scopeIntro : {bs : Bindings} -> DamageScope bs -> Bindings
  scopeIntro Everywhere = bs
  scopeIntro (ToRecipient n) = nomIntro n

  public export
  data DamageAgent : Bindings -> Type where
    Unattributed : DamageAgent bs
    DealtBy : (n : Noun bs Object) -> DamageAgent bs

  public export
  agentIntro : {bs : Bindings} -> DamageAgent bs -> Bindings
  agentIntro Unattributed = bs
  agentIntro (DealtBy n) = nomIntro n

  public export
  byIntro : {bs : Bindings} -> Maybe (Noun bs Object) -> Bindings
  byIntro Nothing = bs
  byIntro (Just n) = nomIntro n

  public export
  data PreventCut : Bindings -> Type where
    CutAll : PreventCut bs
    CutSome : (amt : Amount bs) -> PreventCut bs

  public export
  cutIntro : {bs : Bindings} -> PreventCut bs -> Bindings
  cutIntro CutAll = bs
  cutIntro (CutSome amt) = amtIntro amt

  public export
  data DamageScale : Bindings -> Type where
    Multiplied : (f : ScaleFactor) -> DamageScale bs
    Halved : (r : RoundMode) -> DamageScale bs
    Shifted : (d : ShiftDir) -> (amt : Amount bs) ->
              DamageScale bs

  public export
  scaleIntro : {bs : Bindings} -> DamageScale bs -> Bindings
  scaleIntro (Multiplied _) = bs
  scaleIntro (Halved _) = bs
  scaleIntro (Shifted _ amt) = amtIntro amt

  public export
  isCoord : {0 bs : Bindings} -> StaticEffect bs -> Bool
  isCoord (AndAlso _) = True
  isCoord (OfSubject _ _) = True
  isCoord _ = False

  public export
  NotCoord : StaticEffect bs -> Type
  NotCoord {bs} se = So (not (isCoord se))

  ||| "The same is true for …" is a rider on ONE statement, so a second
  ||| extension repeats the first. Nothing else is refused: no rule forbids a
  ||| static effect from naming the cards off the battlefield it also reaches.
  public export
  notExtended : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notExtended (AlsoOffBattlefield _) = False
  notExtended _ = True

  public export
  NotExtended : StaticEffect bs -> Type
  NotExtended {bs} se = So (notExtended se)

  public export
  staticKind : {0 bs : Bindings} -> StaticEffect bs -> StaticKind
  staticKind (Define _ _) = LetterDefinition
  staticKind (Gets _ _ _) = PtDelta
  staticKind (DefinesPt _ _ _) = PtDefinition
  staticKind (HasBasePt _ _ _) = BasePtSet
  staticKind (SwitchesPt _) = PtSwitch
  staticKind (CostsToCast _ _) = CostModification
  staticKind (AltCost _) = CostModification
  staticKind (Gains _ _) = KeywordGrant
  staticKind (Deontic _ _ _ _ _) = DeedRestriction
  staticKind (DoesntUntap _) = DeedRestriction
  staticKind (CantUntapMoreThan _ _ _) = DeedRestriction
  staticKind (Skips _ _) = TurnSkip
  staticKind (MayDeclineUntap _) = DeedRestriction
  staticKind (OutcomeGate _ _) = DeedRestriction
  staticKind (PlayerCant _ _) = DeedRestriction
  staticKind (ObjectCant _ _) = DeedRestriction
  staticKind (BecomesAlso _ _) = TypeAddition
  staticKind (AddsEveryType _ _) = TypeAddition
  staticKind (LosesEveryType _ _) = TypeLoss
  staticKind (SetsColor _ _) = ColorSet
  staticKind (BecomesCopy _ _ _) = CopyEffect
  staticKind (SetsType _ _ _) = TypeSet
  staticKind (AddsChosenQuality _ _) = TypeAddition
  staticKind (SetsChosenQuality _ _) = TypeSet
  staticKind (LosesAllAbilities _) = AbilityLoss
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _ _ _ _) = Replacement
  staticKind (Prevents _ _ _ _ _) = Prevention
  staticKind (PreventsFrom _ _ _ _ _ _) = Prevention
  staticKind (CantPrevent _ _ _) = Prevention
  staticKind (Redirects _ _ _ _ _) = Replacement
  staticKind (EntersUnderInstead _ _) = Replacement
  staticKind (RedirectsFrom _ _ _ _ _) = Replacement
  staticKind (Scales _ _ _ _ _) = Replacement
  staticKind (Conditionally _ _ _) = Conditional
  staticKind (OnlyWhile _ _ _) = Conditional
  staticKind (AlsoOffBattlefield se) = staticKind se
  staticKind (MayPlay _ _ _ _ _ _ _) = PlayPermission
  staticKind (Visibility _ _ _) = VisibilityRider
  staticKind (MayPlayAdditionalLands _ _) = LandAllowance
  staticKind (EntersRider _ _) = EntryRider
  staticKind (EntersWithCounters _ _ _ _) = EntryRider
  staticKind (EntersChoice _ _ _) = EntryRider
  staticKind (AttachChoice _ _ _) = Replacement
  staticKind (AndAlso _) = Coordination
  staticKind (OfSubject _ _) = Coordination


  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (Define l amt) = defineLetter l (amtIntro amt)
  staticIntro (Gets n pow tou) = shiftDelta tou ++ shiftDelta pow ++ selfSubjIntro n
  -- a definition names a number outright, and a sibling slot in the same
  -- statement reads it back as "that number" (Lhurgoyf) — the quantity
  -- anaphor's machinery, spelled "that number" after a definition.
  staticIntro (DefinesPt n _ amt) =
    outcomeB NamedNumber :: (amtDelta amt ++ selfSubjIntro n)
  staticIntro (HasBasePt n pow tou) = amtDelta tou ++ amtDelta pow ++ selfSubjIntro n
  staticIntro (SwitchesPt n) = selfSubjIntro n
  staticIntro (CostsToCast n sh) = amtDelta (costAmount sh) ++ selfSubjIntro n
  staticIntro (AltCost _) = bs
  staticIntro (Gains n _) = selfSubjIntro n
  staticIntro (Deontic n _ _ _ _) = selfSubjIntro n
  staticIntro (DoesntUntap n) = selfSubjIntro n
  staticIntro (CantUntapMoreThan _ _ _) = bs
  staticIntro (Skips _ _) = bs
  staticIntro (MayDeclineUntap n) = selfSubjIntro n
  staticIntro (OutcomeGate _ who) = selfSubjIntro who
  staticIntro (PlayerCant _ who) = selfSubjIntro who
  staticIntro (ObjectCant _ what) = nomIntro what
  staticIntro (BecomesAlso n _) = selfSubjIntro n
  staticIntro (AddsEveryType n _) = selfSubjIntro n
  staticIntro (LosesEveryType n _) = selfSubjIntro n
  staticIntro (SetsColor n _) = selfSubjIntro n
  staticIntro (BecomesCopy n _ _) = selfSubjIntro n
  staticIntro (SetsType n _ _) = selfSubjIntro n
  staticIntro (AddsChosenQuality n _) = selfSubjIntro n
  staticIntro (SetsChosenQuality n _) = selfSubjIntro n
  staticIntro (LosesAllAbilities n) = selfSubjIntro n
  -- the control change is a labeled act of its own: [CR#613.1b]
  -- applies it in layer 2 and [CR#110.2] makes the controller a
  -- property of the permanent, which stays where it stands. So the row
  -- stamps in place rather than moving, on `Search`'s model of a
  -- constructor that names its own label, and "Gain control of target
  -- creature until end of turn. Untap it" reads the stamped mention
  -- back through `ItVerbed "GainControl"` even where the ability's
  -- header announced a permanent of its own.
  staticIntro (GainsControl who what) = stampIntro (Just "GainControl") what
  staticIntro (Intercepts ev alts window repl use limit) = interceptCtx alts ev
  staticIntro (Prevents kind size scope by also) = byIntro by
  staticIntro (PreventsFrom kind src scope cut use also) = cutIntro cut
  staticIntro (CantPrevent kind scope by) = byIntro by
  staticIntro (Redirects kind size scope by to) = nomIntro to
  staticIntro (EntersUnderInstead n who) = nomIntro who
  staticIntro (RedirectsFrom kind src scope to use) = nomIntro to
  staticIntro (Scales kind src scope op use) = scaleIntro op
  staticIntro (Conditionally c se _) = staticIntro se
  staticIntro (OnlyWhile se c _) = staticIntro se
  staticIntro (AlsoOffBattlefield se) = staticIntro se
  staticIntro (MayPlay who what _ _ _ _ _) = selfSubjIntro what
  staticIntro (Visibility _ who _) = nomIntro who
  staticIntro (MayPlayAdditionalLands who _) = nomIntro who
  staticIntro (EntersRider n _) = selfSubjIntro n
  staticIntro (EntersWithCounters n amt _ _) = amtDelta amt ++ selfSubjIntro n
  staticIntro (EntersChoice n _ _) = selfSubjIntro n
  staticIntro (AttachChoice n _ _) = selfSubjIntro n
  staticIntro (AndAlso parts) = partsIntro parts
  staticIntro (OfSubject n vps) = vpsIntro vps

  ||| The choice a STATEMENT binds, for the abilities after it to read
  ||| [CR#607.2d]. A delta rather than a context because a coordination
  ||| binds as many as it has parts: "choose a color and a creature
  ||| type" is ONE sentence of two choices (Riptide Replicator,
  ||| Volrath's Laboratory; "choose a color and an opponent", Call to
  ||| Arms), and what it wanted was the spelling of two choosers in one
  ||| statement, not a second linkage mechanism.
  public export
  staticChoiceDelta : {0 bs : Bindings} -> StaticEffect bs -> List Binding
  staticChoiceDelta (EntersChoice _ q _) = [choiceB q]
  staticChoiceDelta (AttachChoice _ q _) = [choiceB q]
  staticChoiceDelta (AndAlso parts) = partsChoiceDelta parts
  staticChoiceDelta _ = []

  public export
  staticChoiceIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticChoiceIntro se = staticChoiceDelta se ++ bs

  public export
  data DividedVerb : Bindings -> Type where
    DividedDamage : (src : Noun bs Object) -> DividedVerb bs
    DistributedCounters : (kind : CounterKind) -> DividedVerb bs

  public export
  divIntro : {bs : Bindings} -> DividedVerb bs -> Bindings
  divIntro (DividedDamage src) = nomIntro src
  divIntro (DistributedCounters _) = bs

  public export
  data DivTag = DivDamage | DivCounters

  public export
  divTag : {0 bs : Bindings} -> DividedVerb bs -> DivTag
  divTag (DividedDamage _) = DivDamage
  divTag (DistributedCounters _) = DivCounters

  public export
  data DividedTakes : DivTag -> Noun bs k -> Type where
    DamageDivided : {auto 0 rk : DamageRecipient n} -> DividedTakes DivDamage n
    CountersDistributed : DividedTakes DivCounters {k = Object} n

  public export
  data CounterRider : Bindings -> Type where
    MkCounterRider : (amt : Amount bs) -> (kind : CounterKind) ->
                     CounterRider bs

  public export
  data MoveRiders : Bindings -> Type where
    MkMoveRiders : (entry : List TokenRider) ->
                   (ctrl : Maybe (Noun bs Player)) ->
                   (counters : Maybe (CounterRider bs)) ->
                   {auto 0 one : CtrlSingular ctrl} -> MoveRiders bs

  public export
  data CtrlSingular : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoOverride : CtrlSingular Nothing
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlSingular (Just n)

  public export
  fieldRidersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  fieldRidersWritten (MkMoveRiders [] Nothing _) = False
  fieldRidersWritten _ = True

  public export
  counterRiderWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  counterRiderWritten (MkMoveRiders _ _ Nothing) = False
  counterRiderWritten (MkMoveRiders _ _ (Just _)) = True

  public export
  ridersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  ridersWritten r = fieldRidersWritten r || counterRiderWritten r

  public export
  ridersFitZone : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Bool
  ridersFitZone r z =
    not (fieldRidersWritten r) || z == Battlefield

  public export
  RidersFit : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Type
  RidersFit r z = So (ridersFitZone r z)

  public export
  data Cost : Bindings -> Type where
    Mana : (c : ManaCost) -> {auto 0 wr : ManaRun c} -> Cost bs
    ScaledMana : (amt : Amount bs) ->
                 {auto 0 fe : ForEachAmount amt} -> Cost bs
    TapSymbol : Cost bs
    UntapSymbol : Cost bs
    LoyaltySymbol : (s : LoyaltyCost) -> Cost bs
    Do : (e : Effect bs) -> {auto 0 ok : CostAction e} -> Cost bs
    Compound : {0 n : Nat} -> CostSeq n bs ->
               {auto 0 ne : IsSucc n} -> Cost bs
    ||| "its mana cost": the bearer's own printed cost [CR#202.1a], the
    ||| cost a sentence fixing a granted keyword's parameter names.
    ItsManaCost : Cost bs

  public export
  forEachAmount : {0 bs : Bindings} -> Amount bs -> Bool
  forEachAmount (Times _ _) = True
  forEachAmount _ = False

  public export
  ForEachAmount : Amount bs -> Type
  ForEachAmount {bs} a = So (forEachAmount a)

  public export
  costIntro : {bs : Bindings} -> Cost bs -> Bindings
  costIntro (Mana c) = if manaHasX c then letterB X :: bs else bs
  costIntro (ScaledMana _) = bs
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (LoyaltySymbol LoyaltyDownX) = letterB X :: bs
  costIntro (LoyaltySymbol _) = bs
  costIntro (Do e) = effIntro e
  costIntro (Compound cs) = costsIntro cs
  costIntro ItsManaCost = bs

  public export
  isCompound : {0 bs : Bindings} -> Cost bs -> Bool
  isCompound (Compound _) = True
  isCompound _ = False

  public export
  NotCompound : Cost bs -> Type
  NotCompound {bs} c = So (not (isCompound c))

  public export
  isLoyalty : {0 bs : Bindings} -> Cost bs -> Bool
  isLoyalty (LoyaltySymbol _) = True
  isLoyalty _ = False

  public export
  data ProducedMana : Bindings -> Type where
    Runs : (rs : List ProducedRun) ->
           {auto 0 ok : ProducedRuns rs} -> ProducedMana bs
    AnyColor : ColorFreedom -> ProducedMana bs
    OfChosenColor : (alt : Maybe ProducedRun) ->
                    {auto 0 ar : AltRunWritten alt} ->
                    {auto 0 cq : countChoice (QSort Color) bs = 1} ->
                    {auto 0 rd : ChosenQualityRead Color} -> ProducedMana bs

  public export
  data SpendPurpose : Bindings -> Type where
    ToCast : (p : Predicate bs Object) ->
             SpendPurpose bs
    ToActivate : (src : Maybe (Predicate bs Object)) ->
                 SpendPurpose bs

  public export
  data ManaRider : Bindings -> Type where
    SpendOnly : (ps : List (SpendPurpose bs)) ->
                {auto 0 ne : SpendPurposes ps} -> ManaRider bs

  public export
  data SpendPurposes : {0 bs : Bindings} -> List (SpendPurpose bs) -> Type where
    MkSpendPurposes : {0 p : SpendPurpose bs} -> {0 ps : List (SpendPurpose bs)} ->
                      SpendPurposes (p :: ps)

  public export
  data CopyExcept : Bindings -> Type where
    ExceptTypes : (added : TypeLine) ->
                  {auto 0 ne : LineNonEmpty added} -> CopyExcept bs
    ExceptAbility : (ab : AbilityAt []) ->
                    {auto 0 gr : Grantable ab} -> CopyExcept bs
    ExceptThisAbility : CopyExcept bs
    ExceptPt : (pow : Amount bs) -> (tou : Amount (amtIntro pow)) -> CopyExcept bs
    ExceptNonlegendary : CopyExcept bs
    ExceptColor : (c : Chroma.Color) -> CopyExcept bs
    ||| "…, except it enters with [amt] [kind] counters on it": the
    ||| entry-counter clause as a copy modification — the copy's carrier,
    ||| not `EntersWithCounters`', which is a printed static on the
    ||| entering object itself. Counters are not copiable values
    ||| [CR#707.2], so the clause has to ride the copy effect.
    ExceptEntersWithCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                               (mark : EntryCounterMark) -> CopyExcept bs

  public export
  data Repetition : Bindings -> Type where
    Again : Repetition bs
    MoreTimes : (n : Amount bs) ->
                Repetition bs
    AnyNumber : Repetition bs
    ||| "Repeat this process until [c]": the loop bounded by a TEST
    ||| rather than by a count. A condition stands where the count
    ||| stands, so the row sits beside `MoreTimes` rather than wrapping
    ||| it. The condition is read in the context BEFORE the repetition,
    ||| like every other `Repetition` slot -- [CR#608.2c] has the
    ||| instructions followed in the order written, and the process this
    ||| one repeats was written before it, so the loop states no body of
    ||| its own and names no context the earlier text had not named.
    ||| It exports nothing, for `AnyNumber`'s reason: a stopping test is
    ||| not a bound, so no determinate batch stands after the loop.
    ||| -- spelling: "repeat this process until [c]".
    Until : (c : Condition bs) -> Repetition bs

  ||| One striation of a results table: the results it covers and the
  ||| effect they bring about. [CR#706.3a] gives the left column three
  ||| forms — a single number, a two-ended range "N1–N2", a one-ended
  ||| range "N+" — and each means "If the result was in this range,
  ||| [effect]", so the column is the quantity vocabulary's range and
  ||| needs nothing of its own. The gates are that vocabulary's own: a
  ||| range whose floor tops its ceiling covers no result, and a die is
  ||| numbered from 1 [CR#706.1a], so a row ceiling of zero covers none
  ||| either.
  public export
  data RollRow : Bindings -> Type where
    MkRollRow : (results : Quantity bs) -> (e : Effect bs) ->
                {auto 0 nz : NonZeroQ results} ->
                {auto 0 wf : WellFormedQ results} ->
                {auto 0 lt : So (quantLiteral results)} -> RollRow bs

  public export
  rowCount : {0 bs : Bindings} -> List (RollRow bs) -> Nat
  rowCount [] = Z
  rowCount (_ :: rs) = S (rowCount rs)

  ||| One type for verbs and frames together, where the crate splits them and
  ||| rejoins them through a wrapper. `docs/decisions/effect-atom-independence.md`
  ||| does not grade that count: it is scoped to ENGINE atoms, ruling that one
  ||| "depends only on its literal arguments and explicitly bound references"
  ||| and "does not infer meaning from its parent". The `Bindings` index is
  ||| that explicit-reference channel promoted to a type index, so a
  ||| context-reading constructor here is the ADR's sanctioned form rather
  ||| than an exception to it, and the single-type basis is settled by the
  ||| ADR's scope, not owed a boundary by it.
  |||
  ||| One obligation the ADR does own, at lowering. `Enact` and `Does`
  ||| LABEL a body with the keyword action it performs, and the atom
  ||| emitted has to be the BODY -- never an atom that reads its label to
  ||| learn what it does, which is the parent-inference the ADR forbids.
  ||| The open-label shape makes that free: the macro expands the keyword
  ||| action in full ([CR#701.9a] defines discarding AS the move), so the
  ||| label is data a lowering may drop and the wrapper buys nothing.
  public export
  data Effect : Bindings -> Type where
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) ->
                 {auto 0 pm : PerMember to} ->
                 {auto 0 rk : DamageRecipient to} -> Effect bs
    ||| "[src] deals damage equal to its [c] to [to]": `DealDamage` with
    ||| the amount the SOURCE'S OWN characteristic, written as a slot
    ||| rather than as an amount that names a possessor. [CR#120.1] makes
    ||| the object that deals damage the source of it, and the printed
    ||| "its" is that source -- the clause's own subject, which this row
    ||| supplies from its own earlier argument. So no pronoun is read
    ||| here and no candidate is counted: it is the forward binder
    ||| contract's clause 4, not clause 3.
    ||| Measured 2026-08-27: 275 supported faces write "deals damage …
    ||| equal to its [c]", every one of them source-bound, the 15 that
    ||| write the recipient first ("deals damage to itself equal to its
    ||| power") included -- the word order is spelling and the possessor
    ||| is the subject in all 275. 270 write power, 3 mana value, 2
    ||| toughness, so the characteristic is a slot and not a fixed word.
    ||| It does NOT subsume `DealDamage`: a written amount, an anaphoric
    ||| one and a characteristic of something OTHER than the source all
    ||| stay there, and this row is only the reading whose possessor the
    ||| construction already holds.
    ||| -- spelling: "[src] deals damage equal to its [c] to [to]".
    DealDamageOwn : {k : Kind} -> (src : Noun bs Object) -> (c : Characteristic) ->
                    (to : Noun (nomIntro src) k) ->
                    {auto 0 pm : PerMember to} ->
                    {auto 0 rk : DamageRecipient to} -> Effect bs
    Fights : (a : Noun bs Object) ->
             {auto 0 za : OnBattlefield (nounZone a)} ->
             {auto 0 ta : FightParticipant (nounTy a)} ->
             {auto 0 pa : nounPlur a = OneOf} ->
             (b : Noun (nomIntro a) Object) ->
             {auto 0 zb : OnBattlefield (nounZone b)} ->
             {auto 0 tb : FightParticipant (nounTy b)} ->
             {auto 0 pb : nounPlur b = OneOf} -> Effect bs
    SetStatus : {c : StatusCat} -> (v : StatusVal c) -> (n : Noun bs Object) ->
                {auto 0 ok : OnBattlefield (nounZone n)} ->
                {auto 0 at : StatusEffectVal v} -> Effect bs
    RemoveFromCombat : (n : Noun bs Object) ->
                       {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    ||| "[n] blocks [what]" as an instruction: the write that puts a
    ||| creature already on the battlefield into a blocking assignment.
    ||| [CR#506.3g] is the rule that knows the write -- it speaks of "a
    ||| resolving spell or ability [that] would cause a battle to become an
    ||| attacking or blocking creature" -- and [CR#509.1g] is what the
    ||| write makes true. It is not a requirement: "blocks if able" is a
    ||| deontic and leaves the declaration to its step, where this row
    ||| writes the assignment outright.
    ||| Against `RemoveFromCombat`, this is the second half of a reassign,
    ||| and [CR#509.3a] is what keeps the two readings apart. A creature
    ||| removed from combat and written back in "becomes a blocker as the
    ||| result of an effect" when it "wasn't a blocking creature at that
    ||| time", so a "whenever [it] blocks" ability triggers; a creature
    ||| that only `StopsBlocking` never stopped being one, so none does.
    ||| The blocked side is written, never left bare: [CR#509.1g] gives a
    ||| blocking creature the attacking creatures chosen for it, and a
    ||| creature put onto the battlefield blocking is the entry rider's
    ||| business [CR#506.3e].
    ||| -- spelling: "[n] blocks [what]"; after a removal, "[n] then
    ||| blocks [what]".
    BecomesBlocking : (n : Noun bs Object) ->
                      {auto 0 zn : OnBattlefield (nounZone n)} ->
                      {auto 0 dn : DeedParticipant Block Agent (nounTy n)} ->
                      (what : Noun (nomIntro n) Object) ->
                      {auto 0 zw : OnBattlefield (nounZone what)} ->
                      {auto 0 dw : DeedParticipant Block Patient (nounTy what)} ->
                      Effect bs
    ||| "[n] stops blocking [what]": one assignment unwritten and nothing
    ||| else. [CR#506.4] lists what removes a permanent from combat and a
    ||| dropped assignment is not among them; [CR#506.4a] settles the
    ||| neighbouring case the same way, an effect that would have kept a
    ||| creature from blocking not removing it once declared. So the
    ||| creature stays a blocking creature and stays blocking whatever else
    ||| it was blocking -- which is exactly what `RemoveFromCombat` does
    ||| not leave true, and why the two are separate rows.
    ||| The blocked side is written, never left bare: [CR#509.1g] leaves a
    ||| creature a blocking creature "until it's removed from combat or the
    ||| combat phase ends", so a bare "stops blocking" would name the
    ||| removal this row is defined against.
    ||| -- spelling: "[n] stops blocking [what]"
    StopsBlocking : (n : Noun bs Object) ->
                    {auto 0 zn : OnBattlefield (nounZone n)} ->
                    {auto 0 dn : DeedParticipant Block Agent (nounTy n)} ->
                    (what : Noun (nomIntro n) Object) ->
                    {auto 0 zw : OnBattlefield (nounZone what)} ->
                    {auto 0 dw : DeedParticipant Block Patient (nounTy what)} ->
                    Effect bs
    ||| "[n] is attacking [whom]": the attacking twin of
    ||| `BecomesBlocking`, for a permanent already on the battlefield.
    ||| `EntersAttacking` is the rider on entry and names no defender;
    ||| this row names one, through the same `AttackDefender` slot the
    ||| declaration event takes, so [CR#506.3]'s closed set and
    ||| [CR#508.1b]'s one-defender rule ride it unchanged.
    ||| It is an assignment, not a declaration: [CR#508.3a] keys attack
    ||| triggers on a creature being "declared as an attacker", which a
    ||| resolving effect never does.
    ||| -- spelling: "[n] is attacking [whom]"
    BecomesAttacking : (n : Noun bs Object) ->
                       {auto 0 zn : OnBattlefield (nounZone n)} ->
                       {auto 0 dn : DeedParticipant Attack Agent (nounTy n)} ->
                       (whom : AttackDefender (nomIntro n)) -> Effect bs
    Regenerate : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 Effect bs
    CantBe : {k : Kind} -> (e : Effect bs) -> (act : ObjectAct) ->
             (what : Noun (riderIntro e) k) ->
             {auto 0 rd : So (riderAct act)} ->
             {auto 0 sub : ActSubject act what} -> Effect bs
    ||| The warrant tells the bare instruction from a keyword's expansion
    ||| body. A conferral may state how long it lasts: saddle's expansion
    ||| writes "until end of turn" [CR#702.171a] and ascend's and
    ||| storied's write "for the rest of the game"
    ||| [CR#702.131a,702.195a], while monstrosity and renown write none
    ||| [CR#701.37a,702.112a].
    GainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                       (w : GivingWarrant d) ->
                       (span : Maybe (Duration (nomIntro n))) ->
                       {auto 0 sc : designationScope d = HeldBy k} ->
                       {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
    GameBecomes : (d : Designation) ->
                  {auto 0 sc : designationScope d = HeldByGame} ->
                  {auto 0 at : So (designationGiven d)} -> Effect bs
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Effect bs
    GameDrawn : Effect bs
    Choose : {k : Kind} -> (n : Noun bs k) ->
             (by : Maybe (Noun bs Player)) ->
             {auto 0 ch : ChoiceClause by n} -> Effect bs
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           (riders : MoveRiders (nomIntro what)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 pl : Placeable (nounTy what) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} -> Effect bs
    ||| "Counter target spell", "counter target activated ability",
    ||| "counter that spell or ability": [CR#701.6a] counters a spell or
    ||| an ability alike, so the subject is kind-indexed and `Counterable`
    ||| carries the per-kind demand -- the stack for a spell [CR#112.1],
    ||| no zone for an ability the kind places nowhere.
    CounterSpell : {k : Kind} -> (what : Noun bs k) ->
                   {auto 0 ct : Counterable what} -> Effect bs
    CopyStack : (agent : Noun bs Player) ->
                (what : Noun (nomIntro agent) Object) ->
                (times : Amount (nomIntro what)) ->
                (exc : List (CopyExcept (amtIntro times))) ->
                {auto 0 zn : OnStack (nounZone what)} ->
                Effect bs
    ChooseNewTargets : (what : Noun bs Object) ->
                       {auto 0 zn : OnStack (nounZone what)} -> Effect bs
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    AddMana : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
              (prod : ProducedMana (amtIntro amt)) ->
              (riders : List (ManaRider (amtIntro amt))) ->
              Effect bs
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           Effect bs
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Effect bs
    ||| "Search [scope] for [q] [description]" [CR#701.23a]. The count is
    ||| a slot because printed text writes it -- "up to two basic land
    ||| cards", "a number of Plains cards equal to the difference"
    ||| (Boreas Charger), "up to X basic land cards" (Celebrate the
    ||| Harvest) -- and a bare "a card" is `exactly 1`, which is what the
    ||| row spelled before the slot existed. It rides beside the
    ||| description rather than inside it for `CountedGroup`'s reason: a
    ||| description takes no count, and a group-level constraint ("with
    ||| different names") has to have a group to ride.
    ||| What the clause finds is announced at the count's own plurality,
    ||| so the sentence that says where the cards GO -- a separate clause,
    ||| since [CR#701.23e] makes even the reveal separate -- can name them
    ||| as "those cards", "them", "one of them" or "the rest".
    ||| -- spelling: "[who] search[es] [scope] for [q] [description]".
    Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) ->
             (q : Quantity (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} ->
             {auto 0 zf : ZoneFree p} -> Effect bs
    Shuffle : (whose : Noun bs Player) -> Effect bs
    ||| "Flip a coin", "Flip five coins", "target player flips a coin":
    ||| [CR#705.1]'s instruction and nothing more. The clause says a coin
    ||| is flipped; who calls it, which side lands up and who won are the
    ||| rule's business, and the two readings a later clause may take of
    ||| the flip are `FlipCalled` and `FlipFace`. The count is a slot
    ||| because printed text writes it ("Flip X coins"), and a bare flip
    ||| is `Lit 1`.
    ||| It introduces the flip so those readings have something to name.
    ||| The count slot is a `FlipScope`, so the same row writes the flip
    ||| made FOR each member of a described set ("flip a coin for each
    ||| creature that isn't a Demon, Devil, or Imp"): [CR#705.1] leaves a
    ||| coin owned by no referent and [CR#705.2] gives the flip to whoever
    ||| flips it, so the per-member arm changes how many coins there are
    ||| and never who the subject is.
    ||| -- spelling: "[who] flip[s] [count]"; with `You` in the subject
    ||| slot, the imperative "Flip a coin."
    FlipCoins : (who : Noun bs Player) -> (count : FlipScope (nomIntro who)) ->
                Effect bs
    ||| "Roll a d20", "Roll two six-sided dice", "roll that many dice":
    ||| [CR#706.1]'s instruction, which "will specify what kind of die to
    ||| roll and how many of those dice to roll" — so both are written
    ||| arguments and neither has a default. The kind is a `DieSides`,
    ||| which carries [CR#706.1a]'s positivity on its written arm and,
    ||| on its anaphoric one, the kind an announced roll already named —
    ||| what "instead roll that many dice plus one" writes over a roll it
    ||| replaces.
    ||| It introduces the roll's number, which "the result" reads
    ||| [CR#706.2] and a results table ranges over [CR#706.3a].
    ||| -- spelling: "[who] roll[s] [count] [sides]"
    RollDice : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
               (sides : DieSides (amtIntro count)) -> Effect bs
    ||| The results table [CR#706.3]: the striations that read the roll
    ||| the text has already made. A separate clause rather than a slot on
    ||| `RollDice`, because [CR#706.3b] binds the roll, "any additional
    ||| instructions based on the result of the roll, and the associated
    ||| results table" into one ability WITHOUT binding them into one
    ||| sentence — a modifier clause may stand between — and because
    ||| [CR#706.4] has rolls that carry no table at all. Its gate is the
    ||| roll's number, so a table with no roll before it is unwritable.
    ||| Only one striation happens [CR#706.3a], so like a mode list the
    ||| table introduces nothing for later text to read.
    ||| -- spelling: each row on its own line, "1—9 | [effect]"
    ResultsTable : (rows : List (RollRow bs)) ->
                   {auto 0 ne : IsSucc (rowCount rows)} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "and ignore the lower roll", "and ignore all but the highest
    ||| roll", "and ignore one": the instruction that sets aside some of
    ||| what the same clause has just randomised. [CR#706.6] gives it its
    ||| meaning outright -- an ignored roll "is considered to have never
    ||| happened", no ability triggers because of it and no effect
    ||| applies to it -- and settles the tie the superlative can leave.
    ||| Its own clause and not a slot on `RollDice`, on `ResultsTable`'s
    ||| ground: the roll and what is done with its results are one
    ||| ability [CR#706.3b] without being one node, and a roll may carry
    ||| no ignore at all.
    ||| It leaves what it ignores mentioned rather than reminting it: the
    ||| rolls that survive are the same roll the clause named, which is
    ||| what a following `ResultsTable` or `TheResult` reads.
    ||| Outcomes and not rolls alone. The word is printed over each of
    ||| the three randomisers this vocabulary has -- a roll (Berserker's
    ||| Frenzy), a coin (Krark's Thumb's "instead flip two coins and
    ||| ignore one") and the planar die (Ichor Elixir) -- and no rule
    ||| makes it meaningless on the two [CR#706.6] does not name. The gate
    ||| is asked per arm (`ignorableFor`), because the superlative arms
    ||| stay roll-shaped: `RollExtreme` names the ends of an order, and
    ||| neither a coin's two faces [CR#705.1] nor the planar die's
    ||| [CR#901.3a] are ordered.
    ||| -- spelling: "ignore the lowest/lower roll"; with `IgnoreAllBut`,
    ||| "ignore all but the highest roll"; with `IgnoreChosen`,
    ||| "ignore [n]" or "[who] choose(s) [n] of those rolls to ignore".
    IgnoreOutcomes : (which : IgnoredOutcomes bs) ->
                     {auto 0 ok : So (ignorableFor which)} -> Effect bs
    ||| "increase or decrease the result by 1": [CR#706.2]'s modifier,
    ||| the one the rule says may "come from other sources" rather than
    ||| riding the rolling instruction itself. The direction is not a
    ||| slot: every printed line writes the disjunction whole and leaves
    ||| the choice to whoever applies it, and no line writes a
    ||| one-directional shift, so a direction word here would name
    ||| nothing the corpus says. Gated on the roll like every other read
    ||| of it, and it mints nothing: [CR#706.2] makes the shifted number
    ||| the result of that same roll, which `TheResult` already names.
    ||| -- spelling: "increase or decrease the result by [amt]".
    ShiftResult : (amt : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "Roll the planar die": Planechase's own die as a written
    ||| instruction. Its own row and never a `RollDice` with a side
    ||| count, because [CR#901.3a] gives the planar die one Planeswalker
    ||| face, one chaos face and four blanks, which is not the die
    ||| [CR#706.1a] describes ("N equally likely outcomes, numbered from
    ||| 1 to N"). It announces no number for that reason: [CR#706.7] says
    ||| any effect referring to a numerical result of a die roll "ignores
    ||| the rolling of the planar die", so `TheResult`, `TheTotal` and
    ||| `ResultsTable` are inapplicable to it by rule and this row mints
    ||| nothing for them to read.
    ||| The count IS written, and is a slot for [CR#706.1]'s reason on
    ||| `RollDice`'s model: Ichor Elixir's "instead roll that many planar
    ||| dice plus one" writes it as an anaphor over the roll it replaces.
    ||| A bare "Roll the planar die." is `Lit 1`, as a bare flip is.
    ||| The count is the whole of what this row takes from [CR#706.1]:
    ||| there is no kind slot beside it, because [CR#901.3a]'s die is not
    ||| the numbered one [CR#706.1a] describes and naming it IS naming
    ||| the row.
    ||| It announces the roll as `PlanarRolled` and never as a result:
    ||| the mention carries no value (`outcomeIsQuantity` is False), so
    ||| `TheResult`, `TheTotal` and `ResultsTable` stay inapplicable by
    ||| [CR#706.7] while "ignore one" still has the planar dice to name.
    ||| The special action [CR#116.2i,901.9] is not this row. That one is
    ||| granted to a player by the Planechase rules and is written on no
    ||| card; this is the instruction a card's own ability gives.
    ||| -- spelling: "[who] roll[s] [count] planar dice", and with
    ||| `Lit 1` the singular "[who] roll[s] the planar die".
    RollPlanarDie : (who : Noun bs Player) ->
                    (count : Amount (nomIntro who)) -> Effect bs
    ||| "chaos ensues": the instruction beside the die face. [CR#311.7]
    ||| admits it outright — a chaos ability triggers "if the chaos symbol
    ||| is rolled on the planar die, if a resolving spell or ability says
    ||| that chaos ensues, or if a resolving spell or ability states that
    ||| chaos ensues for a particular object" — so a card may say it
    ||| without a planar roll anywhere in the sentence, which is how both
    ||| printed lines write it.
    ||| No subject and no slot. The rule's middle clause gives the
    ||| instruction no participant, and the object-scoped third clause is
    ||| written by no supported line. It mints nothing: what ensues is a
    ||| trigger on the plane card [CR#311.7], not a phrase this sentence
    ||| can go on to read.
    ||| -- spelling: "chaos ensues".
    ChaosEnsues : Effect bs
    ||| "and store those results on it": [CR#706.8a]'s noting of both the
    ||| kind of die rolled and the result of each roll. Gated on the roll
    ||| the same clause made, since those results are what it stores.
    ||| The holder is singular because a stored result is stored ON a
    ||| permanent and the rule notes it there.
    ||| -- spelling: "store those results on [on]".
    StoreResults : (on : Noun bs Object) ->
                   {auto 0 one : nounPlur on = OneOf} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ||| "reroll any number of this creature's stored results":
    ||| [CR#706.8b]'s rerolling, which rolls one die of each noted kind
    ||| and stores the new results in the old ones' place. No gate on a
    ||| roll: the rolls it repeats are the ones already noted on the
    ||| holder, which is why the rule can define it without a roll in the
    ||| sentence, and [CR#706.8c] links this ability to the one that
    ||| stored them. It mints nothing either -- the new results are
    ||| stored, not left to read -- so `TheResult` finds nothing here.
    ||| -- spelling: "[who] reroll[s] [q] of [whose]'s stored results".
    RerollStored : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                   (whose : Noun (nomIntro who) Object) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 one : nounPlur whose = OneOf} -> Effect bs
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 sp : SpanOk (staticKind se) span} ->
                   {auto 0 cl : ClauseStatic se} -> Effect bs
    ||| The leading span, "During [span], [se]": the span is written first
    ||| and the statement reads the phrase it names -- "During TARGET
    ||| OPPONENT's next turn, creatures THAT PLAYER controls attack ... if
    ||| able". `Continuously` is the postposed twin, whose span sits at
    ||| `staticIntro se` and so announces into nothing; a span naming a
    ||| target announces it like any other phrase [CR#601.2c], and no
    ||| statement reads forward. Neither is a macro over the other, on the
    ||| model of `Conditionally`/`OnlyWhile`.
    ||| -- spelling: "During [span], [se]."
    Throughout : (span : Duration bs) ->
                 (se : StaticEffect (spanIntro span)) ->
                 {auto 0 sp : SpanOk (staticKind se) (Just span)} ->
                 {auto 0 cl : ClauseStatic se} -> Effect bs
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (spec : TokenSpec (amtIntro count)) -> (riders : List TokenRider) ->
             Effect bs
    GetsEmblem : (who : Noun bs Player) -> (abl : List (AbilityAt [])) ->
                 {auto 0 ea : EmblemAbilities abl} -> Effect bs
    ||| "Put [amt] [kind] counter(s) on [on]". The kind slot says how the
    ||| clause gives the kind -- the printed word, or a printed menu its
    ||| own "you" [CR#109.5] picks an arm of. Same verb and same gates
    ||| either way, so the menu widens the slot rather than doubling the
    ||| row; `PutCountersOfThoseKinds` is the contrast, where no kind is
    ||| given at all.
    ||| -- spelling: "put your choice of a [k1], [k2], or [k3] counter on
    ||| [on]" (Me, the Immortal).
    PutCounters : (amt : Amount bs) -> (kind : CounterKindSource bs) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 sc : CounterSourceScope kind Object} -> Effect bs
    Distribute : {k : Kind} -> (v : DividedVerb bs) ->
                 (amt : Amount (divIntro v)) ->
                 (among : Noun (amtIntro amt) k) ->
                 {auto 0 gm : GroupMention among} ->
                 {auto 0 tk : DividedTakes (divTag v) among} -> Effect bs
    RemoveCounters : (amt : Amount bs) -> (kind : Maybe CounterKind) ->
                     (from : Noun (amtIntro amt) Object) ->
                     {auto 0 kn : CounterKindNamed Object kind} ->
                     {auto 0 cm : CounterMemory from} -> Effect bs
    ||| "Move [amt] [kind] counter(s) from [src] onto [dst]": the
    ||| two-holder transfer verb [CR#122.5], which is a remove and a put
    ||| taken together. The kind is a `Maybe`, as everywhere: the slot
    ||| says whether the sentence NAMES a kind. The source carries
    ||| `CounterMemory` (a referent that moved zones has no counters left
    ||| to give [CR#122.2,400.7]) and the destination distributes like a
    ||| put, under `MoveDestination` — [CR#122.5] lists the same-object
    ||| case among the ones that make a move impossible, and a bare
    ||| readback at `dst` is that case written down. Its other listed
    ||| cases are engine questions about a particular game state, so no
    ||| zone gate: the rule bounds the move by whether each half can
    ||| happen, never by fixing one zone this row could name.
    ||| -- spelling: "move [amt] [kind] counter(s) from [src] onto [dst]".
    MoveCounters : (amt : Amount bs) -> (kind : Maybe CounterKind) ->
                   (src : Noun (amtIntro amt) Object) ->
                   (dst : Noun (nomIntro src) Object) ->
                   {auto 0 kn : CounterKindNamed Object kind} ->
                   {auto 0 cm : CounterMemory src} ->
                   {auto 0 md : MoveDestination dst} ->
                   {auto 0 pm : PerMember dst} -> Effect bs
    ||| The counter COPY-read: as many counters of each kind as [src]
    ||| holds are put on [dst], and [src] keeps its own. [CR#122.8] and
    ||| [CR#122.9] write this operation out in rules text and say in so
    ||| many words that it is NOT a move [CR#122.5] -- "the player puts
    ||| the same number of each kind of counter the first object had onto
    ||| the second object". That is the whole gate story: [CR#122.5]'s
    ||| impossibility list bounds a move, so `src` carries no
    ||| `CounterMemory` (the rule is written FOR a source that has left
    ||| the battlefield, and `CountersOn` already reads a dead referent's
    ||| counters) and `dst` no `MoveDestination`. Kind-blind and
    ||| amount-fixed: what `src` holds is both the kinds and the counts,
    ||| so neither is a slot. A plural `src` is tolerated -- nothing in
    ||| [CR#122.1] makes summing two holders' counters meaningless -- and
    ||| nothing writes one.
    ||| -- spelling: "put its counters on [dst]"; where an intervening
    ||| "if" has already named the source, "put the same number of each
    ||| kind of counter on [dst]" (Denry Klin) -- one node, two spellings.
    PutSameCounters : (src : Noun bs Object) ->
                      (dst : Noun (nomIntro src) Object) ->
                      {auto 0 pm : PerMember dst} -> Effect bs
    ||| The distributive counter-kind anaphor: the replacement body that
    ||| puts a derived number of counters OF THE ANNOUNCED BATCH'S KINDS on
    ||| its recipient. Presupposes exactly one announced batch, the same
    ||| way `PreventedThisWay` presupposes its outcome.
    ||| -- spelling: "that many plus one of each of those kinds of
    ||| counters are put on [dst]" (Pir, Doc Samson); with no per-kind
    ||| wording, "twice that many of those counters" (Doubling Season) —
    ||| one node, two spellings, since both distribute per kind.
    PutCountersOfThoseKinds : (amt : Amount bs) ->
                              (on : Noun (amtIntro amt) Object) ->
                              {auto 0 pm : PerMember on} ->
                              {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                              Effect bs
    GetsCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                   (kind : CounterKind) ->
                   {auto 0 sc : counterScope kind = Player} -> Effect bs
    ||| `PutCountersOfThoseKinds`' player-side twin: the same kind-blind
    ||| distributive written under the player's own verb. [CR#122.1]
    ||| places a counter on an object OR a player, so a batch a player
    ||| would get has kinds to range over exactly as an object's does,
    ||| and the same single-announcement presupposition holds.
    ||| -- spelling: "you get that many plus one of each of those kinds
    ||| of counters instead" (Winding Constrictor).
    GetsCountersOfThoseKinds : (who : Noun bs Player) ->
                               (amt : Amount (nomIntro who)) ->
                               {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                               Effect bs
    ||| The SELF-reading kind-blind distributive: each recipient is given
    ||| one more counter of every kind it already carries. [CR#701.34a]
    ||| is the rule -- proliferate gives each chosen permanent or player
    ||| "one additional counter of each kind that permanent or player
    ||| already has" -- and it is what tells the kind-blind rows apart:
    ||| `PutCountersOfThoseKinds` and `GetsCountersOfThoseKinds` range
    ||| over an ANNOUNCED batch's kinds and presuppose one,
    ||| `PutSameCounters` reads a DIFFERENT holder's counters, and this
    ||| row reads its own recipient's with nothing announced anywhere.
    ||| Amount-fixed, on `PutSameCounters`' model: the rule gives one per
    ||| kind, and no printed line writes another number -- "proliferate
    ||| twice" and "proliferate X times" repeat the whole action, which
    ||| is `Repeated`.
    ||| Kind-indexed at [CR#122.1]'s own pair, a marker placed on an
    ||| object or a player, because the choice names both halves at once
    ||| and the giving clause distributes over the union. An ability on
    ||| the stack is an object too [CR#109.1] and is excluded anyway:
    ||| every counter kind `counterScope` names sits on a permanent, a
    ||| card or a player, and no printed line counters an ability.
    ||| A recipient holding no counters is given nothing, which is the
    ||| rule applied rather than a defect -- [CR#701.34a] asks for a
    ||| counter in the CHOICE, so no gate repeats the demand here.
    ||| -- spelling: "give each one additional counter of each kind that
    ||| [n] already has"; the reminder text's "give each another counter
    ||| of each kind already there" is the same node.
    GiveCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                             {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                             {auto 0 pm : PerMember on} -> Effect bs
    ||| The MULTIPLICATIVE twin of the self-reading distributive: every
    ||| kind of counter the recipient already carries is doubled. It is
    ||| the same self-reading -- the recipient's own counters are both
    ||| the kinds and the counts, with nothing announced anywhere -- and
    ||| it is NOT [CR#701.34a]'s giving: proliferate adds ONE per kind
    ||| whatever the holder has, and this adds however many the holder
    ||| already had. That is why it is a row beside
    ||| `GiveCountersOfOwnKinds` rather than an operation slot on it: the
    ||| two differ in the arithmetic, not in a direction, and the
    ||| REMOVING direction the slot would also have to serve is at a
    ||| measured zero over the supported corpus.
    ||| Amount-fixed for `GiveCountersOfOwnKinds`' reason, measured
    ||| rather than assumed: doubling is the only multiplier any
    ||| supported line writes, and a written count outside the operation
    ||| ("do this twice") is `Repeated` over the whole action.
    ||| Kind-indexed at [CR#122.1]'s own pair, a marker placed on an
    ||| object or a player, because the player seat is printed
    ||| ("double the number of each kind of counter you have", Aetheric
    ||| Amplifier) beside the permanent one.
    ||| A recipient holding no counters is doubled to nothing, which is
    ||| the arithmetic applied and not a defect, so no gate demands a
    ||| counter.
    ||| -- spelling: "double the number of each kind of counter on [on]";
    ||| at the player seat, "double the number of each kind of counter
    ||| you have".
    DoubleCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                               {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                               {auto 0 pm : PerMember on} -> Effect bs
    ||| Counters leave a player in a stated number as well as all at
    ||| once: [CR#728.1]'s own rules text has a player remove "one rad
    ||| counter from themselves", and printed removal lines count what
    ||| they take off an opponent. The amount is therefore a slot, and
    ||| leaving it unwritten is the "all" spelling rather than the only
    ||| reading available.
    LosesCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                    (amt : Maybe (Amount (nomIntro who))) ->
                    {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
    ||| The keyword-action carrier: an action, plus the label naming
    ||| which keyword action performing it amounts to. The label rides
    ||| the INNERMOST action rather than the whole expansion --
    ||| [CR#701.9a] defines the discard as the move, and choosing which
    ||| card is the surrounding instruction's business [CR#701.9b] -- so
    ||| an iterated batch is n labeled actions, which is what makes it
    ||| one event with n occurrences [CR#603.2c], and what a
    ||| would-replacement or an "at random" attaches to.
    ||| No gate relates label to body: the body IS the meaning and the
    ||| label only names it, so a mislabel is a spelling defect and not a
    ||| rules impossibility. The gates a keyword action really imposes
    ||| ride its macro, where the expansion is built.
    |||
    ||| **The macro layer is the only sanctioned way to reach `Enact` and
    ||| `Does`.** Keyword actions stack on top of the core rules without
    ||| disturbing them: the label exists because the game rules must
    ||| OBSERVE that a specific keyword action took place -- triggers and
    ||| replacements watch it -- while every keyword action is a composite
    ||| of pre-existing building blocks. So the label is the observability
    ||| hook, the macro is the sanctioned constructor, and the body is the
    ||| meaning. This is authoring policy, recorded here, not a compiler
    ||| gate: the carrier would be private if Idris could hide one
    ||| constructor of a `public export` type, and it cannot. A raw
    ||| `Enact`/`Does` outside `Macros` therefore still typechecks -- and
    ||| whatever it says its body already meant, spelled under a label no
    ||| macro imposed the verb's gates on. Pins do write them raw, on
    ||| purpose: a pin's business is the term the bench must not have.
    ||| -- spelling: the keyword action's own verb, imperative.
    Enact : (v : VerbLabel) -> (e : Effect bs) ->
            {auto 0 kn : KnownVerb v} -> Effect bs
    ||| `Enact`'s agentive surface: the same labeled action with the
    ||| player performing it written as its subject. Macro-only, on
    ||| `Enact`'s policy and for its reasons.
    ||| -- spelling: "[subj] [verb]s [body]".
    Does : (subj : Noun bs Player) -> (v : VerbLabel) ->
           (e : Effect (nomIntro subj)) ->
           {auto 0 kn : KnownVerb v} -> Effect bs
    ||| "[n]'s controller sacrifices it": the sacrifice written with its
    ||| agent, where the agent is DERIVED from the patient and the
    ||| printed pronoun is this row's own single noun slot. [CR#701.21a]
    ||| makes that derivation the rule's -- "to sacrifice a permanent,
    ||| ITS CONTROLLER moves it from the battlefield directly to its
    ||| owner's graveyard" -- so the possessive subject and the object
    ||| are one referent by the act's own definition, and neither the
    ||| possessive nor the pronoun is a read.
    ||| That is the whole of what this row adds over `Does (ControllerOf
    ||| n) "Sacrifice" (Move …)`, which has to write a second mention in
    ||| the object slot and so gates a pronoun that refuses wherever the
    ||| enclosing ability announced a permanent of its own (Basalt
    ||| Golem's blocked-by trigger, Goblin Ski Patrol's pump). Measured
    ||| 2026-08-27: 16 supported faces write "[possessor]'s controller
    ||| sacrifices it", and in all 16 the pronoun is the possessor.
    ||| It announces the controller, because the lines that follow read
    ||| that player back ("… sacrifices it. That player may search …",
    ||| Arcum Dagsson), and it stamps and re-zones the patient exactly as
    ||| the labeled move does.
    ||| -- spelling: "[n]'s controller sacrifices it".
    ControllerSacrifices : (n : Noun bs Object) ->
                           {auto 0 one : nounPlur n = OneOf} ->
                           {auto 0 zn : OnBattlefield (nounZone n)} -> Effect bs
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          {auto 0 pb : Payable c} ->
          {auto 0 ag : PayAgrees who c} -> Effect bs
    May : (offer : Maybe (Noun bs Player)) -> (body : Effect (mayCtx offer)) ->
          (ifDid : Maybe (Effect (effIntro body))) ->
          (ifNot : Maybe (Effect (mayCtx offer))) -> Effect bs
    ||| The postposed conditional, "[e] if [c]" / "[e] unless [c]": the
    ||| condition is written after the clause and reads what the clause has
    ||| announced — "Counter target spell if it's red" — and is checked as
    ||| the clause resolves, before the deed, so it sits at `preIntro e`.
    ||| Not a macro over `If`: the two introduce mentions in different
    ||| places, and oracle text reads backward only.
    |||
    ||| The `otherwise` arm reads what the CONDITION announced as well as
    ||| what the then-branch did, which is the same answer the leading
    ||| orientation gives: [CR#601.2c] announces a target at casting
    ||| whichever arm goes on to run, so an arm's scope cannot depend on
    ||| which side of the clause the condition was written. `If` gets it
    ||| structurally — its consequent is already typed at `condIntro c`, so
    ||| `otherwiseCtx` carries `condDelta c` through — and this row has to
    ||| write the same term explicitly, because its consequent is typed at
    ||| `bs`. The two therefore over-generate alike: a failed comparison's
    ||| margin stands in an arm that ran because the comparison failed,
    ||| tolerated and recorded at both orientations rather than at one.
    ||| -- spelling: "[e] if [c]"; under `NotCond`, "[e] unless [c]".
    OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) ->
             (otherwise : Maybe (Effect (condDelta c ++ otherwiseCtx e))) ->
             Effect bs
    ||| The leading conditional, "If [c], [e]. Otherwise, [o].": the
    ||| consequent reads what the condition introduced (`condDelta`) — a
    ||| comparison's margin, a phrase a comparison named. The otherwise
    ||| arm reads `otherwiseCtx e`.
    ||| -- spelling: "If [c], [e]."; with the arm, "… Otherwise, [o]."
    If : (c : Condition bs) -> (e : Effect (condIntro c)) ->
         (otherwise : Maybe (Effect (otherwiseCtx e))) -> Effect bs
    ||| "[Do something] unless [a player does something else]" [CR#118.12a],
    ||| carried for the cost arm: the offer is written after the effect, so
    ||| the payer reads what the effect has already named. A conditional
    ||| "unless" is `If e (NotCond c) Nothing` instead.
    Unless : (e : Effect bs) -> (who : Noun (preIntro e) Player) ->
             (c : Cost (nomIntro who)) ->
             {auto 0 pb : Payable c} ->
             {auto 0 ag : PayAgrees who c} -> Effect bs
    ||| "..., where [l] is [amt]": the definition of a letter an earlier
    ||| clause brought in by use. [CR#107.3c] fixes a text-defined X's
    ||| value as the ability resolves and has its controller choose
    ||| nothing, so the step presupposes an open X to define -- the gate --
    ||| and settles every open instance at once [CR#107.3i]. A second
    ||| definition finds none open and is refused by the same gate.
    ||| `Static.Define` writes the same construction in a static clause.
    ||| -- spelling: ", where X is [amt]" attached to the clause before it,
    ||| never "then"; a `Sequentially` whose member is a `Define` spells no
    ||| "then" at that seam.
    Define : (l : Letter) -> (amt : Amount bs) ->
             {auto 0 ok : So (anyOpenLetter l bs)} -> Effect bs
    ForEachOf : (grp : Noun bs Object) ->
                (body : Effect (elemIntro grp)) ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                Effect bs
    ||| "For each color among permanents you control, add one mana of
    ||| that color": the DISTRIBUTIVE twin of `DistinctCount`. One pass
    ||| per distinct value the axis takes over the domain, where
    ||| `ForEachOf` runs one pass per MEMBER -- the readings come apart
    ||| for `DistinctCount`'s reason, that one object carries several
    ||| card types [CR#205.2b] and several colours [CR#105.2].
    ||| NOT `Repeated (DistinctCount ax dom)`: every printed line of this
    ||| shape reads the value back ("of that color", "of that type",
    ||| "with that power"), and a counted iteration exports the count and
    ||| the body's own delta, never a value on the axis. So the pass
    ||| BINDS the value, and binds it as a quality, which is the sort the
    ||| existing chosen-quality reads already take: no new read is minted
    ||| for "that color".
    ||| The sort rides beside the axis and is gated to agree with it, as
    ||| `Aggregate` carries its kind beside its `ProjAxis`. An axis whose
    ||| values `QualitySort` does not yet name has no spelling here; see
    ||| `kindAxisSort`.
    ||| UNGATED on plurality, as `DistinctCount` is: a single object
    ||| still takes several values on these axes.
    ||| The domain is OPTIONAL. Written, the pass runs over the values
    ||| its group supplies; left out, it runs over the axis's whole value
    ||| set ("For each color, return up to one target card of that color
    ||| from your graveyard to your hand"), which only a set the rules
    ||| CLOSE can supply -- `kindDomainOk`.
    ||| -- spelling: "for each [axis] among [domain], [body]", or
    ||| "for each [axis], [body]" with no domain; the SCALING reading of
    ||| the same words ("draw a card for each color among permanents you
    ||| control") is no iteration at all and is `Times` over
    ||| `DistinctCount`.
    ForEachKindOf : (ax : KindAxis) -> (dom : Maybe (Noun bs Object)) ->
                    (q : QualitySort) ->
                    {auto 0 sc : kindAxisSort ax = Just q} ->
                    {auto 0 cl : So (kindDomainOk ax dom)} ->
                    (body : Effect (kindValueIntro q dom)) ->
                    Effect bs
    Repeat : (rep : Repetition bs) -> Effect bs
    ||| "[body] [n] times": the counted iteration, and the canonical
    ||| expansion of every counted keyword action -- "discard three
    ||| cards" is three passes of choose-one-and-discard-it. The iterated
    ||| singular is what makes a shortfall structural: each pass's choice
    ||| finds what it finds, which is [CR#609.3]'s do-as-much-as-possible
    ||| written into the term instead of asserted beside it. A batch
    ||| spelling belongs to the cards whose printed English carries the
    ||| cardinality itself ("choose two colors").
    ||| What it exports is the body's OWN delta pluralized -- one summary
    ||| mention per mention the body introduced, same payload and same
    ||| stamp -- so "the discarded cards" and "that many" read the whole
    ||| batch, beside the count the text wrote, which a shortfall can
    ||| leave larger than the batch [CR#609.3].
    ||| -- spelling: the body's, with the count written on it.
    Repeated : (n : Amount bs) -> (body : Effect (amtIntro n)) -> Effect bs
    ||| A coordination of one member denotes that member, so only the
    ||| empty list is refused: it instructs nothing.
    Sequentially : {0 n : Nat} -> Effects n bs ->
                   {auto 0 ne : IsSucc n} -> Effect bs
    Simultaneously : {0 n : Nat} -> SimEffects n bs ->
                     {auto 0 ne : IsSucc n} -> Effect bs
    Modal : (q : Quantity bs) -> (modes : List (Effect bs)) ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit q (modeCount modes)} ->
            {auto 0 dm : So (distinctModes modes)} -> Effect bs
    Delayed : (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (span : Maybe (Duration bs)) ->
              Effect (delayedCtx alts ev) ->
              {auto 0 so : DelaySpanOk span} -> Effect bs
    InsteadOf : (replaced : Effect bs) -> (repl : Effect (replacedCtx replaced)) ->
                {auto 0 na : NotInstead replaced} ->
                {auto 0 nb : NotInstead repl} -> Effect bs
    HeldUntil : (e : Effect bs) -> (ev : GameEvent (annIntro e)) ->
                {auto 0 ok : HeldClause e} -> Effect bs
    Reflexively : (body : Effect bs) -> (trig : Effect (reflexCtx body)) ->
                  {auto 0 en : ReflexEnclosure body} -> Effect bs
    ||| [CR#603.12]'s other trigger template, "when [something happens]
    ||| this way": the trigger names the EVENT the enclosure caused
    ||| rather than the player who acted, so it reads the enclosure's
    ||| outcome mentions. It is checked at once when the enclosure caused
    ||| the event as it resolved [CR#603.12]; when the enclosure's effect
    ||| is a replacement that applies later, as a regeneration shield
    ||| does [CR#701.19a], it waits as a delayed trigger [CR#603.7].
    ThisWay : (body : Effect bs) -> (ev : GameEvent (effIntro body)) ->
              (trig : Effect (thisWayCtx body ev)) ->
              {auto 0 oc : ThisWayOutcome body} -> Effect bs

    DoesntUntapNext : (n : Noun bs Object) -> (steps : Amount bs) ->
                      {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    SkipsNext : (who : Noun bs Player) -> (part : TurnPart) ->
                (count : Amount bs) -> Effect bs
    ExtraTurn : (who : Noun bs Player) -> (count : Amount bs) -> Effect bs
    AdditionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) ->
                     (followedBy : Maybe TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AnchorPart anchor} ->
                     {auto 0 fb : FollowerPart followedBy} -> Effect bs

  ||| [CR#610.3] hangs the "until" rider on a one-shot that changes an
  ||| object's zone and [CR#610.4] on one that phases a permanent out.
  ||| Those are the only two one-shots the CR gives the rider, so every
  ||| False below is that pair of rules speaking rather than a count.
  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (DealDamageOwn _ _ _) = False
  heldUntilOk (ControllerSacrifices _) = False
  heldUntilOk (DoesntUntapNext _ _) = False
  heldUntilOk (SkipsNext _ _ _) = False
  heldUntilOk (ExtraTurn _ _) = False
  heldUntilOk (AdditionalPart _ _ _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  -- [CR#610.4]: "until" also rides a permanent phasing out, and the
  -- second one-shot phases it back in.
  heldUntilOk (SetStatus PhasedOut _) = True
  heldUntilOk (SetStatus _ _) = False
  heldUntilOk (GetsCounters _ _ _) = False
  heldUntilOk (GetsCountersOfThoseKinds _ _) = False
  heldUntilOk (LosesCounters _ _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
  heldUntilOk (BecomesBlocking _ _) = False
  heldUntilOk (StopsBlocking _ _) = False
  heldUntilOk (BecomesAttacking _ _) = False
  heldUntilOk (Regenerate _) = False
  heldUntilOk (CantBe _ _ _) = False
  heldUntilOk (GainsDesignation _ _ _ _) = False
  heldUntilOk (GameBecomes _) = False
  heldUntilOk (Concludes _ _) = False
  heldUntilOk GameDrawn = False
  heldUntilOk (CounterSpell _) = False
  heldUntilOk (CopyStack _ _ _ _) = False
  heldUntilOk (ChooseNewTargets _) = False
  heldUntilOk (Choose _ _) = False
  heldUntilOk (Move _ _ _) = True
  heldUntilOk (ChangeLife _ _) = False
  heldUntilOk (AddMana _ _ _ _) = False
  heldUntilOk (Draw _ _) = False
  heldUntilOk (Expose _ _ _) = False
  heldUntilOk (Search _ _ _ _) = False
  heldUntilOk (Shuffle _) = False
  heldUntilOk (FlipCoins _ _) = False
  heldUntilOk (RollDice _ _ _) = False
  heldUntilOk (ResultsTable _) = False
  heldUntilOk (IgnoreOutcomes _) = False
  heldUntilOk (ShiftResult _) = False
  heldUntilOk (RollPlanarDie _ _) = False
  heldUntilOk ChaosEnsues = False
  heldUntilOk (StoreResults _) = False
  heldUntilOk (RerollStored _ _ _) = False
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Throughout _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (GetsEmblem _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  heldUntilOk (MoveCounters _ _ _ _) = False
  heldUntilOk (PutSameCounters _ _) = False
  heldUntilOk (PutCountersOfThoseKinds _ _) = False
  heldUntilOk (GiveCountersOfOwnKinds _) = False
  heldUntilOk (DoubleCountersOfOwnKinds _) = False
  heldUntilOk (Enact _ (Move _ _ _)) = True
  heldUntilOk (Enact _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (OnlyIf _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (Unless _ _ _) = False
  heldUntilOk (Define _ _) = False
  heldUntilOk (ForEachOf _ _) = False
  heldUntilOk (ForEachKindOf _ _ _ _) = False
  heldUntilOk (Repeat _) = False
  heldUntilOk (Repeated _ _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _ _ _) = False
  heldUntilOk (InsteadOf _ _) = False
  heldUntilOk (HeldUntil _ _) = False
  heldUntilOk (Reflexively _ _) = False
  heldUntilOk (ThisWay _ _ _) = False

  ||| Why a clause is or is not a reflexive trigger's enclosure
  ||| [CR#603.12]; kept as an enum rather than a Bool since a False can
  ||| stand on several different structural grounds.
  public export
  data EncloseUse
    = ||| No PLAYER takes the action, so "you do" has no subject to
      ||| inflect for [CR#603.12].
      EncAgentless
    | ||| No single taken action for the pro-verb to abbreviate.
      EncNotOneAction
    | ||| The clause schedules its action rather than taking it, so
      ||| nothing has "occurred earlier during the resolution" [CR#603.12].
      EncNotYetTaken
    | ||| A player's own single action.
      EncReflexive

  public export
  reflexEncloseUse : {0 bs : Bindings} -> Effect bs -> EncloseUse
  reflexEncloseUse (DealDamage _ _ _) = EncAgentless
  reflexEncloseUse (DealDamageOwn _ _ _) = EncAgentless
  reflexEncloseUse (ControllerSacrifices _) = EncReflexive
  reflexEncloseUse (DoesntUntapNext _ _) = EncAgentless
  reflexEncloseUse (SkipsNext _ _ _) = EncNotYetTaken
  reflexEncloseUse (ExtraTurn _ _) = EncNotYetTaken
  reflexEncloseUse (AdditionalPart _ _ _ _) = EncAgentless
  reflexEncloseUse (Distribute _ _ _) = EncAgentless
  reflexEncloseUse (Fights _ _) = EncAgentless
  reflexEncloseUse (ChangeLife _ _) = EncAgentless
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncReflexive
  reflexEncloseUse (Continuously _ _) = EncAgentless
  reflexEncloseUse (Throughout _ (GainsControl _ _)) = EncReflexive
  reflexEncloseUse (Throughout _ _) = EncAgentless
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _) = EncReflexive        -- 66, all of them offered
  reflexEncloseUse (Enact _ _) = EncReflexive
  -- a status change is the effect's, not a player's: [CR#603.12]'s
  -- agent form has no subject to inflect.
  reflexEncloseUse (SetStatus _ _) = EncAgentless
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (GetsCountersOfThoseKinds _ _) = EncAgentless
  reflexEncloseUse (LosesCounters _ _ _) = EncAgentless
  reflexEncloseUse (RemoveFromCombat _) = EncAgentless
  reflexEncloseUse (BecomesBlocking _ _) = EncAgentless
  reflexEncloseUse (StopsBlocking _ _) = EncAgentless
  reflexEncloseUse (BecomesAttacking _ _) = EncAgentless
  reflexEncloseUse (Regenerate _) = EncAgentless
  reflexEncloseUse (CantBe _ _ _) = EncAgentless
  reflexEncloseUse (GainsDesignation _ _ _ _) = EncAgentless
  reflexEncloseUse (GameBecomes _) = EncAgentless
  reflexEncloseUse (Concludes _ _) = EncAgentless
  reflexEncloseUse GameDrawn = EncAgentless
  reflexEncloseUse (CounterSpell _) = EncAgentless
  reflexEncloseUse (CopyStack _ _ _ _) = EncAgentless
  reflexEncloseUse (ChooseNewTargets _) = EncReflexive
  reflexEncloseUse (Create _ _ _ _) = EncReflexive -- 8
  reflexEncloseUse (GetsEmblem _ _) = EncAgentless
  reflexEncloseUse (PutCounters _ _ _) = EncReflexive    -- 8
  reflexEncloseUse (RemoveCounters _ _ _) = EncReflexive -- 7
  reflexEncloseUse (MoveCounters _ _ _ _) = EncReflexive
  reflexEncloseUse (PutSameCounters _ _) = EncReflexive
  reflexEncloseUse (PutCountersOfThoseKinds _ _) = EncReflexive
  reflexEncloseUse (GiveCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (DoubleCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (Move _ _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _ _) = EncReflexive       -- 1
  reflexEncloseUse (Search _ _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  reflexEncloseUse (FlipCoins _ _) = EncReflexive
  reflexEncloseUse (RollDice _ _ _) = EncReflexive
  reflexEncloseUse (ResultsTable _) = EncNotOneAction
  -- no PLAYER is written taking either action: the ignore and the
  -- shift are stated of the roll the clause made.
  reflexEncloseUse (IgnoreOutcomes _) = EncAgentless
  reflexEncloseUse (ShiftResult _) = EncAgentless
  reflexEncloseUse (RollPlanarDie _ _) = EncReflexive
  reflexEncloseUse ChaosEnsues = EncAgentless
  reflexEncloseUse (StoreResults _) = EncAgentless
  reflexEncloseUse (RerollStored _ _ _) = EncReflexive
  -- [CR#603.12] writes the reflexive over what a player did or didn't
  -- do, so a declined arm leaves one offered action to inflect.
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  -- [CR#603.12] asks whether the player took the action, and a postposed
  -- condition gates whether the enclosure's ONE action happens rather
  -- than adding a second one to abbreviate: "Then sacrifice it if it has
  -- five or more bloodstain counters on it. When you do, ..." (Blood
  -- Spatter Analysis) leaves exactly the question the pro-verb puts.
  -- `May`'s unbranched offer passes through for the same reason.
  reflexEncloseUse (OnlyIf e _ _) = reflexEncloseUse e
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (Unless _ _ _) = EncNotOneAction
  reflexEncloseUse (Define _ _) = EncAgentless
  reflexEncloseUse (ForEachOf _ _) = EncNotOneAction
  reflexEncloseUse (ForEachKindOf _ _ _ _) = EncNotOneAction
  reflexEncloseUse (Repeat _) = EncNotOneAction
  reflexEncloseUse (Repeated _ _) = EncNotOneAction
  reflexEncloseUse (Sequentially _) = EncNotOneAction
  reflexEncloseUse (Simultaneously _) = EncNotOneAction
  reflexEncloseUse (Modal _ _) = EncNotOneAction
  reflexEncloseUse (InsteadOf _ _) = EncNotOneAction
  reflexEncloseUse (Reflexively _ _) = EncNotOneAction
  reflexEncloseUse (ThisWay _ _ _) = EncNotOneAction
  reflexEncloseUse (Delayed _ _ _ _) = EncNotYetTaken
  reflexEncloseUse (HeldUntil _ _) = EncNotYetTaken

  public export
  admitsReflexEnclosure : EncloseUse -> Bool
  admitsReflexEnclosure EncAgentless = False
  admitsReflexEnclosure EncNotOneAction = False
  admitsReflexEnclosure EncNotYetTaken = False
  admitsReflexEnclosure EncReflexive = True

  public export
  ReflexEnclosure : Effect bs -> Type
  ReflexEnclosure e = So (admitsReflexEnclosure (reflexEncloseUse e))

  ||| Does the enclosure itself cause the event "this way" points at?
  ||| Only a clause that creates a delayed triggered ability fails: that
  ||| ability is a separate one that resolves on its own [CR#603.7], so
  ||| the deed when it comes is not the enclosure's for "this way" to
  ||| name. An effect that applies later without a second ability — a
  ||| replacement, a continuous effect, a skipped step — still caused it.
  ||| Separate from `reflexEncloseUse`: that asks for an agent to
  ||| inflect, this asks for a caused event, so a compound or agentless
  ||| enclosure is fine here.
  public export
  thisWayOutcomeOk : {0 bs : Bindings} -> Effect bs -> Bool
  thisWayOutcomeOk (Delayed _ _ _ _) = False
  -- an offer's outcome is its body's; the arms are separate sentences.
  thisWayOutcomeOk (May _ body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (DoesntUntapNext _ _) = True
  thisWayOutcomeOk (SkipsNext _ _ _) = True
  thisWayOutcomeOk (ExtraTurn _ _) = True
  thisWayOutcomeOk (AdditionalPart _ _ _ _) = True
  thisWayOutcomeOk (HeldUntil _ _) = True
  thisWayOutcomeOk (DealDamage _ _ _) = True
  thisWayOutcomeOk (Distribute _ _ _) = True
  thisWayOutcomeOk (Fights _ _) = True
  thisWayOutcomeOk (SetStatus _ _) = True
  thisWayOutcomeOk (DealDamageOwn _ _ _) = True
  thisWayOutcomeOk (ControllerSacrifices _) = True
  thisWayOutcomeOk (GetsCounters _ _ _) = True
  thisWayOutcomeOk (GetsCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (LosesCounters _ _ _) = True
  thisWayOutcomeOk (RemoveFromCombat _) = True
  thisWayOutcomeOk (BecomesBlocking _ _) = True
  thisWayOutcomeOk (StopsBlocking _ _) = True
  thisWayOutcomeOk (BecomesAttacking _ _) = True
  thisWayOutcomeOk (Regenerate _) = True
  thisWayOutcomeOk (CantBe _ _ _) = True
  thisWayOutcomeOk (GainsDesignation _ _ _ _) = True
  thisWayOutcomeOk (GameBecomes _) = True
  thisWayOutcomeOk (Concludes _ _) = True
  thisWayOutcomeOk GameDrawn = True
  thisWayOutcomeOk (CounterSpell _) = True
  thisWayOutcomeOk (CopyStack _ _ _ _) = True
  thisWayOutcomeOk (ChooseNewTargets _) = True
  thisWayOutcomeOk (Choose _ _) = True
  thisWayOutcomeOk (Move _ _ _) = True
  thisWayOutcomeOk (ChangeLife _ _) = True
  thisWayOutcomeOk (AddMana _ _ _ _) = True
  thisWayOutcomeOk (Draw _ _) = True
  thisWayOutcomeOk (Expose _ _ _) = True
  thisWayOutcomeOk (Search _ _ _ _) = True
  thisWayOutcomeOk (Shuffle _) = True
  thisWayOutcomeOk (FlipCoins _ _) = True
  thisWayOutcomeOk (RollDice _ _ _) = True
  thisWayOutcomeOk (ResultsTable _) = True
  thisWayOutcomeOk (IgnoreOutcomes _) = True
  thisWayOutcomeOk (ShiftResult _) = True
  thisWayOutcomeOk (RollPlanarDie _ _) = True
  thisWayOutcomeOk ChaosEnsues = True
  thisWayOutcomeOk (StoreResults _) = True
  thisWayOutcomeOk (RerollStored _ _ _) = True
  thisWayOutcomeOk (Continuously _ _) = True
  thisWayOutcomeOk (Throughout _ _) = True
  thisWayOutcomeOk (Create _ _ _ _) = True
  thisWayOutcomeOk (GetsEmblem _ _) = True
  thisWayOutcomeOk (PutCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCounters _ _ _) = True
  thisWayOutcomeOk (MoveCounters _ _ _ _) = True
  thisWayOutcomeOk (PutSameCounters _ _) = True
  thisWayOutcomeOk (PutCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (GiveCountersOfOwnKinds _) = True
  thisWayOutcomeOk (DoubleCountersOfOwnKinds _) = True
  thisWayOutcomeOk (Enact _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (Pay _ _) = True
  thisWayOutcomeOk (OnlyIf _ _ _) = True
  thisWayOutcomeOk (If _ _ _) = True
  thisWayOutcomeOk (Unless _ _ _) = True
  thisWayOutcomeOk (Define _ _) = True
  thisWayOutcomeOk (ForEachOf _ _) = True
  thisWayOutcomeOk (ForEachKindOf _ _ _ _) = True
  thisWayOutcomeOk (Repeat _) = True
  thisWayOutcomeOk (Repeated _ _) = True
  thisWayOutcomeOk (Sequentially _) = True
  thisWayOutcomeOk (Simultaneously _) = True
  thisWayOutcomeOk (Modal _ _) = True
  thisWayOutcomeOk (InsteadOf _ _) = True
  thisWayOutcomeOk (Reflexively _ _) = True
  thisWayOutcomeOk (ThisWay _ _ _) = True

  public export
  ThisWayOutcome : Effect bs -> Type
  ThisWayOutcome e = So (thisWayOutcomeOk e)

  public export
  payableOk : {0 bs : Bindings} -> Cost bs -> Bool
  payableOk (Mana _) = True
  payableOk (ScaledMana _) = True
  payableOk TapSymbol = False
  payableOk UntapSymbol = False
  payableOk (LoyaltySymbol _) = False
  payableOk (Do _) = True
  payableOk (Compound _) = True
  payableOk ItsManaCost = True

  public export
  Payable : Cost bs -> Type
  Payable {bs} c = So (payableOk c)

  ||| [CR#118.1] makes a cost an action a player carries out, so any
  ||| instruction a payer can follow is admitted; `costNounOk` keeps its
  ||| own hold on which nouns a cost may name. Refused: the nodes that
  ||| instruct nothing at payment (a static, a replacement, a scheduled
  ||| or reflexive trigger, a skip [CR#614.10]) and the coordinations,
  ||| which claim an order [CR#601.2h] pays in any — `Compound` is the
  ||| cost-side telescope.
  public export
  costActionOk : {0 bs : Bindings} -> Effect bs -> Bool
  costActionOk (DealDamage src _ _) = costNounOk src
  costActionOk (DealDamageOwn src _ _) = costNounOk src
  costActionOk (ControllerSacrifices n) = costNounOk n
  costActionOk (DoesntUntapNext n _) = costNounOk n
  costActionOk (SkipsNext _ _ _) = False
  costActionOk (ExtraTurn who _) = costNounOk who
  costActionOk (AdditionalPart _ _ _ _) = True
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (SetStatus _ n) = costNounOk n
  costActionOk (GetsCounters who _ _) = costNounOk who
  -- the distributive twin reads an announced batch, which no cost has.
  costActionOk (GetsCountersOfThoseKinds _ _) = False
  costActionOk (LosesCounters who _ _) = costNounOk who
  costActionOk (RemoveFromCombat n) = costNounOk n
  costActionOk (BecomesBlocking n _) = costNounOk n
  costActionOk (StopsBlocking n _) = costNounOk n
  costActionOk (BecomesAttacking n _) = costNounOk n
  costActionOk (Regenerate n) = costNounOk n
  costActionOk (CantBe e _ _) = costActionOk e
  costActionOk (GainsDesignation n _ _ _) = costNounOk n
  costActionOk (GameBecomes _) = True
  costActionOk (Concludes _ _) = True
  costActionOk GameDrawn = True
  costActionOk (CounterSpell _) = True
  costActionOk (CopyStack _ what _ _) = costNounOk what
  costActionOk (ChooseNewTargets what) = costNounOk what
  costActionOk (Choose n _) = costNounOk n
  costActionOk (Move what _ _) = costNounOk what
  costActionOk (ChangeLife _ _) = True
  costActionOk (AddMana who _ _ _) = costNounOk who
  costActionOk (Draw _ _) = True
  costActionOk (Expose _ who _) = costNounOk who
  costActionOk (Search who _ _ _) = costNounOk who
  costActionOk (Shuffle whose) = costNounOk whose
  costActionOk (FlipCoins who _) = costNounOk who
  costActionOk (RollDice who _ _) = costNounOk who
  costActionOk (ResultsTable _) = False
  -- each reads the roll a clause before it made, which no cost has.
  costActionOk (IgnoreOutcomes _) = False
  costActionOk (ShiftResult _) = False
  costActionOk (StoreResults _) = False
  costActionOk (RollPlanarDie who _) = costNounOk who
  costActionOk ChaosEnsues = False
  costActionOk (RerollStored who _ _) = costNounOk who
  costActionOk (Continuously _ _) = False
  costActionOk (Throughout _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  costActionOk (PutSameCounters src dst) = costNounOk src && costNounOk dst
  costActionOk (GiveCountersOfOwnKinds on) = costNounOk on
  costActionOk (DoubleCountersOfOwnKinds on) = costNounOk on
  -- the distributive kind anaphor reads an announced batch; no cost
  -- announces one, so the clause instructs nothing at payment.
  costActionOk (PutCountersOfThoseKinds _ _) = False
  costActionOk (Enact _ e) = costActionOk e
  costActionOk (Does _ _ e) = costActionOk e
  costActionOk (Pay _ _) = False
  costActionOk (May _ body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (OnlyIf e _ otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (If _ e otherwise) = costActionOk e && costActionOkOpt otherwise
  -- an offer another player answers at resolution [CR#118.12a]
  costActionOk (Unless _ _ _) = False
  -- a definition instructs nothing at payment but rides a cost's own
  -- amount; admitted so the cost telescope can carry "where X is".
  costActionOk (Define _ _) = True
  costActionOk (ForEachOf _ body) = costActionOk body
  costActionOk (ForEachKindOf _ _ _ body) = costActionOk body
  costActionOk (Repeat _) = False
  costActionOk (Repeated _ body) = costRepeatedOk body
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ modes) = costActionsOk modes
  costActionOk (Delayed _ _ _ _) = False
  costActionOk (InsteadOf _ _) = False
  costActionOk (HeldUntil _ _) = False
  costActionOk (Reflexively _ _) = False
  costActionOk (ThisWay _ _ _) = False

  public export
  costActionOkOpt : {0 bs : Bindings} -> Maybe (Effect bs) -> Bool
  costActionOkOpt Nothing = True
  costActionOkOpt (Just e) = costActionOk e

  public export
  costActionsOk : {0 bs : Bindings} -> List (Effect bs) -> Bool
  costActionsOk [] = True
  costActionsOk (e :: es) = costActionOk e && costActionsOk es

  public export
  CostAction : Effect bs -> Type
  CostAction {bs} e = So (costActionOk e)

  public export
  HeldClause : Effect bs -> Type
  HeldClause {bs} e = So (heldUntilOk e)

  public export
  isInstead : {0 bs : Bindings} -> Effect bs -> Bool
  isInstead (InsteadOf _ _) = True
  isInstead _ = False

  public export
  NotInstead : Effect bs -> Type
  NotInstead {bs} e = So (not (isInstead e))

  public export
  modeCount : {0 bs : Bindings} -> List (Effect bs) -> Nat
  modeCount [] = Z
  modeCount (_ :: es) = S (modeCount es)

  public export
  effEq : {0 bs : Bindings} -> Effect bs -> Effect bs -> Bool
  effEq (DealDamage _ _ _) _ = False
  effEq (DealDamageOwn _ _ _) _ = False
  effEq (ControllerSacrifices _) _ = False
  effEq (DoesntUntapNext n s) (DoesntUntapNext m t) = nounEqRef n m && boundEq s t
  effEq (DoesntUntapNext _ _) _ = False
  effEq (SkipsNext w p c) (SkipsNext x q d) =
    nounEqRef w x && p == q && boundEq c d
  effEq (SkipsNext _ _ _) _ = False
  effEq (ExtraTurn w c) (ExtraTurn x d) = nounEqRef w x && boundEq c d
  effEq (ExtraTurn _ _) _ = False
  effEq (AdditionalPart p a c _) (AdditionalPart q b d _) =
    p == q && a == b && boundEq c d
  effEq (AdditionalPart _ _ _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (SetStatus v a) (SetStatus w b) = sameStatusVal v w && nounEqRef a b
  effEq (SetStatus _ _) _ = False
  -- The two player-counter rows compare kind AND amount, and both narrow
  -- to `You` to do it: an `Amount` is indexed by its subject's discourse,
  -- so amounts under two different subjects are not the same type. `Draw`
  -- narrows the same way for the same reason.
  effEq (GetsCounters You x j) (GetsCounters You y l) = j == l && boundEq x y
  effEq (GetsCounters _ _ _) _ = False
  effEq (GetsCountersOfThoseKinds _ _) _ = False
  effEq (LosesCounters You j Nothing) (LosesCounters You l Nothing) = j == l
  effEq (LosesCounters You j (Just x)) (LosesCounters You l (Just y)) =
    j == l && boundEq x y
  effEq (LosesCounters _ _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
  effEq (BecomesBlocking _ _) _ = False
  effEq (StopsBlocking _ _) _ = False
  effEq (BecomesAttacking _ _) _ = False
  effEq (Regenerate a) (Regenerate b) = nounEqRef a b
  effEq (Regenerate _) _ = False
  effEq (CantBe _ _ _) _ = False
  effEq (GainsDesignation _ _ _ _) _ = False
  effEq (GameBecomes a) (GameBecomes b) = a == b
  effEq (GameBecomes _) _ = False
  effEq (Concludes v a) (Concludes w b) = v == w && nounEqRef a b
  effEq (Concludes _ _) _ = False
  effEq GameDrawn GameDrawn = True
  effEq GameDrawn _ = False
  -- kind-indexed, so two subjects need not share a kind to compare;
  -- `Choose`'s row gives up on the same ground.
  effEq (CounterSpell _) _ = False
  effEq (CopyStack _ _ _ _) _ = False
  effEq (ChooseNewTargets a) (ChooseNewTargets b) = nounEqRef a b
  effEq (ChooseNewTargets _) _ = False
  effEq (Choose _ _) _ = False
  effEq (Move a s _) (Move b t _) = nounEqRef a b && zoneSort s == zoneSort t
  effEq (Move _ _ _) _ = False
  effEq (ChangeLife _ _) _ = False
  effEq (AddMana _ _ _ _) _ = False
  effEq (Draw You a) (Draw You b) = boundEq a b
  effEq (Draw _ _) _ = False
  effEq (Expose _ _ _) _ = False
  effEq (Search _ _ _ _) _ = False
  effEq (Shuffle _) _ = False
  effEq (FlipCoins _ _) _ = False
  effEq (RollDice _ _ _) _ = False
  effEq (ResultsTable _) _ = False
  effEq (IgnoreOutcomes _) _ = False
  effEq (ShiftResult _) _ = False
  effEq (RollPlanarDie _ _) _ = False
  effEq ChaosEnsues _ = False
  effEq (StoreResults _) _ = False
  effEq (RerollStored _ _ _) _ = False
  effEq (Continuously _ _) _ = False
  effEq (Throughout _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (GetsEmblem _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (MoveCounters _ _ _ _) _ = False
  effEq (PutSameCounters _ _) _ = False
  effEq (PutCountersOfThoseKinds _ _) _ = False
  effEq (GiveCountersOfOwnKinds _) _ = False
  effEq (DoubleCountersOfOwnKinds _) _ = False
  effEq (Enact v e) (Enact w f) = v == w && effEq e f
  effEq (Enact _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (Pay _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (OnlyIf _ _ _) _ = False
  effEq (If _ _ _) _ = False
  effEq (Unless _ _ _) _ = False
  effEq (Define _ _) _ = False
  effEq (ForEachOf _ _) _ = False
  effEq (ForEachKindOf _ _ _ _) _ = False
  effEq (Repeat _) _ = False
  effEq (Repeated _ _) _ = False
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _ _ _) _ = False
  effEq (InsteadOf _ _) _ = False
  effEq (HeldUntil _ _) _ = False
  effEq (Reflexively _ _) _ = False
  effEq (ThisWay _ _ _) _ = False

  public export
  anyEffEq : {0 bs : Bindings} -> Effect bs -> List (Effect bs) -> Bool
  anyEffEq e [] = False
  anyEffEq e (f :: fs) = effEq e f || anyEffEq e fs

  public export
  distinctModes : {0 bs : Bindings} -> List (Effect bs) -> Bool
  distinctModes [] = True
  distinctModes (e :: es) = not (anyEffEq e es) && distinctModes es

  public export
  data Effects : Nat -> Bindings -> Type where
    Nil : Effects Z bs
    (::) : (e : Effect bs) ->
           Effects n (effIntro e) -> Effects (S n) bs

  namespace Sim
    public export
    data SimEffects : Nat -> Bindings -> Type where
      Nil : SimEffects Z bs
      (::) : (e : Effect bs) ->
             SimEffects n (annIntro e) -> SimEffects (S n) bs

  ||| The mentions a clause adds ON TOP of the context it was read in:
  ||| `effIntro` less that context. Every row of `effIntro` answers
  ||| `<new> ++ bs` or rewrites a binding already in `bs` in place, so the
  ||| length difference is exactly the clause's own introductions and a
  ||| clause that only re-zoned an outer mention has a delta of none.
  public export
  effDelta : {bs : Bindings} -> Effect bs -> Bindings
  effDelta e = take (length (effIntro e) `minus` length bs) (effIntro e)

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (DealDamageOwn src c to) = outcomeB DamageDealt :: nomIntro to
  -- the controller is announced, and the patient is stamped and re-zoned
  -- by the act that took it [CR#701.21a] -- the labeled move's own
  -- answer, written out because the agent is derived rather than given.
  effIntro (ControllerSacrifices n) =
    MkBinding TheD Player OneOf PlayerP
      :: moveIntro (Just "Sacrifice") n (Just Graveyard)
  effIntro (Fights a b) = nomIntro b
  effIntro (SetStatus _ n) = nomIntro n
  effIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  effIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  effIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  effIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  effIntro (GetsCounters who amt _) = amtIntro amt
  effIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  effIntro (LosesCounters who _ amt) = optAmtIntro amt
  effIntro (RemoveFromCombat n) = nomIntro n
  effIntro (BecomesBlocking _ what) = nomIntro what
  effIntro (StopsBlocking _ what) = nomIntro what
  effIntro (BecomesAttacking n NoDefender) = nomIntro n
  effIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  effIntro (Regenerate n) = nomIntro n
  effIntro (CantBe e _ _) = effIntro e
  effIntro (GainsDesignation n _ _ _) = nomIntro n
  effIntro (GameBecomes _) = bs
  effIntro (Concludes _ who) = nomIntro who
  effIntro GameDrawn = bs
  effIntro (CounterSpell what) = nomIntro what
  effIntro (CopyStack agent what times exc) =
    MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
              (ObjectP (nounTy what) (Just Stack) Nothing (Just CopyOrigin))
      :: amtIntro times
  effIntro (ChooseNewTargets what) = nomIntro what
  effIntro (Choose n Nothing) = chosenIntro n
  effIntro (Choose n (Just b)) = nounDelta b ++ chosenIntro n
  -- a destination that shuffles [CR#701.24c] takes the discourse with
  -- it, exactly as the bare `Shuffle` does.
  effIntro (Move what to _) =
    afterMoveTo to (moveIntro Nothing what (Just (zoneSort to)))
  effIntro (ChangeLife who (Up a)) = outcomeB LifeGained :: lifeIntro (Up a)
  effIntro (ChangeLife who (Down a)) = outcomeB LifeLost :: lifeIntro (Down a)
  effIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  effIntro (AddMana who amt _ _) = amtIntro amt
  effIntro (Draw who amt) = amtIntro amt
  effIntro (Expose v who what) = exposedIntro what
  -- the found card is stamped by the label that found it, which is what
  -- keeps it out of a later shuffle [CR#701.24b].
  effIntro (Search who sc q p) =
    MkBinding AD Object (quantPlur q)
              (ObjectP (seedTy p) (searchZone sc)
                       (mkStamp (Just "Search") Nothing False) Nothing)
      :: (quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who)
  effIntro (Shuffle whose) = afterShuffle (nomIntro whose)
  effIntro (FlipCoins who count) = outcomeB CoinFlipped :: flipScopeIntro count
  effIntro (RollDice who count _) = outcomeB RollResult :: amtIntro count
  effIntro (ResultsTable rows) = bs
  effIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  effIntro (ShiftResult amt) = amtIntro amt
  effIntro (RollPlanarDie who count) = outcomeB PlanarRolled :: amtIntro count
  effIntro ChaosEnsues = bs
  effIntro (StoreResults on) = nomIntro on
  effIntro (RerollStored _ _ whose) = nomIntro whose
  effIntro (Continuously se _) = staticIntro se
  effIntro (Throughout _ se) = staticIntro se
  effIntro (Create agent count spec riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin))
      :: (specDelta spec ++ amtIntro count)
  effIntro (GetsEmblem who _) = nomIntro who
  effIntro (PutCounters amt kind on) = nomIntro on
  effIntro (Distribute (DividedDamage _) amt among) = outcomeB DamageDealt :: nomIntro among
  effIntro (Distribute (DistributedCounters _) amt among) = nomIntro among
  effIntro (RemoveCounters amt kind from) = nomIntro from
  effIntro (MoveCounters amt kind src dst) = nomIntro dst
  effIntro (PutSameCounters src dst) = nomIntro dst
  effIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  effIntro (GiveCountersOfOwnKinds on) = nomIntro on
  effIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  effIntro (Enact v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Enact v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Enact _ e) = effIntro e
  effIntro (Does s v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Does s v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Does s v e) = effIntro e
  effIntro (Pay who c) = costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- a conditioned clause exports what it ANNOUNCED and no more: the
  -- condition may have failed, so nothing the clause would have DONE
  -- stands after it -- but [CR#601.2c] chose its targets as the spell was
  -- cast, and a target is chosen whether or not the condition holds. So
  -- "…gets +2/+2 until end of turn if its power is 2. Then IT fights…"
  -- (Savage Swipe, 5 supported lines) reads the target back. The
  -- otherwise arm changes nothing here: both arms are typed over
  -- `annIntro e`, so the announcement is what either of them leaves.
  effIntro (OnlyIf e c oth) = annIntro e
  -- the LEADING conditional exports nothing even so: its consequent is
  -- typed over the condition's own delta, and a condition that failed
  -- announced nothing for the text after it to read.
  effIntro (If c e oth) = bs
  effIntro (Unless e who c) = bs
  effIntro (Define l amt) = defineLetter l (amtIntro amt)
  -- a DOMAIN-DRIVEN loop has a determinate iteration count, so the union
  -- over its passes is well-formed and is exported on `Repeated`'s
  -- precedent: one summary mention per mention the body introduced, same
  -- payload and same stamp, differing only in naming many where one pass
  -- named one. No count rides along -- `Repeated` exports one because the
  -- text WROTE a number, and here the number is the group's own size,
  -- which `GroupSize` already reads off the group's mention.
  effIntro (ForEachOf grp body) = pluralizeDelta (effDelta body) ++ bs
  effIntro (ForEachKindOf _ _ _ _) = bs
  -- an OPEN-ENDED repetition exports nothing: with no bound there is no
  -- determinate batch to summarise, and an until-condition supplies a
  -- stopping test rather than a count.
  effIntro (Repeat _) = bs
  effIntro (Repeated n body) =
    outcomeB RepeatCount :: (pluralizeDelta (effDelta body) ++ amtIntro n)
  effIntro (Sequentially es) = effsIntro es
  effIntro (Simultaneously es) = simIntro es
  effIntro (Modal q modes) = quantDelta q ++ bs
  effIntro (Delayed ev _ _ e) = bs               -- a future clause mentions nothing NOW
  effIntro (Reflexively body trig) = effIntro body
  effIntro (ThisWay body ev trig) = effIntro body
  effIntro (InsteadOf replaced repl) = annIntro replaced
  effIntro (HeldUntil e ev) = annIntro e

  ||| What a clause has announced by the time its own trailing condition
  ||| is read — the pre-resolution twin of `effIntro`, since the
  ||| condition is written after the clause but evaluated before it.
  public export
  preIntro : {bs : Bindings} -> Effect bs -> Bindings
  preIntro (DealDamage src amt to) = nomIntro to
  preIntro (DealDamageOwn src c to) = nomIntro to
  preIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  preIntro (Distribute v amt among) = nomIntro among
  preIntro (Fights a b) = nomIntro b
  preIntro (SetStatus _ n) = nomIntro n
  preIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  preIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  preIntro (ExtraTurn w count) = amtDelta count ++ nomIntro w
  preIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  preIntro (GetsCounters who amt _) = amtIntro amt
  preIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  preIntro (LosesCounters who _ amt) = optAmtIntro amt
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (BecomesBlocking _ what) = nomIntro what
  preIntro (StopsBlocking _ what) = nomIntro what
  preIntro (BecomesAttacking n NoDefender) = nomIntro n
  preIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  preIntro (Regenerate n) = nomIntro n
  preIntro (CantBe e _ _) = preIntro e
  preIntro (GainsDesignation n _ _ _) = nomIntro n
  preIntro (GameBecomes _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro (CounterSpell what) = nomIntro what
  preIntro (CopyStack agent what times exc) = amtIntro times
  preIntro (ChooseNewTargets what) = nomIntro what
  preIntro (Choose n _) = chosenIntro n
  preIntro (Move what to _) = nomIntro what
  preIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  preIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  preIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  preIntro (AddMana who amt _ _) = amtIntro amt
  preIntro (Draw who amt) = amtIntro amt
  preIntro (Expose v who what) = exposedIntro what
  preIntro (Search who sc q p) =
    quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who
  preIntro (Shuffle whose) = nomIntro whose
  preIntro (FlipCoins who count) = flipScopeIntro count
  preIntro (RollDice who count _) = amtIntro count
  preIntro (ResultsTable rows) = bs
  preIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  preIntro (ShiftResult amt) = amtIntro amt
  preIntro (RollPlanarDie who count) = amtIntro count
  preIntro ChaosEnsues = bs
  preIntro (StoreResults on) = nomIntro on
  preIntro (RerollStored _ _ whose) = nomIntro whose
  preIntro (Continuously se _) = staticIntro se
  preIntro (Throughout _ se) = staticIntro se
  preIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  preIntro (GetsEmblem who _) = nomIntro who
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters amt kind from) = nomIntro from
  preIntro (MoveCounters amt kind src dst) = nomIntro dst
  preIntro (PutSameCounters src dst) = nomIntro dst
  preIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  preIntro (GiveCountersOfOwnKinds on) = nomIntro on
  preIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  preIntro (Enact v (Move what to _)) = nomIntro what
  preIntro (Enact _ e) = preIntro e
  preIntro (Does s v (Move what to _)) = nomIntro what
  preIntro (Does s v e) = preIntro e
  preIntro (Pay who c) = nomIntro who
  preIntro (May d body did notd) = mayIntro body did notd
  preIntro (OnlyIf e c oth) = annIntro e
  preIntro (If c e oth) = bs
  preIntro (Unless e who c) = annIntro e
  preIntro (Define l amt) = defineLetter l (amtIntro amt)
  preIntro (ForEachOf _ _) = bs
  preIntro (ForEachKindOf _ _ _ _) = bs
  preIntro (Repeat _) = bs
  preIntro (Repeated n _) = amtIntro n
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = quantDelta q ++ bs
  preIntro (Delayed ev _ _ e) = bs
  preIntro (Reflexively body trig) = preIntro body
  preIntro (ThisWay body ev trig) = preIntro body
  preIntro (InsteadOf replaced repl) = annIntro replaced
  preIntro (HeldUntil e ev) = annIntro e

  ||| What a RIDER reads -- the context its subject is typed in.
  ||| [CR#608.2c] reads a card's later text against its earlier text as
  ||| one statement rather than step by step, and takes THIS sentence as
  ||| its worked example ("Destroy target creature. It can't be
  ||| regenerated."). So the rider's subject names the referent as the
  ||| clause it rides left it -- carrying that clause's own label -- and
  ||| not as a following sentence would find it: [CR#701.19a] regenerates
  ||| a PERMANENT, and a subject reachable only in a graveyard is the
  ||| wrong subject.
  ||| It is `preIntro` with the clause's label written IN PLACE, never a
  ||| re-zoning. That is the shape `settleTargets` and `defineLetter`
  ||| already have -- one mark changed on a binding the prefix already
  ||| held, nothing minted and nothing dropped -- so every gate that was
  ||| a fold over the prefix still is one, and the bare pronoun still
  ||| counts what it counted. What the mark buys is the participle
  ||| ("creatures destroyed this way") and the verb-scoped pronoun,
  ||| which ask which label acted and cannot ask it of an unmarked
  ||| binding.
  ||| The WRAPPER rows recurse with `riderIntro` and not with `preIntro`:
  ||| a rider hangs off the sentence, and the label the sentence's last
  ||| clause wrote is the label the rider names, whatever wrapper the
  ||| sentence was written under. Falling through to `preIntro` there
  ||| dropped the stamp at every seam.
  ||| `Modal` does NOT recurse: the rider follows the list, and which
  ||| mode's label it names is a question the mode's chooser answers at
  ||| resolution, not one the text fixes -- so the wrapper's own
  ||| announcement stands and the verb-scoped reads find nothing.
  ||| `If`/`OnlyIf`/`Unless` do not recurse either: their clause may not
  ||| have run, so the stamp its label would have written may not exist.
  public export
  riderIntro : {bs : Bindings} -> Effect bs -> Bindings
  riderIntro (Enact v (Move what to _)) = stampIntro (Just v) what
  riderIntro (Does s v (Move what to _)) = stampIntro (Just v) what
  riderIntro (Enact _ e) = riderIntro e
  riderIntro (Does _ _ e) = riderIntro e
  riderIntro (CantBe e _ _) = riderIntro e
  riderIntro (Reflexively body _) = riderIntro body
  riderIntro (ThisWay body _ _) = riderIntro body
  riderIntro (Sequentially es) = riderIntros es
  riderIntro e = preIntro e

  ||| A sequence's rider reads its LAST clause, on `preIntros`' model:
  ||| [CR#608.2c] reads the sentence as one statement, and the rider is
  ||| written after the last thing the sentence did.
  public export
  riderIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  riderIntros [] = bs
  riderIntros (e :: []) = riderIntro e
  riderIntros (e :: es) = riderIntros es

  ||| What a clause's phrases have named by the time the spell is cast
  ||| [CR#601.2c] — the announcement channel, distinct from `effIntro`
  ||| (what the clause did) and `preIntro` (its flat-clause approximation).
  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
  annIntro (DealDamageOwn src c to) = nomIntro to
  annIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  annIntro (Distribute v amt among) = nomIntro among
  annIntro (Fights a b) = nomIntro b
  annIntro (SetStatus _ n) = nomIntro n
  annIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  annIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  annIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  annIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  annIntro (GetsCounters who amt _) = amtIntro amt
  annIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  annIntro (LosesCounters who _ amt) = optAmtIntro amt
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (BecomesBlocking _ what) = nomIntro what
  annIntro (StopsBlocking _ what) = nomIntro what
  annIntro (BecomesAttacking n NoDefender) = nomIntro n
  annIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  annIntro (Regenerate n) = nomIntro n
  annIntro (CantBe e _ _) = annIntro e
  annIntro (GainsDesignation n _ _ _) = nomIntro n
  annIntro (GameBecomes _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro (CounterSpell what) = nomIntro what
  annIntro (CopyStack agent what times exc) = amtIntro times
  annIntro (ChooseNewTargets what) = nomIntro what
  annIntro (Choose n _) = chosenIntro n
  annIntro (Move what to _) = nomIntro what
  annIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  annIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  annIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  annIntro (AddMana who amt _ _) = amtIntro amt
  annIntro (Draw who amt) = amtIntro amt
  annIntro (Expose v who what) = exposedIntro what
  annIntro (Search who sc q p) =
    quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who
  annIntro (Shuffle whose) = nomIntro whose
  annIntro (FlipCoins who count) = flipScopeIntro count
  annIntro (RollDice who count _) = amtIntro count
  annIntro (ResultsTable rows) = bs
  annIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  annIntro (ShiftResult amt) = amtIntro amt
  annIntro (RollPlanarDie who count) = amtIntro count
  annIntro ChaosEnsues = bs
  annIntro (StoreResults on) = nomIntro on
  annIntro (RerollStored _ _ whose) = nomIntro whose
  annIntro (Continuously se _) = staticIntro se
  annIntro (Throughout _ se) = staticIntro se
  annIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  annIntro (GetsEmblem who _) = nomIntro who
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters amt kind from) = nomIntro from
  annIntro (MoveCounters amt kind src dst) = nomIntro dst
  annIntro (PutSameCounters src dst) = nomIntro dst
  annIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  annIntro (GiveCountersOfOwnKinds on) = nomIntro on
  annIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  annIntro (Enact v (Move what to _)) = nomIntro what
  annIntro (Enact _ e) = annIntro e
  annIntro (Does s v (Move what to _)) = nomIntro what
  annIntro (Does s v e) = annIntro e
  annIntro (Pay who c) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (OnlyIf e c oth) = annIntro e
  annIntro (If c e oth) = bs
  annIntro (Unless e who c) = annIntro e
  annIntro (Define l amt) = defineLetter l (amtIntro amt)
  annIntro (ForEachOf _ _) = bs
  annIntro (ForEachKindOf _ _ _ _) = bs
  annIntro (Repeat _) = bs
  annIntro (Repeated n _) = amtIntro n
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = quantDelta q ++ bs
  annIntro (Delayed ev _ _ e) = bs
  annIntro (Reflexively body trig) = annIntro body
  annIntro (ThisWay body ev trig) = annIntro body
  annIntro (InsteadOf replaced repl) = annIntro replaced
  annIntro (HeldUntil e ev) = annIntro e

  public export
  annSims : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  annSims [] = bs
  annSims (e :: es) = annSims es

  ||| What a replacement clause reads back. [CR#614.6] makes the replaced
  ||| event never happen, but its announcement still names the quantity —
  ||| "deals double THAT damage instead" — so the replaced deed's own
  ||| outcome is in scope here. A simultaneous list keeps its own telescope
  ||| [CR#608.2f] and is untouched.
  ||| It admits the whole deed delta where `otherwiseCtx` filters the same
  ||| shape down to outcomes, and the asymmetry is deliberate. What the
  ||| replaced clause left is a DEFINITION of characteristics [CR#111.3],
  ||| which every printed line of this family reads back — "instead create
  ||| those tokens plus an additional Food token", "creates twice that
  ||| many of those tokens instead", 14 supported lines, all of them a
  ||| definition or a magnitude and none of them the objects. An
  ||| "otherwise" arm has no such definition to read: its branch was not
  ||| taken and described nothing.
  ||| The object read rides the SAME binding as the definition read —
  ||| `Them` and `TokenAsThose` both find it — so no determiner-aware
  ||| filter separates them: the discriminator is which reader asks, not
  ||| what the binding records. The object read is therefore an
  ||| overgeneration RECORDED at its measured count of zero printed lines
  ||| rather than pinned, per the round that settled what a pin refuses.
  public export
  replacedCtx : {bs : Bindings} -> Effect bs -> Bindings
  replacedCtx (Sequentially es) = annSeqs es
  replacedCtx (May d body did notd) = replacedCtx body
  replacedCtx (OnlyIf e c oth) = replacedCtx e
  replacedCtx (If c e oth) = bs
  replacedCtx (Unless e who c) = replacedCtx e
  replacedCtx e = deedDelta e ++ annIntro e

  ||| The context an "otherwise" arm reads. It is the same ability's
  ||| alternative and runs when the condition failed, so it is typed as if
  ||| the branch it replaces were a deed that never happened: the phrases
  ||| that branch announced at casting [CR#601.2c] and the quantity it
  ||| wrote ("that much"), but none of the referents the deed would have
  ||| made — a token it would have created names nothing here.
  public export
  otherwiseCtx : {bs : Bindings} -> Effect bs -> Bindings
  otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e

  ||| A sequence announces every deed it strings together, so a replacement
  ||| over the whole sequence reads the quantity any of them named.
  public export
  annSeqs : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  annSeqs [] = bs
  annSeqs (e :: es) = deedDelta e ++ annSeqs es

  ||| The per-deed delta a simultaneous list sums — not the whole
  ||| `effIntro` answer, since the announcement telescope already carries
  ||| each element's phrases and folding it whole would double-count
  ||| [CR#608.2f].
  public export
  deedDelta : {bs : Bindings} -> Effect bs -> List Binding
  deedDelta (DealDamage src amt to) = [outcomeB DamageDealt]
  deedDelta (DealDamageOwn src c to) = [outcomeB DamageDealt]
  deedDelta (ControllerSacrifices _) = []
  deedDelta (Distribute (DividedDamage _) amt among) = [outcomeB DamageDealt]
  deedDelta (Distribute (DistributedCounters _) amt among) = []
  deedDelta (Fights a b) = []
  deedDelta (SetStatus _ _) = []
  deedDelta (DoesntUntapNext _ _) = []
  deedDelta (SkipsNext _ _ _) = []
  deedDelta (ExtraTurn _ _) = []
  deedDelta (AdditionalPart _ _ _ _) = []
  deedDelta (GetsCounters _ _ _) = []
  deedDelta (GetsCountersOfThoseKinds _ _) = []
  deedDelta (LosesCounters _ _ _) = []
  deedDelta (RemoveFromCombat _) = []
  deedDelta (BecomesBlocking _ _) = []
  deedDelta (StopsBlocking _ _) = []
  deedDelta (BecomesAttacking _ _) = []
  deedDelta (Regenerate _) = []
  deedDelta (CantBe e _ _) = deedDelta e
  deedDelta (GainsDesignation _ _ _ _) = []
  deedDelta (GameBecomes _) = []
  deedDelta (Concludes _ _) = []
  deedDelta GameDrawn = []
  deedDelta (CounterSpell _) = []
  deedDelta (CopyStack agent what times exc) =
    [MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
               (ObjectP (nounTy what) (Just Stack) Nothing (Just CopyOrigin))]
  deedDelta (ChooseNewTargets _) = []
  deedDelta (Choose n _) = []
  deedDelta (Move what to _) = []
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (AddMana _ _ _ _) = []
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Search who sc q p) =
    [MkBinding AD Object (quantPlur q)
               (ObjectP (seedTy p) (searchZone sc) Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (FlipCoins _ _) = [outcomeB CoinFlipped]
  deedDelta (RollDice _ _ _) = [outcomeB RollResult]
  deedDelta (ResultsTable _) = []
  -- an ignore takes rolls away and a shift restates one; neither
  -- leaves a number the roll had not already left.
  deedDelta (IgnoreOutcomes _) = []
  deedDelta (ShiftResult _) = []
  -- [CR#706.7]: no numerical result to announce.
  deedDelta (RollPlanarDie _ _) = [outcomeB PlanarRolled]
  deedDelta ChaosEnsues = []
  deedDelta (StoreResults _) = []
  deedDelta (RerollStored _ _ _) = []
  deedDelta (Continuously se _) = []
  deedDelta (Throughout _ _) = []
  deedDelta (Create agent count spec riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin))]
  deedDelta (GetsEmblem _ _) = []
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters amt kind from) = []
  deedDelta (MoveCounters amt kind src dst) = []
  deedDelta (PutSameCounters src dst) = []
  deedDelta (PutCountersOfThoseKinds amt on) = []
  deedDelta (GiveCountersOfOwnKinds on) = []
  deedDelta (DoubleCountersOfOwnKinds on) = []
  deedDelta (Enact v (Move what to _)) = []
  deedDelta (Enact _ e) = deedDelta e
  deedDelta (Does s v (Move what to _)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (Pay who c) = []
  deedDelta (May d body did notd) = []
  deedDelta (OnlyIf e c oth) = []
  deedDelta (If c e oth) = []
  deedDelta (Unless e who c) = []
  deedDelta (Define _ _) = []
  deedDelta (ForEachOf _ _) = []
  deedDelta (ForEachKindOf _ _ _ _) = []
  deedDelta (Repeat _) = []
  deedDelta (Repeated _ _) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev _ _ e) = []
  deedDelta (Reflexively body trig) = deedDelta body
  deedDelta (ThisWay body ev trig) = deedDelta body
  deedDelta (InsteadOf replaced repl) = []
  deedDelta (HeldUntil e ev) = []

  public export
  preIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  preIntros [] = bs
  preIntros (e :: []) = preIntro e
  preIntros (e :: es) = preIntros es

  ||| The context a may's body and its declined arm are typed in: an
  ||| offered may writes its decider first [CR#118.12], so the body reads
  ||| it; the mandatory form writes none.
  public export
  mayCtx : {bs : Bindings} -> Maybe (Noun bs Player) -> Bindings
  mayCtx Nothing = bs
  mayCtx (Just d) = nomIntro d

  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bindings
  mayIntro body Nothing Nothing = effIntro body
  mayIntro body (Just did) Nothing = effIntro did
  mayIntro body Nothing (Just notd) = effIntro body
  mayIntro body (Just did) (Just notd) = effIntro body

  ||| The post-state a reflexive trigger's body is typed in: it fires
  ||| because the action was taken [CR#603.12], so it reads `effIntro`
  ||| rather than just the announcement.
  public export
  reflexCtx : {bs : Bindings} -> Effect bs -> Bindings
  reflexCtx body = settleTargets (effIntro body)

  ||| The post-state a "this way" trigger's body is typed in: the event
  ||| has happened, so it reads the event's after-discourse with the
  ||| enclosure's targets settled [CR#603.12,603.6,601.2c].
  public export
  thisWayCtx : {bs : Bindings} -> (body : Effect bs) ->
               GameEvent (effIntro body) -> Bindings
  thisWayCtx body ev = settleTargets (eventAfter ev)

  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

  ||| A repetition writes ONE instruction and a count, so a cost that
  ||| spells one is judged by that instruction. The `Sequentially`
  ||| refusal above targets a cost written as a COORDINATION -- "do A,
  ||| then B", whose components [CR#601.2h] may be paid in any order --
  ||| and "do A n times" is not one: every pass is the same instruction,
  ||| and the canonical iterated-singular expansion writes the choice and
  ||| the act as two steps of that one action. So the body's own sequence
  ||| is looked through and each step judged, which is what lets
  ||| "Discard two cards:" be a cost cards print.
  public export
  costRepeatedOk : {0 bs : Bindings} -> Effect bs -> Bool
  costRepeatedOk (Sequentially es) = costStepsOk es
  costRepeatedOk e = costActionOk e

  public export
  costStepsOk : {0 bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bool
  costStepsOk [] = True
  costStepsOk (e :: es) = costActionOk e && costStepsOk es

  public export
  simIntro : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simIntro [] = bs
  simIntro (e :: es) = deedDelta e ++ simIntro es

  public export
  simPres : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simPres [] = bs
  simPres (e :: []) = preIntro e
  simPres (e :: es) = simPres es

  public export
  data KeywordParam : Bindings -> Type where
    ParamCost : Cost [] -> KeywordParam bs
    ParamQuality : Predicate bs Object -> KeywordParam bs
    ParamSubject : {k : Kind} -> (p : Predicate [] k) -> KeywordParam bs
    ParamNumber : (amt : Amount []) -> 
                  KeywordParam bs

  public export
  paramShapeOf : {0 bs : Bindings} -> Maybe (KeywordParam bs) -> KeywordParamShape
  paramShapeOf Nothing = NoParam
  paramShapeOf (Just (ParamCost _)) = CostParam
  paramShapeOf (Just (ParamQuality _)) = QualityParam
  paramShapeOf (Just (ParamSubject _)) = SubjectParam
  paramShapeOf (Just (ParamNumber _)) = NumberParam

  public export
  keywordParamFits : {0 bs : Bindings} -> KeywordLabel -> Maybe (KeywordParam bs) -> Bool
  -- Knownness is folded in, so this ONE gate refuses a typo as well as a
  -- mismatched parameter: an unknown word has no parameter shape to fit.
  keywordParamFits k p = knownKeyword k && keywordParamShape k == paramShapeOf p

  public export
  KeywordParamFits : KeywordLabel -> Maybe (KeywordParam bs) -> Type
  KeywordParamFits {bs} k p = So (keywordParamFits k p)

  public export
  data AbilityAt : Bindings -> Type where
    KeywordAbility : (k : KeywordLabel) ->
                     (param : Maybe (KeywordParam bs)) ->
                     {auto 0 pf : KeywordParamFits k param} -> AbilityAt bs
    Activated : (cost : Cost bs) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                (window : Maybe Timing) ->
                (limit : Maybe UsageLimit) ->
                (guard : Maybe (Condition bs)) ->
                AbilityAt bs
    ||| The trigger header, its concurrent clause, and its joined second
    ||| header. `while` belongs to the head event [CR#603.1,603.2] and
    ||| `joins` is a second whole header over this one effect; both sit
    ||| BEFORE the intervening slot, which [CR#603.4] places after the
    ||| whole trigger condition and which is checked again on resolution.
    Triggered : (word : TriggerWord) -> (ev : GameEvent bs) ->
                (alts : List (GameEvent bs)) ->
                (while : Maybe (Concurrent (headerCtx alts ev))) ->
                (joins : List (JoinedHeader bs)) ->
                (window : Maybe TriggerWindow) ->
                (limit : Maybe UsageLimit) ->
                (intervening :
                   Maybe (Condition (joinedCtx joins (headerCtx alts ev)))) ->
                (eff : Effect (interveningIntro intervening)) ->
                {auto 0 hn : HeaderNontarget ev} ->
                {auto 0 ae : AltEvent word alts} ->
                {auto 0 cd :
                   ChapterDefaults ev alts while joins window limit intervening} ->
                AbilityAt bs
    Static : (se : StaticEffect bs) ->
             {auto 0 ut : Untargeting se} -> AbilityAt bs
    Spell : (eff : Effect bs) -> AbilityAt bs
    AlsoForKeywords : (ab : AbilityAt bs) -> (ks : List KeywordLabel) ->
                      {auto 0 ex : KeywordExtendable ab} ->
                      {auto 0 lk : KeywordListOk ab ks} -> AbilityAt bs
    ||| [CR#207.2c]: an ability word prefixes an ability of any kind and has
    ||| no special rules meaning, so it wraps `AbilityAt` — the type
    ||| enclosing every ability kind — rather than any one of them, and the
    ||| rules-facing functions below read straight through it. Homed in this
    ||| grammar by user ruling: with no rules meaning there is nothing for
    ||| the core ability model or its `.ron` kinds to carry, and the word is
    ||| printed text the spelling layer must still be able to write.
    ||| The rule puts the word at the *beginning* of the ability, so it is
    ||| the outermost wrapper: `lineKeyword` reads no keyword through it,
    ||| which keeps `AlsoForKeywords` inside the word rather than outside.
    AbilityWord : (word : AbilityWordName) -> (ab : AbilityAt bs) ->
                  {auto 0 nw : NotAbilityWorded ab} -> AbilityAt bs


  ||| One word per ability: [CR#207.2c] gives the word the beginning of an
  ||| ability, and a word wrapping a worded ability spells no second
  ||| beginning — the inner ability is the same ability. Mirrors the
  ||| no-nesting gate `AlsoForKeywords` gets from `lineKeyword`.
  public export
  notAbilityWorded : {0 bs : Bindings} -> AbilityAt bs -> Bool
  notAbilityWorded (AbilityWord _ _) = False
  notAbilityWorded _ = True

  public export
  NotAbilityWorded : AbilityAt bs -> Type
  NotAbilityWorded {bs} ab = So (notAbilityWorded ab)

  public export
  Untargeting : {bs : Bindings} -> StaticEffect bs -> Type
  Untargeting {bs} se = So (not (anyTargetedAt (staticIntro se)))

  public export
  lineKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  lineKeyword (Static se) = statKeyword se
  lineKeyword (Triggered _ _ _ _ _ _ _ _ eff) = effKeyword eff
  lineKeyword _ = Nothing

  public export
  statKeyword : {0 bs : Bindings} -> StaticEffect bs -> Maybe KeywordLabel
  statKeyword (Conditionally _ se _) = statKeyword se
  statKeyword (OnlyWhile se _ _) = statKeyword se
  statKeyword (Gains _ ab) = grantedKeyword ab
  statKeyword _ = Nothing

  public export
  effKeyword : {0 bs : Bindings} -> Effect bs -> Maybe KeywordLabel
  effKeyword (Continuously se _) = statKeyword se
  effKeyword (Throughout _ se) = statKeyword se
  effKeyword _ = Nothing

  public export
  grantedKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  grantedKeyword (KeywordAbility k Nothing) =
    if keywordParamless k then Just k else Nothing
  grantedKeyword _ = Nothing

  public export
  keywordExtendableOk : {0 bs : Bindings} -> AbilityAt bs -> Bool
  keywordExtendableOk ab = case lineKeyword ab of
    Nothing => False
    Just _ => True

  public export
  keywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordLabel -> Bool
  keywordListOk ab ks = case lineKeyword ab of
    Nothing => False
    Just base => not (isNil ks) && allParamless ks && distinctKeywords ks &&
                 not (elem base ks)


  public export
  allParamless : List KeywordLabel -> Bool
  allParamless [] = True
  allParamless (k :: ks) = keywordParamless k && allParamless ks

  public export
  distinctKeywords : List KeywordLabel -> Bool
  distinctKeywords [] = True
  distinctKeywords (k :: ks) = not (elem k ks) && distinctKeywords ks

  public export
  KeywordExtendable : {0 bs : Bindings} -> AbilityAt bs -> Type
  KeywordExtendable {bs} ab = So (keywordExtendableOk ab)

  public export
  KeywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordLabel -> Type
  KeywordListOk {bs} ab ks = So (keywordListOk ab ks)

  ||| [CR#613.1f] applies ability-adding effects over abilities as such,
  ||| and [CR#113.3] gives four kinds of them; neither rule restricts
  ||| which kind an effect may add, nor says how the added text is
  ||| written, so whether the grant puts the ability in quotation marks
  ||| is rendering and no cell here indexes on it. The two Falses
  ||| are structural, not measured: a `Spell` line is [CR#113.3a]'s spell
  ||| ability, followed only while an instant or sorcery spell resolves,
  ||| so an object handed one has nothing to follow; and
  ||| `AlsoForKeywords` is not an ability at all but the spelling that
  ||| extends one line's keyword list, so a grant naming it would name no
  ||| second ability.
  public export
  grantableAb : {0 bs : Bindings} -> AbilityAt bs -> Bool
  grantableAb (KeywordAbility _ _) = True
  grantableAb (Activated _ _ _ _ _) = True
  grantableAb (Triggered _ _ _ _ _ _ _ _ _) = True
  grantableAb (Static _) = True
  grantableAb (Spell _) = False
  grantableAb (AlsoForKeywords _ _) = False
  grantableAb (AbilityWord _ ab) = grantableAb ab

  public export
  Grantable : AbilityAt bs -> Type
  Grantable {bs} ab = So (grantableAb ab)

  public export
  emblemAbilityOk : AbilityAt [] -> Bool
  emblemAbilityOk (KeywordAbility _ _) = False
  emblemAbilityOk (Activated _ _ _ _ _) = True
  emblemAbilityOk (Triggered _ _ _ _ _ _ _ _ _) = True
  emblemAbilityOk (Static _) = True
  emblemAbilityOk (AlsoForKeywords _ _) = False
  emblemAbilityOk (AbilityWord _ ab) = emblemAbilityOk ab
  emblemAbilityOk (Spell _) = False

  public export
  emblemAbilitiesAll : List (AbilityAt []) -> Bool
  emblemAbilitiesAll [] = True
  emblemAbilitiesAll (a :: as) = emblemAbilityOk a && emblemAbilitiesAll as

  public export
  emblemAbilitiesOk : List (AbilityAt []) -> Bool
  emblemAbilitiesOk [] = False
  emblemAbilitiesOk (a :: as) = emblemAbilityOk a && emblemAbilitiesAll as

  public export
  EmblemAbilities : List (AbilityAt []) -> Type
  EmblemAbilities abl = So (emblemAbilitiesOk abl)

  public export
  abilitiesGrantable : List (AbilityAt []) -> Bool
  abilitiesGrantable [] = True
  abilitiesGrantable (a :: as) = grantableAb a && abilitiesGrantable as

  public export
  tokenAbilitiesOk : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenAbilitiesOk t = abilitiesGrantable t.abilities

  public export
  TokenAbilities : TokenChars bs -> Type
  TokenAbilities {bs} t = So (tokenAbilitiesOk t)

  public export
  predRegime : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> Maybe StackRegime
  predRegime (CastBy _) = Just AtCasting
  predRegime (CastFrom _) = Just AtCasting
  predRegime WasCast = Just AtCasting
  predRegime (ControlledBy _) = Just AtResolution
  predRegime (And ps) = predRegimeAll ps
  predRegime (Or ps) = predRegimeAll ps
  predRegime _ = Nothing

  public export
  predRegimeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Maybe StackRegime
  predRegimeAll [] = Nothing
  predRegimeAll (p :: ps) = case predRegime p of
    Just r => Just r
    Nothing => predRegimeAll ps

  public export
  nounRegime : {bs : Bindings} -> Noun bs Object -> Maybe StackRegime
  nounRegime (AllOf p) = predRegime p
  nounRegime (Indefinite _ p) = predRegime p
  nounRegime (Definite p) = predRegime p
  nounRegime (Each p) = predRegime p
  nounRegime (TargetGroup _ p) = predRegime p
  nounRegime (CountedGroup _ _ p) = predRegime p
  nounRegime (NamesAgree _ grp) = nounRegime grp
  nounRegime _ = Nothing

  public export
  abRegime : {0 bs : Bindings} -> AbilityAt bs -> Maybe StackRegime
  abRegime (KeywordAbility k _) = keywordStackRegime k
  abRegime (Activated _ _ _ _ _) = Nothing
  abRegime (Triggered _ _ _ _ _ _ _ _ _) = Nothing
  abRegime (Static _) = Nothing
  abRegime (AlsoForKeywords ab _) = abRegime ab
  abRegime (AbilityWord _ ab) = abRegime ab
  abRegime (Spell _) = Nothing

  public export
  castingOnly : Maybe StackRegime -> Bool
  castingOnly (Just AtCasting) = True
  castingOnly _ = False

  public export
  regimeMatches : Maybe StackRegime -> Maybe StackRegime -> Bool
  regimeMatches (Just a) (Just b) = a == b
  regimeMatches _ _ = False

  public export
  ||| [CR#113.6b]: an ability that states which zones it functions in
  ||| functions from those zones, so a grant may name a subject in any
  ||| zone. The one refusal off the stack is [CR#113.6e]'s: an ability
  ||| granting an object another ability that modifies how that object is
  ||| played or cast functions only on the stack.
  grantSubjectOk : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Bool
  grantSubjectOk ab n = grantSubjectFits (nounZone n) (nounRegime n) ab

  ||| `grantSubjectOk` asked of the two facts it reads off the subject,
  ||| so a coordination whose parts are typed in a LATER context than
  ||| its subject can still ask it. Same rule, same two lines.
  public export
  grantSubjectFits : {0 bs : Bindings} -> Maybe Zone -> Maybe StackRegime ->
                     AbilityAt bs -> Bool
  grantSubjectFits zn reg ab =
    if onStackZone zn
      then regimeMatches (abRegime ab) reg
      else not (castingOnly (abRegime ab))

  public export
  GrantSubject : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Type
  GrantSubject {bs} ab n = So (grantSubjectOk ab n)

  ||| The choice an EFFECT binds, for the abilities after it to read.
  ||| `staticChoiceDelta`'s twin at the chooser positions the as-enters
  ||| rider does not reach -- a combat or upkeep trigger, an activated
  ||| ability, a Saga chapter -- and it is the CONTAINER those positions
  ||| wanted: the `Choose` clause already spells the choice and already
  ||| leaves the binding, and what was missing was the export across the
  ||| ability boundary that [CR#607.2d] links over. It runs at every
  ||| ability with a body, the spell ability included, because
  ||| [CR#607.2d] links two abilities printed on one object and says
  ||| nothing about their kinds; a keyword ability has no body to look
  ||| at.
  ||| Only the composition rows a printed chooser sits inside recurse:
  ||| a coordination ("Put a sleight counter on this Aura and choose a
  ||| color", Chromatic Armor) and an optional one ("you may choose a
  ||| number between 0 and 7", Shapeshifter), the latter on
  ||| `mayIntro`'s standing precedent that a may-body's announcements
  ||| are what the clause leaves. A chooser under a CONDITION exports
  ||| nothing, which is undergeneration and not a refusal; no supported
  ||| line writes one.
  ||| A PLURAL choice exports nothing either: `countChoice` counts
  ||| singular bindings, the plural chooser's reads are a separate and
  ||| declined cell, and exporting a singular binding for "choose two
  ||| colors" would let the singular read name one of two.
  public export
  effChoiceDelta : {0 bs : Bindings} -> Effect bs -> List Binding
  effChoiceDelta (Choose {k} (Indefinite _ _) _) = choiceDeltaAt k
  effChoiceDelta (Choose _ _) = []
  effChoiceDelta (Sequentially es) = effsChoiceDelta es
  effChoiceDelta (May _ body _ _) = effChoiceDelta body
  effChoiceDelta _ = []

  public export
  effsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                    Effects n bs -> List Binding
  effsChoiceDelta [] = []
  effsChoiceDelta (e :: es) = effsChoiceDelta es ++ effChoiceDelta e

  public export
  abIntro : {bs : Bindings} -> AbilityAt bs -> Bindings
  abIntro (KeywordAbility _ _) = bs
  abIntro (Activated _ eff _ _ _) = effChoiceDelta eff ++ bs
  abIntro (Triggered _ _ _ _ _ _ _ _ eff) = effChoiceDelta eff ++ bs
  abIntro (Static se) = staticChoiceIntro se
  abIntro (AlsoForKeywords ab _) = abIntro ab
  abIntro (AbilityWord _ ab) = abIntro ab
  abIntro (Spell eff) = effChoiceDelta eff ++ bs

  namespace Coord
    public export
    data StaticParts : Nat -> Bindings -> Type where
      Nil : StaticParts Z bs
      (::) : (se : StaticEffect bs) -> {auto 0 nc : NotCoord se} ->
             StaticParts n (staticIntro se) -> StaticParts (S n) bs

  namespace Shared
    ||| One statement's VERB PHRASE with its subject left out: what a
    ||| shared-subject coordination coordinates. Each arm is the slot
    ||| list of the `StaticEffect` row that spells it, minus the subject
    ||| the coordination writes once -- so no arm names a referent, and
    ||| the elided subject of English's second conjunct is elided here
    ||| too rather than pronominalised.
    ||| Two arms, which are the two the corpus coordinates: the P/T
    ||| shift (`Gets`) and the ability grant (`Gains`, which spells both
    ||| "gains" and "has"). The vocabulary is OPEN in the same sense
    ||| `VerbLabel` is -- an arm is a row here and a clause in `vpOk`,
    ||| and adding one obliges nothing else -- and the statements the
    ||| corpus coordinates by writing the subject twice, or by reading it
    ||| back, keep writing `AndAlso`.
    public export
    data SubjectVP : Bindings -> Type where
      ||| `Gets`' slots: "… gets +4/+4 …".
      VPGets : (pow : PtShift bs) -> (tou : PtShift (shiftIntro pow)) ->
               SubjectVP bs
      ||| `Gains`' slot: "… and gains trample", "… and has flying".
      VPGains : (ab : AbilityAt bs) -> SubjectVP bs

    ||| The parts, threaded left to right [CR#608.2c] on `StaticParts`'
    ||| model: each typed in the previous one's output.
    public export
    data SubjectVPs : Nat -> Bindings -> Type where
      Nil : SubjectVPs Z bs
      (::) : (vp : SubjectVP bs) -> SubjectVPs n (vpIntro vp) ->
             SubjectVPs (S n) bs

  ||| What one verb phrase announces: its own amounts, and nothing about
  ||| the subject -- which the coordination announced once, before any
  ||| part was typed.
  public export
  vpIntro : {bs : Bindings} -> SubjectVP bs -> Bindings
  vpIntro (VPGets pow tou) = shiftDelta tou ++ shiftDelta pow ++ bs
  vpIntro (VPGains _) = bs

  public export
  vpsIntro : {0 k : Nat} -> {bs : Bindings} -> SubjectVPs k bs -> Bindings
  vpsIntro [] = bs
  vpsIntro (vp :: rest) = vpsIntro rest

  ||| One part's own obligation, asked of the SHARED subject's two
  ||| facts: `Gets` wants a battlefield subject, `Gains` wants a subject
  ||| the ability may be granted to [CR#113.6e] and an ability that may
  ||| be granted at all.
  public export
  vpOk : {0 bs : Bindings} -> Maybe Zone -> Maybe StackRegime ->
         SubjectVP bs -> Bool
  vpOk zn reg (VPGets _ _) = zoneFits zn (Just Battlefield)
  vpOk zn reg (VPGains ab) = grantSubjectFits zn reg ab && grantableAb ab

  public export
  vpsOk : {0 k : Nat} -> {0 bs : Bindings} -> Maybe Zone ->
          Maybe StackRegime -> SubjectVPs k bs -> Bool
  vpsOk zn reg [] = True
  vpsOk zn reg (vp :: rest) = vpOk zn reg vp && vpsOk zn reg rest

  namespace Paid
    public export
    data CostSeq : Nat -> Bindings -> Type where
      Nil : CostSeq Z bs
      (::) : (c : Cost bs) -> {auto 0 nc : NotCompound c} ->
             CostSeq n (costIntro c) -> CostSeq (S n) bs

  namespace Text
    public export
    data AbilitySeq : Bindings -> Type where
      Nil : AbilitySeq bs
      (::) : (ab : AbilityAt bs) -> AbilitySeq (abIntro ab) -> AbilitySeq bs

  public export
  costsIntro : {bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bindings
  costsIntro [] = bs
  costsIntro (c :: cs) = costsIntro cs

  public export
  partsIntro : {0 n : Nat} -> {bs : Bindings} -> StaticParts n bs -> Bindings
  partsIntro [] = bs
  partsIntro (se :: rest) = partsIntro rest

  ||| Later parts bind NEARER, on `staticChoiceIntro`'s own order: a
  ||| coordination is read left to right [CR#608.2c] and the bindings
  ||| are nearest-first.
  public export
  partsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                     StaticParts n bs -> List Binding
  partsChoiceDelta [] = []
  partsChoiceDelta (se :: rest) = partsChoiceDelta rest ++ staticChoiceDelta se



  public export
  partsClauseOk : {0 n : Nat} -> {0 bs : Bindings} -> StaticParts n bs -> Bool
  partsClauseOk [] = True
  partsClauseOk (se :: rest) = clauseStaticOk se && partsClauseOk rest

  ||| Which continuous effects a resolving clause can establish. A
  ||| characteristic-defining ability is printed on the card it affects
  ||| [CR#604.3a], so no resolution establishes one. An alternative cost
  ||| modifies what one particular object costs to cast [CR#113.6d], and
  ||| `AltCost` names no object for the clause to price.
  public export
  clauseStaticOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
  clauseStaticOk (DefinesPt _ _ _) = False
  clauseStaticOk (AltCost _) = False
  clauseStaticOk (AndAlso parts) = partsClauseOk parts
  clauseStaticOk _ = True

  public export
  ClauseStatic : StaticEffect bs -> Type
  ClauseStatic {bs} se = So (clauseStaticOk se)

  public export
  selfTapPayment : {0 bs : Bindings} -> Cost bs -> Bool
  selfTapPayment (Mana _) = False
  selfTapPayment (ScaledMana _) = False
  selfTapPayment TapSymbol = True
  selfTapPayment UntapSymbol = True
  selfTapPayment (LoyaltySymbol _) = False
  selfTapPayment (Do _) = False
  selfTapPayment (Compound _) = False
  selfTapPayment ItsManaCost = False

  public export
  selfTapCount : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Nat
  selfTapCount [] = Z
  selfTapCount (c :: cs) =
    (if selfTapPayment c then S Z else Z) + selfTapCount cs

  public export
  selfTapOnce : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  selfTapOnce cs = lte (selfTapCount cs) 1

  public export
  costTapOnce : {0 bs : Bindings} -> Cost bs -> Bool
  costTapOnce (Mana _) = True
  costTapOnce (ScaledMana _) = True
  costTapOnce TapSymbol = True
  costTapOnce UntapSymbol = True
  costTapOnce (LoyaltySymbol _) = True
  costTapOnce (Do _) = True
  costTapOnce (Compound cs) = selfTapOnce cs
  costTapOnce ItsManaCost = True

  public export
  CostTapOnce : Cost bs -> Type
  CostTapOnce {bs} c = So (costTapOnce c)

  public export
  costPaidByYou : {0 bs : Bindings} -> Cost bs -> Bool
  costPaidByYou (Mana _) = True
  costPaidByYou (ScaledMana _) = True
  costPaidByYou TapSymbol = True
  costPaidByYou UntapSymbol = True
  costPaidByYou (LoyaltySymbol _) = True
  costPaidByYou (Do (ChangeLife who _)) = nounIsYou who
  costPaidByYou (Do (Does subj _ _)) = nounIsYou subj
  costPaidByYou (Do _) = True
  costPaidByYou (Compound cs) = costsPaidByYou cs
  costPaidByYou ItsManaCost = True

  public export
  costsPaidByYou : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsPaidByYou [] = True
  costsPaidByYou (c :: cs) = costPaidByYou c && costsPaidByYou cs

  public export
  CostPaidByYou : Cost bs -> Type
  CostPaidByYou {bs} c = So (costPaidByYou c)

  public export
  payAgreesOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
                Noun bs Player -> Cost cs -> Bool
  payAgreesOk who c = not (nounIsYou who) || costPaidByYou c

  public export
  PayAgrees : Noun bs Player -> Cost cs -> Type
  PayAgrees {bs} {cs} who c = So (payAgreesOk who c)

  public export
  costOffBattlefield : {0 bs : Bindings} -> Cost bs -> Bool
  costOffBattlefield (Mana _) = True
  costOffBattlefield (ScaledMana _) = True
  costOffBattlefield TapSymbol = False
  costOffBattlefield UntapSymbol = False
  costOffBattlefield (LoyaltySymbol _) = False
  costOffBattlefield (Do _) = True
  costOffBattlefield (Compound cs) = costsOffBattlefield cs
  costOffBattlefield ItsManaCost = True

  public export
  costsOffBattlefield : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsOffBattlefield [] = True
  costsOffBattlefield (c :: cs) = costOffBattlefield c && costsOffBattlefield cs

  public export
  data AltPayment : {0 bs : Bindings} -> Maybe (Cost bs) -> Type where
    NoAltPayment : AltPayment Nothing
    AltPaymentWritten : {0 c : Cost bs} ->
                        {auto 0 ok : So (costOffBattlefield c)} ->
                        AltPayment (Just c)

public export
Ability : Type
Ability = AbilityAt []
