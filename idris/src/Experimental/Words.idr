||| The lexical/taxonomy substrate: card types, zones, keywords, subtypes,
||| counters, statuses, bindings, payloads, mana, and designations.
module Experimental.Words

%default total


public export
data CardType = Creature | Artifact | Land | Enchantment | Instant | Sorcery
              | Planeswalker | Battle | Kindred

public export
combatant : CardType -> Bool
combatant Creature = True
combatant Planeswalker = False
combatant Battle = False
combatant Kindred = False
combatant Artifact = False
combatant Land = False
combatant Enchantment = False
combatant Instant = False
combatant Sorcery = False

public export
data FightParticipant : Maybe CardType -> Type where
  Fighter : {auto 0 ok : combatant t = True} -> FightParticipant (Just t)

public export
data Characteristic = Power | Toughness | ManaValue

public export
sameChar : Characteristic -> Characteristic -> Bool
sameChar Power Power = True
sameChar Power _ = False
sameChar Toughness Toughness = True
sameChar Toughness _ = False
sameChar ManaValue ManaValue = True
sameChar ManaValue _ = False

public export
data PlayerStat = LifeTotal | StartingLifeTotal

public export
samePlayerStat : PlayerStat -> PlayerStat -> Bool
samePlayerStat LifeTotal LifeTotal = True
samePlayerStat LifeTotal _ = False
samePlayerStat StartingLifeTotal StartingLifeTotal = True
samePlayerStat StartingLifeTotal _ = False

public export
data Comparator = AtLeast | AtMost | Greater | Less | Eq

public export
sameCmp : Comparator -> Comparator -> Bool
sameCmp AtLeast AtLeast = True
sameCmp AtLeast _ = False
sameCmp AtMost AtMost = True
sameCmp AtMost _ = False
sameCmp Greater Greater = True
sameCmp Greater _ = False
sameCmp Less Less = True
sameCmp Less _ = False
sameCmp Eq Eq = True
sameCmp Eq _ = False

public export
comparedType : Characteristic -> Maybe CardType
comparedType Power = Just Creature
comparedType Toughness = Just Creature
comparedType ManaValue = Nothing

public export
data QualitySort = Color | CreatureType | CardName | Number

public export
sameQ : QualitySort -> QualitySort -> Bool
sameQ Color Color = True
sameQ Color _ = False
sameQ CreatureType CreatureType = True
sameQ CreatureType _ = False
sameQ CardName CardName = True
sameQ CardName _ = False
sameQ Number Number = True
sameQ Number _ = False

||| Whether `OfChosen` honestly reads a chosen sort back: colours and
||| creature types take "of the chosen ...", card names and numbers use
||| their own later equality surfaces instead.
public export
chosenQualityReadOk : QualitySort -> Bool
chosenQualityReadOk Color = True
chosenQualityReadOk CreatureType = True
chosenQualityReadOk CardName = False
chosenQualityReadOk Number = False

public export
data ChosenQualityRead : QualitySort -> Type where
  MkChosenQualityRead : {auto 0 ok : chosenQualityReadOk q = True} ->
                        ChosenQualityRead q

public export
data LetterWord = LetterX | LetterY

public export
sameLetterWord : LetterWord -> LetterWord -> Bool
sameLetterWord LetterX LetterX = True
sameLetterWord LetterX LetterY = False
sameLetterWord LetterY LetterX = False
sameLetterWord LetterY LetterY = True

public export
data Kind = Object | Player | Quality QualitySort | Outcome | Gap
          | Letter LetterWord | TurnRef | Ability

public export
sameKind : Kind -> Kind -> Bool
sameKind Object Object = True
sameKind Object Player = False
sameKind Object (Quality _) = False
sameKind Object Outcome = False
sameKind Object Gap = False
sameKind Object (Letter _) = False
sameKind Object TurnRef = False
sameKind Object Ability = False
sameKind Player Object = False
sameKind Player Player = True
sameKind Player (Quality _) = False
sameKind Player Outcome = False
sameKind Player Gap = False
sameKind Player (Letter _) = False
sameKind Player TurnRef = False
sameKind Player Ability = False
sameKind (Quality _) Object = False
sameKind (Quality _) Player = False
sameKind (Quality a) (Quality b) = sameQ a b
sameKind (Quality _) Outcome = False
sameKind (Quality _) Gap = False
sameKind (Quality _) (Letter _) = False
sameKind (Quality _) TurnRef = False
sameKind (Quality _) Ability = False
sameKind Outcome Object = False
sameKind Outcome Player = False
sameKind Outcome (Quality _) = False
sameKind Outcome Outcome = True
sameKind Outcome Gap = False
sameKind Outcome (Letter _) = False
sameKind Outcome TurnRef = False
sameKind Outcome Ability = False
sameKind Gap Object = False
sameKind Gap Player = False
sameKind Gap (Quality _) = False
sameKind Gap Outcome = False
sameKind Gap Gap = True
sameKind Gap (Letter _) = False
sameKind Gap TurnRef = False
sameKind Gap Ability = False
sameKind (Letter _) Object = False
sameKind (Letter _) Player = False
sameKind (Letter _) (Quality _) = False
sameKind (Letter _) Outcome = False
sameKind (Letter _) Gap = False
sameKind (Letter a) (Letter b) = sameLetterWord a b
sameKind (Letter _) TurnRef = False
sameKind (Letter _) Ability = False
sameKind TurnRef Object = False
sameKind TurnRef Player = False
sameKind TurnRef (Quality _) = False
sameKind TurnRef Outcome = False
sameKind TurnRef Gap = False
sameKind TurnRef (Letter _) = False
sameKind TurnRef TurnRef = True
sameKind TurnRef Ability = False
sameKind Ability Object = False
sameKind Ability Player = False
sameKind Ability (Quality _) = False
sameKind Ability Outcome = False
sameKind Ability Gap = False
sameKind Ability (Letter _) = False
sameKind Ability TurnRef = False
sameKind Ability Ability = True

public export
data AggregateOp = SumOf | MinOf | MaxOf

public export
isExtremal : AggregateOp -> Bool
isExtremal SumOf = False
isExtremal MinOf = True
isExtremal MaxOf = True

public export
data IsExtremal : AggregateOp -> Type where
  MkIsExtremal : {auto 0 ok : isExtremal op = True} -> IsExtremal op

public export
sameAggregateOp : AggregateOp -> AggregateOp -> Bool
sameAggregateOp SumOf SumOf = True
sameAggregateOp SumOf _ = False
sameAggregateOp MinOf MinOf = True
sameAggregateOp MinOf _ = False
sameAggregateOp MaxOf MaxOf = True
sameAggregateOp MaxOf _ = False

public export
data ProjAxis = CharAxis Characteristic | PlayerStatAxis PlayerStat

public export
projScope : ProjAxis -> Kind
projScope (CharAxis _) = Object
projScope (PlayerStatAxis _) = Player

public export
sameProjAxis : ProjAxis -> ProjAxis -> Bool
sameProjAxis (CharAxis a) (CharAxis b) = sameChar a b
sameProjAxis (CharAxis _) _ = False
sameProjAxis (PlayerStatAxis a) (PlayerStatAxis b) = samePlayerStat a b
sameProjAxis (PlayerStatAxis _) _ = False

public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost | CountersPut
                 | DamagePrevented

public export
data Causer = AnEffect

public export
sameCauser : Causer -> Causer -> Bool
sameCauser AnEffect AnEffect = True

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
data Quantity : Type where
  Range : Maybe Nat -> Maybe Nat -> Quantity

public export
data NonZeroQ : Quantity -> Type where
  UnboundedAbove : NonZeroQ (Range lo Nothing)
  MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))

public export
leNat : Nat -> Nat -> Bool
leNat Z _ = True
leNat (S _) Z = False
leNat (S a) (S b) = leNat a b

public export
quantWellFormed : Quantity -> Bool
quantWellFormed (Range Nothing _) = True
quantWellFormed (Range (Just Z) _) = False
quantWellFormed (Range (Just (S n)) Nothing) = True
quantWellFormed (Range (Just (S n)) (Just hi)) = leNat (S n) hi

public export
data WellFormedQ : Quantity -> Type where
  MkWellFormedQ : {auto 0 ok : quantWellFormed q = True} -> WellFormedQ q

public export
boundedIncrease : Quantity -> Bool
boundedIncrease (Range _ Nothing) = False
boundedIncrease (Range _ (Just _)) = True

public export
data BoundedIncrease : Quantity -> Type where
  MkBoundedIncrease : {auto 0 ok : boundedIncrease q = True} ->
                      BoundedIncrease q

public export
quantPlur : Quantity -> Plurality
quantPlur (Range _ (Just (S Z))) = OneOf
quantPlur (Range _ _) = ManyOf

public export
eqNat : Nat -> Nat -> Bool
eqNat a b = leNat a b && leNat b a

public export
modesFit : Quantity -> Nat -> Bool
modesFit (Range Nothing Nothing) n = True
modesFit (Range Nothing (Just hi)) n = leNat hi n
modesFit (Range (Just lo) Nothing) n = leNat lo n
-- [CR#700.2]: a modal spell offers a choice among its modes, so a
-- headcount that fixes the whole list instructs nothing.
modesFit (Range (Just lo) (Just hi)) n = leNat hi n && not (eqNat lo hi && eqNat hi n)

public export
data ModesFit : Quantity -> Nat -> Type where
  MkModesFit : {auto 0 ok : modesFit q n = True} -> ModesFit q n

public export
modalHead : Quantity -> Nat -> Bool
modalHead (Range Nothing Nothing) n = True
modalHead (Range Nothing (Just (S Z))) n = True
modalHead (Range Nothing (Just _)) n = False
modalHead (Range (Just (S Z)) Nothing) n = True
modalHead (Range (Just _) Nothing) n = False
modalHead (Range (Just lo) (Just hi)) n =
  (eqNat lo hi && leNat lo 3) || (eqNat lo 1 && eqNat hi n)

public export
data ModalHead : Quantity -> Nat -> Type where
  MkModalHead : {auto 0 ok : modalHead q n = True} -> ModalHead q n

public export
data AtLeastOne : Nat -> Type where
  OneUp : AtLeastOne (S n)

public export
data AtLeastTwo : Nat -> Type where
  TwoUp : AtLeastTwo (S (S n))

public export
data Determiner = TargetD | AD | EachD | AllD | TheD | PartD
                | CountD
                | SelfD

public export
data Zone = Battlefield | Graveyard | Exile | Hand | Library | Stack

public export
sameZone : Zone -> Zone -> Bool
sameZone Battlefield Battlefield = True
sameZone Battlefield _ = False
sameZone Graveyard Graveyard = True
sameZone Graveyard _ = False
sameZone Exile Exile = True
sameZone Exile _ = False
sameZone Hand Hand = True
sameZone Hand _ = False
sameZone Library Library = True
sameZone Library _ = False
sameZone Stack Stack = True
sameZone Stack _ = False

public export
data LibPos = OnTop | OnBottom

public export
data Arrangement = AnyOrder | RandomOrder

public export
arrangementFits : LibPos -> Maybe Arrangement -> Bool
arrangementFits _ Nothing = True
arrangementFits _ (Just AnyOrder) = True
arrangementFits OnBottom (Just RandomOrder) = True
-- [CR#401.4] already grants the owner any-order arrangement, so "in a
-- random order" only means something where the order is hidden; a
-- library's top is the next draw, not hidden.
arrangementFits OnTop (Just RandomOrder) = False

public export
data ArrangementFits : LibPos -> Maybe Arrangement -> Type where
  MkArrangementFits : {auto 0 ok : arrangementFits p o = True} -> ArrangementFits p o

public export
data LibOrdinal = Second | Third | Fourth | Fifth | Seventh

-- [CR#401.7] states the ordinal-position construction naming only the
-- top of a library, with the bottom as fallback when it is too short.
public export
ordinalFits : LibPos -> Maybe Arrangement -> Maybe LibOrdinal -> Bool
ordinalFits _ _ Nothing = True
ordinalFits OnTop Nothing (Just _) = True
ordinalFits OnTop (Just _) (Just _) = False
ordinalFits OnBottom _ (Just _) = False

public export
data OrdinalFits : LibPos -> Maybe Arrangement -> Maybe LibOrdinal -> Type where
  MkOrdinalFits : {auto 0 ok : ordinalFits p o n = True} -> OrdinalFits p o n

namespace Verb
  public export
  data VerbName = Destroy | Sacrifice | Exile | Discard | Mill | Scry | Surveil

public export
verbAgentive : VerbName -> Bool
verbAgentive Destroy = False
verbAgentive Sacrifice = True
verbAgentive Exile = False
verbAgentive Discard = True
verbAgentive Mill = True
verbAgentive Scry = True
verbAgentive Surveil = True

public export
sameVerb : VerbName -> VerbName -> Bool
sameVerb Scry Scry = True
sameVerb Scry _ = False
sameVerb Surveil Surveil = True
sameVerb Surveil _ = False
sameVerb Destroy Destroy = True
sameVerb Destroy _ = False
sameVerb Sacrifice Sacrifice = True
sameVerb Sacrifice _ = False
sameVerb Exile Exile = True
sameVerb Exile _ = False
sameVerb Discard Discard = True
sameVerb Discard _ = False
sameVerb Mill Mill = True
sameVerb Mill _ = False

public export
record Stamp where
  constructor MkStamp
  verb : VerbName
  wasField : Bool

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
data Payload : Kind -> Type where
  ObjectP : (ty : Maybe CardType) -> (zone : Maybe Zone) ->
            (prov : Maybe Stamp) -> (orig : Maybe Origin) -> Payload Object
  PlayerP : Payload Player
  QualityP : Payload (Quality q)
  OutcomeP : (sort : OutcomeSort) -> Payload Outcome
  GapP : Payload Gap
  LetterP : Payload (Letter w)
  TurnRefP : Payload TurnRef
  AbilityP : Payload Ability
  -- a union mention ("target permanent or player") may name a player, so
  -- it has no zone or card type to project like ObjectP does.
  UnionP : Payload Object

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
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ (ObjectP _ zn _ _)) = zn
bindingZone (MkBinding _ _ _ PlayerP) = Nothing
bindingZone (MkBinding _ _ _ QualityP) = Nothing
bindingZone (MkBinding _ _ _ (OutcomeP _)) = Nothing
bindingZone (MkBinding _ _ _ GapP) = Nothing
bindingZone (MkBinding _ _ _ LetterP) = Nothing
bindingZone (MkBinding _ _ _ TurnRefP) = Nothing
bindingZone (MkBinding _ _ _ AbilityP) = Nothing
bindingZone (MkBinding _ _ _ UnionP) = Nothing

public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ (ObjectP ty _ _ _)) = ty
bindingTy (MkBinding _ _ _ PlayerP) = Nothing
bindingTy (MkBinding _ _ _ QualityP) = Nothing
bindingTy (MkBinding _ _ _ (OutcomeP _)) = Nothing
bindingTy (MkBinding _ _ _ GapP) = Nothing
bindingTy (MkBinding _ _ _ LetterP) = Nothing
bindingTy (MkBinding _ _ _ TurnRefP) = Nothing
bindingTy (MkBinding _ _ _ AbilityP) = Nothing
bindingTy (MkBinding _ _ _ UnionP) = Nothing

public export
sameOutcomeSort : OutcomeSort -> OutcomeSort -> Bool
sameOutcomeSort DamageDealt DamageDealt = True
sameOutcomeSort DamageDealt _ = False
sameOutcomeSort LifeGained LifeGained = True
sameOutcomeSort LifeGained _ = False
sameOutcomeSort LifeLost LifeLost = True
sameOutcomeSort LifeLost _ = False
sameOutcomeSort CountersPut CountersPut = True
sameOutcomeSort CountersPut _ = False
sameOutcomeSort DamagePrevented DamagePrevented = True
sameOutcomeSort DamagePrevented _ = False

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
letterB : LetterWord -> Binding
letterB w = MkBinding TheD (Letter w) OneOf LetterP

public export
qualityB : QualitySort -> Binding
qualityB q = MkBinding AD (Quality q) OneOf QualityP

public export
sameCT : CardType -> CardType -> Bool
sameCT Creature Creature = True
sameCT Creature _ = False
sameCT Artifact Artifact = True
sameCT Artifact _ = False
sameCT Land Land = True
sameCT Land _ = False
sameCT Enchantment Enchantment = True
sameCT Enchantment _ = False
sameCT Instant Instant = True
sameCT Instant _ = False
sameCT Sorcery Sorcery = True
sameCT Sorcery _ = False
sameCT Planeswalker Planeswalker = True
sameCT Planeswalker _ = False
sameCT Battle Battle = True
sameCT Battle _ = False
sameCT Kindred Kindred = True
sameCT Kindred _ = False

public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes k (MkBinding _ k' OneOf _ :: bs) =
  if sameKind k k' then S (countOnes k bs) else countOnes k bs
countOnes k (_ :: bs) = countOnes k bs

public export
countOutcomes : OutcomeSort -> Bindings -> Nat
countOutcomes s [] = Z
countOutcomes s (MkBinding _ Outcome OneOf (OutcomeP s') :: bs) =
  if sameOutcomeSort s s' then S (countOutcomes s bs) else countOutcomes s bs
countOutcomes s (_ :: bs) = countOutcomes s bs

public export
countQuality : QualitySort -> Bindings -> Nat
countQuality q [] = Z
countQuality q (MkBinding _ k OneOf _ :: bs) =
  if sameKind (Quality q) k then S (countQuality q bs) else countQuality q bs
countQuality q (_ :: bs) = countQuality q bs

public export
data ChoiceStands : Nat -> Type where
  ChoiceMade : ChoiceStands (S n)

public export
countLetter : LetterWord -> Bindings -> Nat
countLetter w [] = Z
countLetter w (MkBinding _ k OneOf _ :: bs) =
  if sameKind (Letter w) k then S (countLetter w bs) else countLetter w bs
countLetter w (_ :: bs) = countLetter w bs

public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ :: bs) =
  if sameKind k k' then S (countManys k bs) else countManys k bs
countManys k (_ :: bs) = countManys k bs

public export
countGroups : Bindings -> Nat
countGroups [] = Z
countGroups (MkBinding PartD _ _ _ :: bs) = countGroups bs
countGroups (MkBinding _ Object ManyOf _ :: bs) = S (countGroups bs)
countGroups (_ :: bs) = countGroups bs

public export
countParts : Bindings -> Nat
countParts [] = Z
countParts (MkBinding PartD Object _ _ :: bs) = S (countParts bs)
countParts (_ :: bs) = countParts bs

public export
-- "the rest" presupposes one assembled group (nothing to be the rest OF
-- without it) and at least one part taken from it (nothing taken and the
-- text would have written "them").
theRestOk : Bindings -> Bool
theRestOk bs = eqNat (countGroups bs) 1 && not (eqNat (countParts bs) Z)

public export
groupSpent : Bindings -> Bindings
groupSpent [] = []
groupSpent (b@(MkBinding PartD _ _ _) :: bs) = b :: groupSpent bs
groupSpent (MkBinding _ Object ManyOf _ :: bs) = groupSpent bs
groupSpent (b :: bs) = b :: groupSpent bs

public export
zoneOfGroup : Bindings -> Maybe Zone
zoneOfGroup [] = Nothing
zoneOfGroup (MkBinding PartD _ _ _ :: bs) = zoneOfGroup bs
zoneOfGroup (MkBinding _ Object ManyOf (ObjectP _ zn _ _) :: bs) = zn
zoneOfGroup (MkBinding _ Object ManyOf UnionP :: bs) = Nothing
zoneOfGroup (_ :: bs) = zoneOfGroup bs

public export
tyOfGroup : Bindings -> Maybe CardType
tyOfGroup [] = Nothing
tyOfGroup (MkBinding PartD _ _ _ :: bs) = tyOfGroup bs
tyOfGroup (MkBinding _ Object ManyOf (ObjectP ty _ _ _) :: bs) = ty
tyOfGroup (MkBinding _ Object ManyOf UnionP :: bs) = Nothing
tyOfGroup (_ :: bs) = tyOfGroup bs

public export
anyTargeted : Kind -> Bindings -> Bool
anyTargeted k [] = False
anyTargeted k (MkBinding TargetD k' _ _ :: bs) =
  if sameKind k k' then True else anyTargeted k bs
anyTargeted k (_ :: bs) = anyTargeted k bs

public export
anchorTyOk : CardType -> Maybe CardType -> Bool
anchorTyOk t Nothing = True
anchorTyOk t (Just t') = sameCT t t'

public export
anyTargetedAt : Bindings -> Bool
anyTargetedAt [] = False
anyTargetedAt (MkBinding TargetD _ _ _ :: _) = True
anyTargetedAt (_ :: bs) = anyTargetedAt bs

public export
anyTargetedTy : Kind -> CardType -> Bindings -> Bool
anyTargetedTy k t [] = False
anyTargetedTy k t (b@(MkBinding TargetD k' _ _) :: bs) =
  if sameKind k k' && anchorTyOk t (bindingTy b) then True else anyTargetedTy k t bs
anyTargetedTy k t (_ :: bs) = anyTargetedTy k t bs

public export
anchorFoundSome : Kind -> List CardType -> Bindings -> Bool
anchorFoundSome k [] ctx = False
anchorFoundSome k (t :: ts) ctx = anyTargetedTy k t ctx || anchorFoundSome k ts ctx

public export
anchorFound : Kind -> List CardType -> Bindings -> Bool
anchorFound k [] ctx = anyTargeted k ctx
anchorFound k (t :: ts) ctx = anchorFoundSome k (t :: ts) ctx

public export
settleTargets : Bindings -> Bindings
settleTargets [] = []
settleTargets (MkBinding TargetD k plur payload :: bs) =
  MkBinding TheD k plur payload :: settleTargets bs
settleTargets (b :: bs) = b :: settleTargets bs

public export
publicZone : Zone -> Bool
publicZone Battlefield = True
publicZone Graveyard = True
publicZone Exile = True
publicZone Hand = False
publicZone Library = False
publicZone Stack = True

public export
data ExposeVerb = LookAt | Reveal

-- [CR#400.2]: battlefield, graveyard and exile are already public zones,
-- so revealing them says nothing; the library is hidden but too large.
public export
exposableZone : Zone -> Bool
exposableZone Hand = True
exposableZone Battlefield = False
exposableZone Graveyard = False
exposableZone Exile = False
exposableZone Library = False
exposableZone Stack = False

public export
data ExposableZone : Zone -> Type where
  MkExposableZone : {auto 0 ok : exposableZone z = True} -> ExposableZone z

public export
data VisibleThing = TopOfLibrary | WholeHand

public export
visibilityOk : ExposeVerb -> VisibleThing -> Bool
visibilityOk Reveal TopOfLibrary = True
visibilityOk Reveal WholeHand = True
visibilityOk LookAt TopOfLibrary = True
-- [CR#402.3] already gives a player leave to look at their own hand any
-- time; "look at [subj]'s hand" can only agree with its own subject, so
-- the sentence would just restate the rule.
visibilityOk LookAt WholeHand = False

public export
data VisibilityOk : ExposeVerb -> VisibleThing -> Type where
  MkVisibilityOk : {auto 0 ok : visibilityOk v w = True} -> VisibilityOk v w

public export
searchableZone : Zone -> Bool
searchableZone Library = True
searchableZone Graveyard = True
searchableZone Hand = True
searchableZone Battlefield = False
searchableZone Exile = False
searchableZone Stack = False

public export
data SearchableZone : Zone -> Type where
  MkSearchableZone : {auto 0 ok : searchableZone z = True} -> SearchableZone z

public export
pubB : Binding -> Bool
pubB (MkBinding _ _ _ (ObjectP _ (Just z) _ _)) = publicZone z
pubB (MkBinding _ _ _ (ObjectP _ Nothing _ _)) = True
pubB (MkBinding _ _ _ PlayerP) = True
pubB (MkBinding _ _ _ QualityP) = True
pubB (MkBinding _ _ _ (OutcomeP _)) = True  -- what happened is a public fact
pubB (MkBinding _ _ _ GapP) = True          -- so is a comparison's margin
pubB (MkBinding _ _ _ LetterP) = True       -- and so is a value the text defines
pubB (MkBinding _ _ _ TurnRefP) = True       -- and so is a value the text defines
pubB (MkBinding _ _ _ AbilityP) = True       -- an ability class is public too
pubB (MkBinding _ _ _ UnionP) = True         -- a target is public whichever half it is

public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) = if pubB b then b :: publicOnly bs else publicOnly bs

public export
notInLibrary : Binding -> Bool
notInLibrary (MkBinding _ _ _ (ObjectP _ (Just z) _ _)) = not (sameZone z Library)
notInLibrary (MkBinding _ _ _ (ObjectP _ Nothing _ _)) = True
notInLibrary (MkBinding _ _ _ PlayerP) = True
notInLibrary (MkBinding _ _ _ QualityP) = True
notInLibrary (MkBinding _ _ _ (OutcomeP _)) = True
notInLibrary (MkBinding _ _ _ GapP) = True
notInLibrary (MkBinding _ _ _ LetterP) = True
notInLibrary (MkBinding _ _ _ TurnRefP) = True
notInLibrary (MkBinding _ _ _ AbilityP) = True
notInLibrary (MkBinding _ _ _ UnionP) = True

public export
shuffledAway : Bindings -> Bindings
shuffledAway [] = []
shuffledAway (b :: bs) =
  if notInLibrary b then b :: shuffledAway bs else shuffledAway bs

public export
zoneOfIt : Bindings -> Maybe Zone
zoneOfIt [] = Nothing
zoneOfIt (MkBinding det Object OneOf (ObjectP ty zn _ _) :: bs) = zn
zoneOfIt (MkBinding det Object OneOf UnionP :: bs) = Nothing
zoneOfIt (b :: bs) = zoneOfIt bs

public export
zoneOfThem : Bindings -> Maybe Zone
zoneOfThem [] = Nothing
zoneOfThem (MkBinding det Object ManyOf (ObjectP ty zn _ _) :: bs) = zn
zoneOfThem (MkBinding det Object ManyOf UnionP :: bs) = Nothing
zoneOfThem (b :: bs) = zoneOfThem bs

public export
data NounWord = TypeW CardType | CardW | SpellW | PlayerW
              | PermanentW | TokenW | CopyW | JoinW

public export
data VerbedMarking = Attributive | ThisWay

public export
verbedMarkingOk : VerbName -> VerbedMarking -> Bool
verbedMarkingOk Destroy Attributive = False
verbedMarkingOk Destroy ThisWay = True
verbedMarkingOk Sacrifice Attributive = True
verbedMarkingOk Sacrifice ThisWay = True
verbedMarkingOk Exile Attributive = True
verbedMarkingOk Exile ThisWay = True
verbedMarkingOk Discard Attributive = True
verbedMarkingOk Discard ThisWay = True
verbedMarkingOk Mill Attributive = True
verbedMarkingOk Mill ThisWay = True
verbedMarkingOk Scry Attributive = False
verbedMarkingOk Scry ThisWay = False
verbedMarkingOk Surveil Attributive = False
verbedMarkingOk Surveil ThisWay = False

public export
data VerbedMarkingOk : VerbName -> VerbedMarking -> Type where
  MkVerbedMarkingOk : {auto 0 ok : verbedMarkingOk v m = True} -> VerbedMarkingOk v m

public export
tyIs : CardType -> Maybe CardType -> Bool
tyIs t Nothing = False
tyIs t (Just t') = sameCT t t'

public export
isCardZone : Maybe Zone -> Bool
isCardZone Nothing = False
isCardZone (Just Battlefield) = False
isCardZone (Just Graveyard) = True
isCardZone (Just Exile) = True
isCardZone (Just Hand) = True
isCardZone (Just Library) = True
isCardZone (Just Stack) = False

public export
onFieldZone : Maybe Zone -> Bool
onFieldZone Nothing = False
onFieldZone (Just Battlefield) = True
onFieldZone (Just Graveyard) = False
onFieldZone (Just Exile) = False
onFieldZone (Just Hand) = False
onFieldZone (Just Library) = False
onFieldZone (Just Stack) = False

public export
onStackZone : Maybe Zone -> Bool
onStackZone Nothing = False
onStackZone (Just Battlefield) = False
onStackZone (Just Graveyard) = False
onStackZone (Just Exile) = False
onStackZone (Just Hand) = False
onStackZone (Just Library) = False
onStackZone (Just Stack) = True

public export
mkStamp : Maybe VerbName -> Maybe Zone -> Maybe Stamp
mkStamp Nothing oldZn = Nothing
mkStamp (Just v) oldZn = Just (MkStamp v (onFieldZone oldZn))

public export
wordReaches : NounWord -> Binding -> Bool
wordReaches (TypeW t) (MkBinding _ _ _ (ObjectP ty zn _ _)) = onFieldZone zn && tyIs t ty
wordReaches (TypeW t) (MkBinding _ _ _ PlayerP) = False
wordReaches (TypeW t) (MkBinding _ _ _ QualityP) = False
wordReaches (TypeW t) (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches (TypeW t) (MkBinding _ _ _ GapP) = False
wordReaches (TypeW t) (MkBinding _ _ _ LetterP) = False
wordReaches (TypeW t) (MkBinding _ _ _ TurnRefP) = False
wordReaches (TypeW t) (MkBinding _ _ _ AbilityP) = False
wordReaches (TypeW t) (MkBinding _ _ _ UnionP) = False
wordReaches CardW (MkBinding _ _ _ (ObjectP _ zn _ _)) = isCardZone zn
wordReaches CardW (MkBinding _ _ _ PlayerP) = False
wordReaches CardW (MkBinding _ _ _ QualityP) = False
wordReaches CardW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CardW (MkBinding _ _ _ GapP) = False
wordReaches CardW (MkBinding _ _ _ LetterP) = False
wordReaches CardW (MkBinding _ _ _ TurnRefP) = False
wordReaches CardW (MkBinding _ _ _ AbilityP) = False
wordReaches CardW (MkBinding _ _ _ UnionP) = False
wordReaches SpellW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onStackZone zn && not (isCopyOrigin og)
wordReaches SpellW (MkBinding _ _ _ PlayerP) = False
wordReaches SpellW (MkBinding _ _ _ QualityP) = False
wordReaches SpellW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches SpellW (MkBinding _ _ _ GapP) = False
wordReaches SpellW (MkBinding _ _ _ LetterP) = False
wordReaches SpellW (MkBinding _ _ _ TurnRefP) = False
wordReaches SpellW (MkBinding _ _ _ AbilityP) = False
wordReaches SpellW (MkBinding _ _ _ UnionP) = False
wordReaches PlayerW (MkBinding _ _ _ (ObjectP _ _ _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ PlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ QualityP) = False
wordReaches PlayerW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PlayerW (MkBinding _ _ _ GapP) = False
wordReaches PlayerW (MkBinding _ _ _ LetterP) = False
wordReaches PlayerW (MkBinding _ _ _ TurnRefP) = False
wordReaches PlayerW (MkBinding _ _ _ AbilityP) = False
wordReaches PlayerW (MkBinding _ _ _ UnionP) = False
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn _ _)) = onFieldZone zn
wordReaches PermanentW (MkBinding _ _ _ PlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ QualityP) = False
wordReaches PermanentW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PermanentW (MkBinding _ _ _ GapP) = False
wordReaches PermanentW (MkBinding _ _ _ LetterP) = False
wordReaches PermanentW (MkBinding _ _ _ TurnRefP) = False
wordReaches PermanentW (MkBinding _ _ _ AbilityP) = False
wordReaches PermanentW (MkBinding _ _ _ UnionP) = False
wordReaches TokenW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onFieldZone zn && isTokenOrigin og
wordReaches TokenW (MkBinding _ _ _ PlayerP) = False
wordReaches TokenW (MkBinding _ _ _ QualityP) = False
wordReaches TokenW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches TokenW (MkBinding _ _ _ GapP) = False
wordReaches TokenW (MkBinding _ _ _ LetterP) = False
wordReaches TokenW (MkBinding _ _ _ TurnRefP) = False
wordReaches TokenW (MkBinding _ _ _ AbilityP) = False
wordReaches TokenW (MkBinding _ _ _ UnionP) = False
wordReaches CopyW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onStackZone zn && isCopyOrigin og
wordReaches CopyW (MkBinding _ _ _ PlayerP) = False
wordReaches CopyW (MkBinding _ _ _ QualityP) = False
wordReaches CopyW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CopyW (MkBinding _ _ _ GapP) = False
wordReaches CopyW (MkBinding _ _ _ LetterP) = False
wordReaches CopyW (MkBinding _ _ _ TurnRefP) = False
wordReaches CopyW (MkBinding _ _ _ AbilityP) = False
wordReaches CopyW (MkBinding _ _ _ UnionP) = False
wordReaches JoinW (MkBinding _ _ _ UnionP) = True
wordReaches JoinW (MkBinding _ _ _ (ObjectP _ _ _ _)) = False
wordReaches JoinW (MkBinding _ _ _ PlayerP) = False
wordReaches JoinW (MkBinding _ _ _ QualityP) = False
wordReaches JoinW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches JoinW (MkBinding _ _ _ GapP) = False
wordReaches JoinW (MkBinding _ _ _ LetterP) = False
wordReaches JoinW (MkBinding _ _ _ TurnRefP) = False
wordReaches JoinW (MkBinding _ _ _ AbilityP) = False

public export
wordNow : NounWord -> Binding -> Bool
wordNow w b = case b.det of
                -- the event subject's own self-mention: "it" reads it,
                -- the demonstrative noun words do not.
                SelfD => False
                _ => wordReaches w b


public export
kindOfW : NounWord -> Kind
kindOfW (TypeW _) = Object
kindOfW CardW = Object
kindOfW SpellW = Object
kindOfW PlayerW = Player
kindOfW PermanentW = Object
kindOfW TokenW = Object
kindOfW CopyW = Object
kindOfW JoinW = Object

public export
stampedBy : VerbName -> Stamp -> Bool
stampedBy v (MkStamp v' _) = sameVerb v v'

public export
verbedWordOk : NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
verbedWordOk (TypeW t) (MkStamp _ wasF) ty zn = wasF && tyIs t ty
verbedWordOk CardW st ty zn = isCardZone zn
verbedWordOk SpellW st ty zn = onStackZone zn
verbedWordOk PlayerW st ty zn = False
verbedWordOk PermanentW (MkStamp _ wasF) ty zn = wasF
verbedWordOk TokenW st ty zn = False
verbedWordOk CopyW st ty zn = False
verbedWordOk JoinW st ty zn = False

public export
stampWordOk : VerbName -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v w st ty zn = stampedBy v st && verbedWordOk w st ty zn

public export
verbedMatch : VerbName -> NounWord -> Binding -> Bool
verbedMatch v w (MkBinding _ _ OneOf (ObjectP ty zn (Just st) _)) = stampWordOk v w st ty zn
verbedMatch v w (MkBinding _ _ OneOf (ObjectP _ _ Nothing _)) = False
verbedMatch v w (MkBinding _ _ ManyOf (ObjectP _ _ _ _)) = False
verbedMatch v w (MkBinding _ _ _ PlayerP) = False
verbedMatch v w (MkBinding _ _ _ QualityP) = False
verbedMatch v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatch v w (MkBinding _ _ _ GapP) = False
verbedMatch v w (MkBinding _ _ _ LetterP) = False
verbedMatch v w (MkBinding _ _ _ TurnRefP) = False
verbedMatch v w (MkBinding _ _ _ AbilityP) = False
verbedMatch v w (MkBinding _ _ _ UnionP) = False

public export
verbedMatchMany : VerbName -> NounWord -> Binding -> Bool
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP ty zn (Just st) _)) = stampWordOk v w st ty zn
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP _ _ Nothing _)) = False
verbedMatchMany v w (MkBinding _ _ OneOf (ObjectP _ _ _ _)) = False
verbedMatchMany v w (MkBinding _ _ _ PlayerP) = False
verbedMatchMany v w (MkBinding _ _ _ QualityP) = False
verbedMatchMany v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatchMany v w (MkBinding _ _ _ GapP) = False
verbedMatchMany v w (MkBinding _ _ _ LetterP) = False
verbedMatchMany v w (MkBinding _ _ _ TurnRefP) = False
verbedMatchMany v w (MkBinding _ _ _ AbilityP) = False
verbedMatchMany v w (MkBinding _ _ _ UnionP) = False

public export
countVerbed : VerbName -> NounWord -> Bindings -> Nat
countVerbed v w [] = Z
countVerbed v w (b :: bs) =
  if verbedMatch v w b then S (countVerbed v w bs) else countVerbed v w bs

public export
countManyVerbed : VerbName -> NounWord -> Bindings -> Nat
countManyVerbed v w [] = Z
countManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then S (countManyVerbed v w bs) else countManyVerbed v w bs

public export
zoneOfManyVerbed : VerbName -> NounWord -> Bindings -> Maybe Zone
zoneOfManyVerbed v w [] = Nothing
zoneOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingZone b else zoneOfManyVerbed v w bs

public export
tyOfManyVerbed : VerbName -> NounWord -> Bindings -> Maybe CardType
tyOfManyVerbed v w [] = Nothing
tyOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingTy b else tyOfManyVerbed v w bs

public export
zoneOfVerbed : VerbName -> NounWord -> Bindings -> Maybe Zone
zoneOfVerbed v w [] = Nothing
zoneOfVerbed v w (b :: bs) =
  if verbedMatch v w b then bindingZone b else zoneOfVerbed v w bs

public export
countWord : NounWord -> Bindings -> Nat
countWord w [] = Z
countWord w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => S (countWord w bs)
    _ => countWord w bs

public export
countManyWord : NounWord -> Bindings -> Nat
countManyWord w [] = Z
countManyWord w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => S (countManyWord w bs)
    _ => countManyWord w bs

public export
zoneOfThat : NounWord -> Bindings -> Maybe Zone
zoneOfThat w [] = Nothing
zoneOfThat w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => bindingZone b
    _ => zoneOfThat w bs

public export
zoneOfThose : NounWord -> Bindings -> Maybe Zone
zoneOfThose w [] = Nothing
zoneOfThose w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => bindingZone b
    _ => zoneOfThose w bs

public export
tyOfIt : Bindings -> Maybe CardType
tyOfIt [] = Nothing
tyOfIt (MkBinding det Object OneOf (ObjectP ty zn pv _) :: bs) = ty
tyOfIt (MkBinding det Object OneOf UnionP :: bs) = Nothing
tyOfIt (b :: bs) = tyOfIt bs

public export
tyOfThem : Bindings -> Maybe CardType
tyOfThem [] = Nothing
tyOfThem (MkBinding det Object ManyOf (ObjectP ty zn pv _) :: bs) = ty
tyOfThem (MkBinding det Object ManyOf UnionP :: bs) = Nothing
tyOfThem (b :: bs) = tyOfThem bs

public export
tyOfThat : NounWord -> Bindings -> Maybe CardType
tyOfThat w [] = Nothing
tyOfThat w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => bindingTy b
    _ => tyOfThat w bs

public export
tyOfThose : NounWord -> Bindings -> Maybe CardType
tyOfThose w [] = Nothing
tyOfThose w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => bindingTy b
    _ => tyOfThose w bs

public export
tyOfVerbed : VerbName -> NounWord -> Bindings -> Maybe CardType
tyOfVerbed v w [] = Nothing
tyOfVerbed v w (b :: bs) =
  if verbedMatch v w b then bindingTy b else tyOfVerbed v w bs

public export
countChoosers : Bindings -> Nat
countChoosers bs = countOnes Player bs + countManys Player bs

public export
data OnBattlefield : Maybe Zone -> Type where
  OnField : OnBattlefield (Just Battlefield)

public export
data NextUntapCount : Nat -> Type where
  OneNextStep : NextUntapCount 1
  TwoNextSteps : NextUntapCount 2

public export
data ExtraTurnCount : Nat -> Type where
  OneExtraTurn : ExtraTurnCount 1
  TwoExtraTurns : ExtraTurnCount 2

public export
data SkipCount : Nat -> Type where
  OneSkip : SkipCount 1
  TwoSkips : SkipCount 2

public export
data PhaseCount : Nat -> Type where
  OnePhase : PhaseCount 1
  TwoPhases : PhaseCount 2

public export
data EntryCounterMark = Fresh | Additional

public export
sameEntryCounterMark : EntryCounterMark -> EntryCounterMark -> Bool
sameEntryCounterMark Fresh Fresh = True
sameEntryCounterMark Fresh Additional = False
sameEntryCounterMark Additional Fresh = False
sameEntryCounterMark Additional Additional = True

public export
data PlayerGroupWord = AllPlayers | YourOpponents

public export
samePlayerGroupWord : PlayerGroupWord -> PlayerGroupWord -> Bool
samePlayerGroupWord AllPlayers AllPlayers = True
samePlayerGroupWord AllPlayers _ = False
samePlayerGroupWord YourOpponents YourOpponents = True
samePlayerGroupWord YourOpponents _ = False

public export
data JoinedPlayer = JoinAnyPlayer | JoinOpponent

public export
sameJoinedPlayer : JoinedPlayer -> JoinedPlayer -> Bool
sameJoinedPlayer JoinAnyPlayer JoinAnyPlayer = True
sameJoinedPlayer JoinAnyPlayer _ = False
sameJoinedPlayer JoinOpponent JoinOpponent = True
sameJoinedPlayer JoinOpponent _ = False

public export
data JoinedClass = JoinPermanent | JoinCreature | JoinPlaneswalker | JoinBattle

public export
sameJoinedClass : JoinedClass -> JoinedClass -> Bool
sameJoinedClass JoinPermanent JoinPermanent = True
sameJoinedClass JoinPermanent _ = False
sameJoinedClass JoinCreature JoinCreature = True
sameJoinedClass JoinCreature _ = False
sameJoinedClass JoinPlaneswalker JoinPlaneswalker = True
sameJoinedClass JoinPlaneswalker _ = False
sameJoinedClass JoinBattle JoinBattle = True
sameJoinedClass JoinBattle _ = False

public export
data RoundMode = RoundUp | RoundDown

public export
sameRoundMode : RoundMode -> RoundMode -> Bool
sameRoundMode RoundUp RoundUp = True
sameRoundMode RoundUp _ = False
sameRoundMode RoundDown RoundDown = True
sameRoundMode RoundDown _ = False

public export
data ScaleFactor = Doubled | Tripled

public export
sameScaleFactor : ScaleFactor -> ScaleFactor -> Bool
sameScaleFactor Doubled Doubled = True
sameScaleFactor Doubled _ = False
sameScaleFactor Tripled Tripled = True
sameScaleFactor Tripled _ = False

public export
data ShiftDir = ShiftUp | ShiftDown

public export
sameShiftDir : ShiftDir -> ShiftDir -> Bool
sameShiftDir ShiftUp ShiftUp = True
sameShiftDir ShiftUp _ = False
sameShiftDir ShiftDown ShiftDown = True
sameShiftDir ShiftDown _ = False

public export
data CapBound : Nat -> Type where
  OneUntap : CapBound 1
  TwoUntaps : CapBound 2

namespace Lookback
  public export
  data Lookback = ThisTurn | ThisCombat | LastTurn | ThisGame

public export
data OnStack : Maybe Zone -> Type where
  OnTheStack : OnStack (Just Stack)

public export
counterZone : Maybe Zone -> Bool
counterZone Nothing = False
counterZone (Just Battlefield) = True
counterZone (Just Graveyard) = False
counterZone (Just Exile) = True
counterZone (Just Hand) = False
counterZone (Just Library) = False
counterZone (Just Stack) = False

public export
data CounterHolder : Maybe Zone -> Type where
  MkCounterHolder : {auto 0 ok : counterZone z = True} -> CounterHolder z

public export
zoneFits : Maybe Zone -> Maybe Zone -> Bool
zoneFits Nothing _ = True
zoneFits (Just _) Nothing = True
zoneFits (Just a) (Just b) = sameZone a b

public export
data ZoneFits : Maybe Zone -> Maybe Zone -> Type where
  MkZoneFits : {auto 0 ok : zoneFits subj desc = True} -> ZoneFits subj desc

public export
data Targetable : Kind -> Type where
  ObjectTgt : Targetable Object
  PlayerTgt : Targetable Player

public export
data DamageableTy : Maybe CardType -> Type where
  DamCreature : DamageableTy (Just Creature)
  DamPlaneswalker : DamageableTy (Just Planeswalker)

public export
data Phrasal : Kind -> Type where
  PhObject : Phrasal Object
  PhPlayer : Phrasal Player
  PhQuality : Phrasal (Quality q)
  PhAbility : Phrasal Ability

public export
targetablePhrasal : Targetable k -> Phrasal k
targetablePhrasal ObjectTgt = PhObject
targetablePhrasal PlayerTgt = PhPlayer


public export
data Keyword = Haste | Flying | Trample | Vigilance | Deathtouch
             | DoubleStrike | FirstStrike | Reach
             | Convoke | Improvise | Storm | Lifelink
             | Ward | Protection
             | Enchant | Equip | Ascend | Storied | Renown
             | Indestructible | Flash
             | CumulativeUpkeep
             | Hexproof | Menace | Skulk

public export
data KeywordParamShape = NoParam | CostParam | QualityParam | SubjectParam
                       | NumberParam

public export
keywordParamShape : Keyword -> KeywordParamShape
keywordParamShape Haste = NoParam
keywordParamShape Flying = NoParam
keywordParamShape Trample = NoParam
keywordParamShape Vigilance = NoParam
keywordParamShape Deathtouch = NoParam
keywordParamShape DoubleStrike = NoParam
keywordParamShape FirstStrike = NoParam
keywordParamShape Reach = NoParam
keywordParamShape Convoke = NoParam
keywordParamShape Improvise = NoParam
keywordParamShape Storm = NoParam
keywordParamShape Lifelink = NoParam
keywordParamShape Ward = CostParam
keywordParamShape Protection = QualityParam
keywordParamShape Enchant = SubjectParam
keywordParamShape Equip = CostParam
keywordParamShape Ascend = NoParam
keywordParamShape Storied = NoParam
keywordParamShape Renown = NumberParam
keywordParamShape Indestructible = NoParam
keywordParamShape Flash = NoParam
keywordParamShape CumulativeUpkeep = CostParam
keywordParamShape Hexproof = NoParam
keywordParamShape Menace = NoParam
keywordParamShape Skulk = NoParam

public export
sameParamShape : KeywordParamShape -> KeywordParamShape -> Bool
sameParamShape NoParam NoParam = True
sameParamShape NoParam _ = False
sameParamShape CostParam CostParam = True
sameParamShape CostParam _ = False
sameParamShape QualityParam QualityParam = True
sameParamShape QualityParam _ = False
sameParamShape SubjectParam SubjectParam = True
sameParamShape SubjectParam _ = False
sameParamShape NumberParam NumberParam = True
sameParamShape NumberParam _ = False

public export
keywordParamless : Keyword -> Bool
keywordParamless k = sameParamShape (keywordParamShape k) NoParam

public export
data KeywordParamless : Keyword -> Type where
  MkKeywordParamless : {auto 0 ok : keywordParamless k = True} ->
                       KeywordParamless k

public export
sameKeyword : Keyword -> Keyword -> Bool
sameKeyword Haste Haste = True
sameKeyword Haste _ = False
sameKeyword Flying Flying = True
sameKeyword Flying _ = False
sameKeyword Trample Trample = True
sameKeyword Trample _ = False
sameKeyword Vigilance Vigilance = True
sameKeyword Vigilance _ = False
sameKeyword Deathtouch Deathtouch = True
sameKeyword Deathtouch _ = False
sameKeyword DoubleStrike DoubleStrike = True
sameKeyword DoubleStrike _ = False
sameKeyword FirstStrike FirstStrike = True
sameKeyword FirstStrike _ = False
sameKeyword Reach Reach = True
sameKeyword Reach _ = False
sameKeyword Convoke Convoke = True
sameKeyword Convoke _ = False
sameKeyword Improvise Improvise = True
sameKeyword Improvise _ = False
sameKeyword Storm Storm = True
sameKeyword Storm _ = False
sameKeyword Lifelink Lifelink = True
sameKeyword Lifelink _ = False
sameKeyword Ward Ward = True
sameKeyword Ward _ = False
sameKeyword Protection Protection = True
sameKeyword Protection _ = False
sameKeyword Enchant Enchant = True
sameKeyword Enchant _ = False
sameKeyword Equip Equip = True
sameKeyword Equip _ = False
sameKeyword Ascend Ascend = True
sameKeyword Ascend _ = False
sameKeyword Storied Storied = True
sameKeyword Storied _ = False
sameKeyword Renown Renown = True
sameKeyword Renown _ = False
sameKeyword Indestructible Indestructible = True
sameKeyword Indestructible _ = False
sameKeyword Flash Flash = True
sameKeyword Flash _ = False
sameKeyword CumulativeUpkeep CumulativeUpkeep = True
sameKeyword CumulativeUpkeep _ = False
sameKeyword Hexproof Hexproof = True
sameKeyword Hexproof _ = False
sameKeyword Menace Menace = True
sameKeyword Menace _ = False
sameKeyword Skulk Skulk = True
sameKeyword Skulk _ = False

public export
data AbilityClass = AnyActivated | LoyaltyClass | KeywordClass Keyword

public export
sameAbilityClass : AbilityClass -> AbilityClass -> Bool
sameAbilityClass AnyActivated AnyActivated = True
sameAbilityClass AnyActivated _ = False
sameAbilityClass LoyaltyClass LoyaltyClass = True
sameAbilityClass LoyaltyClass _ = False
sameAbilityClass (KeywordClass a) (KeywordClass b) = sameKeyword a b
sameAbilityClass (KeywordClass _) _ = False



namespace Chroma
  public export
  data Color = White | Blue | Black | Red | Green

  public export
  data ColorOrColorless = Colorless | OfColor Color

public export
sameColor : Color -> Color -> Bool
sameColor White White = True
sameColor White _ = False
sameColor Blue Blue = True
sameColor Blue _ = False
sameColor Black Black = True
sameColor Black _ = False
sameColor Red Red = True
sameColor Red _ = False
sameColor Green Green = True
sameColor Green _ = False


public export
data SimpleManaSymbol = Generic Nat | Specific ColorOrColorless

public export
halvesDistinct : SimpleManaSymbol -> Color -> Bool
halvesDistinct (Generic _) d = True
halvesDistinct (Specific Colorless) d = True
halvesDistinct (Specific (OfColor c)) d = not (sameColor c d)

public export
data HalvesDistinct : SimpleManaSymbol -> Color -> Type where
  MkHalvesDistinct : {auto 0 ok : halvesDistinct l r = True} -> HalvesDistinct l r

public export
phyrexianDistinct : Color -> Maybe Color -> Bool
phyrexianDistinct c Nothing = True
phyrexianDistinct c (Just d) = not (sameColor c d)

public export
data PhyrexianDistinct : Color -> Maybe Color -> Type where
  MkPhyrexianDistinct : {auto 0 ok : phyrexianDistinct c d = True} ->
                        PhyrexianDistinct c d

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
manaRunWritten : ManaCost -> Bool
manaRunWritten [] = False
manaRunWritten (_ :: _) = True

public export
data ManaRun : ManaCost -> Type where
  MkManaRun : {auto 0 ok : manaRunWritten c = True} -> ManaRun c


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
data ProducedRuns : List ProducedRun -> Type where
  MkProducedRuns : {auto 0 ok : producedRunsWritten rs = True} -> ProducedRuns rs

public export
altRunWritten : Maybe ProducedRun -> Bool
altRunWritten Nothing = True
altRunWritten (Just [_]) = True
altRunWritten (Just _) = False

public export
data AltRunWritten : Maybe ProducedRun -> Type where
  MkAltRunWritten : {auto 0 ok : altRunWritten alt = True} -> AltRunWritten alt

public export
data ColorFreedom = SameColor | EachColor

public export
loyaltyStepWritten : Nat -> Bool
loyaltyStepWritten Z = False
loyaltyStepWritten (S _) = True

public export
data LoyaltyStep : Nat -> Type where
  MkLoyaltyStep : {auto 0 ok : loyaltyStepWritten n = True} -> LoyaltyStep n

public export
data LoyaltyCost : Type where
  LoyaltyUp : (n : Nat) -> {auto 0 nz : LoyaltyStep n} -> LoyaltyCost
  LoyaltyDown : (n : Nat) -> {auto 0 nz : LoyaltyStep n} -> LoyaltyCost
  LoyaltyDownX : LoyaltyCost
  LoyaltyZero : LoyaltyCost

public export
loyaltyAnnouncesX : LoyaltyCost -> Bool
loyaltyAnnouncesX (LoyaltyUp _) = False
loyaltyAnnouncesX (LoyaltyDown _) = False
loyaltyAnnouncesX LoyaltyDownX = True
loyaltyAnnouncesX LoyaltyZero = False

public export
data Subtype = Zombie | Army | Soldier | Thopter | Construct | Fractal
             | Coward | Demon | Angel | Elemental | Plant | Dragon | Plains | Island | Swamp
             | Mountain | Forest | Goblin | Equipment | Avatar | Insect
             | Elder | Dinosaur | Human | Advisor | Wizard | Shaman | Merfolk
             | Vedalken | Artificer | Knight | Myr | Elk | Treefolk
             | Jace | Elspeth | Saheeli
             | Imp
             | Sliver
             | Wall
             | Cleric
             | Illusion
             | Siege
             | Aura | Curse
             | Tiefling | Warlock | Pirate
             | Cat | Beast | Dwarf | Bard
             | Hero
             | Elf | Scout
             | Rogue | Arcane
             | Druid | Alien | Warrior
             | Vampire | Mutant
             | AssemblyWorker | Ooze
             | Frog
             | Horse
             | Bird
             | Ally | Gideon
             | Goat
             | Spirit
             | Shapeshifter
             | Saga
             | Centaur
             | Monk
             | Nymph | Dryad
             | Nightmare | Fish
             | Horror | Gargoyle | Assassin
             | Skeleton
             | Town | Desert
             | Pegasus
             | Faerie
             | Kraken | Sphinx
             | Werewolf | Eldrazi

public export
sameSub : Subtype -> Subtype -> Bool
sameSub Goblin Goblin = True
sameSub Goblin _ = False
sameSub Equipment Equipment = True
sameSub Equipment _ = False
sameSub Avatar Avatar = True
sameSub Avatar _ = False
sameSub Insect Insect = True
sameSub Insect _ = False
sameSub Elder Elder = True
sameSub Elder _ = False
sameSub Dinosaur Dinosaur = True
sameSub Dinosaur _ = False
sameSub Horror Horror = True
sameSub Horror _ = False
sameSub Gargoyle Gargoyle = True
sameSub Gargoyle _ = False
sameSub Assassin Assassin = True
sameSub Assassin _ = False
sameSub Skeleton Skeleton = True
sameSub Skeleton _ = False
sameSub Town Town = True
sameSub Town _ = False
sameSub Desert Desert = True
sameSub Desert _ = False
sameSub Pegasus Pegasus = True
sameSub Pegasus _ = False
sameSub Faerie Faerie = True
sameSub Faerie _ = False
sameSub Kraken Kraken = True
sameSub Kraken _ = False
sameSub Sphinx Sphinx = True
sameSub Sphinx _ = False
sameSub Werewolf Werewolf = True
sameSub Werewolf _ = False
sameSub Eldrazi Eldrazi = True
sameSub Eldrazi _ = False
sameSub Goat Goat = True
sameSub Goat _ = False
sameSub Spirit Spirit = True
sameSub Spirit _ = False
sameSub Shapeshifter Shapeshifter = True
sameSub Shapeshifter _ = False
sameSub Centaur Centaur = True
sameSub Centaur _ = False
sameSub Monk Monk = True
sameSub Monk _ = False
sameSub Nymph Nymph = True
sameSub Nymph _ = False
sameSub Dryad Dryad = True
sameSub Dryad _ = False
sameSub Nightmare Nightmare = True
sameSub Nightmare _ = False
sameSub Fish Fish = True
sameSub Fish _ = False
sameSub Saga Saga = True
sameSub Saga _ = False
sameSub Human Human = True
sameSub Human _ = False
sameSub Advisor Advisor = True
sameSub Advisor _ = False
sameSub Wizard Wizard = True
sameSub Wizard _ = False
sameSub Merfolk Merfolk = True
sameSub Merfolk _ = False
sameSub Shaman Shaman = True
sameSub Shaman _ = False
sameSub Vedalken Vedalken = True
sameSub Vedalken _ = False
sameSub Artificer Artificer = True
sameSub Artificer _ = False
sameSub Zombie Zombie = True
sameSub Zombie _ = False
sameSub Army Army = True
sameSub Army _ = False
sameSub Soldier Soldier = True
sameSub Soldier _ = False
sameSub Knight Knight = True
sameSub Knight _ = False
sameSub Myr Myr = True
sameSub Myr _ = False
sameSub Elk Elk = True
sameSub Elk _ = False
sameSub Treefolk Treefolk = True
sameSub Treefolk _ = False
sameSub Siege Siege = True
sameSub Siege _ = False
sameSub Imp Imp = True
sameSub Imp _ = False
sameSub Saheeli Saheeli = True
sameSub Saheeli _ = False
sameSub Jace Jace = True
sameSub Jace _ = False
sameSub Elspeth Elspeth = True
sameSub Elspeth _ = False
sameSub Thopter Thopter = True
sameSub Thopter _ = False
sameSub Construct Construct = True
sameSub Construct _ = False
sameSub Fractal Fractal = True
sameSub Fractal _ = False
sameSub Coward Coward = True
sameSub Coward _ = False
sameSub Demon Demon = True
sameSub Demon _ = False
sameSub Illusion Illusion = True
sameSub Illusion _ = False
sameSub Sliver Sliver = True
sameSub Sliver _ = False
sameSub Wall Wall = True
sameSub Wall _ = False
sameSub Cleric Cleric = True
sameSub Cleric _ = False
sameSub Angel Angel = True
sameSub Angel _ = False
sameSub Elemental Elemental = True
sameSub Elemental _ = False
sameSub Plant Plant = True
sameSub Plant _ = False
sameSub Dragon Dragon = True
sameSub Dragon _ = False
sameSub Plains Plains = True
sameSub Plains _ = False
sameSub Island Island = True
sameSub Island _ = False
sameSub Swamp Swamp = True
sameSub Swamp _ = False
sameSub Mountain Mountain = True
sameSub Mountain _ = False
sameSub Forest Forest = True
sameSub Forest _ = False
sameSub Aura Aura = True
sameSub Aura _ = False
sameSub Curse Curse = True
sameSub Curse _ = False
sameSub Tiefling Tiefling = True
sameSub Tiefling _ = False
sameSub Warlock Warlock = True
sameSub Warlock _ = False
sameSub Pirate Pirate = True
sameSub Pirate _ = False
sameSub Cat Cat = True
sameSub Cat _ = False
sameSub Beast Beast = True
sameSub Beast _ = False
sameSub Dwarf Dwarf = True
sameSub Dwarf _ = False
sameSub Bard Bard = True
sameSub Bard _ = False
sameSub Hero Hero = True
sameSub Hero _ = False
sameSub Elf Elf = True
sameSub Elf _ = False
sameSub Scout Scout = True
sameSub Scout _ = False
sameSub Rogue Rogue = True
sameSub Rogue _ = False
sameSub Druid Druid = True
sameSub Druid _ = False
sameSub Alien Alien = True
sameSub Alien _ = False
sameSub Warrior Warrior = True
sameSub Warrior _ = False
sameSub Vampire Vampire = True
sameSub Vampire _ = False
sameSub Mutant Mutant = True
sameSub Mutant _ = False
sameSub AssemblyWorker AssemblyWorker = True
sameSub AssemblyWorker _ = False
sameSub Ooze Ooze = True
sameSub Ooze _ = False
sameSub Frog Frog = True
sameSub Frog _ = False
sameSub Horse Horse = True
sameSub Horse _ = False
sameSub Bird Bird = True
sameSub Bird _ = False
sameSub Ally Ally = True
sameSub Ally _ = False
sameSub Gideon Gideon = True
sameSub Gideon _ = False
sameSub Arcane Arcane = True
sameSub Arcane _ = False

public export
subtypeType : Subtype -> CardType
subtypeType Zombie = Creature
subtypeType Army = Creature
subtypeType Soldier = Creature
subtypeType Knight = Creature
subtypeType Myr = Creature
subtypeType Elk = Creature
subtypeType Treefolk = Creature
subtypeType Siege = Battle
subtypeType Imp = Creature
subtypeType Saheeli = Planeswalker
subtypeType Jace = Planeswalker
subtypeType Elspeth = Planeswalker
subtypeType Thopter = Creature
subtypeType Construct = Creature
subtypeType Fractal = Creature
subtypeType Coward = Creature
subtypeType Demon = Creature
subtypeType Illusion = Creature
subtypeType Sliver = Creature
subtypeType Wall = Creature
subtypeType Cleric = Creature
subtypeType Angel = Creature
subtypeType Elemental = Creature
subtypeType Plant = Creature
subtypeType Dragon = Creature
subtypeType Plains = Land
subtypeType Island = Land
subtypeType Swamp = Land
subtypeType Mountain = Land
subtypeType Forest = Land
subtypeType Goblin = Creature
subtypeType Equipment = Artifact
subtypeType Avatar = Creature
subtypeType Insect = Creature
subtypeType Elder = Creature
subtypeType Dinosaur = Creature
subtypeType Horror = Creature
subtypeType Gargoyle = Creature
subtypeType Assassin = Creature
subtypeType Skeleton = Creature
subtypeType Town = Land
subtypeType Desert = Land
subtypeType Pegasus = Creature
subtypeType Faerie = Creature
subtypeType Kraken = Creature
subtypeType Sphinx = Creature
subtypeType Werewolf = Creature
subtypeType Eldrazi = Creature
subtypeType Goat = Creature
subtypeType Spirit = Creature
subtypeType Shapeshifter = Creature
subtypeType Centaur = Creature
subtypeType Monk = Creature
subtypeType Nymph = Creature
subtypeType Dryad = Creature
subtypeType Nightmare = Creature
subtypeType Fish = Creature
subtypeType Saga = Enchantment
subtypeType Human = Creature
subtypeType Advisor = Creature
subtypeType Wizard = Creature
subtypeType Merfolk = Creature
subtypeType Shaman = Creature
subtypeType Vedalken = Creature
subtypeType Artificer = Creature
subtypeType Aura = Enchantment
subtypeType Curse = Enchantment
subtypeType Tiefling = Creature
subtypeType Warlock = Creature
subtypeType Pirate = Creature
subtypeType Cat = Creature
subtypeType Beast = Creature
subtypeType Dwarf = Creature
subtypeType Bard = Creature
subtypeType Hero = Creature
subtypeType Elf = Creature
subtypeType Scout = Creature
subtypeType Rogue = Creature
subtypeType Druid = Creature
subtypeType Alien = Creature
subtypeType Warrior = Creature
subtypeType Vampire = Creature
subtypeType Mutant = Creature
subtypeType AssemblyWorker = Creature
subtypeType Ooze = Creature
subtypeType Frog = Creature
subtypeType Horse = Creature
subtypeType Bird = Creature
subtypeType Ally = Creature
subtypeType Gideon = Planeswalker
subtypeType Arcane = Instant

public export
data BasicLandType : Subtype -> Type where
  PlainsBasic   : BasicLandType Plains
  IslandBasic   : BasicLandType Island
  SwampBasic    : BasicLandType Swamp
  MountainBasic : BasicLandType Mountain
  ForestBasic   : BasicLandType Forest

public export
data TypeSpace = BasicLandSpace | LandSpace | CreatureSpace

public export
spaceHosted : TypeSpace -> Maybe CardType -> Bool
spaceHosted BasicLandSpace ty = tyIs Land ty
spaceHosted LandSpace ty = tyIs Land ty
spaceHosted CreatureSpace ty = tyIs Creature ty

public export
data SpaceHosted : TypeSpace -> Maybe CardType -> Type where
  MkSpaceHosted : {auto 0 ok : spaceHosted sp ty = True} -> SpaceHosted sp ty

public export
data BasicLandTypes : List Subtype -> Type where
  NoBasicTypes : BasicLandTypes []
  MoreBasicTypes : {0 s : Subtype} -> {0 ss : List Subtype} ->
                   BasicLandType s -> BasicLandTypes ss ->
                   BasicLandTypes (s :: ss)

-- [CR#109.2]: a bare type word with no zone reads onto the battlefield,
-- so only a word naming a permanent type can be self-ascribed this way.
public export
ascribesAsType : CardType -> Bool
ascribesAsType Creature = True
ascribesAsType Artifact = True
ascribesAsType Land = True
ascribesAsType Enchantment = True
ascribesAsType Planeswalker = True
ascribesAsType Battle = False
-- [CR#308.1]: a kindred card always has another card type, so it is
-- never the word such a card names itself by.
ascribesAsType Kindred = False
ascribesAsType Instant = False
ascribesAsType Sorcery = False

public export
ascribesAsSubtype : Subtype -> Bool
ascribesAsSubtype Aura = True
ascribesAsSubtype Equipment = True
ascribesAsSubtype Curse = False
ascribesAsSubtype Tiefling = False
ascribesAsSubtype Warlock = False
ascribesAsSubtype Pirate = False
ascribesAsSubtype Cat = False
ascribesAsSubtype Beast = False
ascribesAsSubtype Dwarf = False
ascribesAsSubtype Bard = False
ascribesAsSubtype Hero = False
ascribesAsSubtype Elf = False
ascribesAsSubtype Scout = False
ascribesAsSubtype Rogue = False
ascribesAsSubtype Druid = False
ascribesAsSubtype Alien = False
ascribesAsSubtype Warrior = False
ascribesAsSubtype Vampire = False
ascribesAsSubtype AssemblyWorker = False
ascribesAsSubtype Ooze = False
ascribesAsSubtype Frog = False
ascribesAsSubtype Horse = False
ascribesAsSubtype Bird = False
ascribesAsSubtype Ally = False
ascribesAsSubtype Gideon = False
ascribesAsSubtype Mutant = False
ascribesAsSubtype Arcane = False
ascribesAsSubtype Zombie = False
ascribesAsSubtype Army = False
ascribesAsSubtype Soldier = False
ascribesAsSubtype Thopter = False
ascribesAsSubtype Construct = False
ascribesAsSubtype Fractal = False
ascribesAsSubtype Coward = False
ascribesAsSubtype Demon = False
ascribesAsSubtype Illusion = False
ascribesAsSubtype Sliver = False
ascribesAsSubtype Wall = False
ascribesAsSubtype Cleric = False
ascribesAsSubtype Angel = False
ascribesAsSubtype Elemental = False
ascribesAsSubtype Plant = False
ascribesAsSubtype Dragon = False
ascribesAsSubtype Plains = False
ascribesAsSubtype Island = False
ascribesAsSubtype Swamp = False
ascribesAsSubtype Mountain = False
ascribesAsSubtype Forest = False
ascribesAsSubtype Goblin = False
ascribesAsSubtype Avatar = False
ascribesAsSubtype Insect = False
ascribesAsSubtype Elder = False
ascribesAsSubtype Dinosaur = False
ascribesAsSubtype Horror = False
ascribesAsSubtype Gargoyle = False
ascribesAsSubtype Assassin = False
ascribesAsSubtype Skeleton = False
ascribesAsSubtype Town = False
ascribesAsSubtype Desert = False
ascribesAsSubtype Pegasus = False
ascribesAsSubtype Faerie = False
ascribesAsSubtype Kraken = False
ascribesAsSubtype Sphinx = False
ascribesAsSubtype Werewolf = False
ascribesAsSubtype Eldrazi = False
ascribesAsSubtype Goat = False
ascribesAsSubtype Spirit = False
ascribesAsSubtype Shapeshifter = False
ascribesAsSubtype Centaur = False
ascribesAsSubtype Monk = False
ascribesAsSubtype Nymph = False
ascribesAsSubtype Dryad = False
ascribesAsSubtype Nightmare = False
ascribesAsSubtype Fish = False
ascribesAsSubtype Saga = True
ascribesAsSubtype Human = False
ascribesAsSubtype Advisor = False
ascribesAsSubtype Wizard = False
ascribesAsSubtype Merfolk = False
ascribesAsSubtype Shaman = False
ascribesAsSubtype Vedalken = False
ascribesAsSubtype Artificer = False
ascribesAsSubtype Knight = False
ascribesAsSubtype Myr = False
ascribesAsSubtype Elk = False
ascribesAsSubtype Treefolk = False
ascribesAsSubtype Siege = True
ascribesAsSubtype Imp = False
ascribesAsSubtype Saheeli = False
ascribesAsSubtype Jace = False
ascribesAsSubtype Elspeth = False

public export
ascriptionOk : CardType -> Maybe Subtype -> Bool
ascriptionOk t Nothing = ascribesAsType t
ascriptionOk t (Just s) = ascribesAsSubtype s && sameCT (subtypeType s) t

namespace Counter
  public export
  data Delta : Type where
    Up : Nat -> Delta
    Down : Nat -> Delta

  public export
  sameDelta : Delta -> Delta -> Bool
  sameDelta (Up a) (Up b) = a == b
  sameDelta (Up _) _ = False
  sameDelta (Down a) (Down b) = a == b
  sameDelta (Down _) _ = False

public export
keywordCounterOk : Keyword -> Bool
keywordCounterOk Haste = True
keywordCounterOk Flying = True
keywordCounterOk Trample = True
keywordCounterOk Vigilance = True
keywordCounterOk Deathtouch = True
keywordCounterOk DoubleStrike = True
keywordCounterOk FirstStrike = True
keywordCounterOk Reach = True
keywordCounterOk Convoke = False
keywordCounterOk Improvise = False
keywordCounterOk Storm = False
keywordCounterOk Lifelink = True
keywordCounterOk Ward = False
keywordCounterOk Protection = False
keywordCounterOk Enchant = False
keywordCounterOk Equip = False
keywordCounterOk Ascend = False
keywordCounterOk Storied = False
keywordCounterOk Renown = False
keywordCounterOk Indestructible = True
keywordCounterOk Flash = False
keywordCounterOk CumulativeUpkeep = False
keywordCounterOk Hexproof = True
keywordCounterOk Menace = True
keywordCounterOk Skulk = False

public export
data KeywordCounterEligible : Keyword -> Type where
  MkKeywordCounterEligible : {auto 0 ok : keywordCounterOk k = True} ->
                            KeywordCounterEligible k

public export
data StackRegime = AtCasting | AtResolution

public export
sameRegime : StackRegime -> StackRegime -> Bool
sameRegime AtCasting AtCasting = True
sameRegime AtCasting _ = False
sameRegime AtResolution AtResolution = True
sameRegime AtResolution _ = False

public export
keywordStackRegime : Keyword -> Maybe StackRegime
keywordStackRegime Haste = Nothing
keywordStackRegime Flying = Nothing
keywordStackRegime Trample = Nothing
keywordStackRegime Vigilance = Nothing
keywordStackRegime DoubleStrike = Nothing
keywordStackRegime FirstStrike = Nothing
keywordStackRegime Reach = Nothing
keywordStackRegime Convoke = Just AtCasting
keywordStackRegime Improvise = Just AtCasting
keywordStackRegime Storm = Just AtCasting
keywordStackRegime Deathtouch = Just AtResolution
keywordStackRegime Lifelink = Just AtResolution
keywordStackRegime Ward = Nothing
keywordStackRegime Protection = Nothing
keywordStackRegime Enchant = Nothing
keywordStackRegime Equip = Nothing
keywordStackRegime Ascend = Nothing
keywordStackRegime Storied = Nothing
keywordStackRegime Renown = Nothing
keywordStackRegime Indestructible = Nothing
keywordStackRegime Flash = Just AtCasting
keywordStackRegime CumulativeUpkeep = Nothing
keywordStackRegime Hexproof = Nothing
keywordStackRegime Menace = Nothing
keywordStackRegime Skulk = Nothing

public export
data Supertype = Legendary | Basic | Snow

public export
sameSupertype : Supertype -> Supertype -> Bool
sameSupertype Legendary Legendary = True
sameSupertype Legendary _ = False
sameSupertype Basic Basic = True
sameSupertype Basic _ = False
sameSupertype Snow Snow = True
sameSupertype Snow _ = False

public export
data Designation
  = -- PLAYER-held ([CR#725.1], [CR#726.1], [CR#702.131c], [CR#702.195b]).
    Monarch | TheInitiative | CitysBlessing | EnduringStory
  | -- OBJECT-held: the permanent markers ([CR#701.15b], [CR#701.54b]).
    Goaded | RingBearer | Monstrous | Renowned | Suspected | Saddled
  | Prepared
  | -- CARD-held ([CR#903.3]).
    CommanderD
  | -- GAME-held ([CR#731.1]).
    Day | Night

public export
data DesignationScope = HeldBy Kind | HeldByCard | HeldByGame

public export
designationScope : Designation -> DesignationScope
designationScope Monarch = HeldBy Player
designationScope TheInitiative = HeldBy Player
designationScope CitysBlessing = HeldBy Player
designationScope EnduringStory = HeldBy Player
designationScope Goaded = HeldBy Object
designationScope RingBearer = HeldBy Object
designationScope Monstrous = HeldBy Object
designationScope Renowned = HeldBy Object
designationScope Suspected = HeldBy Object
designationScope Saddled = HeldBy Object
designationScope Prepared = HeldBy Object
designationScope CommanderD = HeldByCard
designationScope Day = HeldByGame
designationScope Night = HeldByGame

public export
sameDesignation : Designation -> Designation -> Bool
sameDesignation Monarch Monarch = True
sameDesignation Monarch _ = False
sameDesignation TheInitiative TheInitiative = True
sameDesignation TheInitiative _ = False
sameDesignation CitysBlessing CitysBlessing = True
sameDesignation CitysBlessing _ = False
sameDesignation EnduringStory EnduringStory = True
sameDesignation EnduringStory _ = False
sameDesignation Goaded Goaded = True
sameDesignation Goaded _ = False
sameDesignation RingBearer RingBearer = True
sameDesignation RingBearer _ = False
sameDesignation Monstrous Monstrous = True
sameDesignation Monstrous _ = False
sameDesignation Renowned Renowned = True
sameDesignation Renowned _ = False
sameDesignation Suspected Suspected = True
sameDesignation Suspected _ = False
sameDesignation Saddled Saddled = True
sameDesignation Saddled _ = False
sameDesignation Prepared Prepared = True
sameDesignation Prepared _ = False
sameDesignation CommanderD CommanderD = True
sameDesignation CommanderD _ = False
sameDesignation Day Day = True
sameDesignation Day _ = False
sameDesignation Night Night = True
sameDesignation Night _ = False

public export
designationChecked : Designation -> Bool
designationChecked Monarch = True
designationChecked TheInitiative = True
designationChecked CitysBlessing = True
designationChecked EnduringStory = True
designationChecked Goaded = True
designationChecked RingBearer = True
designationChecked Monstrous = True
designationChecked Renowned = True
designationChecked Suspected = True
designationChecked Saddled = True
designationChecked Prepared = True
designationChecked CommanderD = False
designationChecked Day = False
designationChecked Night = True

public export
designationGiven : Designation -> Bool
designationGiven Monarch = True
designationGiven TheInitiative = True
designationGiven CitysBlessing = False
designationGiven EnduringStory = False
designationGiven Goaded = True
designationGiven RingBearer = False
designationGiven Monstrous = False
designationGiven Renowned = False
designationGiven Suspected = True
designationGiven Saddled = False
designationGiven Prepared = True
designationGiven CommanderD = False
designationGiven Day = True
designationGiven Night = True

public export
designationSeedZone : Designation -> Maybe Zone
designationSeedZone Monarch = Nothing
designationSeedZone TheInitiative = Nothing
designationSeedZone CitysBlessing = Nothing
designationSeedZone EnduringStory = Nothing
designationSeedZone Goaded = Just Battlefield
designationSeedZone RingBearer = Just Battlefield
designationSeedZone Monstrous = Just Battlefield
designationSeedZone Renowned = Just Battlefield
designationSeedZone Suspected = Just Battlefield
designationSeedZone Saddled = Just Battlefield
designationSeedZone Prepared = Just Battlefield
designationSeedZone CommanderD = Nothing
designationSeedZone Day = Nothing
designationSeedZone Night = Nothing

public export
designationSeedType : Designation -> Maybe CardType
designationSeedType Monarch = Nothing
designationSeedType TheInitiative = Nothing
designationSeedType CitysBlessing = Nothing
designationSeedType EnduringStory = Nothing
designationSeedType Goaded = Just Creature
designationSeedType RingBearer = Just Creature
designationSeedType Monstrous = Just Creature
designationSeedType Renowned = Just Creature
designationSeedType Suspected = Just Creature
designationSeedType Saddled = Nothing
designationSeedType Prepared = Just Creature
designationSeedType CommanderD = Nothing
designationSeedType Day = Nothing
designationSeedType Night = Nothing

public export
data DesignationHolder : Designation -> Maybe Zone -> Type where
  HolderUnzoned : {auto 0 sc : designationScope d = HeldBy Player} ->
                  DesignationHolder d z
  HolderOnField : {auto 0 sc : designationScope d = HeldBy Object} ->
                  {auto 0 ok : OnBattlefield z} -> DesignationHolder d z

public export
data AttachWord = Enchanted | Equipped | Fortified

public export
sameAttachWord : AttachWord -> AttachWord -> Bool
sameAttachWord Enchanted Enchanted = True
sameAttachWord Enchanted _ = False
sameAttachWord Equipped Equipped = True
sameAttachWord Equipped _ = False
sameAttachWord Fortified Fortified = True
sameAttachWord Fortified _ = False

public export
attachHeadOk : AttachWord -> NounWord -> Bool
attachHeadOk Enchanted (TypeW Creature) = True
attachHeadOk Enchanted (TypeW Artifact) = True
attachHeadOk Enchanted (TypeW Land) = True
attachHeadOk Enchanted (TypeW Enchantment) = True
attachHeadOk Enchanted (TypeW Instant) = False
attachHeadOk Enchanted (TypeW Sorcery) = False
attachHeadOk Enchanted CardW = False
attachHeadOk Enchanted SpellW = False
attachHeadOk Enchanted (TypeW Planeswalker) = True
attachHeadOk Enchanted (TypeW Battle) = False
attachHeadOk Enchanted (TypeW Kindred) = False
attachHeadOk Enchanted PlayerW = True
attachHeadOk Enchanted PermanentW = True
attachHeadOk Enchanted TokenW = False
attachHeadOk Enchanted CopyW = False
attachHeadOk Enchanted JoinW = False
attachHeadOk Equipped (TypeW Creature) = True
attachHeadOk Equipped (TypeW Artifact) = False
attachHeadOk Equipped (TypeW Land) = False
attachHeadOk Equipped (TypeW Enchantment) = False
attachHeadOk Equipped (TypeW Planeswalker) = False
attachHeadOk Equipped (TypeW Battle) = False
attachHeadOk Equipped (TypeW Kindred) = False
attachHeadOk Equipped (TypeW Instant) = False
attachHeadOk Equipped (TypeW Sorcery) = False
attachHeadOk Equipped CardW = False
attachHeadOk Equipped SpellW = False
attachHeadOk Equipped PlayerW = False
attachHeadOk Equipped PermanentW = False
attachHeadOk Equipped TokenW = False
attachHeadOk Equipped CopyW = False
attachHeadOk Equipped JoinW = False
attachHeadOk Fortified (TypeW Creature) = False
attachHeadOk Fortified (TypeW Artifact) = False
attachHeadOk Fortified (TypeW Land) = True
attachHeadOk Fortified (TypeW Enchantment) = False
attachHeadOk Fortified (TypeW Planeswalker) = False
attachHeadOk Fortified (TypeW Battle) = False
attachHeadOk Fortified (TypeW Kindred) = False
attachHeadOk Fortified (TypeW Instant) = False
attachHeadOk Fortified (TypeW Sorcery) = False
attachHeadOk Fortified CardW = False
attachHeadOk Fortified SpellW = False
attachHeadOk Fortified PlayerW = False
attachHeadOk Fortified PermanentW = False
attachHeadOk Fortified TokenW = False
attachHeadOk Fortified CopyW = False
attachHeadOk Fortified JoinW = False

public export
attachedCheckOk : AttachWord -> Bool
attachedCheckOk Enchanted = True
attachedCheckOk Equipped = True
attachedCheckOk Fortified = False

public export
data AttachHeadOk : AttachWord -> NounWord -> Type where
  MkAttachHeadOk : {auto 0 ok : attachHeadOk w h = True} -> AttachHeadOk w h

public export
attachHostZone : NounWord -> Maybe Zone
attachHostZone PlayerW = Nothing
attachHostZone (TypeW _) = Just Battlefield
attachHostZone CardW = Just Battlefield
attachHostZone SpellW = Just Battlefield
attachHostZone PermanentW = Just Battlefield
attachHostZone TokenW = Just Battlefield
attachHostZone CopyW = Just Stack
attachHostZone JoinW = Nothing

public export
attachHostTy : NounWord -> Maybe CardType
attachHostTy (TypeW t) = Just t
attachHostTy CardW = Nothing
attachHostTy SpellW = Nothing
attachHostTy PlayerW = Nothing
attachHostTy PermanentW = Nothing
attachHostTy TokenW = Nothing
attachHostTy CopyW = Nothing
attachHostTy JoinW = Nothing

public export
data OutcomeVerb = WinGame | LoseGame

public export
sameOutcomeVerb : OutcomeVerb -> OutcomeVerb -> Bool
sameOutcomeVerb WinGame WinGame = True
sameOutcomeVerb WinGame _ = False
sameOutcomeVerb LoseGame LoseGame = True
sameOutcomeVerb LoseGame _ = False

public export
data OutcomeGateKind = CantLose | CantWin

public export
data PlayerAct = GainsLife | PlaysLands | CastsSpells | SearchesLibraries
               | DrawsCards

public export
data ObjectAct = Countered | Cast | Played | Copied | Activated
               | Regenerated

public export
riderAct : ObjectAct -> Bool
riderAct Regenerated = True
riderAct Countered = True
riderAct Cast = False
riderAct Played = False
riderAct Copied = False
riderAct Activated = False

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
data CounterKind : Type where
  BoostCounter : Counter.Delta -> Counter.Delta -> CounterKind
  Stun : CounterKind
  Time : CounterKind
  KeywordCounter : (k : Keyword) -> {auto 0 ok : KeywordCounterEligible k} -> CounterKind
  Charge : CounterKind
  Omen : CounterKind
  Spite : CounterKind
  Rev : CounterKind
  Intervention : CounterKind
  Poison : CounterKind
  Rad : CounterKind
  Experience : CounterKind
  Lore : CounterKind
  Age : CounterKind

public export
counterScope : CounterKind -> Kind
counterScope (BoostCounter _ _) = Object
counterScope Stun = Object
counterScope Time = Object
counterScope (KeywordCounter _) = Object
counterScope Charge = Object
counterScope Omen = Object
counterScope Spite = Object
counterScope Rev = Object
counterScope Intervention = Object
counterScope Poison = Player
counterScope Rad = Player
counterScope Experience = Player
counterScope Lore = Object
counterScope Age = Object

public export
sameCounter : CounterKind -> CounterKind -> Bool
sameCounter (BoostCounter ap at) (BoostCounter bp bt) =
  Counter.sameDelta ap bp && Counter.sameDelta at bt
sameCounter (BoostCounter _ _) _ = False
sameCounter Stun Stun = True
sameCounter Stun _ = False
sameCounter Time Time = True
sameCounter Time _ = False
sameCounter (KeywordCounter a) (KeywordCounter b) = sameKeyword a b
sameCounter (KeywordCounter _) _ = False
sameCounter Charge Charge = True
sameCounter Charge _ = False
sameCounter Omen Omen = True
sameCounter Omen _ = False
sameCounter Spite Spite = True
sameCounter Spite _ = False
sameCounter Rev Rev = True
sameCounter Rev _ = False
sameCounter Intervention Intervention = True
sameCounter Intervention _ = False
sameCounter Poison Poison = True
sameCounter Poison _ = False
sameCounter Rad Rad = True
sameCounter Rad _ = False
sameCounter Experience Experience = True
sameCounter Experience _ = False
sameCounter Lore Lore = True
sameCounter Lore _ = False
sameCounter Age Age = True
sameCounter Age _ = False

public export
data CounterKindNamed : Kind -> Maybe CounterKind -> Type where
  KindUnnamed : CounterKindNamed k Nothing
  KindNamed : {0 c : CounterKind} ->
              {auto 0 sc : counterScope c = k} ->
              CounterKindNamed k (Just c)

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
chapterMarksOk : List ChapterNumber -> Bool
chapterMarksOk [] = False
chapterMarksOk [_] = True
chapterMarksOk (a :: b :: rest) =
  chapterOrd a < chapterOrd b && chapterMarksOk (b :: rest)

public export
data ChapterMarks : List ChapterNumber -> Type where
  MkChapterMarks : {auto 0 ok : chapterMarksOk ns = True} -> ChapterMarks ns

public export
record TypeLine where
  constructor MkTypeLine
  subs : List Subtype
  tys : List CardType

public export
lineHasType : CardType -> List CardType -> Bool
lineHasType t [] = False
lineHasType t (u :: us) = sameCT t u || lineHasType t us

public export
lineNonEmpty : TypeLine -> Bool
lineNonEmpty (MkTypeLine [] []) = False
lineNonEmpty (MkTypeLine _ _) = True


public export
subsFitLine : List Subtype -> List CardType -> Bool
subsFitLine [] tys = True
subsFitLine (s :: ss) tys =
  -- [CR#308.2]: kindred subtypes are the same set as creature subtypes,
  -- so a creature subtype fits a Kindred-typed line too.
  (lineHasType (subtypeType s) tys
     || (sameCT (subtypeType s) Creature && lineHasType Kindred tys))
  && subsFitLine ss tys



public export
typeRank : CardType -> Nat
typeRank Kindred = 0
typeRank Enchantment = 1
typeRank Artifact = 2
typeRank Land = 3
typeRank Creature = 4
typeRank Planeswalker = 5
typeRank Battle = 6
typeRank Instant = 7
typeRank Sorcery = 8

public export
ltNat : Nat -> Nat -> Bool
ltNat a b = leNat (S a) b

public export
typesOrdered : List CardType -> Bool
typesOrdered [] = True
typesOrdered (t :: []) = True
typesOrdered (t :: u :: ts) = ltNat (typeRank t) (typeRank u) &&
                              typesOrdered (u :: ts)

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
spellType : CardType -> Bool
spellType Instant = True
spellType Sorcery = True
spellType Creature = False
spellType Artifact = False
spellType Land = False
spellType Enchantment = False
spellType Planeswalker = False
spellType Battle = False
spellType Kindred = False

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

public export
data Placeable : Maybe CardType -> Zone -> Type where
  MkPlaceable : {auto 0 ok : destTypeOk ty z = True} -> Placeable ty z

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
data StatusWord : StatusVal c -> Type where
  MkStatusWord : {auto 0 ok : statusWordOk v = True} -> StatusWord v

public export
statusEventOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusEventOk Tapped = True
statusEventOk Untapped = True
statusEventOk Flipped = False
statusEventOk Unflipped = False
statusEventOk FaceUp = True
statusEventOk FaceDown = False
statusEventOk PhasedIn = True
statusEventOk PhasedOut = True

public export
data StatusEventVal : StatusVal c -> Type where
  MkStatusEventVal : {auto 0 ok : statusEventOk v = True} -> StatusEventVal v

-- [CR#702.26a]: phasing has no imperative wording of its own, only the
-- intransitive declarative ("target creature phases out").
public export
statusEffectOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusEffectOk Tapped = True
statusEffectOk Untapped = True
statusEffectOk Flipped = True
-- [CR#710.4]: flipping is a one-way process, so a permanent can never
-- become unflipped and no such instruction can exist.
statusEffectOk Unflipped = False
statusEffectOk FaceUp = True
statusEffectOk FaceDown = True
statusEffectOk PhasedIn = True
statusEffectOk PhasedOut = True

public export
data StatusEffectVal : StatusVal c -> Type where
  MkStatusEffectVal : {auto 0 ok : statusEffectOk v = True} -> StatusEffectVal v

public export
colorMember : Color -> List Color -> Bool
colorMember c [] = False
colorMember c (d :: ds) = sameColor c d || colorMember c ds

public export
colorsDistinct : List Color -> Bool
colorsDistinct [] = True
colorsDistinct (c :: cs) = not (colorMember c cs) && colorsDistinct cs

public export
data ColorsDistinct : List Color -> Type where
  MkColorsDistinct : {auto 0 ok : colorsDistinct cs = True} -> ColorsDistinct cs


public export
addedFits : Maybe CardType -> TypeLine -> Bool
addedFits subj (MkTypeLine [] tys) = True
addedFits subj (MkTypeLine (s :: ss) tys) =
  (lineHasType (subtypeType s) tys ||
   (case subj of
      Nothing => False
      Just t => sameCT (subtypeType s) t)) &&
  addedFits subj (MkTypeLine ss tys)

public export
data AddedFits : Maybe CardType -> TypeLine -> Type where
  MkAddedFits : {auto 0 ok : addedFits subj tl = True} -> AddedFits subj tl

public export
anyNewType : Maybe CardType -> List CardType -> Bool
anyNewType subj [] = False
anyNewType subj (t :: ts) = not (tyIs t subj) || anyNewType subj ts

public export
addsSomething : Maybe CardType -> TypeLine -> Bool
addsSomething subj (MkTypeLine [] tys) = anyNewType subj tys
addsSomething subj (MkTypeLine (_ :: _) tys) = True

public export
data AddsSomething : Maybe CardType -> TypeLine -> Type where
  MkAddsSomething : {auto 0 ok : addsSomething subj tl = True} ->
                    AddsSomething subj tl

public export
data LineNonEmpty : TypeLine -> Type where
  MkLineNonEmpty : {auto 0 ok : lineNonEmpty tl = True} -> LineNonEmpty tl

public export
retainable : CardType -> Bool
retainable Land = True
retainable Enchantment = True
retainable Creature = False
retainable Artifact = False
retainable Planeswalker = True
retainable Battle = False
retainable Kindred = False
-- [CR#205.1a]: instant and sorcery retain their card type automatically,
-- so no retention rider is needed for either.
retainable Instant = False
retainable Sorcery = False

public export
retentionOk : TypeLine -> Maybe CardType -> Bool
retentionOk tl Nothing = True
retentionOk (MkTypeLine _ []) (Just t) = False
retentionOk (MkTypeLine _ (u :: us)) (Just t) = retainable t

public export
data RetentionOk : TypeLine -> Maybe CardType -> Type where
  MkRetentionOk : {auto 0 ok : retentionOk tl ret = True} -> RetentionOk tl ret

public export
data TokenRider = EntersTapped | EntersAttacking

-- [CR#508.4]: an attacking creature is tapped, so EntersAttacking
-- requires EntersTapped and never precedes it.
public export
ridersOk : List TokenRider -> Bool
ridersOk [] = True
ridersOk [EntersTapped] = True
ridersOk [EntersTapped, EntersAttacking] = True
ridersOk _ = False

public export
data RidersOk : List TokenRider -> Type where
  MkRidersOk : {auto 0 ok : ridersOk rs = True} -> RidersOk rs

public export
lastType : List CardType -> Maybe CardType
lastType [] = Nothing
lastType [t] = Just t
lastType (_ :: ts) = lastType ts


public export
data TurnPart = Turn | Upkeep | EndStep | Combat | UntapStep | EndOfCombat
              | FirstMain | PostcombatMain | DrawStep
              | MainPhase

public export
sameTurnPart : TurnPart -> TurnPart -> Bool
sameTurnPart Turn Turn = True
sameTurnPart Turn _ = False
sameTurnPart Upkeep Upkeep = True
sameTurnPart Upkeep _ = False
sameTurnPart EndStep EndStep = True
sameTurnPart EndStep _ = False
sameTurnPart Combat Combat = True
sameTurnPart Combat _ = False
sameTurnPart UntapStep UntapStep = True
sameTurnPart UntapStep _ = False
sameTurnPart EndOfCombat EndOfCombat = True
sameTurnPart EndOfCombat _ = False
sameTurnPart FirstMain FirstMain = True
sameTurnPart FirstMain _ = False
sameTurnPart PostcombatMain PostcombatMain = True
sameTurnPart PostcombatMain _ = False
sameTurnPart DrawStep DrawStep = True
sameTurnPart DrawStep _ = False
sameTurnPart MainPhase MainPhase = True
sameTurnPart MainPhase _ = False

public export
sameMaybePart : Maybe TurnPart -> Maybe TurnPart -> Bool
sameMaybePart Nothing Nothing = True
sameMaybePart Nothing (Just _) = False
sameMaybePart (Just _) Nothing = False
sameMaybePart (Just a) (Just b) = sameTurnPart a b

public export
data Whose = Yours | ThatPlayers

namespace Owner
  public export
  data Owner = Yours | ThatPlayers | EachPlayers | EachOpponents
             | EachYours | AnOpponents
             | ThatTurns

public export
sameOwner : Owner -> Owner -> Bool
sameOwner Yours Yours = True
sameOwner Yours _ = False
sameOwner ThatPlayers ThatPlayers = True
sameOwner ThatPlayers _ = False
sameOwner EachPlayers EachPlayers = True
sameOwner EachPlayers _ = False
sameOwner EachOpponents EachOpponents = True
sameOwner EachOpponents _ = False
sameOwner EachYours EachYours = True
sameOwner EachYours _ = False
sameOwner AnOpponents AnOpponents = True
sameOwner AnOpponents _ = False
sameOwner ThatTurns ThatTurns = True
sameOwner ThatTurns _ = False

public export
data DurationEnd = StartOf TurnPart (Maybe Whose)
                 | EndOf TurnPart (Maybe Whose)

||| Which duration-adverbial constructions a span use may be written by.
||| `Unattested` = no line writes the phrase; `Unclaimed` = it is real
||| oracle English, but a clause this grammar does not construct.
public export
data SpanUse = Unattested | Unclaimed | GrantsControlAndTypeSet | GrantsTypesControlReplacementPermissionSetSwitchTypeSetAndLoss
             | KeywordGrantAndTypeSet | RestrictionsShieldsPermissionsAndDelays | GrantsRestrictionsReplacementBaseSetTypeSetAndLoss
             | ControlGrantAndPermission | GrantsRestrictionsControlPermissionTypeSetAndLoss
             | PermissionOnly

