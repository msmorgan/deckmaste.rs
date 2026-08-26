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
    TokenAsThose : {auto 0 ok : countTokenSpecs bs = 1} -> TokenSpec bs
    TokenCopyOf : (src : Noun bs Object) -> (exc : List (CopyExcept bs)) ->
                  {auto 0 pm : PerMember src} -> TokenSpec bs

  public export
  specHeadTy : {bs : Bindings} -> TokenSpec bs -> Maybe CardType
  specHeadTy (TokenWritten t) = tokenHeadTy t
  specHeadTy TokenAsThose = tyOfThose TokenW bs
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
      Gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
             (tou : PtShift (shiftIntro pow)) ->
             {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
             StaticEffect bs
      DefinesPt : (n : Noun bs Object) -> (sl : DefinedSlots) ->
                  (amt : Amount (nomIntro n)) ->
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
      BecomesAlso : (n : Noun bs Object) -> (added : TokenChars bs) ->
                    {auto 0 ne : LineNonEmpty added.line} ->
                    {auto 0 nw : AddsSomething (nounTy n) added.line} ->
                    {auto 0 af : AddedFits (nounTy n) added.line} ->
                    {auto 0 cd : ColorsDistinct added.colors} ->
                    {auto 0 ta : TokenAbilities added} ->
                    {auto 0 un : AdditionUnnamed added} -> StaticEffect bs
      AddsEveryType : (n : Noun bs Object) -> (space : TypeSpace) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      {auto 0 sh : SpaceHosted space (nounTy n)} ->
                      StaticEffect bs
      SetsType : (n : Noun bs Object) -> (t : TokenChars bs) ->
                 (ret : Maybe CardType) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 {auto 0 ne : LineNonEmpty t.line} ->
                 {auto 0 af : AddedFits (nounTy n) t.line} ->
                 {auto 0 ta : TokenAbilities t} ->
                 {auto 0 tc : TokenCanonical t} ->
                 {auto 0 ro : RetentionOk t.line ret} -> StaticEffect bs
      SetsChosenBasicType : (n : Noun bs Object) ->
                            {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                            {auto 0 ls : LandSubject n} -> StaticEffect bs
      AddsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                       StaticEffect bs
      SetsChosenQuality : (n : Noun bs Object) -> (q : Predicate bs Object) ->
                       {auto 0 qr : QualityRead q} ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
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
      Intercepts : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                   (use : ReplUse) ->
                   {auto 0 ok : Interceptable ev} -> StaticEffect bs
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
      EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                           (kind : CounterKind) ->
                           (mark : EntryCounterMark) ->
                           StaticEffect bs
      EntersChoice : (n : Noun bs Object) -> (q : QualitySort) ->
                     (dom : Maybe (ChoiceDomain q)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     StaticEffect bs
      AndAlso : {0 n : Nat} -> StaticParts n bs ->
                {auto 0 ne : IsSucc n} -> StaticEffect bs

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
  staticKind (BecomesCopy _ _ _) = CopyEffect
  staticKind (SetsType _ _ _) = TypeSet
  staticKind (SetsChosenBasicType _) = TypeSet
  staticKind (AddsChosenQuality _ _) = TypeAddition
  staticKind (SetsChosenQuality _ _) = TypeSet
  staticKind (LosesAllAbilities _) = AbilityLoss
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _) = Replacement
  staticKind (Prevents _ _ _ _ _) = Prevention
  staticKind (PreventsFrom _ _ _ _ _ _) = Prevention
  staticKind (CantPrevent _ _ _) = Prevention
  staticKind (Redirects _ _ _ _ _) = Replacement
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
  staticKind (AndAlso _) = Coordination


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
  staticIntro (BecomesCopy n _ _) = selfSubjIntro n
  staticIntro (SetsType n _ _) = selfSubjIntro n
  staticIntro (SetsChosenBasicType n) = selfSubjIntro n
  staticIntro (AddsChosenQuality n _) = selfSubjIntro n
  staticIntro (SetsChosenQuality n _) = selfSubjIntro n
  staticIntro (LosesAllAbilities n) = selfSubjIntro n
  staticIntro (GainsControl who what) = selfSubjIntro what
  staticIntro (Intercepts ev repl use) = eventIntro ev
  staticIntro (Prevents kind size scope by also) = byIntro by
  staticIntro (PreventsFrom kind src scope cut use also) = cutIntro cut
  staticIntro (CantPrevent kind scope by) = byIntro by
  staticIntro (Redirects kind size scope by to) = nomIntro to
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
  staticIntro (AndAlso parts) = partsIntro parts

  public export
  staticChoiceIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticChoiceIntro (EntersChoice _ q _) = qualityB q :: bs
  staticChoiceIntro _ = bs

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
                    {auto 0 cq : countQuality Color bs = 1} ->
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
             (what : Noun (preIntro e) k) ->
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
    CounterSpell : (what : Noun bs Object) ->
                   {auto 0 zn : OnStack (nounZone what)} -> Effect bs
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
    Search : (who : Noun bs Player) -> (sc : SearchScope (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
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
    ||| -- spelling: "[who] flip[s] [count] coin(s)"; with `You` in the
    ||| subject slot, the imperative "Flip a coin."
    FlipCoins : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
                Effect bs
    ||| "Roll a d20", "Roll two six-sided dice", "roll that many dice":
    ||| [CR#706.1]'s instruction, which "will specify what kind of die to
    ||| roll and how many of those dice to roll" — so both are written
    ||| arguments and neither has a default. [CR#706.1a] fixes what the
    ||| kind is: N equally likely outcomes numbered from 1 to N, N a
    ||| positive integer, spelled either "dN" or "N-sided". The gate is
    ||| that positivity and no more; a nought-sided die has no outcome to
    ||| land on, and every other N the rule allows.
    ||| It introduces the roll's number, which "the result" reads
    ||| [CR#706.2] and a results table ranges over [CR#706.3a].
    ||| -- spelling: "[who] roll[s] [count] d[sides]"
    RollDice : (who : Noun bs Player) -> (count : Amount (nomIntro who)) ->
               (sides : Nat) -> {auto 0 nz : IsSucc sides} -> Effect bs
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
    ||| roll": the instruction that sets aside some of the rolls the same
    ||| clause has just made. [CR#706.6] gives it its meaning outright --
    ||| an ignored roll "is considered to have never happened", no
    ||| ability triggers because of it and no effect applies to it -- and
    ||| settles the tie the superlative can leave. Its own clause and not
    ||| a slot on `RollDice`, on `ResultsTable`'s ground: the roll and
    ||| what is done with its results are one ability [CR#706.3b] without
    ||| being one node, and a roll may carry no ignore at all.
    ||| It leaves the roll's mention standing rather than reminting it:
    ||| the rolls that survive are the same roll the clause named, which
    ||| is what a following `ResultsTable` or `TheResult` reads.
    ||| The printed lines that write the instruction on the REPLACEMENT
    ||| side ("If you would roll one or more dice, instead roll that many
    ||| dice plus one and ignore the lowest roll") need more than this
    ||| row: the replaced roll announces neither its die count nor its
    ||| kind, so "that many dice" has no antecedent.
    ||| -- spelling: "ignore the lowest/lower roll"; with `IgnoreAllBut`,
    ||| "ignore all but the highest roll".
    IgnoreRolls : (which : IgnoredRolls) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} -> Effect bs
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
    ||| The special action [CR#116.2i,901.9] is not this row. That one is
    ||| granted to a player by the Planechase rules and is written on no
    ||| card; this is the instruction a card's own ability gives.
    ||| -- spelling: "[who] roll[s] the planar die".
    RollPlanarDie : (who : Noun bs Player) -> Effect bs
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
    PutCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 sc : counterScope kind = Object} -> Effect bs
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
    ||| -- spelling: the keyword action's own verb, imperative.
    Enact : (v : VerbLabel) -> (e : Effect bs) ->
            {auto 0 kn : KnownVerb v} -> Effect bs
    ||| `Enact`'s agentive surface: the same labeled action with the
    ||| player performing it written as its subject.
    ||| -- spelling: "[subj] [verb]s [body]".
    Does : (subj : Noun bs Player) -> (v : VerbLabel) ->
           (e : Effect (nomIntro subj)) ->
           {auto 0 kn : KnownVerb v} -> Effect bs
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
    ||| -- spelling: "[e] if [c]"; under `NotCond`, "[e] unless [c]".
    OnlyIf : (e : Effect bs) -> (c : Condition (preIntro e)) ->
             (otherwise : Maybe (Effect (otherwiseCtx e))) -> Effect bs
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
              (span : Maybe (Duration bs)) ->
              Effect (delayedCtx ev) ->
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
  heldUntilOk (Search _ _ _) = False
  heldUntilOk (Shuffle _) = False
  heldUntilOk (FlipCoins _ _) = False
  heldUntilOk (RollDice _ _ _) = False
  heldUntilOk (ResultsTable _) = False
  heldUntilOk (IgnoreRolls _) = False
  heldUntilOk (ShiftResult _) = False
  heldUntilOk (RollPlanarDie _) = False
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
  heldUntilOk (Repeat _) = False
  heldUntilOk (Repeated _ _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _ _) = False
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
  reflexEncloseUse (Move _ _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _ _) = EncReflexive       -- 1
  reflexEncloseUse (Search _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  reflexEncloseUse (FlipCoins _ _) = EncReflexive
  reflexEncloseUse (RollDice _ _ _) = EncReflexive
  reflexEncloseUse (ResultsTable _) = EncNotOneAction
  -- no PLAYER is written taking either action: the ignore and the
  -- shift are stated of the roll the clause made.
  reflexEncloseUse (IgnoreRolls _) = EncAgentless
  reflexEncloseUse (ShiftResult _) = EncAgentless
  reflexEncloseUse (RollPlanarDie _) = EncReflexive
  reflexEncloseUse (StoreResults _) = EncAgentless
  reflexEncloseUse (RerollStored _ _ _) = EncReflexive
  -- [CR#603.12] writes the reflexive over what a player did or didn't
  -- do, so a declined arm leaves one offered action to inflect.
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  reflexEncloseUse (OnlyIf _ _ _) = EncNotOneAction
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (Unless _ _ _) = EncNotOneAction
  reflexEncloseUse (Define _ _) = EncAgentless
  reflexEncloseUse (ForEachOf _ _) = EncNotOneAction
  reflexEncloseUse (Repeat _) = EncNotOneAction
  reflexEncloseUse (Repeated _ _) = EncNotOneAction
  reflexEncloseUse (Sequentially _) = EncNotOneAction
  reflexEncloseUse (Simultaneously _) = EncNotOneAction
  reflexEncloseUse (Modal _ _) = EncNotOneAction
  reflexEncloseUse (InsteadOf _ _) = EncNotOneAction
  reflexEncloseUse (Reflexively _ _) = EncNotOneAction
  reflexEncloseUse (ThisWay _ _ _) = EncNotOneAction
  reflexEncloseUse (Delayed _ _ _) = EncNotYetTaken
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
  thisWayOutcomeOk (Delayed _ _ _) = False
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
  thisWayOutcomeOk (Search _ _ _) = True
  thisWayOutcomeOk (Shuffle _) = True
  thisWayOutcomeOk (FlipCoins _ _) = True
  thisWayOutcomeOk (RollDice _ _ _) = True
  thisWayOutcomeOk (ResultsTable _) = True
  thisWayOutcomeOk (IgnoreRolls _) = True
  thisWayOutcomeOk (ShiftResult _) = True
  thisWayOutcomeOk (RollPlanarDie _) = True
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
  thisWayOutcomeOk (Enact _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (Pay _ _) = True
  thisWayOutcomeOk (OnlyIf _ _ _) = True
  thisWayOutcomeOk (If _ _ _) = True
  thisWayOutcomeOk (Unless _ _ _) = True
  thisWayOutcomeOk (Define _ _) = True
  thisWayOutcomeOk (ForEachOf _ _) = True
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
  costActionOk (Search who _ _) = costNounOk who
  costActionOk (Shuffle whose) = costNounOk whose
  costActionOk (FlipCoins who _) = costNounOk who
  costActionOk (RollDice who _ _) = costNounOk who
  costActionOk (ResultsTable _) = False
  -- each reads the roll a clause before it made, which no cost has.
  costActionOk (IgnoreRolls _) = False
  costActionOk (ShiftResult _) = False
  costActionOk (StoreResults _) = False
  costActionOk (RollPlanarDie who) = costNounOk who
  costActionOk (RerollStored who _ _) = costNounOk who
  costActionOk (Continuously _ _) = False
  costActionOk (Throughout _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  costActionOk (PutSameCounters src dst) = costNounOk src && costNounOk dst
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
  costActionOk (Repeat _) = False
  costActionOk (Repeated _ body) = costRepeatedOk body
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ modes) = costActionsOk modes
  costActionOk (Delayed _ _ _) = False
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
  effEq (CounterSpell a) (CounterSpell b) = nounEqRef a b
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
  effEq (Search _ _ _) _ = False
  effEq (Shuffle _) _ = False
  effEq (FlipCoins _ _) _ = False
  effEq (RollDice _ _ _) _ = False
  effEq (ResultsTable _) _ = False
  effEq (IgnoreRolls _) _ = False
  effEq (ShiftResult _) _ = False
  effEq (RollPlanarDie _) _ = False
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
  effEq (Repeat _) _ = False
  effEq (Repeated _ _) _ = False
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _ _) _ = False
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
  effIntro (Move what to _) = moveIntro Nothing what (Just (zoneSort to))
  effIntro (ChangeLife who (Up a)) = outcomeB LifeGained :: lifeIntro (Up a)
  effIntro (ChangeLife who (Down a)) = outcomeB LifeLost :: lifeIntro (Down a)
  effIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  effIntro (AddMana who amt _ _) = amtIntro amt
  effIntro (Draw who amt) = amtIntro amt
  effIntro (Expose v who what) = exposedIntro what
  effIntro (Search who sc p) =
    MkBinding AD Object OneOf (ObjectP (seedTy p) (searchZone sc) Nothing Nothing)
      :: (predDelta p ++ searchDelta sc ++ nomIntro who)
  effIntro (Shuffle whose) = nomIntro whose
  effIntro (FlipCoins who count) = outcomeB CoinFlipped :: amtIntro count
  effIntro (RollDice who count _) = outcomeB RollResult :: amtIntro count
  effIntro (ResultsTable rows) = bs
  effIntro (IgnoreRolls _) = bs
  effIntro (ShiftResult amt) = amtIntro amt
  effIntro (RollPlanarDie who) = nomIntro who
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
  effIntro (Enact v (Move what to _)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Enact v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Enact _ e) = effIntro e
  effIntro (Does s v (Move what to _)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Does s v (SetStatus _ n)) = stampIntro (Just v) n
  effIntro (Does s v e) = effIntro e
  effIntro (Pay who c) = costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- a conditioned clause exports nothing: the condition may have failed.
  effIntro (OnlyIf e c oth) = bs
  effIntro (If c e oth) = bs
  effIntro (Unless e who c) = bs
  effIntro (Define l amt) = defineLetter l (amtIntro amt)
  effIntro (ForEachOf _ _) = bs
  effIntro (Repeat _) = bs
  effIntro (Repeated n body) =
    outcomeB RepeatCount :: (pluralizeDelta (effDelta body) ++ amtIntro n)
  effIntro (Sequentially es) = effsIntro es
  effIntro (Simultaneously es) = simIntro es
  effIntro (Modal q modes) = quantDelta q ++ bs
  effIntro (Delayed ev _ e) = bs               -- a future clause mentions nothing NOW
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
  preIntro (Search who sc p) = predDelta p ++ searchDelta sc ++ nomIntro who
  preIntro (Shuffle whose) = nomIntro whose
  preIntro (FlipCoins who count) = amtIntro count
  preIntro (RollDice who count _) = amtIntro count
  preIntro (ResultsTable rows) = bs
  preIntro (IgnoreRolls _) = bs
  preIntro (ShiftResult amt) = amtIntro amt
  preIntro (RollPlanarDie who) = nomIntro who
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
  preIntro (Repeat _) = bs
  preIntro (Repeated n _) = amtIntro n
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = quantDelta q ++ bs
  preIntro (Delayed ev _ e) = bs
  preIntro (Reflexively body trig) = preIntro body
  preIntro (ThisWay body ev trig) = preIntro body
  preIntro (InsteadOf replaced repl) = annIntro replaced
  preIntro (HeldUntil e ev) = annIntro e

  ||| What a clause's phrases have named by the time the spell is cast
  ||| [CR#601.2c] — the announcement channel, distinct from `effIntro`
  ||| (what the clause did) and `preIntro` (its flat-clause approximation).
  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
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
  annIntro (Search who sc p) = predDelta p ++ searchDelta sc ++ nomIntro who
  annIntro (Shuffle whose) = nomIntro whose
  annIntro (FlipCoins who count) = amtIntro count
  annIntro (RollDice who count _) = amtIntro count
  annIntro (ResultsTable rows) = bs
  annIntro (IgnoreRolls _) = bs
  annIntro (ShiftResult amt) = amtIntro amt
  annIntro (RollPlanarDie who) = nomIntro who
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
  annIntro (Repeat _) = bs
  annIntro (Repeated n _) = amtIntro n
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = quantDelta q ++ bs
  annIntro (Delayed ev _ e) = bs
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
  deedDelta (Search who sc p) =
    [MkBinding AD Object OneOf (ObjectP (seedTy p) (searchZone sc) Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (FlipCoins _ _) = [outcomeB CoinFlipped]
  deedDelta (RollDice _ _ _) = [outcomeB RollResult]
  deedDelta (ResultsTable _) = []
  -- an ignore takes rolls away and a shift restates one; neither
  -- leaves a number the roll had not already left.
  deedDelta (IgnoreRolls _) = []
  deedDelta (ShiftResult _) = []
  -- [CR#706.7]: no numerical result to announce.
  deedDelta (RollPlanarDie _) = []
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
  deedDelta (Repeat _) = []
  deedDelta (Repeated _ _) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev _ e) = []
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
  keywordParamFits : {0 bs : Bindings} -> Keyword -> Maybe (KeywordParam bs) -> Bool
  keywordParamFits k p = keywordParamShape k == paramShapeOf p

  public export
  KeywordParamFits : Keyword -> Maybe (KeywordParam bs) -> Type
  KeywordParamFits {bs} k p = So (keywordParamFits k p)

  public export
  data AbilityAt : Bindings -> Type where
    KeywordAbility : (k : Keyword) ->
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
    Triggered : (word : TriggerWord) -> (ev : GameEvent bs) ->
                (alt : Maybe (GameEvent bs)) ->
                (window : Maybe TriggerWindow) ->
                (limit : Maybe UsageLimit) ->
                (intervening : Maybe (Condition (headerCtx alt ev))) ->
                (eff : Effect (interveningIntro intervening)) ->
                {auto 0 hn : HeaderNontarget ev} ->
                {auto 0 ae : AltEvent word alt} ->
                {auto 0 cd : ChapterDefaults ev alt window limit intervening} ->
                AbilityAt bs
    Static : (se : StaticEffect bs) ->
             {auto 0 ut : Untargeting se} -> AbilityAt bs
    Spell : (eff : Effect bs) -> AbilityAt bs
    AlsoForKeywords : (ab : AbilityAt bs) -> (ks : List Keyword) ->
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
  lineKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe Keyword
  lineKeyword (Static se) = statKeyword se
  lineKeyword (Triggered _ _ _ _ _ _ eff) = effKeyword eff
  lineKeyword _ = Nothing

  public export
  statKeyword : {0 bs : Bindings} -> StaticEffect bs -> Maybe Keyword
  statKeyword (Conditionally _ se _) = statKeyword se
  statKeyword (OnlyWhile se _ _) = statKeyword se
  statKeyword (Gains _ ab) = grantedKeyword ab
  statKeyword _ = Nothing

  public export
  effKeyword : {0 bs : Bindings} -> Effect bs -> Maybe Keyword
  effKeyword (Continuously se _) = statKeyword se
  effKeyword (Throughout _ se) = statKeyword se
  effKeyword _ = Nothing

  public export
  grantedKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe Keyword
  grantedKeyword (KeywordAbility k Nothing) =
    if keywordParamless k then Just k else Nothing
  grantedKeyword _ = Nothing

  public export
  keywordExtendableOk : {0 bs : Bindings} -> AbilityAt bs -> Bool
  keywordExtendableOk ab = case lineKeyword ab of
    Nothing => False
    Just _ => True

  public export
  keywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List Keyword -> Bool
  keywordListOk ab ks = case lineKeyword ab of
    Nothing => False
    Just base => not (isNil ks) && allParamless ks && distinctKeywords ks &&
                 not (elem base ks)


  public export
  allParamless : List Keyword -> Bool
  allParamless [] = True
  allParamless (k :: ks) = keywordParamless k && allParamless ks

  public export
  distinctKeywords : List Keyword -> Bool
  distinctKeywords [] = True
  distinctKeywords (k :: ks) = not (elem k ks) && distinctKeywords ks

  public export
  KeywordExtendable : {0 bs : Bindings} -> AbilityAt bs -> Type
  KeywordExtendable {bs} ab = So (keywordExtendableOk ab)

  public export
  KeywordListOk : {0 bs : Bindings} -> AbilityAt bs -> List Keyword -> Type
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
  grantableAb (Triggered _ _ _ _ _ _ _) = True
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
  emblemAbilityOk (Triggered _ _ _ _ _ _ _) = True
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
  nounRegime (CountedGroup _ p) = predRegime p
  nounRegime (NamesAgree _ grp) = nounRegime grp
  nounRegime _ = Nothing

  public export
  abRegime : {0 bs : Bindings} -> AbilityAt bs -> Maybe StackRegime
  abRegime (KeywordAbility k _) = keywordStackRegime k
  abRegime (Activated _ _ _ _ _) = Nothing
  abRegime (Triggered _ _ _ _ _ _ _) = Nothing
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
  grantSubjectOk ab n =
    if onStackZone (nounZone n)
      then regimeMatches (abRegime ab) (nounRegime n)
      else not (castingOnly (abRegime ab))

  public export
  GrantSubject : {bs : Bindings} -> AbilityAt bs -> Noun bs Object -> Type
  GrantSubject {bs} ab n = So (grantSubjectOk ab n)

  public export
  abIntro : {bs : Bindings} -> AbilityAt bs -> Bindings
  abIntro (KeywordAbility _ _) = bs
  abIntro (Activated _ _ _ _ _) = bs
  abIntro (Triggered _ _ _ _ _ _ _) = bs
  abIntro (Static se) = staticChoiceIntro se
  abIntro (AlsoForKeywords ab _) = abIntro ab
  abIntro (AbilityWord _ ab) = abIntro ab
  abIntro (Spell _) = bs

  namespace Coord
    public export
    data StaticParts : Nat -> Bindings -> Type where
      Nil : StaticParts Z bs
      (::) : (se : StaticEffect bs) -> {auto 0 nc : NotCoord se} ->
             StaticParts n (staticIntro se) -> StaticParts (S n) bs

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
