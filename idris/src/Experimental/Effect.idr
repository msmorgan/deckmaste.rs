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
  AsThoughGreater : {bs : Bindings} -> (ch : Stat) ->
                    (amt : Amount bs) ->
                    {auto 0 nd : So (isNil (amtDelta amt))} -> AsThough bs

public export
asThoughSort : {0 bs : Bindings} -> AsThough bs -> PremiseSort
asThoughSort (AsThoughOf _) = ObjectPremise
asThoughSort (AsThoughMana _ _ _) = ManaPremise
asThoughSort (AsThoughGreater _ _) = ValuePremise


public export
twoPartiesOk : {bs : Bindings} -> Noun bs Player -> Bool
twoPartiesOk (Both l r) = case (nounPlur l, nounPlur {bs = nomIntro l} r) of
  (OneOf, OneOf) => True
  _ => False
twoPartiesOk (Described d _) = (detQuant d >>= quantExact) == Just 2
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

public export
data CounterKindSource : Bindings -> Type where
  PrintedKind : CounterKind -> CounterKindSource bs
  ChosenKind : (menu : List CounterKind) ->
               {auto 0 ne : NonEmpty menu} -> CounterKindSource bs
  DistinctChosenKinds : (menu : List CounterKind) ->
                        {auto 0 ne : NonEmpty menu} -> CounterKindSource bs
  BoundKind : {auto 0 ok : countChoice (QSort CounterKindQ) bs = 1} ->
              CounterKindSource bs
  ThoseKinds : {auto 0 ok : countOutcomes CountersPut bs = 1} ->
               CounterKindSource bs
  OwnKinds : CounterKindSource bs
  SameAs : (src : Noun bs Object) -> CounterKindSource bs

public export
counterHolderKind : Kind -> Bool
counterHolderKind k = kindLte k (Object \/ Player)

public export
counterSourceScope : {0 bs : Bindings} -> CounterKindSource bs -> Kind -> Bool
counterSourceScope (PrintedKind c) k = counterScope c == k
counterSourceScope (ChosenKind menu) k = all (\c => counterScope c == k) menu
counterSourceScope (DistinctChosenKinds menu) k = all (\c => counterScope c == k) menu
counterSourceScope BoundKind k = counterHolderKind k
counterSourceScope ThoseKinds k = counterHolderKind k
counterSourceScope OwnKinds k = counterHolderKind k
counterSourceScope (SameAs _) k = kindLte k Object

public export
CounterSourceScope : {bs : Bindings} -> CounterKindSource bs -> Kind -> Type
CounterSourceScope s k = So (counterSourceScope s k)

public export
optCounterSourceScope : {0 bs : Bindings} -> Maybe (CounterKindSource bs) -> Kind -> Bool
optCounterSourceScope Nothing k = counterHolderKind k
optCounterSourceScope (Just s) k = counterSourceScope s k

public export
OptCounterSourceScope : {bs : Bindings} -> Maybe (CounterKindSource bs) -> Kind -> Type
OptCounterSourceScope s k = So (optCounterSourceScope s k)

public export
unitAmount : {0 bs : Bindings} -> Amount bs -> Bool
unitAmount (Lit (S Z)) = True
unitAmount _ = False

||| "the same number and kind of counters": `SameAs` carries the number of
||| counters as well as their kinds, so the amount slot is the unit placeholder.
public export
kindAmountOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
               Amount bs -> CounterKindSource cs -> Bool
kindAmountOk a (SameAs _) = unitAmount a
kindAmountOk _ _ = True

public export
KindAmountOk : {bs : Bindings} -> {cs : Bindings} ->
               Amount bs -> CounterKindSource cs -> Type
KindAmountOk a s = So (kindAmountOk a s)

public export
optKindAmountOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
                  Amount bs -> Maybe (CounterKindSource cs) -> Bool
optKindAmountOk _ Nothing = True
optKindAmountOk a (Just s) = kindAmountOk a s

public export
OptKindAmountOk : {bs : Bindings} -> {cs : Bindings} ->
                  Amount bs -> Maybe (CounterKindSource cs) -> Type
OptKindAmountOk a s = So (optKindAmountOk a s)

public export
kindSourceIntro : {bs : Bindings} -> CounterKindSource bs -> Bindings
kindSourceIntro (SameAs src) = nomIntro src
kindSourceIntro _ = bs


public export
record InstrProfile (bs : Bindings) where
  constructor MkInstrProfile
  pre : Bindings
  announced : Bindings
  rider : Maybe Bindings
  deed : List Binding

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
  tokenCanonical : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenCanonical t = colorsDistinct t.colors && typesDistinct t.line.tys
                       && supersDistinct t.supers

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
  data CharOp = Adds | Sets | Loses

  public export
  data QualityPayload : Bindings -> Type where
    Bundle : (t : TokenChars bs) -> (ret : Maybe CardType) -> QualityPayload bs
    EveryTypeOf : (space : TypeSpace) -> QualityPayload bs
    ChosenQuality : (q : Predicate bs Object) -> QualityPayload bs
    Colored : (cs : ColorSpec) -> QualityPayload bs

  public export
  lossWritesTypes : {0 bs : Bindings} -> TokenChars bs -> Bool
  lossWritesTypes t =
    isNothing t.pt && not (someWritten t.colors) && not (someWritten t.abilities)
      && isNothing t.name && not (someWritten t.quals)
      && (lineNonEmpty t.line || someWritten t.supers)

  public export
  bundleOk : {0 bs : Bindings} -> CharOp -> Maybe CardType -> Maybe Zone ->
             TokenChars bs -> Maybe CardType -> Bool
  bundleOk Adds ty z t ret =
    (addsSomething ty t.line || someWritten t.supers) && addedFits ty t.line
      && tokenCanonical t && tokenAbilitiesOk t && tokenQualsFit t
      && additionUnnamed t && isNothing ret
  bundleOk Sets ty z t ret =
    zoneIsB z Battlefield && lineNonEmpty t.line && addedFits ty t.line
      && tokenAbilitiesOk t && tokenCanonical t && tokenQualsFit t
      && retentionOk t.line ret
  bundleOk Loses ty z t ret =
    zoneIsB z Battlefield && lossWritesTypes t && tokenCanonical t
      && isNothing ret

  public export
  colorOpOk : CharOp -> ColorSpec -> Bool
  colorOpOk Sets _ = True
  colorOpOk _ (SomeColors []) = False
  colorOpOk _ _ = True

  public export
  becomesOk : {bs : Bindings} -> CharOp -> Noun bs Object -> QualityPayload bs -> Bool
  becomesOk op n (Bundle t ret) = bundleOk op (nounTy n) (nounZone n) t ret
  becomesOk op n (EveryTypeOf space) =
    zoneIsB (nounZone n) Battlefield && spaceHosted space (nounTy n)
  becomesOk op n (ChosenQuality q) = qualityReadOk q && hostedRead q n
  becomesOk op n (Colored cs) = colorSpecOk cs && colorOpOk op cs

  public export
  BecomesOk : {bs : Bindings} -> CharOp -> Noun bs Object -> QualityPayload bs -> Type
  BecomesOk op n q = So (becomesOk op n q)

  public export
  becomesKind : {0 bs : Bindings} -> CharOp -> QualityPayload bs -> StaticKind
  becomesKind _ (Colored _) = ColorSet
  becomesKind Adds _ = TypeAddition
  becomesKind Sets _ = TypeSet
  becomesKind Loses _ = TypeLoss


  public export
  tokenHeadTy : {0 bs : Bindings} -> TokenChars bs -> Maybe CardType
  tokenHeadTy t = lastType t.line.tys

  public export
  data TokenSpec : Bindings -> Type where
    ||| The creating spell or ability defines the token's characteristic
    ||| values [CR#111.3] and sets its name and subtypes [CR#111.4]; each
    ||| obligation names one thing it must get right.
    TokenWritten : (t : TokenChars bs) ->
                   {auto 0 tt : So (tokenTyped t)} ->
                   {auto 0 tp : So (tokenPtOk t)} ->
                   {auto 0 sf : So (subsFitLine t.line.subs t.line.tys)} ->
                   {auto 0 ta : So (tokenAbilitiesOk t)} ->
                   {auto 0 tc : So (tokenCanonical t)} ->
                   {auto 0 qf : So (tokenQualsFit t)} -> TokenSpec bs
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

  -- Only modifications are signed; no layer subtracts a characteristic [CR#613.4b,613.4c].
  public export
  ptOpOk : {0 b1 : Bindings} -> {0 b2 : Bindings} ->
           CharOp -> PtShift b1 -> PtShift b2 -> Bool
  ptOpOk Adds _ _ = True
  ptOpOk Sets p t = shiftRises p && shiftRises t
  ptOpOk Loses _ _ = False

  public export
  ptOpKind : CharOp -> StaticKind
  ptOpKind Adds = PtDelta
  ptOpKind Sets = BasePtSet
  ptOpKind Loses = PtDelta

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

  public export
  costShiftDelta : {bs : Bindings} -> CostShift bs -> List Binding
  costShiftDelta (CostLess a _) = amtDelta a
  costShiftDelta (CostMore a) = amtDelta a
  costShiftDelta (CostShiftRun _ _ _) = []

  namespace Static
    public export
    data StaticSpec : Bindings -> Type where
      Gets : (op : CharOp) -> (n : Noun bs Object) ->
             (pow : PtShift (selfSubjIntro n)) ->
             (tou : PtShift (shiftIntro pow)) ->
             {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
             {auto 0 lo : So (ptOpOk op pow tou)} ->
             StaticSpec bs
      DefinesPt : (n : Noun bs Object) -> (sl : DefinedSlots) ->
                  (amt : Amount (selfSubjIntro n)) ->
                  {auto 0 sd : SelfDefined n} ->
                  StaticSpec bs
      SwitchesPt : (n : Noun bs Object) ->
                   {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                   StaticSpec bs
      Costs : {k : Kind} -> (n : Noun bs k) -> (sh : CostShift bs) ->
                    {auto 0 cs : CostSubject n} ->
                    StaticSpec bs
      AltCost : {k : Kind} -> (n : Noun bs k) ->
                (c : Maybe (Cost (selfSubjIntro n))) ->
                {auto 0 ap : AltPayment c} ->
                {auto 0 cs : CostSubject n} -> StaticSpec bs
      AddedCost : (c : Cost bs) -> (offered : Bool) ->
                  {auto 0 ap : AddedPayment c} -> StaticSpec bs
      DefinesLetter : (l : Letter) -> (amt : Amount bs) ->
                      {auto 0 ok : So (anyOpenLetter l bs)} -> StaticSpec bs
      Gains : (n : Noun bs Object) -> (ab : AbilityAt bs) ->
              {auto 0 ok : GrantSubject ab n} ->
              {auto 0 gr : Grantable ab} -> StaticSpec bs
      GainsAbilitiesOf : (n : Noun bs Object) ->
                         (cls : List AbilityClass) ->
                         (src : Noun (nomIntro n) Object) ->
                         (except : Maybe (Predicate (nomIntro src) Object)) ->
                         {auto 0 ne : NonEmpty cls} ->
                         {auto 0 dc : So (distinctClasses cls)} ->
                         StaticSpec bs
      Deontic : {k : Kind} -> (n : Noun bs k) ->
                (c : Compulsion (selfSubjIntro n)) ->
                (deeds : Deeds) -> (role : Role) ->
                (bound : Maybe (CountBound (nomIntro n))) ->
                (patient : DeonticPatient {bs = nomIntro n} deeds role) ->
                (asThough : Maybe (AsThough (nomIntro n))) ->
                (rider : DeonticRider (deonticPatientIntro patient)) ->
                {auto 0 ne : NonEmpty deeds} ->
                {auto 0 dd : So (distinctDeeds deeds)} ->
                {auto 0 kd : KnownActs deeds} ->
                {auto 0 dp : DeedFits deeds role k (nounIsAbility n) (nounHeadTys n) (nounZone n)} ->
                {auto 0 bd : So (deonticBoundOk deeds bound)} ->
                {auto 0 pt : So (deonticPatientOk n deeds role patient rider)} ->
                {auto 0 at : So (asThoughOk c deeds asThough)} ->
                {auto 0 rd : So (deonticRiderOk deeds role c patient
                                                (isJust asThough) rider)} ->
                StaticSpec bs
      KeepsUnspentMana : (who : Noun bs Player) ->
                         (what : ManaHeld (nomIntro who)) -> StaticSpec bs
      Skips : (who : Noun bs Player) -> (part : TurnPart) -> StaticSpec bs
      Becomes : (n : Noun bs Object) -> (op : CharOp) -> (q : QualityPayload bs) ->
                {auto 0 ok : BecomesOk op n q} -> StaticSpec bs
      AlsoOffBattlefield : (se : StaticSpec bs) ->
                           {auto 0 nx : NotExtended se} -> StaticSpec bs
      DoesntRemove : (se : StaticSpec bs) ->
                     (n : Noun (staticIntro se) Object) ->
                     {auto 0 nc : NotCarvedOut se} -> StaticSpec bs
      BecomesCopy : (n : Noun bs Object) -> (src : Noun (nomIntro n) Object) ->
                    (exc : List (CopyExcept (nomIntro src))) ->
                    {auto 0 pm : PerMember src} -> StaticSpec bs
      LosesAllAbilities : (n : Noun bs Object) ->
                          (except : Maybe (Predicate (selfSubjIntro n) Object)) ->
                          {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                          StaticSpec bs
      LosesAbilities : (n : Noun bs Object) -> (abl : List (AbilityLost bs)) ->
                       {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                       {auto 0 ne : NonEmpty abl} ->
                       StaticSpec bs
      GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                     {auto 0 zn : ZoneIs (nounZone what) Battlefield} -> StaticSpec bs
      Intercepts : (ev : GameEvent bs) -> (alts : List (GameEvent bs)) ->
                   (window : Maybe (TriggerWindow bs)) ->
                   (repl : Instruction (interceptCtx alts ev)) ->
                   (use : ReplUse) ->
                   (limit : Maybe UsageLimit) ->
                   {auto 0 ul : So (untriggeredLimitOk limit)} ->
                   {auto 0 ok : Interceptable ev} ->
                   {auto 0 oks : InterceptableArms alts} -> StaticSpec bs
      DamageRule : (kind : DamageKind) ->
                   (src : DamageAgent bs) ->
                   (scope : DamageScope (agentIntro src)) ->
                   (op : DamageOp (scopeIntro scope)) ->
                   (use : ReplUse) ->
                   {auto 0 su : So (damageOpUseOk op use)} ->
                   StaticSpec bs
      CantPrevent : (kind : DamageKind) -> (what : Unpreventable bs) ->
                    (ban : PreventionBan) -> StaticSpec bs
      Conditionally : {0 bs : Bindings} ->
                      (se : StaticSpec bs) ->
                      (c : Condition (staticIntro se)) ->
                      (marking : CondMarking) ->
                      {auto 0 mk : MarkingOk marking c} -> StaticSpec bs
      OnlyDuring : (p : TurnPart) -> (w : Maybe (Noun bs Player)) ->
                   (se : StaticSpec bs) ->
                   {auto 0 wk : WindowOk p w} ->
                   StaticSpec bs
      -- "doesn't lose the game for having 0 or less life" [CR#704.5a]
      NoLossFromZeroLife : (who : Noun bs Player) -> StaticSpec bs
      Visibility : (v : ExposeVerb) -> (who : Noun bs Player) ->
                   (what : VisibleThing (nomIntro who)) ->
                   {auto 0 vo : VisibilityOk v what} -> StaticSpec bs
      TriggersAdditionally : (ev : GameEvent bs) -> (q : Quantity bs) ->
                             {auto 0 nz : NonZeroQ q} ->
                             {auto 0 wf : WellFormedQ q} ->
                             {auto 0 lt : So (isNil (quantDelta q))} ->
                             {auto 0 ok : So (triggerCountOk (eventName ev))} ->
                             StaticSpec bs
      EntersRider : (n : Noun bs Object) -> (rider : TokenRider (nomIntro n)) ->
                    {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                    StaticSpec bs
      EntersChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) -> (disc : Disclosure) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticSpec bs
      AttachChoice : (n : Noun bs Object) -> (q : ChoiceSort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                     StaticSpec bs
      AndAlso : {0 n : Nat} -> (subject : Maybe (Noun bs Object)) ->
                (parts : StaticParts n (subjCtx subject)) ->
                {auto 0 ne : IsSucc n} -> StaticSpec bs

  public export
  gatePayer : Binding
  gatePayer = MkBinding TheD Player OneOf (PlayerP False)

  public export
  data Compulsion : Bindings -> Type where
    Forbid : Compulsion bs
    Require : Compulsion bs
    GatedBy : (c : Cost (Effect.gatePayer :: bs)) -> Compulsion bs
    Permit : Compulsion bs

  public export
  data CountBound : Bindings -> Type where
    MoreThan : (k : Amount bs) -> CountBound bs
    Additional : (q : Quantity bs) ->
                 {auto 0 nz : NonZeroQ q} ->
                 {auto 0 wf : WellFormedQ q} -> CountBound bs

  public export
  boundDelta : {bs : Bindings} -> CountBound bs -> List Binding
  boundDelta (MoreThan k) = amtDelta k
  boundDelta (Additional q) = quantDelta q

  public export
  deonticBoundOk : {bs : Bindings} -> Deeds -> Maybe (CountBound bs) -> Bool
  deonticBoundOk _ Nothing = True
  deonticBoundOk ds (Just b) = all deedBoundedOk ds && isNil (boundDelta b)

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
    deedFits ds r k (nounIsAbility m) (nounHeadTys m)
             (if moved then Nothing else nounZone m)

  public export
  counterpartNotSelf : {bs : Bindings} -> {k : Kind} -> {ka : Kind} ->
                       (n : Noun bs k) -> Noun (nomIntro n) ka -> Bool
  counterpartNotSelf n (Pro r pl Whole) =
    not (reachTracksObject r) || countReach r pl (nounDelta n) == 0
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
  notConditional : {0 bs : Bindings} -> StaticSpec bs -> Bool
  notConditional (Conditionally _ _ _) = False
  notConditional _ = True


  public export
  NotConditional : StaticSpec bs -> Type
  NotConditional {bs} se = So (notConditional se)

  public export
  notWindowed : {0 bs : Bindings} -> StaticSpec bs -> Bool
  notWindowed (OnlyDuring _ _ _) = False
  notWindowed _ = True

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
  data Unpreventable : Bindings -> Type where
    DamageDescribed : (src : DamageAgent bs) ->
                      (scope : DamageScope (agentIntro src)) ->
                      Unpreventable bs
    ThatDamage : {auto 0 ok : So (damageDealtInScope bs)} -> Unpreventable bs

  public export
  unpreventableIntro : {bs : Bindings} -> Unpreventable bs -> Bindings
  unpreventableIntro (DamageDescribed src scope) = scopeIntro scope
  unpreventableIntro ThatDamage = bs

  public export
  data PreventionBan = NoPreventionOnly | NoRedirectEither

  public export
  data PreventCut : Bindings -> Type where
    CutAll : PreventCut bs
    CutSome : (amt : Amount bs) -> PreventCut bs
    Shield : (amt : Amount bs) -> PreventCut bs
    CutAllBut : (amt : Amount bs) -> PreventCut bs
    CutHalf : (r : RoundMode) -> PreventCut bs

  public export
  cutIntro : {bs : Bindings} -> PreventCut bs -> Bindings
  cutIntro CutAll = bs
  cutIntro (CutSome amt) = amtIntro amt
  cutIntro (Shield amt) = amtIntro amt
  cutIntro (CutAllBut amt) = amtIntro amt
  cutIntro (CutHalf _) = bs

  -- [CR#615.7] A shield counts damage across events until its amount is spent.
  public export
  shieldUseOk : {0 bs : Bindings} -> PreventCut bs -> ReplUse -> Bool
  shieldUseOk (Shield _) Repeatedly = True
  shieldUseOk (Shield _) NextTimeOnly = False
  shieldUseOk _ _ = True

  public export
  ShieldUse : {0 bs : Bindings} -> PreventCut bs -> ReplUse -> Type
  ShieldUse cut use = So (shieldUseOk cut use)

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
  data DamageOp : Bindings -> Type where
    Prevent : (cut : PreventCut bs) ->
              (also : Maybe (Instruction (outcomeB DamagePrevented :: cutIntro cut))) ->
              DamageOp bs
    Redirect : {k : Kind} -> (cut : PreventCut bs) ->
               (to : Noun (cutIntro cut) k) ->
               {auto 0 rk : DamageRecipient to} ->
               {auto 0 one : SingleRecipient to} -> DamageOp bs
    Scale : (sc : DamageScale bs) -> DamageOp bs

  public export
  damageOpIntro : {bs : Bindings} -> DamageOp bs -> Bindings
  damageOpIntro (Prevent cut _) = cutIntro cut
  damageOpIntro (Redirect _ to) = nomIntro to
  damageOpIntro (Scale sc) = scaleIntro sc

  public export
  damageOpKind : {0 bs : Bindings} -> DamageOp bs -> StaticKind
  damageOpKind (Prevent _ _) = Prevention
  damageOpKind (Redirect _ _) = Replacement
  damageOpKind (Scale _) = Replacement

  public export
  damageOpUseOk : {0 bs : Bindings} -> DamageOp bs -> ReplUse -> Bool
  damageOpUseOk (Prevent cut _) use = shieldUseOk cut use
  damageOpUseOk (Redirect cut _) use = shieldUseOk cut use
  damageOpUseOk (Scale _) _ = True

  public export
  isCoord : {0 bs : Bindings} -> StaticSpec bs -> Bool
  isCoord (AndAlso _ _) = True
  isCoord _ = False

  public export
  NotCoord : StaticSpec bs -> Type
  NotCoord {bs} se = So (not (isCoord se))

  public export
  notExtended : {0 bs : Bindings} -> StaticSpec bs -> Bool
  notExtended (AlsoOffBattlefield _) = False
  notExtended _ = True

  public export
  NotExtended : StaticSpec bs -> Type
  NotExtended {bs} se = So (notExtended se)

  public export
  notCarvedOut : {0 bs : Bindings} -> StaticSpec bs -> Bool
  notCarvedOut (DoesntRemove _ _) = False
  notCarvedOut _ = True

  public export
  NotCarvedOut : StaticSpec bs -> Type
  NotCarvedOut {bs} se = So (notCarvedOut se)

  public export
  staticKind : {0 bs : Bindings} -> StaticSpec bs -> StaticKind
  staticKind (DefinesLetter _ _) = LetterDefinition
  staticKind (Gets op _ _ _) = ptOpKind op
  staticKind (DefinesPt _ _ _) = PtDefinition
  staticKind (SwitchesPt _) = PtSwitch
  staticKind (Costs _ _) = CostModification
  staticKind (AltCost _ _) = CostModification
  staticKind (AddedCost _ _) = CostModification
  staticKind (Gains _ _) = KeywordGrant
  staticKind (GainsAbilitiesOf _ _ _ _) = KeywordGrant
  staticKind (Deontic _ _ _ _ _ _ _ _) = DeedRestriction
  staticKind (Skips _ _) = TurnSkip
  staticKind (KeepsUnspentMana _ _) = ManaPersistence
  staticKind (Becomes _ op q) = becomesKind op q
  staticKind (BecomesCopy _ _ _) = CopyEffect
  staticKind (LosesAllAbilities _ _) = AbilityLoss
  staticKind (LosesAbilities _ _) = AbilityLoss
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _ _ _ _) = Replacement
  staticKind (DamageRule _ _ _ op _) = damageOpKind op
  staticKind (CantPrevent _ _ _) = Prevention
  staticKind (OnlyDuring _ _ se) = staticKind se
  staticKind (Conditionally _ _ _) = Conditional
  staticKind (AlsoOffBattlefield se) = staticKind se
  staticKind (DoesntRemove se _) = staticKind se
  staticKind (NoLossFromZeroLife _) = OutcomeImmunity
  staticKind (Visibility _ _ _) = VisibilityRider
  staticKind (TriggersAdditionally _ _) = TriggerMultiplier
  staticKind (EntersRider _ _) = EntryRider
  staticKind (EntersChoice _ _ _ _) = EntryRider
  staticKind (AttachChoice _ _ _) = Replacement
  staticKind (AndAlso _ _) = Coordination


  public export
  staticIntro : {bs : Bindings} -> StaticSpec bs -> Bindings
  staticIntro (DefinesLetter l amt) = defineLetter l (amtIntro amt)
  staticIntro (Gets _ n pow tou) = shiftDelta tou ++ shiftDelta pow ++ selfSubjIntro n
  staticIntro (DefinesPt n _ amt) =
    outcomeB NamedNumber :: (amtDelta amt ++ selfSubjIntro n)
  staticIntro (SwitchesPt n) = selfSubjIntro n
  staticIntro (Costs n sh) = costShiftDelta sh ++ selfSubjIntro n
  staticIntro (AltCost n _) = selfSubjIntro n
  staticIntro (AddedCost _ _) = bs
  staticIntro (Gains n ab) = abLetterDelta ab ++ selfSubjIntro n
  staticIntro (GainsAbilitiesOf n _ src _) = nomIntro src
  staticIntro (Deontic n _ _ _ _ _ _ _) = selfSubjIntro n
  staticIntro (Skips who _) = nomIntro who
  staticIntro (KeepsUnspentMana who _) = nomIntro who
  staticIntro (Becomes n _ _) = selfSubjIntro n
  staticIntro (BecomesCopy n _ _) = selfSubjIntro n
  staticIntro (LosesAllAbilities n _) = selfSubjIntro n
  staticIntro (LosesAbilities n _) = selfSubjIntro n
  staticIntro (GainsControl who what) = stampIntro (featureLabel ControlGrant) what
  staticIntro (Intercepts ev alts window repl use limit) = interceptCtx alts ev
  staticIntro (DamageRule kind src scope op use) = damageOpIntro op
  staticIntro (CantPrevent kind what ban) = unpreventableIntro what
  staticIntro (OnlyDuring _ _ se) = staticIntro se
  staticIntro (Conditionally se c _) = condIntro c
  staticIntro (AlsoOffBattlefield se) = staticIntro se
  staticIntro (DoesntRemove _ n) = nomIntro n
  staticIntro (NoLossFromZeroLife who) = nomIntro who
  staticIntro (Visibility _ who what) = visibleIntro what
  staticIntro (TriggersAdditionally _ _) = bs
  staticIntro (EntersRider n rider) = entryRiderIntro rider
  staticIntro (EntersChoice n _ _ _) = selfSubjIntro n
  staticIntro (AttachChoice n _ _) = selfSubjIntro n
  staticIntro (AndAlso _ parts) = partsIntro parts

  public export
  staticChoiceDelta : {bs : Bindings} -> StaticSpec bs -> List Binding
  staticChoiceDelta (EntersChoice _ q _ _) = [choiceB q]
  staticChoiceDelta (AttachChoice _ q _) = [choiceB q]
  staticChoiceDelta (AndAlso _ parts) = partsChoiceDelta parts
  staticChoiceDelta (AddedCost c _) = costDelta c
  staticChoiceDelta _ = []

  public export
  staticChoiceIntro : {bs : Bindings} -> StaticSpec bs -> Bindings
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
  data CtrlOverrideOk : {0 bs : Bindings} -> Noun bs Player -> Type where
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlOverrideOk n
    PerMemberController : {0 bs : Bindings} -> {0 ax : PossessorAxis} ->
                          {0 grp : Noun bs Object} ->
                          {0 ck : So (possessorKind ax Object)} ->
                          CtrlOverrideOk (PossessorOf ax grp {ck})

  public export
  data TokenRider : Bindings -> Type where
    EntersAs : {0 c : StatusCat} -> (v : StatusVal c) ->
               {auto 0 at : StatusMarkable v} -> TokenRider bs
    EntersAttacking : (whom : AttackDefender bs) -> TokenRider bs
    EntersTransformed : TokenRider bs
    EntersMelded : (into : String) -> TokenRider bs
    WithCounters : (amt : Amount bs) -> (kind : CounterKindSource (amtIntro amt)) ->
                   (mark : EntryCounterMark) ->
                   {auto 0 am : KindAmountOk amt kind} -> TokenRider bs
    Under : (who : Noun bs Player) ->
            {auto 0 one : CtrlOverrideOk who} -> TokenRider bs
    AsCopyOf : (optional : Bool) -> (src : Noun bs Object) ->
               (exc : List (CopyExcept bs)) ->
               {auto 0 pm : PerMember src} -> TokenRider bs

  public export
  entryRiderIntro : {bs : Bindings} -> TokenRider bs -> Bindings
  entryRiderIntro (WithCounters amt kind _) = kindSourceIntro kind
  entryRiderIntro (Under who) = nomIntro who
  entryRiderIntro _ = bs

  public export
  ridersZoneFree : {0 bs : Bindings} -> List (TokenRider bs) -> Bool
  ridersZoneFree [] = True
  ridersZoneFree (WithCounters _ _ _ :: rs) = ridersZoneFree rs
  ridersZoneFree (_ :: rs) = False

  public export
  ridersFitZone : {0 bs : Bindings} -> List (TokenRider bs) -> Zone -> Bool
  ridersFitZone rs z = ridersZoneFree rs || z == Battlefield

  public export
  RidersFit : {0 bs : Bindings} -> List (TokenRider bs) -> Zone -> Type
  RidersFit rs z = So (ridersFitZone rs z)

  public export
  data Cost : Bindings -> Type where
    Mana : (c : ManaCost) -> {auto 0 wr : ManaRun c} -> Cost bs
    ScaledCost : (c : Cost bs) -> (amt : Amount bs) ->
                 {auto 0 fe : ForEachAmount amt} -> Cost bs
    TapSymbol : Cost bs
    UntapSymbol : Cost bs
    LoyaltySymbol : (s : LoyaltyCost) -> Cost bs
    Do : (e : Instruction bs) -> {auto 0 ok : CostAction e} -> Cost bs
    Compound : {0 n : Nat} -> CostSeq n bs ->
               {auto 0 ne : IsSucc n} -> Cost bs
    EitherCost : (l : Cost bs) -> (r : Cost bs) ->
                 {auto 0 nl : NotCompound l} ->
                 {auto 0 nr : NotCompound r} -> Cost bs
    ItsManaCost : Cost bs

  public export
  forEachAmount : {0 bs : Bindings} -> Amount bs -> Bool
  forEachAmount (TimesOf _ _) = True
  forEachAmount _ = False

  public export
  ForEachAmount : Amount bs -> Type
  ForEachAmount {bs} a = So (forEachAmount a)

  public export
  costIntro : {bs : Bindings} -> Cost bs -> Bindings
  costIntro (Mana c) = if manaHasX c then letterB X :: bs else bs
  costIntro (ScaledCost c _) = costIntro c
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (LoyaltySymbol LoyaltyDownX) = letterB X :: bs
  costIntro (LoyaltySymbol _) = bs
  costIntro (Do e) = instrIntro e
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
                {auto 0 ne : NonEmpty ps} -> ManaRider bs
    SpendNotOn : (ps : List (SpendPurpose bs)) ->
                 {auto 0 ne : NonEmpty ps} -> ManaRider bs
    OnSpent : (mode : SpentMode) -> (only : Bool) ->
              (what : Noun bs Object) ->
              (says : Instruction (nomIntro what)) ->
              {auto 0 zn : ZoneIs (nounZone what) Stack} -> ManaRider bs

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
                  {auto 0 tc : So (tokenCanonical t)} ->
                  {auto 0 ta : So (tokenAbilitiesOk t)} ->
                  {auto 0 qf : So (tokenQualsFit t)} ->
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
    UntilCond : (c : Condition bs) -> Repetition bs
    AgainExcludingChosen : Repetition bs

  public export
  data RollRow : Bindings -> Type where
    MkRollRow : (results : Quantity bs) -> (e : Instruction bs) ->
                {auto 0 nz : NonZeroQ results} ->
                {auto 0 wf : WellFormedQ results} ->
                {auto 0 lt : So (quantLiteral results)} -> RollRow bs

  public export
  rowCount : {0 bs : Bindings} -> List (RollRow bs) -> Nat
  rowCount [] = Z
  rowCount (_ :: rs) = S (rowCount rs)

  public export
  data Instruction : Bindings -> Type where
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) ->
                 {auto 0 pm : PerMember to} ->
                 {auto 0 rk : DamageRecipient to} -> Instruction bs
    Fights : (a : Noun bs Object) ->
             {auto 0 za : ZoneIs (nounZone a) Battlefield} ->
             {auto 0 ta : So (featureNounOk Attacking Agent a)} ->
             {auto 0 pa : nounPlur a = OneOf} ->
             (b : Noun (nomIntro a) Object) ->
             {auto 0 zb : ZoneIs (nounZone b) Battlefield} ->
             {auto 0 tb : So (featureNounOk Attacking Agent b)} ->
             {auto 0 pb : nounPlur b = OneOf} -> Instruction bs
    SetStatus : {k : Kind} -> {c : StatusCat} -> (v : StatusVal c) ->
                (n : Noun bs k) -> {auto 0 sh : StatusHolder n} ->
                {auto 0 ok : ZoneIs (nounZone n) Battlefield} ->
                {auto 0 at : StatusMarkable v} -> Instruction bs
    TurnOver : (what : Noun bs Object) ->
               {auto 0 ok : ZoneIs (nounZone what) Battlefield} -> Instruction bs
    RemoveFromCombat : (n : Noun bs Object) ->
                       {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> Instruction bs
    AttachTo : {k : Kind} -> (what : Noun bs Object) ->
               {auto 0 zw : ZoneIs (nounZone what) Battlefield} ->
               (host : Noun (nomIntro what) k) ->
               {auto 0 hk : So (kindLte k (Object \/ Player))} -> Instruction bs
    Unattach : (what : Noun bs Object) ->
               {auto 0 zw : ZoneIs (nounZone what) Battlefield} -> Instruction bs
    BecomesBlocking : (n : Noun bs Object) ->
                      {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                      {auto 0 dn : So (featureNounOk Blocking Agent n)} ->
                      (what : Noun (nomIntro n) Object) ->
                      {auto 0 zw : ZoneIs (nounZone what) Battlefield} ->
                      {auto 0 dw : So (featureNounOk Blocking Patient what)} ->
                      Instruction bs
    StopsBlocking : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                    {auto 0 dn : So (featureNounOk Blocking Agent n)} ->
                    (what : Noun (nomIntro n) Object) ->
                    {auto 0 zw : ZoneIs (nounZone what) Battlefield} ->
                    {auto 0 dw : So (featureNounOk Blocking Patient what)} ->
                    Instruction bs
    BecomesAttacking : (n : Noun bs Object) ->
                       {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                       {auto 0 dn : So (featureNounOk Attacking Agent n)} ->
                       (whom : AttackDefender (nomIntro n)) -> Instruction bs
    Regenerate : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                 Instruction bs
    CantBe : {k : Kind} -> (e : Instruction bs) -> (deed : VerbLabel) ->
             (what : Noun (riderIntro e) k) ->
             {auto 0 kd : KnownAct deed} ->
             {auto 0 rd : So (deedRidesOk deed)} ->
             {auto 0 sub : DeedFits [deed] Patient k (nounIsAbility what) (nounHeadTys what) (nounZone what)} ->
             Instruction bs
    GainsDesignation : {k : Kind} -> (n : Noun bs k) -> (d : Designation) ->
                       (w : GivingWarrant d) ->
                       (span : Maybe (Duration (nomIntro n))) ->
                       {auto 0 sc : designationScope d = HeldBy k} ->
                       {auto 0 zn : DesignationHolder d (nounZone n)} -> Instruction bs
    Unlock : (door : Door bs) ->
             {auto 0 nh : DoorNamesHost door} -> Instruction bs
    GameBecomes : (d : Designation) ->
                  {auto 0 sc : designationScope d = HeldByGame} ->
                  {auto 0 at : So (designationChecked d)} -> Instruction bs
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Instruction bs
    GameDrawn : Instruction bs
    RestartsGame : Instruction bs
    SeparateIntoPiles : (who : Noun bs Player) ->
                        (grp : Noun (nomIntro who) Object) ->
                        (piles : Nat) -> (faces : List PileFace) ->
                        {auto 0 ff : FacesFit faces piles} ->
                        {auto 0 pl : nounPlur grp = ManyOf} -> Instruction bs
    Choose : {k : Kind} -> (first : Maybe (Noun bs Player)) ->
             (by : Maybe (Noun bs Player)) ->
             (n : Noun (agentCtx by) k) -> (disc : Disclosure) ->
             {auto 0 od : So (choiceOrderOk first by)} ->
             {auto 0 ch : So (choiceClauseOk by n)} -> Instruction bs
    ChoicesRevealed : (s : HiddenSort) -> Instruction bs
    Vote : (first : Maybe (Noun bs Player)) ->
           (voters : Noun (agentIntro first) Player) ->
           (disc : Disclosure) ->
           (ballot : Ballot (nomIntro voters)) -> Instruction bs
    Move : {k : Kind} -> (what : Noun bs k) -> (to : ZoneExpr (nomIntro what)) ->
           (riders : List (TokenRider (nomIntro what))) ->
           {auto 0 mk : Movable what} ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 pl : Placeable (nounTy what) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} -> Instruction bs
    CounterSpell : {k : Kind} -> (what : Noun bs k) ->
                   {auto 0 ct : Counterable what} -> Instruction bs
    Copy : {k : Kind} -> (src : CopySort) -> (agent : Noun bs Player) ->
           (what : Noun (nomIntro agent) k) ->
           (times : Amount (nomIntro what)) ->
           (exc : List (CopyExcept (amtIntro times))) ->
           {auto ph : Phrasal k} ->
           {auto 0 cp : CopySourceOk src what} ->
           Instruction bs
    ChooseNewTargets : {k : Kind} -> (what : Noun bs k) ->
                       {auto 0 cp : Copiable what} -> Instruction bs
    CopyTargets : {k : Kind} -> {kt : Kind} ->
                  (copy : Noun bs k) ->
                  (whom : Noun (nomIntro copy) kt) ->
                  {auto 0 cp : Copiable copy} ->
                  {auto 0 tk : Targetable kt} -> Instruction bs
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Instruction bs
    ExchangeLife : (parties : Noun bs Player) ->
                   {auto 0 tp : So (twoPartiesOk parties)} -> Instruction bs
    AddMana : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
              (prod : ProducedMana (amtIntro amt)) ->
              (riders : List (ManaRider (amtIntro amt))) ->
              Instruction bs
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           Instruction bs
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Instruction bs
    Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) ->
             (q : Quantity (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} ->
             {auto 0 zf : ZoneFree p} -> Instruction bs
    Shuffle : (whose : Noun bs Player) -> Instruction bs
    FlipCoins : (who : Noun bs Player) -> (count : FlipScope (nomIntro who)) ->
                Instruction bs
    RollDice : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
               (sides : DieSides (amtIntro count)) -> Instruction bs
    ResultsTable : (rows : List (RollRow bs)) ->
                   {auto 0 ne : IsSucc (rowCount rows)} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Instruction bs
    IgnoreOutcomes : (which : IgnoredOutcomes bs) ->
                     {auto 0 ok : So (ignorableFor which)} -> Instruction bs
    ShiftResult : (dir : Maybe ShiftDir) -> (amt : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} -> Instruction bs
    RollPlanarDie : (who : Noun bs Player) ->
                    (count : Amount (nomIntro who)) -> Instruction bs
    ChaosEnsues : (what : Maybe (Noun bs Object)) -> Instruction bs
    StoreResults : (on : Noun bs Object) ->
                   {auto 0 one : nounPlur on = OneOf} ->
                   {auto 0 ok : countOutcomes RollResult bs = 1} -> Instruction bs
    RerollStored : (who : Noun bs Player) -> (q : Quantity (nomIntro who)) ->
                   (whose : Noun (nomIntro who) Object) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 one : nounPlur whose = OneOf} -> Instruction bs
    Continuously : {0 bs : Bindings} ->
                   (se : StaticSpec bs) ->
                   (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 sp : SpanOk span} ->
                   {auto 0 cl : ClauseStatic se} -> Instruction bs
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (spec : TokenSpec (amtIntro count)) ->
             (riders : List (TokenRider (amtIntro count))) ->
             Instruction bs
    GetsEmblem : (who : Noun bs Player) -> (abl : List (AbilityAt [])) ->
                 {auto 0 ea : EmblemAbilities abl} -> Instruction bs
    PutCounters : {k : Kind} -> (amt : Amount bs) ->
                  (kind : CounterKindSource (amtIntro amt)) ->
                  (on : Noun (kindSourceIntro kind) k) ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 am : KindAmountOk amt kind} ->
                  {auto 0 sc : CounterSourceScope kind k} -> Instruction bs
    Distribute : {k : Kind} -> (v : DividedVerb bs) ->
                 (amt : Amount (divIntro v)) ->
                 (among : Noun (amtIntro amt) k) ->
                 {auto 0 gm : GroupMention among} ->
                 {auto 0 tk : DividedTakes (divTag v) among} -> Instruction bs
    RemoveCounters : {k : Kind} -> (q : Maybe (Quantity bs)) ->
                     (kind : Maybe (CounterKindSource bs)) ->
                     (from : Noun (optQuantIntro q) k) ->
                     {auto 0 wf : OptWellFormedQ q} ->
                     {auto 0 sc : OptCounterSourceScope kind k} ->
                     {auto 0 cm : CounterMemory from} -> Instruction bs
    MoveCounters : (amt : Amount bs) ->
                   (kind : Maybe (CounterKindSource bs)) ->
                   (src : Noun (amtIntro amt) Object) ->
                   (dst : Noun (nomIntro src) Object) ->
                   {auto 0 am : OptKindAmountOk amt kind} ->
                   {auto 0 sc : OptCounterSourceScope kind Object} ->
                   {auto 0 cm : CounterMemory src} ->
                   {auto 0 md : MoveDestination dst} ->
                   {auto 0 pm : PerMember dst} -> Instruction bs
    DoubleCounters : {k : Kind} -> (on : Noun bs k) ->
                     {auto 0 hk : So (counterHolderKind k)} ->
                     {auto 0 pm : PerMember on} -> Instruction bs
    LosesCounters : (who : Noun bs Player) ->
                    (kind : Maybe (CounterKindSource bs)) ->
                    (amt : Maybe (Amount (nomIntro who))) ->
                   {auto 0 sc : OptCounterSourceScope kind Player} -> Instruction bs
    Enact : (subj : Maybe (Noun bs Player)) -> (v : VerbLabel) ->
            (e : Instruction (agentCtx subj)) ->
            {auto 0 kn : KnownAct v} ->
            {auto 0 ag : So (enactAgentOk subj v)} ->
            {auto 0 ke : EnactKeepsOuter subj e} -> Instruction bs
    ControllerSacrifices : (n : Noun bs Object) ->
                           {auto 0 one : nounPlur n = OneOf} ->
                           {auto 0 zn : ZoneIs (nounZone n) Battlefield} -> Instruction bs
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          (times : PayTimes) ->
          {auto 0 pb : Payable c} ->
          {auto 0 ag : PayAgrees who c} -> Instruction bs
    May : (offer : Noun bs Player) -> (body : Instruction (mayCtx offer)) ->
          (ifDid : Maybe (Instruction (instrIntro body))) ->
          (ifNot : Maybe (Instruction (mayCtx offer))) -> Instruction bs
    IfDone : (body : Instruction bs) ->
             (ifDid : Maybe (Instruction (instrIntro body))) ->
             (ifNot : Maybe (Instruction bs)) ->
             {auto 0 en : ReflexEnclosure body} ->
             {auto 0 br : So (ifDoneArmed body ifDid ifNot)} -> Instruction bs
    OnlyIf : (e : Instruction bs) -> (c : Condition (preIntro e)) ->
             (otherwise : Maybe (Instruction (condDelta c ++ otherwiseCtx e))) ->
             Instruction bs
    If : (c : Condition bs) -> (e : Instruction (condIntro c)) ->
         (otherwise : Maybe (Instruction (otherwiseCtx e))) -> Instruction bs
    Define : (l : Letter) -> (amt : Amount bs) ->
             {auto 0 ok : So (anyOpenLetter l bs)} -> Instruction bs
    ForEachOf : {k : Kind} -> {auto ph : Phrasal k} ->
                (grp : Noun bs k) ->
                (body : Instruction (elemIntro grp)) ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                {auto 0 ko : KeepsOuter body} ->
                Instruction bs
    ForEachKindOf : (ax : KindAxis) -> (dom : Maybe (Noun bs Object)) ->
                    (q : QualitySort) ->
                    {auto 0 sc : kindAxisSort ax = Just q} ->
                    {auto 0 cl : So (kindDomainOk ax dom)} ->
                    (body : Instruction (kindValueIntro q dom)) ->
                    {auto 0 ko : KeepsOuter body} ->
                    Instruction bs
    Repeat : (rep : Repetition bs) -> Instruction bs
    Repeated : (n : Amount bs) -> (body : Instruction (amtIntro n)) ->
               {auto 0 ko : KeepsOuter body} -> Instruction bs
    Sequentially : {0 n : Nat} -> Instructions n bs ->
                   {auto 0 ne : IsSucc n} -> Instruction bs
    Simultaneously : {0 n : Nat} -> SimInstructions n bs ->
                     {auto 0 ne : IsSucc n} -> Instruction bs
    Modal : (q : Quantity bs) -> (modes : List (Maybe (Cost bs), Instruction bs)) ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit q (modeCount modes)} -> Instruction bs
    Delayed : (ev : GameEvent bs) ->
              (alts : List (GameEvent bs)) ->
              (span : Maybe (Duration bs)) ->
              Instruction (delayedCtx alts ev) ->
              {auto 0 so : DelaySpanOk span} -> Instruction bs
    InsteadOf : (replaced : Instruction bs) -> (repl : Instruction (replacedCtx replaced)) ->
                {auto 0 na : NotInstead replaced} ->
                {auto 0 nb : NotInstead repl} -> Instruction bs
    HeldUntil : (e : Instruction bs) -> (ev : GameEvent (annIntro e)) ->
                {auto 0 ok : HeldClause e} -> Instruction bs
    Reflexively : (body : Instruction bs) -> (trig : Instruction (reflexCtx body)) ->
                  {auto 0 en : ReflexEnclosure body} -> Instruction bs
    ThisWay : (body : Instruction bs) -> (ev : GameEvent (instrIntro body)) ->
              (trig : Instruction (thisWayCtx body ev)) ->
              {auto 0 oc : ThisWayOutcome body} -> Instruction bs

    DoesntUntapNext : (n : Noun bs Object) -> (steps : Amount bs) ->
                      {auto 0 ok : ZoneIs (nounZone n) Battlefield} -> Instruction bs
    SkipsNext : (who : Noun bs Player) -> (part : TurnPart) ->
                (count : Amount bs) -> Instruction bs
    ExtraTurn : (who : Noun bs Player) -> (count : Amount bs) -> Instruction bs
    AdditionalPart : (who : Maybe (Noun bs Player)) -> (part : TurnPart) ->
                     (anchor : Maybe TurnPart) ->
                     (count : Amount bs) ->
                     (followedBy : Maybe TurnPart) ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AddedPartWritten anchor} ->
                     {auto 0 fb : AddedPartWritten followedBy} -> Instruction bs

  public export
  heldUntilOk : {0 bs : Bindings} -> Instruction bs -> Bool
  heldUntilOk (SetStatus PhasedOut _) = True
  heldUntilOk (Move _ _ _) = True
  heldUntilOk (Enact Nothing _ (Move _ _ _)) = True
  heldUntilOk _ = False

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
  reflexEncloseUse : {0 bs : Bindings} -> Instruction bs -> EncloseUse
  reflexEncloseUse (ControllerSacrifices _) = EncReflexive
  reflexEncloseUse (SkipsNext _ _ _) = EncNotYetTaken
  reflexEncloseUse (ExtraTurn _ _) = EncNotYetTaken
  reflexEncloseUse (AdditionalPart (Just _) _ _ _ _) = EncNotYetTaken
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncReflexive
  reflexEncloseUse (Pay _ _ _) = EncReflexive
  reflexEncloseUse (Enact _ _ _) = EncReflexive
  reflexEncloseUse (SeparateIntoPiles _ _ _ _) = EncReflexive
  reflexEncloseUse (ChooseNewTargets _) = EncReflexive
  reflexEncloseUse (Create _ _ _ _) = EncReflexive
  reflexEncloseUse (PutCounters _ _ _) = EncReflexive
  reflexEncloseUse (RemoveCounters _ _ _) = EncReflexive
  reflexEncloseUse (MoveCounters _ _ _ _) = EncReflexive
  reflexEncloseUse (DoubleCounters _) = EncReflexive
  reflexEncloseUse (Move _ _ _) = EncReflexive
  reflexEncloseUse (Expose _ _ _) = EncReflexive
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive
  reflexEncloseUse (Choose _ _ _ _) = EncReflexive
  reflexEncloseUse (Vote _ _ _ _) = EncReflexive
  reflexEncloseUse (Search _ _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  reflexEncloseUse (FlipCoins _ _) = EncReflexive
  reflexEncloseUse (RollDice _ _ _) = EncReflexive
  reflexEncloseUse (ResultsTable _) = EncNotOneAction
  reflexEncloseUse (RollPlanarDie _ _) = EncReflexive
  reflexEncloseUse (RerollStored _ _ _) = EncReflexive
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  reflexEncloseUse (IfDone body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (IfDone _ _ _) = EncNotOneAction
  reflexEncloseUse (OnlyIf e _ _) = reflexEncloseUse e
  reflexEncloseUse (If _ _ _) = EncNotOneAction
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
  reflexEncloseUse _ = EncAgentless

  public export
  admitsReflexEnclosure : EncloseUse -> Bool
  admitsReflexEnclosure EncAgentless = False
  admitsReflexEnclosure EncNotOneAction = False
  admitsReflexEnclosure EncNotYetTaken = False
  admitsReflexEnclosure EncReflexive = True

  public export
  ReflexEnclosure : Instruction bs -> Type
  ReflexEnclosure e = So (admitsReflexEnclosure (reflexEncloseUse e))

  public export
  thisWayOutcomeOk : {0 bs : Bindings} -> Instruction bs -> Bool
  thisWayOutcomeOk (Delayed _ _ _ _) = False
  thisWayOutcomeOk (May _ body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (IfDone body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk _ = True

  public export
  ThisWayOutcome : Instruction bs -> Type
  ThisWayOutcome e = So (thisWayOutcomeOk e)

  public export
  payableOk : {0 bs : Bindings} -> Cost bs -> Bool
  payableOk (Mana _) = True
  payableOk (ScaledCost c _) = payableOk c
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
  costActionOk : {0 bs : Bindings} -> Instruction bs -> Bool
  costActionOk (DealDamage src _ _) = costNounOk src
  costActionOk (ControllerSacrifices n) = costNounOk n
  costActionOk (DoesntUntapNext n _) = costNounOk n
  costActionOk (ExtraTurn who _) = costNounOk who
  costActionOk (AdditionalPart Nothing _ _ _ _) = True
  costActionOk (AdditionalPart (Just who) _ _ _ _) = costNounOk who
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (TurnOver n) = costNounOk n
  costActionOk (SetStatus _ n) = costNounOk n
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
  costActionOk (SeparateIntoPiles _ grp _ _) = costNounOk grp
  costActionOk (CounterSpell _) = True
  costActionOk (Copy _ _ what _ _) = costNounOk what
  costActionOk (ChooseNewTargets what) = costNounOk what
  costActionOk (CopyTargets copy _) = costNounOk copy
  costActionOk (Choose _ _ n _) = costNounOk n
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
  costActionOk (RollPlanarDie who _) = costNounOk who
  costActionOk (RerollStored who _ _) = costNounOk who
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  costActionOk (DoubleCounters on) = costNounOk on
  costActionOk (Enact _ _ e) = costActionOk e
  costActionOk (May _ body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (IfDone body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (OnlyIf e _ otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (If _ e otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (Define _ _) = True
  costActionOk (ForEachOf _ body) = costActionOk body
  costActionOk (ForEachKindOf _ _ _ body) = costActionOk body
  costActionOk (Repeated _ body) = costRepeatedOk body
  costActionOk (Modal _ modes) = costModesOk modes
  costActionOk RestartsGame = False
  costActionOk (ChoicesRevealed _) = False
  costActionOk (Vote _ _ _ _) = False
  costActionOk (ResultsTable _) = False
  costActionOk (IgnoreOutcomes _) = False
  costActionOk (ShiftResult _ _) = False
  costActionOk (ChaosEnsues _) = False
  costActionOk (StoreResults _) = False
  costActionOk (Continuously _ _) = False
  costActionOk (Pay _ _ _) = False
  costActionOk (Repeat _) = False
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Delayed _ _ _ _) = False
  costActionOk (InsteadOf _ _) = False
  costActionOk (HeldUntil _ _) = False
  costActionOk (Reflexively _ _) = False
  costActionOk (ThisWay _ _ _) = False
  costActionOk (SkipsNext _ _ _) = False

  public export
  costActionOkOpt : {0 bs : Bindings} -> Maybe (Instruction bs) -> Bool
  costActionOkOpt Nothing = True
  costActionOkOpt (Just e) = costActionOk e

  public export
  costModesOk : {0 bs : Bindings} -> List (Maybe (Cost bs), Instruction bs) -> Bool
  costModesOk [] = True
  costModesOk ((_, e) :: es) = costActionOk e && costModesOk es

  public export
  CostAction : Instruction bs -> Type
  CostAction {bs} e = So (costActionOk e)

  public export
  HeldClause : Instruction bs -> Type
  HeldClause {bs} e = So (heldUntilOk e)

  public export
  isInstead : {0 bs : Bindings} -> Instruction bs -> Bool
  isInstead (InsteadOf _ _) = True
  isInstead _ = False

  public export
  NotInstead : Instruction bs -> Type
  NotInstead {bs} e = So (not (isInstead e))

  public export
  modeCount : {0 bs : Bindings} -> List (Maybe (Cost bs), Instruction bs) -> Nat
  modeCount [] = Z
  modeCount (_ :: es) = S (modeCount es)

  public export
  allCosted : {0 bs : Bindings} -> List (Maybe (Cost bs), Instruction bs) -> Bool
  allCosted [] = True
  allCosted ((Nothing, _) :: _) = False
  allCosted ((Just _, _) :: es) = allCosted es

  public export
  AllCosted : List (Maybe (Cost bs), Instruction bs) -> Type
  AllCosted {bs} modes = So (allCosted modes)

  public export
  data Instructions : Nat -> Bindings -> Type where
    Nil : Instructions Z bs
    (::) : (e : Instruction bs) ->
           Instructions n (instrIntro e) -> Instructions (S n) bs

  namespace Sim
    public export
    data SimInstructions : Nat -> Bindings -> Type where
      Nil : SimInstructions Z bs
      (::) : (e : Instruction bs) ->
             SimInstructions n (annIntro e) -> SimInstructions (S n) bs

  public export
  instrDelta : {bs : Bindings} -> Instruction bs -> Bindings
  instrDelta e = take (length (instrIntro e) `minus` length bs) (instrIntro e)

  public export
  KeepsOuterOf : (outer : Bindings) -> (out : Bindings) -> Type
  KeepsOuterOf outer out =
    out = take (length out `minus` length outer) out ++ outer

  public export
  KeepsOuter : {bs : Bindings} -> Instruction bs -> Type
  KeepsOuter {bs} e = KeepsOuterOf bs (instrIntro e)

  ||| A distributive deed either only adds to the agent stack, or closes the
  ||| partitives its own agents' choices published — "each player … sacrifices
  ||| the rest" [CR#700.8d] — which spends nothing the table shares
  ||| [CR#701.21a].
  public export
  data EachStackOk : (outer : Bindings) -> (out : Bindings) -> Type where
    EachOnlyAdds : {auto 0 ko : KeepsOuterOf outer out} -> EachStackOk outer out
    EachClosesOwnParts : {auto 0 ds : So (partsDistributed outer)} ->
                         {auto 0 cp : out = partsClosed outer} ->
                         EachStackOk outer out

  public export
  KeepsOuterEach : Plurality -> (outer : Bindings) -> (out : Bindings) -> Type
  KeepsOuterEach OneOf outer out = ()
  KeepsOuterEach ManyOf outer out = EachStackOk outer out

  ||| An enacted act names a subject only where the act's facts row gives its
  ||| agent role a player, the way `verbedVoiceOk` gates a verbed event.
  public export
  enactAgentOk : {0 bs : Bindings} -> Maybe (Noun bs Player) -> VerbLabel -> Bool
  enactAgentOk Nothing v = True
  enactAgentOk (Just _) v = deedKindOk v Agent Player

  public export
  EnactKeepsOuter : {bs : Bindings} -> (subj : Maybe (Noun bs Player)) ->
                    Instruction (agentCtx subj) -> Type
  EnactKeepsOuter Nothing e = ()
  EnactKeepsOuter (Just s) e =
    KeepsOuterEach (nounPlur s) (agentIntro s) (instrIntro e)

  public export
  profileIntro : InstrProfile bs -> Bindings
  profileIntro p = deed p ++ announced p

  public export
  instrIntro : {bs : Bindings} -> Instruction bs -> Bindings
  instrIntro e = profileIntro (instrProfile e)

  public export
  preIntro : {bs : Bindings} -> Instruction bs -> Bindings
  preIntro e = pre (instrProfile e)

  public export
  annIntro : {bs : Bindings} -> Instruction bs -> Bindings
  annIntro e = announced (instrProfile e)

  public export
  profileRider : InstrProfile bs -> Bindings
  profileRider p = fromMaybe (pre p) (rider p)

  public export
  riderIntro : {bs : Bindings} -> Instruction bs -> Bindings
  riderIntro e = profileRider (instrProfile e)

  ||| The last clause of a sequence publishes its deed with what it announced.
  public export
  lastProfile : InstrProfile as -> InstrProfile bs
  lastProfile p = MkInstrProfile (pre p) (profileIntro p) (rider p) []

  public export
  seqProfile : {bs : Bindings} -> {0 n : Nat} -> Instructions n bs -> InstrProfile bs
  seqProfile [] = MkInstrProfile bs bs Nothing []
  seqProfile (e :: []) = lastProfile (instrProfile e)
  seqProfile (e :: es) = reProfile (seqProfile es)

  ||| A simultaneous batch announces every clause's deed at once; its trailing
  ||| condition reads the last clause's own pre-stack.
  public export
  simCons : List Binding -> InstrProfile as -> InstrProfile bs
  simCons d p = MkInstrProfile (pre p) (d ++ announced p) Nothing []

  ||| An offer announces what its body announced; its deed stays a deed, so a
  ||| simultaneous clause cannot read a thing that may not have happened.
  public export
  offerProfile : InstrProfile as -> InstrProfile bs
  offerProfile p = MkInstrProfile (profileIntro p) (announced p) Nothing (deed p)

  public export
  mayProfile : {as : Bindings} -> (body : Instruction as) ->
               Maybe (Instruction (instrIntro body)) -> Maybe (Instruction as) ->
               InstrProfile bs
  mayProfile body Nothing _ = offerProfile (instrProfile body)
  mayProfile body (Just did) Nothing = offerProfile (instrProfile did)
  mayProfile body (Just did) (Just _) = offerProfile (instrProfile body)

  public export
  simLast : InstrProfile as -> InstrProfile bs
  simLast p = MkInstrProfile (pre p) (profileIntro p) Nothing []

  public export
  simProfile : {bs : Bindings} -> {0 n : Nat} -> SimInstructions n bs -> InstrProfile bs
  simProfile [] = MkInstrProfile bs bs Nothing []
  simProfile (e :: []) = simLast (instrProfile e)
  simProfile (e :: es) = simCons (deedDelta e) (simProfile es)

  public export
  distributedDelta : {bs : Bindings} -> (s : Noun bs Player) -> Bindings -> List Binding
  distributedDelta s out =
    pluralizeDelta (take (length out `minus` length (agentIntro s)) out)

  public export
  doesProfile : {bs : Bindings} -> Plurality -> (s : Noun bs Player) ->
                (v : VerbLabel) -> Instruction (agentIntro s) -> InstrProfile bs
  doesProfile ManyOf s v (Move what@(TheRest _ _) to _) =
    MkInstrProfile (distributedDelta s (nomIntro what) ++ nomIntro s)
                 (afterMoveTo to (partsClosed (nomIntro s)))
                 (Just (distributedDelta s (stampIntro (Just v) what) ++ nomIntro s))
                 ([])
  doesProfile ManyOf s v (Move what to _) =
    MkInstrProfile (distributedDelta s (nomIntro what) ++ nomIntro s)
                 (afterMoveTo to (distributedDelta s (moveIntro (Just v) what (Just (zoneSort to)))
                                    ++ nomIntro s))
                 (Just (distributedDelta s (stampIntro (Just v) what) ++ nomIntro s))
                 ([])
  doesProfile ManyOf s v (SetStatus _ n) =
    MkInstrProfile (nomIntro s)
                 (distributedDelta s (stampIntro (Just v) n) ++ nomIntro s)
                 Nothing
                 ([])
  doesProfile ManyOf s v e =
    MkInstrProfile (nomIntro s) (nomIntro s) Nothing (deedDelta e)
  doesProfile OneOf s v (Move what to _) =
    MkInstrProfile (nomIntro what)
                 (afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to))))
                 (Just (stampIntro (Just v) what))
                 ([])
  doesProfile OneOf s v (SetStatus st n) =
    MkInstrProfile (nomIntro n) (stampIntro (Just v) n) Nothing ([])
  doesProfile OneOf s v e = reProfile (instrProfile e)

  ||| A profile's fields never mention its index, so the agent's own profile is
  ||| the enacting clause's.
  public export
  reProfile : InstrProfile as -> InstrProfile bs
  reProfile p = MkInstrProfile (pre p) (announced p) (rider p) (deed p)

  public export
  replacedCtx : {bs : Bindings} -> Instruction bs -> Bindings
  replacedCtx (Sequentially es) = annSeqs es
  replacedCtx (May d body did notd) = replacedCtx body
  replacedCtx (IfDone body did notd) = replacedCtx body
  replacedCtx (OnlyIf e c oth) = replacedCtx e
  replacedCtx (If c e oth) = bs
  replacedCtx e = deedDelta e ++ annIntro e

  public export
  otherwiseCtx : {bs : Bindings} -> Instruction bs -> Bindings
  otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e

  public export
  annSeqs : {bs : Bindings} -> {0 n : Nat} -> Instructions n bs -> Bindings
  annSeqs [] = bs
  annSeqs (e :: es) = deedDelta e ++ annSeqs es

  public export
  deedDelta : {bs : Bindings} -> Instruction bs -> List Binding
  deedDelta e = deed (instrProfile e)

  public export
  sameIntro : Bindings -> List Binding -> InstrProfile bs
  sameIntro b d = MkInstrProfile b b Nothing d

  public export
  instrProfile : {bs : Bindings} -> Instruction bs -> InstrProfile bs
  instrProfile (DealDamage src amt to) = sameIntro (nomIntro to) [outcomeB DamageDealt]
  instrProfile (ControllerSacrifices n) =
    MkInstrProfile (MkBinding TheD Player OneOf (PlayerP False) :: selfSubjIntro n)
                 (MkBinding TheD Player OneOf (PlayerP False)
                    :: moveIntro (Just "Sacrifice") n (Just Graveyard))
                 Nothing
                 ([])
  instrProfile (Distribute (DividedDamage _) amt among) =
    sameIntro (nomIntro among)
              ([outcomeB DamageDealt])
  instrProfile (Distribute (DistributedCounters _) amt among) = sameIntro (nomIntro among) []
  instrProfile (Fights a b) = sameIntro (nomIntro b) []
  instrProfile (TurnOver n) = sameIntro (nomIntro n) []
  instrProfile (SetStatus _ n) = sameIntro (nomIntro n) []
  instrProfile (DoesntUntapNext n steps) = sameIntro (amtDelta steps ++ nomIntro n) []
  instrProfile (SkipsNext w _ count) = sameIntro (amtDelta count ++ nomIntro w) []
  instrProfile (ExtraTurn w count) =
    MkInstrProfile (amtDelta count ++ nomIntro w)
                 (turnRefB :: (amtDelta count ++ nomIntro w))
                 Nothing
                 ([])
  instrProfile (AdditionalPart who _ _ count _) = sameIntro (amtDelta count ++ agentIntro who) []
  instrProfile (LosesCounters who _ amt) = sameIntro (optAmtIntro amt) []
  instrProfile (RemoveFromCombat n) = sameIntro (nomIntro n) []
  instrProfile (AttachTo _ host) = sameIntro (nomIntro host) []
  instrProfile (Unattach what) = sameIntro (nomIntro what) []
  instrProfile (BecomesBlocking _ what) = sameIntro (nomIntro what) []
  instrProfile (StopsBlocking _ what) = sameIntro (nomIntro what) []
  instrProfile (BecomesAttacking n NoDefender) = sameIntro (nomIntro n) []
  instrProfile (BecomesAttacking _ (OneDefender whom)) = sameIntro (nomIntro whom) []
  instrProfile (Regenerate n) = sameIntro (nomIntro n) []
  instrProfile (CantBe e _ _) = instrProfile e
  instrProfile (GainsDesignation n _ _ _) = sameIntro (nomIntro n) []
  instrProfile (Unlock door) = sameIntro (doorIntro door) []
  instrProfile (GameBecomes _) = sameIntro bs []
  instrProfile (Concludes _ who) = sameIntro (nomIntro who) []
  instrProfile GameDrawn = sameIntro bs []
  instrProfile RestartsGame = sameIntro bs []
  instrProfile (SeparateIntoPiles who grp piles faces) =
    MkInstrProfile (nomIntro grp)
                 (partsClosed (nomIntro grp))
                 Nothing
                 ([MkBinding TheD Pile ManyOf
                             (PileP (nounZone grp) (Just piles) (pileMentionFace faces))])
  instrProfile (CounterSpell what) = sameIntro (nomIntro what) []
  instrProfile (Copy {k} {ph} src agent what times exc) =
    sameIntro (amtIntro times)
              ([MkBinding TheD k (outputPlur (nounPlur what) (amtPlur times))
                          (copyPayloadIn ph (nounIsAbility what) (nounTy what)
                                         (copyLandsIn src (nounZone what)))])
  instrProfile (ChooseNewTargets what) = sameIntro (nomIntro what) []
  instrProfile (CopyTargets copy whom) = sameIntro (nomIntro whom) []
  instrProfile (Choose _ by n _) = sameIntro (chooseIntro by n) []
  instrProfile (ChoicesRevealed _) = sameIntro bs []
  instrProfile (Vote _ _ _ _) = sameIntro bs []
  instrProfile (Move what to _) =
    MkInstrProfile (nomIntro what)
                 (afterMoveTo to (moveIntro Nothing what (Just (zoneSort to))))
                 Nothing
                 ([])
  instrProfile (ExchangeLife parties) =
    sameIntro (nomIntro parties)
              ([outcomeB LifeGained, outcomeB LifeLost])
  instrProfile (ChangeLife who (LifeUp a)) = sameIntro (lifeIntro (LifeUp a)) [outcomeB LifeGained]
  instrProfile (ChangeLife who (LifeDown a)) = sameIntro (lifeIntro (LifeDown a)) [outcomeB LifeLost]
  instrProfile (ChangeLife who (Set a)) = sameIntro (lifeIntro (Set a)) []
  instrProfile (AddMana who amt _ _) = sameIntro (amtIntro amt) [outcomeB ManaAdded]
  instrProfile (Draw who amt) = sameIntro (amtIntro amt) []
  instrProfile (Expose v who what) = sameIntro (exposedIntro what) []
  instrProfile (Search who sc q p) =
    sameIntro (quantDelta q ++ predDelta p ++ searchDelta sc ++ nomIntro who)
              ([MkBinding AD Object (quantPlur q)
                          (ObjectP (seedTy p) (searchZone sc)
                                   (mkStamp (Just "Search") Nothing False) Nothing Nothing)])
  instrProfile (Shuffle whose) =
    MkInstrProfile (nomIntro whose) (afterShuffle (nomIntro whose)) Nothing ([])
  instrProfile (FlipCoins who count) = sameIntro (flipScopeIntro count) [outcomeB CoinFlipped]
  instrProfile (RollDice who count _) = sameIntro (amtIntro count) [outcomeB RollResult]
  instrProfile (ResultsTable rows) = sameIntro bs []
  instrProfile (IgnoreOutcomes which) = sameIntro (ignoredOutcomesIntro which) []
  instrProfile (ShiftResult _ amt) = sameIntro (amtIntro amt) []
  instrProfile (RollPlanarDie who count) = sameIntro (amtIntro count) [outcomeB PlanarRolled]
  instrProfile (ChaosEnsues Nothing) = sameIntro bs []
  instrProfile (ChaosEnsues (Just what)) = sameIntro (nomIntro what) []
  instrProfile (StoreResults on) = sameIntro (nomIntro on) []
  instrProfile (RerollStored _ _ whose) = sameIntro (nomIntro whose) []
  instrProfile (Continuously se _) = sameIntro (staticIntro se) []
  instrProfile (Create agent count spec riders) =
    sameIntro (specDelta spec ++ amtIntro count)
              ([MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
                          (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin) Nothing)])
  instrProfile (GetsEmblem who _) = sameIntro (nomIntro who) []
  instrProfile (PutCounters amt kind on) = sameIntro (nomIntro on) []
  instrProfile (RemoveCounters q kind from) = sameIntro (nomIntro from) [outcomeB CountersRemoved]
  instrProfile (MoveCounters amt kind src dst) = sameIntro (nomIntro dst) []
  instrProfile (DoubleCounters on) = sameIntro (nomIntro on) []
  instrProfile (Enact Nothing v (Move what to _)) =
    MkInstrProfile (nomIntro what)
                 (afterMoveTo to (moveIntro (Just v) what (Just (zoneSort to))))
                 (Just (stampIntro (Just v) what))
                 ([])
  instrProfile (Enact Nothing v (SetStatus _ n)) =
    MkInstrProfile (nomIntro n) (stampIntro (Just v) n) Nothing ([])
  instrProfile (Enact Nothing _ e) = instrProfile e
  instrProfile (Enact (Just s) v e) = doesProfile (nounPlur s) s v e
  instrProfile (Pay who c PaidOnce) =
    MkInstrProfile (nomIntro who) (costIntro c) Nothing ([])
  instrProfile (Pay who c AnyNumberOfTimes) =
    MkInstrProfile (nomIntro who) (costIntro c) Nothing ([outcomeB RepeatCount])
  instrProfile (Pay who c (UpToTimes _)) =
    MkInstrProfile (nomIntro who) (costIntro c) Nothing ([outcomeB RepeatCount])
  instrProfile (May d body did notd) = mayProfile body did notd
  instrProfile (IfDone body did notd) = mayProfile body did notd
  instrProfile (OnlyIf e c oth) = sameIntro (annIntro e) []
  instrProfile (If c e oth) = sameIntro bs []
  instrProfile (Define l amt) = sameIntro (defineLetter l (amtIntro amt)) []
  instrProfile (ForEachOf grp body) =
    MkInstrProfile bs
                 (pluralizeDelta (instrDelta body) ++
                    pluralizeDelta (nounDelta grp) ++ bs)
                 Nothing
                 ([])
  instrProfile (ForEachKindOf _ dom _ body) =
    MkInstrProfile bs
                 (pluralizeDelta (instrDelta body) ++
                    pluralizeDelta (maybe [] nounDelta dom) ++ bs)
                 Nothing
                 ([])
  instrProfile (Repeat _) = sameIntro bs []
  instrProfile (Repeated n body) =
    MkInstrProfile (amtIntro n)
                 (pluralizeDelta (instrDelta body) ++ amtIntro n)
                 Nothing
                 ([outcomeB RepeatCount])
  instrProfile (Sequentially es) = seqProfile es
  instrProfile (Simultaneously es) = simProfile es
  instrProfile (Modal q modes) = sameIntro (quantDelta q ++ bs) []
  instrProfile (Delayed ev _ _ e) = sameIntro bs []
  instrProfile (Reflexively body trig) = instrProfile body
  instrProfile (ThisWay body ev trig) = instrProfile body
  instrProfile (InsteadOf replaced repl) = sameIntro (annIntro replaced) []
  instrProfile (HeldUntil e ev) = sameIntro (annIntro e) []

  public export
  mayCtx : {bs : Bindings} -> Noun bs Player -> Bindings
  mayCtx d = agentIntro d

  public export
  ifDoneArmed : {0 bs : Bindings} -> (body : Instruction bs) ->
                Maybe (Instruction (instrIntro body)) -> Maybe (Instruction bs) -> Bool
  ifDoneArmed _ Nothing Nothing = False
  ifDoneArmed _ _ _ = True

  public export
  reflexCtx : {bs : Bindings} -> Instruction bs -> Bindings
  reflexCtx body = settleTargets (instrIntro body)

  public export
  thisWayCtx : {bs : Bindings} -> (body : Instruction bs) ->
               GameEvent (instrIntro body) -> Bindings
  thisWayCtx body ev = settleTargets (eventAfter ev)

  public export
  costRepeatedOk : {0 bs : Bindings} -> Instruction bs -> Bool
  costRepeatedOk (Sequentially es) = costStepsOk es
  costRepeatedOk e = costActionOk e

  public export
  costStepsOk : {0 bs : Bindings} -> {0 n : Nat} -> Instructions n bs -> Bool
  costStepsOk [] = True
  costStepsOk (e :: es) = costActionOk e && costStepsOk es

  public export
  deckAxis : ProjAxis -> Bool
  deckAxis (StatAxis ManaValue) = True
  deckAxis _ = False

  public export
  deckAxes : List ProjAxis -> Bool
  deckAxes [] = True
  deckAxes (a :: as) = deckAxis a && deckAxes as

  public export
  deckBound : Amount [] -> Bool
  deckBound (Lit _) = True
  deckBound _ = False

  ||| A card set aside for the starting deck is outside the game
  ||| [CR#103.2b], so only its characteristics are readable [CR#109.3]: a
  ||| zone is a place objects are during a game [CR#400.1], and status,
  ||| counters and controller are not characteristics.
  public export
  deckReadable : Predicate [] Object -> Bool
  deckReadable IsCard = True
  deckReadable Permanent = True
  deckReadable (HasType _) = True
  deckReadable (HasSubtype _) = True
  deckReadable (Compare axes _ bound) = deckAxes axes && deckBound bound
  deckReadable (And ps) = deckReadableAll ps
  deckReadable (Not p) = deckReadable p
  deckReadable _ = False

  public export
  deckReadableAll : List (Predicate [] Object) -> Bool
  deckReadableAll [] = True
  deckReadableAll (p :: ps) = deckReadable p && deckReadableAll ps

  ||| A characteristic read of a card in the starting deck [CR#109.3].
  public export
  data DeckReadable : Predicate [] Object -> Type where
    ReadsCharacteristics : {0 p : Predicate [] Object} ->
                           {auto 0 ok : deckReadable p = True} ->
                           DeckReadable p

  public export
  ||| The characteristics [CR#109.3] a deck condition compares; a number and
  ||| a counter kind are not characteristics.
  deckComparable : QualitySort -> Bool
  deckComparable Color = True
  deckComparable (SubtypeQ _) = True
  deckComparable CardName = True
  deckComparable CardTypeQ = True
  deckComparable Number = False
  deckComparable CounterKindQ = False

  ||| A characteristic a deck condition compares across the deck's cards
  ||| [CR#109.3].
  public export
  data DeckComparable : QualitySort -> Type where
    ComparesCharacteristic : {0 q : QualitySort} ->
                             {auto 0 ok : deckComparable q = True} ->
                             DeckComparable q

  public export
  data ManaParity = EvenValue | OddValue

  ||| One card's side of a deck condition: a characteristic read
  ||| [CR#109.3], or a reading a printed companion names that the
  ||| characteristic vocabulary does not carry.
  public export
  data DeckTrait : Type where
    ACharacteristic : (p : Predicate [] Object) ->
                      {auto 0 dr : DeckReadable p} -> DeckTrait
    ||| "cards with even mana values" [CR#202.3]
    ManaValueParity : (par : ManaParity) -> DeckTrait
    ||| "more than one of the same mana symbol in its mana cost"
    RepeatedManaSymbol : DeckTrait
    ||| "has an activated ability"
    HasAbilityOf : (cls : AbilityClass) -> DeckTrait
    ||| "... and land cards"
    AnyTraitOf : (ts : List DeckTrait) ->
                 {auto 0 ne : NonEmpty ts} -> DeckTrait

  ||| Companion's restriction, fulfilled by the deck left after sideboarding
  ||| and checked before the game begins [CR#702.139a,702.139b,103.2b].
  public export
  data DeckCondition : Type where
    ||| "Each permanent card in your starting deck has mana value 2 or less."
    EveryCardIs : (scope : Predicate [] Object) -> (trait : DeckTrait) ->
                  {auto 0 dr : DeckReadable scope} -> DeckCondition
    ||| "No card in your starting deck has more than one of the same mana
    ||| symbol in its mana cost."
    NoCardIs : (scope : Predicate [] Object) -> (trait : DeckTrait) ->
               {auto 0 dr : DeckReadable scope} -> DeckCondition
    ||| "Each nonland card in your starting deck has a different name."
    CardsDiffer : (scope : Predicate [] Object) -> (ax : QualitySort) ->
                  {auto 0 dr : DeckReadable scope} ->
                  {auto 0 dc : DeckComparable ax} -> DeckCondition
    ||| "Each nonland card in your starting deck shares a card type."
    CardsShare : (scope : Predicate [] Object) -> (ax : QualitySort) ->
                 {auto 0 dr : DeckReadable scope} ->
                 {auto 0 dc : DeckComparable ax} -> DeckCondition
    ||| "at least twenty cards more than the minimum deck size", a minimum
    ||| the format sets [CR#100.2a,100.2b].
    DeckSizeOverMinimum : (extra : Nat) -> DeckCondition

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
    ParamDeckCondition : (dc : DeckCondition) -> KeywordParam bs

  public export
  paramShapeOf : {0 bs : Bindings} -> Maybe (KeywordParam bs) -> KeywordParamShape
  paramShapeOf Nothing = NoParam
  paramShapeOf (Just (ParamCost _)) = CostParam
  paramShapeOf (Just (ParamQuality _)) = QualityParam
  paramShapeOf (Just (ParamSubject _)) = SubjectParam
  paramShapeOf (Just (ParamNumber _)) = NumberParam
  paramShapeOf (Just (ParamQualityCost _ _)) = CompoundParam QualityHead
  paramShapeOf (Just (ParamNumberCost _ _)) = CompoundParam NumberHead
  paramShapeOf (Just (ParamDeckCondition _)) = DeckConditionParam

  public export
  keywordParamFits : {0 bs : Bindings} -> KeywordLabel -> Maybe (KeywordParam bs) -> Bool
  keywordParamFits k p =
    knownKeyword k && paramShapesFit (keywordParamShapes k) (paramShapeOf p)

  public export
  KeywordParamFits : KeywordLabel -> Maybe (KeywordParam bs) -> Type
  KeywordParamFits {bs} k p = So (keywordParamFits k p)

  public export
  data AbilityLost : Bindings -> Type where
    LostWritten : (ab : AbilityAt bs) ->
                  {auto 0 hd : So (grantableAb ab)} -> AbilityLost bs
    LostTerm : (t : KeywordTerm) ->
               {auto 0 kn : So (knownKeywordTerm t)} -> AbilityLost bs

  public export
  data AbilityAt : Bindings -> Type where
    KeywordAbility : (k : KeywordLabel) ->
                     (param : Maybe (KeywordParam bs)) ->
                     (body : Maybe (AbilityAt [])) ->
                     {auto 0 pf : KeywordParamFits k param} ->
                     {auto 0 bf : KeywordBodyFits k body} -> AbilityAt bs
    Activated : (cost : Cost (dropLetter X bs)) ->
                (instr : Instruction (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                (window : Maybe (Timing bs)) ->
                (limit : Maybe UsageLimit) ->
                {auto 0 ul : So (untriggeredLimitOk limit)} ->
                (guard : Maybe (Condition bs)) ->
                (activator : Maybe (Noun bs Player)) ->
                AbilityAt bs
    Triggered : (word : TriggerWord) -> (ev : GameEvent bs) ->
                (alts : List (GameEvent bs)) ->
                (while : Maybe (Concurrent (headerCtx alts ev))) ->
                (joins : List (JoinedHeader bs)) ->
                (window : Maybe (TriggerWindow bs)) ->
                (limit : Maybe UsageLimit) ->
                (intervening :
                   Maybe (Condition (joinedCtx joins (headerCtx alts ev)))) ->
                (instr : Instruction (interveningIntro intervening)) ->
                {auto 0 hn : HeaderNontarget ev} ->
                {auto 0 hs : HeaderStatus ev} ->
                {auto 0 ae : AltEvent word alts} ->
                {auto 0 cd :
                   ChapterDefaults ev alts while joins window limit intervening} ->
                AbilityAt bs
    Static : (se : StaticSpec bs) ->
             {auto 0 ut : Untargeting se} -> AbilityAt bs
    Spell : (window : Maybe (Timing bs)) -> (instr : Instruction bs) ->
            AbilityAt bs
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
  bodyEventRegime : {0 bs : Bindings} -> GameEvent bs -> Maybe StackRegime
  bodyEventRegime ev = case eventName ev of
    SpellCast => Just AtCasting
    _ => Nothing

  public export
  keywordBodyFits : KeywordLabel -> Maybe (AbilityAt []) -> Bool
  keywordBodyFits k Nothing = True
  keywordBodyFits k (Just (Triggered _ ev _ _ _ _ _ _ _)) =
    keywordBodied k && keywordStackRegime k == bodyEventRegime ev
  keywordBodyFits k (Just _) = False

  public export
  KeywordBodyFits : KeywordLabel -> Maybe (AbilityAt []) -> Type
  KeywordBodyFits k b = So (keywordBodyFits k b)

  public export
  Untargeting : {bs : Bindings} -> StaticSpec bs -> Type
  Untargeting {bs} se = So (not (anyTargetedAt (staticIntro se)))

  public export
  effectNamesThisDoor : {0 bs : Bindings} -> Instruction bs -> Bool
  effectNamesThisDoor (Delayed ev alts _ _) =
    eventNamesThisDoor ev || anyEventNamesThisDoor alts
  effectNamesThisDoor (HeldUntil _ ev) = eventNamesThisDoor ev
  effectNamesThisDoor (ThisWay _ ev _) = eventNamesThisDoor ev
  effectNamesThisDoor _ = False

  public export
  abilityNamesThisDoor : {0 bs : Bindings} -> AbilityAt bs -> Bool
  abilityNamesThisDoor (Activated _ instr _ _ _ _) = effectNamesThisDoor instr
  abilityNamesThisDoor (Triggered _ ev alts while joins _ _ _ instr) =
    eventNamesThisDoor ev || anyEventNamesThisDoor alts ||
      concurrentNamesThisDoor while || joinsNameThisDoor joins ||
      effectNamesThisDoor instr
  abilityNamesThisDoor (Spell _ instr) = effectNamesThisDoor instr
  abilityNamesThisDoor (ItalicHead _ ab) = abilityNamesThisDoor ab
  abilityNamesThisDoor (AlsoForKeywords ab _) = abilityNamesThisDoor ab
  abilityNamesThisDoor _ = False

  public export
  lineKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  lineKeyword (Static se) = statKeyword se
  lineKeyword (Triggered _ _ _ _ _ _ _ _ instr) = instrKeyword instr
  lineKeyword _ = Nothing

  public export
  statKeyword : {0 bs : Bindings} -> StaticSpec bs -> Maybe KeywordLabel
  statKeyword (Conditionally se _ _) = statKeyword se
  statKeyword (Gains _ ab) = grantedKeyword ab
  statKeyword _ = Nothing

  public export
  instrKeyword : {0 bs : Bindings} -> Instruction bs -> Maybe KeywordLabel
  instrKeyword (Continuously se _) = statKeyword se
  instrKeyword _ = Nothing

  public export
  grantedKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe KeywordLabel
  grantedKeyword (KeywordAbility k Nothing _) =
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
  grantableAb (KeywordAbility _ _ _) = True
  grantableAb (Activated _ _ _ _ _ _) = True
  grantableAb (Triggered _ _ _ _ _ _ _ _ _) = True
  grantableAb (Static _) = True
  grantableAb (Spell _ _) = False
  grantableAb MayBeginOnBattlefield = False
  grantableAb (AlsoForKeywords _ _) = False
  grantableAb (ItalicHead _ ab) = grantableAb ab

  public export
  Grantable : AbilityAt bs -> Type
  Grantable {bs} ab = So (grantableAb ab)

  public export
  emblemAbilityOk : AbilityAt [] -> Bool
  emblemAbilityOk (KeywordAbility _ _ _) = False
  emblemAbilityOk (Activated _ _ _ _ _ _) = True
  emblemAbilityOk (Triggered _ _ _ _ _ _ _ _ _) = True
  emblemAbilityOk (Static _) = True
  emblemAbilityOk (AlsoForKeywords _ _) = False
  emblemAbilityOk (ItalicHead _ ab) = emblemAbilityOk ab
  emblemAbilityOk (Spell _ _) = False
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
  predRegime : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> Maybe StackRegime
  predRegime (CastBy _ _) = Just AtCasting
  predRegime (CastFrom _) = Just AtCasting
  predRegime WasCast = Just AtCasting
  predRegime (HasPossessor ControllerAx _) = Just AtResolution
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
  nounRegime (Described _ p) = predRegime p
  nounRegime (NamesAgree _ grp) = nounRegime grp
  nounRegime _ = Nothing

  public export
  abRegime : {0 bs : Bindings} -> AbilityAt bs -> Maybe StackRegime
  abRegime (KeywordAbility k _ _) = keywordStackRegime k
  abRegime (Activated _ _ _ _ _ _) = Nothing
  abRegime (Triggered _ _ _ _ _ _ _ _ _) = Nothing
  abRegime (Static _) = Nothing
  abRegime (AlsoForKeywords ab _) = abRegime ab
  abRegime (ItalicHead _ ab) = abRegime ab
  abRegime (Spell _ _) = Nothing
  abRegime MayBeginOnBattlefield = Nothing

  public export
  abFunctionsOnStack : {0 bs : Bindings} -> AbilityAt bs -> Bool
  abFunctionsOnStack (KeywordAbility k _ _) = keywordFunctionsOnStack k
  abFunctionsOnStack (Activated _ _ _ _ _ _) = False
  abFunctionsOnStack (Triggered _ _ _ _ _ _ _ _ _) = False
  abFunctionsOnStack (Static _) = False
  abFunctionsOnStack (AlsoForKeywords ab _) = abFunctionsOnStack ab
  abFunctionsOnStack (ItalicHead _ ab) = abFunctionsOnStack ab
  abFunctionsOnStack (Spell _ _) = False
  abFunctionsOnStack MayBeginOnBattlefield = False

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
      else not (abFunctionsOnStack ab)

  public export
  GrantSubject : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Type
  GrantSubject {bs} ab n = So (grantSubjectOk ab n)

  public export
  instrChoiceDelta : {0 bs : Bindings} -> Instruction bs -> List Binding
  instrChoiceDelta (Choose {k} _ _ (Described (ADet _) _) _) = choiceDeltaAt k
  instrChoiceDelta (Choose _ _ _ _) = []
  instrChoiceDelta (Sequentially es) = instrsChoiceDelta es
  instrChoiceDelta (May _ body _ _) = instrChoiceDelta body
  instrChoiceDelta (IfDone body _ _) = instrChoiceDelta body
  instrChoiceDelta _ = []

  public export
  instrsChoiceDelta : {0 n : Nat} -> {0 bs : Bindings} ->
                    Instructions n bs -> List Binding
  instrsChoiceDelta [] = []
  instrsChoiceDelta (e :: es) = instrsChoiceDelta es ++ instrChoiceDelta e

  public export
  abIntro : {bs : Bindings} -> AbilityAt bs -> Bindings
  abIntro (KeywordAbility _ _ _) = bs
  abIntro (Activated _ instr _ _ _ _) = instrChoiceDelta instr ++ bs
  abIntro (Triggered _ _ _ _ _ _ _ _ instr) = instrChoiceDelta instr ++ bs
  abIntro (Static se) = staticChoiceIntro se
  abIntro (AlsoForKeywords ab _) = abIntro ab
  abIntro (ItalicHead _ ab) = abIntro ab
  abIntro (Spell _ instr) = instrChoiceDelta instr ++ bs
  abIntro MayBeginOnBattlefield = bs

  public export
  abLetterDelta : {0 bs : Bindings} -> AbilityAt bs -> List Binding
  abLetterDelta (KeywordAbility _ (Just (ParamNumber (LetterVal l))) _) = [letterB l]
  abLetterDelta _ = []


  namespace Coord
    public export
    data StaticParts : Nat -> Bindings -> Type where
      Nil : StaticParts Z bs
      (::) : (se : StaticSpec bs) -> {auto 0 nc : NotCoord se} ->
             StaticParts n (staticIntro se) -> StaticParts (S n) bs

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
  costChoiceDelta (Do e) = instrChoiceDelta e
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
  clauseStaticOk : {0 bs : Bindings} -> StaticSpec bs -> Bool
  clauseStaticOk (DefinesPt _ _ _) = False
  clauseStaticOk (AltCost _ _) = False
  clauseStaticOk (AndAlso _ parts) = partsClauseOk parts
  clauseStaticOk _ = True

  public export
  ClauseStatic : StaticSpec bs -> Type
  ClauseStatic {bs} se = So (clauseStaticOk se)

  ||| How many times a cost turns the source itself over [CR#107.5].
  public export
  selfTapUses : {0 bs : Bindings} -> Cost bs -> Nat
  selfTapUses (Mana _) = Z
  selfTapUses (ScaledCost c _) = selfTapUses c
  selfTapUses TapSymbol = S Z
  selfTapUses UntapSymbol = S Z
  selfTapUses (LoyaltySymbol _) = Z
  selfTapUses (Do _) = Z
  selfTapUses (Compound cs) = selfTapCount cs
  selfTapUses (EitherCost l r) = max (selfTapUses l) (selfTapUses r)
  selfTapUses ItsManaCost = Z

  public export
  selfTapCount : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Nat
  selfTapCount [] = Z
  selfTapCount (c :: cs) = selfTapUses c + selfTapCount cs

  public export
  selfTapOnce : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  selfTapOnce cs = lte (selfTapCount cs) 1

  public export
  costTapOnce : {0 bs : Bindings} -> Cost bs -> Bool
  costTapOnce (Mana _) = True
  costTapOnce (ScaledCost c _) = costTapOnce c
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
  costPaidByYou (ScaledCost c _) = costPaidByYou c
  costPaidByYou TapSymbol = True
  costPaidByYou UntapSymbol = True
  costPaidByYou (LoyaltySymbol _) = True
  -- A life payment targets its payer; granting life may target anyone [CR#119.4].
  costPaidByYou (Do (ChangeLife who (LifeDown _))) = nounIsYou who
  costPaidByYou (Do (ChangeLife _ (LifeUp _))) = True
  costPaidByYou (Do (Enact (Just subj) _ _)) = nounIsYou subj
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
  costOffBattlefield (ScaledCost c _) = costOffBattlefield c
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
  AltPayment : {0 bs : Bindings} -> Maybe (Cost bs) -> Type
  AltPayment = OptOk (\c => So (costOffBattlefield c))

  public export
  data AddedPayment : {0 bs : Bindings} -> Cost bs -> Type where
    AddedPaymentWritten : {0 c : Cost bs} ->
                          {auto 0 ok : So (costOffBattlefield c)} ->
                          AddedPayment c

public export
Ability : Type
Ability = AbilityAt []