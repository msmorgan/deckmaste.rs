||| The semantics-target workbench: zone/predicate/noun/amount/condition/
||| event/effect/ability constructions and the card record, all mutual.
module Experimental

import public Experimental.Words
import public Experimental.Events

%default total

mutual
  public export
  data ZoneScope : Bindings -> Zone -> Type where
    Bare : ZoneScope bs z
    OwnedBy : (n : Noun bs Player) -> {auto 0 ps : Possessable z} ->
              ZoneScope bs z

  ||| Where in a library a card lands: one end of the ordered pile
  ||| [CR#401.2], or the top-or-bottom disjunction. The chooser is a
  ||| separable slot on the
  ||| disjunction alone — Write into Being writes the bare coordination.
  public export
  data LibPlace : Bindings -> Type where
    OneEnd : (pos : LibPos) -> LibPlace bs
    EitherEnd : (chooser : Maybe (Noun bs Player)) ->
                {auto 0 ag : EventAgent chooser} -> LibPlace bs

  ||| One card goes to one end, so a disjunction over the two ends states
  ||| no order [CR#401.4].
  public export
  placeArrangementOk : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Bool
  placeArrangementOk (OneEnd _) _ = True
  placeArrangementOk (EitherEnd _) Nothing = True
  placeArrangementOk (EitherEnd _) (Just _) = False

  public export
  PlaceArrangementFits : {0 bs : Bindings} -> LibPlace bs -> Maybe Arrangement -> Type
  PlaceArrangementFits pl ord = So (placeArrangementOk pl ord)

  public export
  data ZoneExpr : Bindings -> Type where
    ZoneAt : (z : Zone) -> ZoneScope bs z -> ZoneExpr bs
    LibraryAt : (place : LibPlace bs) -> (ord : Maybe Arrangement) ->
                {default Nothing off : Maybe LibOrdinal} ->
                {auto 0 af : PlaceArrangementFits place ord} ->
                ZoneScope bs Library -> ZoneExpr bs

  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort (ZoneAt z _) = z
  zoneSort (LibraryAt _ _ _) = Library

  public export
  zoneArrangement : ZoneExpr bs -> Maybe Arrangement
  zoneArrangement (ZoneAt _ _) = Nothing
  zoneArrangement (LibraryAt _ ord _) = ord

  public export
  zoneOrdinal : ZoneExpr bs -> Maybe LibOrdinal
  zoneOrdinal (ZoneAt _ _) = Nothing
  zoneOrdinal (LibraryAt _ _ {off} _) = off

  public export
  data NameSource : Bindings -> Type where
    PrintedName : (name : String) -> NameSource bs
    ChosenName : {auto 0 ok : countQuality CardName bs = 1} -> NameSource bs
    SameNameAs : (n : Noun bs Object) ->
                 {auto 0 one : nounPlur n = OneOf} -> NameSource bs

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
  nameSrcAnyTargetFree : {0 bs : Bindings} -> NameSource bs -> Bool
  nameSrcAnyTargetFree (PrintedName _) = True
  nameSrcAnyTargetFree ChosenName = True
  nameSrcAnyTargetFree (SameNameAs n) = nounAnyTargetFree n

  ||| Indexed by sort so each domain narrows its own way — a name by a
  ||| card description [CR#201.4a], a color/creature-type by exclusion, a
  ||| number by a floor — and the index refuses every crossing for free.
  public export
  data ChoiceDomain : QualitySort -> Type where
    NameOfCard : (p : Predicate [] Object) -> ChoiceDomain CardName
    ColorOtherThan : (c : Chroma.Color) -> ChoiceDomain Color
    TypeOtherThan : (s : Subtype) ->
                    {auto 0 ct : subtypeType s = Creature} ->
                    ChoiceDomain CreatureType
    NumberAbove : (n : Nat) -> ChoiceDomain Number

  ||| Not an `Eq` instance: this equality calls `predEq`, which calls back,
  ||| and an implementation is opaque to the size-change checker, so the
  ||| mutual block loses totality.
  public export
  sameChoiceDomain : {0 q : QualitySort} -> ChoiceDomain q -> ChoiceDomain q -> Bool
  sameChoiceDomain (NameOfCard a) (NameOfCard b) = predEq a b
  sameChoiceDomain (ColorOtherThan a) (ColorOtherThan b) = a == b
  sameChoiceDomain (TypeOtherThan a) (TypeOtherThan b) = a == b
  sameChoiceDomain (NumberAbove a) (NumberAbove b) = a == b

  public export
  sameDomainOpt : {0 q : QualitySort} ->
                  Maybe (ChoiceDomain q) -> Maybe (ChoiceDomain q) -> Bool
  sameDomainOpt Nothing Nothing = True
  sameDomainOpt (Just a) (Just b) = sameChoiceDomain a b
  sameDomainOpt _ _ = False

  public export
  data EventComplement : Bindings -> EventName -> Kind -> Type where
    Involving : {kc : Kind} -> (what : Noun bs kc) ->
                {auto 0 cp : LookbackComplement ev ks kc} ->
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
  data Predicate : Bindings -> Kind -> Type where
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
    HasSubtype : Subtype -> Predicate bs Object
    AnyPlayer : Predicate bs Player                      -- head noun "player" (any player, [CR#102.1])
    Opponent : Predicate bs Player                       -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    QualityNoun : (q : QualitySort) ->
                  {default Nothing dom : Maybe (ChoiceDomain q)} ->
                  Predicate bs (Quality q)
    -- reads the unique chosen quality; the choice was made at
    -- resolution by another clause [CR#608.2d].
    OfChosen : (q : QualitySort) -> {auto 0 ok : countQuality q bs = 1} ->
               {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    -- the marked read: existence rather than `OfChosen`'s uniqueness
    -- [CR#607.2d]; bindings are nearest-first, so the latest choice is
    -- what "the last chosen" reads.
    OfLastChosenColor : {auto 0 ok : ChoiceStands (countQuality Color bs)} ->
                        Predicate bs Object
    -- the determiner that chooses in its own phrase, where `OfChosen`
    -- reads a choice made by some other clause [CR#607.2d] — there is
    -- no first ability for this one to be linked to.
    OfYourChoice : (q : QualitySort) ->
                   {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    HasKeyword : (k : Keyword) -> 
                 Predicate bs Object
    ControlledBy : (n : Noun bs Player) -> {auto 0 ps : SoleHolder n} -> Predicate bs Object
    CastBy : (n : Noun bs Player) -> {auto 0 ps : SoleHolder n} -> Predicate bs Object
    CastFrom : (z : ZoneExpr bs) ->
               {auto 0 pf : So (playableFrom (Just (zoneSort z)))} ->
               Predicate bs Object
    Attacking : Predicate bs Object
    Blocking : Predicate bs Object
    BlockerOf : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    BlockedBy : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    HappenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
                 {default Nothing what : Maybe (EventComplement bs ev k)} ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
    ColorIs : (c : Chroma.Color) -> Predicate bs Object
    IsColorless : Predicate bs Object
    Multicolored : Predicate bs Object
    Monocolored : Predicate bs Object
    HasSupertype : (s : Supertype) -> Predicate bs Object
    Named : (src : NameSource bs) -> Predicate bs Object
    HasDesignation : (d : Designation) ->
                     {auto 0 sc : designationScope d = HeldBy k} ->
                     {auto 0 at : So (designationChecked d)} ->
                     Predicate bs k
    IsAttached : (w : AttachWord) ->
                 {auto 0 ok : So (attachedCheckOk w)} -> Predicate bs Object
    Permanent : Predicate bs Object
    IsToken : Predicate bs Object
    HasStatus : {c : StatusCat} -> (v : StatusVal c) -> Predicate bs Object
    HasCounters : (kind : Maybe CounterKind) ->
                  {auto 0 kn : CounterKindNamed Object kind} ->
                  Predicate bs Object
    Compare : (c : Characteristic) -> (r : Comparator) -> (bound : Amount bs) ->
              Predicate bs Object
    Superlative : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                  (dom : Predicate bs k) ->
                  {auto 0 ex : IsExtremal op} ->
                  {auto 0 sc : projScope ax = k} ->
                  {auto 0 af : AnyTargetFree dom} -> Predicate bs k
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
    ExiledWith : (src : Noun bs Object) ->
                 {auto 0 ls : LinkSource src} -> Predicate bs Object
    And : (ps : List (Predicate bs k)) -> {auto 0 zc : ZoneCoherent ps} ->
          {auto 0 cf : ContradictionFree ps} -> {auto 0 oa : OtherAnchored ps} ->
          {auto 0 at : AnyTargetLone ps} -> 
          Predicate bs k
    Or : (ps : List (Predicate bs k)) -> {auto 0 ne : NonEmpty ps} ->
         {auto 0 pd : ParallelDisjuncts ps} ->
         {auto 0 cd : CoordinableDisjuncts ps} ->
         {auto 0 dd : DistinctDisjuncts ps} -> Predicate bs k
    Not : (p : Predicate bs k) -> {auto 0 ng : Negatable p} -> Predicate bs k
    Other : {auto 0 ok : So (anyTargeted k bs)} -> Predicate bs k
    OtherThan : (n : Noun bs k) -> {auto 0 ca : ComplementAnchor n} -> Predicate bs k
    AnyTarget : Predicate bs Object
    KindJoin : (who : JoinedPlayer) -> (what : JoinedClass) ->
               Predicate bs Object
    IsSource : Predicate bs Object
    AbilityHead : (cls : AbilityClass) -> Predicate bs Ability
    AbilityOf : (src : Noun bs Object) -> Predicate bs Ability
    ActivatedBy : (who : Noun bs Player) ->
                  {auto 0 ps : SoleHolder who} -> Predicate bs Ability
    IsManaAbility : Predicate bs Ability

  ||| The head type a predicate projects onto its referent — what an
  ||| anaphor remembers across a zone change.
  public export
  seedTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedTy (HasType t) = Just t
  seedTy (And ps) = seedTyAll ps
  seedTy (Or ps) = seedTyJoin ps
  seedTy _ = Nothing

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
  headTys p = optCT (seedTy p)

  public export
  headTysJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> List CardType
  headTysJoin [] = []
  headTysJoin (p :: ps) = headTys p ++ headTysJoin ps

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
  seedZone Blocking = Just Battlefield
  seedZone (BlockerOf _) = Just Battlefield
  seedZone (BlockedBy _) = Just Battlefield
  seedZone (HappenedTo _ _) = Nothing
  seedZone (ColorIs _) = Nothing
  seedZone IsColorless = Nothing
  seedZone Multicolored = Nothing
  seedZone Monocolored = Nothing
  seedZone (HasSupertype _) = Nothing
  seedZone (Named _) = Nothing
  seedZone (HasDesignation d) = designationSeedZone d
  seedZone (IsAttached _) = Just Battlefield
  seedZone IsToken = Just Battlefield
  seedZone (HasStatus _) = Just Battlefield
  seedZone (HasCounters _) = Nothing
  seedZone (ControlledBy _) = Nothing
  -- Casting is history, not a location: [CR#400.7d] lets a permanent's
  -- ability reference the spell it was cast as, and [CR#702.40a] counts
  -- spells cast earlier this turn that have long left the stack.
  seedZone (CastBy _) = Nothing
  seedZone (ExiledWith _) = Just Exile
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
  seedsToken (And ps) = seedsTokenAny ps
  seedsToken (Or ps) = seedsTokenAll ps
  seedsToken _ = False

  public export
  seedsTokenAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAny [] = False
  seedsTokenAny (p :: ps) = seedsToken p || seedsTokenAny ps

  public export
  seedsTokenAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  seedsTokenAll [] = False
  seedsTokenAll (p :: ps) = seedsToken p && seedsTokenAll ps

  public export
  zoneAdmit : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  zoneAdmit (ControlledBy _) = [Battlefield, Stack]
  zoneAdmit _ = []

  ||| The card type a predicate presupposes of its referent — distinct
  ||| from `seedTy`, which projects the phrase's own head [CR#506.3].
  public export
  seedType : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedType Attacking = Just Creature
  seedType Blocking = Just Creature
  seedType (BlockerOf _) = Just Creature
  seedType (BlockedBy _) = Just Creature
  seedType (HappenedTo _ _) = Nothing
  seedType (ColorIs _) = Nothing
  seedType IsColorless = Nothing
  seedType Multicolored = Nothing
  seedType Monocolored = Nothing
  seedType (HasSupertype _) = Nothing
  seedType (Named _) = Nothing
  seedType (HasDesignation d) = designationSeedType d
  seedType (IsAttached _) = Nothing
  seedType (Compare c _ _) = comparedType c
  seedType (Superlative _ (CharAxis c) _) = comparedType c
  seedType (Superlative _ (PlayerStatAxis _) _) = Nothing
  seedType (HasSubtype s) = Just (subtypeType s)
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

  ||| Whether a description writes its own head noun. English word class
  ||| only: the kind index, not the head word, supplies a description's
  ||| domain, and [CR#109.2] assigns a default zone to a description that
  ||| names a card type or subtype without saying anything about one that
  ||| does not. No determiner demands a head, and no gate demands one. The
  ||| word class is read by `parallelDisjuncts` alone, to keep the arms of
  ||| one coordination alike.
  public export
  hasHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasHead (HasType _) = True
  hasHead (HasSubtype _) = True
  hasHead AnyPlayer = True
  hasHead Opponent = True
  hasHead (QualityNoun _) = True
  hasHead (OfChosen _) = False
  hasHead OfLastChosenColor = False
  hasHead (OfYourChoice _) = False
  hasHead (AbilityHead _) = True
  hasHead (AbilityOf _) = False
  hasHead (ActivatedBy _) = False
  hasHead IsManaAbility = False
  hasHead IsSource = True
  hasHead (HasKeyword _) = False
  hasHead (ControlledBy _) = False
  hasHead (CastBy _) = False
  hasHead Attacking = False
  hasHead Blocking = False
  hasHead (BlockerOf _) = False
  hasHead (BlockedBy _) = False
  hasHead (HappenedTo _ _) = False
  hasHead (CastFrom _) = False
  hasHead (ColorIs _) = False
  hasHead IsColorless = False
  hasHead Multicolored = False
  hasHead Monocolored = False
  hasHead (HasSupertype _) = False
  hasHead (Named _) = False
  hasHead (HasDesignation _) = False
  hasHead (IsAttached _) = False
  hasHead Permanent = True
  hasHead IsToken = True
  hasHead (HasStatus _) = False
  hasHead (HasCounters _) = False
  hasHead (Compare _ _ _) = False
  hasHead (Superlative _ _ _) = False
  hasHead (InZone _) = True
  hasHead (ExiledWith _) = True
  -- ANY member heads a conjunction, but EVERY alternative has to head a
  -- disjunction: each disjunct stands where the phrase's head would.
  hasHead (And ps) = hasHeadAny ps
  hasHead (Or ps) = hasHeadAll ps
  hasHead (Not _) = False
  hasHead Other = False
  hasHead (OtherThan _) = False
  hasHead AnyTarget = True
  hasHead (KindJoin _ _) = True

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
  qualityReadOk OfLastChosenColor = True
  qualityReadOk (OfYourChoice _) = True
  qualityReadOk (Named _) = True
  qualityReadOk _ = False

  public export
  QualityRead : Predicate bs Object -> Type
  QualityRead {bs} p = So (qualityReadOk p)

  public export
  uniquifiesAny : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  uniquifiesAny [] = False
  uniquifiesAny (p :: ps) = uniquifies p || uniquifiesAny ps

  public export
  uniquifies : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  uniquifies (Superlative _ _ _) = True
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
  zonesOk ps = zonesAgree Nothing (flattenPs ps) &&
               not (elem (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                         (negZones (flattenPs ps))) &&
               zoneAdmitsAll (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                             (flattenPs ps)

  public export
  ZoneCoherent : List (Predicate bs k) -> Type
  ZoneCoherent {bs} {k} ps = So (zonesOk ps)

  ||| Syntactic predicate equality, deliberately conservative: `False`
  ||| means "not provably the same referent", so the gate under-refuses
  ||| rather than over-refuses.
  public export
  predEq : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  predEq (HasType a) (HasType b) = a == b
  predEq (HasType _) _ = False
  predEq (HasSubtype a) (HasSubtype b) = a == b
  predEq (HasSubtype _) _ = False
  predEq AnyPlayer AnyPlayer = True
  predEq AnyPlayer _ = False
  predEq Opponent Opponent = True
  predEq Opponent _ = False
  predEq (QualityNoun a {dom = d}) (QualityNoun a {dom = e}) = sameDomainOpt d e
  predEq (QualityNoun _) _ = False
  predEq (OfChosen a) (OfChosen b) = a == b
  predEq (OfChosen _) _ = False
  predEq OfLastChosenColor OfLastChosenColor = True
  predEq OfLastChosenColor _ = False
  predEq (OfYourChoice a) (OfYourChoice b) = a == b
  predEq (OfYourChoice _) _ = False
  predEq (AbilityHead a) (AbilityHead b) = a == b
  predEq (AbilityHead _) _ = False
  predEq (AbilityOf a) (AbilityOf b) = nounEqRef a b
  predEq (AbilityOf _) _ = False
  predEq (ActivatedBy a) (ActivatedBy b) = nounEqRef a b
  predEq (ActivatedBy _) _ = False
  predEq IsManaAbility IsManaAbility = True
  predEq IsManaAbility _ = False
  predEq IsSource IsSource = True
  predEq IsSource _ = False
  predEq (HasKeyword a) (HasKeyword b) = a == b
  predEq (HasKeyword _) _ = False
  predEq (ControlledBy a) (ControlledBy b) = nounEqRef a b
  predEq (ControlledBy _) _ = False
  predEq (CastBy a) (CastBy b) = nounEqRef a b
  predEq (CastBy _) _ = False
  predEq (ExiledWith a) (ExiledWith b) = nounEqRef a b
  predEq (ExiledWith _) _ = False
  predEq Attacking Attacking = True
  predEq Attacking _ = False
  predEq Blocking Blocking = True
  predEq Blocking _ = False
  predEq (BlockerOf a) (BlockerOf b) = nounEqRef a b
  predEq (BlockerOf _) _ = False
  predEq (BlockedBy a) (BlockedBy b) = nounEqRef a b
  predEq (BlockedBy _) _ = False
  -- both arguments are closed words, so this row compares wholly rather
  -- than conservatively.
  predEq (HappenedTo a v {what = Nothing}) (HappenedTo b w {what = Nothing}) =
    sameEventName a b && sameLookback v w
  predEq (HappenedTo _ _) _ = False
  predEq (ColorIs a) (ColorIs b) = a == b
  predEq (ColorIs _) _ = False
  predEq IsColorless IsColorless = True
  predEq IsColorless _ = False
  predEq Multicolored Multicolored = True
  predEq Multicolored _ = False
  predEq Monocolored Monocolored = True
  predEq Monocolored _ = False
  predEq (HasSupertype a) (HasSupertype b) = a == b
  predEq (HasSupertype _) _ = False
  predEq (Named a) (Named b) = a == b
  predEq (Named _) _ = False
  predEq (HasDesignation a) (HasDesignation b) = a == b
  predEq (HasDesignation _) _ = False
  predEq (IsAttached a) (IsAttached b) = a == b
  predEq (IsAttached _) _ = False
  predEq Permanent Permanent = True
  predEq Permanent _ = False
  predEq IsToken IsToken = True
  predEq IsToken _ = False
  predEq (HasStatus v) (HasStatus w) = sameStatusVal v w
  predEq (HasStatus _) _ = False
  predEq (HasCounters Nothing) (HasCounters Nothing) = True
  predEq (HasCounters (Just a)) (HasCounters (Just b)) = a == b
  predEq (HasCounters _) _ = False
  predEq (Compare c r b) (Compare d s e) = c == d && r == s &&
                                           boundEq b e
  predEq (Compare _ _ _) _ = False
  predEq (Superlative o a d) (Superlative p b e) =
    o == p && a == b && predEq d e
  predEq (Superlative _ _ _) _ = False
  predEq (CastFrom z) (CastFrom w) = zoneSort z == zoneSort w
  predEq (CastFrom _) _ = False
  predEq (InZone z) (InZone w) = zoneSort z == zoneSort w
  predEq (InZone _) _ = False
  predEq (And xs) (And ys) = predEqAll xs ys
  predEq (And _) _ = False
  -- the blanket row is free here: nothing may nest an `Or` in an `Or`
  -- (`CoordinableDisjuncts`), so two coordinations never meet as
  -- alternatives.
  predEq (Or _) _ = False
  predEq (Not a) (Not b) = predEq a b
  predEq (Not _) _ = False
  predEq Other Other = True
  predEq Other _ = False
  predEq (OtherThan a) (OtherThan b) = nounEqRef a b
  predEq (OtherThan _) _ = False
  predEq AnyTarget AnyTarget = True
  predEq AnyTarget _ = False
  predEq (KindJoin w c) (KindJoin x d) = w == x && c == d
  predEq (KindJoin _ _) _ = False

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
  negTypesOf (Not p) = case seedTy p of
    Just t => [t]
    Nothing => []
  negTypesOf _ = []

  public export
  negTypes : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List CardType
  negTypes [] = []
  negTypes (p :: ps) = negTypesOf p ++ negTypes ps

  ||| The card types a member is satisfiable under. [CR#205.3m] gives
  ||| creatures and kindreds one shared subtype list, so a creature subtype
  ||| word describes a Kindred as readily as a creature.
  public export
  seedTypeAlts : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  seedTypeAlts (HasSubtype s) =
    if subtypeType s == Creature then [Creature, Kindred] else [subtypeType s]
  seedTypeAlts p = case seedType p of
    Just t => [t]
    Nothing => []

  public export
  allNegated : List CardType -> List CardType -> Bool
  allNegated negs [] = True
  allNegated negs (t :: ts) = elem t negs && allNegated negs ts

  ||| A conjunction is empty when a negated type word rules out EVERY card
  ||| type one of its members could be satisfied under.
  public export
  anySeedEmptied : {0 bs : Bindings} -> {0 k : Kind} -> List CardType ->
                   List (Predicate bs k) -> Bool
  anySeedEmptied negs [] = False
  anySeedEmptied negs (p :: ps) = case seedTypeAlts p of
    [] => anySeedEmptied negs ps
    ts => allNegated negs ts || anySeedEmptied negs ps

  public export
  contradictionFree : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  contradictionFree ps = noNegatedPair (flattenPs ps) &&
                         not (anySeedEmptied (negTypes (flattenPs ps))
                                             (flattenPs ps)) &&
                         noStatusClash (flattenPs ps) &&
                         noColorClash (flattenPs ps) &&
                         not (anyPermanentHead (flattenPs ps) &&
                              anyNonPermanentTy (flattenPs ps))

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

  ||| [CR#115.4] lists "another target" among the class words, and
  ||| [CR#601.2c] is why it is written: without it the same object may be
  ||| chosen once for each separate instance of "target". So the word has
  ||| one job — to name a target other than one already chosen — and
  ||| neither rule asks anything of the two descriptions' head nouns; any
  ||| earlier target of the kind anchors it. The
  ||| named complement ("other than this creature") still has to name
  ||| something the phrase could describe.
  public export
  otherAnchorOk : {bs : Bindings} -> (k : Kind) -> List CardType ->
                  List (Predicate bs k) -> Bool
  otherAnchorOk k ts ps =
    if atMostOne (countOthers (flattenPs ps))
      then (if hasBareOtherAny ps
              then anyTargeted k bs
              else complementAnchorsOk ts (flattenPs ps))
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
  isAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isAnyTarget AnyTarget = True
  isAnyTarget _ = False

  public export
  isKindJoin : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isKindJoin (KindJoin _ _) = True
  isKindJoin _ = False

  public export
  anyIsKindJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsKindJoin [] = False
  anyIsKindJoin (p :: ps) = isKindJoin p || anyIsKindJoin ps

  public export
  headIsKindJoin : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsKindJoin p = anyIsKindJoin (flattenPs [p])

  ||| The card type the joined class names, if any -- the Object half's
  ||| type in the join's payload. The class word ("any target") names none.
  public export
  kindJoinTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  kindJoinTy (KindJoin _ c) = joinedClassTy c
  kindJoinTy _ = Nothing

  public export
  anyKindJoinTy : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  anyKindJoinTy [] = Nothing
  anyKindJoinTy (p :: ps) = maybe (anyKindJoinTy ps) Just (kindJoinTy p)

  public export
  headKindJoinTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  headKindJoinTy p = anyKindJoinTy (flattenPs [p])

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
  anyIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsAnyTarget [] = False
  anyIsAnyTarget (p :: ps) = isAnyTarget p || anyIsAnyTarget ps

  public export
  headIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsAnyTarget p = anyIsAnyTarget (flattenPs [p])

  public export
  isSourceHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isSourceHead IsSource = True
  isSourceHead _ = False

  public export
  anyIsSource : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsSource [] = False
  anyIsSource (p :: ps) = isSourceHead p || anyIsSource ps

  public export
  headIsPlaceless : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsPlaceless p = headIsAnyTarget p || anyIsSource (flattenPs [p]) ||
                      headIsKindJoin p

  public export
  ||| [CR#115.4] closes the class word's target set to creatures,
  ||| players, planeswalkers and battles, all of them on the battlefield
  ||| or in no zone at all, so a modifier that places the target
  ||| elsewhere names nothing. Every other qualifier merely restricts the
  ||| set and is the phrase's own business.
  allLoneOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  allLoneOk [] = True
  allLoneOk (p :: ps) = anyTargetZoneOk (seedZone p) && allLoneOk ps

  public export
  anyTargetZoneOk : Maybe Zone -> Bool
  anyTargetZoneOk Nothing = True
  anyTargetZoneOk (Just Battlefield) = True
  anyTargetZoneOk (Just _) = False

  public export
  countAnyTargets : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Nat
  countAnyTargets [] = Z
  countAnyTargets (p :: ps) =
    if isAnyTarget p then S (countAnyTargets ps) else countAnyTargets ps

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
  anyTargetLone : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyTargetLone ps = if anyIsAnyTarget (flattenPs ps)
                       then allLoneOk (flattenPs ps) &&
                            exactlyOne (countAnyTargets (flattenPs ps)) &&
                            atMostOne (countOthers (flattenPs ps))
                       else True

  public export
  AnyTargetLone : List (Predicate bs k) -> Type
  AnyTargetLone {bs} {k} ps = So (anyTargetLone ps)

  public export
  isComparison : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isComparison (Compare _ _ _) = True
  isComparison (Superlative _ _ _) = True
  isComparison _ = False

  public export
  countComparisons : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Nat
  countComparisons [] = Z
  countComparisons (p :: ps) =
    if isComparison p then S (countComparisons ps) else countComparisons ps

  public export
  headsUniform : {0 bs : Bindings} -> {0 k : Kind} ->
                 Bool -> List (Predicate bs k) -> Bool
  headsUniform b [] = True
  headsUniform b (p :: ps) = (if hasHead p then b else not b) && headsUniform b ps

  public export
  seedsUniform : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> Maybe CardType ->
                 List (Predicate bs k) -> Bool
  seedsUniform z t [] = True
  seedsUniform z t (p :: ps) = z == seedZone p &&
                               t == seedType p &&
                               seedsUniform z t ps

  public export
  parallelDisjuncts : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  parallelDisjuncts [] = True
  parallelDisjuncts (p :: ps) = headsUniform (hasHead p) ps &&
                                seedsUniform (seedZone p) (seedType p) ps

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
  coordinable p = not (headIsAnyTarget p) &&
                  not (headIsKindJoin p) &&
                  not (hasOther p)

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

  ||| A "non-" prefix names the complement of one modifier inside its own
  ||| kind. Refused only where the rules leave that complement empty: the
  ||| universal player word covers every person in the game [CR#102.1]; a
  ||| quality noun names its whole sort, which for colour is closed at five
  ||| [CR#105.1] — the cell over-reaches a domain-restricted quality noun,
  ||| whose complement is NOT empty; and [CR#120.7] makes a source the
  ||| object that dealt some damage, a position any object may occupy rather
  ||| than a property it lacks. Every other modifier has something outside it.
  public export
  negatable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  negatable AnyPlayer = False
  negatable (QualityNoun _) = False
  negatable IsSource = False
  negatable _ = True

  public export
  Negatable : Predicate bs k -> Type
  Negatable {bs} {k} p = So (negatable p)

  public export
  predSays : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predSays (HasType _) = True
  predSays (HasSubtype _) = True
  predSays AnyPlayer = True
  predSays Opponent = True
  predSays (QualityNoun _) = True
  predSays (OfChosen _) = True
  predSays OfLastChosenColor = True
  predSays (OfYourChoice _) = True
  predSays (AbilityHead _) = True
  predSays (AbilityOf _) = True
  predSays (ActivatedBy _) = True
  predSays IsManaAbility = True
  predSays IsSource = True
  predSays (HasKeyword _) = True
  predSays (ControlledBy _) = True
  predSays (CastBy _) = True
  predSays (ExiledWith _) = True
  predSays Attacking = True
  predSays Blocking = True
  predSays (BlockerOf _) = True
  predSays (BlockedBy _) = True
  predSays (HappenedTo _ _) = True
  predSays (CastFrom _) = True
  predSays (ColorIs _) = True
  predSays IsColorless = True
  predSays Multicolored = True
  predSays Monocolored = True
  predSays (HasSupertype _) = True
  predSays (Named _) = True
  predSays (HasDesignation _) = True
  predSays (IsAttached _) = True
  predSays Permanent = True
  predSays IsToken = True
  predSays (HasStatus _) = True
  predSays (HasCounters _) = True
  predSays (Compare _ _ _) = True
  predSays (Superlative _ _ _) = True
  predSays (InZone _) = True
  predSays (And ps) = predSaysAny ps
  predSays (Or _) = True
  predSays (Not p) = predSays p
  predSays Other = True
  predSays (OtherThan _) = True
  predSays AnyTarget = True
  predSays (KindJoin _ _) = True

  public export
  predSaysAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  predSaysAny [] = False
  predSaysAny (p :: ps) = predSays p || predSaysAny ps

  public export
  PredSays : Predicate bs k -> Type
  PredSays {bs} {k} p = So (predSays p)

  public export
  predNegFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predNegFree (HasType _) = True
  predNegFree (HasSubtype _) = True
  predNegFree AnyPlayer = True
  predNegFree Opponent = True
  predNegFree (QualityNoun _) = True
  predNegFree (OfChosen _) = True
  predNegFree OfLastChosenColor = True
  predNegFree (OfYourChoice _) = True
  predNegFree (AbilityHead _) = True
  predNegFree (AbilityOf _) = True
  predNegFree (ActivatedBy _) = True
  predNegFree IsManaAbility = True
  predNegFree IsSource = True
  predNegFree (HasKeyword _) = True
  predNegFree (ControlledBy _) = True
  predNegFree (CastBy _) = True
  predNegFree (ExiledWith _) = True
  predNegFree Attacking = True
  predNegFree Blocking = True
  predNegFree (BlockerOf _) = True
  predNegFree (BlockedBy _) = True
  predNegFree (HappenedTo _ _) = True
  predNegFree (CastFrom _) = True
  predNegFree (ColorIs _) = True
  predNegFree IsColorless = True
  predNegFree Multicolored = True
  predNegFree Monocolored = True
  predNegFree (HasSupertype _) = True
  predNegFree (Named _) = True
  predNegFree (HasDesignation _) = True
  predNegFree (IsAttached _) = True
  predNegFree Permanent = True
  predNegFree IsToken = True
  predNegFree (HasStatus _) = True
  predNegFree (HasCounters _) = True
  predNegFree (Compare _ _ _) = True
  predNegFree (Superlative _ _ _) = True
  predNegFree (InZone _) = True
  predNegFree (And ps) = predNegFreeAll ps
  predNegFree (Or ps) = predNegFreeAll ps
  predNegFree (Not _) = False
  predNegFree Other = True
  predNegFree (OtherThan _) = True
  predNegFree AnyTarget = True
  predNegFree (KindJoin _ _) = True

  public export
  predNegFreeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  predNegFreeAll [] = True
  predNegFreeAll (p :: ps) = predNegFree p && predNegFreeAll ps

  public export
  anyTargetFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  anyTargetFree (HasType _) = True
  anyTargetFree (HasSubtype _) = True
  anyTargetFree AnyPlayer = True
  anyTargetFree Opponent = True
  anyTargetFree (QualityNoun _) = True
  anyTargetFree (OfChosen _) = True
  anyTargetFree OfLastChosenColor = True
  anyTargetFree (OfYourChoice _) = True
  anyTargetFree (AbilityHead _) = True
  anyTargetFree (AbilityOf n) = nounAnyTargetFree n
  anyTargetFree (ActivatedBy n) = nounAnyTargetFree n
  anyTargetFree IsManaAbility = True
  anyTargetFree IsSource = True
  anyTargetFree (HasKeyword _) = True
  anyTargetFree (ControlledBy n) = nounAnyTargetFree n
  anyTargetFree (CastBy n) = nounAnyTargetFree n
  anyTargetFree (ExiledWith n) = nounAnyTargetFree n
  anyTargetFree Attacking = True
  anyTargetFree Blocking = True
  anyTargetFree (BlockerOf m) = nounAnyTargetFree m
  anyTargetFree (BlockedBy m) = nounAnyTargetFree m
  anyTargetFree (HappenedTo _ _ {what}) = complementAnyTargetFree what
  anyTargetFree (CastFrom z) = zoneAnyTargetFree z
  anyTargetFree (ColorIs _) = True
  anyTargetFree IsColorless = True
  anyTargetFree Multicolored = True
  anyTargetFree Monocolored = True
  anyTargetFree (HasSupertype _) = True
  anyTargetFree (Named src) = nameSrcAnyTargetFree src
  anyTargetFree (HasDesignation _) = True
  anyTargetFree (IsAttached _) = True
  anyTargetFree Permanent = True
  anyTargetFree IsToken = True
  anyTargetFree (HasStatus _) = True
  anyTargetFree (HasCounters _) = True
  anyTargetFree (Compare _ _ b) = amtAnyTargetFree b
  anyTargetFree (Superlative _ _ d) = anyTargetFree d
  anyTargetFree (InZone z) = zoneAnyTargetFree z
  anyTargetFree (And ps) = anyTargetFreeAll ps
  anyTargetFree (Or ps) = anyTargetFreeAll ps
  anyTargetFree (Not p) = anyTargetFree p
  anyTargetFree Other = True
  anyTargetFree (OtherThan n) = nounAnyTargetFree n
  anyTargetFree AnyTarget = False
  anyTargetFree (KindJoin _ _) = True

  public export
  anyTargetFreeAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyTargetFreeAll [] = True
  anyTargetFreeAll (p :: ps) = anyTargetFree p && anyTargetFreeAll ps

  public export
  AnyTargetFree : Predicate bs k -> Type
  AnyTargetFree {bs} {k} p = So (anyTargetFree p)

  public export
  ZoneFree : Predicate bs k -> Type
  ZoneFree {bs} {k} p = seedZone p = Nothing

  ||| [CR#115.4] writes the class word as the WHOLE target phrase — "any
  ||| target", "another target", "two targets" — so a description either is
  ||| that phrase or contains none of it. The count is free: the rule lists
  ||| "two targets" itself. The quantity stays in the signature and is
  ||| matched on so the obligation is stuck until a determiner is written:
  ||| reducing on the predicate alone drives the positivity checker through
  ||| `flattenPs` and `Noun` stops being strictly positive.
  public export
  anyTargetOkAt : {0 bs : Bindings} -> {0 k : Kind} ->
                  Quantity -> Predicate bs k -> Bool
  anyTargetOkAt (Range _ _) p = headIsAnyTarget p || anyTargetFree p

  public export
  AnyTargetAtCount : Quantity -> Predicate bs k -> Type
  AnyTargetAtCount {bs} {k} q p = So (anyTargetOkAt q p)

  public export
  zoneOr : Zone -> Maybe Zone -> Zone
  zoneOr z Nothing = z
  zoneOr z (Just w) = w

  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Phrasal k -> Predicate bs k -> Binding
  bindFor det plur PhObject p =
    if headIsAnyTarget p || headIsKindJoin p
      then MkBinding det (Object \/ Player) plur
              (JoinP (ObjectP (headKindJoinTy p) Nothing Nothing Nothing) PlayerP)
      else MkBinding det Object plur
              (ObjectP (seedTy p)
                       (Just (zoneOr Battlefield (seedZone p)))
                       Nothing
                       (if seedsToken p then Just TokenOrigin else Nothing))
  bindFor det plur PhPlayer p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} PhQuality p = MkBinding det (Quality q) plur QualityP
  bindFor det plur PhAbility p = MkBinding det Ability plur AbilityP

  public export
  data Noun : Bindings -> Kind -> Type where
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    AsType : (t : CardType) -> (n : Noun bs Object) ->
             {default Nothing sub : Maybe Subtype} ->
             {auto 0 asc : Ascribable n} ->
             {auto 0 way : So (ascriptionOk t sub)} -> Noun bs Object
    You : Noun bs Player        -- "you" [CR#109.5]
    PlayerGroup : (w : PlayerGroupWord) -> Noun bs Player
    Each : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
           {auto 0 af : AnyTargetFree p} -> Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    Indefinite : (m : ChoiceMode bs) -> (p : Predicate bs k) ->
                 {auto ph : Phrasal k} ->
                 {auto 0 af : AnyTargetFree p} -> Noun bs k
    Definite : (p : Predicate bs k) ->
               {auto ph : Phrasal k} ->
               {auto 0 af : AnyTargetFree p} ->
               {auto 0 uq : Uniquifying p} -> Noun bs k
    TargetGroup : (q : Quantity) -> (p : Predicate bs k) ->
                  {auto tk : Targetable k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} ->
                  {auto 0 af : AnyTargetAtCount q p} -> Noun bs k
    CountedGroup : (q : Quantity) -> (p : Predicate bs k) ->
                   {auto ph : Phrasal k} -> {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   {auto 0 af : AnyTargetFree p} -> Noun bs k
    AllOf : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            {auto 0 af : AnyTargetFree p} -> Noun bs k
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    YouAnd : (n : Noun bs Object) ->
             {auto 0 nn : NotMixedGroup n} -> Noun bs Object
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 sp : SlicePossessor whose} ->
                   Noun bs Object
    SomeOf : (q : Quantity) -> (grp : Noun bs Object) ->
             {auto 0 gm : GroupMention grp} ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} -> Noun bs Object
    TheRest : {auto 0 ok : So (theRestOk bs)} -> Noun bs Object
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    Them : {auto 0 ok : countManys Object bs = 1} -> Noun bs Object
    Those : (w : NounWord) -> {auto 0 ok : countManyWord w bs = 1} -> Noun bs (kindOfW w)
    That : (w : NounWord) -> {auto 0 ok : countWord w bs = 1} -> Noun bs (kindOfW w)
    AttachHost : (w : AttachWord) -> (h : NounWord) ->
                 {auto 0 ok : AttachHeadOk w h} -> Noun bs (kindOfW h)
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                {default Attributive marking : VerbedMarking} ->
                {auto 0 ok : countVerbed v w bs = 1} ->
                {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    ThoseVerbed : (v : VerbName) -> (w : NounWord) ->
                  {default Attributive marking : VerbedMarking} ->
                  {auto 0 ok : countManyVerbed v w bs = 1} ->
                  {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    ControllerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player
    OwnerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player
    ||| "your commander" [CR#903.3]: the card-scope designation is an
    ||| attribute of the card itself, so the possessed noun reads it in
    ||| every zone. The possessive is the only determiner written.
    Designated : (d : Designation) -> (whose : Noun bs Player) ->
                 {auto 0 sc : designationScope d = HeldByCard} -> Noun bs Object

  ||| Referent equality between two possessor nouns, deliberately the
  ||| smallest honest relation: `True` only for the atomic words whose
  ||| referent the binding context already fixes, so two occurrences
  ||| inside one phrase provably denote the same thing. Everything else
  ||| is `False`, including two target mentions [CR#601.2c].
  public export
  nounEqRef : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Noun bs k -> Bool
  nounEqRef This This = True
  nounEqRef This _ = False
  nounEqRef (AsType _ _) _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
  nounEqRef (PlayerGroup v) (PlayerGroup w) = v == w
  nounEqRef (PlayerGroup _) _ = False
  nounEqRef (Each _) _ = False
  nounEqRef (Indefinite _ _) _ = False
  nounEqRef (Definite _) _ = False
  nounEqRef (TargetGroup _ _) _ = False
  nounEqRef (CountedGroup _ _) _ = False
  nounEqRef (AllOf _) _ = False
  nounEqRef (EachOf _) _ = False
  nounEqRef (YouAnd _) _ = False
  nounEqRef (LibrarySlice _ _ _) _ = False
  nounEqRef (SomeOf _ _) _ = False
  nounEqRef TheRest _ = False
  nounEqRef It It = True
  nounEqRef It _ = False
  nounEqRef They They = True
  nounEqRef They _ = False
  nounEqRef Them _ = False
  nounEqRef (Those _) _ = False
  nounEqRef (That _) _ = False
  nounEqRef (AttachHost _ _) _ = False
  nounEqRef (TheVerbed _ _) _ = False
  nounEqRef (ThoseVerbed _ _) _ = False
  nounEqRef (ControllerOf _) _ = False
  nounEqRef (OwnerOf _) _ = False
  nounEqRef (Designated _ _) _ = False

  public export
  nounAnyTargetFree : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounAnyTargetFree This = True
  nounAnyTargetFree (AsType t n) = nounAnyTargetFree n
  nounAnyTargetFree You = True
  nounAnyTargetFree (PlayerGroup _) = True
  nounAnyTargetFree (Each p) = anyTargetFree p
  nounAnyTargetFree (Indefinite m p) = anyTargetFree p
  nounAnyTargetFree (Definite p) = anyTargetFree p
  nounAnyTargetFree (TargetGroup _ p) = anyTargetFree p
  nounAnyTargetFree (CountedGroup _ p) = anyTargetFree p
  nounAnyTargetFree (AllOf p) = anyTargetFree p
  nounAnyTargetFree (EachOf grp) = nounAnyTargetFree grp
  nounAnyTargetFree (YouAnd n) = nounAnyTargetFree n
  nounAnyTargetFree (LibrarySlice _ _ whose) = nounAnyTargetFree whose
  nounAnyTargetFree (SomeOf _ grp) = nounAnyTargetFree grp
  nounAnyTargetFree TheRest = True
  nounAnyTargetFree It = True
  nounAnyTargetFree They = True
  nounAnyTargetFree Them = True
  nounAnyTargetFree (Those _) = True
  nounAnyTargetFree (That _) = True
  nounAnyTargetFree (AttachHost _ _) = True
  nounAnyTargetFree (TheVerbed _ _) = True
  nounAnyTargetFree (ThoseVerbed _ _) = True
  nounAnyTargetFree (ControllerOf n) = nounAnyTargetFree n
  nounAnyTargetFree (OwnerOf n) = nounAnyTargetFree n
  nounAnyTargetFree (Designated _ n) = nounAnyTargetFree n

  public export
  placeAnyTargetFree : {0 bs : Bindings} -> LibPlace bs -> Bool
  placeAnyTargetFree (OneEnd _) = True
  placeAnyTargetFree (EitherEnd Nothing) = True
  placeAnyTargetFree (EitherEnd (Just n)) = nounAnyTargetFree n

  public export
  zoneAnyTargetFree : {0 bs : Bindings} -> ZoneExpr bs -> Bool
  zoneAnyTargetFree (ZoneAt z Bare) = True
  zoneAnyTargetFree (ZoneAt z (OwnedBy n)) = nounAnyTargetFree n
  zoneAnyTargetFree (LibraryAt pl _ Bare) = placeAnyTargetFree pl
  zoneAnyTargetFree (LibraryAt pl _ (OwnedBy n)) =
    placeAnyTargetFree pl && nounAnyTargetFree n

  ||| Destination legality for the move primitive [CR#400.3]: an owned
  ||| destination naming an arbitrary player is unwritable — a moved card
  ||| is routed to its owner's zone regardless of the sentence, so the
  ||| bare zone IS the owner-rooted destination. A library destination
  ||| always takes the position form, never the bare zone [CR#401.2].
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)
    LibraryPosOk : {auto 0 af : PlaceArrangementFits place arrg} ->
                   DestOk (LibraryAt place arrg {off = offs} {af} Bare)

  public export
  orderOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Bool
  orderOk pl z = case zoneArrangement z of
                   Nothing => True
                   Just _ => not (isOne pl)

  public export
  ArrangementOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Type
  ArrangementOk {bs} pl z = So (orderOk pl z)

  public export
  nounDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  nounDelta This = []
  nounDelta (AsType t n) = nounDelta n
  nounDelta You = []
  nounDelta (PlayerGroup _) = []
  nounDelta (Each p {ph}) = bindFor EachD ManyOf ph p :: predDelta p
  nounDelta (Indefinite m p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (Definite p {ph}) = bindFor TheD OneOf ph p :: predDelta p
  nounDelta (TargetGroup q p {tk}) = bindFor TargetD (quantPlur q) (targetablePhrasal tk) p :: predDelta p
  nounDelta (CountedGroup q p {ph}) = bindFor CountD (quantPlur q) ph p :: predDelta p
  nounDelta (AllOf p {ph}) = bindFor AllD ManyOf ph p :: predDelta p
  nounDelta (EachOf grp) = nounDelta grp
  nounDelta (YouAnd n) = nounDelta n
  nounDelta (LibrarySlice pos amt whose) =
    MkBinding TheD Object (outputPlur (nounPlur whose) (amtPlur amt))
              (ObjectP Nothing (Just Library) Nothing Nothing)
      :: nounDelta whose
  nounDelta (SomeOf q grp) =
    MkBinding PartD Object (quantPlur q) (ObjectP (nounTy grp) (nounZone grp) Nothing Nothing)
      :: nounDelta grp
  nounDelta TheRest = []
  nounDelta It = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (That w) = []
  nounDelta (AttachHost _ _) = []
  nounDelta (Those w) = []
  nounDelta (TheVerbed v w) = []
  nounDelta (ThoseVerbed v w) = []
  nounDelta (ControllerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n
  nounDelta (OwnerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n
  nounDelta (Designated _ _) = []

  public export
  elemIntro : {bs : Bindings} -> Noun bs Object -> Bindings
  elemIntro grp =
    MkBinding TheD Object OneOf (ObjectP (nounTy grp) (nounZone grp) Nothing Nothing)
      :: nomIntro grp

  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (AbilityOf n) = nounDelta n
  predDelta (ActivatedBy n) = nounDelta n
  predDelta (ControlledBy n) = nounDelta n
  predDelta (CastBy n) = nounDelta n
  predDelta (BlockerOf m) = nounDelta m
  predDelta (BlockedBy m) = nounDelta m
  predDelta (HappenedTo _ _ {what}) = complementDelta what
  predDelta (CastFrom z) = zoneDelta z
  predDelta (ColorIs _) = []
  predDelta IsColorless = []
  predDelta Multicolored = []
  predDelta Monocolored = []
  predDelta (HasSupertype _) = []
  predDelta (Named src) = nameSrcDelta src
  predDelta (HasDesignation _) = []
  predDelta (IsAttached _) = []
  predDelta (InZone z) = zoneDelta z
  predDelta (And ps) = predDeltaAll ps
  predDelta (Not p) = []
  predDelta (Or ps) = []
  predDelta (OtherThan n) = nounDelta n
  predDelta (Compare _ _ b) = amtDelta b
  predDelta (Superlative _ _ d) = predDelta d
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

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (ZoneAt z (OwnedBy n)) = nounDelta n
  zoneDelta (ZoneAt z Bare) = []
  zoneDelta (LibraryAt pl _ (OwnedBy n)) = placeDelta pl ++ nounDelta n
  zoneDelta (LibraryAt pl _ Bare) = placeDelta pl

  ||| What a search clause names: one zone, or the graveyard-hand-library
  ||| sweep written once against its possessor and shared by both name
  ||| families. Each named zone is searched per [CR#701.23a].
  public export
  data SearchScope : Bindings -> Type where
    OneZone : (z : ZoneExpr bs) -> SearchScope bs
    GraveyardHandLibraryOf : (whose : Noun bs Player) -> SearchScope bs

  ||| The sweep fixes no single zone for what it finds.
  public export
  searchZone : {0 bs : Bindings} -> SearchScope bs -> Maybe Zone
  searchZone (OneZone z) = Just (zoneSort z)
  searchZone (GraveyardHandLibraryOf _) = Nothing

  public export
  searchDelta : {bs : Bindings} -> SearchScope bs -> List Binding
  searchDelta (OneZone z) = zoneDelta z
  searchDelta (GraveyardHandLibraryOf whose) = nounDelta whose

  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro n = nounDelta n ++ bs

  public export
  complementDelta : {bs : Bindings} -> {0 ev : EventName} -> {0 ks : Kind} ->
                    Maybe (EventComplement bs ev ks) -> List Binding
  complementDelta Nothing = []
  complementDelta (Just (Involving what)) = nounDelta what

  public export
  complementAnyTargetFree : {0 bs : Bindings} -> {0 ev : EventName} ->
                            {0 ks : Kind} ->
                            Maybe (EventComplement bs ev ks) -> Bool
  complementAnyTargetFree Nothing = True
  complementAnyTargetFree (Just (Involving what)) = nounAnyTargetFree what


  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PlayerStatOf : (w : PlayerStat) -> (n : Noun bs Player) ->
                   {auto 0 one : nounPlur n = OneOf} -> Amount bs
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    CountOf : {k : Kind} -> (p : Predicate bs k) ->
              {auto 0 af : AnyTargetFree p} -> Amount bs
    Aggregate : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                (p : Predicate bs k) ->
                {auto 0 sc : projScope ax = k} ->
                {auto 0 af : AnyTargetFree p} -> Amount bs
    CountersOn : {k : Kind} -> (kind : CounterKind) -> (holder : Noun bs k) ->
                 {auto 0 sc : counterScope kind = k} ->
                 {auto 0 one : nounPlur holder = OneOf} -> Amount bs
    EventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
                 (w : Lookback) ->
                 {default Nothing what :
                    Maybe (EventComplement (nomIntro who) ev k)} ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
    Times : (per : Nat) -> (a : Amount bs) ->
            {auto 0 nz : IsSucc per} -> Amount bs
    ThatMuch : {auto 0 ok : countOnes Outcome bs = 1} -> Amount bs
    PreventedThisWay : {auto 0 ok : countOutcomes DamagePrevented bs = 1} ->
                       Amount bs
    GroupSize : {auto 0 ok : countManysAny bs = 1} -> Amount bs
    TheDifference : {auto 0 ok : countOnes Gap bs = 1} -> Amount bs
    DefinedLetter : (w : LetterWord) ->
                    {auto 0 ok : countLetter w bs = 1} -> Amount bs
    XVal : Amount bs
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    Minus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs

  public export
  amtDelta : {bs : Bindings} -> Amount bs -> List Binding
  amtDelta (Lit _) = []
  amtDelta (StatOf _ nom) = nounDelta nom
  amtDelta (PlayerStatOf _ nom) = nounDelta nom
  amtDelta (CountersOn _ holder) = nounDelta holder
  amtDelta (EventCount _ who _ {what}) = nounDelta who ++ complementDelta what
  amtDelta (CountOf p) = predDelta p
  amtDelta (Aggregate _ _ p) = predDelta p
  amtDelta (Times _ a) = amtDelta a
  amtDelta ThatMuch = []
  amtDelta PreventedThisWay = []
  amtDelta GroupSize = []
  amtDelta TheDifference = []
  amtDelta (DefinedLetter _) = []
  amtDelta XVal = []
  amtDelta (Plus a b) = amtDelta a ++ amtDelta b
  amtDelta (Minus a b) = amtDelta a ++ amtDelta b

  public export
  amtAnyTargetFree : {0 bs : Bindings} -> Amount bs -> Bool
  amtAnyTargetFree (Lit _) = True
  amtAnyTargetFree (StatOf _ n) = nounAnyTargetFree n
  amtAnyTargetFree (PlayerStatOf _ n) = nounAnyTargetFree n
  amtAnyTargetFree (CountersOn _ holder) = nounAnyTargetFree holder
  amtAnyTargetFree (EventCount _ who _ {what}) =
    nounAnyTargetFree who && complementAnyTargetFree what
  amtAnyTargetFree (CountOf p) = anyTargetFree p
  amtAnyTargetFree (Aggregate _ _ p) = anyTargetFree p
  amtAnyTargetFree (Times _ a) = amtAnyTargetFree a
  amtAnyTargetFree ThatMuch = True
  amtAnyTargetFree PreventedThisWay = True
  amtAnyTargetFree GroupSize = True
  amtAnyTargetFree TheDifference = True
  amtAnyTargetFree (DefinedLetter _) = True
  amtAnyTargetFree XVal = True
  amtAnyTargetFree (Plus a b) = amtAnyTargetFree a && amtAnyTargetFree b
  amtAnyTargetFree (Minus a b) = amtAnyTargetFree a && amtAnyTargetFree b

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (PlayerStatOf w nom) = nomIntro nom
  amtIntro (CountersOn _ holder) = nomIntro holder
  amtIntro (EventCount _ who _ {what}) = complementDelta what ++ nomIntro who
  amtIntro (CountOf p) = predDelta p ++ bs
  amtIntro (Aggregate _ _ p) = predDelta p ++ bs
  amtIntro (Times per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro PreventedThisWay = bs
  amtIntro GroupSize = bs
  amtIntro TheDifference = bs
  amtIntro (DefinedLetter _) = bs
  amtIntro XVal = bs
  amtIntro (Plus a b) = amtIntro b
  amtIntro (Minus a b) = amtIntro b

  public export
  amtPlur : {0 bs : Bindings} -> Amount bs -> Plurality
  amtPlur (Lit (S Z)) = OneOf
  amtPlur (Lit _) = ManyOf
  amtPlur (StatOf _ _) = ManyOf
  amtPlur (PlayerStatOf _ _) = ManyOf
  amtPlur (CountersOn _ _) = ManyOf
  amtPlur (EventCount _ _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Aggregate _ _ _) = ManyOf
  amtPlur (Times _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur PreventedThisWay = ManyOf
  amtPlur GroupSize = ManyOf
  amtPlur TheDifference = ManyOf
  amtPlur (DefinedLetter _) = ManyOf
  amtPlur XVal = ManyOf
  amtPlur (Plus _ _) = ManyOf
  amtPlur (Minus _ _) = ManyOf

  public export
  writtenBound : {0 bs : Bindings} -> Amount bs -> Bool
  writtenBound (Lit _) = True
  writtenBound (StatOf _ _) = False
  writtenBound (PlayerStatOf _ _) = False
  writtenBound (CountersOn _ _) = False
  writtenBound (EventCount _ _ _) = False
  writtenBound (CountOf _) = False
  writtenBound (Aggregate _ _ _) = False
  writtenBound (Times _ _) = False
  writtenBound ThatMuch = False
  writtenBound PreventedThisWay = False
  writtenBound GroupSize = False
  writtenBound TheDifference = False
  writtenBound (DefinedLetter _) = False
  writtenBound XVal = True
  writtenBound (Plus _ _) = False
  writtenBound (Minus _ _) = False

  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq XVal XVal = True
  boundEq _ _ = False

  public export
  data LifeOp : Bindings -> Type where
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
    Down : Amount bs -> LifeOp bs   -- "loses [amt] life"
    Set : Amount bs -> LifeOp bs    -- "[whose] life total becomes [amt]"

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a
  lifeIntro (Set a) = amtIntro a

  public export
  readAmount : {0 bs : Bindings} -> Amount bs -> Bool
  readAmount (Lit _) = False
  readAmount (StatOf _ _) = True
  readAmount (PlayerStatOf _ _) = True
  readAmount (CountersOn _ _) = True
  readAmount (EventCount _ _ _) = True
  readAmount (CountOf _) = True
  readAmount (Aggregate _ _ _) = True
  readAmount (Times _ _) = False
  readAmount ThatMuch = False
  readAmount PreventedThisWay = False
  readAmount GroupSize = False
  readAmount TheDifference = True
  readAmount (DefinedLetter _) = False
  readAmount XVal = False
  readAmount (Plus _ _) = False
  readAmount (Minus _ _) = False

  public export
  ReadAmount : Amount bs -> Type
  ReadAmount {bs} a = So (readAmount a)

  public export
  Bindingless : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Bindingless {bs} {k} n = nounDelta n = []

  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase This = True
  anchorPhrase (AsType t n) = anchorPhrase n
  anchorPhrase You = True
  anchorPhrase (PlayerGroup _) = True
  anchorPhrase (Each _) = False
  anchorPhrase (Indefinite _ _) = False
  anchorPhrase (Definite _) = False
  anchorPhrase (TargetGroup _ _) = True
  anchorPhrase (CountedGroup _ _) = False
  anchorPhrase (AllOf _) = False
  anchorPhrase (EachOf _) = False
  anchorPhrase (YouAnd _) = False
  anchorPhrase (LibrarySlice _ _ _) = False
  anchorPhrase (SomeOf _ _) = False
  anchorPhrase TheRest = False
  anchorPhrase It = True
  anchorPhrase They = True
  anchorPhrase Them = True
  anchorPhrase (Those _) = True
  anchorPhrase (That _) = True
  anchorPhrase (AttachHost _ _) = True
  anchorPhrase (TheVerbed _ _) = True
  anchorPhrase (ThoseVerbed _ _) = True
  anchorPhrase (ControllerOf _) = True
  anchorPhrase (OwnerOf _) = True
  anchorPhrase (Designated _ _) = True

  public export
  data ComplementAnchor : Noun bs k -> Type where
    MkComplementAnchor : {auto 0 sh : So (anchorPhrase n)} ->
                         {auto 0 one : nounPlur n = OneOf} ->
                         ComplementAnchor n

  public export
  data LinkSource : Noun bs k -> Type where
    SelfLinked : LinkSource This
    SortedSelfLinked : {0 t : CardType} -> {0 asc : Ascribable This} ->
                       {0 way : So (ascriptionOk t Nothing)} ->
                       LinkSource (AsType t This {sub = Nothing} {asc} {way})

  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable This = False
  choosable (AsType _ _) = False
  choosable You = False
  choosable (PlayerGroup _) = False
  choosable (Each _) = False
  choosable (Indefinite _ _) = True
  choosable (Definite _) = False
  choosable (TargetGroup _ _) = True
  choosable (CountedGroup _ _) = True
  choosable (AllOf _) = False
  choosable (EachOf _) = False
  choosable (YouAnd _) = False
  choosable (LibrarySlice _ _ _) = False
  choosable (SomeOf _ _) = False
  choosable TheRest = False
  choosable It = False
  choosable They = False
  choosable Them = False
  choosable (Those _) = False
  choosable (That _) = False
  choosable (AttachHost _ _) = False
  choosable (TheVerbed _ _) = False
  choosable (ThoseVerbed _ _) = False
  choosable (ControllerOf _) = False
  choosable (OwnerOf _) = False
  choosable (Designated _ _) = False

  public export
  Choosable : Noun bs k -> Type
  Choosable {bs} {k} n = So (choosable n)

  public export
  agentChoosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  agentChoosable (SomeOf _ _) = True
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
  groupMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  groupMention (TargetGroup _ _) = True
  groupMention Them = True
  groupMention (Those _) = True
  groupMention This = False
  groupMention (AsType _ _) = False
  groupMention You = False
  groupMention (PlayerGroup _) = False
  groupMention (Each _) = False
  groupMention (Indefinite _ _) = False
  groupMention (Definite _) = False
  groupMention (CountedGroup _ _) = False
  groupMention (AllOf _) = False
  groupMention (EachOf _) = False
  groupMention (YouAnd _) = False
  groupMention (LibrarySlice _ _ _) = True
  groupMention (SomeOf _ _) = False
  groupMention TheRest = False
  groupMention It = False
  groupMention They = False
  groupMention (That _) = False
  groupMention (AttachHost _ _) = False
  groupMention (TheVerbed _ _) = False
  groupMention (ThoseVerbed _ _) = True
  groupMention (ControllerOf _) = False
  groupMention (OwnerOf _) = False
  groupMention (Designated _ _) = False

  public export
  GroupMention : Noun bs k -> Type
  GroupMention {bs} {k} n = So (groupMention n)

  public export
  perMemberOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  perMemberOk (Each _) = True
  perMemberOk (EachOf _) = True
  perMemberOk (AllOf _) = True
  perMemberOk n = isOne (nounPlur n)

  public export
  PerMember : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  PerMember {bs} {k} n = So (perMemberOk n)

  public export
  LandSubject : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  LandSubject {bs} {k} n = So (tyIs Land (nounTy n))

  public export
  data ActSubject : {0 k : Kind} -> ObjectAct -> Noun bs k -> Type where
    CounteredOnStack : {0 n : Noun bs Object} ->
                       {auto 0 zn : ZoneFits (nounZone n) (Just Stack)} ->
                       ActSubject Countered n
    CastOnStack : {0 n : Noun bs Object} ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Stack)} ->
                  ActSubject Cast n
    CopiedOnStack : {0 n : Noun bs Object} ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Stack)} ->
                    ActSubject Copied n
    PlayedIsLand : {0 n : Noun bs Object} -> ActSubject Played n
    ActivatedIsAbility : {0 n : Noun bs Ability} -> ActSubject Activated n
    RegeneratedOnField : {0 n : Noun bs Object} ->
                         {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                         ActSubject Regenerated n

  ||| The possessor of a relation an object can hold to only ONE player:
  ||| [CR#110.2] gives a permanent one controller, the player under whose
  ||| control it entered, [CR#601.2a] one caster, and [CR#602.2a] one
  ||| activator ("its controller is the player who activated the
  ||| ability"), which is the `ActivatedBy` row. A group word
  ||| distributes — each member holds the relation to its own objects — so
  ||| "creatures players control" names a set. A counted plural does not
  ||| distribute: it asks for the object all of a named two control at
  ||| once, and [CR#110.2] leaves that empty.
  public export
  soleHolderOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  soleHolderOk (PlayerGroup _) = True
  soleHolderOk n = isOne (nounPlur n)

  public export
  SoleHolder : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SoleHolder {bs} {k} n = So (soleHolderOk n)

  public export
  slicePossessorOk : {bs : Bindings} -> Noun bs Player -> Bool
  slicePossessorOk (Each _) = True
  slicePossessorOk (PlayerGroup _) = False
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
    AbilityCostSubject : {0 n : Noun bs Ability} -> CostSubject n

  public export
  selfDefinedOk : {bs : Bindings} -> Noun bs Object -> Bool
  selfDefinedOk This = True
  selfDefinedOk (AsType _ n) = selfDefinedOk n
  selfDefinedOk _ = False

  public export
  SelfDefined : {bs : Bindings} -> Noun bs Object -> Type
  SelfDefined {bs} n = So (selfDefinedOk n)

  public export
  data Condition : Bindings -> Type where
    Exists : {k : Kind} -> (p : Predicate bs k) ->
             {auto 0 af : AnyTargetFree p} -> Condition bs
    Happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               {default Nothing what :
                  Maybe (EventComplement (nomIntro who) ev k)} ->
               {auto 0 cw : ComplementWritten what} ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
    GameIs : (d : Designation) ->
             {auto 0 sc : designationScope d = HeldByGame} ->
             {auto 0 at : So (designationChecked d)} -> Condition bs
    ||| "there is no monarch" [CR#725.1]: an existential over the holder,
    ||| asking whether ANY player holds the designation where
    ||| `HasDesignation` describes one that does. Only the absence is
    ||| written, so the absence is the whole condition.
    NoHolder : (d : Designation) ->
               {auto 0 sc : designationScope d = HeldBy Player} ->
               {auto 0 at : So (designationChecked d)} -> Condition bs
    Matches : {k : Kind} -> (n : Noun bs k) -> (p : Predicate bs k) ->
              {auto 0 bl : Bindingless n} ->
              {auto 0 sy : PredSays p} ->
              {auto 0 zc : ZoneFits (nounZone n) (seedZone p)} ->
              {auto 0 af : AnyTargetFree p} -> Condition bs
    CompareAmt : (subj : Amount bs) -> (r : Comparator) -> (bound : Amount bs) ->
                 {auto 0 rd : ReadAmount subj} ->
                 Condition bs
    NotCond : (c : Condition bs) -> Condition bs
    AndCond : (cs : List (Condition bs)) ->
              {auto 0 tw : TwoConjuncts cs} ->
              {auto 0 fl : FlatConjuncts cs} -> Condition bs

  public export
  atLeastTwoCs : {0 bs : Bindings} -> List (Condition bs) -> Bool
  atLeastTwoCs [] = False
  atLeastTwoCs (_ :: []) = False
  atLeastTwoCs (_ :: _ :: _) = True

  public export
  TwoConjuncts : List (Condition bs) -> Type
  TwoConjuncts {bs} cs = So (atLeastTwoCs cs)

  public export
  isCondCoord : {0 bs : Bindings} -> Condition bs -> Bool
  isCondCoord (AndCond _) = True
  isCondCoord _ = False

  public export
  flatConjuncts : {0 bs : Bindings} -> List (Condition bs) -> Bool
  flatConjuncts [] = True
  flatConjuncts (c :: cs) = not (isCondCoord c) && flatConjuncts cs

  public export
  FlatConjuncts : List (Condition bs) -> Type
  FlatConjuncts {bs} cs = So (flatConjuncts cs)

  public export
  condNegated : {0 bs : Bindings} -> Condition bs -> Bool
  condNegated (Exists _) = False
  condNegated (Happened _ _ _) = False
  condNegated (GameIs _) = False
  -- atomic: the absence is the condition's own content, not a marked
  -- negation of one.
  condNegated (NoHolder _) = False
  condNegated (Matches _ _) = False
  condNegated (CompareAmt _ _ _) = False
  condNegated (NotCond _) = True
  condNegated (AndCond _) = False

  public export
  markingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Bool
  markingOk AsLongAs _ = True
  markingOk Unless c = condNegated c

  public export
  data MarkingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Type where
    MkMarkingOk : {0 c : Condition bs} ->
                  {auto 0 ok : So (markingOk m c)} -> MarkingOk m c

  ||| What a condition introduces for the clause it governs to read: the
  ||| statement's own subject re-mentioned, so the body may say "it"
  ||| (Adanto Vanguard); a comparison's margin, "the difference"; and the
  ||| phrases a comparison's two amounts name — a target inside a
  ||| condition is announced at casting like any other [CR#601.2c]. A
  ||| described set, a lookback, a designation check and a negation
  ||| introduce nothing: they test, and name no referent.
  public export
  condDelta : {bs : Bindings} -> Condition bs -> List Binding
  condDelta (Exists _) = []
  condDelta (Happened _ _ _) = []
  condDelta (GameIs _) = []
  condDelta (NoHolder _) = []
  condDelta (Matches (AsType t This) _) =
    [MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  condDelta (Matches (AttachHost _ (TypeW t)) _) =
    [MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  condDelta (Matches (AttachHost _ PermanentW) _) =
    [MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing)]
  condDelta (Matches _ _) = []
  condDelta (CompareAmt subj _ bound) = gapB :: (amtDelta bound ++ amtDelta subj)
  condDelta (NotCond _) = []
  condDelta (AndCond cs) = condDeltaAll cs

  public export
  condDeltaAll : {bs : Bindings} -> List (Condition bs) -> List Binding
  condDeltaAll [] = []
  condDeltaAll (c :: cs) = condDelta c ++ condDeltaAll cs

  public export
  data Duration : Bindings -> Type where
    ThisTurn : Duration bs
    ||| [CR#702.131a] and [CR#702.195a] both write "for the rest of the
    ||| game", which no turn-part endpoint spells.
    RestOfGame : Duration bs
    Until : DurationEnd -> Duration bs
    ForAsLongAs : Condition bs -> Duration bs
    UntilEvent : GameEvent bs -> Duration bs

  public export
  data AttackDefender : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoDefender : AttackDefender Nothing
    OneDefender : {0 m : Noun bs Player} ->
                  {auto 0 sg : nounPlur m = OneOf} -> AttackDefender (Just m)

  public export
  data BlockPartner : {0 bs : Bindings} -> Maybe (Noun bs Object) -> Type where
    NoPartner : BlockPartner Nothing
    OnePartner : {0 m : Noun bs Object} ->
                 {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} -> BlockPartner (Just m)

  public export
  data EventAgent : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    AgentUnvoiced : EventAgent Nothing
    AgentVoiced : {0 w : Noun bs Player} ->
                  {auto 0 bl : Bindingless w} -> EventAgent (Just w)

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
  data TokenPhrase : {0 bs : Bindings} -> Noun bs Object -> Type where
    CountedTokens : {0 q : Quantity} -> {0 p : Predicate bs Object} ->
                    {0 ph : Phrasal Object} -> {0 nz : NonZeroQ q} ->
                    {0 wf : WellFormedQ q} -> {0 af : AnyTargetFree p} ->
                    {auto 0 ok : So (seedsToken p)} ->
                    TokenPhrase (CountedGroup q p {ph} {nz} {wf} {af})
    OneToken : {0 m : ChoiceMode bs} -> {0 p : Predicate bs Object} ->
               {0 ph : Phrasal Object} -> {0 af : AnyTargetFree p} ->
               {auto 0 ok : So (seedsToken p)} ->
               TokenPhrase (Indefinite m p {ph} {af})

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
              {default Nothing whom : Maybe (Noun (nomIntro n) Player)} ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              {auto 0 df : AttackDefender whom} -> GameEvent bs
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
                         {default Nothing by : Maybe (Noun bs Player)} ->
                         {auto 0 sc : counterScope kind = Object} ->
                         {auto 0 ag : EventAgent by} -> GameEvent bs
    PutInto : (n : Noun bs Object) -> (to : ZoneExpr bs) ->
              {default Nothing from : Maybe (EventSource bs)} ->
              {auto 0 dk : PutDest to} ->
              {auto 0 sk : PutSource from} ->
              {auto 0 zn : ZoneFits (nounZone n) (sourceZone from)} -> GameEvent bs
    CounterEvent : (dir : CounterMove) -> (kind : CounterKind) ->
                   (n : Noun bs Object) ->
                   {default ManyCounters many : CounterBatch} ->
                   {default Nothing by : Maybe (Noun bs Player)} ->
                   {default Nothing cause : Maybe Causer} ->
                   {auto 0 sc : counterScope kind = Object} ->
                   {auto 0 ag : EventAgent by} ->
                   {auto 0 cz : CausedBy cause by} -> GameEvent bs
    TokensCreated : (n : Noun bs Object) ->
                    {default Nothing cause : Maybe Causer} ->
                    {default Nothing by : Maybe (Noun bs Player)} ->
                    {default Nothing under : Maybe (Noun bs Player)} ->
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

  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (Dies _) = Death
  eventName (Leaves _) = Departure
  eventName (IsDestroyed _) = Destruction
  eventName (IsDealtDamage _) = DamageTaken
  eventName (Draws _) = CardDrawn
  eventName (LosesGame _) = GameLoss
  eventName (Enters _) = Entry
  eventName (Attacks _) = AttackDeclaration
  eventName (Blocks _ _) = BlockDeclaration
  eventName (BecomesBlocked _ _) = BlockedDeclaration
  eventName (DealsCombatDamage _ _) = CombatDamage
  eventName (BeginningOf _ _) = PartBeginning
  eventName (Casts _ _) = SpellCast
  eventName (StatusEvent {c} _ _) = statusEventName c
  eventName DayNightShift = TimeShift
  eventName (LastCounterRemoved _ _) = LastCounterRemoval
  eventName (PutInto _ _) = Placement
  eventName (CounterEvent dir _ _) = counterEventName dir
  eventName (TokensCreated _) = TokenCreation
  eventName (ChapterMark _) = ChapterArrival
  eventName (Activates _ _) = AbilityActivation
  eventName (StatBecomes _ _ _) = StatValueChange
  eventName (Regenerates _) = Regeneration

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
  eventIntro (Attacks n {whom = Nothing}) = nomIntro n
  eventIntro (Attacks _ {whom = Just whom}) = nomIntro whom
  eventIntro (Blocks n Nothing) = nomIntro n
  eventIntro (Blocks _ (Just what)) = nomIntro what
  eventIntro (BecomesBlocked n Nothing) = nomIntro n
  eventIntro (BecomesBlocked _ (Just by)) = nomIntro by
  eventIntro (DealsCombatDamage n to) = nomIntro to
  eventIntro (BeginningOf _ _) = bs
  eventIntro (Casts _ what) = nomIntro what
  eventIntro (StatusEvent n _) = nomIntro n
  eventIntro DayNightShift = bs
  eventIntro (LastCounterRemoved _ n) = nomIntro n
  eventIntro (PutInto n _) = nomIntro n
  eventIntro (CounterEvent _ _ n {many = OneCounter}) = nomIntro n
  eventIntro (CounterEvent _ _ n {many = ManyCounters}) =
    outcomeB CountersPut :: nomIntro n
  eventIntro (TokensCreated n) = nomIntro n
  eventIntro (ChapterMark _) = bs
  eventIntro (Activates _ what) = nomIntro what
  eventIntro (StatBecomes _ _ v) = amtIntro v
  eventIntro (Regenerates n) = nomIntro n

  public export
  selfSubjIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  selfSubjIntro (AsType t This) =
    MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  selfSubjIntro (AttachHost _ (TypeW t)) =
    MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  selfSubjIntro (AttachHost _ PermanentW) =
    MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing) :: bs
  selfSubjIntro (AttachHost _ PlayerW) = MkBinding TheD Player OneOf PlayerP :: bs
  selfSubjIntro n = nomIntro n

  public export
  condIntro : {bs : Bindings} -> Condition bs -> Bindings
  condIntro c = condDelta c ++ bs

  public export
  interveningIntro : {bs : Bindings} -> Maybe (Condition bs) -> Bindings
  interveningIntro Nothing = bs
  interveningIntro (Just c) = condIntro c

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
  eventAfter (Attacks n {whom = Nothing}) = selfSubjIntro n
  eventAfter (Attacks n {whom = Just whom}) = nounDelta whom ++ selfSubjIntro n
  eventAfter (Blocks n Nothing) = selfSubjIntro n
  eventAfter (Blocks _ (Just what)) = nomIntro what
  eventAfter (BecomesBlocked n Nothing) = selfSubjIntro n
  eventAfter (BecomesBlocked _ (Just by)) = nomIntro by
  eventAfter (DealsCombatDamage n to) = outcomeB DamageDealt :: nomIntro to
  eventAfter (Casts _ what) = nomIntro what
  eventAfter (BeginningOf _ whose) = possessorIntro whose
  eventAfter (StatusEvent n _) = selfSubjIntro n
  eventAfter DayNightShift = bs
  eventAfter (LastCounterRemoved _ n) = selfSubjIntro n
  eventAfter (PutInto n to) = moveIntro Nothing n (Just (zoneSort to))
  eventAfter (CounterEvent _ _ n {many = OneCounter}) = selfSubjIntro n
  eventAfter (CounterEvent _ _ n {many = ManyCounters}) =
    outcomeB CountersPut :: selfSubjIntro n
  eventAfter (TokensCreated n) = nomIntro n
  eventAfter (ChapterMark _) = bs
  eventAfter (Activates _ what) = nomIntro what
  eventAfter (StatBecomes n _ v) = amtDelta v ++ selfSubjIntro n
  eventAfter (Regenerates n) = selfSubjIntro n

  public export
  eventSubjectPlur : {bs : Bindings} -> GameEvent bs -> Plurality
  eventSubjectPlur (Dies n) = nounPlur n
  eventSubjectPlur (Leaves n) = nounPlur n
  eventSubjectPlur (IsDestroyed n) = nounPlur n
  eventSubjectPlur (IsDealtDamage to) = nounPlur to
  eventSubjectPlur (Draws who) = nounPlur who
  eventSubjectPlur (LosesGame who) = nounPlur who
  eventSubjectPlur (Enters n) = nounPlur n
  eventSubjectPlur (Attacks n) = nounPlur n
  eventSubjectPlur (Blocks n _) = nounPlur n
  eventSubjectPlur (BecomesBlocked n _) = nounPlur n
  eventSubjectPlur (DealsCombatDamage n _) = nounPlur n
  eventSubjectPlur (BeginningOf _ _) = OneOf
  eventSubjectPlur (Casts _ what) = nounPlur what
  eventSubjectPlur (StatusEvent n _) = nounPlur n
  eventSubjectPlur DayNightShift = OneOf
  eventSubjectPlur (LastCounterRemoved _ n) = nounPlur n
  eventSubjectPlur (PutInto n _) = nounPlur n
  eventSubjectPlur (CounterEvent _ _ n) = nounPlur n
  eventSubjectPlur (TokensCreated n) = nounPlur n
  eventSubjectPlur (ChapterMark _) = OneOf
  eventSubjectPlur (Activates who _) = nounPlur who
  eventSubjectPlur (StatBecomes n _ _) = nounPlur n
  eventSubjectPlur (Regenerates n) = nounPlur n

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
  playSourceOk : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) ->
                 Maybe PlayAsThough -> Bool
  playSourceOk zn Nothing Nothing = playableFrom zn
  playSourceOk zn (Just z) Nothing =
    playableFrom (Just (zoneSort z)) &&
    (not (complementLocates zn) || zoneFits zn (Just (zoneSort z)))
  playSourceOk zn Nothing (Just HadFlash) = castComplementOk zn
  playSourceOk zn (Just z) (Just HadFlash) =
    castComplementOk zn && playableFrom (Just (zoneSort z))

  public export
  data PlaySource : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) ->
                    Maybe PlayAsThough -> Type where
    MkPlaySource : {0 fz : Maybe (ZoneExpr bs)} ->
                   {auto 0 ok : So (playSourceOk zn fz at)} -> PlaySource zn fz at

  public export
  record TokenChars (bs : Bindings) where
    constructor MkToken
    pt : Maybe (Amount bs, Amount bs)
    colors : List Color
    line : TypeLine
    abilities : List (AbilityAt [])
    name : Maybe String

  public export
  tokenTyped : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenTyped t = lineNonEmpty (MkTypeLine [] t.line.tys)

  public export
  ptWritten : {0 bs : Bindings} -> Maybe (Amount bs, Amount bs) -> Bool
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

  public export
  data StaticEffect : Bindings -> Type where
    Gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
           (tou : PtShift (nomIntro n)) ->
           {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
           StaticEffect bs
    DefinesPt : (n : Noun bs Object) -> (sl : DefinedSlots) ->
                (amt : Amount (nomIntro n)) ->
                {auto 0 sd : SelfDefined n} ->
                StaticEffect bs
    HasBasePt : (n : Noun bs Object) -> (pow : Amount bs) -> (tou : Amount bs) ->
                StaticEffect bs
    SwitchesPt : (n : Noun bs Object) ->
                 {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                 StaticEffect bs
    CostsToCast : {k : Kind} -> (n : Noun bs k) -> (sh : CostShift bs) ->
                  {auto 0 cs : CostSubject n} ->
                  StaticEffect bs
    AltCost : (c : Maybe (Cost bs)) ->
              {auto 0 ap : AltPayment c} -> StaticEffect bs
    WhereLetterStatic : (w : LetterWord) -> (def : Amount bs) ->
                        (se : StaticEffect (Experimental.Words.letterB w :: bs)) ->
                        StaticEffect bs
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
                        {auto 0 af : AnyTargetFree p} -> StaticEffect bs
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
                    {default AsLongAs marking : CondMarking} ->
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
                {default AsLongAs marking : CondMarking} ->
                {auto 0 nn : NotConditional se} ->
                {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
    MayPlay : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              {default Play verb : PlayVerb} ->
              {default Nothing from : Maybe (ZoneExpr (nomIntro what))} ->
              {default Nothing asThough : Maybe PlayAsThough} ->
              {default Nothing limit : Maybe PlayLimit} ->
              {default Nothing window : Maybe PlayWindow} ->
              {auto 0 pz : PlaySource (nounZone what) from asThough} ->
              {auto 0 cv : CastableTy verb (nounTy what)} -> StaticEffect bs
    Visibility : (v : ExposeVerb) -> (who : Noun bs Player) ->
                 (what : VisibleThing) ->
                 {auto 0 vo : VisibilityOk v what} -> StaticEffect bs
    MayPlayAdditionalLands : (who : Noun bs Player) -> (q : Quantity) ->
                             {auto 0 nz : NonZeroQ q} ->
                             {auto 0 wf : WellFormedQ q} -> StaticEffect bs
    ||| [CR#506.3a] and [CR#508.4d] both say what happens when a
    ||| permanent enters attacking, so either rider is a real entry.
    EntersRider : (n : Noun bs Object) -> (rider : TokenRider) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  StaticEffect bs
    EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                         (kind : CounterKind) ->
                         {default Fresh mark : EntryCounterMark} ->
                         StaticEffect bs
    EntersChoice : (n : Noun bs Object) -> (q : QualitySort) ->
                   {default Nothing dom : Maybe (ChoiceDomain q)} ->
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
    ||| The player an attack is aimed at [CR#506.3].
    DefendingPlayer : (m : Noun bs Player) -> DeonticPatient {bs} Attack Agent
    DeonticCounterpart : (m : Noun bs Object) ->
                         {auto 0 dp : DeedParticipant d (counterRole r) (nounTy m)} ->
                         {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                         DeonticPatient {bs} d r

  public export
  notConditional : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notConditional (Conditionally _ _) = False
  notConditional (OnlyWhile _ _) = False
  notConditional _ = True


  public export
  notLetterRider : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notLetterRider (WhereLetterStatic _ _ _) = False
  notLetterRider _ = True

  public export
  NotLetterRider : StaticEffect bs -> Type
  NotLetterRider {bs} se = So (notLetterRider se)

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
  staticKind (WhereLetterStatic _ _ se) = staticKind se
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
  staticKind (Conditionally _ _) = Conditional
  staticKind (OnlyWhile _ _) = Conditional
  staticKind (AlsoOffBattlefield se) = staticKind se
  staticKind (MayPlay _ _) = PlayPermission
  staticKind (Visibility _ _ _) = VisibilityRider
  staticKind (MayPlayAdditionalLands _ _) = LandAllowance
  staticKind (EntersRider _ _) = EntryRider
  staticKind (EntersWithCounters _ _ _) = EntryRider
  staticKind (EntersChoice _ _) = EntryRider
  staticKind (AndAlso _) = Coordination


  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (WhereLetterStatic _ _ se) = staticIntro se
  staticIntro (Gets n _ _) = selfSubjIntro n
  staticIntro (DefinesPt n _ _) = selfSubjIntro n
  staticIntro (HasBasePt n _ _) = selfSubjIntro n
  staticIntro (SwitchesPt n) = selfSubjIntro n
  staticIntro (CostsToCast n _) = selfSubjIntro n
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
  staticIntro (Conditionally c se) = staticIntro se
  staticIntro (OnlyWhile se c) = staticIntro se
  staticIntro (AlsoOffBattlefield se) = staticIntro se
  staticIntro (MayPlay who what) = selfSubjIntro what
  staticIntro (Visibility _ who _) = nomIntro who
  staticIntro (MayPlayAdditionalLands who _) = nomIntro who
  staticIntro (EntersRider n _) = selfSubjIntro n
  staticIntro (EntersWithCounters n _ _) = selfSubjIntro n
  staticIntro (EntersChoice n _) = selfSubjIntro n
  staticIntro (AndAlso parts) = partsIntro parts

  public export
  staticChoiceIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticChoiceIntro (EntersChoice _ q) = qualityB q :: bs
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
  data Exposed : Bindings -> Type where
    ExposedCards : (n : Noun bs Object) -> Exposed bs
    ExposedZone : (z : ZoneExpr bs) ->
                  {auto 0 ok : ExposableZone (zoneSort z)} -> Exposed bs

  public export
  exposedIntro : {bs : Bindings} -> Exposed bs -> Bindings
  exposedIntro (ExposedCards n) = nomIntro n
  exposedIntro (ExposedZone z) = zoneDelta z ++ bs

  public export
  data CounterRider : Bindings -> Type where
    MkCounterRider : (amt : Amount bs) -> (kind : CounterKind) ->
                     CounterRider bs

  public export
  data MoveRiders : Bindings -> Type where
    MkMoveRiders : (entry : List TokenRider) ->
                   (ctrl : Maybe (Noun bs Player)) ->
                   {default Nothing counters : Maybe (CounterRider bs)} ->
                   {auto 0 one : CtrlSingular ctrl} -> MoveRiders bs

  public export
  data CtrlSingular : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoOverride : CtrlSingular Nothing
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlSingular (Just n)

  public export
  fieldRidersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  fieldRidersWritten (MkMoveRiders [] Nothing) = False
  fieldRidersWritten _ = True

  public export
  counterRiderWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  counterRiderWritten (MkMoveRiders _ _ {counters = Nothing}) = False
  counterRiderWritten (MkMoveRiders _ _ {counters = Just _}) = True

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
  costIntro (Mana _) = bs
  costIntro (ScaledMana _) = bs
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (LoyaltySymbol LoyaltyDownX) = Experimental.Words.letterB LetterX :: bs
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
             {auto 0 af : AnyTargetFree p} -> SpendPurpose bs
    ToActivate : (src : Maybe (Predicate bs Object)) ->
                 {auto 0 sf : SpendSourceOk src} -> SpendPurpose bs

  public export
  spendSourceOk : {0 bs : Bindings} -> Maybe (Predicate bs Object) -> Bool
  spendSourceOk Nothing = True
  spendSourceOk (Just p) = anyTargetFree p

  public export
  data SpendSourceOk : {0 bs : Bindings} -> Maybe (Predicate bs Object) -> Type where
    MkSpendSourceOk : {0 src : Maybe (Predicate bs Object)} ->
                      {auto 0 ok : So (spendSourceOk src)} -> SpendSourceOk src

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
    ExceptPt : (pow : Amount bs) -> (tou : Amount bs) -> CopyExcept bs
    ExceptNonlegendary : CopyExcept bs
    ExceptColor : (c : Chroma.Color) -> CopyExcept bs

  public export
  data Repetition : Bindings -> Type where
    Again : Repetition bs
    MoreTimes : (n : Amount bs) ->
                Repetition bs
    AnyNumber : Repetition bs

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
                       {default Nothing span : Maybe (Duration (nomIntro n))} ->
                       {auto 0 sc : designationScope d = HeldBy k} ->
                       {auto 0 zn : DesignationHolder d (nounZone n)} -> Effect bs
    GameBecomes : (d : Designation) ->
                  {auto 0 sc : designationScope d = HeldByGame} ->
                  {auto 0 at : So (designationGiven d)} -> Effect bs
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Effect bs
    GameDrawn : Effect bs
    Choose : {k : Kind} -> (n : Noun bs k) ->
             {default Nothing by : Maybe (Noun bs Player)} ->
             {auto 0 ch : ChoiceClause by n} -> Effect bs
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           {default (MkMoveRiders [] Nothing) riders : MoveRiders (nomIntro what)} ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 na : NotPlayerSpanning what} ->
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
             {auto 0 af : AnyTargetFree p} ->
             {auto 0 zf : ZoneFree p} -> Effect bs
    Shuffle : (whose : Noun bs Player) -> Effect bs
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 nr : NotLetterRider se} ->
                   {auto 0 sp : SpanOk (staticKind se) span} ->
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
    RemoveCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (from : Noun (amtIntro amt) Object) ->
                     {auto 0 sc : counterScope kind = Object} ->
                     {auto 0 cm : CounterMemory from} -> Effect bs
    GetsCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                   (kind : CounterKind) ->
                   {auto 0 sc : counterScope kind = Player} -> Effect bs
    LosesAllCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                       {auto 0 pk : CounterKindNamed Player kind} -> Effect bs
    Composite : (v : VerbName) -> (e : Effect bs) ->
                {auto 0 ok : TagBody v e} -> {auto 0 na : NonAgentive v} -> Effect bs
    Does : (subj : Noun bs Player) -> (v : VerbName) ->
           (e : Effect (nomIntro subj)) ->
           {auto 0 tb : TagBody v e} -> Effect bs
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
    WhereLetter : (w : LetterWord) -> (def : Amount bs) ->
                  (body : Effect (Experimental.Words.letterB w :: bs)) -> Effect bs
    ForEachOf : (grp : Noun bs Object) ->
                (body : Effect (elemIntro grp)) ->
                {auto 0 pl : nounPlur grp = ManyOf} ->
                Effect bs
    Repeat : (rep : Repetition bs) -> Effect bs
    ||| A coordination of one member denotes that member, so only the
    ||| empty list is refused: it instructs nothing.
    Sequentially : {0 n : Nat} -> Effects n bs ->
                   {auto 0 ne : IsSucc n} -> Effect bs
    Simultaneously : {0 n : Nat} -> SimEffects n bs ->
                     {auto 0 ne : IsSucc n} -> Effect bs
    Modal : (q : Quantity) -> (modes : List (Effect bs)) ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit q (modeCount modes)} ->
            {auto 0 dm : So (distinctModes modes)} -> Effect bs
    Delayed : (ev : GameEvent bs) ->
              {default Nothing span : Maybe (Duration bs)} ->
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
                     {default Nothing followedBy : Maybe TurnPart} ->
                     {auto 0 ad : AddedPart part} ->
                     {auto 0 an : AnchorPart anchor} ->
                     {auto 0 fb : FollowerPart followedBy} -> Effect bs

  ||| [CR#610.3] hangs the "until" rider on a one-shot that changes an
  ||| object's zone, and on nothing else.
  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (DoesntUntapNext _ _) = False
  heldUntilOk (SkipsNext _ _ _) = False
  heldUntilOk (ExtraTurn _ _) = False
  heldUntilOk (AdditionalPart _ _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  heldUntilOk (SetStatus _ _) = False
  heldUntilOk (GetsCounters _ _ _) = False
  heldUntilOk (LosesAllCounters _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
  heldUntilOk (Regenerate _) = False
  heldUntilOk (CantBe _ _ _) = False
  heldUntilOk (GainsDesignation _ _ _) = False
  heldUntilOk (GameBecomes _) = False
  heldUntilOk (Concludes _ _) = False
  heldUntilOk GameDrawn = False
  heldUntilOk (CounterSpell _) = False
  heldUntilOk (CopyStack _ _ _ _) = False
  heldUntilOk (ChooseNewTargets _) = False
  heldUntilOk (Choose _) = False
  heldUntilOk (Move _ _) = True
  heldUntilOk (ChangeLife _ _) = False
  heldUntilOk (AddMana _ _ _ _) = False
  heldUntilOk (Draw _ _) = False
  heldUntilOk (Expose _ _ _) = False
  heldUntilOk (Search _ _ _) = False
  heldUntilOk (Shuffle _) = False
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (GetsEmblem _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  heldUntilOk (Composite _ (Move _ _)) = True
  heldUntilOk (Composite _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (OnlyIf _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (Unless _ _ _) = False
  heldUntilOk (WhereLetter _ _ _) = False
  heldUntilOk (ForEachOf _ _) = False
  heldUntilOk (Repeat _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _) = False
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
  reflexEncloseUse (AdditionalPart _ _ _) = EncAgentless
  reflexEncloseUse (Distribute _ _ _) = EncAgentless
  reflexEncloseUse (Fights _ _) = EncAgentless
  reflexEncloseUse (ChangeLife _ _) = EncAgentless
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncReflexive
  reflexEncloseUse (Continuously _ _) = EncAgentless
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _) = EncReflexive        -- 66, all of them offered
  reflexEncloseUse (Composite _ _) = EncReflexive  -- 51, every one an exile
  -- a status change is the effect's, not a player's: [CR#603.12]'s
  -- agent form has no subject to inflect.
  reflexEncloseUse (SetStatus _ _) = EncAgentless
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (LosesAllCounters _ _) = EncAgentless
  reflexEncloseUse (RemoveFromCombat _) = EncAgentless
  reflexEncloseUse (Regenerate _) = EncAgentless
  reflexEncloseUse (CantBe _ _ _) = EncAgentless
  reflexEncloseUse (GainsDesignation _ _ _) = EncAgentless
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
  reflexEncloseUse (Move _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (AddMana _ _ _ _) = EncReflexive
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _) = EncReflexive       -- 1
  reflexEncloseUse (Search _ _ _) = EncReflexive
  reflexEncloseUse (Shuffle _) = EncReflexive
  -- [CR#603.12] writes the reflexive over what a player did or didn't
  -- do, so a declined arm leaves one offered action to inflect.
  reflexEncloseUse (May _ body Nothing _) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  reflexEncloseUse (OnlyIf _ _ _) = EncNotOneAction
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (Unless _ _ _) = EncNotOneAction
  reflexEncloseUse (WhereLetter _ _ _) = EncNotOneAction
  reflexEncloseUse (ForEachOf _ _) = EncNotOneAction
  reflexEncloseUse (Repeat _) = EncNotOneAction
  reflexEncloseUse (Sequentially _) = EncNotOneAction
  reflexEncloseUse (Simultaneously _) = EncNotOneAction
  reflexEncloseUse (Modal _ _) = EncNotOneAction
  reflexEncloseUse (InsteadOf _ _) = EncNotOneAction
  reflexEncloseUse (Reflexively _ _) = EncNotOneAction
  reflexEncloseUse (ThisWay _ _ _) = EncNotOneAction
  reflexEncloseUse (Delayed _ _) = EncNotYetTaken
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
  thisWayOutcomeOk (Delayed _ _) = False
  -- an offer's outcome is its body's; the arms are separate sentences.
  thisWayOutcomeOk (May _ body _ _) = thisWayOutcomeOk body
  thisWayOutcomeOk (DoesntUntapNext _ _) = True
  thisWayOutcomeOk (SkipsNext _ _ _) = True
  thisWayOutcomeOk (ExtraTurn _ _) = True
  thisWayOutcomeOk (AdditionalPart _ _ _) = True
  thisWayOutcomeOk (HeldUntil _ _) = True
  thisWayOutcomeOk (DealDamage _ _ _) = True
  thisWayOutcomeOk (Distribute _ _ _) = True
  thisWayOutcomeOk (Fights _ _) = True
  thisWayOutcomeOk (SetStatus _ _) = True
  thisWayOutcomeOk (GetsCounters _ _ _) = True
  thisWayOutcomeOk (LosesAllCounters _ _) = True
  thisWayOutcomeOk (RemoveFromCombat _) = True
  thisWayOutcomeOk (Regenerate _) = True
  thisWayOutcomeOk (CantBe _ _ _) = True
  thisWayOutcomeOk (GainsDesignation _ _ _) = True
  thisWayOutcomeOk (GameBecomes _) = True
  thisWayOutcomeOk (Concludes _ _) = True
  thisWayOutcomeOk GameDrawn = True
  thisWayOutcomeOk (CounterSpell _) = True
  thisWayOutcomeOk (CopyStack _ _ _ _) = True
  thisWayOutcomeOk (ChooseNewTargets _) = True
  thisWayOutcomeOk (Choose _) = True
  thisWayOutcomeOk (Move _ _) = True
  thisWayOutcomeOk (ChangeLife _ _) = True
  thisWayOutcomeOk (AddMana _ _ _ _) = True
  thisWayOutcomeOk (Draw _ _) = True
  thisWayOutcomeOk (Expose _ _ _) = True
  thisWayOutcomeOk (Search _ _ _) = True
  thisWayOutcomeOk (Shuffle _) = True
  thisWayOutcomeOk (Continuously _ _) = True
  thisWayOutcomeOk (Create _ _ _ _) = True
  thisWayOutcomeOk (GetsEmblem _ _) = True
  thisWayOutcomeOk (PutCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCounters _ _ _) = True
  thisWayOutcomeOk (Composite _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (Pay _ _) = True
  thisWayOutcomeOk (OnlyIf _ _ _) = True
  thisWayOutcomeOk (If _ _ _) = True
  thisWayOutcomeOk (Unless _ _ _) = True
  thisWayOutcomeOk (WhereLetter _ _ _) = True
  thisWayOutcomeOk (ForEachOf _ _) = True
  thisWayOutcomeOk (Repeat _) = True
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

  public export
  costNounOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  costNounOk This = True
  costNounOk (AsType t n) = costNounOk n
  costNounOk You = True
  costNounOk (PlayerGroup _) = True
  costNounOk (Each _) = True
  costNounOk (Indefinite _ _) = True
  costNounOk (Definite _) = True
  costNounOk (TargetGroup _ _) = True
  costNounOk (CountedGroup _ _) = True
  costNounOk (AllOf _) = True
  costNounOk (EachOf grp) = costNounOk grp
  costNounOk (YouAnd _) = False
  costNounOk (LibrarySlice _ _ _) = True
  costNounOk (SomeOf _ grp) = costNounOk grp
  costNounOk TheRest = True
  costNounOk It = True
  costNounOk They = True
  costNounOk Them = True
  costNounOk (Those _) = True
  costNounOk (That _) = True
  costNounOk (AttachHost _ _) = True
  costNounOk (TheVerbed _ _) = False
  costNounOk (ThoseVerbed _ _) = False
  costNounOk (ControllerOf _) = True
  costNounOk (OwnerOf _) = True
  costNounOk (Designated _ _) = True

  public export
  nounIsYou : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsYou You = True
  nounIsYou (PlayerGroup _) = False
  nounIsYou This = False
  nounIsYou (AsType _ _) = False
  nounIsYou (Each _) = False
  nounIsYou (Indefinite _ _) = False
  nounIsYou (Definite _) = False
  nounIsYou (TargetGroup _ _) = False
  nounIsYou (CountedGroup _ _) = False
  nounIsYou (AllOf _) = False
  nounIsYou (EachOf _) = False
  nounIsYou (YouAnd _) = False
  nounIsYou (LibrarySlice _ _ _) = False
  nounIsYou (SomeOf _ _) = False
  nounIsYou TheRest = False
  nounIsYou It = False
  nounIsYou They = False
  nounIsYou Them = False
  nounIsYou (Those _) = False
  nounIsYou (That _) = False
  nounIsYou (AttachHost _ _) = False
  nounIsYou (TheVerbed _ _) = False
  nounIsYou (ThoseVerbed _ _) = False
  nounIsYou (ControllerOf _) = False
  nounIsYou (OwnerOf _) = False
  nounIsYou (Designated _ _) = False

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
  costActionOk (AdditionalPart _ _ _) = True
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (SetStatus _ n) = costNounOk n
  costActionOk (GetsCounters who _ _) = costNounOk who
  costActionOk (LosesAllCounters who _) = costNounOk who
  costActionOk (RemoveFromCombat n) = costNounOk n
  costActionOk (Regenerate n) = costNounOk n
  costActionOk (CantBe e _ _) = costActionOk e
  costActionOk (GainsDesignation n _ _) = costNounOk n
  costActionOk (GameBecomes _) = True
  costActionOk (Concludes _ _) = True
  costActionOk GameDrawn = True
  costActionOk (CounterSpell _) = True
  costActionOk (CopyStack _ what _ _) = costNounOk what
  costActionOk (ChooseNewTargets what) = costNounOk what
  costActionOk (Choose n) = costNounOk n
  costActionOk (Move what _) = costNounOk what
  costActionOk (ChangeLife _ _) = True
  costActionOk (AddMana who _ _ _) = costNounOk who
  costActionOk (Draw _ _) = True
  costActionOk (Expose _ who _) = costNounOk who
  costActionOk (Search who _ _) = costNounOk who
  costActionOk (Shuffle whose) = costNounOk whose
  costActionOk (Continuously _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (Composite _ e) = costActionOk e
  costActionOk (Does _ _ e) = costActionOk e
  costActionOk (Pay _ _) = False
  costActionOk (May _ body ifDid ifNot) =
    costActionOk body && costActionOkOpt ifDid && costActionOkOpt ifNot
  costActionOk (OnlyIf e _ otherwise) = costActionOk e && costActionOkOpt otherwise
  costActionOk (If _ e otherwise) = costActionOk e && costActionOkOpt otherwise
  -- an offer another player answers at resolution [CR#118.12a]
  costActionOk (Unless _ _ _) = False
  costActionOk (WhereLetter _ _ body) = costActionOk body
  costActionOk (ForEachOf _ body) = costActionOk body
  costActionOk (Repeat _) = False
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ modes) = costActionsOk modes
  costActionOk (Delayed _ _) = False
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
  effEq (AdditionalPart p a c) (AdditionalPart q b d) =
    p == q && a == b && boundEq c d
  effEq (AdditionalPart _ _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (SetStatus v a) (SetStatus w b) = sameStatusVal v w && nounEqRef a b
  effEq (SetStatus _ _) _ = False
  effEq (GetsCounters _ _ _) _ = False
  effEq (LosesAllCounters a Nothing) (LosesAllCounters b Nothing) = nounEqRef a b
  effEq (LosesAllCounters a (Just j)) (LosesAllCounters b (Just l)) =
    nounEqRef a b && j == l
  effEq (LosesAllCounters _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
  effEq (Regenerate a) (Regenerate b) = nounEqRef a b
  effEq (Regenerate _) _ = False
  effEq (CantBe _ _ _) _ = False
  effEq (GainsDesignation _ _ _) _ = False
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
  effEq (Choose _) _ = False
  effEq (Move a s) (Move b t) = nounEqRef a b && zoneSort s == zoneSort t
  effEq (Move _ _) _ = False
  effEq (ChangeLife _ _) _ = False
  effEq (AddMana _ _ _ _) _ = False
  effEq (Draw You a) (Draw You b) = boundEq a b
  effEq (Draw _ _) _ = False
  effEq (Expose _ _ _) _ = False
  effEq (Search _ _ _) _ = False
  effEq (Shuffle _) _ = False
  effEq (Continuously _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (GetsEmblem _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (Composite v e) (Composite w f) = v == w && effEq e f
  effEq (Composite _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (Pay _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (OnlyIf _ _ _) _ = False
  effEq (If _ _ _) _ = False
  effEq (Unless _ _ _) _ = False
  effEq (WhereLetter _ _ _) _ = False
  effEq (ForEachOf _ _) _ = False
  effEq (Repeat _) _ = False
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _) _ = False
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
  nounIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsAnyTarget (TargetGroup _ p) = headIsAnyTarget p
  nounIsAnyTarget (CountedGroup _ _) = False
  nounIsAnyTarget This = False
  nounIsAnyTarget (AsType t n) = nounIsAnyTarget n
  nounIsAnyTarget You = False
  nounIsAnyTarget (PlayerGroup _) = False
  nounIsAnyTarget (Each _) = False
  nounIsAnyTarget (Indefinite _ _) = False
  nounIsAnyTarget (Definite _) = False
  nounIsAnyTarget (AllOf _) = False
  nounIsAnyTarget (EachOf grp) = nounIsAnyTarget grp
  nounIsAnyTarget (YouAnd _) = False
  nounIsAnyTarget (LibrarySlice _ _ _) = False
  nounIsAnyTarget (SomeOf _ grp) = nounIsAnyTarget grp
  nounIsAnyTarget TheRest = False
  nounIsAnyTarget It = False
  nounIsAnyTarget They = False
  nounIsAnyTarget Them = False
  nounIsAnyTarget (Those _) = False
  nounIsAnyTarget (That _) = False
  nounIsAnyTarget (AttachHost _ _) = False
  nounIsAnyTarget (TheVerbed _ _) = False
  nounIsAnyTarget (ThoseVerbed _ _) = False
  nounIsAnyTarget (ControllerOf _) = False
  nounIsAnyTarget (OwnerOf _) = False
  nounIsAnyTarget (Designated _ _) = False

  public export
  nounIsKindJoin : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsKindJoin (TargetGroup _ p) = headIsKindJoin p
  nounIsKindJoin (CountedGroup _ p) = headIsKindJoin p
  nounIsKindJoin (Each p) = headIsKindJoin p
  nounIsKindJoin (Indefinite _ p) = headIsKindJoin p
  nounIsKindJoin (Definite p) = headIsKindJoin p
  nounIsKindJoin (AllOf p) = headIsKindJoin p
  nounIsKindJoin This = False
  nounIsKindJoin (AsType _ n) = nounIsKindJoin n
  nounIsKindJoin You = False
  nounIsKindJoin (PlayerGroup _) = False
  nounIsKindJoin (EachOf grp) = nounIsKindJoin grp
  nounIsKindJoin (YouAnd n) = nounIsKindJoin n
  nounIsKindJoin (LibrarySlice _ _ _) = False
  nounIsKindJoin (SomeOf _ grp) = nounIsKindJoin grp
  nounIsKindJoin TheRest = False
  nounIsKindJoin It = False
  nounIsKindJoin They = False
  nounIsKindJoin Them = False
  nounIsKindJoin (Those _) = False
  nounIsKindJoin (That JoinW) = True
  nounIsKindJoin (That _) = False
  nounIsKindJoin (AttachHost _ _) = False
  nounIsKindJoin (TheVerbed _ _) = False
  nounIsKindJoin (ThoseVerbed _ _) = False
  nounIsKindJoin (ControllerOf _) = False
  nounIsKindJoin (OwnerOf _) = False
  nounIsKindJoin (Designated _ _) = False

  public export
  nounIsMixedGroup : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsMixedGroup (YouAnd _) = True
  nounIsMixedGroup _ = False

  public export
  NotMixedGroup : Noun bs k -> Type
  NotMixedGroup {bs} {k} n = So (not (nounIsMixedGroup n))

  public export
  nounSpansPlayers : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounSpansPlayers n = nounIsAnyTarget n || nounIsKindJoin n ||
                       nounIsMixedGroup n

  public export
  data NotPlayerSpanning : Noun bs k -> Type where
    MkNotPlayerSpanning : {auto 0 ok : So (not (nounSpansPlayers n))} ->
                          NotPlayerSpanning n

  public export
  nounTargeted : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounTargeted (TargetGroup _ _) = True
  nounTargeted (CountedGroup _ _) = False
  nounTargeted This = False
  nounTargeted (AsType _ n) = nounTargeted n
  nounTargeted You = False
  nounTargeted (PlayerGroup _) = False
  nounTargeted (Each _) = False
  nounTargeted (Indefinite _ _) = False
  nounTargeted (Definite _) = False
  nounTargeted (AllOf _) = False
  nounTargeted (EachOf grp) = nounTargeted grp
  nounTargeted (YouAnd n) = nounTargeted n
  nounTargeted (LibrarySlice _ _ _) = False
  nounTargeted (SomeOf _ grp) = nounTargeted grp
  nounTargeted TheRest = False
  nounTargeted It = False
  nounTargeted They = False
  nounTargeted Them = False
  nounTargeted (Those _) = False
  nounTargeted (That _) = False
  nounTargeted (AttachHost _ _) = False
  nounTargeted (TheVerbed _ _) = False
  nounTargeted (ThoseVerbed _ _) = False
  nounTargeted (ControllerOf _) = False
  nounTargeted (OwnerOf _) = False
  nounTargeted (Designated _ _) = False

  public export
  Nontarget : Noun bs k -> Type
  Nontarget {bs} {k} n = So (not (nounTargeted n))

  ||| [CR#122.2]: counters on an object cease to exist when it moves from
  ||| one zone to another, so a clause that takes counters off a referent
  ||| an earlier clause moved names none — the object it reads is a new
  ||| one [CR#400.7]. Only the anaphors that can name a moved referent
  ||| carry the check; a description names its own.
  public export
  counterMemoryOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  counterMemoryOk It = not (stampMoves (provOfIt bs))
  counterMemoryOk Them = not (stampMoves (provOfThem bs))
  counterMemoryOk (TheVerbed v _) = not (verbMoves v)
  counterMemoryOk (ThoseVerbed v _) = not (verbMoves v)
  counterMemoryOk _ = True

  public export
  CounterMemory : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  CounterMemory {bs} n = So (counterMemoryOk n)

  public export
  data DamageRecipient : Noun bs k -> Type where
    PlayerTakes : DamageRecipient {k = Player} n
    AnyTargetTakes : {auto 0 ok : So (nounIsAnyTarget n)} ->
                     DamageRecipient {k = Object} n
    JoinTakes : {auto 0 ok : So (nounIsKindJoin n)} ->
                DamageRecipient {k = Object} n
    GroupTakes : {auto 0 ok : So (nounIsMixedGroup n)} ->
                 DamageRecipient {k = Object} n
    ObjectTakes : {auto 0 field : OnBattlefield (nounZone n)} ->
                  {auto 0 dm : DamageableTy (nounTy n)} ->
                  DamageRecipient {k = Object} n

  public export
  SingleRecipient : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  SingleRecipient {bs} {k} n = So (isOne (nounPlur n))

  public export
  data DiscardOk : Noun bs Object -> Type where
    DiscardThis : DiscardOk This
    DiscardTracked : {auto 0 z : nounZone n = Just Hand} -> DiscardOk n

  public export
  data Ascribable : Noun bs Object -> Type where
    AscribeThis : Ascribable This

  public export
  data TagBody : VerbName -> Effect bs -> Type where
    DestroyB : {auto 0 z : OnBattlefield (nounZone n)} ->
               {auto 0 na : NotPlayerSpanning n} ->
               TagBody Destroy (Move n (ZoneAt Graveyard Bare) {na})
    SacrificeB : {auto 0 z : OnBattlefield (nounZone n)} ->
                 {auto 0 na : NotPlayerSpanning n} ->
                 TagBody Sacrifice (Move n (ZoneAt Graveyard Bare) {na})
    ExileB : {auto 0 na : NotPlayerSpanning n} ->
             TagBody Exile (Move n (ZoneAt Exile Bare) {na})
    ExileWithCountersB : {0 amt : Amount (nomIntro n)} ->
                         {0 kind : CounterKind} ->
                         {auto 0 na : NotPlayerSpanning n} ->
                         TagBody Exile
                                 (Move n (ZoneAt Exile Bare) {na}
                                       {riders = MkMoveRiders [] Nothing
                                          {counters = Just (MkCounterRider amt kind)}})
    DiscardB : {auto 0 d : DiscardOk n} ->
               {auto 0 na : NotPlayerSpanning n} ->
               TagBody Discard (Move n (ZoneAt Graveyard Bare) {na})
    MillB : {0 amt : Amount bs} -> {0 whose : Noun bs Player} ->
            {auto 0 sp : SlicePossessor whose} ->
            {auto 0 na : NotPlayerSpanning (LibrarySlice OnTop amt whose {sp})} ->
            TagBody Mill (Move (LibrarySlice OnTop amt whose {sp})
                               (ZoneAt Graveyard Bare) {na})
    ScryB : {0 amt : Amount bs} ->
            {auto 0 sp : SlicePossessor {bs} You} ->
            TagBody Scry
                    (Expose LookAt You
                            (ExposedCards (LibrarySlice OnTop amt You {sp})))
    SurveilB : {0 amt : Amount bs} ->
               {auto 0 sp : SlicePossessor {bs} You} ->
               TagBody Surveil
                       (Expose LookAt You
                               (ExposedCards (LibrarySlice OnTop amt You {sp})))
    ||| The agentive placement clause: the imperative and "<player> puts it
    ||| into/onto <zone>" spell one event, so the tag rides the same Move.
    ||| Every Move admits: its own gates already bound the destination, and
    ||| the destination's possessive is rendering's business [CR#400.3].
    PutB : 
           TagBody Put (Move n to {riders} {ok} {arr} {na} {pl} {rf})

  public export
  NonAgentive : VerbName -> Type
  NonAgentive v = So (not (verbAgentive v))


  public export
  setZone : Maybe VerbName -> Maybe Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty oldZn _ og)) =
    MkBinding det Object plur (ObjectP ty z (mkStamp p oldZn) og)
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP
  setZone p z (MkBinding det Outcome plur (OutcomeP s)) =
    MkBinding det Outcome plur (OutcomeP s)
  setZone p z (MkBinding det Gap plur GapP) = MkBinding det Gap plur GapP
  setZone p z (MkBinding det (Letter w) plur LetterP) = MkBinding det (Letter w) plur LetterP
  setZone p z (MkBinding det TurnRef plur TurnRefP) = MkBinding det TurnRef plur TurnRefP
  setZone p z (MkBinding det Ability plur AbilityP) = MkBinding det Ability plur AbilityP
  setZone p z (MkBinding det (a \/ b) plur (JoinP l r)) = MkBinding det (a \/ b) plur (JoinP l r)

  public export
  setZoneHead : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  public export
  setZoneIt : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (MkBinding det Object OneOf (ObjectP ty zn _ og) :: bs) =
    MkBinding det Object OneOf (ObjectP ty z (mkStamp p zn) og) :: bs
  setZoneIt p z (b :: bs) = b :: setZoneIt p z bs

  public export
  setZoneThem : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (MkBinding det Object ManyOf (ObjectP ty zn _ og) :: bs) =
    MkBinding det Object ManyOf (ObjectP ty z (mkStamp p zn) og) :: bs
  setZoneThem p z (b :: bs) = b :: setZoneThem p z bs

  public export
  setZoneThose : Maybe VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThose p w z [] = []
  setZoneThose p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (ManyOf, True) => setZone p z b :: bs
      _ => b :: setZoneThose p w z bs

  public export
  setZoneThat : Maybe VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThat p w z [] = []
  setZoneThat p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (OneOf, True) => setZone p z b :: bs
      _ => b :: setZoneThat p w z bs

  public export
  setZoneVerbed : Maybe VerbName -> VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneVerbed p v w z [] = []
  setZoneVerbed p v w z (b :: bs) =
    if verbedMatch v w b then setZone p z b :: bs else b :: setZoneVerbed p v w z bs

  public export
  setZoneManyVerbed : Maybe VerbName -> VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneManyVerbed p v w z [] = []
  setZoneManyVerbed p v w z (b :: bs) =
    if verbedMatchMany v w b then setZone p z b :: bs
                             else b :: setZoneManyVerbed p v w z bs

  ||| Re-zones a moved object, minting a fresh binding for a moved sorted
  ||| self [CR#400.7]. The leading verb stamps the binding so a later
  ||| participle read (`TheVerbed`/`ThoseVerbed`) can find it as "the
  ||| destroyed creature"/"the exiled card" [CR#701.17c].
  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbName -> Noun bs k -> Maybe Zone -> Bindings
  moveIntro p nn@(Each pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(Indefinite m pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(Definite pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(TargetGroup q pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(CountedGroup q pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(AllOf pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p (EachOf grp) z = moveIntro p grp z
  moveIntro p nn@(YouAnd _) z = nomIntro nn
  moveIntro p nn@(LibrarySlice _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(SomeOf _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p TheRest z = groupSpent bs
  moveIntro p It z = setZoneIt p z bs
  moveIntro p Them z = setZoneThem p z bs
  moveIntro p (That w) z = setZoneThat p w z bs
  moveIntro p (Those w) z = setZoneThose p w z bs
  moveIntro p (TheVerbed v w) z = setZoneVerbed p v w z bs
  moveIntro p (ThoseVerbed v w) z = setZoneManyVerbed p v w z bs
  moveIntro p This z = bs
  moveIntro p (AttachHost _ _) z = bs
  moveIntro p (AsType t n) z = MkBinding TheD Object OneOf (ObjectP (Just t) z (mkStamp p Nothing) Nothing) :: bs
  moveIntro p You z = bs
  moveIntro p (PlayerGroup _) z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)
  moveIntro p (Designated d n) z = bs

  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n) = Just Battlefield
  nounZone You = Nothing
  nounZone (PlayerGroup _) = Nothing
  nounZone (Each p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (Indefinite m p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (Definite p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (TargetGroup q p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (CountedGroup q p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (AllOf p) =
    if headIsPlaceless p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (EachOf grp) = nounZone grp
  nounZone (YouAnd _) = Nothing
  nounZone (LibrarySlice _ _ _) = Just Library
  nounZone (SomeOf _ grp) = nounZone grp
  nounZone TheRest = zoneOfGroup bs
  nounZone It = zoneOfIt bs
  nounZone They = Nothing
  nounZone Them = zoneOfThem bs
  nounZone (That w) = zoneOfThat w bs
  nounZone (AttachHost _ h) = attachHostZone h
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w) = zoneOfVerbed v w bs
  nounZone (ThoseVerbed v w) = zoneOfManyVerbed v w bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing
  -- [CR#903.3]: the designation is an attribute of the card, so it
  -- survives a zone change and the noun names no zone.
  nounZone (Designated _ _) = Nothing

  public export
  nounTy : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe CardType
  nounTy This = Nothing
  nounTy (AsType t n) = Just t
  nounTy You = Nothing
  nounTy (PlayerGroup _) = Nothing
  nounTy (Each p) = seedTy p
  nounTy (Indefinite m p) = seedTy p
  nounTy (Definite p) = seedTy p
  nounTy (TargetGroup q p) = seedTy p
  nounTy (CountedGroup q p) = seedTy p
  nounTy (AllOf p) = seedTy p
  nounTy (EachOf grp) = nounTy grp
  nounTy (YouAnd _) = Nothing
  nounTy (LibrarySlice _ _ _) = Nothing
  nounTy (SomeOf _ grp) = nounTy grp
  nounTy TheRest = tyOfGroup bs
  nounTy It = tyOfIt bs
  nounTy They = Nothing
  nounTy Them = tyOfThem bs
  nounTy (That w) = tyOfThat w bs
  nounTy (AttachHost _ h) = attachHostTy h
  nounTy (Those w) = tyOfThose w bs
  nounTy (TheVerbed v w) = tyOfVerbed v w bs
  nounTy (ThoseVerbed v w) = tyOfManyVerbed v w bs
  nounTy (ControllerOf n) = Nothing
  nounTy (OwnerOf n) = Nothing
  nounTy (Designated _ _) = Nothing

  public export
  nounPlur : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Plurality
  nounPlur This = OneOf
  nounPlur (AsType t n) = nounPlur n
  nounPlur You = OneOf
  nounPlur (PlayerGroup _) = ManyOf
  nounPlur (Each p) = ManyOf
  nounPlur (Indefinite m p) = OneOf
  nounPlur (Definite p) = OneOf
  nounPlur (TargetGroup q p) = quantPlur q
  nounPlur (CountedGroup q p) = quantPlur q
  nounPlur (AllOf p) = ManyOf
  nounPlur (EachOf grp) = ManyOf
  nounPlur (YouAnd _) = ManyOf
  nounPlur (LibrarySlice _ amt whose) = outputPlur (nounPlur whose) (amtPlur amt)
  nounPlur (SomeOf q _) = quantPlur q
  nounPlur TheRest = ManyOf
  nounPlur It = OneOf
  nounPlur They = OneOf
  nounPlur Them = ManyOf
  nounPlur (That w) = OneOf
  nounPlur (AttachHost _ _) = OneOf
  nounPlur (Those w) = ManyOf
  nounPlur (TheVerbed v w) = OneOf
  nounPlur (ThoseVerbed v w) = ManyOf
  nounPlur (ControllerOf n) = OneOf
  nounPlur (OwnerOf n) = OneOf
  nounPlur (Designated _ _) = OneOf

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (SetStatus _ n) = nomIntro n
  effIntro (DoesntUntapNext n _) = nomIntro n
  effIntro (SkipsNext w _ _) = nomIntro w
  effIntro (ExtraTurn w _) = turnRefB :: nomIntro w
  effIntro (AdditionalPart _ _ _) = bs
  effIntro (GetsCounters who amt _) = amtIntro amt
  effIntro (LosesAllCounters who _) = nomIntro who
  effIntro (RemoveFromCombat n) = nomIntro n
  effIntro (Regenerate n) = nomIntro n
  effIntro (CantBe e _ _) = effIntro e
  effIntro (GainsDesignation n _ _) = nomIntro n
  effIntro (GameBecomes _) = bs
  effIntro (Concludes _ who) = nomIntro who
  effIntro GameDrawn = bs
  effIntro (CounterSpell what) = nomIntro what
  effIntro (CopyStack agent what times exc) =
    MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
              (ObjectP (nounTy what) (Just Stack) Nothing (Just CopyOrigin))
      :: amtIntro times
  effIntro (ChooseNewTargets what) = nomIntro what
  effIntro (Choose n {by = Nothing}) = nomIntro n
  effIntro (Choose n {by = Just b}) = nounDelta b ++ nomIntro n
  effIntro (Move what to) = moveIntro Nothing what (Just (zoneSort to))
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
  effIntro (Continuously se _) = staticIntro se
  effIntro (Create agent count spec riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin))
      :: amtIntro count
  effIntro (GetsEmblem who _) = nomIntro who
  effIntro (PutCounters amt kind on) = nomIntro on
  effIntro (Distribute (DividedDamage _) amt among) = outcomeB DamageDealt :: nomIntro among
  effIntro (Distribute (DistributedCounters _) amt among) = nomIntro among
  effIntro (RemoveCounters amt kind from) = nomIntro from
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Does s v e) = effIntro e
  effIntro (Pay who c) = costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- a conditioned clause exports nothing: the condition may have failed.
  effIntro (OnlyIf e c oth) = bs
  effIntro (If c e oth) = bs
  effIntro (Unless e who c) = bs
  effIntro (WhereLetter _ def body) = effIntro body
  effIntro (ForEachOf _ _) = bs
  effIntro (Repeat _) = bs
  effIntro (Sequentially es) = effsIntro es
  effIntro (Simultaneously es) = simIntro es
  effIntro (Modal q modes) = bs
  effIntro (Delayed ev e) = bs               -- a future clause mentions nothing NOW
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
  preIntro (DoesntUntapNext n _) = nomIntro n
  preIntro (SkipsNext w _ _) = nomIntro w
  preIntro (ExtraTurn w _) = nomIntro w
  preIntro (AdditionalPart _ _ _) = bs
  preIntro (GetsCounters who amt _) = amtIntro amt
  preIntro (LosesAllCounters who _) = nomIntro who
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (Regenerate n) = nomIntro n
  preIntro (CantBe e _ _) = preIntro e
  preIntro (GainsDesignation n _ _) = nomIntro n
  preIntro (GameBecomes _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro (CounterSpell what) = nomIntro what
  preIntro (CopyStack agent what times exc) = amtIntro times
  preIntro (ChooseNewTargets what) = nomIntro what
  preIntro (Choose n) = nomIntro n
  preIntro (Move what to) = nomIntro what
  preIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  preIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  preIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  preIntro (AddMana who amt _ _) = amtIntro amt
  preIntro (Draw who amt) = amtIntro amt
  preIntro (Expose v who what) = exposedIntro what
  preIntro (Search who sc p) = predDelta p ++ searchDelta sc ++ nomIntro who
  preIntro (Shuffle whose) = nomIntro whose
  preIntro (Continuously se _) = staticIntro se
  preIntro (Create agent count spec riders) = amtIntro count
  preIntro (GetsEmblem who _) = nomIntro who
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters amt kind from) = nomIntro from
  preIntro (Composite v (Move what to)) = nomIntro what
  preIntro (Composite _ e) = preIntro e
  preIntro (Does s v (Move what to)) = nomIntro what
  preIntro (Does s v e) = preIntro e
  preIntro (Pay who c) = nomIntro who
  preIntro (May d body did notd) = mayIntro body did notd
  preIntro (OnlyIf e c oth) = annIntro e
  preIntro (If c e oth) = bs
  preIntro (Unless e who c) = annIntro e
  preIntro (WhereLetter _ def body) = preIntro body
  preIntro (ForEachOf _ _) = bs
  preIntro (Repeat _) = bs
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = bs
  preIntro (Delayed ev e) = bs
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
  annIntro (DoesntUntapNext n _) = nomIntro n
  annIntro (SkipsNext w _ _) = nomIntro w
  annIntro (ExtraTurn w _) = turnRefB :: nomIntro w
  annIntro (AdditionalPart _ _ _) = bs
  annIntro (GetsCounters who amt _) = amtIntro amt
  annIntro (LosesAllCounters who _) = nomIntro who
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (Regenerate n) = nomIntro n
  annIntro (CantBe e _ _) = annIntro e
  annIntro (GainsDesignation n _ _) = nomIntro n
  annIntro (GameBecomes _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro (CounterSpell what) = nomIntro what
  annIntro (CopyStack agent what times exc) = amtIntro times
  annIntro (ChooseNewTargets what) = nomIntro what
  annIntro (Choose n) = nomIntro n
  annIntro (Move what to) = nomIntro what
  annIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  annIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  annIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  annIntro (AddMana who amt _ _) = amtIntro amt
  annIntro (Draw who amt) = amtIntro amt
  annIntro (Expose v who what) = exposedIntro what
  annIntro (Search who sc p) = predDelta p ++ searchDelta sc ++ nomIntro who
  annIntro (Shuffle whose) = nomIntro whose
  annIntro (Continuously se _) = staticIntro se
  annIntro (Create agent count spec riders) = amtIntro count
  annIntro (GetsEmblem who _) = nomIntro who
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters amt kind from) = nomIntro from
  annIntro (Composite v (Move what to)) = nomIntro what
  annIntro (Composite _ e) = annIntro e
  annIntro (Does s v (Move what to)) = nomIntro what
  annIntro (Does s v e) = annIntro e
  annIntro (Pay who c) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (OnlyIf e c oth) = annIntro e
  annIntro (If c e oth) = bs
  annIntro (Unless e who c) = annIntro e
  annIntro (WhereLetter _ def body) = annIntro body
  annIntro (ForEachOf _ _) = bs
  annIntro (Repeat _) = bs
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = bs
  annIntro (Delayed ev e) = bs
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
  deedDelta (AdditionalPart _ _ _) = []
  deedDelta (GetsCounters _ _ _) = []
  deedDelta (LosesAllCounters _ _) = []
  deedDelta (RemoveFromCombat _) = []
  deedDelta (Regenerate _) = []
  deedDelta (CantBe e _ _) = deedDelta e
  deedDelta (GainsDesignation _ _ _) = []
  deedDelta (GameBecomes _) = []
  deedDelta (Concludes _ _) = []
  deedDelta GameDrawn = []
  deedDelta (CounterSpell _) = []
  deedDelta (CopyStack agent what times exc) =
    [MkBinding TheD Object (outputPlur (nounPlur what) (amtPlur times))
               (ObjectP (nounTy what) (Just Stack) Nothing (Just CopyOrigin))]
  deedDelta (ChooseNewTargets _) = []
  deedDelta (Choose n) = []
  deedDelta (Move what to) = []
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (AddMana _ _ _ _) = []
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Search who sc p) =
    [MkBinding AD Object OneOf (ObjectP (seedTy p) (searchZone sc) Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (Continuously se _) = []
  deedDelta (Create agent count spec riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin))]
  deedDelta (GetsEmblem _ _) = []
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters amt kind from) = []
  deedDelta (Composite v (Move what to)) = []
  deedDelta (Composite _ e) = deedDelta e
  deedDelta (Does s v (Move what to)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (Pay who c) = []
  deedDelta (May d body did notd) = []
  deedDelta (OnlyIf e c oth) = []
  deedDelta (If c e oth) = []
  deedDelta (Unless e who c) = []
  deedDelta (WhereLetter _ _ _) = []
  deedDelta (ForEachOf _ _) = []
  deedDelta (Repeat _) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev e) = []
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

  public export
  simIntro : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simIntro [] = bs
  simIntro (e :: es) = deedDelta e ++ simIntro es

  public export
  simPres : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simPres [] = bs
  simPres (e :: []) = preIntro e
  simPres (e :: es) = simPres es

  ||| [CR#500.1] runs every phase and step on every turn, so a part in
  ||| a named player's turn picks out a real span. A bare turn picks out
  ||| none: every moment of the game is during some turn. And a window
  ||| introduces no turn, so the deictic possessor reaches no antecedent.
  public export
  windowOk : TurnPart -> Maybe Owner -> Bool
  windowOk Turn Nothing = False
  windowOk _ (Just ThatTurns) = False
  windowOk _ _ = True

  public export
  WindowOk : TurnPart -> Maybe Owner -> Type
  WindowOk p w = So (windowOk p w)

  ||| Whose turn a boundary-relative window falls in. An activation
  ||| restriction introduces no turn, so the deictic possessor reaches no
  ||| antecedent; every quantifier word names a turn on its own.
  public export
  pointWindowOk : TurnPoint -> Maybe Owner -> Bool
  pointWindowOk _ (Just ThatTurns) = False
  pointWindowOk _ _ = True

  public export
  PointWindowOk : TurnPoint -> Maybe Owner -> Type
  PointWindowOk pt w = So (pointWindowOk pt w)

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

  public export
  headerCtx : {bs : Bindings} -> Maybe (GameEvent bs) -> GameEvent bs -> Bindings
  headerCtx Nothing ev = eventAfter ev
  headerCtx (Just _) _ = bs

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
                     {default Nothing param : Maybe (KeywordParam bs)} ->
                     {auto 0 pf : KeywordParamFits k param} -> AbilityAt bs
    Activated : (cost : Cost bs) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                {default Nothing window : Maybe Timing} ->
                {default Nothing limit : Maybe UsageLimit} ->
                {default Nothing guard : Maybe (Condition bs)} ->
                AbilityAt bs
    Triggered : (word : TriggerWord) -> (ev : GameEvent bs) ->
                {default Nothing alt : Maybe (GameEvent bs)} ->
                {default Nothing window : Maybe TriggerWindow} ->
                {default Nothing limit : Maybe UsageLimit} ->
                {default Nothing intervening : Maybe (Condition (headerCtx alt ev))} ->
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
  HeaderNontarget : {bs : Bindings} -> GameEvent bs -> Type
  HeaderNontarget {bs} ev = So (not (anyTargetedAt (eventIntro ev)))

  public export
  lineKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe Keyword
  lineKeyword (Static se) = statKeyword se
  lineKeyword (Triggered _ _ eff) = effKeyword eff
  lineKeyword _ = Nothing

  public export
  statKeyword : {0 bs : Bindings} -> StaticEffect bs -> Maybe Keyword
  statKeyword (Conditionally _ se) = statKeyword se
  statKeyword (OnlyWhile se _) = statKeyword se
  statKeyword (Gains _ ab) = grantedKeyword ab
  statKeyword _ = Nothing

  public export
  effKeyword : {0 bs : Bindings} -> Effect bs -> Maybe Keyword
  effKeyword (Continuously se _) = statKeyword se
  effKeyword _ = Nothing

  public export
  grantedKeyword : {0 bs : Bindings} -> AbilityAt bs -> Maybe Keyword
  grantedKeyword (KeywordAbility k {param = Nothing}) =
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

  public export
  grantableAb : {0 bs : Bindings} -> AbilityAt bs -> Bool
  grantableAb (KeywordAbility _) = True
  grantableAb (Activated _ _) = True
  grantableAb (Triggered _ _ _) = True
  grantableAb (Static _) = True
  grantableAb (Spell _) = False
  grantableAb (AlsoForKeywords _ _) = False
  grantableAb (AbilityWord _ ab) = grantableAb ab

  public export
  Grantable : AbilityAt bs -> Type
  Grantable {bs} ab = So (grantableAb ab)

  public export
  emblemAbilityOk : AbilityAt [] -> Bool
  emblemAbilityOk (KeywordAbility _) = False
  emblemAbilityOk (Activated _ _) = True
  emblemAbilityOk (Triggered _ _ _) = True
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
  nounRegime _ = Nothing

  public export
  abRegime : {0 bs : Bindings} -> AbilityAt bs -> Maybe StackRegime
  abRegime (KeywordAbility k) = keywordStackRegime k
  abRegime (Activated _ _) = Nothing
  abRegime (Triggered _ _ _) = Nothing
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
  abIntro (KeywordAbility _) = bs
  abIntro (Activated _ _) = bs
  abIntro (Triggered _ _ _) = bs
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
  clauseStaticOk (WhereLetterStatic _ _ se) = clauseStaticOk se
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


public export
supersDistinct : List Supertype -> Bool
supersDistinct [] = True
supersDistinct (s :: ss) = not (elem s ss) && supersDistinct ss

public export
CardSupers : List Supertype -> Type
CardSupers ss = So (supersDistinct ss)

public export
data CardClass = PermanentCard | SpellCard

public export
cardClassOf : List CardType -> CardClass
cardClassOf [] = PermanentCard
cardClassOf (t :: ts) = if spellType t then SpellCard else cardClassOf ts

public export
anyPermanentType : List CardType -> Bool
anyPermanentType [] = False
anyPermanentType (t :: ts) = permanentType t || anyPermanentType ts

public export
anySpellType : List CardType -> Bool
anySpellType [] = False
anySpellType (t :: ts) = spellType t || anySpellType ts

public export
hasNonKindredType : List CardType -> Bool
hasNonKindredType [] = False
hasNonKindredType (t :: ts) = not (t == Kindred) || hasNonKindredType ts

public export
typesCombinable : List CardType -> Bool
typesCombinable tys =
  not (anyPermanentType tys && anySpellType tys)
    && (not (elem Kindred tys) || hasNonKindredType tys)

public export
keywordCardOk : CardClass -> Keyword -> Bool
keywordCardOk PermanentCard Haste = True
keywordCardOk PermanentCard Flying = True
keywordCardOk PermanentCard Trample = True
keywordCardOk PermanentCard Vigilance = True
keywordCardOk PermanentCard Deathtouch = True
keywordCardOk PermanentCard DoubleStrike = True
keywordCardOk PermanentCard FirstStrike = True
keywordCardOk SpellCard Haste = False
keywordCardOk SpellCard Flying = False
keywordCardOk SpellCard Trample = False
keywordCardOk SpellCard Vigilance = False
keywordCardOk SpellCard Deathtouch = False
keywordCardOk SpellCard DoubleStrike = False
keywordCardOk SpellCard FirstStrike = False
keywordCardOk PermanentCard Reach = True
keywordCardOk SpellCard Reach = False
keywordCardOk PermanentCard Convoke = True
keywordCardOk PermanentCard Improvise = True
keywordCardOk PermanentCard Storm = True
keywordCardOk PermanentCard Lifelink = True
keywordCardOk SpellCard Convoke = True
keywordCardOk SpellCard Improvise = True
keywordCardOk SpellCard Storm = True
keywordCardOk SpellCard Lifelink = False
keywordCardOk PermanentCard Ward = True
keywordCardOk PermanentCard Protection = True
keywordCardOk SpellCard Ward = False
keywordCardOk SpellCard Protection = False
keywordCardOk PermanentCard Enchant = True
keywordCardOk PermanentCard Equip = True
keywordCardOk PermanentCard Storied = True
keywordCardOk PermanentCard Renown = True
keywordCardOk SpellCard Enchant = False
keywordCardOk SpellCard Equip = False
keywordCardOk SpellCard Storied = False
keywordCardOk SpellCard Renown = False
keywordCardOk PermanentCard Indestructible = True
keywordCardOk SpellCard Indestructible = False
keywordCardOk PermanentCard Flash = True
keywordCardOk SpellCard Flash = True
keywordCardOk PermanentCard Ascend = True
keywordCardOk SpellCard Ascend = True
keywordCardOk PermanentCard CumulativeUpkeep = True
keywordCardOk SpellCard CumulativeUpkeep = False
keywordCardOk PermanentCard Hexproof = True
keywordCardOk SpellCard Hexproof = False
keywordCardOk PermanentCard Menace = True
keywordCardOk SpellCard Menace = False
keywordCardOk PermanentCard Skulk = True
keywordCardOk SpellCard Skulk = False
-- [CR#702.84a] and [CR#702.49a] put the card onto the battlefield, which
-- [CR#110.4] denies an instant or sorcery card; [CR#702.34a] permits the
-- flashback cast only if the resulting spell is an instant or sorcery.
keywordCardOk PermanentCard Unearth = True
keywordCardOk SpellCard Unearth = False
keywordCardOk PermanentCard Ninjutsu = True
keywordCardOk SpellCard Ninjutsu = False
keywordCardOk PermanentCard Flashback = False
keywordCardOk SpellCard Flashback = True
keywordCardOk PermanentCard Dredge = True
keywordCardOk SpellCard Dredge = True
keywordCardOk PermanentCard Retrace = True
keywordCardOk SpellCard Retrace = True
keywordCardOk PermanentCard Cycling = True
keywordCardOk SpellCard Cycling = True
keywordCardOk PermanentCard Miracle = True
keywordCardOk SpellCard Miracle = True
keywordCardOk PermanentCard Warp = True
keywordCardOk SpellCard Warp = True

public export
staticOnSpellCardOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
-- [CR#113.6g] licenses a "can't be countered" static on any object and
-- says nothing about how broadly its subject may be described.
staticOnSpellCardOk (ObjectCant Countered _) = True
staticOnSpellCardOk (ObjectCant Copied _) = True
staticOnSpellCardOk (AltCost _) = True
staticOnSpellCardOk (Conditionally _ se) = staticOnSpellCardOk se
staticOnSpellCardOk (OnlyWhile se _) = staticOnSpellCardOk se
staticOnSpellCardOk _ = False

public export
cardAbilityOk : {0 bs : Bindings} -> CardClass -> AbilityAt bs -> Bool
cardAbilityOk PermanentCard (KeywordAbility k) = keywordCardOk PermanentCard k
cardAbilityOk PermanentCard (Activated _ _) = True
cardAbilityOk PermanentCard (Triggered _ _ _) = True
cardAbilityOk PermanentCard (Static _) = True
cardAbilityOk PermanentCard (AlsoForKeywords ab _) = cardAbilityOk PermanentCard ab
cardAbilityOk PermanentCard (Spell _) = False
cardAbilityOk PermanentCard (AbilityWord _ ab) = cardAbilityOk PermanentCard ab
cardAbilityOk SpellCard (KeywordAbility k) = keywordCardOk SpellCard k
cardAbilityOk SpellCard (Activated c _) = costOffBattlefield c
cardAbilityOk SpellCard (Triggered _ _ _) = True
cardAbilityOk SpellCard (Static se) = staticOnSpellCardOk se
cardAbilityOk SpellCard (AlsoForKeywords ab _) = cardAbilityOk SpellCard ab
cardAbilityOk SpellCard (Spell _) = True
cardAbilityOk SpellCard (AbilityWord _ ab) = cardAbilityOk SpellCard ab

public export
cardTextOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs -> Bool
cardTextOk tys [] = True
cardTextOk tys (a :: as) = cardAbilityOk (cardClassOf tys) a && cardTextOk tys as

public export
chapterLineOk : {0 bs : Bindings} -> List Subtype -> AbilityAt bs -> Bool
chapterLineOk subs (Triggered _ (ChapterMark _) _) = elem Saga subs
chapterLineOk subs (AbilityWord _ ab) = chapterLineOk subs ab
chapterLineOk subs (AlsoForKeywords ab _) = chapterLineOk subs ab
chapterLineOk _ _ = True

public export
chapterFrameOk : {0 bs : Bindings} -> List Subtype -> AbilitySeq bs -> Bool
chapterFrameOk subs [] = True
chapterFrameOk subs (a :: as) = chapterLineOk subs a && chapterFrameOk subs as

public export
data PrintedStat = PrintedNum Integer | PrintedStar | PrintedStarPlus Nat

public export
starred : PrintedStat -> Bool
starred (PrintedNum _) = False
starred PrintedStar = True
starred (PrintedStarPlus _) = True

public export
staticDefinesPt : {0 bs : Bindings} -> StaticEffect bs -> Maybe DefinedSlots
staticDefinesPt (DefinesPt _ sl _) = Just sl
staticDefinesPt (WhereLetterStatic _ _ se) = staticDefinesPt se
staticDefinesPt (Conditionally _ se) = staticDefinesPt se
staticDefinesPt (OnlyWhile se _) = staticDefinesPt se
staticDefinesPt _ = Nothing

||| [CR#207.2c] gives the ability word no rules meaning, so a characteristic-
||| defining line is still one when a word prefixes it: the starred-print gate
||| reads through the wrapper.
public export
abDefinesPt : {0 bs : Bindings} -> AbilityAt bs -> Maybe DefinedSlots
abDefinesPt (Static se) = staticDefinesPt se
abDefinesPt (AbilityWord _ ab) = abDefinesPt ab
abDefinesPt _ = Nothing

public export
textDefines : {0 bs : Bindings} -> (DefinedSlots -> Bool) -> AbilitySeq bs -> Bool
textDefines f [] = False
textDefines f (a :: as) =
  (case abDefinesPt a of
     Just sl => f sl
     Nothing => False) || textDefines f as

public export
definedSlotsStarred : Maybe (PrintedStat, PrintedStat) -> Bool -> Bool -> Bool
definedSlotsStarred Nothing dp dt = not dp && not dt
definedSlotsStarred (Just (p, t)) dp dt =
  (not dp || starred p) && (not dt || starred t)

public export
cardPtOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs ->
           Maybe (PrintedStat, PrintedStat) -> Bool
cardPtOk tys text pt =
  (not (elem Creature tys) || isJust pt) &&
  definedSlotsStarred pt (textDefines definesPower text)
                         (textDefines definesToughness text)

public export
cardCostOk : List CardType -> Maybe ManaCost -> Bool
cardCostOk tys cost = case (elem Land tys, cost) of
  (True, Just _) => False
  _ => True

public export
data CardLine : TypeLine -> Type where
  MkCardLine : {auto 0 ne : So (lineNonEmpty l)} ->
               {auto 0 dst : So (typesDistinct l.tys)} ->
               {auto 0 cmb : So (typesCombinable l.tys)} ->
               {auto 0 sf : So (subsFitLine l.subs l.tys)} -> CardLine l

public export
CardText : TypeLine -> AbilitySeq [] -> Type
CardText l as = So (cardTextOk l.tys as)

public export
CardChapters : TypeLine -> AbilitySeq [] -> Type
CardChapters l as = So (chapterFrameOk l.subs as)

public export
data CardPt : TypeLine -> AbilitySeq [] -> Maybe (PrintedStat, PrintedStat) -> Type where
  MkCardPt : {0 stats : Maybe (PrintedStat, PrintedStat)} ->
             {auto 0 ok : So (cardPtOk l.tys as stats)} -> CardPt l as stats

public export
CardCost : TypeLine -> Maybe ManaCost -> Type
CardCost l c = So (cardCostOk l.tys c)

public export
record Card where
  constructor MkCard
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq []
  pt : Maybe (PrintedStat, PrintedStat)

