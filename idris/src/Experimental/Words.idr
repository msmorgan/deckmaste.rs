module Experimental.Words

import public Data.List
import public Data.List.Quantifiers
import public Data.Maybe
import public Data.Nat
import public Data.So

import public Experimental.KeywordShapes
import public Experimental.FactsGen

%default total

public export
data OptOk : (a -> Type) -> Maybe a -> Type where
  Absent : OptOk p Nothing
  Present : {0 x : a} -> {auto 0 ok : p x} -> OptOk p (Just x)


public export
data CardType = Creature | Artifact | Land | Enchantment | Instant | Sorcery
              | Planeswalker | Battle | Kindred

public export
cardTypeIx : CardType -> Nat
cardTypeIx Creature = 0
cardTypeIx Artifact = 1
cardTypeIx Land = 2
cardTypeIx Enchantment = 3
cardTypeIx Instant = 4
cardTypeIx Sorcery = 5
cardTypeIx Planeswalker = 6
cardTypeIx Battle = 7
cardTypeIx Kindred = 8

public export
Eq CardType where
  (==) a b = cardTypeIx a == cardTypeIx b

public export
natEqRefl : (n : Nat) -> So (n == n)
natEqRefl Z = Oh
natEqRefl (S k) = natEqRefl k

public export
data Stat = Power | Toughness | ManaValue | Loyalty

public export
statIx : Stat -> Nat
statIx Power = 0
statIx Toughness = 1
statIx ManaValue = 2
statIx Loyalty = 3

public export
Eq Stat where
  (==) a b = statIx a == statIx b

public export
data PlayerStat = LifeTotal | StartingLifeTotal

public export
playerStatIx : PlayerStat -> Nat
playerStatIx LifeTotal = 0
playerStatIx StartingLifeTotal = 1

public export
Eq PlayerStat where
  (==) a b = playerStatIx a == playerStatIx b

public export
data Comparator = AtLeast | AtMost | Greater | Less | Eq

public export
comparatorIx : Comparator -> Nat
comparatorIx AtLeast = 0
comparatorIx AtMost = 1
comparatorIx Greater = 2
comparatorIx Less = 3
comparatorIx Eq = 4

public export
Eq Comparator where
  (==) a b = comparatorIx a == comparatorIx b

public export
comparedType : Stat -> Maybe CardType
comparedType Power = Just Creature
comparedType Toughness = Just Creature
comparedType ManaValue = Nothing
comparedType Loyalty = Just Planeswalker

||| A noncreature permanent has no power or toughness [CR#208.3], so a stat
||| read is refused when every head-type alternative of the noun lacks the type
||| the stat belongs to. No known type is permissive, and one alternative that
||| carries the type suffices — an animated land is a land and a creature.
public export
statHeadTysOk : Stat -> List (List CardType) -> Bool
statHeadTysOk c alts = case comparedType c of
                         Nothing => True
                         Just want => all (elem want) alts


public export
data QualitySort : Type where
  Color : QualitySort
  SubtypeQ : CardType -> QualitySort
  CardName : QualitySort
  Number : QualitySort
  CardTypeQ : QualitySort
  CounterKindQ : QualitySort

public export
qualityIx : QualitySort -> Nat
qualityIx Color = 0
qualityIx CardName = 1
qualityIx Number = 2
qualityIx CardTypeQ = 3
qualityIx CounterKindQ = 4
qualityIx (SubtypeQ t) = 5 + cardTypeIx t

public export
Eq QualitySort where
  (==) a b = qualityIx a == qualityIx b

public export
chosenQualityReadOk : QualitySort -> Bool
chosenQualityReadOk Color = True
chosenQualityReadOk (SubtypeQ _) = True
chosenQualityReadOk CardName = True
chosenQualityReadOk Number = False
chosenQualityReadOk CardTypeQ = True
chosenQualityReadOk CounterKindQ = False

public export
ChosenQualityRead : QualitySort -> Type
ChosenQualityRead q = So (chosenQualityReadOk q)

public export
data Letter = X | Y

public export
letterIx : Letter -> Nat
letterIx X = 0
letterIx Y = 1

public export
Eq Letter where
  (==) a b = letterIx a == letterIx b

export infixl 5 \/

public export
data Kind : Type where
  Object : Kind
  Player : Kind
  Quality : QualitySort -> Kind
  Outcome : Kind
  Gap : Kind
  LetterK : Letter -> Kind
  TurnRef : Kind
  ||| A pile of objects; the pile itself is not an object [CR#700.3b].
  Pile : Kind
  (\/) : Kind -> Kind -> Kind

public export
kindIx : Kind -> Nat
kindIx Object = 0
kindIx Player = 1
kindIx (Quality _) = 2
kindIx Outcome = 3
kindIx Gap = 4
kindIx (LetterK _) = 5
kindIx TurnRef = 6
kindIx Pile = 7
kindIx (_ \/ _) = 8

mutual
  public export
  sameKindValue : Kind -> Kind -> Bool
  sameKindValue a b = kindIx a == kindIx b && sameKindPayload a b

  public export
  sameKindPayload : Kind -> Kind -> Bool
  sameKindPayload (Quality a) (Quality b) = a == b
  sameKindPayload (LetterK a) (LetterK b) = a == b
  sameKindPayload (a \/ b) (c \/ d) = sameKindValue a c && sameKindValue b d
  sameKindPayload _ _ = True

public export
Eq Kind where
  (==) = sameKindValue

public export
sameQRefl : (q : QualitySort) -> So (q == q)
sameQRefl q = natEqRefl (qualityIx q)

public export
sameLetterRefl : (w : Letter) -> So (w == w)
sameLetterRefl X = Oh
sameLetterRefl Y = Oh

public export
data Joins : Kind -> Kind -> Kind -> Type where
  JoinSame : Joins k k k
  JoinDiff : {auto 0 ne : So (not (a == b))} -> Joins a b (a \/ b)

public export
kindLte : Kind -> Kind -> Bool
kindLte (a \/ b) y = kindLte a y && kindLte b y
kindLte x (a \/ b) = kindLte x a || kindLte x b
kindLte x y = x == y

public export
kindLteInL : (x, a, b : Kind) -> So (kindLte x a) -> So (kindLte x (a \/ b))
kindLteInL Object a b ok = orSo (Left ok)
kindLteInL Player a b ok = orSo (Left ok)
kindLteInL (Quality _) a b ok = orSo (Left ok)
kindLteInL Outcome a b ok = orSo (Left ok)
kindLteInL Gap a b ok = orSo (Left ok)
kindLteInL (LetterK _) a b ok = orSo (Left ok)
kindLteInL TurnRef a b ok = orSo (Left ok)
kindLteInL Pile a b ok = orSo (Left ok)
kindLteInL (p \/ q) a b ok =
  andSo (kindLteInL p a b (fst (soAnd ok)), kindLteInL q a b (snd (soAnd ok)))

public export
kindLteInR : (x, a, b : Kind) -> So (kindLte x b) -> So (kindLte x (a \/ b))
kindLteInR Object a b ok = orSo (Right ok)
kindLteInR Player a b ok = orSo (Right ok)
kindLteInR (Quality _) a b ok = orSo (Right ok)
kindLteInR Outcome a b ok = orSo (Right ok)
kindLteInR Gap a b ok = orSo (Right ok)
kindLteInR (LetterK _) a b ok = orSo (Right ok)
kindLteInR TurnRef a b ok = orSo (Right ok)
kindLteInR Pile a b ok = orSo (Right ok)
kindLteInR (p \/ q) a b ok =
  andSo (kindLteInR p a b (fst (soAnd ok)), kindLteInR q a b (snd (soAnd ok)))

public export
kindLteRefl : (k : Kind) -> So (kindLte k k)
kindLteRefl Object = Oh
kindLteRefl Player = Oh
kindLteRefl (Quality q) = sameQRefl q
kindLteRefl Outcome = Oh
kindLteRefl Gap = Oh
kindLteRefl (LetterK w) = sameLetterRefl w
kindLteRefl TurnRef = Oh
kindLteRefl Pile = Oh
kindLteRefl (a \/ b) =
  andSo (kindLteInL a a b (kindLteRefl a), kindLteInR b a b (kindLteRefl b))

public export
kindLteJoinL : (a, b : Kind) -> So (kindLte a (a \/ b))
kindLteJoinL a b = kindLteInL a a b (kindLteRefl a)

public export
kindLteJoinR : (a, b : Kind) -> So (kindLte b (a \/ b))
kindLteJoinR a b = kindLteInR b a b (kindLteRefl b)

public export
data AggregateOp = SumOf | MinOf | MaxOf

public export
isExtremal : AggregateOp -> Bool
isExtremal SumOf = False
isExtremal MinOf = True
isExtremal MaxOf = True

public export
IsExtremal : AggregateOp -> Type
IsExtremal op = So (isExtremal op)

public export
aggregateOpIx : AggregateOp -> Nat
aggregateOpIx SumOf = 0
aggregateOpIx MinOf = 1
aggregateOpIx MaxOf = 2

public export
Eq AggregateOp where
  (==) a b = aggregateOpIx a == aggregateOpIx b

public export
data SubtypeScope = AnySubtype | BasicOnly | NonbasicOnly

public export
subtypeScopeOk : CardType -> SubtypeScope -> Bool
subtypeScopeOk _ AnySubtype = True
subtypeScopeOk Land BasicOnly = True
subtypeScopeOk Land NonbasicOnly = True
subtypeScopeOk _ BasicOnly = False
subtypeScopeOk _ NonbasicOnly = False

public export
data KindAxis : Type where
  CardTypeAxis : KindAxis
  PermanentTypeAxis : KindAxis
  ColorAxis : KindAxis
  SubtypeAxis : (host : CardType) -> (only : SubtypeScope) ->
                {auto 0 sc : So (subtypeScopeOk host only)} -> KindAxis
  ValueAxis : Stat -> KindAxis
  CounterKindAxis : KindAxis
  ColorPairAxis : KindAxis

public export
kindAxisSort : KindAxis -> Maybe QualitySort
kindAxisSort CardTypeAxis = Just CardTypeQ
kindAxisSort PermanentTypeAxis = Nothing
kindAxisSort ColorAxis = Just Color
kindAxisSort (SubtypeAxis host _) = Just (SubtypeQ host)
kindAxisSort (ValueAxis _) = Just Number
kindAxisSort CounterKindAxis = Just CounterKindQ
kindAxisSort ColorPairAxis = Nothing

public export
kindAxisClosed : KindAxis -> Bool
kindAxisClosed CardTypeAxis = True
kindAxisClosed PermanentTypeAxis = True
kindAxisClosed ColorAxis = True
kindAxisClosed (SubtypeAxis Land BasicOnly) = True
kindAxisClosed (SubtypeAxis _ _) = False
kindAxisClosed (ValueAxis _) = False
kindAxisClosed CounterKindAxis = False
kindAxisClosed ColorPairAxis = True

public export
colorCountOk : Nat -> Bool
colorCountOk n = n >= 2 && n <= 5

-- an object is 0..5 colors [CR#105.1]; `Eq 0` is `IsColorless`
public export
colorBoundOk : Comparator -> Nat -> Bool
colorBoundOk Eq n = n >= 1 && n <= 5
colorBoundOk AtLeast n = n >= 2 && n <= 5
colorBoundOk AtMost n = n >= 1 && n <= 4
colorBoundOk Greater n = n >= 1 && n <= 4
colorBoundOk Less n = n >= 2 && n <= 5

public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost | CountersPut
                 | DamagePrevented | RollResult | CoinFlipped
                 | DiceRolled
                 | PlanarRolled
                 | NamedNumber
                 | RepeatCount
                 | CountersRemoved
                 | ManaAdded
                 | ManaProduced
                 | CeilingShortfall

public export
outcomeComparable : OutcomeSort -> Bool
outcomeComparable RollResult = True
outcomeComparable _ = False

public export
data FlipCall = WinsFlip | LosesFlip

public export
data CoinFace = Heads | Tails

public export
data RollExtreme = LowestRoll | HighestRoll

public export
data Plurality = OneOf | ManyOf

public export
isOne : Plurality -> Bool
isOne OneOf = True
isOne ManyOf = False

public export
outputPlur : (agent : Plurality) -> (perAgent : Plurality) -> Plurality
outputPlur OneOf OneOf = OneOf
outputPlur OneOf ManyOf = ManyOf
outputPlur ManyOf OneOf = ManyOf
outputPlur ManyOf ManyOf = ManyOf


public export
data NameAgreement : Type where
  DifferentNames : NameAgreement
  SameName : NameAgreement

public export
atLeastTwo : Nat -> Bool
atLeastTwo (S (S _)) = True
atLeastTwo _ = False

public export
AtLeastTwo : Nat -> Type
AtLeastTwo n = So (atLeastTwo n)

public export
data Determiner = TargetD | AD | EachD | AllD | TheD | PartD
                | CountD
                | SelfD
                | BareD

public export
data PossessorAxis = OwnerAx | ControllerAx

public export
possessorAxisIx : PossessorAxis -> Nat
possessorAxisIx OwnerAx = 0
possessorAxisIx ControllerAx = 1

public export
Eq PossessorAxis where
  (==) a b = possessorAxisIx a == possessorAxisIx b

public export
data CombatRelation = BlockerOf | BlockedBy | AttackedBy | AttackerOf
                    | CouldBlock | CouldBeBlockedBy

public export
combatRelationIx : CombatRelation -> Nat
combatRelationIx BlockerOf = 0
combatRelationIx BlockedBy = 1
combatRelationIx AttackedBy = 2
combatRelationIx AttackerOf = 3
combatRelationIx CouldBlock = 4
combatRelationIx CouldBeBlockedBy = 5

public export
Eq CombatRelation where
  (==) a b = combatRelationIx a == combatRelationIx b

public export
data Zone = Battlefield | Graveyard | Exile | Hand | Library | Stack
          | Command

public export
zoneIx : Zone -> Nat
zoneIx Battlefield = 0
zoneIx Graveyard = 1
zoneIx Exile = 2
zoneIx Hand = 3
zoneIx Library = 4
zoneIx Stack = 5
zoneIx Command = 6

public export
Eq Zone where
  (==) a b = zoneIx a == zoneIx b

public export
data LibPos = OnTop | OnBottom

public export
data Arrangement = AnyOrder | RandomOrder

public export
data Ordinal : Type where   -- the N of "Nth" [CR#401.7]
  Nth : (n : Nat) -> {auto 0 nz : IsSucc n} -> Ordinal

public export
LibOrdinal : Type
LibOrdinal = Ordinal

public export
VerbLabel : Type
VerbLabel = String

public export
data PremiseSort = ObjectPremise | ManaPremise | ValuePremise

public export
premiseSortIx : PremiseSort -> Nat
premiseSortIx ObjectPremise = 0
premiseSortIx ManaPremise = 1
premiseSortIx ValuePremise = 2

public export
Eq PremiseSort where
  (==) a b = premiseSortIx a == premiseSortIx b

public export
record DeedRole where
  constructor MkDeedRole
  roleKinds : List Kind
  roleTypes : List CardType
  roleBare : Bool
  roleZone : Maybe Zone

public export
noRole : DeedRole
noRole = MkDeedRole [] [] False Nothing

public export
record ActFacts where
  constructor MkActFacts
  label : VerbLabel
  participle : Maybe String
  actDest : Maybe Zone
  actStepwise : Bool
  actLoci : List Zone
  actIntransitive : Bool
  agentRole : DeedRole
  patientRole : DeedRole
  actDefends : Bool
  actTargeted : Bool
  actCounterfactual : Maybe PremiseSort
  actRides : Bool
  actPlays : Bool
  actBounded : Bool

||| Keyword actions ([CR#701]) — one-shot verbs in effect position that confer
||| nothing, unlike the keyword abilities of `keywordFacts`: mill [CR#701.17],
||| scry [CR#701.22].
public export
actFacts : List ActFacts
actFacts =
  [ MkActFacts "Destroy"     (Just "destroyed") (Just Graveyard) False [] False
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Sacrifice"   (Just "sacrificed") (Just Graveyard) False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False True
  , MkActFacts "Exile"       (Just "exiled") (Just Exile) False [] False
      noRole (MkDeedRole [Object] [] False Nothing)
      False False Nothing False False False
  , MkActFacts "Discard"     (Just "discarded") (Just Graveyard) False [] False
      noRole (MkDeedRole [Object] [] False (Just Hand))
      False False Nothing False False False
  , MkActFacts "Mill"        (Just "milled") (Just Graveyard) False [] False
      noRole (MkDeedRole [Object] [] False (Just Library))
      False False Nothing False False False
  , MkActFacts "Scry"        Nothing Nothing True [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Surveil"     Nothing Nothing True [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Tap"         (Just "tapped") Nothing False [] False
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Untap"       (Just "untapped") Nothing False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing False False True
  , MkActFacts "Return"      Nothing Nothing False [] False
      noRole (MkDeedRole [Object] [] False Nothing)
      False False Nothing False False False
  , MkActFacts "GainControl" Nothing Nothing False [] False
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Put"         Nothing Nothing False [] False
      noRole (MkDeedRole [Object] [] False Nothing)
      False False Nothing False False False
  , MkActFacts "Search"      Nothing Nothing False
      [Battlefield, Graveyard, Exile, Hand, Library, Stack, Command] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False True
  , MkActFacts "Shuffle"     Nothing Nothing False [Library] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Proliferate" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "The Ring Tempts You" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Transform"   Nothing Nothing False [] True
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Convert"     Nothing Nothing False [] True
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Meld"        Nothing (Just Battlefield) False [] False
      noRole (MkDeedRole [Object] [] False Nothing)
      False False Nothing False False False
  , MkActFacts "Unlock"      Nothing Nothing False [] False
      noRole (MkDeedRole [] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Fully Unlock" Nothing Nothing False [] False
      noRole (MkDeedRole [Object] [] False (Just Battlefield))
      False False Nothing False False False
  , MkActFacts "Attack"      Nothing Nothing False [] False
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Planeswalker, Battle] False (Just Battlefield))
      True False (Just ObjectPremise) False False True
  , MkActFacts "Block"       Nothing Nothing False [] False
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Creature] False (Just Battlefield))
      False False (Just ObjectPremise) False False True
  , MkActFacts "Target"      Nothing Nothing False [] False
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object, Player] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False True (Just ObjectPremise) False False True
  , MkActFacts "Cast"        Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False (Just ObjectPremise) True True True
  , MkActFacts "Play"        Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True Nothing)
      False False (Just ObjectPremise) True True True
  , MkActFacts "Counter"     Nothing Nothing False [] False
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing True False False
  , MkActFacts "Copy"        Nothing Nothing False [] False
      (MkDeedRole [] [] True (Just Stack))
      (MkDeedRole [Object] [Creature, Artifact, Enchantment, Instant, Sorcery,
                   Planeswalker, Battle, Kindred] True (Just Stack))
      False False Nothing False False True
  , MkActFacts "Activate"    Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [] True (Just Stack))
      False False Nothing False False True
  , MkActFacts "Regenerate"  Nothing Nothing False [] False
      (MkDeedRole [] [] True Nothing)
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False Nothing True False True
  , MkActFacts "GainLife"    Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False True
  , MkActFacts "Draw"        Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing)
      (MkDeedRole [Object] [] True (Just Library))
      False False Nothing False False True
  , MkActFacts "Trigger"     Nothing Nothing False [] False
      (MkDeedRole [Object] [] True (Just Stack)) noRole
      False False Nothing False False True
  , MkActFacts "LoseGame"    Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False False
  , MkActFacts "WinGame"     Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False False
  , MkActFacts "Spend"       Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False (Just ManaPremise) False False True
  , MkActFacts "Crew"        Nothing Nothing False [] False
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Artifact] True (Just Battlefield))
      False False (Just ValuePremise) False False True
  , MkActFacts "Saddle"      Nothing Nothing False [] False
      (MkDeedRole [Object] [Creature] True (Just Battlefield))
      (MkDeedRole [Object] [Creature, Artifact, Land, Enchantment,
                            Planeswalker, Battle] True (Just Battlefield))
      False False (Just ValuePremise) False False True
  , MkActFacts "Vote"        Nothing Nothing False [] False
      (MkDeedRole [Player] [] True Nothing) noRole
      False False Nothing False False True
  , MkActFacts "Venture Into The Dungeon" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Abandon" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Adapt" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Airbend" Nothing (Just Exile) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Amass" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Assemble" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Attach" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Behold" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Blight" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Bolster" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Clash" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Cloak" Nothing (Just Battlefield) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Collect Evidence" Nothing (Just Exile) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Connive" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Create" Nothing (Just Battlefield) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Detain" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Discover" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Double" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Earthbend" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Endure" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Exchange" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Exert" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Explore" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Face A Villainous Choice" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Fateseal" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Fight" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Forage" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Goad" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Harness" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Heal" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Incubate" Nothing (Just Battlefield) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Investigate" Nothing (Just Battlefield) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Learn" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Manifest" Nothing (Just Battlefield) False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Manifest Dread" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Monstrosity" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Open An Attraction" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Planeswalk" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Populate" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Recruit" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Reveal" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Roll To Visit Your Attractions" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Set In Motion" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Support" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Suspect" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Time Travel" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Triple" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  , MkActFacts "Phase In" Nothing Nothing False [] True
      (MkDeedRole [Object] [] True (Just Battlefield)) noRole
      False False Nothing False False False
  , MkActFacts "Waterbend" Nothing Nothing False [] False
      noRole noRole
      False False Nothing False False False
  ]

public export
distinctActLabels : List ActFacts -> Bool
distinctActLabels [] = True
distinctActLabels (f :: fs) =
  not (elem (label f) (map label fs)) && distinctActLabels fs

export
actLabelsDistinct : So (distinctActLabels Experimental.Words.actFacts)
actLabelsDistinct = Oh

public export
factsIn : VerbLabel -> List ActFacts -> Maybe ActFacts
factsIn v [] = Nothing
factsIn v (f :: fs) = if label f == v then Just f else factsIn v fs

public export
actFactsFor : VerbLabel -> Maybe ActFacts
actFactsFor v = factsIn v actFacts

public export
knownAct : VerbLabel -> Bool
knownAct v = isJust (actFactsFor v)

public export
data KnownAct : VerbLabel -> Type where
  ActInFactsTable : {0 v : VerbLabel} -> {auto 0 ok : knownAct v = True} ->
                    KnownAct v

public export
participleOf : VerbLabel -> Maybe String
participleOf v = actFactsFor v >>= participle

public export
actPatientKindsOf : VerbLabel -> List Kind
actPatientKindsOf v = maybe [] (roleKinds . patientRole) (actFactsFor v)

public export
actNamesPatient : VerbLabel -> Bool
actNamesPatient v = not (null (actPatientKindsOf v))

public export
actZoneOf : VerbLabel -> Maybe Zone
actZoneOf v = actFactsFor v >>= roleZone . patientRole

public export
actDestOf : VerbLabel -> Maybe Zone
actDestOf v = actFactsFor v >>= actDest

public export
actLociOf : VerbLabel -> List Zone
actLociOf v = maybe [] actLoci (actFactsFor v)

public export
actNamesLocus : VerbLabel -> Bool
actNamesLocus v = case actLociOf v of
                    [] => False
                    _ => True

public export
actStepwiseOf : VerbLabel -> Bool
actStepwiseOf v = maybe False actStepwise (actFactsFor v)

public export
actIntransitiveOf : VerbLabel -> Bool
actIntransitiveOf v = maybe False actIntransitive (actFactsFor v)

public export
actNamesParticiple : VerbLabel -> Bool
actNamesParticiple v = isJust (participleOf v)

public export
ActNamesParticiple : VerbLabel -> Type
ActNamesParticiple v = So (actNamesParticiple v)

public export
record Stamp where
  constructor MkStamp
  verb : VerbLabel
  wasField : Bool
  moved : Bool

public export
data Origin = TokenOrigin | CopyOrigin

public export
isTokenOrigin : Maybe Origin -> Bool
isTokenOrigin (Just TokenOrigin) = True
isTokenOrigin (Just CopyOrigin) = False
isTokenOrigin Nothing = False

public export
isCopyOrigin : Maybe Origin -> Bool
isCopyOrigin (Just CopyOrigin) = True
isCopyOrigin (Just TokenOrigin) = False
isCopyOrigin Nothing = False

public export
data PileFace = FaceDownPile | FaceUpPile

public export
pileFaceIx : PileFace -> Nat
pileFaceIx FaceDownPile = 0
pileFaceIx FaceUpPile = 1

public export
Eq PileFace where
  (==) a b = pileFaceIx a == pileFaceIx b

public export
facesFit : List PileFace -> Nat -> Bool
facesFit [] _ = True
facesFit fs n = length fs == n

public export
FacesFit : List PileFace -> Nat -> Type
FacesFit fs n = So (facesFit fs n)

public export
pileMentionFace : List PileFace -> Maybe PileFace
pileMentionFace [] = Nothing
pileMentionFace (f :: fs) = if all (== f) fs then Just f else Nothing

public export
data Payload : Kind -> Type where
  ObjectP : (ty : Maybe CardType) -> (zone : Maybe Zone) ->
            (prov : Maybe Stamp) -> (orig : Maybe Origin) ->
            (size : Maybe Nat) -> Payload Object
  PlayerP : Payload Player
  ChosenPlayerP : Payload Player
  QualityP : Payload (Quality q)
  OutcomeP : (sort : OutcomeSort) -> Payload Outcome
  GapP : Payload Gap
  LetterP : Payload (LetterK l)
  TurnRefP : Payload TurnRef
  ||| An ability on the stack: an object with no printed characteristics
  ||| and no zone of its own [CR#109.1,113.1c,405.1].
  AbilityP : (orig : Maybe Origin) -> Payload Object
  PileP : (zone : Maybe Zone) -> (size : Maybe Nat) ->
          (face : Maybe PileFace) -> Payload Pile
  JoinP : Payload a -> Payload b -> Payload (a \/ b)

public export
record Binding where
  constructor MkBinding
  det : Determiner
  kind : Kind
  plur : Plurality
  payload : Payload kind

public export
Bindings : Type
Bindings = List Binding

public export
payloadZone : Payload k -> Maybe Zone
payloadZone (ObjectP _ zn _ _ _) = zn
payloadZone PlayerP = Nothing
payloadZone ChosenPlayerP = Nothing
payloadZone QualityP = Nothing
payloadZone (OutcomeP _) = Nothing
payloadZone GapP = Nothing
payloadZone LetterP = Nothing
payloadZone TurnRefP = Nothing
payloadZone (AbilityP _) = Just Stack
payloadZone (PileP zn _ _) = zn
payloadZone (JoinP l r) = maybe (payloadZone r) Just (payloadZone l)

public export
joinSeed : Maybe CardType -> Maybe CardType -> Maybe CardType
joinSeed Nothing t = t
joinSeed t Nothing = t
joinSeed (Just t) (Just u) = if t == u then Just t else Nothing

public export
payloadTy : Payload k -> Maybe CardType
payloadTy (ObjectP ty _ _ _ _) = ty
payloadTy PlayerP = Nothing
payloadTy ChosenPlayerP = Nothing
payloadTy QualityP = Nothing
payloadTy (OutcomeP _) = Nothing
payloadTy GapP = Nothing
payloadTy LetterP = Nothing
payloadTy TurnRefP = Nothing
payloadTy (AbilityP _) = Nothing
payloadTy (PileP _ _ _) = Nothing
payloadTy (JoinP l r) = joinSeed (payloadTy l) (payloadTy r)

public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ pl) = payloadZone pl

public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ pl) = payloadTy pl

public export
payloadSize : Payload k -> Maybe Nat
payloadSize (ObjectP _ _ _ _ sz) = sz
payloadSize PlayerP = Nothing
payloadSize ChosenPlayerP = Nothing
payloadSize QualityP = Nothing
payloadSize (OutcomeP _) = Nothing
payloadSize GapP = Nothing
payloadSize LetterP = Nothing
payloadSize TurnRefP = Nothing
payloadSize (AbilityP _) = Nothing
payloadSize (PileP _ sz _) = sz
payloadSize (JoinP l r) = maybe (payloadSize r) Just (payloadSize l)

public export
bindingSize : Binding -> Maybe Nat
bindingSize (MkBinding _ _ _ pl) = payloadSize pl

public export
payloadFace : Payload k -> Maybe PileFace
payloadFace (PileP _ _ fc) = fc
payloadFace _ = Nothing

public export
bindingFace : Binding -> Maybe PileFace
bindingFace (MkBinding _ _ _ pl) = payloadFace pl

public export
sized : Maybe Nat -> Binding -> Binding
sized sz (MkBinding det Object pl (ObjectP ty zn pv og _)) =
  MkBinding det Object pl (ObjectP ty zn pv og sz)
sized sz (MkBinding det Pile pl (PileP zn _ fc)) =
  MkBinding det Pile pl (PileP zn sz fc)
sized _ b = b


public export
data HeadTy : Kind -> Type where
  SoleTy : Maybe CardType -> HeadTy k
  JoinTy : HeadTy ka -> HeadTy kb -> HeadTy (ka \/ kb)

public export
outcomeSortIx : OutcomeSort -> Nat
outcomeSortIx DamageDealt = 0
outcomeSortIx LifeGained = 1
outcomeSortIx LifeLost = 2
outcomeSortIx CountersPut = 3
outcomeSortIx DamagePrevented = 4
outcomeSortIx RollResult = 5
outcomeSortIx CoinFlipped = 6
outcomeSortIx DiceRolled = 7
outcomeSortIx PlanarRolled = 8
outcomeSortIx NamedNumber = 9
outcomeSortIx RepeatCount = 10
outcomeSortIx CountersRemoved = 11
outcomeSortIx ManaAdded = 12
outcomeSortIx ManaProduced = 13
outcomeSortIx CeilingShortfall = 14

public export
Eq OutcomeSort where
  (==) a b = outcomeSortIx a == outcomeSortIx b

public export
outcomeB : OutcomeSort -> Binding
outcomeB s = MkBinding TheD Outcome OneOf (OutcomeP s)

public export
gapB : Binding
gapB = MkBinding TheD Gap OneOf GapP

public export
turnRefB : Binding
turnRefB = MkBinding TheD TurnRef OneOf TurnRefP

public export
letterB : Letter -> Binding
letterB l = MkBinding AD (LetterK l) OneOf LetterP

public export
qualityB : QualitySort -> Binding
qualityB q = MkBinding AD (Quality q) OneOf QualityP

public export
data ChoiceSort : Type where
  QSort : QualitySort -> ChoiceSort
  PlayerC : ChoiceSort

public export
choiceSortIx : ChoiceSort -> Nat
choiceSortIx (QSort _) = 0
choiceSortIx PlayerC = 1

public export
sameChoicePayload : ChoiceSort -> ChoiceSort -> Bool
sameChoicePayload (QSort a) (QSort b) = a == b
sameChoicePayload _ _ = True

public export
Eq ChoiceSort where
  (==) a b = choiceSortIx a == choiceSortIx b && sameChoicePayload a b

public export
choiceB : ChoiceSort -> Binding
choiceB (QSort q) = qualityB q
choiceB PlayerC = MkBinding AD Player OneOf ChosenPlayerP

public export
choiceBinds : ChoiceSort -> (k : Kind) -> Payload k -> Bool
choiceBinds (QSort q) k _ = kindLte (Quality q) k
choiceBinds PlayerC _ ChosenPlayerP = True
choiceBinds PlayerC _ _ = False

public export
choiceSortAt : Kind -> Maybe ChoiceSort
choiceSortAt (Quality q) = Just (QSort q)
choiceSortAt Player = Just PlayerC
choiceSortAt _ = Nothing

public export
choiceDeltaAt : Kind -> List Binding
choiceDeltaAt k = case choiceSortAt k of
                    Nothing => []
                    Just s => [choiceB s]

public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes k (MkBinding _ k' OneOf _ :: bs) =
  if kindLte k k' then S (countOnes k bs) else countOnes k bs
countOnes k (_ :: bs) = countOnes k bs

public export
countOutcomes : OutcomeSort -> Bindings -> Nat
countOutcomes s [] = Z
countOutcomes s (MkBinding _ Outcome OneOf (OutcomeP s') :: bs) =
  if s == s' then S (countOutcomes s bs) else countOutcomes s bs
countOutcomes s (_ :: bs) = countOutcomes s bs

public export
data DieSides : Bindings -> Type where
  SidesOf : (n : Nat) -> {auto 0 nz : IsSucc n} -> DieSides bs
  ThoseDice : {auto 0 ok : countOutcomes DiceRolled bs = 1} -> DieSides bs

public export
damageDealtInScope : Bindings -> Bool
damageDealtInScope [] = False
damageDealtInScope (MkBinding _ Outcome OneOf (OutcomeP DamageDealt) :: _) = True
damageDealtInScope (_ :: bs) = damageDealtInScope bs

public export
coinFlipInScope : Bindings -> Bool
coinFlipInScope [] = False
coinFlipInScope (MkBinding _ Outcome OneOf (OutcomeP CoinFlipped) :: _) = True
coinFlipInScope (_ :: bs) = coinFlipInScope bs

public export
planarRollInScope : Bindings -> Bool
planarRollInScope [] = False
planarRollInScope (MkBinding _ Outcome OneOf (OutcomeP PlanarRolled) :: _) = True
planarRollInScope (_ :: bs) = planarRollInScope bs

public export
ignorableInScope : Bindings -> Bool
ignorableInScope bs =
  countOutcomes RollResult bs == 1 || coinFlipInScope bs ||
  planarRollInScope bs

public export
outcomeIsQuantity : OutcomeSort -> Bool
outcomeIsQuantity DamageDealt = True
outcomeIsQuantity LifeGained = True
outcomeIsQuantity LifeLost = True
outcomeIsQuantity CountersPut = True
outcomeIsQuantity DamagePrevented = True
outcomeIsQuantity RollResult = True
outcomeIsQuantity CoinFlipped = False
outcomeIsQuantity DiceRolled = True
outcomeIsQuantity PlanarRolled = False
outcomeIsQuantity NamedNumber = True
outcomeIsQuantity RepeatCount = True
outcomeIsQuantity CountersRemoved = True
outcomeIsQuantity ManaAdded = False
outcomeIsQuantity ManaProduced = False
outcomeIsQuantity CeilingShortfall = False

public export
countQuantOutcomes : Bindings -> Nat
countQuantOutcomes [] = Z
countQuantOutcomes (MkBinding _ Outcome OneOf (OutcomeP s) :: bs) =
  if outcomeIsQuantity s then S (countQuantOutcomes bs) else countQuantOutcomes bs
countQuantOutcomes (_ :: bs) = countQuantOutcomes bs

public export
countChoice : ChoiceSort -> Bindings -> Nat
countChoice s [] = Z
countChoice s (MkBinding _ k OneOf p :: bs) =
  if choiceBinds s k p then S (countChoice s bs) else countChoice s bs
countChoice s (_ :: bs) = countChoice s bs

public export
ChoiceStands : Nat -> Type
ChoiceStands n = So (isSucc n)

||| Which announced choice a read names: the one standing choice, or the most
||| recent of several.
public export
data ChoiceRef = TheChoice | TheLatestChoice

public export
choiceRefIx : ChoiceRef -> Nat
choiceRefIx TheChoice = 0
choiceRefIx TheLatestChoice = 1

public export
Eq ChoiceRef where
  (==) a b = choiceRefIx a == choiceRefIx b

public export
choiceRefOk : ChoiceRef -> Nat -> Type
choiceRefOk TheChoice n = n = 1
choiceRefOk TheLatestChoice n = ChoiceStands n

public export
data Disclosure = Openly | Secretly

public export
disclosureIx : Disclosure -> Nat
disclosureIx Openly = 0
disclosureIx Secretly = 1

public export
Eq Disclosure where
  (==) a b = disclosureIx a == disclosureIx b

public export
data HiddenSort = HiddenNumbers | HiddenChoices

public export
hiddenSortIx : HiddenSort -> Nat
hiddenSortIx HiddenNumbers = 0
hiddenSortIx HiddenChoices = 1

public export
Eq HiddenSort where
  (==) a b = hiddenSortIx a == hiddenSortIx b

public export
VoteLabel : Type
VoteLabel = String

public export
distinctLabels : List VoteLabel -> Bool
distinctLabels [] = True
distinctLabels (l :: ls) = not (elem l ls) && distinctLabels ls

public export
ballotLabelsOk : List VoteLabel -> Bool
ballotLabelsOk opts = 2 <= length opts && distinctLabels opts

public export
0 BallotLabelsOk : List VoteLabel -> Type
BallotLabelsOk opts = So (ballotLabelsOk opts)

public export
countLetter : Letter -> Bindings -> Nat
countLetter l [] = Z
countLetter l (MkBinding _ k OneOf _ :: bs) =
  if kindLte (LetterK l) k then S (countLetter l bs) else countLetter l bs
countLetter l (_ :: bs) = countLetter l bs

public export
dropLetter : Letter -> Bindings -> Bindings
dropLetter l [] = []
dropLetter l (MkBinding d k pl pd :: bs) =
  if kindLte (LetterK l) k then dropLetter l bs
                           else MkBinding d k pl pd :: dropLetter l bs

public export
openLetter : Letter -> Binding -> Bool
openLetter l (MkBinding AD k pl _) = isOne pl && kindLte (LetterK l) k
openLetter l (MkBinding TargetD _ _ _) = False
openLetter l (MkBinding EachD _ _ _) = False
openLetter l (MkBinding AllD _ _ _) = False
openLetter l (MkBinding TheD _ _ _) = False
openLetter l (MkBinding PartD _ _ _) = False
openLetter l (MkBinding CountD _ _ _) = False
openLetter l (MkBinding SelfD _ _ _) = False
openLetter l (MkBinding BareD _ _ _) = False

public export
anyOpenLetter : Letter -> Bindings -> Bool
anyOpenLetter l [] = False
anyOpenLetter l (b :: bs) = openLetter l b || anyOpenLetter l bs

public export
defineLetter : Letter -> Bindings -> Bindings
defineLetter l [] = []
defineLetter l (b :: bs) =
  if openLetter l b then MkBinding TheD b.kind b.plur b.payload :: defineLetter l bs
                    else b :: defineLetter l bs

public export
letterDelta : Letter -> Bindings -> List Binding
letterDelta l bs = if countLetter l bs == 0 then [letterB l] else []

public export
countManysAny : Bindings -> Nat
countManysAny [] = Z
countManysAny (MkBinding _ _ ManyOf _ :: bs) = S (countManysAny bs)
countManysAny (_ :: bs) = countManysAny bs

public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ :: bs) =
  if kindLte k k' then S (countManys k bs) else countManys k bs
countManys k (_ :: bs) = countManys k bs

public export
objGroup : Kind -> Binding -> Bool
objGroup k b = kindLte k b.kind && not (isOne b.plur)

public export
countGroups : Kind -> Bindings -> Nat
countGroups k [] = Z
countGroups k (MkBinding PartD _ _ _ :: bs) = countGroups k bs
countGroups k (MkBinding BareD _ _ _ :: bs) = countGroups k bs
countGroups k (b :: bs) =
  if objGroup k b then S (countGroups k bs) else countGroups k bs

public export
countParts : Kind -> Bindings -> Nat
countParts k [] = Z
countParts k (b@(MkBinding PartD _ _ _) :: bs) =
  if kindLte k b.kind then S (countParts k bs) else countParts k bs
countParts k (_ :: bs) = countParts k bs

public export
theRestOk : Kind -> Bindings -> Bool
theRestOk k bs = countGroups k bs <= 1 && not (countParts k bs == Z)

public export
partsTaken : Kind -> Bindings -> Nat
partsTaken k [] = Z
partsTaken k (b@(MkBinding PartD _ _ _) :: bs) =
  if kindLte k b.kind
    then (case bindingSize b of
            Just n => n + partsTaken k bs
            Nothing => S (partsTaken k bs))
    else partsTaken k bs
partsTaken k (_ :: bs) = partsTaken k bs

public export
countedGroupSize : Kind -> Bindings -> Maybe Nat
countedGroupSize k [] = Nothing
countedGroupSize k (MkBinding PartD _ _ _ :: bs) = countedGroupSize k bs
countedGroupSize k (MkBinding BareD _ _ _ :: bs) = countedGroupSize k bs
countedGroupSize k (b :: bs) =
  if objGroup k b then bindingSize b else countedGroupSize k bs

public export
theOtherOk : Kind -> Bindings -> Bool
theOtherOk k bs = theRestOk k bs &&
                  (case countedGroupSize k bs of
                     Nothing => False
                     Just n => n == S (partsTaken k bs))

public export
theRestFits : Kind -> Plurality -> Bindings -> Bool
theRestFits k OneOf bs = theOtherOk k bs
theRestFits k ManyOf bs = theRestOk k bs

public export
groupSpent : Kind -> Bindings -> Bindings
groupSpent k [] = []
groupSpent k (MkBinding PartD j pl p :: bs) =
  MkBinding TheD j pl p :: groupSpent k bs
groupSpent k (b :: bs) =
  if objGroup k b then groupSpent k bs else b :: groupSpent k bs

public export
partsClosed : Bindings -> Bindings
partsClosed [] = []
partsClosed (MkBinding PartD j pl p :: bs) =
  MkBinding TheD j pl p :: partsClosed bs
partsClosed (b :: bs) = b :: partsClosed bs

public export
partsDistributed : Bindings -> Bool
partsDistributed [] = True
partsDistributed (MkBinding PartD _ pl _ :: bs) =
  not (isOne pl) && partsDistributed bs
partsDistributed (_ :: bs) = partsDistributed bs

public export
restSource : Kind -> Bindings -> Maybe Binding
restSource k [] = Nothing
restSource k (b@(MkBinding PartD _ _ _) :: bs) =
  case restSource k bs of
    Just s => Just s
    Nothing => if kindLte k b.kind then Just b else Nothing
restSource k (b :: bs) = if objGroup k b then Just b else restSource k bs

public export
zoneOfGroup : Kind -> Bindings -> Maybe Zone
zoneOfGroup k bs = restSource k bs >>= bindingZone

public export
tyOfGroup : Kind -> Bindings -> Maybe CardType
tyOfGroup k bs = restSource k bs >>= bindingTy

public export
anyTargeted : Kind -> Bindings -> Bool
anyTargeted k [] = False
anyTargeted k (MkBinding TargetD k' _ _ :: bs) =
  if kindLte k k' then True else anyTargeted k bs
anyTargeted k (_ :: bs) = anyTargeted k bs

public export
anchorTyOk : CardType -> Maybe CardType -> Bool
anchorTyOk t Nothing = True
anchorTyOk t (Just t') = t == t'

public export
anyTargetedAt : Bindings -> Bool
anyTargetedAt [] = False
anyTargetedAt (MkBinding TargetD _ _ _ :: _) = True
anyTargetedAt (_ :: bs) = anyTargetedAt bs


public export
settleTargets : Bindings -> Bindings
settleTargets [] = []
settleTargets (MkBinding TargetD k plur payload :: bs) =
  MkBinding TheD k plur payload :: settleTargets bs
settleTargets (b :: bs) = b :: settleTargets bs

public export
outcomesOnly : Bindings -> Bindings
outcomesOnly [] = []
outcomesOnly (b@(MkBinding _ Outcome _ _) :: bs) = b :: outcomesOnly bs
outcomesOnly (_ :: bs) = outcomesOnly bs

public export
determinerIx : Determiner -> Nat
determinerIx TargetD = 0
determinerIx AD = 1
determinerIx EachD = 2
determinerIx AllD = 3
determinerIx TheD = 4
determinerIx PartD = 5
determinerIx CountD = 6
determinerIx SelfD = 7
determinerIx BareD = 8

public export
sameDet : Determiner -> Determiner -> Bool
sameDet a b = determinerIx a == determinerIx b

public export
Eq Determiner where
  (==) a b = determinerIx a == determinerIx b

public export
samePlur : Plurality -> Plurality -> Bool
samePlur OneOf OneOf = True
samePlur OneOf ManyOf = False
samePlur ManyOf OneOf = False
samePlur ManyOf ManyOf = True

public export
sameStamp : Stamp -> Stamp -> Bool
sameStamp (MkStamp v f m) (MkStamp w g n) = v == w && f == g && m == n

public export
sameOrigin : Origin -> Origin -> Bool
sameOrigin TokenOrigin TokenOrigin = True
sameOrigin TokenOrigin CopyOrigin = False
sameOrigin CopyOrigin TokenOrigin = False
sameOrigin CopyOrigin CopyOrigin = True

public export
sameMaybeBy : (a -> a -> Bool) -> Maybe a -> Maybe a -> Bool
sameMaybeBy f Nothing Nothing = True
sameMaybeBy f (Just x) (Just y) = f x y
sameMaybeBy f _ _ = False

public export
samePayload : {0 j, k : Kind} -> Payload j -> Payload k -> Bool
samePayload (ObjectP ty zn pv og _) (ObjectP ty' zn' pv' og' _) =
  sameMaybeBy (==) ty ty' && sameMaybeBy (==) zn zn' &&
  sameMaybeBy sameStamp pv pv' && sameMaybeBy sameOrigin og og'
samePayload (ObjectP _ _ _ _ _) _ = False
samePayload PlayerP PlayerP = True
samePayload ChosenPlayerP ChosenPlayerP = True
samePayload ChosenPlayerP _ = False
samePayload PlayerP _ = False
samePayload QualityP QualityP = True
samePayload QualityP _ = False
samePayload (OutcomeP s) (OutcomeP s') = s == s'
samePayload (OutcomeP _) _ = False
samePayload GapP GapP = True
samePayload GapP _ = False
samePayload LetterP LetterP = True
samePayload LetterP _ = False
samePayload TurnRefP TurnRefP = True
samePayload TurnRefP _ = False
samePayload (AbilityP og) (AbilityP og') = sameMaybeBy sameOrigin og og'
samePayload (AbilityP _) _ = False
samePayload (PileP zn _ _) (PileP zn' _ _) = sameMaybeBy (==) zn zn'
samePayload (PileP _ _ _) _ = False
samePayload (JoinP l r) (JoinP l' r') = samePayload l l' && samePayload r r'
samePayload (JoinP _ _) _ = False

public export
sameBinding : Binding -> Binding -> Bool
sameBinding (MkBinding d k p pl) (MkBinding d' k' p' pl') =
  sameDet d d' && k == k' && samePlur p p' && samePayload pl pl'

public export
sameBindings : Bindings -> Bindings -> Bool
sameBindings [] [] = True
sameBindings (b :: bs) (c :: cs) = sameBinding b c && sameBindings bs cs
sameBindings [] (_ :: _) = False
sameBindings (_ :: _) [] = False

public export
agreedField : (a -> a -> Bool) -> Maybe a -> Maybe a -> Maybe a
agreedField f (Just x) (Just y) = if f x y then Just x else Nothing
agreedField f _ _ = Nothing

public export
unionPayload : {j : Kind} -> {k : Kind} -> Payload j -> Payload k ->
               Maybe (m : Kind ** Payload m)
unionPayload (ObjectP t1 z1 v1 o1 s1) (ObjectP t2 z2 v2 o2 s2) =
  Just (Object ** ObjectP (agreedField (==) t1 t2) (agreedField (==) z1 z2)
                          (agreedField sameStamp v1 v2)
                          (agreedField sameOrigin o1 o2)
                          (agreedField (==) s1 s2))
unionPayload (AbilityP o1) (AbilityP o2) =
  Just (Object ** AbilityP (agreedField sameOrigin o1 o2))
unionPayload PlayerP PlayerP = Just (Player ** PlayerP)
unionPayload ChosenPlayerP ChosenPlayerP = Just (Player ** ChosenPlayerP)
unionPayload (ObjectP _ _ _ o1 _) (AbilityP o2) =
  Just (Object ** ObjectP Nothing (Just Stack) Nothing (agreedField sameOrigin o1 o2) Nothing)
unionPayload (AbilityP o1) (ObjectP _ _ _ o2 _) =
  Just (Object ** ObjectP Nothing (Just Stack) Nothing (agreedField sameOrigin o1 o2) Nothing)
unionPayload p@(ObjectP _ _ _ _ _) PlayerP = Just (Object \/ Player ** JoinP p PlayerP)
unionPayload PlayerP q@(ObjectP _ _ _ _ _) = Just (Object \/ Player ** JoinP q PlayerP)
unionPayload _ _ = Nothing

public export
unionBinding : Binding -> Binding -> Maybe Binding
unionBinding b c =
  if sameBinding b c then Just b
  else case (b, c) of
         (MkBinding d1 _ p1 pl1, MkBinding d2 _ p2 pl2) =>
           if sameDet d1 d2 && samePlur p1 p2
             then map (\(m ** pl) => MkBinding d1 m p1 pl) (unionPayload pl1 pl2)
             else Nothing

public export
unionBindings : Bindings -> Bindings -> Maybe Bindings
unionBindings [] [] = Just []
unionBindings (b :: bs) (c :: cs) =
  case (unionBinding b c, unionBindings bs cs) of
    (Just u, Just us) => Just (u :: us)
    _ => Nothing
unionBindings [] (_ :: _) = Nothing
unionBindings (_ :: _) [] = Nothing

public export
publicZone : Zone -> Bool
publicZone Battlefield = True
publicZone Graveyard = True
publicZone Exile = True
publicZone Hand = False
publicZone Library = False
publicZone Stack = True
publicZone Command = True

public export
data ExposeVerb = LookAt | Reveal

public export
exposableZone : Zone -> Bool
exposableZone Hand = True
exposableZone Battlefield = False
exposableZone Graveyard = False
exposableZone Exile = False
exposableZone Library = True
exposableZone Stack = False
exposableZone Command = False

public export
ExposableZone : Zone -> Type
ExposableZone z = So (exposableZone z)


public export
pubB : Binding -> Bool
pubB (MkBinding _ _ _ (ObjectP _ (Just z) _ _ _)) = publicZone z
pubB (MkBinding _ _ _ (ObjectP _ Nothing _ _ _)) = True
pubB (MkBinding _ _ _ PlayerP) = True
pubB (MkBinding _ _ _ ChosenPlayerP) = True
pubB (MkBinding _ _ _ QualityP) = True
pubB (MkBinding _ _ _ (OutcomeP _)) = True
pubB (MkBinding _ _ _ GapP) = True
pubB (MkBinding _ _ _ LetterP) = True
pubB (MkBinding _ _ _ TurnRefP) = True
pubB (MkBinding _ _ _ (AbilityP _)) = True
pubB (MkBinding _ _ _ (PileP _ _ (Just FaceDownPile))) = False
pubB (MkBinding _ _ _ (PileP (Just z) _ _)) = publicZone z
pubB (MkBinding _ _ _ (PileP Nothing _ _)) = True
pubB (MkBinding _ _ _ (JoinP _ _)) = True

public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) = if pubB b then b :: publicOnly bs else publicOnly bs


public export
pluralizeBinding : Binding -> Binding
pluralizeBinding (MkBinding SelfD k pl p) = MkBinding SelfD k pl p
pluralizeBinding (MkBinding TargetD k pl p) = MkBinding TargetD k ManyOf p
pluralizeBinding (MkBinding AD k pl p) = MkBinding AD k ManyOf p
pluralizeBinding (MkBinding EachD k pl p) = MkBinding EachD k ManyOf p
pluralizeBinding (MkBinding AllD k pl p) = MkBinding AllD k ManyOf p
pluralizeBinding (MkBinding TheD k pl p) = MkBinding TheD k ManyOf p
pluralizeBinding (MkBinding PartD k pl p) = MkBinding PartD k ManyOf p
pluralizeBinding (MkBinding CountD k pl p) = MkBinding CountD k ManyOf p
pluralizeBinding (MkBinding BareD k pl p) = MkBinding BareD k ManyOf p

public export
pluralizeDelta : Bindings -> Bindings
pluralizeDelta [] = []
pluralizeDelta (b :: bs) = pluralizeBinding b :: pluralizeDelta bs


public export
stampMoves : Maybe Stamp -> Bool
stampMoves Nothing = False
stampMoves (Just (MkStamp _ _ mv)) = mv

public export
stampWasField : Maybe Stamp -> Bool
stampWasField Nothing = False
stampWasField (Just (MkStamp _ wasF _)) = wasF

public export
payloadProv : Payload k -> Maybe Stamp
payloadProv (ObjectP _ _ pv _ _) = pv
payloadProv PlayerP = Nothing
payloadProv ChosenPlayerP = Nothing
payloadProv QualityP = Nothing
payloadProv (OutcomeP _) = Nothing
payloadProv GapP = Nothing
payloadProv LetterP = Nothing
payloadProv TurnRefP = Nothing
payloadProv (AbilityP _) = Nothing
payloadProv (PileP _ _ _) = Nothing
payloadProv (JoinP l r) = maybe (payloadProv r) Just (payloadProv l)

public export
payloadOrig : Payload k -> Maybe Origin
payloadOrig (ObjectP _ _ _ og _) = og
payloadOrig PlayerP = Nothing
payloadOrig ChosenPlayerP = Nothing
payloadOrig QualityP = Nothing
payloadOrig (OutcomeP _) = Nothing
payloadOrig GapP = Nothing
payloadOrig LetterP = Nothing
payloadOrig TurnRefP = Nothing
payloadOrig (AbilityP og) = og
payloadOrig (PileP _ _ _) = Nothing
payloadOrig (JoinP l r) = maybe (payloadOrig r) Just (payloadOrig l)

public export
data NounWord = TypeW CardType | CardW | SpellW | PlayerW
              | PermanentW | TokenW | CopyW | JoinW
              | ||| "Counter target spell or ability. ... THAT SPELL OR
                StackW
              | ||| "Whenever you activate an ability, ... copy THAT
                AbilityW
              | ||| "Copy target triggered ability you control. You may
                AbilityCopyW
              | ||| "the exiled creature card", "that land card": the type
                TypedCardW CardType
              | ||| "Put THAT PILE into your hand and the other into your
                PileW

public export
data VerbedMarking = Attributive | ThisWay


public export
tyIs : CardType -> Maybe CardType -> Bool
tyIs t Nothing = False
tyIs t (Just t') = t == t'

public export
isCardZone : Maybe Zone -> Bool
isCardZone Nothing = False
isCardZone (Just Battlefield) = False
isCardZone (Just Graveyard) = True
isCardZone (Just Exile) = True
isCardZone (Just Hand) = True
isCardZone (Just Library) = True
isCardZone (Just Stack) = False
isCardZone (Just Command) = True

public export
zoneIsB : Maybe Zone -> Zone -> Bool
zoneIsB Nothing _ = False
zoneIsB (Just a) b = a == b

public export
ZoneIs : Maybe Zone -> Zone -> Type
ZoneIs subj z = So (zoneIsB subj z)

public export
onFieldZone : Maybe Zone -> Bool
onFieldZone z = zoneIsB z Battlefield

public export
onStackZone : Maybe Zone -> Bool
onStackZone z = zoneIsB z Stack

public export
data SlotCarrier = PermanentSlot | CardSlot | SpellSlot

public export
data Reach = Bare | AtSlot SlotCarrier | Stamped VerbLabel | TokenBorn
           | Word NounWord | UnionHalf NounWord
           | Verbed VerbLabel NounWord VerbedMarking
           | ThatTurn

public export
slotZoneOk : SlotCarrier -> Maybe Zone -> Bool
slotZoneOk PermanentSlot zn = onFieldZone zn
slotZoneOk CardSlot zn = isCardZone zn
slotZoneOk SpellSlot zn = onStackZone zn

public export
mkStamp : Maybe VerbLabel -> (oldZn : Maybe Zone) -> (moved : Bool) -> Maybe Stamp
mkStamp Nothing oldZn moved = Nothing
mkStamp (Just v) oldZn moved = Just (MkStamp v (onFieldZone oldZn) moved)

public export
halfReaches : NounWord -> Payload k -> Bool
halfReaches w (JoinP l r) = halfReaches w l || halfReaches w r
halfReaches (TypeW t) (ObjectP ty _ _ _ _) = tyIs t ty
halfReaches PermanentW (ObjectP ty _ _ _ _) = isNothing ty
halfReaches PlayerW PlayerP = True
halfReaches PlayerW ChosenPlayerP = True
halfReaches _ _ = False

public export
joinedPayload : Payload k -> Bool
joinedPayload (ObjectP _ _ _ _ _) = False
joinedPayload PlayerP = False
joinedPayload ChosenPlayerP = False
joinedPayload QualityP = False
joinedPayload (OutcomeP _) = False
joinedPayload GapP = False
joinedPayload LetterP = False
joinedPayload TurnRefP = False
joinedPayload (AbilityP _) = False
joinedPayload (PileP _ _ _) = False
joinedPayload (JoinP _ _) = True

public export
wordReaches : NounWord -> Binding -> Bool
wordReaches (TypeW t) (MkBinding _ _ _ (ObjectP ty zn _ _ _)) = onFieldZone zn && tyIs t ty
wordReaches (TypeW t) (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches (TypeW t) pl
wordReaches CardW (MkBinding _ _ _ (ObjectP _ zn _ _ _)) = isCardZone zn
wordReaches (TypedCardW t) (MkBinding _ _ _ (ObjectP ty zn _ _ _)) =
  isCardZone zn && tyIs t ty
wordReaches SpellW (MkBinding _ _ _ (ObjectP _ zn _ _ _)) = onStackZone zn
wordReaches PlayerW (MkBinding _ _ _ PlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ ChosenPlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PlayerW pl
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn pv _ _)) =
  onFieldZone zn || stampWasField pv
wordReaches PermanentW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PermanentW pl
wordReaches TokenW (MkBinding _ _ _ (ObjectP _ zn _ og _)) =
  onFieldZone zn && isTokenOrigin og
wordReaches CopyW (MkBinding _ _ _ (ObjectP _ zn _ og _)) = isCopyOrigin og
wordReaches AbilityW (MkBinding _ _ _ (AbilityP _)) = True
wordReaches AbilityCopyW (MkBinding _ _ _ (AbilityP og)) = isCopyOrigin og
wordReaches PileW (MkBinding _ _ _ (PileP _ _ _)) = True
wordReaches JoinW (MkBinding _ kd _ pl) = joinedPayload pl && kindLte Player kd
wordReaches StackW (MkBinding _ _ _ pl) = onStackZone (payloadZone pl)
wordReaches _ _ = False

public export
wordNow : NounWord -> Binding -> Bool
wordNow w b = case b.det of
                SelfD => False
                _ => wordReaches w b


public export
isPileW : NounWord -> Bool
isPileW PileW = True
isPileW _ = False

public export
kindOfW : NounWord -> Kind
kindOfW (TypeW _) = Object
kindOfW CardW = Object
kindOfW (TypedCardW _) = Object
kindOfW SpellW = Object
kindOfW PlayerW = Player
kindOfW PermanentW = Object
kindOfW TokenW = Object
kindOfW CopyW = Object
kindOfW JoinW = Object \/ Player
kindOfW StackW = Object
kindOfW AbilityW = Object
kindOfW AbilityCopyW = Object
kindOfW PileW = Pile

public export
reachKind : Reach -> Kind
reachKind Bare = Object
reachKind (AtSlot _) = Object
reachKind (Stamped _) = Object
reachKind TokenBorn = Object
reachKind (Word w) = kindOfW w
reachKind (UnionHalf w) = kindOfW w
reachKind (Verbed _ w _) = kindOfW w
reachKind ThatTurn = TurnRef

public export
stampedBy : VerbLabel -> Stamp -> Bool
stampedBy v (MkStamp v' _ _) = v == v'

public export
verbedWordOk : NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
verbedWordOk (TypeW t) (MkStamp _ wasF _) ty zn = wasF && tyIs t ty
verbedWordOk CardW st ty zn = isCardZone zn
verbedWordOk (TypedCardW t) st ty zn = isCardZone zn && tyIs t ty
verbedWordOk SpellW st ty zn = onStackZone zn
verbedWordOk PermanentW (MkStamp _ wasF _) ty zn = wasF
verbedWordOk _ _ _ _ = False

public export
stampIs : VerbLabel -> Maybe Stamp -> Bool
stampIs v Nothing = False
stampIs v (Just st) = stampedBy v st

public export
markTy : Maybe CardType -> Binding -> Binding
markTy ty (MkBinding det Object plur (ObjectP Nothing zn st og sz)) =
  MkBinding det Object plur (ObjectP ty zn st og sz)
markTy ty b = b

public export
markFirst : (Binding -> Bool) -> Maybe CardType -> Bindings -> Bindings
markFirst q ty [] = []
markFirst q ty (b :: bs) =
  if q b then markTy ty b :: bs else b :: markFirst q ty bs

public export
survivesShuffle : Binding -> Bool
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) (Just st) _ _)) =
  stampedBy "Search" st
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) Nothing _ _)) = False
survivesShuffle _ = True

public export
afterShuffle : Bindings -> Bindings
afterShuffle [] = []
afterShuffle (b :: bs) =
  if survivesShuffle b then b :: afterShuffle bs else afterShuffle bs

public export
stampWordOk : VerbLabel -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v w st ty zn = stampedBy v st && verbedWordOk w st ty zn

public export
reaches : Reach -> Plurality -> Binding -> Bool
reaches Bare pl b = kindLte Object b.kind && isOne pl == isOne b.plur
reaches (AtSlot sl) pl b =
  kindLte Object b.kind && isOne pl == isOne b.plur && slotZoneOk sl (bindingZone b)
reaches (Stamped v) pl b =
  kindLte Object b.kind && isOne pl == isOne b.plur && stampIs v (payloadProv b.payload)
reaches TokenBorn pl b =
  kindLte Object b.kind && isOne pl == isOne b.plur && isTokenOrigin (payloadOrig b.payload)
reaches (Word w) pl b = isOne pl == isOne b.plur && wordNow w b
reaches (UnionHalf w) pl b =
  isOne pl == isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
reaches (Verbed v w _) pl (MkBinding _ _ bpl (ObjectP ty zn (Just st) _ _)) =
  isOne pl == isOne bpl && stampWordOk v w st ty zn
reaches (Verbed _ _ _) _ _ = False
reaches ThatTurn pl b = kindLte TurnRef b.kind && isOne pl == isOne b.plur

public export
countReach : Reach -> Plurality -> Bindings -> Nat
countReach r pl [] = Z
countReach r pl (b :: bs) =
  if reaches r pl b then S (countReach r pl bs) else countReach r pl bs

public export
provOfReach : Reach -> Plurality -> Bindings -> Maybe Stamp
provOfReach r pl [] = Nothing
provOfReach r pl (b :: bs) =
  if reaches r pl b then payloadProv b.payload else provOfReach r pl bs

public export
zoneOfReach : Reach -> Plurality -> Bindings -> Maybe Zone
zoneOfReach r pl [] = Nothing
zoneOfReach r pl (b :: bs) =
  if reaches r pl b then bindingZone b else zoneOfReach r pl bs

public export
tyOfReach : Reach -> Plurality -> Bindings -> Maybe CardType
tyOfReach r pl [] = Nothing
tyOfReach r pl (b :: bs) =
  if reaches r pl b then bindingTy b else tyOfReach r pl bs

public export
faceOfReach : Reach -> Plurality -> Bindings -> Maybe PileFace
faceOfReach r pl [] = Nothing
faceOfReach r pl (b :: bs) =
  if reaches r pl b then bindingFace b else faceOfReach r pl bs

||| The stretch of the binding stack a read resolves in.
public export
data Window = Whole | Top Nat | Below Nat

public export
view : Window -> Bindings -> Bindings
view Whole bs = bs
view (Top n) bs = take n bs
view (Below n) bs = drop n bs

public export
overWindow : (Bindings -> Bindings) -> Window -> Bindings -> Bindings
overWindow f Whole bs = f bs
overWindow f (Top n) bs = f (take n bs) ++ drop n bs
overWindow f (Below n) bs = take n bs ++ f (drop n bs)

public export
countTokenSpecs : Bindings -> Nat
countTokenSpecs [] = Z
countTokenSpecs (MkBinding SelfD _ _ _ :: bs) = countTokenSpecs bs
countTokenSpecs (MkBinding _ _ _ (ObjectP _ _ _ og _) :: bs) =
  if isTokenOrigin og then S (countTokenSpecs bs) else countTokenSpecs bs
countTokenSpecs (_ :: bs) = countTokenSpecs bs

public export
provOfGroup : Kind -> Bindings -> Maybe Stamp
provOfGroup k bs = restSource k bs >>= (\b => payloadProv b.payload)

public export
tyOfThoseAny : NounWord -> Bindings -> Maybe CardType
tyOfThoseAny w [] = Nothing
tyOfThoseAny w (b :: bs) =
  if wordNow w b then bindingTy b else tyOfThoseAny w bs

public export
countChoosers : Bindings -> Nat
countChoosers bs = countOnes Player bs + countManys Player bs

public export
data EntryCounterMark = Fresh | Additional | Fewer

public export
entryCounterMarkIx : EntryCounterMark -> Nat
entryCounterMarkIx Fresh = 0
entryCounterMarkIx Additional = 1
entryCounterMarkIx Fewer = 2

public export
Eq EntryCounterMark where
  (==) a b = entryCounterMarkIx a == entryCounterMarkIx b

public export
data PlayerGroupWord = AllPlayers | YourOpponents | YourTeam

public export
playerGroupWordIx : PlayerGroupWord -> Nat
playerGroupWordIx AllPlayers = 0
playerGroupWordIx YourOpponents = 1
playerGroupWordIx YourTeam = 2

public export
Eq PlayerGroupWord where
  (==) a b = playerGroupWordIx a == playerGroupWordIx b

public export
data RoundMode = RoundUp | RoundDown

public export
roundModeIx : RoundMode -> Nat
roundModeIx RoundUp = 0
roundModeIx RoundDown = 1

public export
Eq RoundMode where
  (==) a b = roundModeIx a == roundModeIx b

public export
data ScaleFactor = Doubled | Tripled

public export
scaleFactorIx : ScaleFactor -> Nat
scaleFactorIx Doubled = 0
scaleFactorIx Tripled = 1

public export
Eq ScaleFactor where
  (==) a b = scaleFactorIx a == scaleFactorIx b

public export
data ShiftDir = ShiftUp | ShiftDown

public export
shiftDirIx : ShiftDir -> Nat
shiftDirIx ShiftUp = 0
shiftDirIx ShiftDown = 1

public export
Eq ShiftDir where
  (==) a b = shiftDirIx a == shiftDirIx b


namespace Lookback
  public export
  data Lookback = ThisTurn | ThisCombat | LastTurn | ThisGame | ThisWay
                | Triggering

  public export
  sameWindow : Maybe Lookback -> Maybe Lookback -> Bool
  sameWindow Nothing Nothing = True
  sameWindow (Just ThisTurn) (Just ThisTurn) = True
  sameWindow (Just ThisCombat) (Just ThisCombat) = True
  sameWindow (Just LastTurn) (Just LastTurn) = True
  sameWindow (Just ThisGame) (Just ThisGame) = True
  sameWindow (Just ThisWay) (Just ThisWay) = True
  sameWindow (Just Triggering) (Just Triggering) = True
  sameWindow _ _ = False

public export
zoneFits : Maybe Zone -> Maybe Zone -> Bool
zoneFits _ Nothing = True
zoneFits Nothing (Just _) = True
zoneFits subj (Just b) = zoneIsB subj b

public export
ZoneFits : Maybe Zone -> Maybe Zone -> Type
ZoneFits subj desc = So (zoneFits subj desc)

public export
data Targetable : Kind -> Type where
  ObjectTgt : Targetable Object
  PlayerTgt : Targetable Player
  JoinTgt : Targetable a -> Targetable b -> Targetable (a \/ b)

public export
data Targeter : Kind -> Type where
  SpellTargets : Targeter Object
  EitherTargets : Targeter a -> Targeter b -> Targeter (a \/ b)

public export
data TargetExtent = SomeTarget | SoleTarget

public export
damageableType : CardType -> Bool
damageableType Creature = True
damageableType Planeswalker = True
damageableType Battle = True
damageableType _ = False

||| [] means unknown-therefore-permissive, at both levels.
public export
damageableHeadTysOk : List (List CardType) -> Bool
damageableHeadTysOk = all (all damageableType)

public export
data Phrasal : Kind -> Type where
  PhObject : Phrasal Object
  PhPlayer : Phrasal Player
  PhQuality : Phrasal (Quality q)
  PhJoin : Phrasal a -> Phrasal b -> Phrasal (a \/ b)

public export
targetablePhrasal : Targetable k -> Phrasal k
targetablePhrasal ObjectTgt = PhObject
targetablePhrasal PlayerTgt = PhPlayer
targetablePhrasal (JoinTgt l r) =
  PhJoin (targetablePhrasal l) (targetablePhrasal r)

public export
||| A copy of an ability is itself an ability [CR#707.10]: the source's
||| payload shape, not its kind, decides which object the copy is.
copyPayloadIn : {k : Kind} -> Phrasal k -> Bool -> Maybe CardType -> Maybe Zone ->
                Payload k
copyPayloadIn PhObject True _ _ = AbilityP (Just CopyOrigin)
copyPayloadIn PhObject False ty z = ObjectP ty z Nothing (Just CopyOrigin) Nothing
copyPayloadIn (PhJoin l r) ab ty z =
  JoinP (copyPayloadIn l ab ty z) (copyPayloadIn r ab ty z)
copyPayloadIn PhPlayer _ _ _ = PlayerP
copyPayloadIn {k = Quality q} PhQuality _ _ _ = QualityP

public export
data CopySort = FromStack | FromCardZone

public export
copyLandsIn : CopySort -> Maybe Zone -> Maybe Zone
copyLandsIn FromStack _ = Just Stack
copyLandsIn FromCardZone z = z

public export
copyPayload : {k : Kind} -> Phrasal k -> Bool -> Maybe CardType -> Payload k
copyPayload ph ab ty = copyPayloadIn ph ab ty (Just Stack)


public export
qualityParamKind : Kind -> Bool
qualityParamKind Object = True
qualityParamKind Player = True
qualityParamKind _ = False


public export
keywordFactsIn : KeywordLabel -> List KeywordFacts -> Maybe KeywordFacts
keywordFactsIn k [] = Nothing
keywordFactsIn k (f :: fs) = if word f == k then Just f else keywordFactsIn k fs

public export
keywordFactsFor : KeywordLabel -> Maybe KeywordFacts
keywordFactsFor k = keywordFactsIn k keywordFacts

public export
knownKeyword : KeywordLabel -> Bool
knownKeyword k = isJust (keywordFactsFor k)

public export
data KnownKeyword : KeywordLabel -> Type where
  KeywordInFactsTable : {0 k : KeywordLabel} ->
                        {auto 0 ok : knownKeyword k = True} -> KnownKeyword k

public export
keywordParamShapes : KeywordLabel -> List KeywordParamShape
keywordParamShapes k = maybe [] paramShapes (keywordFactsFor k)

public export
paramShapeCosts : KeywordParamShape -> Bool
paramShapeCosts CostParam = True
paramShapeCosts (CompoundParam _) = True
paramShapeCosts _ = False

public export
keywordCosts : KeywordLabel -> Bool
keywordCosts k = any paramShapeCosts (keywordParamShapes k)

public export
keywordBodied : KeywordLabel -> Bool
keywordBodied k = maybe False bodied (keywordFactsFor k)

public export
keywordParamless : KeywordLabel -> Bool
keywordParamless k =
  maybe False (\f => elem NoParam (paramShapes f)) (keywordFactsFor k)

public export
record KeywordFamily where
  constructor MkKeywordFamily
  familyWord : KeywordLabel
  familySort : Maybe QualitySort

public export
keywordFamilyIx : KeywordFamily -> (KeywordLabel, Maybe QualitySort)
keywordFamilyIx f = (familyWord f, familySort f)

public export
Eq KeywordFamily where
  (==) a b = keywordFamilyIx a == keywordFamilyIx b

public export
keywordFamilyOk : KeywordFamily -> Bool
keywordFamilyOk c = case keywordFactsFor (familyWord c) of
  Nothing => False
  Just f => case familySort c of
    Nothing => any (\s => not (s == NoParam)) (paramShapes f)
    Just _ => elem QualityParam (paramShapes f)

public export
KeywordFamilyOk : KeywordFamily -> Type
KeywordFamilyOk c = So (keywordFamilyOk c)

public export
data KeywordTerm : Type where
  TheKeyword : (k : KeywordLabel) -> KeywordTerm
  AnyKeywordIn : (c : KeywordFamily) -> KeywordTerm

public export
keywordTermIx : KeywordTerm -> Nat
keywordTermIx (TheKeyword _) = 0
keywordTermIx (AnyKeywordIn _) = 1

public export
sameKeywordTerm : KeywordTerm -> KeywordTerm -> Bool
sameKeywordTerm (TheKeyword a) (TheKeyword b) = a == b
sameKeywordTerm (AnyKeywordIn a) (AnyKeywordIn b) = a == b
sameKeywordTerm _ _ = True

public export
Eq KeywordTerm where
  (==) a b = keywordTermIx a == keywordTermIx b && sameKeywordTerm a b

public export
knownKeywordTerm : KeywordTerm -> Bool
knownKeywordTerm (TheKeyword k) = knownKeyword k
knownKeywordTerm (AnyKeywordIn c) = keywordFamilyOk c

public export
data KnownKeywordTerm : KeywordTerm -> Type where
  KeywordTermInFactsTable : {0 t : KeywordTerm} ->
                            {auto 0 ok : knownKeywordTerm t = True} ->
                            KnownKeywordTerm t

public export
keywordTermBare : KeywordTerm -> Bool
keywordTermBare (TheKeyword k) = keywordParamless k
keywordTermBare (AnyKeywordIn c) = keywordFamilyOk c

public export
keywordCounterOk : KeywordLabel -> Bool
keywordCounterOk k = maybe False counterEligible (keywordFactsFor k)

public export
KeywordCounterEligible : KeywordLabel -> Type
KeywordCounterEligible k = So (keywordCounterOk k)

public export
keywordStackRegime : KeywordLabel -> Maybe StackRegime
keywordStackRegime k = keywordFactsFor k >>= regime

public export
keywordFunctionsOnStack : KeywordLabel -> Bool
keywordFunctionsOnStack k = maybe False functionsOnStack (keywordFactsFor k)

public export
data PaidCostName : Type where
  ByKeyword : (kw : KeywordLabel) -> PaidCostName
  ByNthKeyword : (ord : Ordinal) -> (kw : KeywordLabel) -> PaidCostName
  TheAlternative : PaidCostName
  TheAdditional : PaidCostName

public export
paidCostNameIx : PaidCostName -> Nat
paidCostNameIx (ByKeyword _) = 0
paidCostNameIx (ByNthKeyword _ _) = 1
paidCostNameIx TheAlternative = 2
paidCostNameIx TheAdditional = 3

public export
samePaidCostName : PaidCostName -> PaidCostName -> Bool
samePaidCostName (ByKeyword a) (ByKeyword b) = a == b
samePaidCostName (ByNthKeyword (Nth m) a) (ByNthKeyword (Nth n) b) =
  m == n && a == b
samePaidCostName _ _ = True

public export
Eq PaidCostName where
  (==) a b = paidCostNameIx a == paidCostNameIx b && samePaidCostName a b

public export
paidCostNamed : PaidCostName -> Bool
paidCostNamed (ByKeyword kw) = maybe False paidCost (keywordFactsFor kw)
paidCostNamed (ByNthKeyword _ kw) = maybe False paidCost (keywordFactsFor kw)
paidCostNamed TheAlternative = True
paidCostNamed TheAdditional = True

public export
PaidCostNamed : PaidCostName -> Type
PaidCostNamed n = So (paidCostNamed n)

public export
data PaidFacet : Type where
  ColorsSpent : PaidFacet     -- colors of mana spent to cast it [CR#702.44a]
  ManaValueSpent : PaidFacet  -- mana spent to pay the total cost [CR#601.2h]
  TimesPaid : (which : PaidCostName) -> PaidFacet   -- [CR#702.33c..702.33d]
  PaidCostReadback : (which : PaidCostName) -> (window : Maybe Lookback) ->
                     PaidFacet                      -- "was kicked" [CR#702.33d]

public export
paidFacetNamed : PaidFacet -> Bool
paidFacetNamed ColorsSpent = True
paidFacetNamed ManaValueSpent = True
paidFacetNamed (TimesPaid which) = paidCostNamed which
paidFacetNamed (PaidCostReadback which _) = paidCostNamed which

public export
PaidFacetNamed : PaidFacet -> Type
PaidFacetNamed f = So (paidFacetNamed f)

public export
distinctKeywordWords : List KeywordFacts -> Bool
distinctKeywordWords [] = True
distinctKeywordWords (f :: fs) =
  not (elem (word f) (map word fs)) && distinctKeywordWords fs

export
keywordWordsDistinct : So (distinctKeywordWords Experimental.FactsGen.keywordFacts)
keywordWordsDistinct = Oh

public export
data AbilityClass : Type where
  AnyOnStack : AbilityClass
  AnyActivated : AbilityClass
  AnyTriggered : AbilityClass
  LoyaltyClass : AbilityClass
  KeywordClass : (k : KeywordLabel) -> {auto 0 kn : KnownKeyword k} ->
                 AbilityClass

public export
abilityClassIx : AbilityClass -> Nat
abilityClassIx AnyOnStack = 0
abilityClassIx AnyActivated = 1
abilityClassIx AnyTriggered = 2
abilityClassIx LoyaltyClass = 3
abilityClassIx (KeywordClass _) = 4

public export
sameAbilityClass : AbilityClass -> AbilityClass -> Bool
sameAbilityClass (KeywordClass a) (KeywordClass b) = a == b
sameAbilityClass _ _ = True

public export
Eq AbilityClass where
  (==) a b = abilityClassIx a == abilityClassIx b && sameAbilityClass a b

public export
distinctClasses : List AbilityClass -> Bool
distinctClasses [] = True
distinctClasses (c :: cs) = not (elem c cs) && distinctClasses cs


public export
-- ability words have no rules meaning [CR#207.2c], so the label is open
AbilityWordLabel : Type
AbilityWordLabel = String

public export
FlavorWordLabel : Type
FlavorWordLabel = String

public export
data ItalicWord = AnAbilityWord AbilityWordLabel
                | AFlavorWord FlavorWordLabel


namespace Chroma
  public export
  data Color = White | Blue | Black | Red | Green

  public export
  data ColorOrColorless = Colorless | OfColor Color

public export
colorIx : Color -> Nat
colorIx White = 0
colorIx Blue = 1
colorIx Black = 2
colorIx Red = 3
colorIx Green = 4

public export
Eq Color where
  (==) a b = colorIx a == colorIx b

public export
data SimpleManaSymbol = Generic Nat | Specific ColorOrColorless

public export
halvesDistinct : SimpleManaSymbol -> Color -> Bool
halvesDistinct (Generic _) d = True
halvesDistinct (Specific Colorless) d = True
halvesDistinct (Specific (OfColor c)) d = not (c == d)

public export
HalvesDistinct : SimpleManaSymbol -> Color -> Type
HalvesDistinct l r = So (halvesDistinct l r)

public export
phyrexianDistinct : Color -> Maybe Color -> Bool
phyrexianDistinct c Nothing = True
phyrexianDistinct c (Just d) = not (c == d)

public export
PhyrexianDistinct : Color -> Maybe Color -> Type
PhyrexianDistinct c d = So (phyrexianDistinct c d)

public export
data ManaSymbol : Type where
  Simple : SimpleManaSymbol -> ManaSymbol
  Hybrid : (l : SimpleManaSymbol) -> (r : Color) ->
           {auto 0 ds : HalvesDistinct l r} -> ManaSymbol
  Phyrexian : (c : Color) -> (d : Maybe Color) ->
                  {auto 0 ds : PhyrexianDistinct c d} -> ManaSymbol
  Variable : ManaSymbol
  SnowMana : ManaSymbol

public export
ManaCost : Type
ManaCost = List ManaSymbol

public export
manaHasX : ManaCost -> Bool
manaHasX [] = False
manaHasX (Variable :: _) = True
manaHasX (_ :: ms) = manaHasX ms

public export
data PayTimes = PaidOnce | AnyNumberOfTimes | UpToTimes Nat

public export
payTimesIx : PayTimes -> Nat
payTimesIx PaidOnce = 0
payTimesIx AnyNumberOfTimes = 1
payTimesIx (UpToTimes _) = 2

public export
samePayTimes : PayTimes -> PayTimes -> Bool
samePayTimes (UpToTimes m) (UpToTimes n) = m == n
samePayTimes _ _ = True

public export
Eq PayTimes where
  (==) a b = payTimesIx a == payTimesIx b && samePayTimes a b

public export
payRepeats : PayTimes -> Bool
payRepeats PaidOnce = False
payRepeats AnyNumberOfTimes = True
payRepeats (UpToTimes _) = True

public export
costLetters : Maybe ManaCost -> Bindings
costLetters Nothing = []
costLetters (Just c) = if manaHasX c then [letterB X] else []

public export
ManaRun : ManaCost -> Type
ManaRun c = NonEmpty c

public export
data ManaUnit : Type where
  GenericUnit : ManaUnit
  RunUnit : (run : ManaCost) -> {auto 0 wr : ManaRun run} -> ManaUnit

||| `PutCompanionIntoHand` is the {3} special action [CR#116.2g] a player takes
||| for the companion they revealed [CR#702.139a].
public export
data SpecialAction = TurnFaceUp | PutCompanionIntoHand | Foretell | UnlockDoor

public export
specialActionIx : SpecialAction -> Nat
specialActionIx TurnFaceUp = 0
specialActionIx PutCompanionIntoHand = 1
specialActionIx Foretell = 2
specialActionIx UnlockDoor = 3

public export
Eq SpecialAction where
  (==) a b = specialActionIx a == specialActionIx b

public export
data CostNamed : Type where
  Containing : (sym : ManaSymbol) -> CostNamed
  OfKeyword : (kw : KeywordLabel) -> CostNamed
  OfSpecialAction : (act : SpecialAction) -> CostNamed

public export
costNameable : CostNamed -> Bool
costNameable (Containing _) = True
costNameable (OfKeyword kw) = keywordCosts kw
costNameable (OfSpecialAction _) = True

public export
CostNameable : CostNamed -> Type
CostNameable c = So (costNameable c)


public export
ProducedRun : Type
ProducedRun = List ColorOrColorless

public export
runsNonEmpty : List ProducedRun -> Bool
runsNonEmpty [] = True
runsNonEmpty ([] :: _) = False
runsNonEmpty ((_ :: _) :: rs) = runsNonEmpty rs

public export
producedRunsWritten : List ProducedRun -> Bool
producedRunsWritten [] = False
producedRunsWritten rs@(_ :: _) = runsNonEmpty rs

public export
ProducedRuns : List ProducedRun -> Type
ProducedRuns rs = So (producedRunsWritten rs)

public export
data ColorFreedom = SameColor | EachColor | DistinctColors

public export
data ManaMatch = MatchAnyColor | MatchAnyType | MatchOf ColorOrColorless

public export
data LoyaltyCost : Type where
  LoyaltyUp : (n : Nat) -> LoyaltyCost
  LoyaltyDown : (n : Nat) -> LoyaltyCost
  LoyaltyDownX : LoyaltyCost
  LoyaltyZero : LoyaltyCost

public export
loyaltyAnnouncesX : LoyaltyCost -> Bool
loyaltyAnnouncesX (LoyaltyUp _) = False
loyaltyAnnouncesX (LoyaltyDown _) = False
loyaltyAnnouncesX LoyaltyDownX = True
loyaltyAnnouncesX LoyaltyZero = False

public export
data Subtype : Type where
  MkSubtype : (host : CardType) -> (label : String) -> Subtype
  -- instants and sorceries share their spell types [CR#205.3k]
  MkSpellSubtype : (label : String) -> Subtype

public export
subtypeType : Subtype -> Maybe CardType
subtypeType (MkSubtype host _) = Just host
subtypeType (MkSpellSubtype _) = Nothing

public export
subtypeFits : Subtype -> CardType -> Bool
subtypeFits (MkSubtype host _) t = host == t
subtypeFits (MkSpellSubtype _) t = t == Instant || t == Sorcery

public export
subtypeLabel : Subtype -> String
subtypeLabel (MkSubtype _ label) = label
subtypeLabel (MkSpellSubtype label) = label

public export
subtypeIx : Subtype -> (Maybe CardType, String)
subtypeIx (MkSubtype host label) = (Just host, label)
subtypeIx (MkSpellSubtype label) = (Nothing, label)

public export
Eq Subtype where
  (==) a b = subtypeIx a == subtypeIx b

public export
creatureType : String -> Subtype
creatureType n = MkSubtype Creature n

public export
landType : String -> Subtype
landType n = MkSubtype Land n

public export
artifactType : String -> Subtype
artifactType n = MkSubtype Artifact n

public export
enchantmentType : String -> Subtype
enchantmentType n = MkSubtype Enchantment n

public export
planeswalkerType : String -> Subtype
planeswalkerType n = MkSubtype Planeswalker n

public export
battleType : String -> Subtype
battleType n = MkSubtype Battle n

public export
spellType : String -> Subtype
spellType n = MkSpellSubtype n

public export
data BasicLandType : Subtype -> Type where
  PlainsBasic   : BasicLandType (landType "Plains")
  IslandBasic   : BasicLandType (landType "Island")
  SwampBasic    : BasicLandType (landType "Swamp")
  MountainBasic : BasicLandType (landType "Mountain")
  ForestBasic   : BasicLandType (landType "Forest")

public export
data TypeSpace = BasicLandSpace | LandSpace | CreatureSpace

public export
spaceHosted : TypeSpace -> Maybe CardType -> Bool
spaceHosted BasicLandSpace ty = tyIs Land ty
spaceHosted LandSpace ty = tyIs Land ty
spaceHosted CreatureSpace ty = tyIs Creature ty || tyIs Kindred ty

public export
SpaceHosted : TypeSpace -> Maybe CardType -> Type
SpaceHosted sp ty = So (spaceHosted sp ty)

public export
data BasicLandTypes : List Subtype -> Type where
  NoBasicTypes : BasicLandTypes []
  MoreBasicTypes : {0 s : Subtype} -> {0 ss : List Subtype} ->
                   BasicLandType s -> BasicLandTypes ss ->
                   BasicLandTypes (s :: ss)

public export
ascribesAsType : CardType -> Bool
ascribesAsType Creature = True
ascribesAsType Artifact = True
ascribesAsType Land = True
ascribesAsType Enchantment = True
ascribesAsType Planeswalker = True
ascribesAsType Battle = True
ascribesAsType Kindred = False
ascribesAsType Instant = False
ascribesAsType Sorcery = False

public export
data MarkerWord = TokenMarker | EmblemMarker | SpellMarker | PermanentMarker

public export
markerWordIx : MarkerWord -> Nat
markerWordIx TokenMarker = 0
markerWordIx EmblemMarker = 1
markerWordIx SpellMarker = 2
markerWordIx PermanentMarker = 3

public export
Eq MarkerWord where
  (==) a b = markerWordIx a == markerWordIx b

public export
grantorOrigin : MarkerWord -> Maybe Zone
grantorOrigin EmblemMarker = Just Command
grantorOrigin _ = Nothing

public export
markerZone : MarkerWord -> Zone
markerZone TokenMarker = Battlefield
markerZone EmblemMarker = Command
markerZone SpellMarker = Stack
markerZone PermanentMarker = Battlefield

public export
ascriptionOk : CardType -> Maybe Subtype -> Bool
ascriptionOk t Nothing = ascribesAsType t
ascriptionOk t (Just s) = ascribesAsType t && subtypeFits s t

namespace Counter
  public export
  data Delta : Type where
    Up : Nat -> Delta
    Down : Nat -> Delta

  public export
  deltaIx : Delta -> (Nat, Nat)
  deltaIx (Up n) = (0, n)
  deltaIx (Down n) = (1, n)

  public export
  Eq Delta where
    (==) a b = deltaIx a == deltaIx b

public export
data Supertype = Legendary | Basic | Snow | Ongoing | World

public export
supertypeIx : Supertype -> Nat
supertypeIx Legendary = 0
supertypeIx Basic = 1
supertypeIx Snow = 2
supertypeIx Ongoing = 3
supertypeIx World = 4

public export
Eq Supertype where
  (==) a b = supertypeIx a == supertypeIx b

public export
supersDistinct : List Supertype -> Bool
supersDistinct [] = True
supersDistinct (s :: ss) = not (elem s ss) && supersDistinct ss

public export
data Designation
  = -- PLAYER-held ([CR#725.1], [CR#726.1], [CR#702.131c], [CR#702.195b]).
    Monarch | TheInitiative | CitysBlessing | EnduringStory
  | -- OBJECT-held: the permanent markers ([CR#701.15b], [CR#701.54b],
    -- [CR#701.64b], [CR#716.2b], [CR#719.3b]).
    Goaded | RingBearer | Monstrous | Renowned | Suspected | Saddled
  | Prepared | Harnessed | Level | Solved
  | -- OBJECT-held, the three sector designations ([CR#702.158b]).
    AlphaSector | BetaSector | GammaSector
  | -- OBJECT-held, the unlocked pair ([CR#709.5c]).
    LeftHalfUnlocked | RightHalfUnlocked
  | -- CARD-held ([CR#903.3]).
    CommanderD
  | -- GAME-held ([CR#731.1]).
    Day | Night

public export
data DesignationScope = HeldBy Kind | HeldByCard | HeldByGame

public export
record DesignationFacts where
  constructor MkDesignationFacts
  designationFactScope : DesignationScope
  designationEffectful : Bool
  designationFactZone : Maybe Zone
  designationFactType : Maybe CardType

public export
designationFacts : Designation -> DesignationFacts
designationFacts Monarch = MkDesignationFacts (HeldBy Player) True Nothing Nothing
designationFacts TheInitiative = MkDesignationFacts (HeldBy Player) True Nothing Nothing
designationFacts CitysBlessing = MkDesignationFacts (HeldBy Player) True Nothing Nothing
designationFacts EnduringStory = MkDesignationFacts (HeldBy Player) True Nothing Nothing
designationFacts Goaded = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts RingBearer = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts Monstrous = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts Renowned = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts Suspected = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts Saddled = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts Prepared = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts Harnessed = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts Level = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts Solved = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts AlphaSector = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts BetaSector = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts GammaSector = MkDesignationFacts (HeldBy Object) True (Just Battlefield) (Just Creature)
designationFacts LeftHalfUnlocked = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts RightHalfUnlocked = MkDesignationFacts (HeldBy Object) True (Just Battlefield) Nothing
designationFacts CommanderD = MkDesignationFacts HeldByCard False Nothing Nothing
designationFacts Day = MkDesignationFacts HeldByGame True Nothing Nothing
designationFacts Night = MkDesignationFacts HeldByGame True Nothing Nothing

public export
designationScope : Designation -> DesignationScope
designationScope d = designationFactScope (designationFacts d)

public export
designationIx : Designation -> Nat
designationIx Monarch = 0
designationIx TheInitiative = 1
designationIx CitysBlessing = 2
designationIx EnduringStory = 3
designationIx Goaded = 4
designationIx RingBearer = 5
designationIx Monstrous = 6
designationIx Renowned = 7
designationIx Suspected = 8
designationIx Saddled = 9
designationIx Prepared = 10
designationIx Harnessed = 11
designationIx Level = 12
designationIx Solved = 13
designationIx AlphaSector = 14
designationIx BetaSector = 15
designationIx GammaSector = 16
designationIx LeftHalfUnlocked = 17
designationIx RightHalfUnlocked = 18
designationIx CommanderD = 19
designationIx Day = 20
designationIx Night = 21

public export
Eq Designation where
  (==) a b = designationIx a == designationIx b

public export
data RoomHalf = LeftHalf | RightHalf

public export
roomHalfIx : RoomHalf -> Nat
roomHalfIx LeftHalf = 0
roomHalfIx RightHalf = 1

public export
Eq RoomHalf where
  (==) a b = roomHalfIx a == roomHalfIx b

public export
halfDesignation : RoomHalf -> Designation
halfDesignation LeftHalf = LeftHalfUnlocked
halfDesignation RightHalf = RightHalfUnlocked

public export
designationHalf : Designation -> Maybe RoomHalf
designationHalf LeftHalfUnlocked = Just LeftHalf
designationHalf RightHalfUnlocked = Just RightHalf
designationHalf _ = Nothing

public export
data LockState = Locked | Unlocked

public export
lockStateIx : LockState -> Nat
lockStateIx Locked = 0
lockStateIx Unlocked = 1

public export
Eq LockState where
  (==) a b = lockStateIx a == lockStateIx b

public export
designationChecked : Designation -> Bool
designationChecked d = designationEffectful (designationFacts d)

public export
designationGiven : Designation -> Bool
designationGiven = designationChecked

public export
data ConferringWord = MonstrosityW | SaddleW | AscendW | StoriedW | RenownW

public export
conferredDesignation : ConferringWord -> Designation
conferredDesignation MonstrosityW = Monstrous
conferredDesignation SaddleW = Saddled
conferredDesignation AscendW = CitysBlessing
conferredDesignation StoriedW = EnduringStory
conferredDesignation RenownW = Renowned

public export
data GivingWarrant : Designation -> Type where
  Instructed : {auto 0 at : So (designationGiven d)} -> GivingWarrant d
  InExpansionOf : (w : ConferringWord) -> GivingWarrant (conferredDesignation w)

public export
designationSeedZone : Designation -> Maybe Zone
designationSeedZone d = designationFactZone (designationFacts d)

public export
designationSeedType : Designation -> Maybe CardType
designationSeedType d = designationFactType (designationFacts d)

public export
designationHolder : Designation -> Maybe Kind
designationHolder d = case designationScope d of
  HeldBy k => Just k
  HeldByCard => Just Object
  HeldByGame => Nothing

||| A possessive on a designation ("your Ring-bearer", "your commander") is
||| meaningful only where an object holds the designation for a player; a
||| player-held or game-wide designation has no possessor [CR#701.54e].
public export
designationPossessorOk : Designation -> Bool
designationPossessorOk d = designationHolder d == Just Object

public export
heldByItsCard : Designation -> Bool
heldByItsCard d = case designationScope d of
  HeldByCard => True
  _ => False

public export
data DesignationHolder : Designation -> Maybe Zone -> Type where
  HolderUnzoned : {auto 0 sc : designationScope d = HeldBy Player} ->
                  DesignationHolder d z
  HolderOnField : {auto 0 sc : designationScope d = HeldBy Object} ->
                  {auto 0 ok : ZoneIs z Battlefield} -> DesignationHolder d z

public export
data AttachWord = Enchanted | Equipped | Fortified

public export
attachWordIx : AttachWord -> Nat
attachWordIx Enchanted = 0
attachWordIx Equipped = 1
attachWordIx Fortified = 2

public export
Eq AttachWord where
  (==) a b = attachWordIx a == attachWordIx b

public export
attachHeadOk : AttachWord -> NounWord -> Bool
attachHeadOk Enchanted _ = True
attachHeadOk Equipped (TypeW Creature) = True
attachHeadOk Equipped PermanentW = True
attachHeadOk Fortified (TypeW Land) = True
attachHeadOk _ _ = False

public export
attachedCheckOk : AttachWord -> Bool
attachedCheckOk Enchanted = True
attachedCheckOk Equipped = True
attachedCheckOk Fortified = True

public export
AttachHeadOk : AttachWord -> NounWord -> Type
AttachHeadOk w h = So (attachHeadOk w h)

public export
attachHostZone : NounWord -> Maybe Zone
attachHostZone (TypeW _) = Just Battlefield
attachHostZone CardW = Just Battlefield
attachHostZone (TypedCardW _) = Just Battlefield
attachHostZone SpellW = Just Battlefield
attachHostZone PermanentW = Just Battlefield
attachHostZone TokenW = Just Battlefield
attachHostZone CopyW = Just Stack
attachHostZone _ = Nothing

public export
attachHostTy : NounWord -> Maybe CardType
attachHostTy (TypeW t) = Just t
attachHostTy (TypedCardW t) = Just t
attachHostTy _ = Nothing

public export
data OutcomeVerb = WinGame | LoseGame

public export
outcomeVerbIx : OutcomeVerb -> Nat
outcomeVerbIx WinGame = 0
outcomeVerbIx LoseGame = 1

public export
Eq OutcomeVerb where
  (==) a b = outcomeVerbIx a == outcomeVerbIx b

public export
data DefinedSlots = PowerAlone | ToughnessAlone | BothEach

public export
definesPower : DefinedSlots -> Bool
definesPower PowerAlone = True
definesPower ToughnessAlone = False
definesPower BothEach = True

public export
definesToughness : DefinedSlots -> Bool
definesToughness PowerAlone = False
definesToughness ToughnessAlone = True
definesToughness BothEach = True

public export
record CounterFacts where
  constructor MkCounterFacts
  counterLabel : String
  ||| What the counter is placed on: an object or a player [CR#122.1].
  counterHolder : Kind

public export
counterFacts : List CounterFacts
counterFacts =
  [ MkCounterFacts "Charge"       Object
  , MkCounterFacts "Time"         Object
  , MkCounterFacts "Lore"         Object
  , MkCounterFacts "Poison"       Player
  , MkCounterFacts "Age"          Object
  , MkCounterFacts "Stun"         Object
  , MkCounterFacts "Energy"       Player
  , MkCounterFacts "Oil"          Object
  , MkCounterFacts "Loyalty"      Object
  , MkCounterFacts "Quest"        Object
  , MkCounterFacts "Finality"     Object
  , MkCounterFacts "Shield"       Object
  , MkCounterFacts "Storage"      Object
  , MkCounterFacts "Fade"         Object
  , MkCounterFacts "Spore"        Object
  , MkCounterFacts "Experience"   Player
  , MkCounterFacts "Depletion"    Object
  , MkCounterFacts "Level"        Object
  , MkCounterFacts "Verse"        Object
  , MkCounterFacts "Rad"          Player
  , MkCounterFacts "Ki"           Object
  , MkCounterFacts "Divinity"     Object
  , MkCounterFacts "Study"        Object
  , MkCounterFacts "Plan"         Object
  , MkCounterFacts "Ice"          Object
  , MkCounterFacts "Doom"         Object
  , MkCounterFacts "Tide"         Object
  , MkCounterFacts "Soul"         Object
  , MkCounterFacts "Page"         Object
  , MkCounterFacts "Fuse"         Object
  , MkCounterFacts "Flood"        Object
  , MkCounterFacts "Bounty"       Object
  , MkCounterFacts "Strike"       Object
  , MkCounterFacts "Hour"         Object
  , MkCounterFacts "Growth"       Object
  , MkCounterFacts "Egg"          Object
  , MkCounterFacts "Dream"        Object
  , MkCounterFacts "Brick"        Object
  , MkCounterFacts "Blood"        Object
  , MkCounterFacts "Omen"         Object
  , MkCounterFacts "Luck"         Object
  , MkCounterFacts "Plague"       Object
  , MkCounterFacts "Slime"        Object
  , MkCounterFacts "Fungus"       Object
  , MkCounterFacts "Wish"         Object
  , MkCounterFacts "Wind"         Object
  , MkCounterFacts "Stash"        Object
  , MkCounterFacts "Scream"       Object
  , MkCounterFacts "Defense"      Object
  , MkCounterFacts "Hone"         Object
  , MkCounterFacts "Sleight"      Object
  , MkCounterFacts "Spite"        Object
  , MkCounterFacts "Rev"          Object
  , MkCounterFacts "Intervention" Object
  , MkCounterFacts "Suspect"      Object
  , MkCounterFacts "Bloodstain"   Object
  ]

public export
distinctCounterLabels : List CounterFacts -> Bool
distinctCounterLabels [] = True
distinctCounterLabels (f :: fs) =
  not (elem (counterLabel f) (map counterLabel fs)) && distinctCounterLabels fs

export
counterLabelsDistinct : So (distinctCounterLabels Experimental.Words.counterFacts)
counterLabelsDistinct = Oh

public export
counterFactsIn : String -> List CounterFacts -> Maybe CounterFacts
counterFactsIn l [] = Nothing
counterFactsIn l (f :: fs) =
  if counterLabel f == l then Just f else counterFactsIn l fs

public export
counterFactsFor : String -> Maybe CounterFacts
counterFactsFor l = counterFactsIn l counterFacts

public export
knownCounter : String -> Bool
knownCounter l = isJust (counterFactsFor l)

public export
data KnownCounter : String -> Type where
  CounterInFactsTable : {0 l : String} -> {auto 0 ok : knownCounter l = True} ->
                        KnownCounter l

public export
data CounterKind : Type where
  BoostCounter : Counter.Delta -> Counter.Delta -> CounterKind
  KeywordCounter : (k : KeywordLabel) ->
                   {auto 0 ok : KeywordCounterEligible k} -> CounterKind
  NamedCounter : (label : String) ->
          {auto 0 ok : KnownCounter label} -> CounterKind

public export
counterScope : CounterKind -> Kind
counterScope (BoostCounter _ _) = Object
counterScope (KeywordCounter _) = Object
counterScope (NamedCounter l) = maybe Object counterHolder (counterFactsFor l)

public export
sameCounterPayload : CounterKind -> CounterKind -> Bool
sameCounterPayload (BoostCounter ap at) (BoostCounter bp bt) = ap == bp && at == bt
sameCounterPayload (KeywordCounter a) (KeywordCounter b) = a == b
sameCounterPayload (NamedCounter a) (NamedCounter b) = a == b
sameCounterPayload _ _ = True

public export
counterKindIx : CounterKind -> Nat
counterKindIx (BoostCounter _ _) = 0
counterKindIx (KeywordCounter _) = 1
counterKindIx (NamedCounter _) = 2

public export
Eq CounterKind where
  (==) a b = counterKindIx a == counterKindIx b && sameCounterPayload a b

public export
data ProjAxis = StatAxis Stat | PlayerStatAxis PlayerStat
             | CounterAxis CounterKind | AnyCounterAxis Kind

public export
projScope : ProjAxis -> Kind
projScope (StatAxis _) = Object
projScope (PlayerStatAxis _) = Player
projScope (CounterAxis c) = counterScope c
projScope (AnyCounterAxis k) = k

public export
projAxisIx : ProjAxis -> Nat
projAxisIx (StatAxis _) = 0
projAxisIx (PlayerStatAxis _) = 1
projAxisIx (CounterAxis _) = 2
projAxisIx (AnyCounterAxis _) = 3

public export
sameProjAxis : ProjAxis -> ProjAxis -> Bool
sameProjAxis (StatAxis a) (StatAxis b) = a == b
sameProjAxis (PlayerStatAxis a) (PlayerStatAxis b) = a == b
sameProjAxis (CounterAxis a) (CounterAxis b) = a == b
sameProjAxis (AnyCounterAxis a) (AnyCounterAxis b) = a == b
sameProjAxis _ _ = True

public export
Eq ProjAxis where
  (==) a b = projAxisIx a == projAxisIx b && sameProjAxis a b


public export
axisType : ProjAxis -> Maybe CardType
axisType (StatAxis c) = comparedType c
axisType (PlayerStatAxis _) = Nothing
axisType (CounterAxis _) = Nothing
axisType (AnyCounterAxis _) = Nothing

public export
allAxisType : CardType -> List ProjAxis -> Bool
allAxisType t [] = True
allAxisType t (a :: as) = case axisType a of
  Nothing => False
  Just u => t == u && allAxisType t as

public export
axisTypes : List ProjAxis -> Maybe CardType
axisTypes [] = Nothing
axisTypes (a :: as) = case axisType a of
  Nothing => Nothing
  Just t => if allAxisType t as then Just t else Nothing

public export
sameAxes : List ProjAxis -> List ProjAxis -> Bool
sameAxes [] [] = True
sameAxes (a :: as) (b :: bs) = a == b && sameAxes as bs
sameAxes _ _ = False

public export
data AxesAt : Kind -> List ProjAxis -> Type where
  LastAxis : (0 a : ProjAxis) -> AxesAt (projScope a) [a]
  NextAxis : (0 a : ProjAxis) -> AxesAt (projScope a) (b :: as) ->
             AxesAt (projScope a) (a :: b :: as)

public export
CounterKindNamed : Kind -> Maybe CounterKind -> Type
CounterKindNamed k = OptOk (\c => counterScope c = k)

public export
data ChapterNumber = ChapterI | ChapterII | ChapterIII
                   | ChapterIV | ChapterV | ChapterVI

public export
chapterOrd : ChapterNumber -> Nat
chapterOrd ChapterI = 1
chapterOrd ChapterII = 2
chapterOrd ChapterIII = 3
chapterOrd ChapterIV = 4
chapterOrd ChapterV = 5
chapterOrd ChapterVI = 6

public export
chapterMarksDistinct : List ChapterNumber -> Bool
chapterMarksDistinct [] = True
chapterMarksDistinct (a :: rest) =
  not (any (\b => chapterOrd a == chapterOrd b) rest) && chapterMarksDistinct rest

public export
chapterMarksOk : List ChapterNumber -> Bool
chapterMarksOk [] = False
chapterMarksOk ns@(_ :: _) = chapterMarksDistinct ns

public export
ChapterMarks : List ChapterNumber -> Type
ChapterMarks ns = So (chapterMarksOk ns)

public export
record TypeLine where
  constructor MkTypeLine
  subs : List Subtype
  tys : List CardType

public export
lineNonEmpty : TypeLine -> Bool
lineNonEmpty (MkTypeLine [] []) = False
lineNonEmpty (MkTypeLine _ _) = True


public export
subsFitLine : List Subtype -> List CardType -> Bool
subsFitLine [] tys = True
subsFitLine (s :: ss) tys =
  (any (subtypeFits s) tys
     || (subtypeFits s Creature && elem Kindred tys))
  && subsFitLine ss tys



public export
permanentType : CardType -> Bool
permanentType Creature = True
permanentType Planeswalker = True
permanentType Battle = True
permanentType Kindred = False
permanentType Artifact = True
permanentType Land = True
permanentType Enchantment = True
permanentType Instant = False
permanentType Sorcery = False

public export
permanentSpellType : Maybe CardType -> Bool
permanentSpellType Nothing = False
permanentSpellType (Just Land) = False
permanentSpellType (Just t) = permanentType t

public export
spellCardType : CardType -> Bool
spellCardType Instant = True
spellCardType Sorcery = True
spellCardType Creature = False
spellCardType Artifact = False
spellCardType Land = False
spellCardType Enchantment = False
spellCardType Planeswalker = False
spellCardType Battle = False
spellCardType Kindred = False

public export
placeableTy : Maybe CardType -> Bool
placeableTy Nothing = True
placeableTy (Just t) = permanentType t

public export
destTypeOk : Maybe CardType -> Zone -> Bool
destTypeOk ty Battlefield = placeableTy ty
destTypeOk ty Graveyard = True
destTypeOk ty Exile = True
destTypeOk ty Hand = True
destTypeOk ty Library = True
destTypeOk ty Stack = True
destTypeOk ty Command = True

public export
Placeable : Maybe CardType -> Zone -> Type
Placeable ty z = So (destTypeOk ty z)

public export
data StatusCat = TapC | FlipC | FaceC | PhaseC

public export
data StatusVal : StatusCat -> Type where
  Tapped    : StatusVal TapC
  Untapped  : StatusVal TapC
  Flipped   : StatusVal FlipC
  Unflipped : StatusVal FlipC
  FaceUp    : StatusVal FaceC
  FaceDown  : StatusVal FaceC
  PhasedIn  : StatusVal PhaseC
  PhasedOut : StatusVal PhaseC

public export
sameStatusVal : {0 c, c' : StatusCat} -> StatusVal c -> StatusVal c' -> Bool
sameStatusVal Tapped Tapped = True
sameStatusVal Tapped _ = False
sameStatusVal Untapped Untapped = True
sameStatusVal Untapped _ = False
sameStatusVal Flipped Flipped = True
sameStatusVal Flipped _ = False
sameStatusVal Unflipped Unflipped = True
sameStatusVal Unflipped _ = False
sameStatusVal FaceUp FaceUp = True
sameStatusVal FaceUp _ = False
sameStatusVal FaceDown FaceDown = True
sameStatusVal FaceDown _ = False
sameStatusVal PhasedIn PhasedIn = True
sameStatusVal PhasedIn _ = False
sameStatusVal PhasedOut PhasedOut = True
sameStatusVal PhasedOut _ = False

public export
statusClash : {0 c, c' : StatusCat} -> StatusVal c -> StatusVal c' -> Bool
statusClash Tapped Untapped = True
statusClash Tapped _ = False
statusClash Untapped Tapped = True
statusClash Untapped _ = False
statusClash Flipped Unflipped = True
statusClash Flipped _ = False
statusClash Unflipped Flipped = True
statusClash Unflipped _ = False
statusClash FaceUp FaceDown = True
statusClash FaceUp _ = False
statusClash FaceDown FaceUp = True
statusClash FaceDown _ = False
statusClash PhasedIn PhasedOut = True
statusClash PhasedIn _ = False
statusClash PhasedOut PhasedIn = True
statusClash PhasedOut _ = False


public export
statusWordOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusWordOk Tapped = True
statusWordOk Untapped = True
statusWordOk Flipped = False
statusWordOk Unflipped = False
statusWordOk FaceUp = True
statusWordOk FaceDown = True
statusWordOk PhasedIn = False
statusWordOk PhasedOut = False

public export
StatusWord : StatusVal c -> Type
StatusWord v = So (statusWordOk v)

public export
statusEventOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusEventOk Tapped = True
statusEventOk Untapped = True
statusEventOk Flipped = True
statusEventOk Unflipped = False
statusEventOk FaceUp = True
statusEventOk FaceDown = True
statusEventOk PhasedIn = True
statusEventOk PhasedOut = True

public export
StatusEventVal : StatusVal c -> Type
StatusEventVal v = So (statusEventOk v)

public export
statusHeaderOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusHeaderOk FaceDown = True
statusHeaderOk v = statusEventOk v

public export
statusEffectOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusEffectOk Tapped = True
statusEffectOk Untapped = True
statusEffectOk Flipped = True
statusEffectOk Unflipped = False
statusEffectOk FaceUp = True
statusEffectOk FaceDown = True
statusEffectOk PhasedIn = True
statusEffectOk PhasedOut = True

public export
StatusEffectVal : StatusVal c -> Type
StatusEffectVal v = So (statusEffectOk v)

public export
colorsDistinct : List Color -> Bool
colorsDistinct [] = True
colorsDistinct (c :: cs) = not (elem c cs) && colorsDistinct cs

public export
ColorsDistinct : List Color -> Type
ColorsDistinct cs = So (colorsDistinct cs)

public export
data ColorSpec = SomeColors (List Color) | EveryColor

public export
colorSpecOk : ColorSpec -> Bool
colorSpecOk (SomeColors cs) = colorsDistinct cs
colorSpecOk EveryColor = True

public export
ColorSpecOk : ColorSpec -> Type
ColorSpecOk cs = So (colorSpecOk cs)

public export
typesDistinct : List CardType -> Bool
typesDistinct [] = True
typesDistinct (t :: ts) = not (elem t ts) && typesDistinct ts


public export
addedFits : Maybe CardType -> TypeLine -> Bool
addedFits subj (MkTypeLine [] tys) = True
addedFits subj (MkTypeLine (s :: ss) tys) =
  (any (subtypeFits s) tys ||
   (case subj of
      Nothing => False
      Just t => subtypeFits s t)) &&
  addedFits subj (MkTypeLine ss tys)

public export
AddedFits : Maybe CardType -> TypeLine -> Type
AddedFits subj tl = So (addedFits subj tl)

public export
anyNewType : Maybe CardType -> List CardType -> Bool
anyNewType subj [] = False
anyNewType subj (t :: ts) = not (tyIs t subj) || anyNewType subj ts

public export
addsSomething : Maybe CardType -> TypeLine -> Bool
addsSomething subj (MkTypeLine [] tys) = anyNewType subj tys
addsSomething subj (MkTypeLine (_ :: _) tys) = True

public export
AddsSomething : Maybe CardType -> TypeLine -> Type
AddsSomething subj tl = So (addsSomething subj tl)

public export
LineNonEmpty : TypeLine -> Type
LineNonEmpty tl = So (lineNonEmpty tl)

public export
retainable : CardType -> Bool
retainable Land = True
retainable Enchantment = True
retainable Creature = True
retainable Artifact = True
retainable Planeswalker = True
retainable Battle = True
retainable Kindred = True
retainable Instant = False
retainable Sorcery = False

public export
retentionOk : TypeLine -> Maybe CardType -> Bool
retentionOk tl Nothing = True
retentionOk (MkTypeLine _ []) (Just t) = False
retentionOk (MkTypeLine _ (u :: us)) (Just t) = retainable t

public export
RetentionOk : TypeLine -> Maybe CardType -> Type
RetentionOk tl ret = So (retentionOk tl ret)

public export
lastType : List CardType -> Maybe CardType
lastType [] = Nothing
lastType [t] = Just t
lastType (_ :: ts) = lastType ts


||| The combat phase's declare-attackers and declare-blockers steps [CR#508.1,509.1],
||| its combat damage step [CR#510.1] and the cleanup step [CR#514.1].
public export
data TurnPart = Turn | Upkeep | EndStep | Combat | UntapStep | EndOfCombat
              | FirstMain | PostcombatMain | DrawStep
              | MainPhase
              | BeginningPhase
              | DeclareAttackers | DeclareBlockers | CombatDamage | Cleanup

||| A "only before [part]" point names a phase or step inside the turn; a turn
||| is made of its phases [CR#500.1], so there is no point before the turn one
||| is already taking.
public export
beforePartOk : TurnPart -> Bool
beforePartOk Turn = False
beforePartOk _ = True

public export
data RankPeriod : Type where
  RankWithin : Lookback -> RankPeriod
  RankEach : TurnPart -> RankPeriod

public export
turnPartIx : TurnPart -> Nat
turnPartIx Turn = 0
turnPartIx Upkeep = 1
turnPartIx EndStep = 2
turnPartIx Combat = 3
turnPartIx UntapStep = 4
turnPartIx EndOfCombat = 5
turnPartIx FirstMain = 6
turnPartIx PostcombatMain = 7
turnPartIx DrawStep = 8
turnPartIx MainPhase = 9
turnPartIx BeginningPhase = 10
turnPartIx DeclareAttackers = 11
turnPartIx DeclareBlockers = 12
turnPartIx CombatDamage = 13
turnPartIx Cleanup = 14

public export
Eq TurnPart where
  (==) a b = turnPartIx a == turnPartIx b

||| Turn parts can repeat, so headers may distribute over them [CR#500.8,505.1a].
public export
data PartQuant = ThePart | EachPart

