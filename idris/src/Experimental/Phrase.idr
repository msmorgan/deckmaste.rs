module Experimental.Phrase

import public Experimental.Words
import public Experimental.Events

%default total

mutual
  public export
  data ZoneScope : Bindings -> Zone -> Type where
    Bare : ZoneScope bs z
    PossessedBy : (n : Noun bs Player) -> {auto 0 ps : Possessable z} ->
                  ZoneScope bs z

  public export
  data LibPlace : Bindings -> Type where
    OneEnd : (pos : LibPos) -> LibPlace bs
    EitherEnd : (chooser : Maybe (Noun bs Player)) ->
                {auto 0 ag : EventAgent chooser} -> LibPlace bs
    Shuffled : LibPlace bs

  public export
  placeArrangementOk : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Bool
  placeArrangementOk (OneEnd _) _ = True
  placeArrangementOk (EitherEnd _) Nothing = True
  placeArrangementOk (EitherEnd _) (Just _) = False
  placeArrangementOk Shuffled Nothing = True
  placeArrangementOk Shuffled (Just _) = False

  public export
  PlaceArrangementFits : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Type
  PlaceArrangementFits pl ord = So (placeArrangementOk pl ord)

  public export
  placeOrdinalOk : {0 bs : Bindings} -> LibPlace bs -> Maybe LibOrdinal -> Bool
  placeOrdinalOk Shuffled (Just _) = False
  placeOrdinalOk Shuffled Nothing = True
  placeOrdinalOk (OneEnd _) _ = True
  placeOrdinalOk (EitherEnd _) _ = True

  public export
  PlaceOrdinalFits : {0 bs : Bindings} -> LibPlace bs -> Maybe LibOrdinal -> Type
  PlaceOrdinalFits pl off = So (placeOrdinalOk pl off)

  public export
  data ZoneExpr : Bindings -> Type where
    ZoneAt : (z : Zone) -> ZoneScope bs z -> ZoneExpr bs
    LibraryAt : (place : LibPlace bs) -> (ord : Maybe Arrangement) ->
                (off : Maybe LibOrdinal) ->
                {auto 0 af : PlaceArrangementFits place ord} ->
                {auto 0 nf : PlaceOrdinalFits place off} ->
                ZoneScope bs Library -> ZoneExpr bs

  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort (ZoneAt z _) = z
  zoneSort (LibraryAt _ _ _ _) = Library

  public export
  zoneArrangement : ZoneExpr bs -> Maybe Arrangement
  zoneArrangement (ZoneAt _ _) = Nothing
  zoneArrangement (LibraryAt _ ord _ _) = ord

  public export
  zoneOrdinal : ZoneExpr bs -> Maybe LibOrdinal
  zoneOrdinal (ZoneAt _ _) = Nothing
  zoneOrdinal (LibraryAt _ _ off _) = off

  public export
  placeShuffles : {0 bs : Bindings} -> LibPlace bs -> Bool
  placeShuffles Shuffled = True
  placeShuffles (OneEnd _) = False
  placeShuffles (EitherEnd _) = False

  public export
  zoneShuffles : ZoneExpr bs -> Bool
  zoneShuffles (ZoneAt _ _) = False
  zoneShuffles (LibraryAt place _ _ _) = placeShuffles place

  public export
  afterMoveTo : {0 bs : Bindings} -> ZoneExpr bs -> Bindings -> Bindings
  afterMoveTo to out = if zoneShuffles to then afterShuffle out else out

  public export
  data NameSource : Bindings -> Type where
    PrintedName : (name : String) -> NameSource bs
    ChosenName : {auto 0 ok : countChoice (QSort CardName) bs = 1} -> NameSource bs
    SameNameAs : (n : Noun bs Object) -> NameSource bs

  public export
  Eq (NameSource bs) where
    (==) (PrintedName a) (PrintedName b) = a == b
    (==) (PrintedName _) _ = False
    (==) ChosenName ChosenName = True
    (==) ChosenName _ = False
    (==) (SameNameAs a) (SameNameAs b) = nounEqRef a b
    (==) (SameNameAs _) _ = False

  public export
  nameSrcDelta : {bs : Bindings} -> NameSource bs -> List Binding
  nameSrcDelta (PrintedName _) = []
  nameSrcDelta ChosenName = []
  nameSrcDelta (SameNameAs n) = nounDelta n

  public export
  data ChoiceDomain : ChoiceSort -> Type where
    NameOfCard : (p : Predicate [] Object) -> ChoiceDomain (QSort CardName)
    ColorOtherThan : (c : Chroma.Color) -> ChoiceDomain (QSort Color)
    TypeOtherThan : (s : Subtype) ->
                    {auto 0 ct : subtypeType s = Just Creature} ->
                    ChoiceDomain (QSort (SubtypeQ Creature))
    BasicTypesOnly : ChoiceDomain (QSort (SubtypeQ Land))
    NonbasicTypesOnly : ChoiceDomain (QSort (SubtypeQ Land))
    NumberAbove : (n : Nat) -> ChoiceDomain (QSort Number)
    OpponentsOnly : ChoiceDomain PlayerC
    NumberBetween : (lo : Nat) -> (hi : Nat) ->
                    {auto 0 ok : So (lo <= hi)} -> ChoiceDomain (QSort Number)

  public export
  sameChoiceDomain : {0 a, b : ChoiceSort} ->
                     ChoiceDomain a -> ChoiceDomain b -> Bool
  sameChoiceDomain (NameOfCard a) (NameOfCard b) = predEq a b
  sameChoiceDomain (ColorOtherThan a) (ColorOtherThan b) = a == b
  sameChoiceDomain (TypeOtherThan a) (TypeOtherThan b) = a == b
  sameChoiceDomain BasicTypesOnly BasicTypesOnly = True
  sameChoiceDomain NonbasicTypesOnly NonbasicTypesOnly = True
  sameChoiceDomain (NumberAbove a) (NumberAbove b) = a == b
  sameChoiceDomain (NumberBetween a b) (NumberBetween c d) = a == c && b == d
  sameChoiceDomain OpponentsOnly OpponentsOnly = True
  sameChoiceDomain _ _ = False

  public export
  sameDomainOpt : {0 a, b : ChoiceSort} ->
                  Maybe (ChoiceDomain a) -> Maybe (ChoiceDomain b) -> Bool
  sameDomainOpt Nothing Nothing = True
  sameDomainOpt (Just a) (Just b) = sameChoiceDomain a b
  sameDomainOpt _ _ = False

  public export
  data EventSource : Bindings -> Type where
    FromAnywhere : EventSource bs
    FromZone : (zs : List (ZoneExpr bs)) -> EventSource bs
    FromAnywhereBut : (zs : List (ZoneExpr bs)) -> EventSource bs

  public export
  sourceZone : {0 bs : Bindings} -> Maybe (EventSource bs) -> Maybe Zone
  sourceZone Nothing = Nothing
  sourceZone (Just FromAnywhere) = Nothing
  sourceZone (Just (FromZone [z])) = Just (zoneSort z)
  sourceZone (Just (FromZone _)) = Nothing
  sourceZone (Just (FromAnywhereBut _)) = Nothing

  public export
  sourceDelta : {bs : Bindings} -> EventSource bs -> List Binding
  sourceDelta FromAnywhere = []
  sourceDelta (FromZone zs) = zonesDelta zs
  sourceDelta (FromAnywhereBut zs) = zonesDelta zs

  public export
  lookbackSourceOk : {0 bs : Bindings} -> EventName -> EventSource bs -> Bool
  lookbackSourceOk ev FromAnywhere = eventNamesOrigin ev
  lookbackSourceOk ev (FromZone zs) = lookbackZonesOk ev zs
  lookbackSourceOk ev (FromAnywhereBut zs) = lookbackZonesOk ev zs

  public export
  data EventComplement : Bindings -> EventName -> Kind -> Type where
    Involving : {kc : Kind} -> (what : Noun bs kc) ->
                {auto 0 cp : LookbackComplement ev ks kc} ->
                EventComplement bs ev ks
    FromZones : (src : EventSource bs) ->
                (what : Maybe (EventComplement bs ev ks)) ->
                {auto 0 pl : So (complementPlain what)} ->
                {auto 0 ok : So (lookbackSourceOk ev src)} ->
                EventComplement bs ev ks
    IntoZone : (to : ZoneExpr bs) ->
               (what : Maybe (EventComplement bs ev ks)) ->
               {auto 0 pl : So (complementSourced what)} ->
               {auto 0 ok : So (lookbackDestOk ev (zoneSort to))} ->
               EventComplement bs ev ks
    AtZone : (z : ZoneExpr bs) ->
             {auto 0 ok : So (lookbackLocusOk ev (zoneSort z))} ->
             EventComplement bs ev ks

  public export
  data ComplementWritten : {0 bs : Bindings} -> {0 ev : EventName} ->
                           {0 ks : Kind} ->
                           Maybe (EventComplement bs ev ks) -> Type where
    LeftBare : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
               {auto 0 ok : So (bareLookbackOk ev ks)} ->
               ComplementWritten {bs} {ev} {ks} Nothing
    Written : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
              {0 c : EventComplement bs ev ks} -> ComplementWritten (Just c)

  public export
  complementPlain : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                    Maybe (EventComplement bs ev ks) -> Bool
  complementPlain Nothing = True
  complementPlain (Just (Involving _)) = True
  complementPlain (Just (FromZones _ _)) = False
  complementPlain (Just (IntoZone _ _)) = False
  complementPlain (Just (AtZone _)) = False

  public export
  complementSourced : {0 bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                      Maybe (EventComplement bs ev ks) -> Bool
  complementSourced Nothing = True
  complementSourced (Just (Involving _)) = True
  complementSourced (Just (FromZones _ _)) = True
  complementSourced (Just (IntoZone _ _)) = False
  complementSourced (Just (AtZone _)) = False

  public export
  lookbackZonesOk : {0 bs : Bindings} -> EventName -> List (ZoneExpr bs) -> Bool
  lookbackZonesOk _ [] = False
  lookbackZonesOk ev (z :: zs) =
    lookbackOriginOk ev (zoneSort z) && allOriginZonesOk ev zs

  public export
  allOriginZonesOk : {0 bs : Bindings} -> EventName -> List (ZoneExpr bs) -> Bool
  allOriginZonesOk _ [] = True
  allOriginZonesOk ev (z :: zs) =
    lookbackOriginOk ev (zoneSort z) && allOriginZonesOk ev zs

  public export
  data Predicate : Bindings -> Kind -> Type where
    HasType : CardType -> Predicate bs Object
    HasSubtype : Subtype -> Predicate bs Object
    AnyPlayer : Predicate bs Player
    Opponent : Predicate bs Player
    ChosenPlayer : {auto 0 ok : countChoice PlayerC bs = 1} ->
                   Predicate bs Player
    TheLastChosenPlayer : {auto 0 ok : ChoiceStands (countChoice PlayerC bs)} ->
                          Predicate bs Player
    QualityNoun : (q : QualitySort) ->
                  (dom : Maybe (ChoiceDomain (QSort q))) ->
                  Predicate bs (Quality q)
    CounterKindOn : (n : Noun bs Object) ->
                    {auto 0 zn : ZoneIs (nounZone n) Battlefield} ->
                    Predicate bs (Quality CounterKindQ)
    OfChosen : (q : QualitySort) -> {auto 0 ok : countChoice (QSort q) bs = 1} ->
               {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    OfTheLastChosen : (q : QualitySort) ->
                      {auto 0 ok : ChoiceStands (countChoice (QSort q) bs)} ->
                      {auto 0 read : ChosenQualityRead q} ->
                      Predicate bs Object
    OfYourChoice : (q : QualitySort) -> (dom : Maybe (ChoiceDomain (QSort q))) ->
                   {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    HasKeyword : (k : KeywordTerm) -> {auto 0 kn : KnownKeywordTerm k} ->
                 Predicate bs Object
    HasPossessor : {k : Kind} -> (ax : PossessorAxis) -> (n : Noun bs Player) ->
                   {auto 0 ps : SoleHolder n} ->
                   {auto 0 ck : So (possessorKind ax k)} -> Predicate bs k
    CastBy : (n : Noun bs Player) -> (rank : Maybe (Ordinal, RankPeriod)) ->
             {auto 0 ps : SoleHolder n} -> Predicate bs Object
    CastFrom : (z : ZoneExpr bs) ->
               {auto 0 pf : So (playableFrom (Just (zoneSort z)))} ->
               Predicate bs Object
    WasCast : Predicate bs Object
    Attacking : Predicate bs Object
    BeingDeclaredAttacker : Predicate bs Object
    Blocking : Predicate bs Object
    Blocked : Predicate bs Object
    CombatRel : {k : Kind} -> {km : Kind} -> (r : CombatRelation) ->
                (m : Noun bs km) ->
                {auto 0 ok : So (combatRelOk r k km (nounZone m) (nounTys m))} ->
                Predicate bs k
    HappenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
                 (what : Maybe (EventComplement bs ev k)) ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
    ColorIs : (c : Chroma.Color) -> Predicate bs Object
    IsColorless : Predicate bs Object
    ColorCount : (r : Comparator) -> (n : Nat) ->
                 {auto 0 ok : So (colorBoundOk r n)} -> Predicate bs Object
    HasSupertype : (s : Supertype) -> Predicate bs Object
    Named : (src : NameSource bs) -> Predicate bs Object
    HasDesignation : (d : Designation) ->
                     {auto 0 sc : designationHolder d = Just k} -> Predicate bs k
    IsAttached : (w : AttachWord) ->
                 {auto 0 ok : So (attachedCheckOk w)} -> Predicate bs Object
    AttachedBy : (w : AttachWord) -> (by : Noun bs Object) ->
                 {auto 0 ok : So (attachedCheckOk w)} -> Predicate bs Object
    AttachedTo : {k : Kind} -> (host : Noun bs k) ->
                 {auto 0 hk : So (kindLte k (Object \/ Player))} ->
                 Predicate bs Object
    Permanent : Predicate bs Object
    IsCard : Predicate bs Object
    IsToken : Predicate bs Object
    IsSpell : Predicate bs Object
    IsEmblem : Predicate bs Object
    IsCopyOfACard : Predicate bs Object
    IsHistoric : Predicate bs Object
    IsTransformed : Predicate bs Object
    HasStatus : {c : StatusCat} -> (v : StatusVal c) -> Predicate bs Object
    HasCounters : (kind : Maybe CounterKind) ->
                  {auto 0 kn : CounterKindNamed Object kind} ->
                  Predicate bs Object
    Compare : {k : Kind} -> (axes : List ProjAxis) -> (r : Comparator) ->
              (bound : Amount bs) ->
              {auto 0 at : AxesAt k axes} -> Predicate bs k
    Superlative : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                  (dom : Predicate bs k) ->
                  {auto 0 ex : IsExtremal op} ->
                  {auto 0 sc : projScope ax = k} ->
                  Predicate bs k
    WithMostVotes : {k : Kind} -> Predicate bs k
    ChoseExtreme : (op : AggregateOp) ->
                   {auto 0 ex : IsExtremal op} -> Predicate bs Player
    CompareOver : {k : Kind} -> (dom : Predicate bs k) ->
                  {auto ph : Phrasal k} ->
                  (measure : Amount (bindFor TheD OneOf ph dom
                                       :: (predDelta dom ++ bs))) ->
                  (r : Comparator) -> (bound : Amount bs) ->
                  Predicate bs k
    InZone : ZoneExpr bs -> Predicate bs Object
    InPile : (pile : Noun bs Pile) -> {auto 0 pm : PileMention pile} ->
             Predicate bs Object
    ExiledWith : (src : Noun bs Object) ->
                 {auto 0 ls : LinkSource src} -> Predicate bs Object
    And : (ps : List (Predicate bs k)) -> {auto 0 zc : ZoneCoherent ps} ->
          {auto 0 cf : ContradictionFree ps} -> {auto 0 oa : OtherAnchored ps} ->
          Predicate bs k
    Or : (ps : List (Predicate bs k)) -> {auto 0 ne : NonEmpty ps} ->
         {auto 0 pd : ParallelDisjuncts ps} ->
         {auto 0 cd : CoordinableDisjuncts ps} ->
         {auto 0 dd : DistinctDisjuncts ps} -> Predicate bs k
    Not : (p : Predicate bs k) -> {auto 0 ng : Negatable p} -> Predicate bs k
    Other : {auto 0 ok : So (anyTargeted k bs)} -> Predicate bs k
    OtherThan : {kn : Kind} -> (n : Noun bs kn) ->
                {auto 0 ca : ComplementAnchor n} -> Predicate bs k
    Joined : {ka : Kind} -> {kb : Kind} -> (l : Predicate bs ka) ->
             (r : Predicate bs kb) -> Predicate bs (ka \/ kb)
    CoinCameUp : {k : Kind} -> (face : CoinFace) ->
                 {auto 0 fl : So (coinFlipInScope bs)} ->
                 {auto 0 rk : So (kindLte k (Object \/ Player))} ->
                 Predicate bs k
    IsSource : Predicate bs Object
    ManaCostHas : (sym : ManaSymbol) -> Predicate bs Object
    AbilityHead : (cls : AbilityClass) -> Predicate bs Object
    AbilityOf : (src : Noun bs Object) -> Predicate bs Object
    ActivatedBy : (who : Noun bs Player) ->
                  {auto 0 ps : SoleHolder who} -> Predicate bs Object
    IsManaAbility : Predicate bs Object
    Targets : {kt : Kind} -> (m : Noun bs kt) -> (extent : TargetExtent) ->
              {auto 0 tk : Targetable kt} ->
              {auto 0 tr : Targeter k} -> Predicate bs k

  public export
  seedTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedTy (HasType t) = Just t
  seedTy (HasSubtype s) = subtypeType s
  seedTy (And ps) = seedTyAll ps
  seedTy (Or ps) = seedTyJoin ps
  seedTy (Joined l r) = joinSeed (seedTy l) (seedTy r)
  seedTy (CompareOver dom _ _ _) = seedTy dom
  seedTy _ = Nothing

  public export
  seedTys : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> HeadTy k
  seedTys (Joined l r) = JoinTy (seedTys l) (seedTys r)
  seedTys p = SoleTy (seedTy p)

  public export
  seedTyAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyAll [] = Nothing
  seedTyAll (p :: ps) = case seedTy p of
    Just t => Just t
    Nothing => seedTyAll ps

  public export
  seedTyJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyJoin [] = Nothing
  seedTyJoin (p :: ps) = case seedTy p of
    Nothing => Nothing
    Just t => if allSeedTy t ps then Just t else Nothing

  public export
  allSeedTy : {0 bs : Bindings} -> {0 k : Kind} ->
              CardType -> List (Predicate bs k) -> Bool
  allSeedTy t [] = True
  allSeedTy t (p :: ps) = case seedTy p of
    Nothing => False
    Just u => t == u && allSeedTy t ps

  public export
  optCT : Maybe CardType -> List CardType
  optCT Nothing = []
  optCT (Just t) = [t]

  public export
  headTys : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  headTys (And ps) = headTysAll ps
  headTys (Or ps) = headTysJoin ps
  headTys (Joined l r) = headTys l ++ headTys r
  headTys p = optCT (seedTy p)

  public export
  headTysJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> List CardType
  headTysJoin [] = []
  headTysJoin (p :: ps) = headTys p ++ headTysJoin ps

  public export
  soleAlt : List CardType -> List (List CardType)
  soleAlt [] = []
  soleAlt ts = [ts]

  public export
  headTyAlts : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> List (List CardType)
  headTyAlts (And ps) = soleAlt (headTysJoin ps)
  headTyAlts (Or ps) = headTyAltsJoin ps
  headTyAlts (Joined l r) = headTyAlts l ++ headTyAlts r
  headTyAlts p = soleAlt (optCT (seedTy p))

  public export
  headTyAltsJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> List (List CardType)
  headTyAltsJoin [] = []
  headTyAltsJoin (p :: ps) = headTyAlts p ++ headTyAltsJoin ps

  public export
  headTysAll : {0 bs : Bindings} -> {0 k : Kind} ->
               List (Predicate bs k) -> List CardType
  headTysAll [] = []
  headTysAll (p :: ps) = case headTys p of
    [] => headTysAll ps
    ts => ts

  public export
  seedZone : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe Zone
  seedZone (InZone z) = Just (zoneSort z)
  seedZone Attacking = Just Battlefield
  seedZone BeingDeclaredAttacker = Just Battlefield
  seedZone Blocking = Just Battlefield
  seedZone (CombatRel AttackedBy _) = Nothing
  seedZone (CombatRel _ _) = Just Battlefield
  seedZone Blocked = Just Battlefield
  seedZone (HasDesignation d) = designationSeedZone d
  seedZone (IsAttached _) = Just Battlefield
  seedZone (AttachedBy _ _) = Just Battlefield
  seedZone (AttachedTo _) = Just Battlefield
  seedZone IsToken = Just Battlefield
  seedZone IsSpell = Just Stack
  seedZone IsEmblem = Just Command
  seedZone IsTransformed = Just Battlefield
  seedZone (HasStatus _) = Just Battlefield
  seedZone (Targets _ _) = Just Stack
  seedZone (ExiledWith _) = Just Exile
  seedZone (CompareOver dom _ _ _) = seedZone dom
  seedZone (AbilityHead _) = Just Stack
  seedZone (AbilityOf _) = Just Stack
  seedZone (ActivatedBy _) = Just Stack
  seedZone IsManaAbility = Just Stack
  seedZone (And ps) = seedZoneAll ps
  seedZone (Or ps) = seedZoneJoin ps
  seedZone _ = Nothing

  public export
  seedZoneAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneAll [] = Nothing
  seedZoneAll (p :: ps) = case seedZone p of
    Just z => Just z
    Nothing => seedZoneAll ps

  public export
  seedZoneJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneJoin [] = Nothing
  seedZoneJoin (p :: ps) = case seedZone p of
    Nothing => Nothing
    Just z => if allSeedZone z ps then Just z else Nothing

  public export
  allSeedZone : {0 bs : Bindings} -> {0 k : Kind} ->
                Zone -> List (Predicate bs k) -> Bool
  allSeedZone z [] = True
  allSeedZone z (p :: ps) = case seedZone p of
    Nothing => False
    Just w => z == w && allSeedZone z ps

  public export
  seedsToken : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  seedsToken IsToken = True
  seedsToken IsTransformed = False
  seedsToken (And ps) = seedsTokenAny ps
  seedsToken (Or ps) = seedsTokenAll ps
  seedsToken (CompareOver dom _ _ _) = seedsToken dom
  seedsToken _ = False

  public export
  seedsTokenAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAny [] = False
  seedsTokenAny (p :: ps) = seedsToken p || seedsTokenAny ps

  public export
  seedsTokenAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAll [] = False
  seedsTokenAll (p :: ps) = seedsToken p && seedsTokenAll ps

  ||| An activated or triggered ability on the stack is an object [CR#113.1c].
  public export
  seedsAbility : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  seedsAbility (AbilityHead _) = True
  seedsAbility (AbilityOf _) = True
  seedsAbility (ActivatedBy _) = True
  seedsAbility IsManaAbility = True
  seedsAbility (And ps) = seedsAbilityAny ps
  seedsAbility (Or ps) = seedsAbilityAll ps
  seedsAbility (CompareOver dom _ _ _) = seedsAbility dom
  seedsAbility _ = False

  public export
  seedsAbilityAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsAbilityAny [] = False
  seedsAbilityAny (p :: ps) = seedsAbility p || seedsAbilityAny ps

  public export
  seedsAbilityAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsAbilityAll [] = False
  seedsAbilityAll (p :: ps) = seedsAbility p && seedsAbilityAll ps

  public export
  zoneAdmit : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  zoneAdmit (HasPossessor ControllerAx _) = [Battlefield, Stack]
  zoneAdmit _ = []

  public export
  seedType : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedType Attacking = Just Creature
  seedType BeingDeclaredAttacker = Just Creature
  seedType Blocking = Just Creature
  seedType (CombatRel AttackedBy _) = Nothing
  seedType (CombatRel _ _) = Just Creature
  seedType Blocked = Just Creature
  seedType (HasDesignation d) = designationSeedType d
  seedType (Compare cs _ _) = axisTypes cs
  seedType (Superlative _ (CharAxis c) _) = comparedType c
  seedType (CompareOver dom _ _ _) = seedType dom
  seedType (HasSubtype s) = subtypeType s
  seedType (And ps) = seedTypeAll ps
  seedType (Or ps) = seedTypeJoin ps
  seedType _ = Nothing

  public export
  seedTypeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> Maybe CardType
  seedTypeAll [] = Nothing
  seedTypeAll (p :: ps) = case seedType p of
    Just t => Just t
    Nothing => seedTypeAll ps

  public export
  seedTypeJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                 List (Predicate bs k) -> Maybe CardType
  seedTypeJoin [] = Nothing
  seedTypeJoin (p :: ps) = case seedType p of
    Nothing => Nothing
    Just t => if allSeedType t ps then Just t else Nothing

  public export
  allSeedType : {0 bs : Bindings} -> {0 k : Kind} ->
                CardType -> List (Predicate bs k) -> Bool
  allSeedType t [] = True
  allSeedType t (p :: ps) = case seedType p of
    Nothing => False
    Just u => t == u && allSeedType t ps

  public export
  hasHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasHead (HasType _) = True
  hasHead (HasSubtype _) = True
  hasHead AnyPlayer = True
  hasHead Opponent = True
  hasHead ChosenPlayer = True
  hasHead TheLastChosenPlayer = True
  hasHead (QualityNoun _ _) = True
  hasHead (CounterKindOn _) = True
  hasHead (AbilityHead _) = True
  hasHead IsSource = True
  hasHead (HasDesignation d) = heldByItsCard d
  hasHead Permanent = True
  hasHead IsCard = True
  hasHead IsToken = True
  hasHead IsSpell = True
  hasHead IsEmblem = True
  hasHead IsCopyOfACard = True
  hasHead (CompareOver dom _ _ _) = hasHead dom
  hasHead (InZone _) = True
  hasHead (ExiledWith _) = True
  hasHead (And ps) = hasHeadAny ps
  hasHead (Or ps) = hasHeadAll ps
  hasHead (Joined _ _) = True
  hasHead _ = False

  public export
  hasHeadAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAny [] = False
  hasHeadAny (p :: ps) = hasHead p || hasHeadAny ps

  public export
  hasHeadAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAll [] = True
  hasHeadAll (p :: ps) = hasHead p && hasHeadAll ps

  public export
  qualityReadOk : {0 bs : Bindings} -> Predicate bs Object -> Bool
  qualityReadOk (OfChosen _) = True
  qualityReadOk (OfTheLastChosen _) = True
  qualityReadOk (OfYourChoice _ _) = True
  qualityReadOk (Named _) = True
  qualityReadOk _ = False

  public export
  QualityRead : Predicate bs Object -> Type
  QualityRead {bs} p = So (qualityReadOk p)

  public export
  qualityReadHost : {0 bs : Bindings} -> Predicate bs Object -> Maybe CardType
  qualityReadHost (OfChosen (SubtypeQ h)) = Just h
  qualityReadHost (OfTheLastChosen (SubtypeQ h)) = Just h
  qualityReadHost (OfYourChoice (SubtypeQ h) _) = Just h
  qualityReadHost _ = Nothing

  public export
  uniquifiesAny : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  uniquifiesAny [] = False
  uniquifiesAny (p :: ps) = uniquifies p || uniquifiesAny ps

  public export
  uniquifies : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  uniquifies ChosenPlayer = True
  uniquifies TheLastChosenPlayer = True
  uniquifies (Superlative _ _ _) = True
  uniquifies WithMostVotes = False
  uniquifies (ChoseExtreme _) = False
  uniquifies (CombatRel AttackedBy _) = True
  uniquifies (CastBy _ rank) = isJust rank
  uniquifies (And ps) = uniquifiesAny ps
  uniquifies _ = False

  public export
  Uniquifying : Predicate bs k -> Type
  Uniquifying {bs} {k} p = So (uniquifies p)

  public export
  flattenPs : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k)
  flattenPs [] = []
  flattenPs (And qs :: ps) = flattenPs qs ++ flattenPs ps
  flattenPs (p :: ps) = p :: flattenPs ps

  public export
  zonesAgree : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> List (Predicate bs k) -> Bool
  zonesAgree acc [] = True
  zonesAgree acc (p :: ps) = case seedZone p of
    Nothing => zonesAgree acc ps
    Just z => case acc of
      Nothing => zonesAgree (Just z) ps
      Just w => w == z && zonesAgree (Just w) ps

  public export
  negZonesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  negZonesOf (Not (InZone z)) = [zoneSort z]
  negZonesOf _ = []

  public export
  negZones : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List Zone
  negZones [] = []
  negZones (p :: ps) = negZonesOf p ++ negZones ps

  public export
  zoneAdmits : Zone -> List Zone -> Bool
  zoneAdmits z [] = True
  zoneAdmits z zs = elem z zs

  public export
  zoneAdmitsAll : {0 bs : Bindings} -> {0 k : Kind} ->
                  Zone -> List (Predicate bs k) -> Bool
  zoneAdmitsAll z [] = True
  zoneAdmitsAll z (p :: ps) = zoneAdmits z (zoneAdmit p) && zoneAdmitsAll z ps

  public export
  zonesOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  zonesOk ps =
    let fs = flattenPs ps
        z = zoneOr Battlefield (seedZoneAll fs) in
    zonesAgree Nothing fs && not (elem z (negZones fs)) && zoneAdmitsAll z fs

  public export
  ZoneCoherent : List (Predicate bs k) -> Type
  ZoneCoherent {bs} {k} ps = So (zonesOk ps)

  public export
  predEq : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  predEq (HasType a) (HasType b) = a == b
  predEq (HasType _) _ = False
  predEq (HasSubtype a) (HasSubtype b) = a == b
  predEq (HasSubtype _) _ = False
  predEq ChosenPlayer ChosenPlayer = True
  predEq ChosenPlayer _ = False
  predEq TheLastChosenPlayer TheLastChosenPlayer = True
  predEq TheLastChosenPlayer _ = False
  predEq AnyPlayer AnyPlayer = True
  predEq AnyPlayer _ = False
  predEq Opponent Opponent = True
  predEq Opponent _ = False
  predEq (QualityNoun a d) (QualityNoun a e) = sameDomainOpt d e
  predEq (QualityNoun _ _) _ = False
  predEq (CounterKindOn a) (CounterKindOn b) = nounEqRef a b
  predEq (CounterKindOn _) _ = False
  predEq (OfChosen a) (OfChosen b) = a == b
  predEq (OfChosen _) _ = False
  predEq (OfTheLastChosen a) (OfTheLastChosen b) = a == b
  predEq (OfTheLastChosen _) _ = False
  predEq (OfYourChoice a d) (OfYourChoice b e) = a == b && sameDomainOpt d e
  predEq (OfYourChoice _ _) _ = False
  predEq (AbilityHead a) (AbilityHead b) = a == b
  predEq (AbilityHead _) _ = False
  predEq (AbilityOf a) (AbilityOf b) = nounEqRef a b
  predEq (AbilityOf _) _ = False
  predEq (ActivatedBy a) (ActivatedBy b) = nounEqRef a b
  predEq (ActivatedBy _) _ = False
  predEq IsManaAbility IsManaAbility = True
  predEq IsManaAbility _ = False
  predEq (Targets _ _) _ = False
  predEq IsSource IsSource = True
  predEq IsSource _ = False
  predEq (ManaCostHas _) _ = False
  predEq (HasKeyword a) (HasKeyword b) = a == b
  predEq (HasKeyword _) _ = False
  predEq (HasPossessor ax a) (HasPossessor bx b) = ax == bx && nounEqRef a b
  predEq (HasPossessor _ _) _ = False
  predEq (CastBy a Nothing) (CastBy b Nothing) = nounEqRef a b
  predEq (CastBy _ _) _ = False
  predEq (ExiledWith a) (ExiledWith b) = nounEqRef a b
  predEq (ExiledWith _) _ = False
  predEq (InPile a) (InPile b) = nounEqRef a b
  predEq (InPile _) _ = False
  predEq Attacking Attacking = True
  predEq Attacking _ = False
  predEq BeingDeclaredAttacker BeingDeclaredAttacker = True
  predEq BeingDeclaredAttacker _ = False
  predEq Blocking Blocking = True
  predEq Blocking _ = False
  predEq (CombatRel AttackerOf _) _ = False
  predEq Blocked Blocked = True
  predEq Blocked _ = False
  predEq (CombatRel {km = Object} r a) (CombatRel {km = Object} s b) =
    r == s && nounEqRef a b
  predEq (CombatRel _ _) _ = False
  predEq (HappenedTo a v Nothing) (HappenedTo b w Nothing) =
    sameEventName a b && sameLookback v w
  predEq (HappenedTo _ _ _) _ = False
  predEq (ColorIs a) (ColorIs b) = a == b
  predEq (ColorIs _) _ = False
  predEq IsColorless IsColorless = True
  predEq IsColorless _ = False
  predEq (ColorCount r a) (ColorCount s b) = r == s && a == b
  predEq (ColorCount _ _) _ = False
  predEq (HasSupertype a) (HasSupertype b) = a == b
  predEq (HasSupertype _) _ = False
  predEq (Named a) (Named b) = a == b
  predEq (Named _) _ = False
  predEq (HasDesignation a) (HasDesignation b) = a == b
  predEq (HasDesignation _) _ = False
  predEq (CoinCameUp Heads) (CoinCameUp Heads) = True
  predEq (CoinCameUp Tails) (CoinCameUp Tails) = True
  predEq (CoinCameUp _) _ = False
  predEq (IsAttached a) (IsAttached b) = a == b
  predEq (IsAttached _) _ = False
  predEq (AttachedBy a x) (AttachedBy b y) = a == b && nounEqRef x y
  predEq (AttachedBy _ _) _ = False
  predEq (AttachedTo _) _ = False
  predEq Permanent Permanent = True
  predEq Permanent _ = False
  predEq IsCard IsCard = True
  predEq IsCard _ = False
  predEq IsToken IsToken = True
  predEq IsToken _ = False
  predEq IsSpell IsSpell = True
  predEq IsSpell _ = False
  predEq IsEmblem IsEmblem = True
  predEq IsEmblem _ = False
  predEq IsCopyOfACard IsCopyOfACard = True
  predEq IsCopyOfACard _ = False
  predEq IsHistoric IsHistoric = True
  predEq IsHistoric _ = False
  predEq IsTransformed IsTransformed = True
  predEq IsTransformed _ = False
  predEq (HasStatus v) (HasStatus w) = sameStatusVal v w
  predEq (HasStatus _) _ = False
  predEq (HasCounters Nothing) (HasCounters Nothing) = True
  predEq (HasCounters (Just a)) (HasCounters (Just b)) = a == b
  predEq (HasCounters _) _ = False
  predEq (Compare cs r b) (Compare ds s e) = sameAxes cs ds && r == s &&
                                           boundEq b e
  predEq (Compare _ _ _) _ = False
  predEq (Superlative o a d) (Superlative p b e) =
    o == p && a == b && predEq d e
  predEq (Superlative _ _ _) _ = False
  predEq WithMostVotes WithMostVotes = True
  predEq WithMostVotes _ = False
  predEq (ChoseExtreme o) (ChoseExtreme p) = o == p
  predEq (ChoseExtreme _) _ = False
  predEq (CompareOver _ _ _ _) _ = False
  predEq (CastFrom z) (CastFrom w) = zoneSort z == zoneSort w
  predEq (CastFrom _) _ = False
  predEq WasCast WasCast = True
  predEq WasCast _ = False
  predEq (InZone z) (InZone w) = zoneSort z == zoneSort w
  predEq (InZone _) _ = False
  predEq (And xs) (And ys) = predEqAll xs ys
  predEq (And _) _ = False
  predEq (Or _) _ = False
  predEq (Not a) (Not b) = predEq a b
  predEq (Not _) _ = False
  predEq Other Other = True
  predEq Other _ = False
  predEq (OtherThan _) _ = False
  predEq (Joined _ _) _ = False

  public export
  predEqAll : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k) -> Bool
  predEqAll [] [] = True
  predEqAll (x :: xs) (y :: ys) = predEq x y && predEqAll xs ys
  predEqAll _ _ = False

  public export
  negates : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  negates (Not a) b = predEq a b
  negates a (Not b) = predEq a b
  negates _ _ = False

  public export
  anyNegates : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> List (Predicate bs k) -> Bool
  anyNegates p [] = False
  anyNegates p (q :: qs) = negates p q || anyNegates p qs

  public export
  noNegatedPair : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noNegatedPair [] = True
  noNegatedPair (p :: ps) = not (anyNegates p ps) && noNegatedPair ps

  public export
  statusClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> Predicate bs k -> Bool
  statusClashOf (HasStatus v) (HasStatus w) = statusClash v w
  statusClashOf _ _ = False

  public export
  colorClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                 Predicate bs k -> Predicate bs k -> Bool
  colorClashOf IsColorless (ColorIs _) = True
  colorClashOf (ColorIs _) IsColorless = True
  colorClashOf _ _ = False

  public export
  cardTokenClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                     Predicate bs k -> Predicate bs k -> Bool
  cardTokenClashOf IsCard IsToken = True
  cardTokenClashOf IsToken IsCard = True
  cardTokenClashOf IsEmblem IsCard = True
  cardTokenClashOf IsCard IsEmblem = True
  cardTokenClashOf IsEmblem Permanent = True
  cardTokenClashOf Permanent IsEmblem = True
  cardTokenClashOf IsEmblem IsToken = True
  cardTokenClashOf IsToken IsEmblem = True
  cardTokenClashOf IsCard IsCopyOfACard = True
  cardTokenClashOf IsCopyOfACard IsCard = True
  cardTokenClashOf _ _ = False

  public export
  anyCardTokenClash : {0 bs : Bindings} -> {0 k : Kind} ->
                      Predicate bs k -> List (Predicate bs k) -> Bool
  anyCardTokenClash p [] = False
  anyCardTokenClash p (q :: qs) = cardTokenClashOf p q || anyCardTokenClash p qs

  public export
  noCardTokenClash : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noCardTokenClash [] = True
  noCardTokenClash (p :: ps) = not (anyCardTokenClash p ps) && noCardTokenClash ps

  public export
  anyColorClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> List (Predicate bs k) -> Bool
  anyColorClash p [] = False
  anyColorClash p (q :: qs) = colorClashOf p q || anyColorClash p qs

  public export
  noColorClash : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noColorClash [] = True
  noColorClash (p :: ps) = not (anyColorClash p ps) && noColorClash ps

  public export
  anyStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                   Predicate bs k -> List (Predicate bs k) -> Bool
  anyStatusClash p [] = False
  anyStatusClash p (q :: qs) = statusClashOf p q || anyStatusClash p qs

  public export
  noStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  noStatusClash [] = True
  noStatusClash (p :: ps) = not (anyStatusClash p ps) && noStatusClash ps

  public export
  isPermanentHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isPermanentHead Permanent = True
  isPermanentHead _ = False

  public export
  anyPermanentHead : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Bool
  anyPermanentHead [] = False
  anyPermanentHead (p :: ps) = isPermanentHead p || anyPermanentHead ps

  public export
  anyNonPermanentTy : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  anyNonPermanentTy [] = False
  anyNonPermanentTy (p :: ps) = case seedTy p of
    Just t => not (permanentType t) || anyNonPermanentTy ps
    Nothing => anyNonPermanentTy ps

  public export
  negTypesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  negTypesOf (Not (HasSubtype _)) = []
  negTypesOf (Not p) = case seedTy p of
    Just t => [t]
    Nothing => []
  negTypesOf _ = []

  public export
  negTypes : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List CardType
  negTypes [] = []
  negTypes (p :: ps) = negTypesOf p ++ negTypes ps

  public export
  seedTypeAlts : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  seedTypeAlts (HasSubtype s) = case subtypeType s of
    Just Creature => [Creature, Kindred]
    Just t => [t]
    Nothing => [Instant, Sorcery]
  seedTypeAlts p = case seedType p of
    Just t => [t]
    Nothing => []

  public export
  allNegated : List CardType -> List CardType -> Bool
  allNegated negs [] = True
  allNegated negs (t :: ts) = elem t negs && allNegated negs ts

  public export
  anySeedEmptied : {0 bs : Bindings} -> {0 k : Kind} -> List CardType ->
                   List (Predicate bs k) -> Bool
  anySeedEmptied negs [] = False
  anySeedEmptied negs (p :: ps) = case seedTypeAlts p of
    [] => anySeedEmptied negs ps
    ts => allNegated negs ts || anySeedEmptied negs ps

  public export
  contradictionFree : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  contradictionFree ps =
    let fs = flattenPs ps in
    noNegatedPair fs &&
    not (anySeedEmptied (negTypes fs) fs) &&
    noStatusClash fs &&
    noColorClash fs &&
    noCardTokenClash fs &&
    not (anyPermanentHead fs && anyNonPermanentTy fs)

  public export
  ContradictionFree : List (Predicate bs k) -> Type
  ContradictionFree {bs} {k} ps = So (contradictionFree ps)

  public export
  hasOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasOther Other = True
  hasOther (OtherThan _) = True
  hasOther (And ps) = hasOtherAny ps
  hasOther (Or _) = False
  hasOther _ = False

  public export
  hasOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasOtherAny [] = False
  hasOtherAny (p :: ps) = hasOther p || hasOtherAny ps

  public export
  otherAnchorOk : {bs : Bindings} -> (k : Kind) -> List CardType ->
                  List (Predicate bs k) -> Bool
  otherAnchorOk k ts ps =
    let fs = flattenPs ps in
    if atMostOne (countOthers fs)
      then (if hasBareOtherAny ps
              then anyTargeted k bs
              else complementAnchorsOk ts fs)
      else False

  public export
  anchorTyFitsSome : List CardType -> Maybe CardType -> Bool
  anchorTyFitsSome [] t = False
  anchorTyFitsSome (u :: us) t = anchorTyOk u t || anchorTyFitsSome us t

  public export
  anchorTyFits : List CardType -> Maybe CardType -> Bool
  anchorTyFits [] t = True
  anchorTyFits (u :: us) t = anchorTyFitsSome (u :: us) t

  public export
  complementAnchorsOk : {bs : Bindings} -> {k : Kind} -> List CardType ->
                        List (Predicate bs k) -> Bool
  complementAnchorsOk ts [] = True
  complementAnchorsOk ts (OtherThan n :: ps) =
    complementAnchorsOk ts ps && anchorTyFits ts (nounTy n)
  complementAnchorsOk ts (_ :: ps) = complementAnchorsOk ts ps

  public export
  OtherAnchored : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> Type
  OtherAnchored {bs} {k} ps = So (otherAnchorOk k (headTysAll ps) ps)

  public export
  isOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOther Other = True
  isOther (OtherThan _) = True
  isOther _ = False

  public export
  hasBareOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasBareOther Other = True
  hasBareOther (And ps) = hasBareOtherAny ps
  hasBareOther _ = False

  public export
  hasBareOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasBareOtherAny [] = False
  hasBareOtherAny (p :: ps) = hasBareOther p || hasBareOtherAny ps

  public export
  isSourceHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isSourceHead IsSource = True
  isSourceHead IsCard = True
  isSourceHead _ = False

  public export
  anyIsSource : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsSource [] = False
  anyIsSource (p :: ps) = isSourceHead p || anyIsSource ps

  public export
  headIsPlaceless : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsPlaceless p = anyIsSource (flattenPs [p])

  public export
  phraseZone : {0 bs : Bindings} -> {k : Kind} -> Predicate bs k -> Maybe Zone
  phraseZone p = if headIsPlaceless p || not (kindLte k Object)
                   then Nothing
                   else Just (zoneOr Battlefield (seedZone p))

  public export
  countOthers : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Nat
  countOthers [] = Z
  countOthers (p :: ps) = if isOther p then S (countOthers ps) else countOthers ps

  public export
  atMostOne : Nat -> Bool
  atMostOne Z = True
  atMostOne (S Z) = True
  atMostOne (S (S _)) = False

  public export
  exactlyOne : Nat -> Bool
  exactlyOne Z = False
  exactlyOne (S Z) = True
  exactlyOne (S (S _)) = False

  public export
  isComparison : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isComparison (Compare _ _ _) = True
  isComparison (Superlative _ _ _) = True
  isComparison WithMostVotes = True
  isComparison (ChoseExtreme _) = True
  isComparison (CompareOver _ _ _ _) = True
  isComparison _ = False

  public export
  countComparisons : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Nat
  countComparisons [] = Z
  countComparisons (p :: ps) =
    if isComparison p then S (countComparisons ps) else countComparisons ps

  public export
  armPresupposes : {0 bs : Bindings} -> {0 k : Kind} ->
                   Predicate bs k -> Maybe CardType
  armPresupposes p = if hasHead p then Nothing else seedType p

  public export
  seedsUniform : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> Maybe CardType ->
                 List (Predicate bs k) -> Bool
  seedsUniform z t [] = True
  seedsUniform z t (p :: ps) = z == seedZone p &&
                               t == armPresupposes p &&
                               seedsUniform z t ps

  public export
  parallelDisjuncts : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  parallelDisjuncts [] = True
  parallelDisjuncts (p :: ps) = seedsUniform (seedZone p) (armPresupposes p) ps

  public export
  ParallelDisjuncts : List (Predicate bs k) -> Type
  ParallelDisjuncts {bs} {k} ps = So (parallelDisjuncts ps)

  public export
  isOr : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOr (Or _) = True
  isOr _ = False

  public export
  anyIsOr : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsOr [] = False
  anyIsOr (p :: ps) = isOr p || anyIsOr ps

  public export
  coordinable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  coordinable p = not (hasOther p)

  public export
  coordinableAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  coordinableAll [] = True
  coordinableAll (p :: ps) = coordinable p && coordinableAll ps

  public export
  CoordinableDisjuncts : List (Predicate bs k) -> Type
  CoordinableDisjuncts {bs} {k} ps = So (coordinableAll ps)

  public export
  anyPredEq : {0 bs : Bindings} -> {0 k : Kind} ->
              Predicate bs k -> List (Predicate bs k) -> Bool
  anyPredEq p [] = False
  anyPredEq p (q :: qs) = predEq p q || anyPredEq p qs

  public export
  noRepeatedPair : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  noRepeatedPair [] = True
  noRepeatedPair (p :: ps) = not (anyPredEq p ps) && noRepeatedPair ps

  public export
  DistinctDisjuncts : List (Predicate bs k) -> Type
  DistinctDisjuncts {bs} {k} ps = So (noRepeatedPair ps)

  public export
  negatable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  negatable AnyPlayer = False
  negatable ChosenPlayer = False
  negatable TheLastChosenPlayer = False
  negatable (QualityNoun _ Nothing) = False
  negatable (QualityNoun _ (Just _)) = True
  negatable (CounterKindOn _) = False
  negatable IsSource = False
  negatable _ = True

  public export
  Negatable : Predicate bs k -> Type
  Negatable {bs} {k} p = So (negatable p)

  public export
  predSays : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predSays (And ps) = predSaysAny ps
  predSays (Not p) = predSays p
  predSays _ = True

  public export
  predSaysAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  predSaysAny [] = False
  predSaysAny (p :: ps) = predSays p || predSaysAny ps

  public export
  PredSays : Predicate bs k -> Type
  PredSays {bs} {k} p = So (predSays p)

  public export
  predNegFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predNegFree (And ps) = predNegFreeAll ps
  predNegFree (Or ps) = predNegFreeAll ps
  predNegFree (Not _) = False
  predNegFree (Joined l r) = predNegFree l && predNegFree r
  predNegFree _ = True

  public export
  predNegFreeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  predNegFreeAll [] = True
  predNegFreeAll (p :: ps) = predNegFree p && predNegFreeAll ps

  public export
  ZoneFree : Predicate bs k -> Type
  ZoneFree {bs} {k} p = seedZone p = Nothing

  public export
  zoneOr : Zone -> Maybe Zone -> Zone
  zoneOr z Nothing = z
  zoneOr z (Just w) = w

  public export
  joinHalfPayload : {k : Kind} -> Phrasal k -> HeadTy k -> Payload k
  joinHalfPayload PhObject (SoleTy ty) = ObjectP ty Nothing Nothing Nothing Nothing
  joinHalfPayload PhPlayer _ = PlayerP
  joinHalfPayload {k = Quality q} PhQuality _ = QualityP
  joinHalfPayload (PhJoin l r) (JoinTy a b) =
    JoinP (joinHalfPayload l a) (joinHalfPayload r b)
  joinHalfPayload (PhJoin l r) (SoleTy ty) =
    JoinP (joinHalfPayload l (SoleTy ty)) (joinHalfPayload r (SoleTy ty))

  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Phrasal k -> Predicate bs k -> Binding
  bindFor det plur PhObject p =
    if seedsAbility p
      then MkBinding det Object plur (AbilityP Nothing)
      else MkBinding det Object plur
                     (ObjectP (seedTy p)
                              (Just (zoneOr Battlefield (seedZone p)))
                              Nothing
                              (if seedsToken p then Just TokenOrigin else Nothing)
                              Nothing)
  bindFor det plur PhPlayer p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} PhQuality p = MkBinding det (Quality q) plur QualityP
  bindFor det plur ph@(PhJoin l r) p =
    MkBinding det k plur (joinHalfPayload ph (seedTys p))

  public export
  data DetPhrase : Bindings -> Type where
    TargetDet : (q : Quantity bs) -> DetPhrase bs
    ADet : (m : ChoiceMode bs) -> DetPhrase bs
    EachDet : DetPhrase bs   -- "each …": a group, resolution-time [CR#608.2]
    AllDet : DetPhrase bs
    TheDet : DetPhrase bs
    CountDet : (q : Quantity bs) -> (mode : Maybe (ChoiceMode bs)) -> DetPhrase bs
    BareDet : DetPhrase bs   -- the bare plural: a description, no determiner [CR#109.2]

  public export
  detOf : {0 bs : Bindings} -> DetPhrase bs -> Determiner
  detOf (TargetDet _) = TargetD
  detOf (ADet _) = AD
  detOf EachDet = EachD
  detOf AllDet = AllD
  detOf TheDet = TheD
  detOf (CountDet _ _) = CountD
  detOf BareDet = BareD

  public export
  detPlur : {0 bs : Bindings} -> DetPhrase bs -> Plurality
  detPlur (TargetDet q) = quantPlur q
  detPlur (CountDet q _) = quantPlur q
  detPlur (ADet _) = OneOf
  detPlur TheDet = OneOf
  detPlur EachDet = ManyOf
  detPlur AllDet = ManyOf
  detPlur BareDet = ManyOf

  public export
  detOk : {bs : Bindings} -> {k : Kind} -> DetPhrase bs -> Predicate bs k -> Type
  detOk (TargetDet q) p = (NonZeroQ q, WellFormedQ q, Targetable k)
  detOk (CountDet q _) p = (NonZeroQ q, WellFormedQ q)
  detOk TheDet p = Uniquifying p
  detOk _ p = ()

  public export
  roleElem : {0 bs : Bindings} -> Predicate bs Object ->
             List (Predicate bs Object) -> Bool
  roleElem _ [] = False
  roleElem p (q :: qs) = predEq p q || roleElem p qs

  ||| One creature is the party member for only one role [CR#700.8b], so a
  ||| repeated role would count it twice.
  public export
  rolesDistinct : {0 bs : Bindings} -> List (Predicate bs Object) -> Bool
  rolesDistinct [] = True
  rolesDistinct (p :: ps) = not (roleElem p ps) && rolesDistinct ps

  ||| A one-each group is written from at least one distinct role [CR#700.8].
  public export
  data RolesOk : {0 bs : Bindings} -> List (Predicate bs Object) -> Type where
    RolesAre : {0 r : Predicate bs Object} ->
               {0 rs : List (Predicate bs Object)} ->
               {auto 0 di : So (rolesDistinct (r :: rs))} -> RolesOk (r :: rs)

  public export
  data Noun : Bindings -> Kind -> Type where
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    AsType : (t : CardType) -> (n : Noun bs Object) ->
             (sub : Maybe Subtype) ->
             {auto 0 asc : Ascribable n} ->
             {auto 0 way : So (ascriptionOk t sub)} -> Noun bs Object
    AsMarker : (m : MarkerWord) -> (n : Noun bs Object) ->
               {auto 0 asc : Ascribable n} -> Noun bs Object
    ResolvedPermanent : (spell : Noun bs Object) ->
                        {auto 0 zn : ZoneIs (nounZone spell) Stack} ->
                        {auto 0 pm : So (permanentSpellType (nounTy spell))} ->
                        Noun bs Object
    TheGrantor : (m : MarkerWord) -> Noun bs Object
    You : Noun bs Player
    TheDefendingPlayer : Noun bs Player
    TheAttackingPlayer : Noun bs Player
    PlayerGroup : (w : PlayerGroupWord) -> Noun bs Player
    Described : (d : DetPhrase bs) -> (p : Predicate bs k) ->
                {auto ph : Phrasal k} -> {auto 0 ok : detOk d p} -> Noun bs k
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    Both : {ka : Kind} -> {kb : Kind} -> (l : Noun bs ka) ->
           (r : Noun (nomIntro l) kb) -> {auto jk : Joins ka kb k} -> Noun bs k
    EitherOf : {ka : Kind} -> {kb : Kind} -> (l : Noun bs ka) ->
               (r : Noun bs kb) -> {auto jk : Joins ka kb k} -> Noun bs k
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 sp : SlicePossessor whose} ->
                   Noun bs Object
    SomeOf : (q : SliceCount bs) -> (descr : Maybe (Predicate bs Object)) ->
             (grp : Noun bs Object) ->
             {auto 0 gm : PartitiveBase grp} -> Noun bs Object
    NamesAgree : (agr : NameAgreement) -> (grp : Noun bs Object) ->
                 {auto 0 cm : CountedMention grp} ->
                 {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
    TheRest : (k : Kind) -> (pl : Plurality) ->
              {auto 0 ok : So (theRestFits k pl bs)} -> Noun bs k
    PileOf : (q : SliceCount bs) -> (by : Maybe (Noun bs Player)) ->
             {auto 0 ok : countReach (Word PileW) ManyOf bs = 1} -> Noun bs Pile
    Pro : (r : Reach) -> (pl : Plurality) ->
          {auto 0 ok : countReach r pl bs = 1} -> Noun bs (reachKind r)
    ItOtherThan : (co : Bindings) -> (rest : Bindings) ->
                  {auto 0 sp : bs = co ++ rest} ->
                  {auto 0 ok : countOnes Object rest = 1} -> Noun bs Object
    Own : (pl : Plurality) -> (own : Bindings) -> (outer : Bindings) ->
          {auto 0 sp : bs = own ++ outer} ->
          {auto 0 ok : countReach Bare pl own = 1} -> Noun bs Object
    AttachHost : (w : AttachWord) -> (h : NounWord) ->
                 {auto 0 ok : AttachHeadOk w h} -> Noun bs (kindOfW h)
    PossessorOf : {k : Kind} -> (ax : PossessorAxis) -> (n : Noun bs k) ->
                  {auto 0 one : nounPlur n = OneOf} ->
                  {auto 0 ck : So (possessorKind ax k)} -> Noun bs Player
    PossessorsOf : (ax : PossessorAxis) -> (grp : Noun bs Object) ->
                   {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Player
    Designated : (d : Designation) -> (whose : Noun bs Player) ->
                 {auto 0 sc : designationScope d = HeldByCard} -> Noun bs Object
    OneEachOf : (roles : List (Predicate bs Object)) -> (pool : Noun bs Object) ->
                {auto 0 pl : nounPlur pool = ManyOf} ->
                {auto 0 rk : RolesOk roles} -> Noun bs Object

  public export
  ascribable : {0 bs : Bindings} -> Noun bs Object -> Bool
  ascribable This = True
  ascribable _ = False

  public export
  Ascribable : {0 bs : Bindings} -> Noun bs Object -> Type
  Ascribable n = So (ascribable n)

  public export
  nounEqRef : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Noun bs k -> Bool
  nounEqRef This This = True
  nounEqRef This _ = False
  nounEqRef (AsType _ _ _) _ = False
  nounEqRef (ResolvedPermanent _) _ = False
  nounEqRef (AsMarker _ _) _ = False
  nounEqRef (TheGrantor a) (TheGrantor b) = a == b
  nounEqRef (TheGrantor _) _ = False
  nounEqRef TheDefendingPlayer TheDefendingPlayer = True
  nounEqRef TheDefendingPlayer _ = False
  nounEqRef TheAttackingPlayer TheAttackingPlayer = True
  nounEqRef TheAttackingPlayer _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
  nounEqRef (PlayerGroup v) (PlayerGroup w) = v == w
  nounEqRef (PlayerGroup _) _ = False
  nounEqRef (Described _ _) _ = False
  nounEqRef (EachOf _) _ = False
  nounEqRef (Both _ _) _ = False
  nounEqRef (EitherOf _ _) _ = False
  nounEqRef (LibrarySlice _ _ _) _ = False
  nounEqRef (SomeOf _ _ _) _ = False
  nounEqRef (NamesAgree _ _) _ = False
  nounEqRef (TheRest _ _) _ = False
  nounEqRef (PileOf _ _) _ = False
  nounEqRef (Pro Bare OneOf) (Pro Bare OneOf) = True
  nounEqRef (Pro (Word AbilityW) OneOf) (Pro (Word AbilityW) OneOf) = True
  nounEqRef (Pro (Word PlayerW) OneOf) (Pro (Word PlayerW) OneOf) = True
  nounEqRef (Pro _ _) _ = False
  nounEqRef (ItOtherThan _ _) _ = False
  nounEqRef (Own _ _ _) _ = False
  nounEqRef (AttachHost _ _) _ = False
  nounEqRef (PossessorOf _ _) _ = False
  nounEqRef (PossessorsOf _ _) _ = False
  nounEqRef (Designated _ _) _ = False
  nounEqRef (OneEachOf _ _) _ = False

  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)
    LibraryPosOk : {auto 0 af : PlaceArrangementFits place arrg} ->
                   {auto 0 nf : PlaceOrdinalFits place offs} ->
                   DestOk (LibraryAt place arrg offs {af} {nf} Bare)

  public export
  orderOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Bool
  orderOk pl z = case zoneArrangement z of
                   Nothing => True
                   Just _ => not (isOne pl)

  public export
  ArrangementOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Type
  ArrangementOk {bs} pl z = So (orderOk pl z)

  public export
  sliceTy : {bs : Bindings} -> Maybe (Predicate bs Object) -> Noun bs Object ->
            Maybe CardType
  sliceTy Nothing grp = nounTy grp
  sliceTy (Just p) grp = case seedTy p of
                           Just t => Just t
                           Nothing => nounTy grp

  public export
  sliceDelta : {bs : Bindings} -> Maybe (Predicate bs Object) -> List Binding
  sliceDelta Nothing = []
  sliceDelta (Just p) = predDelta p

  ||| A definite description re-reads the chosen-player binding [CR#607.2d].
  public export
  choiceRead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  choiceRead ChosenPlayer = True
  choiceRead TheLastChosenPlayer = True
  choiceRead _ = False

  public export
  detDelta : {bs : Bindings} -> {k : Kind} -> DetPhrase bs -> Phrasal k ->
             Predicate bs k -> List Binding
  detDelta (TargetDet q) ph p =
    sized (quantExact q) (bindFor TargetD (quantPlur q) ph p)
      :: (quantDelta q ++ predDelta p)
  detDelta (CountDet q _) ph p =
    sized (quantExact q) (bindFor CountD (quantPlur q) ph p)
      :: (quantDelta q ++ predDelta p)
  detDelta TheDet ph p =
    if choiceRead p then predDelta p
                    else bindFor TheD OneOf ph p :: predDelta p
  detDelta d ph p = bindFor (detOf d) (detPlur d) ph p :: predDelta p

  public export
  nounDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  nounDelta This = []
  nounDelta (AsType t n _) = nounDelta n
  nounDelta (ResolvedPermanent n) = nounDelta n
  nounDelta (AsMarker _ n) = nounDelta n
  nounDelta (TheGrantor _) = []
  nounDelta TheDefendingPlayer = []
  nounDelta TheAttackingPlayer = []
  nounDelta You = []
  nounDelta (PlayerGroup _) = []
  nounDelta (Described d p {ph}) = detDelta d ph p
  nounDelta (EachOf grp) = nounDelta grp
  nounDelta (Both l r) = nounDelta r ++ nounDelta l
  nounDelta (EitherOf l r) = nounDelta l ++ nounDelta r
  nounDelta (LibrarySlice pos amt whose) =
    MkBinding TheD Object (outputPlur (nounPlur whose) (amtPlur amt))
              (ObjectP Nothing (Just Library) Nothing Nothing (amtExact amt))
      :: nounDelta whose
  nounDelta (NamesAgree _ grp) = nounDelta grp
  nounDelta (SomeOf q d grp) =
    MkBinding PartD Object (slicePlur q)
              (ObjectP (sliceTy d grp) (nounZone grp) Nothing Nothing (sliceExact q))
      :: (sliceCountDelta q ++ sliceDelta d ++ nounDelta grp)
  nounDelta (TheRest _ _) = []
  nounDelta (PileOf q Nothing) =
    MkBinding PartD Pile (slicePlur q)
              (PileP (zoneOfReach (Word PileW) ManyOf bs) (sliceExact q)
                     (faceOfReach (Word PileW) ManyOf bs))
      :: sliceCountDelta q
  nounDelta (PileOf q (Just by)) =
    MkBinding PartD Pile (slicePlur q)
              (PileP (zoneOfReach (Word PileW) ManyOf bs) (sliceExact q)
                     (faceOfReach (Word PileW) ManyOf bs))
      :: (sliceCountDelta q ++ nounDelta by)
  nounDelta (Pro _ _) = []
  nounDelta (ItOtherThan _ _) = []
  nounDelta (Own _ _ _) = []
  nounDelta (AttachHost _ _) = []
  nounDelta (PossessorOf _ n) =
    MkBinding TheD Player OneOf PlayerP :: (selfSubjDelta n ++ nounDelta n)
  nounDelta (PossessorsOf _ n) =
    MkBinding TheD Player ManyOf PlayerP :: (selfSubjDelta n ++ nounDelta n)
  nounDelta (Designated _ _) = []
  nounDelta (OneEachOf roles pool) =
    MkBinding BareD Object ManyOf
              (ObjectP (nounTy pool) (nounZone pool) Nothing Nothing Nothing)
      :: (predDeltaAll roles ++ nounDelta pool)

  public export
  elemPayload : {k : Kind} -> Phrasal k -> Bool -> Maybe CardType -> Maybe Zone ->
                Maybe Stamp -> Payload k
  elemPayload PhObject True _ _ _ = AbilityP Nothing
  elemPayload PhObject False ty zn pv = ObjectP ty zn pv Nothing (Just 1)
  elemPayload PhPlayer _ _ _ _ = PlayerP
  elemPayload {k = Quality q} PhQuality _ _ _ _ = QualityP
  elemPayload (PhJoin l r) ab ty zn pv =
    JoinP (elemPayload l ab ty zn pv) (elemPayload r ab ty zn pv)

  public export
  elemIntro : {bs : Bindings} -> {k : Kind} -> {auto ph : Phrasal k} ->
              Noun bs k -> Bindings
  elemIntro {k} grp =
    MkBinding TheD k OneOf
              (elemPayload ph (nounIsAbility grp) (nounTy grp) (nounZone grp)
                          (nounProv grp))
      :: nomIntro grp

  public export
  agentIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  agentIntro (Described EachDet p {ph}) = bindFor TheD OneOf ph p :: predDelta p ++ bs
  agentIntro n = nomIntro n

  public export
  agentCtx : {bs : Bindings} -> {k : Kind} -> Maybe (Noun bs k) -> Bindings
  agentCtx Nothing = bs
  agentCtx (Just n) = agentIntro n

  public export
  subjCtx : {bs : Bindings} -> {k : Kind} -> Maybe (Noun bs k) -> Bindings
  subjCtx Nothing = bs
  subjCtx (Just n) = selfSubjIntro n

  public export
  kindValueIntro : {bs : Bindings} -> QualitySort -> Maybe (Noun bs Object) -> Bindings
  kindValueIntro q (Just dom) = qualityB q :: nomIntro dom
  kindValueIntro {bs} q Nothing = qualityB q :: bs

  public export
  kindDomainOk : {0 bs : Bindings} -> KindAxis -> Maybe (Noun bs Object) -> Bool
  kindDomainOk _ (Just _) = True
  kindDomainOk ax Nothing = kindAxisClosed ax

  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (AbilityOf n) = nounDelta n
  predDelta (ActivatedBy n) = nounDelta n
  predDelta (Targets m _) = nounDelta m
  predDelta (HasPossessor _ n) = nounDelta n
  predDelta (CastBy n _) = nounDelta n
  predDelta (CombatRel _ m) = nounDelta m
  predDelta (CounterKindOn n) = nounDelta n
  predDelta (HappenedTo _ _ what) = complementDelta what
  predDelta (CastFrom z) = zoneDelta z
  predDelta (ColorIs _) = []
  predDelta IsColorless = []
  predDelta (ColorCount _ _) = []
  predDelta (HasSupertype _) = []
  predDelta (Named src) = nameSrcDelta src
  predDelta (HasDesignation _) = []
  predDelta (CoinCameUp _) = []
  predDelta (IsAttached _) = []
  predDelta (AttachedBy _ by) = nounDelta by
  predDelta (AttachedTo host) = nounDelta host
  predDelta (InPile p) = nounDelta p
  predDelta (InZone z) = zoneDelta z
  predDelta (And ps) = predDeltaAll ps
  predDelta (Not p) = []
  predDelta (Or ps) = []
  predDelta (OtherThan n) = nounDelta n
  predDelta (Compare _ _ b) = amtDelta b
  predDelta (Superlative _ _ d) = predDelta d
  predDelta WithMostVotes = []
  predDelta (ChoseExtreme _) = [outcomeB NamedNumber]
  predDelta (CompareOver dom _ _ bound) = gapB :: (predDelta dom ++ amtDelta bound)
  predDelta (Joined l r) = predDelta l ++ predDelta r
  predDelta _ = []

  public export
  predDeltaAll : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> List Binding
  predDeltaAll [] = []
  predDeltaAll (p :: ps) = predDelta p ++ predDeltaAll ps

  public export
  placeDelta : {bs : Bindings} -> LibPlace bs -> List Binding
  placeDelta (OneEnd _) = []
  placeDelta (EitherEnd Nothing) = []
  placeDelta (EitherEnd (Just n)) = nounDelta n
  placeDelta Shuffled = []

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (ZoneAt z (PossessedBy n)) = nounDelta n
  zoneDelta (ZoneAt z Bare) = []
  zoneDelta (LibraryAt pl _ _ (PossessedBy n)) = placeDelta pl ++ nounDelta n
  zoneDelta (LibraryAt pl _ _ Bare) = placeDelta pl

  public export
  zonesDistinct : List Zone -> Bool
  zonesDistinct [] = True
  zonesDistinct (z :: zs) = not (elem z zs) && zonesDistinct zs

  public export
  atLeastTwoZones : List Zone -> Bool
  atLeastTwoZones zs = case zs of
    (_ :: _ :: _) => zonesDistinct zs
    _ => False

  public export
  AtLeastTwoZones : List Zone -> Type
  AtLeastTwoZones zs = So (atLeastTwoZones zs)

  public export
  data SearchScope : Bindings -> Type where
    OneZone : (z : ZoneExpr bs) -> SearchScope bs
    SomeZones : (whose : Maybe (Noun bs Player)) -> (zs : List Zone) ->
                {auto 0 tw : AtLeastTwoZones zs} -> SearchScope bs

  public export
  searchZone : {0 bs : Bindings} -> SearchScope bs -> Maybe Zone
  searchZone (OneZone z) = Just (zoneSort z)
  searchZone (SomeZones _ _) = Nothing

  public export
  searchDelta : {bs : Bindings} -> SearchScope bs -> List Binding
  searchDelta (OneZone z) = zoneDelta z
  searchDelta (SomeZones Nothing _) = []
  searchDelta (SomeZones (Just whose) _) = nounDelta whose

  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro n = nounDelta n ++ bs

  public export
  chosenDet : {0 k : Kind} -> Phrasal k -> Determiner -> Determiner
  chosenDet PhObject _ = PartD
  chosenDet _ d = d

  public export
  chosenBind : Determiner -> Plurality -> {k : Kind} ->
               Phrasal k -> Predicate bs k -> Binding
  chosenBind det plur PhPlayer p = MkBinding det Player plur ChosenPlayerP
  chosenBind det plur ph p = bindFor det plur ph p

  public export
  chosenDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  chosenDelta (Described (ADet m) p {ph}) = chosenBind (chosenDet ph AD) OneOf ph p :: predDelta p
  chosenDelta (Described (CountDet q _) p {ph}) =
    chosenBind (chosenDet ph CountD) (quantPlur q) ph p :: (quantDelta q ++ predDelta p)
  chosenDelta (NamesAgree _ grp) = chosenDelta grp
  chosenDelta n = nounDelta n

  public export
  chosenIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  chosenIntro n = chosenDelta n ++ bs

  public export
  complementDelta : {bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                    Maybe (EventComplement bs ev ks) -> List Binding
  complementDelta Nothing = []
  complementDelta (Just (Involving what)) = nounDelta what
  complementDelta (Just (FromZones src what)) =
    sourceDelta src ++ complementDelta what
  complementDelta (Just (IntoZone to what)) =
    zoneDelta to ++ complementDelta what
  complementDelta (Just (AtZone z)) = zoneDelta z

  public export
  zonesDelta : {bs : Bindings} -> List (ZoneExpr bs) -> List Binding
  zonesDelta [] = []
  zonesDelta (z :: zs) = zoneDelta z ++ zonesDelta zs

  public export
  data ColorTerm : Bindings -> Type where
    LitColor : Chroma.Color -> ColorTerm bs
    ThatColor : {auto 0 ok : countChoice (QSort Color) bs = 1} ->
                {auto 0 rd : ChosenQualityRead Color} -> ColorTerm bs
    TheLastChosenColor : {auto 0 ok : ChoiceStands (countChoice (QSort Color) bs)} ->
                         {auto 0 rd : ChosenQualityRead Color} ->
                         ColorTerm bs

  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PlayerStatOf : (w : PlayerStat) -> (n : Noun bs Player) ->
                   {auto 0 one : nounPlur n = OneOf} -> Amount bs
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    CountOf : {k : Kind} -> (grp : Noun bs k) ->
              {auto 0 pl : nounPlur grp = ManyOf} ->
              {auto 0 cg : CountableGroup grp} ->
              Amount bs
    Aggregate : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                (grp : Noun bs k) ->
                {auto 0 sc : projScope ax = k} ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                Amount bs
    CountersOn : {k : Kind} -> (kind : CounterKind) -> (holder : Noun bs k) ->
                 {auto 0 sc : counterScope kind = k} ->
                 {auto 0 one : nounPlur holder = OneOf} -> Amount bs
    Paid : (f : PaidFacet) -> (n : Noun bs Object) ->
           {auto 0 nf : PaidFacetNamed f} ->
           {auto 0 sb : PaidSubject n} ->
           {auto 0 one : nounPlur n = OneOf} -> Amount bs
    EventTally : {k : Kind} -> (op : TallyOp) -> (ev : EventName) ->
                 (who : Noun bs k) -> (w : Lookback) ->
                 (what :
                    Maybe (EventComplement (nomIntro who) ev k)) ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} ->
                 {auto 0 qm : So (tallyOk op ev)} -> Amount bs
    TimesOf : (per : Amount bs) -> (a : Amount (amtIntro per)) ->
              {auto 0 nz : So (amtNonZero per)} -> Amount bs
    ThatMuch : {auto 0 ok : countQuantOutcomes bs = 1} -> Amount bs
    ChosenNumber : {auto 0 ok : countChoice (QSort Number) bs = 1} ->
                   Amount bs
    TheLastChosenNumber : {auto 0 ok : ChoiceStands (countChoice (QSort Number) bs)} ->
                          Amount bs
    VotesFor : (l : VoteLabel) -> Amount bs
    TheOutcome : (s : OutcomeSort) -> {auto 0 ok : countOutcomes s bs = 1} ->
                 Amount bs
    CoinsShowing : (face : CoinFace) ->
                   {auto 0 fl : So (coinFlipInScope bs)} -> Amount bs
    GreatestStoredMatch : (n : Noun bs Object) ->
                          {auto 0 one : nounPlur n = OneOf} -> Amount bs
    GroupSize : {auto 0 ok : countManysAny bs = 1} -> Amount bs
    TheDifference : {auto 0 ok : countOnes Gap bs = 1} -> Amount bs
    LetterVal : (l : Letter) -> Amount bs
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    Minus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    Devotion : (who : Noun bs Player) -> (c : ColorTerm bs) ->
               (d : Maybe (ColorTerm bs)) ->
               {auto 0 one : nounPlur who = OneOf} -> Amount bs
    Half : (r : RoundMode) -> (a : Amount bs) -> Amount bs
    DifferenceBetween : (a : Amount bs) -> (b : Amount (amtIntro a)) ->
                        Amount bs
    AggregateOver : {k : Kind} -> (op : AggregateOp) ->
                    (dom : Predicate bs k) -> {auto ph : Phrasal k} ->
                    (body : Amount (bindFor TheD OneOf ph dom
                                      :: (predDelta dom ++ bs))) ->
                    Amount bs
    DistinctCount : (ax : KindAxis) -> (dom : Noun bs Object) -> Amount bs
    UpTo : (bound : Amount bs) -> Amount bs

  public export
  amtNonZero : {bs : Bindings} -> Amount bs -> Bool
  amtNonZero (Lit Z) = False
  amtNonZero _ = True

  public export
  amtDelta : {bs : Bindings} -> Amount bs -> List Binding
  amtDelta (Lit _) = []
  amtDelta (StatOf _ nom) = nounDelta nom
  amtDelta (PlayerStatOf _ nom) = nounDelta nom
  amtDelta (CountersOn _ holder) = nounDelta holder
  amtDelta (Paid _ n) = nounDelta n
  amtDelta (EventTally _ _ who _ what) = nounDelta who ++ complementDelta what
  amtDelta (CountOf grp) = nounDelta grp
  amtDelta (Aggregate _ _ grp) = nounDelta grp
  amtDelta (TimesOf per a) = amtDelta per ++ amtDelta a
  amtDelta ThatMuch = []
  amtDelta ChosenNumber = []
  amtDelta TheLastChosenNumber = []
  amtDelta (VotesFor _) = []
  amtDelta (TheOutcome _) = []
  amtDelta (CoinsShowing _) = []
  amtDelta (GreatestStoredMatch _) = []
  amtDelta GroupSize = []
  amtDelta TheDifference = []
  amtDelta (LetterVal l) = letterDelta l bs
  amtDelta (Plus a b) = amtDelta a ++ amtDelta b
  amtDelta (Minus a b) = amtDelta a ++ amtDelta b
  amtDelta (Devotion who _ _) = nounDelta who
  amtDelta (Half _ a) = amtDelta a
  amtDelta (DifferenceBetween a b) = amtDelta a ++ amtDelta b
  amtDelta (AggregateOver _ dom _) = predDelta dom
  amtDelta (DistinctCount _ dom) = nounDelta dom
  amtDelta (UpTo b) = outcomeB CeilingShortfall :: amtDelta b

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (PlayerStatOf w nom) = nomIntro nom
  amtIntro (CountersOn _ holder) = nomIntro holder
  amtIntro (Paid _ n) = nomIntro n
  amtIntro (EventTally _ _ who _ what) = complementDelta what ++ nomIntro who
  amtIntro (CountOf grp) = nomIntro grp
  amtIntro (Aggregate _ _ grp) = nomIntro grp
  amtIntro (TimesOf per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro ChosenNumber = bs
  amtIntro TheLastChosenNumber = bs
  amtIntro (VotesFor _) = bs
  amtIntro (TheOutcome _) = bs
  amtIntro (CoinsShowing _) = bs
  amtIntro (GreatestStoredMatch _) = bs
  amtIntro GroupSize = bs
  amtIntro TheDifference = bs
  amtIntro (LetterVal l) = letterDelta l bs ++ bs
  amtIntro (Plus a b) = amtIntro b
  amtIntro (Minus a b) = amtIntro b
  amtIntro (Devotion who _ _) = nomIntro who
  amtIntro (Half _ a) = amtIntro a
  amtIntro (DifferenceBetween a b) = amtIntro b
  amtIntro (AggregateOver _ dom _) = predDelta dom ++ bs
  amtIntro (DistinctCount _ dom) = nomIntro dom
  amtIntro (UpTo b) = outcomeB CeilingShortfall :: amtIntro b

  public export
  optAmtIntro : {bs : Bindings} -> Maybe (Amount bs) -> Bindings
  optAmtIntro Nothing = bs
  optAmtIntro (Just a) = amtIntro a

  public export
  amtPlur : {0 bs : Bindings} -> Amount bs -> Plurality
  amtPlur (Lit (S Z)) = OneOf
  amtPlur (Lit _) = ManyOf
  amtPlur (StatOf _ _) = ManyOf
  amtPlur (PlayerStatOf _ _) = ManyOf
  amtPlur (CountersOn _ _) = ManyOf
  amtPlur (Paid _ _) = ManyOf
  amtPlur (EventTally _ _ _ _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Aggregate _ _ _) = ManyOf
  amtPlur (TimesOf _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur ChosenNumber = ManyOf
  amtPlur TheLastChosenNumber = ManyOf
  amtPlur (VotesFor _) = ManyOf
  amtPlur (TheOutcome _) = ManyOf
  amtPlur (CoinsShowing _) = ManyOf
  amtPlur (GreatestStoredMatch _) = ManyOf
  amtPlur GroupSize = ManyOf
  amtPlur TheDifference = ManyOf
  amtPlur (LetterVal _) = ManyOf
  amtPlur (Plus _ _) = ManyOf
  amtPlur (Minus _ _) = ManyOf
  amtPlur (Devotion _ _ _) = ManyOf
  amtPlur (Half _ _) = ManyOf
  amtPlur (DifferenceBetween _ _) = ManyOf
  amtPlur (AggregateOver _ _ _) = ManyOf
  amtPlur (DistinctCount _ _) = ManyOf
  amtPlur (UpTo b) = amtPlur b

  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq (LetterVal a) (LetterVal b) = a == b
  boundEq _ _ = False

  public export
  data LifeOp : Bindings -> Type where
    Up : Amount bs -> LifeOp bs
    Down : Amount bs -> LifeOp bs
    Set : Amount bs -> LifeOp bs

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a
  lifeIntro (Set a) = amtIntro a

  public export
  data FlipScope : Bindings -> Type where
    FlipCount : (n : Amount bs) -> FlipScope bs
    FlipPer : {k : Kind} -> (each : Noun bs k) ->
              {auto 0 pl : nounPlur each = ManyOf} ->
              {auto 0 rk : So (kindLte k (Object \/ Player))} -> FlipScope bs

  public export
  flipScopeIntro : {bs : Bindings} -> FlipScope bs -> Bindings
  flipScopeIntro (FlipCount n) = amtIntro n
  flipScopeIntro (FlipPer each) = nomIntro each

  public export
  data IgnoredOutcomes : Bindings -> Type where
    IgnoreExtreme : RollExtreme -> IgnoredOutcomes bs
    IgnoreAllBut : RollExtreme -> IgnoredOutcomes bs
    IgnoreChosen : (chooser : Maybe (Noun bs Player)) -> (n : Amount bs) ->
                   {auto 0 ag : EventAgent chooser} -> IgnoredOutcomes bs

  public export
  ignoredOutcomesIntro : {bs : Bindings} -> IgnoredOutcomes bs -> Bindings
  ignoredOutcomesIntro (IgnoreExtreme _) = bs
  ignoredOutcomesIntro (IgnoreAllBut _) = bs
  ignoredOutcomesIntro (IgnoreChosen _ n) = amtIntro n

  public export
  ignorableFor : {bs : Bindings} -> IgnoredOutcomes bs -> Bool
  ignorableFor (IgnoreExtreme _) = countOutcomes RollResult bs == 1
  ignorableFor (IgnoreAllBut _) = countOutcomes RollResult bs == 1
  ignorableFor (IgnoreChosen _ _) = ignorableInScope bs

  public export
  readAmount : {0 bs : Bindings} -> Amount bs -> Bool
  readAmount (Lit _) = False
  readAmount (StatOf _ _) = True
  readAmount (PlayerStatOf _ _) = True
  readAmount (CountersOn _ _) = True
  readAmount (Paid _ _) = True
  readAmount (EventTally _ _ _ _ _) = True
  readAmount (CountOf _) = True
  readAmount (Aggregate _ _ _) = True
  readAmount (TimesOf _ _) = False
  readAmount ThatMuch = False
  readAmount ChosenNumber = False
  readAmount TheLastChosenNumber = False
  readAmount (VotesFor _) = True
  readAmount (TheOutcome s) = outcomeComparable s
  readAmount (CoinsShowing _) = True
  readAmount (GreatestStoredMatch _) = True
  readAmount GroupSize = False
  readAmount TheDifference = True
  readAmount (LetterVal _) = True
  readAmount (Plus _ _) = False
  readAmount (Minus _ _) = False
  readAmount (Devotion _ _ _) = True
  readAmount (Half _ _) = False
  readAmount (DifferenceBetween _ _) = False
  readAmount (AggregateOver _ _ _) = True
  readAmount (DistinctCount _ _) = True
  readAmount (UpTo _) = False

  public export
  ReadAmount : Amount bs -> Type
  ReadAmount {bs} a = So (readAmount a)

  public export
  data Quantity : Bindings -> Type where
    Range : Maybe Nat -> Maybe Nat -> Quantity bs
    UpToOf : (a : Amount bs) -> Quantity bs
    ExactlyOf : (a : Amount bs) -> Quantity bs

  public export
  data NonZeroQ : Quantity bs -> Type where
    UnboundedAbove : NonZeroQ (Range lo Nothing)
    MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))
    AmountCeiling : NonZeroQ (UpToOf a)
    AmountExact : NonZeroQ (ExactlyOf a)

  public export
  quantWellFormed : {0 bs : Bindings} -> Quantity bs -> Bool
  quantWellFormed (Range Nothing _) = True
  quantWellFormed (Range (Just _) Nothing) = True
  quantWellFormed (Range (Just lo) (Just hi)) = lte lo hi
  quantWellFormed (UpToOf _) = True
  quantWellFormed (ExactlyOf _) = True

  public export
  WellFormedQ : Quantity bs -> Type
  WellFormedQ q = So (quantWellFormed q)

  public export
  OptWellFormedQ : Maybe (Quantity bs) -> Type
  OptWellFormedQ = OptOk WellFormedQ

  public export
  data SliceCount : Bindings -> Type where
    CountedSlice : (q : Quantity bs) ->
                   {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} -> SliceCount bs
    WholeSlice : SliceCount bs

  public export
  amtExact : {0 bs : Bindings} -> Amount bs -> Maybe Nat
  amtExact (Lit n) = Just n
  amtExact _ = Nothing

  public export
  quantExact : {0 bs : Bindings} -> Quantity bs -> Maybe Nat
  quantExact (Range (Just lo) (Just hi)) = if lo == hi then Just lo else Nothing
  quantExact (Range _ _) = Nothing
  quantExact (UpToOf _) = Nothing
  quantExact (ExactlyOf a) = amtExact a

  public export
  quantPlur : {0 bs : Bindings} -> Quantity bs -> Plurality
  quantPlur (Range _ (Just (S Z))) = OneOf
  quantPlur (Range _ _) = ManyOf
  quantPlur (UpToOf _) = ManyOf
  quantPlur (ExactlyOf _) = ManyOf

  public export
  detQuant : {0 bs : Bindings} -> DetPhrase bs -> Maybe (Quantity bs)
  detQuant (TargetDet q) = Just q
  detQuant (CountDet q _) = Just q
  detQuant _ = Nothing

  public export
  modesFit : {0 bs : Bindings} -> Quantity bs -> Nat -> Bool
  modesFit (Range Nothing Nothing) n = True
  modesFit (Range Nothing (Just hi)) n = lte hi n
  modesFit (Range (Just lo) Nothing) n = lte lo n
  modesFit (Range (Just lo) (Just hi)) n = lte hi n
  modesFit (UpToOf _) n = True
  modesFit (ExactlyOf _) n = True

  public export
  ModesFit : Quantity bs -> Nat -> Type
  ModesFit q n = So (modesFit q n)

  public export
  quantLiteral : {0 bs : Bindings} -> Quantity bs -> Bool
  quantLiteral (Range _ _) = True
  quantLiteral (UpToOf _) = False
  quantLiteral (ExactlyOf _) = False

  public export
  quantDelta : {bs : Bindings} -> Quantity bs -> List Binding
  quantDelta (Range _ _) = []
  quantDelta (UpToOf a) = amtDelta a
  quantDelta (ExactlyOf a) = amtDelta a

  public export
  sliceExact : {0 bs : Bindings} -> SliceCount bs -> Maybe Nat
  sliceExact (CountedSlice q) = quantExact q
  sliceExact WholeSlice = Nothing

  public export
  slicePlur : {0 bs : Bindings} -> SliceCount bs -> Plurality
  slicePlur (CountedSlice q) = quantPlur q
  slicePlur WholeSlice = ManyOf

  public export
  sliceCountDelta : {bs : Bindings} -> SliceCount bs -> List Binding
  sliceCountDelta (CountedSlice q) = quantDelta q
  sliceCountDelta WholeSlice = []

  public export
  quantIntro : {bs : Bindings} -> Quantity bs -> Bindings
  quantIntro (Range _ _) = bs
  quantIntro (UpToOf a) = amtIntro a
  quantIntro (ExactlyOf a) = amtIntro a

  public export
  optQuantIntro : {bs : Bindings} -> Maybe (Quantity bs) -> Bindings
  optQuantIntro Nothing = bs
  optQuantIntro (Just q) = quantIntro q

  public export
  Bindingless : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Bindingless {bs} {k} n = nounDelta n = []

  public export
  testSubjectOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  testSubjectOk (Described TheDet _) = True
  testSubjectOk (LibrarySlice _ _ _) = True
  testSubjectOk n = case nounDelta n of
                      [] => True
                      _ => False

  public export
  TestSubject : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  TestSubject {bs} {k} n = So (testSubjectOk n)

  public export
  nounDet : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Maybe Determiner
  nounDet (Described d _) = Just (detOf d)
  nounDet (EachOf _) = Just EachD
  nounDet (LibrarySlice _ _ _) = Just TheD
  nounDet (SomeOf _ _ _) = Just PartD
  nounDet (NamesAgree _ grp) = nounDet grp
  nounDet (TheRest _ _) = Just TheD
  nounDet (PileOf _ _) = Just PartD
  nounDet (OneEachOf _ _) = Just BareD
  nounDet _ = Nothing

  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase (EitherOf l r) = anchorPhrase l && anchorPhrase r
  anchorPhrase (Both _ _) = False
  anchorPhrase n = maybe True (== TargetD) (nounDet n)

  public export
  data ComplementAnchor : Noun bs k -> Type where
    MkComplementAnchor : {auto 0 sh : So (anchorPhrase n)} ->
                         {auto 0 one : nounPlur n = OneOf} ->
                         ComplementAnchor n

  public export
  data LinkSource : Noun bs k -> Type where
    SelfLinked : LinkSource This
    SortedSelfLinked : {0 t : CardType} -> {0 sub : Maybe Subtype} ->
                       {0 way : So (ascriptionOk t sub)} ->
                       LinkSource (AsType t This sub {asc = Oh} {way})

  public export
  data PileMention : Noun bs Pile -> Type where
    PilePartitive : {0 q : SliceCount bs} ->
                    {0 by : Maybe (Noun bs Player)} ->
                    {0 ok : countReach (Word PileW) ManyOf bs = 1} ->
                    PileMention (PileOf q by {ok})
    ThatPile : {0 ok : countReach (Word PileW) OneOf bs = 1} ->
               PileMention {bs} (Pro (Word PileW) OneOf {ok})
    ThosePiles : {0 ok : countReach (Word PileW) ManyOf bs = 1} ->
                 PileMention {bs} (Pro (Word PileW) ManyOf {ok})

  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable n = elem (nounDet n) [Just AD, Just TargetD, Just CountD]

  public export
  Choosable : Noun bs k -> Type
  Choosable {bs} {k} n = So (choosable n)

  public export
  agentChoosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  agentChoosable (SomeOf _ _ _) = True
  agentChoosable (PileOf _ _) = True
  agentChoosable n = choosable n

  public export
  data ChoiceClause : {0 bs : Bindings} -> {0 k : Kind} ->
                      Maybe (Noun bs Player) -> Noun bs k -> Type where
    BareChoice : {0 n : Noun bs k} ->
                 {auto 0 ok : So (choosable n)} -> ChoiceClause Nothing n
    AgentChoice : {0 by : Noun bs Player} -> {0 n : Noun bs k} ->
                  {auto 0 ok : So (agentChoosable n)} ->
                  ChoiceClause (Just by) n

  public export
  data Ballot : Bindings -> Type where
    ByLabel : (opts : List VoteLabel) ->
              {auto 0 ok : BallotLabelsOk opts} -> Ballot bs
    ByCandidate : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 ok : So (choosable n)} -> Ballot bs

  public export
  groupMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  groupMention (LibrarySlice _ _ _) = True
  groupMention (Pro _ pl) = not (isOne pl)
  groupMention (Both _ _) = True
  groupMention (OneEachOf _ _) = True
  groupMention n = nounDet n == Just TargetD

  public export
  GroupMention : Noun bs k -> Type
  GroupMention {bs} {k} n = So (groupMention n)

  public export
  partitiveBase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  partitiveBase n = nounDet n == Just AllD || groupMention n

  public export
  PartitiveBase : Noun bs k -> Type
  PartitiveBase {bs} {k} n = So (partitiveBase n)

  public export
  countableGroup : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countableGroup n = nounDet n == Just BareD || groupMention n

  public export
  CountableGroup : Noun bs k -> Type
  CountableGroup {bs} {k} n = So (countableGroup n)

  public export
  countedMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedMention (NamesAgree _ _) = False
  countedMention n = elem (nounDet n) [Just CountD, Just TargetD]

  public export
  CountedMention : Noun bs k -> Type
  CountedMention {bs} {k} n = So (countedMention n)

  public export
  countedExistential : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedExistential (NamesAgree _ grp) = countedMention grp
  countedExistential _ = False

  public export
  CountedExistential : Noun bs k -> Type
  CountedExistential {bs} {k} n = So (countedExistential n)

  public export
  existentialMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  existentialMention n = nounDet n == Just BareD || countedExistential n

  public export
  ExistentialMention : Noun bs k -> Type
  ExistentialMention {bs} {k} n = So (existentialMention n)

  public export
  perMemberOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  perMemberOk n = elem (nounDet n) [Just EachD, Just AllD] || isOne (nounPlur n)

  public export
  PerMember : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  PerMember {bs} {k} n = So (perMemberOk n)


  public export
  hostedRead : {bs : Bindings} -> {k : Kind} ->
               Predicate bs Object -> Noun bs k -> Bool
  hostedRead p n = case qualityReadHost p of
                     Nothing => True
                     Just h => tyIs h (nounTy n)

  public export
  HostedRead : {bs : Bindings} -> {k : Kind} ->
               Predicate bs Object -> Noun bs k -> Type
  HostedRead {bs} {k} p n = So (hostedRead p n)


  public export
  soleHolderOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  soleHolderOk (PlayerGroup _) = True
  soleHolderOk n = isOne (nounPlur n)

  public export
  SoleHolder : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SoleHolder {bs} {k} n = So (soleHolderOk n)

  public export
  slicePossessorOk : {bs : Bindings} -> Noun bs Player -> Bool
  slicePossessorOk (Described EachDet _) = True
  slicePossessorOk (EachOf _) = True
  slicePossessorOk (PlayerGroup _) = True
  slicePossessorOk n = isOne (nounPlur n)

  public export
  SlicePossessor : {bs : Bindings} -> Noun bs Player -> Type
  SlicePossessor {bs} n = So (slicePossessorOk n)

  public export
  costSubjectOk : {bs : Bindings} -> Noun bs Object -> Bool
  costSubjectOk This = True
  costSubjectOk n = onStackZone (nounZone n)

  public export
  data CostSubject : {0 k : Kind} -> Noun bs k -> Type where
    MkCostSubject : {0 n : Noun bs Object} ->
                    {auto 0 ok : So (costSubjectOk n)} -> CostSubject n

  public export
  paidSubjectOk : {bs : Bindings} -> Noun bs Object -> Bool
  paidSubjectOk This = True
  paidSubjectOk (AsType _ n _) = paidSubjectOk n
  paidSubjectOk (AsMarker _ n) = paidSubjectOk n
  paidSubjectOk n = onStackZone (nounZone n)

  public export
  data PaidSubject : Noun bs Object -> Type where
    PaymentHappened : {0 n : Noun bs Object} ->
                      {auto 0 ok : So (paidSubjectOk n)} -> PaidSubject n

  public export
  counterKind : Kind -> Bool
  counterKind Object = True
  counterKind (a \/ b) = counterKind a && counterKind b
  counterKind _ = False

  public export
  controlKind : Kind -> Bool
  controlKind Object = True
  controlKind (a \/ b) = controlKind a && controlKind b
  controlKind _ = False

  public export
  possessorKind : PossessorAxis -> Kind -> Bool
  possessorKind ControllerAx k = controlKind k
  possessorKind OwnerAx k = kindLte k Object

  public export
  copyKind : Kind -> Bool
  copyKind Object = True
  copyKind (a \/ b) = copyKind a && copyKind b
  copyKind _ = False

  public export
  data StackActOn : (Kind -> Bool) -> {0 k : Kind} -> Noun bs k -> Type where
    StackSpell : {0 n : Noun bs Object} ->
                 {auto 0 zn : ZoneIs (nounZone n) Stack} -> StackActOn p n
    StackJoin : {0 ka : Kind} -> {0 kb : Kind} ->
                {0 n : Noun bs (ka \/ kb)} ->
                {auto 0 ok : So (p (ka \/ kb))} -> StackActOn p n

  public export
  Counterable : {0 k : Kind} -> Noun bs k -> Type
  Counterable = StackActOn counterKind

  public export
  Copiable : {0 k : Kind} -> Noun bs k -> Type
  Copiable = StackActOn copyKind

  public export
  CopySourceOk : {bs : Bindings} -> {k : Kind} -> CopySort -> Noun bs k -> Type
  CopySourceOk FromStack n = Copiable n
  CopySourceOk FromCardZone n = So (isCardZone (nounZone n))

  public export
  selfDefinedOk : {bs : Bindings} -> Noun bs Object -> Bool
  selfDefinedOk This = True
  selfDefinedOk (AsType _ n _) = selfDefinedOk n
  selfDefinedOk _ = False

  public export
  SelfDefined : {bs : Bindings} -> Noun bs Object -> Type
  SelfDefined {bs} n = So (selfDefinedOk n)

  public export
  data Condition : Bindings -> Type where
    Exists : {k : Kind} -> (n : Noun bs k) ->
             {auto 0 ex : ExistentialMention n} -> Condition bs
    Happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               (what :
                  Maybe (EventComplement (nomIntro who) ev k)) ->
               {auto 0 cw : ComplementWritten what} ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
    GameIs : (d : Designation) ->
             {auto 0 sc : designationScope d = HeldByGame} ->
             {auto 0 at : So (designationChecked d)} -> Condition bs
    NoHolder : (d : Designation) ->
               {auto 0 sc : designationScope d = HeldBy Player} ->
               {auto 0 at : So (designationChecked d)} -> Condition bs
    Matches : {k : Kind} -> (n : Noun bs k) -> (p : Predicate bs k) ->
              {auto 0 bl : TestSubject n} ->
              {auto 0 sy : PredSays p} ->
              {auto 0 zc : ZoneFits (nounZone n) (seedZone p)} ->
              Condition bs
    CompareAmt : (subj : Amount bs) -> (r : Comparator) ->
                 (bound : Amount (amtIntro subj)) ->
                 {auto 0 rd : ReadAmount subj} ->
                 Condition bs
    DealtThisWay : {k : Kind} -> (p : Predicate bs k) ->
                   {auto 0 wy : So (damageDealtInScope bs)} ->
                   {auto 0 rk : So (kindLte k (Object \/ Player))} ->
                   {auto 0 nz : So (not (zoneIsB (seedZone p) Stack))} ->
                   Condition bs
    PreventedFromSource : (p : Predicate bs Object) ->
                          {auto 0 ok : countOutcomes DamagePrevented bs = 1} ->
                          Condition bs
    FlipCalled : (who : Noun bs Player) -> (call : FlipCall) ->
                 {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
    FlipFace : (face : CoinFace) ->
               {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
    VoteLead : (l : VoteLabel) -> (orTied : Bool) -> Condition bs
    AnyResultIs : (r : Comparator) -> (bound : Amount bs) ->
                  {auto 0 ok : countOutcomes RollResult bs = 1} ->
                  Condition bs
    RolledDoubles : {auto 0 ok : countOutcomes RollResult bs = 1} ->
                    Condition bs
    NotCond : (c : Condition bs) -> Condition bs
    AndCond : (cs : List (Condition bs)) ->
              {auto 0 tw : AtLeastTwoArms cs} ->
              {auto 0 fl : FlatConjuncts cs} -> Condition bs
    OrCond : (cs : List (Condition bs)) ->
             {auto 0 tw : AtLeastTwoArms cs} ->
             {auto 0 fd : FlatDisjuncts cs} -> Condition bs

  public export
  atLeastTwoCs : {0 bs : Bindings} -> List (Condition bs) -> Bool
  atLeastTwoCs [] = False
  atLeastTwoCs (_ :: []) = False
  atLeastTwoCs (_ :: _ :: _) = True

  public export
  AtLeastTwoArms : List (Condition bs) -> Type
  AtLeastTwoArms {bs} cs = So (atLeastTwoCs cs)

  public export
  isAndCond : {0 bs : Bindings} -> Condition bs -> Bool
  isAndCond (AndCond _) = True
  isAndCond _ = False

  public export
  isOrCond : {0 bs : Bindings} -> Condition bs -> Bool
  isOrCond (OrCond _) = True
  isOrCond _ = False

  public export
  flatConjuncts : {0 bs : Bindings} -> List (Condition bs) -> Bool
  flatConjuncts [] = True
  flatConjuncts (c :: cs) = not (isAndCond c) && flatConjuncts cs

  public export
  FlatConjuncts : List (Condition bs) -> Type
  FlatConjuncts {bs} cs = So (flatConjuncts cs)

  public export
  flatDisjuncts : {0 bs : Bindings} -> List (Condition bs) -> Bool
  flatDisjuncts [] = True
  flatDisjuncts (c :: cs) = not (isOrCond c) && flatDisjuncts cs

  public export
  FlatDisjuncts : List (Condition bs) -> Type
  FlatDisjuncts {bs} cs = So (flatDisjuncts cs)

  public export
  condNegated : {0 bs : Bindings} -> Condition bs -> Bool
  condNegated (NotCond _) = True
  condNegated _ = False

  public export
  markingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Bool
  markingOk AsLongAs _ = True
  markingOk IfSo _ = True
  markingOk Unless c = condNegated c

  public export
  data MarkingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Type where
    MkMarkingOk : {0 c : Condition bs} ->
                  {auto 0 ok : So (markingOk m c)} -> MarkingOk m c

  public export
  condDelta : {bs : Bindings} -> Condition bs -> List Binding
  condDelta (Exists _) = []
  condDelta (Happened _ who _ _) = selfSubjDelta who
  condDelta (GameIs _) = []
  condDelta (NoHolder _) = []
  condDelta (Matches n _) = nounDelta n ++ selfSubjDelta n
  condDelta (CompareAmt subj _ bound) =
    gapB :: (amtDelta bound ++ amtDelta subj)
  condDelta (DealtThisWay _) = []
  condDelta (PreventedFromSource _) = []
  condDelta (FlipCalled _ _) = []
  condDelta (FlipFace _) = []
  condDelta (VoteLead _ _) = []
  condDelta (AnyResultIs _ _) = []
  condDelta RolledDoubles = []
  condDelta (NotCond (CompareAmt subj _ bound)) = amtDelta bound ++ amtDelta subj
  condDelta (NotCond c) = dropGaps (condDelta c)
  condDelta (AndCond cs) = condDeltaAll cs
  condDelta (OrCond _) = []

  public export
  condDeltaAll : {bs : Bindings} -> List (Condition bs) -> List Binding
  condDeltaAll [] = []
  condDeltaAll (c :: cs) = condDelta c ++ condDeltaAll cs

  public export
  dropGaps : List Binding -> List Binding
  dropGaps [] = []
  dropGaps (MkBinding _ Gap _ GapP :: bs) = dropGaps bs
  dropGaps (b :: bs) = b :: dropGaps bs

  public export
  attackableKind : (k : Kind) -> HeadTy k -> Bool
  attackableKind Player _ = True
  attackableKind Object (SoleTy t) = deedAltOk "Attack" Patient (optCT t)
  attackableKind (a \/ b) (JoinTy l r) = attackableKind a l && attackableKind b r
  attackableKind (a \/ b) (SoleTy t) =
    attackableKind a (SoleTy t) && attackableKind b (SoleTy t)
  attackableKind _ _ = False

  public export
  Attackable : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Attackable {k} n = So (attackableKind k (nounTys n))

  public export
  combatRelOk : CombatRelation -> (k : Kind) -> (km : Kind) ->
                Maybe Zone -> HeadTy km -> Bool
  combatRelOk AttackerOf k km _ tys = k == Object && attackableKind km tys
  combatRelOk AttackedBy _ km z _ = km == Object && zoneIsB z Battlefield
  combatRelOk _ k km z _ =
    k == Object && km == Object && zoneIsB z Battlefield

  public export
  EventAgent : {bs : Bindings} -> Maybe (Noun bs Player) -> Type
  EventAgent m = OptOk (\w => nounDelta w = []) m

  public export
  data TokenPhrase : {0 bs : Bindings} -> Noun bs Object -> Type where
    CountedTokens : {0 q : Quantity bs} -> {0 m : Maybe (ChoiceMode bs)} ->
                    {0 p : Predicate bs Object} ->
                    {0 ph : Phrasal Object} -> {0 dk : detOk (CountDet q m) p} ->
                    {auto 0 ok : So (seedsToken p)} ->
                    TokenPhrase (Described (CountDet q m) p {ph} {ok = dk})
    OneToken : {0 m : ChoiceMode bs} -> {0 p : Predicate bs Object} ->
               {0 ph : Phrasal Object} -> {0 dk : detOk (ADet m) p} ->
               {auto 0 ok : So (seedsToken p)} ->
               TokenPhrase (Described (ADet m) p {ph} {ok = dk})

  public export
  selfSubjDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  selfSubjDelta (AsType t This _) =
    [MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing Nothing)]
  selfSubjDelta (AttachHost _ (TypeW t)) =
    [MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing Nothing)]
  selfSubjDelta (AttachHost _ PermanentW) =
    [MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing Nothing)]
  selfSubjDelta (AttachHost _ PlayerW) = [MkBinding TheD Player OneOf PlayerP]
  selfSubjDelta _ = []

  public export
  selfSubjIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  selfSubjIntro n = selfSubjDelta n ++ nomIntro n

  public export
  remarkTest : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k ->
               Maybe (Binding -> Bool)
  remarkTest (Pro Bare pl) = Just (reaches Bare pl)
  remarkTest (Pro (AtSlot sl) pl) = Just (reaches (AtSlot sl) pl)
  remarkTest (Pro (Stamped v) pl) = Just (reaches (Stamped v) pl)
  remarkTest (Pro TokenBorn pl) = Just (reaches TokenBorn pl)
  remarkTest (Pro (Word AbilityW) OneOf) = Just (reaches (Word AbilityW) OneOf)
  remarkTest (Pro _ _) = Nothing
  remarkTest (ItOtherThan _ _) = Nothing
  remarkTest (Own _ _ _) = Nothing
  remarkTest _ = Nothing

  public export
  condRemarkAt : {0 bs : Bindings} -> Condition bs ->
                 Maybe (Binding -> Bool, Maybe CardType)
  condRemarkAt (Matches n p) =
    case remarkTest n of
      Nothing => Nothing
      Just q => Just (q, seedTy p)
  condRemarkAt _ = Nothing

  public export
  condRemark : {bs : Bindings} -> Condition bs -> Bindings
  condRemark c = maybe bs (\qt => markFirst (fst qt) (snd qt) bs) (condRemarkAt c)

  public export
  condIntro : {bs : Bindings} -> Condition bs -> Bindings
  condIntro c = condDelta c ++ condRemark c

  public export
  interveningIntro : {bs : Bindings} -> Maybe (Condition bs) -> Bindings
  interveningIntro Nothing = bs
  interveningIntro (Just c) = condIntro c

  public export
  playSourceOk : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) ->
                 Bool -> Bool
  playSourceOk zn Nothing False = complementLocates zn
  playSourceOk zn (Just z) False =
    playableFrom (Just (zoneSort z)) &&
    (not (complementLocates zn) || zoneFits zn (Just (zoneSort z)))
  playSourceOk _ Nothing True = True
  playSourceOk _ (Just z) True = playableFrom (Just (zoneSort z))

  public export
  data Exposed : Bindings -> Type where
    ExposedCards : (n : Noun bs Object) -> Exposed bs
    ExposedZone : (z : ZoneExpr bs) ->
                  {auto 0 ok : ExposableZone (zoneSort z)} -> Exposed bs
    ExposedChoice : (q : ChoiceSort) ->
                    {auto 0 ok : countChoice q bs = 1} ->
                    Exposed bs

  public export
  exposedIntro : {bs : Bindings} -> Exposed bs -> Bindings
  exposedIntro (ExposedCards n) = nomIntro n
  exposedIntro (ExposedZone z) = zoneDelta z ++ bs
  exposedIntro (ExposedChoice _) = bs

  public export
  data VisibleThing : Bindings -> Type where
    TopOfLibrary : VisibleThing bs
    WholeHand : VisibleThing bs
    VisibleObjects : (n : Noun bs Object) -> VisibleThing bs

  public export
  visibleIntro : {bs : Bindings} -> VisibleThing bs -> Bindings
  visibleIntro TopOfLibrary = bs
  visibleIntro WholeHand = bs
  visibleIntro (VisibleObjects n) = nomIntro n

  public export
  visibilityOk : {0 bs : Bindings} -> ExposeVerb -> VisibleThing bs -> Bool
  visibilityOk Reveal TopOfLibrary = True
  visibilityOk Reveal WholeHand = True
  visibilityOk LookAt TopOfLibrary = True
  visibilityOk LookAt WholeHand = False
  visibilityOk _ (VisibleObjects _) = True

  public export
  VisibilityOk : {0 bs : Bindings} -> ExposeVerb -> VisibleThing bs -> Type
  VisibilityOk v w = So (visibilityOk v w)

  public export
  costNounOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  costNounOk (AsType _ n _) = costNounOk n
  costNounOk (ResolvedPermanent n) = costNounOk n
  costNounOk (AsMarker _ n) = costNounOk n
  costNounOk (EachOf grp) = costNounOk grp
  costNounOk (NamesAgree _ grp) = costNounOk grp
  costNounOk (SomeOf _ _ grp) = costNounOk grp
  costNounOk (EitherOf l r) = costNounOk l && costNounOk r
  costNounOk (Both _ _) = False
  costNounOk (Pro (Verbed _ _ _) _) = False
  costNounOk _ = True

  public export
  nounIsYou : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsYou You = True
  nounIsYou _ = False

  public export
  nounTargeted : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounTargeted (AsType _ n _) = nounTargeted n
  nounTargeted (ResolvedPermanent n) = nounTargeted n
  nounTargeted (AsMarker _ n) = nounTargeted n
  nounTargeted (EachOf grp) = nounTargeted grp
  nounTargeted (SomeOf _ _ grp) = nounTargeted grp
  nounTargeted (Both l r) = nounTargeted l || nounTargeted r
  nounTargeted (EitherOf l r) = nounTargeted l || nounTargeted r
  nounTargeted n = nounDet n == Just TargetD

  public export
  Nontarget : Noun bs k -> Type
  Nontarget {bs} {k} n = So (not (nounTargeted n))

  public export
  counterMemoryOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  counterMemoryOk (Pro (Verbed _ _ _) _) = False
  counterMemoryOk (Pro Bare pl) = not (stampMoves (provOfReach Bare pl bs))
  counterMemoryOk (Pro (AtSlot sl) pl) =
    not (stampMoves (provOfReach (AtSlot sl) pl bs))
  counterMemoryOk (Pro (Stamped v) pl) =
    not (stampMoves (provOfReach (Stamped v) pl bs))
  counterMemoryOk (Pro TokenBorn pl) =
    not (stampMoves (provOfReach TokenBorn pl bs))
  counterMemoryOk (Pro _ _) = True
  counterMemoryOk (ItOtherThan _ rest) =
    not (stampMoves (provOfReach Bare OneOf rest))
  counterMemoryOk (Own pl own _) =
    not (stampMoves (provOfReach Bare pl own))
  counterMemoryOk _ = True

  public export
  CounterMemory : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  CounterMemory {bs} n = So (counterMemoryOk n)

  public export
  moveDestOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  moveDestOk (Pro Bare _) = False
  moveDestOk (Pro (AtSlot _) _) = False
  moveDestOk (Pro (Stamped _) _) = False
  moveDestOk (Pro TokenBorn _) = False
  moveDestOk (Pro _ _) = True
  moveDestOk (ItOtherThan _ _) = False
  moveDestOk (Own _ _ _) = False
  moveDestOk _ = True

  public export
  MoveDestination : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  MoveDestination {bs} n = So (moveDestOk n)

  public export
  damageableKind : (k : Kind) -> HeadTy k -> Bool
  damageableKind Player _ = True
  damageableKind Object (SoleTy t) = damageableHeadTysOk (soleAlt (optCT t))
  damageableKind (a \/ b) (JoinTy l r) = damageableKind a l && damageableKind b r
  damageableKind (a \/ b) (SoleTy t) =
    damageableKind a (SoleTy t) && damageableKind b (SoleTy t)
  damageableKind _ _ = False

  public export
  data DamageRecipient : Noun bs k -> Type where
    PlayerTakes : DamageRecipient {k = Player} n
    JoinTakes : {auto 0 dm : So (damageableKind (ka \/ kb) (nounTys n))} ->
                DamageRecipient {k = ka \/ kb} n
    ObjectTakes : {auto 0 field : ZoneIs (nounZone n) Battlefield} ->
                  {auto 0 dm : So (damageableHeadTysOk (nounHeadTys n))} ->
                  DamageRecipient {k = Object} n

  public export
  SingleRecipient : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SingleRecipient {bs} {k} n = nounPlur n = OneOf

  ||| Only permanents have status [CR#110.5d]; a pile is not an object [CR#700.3b].
  public export
  data StatusHolder : Noun bs k -> Type where
    ObjectHoldsStatus : StatusHolder {k = Object} n

  public export
  data Movable : Noun bs k -> Type where
    ObjectMoves : Movable {k = Object} n
    PileMoves : Movable {k = Pile} n

  public export
  data DiscardOk : Noun bs Object -> Type where
    DiscardThis : DiscardOk This
    DiscardTracked : {auto 0 z : nounZone n = Just Hand} -> DiscardOk n

  public export
  setZone : Maybe VerbLabel -> Maybe Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty oldZn _ og sz)) =
    MkBinding det Object plur (ObjectP ty z (mkStamp p oldZn (not (oldZn == z))) og sz)
  setZone p z (MkBinding det Pile plur (PileP _ sz fc)) =
    MkBinding det Pile plur (PileP z sz fc)
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det Player plur ChosenPlayerP) =
    MkBinding det Player plur ChosenPlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP
  setZone p z (MkBinding det Outcome plur (OutcomeP s)) =
    MkBinding det Outcome plur (OutcomeP s)
  setZone p z (MkBinding det Gap plur GapP) = MkBinding det Gap plur GapP
  setZone p z (MkBinding det (LetterK l) plur LetterP) = MkBinding det (LetterK l) plur LetterP
  setZone p z (MkBinding det TurnRef plur TurnRefP) = MkBinding det TurnRef plur TurnRefP
  setZone p z (MkBinding det Object plur (AbilityP og)) =
    MkBinding det Object plur (AbilityP og)
  setZone p z (MkBinding det (a \/ b) plur (JoinP l r)) = MkBinding det (a \/ b) plur (JoinP l r)

  public export
  setZoneHead : Maybe VerbLabel -> Maybe Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  public export
  setZoneReach : Reach -> Plurality -> Maybe VerbLabel -> Maybe Zone ->
                 Bindings -> Bindings
  setZoneReach r pl p z [] = []
  setZoneReach r pl p z (b :: bs) =
    if reaches r pl b then setZone p z b :: bs
                       else b :: setZoneReach r pl p z bs

  public export
  stampIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbLabel -> Noun bs k -> Bindings
  stampIntro p n = moveIntro p n (nounZone n)

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbLabel -> Noun bs k -> Maybe Zone -> Bindings
  moveIntro p nn@(Described _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p (EachOf grp) z = moveIntro p grp z
  moveIntro p (NamesAgree _ grp) z = moveIntro p grp z
  moveIntro p nn@(Both _ _) z = nomIntro nn
  moveIntro p nn@(EitherOf _ _) z = nomIntro nn
  moveIntro p nn@(LibrarySlice _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(SomeOf _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(OneEachOf _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p (TheRest k _) z = groupSpent k bs
  moveIntro p nn@(PileOf _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p (Pro r pl) z = setZoneReach r pl p z bs
  moveIntro p (ItOtherThan co rest) z = co ++ setZoneReach Bare OneOf p z rest
  moveIntro p (Own pl own outer) z = setZoneReach Bare pl p z own ++ outer
  moveIntro p This z =
    MkBinding SelfD Object OneOf (ObjectP Nothing z (mkStamp p Nothing (isJust z)) Nothing Nothing) :: bs
  moveIntro p (AttachHost _ (TypeW t)) z =
    MkBinding TheD Object OneOf
              (ObjectP (Just t) z (mkStamp p (Just Battlefield)
                                            (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (AttachHost _ PermanentW) z =
    MkBinding TheD Object OneOf
              (ObjectP Nothing z (mkStamp p (Just Battlefield)
                                           (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (AttachHost _ PlayerW) z = MkBinding TheD Player OneOf PlayerP :: bs
  moveIntro p (AttachHost _ w) z = bs
  moveIntro p (AsType t This _) z =
    MkBinding SelfD Object OneOf
              (ObjectP (Just t) z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (AsType t n _) z =
    MkBinding TheD Object OneOf
              (ObjectP (Just t) z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (ResolvedPermanent n) z =
    MkBinding TheD Object (nounPlur n)
              (ObjectP (nounTy n) z (mkStamp p (Just Battlefield)
                                              (not (z == Just Battlefield)))
                       Nothing Nothing)
      :: bs
  moveIntro p (AsMarker _ This) z =
    MkBinding SelfD Object OneOf
              (ObjectP Nothing z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (AsMarker _ n) z =
    MkBinding TheD Object OneOf
              (ObjectP Nothing z (mkStamp p Nothing (not (z == Just Battlefield))) Nothing Nothing)
      :: bs
  moveIntro p (TheGrantor m) z =
    MkBinding TheD Object OneOf
              (ObjectP Nothing z (mkStamp p (grantorOrigin m) (not (z == Just (markerZone m))))
                       Nothing Nothing)
      :: bs
  moveIntro p You z = bs
  moveIntro p TheDefendingPlayer z = bs
  moveIntro p TheAttackingPlayer z = bs
  moveIntro p (PlayerGroup _) z = bs
  moveIntro p (PossessorOf ax n) z = nomIntro (PossessorOf ax n)
  moveIntro p (PossessorsOf ax n) z = nomIntro (PossessorsOf ax n)
  moveIntro p (Designated d n) z = bs

  public export
  nounProv : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Stamp
  nounProv (TheRest k _) = provOfGroup k bs
  nounProv (Pro r pl) = provOfReach r pl bs
  nounProv (ItOtherThan _ rest) = provOfReach Bare OneOf rest
  nounProv (Own pl own _) = provOfReach Bare pl own
  nounProv (EachOf grp) = nounProv grp
  nounProv (NamesAgree _ grp) = nounProv grp
  nounProv (SomeOf _ _ grp) = nounProv grp
  nounProv _ = Nothing

  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n _) = Just Battlefield
  nounZone (ResolvedPermanent _) = Just Battlefield
  nounZone (AsMarker m _) = Just (markerZone m)
  nounZone (TheGrantor m) = Just (markerZone m)
  nounZone TheDefendingPlayer = Nothing
  nounZone TheAttackingPlayer = Nothing
  nounZone You = Nothing
  nounZone (PlayerGroup _) = Nothing
  nounZone (Described _ p) = phraseZone p
  nounZone (EachOf grp) = nounZone grp
  nounZone (NamesAgree _ grp) = nounZone grp
  nounZone (Both l r) = if nounZone l == nounZone r then nounZone l else Nothing
  nounZone (EitherOf _ _) = Nothing
  nounZone (LibrarySlice _ _ _) = Just Library
  nounZone (SomeOf _ _ grp) = nounZone grp
  nounZone (TheRest k _) = zoneOfGroup k bs
  nounZone (PileOf _ _) = zoneOfReach (Word PileW) ManyOf bs
  nounZone (Pro r pl) = zoneOfReach r pl bs
  nounZone (ItOtherThan _ rest) = zoneOfReach Bare OneOf rest
  nounZone (Own pl own _) = zoneOfReach Bare pl own
  nounZone (AttachHost _ h) = attachHostZone h
  nounZone (PossessorOf _ n) = Nothing
  nounZone (PossessorsOf _ n) = Nothing
  nounZone (Designated _ _) = Nothing
  nounZone (OneEachOf _ pool) = nounZone pool

  ||| Whether the noun names an ability on the stack [CR#113.1c] -- read off
  ||| the description and the reading word, never the antecedent stack, so it
  ||| reduces where the bindings are still abstract.
  public export
  nounIsAbility : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsAbility (Described _ p) = seedsAbility p
  nounIsAbility (Pro (Word AbilityW) _) = True
  nounIsAbility (Pro (Word AbilityCopyW) _) = True
  nounIsAbility (EachOf grp) = nounIsAbility grp
  nounIsAbility (NamesAgree _ grp) = nounIsAbility grp
  nounIsAbility (ResolvedPermanent n) = nounIsAbility n
  nounIsAbility (AsMarker _ n) = nounIsAbility n
  nounIsAbility (SomeOf _ _ grp) = nounIsAbility grp
  nounIsAbility _ = False

  public export
  nounTy : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe CardType
  nounTy This = Nothing
  nounTy (AsType t n _) = Just t
  nounTy (ResolvedPermanent n) = nounTy n
  nounTy (AsMarker _ n) = nounTy n
  nounTy (TheGrantor _) = Nothing
  nounTy TheDefendingPlayer = Nothing
  nounTy TheAttackingPlayer = Nothing
  nounTy You = Nothing
  nounTy (PlayerGroup _) = Nothing
  nounTy (Described _ p) = seedTy p
  nounTy (EachOf grp) = nounTy grp
  nounTy (NamesAgree _ grp) = nounTy grp
  nounTy (Both l r) = if nounTy l == nounTy r then nounTy l else Nothing
  nounTy (EitherOf _ _) = Nothing
  nounTy (LibrarySlice _ _ _) = Nothing
  nounTy (SomeOf _ d grp) = sliceTy d grp
  nounTy (TheRest k _) = tyOfGroup k bs
  nounTy (PileOf _ _) = Nothing
  nounTy (Pro r pl) = tyOfReach r pl bs
  nounTy (ItOtherThan _ rest) = tyOfReach Bare OneOf rest
  nounTy (Own pl own _) = tyOfReach Bare pl own
  nounTy (AttachHost _ h) = attachHostTy h
  nounTy (PossessorOf _ n) = Nothing
  nounTy (PossessorsOf _ n) = Nothing
  nounTy (Designated _ _) = Nothing
  nounTy (OneEachOf _ pool) = nounTy pool

  public export
  nounHeadTys : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List (List CardType)
  nounHeadTys (Described _ p) = headTyAlts p
  nounHeadTys (EachOf grp) = nounHeadTys grp
  nounHeadTys (NamesAgree _ grp) = nounHeadTys grp
  nounHeadTys (ResolvedPermanent n) = nounHeadTys n
  nounHeadTys (AsMarker _ n) = nounHeadTys n
  nounHeadTys (Both l r) = nounHeadTys l ++ nounHeadTys r
  nounHeadTys (EitherOf l r) = nounHeadTys l ++ nounHeadTys r
  nounHeadTys (OneEachOf _ pool) = nounHeadTys pool
  nounHeadTys n = soleAlt (optCT (nounTy n))

  public export
  deedNounOk : {bs : Bindings} -> {k : Kind} ->
               VerbLabel -> Role -> Noun bs k -> Bool
  deedNounOk v r n = case nounHeadTys n of
    [] => isNothing (nounDet n) && deedBareOk v r
    ts => deedHeadTysOk v r ts

  public export
  nounTys : {bs : Bindings} -> {k : Kind} -> Noun bs k -> HeadTy k
  nounTys (Described _ p) = seedTys p
  nounTys (EachOf grp) = nounTys grp
  nounTys (NamesAgree _ grp) = nounTys grp
  nounTys (SomeOf _ d grp) = SoleTy (sliceTy d grp)
  nounTys n@(Both l r {jk = JoinSame}) = SoleTy (nounTy n)
  nounTys (Both l r {jk = JoinDiff}) = JoinTy (nounTys l) (nounTys r)
  nounTys n@(EitherOf l r {jk = JoinSame}) = SoleTy (nounTy n)
  nounTys (EitherOf l r {jk = JoinDiff}) = JoinTy (nounTys l) (nounTys r)
  nounTys n = SoleTy (nounTy n)

  public export
  nounPlur : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Plurality
  nounPlur This = OneOf
  nounPlur (AsType t n _) = nounPlur n
  nounPlur (ResolvedPermanent n) = nounPlur n
  nounPlur (AsMarker _ n) = nounPlur n
  nounPlur (TheGrantor _) = OneOf
  nounPlur TheDefendingPlayer = OneOf
  nounPlur TheAttackingPlayer = OneOf
  nounPlur You = OneOf
  nounPlur (PlayerGroup _) = ManyOf
  nounPlur (Described d _) = detPlur d
  nounPlur (EachOf grp) = ManyOf
  nounPlur (NamesAgree _ grp) = nounPlur grp
  nounPlur (Both _ _) = ManyOf
  nounPlur (EitherOf l r) =
    if samePlur (nounPlur l) (nounPlur r) then nounPlur l else ManyOf
  nounPlur (LibrarySlice _ amt whose) = outputPlur (nounPlur whose) (amtPlur amt)
  nounPlur (SomeOf q _ _) = slicePlur q
  nounPlur (TheRest _ pl) = pl
  nounPlur (PileOf q _) = slicePlur q
  nounPlur (Pro _ pl) = pl
  nounPlur (ItOtherThan _ _) = OneOf
  nounPlur (Own pl _ _) = pl
  nounPlur (AttachHost _ _) = OneOf
  nounPlur (PossessorOf _ n) = OneOf
  nounPlur (PossessorsOf _ _) = ManyOf
  nounPlur (Designated _ _) = OneOf
  nounPlur (OneEachOf _ _) = ManyOf

  ||| A plural possessor must distribute over players [CR#102.1].
  public export
  partPossessorOk : {bs : Bindings} -> Maybe (Noun bs Player) -> Bool
  partPossessorOk Nothing = True
  partPossessorOk (Just n) = isOne (nounPlur n) || nounDet n == Just EachD

  public export
  windowOk : {bs : Bindings} -> TurnPart -> Maybe (Noun bs Player) -> Bool
  windowOk Turn Nothing = False
  windowOk _ w = partPossessorOk w

  public export
  WindowOk : {bs : Bindings} -> TurnPart -> Maybe (Noun bs Player) -> Type
  WindowOk p w = So (windowOk p w)

  public export
  pointWindowOk : {bs : Bindings} -> Maybe (Noun bs Player) -> Bool
  pointWindowOk w = partPossessorOk w

  public export
  PointWindowOk : {bs : Bindings} -> Maybe (Noun bs Player) -> Type
  PointWindowOk w = So (pointWindowOk w)

  ||| A duration's possessor must be a single definite player.
  public export
  durationPossessorOk : {bs : Bindings} -> Maybe (Noun bs Player) -> Bool
  durationPossessorOk Nothing = True
  durationPossessorOk (Just n) =
    isOne (nounPlur n) && maybe True (== TheD) (nounDet n)

  public export
  DurationPossessor : {bs : Bindings} -> Maybe (Noun bs Player) -> Type
  DurationPossessor w = So (durationPossessorOk w)

  public export
  data DurationEnd : Bindings -> Type where
    StartOf : TurnPart -> (w : Maybe (Noun bs Player)) ->
              {auto 0 dp : DurationPossessor w} -> DurationEnd bs
    EndOf : TurnPart -> (w : Maybe (Noun bs Player)) ->
            {auto 0 dp : DurationPossessor w} -> DurationEnd bs
