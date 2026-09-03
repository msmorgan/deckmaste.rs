module Experimental.Effect

import public Experimental.Triggers

%default total

public export
data SpendPurpose : Bindings -> Type where
  ToCast : (p : Predicate bs Object) ->
           SpendPurpose bs
  ToActivate : (src : Maybe (Predicate bs Object)) ->
               SpendPurpose bs
  ToPay : (c : CostNamed) -> {auto 0 nm : CostNameable c} ->
          SpendPurpose bs

public export
data ManaHeld : Bindings -> Type where
  ThisMana : {auto 0 ok : countOutcomes ManaAdded bs = 1} -> ManaHeld bs
  UnspentMana : (ty : Maybe ColorOrColorless) -> ManaHeld bs

public export
data AsThough : Bindings -> Type where
  AsThoughOf : (p : Predicate bs Object) -> AsThough bs
  AsThoughMana : (what : Maybe ColorOrColorless) ->
                 (as : ManaMatch) ->
                 (purpose : Maybe (SpendPurpose bs)) -> AsThough bs
  AsThoughGreater : {bs : Bindings} -> (ch : Characteristic) ->
                    (amt : Amount bs) ->
                    {auto 0 nd : So (isNil (amtDelta amt))} -> AsThough bs
  AsThoughLess : {bs : Bindings} -> (ch : Characteristic) ->
                 (amt : Amount bs) ->
                 {auto 0 nd : So (isNil (amtDelta amt))} -> AsThough bs

public export
asThoughSort : {0 bs : Bindings} -> AsThough bs -> PremiseSort
asThoughSort (AsThoughOf _) = ObjectPremise
asThoughSort (AsThoughMana _ _ _) = ManaPremise
asThoughSort (AsThoughGreater _ _) = ValuePremise
asThoughSort (AsThoughLess _ _) = ValuePremise


public export
twoPartiesOk : {bs : Bindings} -> Noun bs Player -> Bool
twoPartiesOk (BothOf l r) = case (nounPlur l, nounPlur {bs = nomIntro l} r) of
  (OneOf, OneOf) => True
  _ => False
twoPartiesOk (TargetGroup q _) = quantExact q == Just 2
twoPartiesOk (CountedGroup q _ _) = quantExact q == Just 2
twoPartiesOk _ = False


public export
data TokenQuality : Bindings -> Type where
  WithEveryType : (space : TypeSpace) -> TokenQuality bs
  WithQuality : (q : Predicate bs Object) ->
                {auto 0 qr : QualityRead q} -> TokenQuality bs

public export
tokenQualHosted : {0 bs : Bindings} -> List CardType -> TokenQuality bs -> Bool
tokenQualHosted tys (WithEveryType space) =
  any (\t => spaceHosted space (Just t)) tys
tokenQualHosted tys (WithQuality q) = case qualityReadHost q of
                                        Nothing => True
                                        Just h => elem h tys


mutual
  public export
  record TokenChars (bs : Bindings) where
    constructor MkTokenChars
    pt : Maybe (p : Amount bs ** Amount (amtIntro p))
    colors : List Color
    supers : List Supertype
    line : TypeLine
    abilities : List (AbilityAt [])
    name : Maybe String
    quals : List (TokenQuality bs)

  public export
  tokenQualsFit : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenQualsFit t = all (tokenQualHosted t.line.tys) t.quals

  public export
  TokenQualsFit : TokenChars bs -> Type
  TokenQualsFit {bs} t = So (tokenQualsFit t)

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
                       && supersDistinct t.supers

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
                   {auto 0 tc : TokenCanonical t} ->
                   {auto 0 qf : TokenQualsFit t} -> TokenSpec bs
    TokenAsThose : {auto 0 ok : countTokenSpecs bs = 1} -> TokenSpec bs
    TokenCopyOf : (src : Noun bs Object) -> (exc : List (CopyExcept bs)) ->
                  {auto 0 pm : PerMember src} -> TokenSpec bs

  public export
  specHeadTy : {bs : Bindings} -> TokenSpec bs -> Maybe CardType
  specHeadTy (TokenWritten t) = tokenHeadTy t
  specHeadTy TokenAsThose = tyOfThoseAny TokenW bs
  specHeadTy (TokenCopyOf src _) = nounTy src

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

  public export
  sameDirection : {0 bs : Bindings} -> PtShift bs -> PtShift bs -> Bool
  sameDirection p t = if shiftRises p then shiftRises t else not (shiftRises t)

  public export
  data CostShift : Bindings -> Type where
    CostLess : (amt : Amount bs) -> (floor : Maybe (Amount bs)) -> CostShift bs
    CostMore : (amt : Amount bs) -> CostShift bs
    CostShiftRun : (run : ManaCost) -> (rises : Bool) ->
                   (coloredOnly : Bool) ->
                   {auto 0 wr : ManaRun run} -> CostShift bs
    CostShiftRunWithFloor : (run : ManaCost) -> (floor : Amount bs) ->
                            (coloredOnly : Bool) ->
                            {auto 0 wr : ManaRun run} -> CostShift bs

  public export
  costShiftDelta : {bs : Bindings} -> CostShift bs -> List Binding
  costShiftDelta (CostLess a _) = amtDelta a
  costShiftDelta (CostMore a) = amtDelta a
  costShiftDelta (CostShiftRun _ _ _) = []
  costShiftDelta (CostShiftRunWithFloor _ floor _) = amtDelta floor

  namespace Static
    public export
    data StaticEffect : Bindings -> Type where
      Gets : (n : Noun bs Object) -> (pow : PtShift (selfSubjIntro n)) ->
             (tou : PtShift (shiftIntro pow)) ->
             {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
             StaticEffect bs
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
      AltCost : {k : Kind} -> (n : Noun bs k) ->
                (c : Maybe (Cost (selfSubjIntro n))) ->
                {auto 0 ap : AltPayment c} ->
                {auto 0 cs : CostSubject n} -> StaticEffect bs
      AddedCost : (c : Cost bs) -> (offered : Bool) ->
                  {auto 0 ap : AddedPayment c} -> StaticEffect bs
      Define : (l : Letter) -> (amt : Amount bs) ->
               {auto 0 ok : So (anyOpenLetter l bs)} -> StaticEffect bs
      Gains : (n : Noun bs Object) -> (ab : AbilityAt bs) ->
              {auto 0 ok : GrantSubject ab n} ->
              {auto 0 gr : Grantable ab} -> StaticEffect bs
      GainsAbilitiesOf : (n : Noun bs Object) ->
                         (cls : List AbilityClass) ->
                         (src : Noun (nomIntro n) Object) ->
                         (except : Maybe (Predicate (nomIntro src) Ability)) ->
                         {auto 0 ne : NonEmpty cls} ->
                         {auto 0 dc : So (distinctClasses cls)} ->
                         StaticEffect bs
      Deontic : {k : Kind} -> (n : Noun bs k) ->
                (c : Compulsion (selfSubjIntro n)) ->
                (deeds : Deeds) -> (role : Role) ->
                (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
                (asThough : Maybe (AsThough (nomIntro n))) ->
                (rider : DeonticRider (deonticPatientIntro patient)) ->
                {auto 0 ne : NonEmpty deeds} ->
                {auto 0 dd : So (distinctDeeds deeds)} ->
                {auto 0 kd : KnownDeeds deeds} ->
                {auto 0 zn : ZoneFits (nounZone n) (deedsZone deeds role)} ->
                {auto 0 dp : DeedParticipant deeds role k (nounHeadTys n)} ->
                {auto 0 pt : So (deonticPatientOk n deeds role patient rider)} ->
                {auto 0 at : So (asThoughOk c deeds asThough)} ->
                {auto 0 rd : So (deonticRiderOk deeds role c patient
                                                (isJust asThough) rider)} ->
                StaticEffect bs
      KeepsUnspentMana : (who : Noun bs Player) ->
                         (what : ManaHeld (nomIntro who)) -> StaticEffect bs
      MayDeclineUntap : (n : Noun bs Object) ->
                        {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                        StaticEffect bs
      DoesntUntap : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      UntapsDuringStep : (n : Noun bs Object) ->
                         {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                         StaticEffect bs
      CantMoreThan : (who : Noun bs Player) -> (deed : VerbLabel) ->
                     (k : Nat) -> (p : Predicate bs Object) ->
                     {auto 0 kd : KnownDeed deed} ->
                     {auto 0 pk : So (deedKindOk deed Patient Object)} ->
                     {auto 0 zn : ZoneFits (seedZone p) (deedZoneOf deed Patient)} ->
                     StaticEffect bs
      Skips : (who : Noun bs Player) -> (part : TurnPart) -> StaticEffect bs
      BecomesAlso : (n : Noun bs Object) -> (added : TokenChars bs) ->
                    {auto 0 sw : AdditionSaysSomething (nounTy n) added} ->
                    {auto 0 af : AddedFits (nounTy n) added.line} ->
                    {auto 0 tc : TokenCanonical added} ->
                    {auto 0 ta : TokenAbilities added} ->
                    {auto 0 qf : TokenQualsFit added} ->
                    {auto 0 un : AdditionUnnamed added} -> StaticEffect bs
      AddsEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      {auto 0 sh : SpaceHosted space (nounTy n)} ->
                      StaticEffect bs
      LosesEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       {auto 0 sh : SpaceHosted space (nounTy n)} ->
                       StaticEffect bs
      LosesType : (n : Noun bs Object) -> (t : CardType) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  StaticEffect bs
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
                 {auto 0 qf : TokenQualsFit t} ->
                 {auto 0 ro : RetentionOk t.line ret} -> StaticEffect bs
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
      DoesntRemove : (se : StaticEffect bs) ->
                     (n : Noun (staticIntro se) Object) ->
                     {auto 0 nc : NotCarvedOut se} -> StaticEffect bs
      BecomesCopy : (n : Noun bs Object) -> (src : Noun (nomIntro n) Object) ->
                    (exc : List (CopyExcept (nomIntro src))) ->
                    {auto 0 pm : PerMember src} -> StaticEffect bs
      LosesAllAbilities : (n : Noun bs Object) ->
                          (except : Maybe (Predicate (selfSubjIntro n) Ability)) ->
                          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                          StaticEffect bs
      LosesAbilities : (n : Noun bs Object) -> (abl : List (AbilityLost bs)) ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       {auto 0 ne : NonEmpty abl} ->
                       StaticEffect bs
      GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} -> StaticEffect bs
      Intercepts : (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
                   (window : Maybe TriggerWindow) ->
                   (repl : Effect (interceptCtx alts ev)) ->
                   (use : ReplUse) ->
                   (limit : Maybe UsageLimit) ->
                   {auto 0 ul : So (untriggeredLimitOk limit)} ->
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
      EntersUnderInstead : (n : Noun bs Object) ->
                           (who : Noun (nomIntro n) Player) ->
                           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                           {auto 0 ps : SoleHolder who} -> StaticEffect bs
      CantPrevent : (kind : DamageKind) -> (what : Unpreventable bs) ->
                    (ban : PreventionBan) -> StaticEffect bs
      Conditionally : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
                      (marking : CondMarking) ->
                      {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      OnlyWhile : (se : StaticEffect bs) -> (c : Condition (staticIntro se)) ->
                  (marking : CondMarking) ->
                  {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
      OnlyDuring : (p : TurnPart) -> (w : Maybe Owner) ->
                   (se : StaticEffect bs) ->
                   {auto 0 wk : WindowOk p w} ->
                   StaticEffect bs
      NoLossFrom : (who : Noun bs Player) -> (cause : LoseCause) ->
                   StaticEffect bs
      Visibility : (v : ExposeVerb) -> (who : Noun bs Player) ->
                   (what : VisibleThing (nomIntro who)) ->
                   {auto 0 vo : VisibilityOk v what} -> StaticEffect bs
      MayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity bs) ->
                               {auto 0 nz : NonZeroQ q} ->
                               {auto 0 wf : WellFormedQ q} ->
                               {auto 0 lt : So (isNil (quantDelta q))} ->
                               StaticEffect bs
      MayBlockAdditional : (n : Noun bs Object) -> (q : Quantity bs) ->
                           {auto 0 nz : NonZeroQ q} ->
                           {auto 0 wf : WellFormedQ q} ->
                           {auto 0 lt : So (isNil (quantDelta q))} ->
                           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                           StaticEffect bs
      MayVoteAdditional : (who : Noun bs Player) -> (q : Quantity bs) ->
                          {auto 0 nz : NonZeroQ q} ->
                          {auto 0 wf : WellFormedQ q} ->
                          {auto 0 lt : So (isNil (quantDelta q))} ->
                          StaticEffect bs
      TriggersAdditionally : (ev : GameEvent bs) -> (q : Quantity bs) ->
                             {auto 0 nz : NonZeroQ q} ->
                             {auto 0 wf : WellFormedQ q} ->
                             {auto 0 lt : So (isNil (quantDelta q))} ->
                             {auto 0 ok : So (triggerCountOk (eventName ev))} ->
                             StaticEffect bs
      EntersRider : (n : Noun bs Object) -> (rider : TokenRider (selfSubjIntro n)) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    StaticEffect bs
      EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                           (kind : CounterKindSource bs) ->
                           (mark : EntryCounterMark) ->
                           StaticEffect bs
      EntersAsCopy : (n : Noun bs Object) -> (optional : Bool) ->
                     (src : Noun (selfSubjIntro n) Object) ->
                     (exc : List (CopyExcept (selfSubjIntro n))) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 pm : PerMember src} -> StaticEffect bs
      EntersChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) -> (disc : Disclosure) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      AttachChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      AndAlso : {0 n : Nat} -> StaticParts n bs ->
                {auto 0 ne : IsSucc n} -> StaticEffect bs
      OfSubject : {0 k : Nat} -> (n : Noun bs Object) ->
                  (vps : SubjectVPs k (selfSubjIntro n)) ->
                  {auto 0 ne : IsSucc k} ->
                  {auto 0 ok :
                     So (vpsOk (nounZone n) (nounRegime n) (nounHeadTys n) vps)} ->
                  StaticEffect bs

  public export
  gatePayer : Binding
  gatePayer = MkBinding TheD Player OneOf PlayerP

  public export
  data Compulsion : Bindings -> Type where
    Forbid : Compulsion bs
    Require : Compulsion bs
    GatedBy : (c : Cost (Effect.gatePayer :: bs)) -> Compulsion bs
    Permit : Compulsion bs

  public export
  data DeonticPatient : {0 bs : Bindings} -> Deeds -> Role -> Type where
    NoDeonticPatient : DeonticPatient {bs} ds r
    DefendingPlayer : {k : Kind} -> (m : Noun bs k) ->
                      {auto 0 at : Attackable m} ->
                      DeonticPatient {bs} ds r
    DeonticCounterpart : {k : Kind} -> (m : Noun bs k) ->
                         DeonticPatient {bs} ds r
    TargetedBy : {k : Kind} -> (m : Noun bs k) ->
                 {auto 0 tr : Targeter k} ->
                 DeonticPatient {bs} ds r
    CounterpartsAt : (cs : List (DeedComplement bs)) ->
                     {auto 0 ne : NonEmpty cs} ->
                     DeonticPatient {bs} ds r

  public export
  data DeedComplement : Bindings -> Type where
    MkDeedComplement : {k : Kind} -> (d : VerbLabel) -> (m : Noun bs k) ->
                       DeedComplement bs

  public export
  complementDeed : {0 bs : Bindings} -> DeedComplement bs -> VerbLabel
  complementDeed (MkDeedComplement d _) = d

  public export
  deonticPatientIntro : {bs : Bindings} -> {0 ds : Deeds} -> {0 r : Role} ->
                        DeonticPatient {bs} ds r -> Bindings
  deonticPatientIntro NoDeonticPatient = bs
  deonticPatientIntro (DefendingPlayer m) = nomIntro m
  deonticPatientIntro (DeonticCounterpart m) = nomIntro m
  deonticPatientIntro (TargetedBy m) = nomIntro m
  deonticPatientIntro (CounterpartsAt cs) = bs

  public export
  data PlayPayment : Bindings -> Type where
    ItsOwnCost : PlayPayment bs
    WithoutPaying : PlayPayment bs
    PayingInstead : (c : Cost bs) ->
                    {auto 0 ok : So (costOffBattlefield c)} ->
                    PlayPayment bs

  public export
  data DeonticRider : Bindings -> Type where
    NoDeonticRider : DeonticRider bs
    PlayRider : (from : Maybe (ZoneExpr bs)) ->
                (limit : Maybe PlayLimit) ->
                (window : Maybe PlayWindow) ->
                (exclusive : Bool) ->
                (payment : PlayPayment bs) -> DeonticRider bs

  public export
  playRidden : {0 bs : Bindings} -> DeonticRider bs -> Bool
  playRidden NoDeonticRider = False
  playRidden (PlayRider _ _ _ _ _) = True

  public export
  agentRole : Role -> Bool
  agentRole Agent = True
  agentRole Patient = False

  public export
  counterpartFits : {bs : Bindings} -> {k : Kind} -> Deeds -> Role ->
                    Noun bs k -> Bool -> Bool
  counterpartFits {k} ds r m moved =
    all (\d => deedKindOk d r k) ds &&
    all (\d => deedHeadTysOk d r (nounHeadTys m)) ds &&
    (moved || zoneFits (nounZone m) (deedsZone ds r))

  public export
  counterpartNotSelf : {bs : Bindings} -> {k : Kind} -> {ka : Kind} ->
                       (n : Noun bs k) -> Noun (nomIntro n) ka -> Bool
  counterpartNotSelf n (Pro Bare OneOf) = countReach Bare OneOf (nounDelta n) == 0
  counterpartNotSelf n _ = True

  public export
  deonticPatientOk : {bs : Bindings} -> {k : Kind} -> (n : Noun bs k) ->
                     (ds : Deeds) -> (r : Role) ->
                     (patient : DeonticPatient {bs = nomIntro n} ds r) ->
                     DeonticRider (deonticPatientIntro patient) -> Bool
  deonticPatientOk n ds r NoDeonticPatient _ = True
  deonticPatientOk n ds r (DefendingPlayer m) _ = all deedDefendsOk ds && agentRole r
  deonticPatientOk n ds r (DeonticCounterpart m) rider =
    counterpartFits ds (counterRole r) m (playRidden rider) &&
    counterpartNotSelf n m
  deonticPatientOk n ds r (TargetedBy m) _ = all deedTargetedOk ds
  deonticPatientOk n ds r (CounterpartsAt cs) _ =
    distinctDeeds (map complementDeed cs) &&
    all (complementAtOk n ds r) cs

  public export
  complementAtOk : {bs : Bindings} -> {k : Kind} -> (n : Noun bs k) ->
                   (ds : Deeds) -> (r : Role) ->
                   DeedComplement (nomIntro n) -> Bool
  complementAtOk n ds r (MkDeedComplement d m) =
    elem d ds && counterpartFits [d] (counterRole r) m False &&
    counterpartNotSelf n m

  public export
  asThoughOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
               Compulsion bs -> Deeds -> Maybe (AsThough cs) -> Bool
  asThoughOk _ ds Nothing = True
  asThoughOk Permit ds (Just a) = all (deedPremiseOk (asThoughSort a)) ds
  asThoughOk _ _ (Just _) = False

  public export
  permits : {0 cs : Bindings} -> Compulsion cs -> Bool
  permits Permit = True
  permits _ = False

  public export
  deonticRiderOk : {bs : Bindings} -> {0 cs : Bindings} ->
                   (ds : Deeds) -> (r : Role) -> Compulsion cs ->
                   (patient : DeonticPatient {bs} ds r) -> Bool ->
                   DeonticRider (deonticPatientIntro patient) -> Bool
  deonticRiderOk ds r c pat at NoDeonticRider = True
  deonticRiderOk ds r c (DeonticCounterpart m) at (PlayRider from lim win exc pay) =
    permits c && all deedPlaysOk ds && agentRole r &&
    playSourceOk (nounZone m) from at &&
    playWindowOk lim win && (not exc || isJust from)
  deonticRiderOk ds r c _ at (PlayRider _ _ _ _ _) = False

  public export
  notConditional : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notConditional (Conditionally _ _ _) = False
  notConditional (OnlyWhile _ _ _) = False
  notConditional _ = True


  public export
  NotConditional : StaticEffect bs -> Type
  NotConditional {bs} se = So (notConditional se)

  public export
  notWindowed : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notWindowed (OnlyDuring _ _ _) = False
  notWindowed _ = True

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
  data Unpreventable : Bindings -> Type where
    DamageDescribed : (scope : DamageScope bs) ->
                      (by : Maybe (Noun (scopeIntro scope) Object)) ->
                      Unpreventable bs
    ThatDamage : {auto 0 ok : So (damageDealtInScope bs)} -> Unpreventable bs

  public export
  unpreventableIntro : {bs : Bindings} -> Unpreventable bs -> Bindings
  unpreventableIntro (DamageDescribed scope by) = byIntro by
  unpreventableIntro ThatDamage = bs

  public export
  data PreventionBan = NoPreventionOnly | NoRedirectEither

  public export
  data PreventCut : Bindings -> Type where
    CutAll : PreventCut bs
    CutSome : (amt : Amount bs) -> PreventCut bs
    CutAllBut : (amt : Amount bs) -> PreventCut bs
    CutHalf : (r : RoundMode) -> PreventCut bs

  public export
  cutIntro : {bs : Bindings} -> PreventCut bs -> Bindings
  cutIntro CutAll = bs
  cutIntro (CutSome amt) = amtIntro amt
  cutIntro (CutAllBut amt) = amtIntro amt
  cutIntro (CutHalf _) = bs

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

  public export
  notExtended : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notExtended (AlsoOffBattlefield _) = False
  notExtended _ = True

  public export
  NotExtended : StaticEffect bs -> Type
  NotExtended {bs} se = So (notExtended se)

  public export
  notCarvedOut : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notCarvedOut (DoesntRemove _ _) = False
  notCarvedOut _ = True

  public export
  NotCarvedOut : StaticEffect bs -> Type
  NotCarvedOut {bs} se = So (notCarvedOut se)

  public export
  staticKind : {0 bs : Bindings} -> StaticEffect bs -> StaticKind
  staticKind (Define _ _) = LetterDefinition
  staticKind (Gets _ _ _) = PtDelta
  staticKind (DefinesPt _ _ _) = PtDefinition
  staticKind (HasBasePt _ _ _) = BasePtSet
  staticKind (SwitchesPt _) = PtSwitch
  staticKind (CostsToCast _ _) = CostModification
  staticKind (AltCost _ _) = CostModification
  staticKind (AddedCost _ _) = CostModification
  staticKind (Gains _ _) = KeywordGrant
  staticKind (GainsAbilitiesOf _ _ _ _) = KeywordGrant
  staticKind (Deontic _ _ _ _ _ _ _) = DeedRestriction
  staticKind (DoesntUntap _) = DeedRestriction
  staticKind (CantMoreThan _ _ _ _) = DeedRestriction
  staticKind (Skips _ _) = TurnSkip
  staticKind (KeepsUnspentMana _ _) = ManaPersistence
  staticKind (MayDeclineUntap _) = DeedRestriction
  staticKind (UntapsDuringStep _) = UntapGrant
  staticKind (BecomesAlso _ _) = TypeAddition
  staticKind (AddsEveryType _ _) = TypeAddition
  staticKind (LosesEveryType _ _) = TypeLoss
  staticKind (LosesType _ _) = TypeLoss
  staticKind (SetsColor _ _) = ColorSet
  staticKind (BecomesCopy _ _ _) = CopyEffect
  staticKind (SetsType _ _ _) = TypeSet
  staticKind (AddsChosenQuality _ _) = TypeAddition
  staticKind (SetsChosenQuality _ _) = TypeSet
  staticKind (LosesAllAbilities _ _) = AbilityLoss
  staticKind (LosesAbilities _ _) = AbilityLoss
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _ _ _ _) = Replacement
  staticKind (Prevents _ _ _ _ _) = Prevention
  staticKind (PreventsFrom _ _ _ _ _ _) = Prevention
  staticKind (CantPrevent _ _ _) = Prevention
  staticKind (Redirects _ _ _ _ _) = Replacement
  staticKind (EntersUnderInstead _ _) = Replacement
  staticKind (RedirectsFrom _ _ _ _ _) = Replacement
  staticKind (Scales _ _ _ _ _) = Replacement
  staticKind (OnlyDuring _ _ se) = staticKind se
  staticKind (Conditionally _ _ _) = Conditional
  staticKind (OnlyWhile _ _ _) = Conditional
  staticKind (AlsoOffBattlefield se) = staticKind se
  staticKind (DoesntRemove se _) = staticKind se
  staticKind (NoLossFrom _ _) = OutcomeImmunity
  staticKind (Visibility _ _ _) = VisibilityRider
  staticKind (MayPlayAdditionalLands _ _) = LandAllowance
  staticKind (MayBlockAdditional _ _) = BlockAllowance
  staticKind (MayVoteAdditional _ _) = VoteAllowance
  staticKind (TriggersAdditionally _ _) = TriggerMultiplier
  staticKind (EntersRider _ _) = EntryRider
  staticKind (EntersAsCopy _ _ _ _) = EntryRider
  staticKind (EntersWithCounters _ _ _ _) = EntryRider
  staticKind (EntersChoice _ _ _ _) = EntryRider
  staticKind (AttachChoice _ _ _) = Replacement
  staticKind (AndAlso _) = Coordination
  staticKind (OfSubject _ _) = Coordination


  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (Define l amt) = defineLetter l (amtIntro amt)
  staticIntro (Gets n pow tou) = shiftDelta tou ++ shiftDelta pow ++ selfSubjIntro n
  staticIntro (DefinesPt n _ amt) =
    outcomeB NamedNumber :: (amtDelta amt ++ selfSubjIntro n)
  staticIntro (HasBasePt n pow tou) = amtDelta tou ++ amtDelta pow ++ selfSubjIntro n
  staticIntro (SwitchesPt n) = selfSubjIntro n
  staticIntro (CostsToCast n sh) = costShiftDelta sh ++ selfSubjIntro n
  staticIntro (AltCost n _) = selfSubjIntro n
  staticIntro (AddedCost _ _) = bs
  staticIntro (Gains n ab) = abLetterDelta ab ++ selfSubjIntro n
  staticIntro (GainsAbilitiesOf n _ src _) = nomIntro src
  staticIntro (Deontic n _ _ _ _ _ _) = selfSubjIntro n
  staticIntro (DoesntUntap n) = selfSubjIntro n
  staticIntro (CantMoreThan _ _ _ _) = bs
  staticIntro (Skips _ _) = bs
  staticIntro (KeepsUnspentMana who _) = nomIntro who
  staticIntro (MayDeclineUntap n) = selfSubjIntro n
  staticIntro (UntapsDuringStep n) = selfSubjIntro n
  staticIntro (BecomesAlso n _) = selfSubjIntro n
  staticIntro (AddsEveryType n _) = selfSubjIntro n
  staticIntro (LosesEveryType n _) = selfSubjIntro n
  staticIntro (LosesType n _) = selfSubjIntro n
  staticIntro (SetsColor n _) = selfSubjIntro n
  staticIntro (BecomesCopy n _ _) = selfSubjIntro n
  staticIntro (SetsType n _ _) = selfSubjIntro n
  staticIntro (AddsChosenQuality n _) = selfSubjIntro n
  staticIntro (SetsChosenQuality n _) = selfSubjIntro n
  staticIntro (LosesAllAbilities n _) = selfSubjIntro n
  staticIntro (LosesAbilities n _) = selfSubjIntro n
  staticIntro (GainsControl who what) = stampIntro (Just "GainControl") what
  staticIntro (Intercepts ev alts window repl use limit) = interceptCtx alts ev
  staticIntro (Prevents kind size scope by also) = byIntro by
  staticIntro (PreventsFrom kind src scope cut use also) = cutIntro cut
  staticIntro (CantPrevent kind what ban) = unpreventableIntro what
  staticIntro (Redirects kind size scope by to) = nomIntro to
  staticIntro (EntersUnderInstead n who) = nomIntro who
  staticIntro (RedirectsFrom kind src scope to use) = nomIntro to
  staticIntro (Scales kind src scope op use) = scaleIntro op
  staticIntro (OnlyDuring _ _ se) = staticIntro se
  staticIntro (Conditionally c se _) = staticIntro se
  staticIntro (OnlyWhile se c _) = staticIntro se
  staticIntro (AlsoOffBattlefield se) = staticIntro se
  staticIntro (DoesntRemove _ n) = nomIntro n
  staticIntro (NoLossFrom who _) = nomIntro who
  staticIntro (Visibility _ who what) = visibleIntro what
  staticIntro (MayPlayAdditionalLands who _) = nomIntro who
  staticIntro (MayBlockAdditional n _) = selfSubjIntro n
  staticIntro (MayVoteAdditional who _) = nomIntro who
  staticIntro (TriggersAdditionally _ _) = bs
  staticIntro (EntersRider n _) = selfSubjIntro n
  staticIntro (EntersAsCopy n _ _ _) = selfSubjIntro n
  staticIntro (EntersWithCounters n amt _ _) = amtDelta amt ++ selfSubjIntro n
  staticIntro (EntersChoice n _ _ _) = selfSubjIntro n
  staticIntro (AttachChoice n _ _) = selfSubjIntro n
  staticIntro (AndAlso parts) = partsIntro parts
  staticIntro (OfSubject n vps) = vpsIntro vps

  public export
  staticChoiceDelta : {bs : Bindings} -> StaticEffect bs -> List Binding
  staticChoiceDelta (EntersChoice _ q _ _) = [choiceB q]
  staticChoiceDelta (AttachChoice _ q _) = [choiceB q]
  staticChoiceDelta (AndAlso parts) = partsChoiceDelta parts
  staticChoiceDelta (AddedCost c _) = costDelta c
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
    MkMoveRiders : (entry : List (TokenRider bs)) ->
                   (ctrl : Maybe (Noun bs Player)) ->
                   (counters : Maybe (CounterRider bs)) ->
                   {auto 0 one : CtrlOverrideOk ctrl} -> MoveRiders bs

  public export
  data CtrlOverrideOk : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoOverride : CtrlOverrideOk Nothing
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlOverrideOk (Just n)
    PerMemberController : {0 bs : Bindings} -> {0 ax : PossessorAxis} ->
                          {0 grp : Noun bs Object} ->
                          {0 pl : nounPlur grp = ManyOf} ->
                          CtrlOverrideOk (Just (PossessorsOf ax grp {pl}))

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
    ScaledMana : (unit : ManaUnit) -> (amt : Amount bs) ->
                 {auto 0 fe : ForEachAmount amt} -> Cost bs
    TapSymbol : Cost bs
    UntapSymbol : Cost bs
    LoyaltySymbol : (s : LoyaltyCost) -> Cost bs
    Do : (e : Effect bs) -> {auto 0 ok : CostAction e} -> Cost bs
    Compound : {0 n : Nat} -> CostSeq n bs ->
               {auto 0 ne : IsSucc n} -> Cost bs
    EitherCost : (l : Cost bs) -> (r : Cost bs) ->
                 {auto 0 nl : NotCompound l} ->
                 {auto 0 nr : NotCompound r} -> Cost bs
    ItsManaCost : Cost bs

  public export
  forEachAmount : {0 bs : Bindings} -> Amount bs -> Bool
  forEachAmount (Times _ _) = True
  forEachAmount (TimesOf _ _) = True
  forEachAmount _ = False

  public export
  ForEachAmount : Amount bs -> Type
  ForEachAmount {bs} a = So (forEachAmount a)

  public export
  costIntro : {bs : Bindings} -> Cost bs -> Bindings
  costIntro (Mana c) = if manaHasX c then letterB X :: bs else bs
  costIntro (ScaledMana _ _) = bs
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (LoyaltySymbol LoyaltyDownX) = letterB X :: bs
  costIntro (LoyaltySymbol _) = bs
  costIntro (Do e) = effIntro e
  costIntro (Compound cs) = costsIntro cs
  costIntro (EitherCost _ _) = bs
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
    AsPrintedCost : (n : Noun bs Object) ->
                    {auto 0 one : nounPlur n = OneOf} -> ProducedMana bs
    ProducedByEvent : (n : Noun bs Object) ->
                      {auto 0 one : nounPlur n = OneOf} ->
                      {auto 0 pm : countOutcomes ManaProduced bs = 1} ->
                      ProducedMana bs
    CouldProduce : (n : Noun bs Object) -> ProducedMana bs
    AmongColorsOf : (n : Noun bs Object) -> ProducedMana bs
    AmongWritten : (cs : List Color) ->
                   {auto 0 tw : So (colorCountOk (length cs))} ->
                   ProducedMana bs
    LastNotedMana : (n : Noun bs Object) ->
                    {auto 0 one : nounPlur n = OneOf} -> ProducedMana bs

  public export
  data SpentMode = AffectsIt | TriggersThen

  public export
  data ManaRider : Bindings -> Type where
    SpendOnly : (ps : List (SpendPurpose bs)) ->
                {auto 0 ne : SpendPurposes ps} -> ManaRider bs
    SpendNotOn : (ps : List (SpendPurpose bs)) ->
                 {auto 0 ne : SpendPurposes ps} -> ManaRider bs
    OnSpent : (mode : SpentMode) -> (only : Bool) ->
              (what : Noun bs Object) ->
              (says : Effect (nomIntro what)) ->
              {auto 0 zn : OnStack (nounZone what)} -> ManaRider bs

  public export
  data SpendPurposes : {0 bs : Bindings} -> List (SpendPurpose bs) -> Type where
    MkSpendPurposes : {0 p : SpendPurpose bs} -> {0 ps : List (SpendPurpose bs)} ->
                      SpendPurposes (p :: ps)

  public export
  copyBundleSays : {0 bs : Bindings} -> TokenChars bs -> Bool
  copyBundleSays t =
    let said = (if ptWritten t.pt then 1 else 0) +
               (if someWritten t.colors then 1 else 0) +
               (if lineNonEmpty t.line then 1 else 0)
    in said >= 2

  public export
  CopyBundle : TokenChars bs -> Type
  CopyBundle {bs} t = So (copyBundleSays t)

  public export
  data CopyExcept : Bindings -> Type where
    ExceptTypes : (added : TypeLine) ->
                  {auto 0 ne : LineNonEmpty added} -> CopyExcept bs
    ExceptName : (nm : String) -> CopyExcept bs
    ||| [CR#707.9d] “In addition” retains only type characteristics.
    ExceptChars : (t : TokenChars bs) -> (typesAdded : Bool) ->
                  {auto 0 bd : CopyBundle t} ->
                  {auto 0 tc : TokenCanonical t} ->
                  {auto 0 ta : TokenAbilities t} ->
                  {auto 0 qf : TokenQualsFit t} ->
                  {auto 0 un : AdditionUnnamed t} -> CopyExcept bs
    ExceptAbility : (ab : AbilityAt []) ->
                    {auto 0 gr : Grantable ab} -> CopyExcept bs
    ExceptThisAbility : CopyExcept bs
    ExceptPt : (pow : Amount bs) -> (tou : Amount (amtIntro pow)) -> CopyExcept bs
    ExceptNonlegendary : CopyExcept bs
    ExceptColor : (c : Chroma.Color) -> CopyExcept bs
    ExceptEntersWithCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                               (mark : EntryCounterMark) -> CopyExcept bs

  public export
  data Repetition : Bindings -> Type where
    Again : Repetition bs
    MoreTimes : (n : Amount bs) ->
                Repetition bs
    AnyNumber : Repetition bs
    Until : (c : Condition bs) -> Repetition bs
    AgainExcludingChosen : Repetition bs
    AgainExcept : (c : Condition bs) -> Repetition bs

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

  public export
  data Effect : Bindings -> Type where
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) ->
                 {auto 0 pm : PerMember to} ->
                 {auto 0 rk : DamageRecipient to} -> Effect bs
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
    TurnOver : (what : Noun bs Object) ->
               {auto 0 ok : OnBattlefield (nounZone what)} -> Effect bs
    RemoveFromCombat : (n : Noun bs Object) ->
                       {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    AttachTo : {k : Kind} -> (what : Noun bs Object) ->
               {auto 0 zw : OnBattlefield (nounZone what)} ->
               (host : Noun (nomIntro what) k) ->
               {auto 0 hk : So (kindLte k (Object \/ Player))} -> Effect bs
    Unattach : (what : Noun bs Object) ->
               {auto 0 zw : OnBattlefield (nounZone what)} -> Effect bs
    BecomesBlocking : (n : Noun bs Object) ->
                      {auto 0 zn : OnBattlefield (nounZone n)} ->
                      {auto 0 dn : DeedParticipant ["Block"] Agent Object (nounHeadTys n)} ->
                      (what : Noun (nomIntro n) Object) ->
                      {auto 0 zw : OnBattlefield (nounZone what)} ->
                      {auto 0 dw : DeedParticipant ["Block"] Patient Object (nounHeadTys what)} ->
                      Effect bs
    StopsBlocking : (n : Noun bs Object) ->
                    {auto 0 zn : OnBattlefield (nounZone n)} ->
                    {auto 0 dn : DeedParticipant ["Block"] Agent Object (nounHeadTys n)} ->
                    (what : Noun (nomIntro n) Object) ->
                    {auto 0 zw : OnBattlefield (nounZone what)} ->
                    {auto 0 dw : DeedParticipant ["Block"] Patient Object (nounHeadTys what)} ->
                    Effect bs
    BecomesAttacking : (n : Noun bs Object) ->
                       {auto 0 zn : OnBattlefield (nounZone n)} ->
                       {auto 0 dn : DeedParticipant ["Attack"] Agent Object (nounHeadTys n)} ->
                       (whom : AttackDefender (nomIntro n)) -> Effect bs
    Regenerate : (n : Noun bs Object) ->
                 {auto 0 zn : OnBattlefield (nounZone n)} ->
                 Effect bs
    CantBe : {k : Kind} -> (e : Effect bs) -> (deed : VerbLabel) ->
             (what : Noun (riderIntro e) k) ->
             {auto 0 kd : KnownDeed deed} ->
             {auto 0 rd : So (deedRidesOk deed)} ->
             {auto 0 kk : So (deedKindOk deed Patient k)} ->
             {auto 0 sub : DeedParticipant [deed] Patient k (nounHeadTys what)} ->
             {auto 0 zn : ZoneFits (nounZone what) (deedZoneOf deed Patient)} ->
             Effect bs
    GainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                       (w : GivingWarrant d) ->
                       (span : Maybe (Duration (nomIntro n))) ->
                       {auto 0 sc : designationScope d = HeldBy k} ->
                       {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
    Unlock : (door : Door bs) ->
             {auto 0 nh : DoorNamesHost door} -> Effect bs
    GameBecomes : (d : Designation) ->
                  {auto 0 sc : designationScope d = HeldByGame} ->
                  {auto 0 at : So (designationGiven d)} -> Effect bs
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Effect bs
    GameDrawn : Effect bs
    RestartsGame : Effect bs
    SeparateIntoPiles : (who : Noun bs Player) ->
                        (grp : Noun (nomIntro who) Object) ->
                        (piles : Nat) -> (faces : List PileFace) ->
                        {auto 0 ff : FacesFit faces piles} ->
                        {auto 0 pl : nounPlur grp = ManyOf} -> Effect bs
    Choose : {k : Kind} -> (n : Noun bs k) ->
             (by : Maybe (Noun bs Player)) -> (disc : Disclosure) ->
             {auto 0 ch : ChoiceClause by n} -> Effect bs
    ChoicesRevealed : (s : HiddenSort) -> Effect bs
    Vote : (voters : Noun bs Player) -> (disc : Disclosure) ->
           (ballot : Ballot (nomIntro voters)) -> Effect bs
    VoteStarting : (first : Noun bs Player) ->
                   (voters : Noun (nomIntro first) Player) ->
                   (ballot : Ballot (nomIntro voters)) -> Effect bs
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           (riders : MoveRiders (nomIntro what)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 pl : Placeable (nounTy what) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} -> Effect bs
    CounterSpell : {k : Kind} -> (what : Noun bs k) ->
                   {auto 0 ct : Counterable what} -> Effect bs
    CopyStack : {k : Kind} -> (agent : Noun bs Player) ->
                (what : Noun (nomIntro agent) k) ->
                (times : Amount (nomIntro what)) ->
                (exc : List (CopyExcept (amtIntro times))) ->
                {auto ph : Phrasal k} ->
                {auto 0 cp : Copiable what} ->
                Effect bs
    CopyCard : (who : Noun bs Player) ->
               (what : Noun (nomIntro who) Object) ->
               (times : Amount (nomIntro what)) ->
               {auto 0 zn : So (isCardZone (nounZone what))} -> Effect bs
    ChooseNewTargets : {k : Kind} -> (what : Noun bs k) ->
                       {auto 0 cp : Copiable what} -> Effect bs
    CopyTargets : {k : Kind} -> {kt : Kind} ->
                  (copy : Noun bs k) ->
                  (whom : Noun (nomIntro copy) kt) ->
                  {auto 0 cp : Copiable copy} ->
                  {auto 0 tk : Targetable kt} -> Effect bs
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    ExchangeLife : (parties : Noun bs Player) ->
                   {auto 0 tp : So (twoPartiesOk parties)} -> Effect bs
    AddMana : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
              (prod : ProducedMana (amtIntro amt)) ->
              (riders : List (ManaRider (amtIntro amt))) ->
              Effect bs
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           Effect bs
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Effect bs
    Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) ->
             (q : Quantity (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} ->
             {auto 0 zf : ZoneFree p} -> Effect bs
    Shuffle : (whose : Noun bs Player) -> Effect bs
    FlipCoins : (who : Noun bs Player) -> (count : FlipScope (nomIntro who)) ->
                Effect bs
    RollDice : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
               (sides : DieSides (amtIntro count)) -> Effect bs
    ResultsTable : (rows : List (RollRow bs)) ->
                   {auto 0 ne : IsSucc (rowCount rows)} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    IgnoreOutcomes : (which : IgnoredOutcomes bs) ->
                     {auto 0 ok : So (ignorableFor which)} -> Effect bs
    ShiftResult : (amt : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    ShiftResultOneWay : (rises : Bool) -> (amt : Amount bs) ->
                        {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    RollPlanarDie : (who : Noun bs Player) ->
                    (count : Amount (nomIntro who)) -> Effect bs
    ChaosEnsues : Effect bs
    ChaosEnsuesFor : (what : Noun bs Object) -> Effect bs
    StoreResults : (on : Noun bs Object) ->
                   {auto 0 one : nounPlur on = OneOf} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
    RerollStored : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                   (whose : Noun (nomIntro who) Object) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 one : nounPlur whose = OneOf} -> Effect bs
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 sp : SpanOk (staticKind se) span} ->
                   {auto 0 cl : ClauseStatic se} -> Effect bs
    Throughout : (span : Duration bs) ->
                 (se : StaticEffect (spanIntro span)) ->
                 {auto 0 sp : SpanOk (staticKind se) (Just span)} ->
                 {auto 0 cl : ClauseStatic se} -> Effect bs
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (spec : TokenSpec (amtIntro count)) ->
             (riders : List (TokenRider (amtIntro count))) ->
             Effect bs
    GetsEmblem : (who : Noun bs Player) -> (abl : List (AbilityAt [])) ->
                 {auto 0 ea : EmblemAbilities abl} -> Effect bs
    PutCounters : (amt : Amount bs) -> (kind : CounterKindSource bs) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 sc : CounterSourceScope kind Object} -> Effect bs
    Distribute : {k : Kind} -> (v : DividedVerb bs) ->
                 (amt : Amount (divIntro v)) ->
                 (among : Noun (amtIntro amt) k) ->
                 {auto 0 gm : GroupMention among} ->
                 {auto 0 tk : DividedTakes (divTag v) among} -> Effect bs
    RemoveCounters : (q : Maybe (Quantity bs)) -> (kind : Maybe CounterKind) ->
                     (from : Noun (optQuantIntro q) Object) ->
                     {auto 0 wf : OptWellFormedQ q} ->
                     {auto 0 kn : CounterKindNamed Object kind} ->
                     {auto 0 cm : CounterMemory from} -> Effect bs
    RemoveCountersAmong : (q : Quantity bs) -> (kind : Maybe CounterKind) ->
                          (among : Noun (quantIntro q) Object) ->
                          {auto 0 wf : WellFormedQ q} ->
                          {auto 0 kn : CounterKindNamed Object kind} ->
                          {auto 0 cm : CounterMemory among} ->
                          {auto 0 pb : PartitiveBase among} -> Effect bs
    MoveCounters : (amt : Amount bs) -> (kind : Maybe CounterKind) ->
                   (src : Noun (amtIntro amt) Object) ->
                   (dst : Noun (nomIntro src) Object) ->
                   {auto 0 kn : CounterKindNamed Object kind} ->
                   {auto 0 cm : CounterMemory src} ->
                   {auto 0 md : MoveDestination dst} ->
                   {auto 0 pm : PerMember dst} -> Effect bs
    PutSameCounters : (src : Noun bs Object) ->
                      (dst : Noun (nomIntro src) Object) ->
                      {auto 0 pm : PerMember dst} -> Effect bs
    PutCountersOfThoseKinds : (amt : Amount bs) ->
                              (on : Noun (amtIntro amt) Object) ->
                              {auto 0 pm : PerMember on} ->
                              {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                              Effect bs
    GetsCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                   (kind : CounterKind) ->
                   {auto 0 sc : counterScope kind = Player} -> Effect bs
    GetsCountersOfThoseKinds : (who : Noun bs Player) ->
                               (amt : Amount (nomIntro who)) ->
                               {auto 0 ok : countOutcomes CountersPut bs = 1} ->
                               Effect bs
    GiveCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                             {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                             {auto 0 pm : PerMember on} -> Effect bs
    DoubleCountersOfOwnKinds : {k : Kind} -> (on : Noun bs k) ->
                               {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                               {auto 0 pm : PerMember on} -> Effect bs
    GiveAbilityCountersOfOwnKinds : (on : Noun bs Ability) -> Effect bs
    RemoveCountersOfOwnKinds : {k : Kind} -> (from : Noun bs k) ->
                               {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                               {auto 0 pm : PerMember from} -> Effect bs
    LosesCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                    (amt : Maybe (Amount (nomIntro who))) ->
                    {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
    Enact : (v : VerbLabel) -> (e : Effect bs) ->
            {auto 0 kn : KnownVerb v} -> Effect bs
    Does : (subj : Noun bs Player) -> (v : VerbLabel) ->
           (e : Effect (agentIntro subj)) ->
           {auto 0 kn : KnownVerb v} -> Effect bs
    DoesGroup : (subj : Noun bs Player) -> (v : VerbLabel) ->
                (e : Effect (agentIntro subj)) ->
                {auto 0 pl : nounPlur subj = ManyOf} ->
                {auto 0 kn : KnownVerb v} -> Effect bs
    ControllerSacrifices : (n : Noun bs Object) ->
                           {auto 0 one : nounPlur n = OneOf} ->
                           {auto 0 zn : OnBattlefield (nounZone n)} -> Effect bs
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          (times : PayTimes) ->
          {auto 0 pb : Payable c} ->
          {auto 0 ag : PayAgrees who c} -> Effect bs
    May : (offer : Noun bs Player) -> (body : Effect (mayCtx offer)) ->
          (ifDid : Maybe (Effect (effIntro body))) ->
          (ifNot : Maybe (Effect (mayCtx offer))) -> Effect bs
    IfDone : (body : Effect bs) ->
             (ifDid : Maybe (Effect (effIntro body))) ->
             (ifNot : Maybe (Effect bs)) ->
             {auto 0 en : ReflexEnclosure body} ->
             {auto 0 br : So (ifDoneArmed body ifDid ifNot)} -> Effect bs
    OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) ->
             (otherwise : Maybe (Effect (condDelta c ++ otherwiseCtx e))) ->
             Effect bs
    If : (c : Condition bs) -> (e : Effect (condIntro c)) ->
         (otherwise : Maybe (Effect (otherwiseCtx e))) -> Effect bs
    Unless : (e : Effect bs) -> (who : Noun (preIntro e) Player) ->
             (c : Cost (nomIntro who)) ->
             {auto 0 pb : Payable c} ->
             {auto 0 ag : PayAgrees who c} -> Effect bs
    Define : (l : Letter) -> (amt : Amount bs) ->
             {auto 0 ok : So (anyOpenLetter l bs)} -> Effect bs
    ForEachOf : {k : Kind} -> {auto ph : Phrasal k} ->
                (grp : Noun bs k) ->
                (body : Effect (elemIntro grp)) ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                Effect bs
    ForEachKindOf : (ax : KindAxis) -> (dom : Maybe (Noun bs Object)) ->
                    (q : QualitySort) ->
                    {auto 0 sc : kindAxisSort ax = Just q} ->
                    {auto 0 cl : So (kindDomainOk ax dom)} ->
                    (body : Effect (kindValueIntro q dom)) ->
                    Effect bs
    Repeat : (rep : Repetition bs) -> Effect bs
    Repeated : (n : Amount bs) -> (body : Effect (amtIntro n)) -> Effect bs
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
    ThisWay : (body : Effect bs) -> (ev : GameEvent (effIntro body)) ->
              (trig : Effect (thisWayCtx body ev)) ->
              {auto 0 oc : ThisWayOutcome body} -> Effect bs

    DoesntUntapNext : (n : Noun bs Object) -> (steps : Amount bs) ->
                      {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    SkipsNext : (who : Noun bs Player) -> (part : TurnPart) ->
                (count : Amount bs) -> Effect bs
    SkipsAllOf : (who : Noun bs Player) -> (part : TurnPart) ->
                 {auto 0 ad : AddedPart part} -> Effect bs
    ExtraTurn : (who : Noun bs Player) -> (count : Amount bs) -> Effect bs
    AdditionalPart : (part : TurnPart) -> (anchor : Maybe TurnPart) ->
                     (count : Amount bs) ->
                     (followedBy : Maybe TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten followedBy} -> Effect bs
    GetsAdditionalPart : (who : Noun bs Player) -> (part : TurnPart) ->
                         (count : Amount bs) ->
                         {auto 0 ad : AddedPart part} -> Effect bs
    GetsAdditionalPartAfter : (who : Noun bs Player) -> (part : TurnPart) ->
                              (count : Amount bs) -> (anchor : TurnPart) ->
                              {auto 0 ad : AddedPart part} -> Effect bs

  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (DealDamageOwn _ _ _) = False
  heldUntilOk (ControllerSacrifices _) = False
  heldUntilOk (DoesntUntapNext _ _) = False
  heldUntilOk (SkipsNext _ _ _) = False
  heldUntilOk (ExtraTurn _ _) = False
  heldUntilOk (AdditionalPart _ _ _ _) = False
  heldUntilOk (GetsAdditionalPart _ _ _) = False
  heldUntilOk (GetsAdditionalPartAfter _ _ _ _) = False
  heldUntilOk (SkipsAllOf _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  heldUntilOk (TurnOver _) = False
  heldUntilOk (SetStatus PhasedOut _) = True
  heldUntilOk (SetStatus _ _) = False
  heldUntilOk (GetsCounters _ _ _) = False
  heldUntilOk (GetsCountersOfThoseKinds _ _) = False
  heldUntilOk (LosesCounters _ _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
  heldUntilOk (AttachTo _ _) = False
  heldUntilOk (Unattach _) = False
  heldUntilOk (BecomesBlocking _ _) = False
  heldUntilOk (StopsBlocking _ _) = False
  heldUntilOk (BecomesAttacking _ _) = False
  heldUntilOk (Regenerate _) = False
  heldUntilOk (CantBe _ _ _) = False
  heldUntilOk (GainsDesignation _ _ _ _) = False
  heldUntilOk (Unlock _) = False
  heldUntilOk (GameBecomes _) = False
  heldUntilOk (Concludes _ _) = False
  heldUntilOk GameDrawn = False
  heldUntilOk RestartsGame = False
  heldUntilOk (SeparateIntoPiles _ _ _ _) = False
  heldUntilOk (CounterSpell _) = False
  heldUntilOk (CopyStack _ _ _ _) = False
  heldUntilOk (ChooseNewTargets _) = False
  heldUntilOk (CopyTargets _ _) = False
  heldUntilOk (CopyCard _ _ _) = False
  heldUntilOk (Choose _ _ _) = False
  heldUntilOk (ChoicesRevealed _) = False
  heldUntilOk (Vote _ _ _) = False
  heldUntilOk (VoteStarting _ _ _) = False
  heldUntilOk (Move _ _ _) = True
  heldUntilOk (ExchangeLife _) = False
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
  heldUntilOk (ShiftResultOneWay _ _) = False
  heldUntilOk (RollPlanarDie _ _) = False
  heldUntilOk ChaosEnsues = False
  heldUntilOk (ChaosEnsuesFor _) = False
  heldUntilOk (StoreResults _) = False
  heldUntilOk (RerollStored _ _ _) = False
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Throughout _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (GetsEmblem _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  heldUntilOk (RemoveCountersAmong _ _ _) = False
  heldUntilOk (MoveCounters _ _ _ _) = False
  heldUntilOk (PutSameCounters _ _) = False
  heldUntilOk (PutCountersOfThoseKinds _ _) = False
  heldUntilOk (GiveCountersOfOwnKinds _) = False
  heldUntilOk (DoubleCountersOfOwnKinds _) = False
  heldUntilOk (GiveAbilityCountersOfOwnKinds _) = False
  heldUntilOk (RemoveCountersOfOwnKinds _) = False
  heldUntilOk (Enact _ (Move _ _ _)) = True
  heldUntilOk (Enact _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (DoesGroup _ _ _) = False
  heldUntilOk (Pay _ _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (IfDone _ _ _) = False
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

  public export
  data EncloseUse
    = ||| No PLAYER takes the action, so "you do" has no subject to
      EncAgentless
    | ||| No single taken action for the pro-verb to abbreviate.
      EncNotOneAction
    | ||| The clause schedules its action rather than taking it, so
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
  reflexEncloseUse (GetsAdditionalPart _ _ _) = EncNotYetTaken
  reflexEncloseUse (GetsAdditionalPartAfter _ _ _ _) = EncNotYetTaken
  reflexEncloseUse (SkipsAllOf _ _) = EncNotYetTaken
  reflexEncloseUse (Distribute _ _ _) = EncAgentless
  reflexEncloseUse (Fights _ _) = EncAgentless
  reflexEncloseUse (ExchangeLife _) = EncAgentless
  reflexEncloseUse (ChangeLife _ _) = EncAgentless
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncReflexive
  reflexEncloseUse (Continuously _ _) = EncAgentless
  reflexEncloseUse (Throughout _ (GainsControl _ _)) = EncReflexive
  reflexEncloseUse (Throughout _ _) = EncAgentless
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (DoesGroup _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _ _) = EncReflexive      -- 66, all of them offered
  reflexEncloseUse (Enact _ _) = EncReflexive
  reflexEncloseUse (TurnOver _) = EncAgentless
  reflexEncloseUse (SetStatus _ _) = EncAgentless
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (GetsCountersOfThoseKinds _ _) = EncAgentless
  reflexEncloseUse (LosesCounters _ _ _) = EncAgentless
  reflexEncloseUse (RemoveFromCombat _) = EncAgentless
  reflexEncloseUse (AttachTo _ _) = EncAgentless
  reflexEncloseUse (Unattach _) = EncAgentless
  reflexEncloseUse (BecomesBlocking _ _) = EncAgentless
  reflexEncloseUse (StopsBlocking _ _) = EncAgentless
  reflexEncloseUse (BecomesAttacking _ _) = EncAgentless
  reflexEncloseUse (Regenerate _) = EncAgentless
  reflexEncloseUse (CantBe _ _ _) = EncAgentless
  reflexEncloseUse (GainsDesignation _ _ _ _) = EncAgentless
  reflexEncloseUse (Unlock _) = EncAgentless
  reflexEncloseUse (GameBecomes _) = EncAgentless
  reflexEncloseUse (Concludes _ _) = EncAgentless
  reflexEncloseUse GameDrawn = EncAgentless
  reflexEncloseUse RestartsGame = EncAgentless
  reflexEncloseUse (SeparateIntoPiles _ _ _ _) = EncReflexive
  reflexEncloseUse (CounterSpell _) = EncAgentless
  reflexEncloseUse (CopyStack _ _ _ _) = EncAgentless
  reflexEncloseUse (ChooseNewTargets _) = EncReflexive
  reflexEncloseUse (CopyTargets _ _) = EncAgentless
  reflexEncloseUse (CopyCard _ _ _) = EncAgentless
  reflexEncloseUse (Create _ _ _ _) = EncReflexive -- 8
  reflexEncloseUse (GetsEmblem _ _) = EncAgentless
  reflexEncloseUse (PutCounters _ _ _) = EncReflexive    -- 8
  reflexEncloseUse (RemoveCounters _ _ _) = EncReflexive -- 7
  reflexEncloseUse (RemoveCountersAmong _ _ _) = EncReflexive
  reflexEncloseUse (MoveCounters _ _ _ _) = EncReflexive
  reflexEncloseUse (PutSameCounters _ _) = EncReflexive
  reflexEncloseUse (PutCountersOfThoseKinds _ _) = EncReflexive
  reflexEncloseUse (GiveCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (DoubleCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (GiveAbilityCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (RemoveCountersOfOwnKinds _) = EncReflexive
  reflexEncloseUse (Move _ _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _ _ _) = EncReflexive       -- 1
  reflexEncloseUse (ChoicesRevealed _) = EncAgentless
  reflexEncloseUse (Vote _ _ _) = EncReflexive
  reflexEncloseUse (VoteStarting _ _ _) = EncReflexive
  reflexEncloseUse (Search _ _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  reflexEncloseUse (FlipCoins _ _) = EncReflexive
  reflexEncloseUse (RollDice _ _ _) = EncReflexive
  reflexEncloseUse (ResultsTable _) = EncNotOneAction
  reflexEncloseUse (IgnoreOutcomes _) = EncAgentless
  reflexEncloseUse (ShiftResult _) = EncAgentless
  reflexEncloseUse (ShiftResultOneWay _ _) = EncAgentless
  reflexEncloseUse (RollPlanarDie _ _) = EncReflexive
  reflexEncloseUse ChaosEnsues = EncAgentless
  reflexEncloseUse (ChaosEnsuesFor _) = EncAgentless
  reflexEncloseUse (StoreResults _) = EncAgentless
  reflexEncloseUse (RerollStored _ _ _) = EncReflexive
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  reflexEncloseUse (IfDone body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (IfDone _ _ _) = EncNotOneAction
  reflexEncloseUse (OnlyIf e _ _) = reflexEncloseUse e
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (Unless _ _ _) = EncNotOneAction
  reflexEncloseUse (Define _ _) = EncAgentless
  reflexEncloseUse (ForEachOf _ _) = EncNotOneAction
  reflexEncloseUse (ForEachKindOf _ _ _ _) = EncNotOneAction
  reflexEncloseUse (Repeat _) = EncNotOneAction
  reflexEncloseUse (Repeated _ (Pay _ _ _)) = EncReflexive
  reflexEncloseUse (Repeated _ (May _ (Pay _ _ _) Nothing _)) = EncReflexive
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

  public export
  thisWayOutcomeOk : {0 bs : Bindings} -> Effect bs -> Bool
  thisWayOutcomeOk (Delayed _ _ _ _) = False
  thisWayOutcomeOk (May _ body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (IfDone body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (DoesntUntapNext _ _) = True
  thisWayOutcomeOk (SkipsNext _ _ _) = True
  thisWayOutcomeOk (ExtraTurn _ _) = True
  thisWayOutcomeOk (AdditionalPart _ _ _ _) = True
  thisWayOutcomeOk (GetsAdditionalPart _ _ _) = True
  thisWayOutcomeOk (GetsAdditionalPartAfter _ _ _ _) = True
  thisWayOutcomeOk (SkipsAllOf _ _) = True
  thisWayOutcomeOk (HeldUntil _ _) = True
  thisWayOutcomeOk (DealDamage _ _ _) = True
  thisWayOutcomeOk (Distribute _ _ _) = True
  thisWayOutcomeOk (Fights _ _) = True
  thisWayOutcomeOk (TurnOver _) = True
  thisWayOutcomeOk (SetStatus _ _) = True
  thisWayOutcomeOk (DealDamageOwn _ _ _) = True
  thisWayOutcomeOk (ControllerSacrifices _) = True
  thisWayOutcomeOk (GetsCounters _ _ _) = True
  thisWayOutcomeOk (GetsCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (LosesCounters _ _ _) = True
  thisWayOutcomeOk (RemoveFromCombat _) = True
  thisWayOutcomeOk (AttachTo _ _) = True
  thisWayOutcomeOk (Unattach _) = True
  thisWayOutcomeOk (BecomesBlocking _ _) = True
  thisWayOutcomeOk (StopsBlocking _ _) = True
  thisWayOutcomeOk (BecomesAttacking _ _) = True
  thisWayOutcomeOk (Regenerate _) = True
  thisWayOutcomeOk (CantBe _ _ _) = True
  thisWayOutcomeOk (GainsDesignation _ _ _ _) = True
  thisWayOutcomeOk (Unlock _) = True
  thisWayOutcomeOk (GameBecomes _) = True
  thisWayOutcomeOk (Concludes _ _) = True
  thisWayOutcomeOk GameDrawn = True
  thisWayOutcomeOk RestartsGame = True
  thisWayOutcomeOk (SeparateIntoPiles _ _ _ _) = True
  thisWayOutcomeOk (CounterSpell _) = True
  thisWayOutcomeOk (CopyStack _ _ _ _) = True
  thisWayOutcomeOk (ChooseNewTargets _) = True
  thisWayOutcomeOk (CopyTargets _ _) = True
  thisWayOutcomeOk (CopyCard _ _ _) = True
  thisWayOutcomeOk (Choose _ _ _) = True
  thisWayOutcomeOk (ChoicesRevealed _) = True
  thisWayOutcomeOk (Vote _ _ _) = True
  thisWayOutcomeOk (VoteStarting _ _ _) = True
  thisWayOutcomeOk (Move _ _ _) = True
  thisWayOutcomeOk (ExchangeLife _) = True
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
  thisWayOutcomeOk (ShiftResultOneWay _ _) = True
  thisWayOutcomeOk (RollPlanarDie _ _) = True
  thisWayOutcomeOk ChaosEnsues = True
  thisWayOutcomeOk (ChaosEnsuesFor _) = True
  thisWayOutcomeOk (StoreResults _) = True
  thisWayOutcomeOk (RerollStored _ _ _) = True
  thisWayOutcomeOk (Continuously _ _) = True
  thisWayOutcomeOk (Throughout _ _) = True
  thisWayOutcomeOk (Create _ _ _ _) = True
  thisWayOutcomeOk (GetsEmblem _ _) = True
  thisWayOutcomeOk (PutCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCountersAmong _ _ _) = True
  thisWayOutcomeOk (MoveCounters _ _ _ _) = True
  thisWayOutcomeOk (PutSameCounters _ _) = True
  thisWayOutcomeOk (PutCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (GiveCountersOfOwnKinds _) = True
  thisWayOutcomeOk (DoubleCountersOfOwnKinds _) = True
  thisWayOutcomeOk (GiveAbilityCountersOfOwnKinds _) = True
  thisWayOutcomeOk (RemoveCountersOfOwnKinds _) = True
  thisWayOutcomeOk (Enact _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (DoesGroup _ _ _) = True
  thisWayOutcomeOk (Pay _ _ _) = True
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
  payableOk (ScaledMana _ _) = True
  payableOk TapSymbol = False
  payableOk UntapSymbol = False
  payableOk (LoyaltySymbol _) = False
  payableOk (Do _) = True
  payableOk (Compound _) = True
  payableOk (EitherCost l r) = payableOk l && payableOk r
  payableOk ItsManaCost = True

  public export
  Payable : Cost bs -> Type
  Payable {bs} c = So (payableOk c)

  public export
  costActionOk : {0 bs : Bindings} -> Effect bs -> Bool
  costActionOk (DealDamage src _ _) = costNounOk src
  costActionOk (DealDamageOwn src _ _) = costNounOk src
  costActionOk (ControllerSacrifices n) = costNounOk n
  costActionOk (DoesntUntapNext n _) = costNounOk n
  costActionOk (SkipsNext _ _ _) = False
  costActionOk (ExtraTurn who _) = costNounOk who
  costActionOk (AdditionalPart _ _ _ _) = True
  costActionOk (GetsAdditionalPart who _ _) = costNounOk who
  costActionOk (GetsAdditionalPartAfter who _ _ _) = costNounOk who
  costActionOk (SkipsAllOf _ _) = False
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (TurnOver n) = costNounOk n
  costActionOk (SetStatus _ n) = costNounOk n
  costActionOk (GetsCounters who _ _) = costNounOk who
  costActionOk (GetsCountersOfThoseKinds _ _) = False
  costActionOk (LosesCounters who _ _) = costNounOk who
  costActionOk (RemoveFromCombat n) = costNounOk n
  costActionOk (AttachTo what _) = costNounOk what
  costActionOk (Unattach what) = costNounOk what
  costActionOk (BecomesBlocking n _) = costNounOk n
  costActionOk (StopsBlocking n _) = costNounOk n
  costActionOk (BecomesAttacking n _) = costNounOk n
  costActionOk (Regenerate n) = costNounOk n
  costActionOk (CantBe e _ _) = costActionOk e
  costActionOk (GainsDesignation n _ _ _) = costNounOk n
  costActionOk (Unlock ThisDoor) = True
  costActionOk (Unlock (DoorOf _ room)) = costNounOk room
  costActionOk (GameBecomes _) = True
  costActionOk (Concludes _ _) = True
  costActionOk GameDrawn = True
  costActionOk RestartsGame = False
  costActionOk (SeparateIntoPiles _ grp _ _) = costNounOk grp
  costActionOk (CounterSpell _) = True
  costActionOk (CopyStack _ what _ _) = costNounOk what
  costActionOk (ChooseNewTargets what) = costNounOk what
  costActionOk (CopyTargets copy _) = costNounOk copy
  costActionOk (CopyCard _ what _) = costNounOk what
  costActionOk (Choose n _ _) = costNounOk n
  costActionOk (ChoicesRevealed _) = False
  costActionOk (Vote _ _ _) = False
  costActionOk (VoteStarting _ _ _) = False
  costActionOk (Move what _ _) = costNounOk what
  costActionOk (ExchangeLife parties) = costNounOk parties
  costActionOk (ChangeLife _ _) = True
  costActionOk (AddMana who _ _ _) = costNounOk who
  costActionOk (Draw _ _) = True
  costActionOk (Expose _ who _) = costNounOk who
  costActionOk (Search who _ _ _) = costNounOk who
  costActionOk (Shuffle whose) = costNounOk whose
  costActionOk (FlipCoins who _) = costNounOk who
  costActionOk (RollDice who _ _) = costNounOk who
  costActionOk (ResultsTable _) = False
  costActionOk (IgnoreOutcomes _) = False
  costActionOk (ShiftResult _) = False
  costActionOk (ShiftResultOneWay _ _) = False
  costActionOk (StoreResults _) = False
  costActionOk (RollPlanarDie who _) = costNounOk who
  costActionOk ChaosEnsues = False
  costActionOk (ChaosEnsuesFor _) = False
  costActionOk (RerollStored who _ _) = costNounOk who
  costActionOk (Continuously _ _) = False
  costActionOk (Throughout _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (RemoveCountersAmong _ _ among) = costNounOk among
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  costActionOk (PutSameCounters src dst) = costNounOk src && costNounOk dst
  costActionOk (GiveCountersOfOwnKinds on) = costNounOk on
  costActionOk (DoubleCountersOfOwnKinds on) = costNounOk on
  costActionOk (GiveAbilityCountersOfOwnKinds on) = costNounOk on
  costActionOk (RemoveCountersOfOwnKinds from) = costNounOk from
  costActionOk (PutCountersOfThoseKinds _ _) = False
  costActionOk (Enact _ e) = costActionOk e
  costActionOk (Does _ _ e) = costActionOk e
  costActionOk (DoesGroup _ _ e) = costActionOk e
  costActionOk (Pay _ _ _) = False
  costActionOk (May _ body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (IfDone body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (OnlyIf e _ otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (If _ e otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (Unless _ _ _) = False
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
  effEq (GetsAdditionalPart w p c) (GetsAdditionalPart x q d) =
    nounEqRef w x && p == q && boundEq c d
  effEq (GetsAdditionalPart _ _ _) _ = False
  effEq (GetsAdditionalPartAfter _ _ _ _) _ = False
  effEq (SkipsAllOf w p) (SkipsAllOf x q) = nounEqRef w x && p == q
  effEq (SkipsAllOf _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (TurnOver a) (TurnOver b) = nounEqRef a b
  effEq (TurnOver _) _ = False
  effEq (SetStatus v a) (SetStatus w b) = sameStatusVal v w && nounEqRef a b
  effEq (SetStatus _ _) _ = False
  effEq (GetsCounters You x j) (GetsCounters You y l) = j == l && boundEq x y
  effEq (GetsCounters _ _ _) _ = False
  effEq (GetsCountersOfThoseKinds _ _) _ = False
  effEq (LosesCounters You j Nothing) (LosesCounters You l Nothing) = j == l
  effEq (LosesCounters You j (Just x)) (LosesCounters You l (Just y)) =
    j == l && boundEq x y
  effEq (LosesCounters _ _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
  effEq (AttachTo a _) (AttachTo b _) = nounEqRef a b
  effEq (AttachTo _ _) _ = False
  effEq (Unattach a) (Unattach b) = nounEqRef a b
  effEq (Unattach _) _ = False
  effEq (BecomesBlocking _ _) _ = False
  effEq (StopsBlocking _ _) _ = False
  effEq (BecomesAttacking _ _) _ = False
  effEq (Regenerate a) (Regenerate b) = nounEqRef a b
  effEq (Regenerate _) _ = False
  effEq (CantBe _ _ _) _ = False
  effEq (GainsDesignation _ _ _ _) _ = False
  effEq (Unlock _) _ = False
  effEq (GameBecomes a) (GameBecomes b) = a == b
  effEq (GameBecomes _) _ = False
  effEq (Concludes v a) (Concludes w b) = v == w && nounEqRef a b
  effEq (Concludes _ _) _ = False
  effEq GameDrawn GameDrawn = True
  effEq GameDrawn _ = False
  effEq RestartsGame RestartsGame = True
  effEq RestartsGame _ = False
  effEq (SeparateIntoPiles _ _ _ _) _ = False
  effEq (CounterSpell _) _ = False
  effEq (CopyStack _ _ _ _) _ = False
  effEq (ChooseNewTargets _) _ = False
  effEq (CopyTargets _ _) _ = False
  effEq (CopyCard _ _ _) _ = False
  effEq (Choose _ _ _) _ = False
  effEq (ChoicesRevealed a) (ChoicesRevealed b) = a == b
  effEq (ChoicesRevealed _) _ = False
  effEq (Vote _ _ _) _ = False
  effEq (VoteStarting _ _ _) _ = False
  effEq (Move a s _) (Move b t _) = nounEqRef a b && zoneSort s == zoneSort t
  effEq (Move _ _ _) _ = False
  effEq (ExchangeLife a) (ExchangeLife b) = nounEqRef a b
  effEq (ExchangeLife _) _ = False
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
  effEq (ShiftResultOneWay _ _) _ = False
  effEq (RollPlanarDie _ _) _ = False
  effEq ChaosEnsues _ = False
  effEq (ChaosEnsuesFor _) _ = False
  effEq (StoreResults _) _ = False
  effEq (RerollStored _ _ _) _ = False
  effEq (Continuously _ _) _ = False
  effEq (Throughout _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (GetsEmblem _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (RemoveCountersAmong _ _ _) _ = False
  effEq (MoveCounters _ _ _ _) _ = False
  effEq (PutSameCounters _ _) _ = False
  effEq (PutCountersOfThoseKinds _ _) _ = False
  effEq (GiveCountersOfOwnKinds _) _ = False
  effEq (DoubleCountersOfOwnKinds _) _ = False
  effEq (GiveAbilityCountersOfOwnKinds _) _ = False
  effEq (RemoveCountersOfOwnKinds _) _ = False
  effEq (Enact v e) (Enact w f) = v == w && effEq e f
  effEq (Enact _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (DoesGroup _ _ _) _ = False
  effEq (Pay _ _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (IfDone _ _ _) _ = False
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

  public export
  effDelta : {bs : Bindings} -> Effect bs -> Bindings
  effDelta e = take (length (effIntro e) `minus` length bs) (effIntro e)

  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (DealDamageOwn src c to) = outcomeB DamageDealt :: nomIntro to
  effIntro (ControllerSacrifices n) =
    MkBinding TheD Player OneOf PlayerP
      :: moveIntro (Just "Sacrifice") n (Just Graveyard)
  effIntro (Fights a b) = nomIntro b
  effIntro (TurnOver n) = nomIntro n
  effIntro (SetStatus _ n) = nomIntro n
  effIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  effIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  effIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  effIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  effIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  effIntro (GetsAdditionalPartAfter w _ count _) = amtDelta count ++ nomIntro w
  effIntro (SkipsAllOf w _) = nomIntro w
  effIntro (GetsCounters who amt _) = amtIntro amt
  effIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  effIntro (LosesCounters who _ amt) = optAmtIntro amt
  effIntro (RemoveFromCombat n) = nomIntro n
  effIntro (AttachTo _ host) = nomIntro host
  effIntro (Unattach what) = nomIntro what
  effIntro (BecomesBlocking _ what) = nomIntro what
  effIntro (StopsBlocking _ what) = nomIntro what
  effIntro (BecomesAttacking n NoDefender) = nomIntro n
  effIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  effIntro (Regenerate n) = nomIntro n
  effIntro (CantBe e _ _) = effIntro e
  effIntro (GainsDesignation n _ _ _) = nomIntro n
  effIntro (Unlock door) = doorIntro door
  effIntro (GameBecomes _) = bs
  effIntro (Concludes _ who) = nomIntro who
  effIntro GameDrawn = bs
  effIntro RestartsGame = bs
  effIntro (SeparateIntoPiles who grp piles faces) =
    MkBinding TheD Object ManyOf
              (PileP (nounZone grp) (Just piles) (pileMentionFace faces))
      :: groupSpent (nomIntro grp)
  effIntro (CounterSpell what) = nomIntro what
  effIntro (CopyStack {k} {ph} agent what times exc) =
    MkBinding TheD k (outputPlur (nounPlur what) (amtPlur times))
              (copyPayload ph (nounTy what))
      :: amtIntro times
  effIntro (ChooseNewTargets what) = nomIntro what
  effIntro (CopyTargets copy whom) = nomIntro whom
  effIntro (CopyCard who what times) =
    MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
              (ObjectP (nounTy what) (nounZone what) Nothing (Just CopyOrigin) Nothing)
      :: amtIntro times
  effIntro (Choose n Nothing _) = chosenIntro n
  effIntro (Choose n (Just b) _) = nounDelta b ++ chosenIntro n
  effIntro (ChoicesRevealed _) = bs
  effIntro (Vote _ _ _) = bs
  effIntro (VoteStarting _ _ _) = bs
  effIntro (Move what to _) =
    afterMoveTo to (moveIntro Nothing what (Just (zoneSort to)))
  effIntro (ExchangeLife parties) =
    outcomeB LifeGained :: outcomeB LifeLost :: nomIntro parties
  effIntro (ChangeLife who (Up a)) = outcomeB LifeGained :: lifeIntro (Up a)
  effIntro (ChangeLife who (Down a)) = outcomeB LifeLost :: lifeIntro (Down a)
  effIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  effIntro (AddMana who amt _ _) = outcomeB ManaAdded :: amtIntro amt
  effIntro (Draw who amt) = amtIntro amt
  effIntro (Expose v who what) = exposedIntro what
  effIntro (Search who sc q p) =
    MkBinding AD Object (quantPlur q)
              (ObjectP (seedTy p) (searchZone sc)
                       (mkStamp (Just "Search") Nothing False) Nothing Nothing)
      :: (quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who)
  effIntro (Shuffle whose) = afterShuffle (nomIntro whose)
  effIntro (FlipCoins who count) = outcomeB CoinFlipped :: flipScopeIntro count
  effIntro (RollDice who count _) = outcomeB RollResult :: amtIntro count
  effIntro (ResultsTable rows) = bs
  effIntro (IgnoreOutcomes which) = ignoredOutcomesIntro which
  effIntro (ShiftResult amt) = amtIntro amt
  effIntro (ShiftResultOneWay _ amt) = amtIntro amt
  effIntro (RollPlanarDie who count) = outcomeB PlanarRolled :: amtIntro count
  effIntro ChaosEnsues = bs
  effIntro (ChaosEnsuesFor what) = nomIntro what
  effIntro (StoreResults on) = nomIntro on
  effIntro (RerollStored _ _ whose) = nomIntro whose
  effIntro (Continuously se _) = staticIntro se
  effIntro (Throughout _ se) = staticIntro se
  effIntro (Create agent count spec riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin) Nothing)
      :: (specDelta spec ++ amtIntro count)
  effIntro (GetsEmblem who _) = nomIntro who
  effIntro (PutCounters amt kind on) = nomIntro on
  effIntro (Distribute (DividedDamage _) amt among) = outcomeB DamageDealt :: nomIntro among
  effIntro (Distribute (DistributedCounters _) amt among) = nomIntro among
  effIntro (RemoveCounters q kind from) = outcomeB CountersRemoved :: nomIntro from
  effIntro (RemoveCountersAmong q kind among) = outcomeB CountersRemoved :: nomIntro among
  effIntro (MoveCounters amt kind src dst) = nomIntro dst
  effIntro (PutSameCounters src dst) = nomIntro dst
  effIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  effIntro (GiveCountersOfOwnKinds on) = nomIntro on
  effIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  effIntro (GiveAbilityCountersOfOwnKinds on) = nomIntro on
  effIntro (RemoveCountersOfOwnKinds from) = nomIntro from
  effIntro (Enact v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Enact v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Enact _ e) = effIntro e
  effIntro (Does s v (Move what to _)) =
    afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to)))
  effIntro (Does s v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Does s v e) = effIntro e
  effIntro (DoesGroup s v e) = deedDelta e ++ nomIntro s
  effIntro (Pay who c PaidOnce) = costIntro c
  effIntro (Pay who c AnyNumberOfTimes) = outcomeB RepeatCount :: costIntro c
  effIntro (Pay who c (UpToTimes _)) = outcomeB RepeatCount :: costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  effIntro (IfDone body did notd) = mayIntro body did notd
  effIntro (OnlyIf e c oth) = annIntro e
  effIntro (If c e oth) = bs
  effIntro (Unless e who c) = bs
  effIntro (Define l amt) = defineLetter l (amtIntro amt)
  effIntro (ForEachOf grp body) = pluralizeDelta (effDelta body) ++ bs
  effIntro (ForEachKindOf _ _ _ _) = bs
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

  public export
  preIntro : {bs : Bindings} -> Effect bs -> Bindings
  preIntro (DealDamage src amt to) = nomIntro to
  preIntro (DealDamageOwn src c to) = nomIntro to
  preIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  preIntro (Distribute v amt among) = nomIntro among
  preIntro (Fights a b) = nomIntro b
  preIntro (TurnOver n) = nomIntro n
  preIntro (SetStatus _ n) = nomIntro n
  preIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  preIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  preIntro (ExtraTurn w count) = amtDelta count ++ nomIntro w
  preIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  preIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  preIntro (GetsAdditionalPartAfter w _ count _) = amtDelta count ++ nomIntro w
  preIntro (SkipsAllOf w _) = nomIntro w
  preIntro (GetsCounters who amt _) = amtIntro amt
  preIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  preIntro (LosesCounters who _ amt) = optAmtIntro amt
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (AttachTo _ host) = nomIntro host
  preIntro (Unattach what) = nomIntro what
  preIntro (BecomesBlocking _ what) = nomIntro what
  preIntro (StopsBlocking _ what) = nomIntro what
  preIntro (BecomesAttacking n NoDefender) = nomIntro n
  preIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  preIntro (Regenerate n) = nomIntro n
  preIntro (CantBe e _ _) = preIntro e
  preIntro (GainsDesignation n _ _ _) = nomIntro n
  preIntro (Unlock door) = doorIntro door
  preIntro (GameBecomes _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro RestartsGame = bs
  preIntro (SeparateIntoPiles who grp _ _) = nomIntro grp
  preIntro (CounterSpell what) = nomIntro what
  preIntro (CopyStack agent what times exc) = amtIntro times
  preIntro (ChooseNewTargets what) = nomIntro what
  preIntro (CopyTargets copy whom) = nomIntro whom
  preIntro (CopyCard who what times) = amtIntro times
  preIntro (Choose n _ _) = chosenIntro n
  preIntro (ChoicesRevealed _) = bs
  preIntro (Vote _ _ _) = bs
  preIntro (VoteStarting _ _ _) = bs
  preIntro (Move what to _) = nomIntro what
  preIntro (ExchangeLife parties) = nomIntro parties
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
  preIntro (ShiftResultOneWay _ amt) = amtIntro amt
  preIntro (RollPlanarDie who count) = amtIntro count
  preIntro ChaosEnsues = bs
  preIntro (ChaosEnsuesFor what) = nomIntro what
  preIntro (StoreResults on) = nomIntro on
  preIntro (RerollStored _ _ whose) = nomIntro whose
  preIntro (Continuously se _) = staticIntro se
  preIntro (Throughout _ se) = staticIntro se
  preIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  preIntro (GetsEmblem who _) = nomIntro who
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters q kind from) = nomIntro from
  preIntro (RemoveCountersAmong q kind among) = nomIntro among
  preIntro (MoveCounters amt kind src dst) = nomIntro dst
  preIntro (PutSameCounters src dst) = nomIntro dst
  preIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  preIntro (GiveCountersOfOwnKinds on) = nomIntro on
  preIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  preIntro (GiveAbilityCountersOfOwnKinds on) = nomIntro on
  preIntro (RemoveCountersOfOwnKinds from) = nomIntro from
  preIntro (Enact v (Move what to _)) = nomIntro what
  preIntro (Enact _ e) = preIntro e
  preIntro (Does s v (Move what to _)) = nomIntro what
  preIntro (Does s v e) = preIntro e
  preIntro (DoesGroup s v e) = nomIntro s
  preIntro (Pay who c _) = nomIntro who
  preIntro (May d body did notd) = mayIntro body did notd
  preIntro (IfDone body did notd) = mayIntro body did notd
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

  public export
  riderIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  riderIntros [] = bs
  riderIntros (e :: []) = riderIntro e
  riderIntros (e :: es) = riderIntros es

  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
  annIntro (DealDamageOwn src c to) = nomIntro to
  annIntro (ControllerSacrifices n) = MkBinding TheD Player OneOf PlayerP :: selfSubjIntro n
  annIntro (Distribute v amt among) = nomIntro among
  annIntro (Fights a b) = nomIntro b
  annIntro (TurnOver n) = nomIntro n
  annIntro (SetStatus _ n) = nomIntro n
  annIntro (DoesntUntapNext n steps) = amtDelta steps ++ nomIntro n
  annIntro (SkipsNext w _ count) = amtDelta count ++ nomIntro w
  annIntro (ExtraTurn w count) = turnRefB :: (amtDelta count ++ nomIntro w)
  annIntro (AdditionalPart _ _ count _) = amtDelta count ++ bs
  annIntro (GetsAdditionalPart w _ count) = amtDelta count ++ nomIntro w
  annIntro (GetsAdditionalPartAfter w _ count _) = amtDelta count ++ nomIntro w
  annIntro (SkipsAllOf w _) = nomIntro w
  annIntro (GetsCounters who amt _) = amtIntro amt
  annIntro (GetsCountersOfThoseKinds who amt) = amtIntro amt
  annIntro (LosesCounters who _ amt) = optAmtIntro amt
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (AttachTo _ host) = nomIntro host
  annIntro (Unattach what) = nomIntro what
  annIntro (BecomesBlocking _ what) = nomIntro what
  annIntro (StopsBlocking _ what) = nomIntro what
  annIntro (BecomesAttacking n NoDefender) = nomIntro n
  annIntro (BecomesAttacking _ (OneDefender whom)) = nomIntro whom
  annIntro (Regenerate n) = nomIntro n
  annIntro (CantBe e _ _) = annIntro e
  annIntro (GainsDesignation n _ _ _) = nomIntro n
  annIntro (Unlock door) = doorIntro door
  annIntro (GameBecomes _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro RestartsGame = bs
  annIntro (SeparateIntoPiles who grp piles faces) =
    MkBinding TheD Object ManyOf
              (PileP (nounZone grp) (Just piles) (pileMentionFace faces))
      :: groupSpent (nomIntro grp)
  annIntro (CounterSpell what) = nomIntro what
  annIntro (CopyStack agent what times exc) = amtIntro times
  annIntro (ChooseNewTargets what) = nomIntro what
  annIntro (CopyTargets copy whom) = nomIntro whom
  annIntro (CopyCard who what times) = amtIntro times
  annIntro (Choose n _ _) = chosenIntro n
  annIntro (ChoicesRevealed _) = bs
  annIntro (Vote _ _ _) = bs
  annIntro (VoteStarting _ _ _) = bs
  annIntro (Move what to _) = nomIntro what
  annIntro (ExchangeLife parties) = nomIntro parties
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
  annIntro (ShiftResultOneWay _ amt) = amtIntro amt
  annIntro (RollPlanarDie who count) = amtIntro count
  annIntro ChaosEnsues = bs
  annIntro (ChaosEnsuesFor what) = nomIntro what
  annIntro (StoreResults on) = nomIntro on
  annIntro (RerollStored _ _ whose) = nomIntro whose
  annIntro (Continuously se _) = staticIntro se
  annIntro (Throughout _ se) = staticIntro se
  annIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  annIntro (GetsEmblem who _) = nomIntro who
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters q kind from) = nomIntro from
  annIntro (RemoveCountersAmong q kind among) = nomIntro among
  annIntro (MoveCounters amt kind src dst) = nomIntro dst
  annIntro (PutSameCounters src dst) = nomIntro dst
  annIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  annIntro (GiveCountersOfOwnKinds on) = nomIntro on
  annIntro (DoubleCountersOfOwnKinds on) = nomIntro on
  annIntro (GiveAbilityCountersOfOwnKinds on) = nomIntro on
  annIntro (RemoveCountersOfOwnKinds from) = nomIntro from
  annIntro (Enact v (Move what to _)) = nomIntro what
  annIntro (Enact _ e) = annIntro e
  annIntro (Does s v (Move what to _)) = nomIntro what
  annIntro (Does s v e) = annIntro e
  annIntro (DoesGroup s v e) = deedDelta e ++ nomIntro s
  annIntro (Pay who c _) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (IfDone body did notd) = annIntro body
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

  public export
  replacedCtx : {bs : Bindings} -> Effect bs -> Bindings
  replacedCtx (Sequentially es) = annSeqs es
  replacedCtx (May d body did notd) = replacedCtx body
  replacedCtx (IfDone body did notd) = replacedCtx body
  replacedCtx (OnlyIf e c oth) = replacedCtx e
  replacedCtx (If c e oth) = bs
  replacedCtx (Unless e who c) = replacedCtx e
  replacedCtx e = deedDelta e ++ annIntro e

  public export
  otherwiseCtx : {bs : Bindings} -> Effect bs -> Bindings
  otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e

  public export
  annSeqs : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  annSeqs [] = bs
  annSeqs (e :: es) = deedDelta e ++ annSeqs es

  public export
  deedDelta : {bs : Bindings} -> Effect bs -> List Binding
  deedDelta (DealDamage src amt to) = [outcomeB DamageDealt]
  deedDelta (DealDamageOwn src c to) = [outcomeB DamageDealt]
  deedDelta (ControllerSacrifices _) = []
  deedDelta (Distribute (DividedDamage _) amt among) = [outcomeB DamageDealt]
  deedDelta (Distribute (DistributedCounters _) amt among) = []
  deedDelta (Fights a b) = []
  deedDelta (TurnOver _) = []
  deedDelta (SetStatus _ _) = []
  deedDelta (DoesntUntapNext _ _) = []
  deedDelta (SkipsNext _ _ _) = []
  deedDelta (ExtraTurn _ _) = []
  deedDelta (AdditionalPart _ _ _ _) = []
  deedDelta (GetsAdditionalPart _ _ _) = []
  deedDelta (GetsAdditionalPartAfter _ _ _ _) = []
  deedDelta (SkipsAllOf _ _) = []
  deedDelta (GetsCounters _ _ _) = []
  deedDelta (GetsCountersOfThoseKinds _ _) = []
  deedDelta (LosesCounters _ _ _) = []
  deedDelta (RemoveFromCombat _) = []
  deedDelta (AttachTo _ _) = []
  deedDelta (Unattach _) = []
  deedDelta (BecomesBlocking _ _) = []
  deedDelta (StopsBlocking _ _) = []
  deedDelta (BecomesAttacking _ _) = []
  deedDelta (Regenerate _) = []
  deedDelta (CantBe e _ _) = deedDelta e
  deedDelta (GainsDesignation _ _ _ _) = []
  deedDelta (Unlock _) = []
  deedDelta (GameBecomes _) = []
  deedDelta (Concludes _ _) = []
  deedDelta GameDrawn = []
  deedDelta RestartsGame = []
  deedDelta (SeparateIntoPiles who grp piles faces) =
    [MkBinding TheD Object ManyOf
               (PileP (nounZone grp) (Just piles) (pileMentionFace faces))]
  deedDelta (CounterSpell _) = []
  deedDelta (CopyStack {k} {ph} agent what times exc) =
    [MkBinding TheD k (outputPlur (nounPlur what) (amtPlur times))
               (copyPayload ph (nounTy what))]
  deedDelta (ChooseNewTargets _) = []
  deedDelta (CopyTargets _ _) = []
  deedDelta (CopyCard who what times) =
    [MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
               (ObjectP (nounTy what) (nounZone what) Nothing (Just CopyOrigin) Nothing)]
  deedDelta (Choose n _ _) = []
  deedDelta (ChoicesRevealed _) = []
  deedDelta (Vote _ _ _) = []
  deedDelta (VoteStarting _ _ _) = []
  deedDelta (Move what to _) = []
  deedDelta (ExchangeLife _) = [outcomeB LifeGained, outcomeB LifeLost]
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (AddMana _ _ _ _) = [outcomeB ManaAdded]
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Search who sc q p) =
    [MkBinding AD Object (quantPlur q)
               (ObjectP (seedTy p) (searchZone sc) Nothing Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (FlipCoins _ _) = [outcomeB CoinFlipped]
  deedDelta (RollDice _ _ _) = [outcomeB RollResult]
  deedDelta (ResultsTable _) = []
  deedDelta (IgnoreOutcomes _) = []
  deedDelta (ShiftResult _) = []
  deedDelta (ShiftResultOneWay _ _) = []
  deedDelta (RollPlanarDie _ _) = [outcomeB PlanarRolled]
  deedDelta ChaosEnsues = []
  deedDelta (ChaosEnsuesFor _) = []
  deedDelta (StoreResults _) = []
  deedDelta (RerollStored _ _ _) = []
  deedDelta (Continuously se _) = []
  deedDelta (Throughout _ _) = []
  deedDelta (Create agent count spec riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin) Nothing)]
  deedDelta (GetsEmblem _ _) = []
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters q kind from) = [outcomeB CountersRemoved]
  deedDelta (RemoveCountersAmong q kind among) = [outcomeB CountersRemoved]
  deedDelta (MoveCounters amt kind src dst) = []
  deedDelta (PutSameCounters src dst) = []
  deedDelta (PutCountersOfThoseKinds amt on) = []
  deedDelta (GiveCountersOfOwnKinds on) = []
  deedDelta (DoubleCountersOfOwnKinds on) = []
  deedDelta (GiveAbilityCountersOfOwnKinds _) = []
  deedDelta (RemoveCountersOfOwnKinds _) = []
  deedDelta (Enact v (Move what to _)) = []
  deedDelta (Enact _ e) = deedDelta e
  deedDelta (Does s v (Move what to _)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (DoesGroup s v e) = deedDelta e
  deedDelta (Pay who c _) = []
  deedDelta (May d body did notd) = []
  deedDelta (IfDone body did notd) = []
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

  public export
  mayCtx : {bs : Bindings} -> Noun bs Player -> Bindings
  mayCtx d = agentIntro d

  public export
  ifDoneArmed : {0 bs : Bindings} -> (body : Effect bs) ->
                Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bool
  ifDoneArmed _ Nothing Nothing = False
  ifDoneArmed _ _ _ = True

  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bindings
  mayIntro body Nothing Nothing = effIntro body
  mayIntro body (Just did) Nothing = effIntro did
  mayIntro body Nothing (Just notd) = effIntro body
  mayIntro body (Just did) (Just notd) = effIntro body

  public export
  reflexCtx : {bs : Bindings} -> Effect bs -> Bindings
  reflexCtx body = settleTargets (effIntro body)

  public export
  thisWayCtx : {bs : Bindings} -> (body : Effect bs) ->
               GameEvent (effIntro body) -> Bindings
  thisWayCtx body ev = settleTargets (eventAfter ev)

  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

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
    ParamQuality : {k : Kind} -> (p : Predicate bs k) ->
                   {auto 0 pk : So (qualityParamKind k)} ->
                   KeywordParam bs
    ParamSubject : {k : Kind} -> (p : Predicate [] k) -> KeywordParam bs
    ParamNumber : (amt : Amount []) ->
                  KeywordParam bs
    ParamQualityCost : (p : Predicate bs Object) -> (cost : Cost []) ->
                       KeywordParam bs
    ParamNumberCost : (amt : Amount []) -> (cost : Cost []) ->
                      KeywordParam bs

  public export
  paramShapeOf : {0 bs : Bindings} -> Maybe (KeywordParam bs) -> KeywordParamShape
  paramShapeOf Nothing = NoParam
  paramShapeOf (Just (ParamCost _)) = CostParam
  paramShapeOf (Just (ParamQuality _)) = QualityParam
  paramShapeOf (Just (ParamSubject _)) = SubjectParam
  paramShapeOf (Just (ParamNumber _)) = NumberParam
  paramShapeOf (Just (ParamQualityCost _ _)) = CompoundParam QualityHead
  paramShapeOf (Just (ParamNumberCost _ _)) = CompoundParam NumberHead

  public export
  keywordParamFits : {0 bs : Bindings} -> KeywordLabel -> Maybe (KeywordParam bs) -> Bool
  keywordParamFits k p =
    knownKeyword k && paramShapeFits (keywordParamShape k) (paramShapeOf p)

  public export
  KeywordParamFits : KeywordLabel -> Maybe (KeywordParam bs) -> Type
  KeywordParamFits {bs} k p = So (keywordParamFits k p)

  public export
  data AbilityLost : Bindings -> Type where
    LostWritten : (ab : AbilityAt bs) ->
                  {auto 0 hd : So (grantableAb ab)} -> AbilityLost bs
    LostTerm : (t : KeywordTerm) ->
               {auto 0 kn : KnownKeywordTerm t} -> AbilityLost bs

  public export
  data AbilityAt : Bindings -> Type where
    KeywordAbility : (k : KeywordLabel) ->
                     (param : Maybe (KeywordParam bs)) ->
                     {auto 0 pf : KeywordParamFits k param} -> AbilityAt bs
    Activated : (cost : Cost (dropLetter X bs)) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                (window : Maybe Timing) ->
                (limit : Maybe UsageLimit) ->
                {auto 0 ul : So (untriggeredLimitOk limit)} ->
                (guard : Maybe (Condition bs)) ->
                (activator : Maybe (Noun bs Player)) ->
                AbilityAt bs
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
                {auto 0 hs : HeaderStatus ev} ->
                {auto 0 ae : AltEvent word alts} ->
                {auto 0 cd :
                   ChapterDefaults ev alts while joins window limit intervening} ->
                AbilityAt bs
    Static : (se : StaticEffect bs) ->
             {auto 0 ut : Untargeting se} -> AbilityAt bs
    Spell : (eff : Effect bs) -> AbilityAt bs
    MayBeginOnBattlefield : AbilityAt bs
    AlsoForKeywords : (ab : AbilityAt bs) -> (ks : List KeywordTerm) ->
                      {auto 0 ex : KeywordExtendable ab} ->
                      {auto 0 lk : KeywordListOk ab ks} -> AbilityAt bs
    ItalicHead : (word : ItalicWord) -> (ab : AbilityAt bs) ->
                 {auto 0 nw : NotWordHeaded ab} -> AbilityAt bs


  public export
  notWordHeaded : {0 bs : Bindings} -> AbilityAt bs -> Bool
  notWordHeaded (ItalicHead _ _) = False
  notWordHeaded _ = True

  public export
  NotWordHeaded : AbilityAt bs -> Type
  NotWordHeaded {bs} ab = So (notWordHeaded ab)

  public export
  Untargeting : {bs : Bindings} -> StaticEffect bs -> Type
  Untargeting {bs} se = So (not (anyTargetedAt (staticIntro se)))

  public export
  effectNamesThisDoor : {0 bs : Bindings} -> Effect bs -> Bool
  effectNamesThisDoor (Delayed ev alts _ _) =
    eventNamesThisDoor ev || anyEventNamesThisDoor alts
  effectNamesThisDoor (HeldUntil _ ev) = eventNamesThisDoor ev
  effectNamesThisDoor (ThisWay _ ev _) = eventNamesThisDoor ev
  effectNamesThisDoor _ = False

  public export
  abilityNamesThisDoor : {0 bs : Bindings} -> AbilityAt bs -> Bool
  abilityNamesThisDoor (Activated _ eff _ _ _ _) = effectNamesThisDoor eff
  abilityNamesThisDoor (Triggered _ ev alts while joins _ _ _ eff) =
    eventNamesThisDoor ev || anyEventNamesThisDoor alts ||
      concurrentNamesThisDoor while || joinsNameThisDoor joins ||
      effectNamesThisDoor eff
  abilityNamesThisDoor (Spell eff) = effectNamesThisDoor eff
  abilityNamesThisDoor (ItalicHead _ ab) = abilityNamesThisDoor ab
  abilityNamesThisDoor (AlsoForKeywords ab _) = abilityNamesThisDoor ab
  abilityNamesThisDoor _ = False

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
  keywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordTerm -> Bool
  keywordListOk ab ks = case lineKeyword ab of
    Nothing => False
    Just base => not (isNil ks) && allTermsBare ks && distinctTerms ks &&
                 not (elem (TheKeyword base) ks)


  public export
  allTermsBare : List KeywordTerm -> Bool
  allTermsBare [] = True
  allTermsBare (k :: ks) = keywordTermBare k && allTermsBare ks

  public export
  distinctTerms : List KeywordTerm -> Bool
  distinctTerms [] = True
  distinctTerms (k :: ks) = not (elem k ks) && distinctTerms ks

  public export
  KeywordExtendable : {0 bs : Bindings} -> AbilityAt bs -> Type
  KeywordExtendable {bs} ab = So (keywordExtendableOk ab)

  public export
  KeywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List KeywordTerm -> Type
  KeywordListOk {bs} ab ks = So (keywordListOk ab ks)

  public export
  grantableAb : {0 bs : Bindings} -> AbilityAt bs -> Bool
  grantableAb (KeywordAbility _ _) = True
  grantableAb (Activated _ _ _ _ _ _) = True
  grantableAb (Triggered _ _ _ _ _ _ _ _ _) = True
  grantableAb (Static _) = True
  grantableAb (Spell _) = False
  grantableAb MayBeginOnBattlefield = False
  grantableAb (AlsoForKeywords _ _) = False
  grantableAb (ItalicHead _ ab) = grantableAb ab

  public export
  Grantable : AbilityAt bs -> Type
  Grantable {bs} ab = So (grantableAb ab)

  public export
  emblemAbilityOk : AbilityAt [] -> Bool
  emblemAbilityOk (KeywordAbility _ _) = False
  emblemAbilityOk (Activated _ _ _ _ _ _) = True
  emblemAbilityOk (Triggered _ _ _ _ _ _ _ _ _) = True
  emblemAbilityOk (Static _) = True
  emblemAbilityOk (AlsoForKeywords _ _) = False
  emblemAbilityOk (ItalicHead _ ab) = emblemAbilityOk ab
  emblemAbilityOk (Spell _) = False
  emblemAbilityOk MayBeginOnBattlefield = False

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
  abilitiesHoldable : {0 bs : Bindings} -> List (AbilityAt bs) -> Bool
  abilitiesHoldable [] = True
  abilitiesHoldable (a :: as) = grantableAb a && abilitiesHoldable as

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
  predRegime (NthCastBy _ _ _) = Just AtCasting
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
  abRegime (Activated _ _ _ _ _ _) = Nothing
  abRegime (Triggered _ _ _ _ _ _ _ _ _) = Nothing
  abRegime (Static _) = Nothing
  abRegime (AlsoForKeywords ab _) = abRegime ab
  abRegime (ItalicHead _ ab) = abRegime ab
  abRegime (Spell _) = Nothing
  abRegime MayBeginOnBattlefield = Nothing

  public export
  castingOnly : Maybe StackRegime -> Bool
  castingOnly (Just AtCasting) = True
  castingOnly _ = False

  public export
  regimeMatches : Maybe StackRegime -> Maybe StackRegime -> Bool
  regimeMatches (Just a) (Just b) = a == b
  regimeMatches _ _ = False

  public export
  grantSubjectOk : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Bool
  grantSubjectOk ab n = grantSubjectFits (nounZone n) (nounRegime n) ab

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

  public export
  effChoiceDelta : {0 bs : Bindings} -> Effect bs -> List Binding
  effChoiceDelta (Choose {k} (Indefinite _ _) _ _) = choiceDeltaAt k
  effChoiceDelta (Choose _ _ _) = []
  effChoiceDelta (Sequentially es) = effsChoiceDelta es
  effChoiceDelta (May _ body _ _) = effChoiceDelta body
  effChoiceDelta (IfDone body _ _) = effChoiceDelta body
  effChoiceDelta _ = []

  public export
  effsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                    Effects n bs -> List Binding
  effsChoiceDelta [] = []
  effsChoiceDelta (e :: es) = effsChoiceDelta es ++ effChoiceDelta e

  public export
  abIntro : {bs : Bindings} -> AbilityAt bs -> Bindings
  abIntro (KeywordAbility _ _) = bs
  abIntro (Activated _ eff _ _ _ _) = effChoiceDelta eff ++ bs
  abIntro (Triggered _ _ _ _ _ _ _ _ eff) = effChoiceDelta eff ++ bs
  abIntro (Static se) = staticChoiceIntro se
  abIntro (AlsoForKeywords ab _) = abIntro ab
  abIntro (ItalicHead _ ab) = abIntro ab
  abIntro (Spell eff) = effChoiceDelta eff ++ bs
  abIntro MayBeginOnBattlefield = bs

  public export
  abLetterDelta : {0 bs : Bindings} -> AbilityAt bs -> List Binding
  abLetterDelta (KeywordAbility _ (Just (ParamNumber (LetterVal l)))) = [letterB l]
  abLetterDelta _ = []


  namespace Coord
    public export
    data StaticParts : Nat -> Bindings -> Type where
      Nil : StaticParts Z bs
      (::) : (se : StaticEffect bs) -> {auto 0 nc : NotCoord se} ->
             StaticParts n (staticIntro se) -> StaticParts (S n) bs

  namespace Shared
    public export
    data SubjectVP : Bindings -> Type where
      VPGets : (pow : PtShift bs) -> (tou : PtShift (shiftIntro pow)) ->
               (span : Maybe (Duration (shiftIntro tou))) -> SubjectVP bs
      VPGains : (ab : AbilityAt bs) -> (span : Maybe (Duration bs)) ->
                SubjectVP bs
      VPDeontic : (c : Compulsion bs) -> (deeds : Deeds) -> (role : Role) ->
                  (span : Maybe (Duration bs)) -> SubjectVP bs

    public export
    data SubjectVPs : Nat -> Bindings -> Type where
      Nil : SubjectVPs Z bs
      (::) : (vp : SubjectVP bs) -> SubjectVPs n (vpIntro vp) ->
             SubjectVPs (S n) bs

  public export
  vpIntro : {bs : Bindings} -> SubjectVP bs -> Bindings
  vpIntro (VPGets pow tou _) = shiftDelta tou ++ shiftDelta pow ++ bs
  vpIntro (VPGains _ _) = bs
  vpIntro (VPDeontic _ _ _ _) = bs

  public export
  vpsIntro : {0 k : Nat} -> {bs : Bindings} -> SubjectVPs k bs -> Bindings
  vpsIntro [] = bs
  vpsIntro (vp :: rest) = vpsIntro rest

  public export
  deedSubjectFits : Maybe Zone -> List CardType -> Deeds -> Role -> Bool
  deedSubjectFits zn tys ds r =
    all (\d => deedKindOk d r Object) ds &&
    all (\d => deedHeadTysOk d r tys) ds &&
    zoneFits zn (deedsZone ds r)

  public export
  vpSpanOk : {0 bs : Bindings} -> Maybe (Duration bs) -> Bool
  vpSpanOk Nothing = True
  vpSpanOk (Just d) = durationOk d

  public export
  vpOk : {0 bs : Bindings} -> Maybe Zone -> Maybe StackRegime ->
         List CardType -> SubjectVP bs -> Bool
  vpOk zn reg tys (VPGets _ _ sp) = zoneFits zn (Just Battlefield) && vpSpanOk sp
  vpOk zn reg tys (VPGains ab sp) =
    grantSubjectFits zn reg ab && grantableAb ab && vpSpanOk sp
  vpOk zn reg tys (VPDeontic _ ds r sp) =
    not (isNil ds) && knownDeeds ds && deedSubjectFits zn tys ds r && vpSpanOk sp

  public export
  vpsOk : {0 k : Nat} -> {0 bs : Bindings} -> Maybe Zone ->
          Maybe StackRegime -> List CardType -> SubjectVPs k bs -> Bool
  vpsOk zn reg tys [] = True
  vpsOk zn reg tys (vp :: rest) = vpOk zn reg tys vp && vpsOk zn reg tys rest

  namespace Paid
    public export
    data CostSeq : Nat -> Bindings -> Type where
      Nil : CostSeq Z bs
      (::) : (c : Cost bs) -> CostSeq n (costIntro c) -> CostSeq (S n) bs

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
  costDelta : {bs : Bindings} -> Cost bs -> List Binding
  costDelta c = take (length (costIntro c) `minus` length bs) (costIntro c)

  public export
  costChoiceDelta : {0 bs : Bindings} -> Cost bs -> List Binding
  costChoiceDelta (Do e) = effChoiceDelta e
  costChoiceDelta (Compound cs) = costsChoiceDelta cs
  costChoiceDelta _ = []

  public export
  costsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                     CostSeq n bs -> List Binding
  costsChoiceDelta [] = []
  costsChoiceDelta (c :: cs) = costsChoiceDelta cs ++ costChoiceDelta c

  public export
  partsIntro : {0 n : Nat} -> {bs : Bindings} -> StaticParts n bs -> Bindings
  partsIntro [] = bs
  partsIntro (se :: rest) = partsIntro rest

  public export
  partsChoiceDelta : {0 n : Nat} -> {bs : Bindings} ->
                     StaticParts n bs -> List Binding
  partsChoiceDelta [] = []
  partsChoiceDelta (se :: rest) = partsChoiceDelta rest ++ staticChoiceDelta se



  public export
  partsClauseOk : {0 n : Nat} -> {0 bs : Bindings} -> StaticParts n bs -> Bool
  partsClauseOk [] = True
  partsClauseOk (se :: rest) = clauseStaticOk se && partsClauseOk rest

  public export
  clauseStaticOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
  clauseStaticOk (DefinesPt _ _ _) = False
  clauseStaticOk (AltCost _ _) = False
  clauseStaticOk (AndAlso parts) = partsClauseOk parts
  clauseStaticOk _ = True

  public export
  ClauseStatic : StaticEffect bs -> Type
  ClauseStatic {bs} se = So (clauseStaticOk se)

  public export
  selfTapPayment : {0 bs : Bindings} -> Cost bs -> Bool
  selfTapPayment (Mana _) = False
  selfTapPayment (ScaledMana _ _) = False
  selfTapPayment TapSymbol = True
  selfTapPayment UntapSymbol = True
  selfTapPayment (LoyaltySymbol _) = False
  selfTapPayment (Do _) = False
  selfTapPayment (Compound _) = False
  selfTapPayment (EitherCost l r) = selfTapPayment l || selfTapPayment r
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
  costTapOnce (ScaledMana _ _) = True
  costTapOnce TapSymbol = True
  costTapOnce UntapSymbol = True
  costTapOnce (LoyaltySymbol _) = True
  costTapOnce (Do _) = True
  costTapOnce (Compound cs) = selfTapOnce cs
  costTapOnce (EitherCost l r) = costTapOnce l && costTapOnce r
  costTapOnce ItsManaCost = True

  public export
  CostTapOnce : Cost bs -> Type
  CostTapOnce {bs} c = So (costTapOnce c)

  public export
  costPaidByYou : {0 bs : Bindings} -> Cost bs -> Bool
  costPaidByYou (Mana _) = True
  costPaidByYou (ScaledMana _ _) = True
  costPaidByYou TapSymbol = True
  costPaidByYou UntapSymbol = True
  costPaidByYou (LoyaltySymbol _) = True
  costPaidByYou (Do (ChangeLife who _)) = nounIsYou who
  costPaidByYou (Do (Does subj _ _)) = nounIsYou subj
  costPaidByYou (Do _) = True
  costPaidByYou (Compound cs) = costsPaidByYou cs
  costPaidByYou (EitherCost l r) = costPaidByYou l && costPaidByYou r
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
  costOffBattlefield (ScaledMana _ _) = True
  costOffBattlefield TapSymbol = False
  costOffBattlefield UntapSymbol = False
  costOffBattlefield (LoyaltySymbol _) = False
  costOffBattlefield (Do _) = True
  costOffBattlefield (Compound cs) = costsOffBattlefield cs
  costOffBattlefield (EitherCost l r) = costOffBattlefield l && costOffBattlefield r
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
  data AddedPayment : {0 bs : Bindings} -> Cost bs -> Type where
    AddedPaymentWritten : {0 c : Cost bs} ->
                          {auto 0 ok : So (costOffBattlefield c)} ->
                          AddedPayment c

public export
Ability : Type
Ability = AbilityAt []

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
