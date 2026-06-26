||| A RON (Rusty Object Notation) serializer for the grammar-probe model. It does
||| NOT reproduce the Rust engine's exact wire format — the Idris grammar has its
||| own constructors — so each value renders as the SHAPE of its Idris constructor:
||| a nullary constructor is a bare RON unit (`Flying`), a constructor with fields
||| is a tuple/struct variant (`DealDamage(This, GetTargets(0), 3)`), a record is a
||| struct, a `List` is a `[ … ]`, and a `Maybe` is RON's `None` / `Some(x)`.
module Ron

import public Core
import public Macros
import public Cards

import Data.List
import Data.String

import Derive.Prelude
%language ElabReflection


interface ToRon a where
  toRon : (indent : Nat) -> a -> String

mkIndent : Nat -> String
mkIndent n = fastConcat $ replicate n "  "

commaSep : List String -> String
commaSep xs = fastConcat (intersperse ", " xs)

-- a tuple/struct variant: `Name` when nullary, `Name(a, b, …)` otherwise. Pass
-- plain element strings for a tuple variant, or `"field: v"` strings for a struct one.
ctor : String -> List String -> String
ctor name [] = name
ctor name args = name ++ "(" ++ commaSep args ++ ")"

implementation ToRon String where
    toRon _ x = "\"" ++ x ++ "\""

implementation ToRon Nat where
    toRon _ n = show n

implementation ToRon Bool where
    toRon _ True = "true"
    toRon _ False = "false"

ronProperty : (ToRon a) => (indent : Nat) -> String -> Maybe a -> String
ronProperty indent name Nothing = ""
ronProperty indent name (Just x) = mkIndent indent
                                ++ name ++ ": "
                                ++ toRon indent x ++ ",\n"

ronListItem : (ToRon a) => (indent : Nat) -> a -> String
ronListItem indent x = mkIndent indent ++ toRon indent x ++ ",\n"

-- RON's `Option`: `None` / `Some(x)`. (A struct field that is `Nothing` is omitted
-- by `ronProperty`; this is for a `Maybe` that sits inside a constructor's arguments.)
ronOpt : (ToRon a) => (indent : Nat) -> Maybe a -> String
ronOpt _ Nothing = "None"
ronOpt indent (Just x) = "Some(" ++ toRon indent x ++ ")"

nonEmpty : List a -> Maybe (List a)
nonEmpty [] = Nothing
nonEmpty xs = Just xs


-- Leaf value types ----------------------------------------------------------

%runElab derive "Color" [Show]
implementation ToRon Color where
    toRon _ c = show c

implementation ToRon (Maybe Color) where
    toRon indent Nothing = "Colorless"
    toRon indent (Just c) = toRon indent c

implementation ToRon SimpleManaSymbol where
    toRon _ (Generic k) = ctor "Generic" [show k]
    toRon indent (Specific c) = toRon indent c

implementation ToRon ManaSymbol where
    toRon indent (Simple s) = toRon indent s
    toRon indent (Hybrid x y) = ctor "Hybrid" [toRon indent x, toRon indent y]
    toRon indent Variable = "Variable"
    toRon indent (Phyrexian c mc) = ctor "Phyrexian" [toRon indent c, toRon indent mc]
    toRon indent SnowMana = "Snow"

implementation ToRon a => ToRon (List a) where
    toRon indent [] = "[]"
    toRon indent (x::[]) = "[" ++ toRon indent x ++ "]"
    toRon indent xs = "[\n"
        ++ fastConcat (map (ronListItem (S indent)) xs)
        ++ mkIndent indent ++ "]"

implementation ToRon Type_ where
    toRon indent Artifact = "Artifact"
    toRon indent Battle = "Battle"
    toRon indent Creature = "Creature"
    toRon indent Enchantment = "Enchantment"
    toRon indent Instant = "Instant"
    toRon indent Kindred = "Kindred"
    toRon indent Land = "Land"
    toRon indent Planeswalker = "Planeswalker"
    toRon indent Sorcery = "Sorcery"

%runElab derive "Supertype" [Show]
implementation ToRon Supertype where
    toRon _ st = show st

%runElab derive "CreatureSubtype" [Show]
implementation ToRon CreatureSubtype where
    toRon _ ct = show ct
%runElab derive "EnchantmentSubtype" [Show]
implementation ToRon EnchantmentSubtype where
    toRon _ et = show et
%runElab derive "ArtifactSubtype" [Show]
implementation ToRon ArtifactSubtype where
    toRon _ at = show at
%runElab derive "LandSubtype" [Show]
implementation ToRon LandSubtype where
    toRon _ lt = show lt
%runElab derive "BattleSubtype" [Show]
implementation ToRon BattleSubtype where
    toRon _ bt = show bt

implementation ToRon Subtype where
    toRon indent (CreatureSub x) = toRon indent x
    toRon indent (EnchantmentSub x) = toRon indent x
    toRon indent (ArtifactSub x) = toRon indent x
    toRon indent (LandSub x) = toRon indent x
    toRon indent (BattleSub x) = toRon indent x

-- The remaining leaf enums: nullary constructors, so a derived `Show` already
-- prints the RON unit-variant name. (Types with payloads are rendered by hand below.)
%runElab derive "Stat" [Show]
implementation ToRon Stat where toRon _ x = show x
%runElab derive "RoundMode" [Show]
implementation ToRon RoundMode where toRon _ x = show x
implementation ToRon AggregateOp where
    toRon _ SumOf = "SumOf"
    toRon _ MinOf = "MinOf"
    toRon _ MaxOf = "MaxOf"
    toRon i (AverageOf rm) = ctor "AverageOf" [toRon i rm]
implementation ToRon Attribute where
    toRon _ OfColor = "OfColor"
    toRon _ OfName = "OfName"
    toRon _ OfBasicLandType = "OfBasicLandType"
    toRon _ OfCardType = "OfCardType"
    toRon i (OfStat s) = ctor "OfStat" [toRon i s]
implementation ToRon SymbolPred where
    toRon i (CountsAs c) = ctor "CountsAs" [toRon i c]
    toRon _ IsGeneric = "IsGeneric"
    toRon i (And ps) = ctor "And" [toRon i ps]
    toRon i (Or ps) = ctor "Or" [toRon i ps]
    toRon i (Not p) = ctor "Not" [toRon i p]
%runElab derive "Cmp" [Show]
implementation ToRon Cmp where toRon _ x = show x
%runElab derive "ObjectKind" [Show]
implementation ToRon ObjectKind where toRon _ x = show x
%runElab derive "Zone" [Show]
implementation ToRon Zone where toRon _ x = show x
%runElab derive "CounterKind" [Show]
implementation ToRon CounterKind where toRon _ x = show x
%runElab derive "Designation" [Show]
implementation ToRon Designation where toRon _ x = show x
%runElab derive "Role" [Show]
implementation ToRon Role where toRon _ x = show x
%runElab derive "ObjectState" [Show]
implementation ToRon ObjectState where toRon _ x = show x
%runElab derive "TextWordClass" [Show]
implementation ToRon TextWordClass where toRon _ x = show x
%runElab derive "Timing" [Show]
implementation ToRon Timing where toRon _ x = show x
%runElab derive "UsageLimit" [Show]
implementation ToRon UsageLimit where toRon _ x = show x
implementation ToRon Arrangement where
    toRon _ ChosenOrder = "ChosenOrder"
    toRon _ RandomOrder = "RandomOrder"
    toRon _ SameOrder = "SameOrder"
implementation ToRon TollTiming where
    toRon _ AtDeclaration = "AtDeclaration"
    toRon _ Downstream = "Downstream"
implementation ToRon RefKind where
    toRon _ Empty = "Empty"
    toRon _ AnObject = "AnObject"
    toRon _ APlayer = "APlayer"
    toRon _ Anything = "Anything"
%runElab derive "BeginningStep" [Show]
implementation ToRon BeginningStep where toRon _ x = show x
%runElab derive "CombatStep" [Show]
implementation ToRon CombatStep where toRon _ x = show x
%runElab derive "EndingStep" [Show]
implementation ToRon EndingStep where toRon _ x = show x
%runElab derive "MainPhaseKind" [Show]
implementation ToRon MainPhaseKind where toRon _ x = show x
%runElab derive "Window" [Show]
implementation ToRon Window where toRon _ x = show x
%runElab derive "Compulsion" [Show]
implementation ToRon Compulsion where toRon _ x = show x
%runElab derive "FaceLayout" [Show]
implementation ToRon FaceLayout where toRon _ x = show x
%runElab derive "OutcomeGateKind" [Show]
implementation ToRon OutcomeGateKind where toRon _ x = show x

-- `Relation` is namespaced; spell its (nullary) constructors out by hand.
implementation ToRon Relation where
    toRon _ Attack = "Attack"
    toRon _ Block = "Block"
    toRon _ Cast = "Cast"
    toRon _ Activate = "Activate"
    toRon _ Play = "Play"
    toRon _ Attach = "Attach"
    toRon _ Target = "Target"
    toRon _ Counter = "Counter"

implementation ToRon PhaseStep where
    toRon i (BeginningPhase s) = ctor "BeginningPhase" [toRon i s]
    toRon i (MainPhase k) = ctor "MainPhase" [toRon i k]
    toRon i (CombatPhase s) = ctor "CombatPhase" [toRon i s]
    toRon i (EndingPhase s) = ctor "EndingPhase" [toRon i s]

implementation ToRon ChooseDomain where
    toRon _ AColor = "AColor"
    toRon _ ACreatureType = "ACreatureType"
    toRon i (AMode n) = ctor "AMode" [show n]
    toRon _ AName = "AName"
    toRon _ ANumber = "ANumber"

implementation ToRon ProducedMana where
    toRon i (OfColor c) = ctor "OfColor" [toRon i c]
    toRon i (OneOf cs) = ctor "OneOf" [toRon i cs]
    toRon _ AnyColor = "AnyColor"

implementation ToRon EventKind where
    toRon _ Sacrifice = "Sacrifice"
    toRon _ Draw = "Draw"
    toRon _ Discard = "Discard"
    toRon i (DealDamage mb) = ctor "DealDamage" [ronOpt i mb]
    toRon _ CreateToken = "CreateToken"
    toRon _ PutCounters = "PutCounters"
    toRon _ Destroy = "Destroy"
    toRon i (ZoneChanged from to) = ctor "ZoneChanged" [ronOpt i from, ronOpt i to]
    toRon i (BeginStep ps) = ctor "BeginStep" [toRon i ps]
    toRon i (Becomes s) = ctor "Becomes" [toRon i s]
    toRon i (Begins r) = ctor "Begins" [toRon i r]
    toRon _ RemoveCounters = "RemoveCounters"


-- The grammar proper. Count / Reference / Predicate / Condition / … and the
-- effect / ability trees are one mutually recursive language, so their `ToRon`
-- instances live in a single `mutual` block (each may call any other).
mutual
  -- heterogeneous lists (`All`) render like a homogeneous `List` would, but the
  -- element kinds differ per slot, so they need their own little folds.
  ronTargets : {0 b : Bindings} -> {0 ks : List RefKind} -> (indent : Nat) -> All (TargetSpec b) ks -> List String
  ronTargets _ [] = []
  ronTargets i (x :: xs) = toRon i x :: ronTargets i xs

  ronPreds : {0 b : Bindings} -> {0 ks : List RefKind} -> (indent : Nat) -> All (Predicate b) ks -> List String
  ronPreds _ [] = []
  ronPreds i (p :: ps) = toRon i p :: ronPreds i ps

  ronBracket : List String -> String
  ronBracket xs = "[" ++ commaSep xs ++ "]"

  implementation ToRon (Reference b k) where
    toRon _ This = "This"
    toRon i (Single s) = ctor "Single" [toRon i s]
    toRon i (AttachHostOf r) = ctor "AttachHostOf" [toRon i r]
    toRon i (AttachedTo r) = ctor "AttachedTo" [toRon i r]
    toRon _ It = "It"
    toRon _ EventObject = "EventObject"
    toRon _ You = "You"
    toRon i (ControllerOf r) = ctor "ControllerOf" [toRon i r]
    toRon i (OwnerOf r) = ctor "OwnerOf" [toRon i r]
    toRon _ EventActor = "EventActor"
    toRon _ ChosenPlayer = "ChosenPlayer"
    toRon _ ChosenObject = "ChosenObject"

  implementation ToRon (Count b) where
    toRon _ (Literal k) = show k
    toRon _ X = "X"
    toRon i (CountOf c) = ctor "CountOf" [toRon i c]
    toRon i (CountDistinct a c) = ctor "CountDistinct" [toRon i a, toRon i c]
    toRon i (StatOf r s) = ctor "StatOf" [toRon i r, toRon i s]
    toRon i (Aggregate op cs) = ctor "Aggregate" [toRon i op, toRon i cs]
    toRon i (EventAgg op q) = ctor "EventAgg" [toRon i op, toRon i q]
    toRon i (Damage r) = ctor "Damage" [toRon i r]
    toRon i (CountersOn c r) = ctor "CountersOn" [toRon i c, toRon i r]
    toRon i (LifeTotal r) = ctor "LifeTotal" [toRon i r]
    toRon i (HandSize r) = ctor "HandSize" [toRon i r]
    toRon i (Plus x y) = ctor "Plus" [toRon i x, toRon i y]
    toRon i (Minus x y) = ctor "Minus" [toRon i x, toRon i y]
    toRon i (Times x y) = ctor "Times" [toRon i x, toRon i y]
    toRon i (Half rm x) = ctor "Half" [toRon i rm, toRon i x]
    toRon i (Min x y) = ctor "Min" [toRon i x, toRon i y]
    toRon i (Max x y) = ctor "Max" [toRon i x, toRon i y]
    toRon _ ThatMuch = "ThatMuch"
    toRon _ Allotment = "Allotment"
    toRon _ ChosenNumber = "ChosenNumber"

  implementation ToRon (Countable b) where
    toRon i (Objects p) = ctor "Objects" [toRon i p]
    toRon i (Players p) = ctor "Players" [toRon i p]
    toRon i (Events q) = ctor "Events" [toRon i q]
    toRon i (ManaSymbols r sp) = ctor "ManaSymbols" [toRon i r, toRon i sp]
    toRon i (ManaSpent {forObj}) = ctor "ManaSpent" [toRon i forObj]

  implementation ToRon (Projection b) where
    toRon i (Project src acc) = ctor "Project" [toRon i src, toRon i acc]

  implementation ToRon (Quantity b) where
    toRon i (Range lo hi) = ctor "Range" [ronOpt i lo, ronOpt i hi]

  implementation ToRon (Selection b k) where
    toRon i (SelectAll p) = ctor "SelectAll" [toRon i p]
    toRon i (Union sels) = ctor "Union" [toRon i sels]
    toRon _ That = "That"
    toRon _ (GetTargets n) = ctor "GetTargets" [show n]
    toRon i (Random q p) = ctor "Random" [toRon i q, toRon i p]
    toRon i (TopOfLibrary c {whose}) = ctor "TopOfLibrary" [toRon i c, toRon i whose]
    toRon i (BottomOfLibrary c {whose}) = ctor "BottomOfLibrary" [toRon i c, toRon i whose]
    toRon i (Pick op pr) = ctor "Pick" [toRon i op, toRon i pr]

  implementation ToRon (Predicate b k) where
    toRon i (HasType t) = ctor "HasType" [toRon i t]
    toRon i (HasSupertype s) = ctor "HasSupertype" [toRon i s]
    toRon i (HasSubtype s) = ctor "HasSubtype" [toRon i s]
    toRon i (HasColor c) = ctor "HasColor" [toRon i c]
    toRon i (IsKind k) = ctor "IsKind" [toRon i k]
    toRon i (InZone z) = ctor "InZone" [toRon i z]
    toRon i (HasKeyword ks) = ctor "HasKeyword" [toRon i ks]
    toRon i (SameAs r) = ctor "SameAs" [toRon i r]
    toRon i (SameName r) = ctor "SameName" [toRon i r]
    toRon i (SharesSubtype r) = ctor "SharesSubtype" [toRon i r]
    toRon i (WasCastFrom z) = ctor "WasCastFrom" [toRon i z]
    toRon i (ExiledBy r) = ctor "ExiledBy" [toRon i r]
    toRon i (DamagedBy r) = ctor "DamagedBy" [toRon i r]
    toRon i (HasName n) = ctor "HasName" [toRon i n]
    toRon i (HasCounter c) = ctor "HasCounter" [toRon i c]
    toRon i (HasState s) = ctor "HasState" [toRon i s]
    toRon i (Holds rel role) = ctor "Holds" [toRon i rel, toRon i role]
    toRon i (HasDesignation d) = ctor "HasDesignation" [toRon i d]
    toRon i (StatCmp s c n) = ctor "StatCmp" [toRon i s, toRon i c, toRon i n]
    toRon i (ControlledBy p) = ctor "ControlledBy" [toRon i p]
    toRon i (OwnedBy p) = ctor "OwnedBy" [toRon i p]
    toRon i (Controls p) = ctor "Controls" [toRon i p]
    toRon _ Multicolored = "Multicolored"
    toRon _ IsColorless = "IsColorless"
    toRon i (Targets p) = ctor "Targets" [toRon i p]
    toRon i (TargetCount c n) = ctor "TargetCount" [toRon i c, toRon i n]
    toRon _ WasKicked = "WasKicked"
    toRon _ OfChosen = "OfChosen"
    toRon _ Anyone = "Anyone"
    toRon i (And ps) = ctor "And" [toRon i ps]
    toRon i (Or ps) = ctor "Or" [ronBracket (ronPreds i ps)]
    toRon i (Not p) = ctor "Not" [toRon i p]

  implementation ToRon (Condition b) where
    toRon i (Matches r p) = ctor "Matches" [toRon i r, toRon i p]
    toRon i (Compare x c y) = ctor "Compare" [toRon i x, toRon i c, toRon i y]
    toRon i (TurnOf p) = ctor "TurnOf" [toRon i p]
    toRon i (During ps) = ctor "During" [toRon i ps]
    toRon i (LegallyAttached r) = ctor "LegallyAttached" [toRon i r]
    toRon _ (ChosenIs n) = ctor "ChosenIs" [show n]
    toRon i (And cs) = ctor "And" [toRon i cs]
    toRon i (Or cs) = ctor "Or" [toRon i cs]
    toRon i (Not c) = ctor "Not" [toRon i c]

  implementation ToRon (Facet b) where
    toRon i (Actor p) = ctor "Actor" [toRon i p]
    toRon i (Agent p) = ctor "Agent" [toRon i p]
    toRon i (Patient p) = ctor "Patient" [toRon i p]
    toRon i (Within w) = ctor "Within" [toRon i w]
    toRon i (DuringStep ps) = ctor "DuringStep" [toRon i ps]
    toRon i (DuringTurn p) = ctor "DuringTurn" [toRon i p]
    toRon i (IsFirst w) = ctor "IsFirst" [toRon i w]
    toRon i (And fs) = ctor "And" [toRon i fs]
    toRon i (Or fs) = ctor "Or" [toRon i fs]
    toRon i (Not f) = ctor "Not" [toRon i f]

  implementation ToRon (EventQuery b) where
    toRon i (MkQuery kinds facets) =
      "(kinds: " ++ toRon i kinds ++ ", facets: " ++ toRon i facets ++ ")"

  implementation ToRon (Delta b) where
    toRon i (Up c) = ctor "Up" [toRon i c]
    toRon i (Down c) = ctor "Down" [toRon i c]

  implementation ToRon (Outcome b) where
    toRon i (WinGame r) = ctor "WinGame" [toRon i r]
    toRon i (LoseGame r) = ctor "LoseGame" [toRon i r]

  implementation ToRon (Anchor b) where
    toRon i (FromTop c) = ctor "FromTop" [toRon i c]
    toRon i (FromBottom c) = ctor "FromBottom" [toRon i c]

  implementation ToRon (Duration b) where
    toRon _ UntilEndOfTurn = "UntilEndOfTurn"
    toRon i (UntilEvent q) = ctor "UntilEvent" [toRon i q]
    toRon i (ForAsLongAs c) = ctor "ForAsLongAs" [toRon i c]
    toRon _ Permanent = "Permanent"

  implementation ToRon (Cost b) where
    toRon i (Mana m) = ctor "Mana" [toRon i m]
    toRon i (Do a) = ctor "Do" [toRon i a]
    toRon i (Scaled c cost) = ctor "Scaled" [toRon i c, toRon i cost]
    toRon i (Costs cs) = ctor "Costs" [toRon i cs]
    toRon i (TapTotal s c n p) = ctor "TapTotal" [toRon i s, toRon i c, toRon i n, toRon i p]

  implementation ToRon (CostChange b) where
    toRon i (Reduce cs) = ctor "Reduce" [toRon i cs]
    toRon i (Increase cs) = ctor "Increase" [toRon i cs]
    toRon i (Additional cs opt) = ctor "Additional" [toRon i cs, toRon i opt]
    toRon i (ScaledBy ch c) = ctor "ScaledBy" [toRon i ch, toRon i c]

  implementation ToRon (AlternativeCost b) where
    toRon i (AltCost cs) = ctor "AltCost" [toRon i cs]

  implementation ToRon (ReplaceLimit b) where
    toRon _ Unlimited = "Unlimited"
    toRon i (UpTo c) = ctor "UpTo" [toRon i c]

  implementation ToRon (ChooseSpec b) where
    toRon i (MkChooseSpec count {repeats}) =
      "(count: " ++ toRon i count ++ ", repeats: " ++ toRon i repeats ++ ")"

  implementation ToRon (Deed b) where
    toRon i (Enact r agent patient) = ctor "Enact" [toRon i r, toRon i agent, toRon i patient]
    toRon i (BlockedBy attacker size) = ctor "BlockedBy" [toRon i attacker, toRon i size]

  implementation ToRon (TargetSpec b k) where
    toRon i (Target q p) = ctor "Target" [toRon i q, toRon i p]

  implementation ToRon (Bindable b k) where
    toRon i (Existing s) = ctor "Existing" [toRon i s]
    toRon i (Produce a) = ctor "Produce" [toRon i a]
    toRon i (Choose q p {by}) = ctor "Choose" [toRon i by, toRon i q, toRon i p]
    toRon i (Search q p {by} {whose} {from}) =
      ctor "Search" [toRon i by, toRon i whose, toRon i from, toRon i q, toRon i p]

  implementation ToRon (Destination b) where
    toRon i (ToZone z) = ctor "ToZone" [toRon i z]
    toRon i (ToLibrary a) = ctor "ToLibrary" [toRon i a]

  implementation ToRon (CounterSpec b) where
    toRon i (Some c n) = ctor "Some" [toRon i c, toRon i n]
    toRon _ AllKinds = "AllKinds"

  implementation ToRon (ManaRider b) where
    toRon i (SpendOnly p) = ctor "SpendOnly" [toRon i p]
    toRon i (GrantOnSpend se) = ctor "GrantOnSpend" [toRon i se]
    toRon i (TriggerOnSpend e) = ctor "TriggerOnSpend" [toRon i e]

  implementation ToRon (Action b) where
    toRon i (DealDamage {source} r n) = ctor "DealDamage" [toRon i source, toRon i r, toRon i n]
    toRon i (Move r z) = ctor "Move" [toRon i r, toRon i z]
    toRon i (ExileUntil r d) = ctor "ExileUntil" [toRon i r, toRon i d]
    toRon i (Destroy r) = ctor "Destroy" [toRon i r]
    toRon i (Counter r) = ctor "Counter" [toRon i r]
    toRon i (Tap r) = ctor "Tap" [toRon i r]
    toRon i (Untap r) = ctor "Untap" [toRon i r]
    toRon i (RemoveAllDamage r) = ctor "RemoveAllDamage" [toRon i r]
    toRon i (RemoveFromCombat r) = ctor "RemoveFromCombat" [toRon i r]
    toRon i (Transform r) = ctor "Transform" [toRon i r]
    toRon i (GrantDesignation d r) = ctor "GrantDesignation" [toRon i d, toRon i r]
    toRon i (Attach what to) = ctor "Attach" [toRon i what, toRon i to]
    toRon i (Unattach r) = ctor "Unattach" [toRon i r]
    toRon i (Draw {actor} n) = ctor "Draw" [toRon i actor, toRon i n]
    toRon i (GainLife {actor} n) = ctor "GainLife" [toRon i actor, toRon i n]
    toRon i (PutCounters c n r) = ctor "PutCounters" [toRon i c, toRon i n, toRon i r]
    toRon i (Discard {actor} n) = ctor "Discard" [toRon i actor, toRon i n]
    toRon i (LoseLife {actor} n) = ctor "LoseLife" [toRon i actor, toRon i n]
    toRon i (Sacrifice r p) = ctor "Sacrifice" [toRon i r, toRon i p]
    toRon i (PhaseOut r) = ctor "PhaseOut" [toRon i r]
    toRon i (MoveArranged sel arr dest) = ctor "MoveArranged" [toRon i sel, toRon i arr, toRon i dest]
    toRon i (RemoveCounters c n r) = ctor "RemoveCounters" [toRon i c, toRon i n, toRon i r]
    toRon i (MoveCounters cs from to) = ctor "MoveCounters" [toRon i cs, toRon i from, toRon i to]
    toRon i (Reveal r) = ctor "Reveal" [toRon i r]
    toRon i (Shuffle {actor}) = ctor "Shuffle" [toRon i actor]
    toRon i (ExtraTurn {actor}) = ctor "ExtraTurn" [toRon i actor]
    toRon i (ControlPlayer whom) = ctor "ControlPlayer" [toRon i whom]
    toRon i (CreateToken n c) = ctor "CreateToken" [toRon i n, toRon i c]
    toRon i (CopySpell r) = ctor "CopySpell" [toRon i r]
    toRon i (CreateTokenCopy r) = ctor "CreateTokenCopy" [toRon i r]
    toRon i (AddMana {actor} amount produced {riders}) =
      ctor "AddMana" [toRon i actor, toRon i amount, toRon i produced, toRon i riders]

  implementation ToRon (Mode b) where
    toRon i (MkMode effect {cost}) =
      "(effect: " ++ toRon i effect ++ ", cost: " ++ ronOpt i cost ++ ")"

  implementation ToRon (Modification b) where
    toRon i (ModifyPT x y) = ctor "ModifyPT" [toRon i x, toRon i y]
    toRon i (Set Colors v) = ctor "Set" ["Colors", toRon i v]
    toRon i (Set CardTypes v) = ctor "Set" ["CardTypes", toRon i v]
    toRon i (Set Subtypes v) = ctor "Set" ["Subtypes", toRon i v]
    toRon i (Set Supertypes v) = ctor "Set" ["Supertypes", toRon i v]
    toRon i (Set BasePower v) = ctor "Set" ["BasePower", toRon i v]
    toRon i (Set BaseToughness v) = ctor "Set" ["BaseToughness", toRon i v]
    toRon i (Set Name v) = ctor "Set" ["Name", ronOpt i v]
    toRon i (AddType t) = ctor "AddType" [toRon i t]
    toRon i (AddSubtype s) = ctor "AddSubtype" [toRon i s]
    toRon i (ChangeText ws) = ctor "ChangeText" [toRon i ws]
    toRon _ LoseAbilities = "LoseAbilities"
    toRon i (GainControl r) = ctor "GainControl" [toRon i r]
    toRon i (GrantAbility a) = ctor "GrantAbility" [toRon i a]
    toRon i (BecomeCopyOf r) = ctor "BecomeCopyOf" [toRon i r]

  implementation ToRon (StaticEffect b) where
    toRon i (Modify r mods) = ctor "Modify" [toRon i r, toRon i mods]
    toRon i (CostModifier p ch) = ctor "CostModifier" [toRon i p, toRon i ch]
    toRon i (Replaces q body {limit}) = ctor "Replaces" [toRon i q, toRon i body, toRon i limit]
    toRon i (CantHappen q) = ctor "CantHappen" [toRon i q]
    toRon i (OutcomeGate g p) = ctor "OutcomeGate" [toRon i g, toRon i p]
    toRon i (Also q body) = ctor "Also" [toRon i q, toRon i body]
    toRon i (Sba c body) = ctor "Sba" [toRon i c, toRon i body]
    toRon i (ManaPersists p) = ctor "ManaPersists" [toRon i p]
    toRon i (MayCastFor alt {from}) = ctor "MayCastFor" [toRon i alt, toRon i from]
    toRon i (CastFaceDown c) = ctor "CastFaceDown" [toRon i c]
    toRon i (While c se) = ctor "While" [toRon i c, toRon i se]
    toRon i (Can d {window}) = ctor "Can" [toRon i d, ronOpt i window]
    toRon i (AsThough c se) = ctor "AsThough" [toRon i c, toRon i se]
    toRon i (Constrain comp d) = ctor "Constrain" [toRon i comp, toRon i d]
    toRon i (ReplaceAmount q newAmount) = ctor "ReplaceAmount" [toRon i q, toRon i newAmount]
    toRon i (Priced t cost deed) = ctor "Priced" [toRon i t, toRon i cost, toRon i deed]

  implementation ToRon (OneShotEffect b) where
    toRon i (Sequence xs) = ctor "Sequence" [toRon i xs]
    toRon i (Targeted ts body) = ctor "Targeted" [ronBracket (ronTargets i ts), toRon i body]
    toRon i (With bnd body) = ctor "With" [toRon i bnd, toRon i body]
    toRon i (Act a) = ctor "Act" [toRon i a]
    toRon i (Conclude o) = ctor "Conclude" [toRon i o]
    toRon i (May effect {ifDid} {ifNot}) =
      ctor "May" [toRon i effect, ronOpt i ifDid, ronOpt i ifNot]
    toRon i (If c thenDo {otherwise}) =
      ctor "If" [toRon i c, toRon i thenDo, ronOpt i otherwise]
    toRon i (MayPay {actor} cost andThen {or_else}) =
      ctor "MayPay" [toRon i actor, toRon i cost, toRon i andThen, ronOpt i or_else]
    toRon i (MustPay {actor} cost orElse) =
      ctor "MustPay" [toRon i actor, toRon i cost, toRon i orElse]
    toRon i (Continuously se d) = ctor "Continuously" [toRon i se, toRon i d]
    toRon i (Modal spec modes) = ctor "Modal" [toRon i spec, toRon i modes]
    toRon i (Each sel body) = ctor "Each" [toRon i sel, toRon i body]
    toRon i (Distribute amount sel body) =
      ctor "Distribute" [toRon i amount, toRon i sel, toRon i body]
    toRon i (Reflexive e) = ctor "Reflexive" [toRon i e]
    toRon i (Delayed q body) = ctor "Delayed" [toRon i q, toRon i body]

  implementation ToRon (KeywordSpec b) where
    toRon _ Flying = "Flying"
    toRon _ FirstStrike = "FirstStrike"
    toRon _ DoubleStrike = "DoubleStrike"
    toRon _ Deathtouch = "Deathtouch"
    toRon _ Reach = "Reach"
    toRon _ Trample = "Trample"
    toRon _ Vigilance = "Vigilance"
    toRon _ Flash = "Flash"
    toRon _ Haste = "Haste"
    toRon _ Indestructible = "Indestructible"
    toRon _ Defender = "Defender"
    toRon _ Shroud = "Shroud"
    toRon _ Menace = "Menace"
    toRon i (Hexproof mp) = ctor "Hexproof" [ronOpt i mp]
    toRon _ Morph = "Morph"
    toRon _ Devoid = "Devoid"
    toRon i (Protection p) = ctor "Protection" [toRon i p]

  implementation ToRon (KeywordAbility b) where
    toRon i (Bare x) = toRon i x
    toRon i (Composite x xs) = ctor "Composite" [toRon i x, toRon i xs]

  implementation ToRon (Ability b) where
    toRon indent (Spell x) = "Spell(\n" ++ mkIndent (S indent)
                          ++ toRon (S indent) x ++ "\n"
                          ++ mkIndent indent ++ ")"
    toRon i (Keyword x) = toRon i x
    toRon i (Activated cost effect {window} {limits}) =
      ctor "Activated" [toRon i cost, toRon i effect, toRon i window, toRon i limits]
    toRon i (Triggered q effect {limits}) =
      ctor "Triggered" [toRon i q, toRon i effect, toRon i limits]
    toRon i (TurnBased ps e) = ctor "TurnBased" [toRon i ps, toRon i e]
    toRon i (Static se) = ctor "Static" [toRon i se]
    toRon i (TurnFaceUp cost) = ctor "TurnFaceUp" [toRon i cost]
    toRon i (AsEnters d xs) = ctor "AsEnters" [toRon i d, toRon i xs]
    toRon i (AsEntersChoosing k p xs) = ctor "AsEntersChoosing" [toRon i k, toRon i p, toRon i xs]

  -- a printed face: a struct whose absent (empty / `Nothing`) fields are omitted.
  implementation ToRon (Characteristics b) where
    toRon indent (MkCharacteristics name manaCost colors types supertypes
                                     subtypes abilities power toughness
                                     loyalty defense) =
      "(\n"
        ++ fastConcat [ ronProperty (S indent) "name" name
                      , ronProperty (S indent) "mana_cost" (nonEmpty manaCost)
                      , ronProperty (S indent) "color_indicator" (nonEmpty colors)
                      , ronProperty (S indent) "types" (Just types)
                      , ronProperty (S indent) "supertypes" (nonEmpty supertypes)
                      , ronProperty (S indent) "subtypes" (nonEmpty subtypes)
                      , ronProperty (S indent) "abilities" (nonEmpty abilities)
                      , ronProperty (S indent) "power" power
                      , ronProperty (S indent) "toughness" toughness
                      , ronProperty (S indent) "loyalty" loyalty
                      , ronProperty (S indent) "defense" defense
                      ]
        ++ mkIndent indent ++ ")"

  implementation ToRon Card where
    toRon indent (Normal face) = "Normal" ++ toRon (S indent) face
    toRon indent (TwoFaced layout front back) =
      "TwoFaced(\n"
        ++ fastConcat [ ronProperty (S indent) "layout" (Just layout)
                      , ronProperty (S indent) "front" (Just front)
                      , ronProperty (S indent) "back" (Just back)
                      ]
        ++ mkIndent indent ++ ")"

-- A tiny demonstration entry point: print a few cards from `Cards` as RON.
main : IO ()
main = do
  putStrLn $ toRon 0 card_LightningBolt
  putStrLn $ toRon 0 card_GrizzlyBears
  putStrLn $ toRon 0 card_TyphoidRats
