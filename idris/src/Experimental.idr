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
                (off : Maybe LibOrdinal) ->
                {auto 0 af : PlaceArrangementFits place ord} ->
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
  data NameSource : Bindings -> Type where
    PrintedName : (name : String) -> NameSource bs
    ChosenName : {auto 0 ok : countQuality CardName bs = 1} -> NameSource bs
    ||| "… with the same name as that creature", "… as those creatures".
    ||| The relatum is ungated in number: [CR#201.2c] states the comparison
    ||| against "a second object or group of objects" outright, and
    ||| [CR#201.2a] states the positive relation over two or more objects,
    ||| so a plural relatum is a name held in common with them and not a
    ||| category error.
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
                  (dom : Maybe (ChoiceDomain q)) ->
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
                 (what : Maybe (EventComplement bs ev k)) ->
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
    -- the bound slot is open to any amount at every relation; see
    -- `CompareAmt`'s docstring for the recorded verdict.
    Compare : (c : Characteristic) -> (r : Comparator) -> (bound : Amount bs) ->
              Predicate bs Object
    ||| The bound read of a counter-bearing description: the referent's own
    ||| count of [kind] counters on a comparison's left. At the kind index —
    ||| the poison lines are the player cells ([CR#122.1f] states the
    ||| player-side test in this ranged shape) and no marked player twin
    ||| exists. Not a quantity slot on `HasCounters`, which stays the bare
    ||| existence read.
    ||| -- spelling: at Object, "with [bound] or more/fewer [kind] counters
    ||| on it/them"; at Player, "who has [bound] or more [kind] counters".
    CounterCompare : (kind : Maybe CounterKind) -> (r : Comparator) ->
                     (bound : Amount bs) ->
                     {auto 0 kn : CounterKindNamed k kind} ->
                     Predicate bs k
    Superlative : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                  (dom : Predicate bs k) ->
                  {auto 0 ex : IsExtremal op} ->
                  {auto 0 sc : projScope ax = k} ->
                  Predicate bs k
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
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
    ||| The complement names a referent to SUBTRACT [CR#601.2c], not a
    ||| member of the phrase's own domain, so it is described at its own
    ||| kind: "any target other than this creature" subtracts an object
    ||| from a phrase that may denote a player too.
    OtherThan : {kn : Kind} -> (n : Noun bs kn) ->
                {auto 0 ca : ComplementAnchor n} -> Predicate bs k
    ||| The cross-kind head: one description per kind, and the phrase's kind
    ||| is their join. "Target creature or player" is
    ||| `Joined (HasType Creature) AnyPlayer`, admitted by [CR#115.1]: a
    ||| spell's targets are objects and/or players. "Any target" is the same
    ||| row over [CR#115.4]'s own four-way list. Which half English writes first
    ||| is the spelling layer's business, so the semantics fixes the object
    ||| arm left and one kind order serves every phrasing.
    Joined : {ka : Kind} -> {kb : Kind} -> (l : Predicate bs ka) ->
             (r : Predicate bs kb) -> Predicate bs (ka \/ kb)
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
  seedTy (Joined l r) = maybe (seedTy r) Just (seedTy l)
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
  seedZone (HappenedTo _ _ _) = Nothing
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
  seedType (HappenedTo _ _ _) = Nothing
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
  hasHead (QualityNoun _ _) = True
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
  hasHead (HappenedTo _ _ _) = False
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
  hasHead (CounterCompare _ _ _) = False
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
  hasHead (Joined _ _) = True

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
  predEq (QualityNoun a d) (QualityNoun a e) = sameDomainOpt d e
  predEq (QualityNoun _ _) _ = False
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
  predEq (HappenedTo a v Nothing) (HappenedTo b w Nothing) =
    sameEventName a b && sameLookback v w
  predEq (HappenedTo _ _ _) _ = False
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
  predEq (CounterCompare Nothing r b) (CounterCompare Nothing s e) =
    r == s && boundEq b e
  predEq (CounterCompare (Just a) r b) (CounterCompare (Just c) s e) =
    a == c && r == s && boundEq b e
  predEq (CounterCompare _ _ _) _ = False
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
  -- the complement carries its own kind index, so two anchors need not be
  -- comparable; conservative, like the `Or` row above. Nothing is lost:
  -- `coordinable` keeps a complement out of an `Or`, and `OtherAnchored`
  -- admits at most one per `And`, so two never have to be told apart.
  predEq (OtherThan _) _ = False
  -- deliberately conservative, like the `Or` row above: two joined
  -- descriptions are never provably the same referent [CR#601.2c].
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
  isSourceHead _ = False

  public export
  anyIsSource : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsSource [] = False
  anyIsSource (p :: ps) = isSourceHead p || anyIsSource ps

  ||| [CR#120.7] makes a source the object that dealt some damage — a
  ||| position in an event rather than an object in a zone — so a phrase
  ||| headed by the source word places nothing. This is the only head-word
  ||| placelessness left; the union family's is the kind's (see
  ||| `phraseZone`).
  public export
  headIsPlaceless : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsPlaceless p = anyIsSource (flattenPs [p])

  ||| Where a description places its referent. [CR#109.2] reads a bare type
  ||| word onto the battlefield, so an object description defaults there.
  ||| A phrase whose kind reaches past objects places NOTHING: [CR#400.1]
  ||| makes a zone a place where objects can be and [CR#109.1] lists what
  ||| an object is, and a player is none of them. That single fact is what
  ||| refuses destroy, exile, tap, untap, return, counter and sacrifice
  ||| over a joined phrase, with no rule written for the purpose.
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
  isComparison (CounterCompare _ _ _) = True
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

  ||| A "non-" prefix names the complement of one modifier inside its own
  ||| kind. Refused only where the rules leave that complement empty: the
  ||| universal player word covers every person in the game [CR#102.1]; a
  ||| quality noun names its whole sort, which for colour is closed at five
  ||| [CR#105.1] — the cell over-reaches a domain-restricted quality noun,
  ||| whose complement is NOT empty; and [CR#120.7] makes a source the
  ||| object that dealt some damage, a position any object may occupy rather
  ||| than a property it lacks. Every other modifier has something outside it.
  ||| A CONJUNCTION is among those others, and deliberately so. By De
  ||| Morgan its complement is the disjunction of the conjuncts'
  ||| complements, and the rules read object properties one at a time --
  ||| [CR#205.2b] has an object satisfy the criteria for any of its card
  ||| types, and [CR#205.4c] makes every land without the supertype a
  ||| nonbasic land -- so "other than a basic land card" leaves nonbasic
  ||| cards and nonland cards behind. That is a property of the conjunction
  ||| and not of the modifiers inside it, which is why no arm-by-arm test
  ||| stands here: an arm whose own complement is empty contributes an empty
  ||| disjunct and takes nothing away from the others.
  public export
  negatable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  negatable AnyPlayer = False
  negatable (QualityNoun _ _) = False
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
  predSays (QualityNoun _ _) = True
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
  predSays (HappenedTo _ _ _) = True
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
  predSays (CounterCompare _ _ _) = True
  predSays (Superlative _ _ _) = True
  predSays (InZone _) = True
  predSays (And ps) = predSaysAny ps
  predSays (Or _) = True
  predSays (Not p) = predSays p
  predSays Other = True
  predSays (OtherThan _) = True
  predSays (Joined _ _) = True

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
  predNegFree (QualityNoun _ _) = True
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
  predNegFree (HappenedTo _ _ _) = True
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
  predNegFree (CounterCompare _ _ _) = True
  predNegFree (Superlative _ _ _) = True
  predNegFree (InZone _) = True
  predNegFree (And ps) = predNegFreeAll ps
  predNegFree (Or ps) = predNegFreeAll ps
  predNegFree (Not _) = False
  predNegFree Other = True
  predNegFree (OtherThan _) = True
  predNegFree (Joined l r) = predNegFree l && predNegFree r

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

  ||| What each half of a joined phrase carries. A joined phrase places
  ||| nothing — [CR#400.1] makes a zone a place where objects can be and
  ||| [CR#109.1] lists what an object is, so a phrase that may denote a
  ||| player names no zone — and the object half carries the head type the
  ||| description named, which is what the demonstrative echo reads back.
  public export
  joinHalfPayload : {k : Kind} -> Phrasal k -> Maybe CardType -> Payload k
  joinHalfPayload PhObject ty = ObjectP ty Nothing Nothing Nothing
  joinHalfPayload PhPlayer ty = PlayerP
  joinHalfPayload {k = Quality q} PhQuality ty = QualityP
  joinHalfPayload PhAbility ty = AbilityP
  joinHalfPayload (PhJoin l r) ty =
    JoinP (joinHalfPayload l ty) (joinHalfPayload r ty)

  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Phrasal k -> Predicate bs k -> Binding
  bindFor det plur PhObject p =
    MkBinding det Object plur
              (ObjectP (seedTy p)
                       (Just (zoneOr Battlefield (seedZone p)))
                       Nothing
                       (if seedsToken p then Just TokenOrigin else Nothing))
  bindFor det plur PhPlayer p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} PhQuality p = MkBinding det (Quality q) plur QualityP
  bindFor det plur PhAbility p = MkBinding det Ability plur AbilityP
  -- the branch that used to CHOOSE a payload: it now fills one in, since
  -- the kind fixes the shape and the description fixes the object half's
  -- type [CR#205.2a] — a class word naming no card type leaves none.
  bindFor det plur (PhJoin l r) p =
    MkBinding det k plur (JoinP (joinHalfPayload l (seedTy p))
                                (joinHalfPayload r (seedTy p)))

  public export
  data Noun : Bindings -> Kind -> Type where
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    AsType : (t : CardType) -> (n : Noun bs Object) ->
             (sub : Maybe Subtype) ->
             {auto 0 asc : Ascribable n} ->
             {auto 0 way : So (ascriptionOk t sub)} -> Noun bs Object
    You : Noun bs Player        -- "you" [CR#109.5]
    PlayerGroup : (w : PlayerGroupWord) -> Noun bs Player
    Each : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
           Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    Indefinite : (m : ChoiceMode bs) -> (p : Predicate bs k) ->
                 {auto ph : Phrasal k} ->
                 Noun bs k
    Definite : (p : Predicate bs k) ->
               {auto ph : Phrasal k} ->
               {auto 0 uq : Uniquifying p} -> Noun bs k
    TargetGroup : (q : Quantity bs) -> (p : Predicate bs k) ->
                  {auto tk : Targetable k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} -> Noun bs k
    CountedGroup : (q : Quantity bs) -> (p : Predicate bs k) ->
                   {auto ph : Phrasal k} -> {auto 0 nz : NonZeroQ q} ->
                   {auto 0 wf : WellFormedQ q} ->
                   Noun bs k
    AllOf : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            Noun bs k
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    ||| "you and permanents you control" [CR#109.5]: two phrases coordinated
    ||| across kinds, so the pair's kind is their join. It leaves NO joint
    ||| referent — each arm mints its own bindings and no constructor writes
    ||| a third — which is why nothing reads a mixed group back. The
    ||| asymmetry with a joined HEAD is structural, not a gate: a head is a
    ||| description, and a determiner over it goes through `bindFor`.
    Both : {ka : Kind} -> {kb : Kind} -> (l : Noun bs ka) ->
           (r : Noun (nomIntro l) kb) -> Noun bs (ka \/ kb)
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 sp : SlicePossessor whose} ->
                   Noun bs Object
    SomeOf : (q : Quantity bs) -> (grp : Noun bs Object) ->
             {auto 0 gm : GroupMention grp} ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} -> Noun bs Object
    ||| "three artifact cards with different names", "two or more
    ||| permanents with the same name as one another": a constraint on the
    ||| GROUP a counted mention picks out. Every `Predicate` in this
    ||| vocabulary tests one member, and no test on one member can say that
    ||| no two of n share a name, so the constraint rides the counted
    ||| mention and not the mention's description. [CR#201.2b] states the
    ||| negative pole in exactly that shape, over the group.
    ||| It wraps rather than binds: the wrapped mention keeps its own
    ||| count, zone, head type and bindings, so the phrase stays one
    ||| mention of one referent. The gate is the count, since a group of
    ||| one has no two members to compare [CR#201.2a].
    NamesAgree : (agr : NameAgreement) -> (grp : Noun bs Object) ->
                 {auto 0 cm : CountedMention grp} ->
                 {auto 0 pl : nounPlur grp = ManyOf} -> Noun bs Object
    TheRest : {auto 0 ok : So (theRestOk bs)} -> Noun bs Object
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    Them : {auto 0 ok : countManys Object bs = 1} -> Noun bs Object
    Those : (w : NounWord) -> {auto 0 ok : countManyWord w bs = 1} -> Noun bs (kindOfW w)
    That : (w : NounWord) -> {auto 0 ok : countWord w bs = 1} -> Noun bs (kindOfW w)
    AttachHost : (w : AttachWord) -> (h : NounWord) ->
                 {auto 0 ok : AttachHeadOk w h} -> Noun bs (kindOfW h)
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                (marking : VerbedMarking) ->
                {auto 0 ok : countVerbed v w bs = 1} ->
                {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    ThoseVerbed : (v : VerbName) -> (w : NounWord) ->
                  (marking : VerbedMarking) ->
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
  nounEqRef (AsType _ _ _) _ = False
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
  nounEqRef (Both _ _) _ = False
  nounEqRef (LibrarySlice _ _ _) _ = False
  nounEqRef (SomeOf _ _) _ = False
  nounEqRef (NamesAgree _ _) _ = False
  nounEqRef TheRest _ = False
  nounEqRef It It = True
  nounEqRef It _ = False
  nounEqRef They They = True
  nounEqRef They _ = False
  nounEqRef Them _ = False
  nounEqRef (Those _) _ = False
  nounEqRef (That _) _ = False
  nounEqRef (AttachHost _ _) _ = False
  nounEqRef (TheVerbed _ _ _) _ = False
  nounEqRef (ThoseVerbed _ _ _) _ = False
  nounEqRef (ControllerOf _) _ = False
  nounEqRef (OwnerOf _) _ = False
  nounEqRef (Designated _ _) _ = False

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
                   DestOk (LibraryAt place arrg offs {af} Bare)

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
  nounDelta (AsType t n _) = nounDelta n
  nounDelta You = []
  nounDelta (PlayerGroup _) = []
  nounDelta (Each p {ph}) = bindFor EachD ManyOf ph p :: predDelta p
  nounDelta (Indefinite m p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (Definite p {ph}) = bindFor TheD OneOf ph p :: predDelta p
  nounDelta (TargetGroup q p {tk}) =
    bindFor TargetD (quantPlur q) (targetablePhrasal tk) p
      :: (quantDelta q ++ predDelta p)
  nounDelta (CountedGroup q p {ph}) =
    bindFor CountD (quantPlur q) ph p :: (quantDelta q ++ predDelta p)
  nounDelta (AllOf p {ph}) = bindFor AllD ManyOf ph p :: predDelta p
  nounDelta (EachOf grp) = nounDelta grp
  nounDelta (Both l r) = nounDelta r ++ nounDelta l
  nounDelta (LibrarySlice pos amt whose) =
    MkBinding TheD Object (outputPlur (nounPlur whose) (amtPlur amt))
              (ObjectP Nothing (Just Library) Nothing Nothing)
      :: nounDelta whose
  -- the constraint is a modifier on the wrapped mention, so the mention
  -- binds once and the phrase reads back as itself.
  nounDelta (NamesAgree _ grp) = nounDelta grp
  nounDelta (SomeOf q grp) =
    MkBinding PartD Object (quantPlur q) (ObjectP (nounTy grp) (nounZone grp) Nothing Nothing)
      :: (quantDelta q ++ nounDelta grp)
  nounDelta TheRest = []
  nounDelta It = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (That w) = []
  nounDelta (AttachHost _ _) = []
  nounDelta (Those w) = []
  nounDelta (TheVerbed v w _) = []
  nounDelta (ThoseVerbed v w _) = []
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
  predDelta (HappenedTo _ _ what) = complementDelta what
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
  predDelta (Joined l r) = predDelta l ++ predDelta r
  predDelta (CounterCompare _ _ b) = amtDelta b
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
  zoneDelta (LibraryAt pl _ _ (OwnedBy n)) = placeDelta pl ++ nounDelta n
  zoneDelta (LibraryAt pl _ _ Bare) = placeDelta pl

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
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PlayerStatOf : (w : PlayerStat) -> (n : Noun bs Player) ->
                   {auto 0 one : nounPlur n = OneOf} -> Amount bs
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    CountOf : {k : Kind} -> (p : Predicate bs k) ->
              Amount bs
    Aggregate : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                (p : Predicate bs k) ->
                {auto 0 sc : projScope ax = k} ->
                Amount bs
    CountersOn : {k : Kind} -> (kind : CounterKind) -> (holder : Noun bs k) ->
                 {auto 0 sc : counterScope kind = k} ->
                 {auto 0 one : nounPlur holder = OneOf} -> Amount bs
    EventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
                 (w : Lookback) ->
                 (what :
                    Maybe (EventComplement (nomIntro who) ev k)) ->
                 {auto 0 cw : ComplementWritten what} ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
    Times : (per : Nat) -> (a : Amount bs) ->
            {auto 0 nz : IsSucc per} -> Amount bs
    ||| "that much": the quantity an earlier clause's outcome wrote. The
    ||| gate counts the outcome mentions that CARRY a number
    ||| (`outcomeIsQuantity`) rather than every outcome mention, because a
    ||| coin flip leaves one that carries none [CR#705.2].
    ThatMuch : {auto 0 ok : countQuantOutcomes bs = 1} -> Amount bs
    PreventedThisWay : {auto 0 ok : countOutcomes DamagePrevented bs = 1} ->
                       Amount bs
    ||| "the result": the number on the die a clause rolled. [CR#706.2]
    ||| names it -- the natural result once every modifier has been
    ||| applied -- so the read is sorted to the roll rather than left to
    ||| `ThatMuch`, exactly as `PreventedThisWay` is sorted to prevention.
    ||| -- spelling: "equal to the result", "where X is the result"
    TheResult : {auto 0 ok : countOutcomes RollResult bs = 1} -> Amount bs
    GroupSize : {auto 0 ok : countManysAny bs = 1} -> Amount bs
    TheDifference : {auto 0 ok : countOnes Gap bs = 1} -> Amount bs
    ||| The letter, wherever the text writes it. It INTRODUCES the letter
    ||| when no earlier mention -- cost or text -- put one in the prefix,
    ||| and reads it otherwise; cost X and text X are one variable
    ||| [CR#107.3i]. Ungated, and no discharge gate stands at the ability
    ||| boundary either: the rules give every X a value. An ability may
    ||| define it [CR#107.3c]; a cost announces it [CR#107.3a]; failing
    ||| both, its controller chooses it [CR#107.3]; and in a gained
    ||| ability that defines none it is 0 [CR#107.3j]. A gate would refuse
    ||| those last two, which are rules-meaningful, so an undefined letter
    ||| leaving an ability is recorded and not refused.
    LetterVal : (l : Letter) -> Amount bs
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    Minus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    ||| "your devotion to [color]" / "… to [color] and [color]": the count
    ||| of mana symbols of the named colour(s) among the mana costs of
    ||| permanents the player controls [CR#700.5]. Its own row, never
    ||| `CountOf`: the domain is symbols inside costs, not a set of
    ||| objects. The pair slot is UNGATED: [CR#700.5] computes a pair over
    ||| the symbols that are "[color 1], [color 2], or both colors", which
    ||| is well defined when the two names coincide (it is then the
    ||| single-colour count), so no rule refuses the repeated colour.
    ||| -- spelling: "[whose] devotion to [color]"; with the second colour,
    ||| "[whose] devotion to [color] and [color]".
    Devotion : (who : Noun bs Player) -> (c : Chroma.Color) ->
               (d : Maybe Chroma.Color) ->
               {auto 0 one : nounPlur who = OneOf} -> Amount bs
    ||| "half [amt], rounded down/up": the halving read the corpus writes,
    ||| generalising `RoundMode`'s one existing site (`DamageScale.Halved`).
    ||| The mode is a required slot: oracle text always writes it.
    ||| -- spelling: "half [amt], rounded down" / "…, rounded up".
    Half : (r : RoundMode) -> (a : Amount bs) -> Amount bs
    ||| "the difference between [a] and [b]": the SYMMETRIC margin of two
    ||| written amounts — plain English's absolute difference, so the
    ||| directional floored `Minus` stays untouched beside it.
    ||| -- spelling: "the difference between [a] and [b]".
    DifferenceBetween : (a : Amount bs) -> (b : Amount (amtIntro a)) ->
                        Amount bs
    ||| "the amount of damage dealt to you this turn", "the amount of life
    ||| you gained this turn": `EventCount`'s numeric twin — the SUM of a
    ||| magnitude-bearing event's amounts over the lookback window, gated
    ||| to the events that happen in an amount (`eventHasMagnitude`).
    ||| -- spelling: "the amount of [event phrase] [window]", "the total
    ||| amount of …".
    EventSum : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               (what :
                  Maybe (EventComplement (nomIntro who) ev k)) ->
               {auto 0 cw : ComplementWritten what} ->
               {auto 0 sb : LookbackSubject ev k} ->
               {auto 0 qm : So (eventHasMagnitude ev)} -> Amount bs
    ||| "the total power of the sacrificed creatures", "the greatest power
    ||| among them", "their total toughness": the fold whose complement is
    ||| a group MENTION rather than a description — the plural twin of
    ||| `StatOf`, whose gate takes one referent. A second slot shape, no
    ||| new fold machinery.
    ||| -- spelling: with `SumOf`, "the total [axis] of [grp]" and the
    ||| possessive "[grp]'s total [axis]"; with an extremal op, "the
    ||| greatest/least [axis] among [grp]".
    AggregateOf : (op : AggregateOp) -> (ax : ProjAxis) ->
                  {k : Kind} -> (grp : Noun bs k) ->
                  {auto 0 sc : projScope ax = k} ->
                  {auto 0 pl : nounPlur grp = ManyOf} -> Amount bs
    ||| "the greatest number of creatures a player controls": the fold
    ||| whose per-element read is RELATIVIZED to the member — the element
    ||| binder both prior arts carry (core's `Projection` over a bound
    ||| `It`; the legacy module's `Project`/`bindIt`). The domain binds one
    ||| member (`TheD`, `OneOf`) for the body to read back as `It`/`They`.
    ||| The body's own phrase deltas are NOT exported: a per-member phrase
    ||| has no single announcement to make, so a target written inside the
    ||| body is dropped — tolerated overgeneration, noted here.
    ||| -- spelling: "the greatest/least [body] [domain relative clause]",
    ||| e.g. "the greatest number of creatures a player controls".
    AggregateOver : {k : Kind} -> (op : AggregateOp) ->
                    (dom : Predicate bs k) -> {auto ph : Phrasal k} ->
                    (body : Amount (bindFor TheD OneOf ph dom
                                      :: (predDelta dom ++ bs))) ->
                    Amount bs

  public export
  amtDelta : {bs : Bindings} -> Amount bs -> List Binding
  amtDelta (Lit _) = []
  amtDelta (StatOf _ nom) = nounDelta nom
  amtDelta (PlayerStatOf _ nom) = nounDelta nom
  amtDelta (CountersOn _ holder) = nounDelta holder
  amtDelta (EventCount _ who _ what) = nounDelta who ++ complementDelta what
  amtDelta (CountOf p) = predDelta p
  amtDelta (Aggregate _ _ p) = predDelta p
  amtDelta (Times _ a) = amtDelta a
  amtDelta ThatMuch = []
  amtDelta PreventedThisWay = []
  amtDelta TheResult = []
  amtDelta GroupSize = []
  amtDelta TheDifference = []
  amtDelta (LetterVal l) = letterDelta l bs
  amtDelta (Plus a b) = amtDelta a ++ amtDelta b
  amtDelta (Minus a b) = amtDelta a ++ amtDelta b
  amtDelta (Devotion who _ _) = nounDelta who
  amtDelta (Half _ a) = amtDelta a
  amtDelta (DifferenceBetween a b) = amtDelta a ++ amtDelta b
  amtDelta (EventSum _ who _ what) = nounDelta who ++ complementDelta what
  amtDelta (AggregateOf _ _ grp) = nounDelta grp
  amtDelta (AggregateOver _ dom _) = predDelta dom

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (PlayerStatOf w nom) = nomIntro nom
  amtIntro (CountersOn _ holder) = nomIntro holder
  amtIntro (EventCount _ who _ what) = complementDelta what ++ nomIntro who
  amtIntro (CountOf p) = predDelta p ++ bs
  amtIntro (Aggregate _ _ p) = predDelta p ++ bs
  amtIntro (Times per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro PreventedThisWay = bs
  amtIntro TheResult = bs
  amtIntro GroupSize = bs
  amtIntro TheDifference = bs
  amtIntro (LetterVal l) = letterDelta l bs ++ bs
  amtIntro (Plus a b) = amtIntro b
  amtIntro (Minus a b) = amtIntro b
  amtIntro (Devotion who _ _) = nomIntro who
  amtIntro (Half _ a) = amtIntro a
  amtIntro (DifferenceBetween a b) = amtIntro b
  amtIntro (EventSum _ who _ what) = complementDelta what ++ nomIntro who
  amtIntro (AggregateOf _ _ grp) = nomIntro grp
  amtIntro (AggregateOver _ dom _) = predDelta dom ++ bs

  ||| An unwritten amount introduces nothing: the slot's absence is the
  ||| bare "all" spelling, not a mention a later clause could read.
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
  amtPlur (EventCount _ _ _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Aggregate _ _ _) = ManyOf
  amtPlur (Times _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur PreventedThisWay = ManyOf
  amtPlur TheResult = ManyOf
  amtPlur GroupSize = ManyOf
  amtPlur TheDifference = ManyOf
  amtPlur (LetterVal _) = ManyOf
  amtPlur (Plus _ _) = ManyOf
  amtPlur (Minus _ _) = ManyOf
  amtPlur (Devotion _ _ _) = ManyOf
  amtPlur (Half _ _) = ManyOf
  amtPlur (DifferenceBetween _ _) = ManyOf
  amtPlur (EventSum _ _ _ _) = ManyOf
  amtPlur (AggregateOf _ _ _) = ManyOf
  amtPlur (AggregateOver _ _ _) = ManyOf

  public export
  writtenBound : {0 bs : Bindings} -> Amount bs -> Bool
  writtenBound (Lit _) = True
  writtenBound (StatOf _ _) = False
  writtenBound (PlayerStatOf _ _) = False
  writtenBound (CountersOn _ _) = False
  writtenBound (EventCount _ _ _ _) = False
  writtenBound (CountOf _) = False
  writtenBound (Aggregate _ _ _) = False
  writtenBound (Times _ _) = False
  writtenBound ThatMuch = False
  writtenBound PreventedThisWay = False
  writtenBound TheResult = False
  writtenBound GroupSize = False
  writtenBound TheDifference = False
  writtenBound (LetterVal _) = True
  writtenBound (Plus _ _) = False
  writtenBound (Minus _ _) = False
  writtenBound (Devotion _ _ _) = False
  writtenBound (Half _ _) = False
  writtenBound (DifferenceBetween _ _) = False
  writtenBound (EventSum _ _ _ _) = False
  writtenBound (AggregateOf _ _ _) = False
  writtenBound (AggregateOver _ _ _) = False

  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq (LetterVal a) (LetterVal b) = a == b
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
  readAmount (EventCount _ _ _ _) = True
  readAmount (CountOf _) = True
  readAmount (Aggregate _ _ _) = True
  readAmount (Times _ _) = False
  readAmount ThatMuch = False
  readAmount PreventedThisWay = False
  -- the die's number is a read of the roll, not a re-mention of a
  -- quantity the text already stated, so a comparison may take it as
  -- its subject: "If the result is 0 or less, …" [CR#706.2].
  readAmount TheResult = True
  readAmount GroupSize = False
  readAmount TheDifference = True
  -- the announced letter READS game state — the value its announcement
  -- fixed [CR#107.3a] — so "If X is 1" measures a fact, where a bare
  -- numeral on the left states arithmetic (badCompareLiteralSubject).
  readAmount (LetterVal _) = True
  readAmount (Plus _ _) = False
  readAmount (Minus _ _) = False
  readAmount (Devotion _ _ _) = True
  readAmount (Half _ _) = False
  readAmount (DifferenceBetween _ _) = False
  readAmount (EventSum _ _ _ _) = True
  readAmount (AggregateOf _ _ _) = True
  readAmount (AggregateOver _ _ _) = True

  public export
  ReadAmount : Amount bs -> Type
  ReadAmount {bs} a = So (readAmount a)

  ||| How many of a described set a phrase picks out. Moved here from the
  ||| word catalog the day a bound became able to carry a written AMOUNT:
  ||| "up to X target creatures" prints, so the ceiling column is open to
  ||| the amount vocabulary, and that vocabulary lives in this mutual
  ||| block. The literal gates below keep their literal rows — an amount
  ||| ceiling is statically no bound at all, so each gate answers it
  ||| whole-constructor rather than becoming runtime-undecidable.
  public export
  data Quantity : Bindings -> Type where
    Range : Maybe Nat -> Maybe Nat -> Quantity bs
    ||| "up to [amt]": the ceiling that is a written amount. Only the
    ||| ceiling arm is minted — the printed forms are ceilings — and a
    ||| written floor or exact amount waits on a measured line.
    ||| -- spelling: "up to [amt]".
    UpToOf : (a : Amount bs) -> Quantity bs

  public export
  data NonZeroQ : Quantity bs -> Type where
    UnboundedAbove : NonZeroQ (Range lo Nothing)
    MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))
    -- "up to X" admits X = 0 at resolution and still permits one when
    -- X is positive; the statically-zero ceiling the literal arm refuses
    -- (badZeroGroup) cannot be written here.
    AmountCeiling : NonZeroQ (UpToOf a)

  public export
  ||| A range whose floor is above its ceiling picks out nothing;
  ||| [CR#107.1c] otherwise leaves the vocabulary open, and a written zero
  ||| floor is a second spelling of "any number of".
  quantWellFormed : {0 bs : Bindings} -> Quantity bs -> Bool
  quantWellFormed (Range Nothing _) = True
  quantWellFormed (Range (Just _) Nothing) = True
  quantWellFormed (Range (Just lo) (Just hi)) = lte lo hi
  quantWellFormed (UpToOf _) = True

  public export
  WellFormedQ : Quantity bs -> Type
  WellFormedQ q = So (quantWellFormed q)

  public export
  quantPlur : {0 bs : Bindings} -> Quantity bs -> Plurality
  quantPlur (Range _ (Just (S Z))) = OneOf
  quantPlur (Range _ _) = ManyOf
  -- "up to X creatures" is written plural whatever X resolves to.
  quantPlur (UpToOf _) = ManyOf

  public export
  ||| [CR#700.2]: a mode is chosen from the list printed on the card, so a
  ||| headcount reaching past the list names modes that are not there. An
  ||| amount headcount is not statically past any list, so it is admitted.
  modesFit : {0 bs : Bindings} -> Quantity bs -> Nat -> Bool
  modesFit (Range Nothing Nothing) n = True
  modesFit (Range Nothing (Just hi)) n = lte hi n
  modesFit (Range (Just lo) Nothing) n = lte lo n
  modesFit (Range (Just lo) (Just hi)) n = lte hi n
  modesFit (UpToOf _) n = True

  public export
  ModesFit : Quantity bs -> Nat -> Type
  ModesFit q n = So (modesFit q n)

  ||| [CR#706.3a] gives a results table's left column three forms — a
  ||| single number, "N1—N2", "N+" — all numbers, so a table row's range
  ||| is literal by rule.
  public export
  quantLiteral : {0 bs : Bindings} -> Quantity bs -> Bool
  quantLiteral (Range _ _) = True
  quantLiteral (UpToOf _) = False

  ||| What a quantity's bound mentions: nothing for the literal ranges,
  ||| the amount's own delta for the amount ceiling — so "up to X target
  ||| creatures" introduces its letter like any other written X.
  public export
  quantDelta : {bs : Bindings} -> Quantity bs -> List Binding
  quantDelta (Range _ _) = []
  quantDelta (UpToOf a) = amtDelta a

  public export
  Bindingless : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Bindingless {bs} {k} n = nounDelta n = []

  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase This = True
  anchorPhrase (AsType t n _) = anchorPhrase n
  anchorPhrase You = True
  anchorPhrase (PlayerGroup _) = True
  anchorPhrase (Each _) = False
  anchorPhrase (Indefinite _ _) = False
  anchorPhrase (Definite _) = False
  anchorPhrase (TargetGroup _ _) = True
  anchorPhrase (CountedGroup _ _) = False
  anchorPhrase (AllOf _) = False
  anchorPhrase (EachOf _) = False
  anchorPhrase (Both _ _) = False
  anchorPhrase (LibrarySlice _ _ _) = False
  anchorPhrase (SomeOf _ _) = False
  anchorPhrase (NamesAgree _ grp) = anchorPhrase grp
  anchorPhrase TheRest = False
  anchorPhrase It = True
  anchorPhrase They = True
  anchorPhrase Them = True
  anchorPhrase (Those _) = True
  anchorPhrase (That _) = True
  anchorPhrase (AttachHost _ _) = True
  anchorPhrase (TheVerbed _ _ _) = True
  anchorPhrase (ThoseVerbed _ _ _) = True
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
                       LinkSource (AsType t This Nothing {asc} {way})

  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable This = False
  choosable (AsType _ _ _) = False
  choosable You = False
  choosable (PlayerGroup _) = False
  choosable (Each _) = False
  choosable (Indefinite _ _) = True
  choosable (Definite _) = False
  choosable (TargetGroup _ _) = True
  choosable (CountedGroup _ _) = True
  choosable (AllOf _) = False
  choosable (EachOf _) = False
  choosable (Both _ _) = False
  choosable (LibrarySlice _ _ _) = False
  choosable (SomeOf _ _) = False
  choosable (NamesAgree _ grp) = choosable grp
  choosable TheRest = False
  choosable It = False
  choosable They = False
  choosable Them = False
  choosable (Those _) = False
  choosable (That _) = False
  choosable (AttachHost _ _) = False
  choosable (TheVerbed _ _ _) = False
  choosable (ThoseVerbed _ _ _) = False
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
  groupMention (AsType _ _ _) = False
  groupMention You = False
  groupMention (PlayerGroup _) = False
  groupMention (Each _) = False
  groupMention (Indefinite _ _) = False
  groupMention (Definite _) = False
  groupMention (CountedGroup _ _) = False
  groupMention (AllOf _) = False
  groupMention (EachOf _) = False
  groupMention (Both _ _) = False
  groupMention (LibrarySlice _ _ _) = True
  groupMention (SomeOf _ _) = False
  groupMention (NamesAgree _ grp) = groupMention grp
  groupMention TheRest = False
  groupMention It = False
  groupMention They = False
  groupMention (That _) = False
  groupMention (AttachHost _ _) = False
  groupMention (TheVerbed _ _ _) = False
  groupMention (ThoseVerbed _ _ _) = True
  groupMention (ControllerOf _) = False
  groupMention (OwnerOf _) = False
  groupMention (Designated _ _) = False

  public export
  GroupMention : Noun bs k -> Type
  GroupMention {bs} {k} n = So (groupMention n)

  ||| Which mentions write their own headcount: the two that carry a
  ||| `Quantity` over their own description. A partitive counts a slice of a
  ||| group some earlier phrase introduced, and every other mention counts
  ||| nothing, so neither has a written group for a group-level constraint
  ||| to attach to.
  public export
  countedMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedMention (CountedGroup _ _) = True
  countedMention (TargetGroup _ _) = True
  countedMention _ = False

  public export
  CountedMention : Noun bs k -> Type
  CountedMention {bs} {k} n = So (countedMention n)

  ||| The counted mention a condition can quantify over: a counted mention
  ||| UNDER a group-level name constraint, and nothing else. The bare form
  ||| is refused twice over. "Three or more artifacts" already has a
  ||| spelling — a comparison against the described set's headcount — and a
  ||| second one would say the same thing in a second shape. And a targeted
  ||| group would be worse than redundant: a comparison's two amounts carry
  ||| their phrases' bindings out to the clause they govern, where
  ||| [CR#601.2c] has every target announced, while this condition tests and
  ||| introduces nothing, so a target written inside it would be announced
  ||| nowhere.
  public export
  countedExistential : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  countedExistential (NamesAgree _ grp) = countedMention grp
  countedExistential _ = False

  public export
  CountedExistential : Noun bs k -> Type
  CountedExistential {bs} {k} n = So (countedExistential n)

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
  selfDefinedOk (AsType _ n _) = selfDefinedOk n
  selfDefinedOk _ = False

  public export
  SelfDefined : {bs : Bindings} -> Noun bs Object -> Type
  SelfDefined {bs} n = So (selfDefinedOk n)

  public export
  data Condition : Bindings -> Type where
    Exists : {k : Kind} -> (p : Predicate bs k) ->
             Condition bs
    ||| "if you control two or more nonland, nontoken permanents with the
    ||| same name as one another", "if you control three or more lands with
    ||| the same name": the counted existential. `Exists` asks whether ANY
    ||| object answers a description; this asks whether a group of the
    ||| written size does, which is the only place a group-level constraint
    ||| [CR#201.2a] can be tested, since the constraint rides the mention
    ||| and a description takes no count. That constraint is the whole
    ||| reason it exists, and its gate admits nothing else: a bare counted
    ||| mention is the comparison's business, and a targeted one would go
    ||| unannounced here. It tests and names nothing, so it introduces
    ||| nothing, like every other described-set condition.
    ExistsGroup : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 ce : CountedExistential n} -> Condition bs
    Happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               (what :
                  Maybe (EventComplement (nomIntro who) ev k)) ->
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
              Condition bs
    ||| The bound slot is open to ANY amount at every relation: the rules
    ||| make a comparison against any number meaningful, and what bound
    ||| shapes English prints at which relation is spelling-boundary
    ||| knowledge, recorded on the deciding ticket rather than gated here.
    CompareAmt : (subj : Amount bs) -> (r : Comparator) ->
                 (bound : Amount (amtIntro subj)) ->
                 {auto 0 rd : ReadAmount subj} ->
                 Condition bs
    ||| "If a player is dealt damage this way, ..." (Screaming Nemesis),
    ||| "If a creature is dealt damage this way, ..." (Burn from Within):
    ||| the kind refinement on damage the text has dealt.
    ||| [CR#608.2c] is the rule that lets later text read the instruction it
    ||| follows -- one of its two worked examples is a "...this way"
    ||| back-reference -- and it settles no more than that the reading is
    ||| available. WHICH earlier instruction "this way" names is left to
    ||| whoever reads the card, so the licence here is deliberately loose:
    ||| the gate asks only that SOME damage a clause dealt is in scope
    ||| (`damageDealtInScope`), and no discrimination between two of them is
    ||| attempted. A text carrying two damage mentions can therefore write a
    ||| refinement pointing at one and an anaphor resolving against the
    ||| other; that mis-pairing is tolerated overgeneration, refused at the
    ||| spelling boundary, and narrowing it would need the discourse to
    ||| order its outcome mentions, which nothing else asks for.
    ||| The description is bounded by what damage can be dealt
    ||| to -- battles, creatures, planeswalkers and players [CR#120.1] --
    ||| whose coarsest bound in the kind lattice is `Object \/ Player`; a
    ||| refinement at any other kind is a category error.
    ||| It tests and names no referent, like every other described-set
    ||| condition. Under a joined kind it needs to name none: the body's
    ||| "they" reads the union binding the damage went to, at its player
    ||| half, and this condition is what discharges the presupposition that
    ||| read carries.
    ||| -- spelling: "If [description] is dealt damage this way, [e]."
    DealtThisWay : {k : Kind} -> (p : Predicate bs k) ->
                   {auto 0 wy : So (damageDealtInScope bs)} ->
                   {auto 0 rk : So (kindLte k (Object \/ Player))} ->
                   Condition bs
    ||| "If you win the flip, …", "If you lose the flip, …": the called
    ||| reading of a coin an earlier clause flipped. [CR#705.2] settles
    ||| both halves of the shape — the flipper calls heads or tails and
    ||| wins the flip when the call matches, and "only the player who
    ||| flips the coin wins or loses the flip; no other players are
    ||| involved", which is why the subject is a slot and not the whole
    ||| construction's business. The arms are conditions over the flip
    ||| rather than slots ON it because the same rule gives a flip a
    ||| SECOND reading (`FlipFace`) that no winner attends, and because
    ||| [CR#705.1]'s flip is complete without either: a cumulative upkeep
    ||| that is a bare flip writes no arm at all.
    ||| The gate is an existence test on the flip's mention, like every
    ||| other back-reference to a clause's own outcome; it tests and
    ||| names no referent, so it introduces nothing.
    ||| -- spelling: "If you win the flip, [e]." / "… lose the flip, …"
    FlipCalled : (who : Noun bs Player) -> (call : FlipCall) ->
                 {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
    ||| "If the coin comes up heads, …": [CR#705.2]'s other reading, for
    ||| the effects that "care only about whether the coin comes up heads
    ||| or tails". The rule says no player wins or loses a flip read this
    ||| way, so this condition takes no subject.
    ||| -- spelling: "If the coin comes up heads, [e]."; after a flip
    ||| already named, "If it comes up tails, [e]."
    FlipFace : (face : CoinFace) ->
               {auto 0 fl : So (coinFlipInScope bs)} -> Condition bs
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
  condNegated (ExistsGroup _) = False
  condNegated (Happened _ _ _ _) = False
  condNegated (GameIs _) = False
  -- atomic: the absence is the condition's own content, not a marked
  -- negation of one.
  condNegated (NoHolder _) = False
  condNegated (Matches _ _) = False
  condNegated (CompareAmt _ _ _) = False
  condNegated (DealtThisWay _) = False
  condNegated (FlipCalled _ _) = False
  condNegated (FlipFace _) = False
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
  condDelta (ExistsGroup _) = []
  condDelta (Happened _ _ _ _) = []
  condDelta (GameIs _) = []
  condDelta (NoHolder _) = []
  condDelta (Matches (AsType t This _) _) =
    [MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  condDelta (Matches (AttachHost _ (TypeW t)) _) =
    [MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing)]
  condDelta (Matches (AttachHost _ PermanentW) _) =
    [MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing)]
  condDelta (Matches _ _) = []
  condDelta (CompareAmt subj _ bound) = gapB :: (amtDelta bound ++ amtDelta subj)
  condDelta (DealtThisWay _) = []
  condDelta (FlipCalled _ _) = []
  condDelta (FlipFace _) = []
  condDelta (NotCond c) = dropGaps (condDelta c)
  condDelta (AndCond cs) = condDeltaAll cs

  public export
  condDeltaAll : {bs : Bindings} -> List (Condition bs) -> List Binding
  condDeltaAll [] = []
  condDeltaAll (c :: cs) = condDelta c ++ condDeltaAll cs

  ||| A negated condition unmakes no announcement — a target written
  ||| inside "unless [comparison]" is announced at casting like any other
  ||| [CR#601.2c] — but a comparison that did NOT hold leaves no margin,
  ||| so the `Gap` binding alone is dropped.
  public export
  dropGaps : List Binding -> List Binding
  dropGaps [] = []
  dropGaps (MkBinding _ Gap _ GapP :: bs) = dropGaps bs
  dropGaps (b :: bs) = b :: dropGaps bs

  public export
  data Duration : Bindings -> Type where
    ThisTurn : Duration bs
    ||| [CR#702.131a] and [CR#702.195a] both write "for the rest of the
    ||| game", which no turn-part endpoint spells.
    RestOfGame : Duration bs
    Until : DurationEnd -> Duration bs
    ForAsLongAs : Condition bs -> Duration bs
    UntilEvent : GameEvent bs -> Duration bs

  ||| The card types an attack may name [CR#506.3]: "Only a player, a
  ||| planeswalker, or a battle can be attacked." A creature is never
  ||| attacked, and neither is any other permanent type.
  public export
  attackableTy : Maybe CardType -> Bool
  attackableTy (Just Planeswalker) = True
  attackableTy (Just Battle) = True
  attackableTy _ = False

  ||| The same closed set read off a phrase's kind. A player is attackable
  ||| however it is described; an object only at an attackable type; a
  ||| joined phrase only when both halves are, so "that player or
  ||| planeswalker" passes while "any target" [CR#115.4] does not -- its
  ||| object half projects no single attackable type.
  public export
  attackableKind : Kind -> Maybe CardType -> Bool
  attackableKind Player _ = True
  attackableKind Object t = attackableTy t
  attackableKind (a \/ b) t = attackableKind a t && attackableKind b t
  attackableKind _ _ = False

  ||| The phrase names something that can be attacked [CR#506.3]. The
  ||| joined kind alone would admit a creature defender, which the rule
  ||| closes out.
  public export
  Attackable : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Type
  Attackable {k} n = So (attackableKind k (nounTy n))

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
    CountedTokens : {0 q : Quantity bs} -> {0 p : Predicate bs Object} ->
                    {0 ph : Phrasal Object} -> {0 nz : NonZeroQ q} ->
                    {0 wf : WellFormedQ q} ->
                    {auto 0 ok : So (seedsToken p)} ->
                    TokenPhrase (CountedGroup q p {ph} {nz} {wf})
    OneToken : {0 m : ChoiceMode bs} -> {0 p : Predicate bs Object} ->
               {0 ph : Phrasal Object} ->
               {auto 0 ok : So (seedsToken p)} ->
               TokenPhrase (Indefinite m p {ph})

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
    CounterEvent : (dir : CounterMove) -> (kind : Maybe CounterKind) ->
                   (n : Noun bs Object) ->
                   (many : CounterBatch) ->
                   (by : Maybe (Noun bs Player)) ->
                   (cause : Maybe Causer) ->
                   {auto 0 kn : CounterKindNamed Object kind} ->
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

  public export
  selfSubjIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  selfSubjIntro (AsType t This _) =
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
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
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
    ||| Counters leave a player in a stated number as well as all at
    ||| once: [CR#728.1]'s own rules text has a player remove "one rad
    ||| counter from themselves", and printed removal lines count what
    ||| they take off an opponent. The amount is therefore a slot, and
    ||| leaving it unwritten is the "all" spelling rather than the only
    ||| reading available.
    LosesCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                    (amt : Maybe (Amount (nomIntro who))) ->
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
  heldUntilOk (LosesCounters _ _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
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
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (GetsEmblem _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  heldUntilOk (MoveCounters _ _ _ _) = False
  heldUntilOk (PutCountersOfThoseKinds _ _) = False
  heldUntilOk (Composite _ (Move _ _ _)) = True
  heldUntilOk (Composite _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (OnlyIf _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (Unless _ _ _) = False
  heldUntilOk (Define _ _) = False
  heldUntilOk (ForEachOf _ _) = False
  heldUntilOk (Repeat _) = False
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
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _) = EncReflexive        -- 66, all of them offered
  reflexEncloseUse (Composite _ _) = EncReflexive  -- 51, every one an exile
  -- a status change is the effect's, not a player's: [CR#603.12]'s
  -- agent form has no subject to inflect.
  reflexEncloseUse (SetStatus _ _) = EncAgentless
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (LosesCounters _ _ _) = EncAgentless
  reflexEncloseUse (RemoveFromCombat _) = EncAgentless
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
  thisWayOutcomeOk (LosesCounters _ _ _) = True
  thisWayOutcomeOk (RemoveFromCombat _) = True
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
  thisWayOutcomeOk (Continuously _ _) = True
  thisWayOutcomeOk (Create _ _ _ _) = True
  thisWayOutcomeOk (GetsEmblem _ _) = True
  thisWayOutcomeOk (PutCounters _ _ _) = True
  thisWayOutcomeOk (RemoveCounters _ _ _) = True
  thisWayOutcomeOk (MoveCounters _ _ _ _) = True
  thisWayOutcomeOk (PutCountersOfThoseKinds _ _) = True
  thisWayOutcomeOk (Composite _ _) = True
  thisWayOutcomeOk (Does _ _ _) = True
  thisWayOutcomeOk (Pay _ _) = True
  thisWayOutcomeOk (OnlyIf _ _ _) = True
  thisWayOutcomeOk (If _ _ _) = True
  thisWayOutcomeOk (Unless _ _ _) = True
  thisWayOutcomeOk (Define _ _) = True
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
  costNounOk (AsType t n _) = costNounOk n
  costNounOk You = True
  costNounOk (PlayerGroup _) = True
  costNounOk (Each _) = True
  costNounOk (Indefinite _ _) = True
  costNounOk (Definite _) = True
  costNounOk (TargetGroup _ _) = True
  costNounOk (CountedGroup _ _) = True
  costNounOk (AllOf _) = True
  costNounOk (EachOf grp) = costNounOk grp
  costNounOk (NamesAgree _ grp) = costNounOk grp
  costNounOk (Both _ _) = False
  costNounOk (LibrarySlice _ _ _) = True
  costNounOk (SomeOf _ grp) = costNounOk grp
  costNounOk TheRest = True
  costNounOk It = True
  costNounOk They = True
  costNounOk Them = True
  costNounOk (Those _) = True
  costNounOk (That _) = True
  costNounOk (AttachHost _ _) = True
  costNounOk (TheVerbed _ _ _) = False
  costNounOk (ThoseVerbed _ _ _) = False
  costNounOk (ControllerOf _) = True
  costNounOk (OwnerOf _) = True
  costNounOk (Designated _ _) = True

  public export
  nounIsYou : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsYou You = True
  nounIsYou (PlayerGroup _) = False
  nounIsYou This = False
  nounIsYou (AsType _ _ _) = False
  nounIsYou (Each _) = False
  nounIsYou (Indefinite _ _) = False
  nounIsYou (Definite _) = False
  nounIsYou (TargetGroup _ _) = False
  nounIsYou (CountedGroup _ _) = False
  nounIsYou (AllOf _) = False
  nounIsYou (EachOf _) = False
  nounIsYou (NamesAgree _ _) = False
  nounIsYou (Both _ _) = False
  nounIsYou (LibrarySlice _ _ _) = False
  nounIsYou (SomeOf _ _) = False
  nounIsYou TheRest = False
  nounIsYou It = False
  nounIsYou They = False
  nounIsYou Them = False
  nounIsYou (Those _) = False
  nounIsYou (That _) = False
  nounIsYou (AttachHost _ _) = False
  nounIsYou (TheVerbed _ _ _) = False
  nounIsYou (ThoseVerbed _ _ _) = False
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
  costActionOk (AdditionalPart _ _ _ _) = True
  costActionOk (Distribute _ _ among) = costNounOk among
  costActionOk (Fights a _) = costNounOk a
  costActionOk (SetStatus _ n) = costNounOk n
  costActionOk (GetsCounters who _ _) = costNounOk who
  costActionOk (LosesCounters who _ _) = costNounOk who
  costActionOk (RemoveFromCombat n) = costNounOk n
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
  costActionOk (Continuously _ _) = False
  costActionOk (Create agent _ _ _) = costNounOk agent
  costActionOk (GetsEmblem _ _) = True
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (MoveCounters _ _ src dst) = costNounOk src && costNounOk dst
  -- the distributive kind anaphor reads an announced batch; no cost
  -- announces one, so the clause instructs nothing at payment.
  costActionOk (PutCountersOfThoseKinds _ _) = False
  costActionOk (Composite _ e) = costActionOk e
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
  effEq (GetsCounters _ _ _) _ = False
  effEq (LosesCounters a Nothing _) (LosesCounters b Nothing _) = nounEqRef a b
  effEq (LosesCounters a (Just j) _) (LosesCounters b (Just l) _) =
    nounEqRef a b && j == l
  effEq (LosesCounters _ _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
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
  effEq (Continuously _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (GetsEmblem _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (MoveCounters _ _ _ _) _ = False
  effEq (PutCountersOfThoseKinds _ _) _ = False
  effEq (Composite v e) (Composite w f) = v == w && effEq e f
  effEq (Composite _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (Pay _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (OnlyIf _ _ _) _ = False
  effEq (If _ _ _) _ = False
  effEq (Unless _ _ _) _ = False
  effEq (Define _ _) _ = False
  effEq (ForEachOf _ _) _ = False
  effEq (Repeat _) _ = False
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

  public export
  nounTargeted : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounTargeted (TargetGroup _ _) = True
  nounTargeted (CountedGroup _ _) = False
  nounTargeted This = False
  nounTargeted (AsType _ n _) = nounTargeted n
  nounTargeted You = False
  nounTargeted (PlayerGroup _) = False
  nounTargeted (Each _) = False
  nounTargeted (Indefinite _ _) = False
  nounTargeted (Definite _) = False
  nounTargeted (AllOf _) = False
  nounTargeted (EachOf grp) = nounTargeted grp
  nounTargeted (NamesAgree _ grp) = nounTargeted grp
  nounTargeted (Both l r) = nounTargeted l || nounTargeted r
  nounTargeted (LibrarySlice _ _ _) = False
  nounTargeted (SomeOf _ grp) = nounTargeted grp
  nounTargeted TheRest = False
  nounTargeted It = False
  nounTargeted They = False
  nounTargeted Them = False
  nounTargeted (Those _) = False
  nounTargeted (That _) = False
  nounTargeted (AttachHost _ _) = False
  nounTargeted (TheVerbed _ _ _) = False
  nounTargeted (ThoseVerbed _ _ _) = False
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
  counterMemoryOk (TheVerbed v _ _) = not (verbMoves v)
  counterMemoryOk (ThoseVerbed v _ _) = not (verbMoves v)
  counterMemoryOk _ = True

  public export
  CounterMemory : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  CounterMemory {bs} n = So (counterMemoryOk n)

  ||| [CR#122.5] makes a move impossible when the first and second objects
  ||| are the same object. A destination that is only the source read back
  ||| names that case, so it is refused here rather than left to the
  ||| engine.
  public export
  moveDestOk : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  moveDestOk It = False
  moveDestOk Them = False
  moveDestOk _ = True

  public export
  MoveDestination : {bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Type
  MoveDestination {bs} n = So (moveDestOk n)

  public export
  data DamageRecipient : Noun bs k -> Type where
    PlayerTakes : DamageRecipient {k = Player} n
    ||| A joined phrase is dealt damage on either half's account:
    ||| [CR#120.1] admits a battle, a creature, a planeswalker or a player
    ||| and nothing else, and the phrase names an object description beside
    ||| a player one. No zone is asked, because damage asks none —
    ||| `ObjectTakes` asks for one only where the phrase has one to give.
    ||| The row is the kind's: every noun at a joined kind, and no
    ||| others. That is WIDER than the three rows it replaces, whose
    ||| heads were a closed class enum or the deictic "you": a same-kind
    ||| join such as `Joined (HasType Land) (HasType Land)` now passes
    ||| without `ObjectTakes`'s type check. Overgeneration, tolerated:
    ||| such a value has no English production and is refused at the
    ||| boundary.
    JoinTakes : DamageRecipient {k = ka \/ kb} n
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
               TagBody Destroy (Move n (ZoneAt Graveyard Bare) (MkMoveRiders [] Nothing Nothing))
    SacrificeB : {auto 0 z : OnBattlefield (nounZone n)} ->
                 TagBody Sacrifice (Move n (ZoneAt Graveyard Bare) (MkMoveRiders [] Nothing Nothing))
    ExileB : TagBody Exile (Move n (ZoneAt Exile Bare) (MkMoveRiders [] Nothing Nothing))
    ExileWithCountersB : {0 amt : Amount (nomIntro n)} ->
                         {0 kind : CounterKind} ->
                         TagBody Exile
                                 (Move n (ZoneAt Exile Bare) (MkMoveRiders [] Nothing (Just (MkCounterRider amt kind))))
    DiscardB : {auto 0 d : DiscardOk n} ->
               TagBody Discard (Move n (ZoneAt Graveyard Bare) (MkMoveRiders [] Nothing Nothing))
    MillB : {0 amt : Amount bs} -> {0 whose : Noun bs Player} ->
            {auto 0 sp : SlicePossessor whose} ->
            TagBody Mill (Move (LibrarySlice OnTop amt whose {sp})
                               (ZoneAt Graveyard Bare) (MkMoveRiders [] Nothing Nothing))
    ||| [CR#701.22a] states one player twice over — the looker and the
    ||| owner of the library looked at are the same person — and says
    ||| nothing about WHICH player that is, so "target player scries 3"
    ||| (Bumi, King of Three Trials) is as much a scry as "you scry 3".
    ||| The looker is therefore a two-row table over how the clause
    ||| writes that one person rather than a value fixed at `You`: second
    ||| person writes itself, and any other subject is read back by the
    ||| anaphor, whose own gate checks the mention is there to read. Both
    ||| rows repeat the same mention in both positions, which is the rule
    ||| doing the tying.
    ScryB : {0 amt : Amount bs} ->
            {auto 0 sp : SlicePossessor {bs} You} ->
            TagBody Scry
                    (Expose LookAt You
                            (ExposedCards (LibrarySlice OnTop amt You {sp})))
    ScryTheyB : {0 amt : Amount bs} ->
                {auto 0 an : countOnes Player bs = 1} ->
                {auto 0 sp : SlicePossessor (They {bs} {ok = an})} ->
                TagBody Scry
                        (Expose LookAt (They {ok = an})
                                (ExposedCards
                                   (LibrarySlice OnTop amt (They {ok = an}) {sp})))
    ||| [CR#701.25a] words surveil the same way, over the same two rows.
    SurveilB : {0 amt : Amount bs} ->
               {auto 0 sp : SlicePossessor {bs} You} ->
               TagBody Surveil
                       (Expose LookAt You
                               (ExposedCards (LibrarySlice OnTop amt You {sp})))
    SurveilTheyB : {0 amt : Amount bs} ->
                   {auto 0 an : countOnes Player bs = 1} ->
                   {auto 0 sp : SlicePossessor (They {bs} {ok = an})} ->
                   TagBody Surveil
                           (Expose LookAt (They {ok = an})
                                   (ExposedCards
                                      (LibrarySlice OnTop amt (They {ok = an}) {sp})))
    ||| The agentive placement clause: the imperative and "<player> puts it
    ||| into/onto <zone>" spell one event, so the tag rides the same Move.
    ||| Every Move admits: its own gates already bound the destination, and
    ||| the destination's possessive is rendering's business [CR#400.3].
    PutB :
           TagBody Put (Move n to riders {ok} {arr} {pl} {rf})

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
  setZone p z (MkBinding det (LetterK l) plur LetterP) = MkBinding det (LetterK l) plur LetterP
  setZone p z (MkBinding det TurnRef plur TurnRefP) = MkBinding det TurnRef plur TurnRefP
  setZone p z (MkBinding det Ability plur AbilityP) = MkBinding det Ability plur AbilityP
  setZone p z (MkBinding det (a \/ b) plur (JoinP l r)) = MkBinding det (a \/ b) plur (JoinP l r)

  public export
  setZoneHead : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  ||| A moved singular referent is re-zoned where the pronoun that names it
  ||| would find it, so the write uses the same test `zoneOfIt` reads with.
  ||| A joined binding is reached and left alone: `setZone`'s own join row
  ||| is the identity, because moving an object says nothing about the half
  ||| of the reference that was never an object [CR#400.1].
  public export
  setZoneIt : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (b :: bs) =
    if itReaches OneOf b then setZone p z b :: bs else b :: setZoneIt p z bs

  public export
  setZoneThem : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (b :: bs) =
    if itReaches ManyOf b then setZone p z b :: bs else b :: setZoneThem p z bs

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
  moveIntro p (NamesAgree _ grp) z = moveIntro p grp z
  moveIntro p nn@(Both _ _) z = nomIntro nn
  moveIntro p nn@(LibrarySlice _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(SomeOf _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p TheRest z = groupSpent bs
  moveIntro p It z = setZoneIt p z bs
  moveIntro p Them z = setZoneThem p z bs
  moveIntro p (That w) z = setZoneThat p w z bs
  moveIntro p (Those w) z = setZoneThose p w z bs
  moveIntro p (TheVerbed v w _) z = setZoneVerbed p v w z bs
  moveIntro p (ThoseVerbed v w _) z = setZoneManyVerbed p v w z bs
  moveIntro p This z = bs
  moveIntro p (AttachHost _ _) z = bs
  moveIntro p (AsType t n _) z = MkBinding TheD Object OneOf (ObjectP (Just t) z (mkStamp p Nothing) Nothing) :: bs
  moveIntro p You z = bs
  moveIntro p (PlayerGroup _) z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)
  moveIntro p (Designated d n) z = bs

  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n _) = Just Battlefield
  nounZone You = Nothing
  nounZone (PlayerGroup _) = Nothing
  nounZone (Each p) = phraseZone p
  nounZone (Indefinite m p) = phraseZone p
  nounZone (Definite p) = phraseZone p
  nounZone (TargetGroup q p) = phraseZone p
  nounZone (CountedGroup q p) = phraseZone p
  nounZone (AllOf p) = phraseZone p
  nounZone (EachOf grp) = nounZone grp
  nounZone (NamesAgree _ grp) = nounZone grp
  nounZone (Both _ _) = Nothing
  nounZone (LibrarySlice _ _ _) = Just Library
  nounZone (SomeOf _ grp) = nounZone grp
  nounZone TheRest = zoneOfGroup bs
  nounZone It = zoneOfIt bs
  nounZone They = Nothing
  nounZone Them = zoneOfThem bs
  nounZone (That w) = zoneOfThat w bs
  nounZone (AttachHost _ h) = attachHostZone h
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w _) = zoneOfVerbed v w bs
  nounZone (ThoseVerbed v w _) = zoneOfManyVerbed v w bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing
  -- [CR#903.3]: the designation is an attribute of the card, so it
  -- survives a zone change and the noun names no zone.
  nounZone (Designated _ _) = Nothing

  public export
  nounTy : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe CardType
  nounTy This = Nothing
  nounTy (AsType t n _) = Just t
  nounTy You = Nothing
  nounTy (PlayerGroup _) = Nothing
  nounTy (Each p) = seedTy p
  nounTy (Indefinite m p) = seedTy p
  nounTy (Definite p) = seedTy p
  nounTy (TargetGroup q p) = seedTy p
  nounTy (CountedGroup q p) = seedTy p
  nounTy (AllOf p) = seedTy p
  nounTy (EachOf grp) = nounTy grp
  nounTy (NamesAgree _ grp) = nounTy grp
  nounTy (Both _ _) = Nothing
  nounTy (LibrarySlice _ _ _) = Nothing
  nounTy (SomeOf _ grp) = nounTy grp
  nounTy TheRest = tyOfGroup bs
  nounTy It = tyOfIt bs
  nounTy They = Nothing
  nounTy Them = tyOfThem bs
  nounTy (That w) = tyOfThat w bs
  nounTy (AttachHost _ h) = attachHostTy h
  nounTy (Those w) = tyOfThose w bs
  nounTy (TheVerbed v w _) = tyOfVerbed v w bs
  nounTy (ThoseVerbed v w _) = tyOfManyVerbed v w bs
  nounTy (ControllerOf n) = Nothing
  nounTy (OwnerOf n) = Nothing
  nounTy (Designated _ _) = Nothing

  public export
  nounPlur : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Plurality
  nounPlur This = OneOf
  nounPlur (AsType t n _) = nounPlur n
  nounPlur You = OneOf
  nounPlur (PlayerGroup _) = ManyOf
  nounPlur (Each p) = ManyOf
  nounPlur (Indefinite m p) = OneOf
  nounPlur (Definite p) = OneOf
  nounPlur (TargetGroup q p) = quantPlur q
  nounPlur (CountedGroup q p) = quantPlur q
  nounPlur (AllOf p) = ManyOf
  nounPlur (EachOf grp) = ManyOf
  nounPlur (NamesAgree _ grp) = nounPlur grp
  nounPlur (Both _ _) = ManyOf
  nounPlur (LibrarySlice _ amt whose) = outputPlur (nounPlur whose) (amtPlur amt)
  nounPlur (SomeOf q _) = quantPlur q
  nounPlur TheRest = ManyOf
  nounPlur It = OneOf
  nounPlur They = OneOf
  nounPlur Them = ManyOf
  nounPlur (That w) = OneOf
  nounPlur (AttachHost _ _) = OneOf
  nounPlur (Those w) = ManyOf
  nounPlur (TheVerbed v w _) = OneOf
  nounPlur (ThoseVerbed v w _) = ManyOf
  nounPlur (ControllerOf n) = OneOf
  nounPlur (OwnerOf n) = OneOf
  nounPlur (Designated _ _) = OneOf

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
  effIntro (LosesCounters who _ amt) = optAmtIntro amt
  effIntro (RemoveFromCombat n) = nomIntro n
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
  effIntro (Choose n Nothing) = nomIntro n
  effIntro (Choose n (Just b)) = nounDelta b ++ nomIntro n
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
  effIntro (Continuously se _) = staticIntro se
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
  effIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  effIntro (Composite v (Move what to _)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to _)) = moveIntro (Just v) what (Just (zoneSort to))
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
  preIntro (LosesCounters who _ amt) = optAmtIntro amt
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (Regenerate n) = nomIntro n
  preIntro (CantBe e _ _) = preIntro e
  preIntro (GainsDesignation n _ _ _) = nomIntro n
  preIntro (GameBecomes _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro (CounterSpell what) = nomIntro what
  preIntro (CopyStack agent what times exc) = amtIntro times
  preIntro (ChooseNewTargets what) = nomIntro what
  preIntro (Choose n _) = nomIntro n
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
  preIntro (Continuously se _) = staticIntro se
  preIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  preIntro (GetsEmblem who _) = nomIntro who
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters amt kind from) = nomIntro from
  preIntro (MoveCounters amt kind src dst) = nomIntro dst
  preIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  preIntro (Composite v (Move what to _)) = nomIntro what
  preIntro (Composite _ e) = preIntro e
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
  annIntro (LosesCounters who _ amt) = optAmtIntro amt
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (Regenerate n) = nomIntro n
  annIntro (CantBe e _ _) = annIntro e
  annIntro (GainsDesignation n _ _ _) = nomIntro n
  annIntro (GameBecomes _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro (CounterSpell what) = nomIntro what
  annIntro (CopyStack agent what times exc) = amtIntro times
  annIntro (ChooseNewTargets what) = nomIntro what
  annIntro (Choose n _) = nomIntro n
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
  annIntro (Continuously se _) = staticIntro se
  annIntro (Create agent count spec riders) = specDelta spec ++ amtIntro count
  annIntro (GetsEmblem who _) = nomIntro who
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters amt kind from) = nomIntro from
  annIntro (MoveCounters amt kind src dst) = nomIntro dst
  annIntro (PutCountersOfThoseKinds amt on) = nomIntro on
  annIntro (Composite v (Move what to _)) = nomIntro what
  annIntro (Composite _ e) = annIntro e
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
  deedDelta (LosesCounters _ _ _) = []
  deedDelta (RemoveFromCombat _) = []
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
  deedDelta (Continuously se _) = []
  deedDelta (Create agent count spec riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (specHeadTy spec) (Just Battlefield) Nothing (Just TokenOrigin))]
  deedDelta (GetsEmblem _ _) = []
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters amt kind from) = []
  deedDelta (MoveCounters amt kind src dst) = []
  deedDelta (PutCountersOfThoseKinds amt on) = []
  deedDelta (Composite v (Move what to _)) = []
  deedDelta (Composite _ e) = deedDelta e
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
  HeaderNontarget : {bs : Bindings} -> GameEvent bs -> Type
  HeaderNontarget {bs} ev = So (not (anyTargetedAt (eventIntro ev)))

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

||| Which card class may print a keyword on itself. This is not
||| `keywordStackRegime`'s question re-asked: that table says from which
||| zone a keyword's ability functions, this one says whether the word
||| can sit on the card at all, and the two would disagree even if every
||| cell agreed today. Flash is the case that used to look like a
||| disagreement: [CR#702.8a] says only that flash functions in any zone
||| the card could be played from, and restricts the word to no card
||| type, so an instant or sorcery may carry it. On an instant the word
||| grants what the card already has — redundant, which is not the same
||| as meaningless — and a sorcery carrying it is not redundant at all.
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
staticOnSpellCardOk (Conditionally _ se _) = staticOnSpellCardOk se
staticOnSpellCardOk (OnlyWhile se _ _) = staticOnSpellCardOk se
staticOnSpellCardOk _ = False

public export
cardAbilityOk : {0 bs : Bindings} -> CardClass -> AbilityAt bs -> Bool
cardAbilityOk PermanentCard (KeywordAbility k _) = keywordCardOk PermanentCard k
cardAbilityOk PermanentCard (Activated _ _ _ _ _) = True
cardAbilityOk PermanentCard (Triggered _ _ _ _ _ _ _) = True
cardAbilityOk PermanentCard (Static _) = True
cardAbilityOk PermanentCard (AlsoForKeywords ab _) = cardAbilityOk PermanentCard ab
cardAbilityOk PermanentCard (Spell _) = False
cardAbilityOk PermanentCard (AbilityWord _ ab) = cardAbilityOk PermanentCard ab
cardAbilityOk SpellCard (KeywordAbility k _) = keywordCardOk SpellCard k
cardAbilityOk SpellCard (Activated c _ _ _ _) = costOffBattlefield c
cardAbilityOk SpellCard (Triggered _ _ _ _ _ _ _) = True
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
chapterLineOk subs (Triggered _ (ChapterMark _) _ _ _ _ _) = elem Saga subs
chapterLineOk subs (AbilityWord _ ab) = chapterLineOk subs ab
chapterLineOk subs (AlsoForKeywords ab _) = chapterLineOk subs ab
chapterLineOk _ _ = True

public export
chapterFrameOk : {0 bs : Bindings} -> List Subtype -> AbilitySeq bs -> Bool
chapterFrameOk subs [] = True
chapterFrameOk subs (a :: as) = chapterLineOk subs a && chapterFrameOk subs as

public export
data PrintedStat = PrintedNum Integer | PrintedStar | PrintedStarPlus Nat
                 | PrintedMinusStar Nat

public export
starred : PrintedStat -> Bool
starred (PrintedNum _) = False
starred PrintedStar = True
starred (PrintedStarPlus _) = True
starred (PrintedMinusStar _) = True

||| The one box in a face's lower right corner. [CR#208.1] prints a creature
||| card's power and toughness there, [CR#209.1] a planeswalker card's
||| starting loyalty, and [CR#210.1] a battle card's defense. [CR#200.1]
||| lists the three as separate parts of a card and all three rules name the
||| same corner, so a face prints at most one of them and its type line
||| decides which.
public export
data PrintedBox : Type where
  PtBox : (pow : PrintedStat) -> (tou : PrintedStat) -> PrintedBox
  LoyaltyBox : (start : PrintedStat) -> PrintedBox
  DefenseBox : (def : PrintedStat) -> PrintedBox

||| The power/toughness pair a corner box writes, if that is what it writes.
||| Only [CR#208.1]'s box has slots a characteristic-defining line can star.
public export
boxPt : Maybe PrintedBox -> Maybe (PrintedStat, PrintedStat)
boxPt (Just (PtBox p t)) = Just (p, t)
boxPt _ = Nothing

public export
staticDefinesPt : {0 bs : Bindings} -> StaticEffect bs -> Maybe DefinedSlots
staticDefinesPt (DefinesPt _ sl _) = Just sl
staticDefinesPt (Conditionally _ se _) = staticDefinesPt se
staticDefinesPt (OnlyWhile se _ _) = staticDefinesPt se
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

||| Which corner box a card type demands. [CR#208.1] has a creature card
||| write its two numbers, [CR#209.1] a planeswalker card its loyalty number,
||| and [CR#210.1] a battle card its defense number; a type that writes no
||| number in that corner leaves the box to the rest of the line. A line that
||| names two of the three demands two numbers in one corner and so has no
||| box that fits it.
public export
boxSuitsType : CardType -> Maybe PrintedBox -> Bool
boxSuitsType Creature (Just (PtBox _ _)) = True
boxSuitsType Creature _ = False
boxSuitsType Planeswalker (Just (LoyaltyBox _)) = True
boxSuitsType Planeswalker _ = False
boxSuitsType Battle (Just (DefenseBox _)) = True
boxSuitsType Battle _ = False
boxSuitsType _ _ = True

public export
boxFitsLine : List CardType -> Maybe PrintedBox -> Bool
boxFitsLine [] box = True
boxFitsLine (t :: ts) box = boxSuitsType t box && boxFitsLine ts box

public export
cardBoxOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs ->
            Maybe PrintedBox -> Bool
cardBoxOk tys text box =
  boxFitsLine tys box &&
  definedSlotsStarred (boxPt box) (textDefines definesPower text)
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
data CardBox : TypeLine -> AbilitySeq [] -> Maybe PrintedBox -> Type where
  MkCardBox : {0 box : Maybe PrintedBox} ->
              {auto 0 ok : So (cardBoxOk l.tys as box)} -> CardBox l as box

||| Which corner box a card type demands at a face that is not a card of its
||| own. The creature demand stands: [CR#710.1b] lists power and toughness
||| among what a flip card's alternative half prints, and a nonmodal creature
||| back face prints them too. The loyalty demand does not: [CR#209.1] puts the
||| number on each planeswalker *card*, and [CR#712.8a] reads a double-faced
||| card's characteristics off its front face in every zone but the battlefield
||| and the stack, so a planeswalker back face has a loyalty to read without
||| printing one. Both spellings are printed, so the box stays optional there.
public export
altBoxSuitsType : CardType -> Maybe PrintedBox -> Bool
altBoxSuitsType Creature (Just (PtBox _ _)) = True
altBoxSuitsType Creature _ = False
altBoxSuitsType Planeswalker (Just (LoyaltyBox _)) = True
altBoxSuitsType Planeswalker Nothing = True
altBoxSuitsType Planeswalker _ = False
altBoxSuitsType Battle (Just (DefenseBox _)) = True
altBoxSuitsType Battle _ = False
altBoxSuitsType _ _ = True

public export
altBoxFitsLine : List CardType -> Maybe PrintedBox -> Bool
altBoxFitsLine [] box = True
altBoxFitsLine (t :: ts) box = altBoxSuitsType t box && altBoxFitsLine ts box

public export
altBoxOk : {0 bs : Bindings} -> List CardType -> AbilitySeq bs ->
           Maybe PrintedBox -> Bool
altBoxOk tys text box =
  altBoxFitsLine tys box &&
  definedSlotsStarred (boxPt box) (textDefines definesPower text)
                                  (textDefines definesToughness text)

public export
data AltCardBox : TypeLine -> AbilitySeq [] -> Maybe PrintedBox -> Type where
  MkAltCardBox : {0 box : Maybe PrintedBox} ->
                 {auto 0 ok : So (altBoxOk l.tys as box)} -> AltCardBox l as box

public export
CardCost : TypeLine -> Maybe ManaCost -> Type
CardCost l c = So (cardCostOk l.tys c)

||| A printed face whose mana cost, where it prints one, is its own rather
||| than another face's. Four layouts print such a face: the single face of a
||| one-faced card, either face of a modal double-faced card [CR#712.3],
||| either half of a split card [CR#709.4b], and both parts of an adventurer
||| card's frame [CR#715.2]. The cost stays a `Maybe` because a land face
||| writes none; what separates these faces from `AltFace` is having a cost
||| slot at all.
|||
||| Its text is its own `AbilitySeq []`, so no face's words read a binding
||| another face introduced. That is not a convenience: a face's
||| characteristics exist only while that face is the one in play
||| [CR#712.8f,709.3b,715.3b], so there is no moment at which one face's
||| clause could resolve against the other's antecedent.
public export
record CardFace where
  constructor MkFace
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq []
  box : Maybe PrintedBox

||| A printed face that writes no mana cost at all. Two layouts print one:
||| the back face of a nonmodal double-faced card, whose mana value is read
||| off the front face precisely because the back has no cost of its own
||| [CR#202.3a,202.3b], and a flip card's upside-down half, which shares the
||| card's single printed cost [CR#710.1c]. [CR#710.1b] lists what such a half
||| does print — a name, a text box, a type line, and its power and toughness
||| — and a mana cost is not among them, so this record has no field for one
||| rather than a law refusing one.
public export
record AltFace where
  constructor MkAltFace
  name : String
  supers : List Supertype
  line : TypeLine
  text : AbilitySeq []
  box : Maybe PrintedBox

||| Every card-level law, re-stated at one full printed face. [CR#712.8] gives
||| each face of a double-faced card its own set of characteristics, [CR#709.4c]
||| reads each split half's types and text on its own, and [CR#715.2] does the
||| same for an adventurer card's two frames: the line, supertype, text,
||| chapter, corner-box and cost laws are all face laws, and not one of them is
||| a whole-card law that a second face could escape.
public export
data FaceLaws : CardFace -> Type where
  MkFaceLaws : {0 f : CardFace} ->
               {auto 0 ln : CardLine f.line} ->
               {auto 0 sp : CardSupers f.supers} ->
               {auto 0 tx : CardText f.line f.text} ->
               {auto 0 ch : CardChapters f.line f.text} ->
               {auto 0 bx : CardBox f.line f.text f.box} ->
               {auto 0 mc : CardCost f.line f.cost} ->
               FaceLaws f

||| The same laws at a costless face, with two of them restated for it.
||| `CardCost` is the one card-level law with nothing left to say: [CR#202.3a]
||| and [CR#710.1c] leave the face without a mana cost, so the land-cost gate
||| has no cost to read. The corner-box law is `AltCardBox`, not `CardBox`,
||| because a face is not a card and [CR#209.1] speaks of cards.
public export
data AltFaceLaws : AltFace -> Type where
  MkAltFaceLaws : {0 f : AltFace} ->
                  {auto 0 ln : CardLine f.line} ->
                  {auto 0 sp : CardSupers f.supers} ->
                  {auto 0 tx : CardText f.line f.text} ->
                  {auto 0 ch : CardChapters f.line f.text} ->
                  {auto 0 bx : AltCardBox f.line f.text f.box} ->
                  AltFaceLaws f

||| An adventurer card's inset frame [CR#715.1]. A player chooses to play the
||| card "as an Adventure" [CR#715.3], and Adventure is a spell type
||| [CR#205.3k], so the inset names that spell type. That its line is then an
||| instant or a sorcery is not restated here: `CardLine`'s `subsFitLine`
||| already fits a spell type to no other card type, and a second conjunct
||| saying so would be unreachable.
public export
adventureInsetOk : TypeLine -> Bool
adventureInsetOk l = elem Adventure l.subs

public export
AdventureInset : TypeLine -> Type
AdventureInset l = So (adventureInsetOk l)

||| Either half of a flip card [CR#710.1]. [CR#710.2] applies the alternative
||| characteristics only once the permanent is flipped, and only on the
||| battlefield, so each half names a permanent type — the six [CR#110.4]
||| lists, which is what `anyPermanentType` reads. Excluding spell types is
||| not restated here: `CardLine`'s `typesCombinable` already refuses a line
||| mixing the two.
public export
flipHalfOk : TypeLine -> Bool
flipHalfOk l = anyPermanentType l.tys

public export
FlipHalf : TypeLine -> Type
FlipHalf l = So (flipHalfOk l)

||| One card: its faces, and the rule its layout answers.
|||
||| Five layouts, five constructors — not one record with a layout tag. The
||| layouts disagree about which boxes a face prints, and a uniform face record
||| would state that disagreement away: it would let a flip card's upside-down
||| half [CR#710.1c] or a nonmodal back face [CR#202.3a] carry a mana cost
||| neither prints, and would give a costed face to a layout that has none to
||| give. What the layouts do share — a full face with a cost of its own — is
||| `CardFace`; a costless half is `AltFace`; and every card-level law is
||| re-stated at each face by `FaceLaws` and `AltFaceLaws`, so no law silently
||| applies to one face of two.
|||
||| Meld [CR#712.4] is not among them. A meld pair's combined back face belongs
||| to two cards at once [CR#712.4b], so it is not a second face of one card
||| and does not fit this shape.
public export
data Card : Type where
  ||| A card with a single face; the other side is the normal Magic card back.
  SingleFaced : (face : CardFace) ->
                {auto 0 fl : FaceLaws face} -> Card

  ||| A nonmodal double-faced card [CR#712.2]: abilities on one or both faces
  ||| turn it over. [CR#712.8] gives each face its own characteristics, and
  ||| [CR#202.3a,202.3b] leave the back face costless, reading its mana value
  ||| off the front — so the back is an `AltFace`.
  Transforming : (front : CardFace) -> (back : AltFace) ->
                 {auto 0 ff : FaceLaws front} ->
                 {auto 0 bf : AltFaceLaws back} -> Card

  ||| A modal double-faced card [CR#712.3]: two Magic card faces whose
  ||| characteristics are usually independent of one another. Each is a full
  ||| face — [CR#712.11b] has the caster choose which of them they are casting
  ||| and [CR#712.12] which land face enters — so each face's cost is its own,
  ||| never read off the other the way a nonmodal back face's is [CR#202.3b].
  ||| Two land faces write no cost at all.
  ModalDfc : (front : CardFace) -> (back : CardFace) ->
             {auto 0 ff : FaceLaws front} ->
             {auto 0 bf : FaceLaws back} -> Card

  ||| A split card [CR#709.1]: two faces on one card whose other side is the
  ||| normal card back. [CR#709.4b] gives each half its own mana cost and
  ||| [CR#709.4c] its own card types and text box.
  SplitCard : (left : CardFace) -> (right : CardFace) ->
              {auto 0 lf : FaceLaws left} ->
              {auto 0 rf : FaceLaws right} -> Card

  ||| An adventurer card [CR#715.1]: the normal face, printed as usual, and the
  ||| inset frame whose alternative characteristics the object has while it is
  ||| a spell [CR#715.2].
  Adventurer : (normal : CardFace) -> (inset : CardFace) ->
               {auto 0 nf : FaceLaws normal} ->
               {auto 0 sf : FaceLaws inset} ->
               {auto 0 ai : AdventureInset inset.line} -> Card

  ||| A flip card [CR#710.1]: the right-side-up half writes the card's normal
  ||| characteristics and the upside-down half the alternative ones. [CR#710.1c]
  ||| leaves the one printed mana cost with the card however it is turned, so
  ||| the alternative half is an `AltFace`. The transition itself is the landed
  ||| `Flipped` status, one-way by [CR#710.4]; this constructor adds no second
  ||| verb for it.
  FlipCard : (normal : CardFace) -> (alternative : AltFace) ->
             {auto 0 nf : FaceLaws normal} ->
             {auto 0 af : AltFaceLaws alternative} ->
             {auto 0 nh : FlipHalf normal.line} ->
             {auto 0 ah : FlipHalf alternative.line} -> Card

