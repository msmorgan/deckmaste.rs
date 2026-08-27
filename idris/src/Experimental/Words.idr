||| The lexical/taxonomy substrate: card types, zones, keywords, subtypes,
||| counters, statuses, bindings, payloads, mana, and designations.
module Experimental.Words

import public Data.List
import public Data.Maybe
import public Data.Nat
import public Data.So

%default total


||| [CR#205.2a] closes the card types by enumeration, so this catalog
||| ports the rule's whole set rather than the types a card has been
||| observed to write. Conspiracy, dungeon, phenomenon, plane, scheme and
||| vanguard are rows here for the same reason the other nine are: the
||| rule names them. Every total table below therefore answers for all
||| fifteen, and answers a command-zone type on a rule — [CR#110.4] for
||| permanence, the type's own "can't be cast" rule for casting.
public export
data CardType = Creature | Artifact | Land | Enchantment | Instant | Sorcery
              | Planeswalker | Battle | Kindred
              | Conspiracy | Dungeon | Phenomenon | Plane | Scheme | Vanguard

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
-- [CR#701.14a] has only creatures fight, and [CR#110.4] keeps the
-- command-zone types off the battlefield altogether.
combatant Conspiracy = False
combatant Dungeon = False
combatant Phenomenon = False
combatant Plane = False
combatant Scheme = False
combatant Vanguard = False

public export
data FightParticipant : Maybe CardType -> Type where
  Fighter : {auto 0 ok : So (combatant t)} -> FightParticipant (Just t)

public export
data Characteristic = Power | Toughness | ManaValue | Loyalty

public export
Eq Characteristic where
  (==) Power Power = True
  (==) Power _ = False
  (==) Toughness Toughness = True
  (==) Toughness _ = False
  (==) ManaValue ManaValue = True
  (==) ManaValue _ = False
  (==) Loyalty Loyalty = True
  (==) Loyalty _ = False

public export
data PlayerStat = LifeTotal | StartingLifeTotal

public export
Eq PlayerStat where
  (==) LifeTotal LifeTotal = True
  (==) LifeTotal _ = False
  (==) StartingLifeTotal StartingLifeTotal = True
  (==) StartingLifeTotal _ = False

public export
data Comparator = AtLeast | AtMost | Greater | Less | Eq

public export
Eq Comparator where
  (==) AtLeast AtLeast = True
  (==) AtLeast _ = False
  (==) AtMost AtMost = True
  (==) AtMost _ = False
  (==) Greater Greater = True
  (==) Greater _ = False
  (==) Less Less = True
  (==) Less _ = False
  (==) Eq Eq = True
  (==) Eq _ = False

public export
comparedType : Characteristic -> Maybe CardType
comparedType Power = Just Creature
comparedType Toughness = Just Creature
comparedType ManaValue = Nothing
-- [CR#306.5]: loyalty is a characteristic only planeswalkers have.
comparedType Loyalty = Just Planeswalker

public export
data QualitySort = Color | CreatureType | CardName | Number

public export
Eq QualitySort where
  (==) Color Color = True
  (==) Color _ = False
  (==) CreatureType CreatureType = True
  (==) CreatureType _ = False
  (==) CardName CardName = True
  (==) CardName _ = False
  (==) Number Number = True
  (==) Number _ = False

||| Whether `OfChosen` honestly reads a chosen sort back. Colour, creature
||| type and card name are all characteristics [CR#109.3], so "of the
||| chosen ..." has something on the object to match (Cheering Fanatic,
||| Nevermore). A bare number is not among them, so the object-side read
||| matches against nothing. Every printed use of "the chosen number"
||| reads it as an amount or against a numeric characteristic instead
||| ("mana value equal to the chosen number"); that amount-side read is a
||| separate node and is not yet minted.
public export
chosenQualityReadOk : QualitySort -> Bool
chosenQualityReadOk Color = True
chosenQualityReadOk CreatureType = True
chosenQualityReadOk CardName = True
chosenQualityReadOk Number = False

public export
ChosenQualityRead : QualitySort -> Type
ChosenQualityRead q = So (chosenQualityReadOk q)

||| The letters an ability may define. Closed at two by [CR#107.3p]:
||| "Some objects use the letter Y in addition to the letter X. Y follows
||| the same rules as X." No third letter exists in the rules.
public export
data Letter = X | Y

public export
Eq Letter where
  (==) X X = True
  (==) X Y = False
  (==) Y X = False
  (==) Y Y = True

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
  Ability : Kind
  ||| A joined kind: the term names either half ("any target" [CR#115.4],
  ||| "target spell or ability", "target creature or player").
  |||
  ||| THE JOIN IS SYNTAX -- a plain constructor, never reduced. The earlier
  ||| flat-vs-pair argument was a level confusion: the PAIR is `JoinP`, one
  ||| `Payload` per half, and the kind index only records that a join
  ||| happened. So no gate says which pairs may join; the rules make every
  ||| such phrase meaningful and the semantics refuses none.
  |||
  ||| Order is `kindLte`, not `=`: an `Object` anaphor resolves against an
  ||| `Object \/ Player` antecedent because `Object` sits below it.
  ||| Idempotence, commutativity and associativity hold up to that order.
  ||| The semilattice is UNBOUNDED by decision -- `Or` carries
  ||| `TwoDisjuncts`, so a heterogeneous fold seeds with a non-empty list's
  ||| head and needs no unit: no `Top`, no `Bottom`.
  (\/) : Kind -> Kind -> Kind

public export
Eq Kind where
  (==) Object Object = True
  (==) Object Player = False
  (==) Object (Quality _) = False
  (==) Object Outcome = False
  (==) Object Gap = False
  (==) Object (LetterK _) = False
  (==) Object TurnRef = False
  (==) Object Ability = False
  (==) Object (_ \/ _) = False
  (==) Player Object = False
  (==) Player Player = True
  (==) Player (Quality _) = False
  (==) Player Outcome = False
  (==) Player Gap = False
  (==) Player (LetterK _) = False
  (==) Player TurnRef = False
  (==) Player Ability = False
  (==) Player (_ \/ _) = False
  (==) (Quality _) Object = False
  (==) (Quality _) Player = False
  (==) (Quality a) (Quality b) = a == b
  (==) (Quality _) Outcome = False
  (==) (Quality _) Gap = False
  (==) (Quality _) (LetterK _) = False
  (==) (Quality _) TurnRef = False
  (==) (Quality _) Ability = False
  (==) (Quality _) (_ \/ _) = False
  (==) Outcome Object = False
  (==) Outcome Player = False
  (==) Outcome (Quality _) = False
  (==) Outcome Outcome = True
  (==) Outcome Gap = False
  (==) Outcome (LetterK _) = False
  (==) Outcome TurnRef = False
  (==) Outcome Ability = False
  (==) Outcome (_ \/ _) = False
  (==) Gap Object = False
  (==) Gap Player = False
  (==) Gap (Quality _) = False
  (==) Gap Outcome = False
  (==) Gap Gap = True
  (==) Gap (LetterK _) = False
  (==) Gap TurnRef = False
  (==) Gap Ability = False
  (==) Gap (_ \/ _) = False
  (==) (LetterK _) Object = False
  (==) (LetterK _) Player = False
  (==) (LetterK _) (Quality _) = False
  (==) (LetterK _) Outcome = False
  (==) (LetterK _) Gap = False
  (==) (LetterK a) (LetterK b) = a == b
  (==) (LetterK _) TurnRef = False
  (==) (LetterK _) Ability = False
  (==) (LetterK _) (_ \/ _) = False
  (==) TurnRef Object = False
  (==) TurnRef Player = False
  (==) TurnRef (Quality _) = False
  (==) TurnRef Outcome = False
  (==) TurnRef Gap = False
  (==) TurnRef (LetterK _) = False
  (==) TurnRef TurnRef = True
  (==) TurnRef Ability = False
  (==) TurnRef (_ \/ _) = False
  (==) Ability Object = False
  (==) Ability Player = False
  (==) Ability (Quality _) = False
  (==) Ability Outcome = False
  (==) Ability Gap = False
  (==) Ability (LetterK _) = False
  (==) Ability TurnRef = False
  (==) Ability Ability = True
  (==) Ability (_ \/ _) = False
  (==) (_ \/ _) Object = False
  (==) (_ \/ _) Player = False
  (==) (_ \/ _) (Quality _) = False
  (==) (_ \/ _) Outcome = False
  (==) (_ \/ _) Gap = False
  (==) (_ \/ _) (LetterK _) = False
  (==) (_ \/ _) TurnRef = False
  (==) (_ \/ _) Ability = False
  (==) (a \/ b) (c \/ d) = a == c && b == d

||| A `QualitySort` matches only itself.
public export
sameQRefl : (q : QualitySort) -> So (q == q)
sameQRefl Color = Oh
sameQRefl CreatureType = Oh
sameQRefl CardName = Oh
sameQRefl Number = Oh

||| `==` decides equality: a match is the identity of the two sorts.
public export
sameQEq : (a, b : QualitySort) -> So (a == b) -> a = b
sameQEq Color Color _ = Refl
sameQEq Color CreatureType ok = absurd ok
sameQEq Color CardName ok = absurd ok
sameQEq Color Number ok = absurd ok

sameQEq CreatureType Color ok = absurd ok
sameQEq CreatureType CreatureType _ = Refl
sameQEq CreatureType CardName ok = absurd ok
sameQEq CreatureType Number ok = absurd ok

sameQEq CardName Color ok = absurd ok
sameQEq CardName CreatureType ok = absurd ok
sameQEq CardName CardName _ = Refl
sameQEq CardName Number ok = absurd ok

sameQEq Number Color ok = absurd ok
sameQEq Number CreatureType ok = absurd ok
sameQEq Number CardName ok = absurd ok
sameQEq Number Number _ = Refl

||| A `Letter` matches only itself.
public export
sameLetterRefl : (w : Letter) -> So (w == w)
sameLetterRefl X = Oh
sameLetterRefl Y = Oh

||| `==` decides equality likewise.
public export
sameLetterEq : (a, b : Letter) -> So (a == b) -> a = b
sameLetterEq X X _ = Refl
sameLetterEq X Y ok = absurd ok
sameLetterEq Y X ok = absurd ok
sameLetterEq Y Y _ = Refl

||| Every kind matches itself.
public export
sameKindRefl : (k : Kind) -> So (k == k)
sameKindRefl Object = Oh
sameKindRefl Player = Oh
sameKindRefl (Quality q) = sameQRefl q
sameKindRefl Outcome = Oh
sameKindRefl Gap = Oh
sameKindRefl (LetterK w) = sameLetterRefl w
sameKindRefl TurnRef = Oh
sameKindRefl Ability = Oh
sameKindRefl (a \/ b) = andSo (sameKindRefl a, sameKindRefl b)

||| The kind ORDER, replacing the old join gate and join FUNCTION.
||| `kindLte x y` asks whether a term of kind `x` is admitted where kind
||| `y` was named: every half of a join on the left must fit, and a join
||| on the right is fitted by either half. Off the joins it is `==`, so
||| nothing changes at an unjoined kind.
|||
||| The left-join clause comes FIRST: that is what makes the definition
||| total and what makes `(a \/ b) <= c` mean "both halves fit". `&&`/`||`
||| are lazy; harmless, since a `So` gate only reduces at concrete kinds.
|||
||| This is a DECISION PROCEDURE, not a witness: it says an `Object`
||| anaphor may resolve against an `Object \/ Player` antecedent, but not
||| WHICH half it resolved to. The documented upgrade, if a reader ever
||| needs that half, is the inductive
|||
||| ```idris
||| data KindLte : Kind -> Kind -> Type where
|||   Same  : KindLte k k
|||   JoinL : KindLte a c -> KindLte b c -> KindLte (a \/ b) c
|||   InL   : KindLte x a -> KindLte x (a \/ b)
|||   InR   : KindLte x b -> KindLte x (a \/ b)
||| ```
|||
||| whose `InL`/`InR` carry it. Not minted now: no reader asks.
public export
kindLte : Kind -> Kind -> Bool
kindLte (a \/ b) y = kindLte a y && kindLte b y
kindLte x (a \/ b) = kindLte x a || kindLte x b
kindLte x y = x == y

||| Widening into the left half of a join.
public export
kindLteInL : (x, a, b : Kind) -> So (kindLte x a) -> So (kindLte x (a \/ b))
kindLteInL Object a b ok = orSo (Left ok)
kindLteInL Player a b ok = orSo (Left ok)
kindLteInL (Quality _) a b ok = orSo (Left ok)
kindLteInL Outcome a b ok = orSo (Left ok)
kindLteInL Gap a b ok = orSo (Left ok)
kindLteInL (LetterK _) a b ok = orSo (Left ok)
kindLteInL TurnRef a b ok = orSo (Left ok)
kindLteInL Ability a b ok = orSo (Left ok)
kindLteInL (p \/ q) a b ok =
  andSo (kindLteInL p a b (fst (soAnd ok)), kindLteInL q a b (snd (soAnd ok)))

||| Widening into the right half of a join.
public export
kindLteInR : (x, a, b : Kind) -> So (kindLte x b) -> So (kindLte x (a \/ b))
kindLteInR Object a b ok = orSo (Right ok)
kindLteInR Player a b ok = orSo (Right ok)
kindLteInR (Quality _) a b ok = orSo (Right ok)
kindLteInR Outcome a b ok = orSo (Right ok)
kindLteInR Gap a b ok = orSo (Right ok)
kindLteInR (LetterK _) a b ok = orSo (Right ok)
kindLteInR TurnRef a b ok = orSo (Right ok)
kindLteInR Ability a b ok = orSo (Right ok)
kindLteInR (p \/ q) a b ok =
  andSo (kindLteInR p a b (fst (soAnd ok)), kindLteInR q a b (snd (soAnd ok)))

||| Reflexivity: every kind fits itself, joins included.
public export
kindLteRefl : (k : Kind) -> So (kindLte k k)
kindLteRefl Object = Oh
kindLteRefl Player = Oh
kindLteRefl (Quality q) = sameQRefl q
kindLteRefl Outcome = Oh
kindLteRefl Gap = Oh
kindLteRefl (LetterK w) = sameLetterRefl w
kindLteRefl TurnRef = Oh
kindLteRefl Ability = Oh
kindLteRefl (a \/ b) =
  andSo (kindLteInL a a b (kindLteRefl a), kindLteInR b a b (kindLteRefl b))

||| Each half sits below the join: the upper-bound half of the lattice law.
public export
kindLteJoinL : (a, b : Kind) -> So (kindLte a (a \/ b))
kindLteJoinL a b = kindLteInL a a b (kindLteRefl a)

public export
kindLteJoinR : (a, b : Kind) -> So (kindLte b (a \/ b))
kindLteJoinR a b = kindLteInR b a b (kindLteRefl b)

||| Commutativity, up to the order: a join and its swap admit each other.
||| (They are distinct TERMS -- the join is syntax -- so this is not `=`.)
public export
kindLteComm : (a, b : Kind) -> So (kindLte (a \/ b) (b \/ a))
kindLteComm a b = andSo (kindLteJoinR b a, kindLteJoinL b a)

||| Associativity, up to the order, in both directions.
public export
kindLteAssocR : (a, b, c : Kind) ->
                So (kindLte ((a \/ b) \/ c) (a \/ (b \/ c)))
kindLteAssocR a b c =
  andSo (andSo (kindLteJoinL a (b \/ c),
                kindLteInR b a (b \/ c) (kindLteJoinL b c)),
         kindLteInR c a (b \/ c) (kindLteJoinR b c))

public export
kindLteAssocL : (a, b, c : Kind) ->
                So (kindLte (a \/ (b \/ c)) ((a \/ b) \/ c))
kindLteAssocL a b c =
  andSo (kindLteInL a (a \/ b) c (kindLteJoinL a b),
         andSo (kindLteInL b (a \/ b) c (kindLteJoinR a b),
                kindLteJoinR (a \/ b) c))

||| Transitivity, by induction on the left kind and then the middle one:
||| a join on the left splits into both halves, a join in the middle is
||| entered through whichever half the left kind fit, and two unjoined
||| kinds fit only by `==`, which is identity.
public export
kindLteTrans : (a, b, c : Kind) ->
               So (kindLte a b) -> So (kindLte b c) -> So (kindLte a c)
kindLteTrans (p \/ q) b c ab bc =
  andSo (kindLteTrans p b c (fst (soAnd ab)) bc,
         kindLteTrans q b c (snd (soAnd ab)) bc)

kindLteTrans Object (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans Object r c l (fst (soAnd bc))
  Right m => kindLteTrans Object s c m (snd (soAnd bc))
kindLteTrans Object Object c ab bc = bc
kindLteTrans Object Player c ab bc = absurd ab
kindLteTrans Object (Quality _) c ab bc = absurd ab
kindLteTrans Object Outcome c ab bc = absurd ab
kindLteTrans Object Gap c ab bc = absurd ab
kindLteTrans Object (LetterK _) c ab bc = absurd ab
kindLteTrans Object TurnRef c ab bc = absurd ab
kindLteTrans Object Ability c ab bc = absurd ab

kindLteTrans Player (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans Player r c l (fst (soAnd bc))
  Right m => kindLteTrans Player s c m (snd (soAnd bc))
kindLteTrans Player Object c ab bc = absurd ab
kindLteTrans Player Player c ab bc = bc
kindLteTrans Player (Quality _) c ab bc = absurd ab
kindLteTrans Player Outcome c ab bc = absurd ab
kindLteTrans Player Gap c ab bc = absurd ab
kindLteTrans Player (LetterK _) c ab bc = absurd ab
kindLteTrans Player TurnRef c ab bc = absurd ab
kindLteTrans Player Ability c ab bc = absurd ab

kindLteTrans (Quality _) (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans (Quality _) r c l (fst (soAnd bc))
  Right m => kindLteTrans (Quality _) s c m (snd (soAnd bc))
kindLteTrans (Quality _) Object c ab bc = absurd ab
kindLteTrans (Quality _) Player c ab bc = absurd ab
kindLteTrans (Quality q) (Quality r) c ab bc = case sameQEq q r ab of Refl => bc
kindLteTrans (Quality _) Outcome c ab bc = absurd ab
kindLteTrans (Quality _) Gap c ab bc = absurd ab
kindLteTrans (Quality _) (LetterK _) c ab bc = absurd ab
kindLteTrans (Quality _) TurnRef c ab bc = absurd ab
kindLteTrans (Quality _) Ability c ab bc = absurd ab

kindLteTrans Outcome (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans Outcome r c l (fst (soAnd bc))
  Right m => kindLteTrans Outcome s c m (snd (soAnd bc))
kindLteTrans Outcome Object c ab bc = absurd ab
kindLteTrans Outcome Player c ab bc = absurd ab
kindLteTrans Outcome (Quality _) c ab bc = absurd ab
kindLteTrans Outcome Outcome c ab bc = bc
kindLteTrans Outcome Gap c ab bc = absurd ab
kindLteTrans Outcome (LetterK _) c ab bc = absurd ab
kindLteTrans Outcome TurnRef c ab bc = absurd ab
kindLteTrans Outcome Ability c ab bc = absurd ab

kindLteTrans Gap (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans Gap r c l (fst (soAnd bc))
  Right m => kindLteTrans Gap s c m (snd (soAnd bc))
kindLteTrans Gap Object c ab bc = absurd ab
kindLteTrans Gap Player c ab bc = absurd ab
kindLteTrans Gap (Quality _) c ab bc = absurd ab
kindLteTrans Gap Outcome c ab bc = absurd ab
kindLteTrans Gap Gap c ab bc = bc
kindLteTrans Gap (LetterK _) c ab bc = absurd ab
kindLteTrans Gap TurnRef c ab bc = absurd ab
kindLteTrans Gap Ability c ab bc = absurd ab

kindLteTrans (LetterK _) (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans (LetterK _) r c l (fst (soAnd bc))
  Right m => kindLteTrans (LetterK _) s c m (snd (soAnd bc))
kindLteTrans (LetterK _) Object c ab bc = absurd ab
kindLteTrans (LetterK _) Player c ab bc = absurd ab
kindLteTrans (LetterK _) (Quality _) c ab bc = absurd ab
kindLteTrans (LetterK _) Outcome c ab bc = absurd ab
kindLteTrans (LetterK _) Gap c ab bc = absurd ab
kindLteTrans (LetterK v) (LetterK w) c ab bc = case sameLetterEq v w ab of Refl => bc
kindLteTrans (LetterK _) TurnRef c ab bc = absurd ab
kindLteTrans (LetterK _) Ability c ab bc = absurd ab

kindLteTrans TurnRef (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans TurnRef r c l (fst (soAnd bc))
  Right m => kindLteTrans TurnRef s c m (snd (soAnd bc))
kindLteTrans TurnRef Object c ab bc = absurd ab
kindLteTrans TurnRef Player c ab bc = absurd ab
kindLteTrans TurnRef (Quality _) c ab bc = absurd ab
kindLteTrans TurnRef Outcome c ab bc = absurd ab
kindLteTrans TurnRef Gap c ab bc = absurd ab
kindLteTrans TurnRef (LetterK _) c ab bc = absurd ab
kindLteTrans TurnRef TurnRef c ab bc = bc
kindLteTrans TurnRef Ability c ab bc = absurd ab

kindLteTrans Ability (r \/ s) c ab bc = case soOr ab of
  Left l => kindLteTrans Ability r c l (fst (soAnd bc))
  Right m => kindLteTrans Ability s c m (snd (soAnd bc))
kindLteTrans Ability Object c ab bc = absurd ab
kindLteTrans Ability Player c ab bc = absurd ab
kindLteTrans Ability (Quality _) c ab bc = absurd ab
kindLteTrans Ability Outcome c ab bc = absurd ab
kindLteTrans Ability Gap c ab bc = absurd ab
kindLteTrans Ability (LetterK _) c ab bc = absurd ab
kindLteTrans Ability TurnRef c ab bc = absurd ab
kindLteTrans Ability Ability c ab bc = bc

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
Eq AggregateOp where
  (==) SumOf SumOf = True
  (==) SumOf _ = False
  (==) MinOf MinOf = True
  (==) MinOf _ = False
  (==) MaxOf MaxOf = True
  (==) MaxOf _ = False

public export
data ProjAxis = CharAxis Characteristic | PlayerStatAxis PlayerStat

public export
projScope : ProjAxis -> Kind
projScope (CharAxis _) = Object
projScope (PlayerStatAxis _) = Player

public export
Eq ProjAxis where
  (==) (CharAxis a) (CharAxis b) = a == b
  (==) (CharAxis _) _ = False
  (==) (PlayerStatAxis a) (PlayerStatAxis b) = a == b
  (==) (PlayerStatAxis _) _ = False

||| Which of a card type's subtypes a distinct-kind count runs over.
||| [CR#305.6] names the five basic land types and says an object writing
||| "basic land type" means one of those, so the split belongs to the land
||| type alone; no other card type's subtypes are divided this way.
public export
data SubtypeScope = AnySubtype | BasicOnly | NonbasicOnly

public export
subtypeScopeOk : CardType -> SubtypeScope -> Bool
subtypeScopeOk _ AnySubtype = True
subtypeScopeOk Land BasicOnly = True
subtypeScopeOk Land NonbasicOnly = True
subtypeScopeOk _ BasicOnly = False
subtypeScopeOk _ NonbasicOnly = False

||| The label set a distinct-kind count runs over. Its own vocabulary
||| beside `ProjAxis`, not rows on it: a `ProjAxis` names ONE number per
||| object, where [CR#205.2b] gives one object more than one card type at
||| once and [CR#105.2] more than one colour, so an axis here names a SET
||| and nothing sums or orders its values.
||| `ValueAxis` is the same reading over a numeric characteristic -- "the
||| number of different mana values among cards in your graveyard" counts
||| the values, not the cards -- and is why a distinct count is not an
||| `AggregateOp`: its result does not depend on the axis being numeric.
public export
data KindAxis : Type where
  ||| "the number of card types among …" [CR#205.2a]
  CardTypeAxis : KindAxis
  ||| "the number of permanent types among …" [CR#110.4]
  PermanentTypeAxis : KindAxis
  ||| "the number of colors among …" [CR#105.1]
  ColorAxis : KindAxis
  ||| "the number of basic land types among …", "the number of creature
  ||| types among …": the subtypes correlated to one card type [CR#205.3c].
  SubtypeAxis : (host : CardType) -> (only : SubtypeScope) ->
                {auto 0 sc : So (subtypeScopeOk host only)} -> KindAxis
  ||| "the number of different mana values among …", "… different powers
  ||| among …": the values a numeric characteristic takes, counted once
  ||| each.
  ValueAxis : Characteristic -> KindAxis
  ||| "the number of different kinds of counters among …"
  CounterKindAxis : KindAxis
  ||| "the number of different color pairs among …": [CR#105.5]'s ten
  ||| pairs. Its own arm and not `ColorAxis` under a restriction, because
  ||| its values are PAIRS -- a permanent that is white and blue supplies
  ||| one value here and two on `ColorAxis`.
  ColorPairAxis : KindAxis

||| The sort at which a distributive pass over an axis BINDS the value it
||| is running over. `ForEachKindOf` carries it beside the axis, and the
||| body reads it back through the chosen-quality reads that already
||| exist, so the pass mints no read of its own.
||| `Nothing` is a vocabulary gap, never a refusal. [CR#205.2a]'s card
||| types, [CR#205.3e]'s subtypes off the creature type, [CR#105.5]'s
||| pairs and [CR#122.1]'s counter kinds are all nameable things, and the
||| corpus writes a choice over three of them ("choose a card type",
||| "choose a land type", "choose a kind of counter"); `QualitySort`
||| carries none of them yet.
public export
kindAxisSort : KindAxis -> Maybe QualitySort
kindAxisSort CardTypeAxis = Nothing
kindAxisSort PermanentTypeAxis = Nothing
kindAxisSort ColorAxis = Just Color
kindAxisSort (SubtypeAxis Creature _) = Just CreatureType
kindAxisSort (SubtypeAxis _ _) = Nothing
kindAxisSort (ValueAxis _) = Just Number
kindAxisSort CounterKindAxis = Nothing
kindAxisSort ColorPairAxis = Nothing

||| How many of [CR#105.1]'s five colours an object may be said to be
||| EXACTLY. Two is the floor because the lower counts have printed words
||| of their own -- [CR#105.2a]'s "monocolored" and [CR#105.2c]'s
||| "colorless" -- and five the ceiling because there is no sixth colour
||| to be.
public export
colorCountOk : Nat -> Bool
colorCountOk n = n >= 2 && n <= 5

public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost | CountersPut
                 | DamagePrevented | RollResult | CoinFlipped
                 -- the dice a roll CALLED FOR, which is not the number it
                 -- produced: [CR#706.1] has the instruction specify "how
                 -- many of those dice to roll", and the replacement side
                 -- reads that back off a roll that never happened
                 -- [CR#614.6] -- "instead roll THAT MANY dice plus one".
                 | DiceRolled
                 | NamedNumber
                 -- `RepeatCount` is the number of iterations a repetition
                 -- WROTE, not the size of the batch it produced:
                 -- [CR#609.3] has an effect do only as much as possible,
                 -- so a player told to discard three cards holding two
                 -- discards two. The batch's own size is read off its
                 -- summary mention (`GroupSize`).
                 | RepeatCount

||| Which reading of a flipped coin a clause takes. [CR#705.2] gives a
||| flip two and only two: the face it came up, and -- when the flipper
||| called it -- the winner. `FlipCall` is the called reading, which has
||| a subject, since the rule says only the player who flips the coin
||| wins or loses it.
public export
data FlipCall = WinsFlip | LosesFlip

||| The other reading: the side the coin landed on. [CR#705.1] fixes the
||| two sides and makes the grammar's pair exhaustive, and [CR#705.2]
||| says no player wins or loses a flip read this way, so the face takes
||| no subject.
public export
data CoinFace = Heads | Tails

||| Which end of a clause's rolls an ignore instruction names.
||| [CR#706.6] writes the superlative form itself -- "If that player was
||| instructed to ignore the lowest roll and multiple results are tied
||| for the lowest, the player chooses one of those rolls to be ignored"
||| -- so the extreme is what the rule settles, and the two ends are one
||| pair on `CoinFace`'s model. "The lower roll" over two dice is the
||| two-item spelling of the same end, not a third word.
public export
data RollExtreme = LowestRoll | HighestRoll

||| Which rolls an ignore instruction sets aside [CR#706.6]: the extreme
||| itself, or everything but it. Both arms are printed -- "ignore the
||| lower roll" against "ignore all but the highest roll" -- and neither
||| is a spelling of the other, since a clause that rolled more than two
||| dice keeps a different number of rolls under each.
public export
data IgnoredRolls = IgnoreExtreme RollExtreme | IgnoreAllBut RollExtreme

public export
data Causer = AnEffect

public export
Eq Causer where
  (==) AnEffect AnEffect = True

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

-- `Quantity` and its four gates moved to Experimental.idr's mutual block
-- the day a bound became able to carry a written AMOUNT; see the type's
-- own docstring there.

||| Which way the names inside a counted group line up. Both poles are one
||| word class: [CR#201.2b] states the negative one over a group -- those
||| objects have different names only if no two of them have a name in
||| common -- and [CR#201.2a] the positive one, two or more objects having
||| at least one name in common. Neither pole carries a relatum, because
||| the comparison is group-internal: "with the same name as one another"
||| and the elliptical "with the same name" state the same relation and
||| differ only in whether English writes the reciprocal out.
public export
data NameAgreement : Type where
  DifferentNames : NameAgreement
  SameName : NameAgreement

||| [CR#700.2]: a spell or ability is modal only if it has two or more
||| instructions to choose among, so a one-mode list offers no choice.
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
Eq Zone where
  (==) Battlefield Battlefield = True
  (==) Battlefield _ = False
  (==) Graveyard Graveyard = True
  (==) Graveyard _ = False
  (==) Exile Exile = True
  (==) Exile _ = False
  (==) Hand Hand = True
  (==) Hand _ = False
  (==) Library Library = True
  (==) Library _ = False
  (==) Stack Stack = True
  (==) Stack _ = False

public export
data LibPos = OnTop | OnBottom

public export
data Arrangement = AnyOrder | RandomOrder

||| The ordinal occurrence word: WHICH member of a sequence, counted from
||| the first — not how many. One vocabulary for its three sites: the
||| library offset ([CR#401.7] states "Nth from the top" for any N, so the
||| position is a number and not a closed vocabulary — the ordinals cards
||| have printed are spelling, and the frame is the rule's), the ordinal
||| counter event, and the ordinal cast restriction. `Nth 0` is the one
||| refusal — a sequence's first member is its 1st, and [CR#401.7] counts
||| library positions from the top card, which is the first.
public export
data Ordinal : Type where
  Nth : (n : Nat) -> {auto 0 nz : IsSucc n} -> Ordinal

||| The library site's old name, kept as the same type so its use sites
||| read unchanged.
public export
LibOrdinal : Type
LibOrdinal = Ordinal

||| The NAME a clause writes instead of spelling its effect out: "discard",
||| "exile", "scry". [CR#701.1] holds that a verb the rules do not keyword
||| "use[s] the standard English definition", and the rules that do keyword
||| one state its expansion -- [CR#701.9a] defines discarding AS moving a
||| card from its owner's hand to that player's graveyard. So the MEANING
||| lives in the expansion a macro builds, and the label only names which
||| keyword action that expansion performs.
|||
||| That makes the vocabulary OPEN. A label needs no row in a total table
||| and no rules entry of its own: a new keyword action is a new macro
||| plus a new label, never a core enum row plus a coverage re-decide. The
||| label is still typo-checked, at every gate that writes one, by
||| `KnownVerb` against `verbFacts` -- and `verbFacts` is DATA, one row per
||| label, carrying only what the expansion cannot say for itself.
|||
||| What the label buys downstream is granularity and provenance, both of
||| which are the engine's to apply and neither of which the term shape
||| encodes: draws are individual events [CR#121.2] while a multi-card
||| discard is one event with several occurrences [CR#603.2c], and the
||| stamp a labeled action leaves is what the participle anaphors read
||| back.
public export
VerbLabel : Type
VerbLabel = String

||| What a keyword action's label records beyond its expansion. The first
||| two fields are SPELLING. The last two are the action's own rule
||| speaking, on `KeywordFacts`' model, for the one reader that meets a
||| label with no expansion written: the EVENT reading, whose tables are
||| keyed by the event's name and have nothing but this data to ask.
public export
record VerbFacts where
  constructor MkVerbFacts
  ||| the label as a clause writes it
  label : VerbLabel
  ||| the participle a verbed anaphor spells ("the exiled card", "those
  ||| cards destroyed this way"), when the keyword action leaves a
  ||| patient a later clause can name that way; the two markings share
  ||| it, since both spell the same participle and differ only in where
  ||| the phrase puts it. `Nothing` where the action leaves no such
  ||| patient (scry and surveil name none) or where the marking would
  ||| have to name the destination too ("put onto the battlefield this
  ||| way"), never merely because a participle exists.
  participle : Maybe String
  ||| the participant the action's own rule performs it ON, when the rule
  ||| names one: [CR#701.9a] discards a card, [CR#701.8a] destroys a
  ||| permanent. `Nothing` where the rule's own statement takes a number
  ||| instead ([CR#701.22a,701.25a]) or where what a printed line writes
  ||| after the verb is a zone, which is no `Kind` here. It is what a
  ||| verbed event announces and what a participial lookback names.
  actPatient : Maybe Kind
  ||| where the action's own rule leaves that patient: a graveyard for
  ||| [CR#701.9a] and [CR#701.17a], the exile zone for [CR#701.13a].
  ||| `Nothing` where the rule moves nothing ([CR#701.26a] turns a
  ||| permanent sideways) or states no destination of its own, in which
  ||| case the patient stays in the zone it was named in.
  actDest : Maybe Zone

||| The label vocabulary, open by construction: a row is a name, its
||| participle and what its own rule says about the act, and adding one
||| adds no obligation anywhere else.
public export
verbFacts : List VerbFacts
verbFacts =
  --                                       patient       destination
  [ MkVerbFacts "Destroy"   (Just "destroyed")
                                           (Just Object) (Just Graveyard)
  , MkVerbFacts "Sacrifice" (Just "sacrificed")
                                           (Just Object) (Just Graveyard)
  , MkVerbFacts "Exile"     (Just "exiled")
                                           (Just Object) (Just Exile)
  , MkVerbFacts "Discard"   (Just "discarded")
                                           (Just Object) (Just Graveyard)
  , MkVerbFacts "Mill"      (Just "milled")
                                           (Just Object) (Just Graveyard)
  -- [CR#701.22b] and [CR#701.25c] both name a trigger on the act, and
  -- both rules write a NUMBER of cards looked at rather than a patient
  -- the act carries off, so the event announces no such thing.
  , MkVerbFacts "Scry"      Nothing        Nothing       Nothing
  , MkVerbFacts "Surveil"   Nothing        Nothing       Nothing
  , MkVerbFacts "Tap"       (Just "tapped")
                                           (Just Object) Nothing
  -- "Put" is no [CR#701] keyword action, and under labels that is
  -- unremarkable: a label needs no rules entry of its own, because its
  -- body speaks for it [CR#701.1]. Its destination is the clause's, so
  -- the label states none.
  , MkVerbFacts "Put"       Nothing        (Just Object) Nothing
  -- the stamp a search leaves is read by the shuffle gate, not by a
  -- participle anaphor: no printed line names a search's patient that
  -- way, and "the searched card" is not what English would spell. What a
  -- printed line writes after the verb is the ZONE [CR#701.23a] has the
  -- act look in; the card it finds is named by the instructing clause.
  , MkVerbFacts "Search"    Nothing        Nothing       Nothing
  ]

public export
factsIn : VerbLabel -> List VerbFacts -> Maybe VerbFacts
factsIn v [] = Nothing
factsIn v (f :: fs) = if label f == v then Just f else factsIn v fs

public export
verbFactsFor : VerbLabel -> Maybe VerbFacts
verbFactsFor v = factsIn v verbFacts

||| The membership gate: typo-safety without a per-label type.
public export
knownVerb : VerbLabel -> Bool
knownVerb v = isJust (verbFactsFor v)

public export
KnownVerb : VerbLabel -> Type
KnownVerb v = So (knownVerb v)

||| The participle a verbed anaphor spells, when the label has one.
public export
participleOf : VerbLabel -> Maybe String
participleOf v = verbFactsFor v >>= participle

||| The participant a verbed event announces, when the action's own rule
||| names one.
public export
actPatientOf : VerbLabel -> Maybe Kind
actPatientOf v = verbFactsFor v >>= actPatient

||| Whether the act names a patient at all: what decides between the
||| transitive reading ("you discard a card") and the intransitive one
||| ("you scry").
public export
actNamesPatient : VerbLabel -> Bool
actNamesPatient v = isJust (actPatientOf v)

||| Where the action's own rule leaves its patient, when it moves it.
public export
actDestOf : VerbLabel -> Maybe Zone
actDestOf v = verbFactsFor v >>= actDest

public export
record Stamp where
  constructor MkStamp
  verb : VerbLabel
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
  LetterP : Payload (LetterK l)
  TurnRefP : Payload TurnRef
  AbilityP : Payload Ability
  ||| A union mention carries what it knows about EACH half -- "target
  ||| creature or player" is `JoinP (ObjectP (Just Creature) ...) PlayerP`
  ||| -- and that pair is what the demonstrative echo reads back.
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
Eq CardType where
  (==) Creature Creature = True
  (==) Creature _ = False
  (==) Artifact Artifact = True
  (==) Artifact _ = False
  (==) Land Land = True
  (==) Land _ = False
  (==) Enchantment Enchantment = True
  (==) Enchantment _ = False
  (==) Instant Instant = True
  (==) Instant _ = False
  (==) Sorcery Sorcery = True
  (==) Sorcery _ = False
  (==) Planeswalker Planeswalker = True
  (==) Planeswalker _ = False
  (==) Battle Battle = True
  (==) Battle _ = False
  (==) Kindred Kindred = True
  (==) Kindred _ = False
  (==) Conspiracy Conspiracy = True
  (==) Conspiracy _ = False
  (==) Dungeon Dungeon = True
  (==) Dungeon _ = False
  (==) Phenomenon Phenomenon = True
  (==) Phenomenon _ = False
  (==) Plane Plane = True
  (==) Plane _ = False
  (==) Scheme Scheme = True
  (==) Scheme _ = False
  (==) Vanguard Vanguard = True
  (==) Vanguard _ = False

||| The zone a payload places its referent in. A join is placeless
||| unless one half places itself: only the Object half ever carries a
||| zone, so the join reports that half's, and a join with no Object half
||| reports none.
public export
payloadZone : Payload k -> Maybe Zone
payloadZone (ObjectP _ zn _ _) = zn
payloadZone PlayerP = Nothing
payloadZone QualityP = Nothing
payloadZone (OutcomeP _) = Nothing
payloadZone GapP = Nothing
payloadZone LetterP = Nothing
payloadZone TurnRefP = Nothing
payloadZone AbilityP = Nothing
payloadZone (JoinP l r) = maybe (payloadZone r) Just (payloadZone l)

||| One head type for a two-half phrase: the type its halves agree on, and
||| the typed half's where the other names none — a player half names no
||| card type, so a cross-kind head still reports its object half's. Halves
||| naming DIFFERENT types have no single head, and picking one would be
||| arbitrary, so the collapse reports none and a gate that must ask each
||| half reads the pair. This is `Or`'s rule for its disjuncts, weakened
||| only where a half is silent.
public export
joinSeed : Maybe CardType -> Maybe CardType -> Maybe CardType
joinSeed Nothing t = t
joinSeed t Nothing = t
joinSeed (Just t) (Just u) = if t == u then Just t else Nothing

||| The card type a payload names, collapsed by `joinSeed` where the
||| mention was joined.
public export
payloadTy : Payload k -> Maybe CardType
payloadTy (ObjectP ty _ _ _) = ty
payloadTy PlayerP = Nothing
payloadTy QualityP = Nothing
payloadTy (OutcomeP _) = Nothing
payloadTy GapP = Nothing
payloadTy LetterP = Nothing
payloadTy TurnRefP = Nothing
payloadTy AbilityP = Nothing
payloadTy (JoinP l r) = joinSeed (payloadTy l) (payloadTy r)

public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ pl) = payloadZone pl

public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ pl) = payloadTy pl

||| The head type a phrase projects onto EACH half of its kind -- the
||| description-side twin of `Payload`'s `JoinP` pair, and read the same
||| way. A phrase that is not itself a join names ONE description, which
||| every half of a joined kind then shares; a joined head names one per
||| half. A gate that asks each half about its own type reads this, where
||| one that wants a single answer reads the collapsing `seedTy`/`nounTy`.
public export
data HeadTy : Kind -> Type where
  SoleTy : Maybe CardType -> HeadTy k
  JoinTy : HeadTy ka -> HeadTy kb -> HeadTy (ka \/ kb)

public export
Eq OutcomeSort where
  (==) DamageDealt DamageDealt = True
  (==) DamageDealt _ = False
  (==) LifeGained LifeGained = True
  (==) LifeGained _ = False
  (==) LifeLost LifeLost = True
  (==) LifeLost _ = False
  (==) CountersPut CountersPut = True
  (==) CountersPut _ = False
  (==) DamagePrevented DamagePrevented = True
  (==) DamagePrevented _ = False
  (==) RollResult RollResult = True
  (==) RollResult _ = False
  (==) CoinFlipped CoinFlipped = True
  (==) CoinFlipped _ = False
  (==) DiceRolled DiceRolled = True
  (==) DiceRolled _ = False
  (==) NamedNumber NamedNumber = True
  (==) NamedNumber _ = False
  (==) RepeatCount RepeatCount = True
  (==) RepeatCount _ = False

public export
outcomeB : OutcomeSort -> Binding
outcomeB s = MkBinding TheD Outcome OneOf (OutcomeP s)

public export
gapB : Binding
gapB = MkBinding TheD Gap OneOf GapP

public export
turnRefB : Binding
turnRefB = MkBinding TheD TurnRef OneOf TurnRefP

||| The introducing mention of a letter: indefinite, awaiting "where X is".
||| A cost's own {X} mints this form too, not a definite one: [CR#107.3c]
||| reads "an {X}, [-X], or X in its cost and/or its text" together and
||| lets the text define the value of a letter the cost writes, so a cost
||| X is open to a later definition and only becomes definite without one,
||| by its controller's announcement [CR#107.3a].
public export
letterB : Letter -> Binding
letterB l = MkBinding AD (LetterK l) OneOf LetterP

public export
qualityB : QualitySort -> Binding
qualityB q = MkBinding AD (Quality q) OneOf QualityP

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

||| Which die a rolling instruction names. [CR#706.1] has such an effect
||| "specify what kind of die to roll", and [CR#706.1a] fixes a written
||| kind as N equally likely outcomes numbered from 1 to N with N
||| positive -- the gate on the written arm.
||| The anaphoric arm names no kind of its own; it takes the one an
||| announced roll already carried, the way [CR#706.3c] defines "Roll
||| again" as using "the same kind of and number of dice originally
||| called for". It is what the replacement side writes -- "instead roll
||| that many DICE plus one", where the bare word repeats the replaced
||| roll's kind -- so it is gated on that roll's announcement.
||| -- spelling: with `SidesOf`, "d[n]" or "[n]-sided"; with `ThoseDice`,
||| the bare "dice", or "them" when the count is anaphoric too.
public export
data DieSides : Bindings -> Type where
  SidesOf : (n : Nat) -> {auto 0 nz : IsSucc n} -> DieSides bs
  ThoseDice : {auto 0 ok : countOutcomes DiceRolled bs = 1} -> DieSides bs

||| Whether the discourse carries damage some clause has dealt -- the whole
||| licence a "...this way" back-reference needs. [CR#608.2c] is what makes
||| the reading available (later text may modify the meaning of earlier
||| text, one of its two worked examples being a "...this way"
||| back-reference) and it leaves WHICH earlier instruction the phrase names
||| to the reader, so this is an existence test. Not a count: a text may
||| carry two damage mentions at once, and not a head test either, since an
||| intervening clause displaces the head without making "this way"
||| unreadable.
public export
damageDealtInScope : Bindings -> Bool
damageDealtInScope [] = False
damageDealtInScope (MkBinding _ Outcome OneOf (OutcomeP DamageDealt) :: _) = True
damageDealtInScope (_ :: bs) = damageDealtInScope bs

||| Whether the discourse carries a coin some clause flipped -- the whole
||| licence either reading of a flip needs [CR#705.2]. An existence test
||| for the reason `damageDealtInScope` is one: a text may flip several
||| coins ("Flip five coins"), and which flip an arm names is left to
||| whoever reads the card.
public export
coinFlipInScope : Bindings -> Bool
coinFlipInScope [] = False
coinFlipInScope (MkBinding _ Outcome OneOf (OutcomeP CoinFlipped) :: _) = True
coinFlipInScope (_ :: bs) = coinFlipInScope bs

||| Whether an outcome mention leaves a NUMBER behind for a quantity read
||| to name. Damage, life, counters and a die's result each do --
||| [CR#706.2] makes the number on the die the result of the roll -- and a
||| coin flip does not: [CR#705.2] gives the flip a face and, when it was
||| called, a winner, and the rules give it nothing else, so "that much"
||| after a bare flip names no value.
public export
outcomeIsQuantity : OutcomeSort -> Bool
outcomeIsQuantity DamageDealt = True
outcomeIsQuantity LifeGained = True
outcomeIsQuantity LifeLost = True
outcomeIsQuantity CountersPut = True
outcomeIsQuantity DamagePrevented = True
outcomeIsQuantity RollResult = True
outcomeIsQuantity CoinFlipped = False
-- the dice a roll called for are a number the instruction wrote
-- [CR#706.1], which is what "that many dice" reads.
outcomeIsQuantity DiceRolled = True
-- a defining sentence names a number outright [CR#604.3,208.1], so the
-- anaphor that reads a quantity back ("that number") has one to name.
outcomeIsQuantity NamedNumber = True
-- so does a repetition: the text wrote how many times.
outcomeIsQuantity RepeatCount = True

||| What "that much" folds: the outcome mentions that carry a number.
public export
countQuantOutcomes : Bindings -> Nat
countQuantOutcomes [] = Z
countQuantOutcomes (MkBinding _ Outcome OneOf (OutcomeP s) :: bs) =
  if outcomeIsQuantity s then S (countQuantOutcomes bs) else countQuantOutcomes bs
countQuantOutcomes (_ :: bs) = countQuantOutcomes bs

public export
countQuality : QualitySort -> Bindings -> Nat
countQuality q [] = Z
countQuality q (MkBinding _ k OneOf _ :: bs) =
  if kindLte (Quality q) k then S (countQuality q bs) else countQuality q bs
countQuality q (_ :: bs) = countQuality q bs

public export
data ChoiceStands : Nat -> Type where
  ChoiceMade : ChoiceStands (S n)

||| Every mention of the letter, introduced or defined alike.
public export
countLetter : Letter -> Bindings -> Nat
countLetter l [] = Z
countLetter l (MkBinding _ k OneOf _ :: bs) =
  if kindLte (LetterK l) k then S (countLetter l bs) else countLetter l bs
countLetter l (_ :: bs) = countLetter l bs

||| An introduced letter still awaiting its definition. The determiner is
||| matched first so the table reduces under an abstract plurality.
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

public export
anyOpenLetter : Letter -> Bindings -> Bool
anyOpenLetter l [] = False
anyOpenLetter l (b :: bs) = openLetter l b || anyOpenLetter l bs

||| "where X is ..." settles EVERY open X into the definite: [CR#107.3i]
||| gives all instances of X on an object one value, so the X written
||| twice in "+X/+X" is one variable and one definition closes it.
public export
defineLetter : Letter -> Bindings -> Bindings
defineLetter l [] = []
defineLetter l (b :: bs) =
  if openLetter l b then MkBinding TheD b.kind b.plur b.payload :: defineLetter l bs
                    else b :: defineLetter l bs

||| What a letter mention contributes: the introduction, when the prefix
||| holds no such letter yet; nothing, when it reads one already there.
public export
letterDelta : Letter -> Bindings -> List Binding
letterDelta l bs = if countLetter l bs == 0 then [letterB l] else []

||| Any group mention, whatever it is a group of: "one or more opponents"
||| leaves a size to read back as surely as a group of objects does.
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

||| An assembled group of objects. `kindLte` is the test, not `==`: a
||| joined group ("two targets") is a group of objects too.
public export
objGroup : Binding -> Bool
objGroup b = kindLte Object b.kind && not (isOne b.plur)

public export
countGroups : Bindings -> Nat
countGroups [] = Z
countGroups (MkBinding PartD _ _ _ :: bs) = countGroups bs
countGroups (b :: bs) =
  if objGroup b then S (countGroups bs) else countGroups bs

public export
countParts : Bindings -> Nat
countParts [] = Z
countParts (b@(MkBinding PartD _ _ _) :: bs) =
  if kindLte Object b.kind then S (countParts bs) else countParts bs
countParts (_ :: bs) = countParts bs

||| "the rest" presupposes a set to be the rest OF and at least one part
||| already taken out of it -- nothing taken, and the text would have
||| written "them". The set is EITHER a group the text assembled ("one of
||| them") or the description a choice partitioned as it resolved:
||| [CR#608.2d] has the player announce a choice while applying the
||| effect and forbids an option that is illegal or impossible, so the
||| choice picks its members out of the described set and leaves the
||| unchosen behind. A choice's set is never mentioned, so the assembled
||| groups may number none; what may not be none is the part.
public export
theRestOk : Bindings -> Bool
theRestOk bs = countGroups bs <= 1 && not (countParts bs == Z)

||| What a disposal of "the rest" spends: the remainder itself. The
||| assembled group goes, and the parts stop being parts -- the members
||| they name are still there to read, but nothing is outstanding, so a
||| second "the rest" finds no partition to be the rest of. One
||| disposition per remainder, in both shapes of the licence: the text's
||| own group, and a choice's [CR#608.2d].
public export
groupSpent : Bindings -> Bindings
groupSpent [] = []
groupSpent (MkBinding PartD k pl p :: bs) = MkBinding TheD k pl p :: groupSpent bs
groupSpent (b :: bs) = if objGroup b then groupSpent bs else b :: groupSpent bs

||| The binding "the rest" reads its description off: the assembled
||| group when the text wrote one, and otherwise the part a choice took,
||| which carries the description the choice was made under.
public export
restSource : Bindings -> Maybe Binding
restSource [] = Nothing
restSource (b@(MkBinding PartD _ _ _) :: bs) =
  case restSource bs of
    Just s => Just s
    Nothing => if kindLte Object b.kind then Just b else Nothing
restSource (b :: bs) = if objGroup b then Just b else restSource bs

public export
zoneOfGroup : Bindings -> Maybe Zone
zoneOfGroup bs = restSource bs >>= bindingZone

public export
tyOfGroup : Bindings -> Maybe CardType
tyOfGroup bs = restSource bs >>= bindingTy

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

||| The bindings of kind `Outcome` alone: the quantities a clause wrote,
||| which an alternative arm may still name as "that much".
public export
outcomesOnly : Bindings -> Bindings
outcomesOnly [] = []
outcomesOnly (b@(MkBinding _ Outcome _ _) :: bs) = b :: outcomesOnly bs
outcomesOnly (_ :: bs) = outcomesOnly bs

||| Structural agreement on the discourse: whether two announcement
||| telescopes are one announcement. Decidable outright — a `Binding` is
||| closed first-order data — and read by the coordinated trigger header,
||| whose tail may see only the announcement its two arms share.
public export
sameDet : Determiner -> Determiner -> Bool
sameDet TargetD TargetD = True
sameDet TargetD _ = False
sameDet AD AD = True
sameDet AD _ = False
sameDet EachD EachD = True
sameDet EachD _ = False
sameDet AllD AllD = True
sameDet AllD _ = False
sameDet TheD TheD = True
sameDet TheD _ = False
sameDet PartD PartD = True
sameDet PartD _ = False
sameDet CountD CountD = True
sameDet CountD _ = False
sameDet SelfD SelfD = True
sameDet SelfD _ = False

public export
samePlur : Plurality -> Plurality -> Bool
samePlur OneOf OneOf = True
samePlur OneOf ManyOf = False
samePlur ManyOf OneOf = False
samePlur ManyOf ManyOf = True

public export
sameStamp : Stamp -> Stamp -> Bool
sameStamp (MkStamp v f) (MkStamp w g) = v == w && f == g

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

||| Heterogeneously indexed on purpose: the binding's `kind` field is
||| compared separately by `sameBinding`, so two payloads are compared by
||| constructor and field alone.
public export
samePayload : {0 j, k : Kind} -> Payload j -> Payload k -> Bool
samePayload (ObjectP ty zn pv og) (ObjectP ty' zn' pv' og') =
  sameMaybeBy (==) ty ty' && sameMaybeBy (==) zn zn' &&
  sameMaybeBy sameStamp pv pv' && sameMaybeBy sameOrigin og og'
samePayload (ObjectP _ _ _ _) _ = False
samePayload PlayerP PlayerP = True
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
samePayload AbilityP AbilityP = True
samePayload AbilityP _ = False
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
exposableZone Library = True
exposableZone Stack = False

public export
ExposableZone : Zone -> Type
ExposableZone z = So (exposableZone z)

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
VisibilityOk : ExposeVerb -> VisibleThing -> Type
VisibilityOk v w = So (visibilityOk v w)

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
pubB (MkBinding _ _ _ (JoinP _ _)) = True         -- a target is public whichever half it is

public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) = if pubB b then b :: publicOnly bs else publicOnly bs


||| One mention, read as the BATCH of everything an iterated body did to
||| its kind: same determiner, same kind, same payload -- so the same
||| stamp, the same zone and the same head type -- differing only in
||| naming many where the single pass named one.
|||
||| Identity on the deictic self: a clause repeated over "this permanent"
||| acts on one and the same permanent every pass, so its batch has one
||| member and the mention stays singular.
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

public export
pluralizeDelta : Bindings -> Bindings
pluralizeDelta [] = []
pluralizeDelta (b :: bs) = pluralizeBinding b :: pluralizeDelta bs


||| [CR#122.2]: counters on an object cease to exist when it moves from
||| one zone to another, so a clause that reads counters off a referent an
||| earlier clause moved names none [CR#400.7]. No table over labels is
||| consulted, and none could be: a stamp is written only where a labeled
||| action MOVED its patient (`moveIntro`), so carrying one is already the
||| evidence that the referent changed zones.
public export
stampMoves : Maybe Stamp -> Bool
stampMoves Nothing = False
stampMoves (Just _) = True

||| The stamp a payload was left with, read like `payloadZone`.
public export
payloadProv : Payload k -> Maybe Stamp
payloadProv (ObjectP _ _ pv _) = pv
payloadProv PlayerP = Nothing
payloadProv QualityP = Nothing
payloadProv (OutcomeP _) = Nothing
payloadProv GapP = Nothing
payloadProv LetterP = Nothing
payloadProv TurnRefP = Nothing
payloadProv AbilityP = Nothing
payloadProv (JoinP l r) = maybe (payloadProv r) Just (payloadProv l)

||| An `It`/`Them` anaphor is of kind `Object`, and `kindLte` is what
||| lets it land on a JOINED antecedent: "any target ... that permanent
||| or player" resolves `Object` against `Object \/ Player`.
public export
itReaches : Plurality -> Binding -> Bool
itReaches pl b = kindLte Object b.kind && isOne pl == isOne b.plur

public export
provOfIt : Bindings -> Maybe Stamp
provOfIt [] = Nothing
provOfIt (b :: bs) =
  if itReaches OneOf b then payloadProv b.payload else provOfIt bs

public export
provOfThem : Bindings -> Maybe Stamp
provOfThem [] = Nothing
provOfThem (b :: bs) =
  if itReaches ManyOf b then payloadProv b.payload else provOfThem bs

public export
zoneOfIt : Bindings -> Maybe Zone
zoneOfIt [] = Nothing
zoneOfIt (b :: bs) =
  if itReaches OneOf b then bindingZone b else zoneOfIt bs

public export
zoneOfThem : Bindings -> Maybe Zone
zoneOfThem [] = Nothing
zoneOfThem (b :: bs) =
  if itReaches ManyOf b then bindingZone b else zoneOfThem bs

public export
data NounWord = TypeW CardType | CardW | SpellW | PlayerW
              | PermanentW | TokenW | CopyW | JoinW

public export
data VerbedMarking = Attributive | ThisWay

||| Whether the participle read is spellable at all: it is exactly when
||| the label's data carries a participle to spell, and both markings
||| stand or fall together because both spell that one word.
public export
verbedMarkingOk : VerbLabel -> VerbedMarking -> Bool
verbedMarkingOk v _ = isJust (participleOf v)

public export
VerbedMarkingOk : VerbLabel -> VerbedMarking -> Type
VerbedMarkingOk v m = So (verbedMarkingOk v m)

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

||| The provenance a LABELED action leaves on the mention it acted on,
||| whatever the action did to it: a move writes one ("the exiled card"),
||| and so does a status change ("each creature tapped this way"). What
||| the stamp records is that this label acted on this referent, plus
||| whether it found it on the battlefield -- never which kind of body
||| the label rode. Body shapes past those two get a row when a printed
||| line needs one.
public export
mkStamp : Maybe VerbLabel -> Maybe Zone -> Maybe Stamp
mkStamp Nothing oldZn = Nothing
mkStamp (Just v) oldZn = Just (MkStamp v (onFieldZone oldZn))

||| Which word reaches ONE HALF of a union mention, where `JoinW` reads
||| the whole. The two halves take DIFFERENT gates. The class half ECHOES
||| what its antecedent recorded, and the echo is a function: an object
||| half that named a card type is reached by that type's word and no
||| other, and one that named none is reached by the generic `PermanentW`,
||| which is honest for a class word [CR#115.4] because its three object
||| choices are all permanent types [CR#110.4]. The player half echoes
||| nothing -- `PlayerP` records no describing word -- so the one row
||| serves an antecedent that said "player" and one that said "opponent"
||| alike.
public export
halfReaches : NounWord -> Payload k -> Bool
halfReaches w (JoinP l r) = halfReaches w l || halfReaches w r
halfReaches (TypeW t) (ObjectP ty _ _ _) = tyIs t ty
halfReaches PermanentW (ObjectP ty _ _ _) = isNothing ty
halfReaches PlayerW PlayerP = True
halfReaches _ _ = False

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
wordReaches (TypeW t) (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches (TypeW t) pl
wordReaches CardW (MkBinding _ _ _ (ObjectP _ zn _ _)) = isCardZone zn
wordReaches CardW (MkBinding _ _ _ PlayerP) = False
wordReaches CardW (MkBinding _ _ _ QualityP) = False
wordReaches CardW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CardW (MkBinding _ _ _ GapP) = False
wordReaches CardW (MkBinding _ _ _ LetterP) = False
wordReaches CardW (MkBinding _ _ _ TurnRefP) = False
wordReaches CardW (MkBinding _ _ _ AbilityP) = False
wordReaches CardW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches SpellW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onStackZone zn && not (isCopyOrigin og)
wordReaches SpellW (MkBinding _ _ _ PlayerP) = False
wordReaches SpellW (MkBinding _ _ _ QualityP) = False
wordReaches SpellW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches SpellW (MkBinding _ _ _ GapP) = False
wordReaches SpellW (MkBinding _ _ _ LetterP) = False
wordReaches SpellW (MkBinding _ _ _ TurnRefP) = False
wordReaches SpellW (MkBinding _ _ _ AbilityP) = False
wordReaches SpellW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ (ObjectP _ _ _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ PlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ QualityP) = False
wordReaches PlayerW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PlayerW (MkBinding _ _ _ GapP) = False
wordReaches PlayerW (MkBinding _ _ _ LetterP) = False
wordReaches PlayerW (MkBinding _ _ _ TurnRefP) = False
wordReaches PlayerW (MkBinding _ _ _ AbilityP) = False
wordReaches PlayerW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PlayerW pl
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn _ _)) = onFieldZone zn
wordReaches PermanentW (MkBinding _ _ _ PlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ QualityP) = False
wordReaches PermanentW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PermanentW (MkBinding _ _ _ GapP) = False
wordReaches PermanentW (MkBinding _ _ _ LetterP) = False
wordReaches PermanentW (MkBinding _ _ _ TurnRefP) = False
wordReaches PermanentW (MkBinding _ _ _ AbilityP) = False
wordReaches PermanentW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PermanentW pl
wordReaches TokenW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onFieldZone zn && isTokenOrigin og
wordReaches TokenW (MkBinding _ _ _ PlayerP) = False
wordReaches TokenW (MkBinding _ _ _ QualityP) = False
wordReaches TokenW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches TokenW (MkBinding _ _ _ GapP) = False
wordReaches TokenW (MkBinding _ _ _ LetterP) = False
wordReaches TokenW (MkBinding _ _ _ TurnRefP) = False
wordReaches TokenW (MkBinding _ _ _ AbilityP) = False
wordReaches TokenW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches CopyW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onStackZone zn && isCopyOrigin og
wordReaches CopyW (MkBinding _ _ _ PlayerP) = False
wordReaches CopyW (MkBinding _ _ _ QualityP) = False
wordReaches CopyW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CopyW (MkBinding _ _ _ GapP) = False
wordReaches CopyW (MkBinding _ _ _ LetterP) = False
wordReaches CopyW (MkBinding _ _ _ TurnRefP) = False
wordReaches CopyW (MkBinding _ _ _ AbilityP) = False
wordReaches CopyW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches JoinW (MkBinding _ _ _ (JoinP _ _)) = True
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
kindOfW JoinW = Object \/ Player

public export
stampedBy : VerbLabel -> Stamp -> Bool
stampedBy v (MkStamp v' _) = v == v'

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

||| Whether a shuffle leaves a mention readable. [CR#701.24b] keeps the
||| cards a search FOUND out of the shuffle, so a mention of one survives
||| it. Every other card in the pile is randomized where no player knows
||| its order [CR#701.24a], and a revealed one stops being revealed and
||| becomes a new object outright [CR#701.20d], so a mention of one does
||| not survive. Only a library mention is at stake: a card an earlier
||| clause moved elsewhere is not in the pile being randomized. The gate
||| reads the label's own stamp, which is why "Search" is a catalogued
||| label.
public export
survivesShuffle : Binding -> Bool
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) (Just st) _)) =
  stampedBy "Search" st
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) Nothing _)) = False
survivesShuffle _ = True

||| What a shuffle leaves the discourse holding.
public export
afterShuffle : Bindings -> Bindings
afterShuffle [] = []
afterShuffle (b :: bs) =
  if survivesShuffle b then b :: afterShuffle bs else afterShuffle bs

public export
stampWordOk : VerbLabel -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v w st ty zn = stampedBy v st && verbedWordOk w st ty zn

public export
verbedMatch : VerbLabel -> NounWord -> Binding -> Bool
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
verbedMatch v w (MkBinding _ _ _ (JoinP _ _)) = False

public export
verbedMatchMany : VerbLabel -> NounWord -> Binding -> Bool
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
verbedMatchMany v w (MkBinding _ _ _ (JoinP _ _)) = False

public export
countVerbed : VerbLabel -> NounWord -> Bindings -> Nat
countVerbed v w [] = Z
countVerbed v w (b :: bs) =
  if verbedMatch v w b then S (countVerbed v w bs) else countVerbed v w bs

public export
countManyVerbed : VerbLabel -> NounWord -> Bindings -> Nat
countManyVerbed v w [] = Z
countManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then S (countManyVerbed v w bs) else countManyVerbed v w bs

public export
zoneOfManyVerbed : VerbLabel -> NounWord -> Bindings -> Maybe Zone
zoneOfManyVerbed v w [] = Nothing
zoneOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingZone b else zoneOfManyVerbed v w bs

public export
tyOfManyVerbed : VerbLabel -> NounWord -> Bindings -> Maybe CardType
tyOfManyVerbed v w [] = Nothing
tyOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingTy b else tyOfManyVerbed v w bs

public export
zoneOfVerbed : VerbLabel -> NounWord -> Bindings -> Maybe Zone
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

||| "those tokens" names the characteristics definition a create clause
||| wrote [CR#111.3], not the objects; [CR#111.7] ends the objects when
||| they leave the battlefield and leaves that definition standing.
public export
countTokenSpecs : Bindings -> Nat
countTokenSpecs [] = Z
countTokenSpecs (MkBinding SelfD _ _ _ :: bs) = countTokenSpecs bs
countTokenSpecs (MkBinding _ _ ManyOf (ObjectP _ _ _ og) :: bs) =
  if isTokenOrigin og then S (countTokenSpecs bs) else countTokenSpecs bs
countTokenSpecs (_ :: bs) = countTokenSpecs bs

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
tyOfIt (b :: bs) =
  if itReaches OneOf b then bindingTy b else tyOfIt bs

public export
tyOfThem : Bindings -> Maybe CardType
tyOfThem [] = Nothing
tyOfThem (b :: bs) =
  if itReaches ManyOf b then bindingTy b else tyOfThem bs

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
tyOfVerbed : VerbLabel -> NounWord -> Bindings -> Maybe CardType
tyOfVerbed v w [] = Nothing
tyOfVerbed v w (b :: bs) =
  if verbedMatch v w b then bindingTy b else tyOfVerbed v w bs

public export
countChoosers : Bindings -> Nat
countChoosers bs = countOnes Player bs + countManys Player bs

public export
data OnBattlefield : Maybe Zone -> Type where
  OnField : OnBattlefield (Just Battlefield)

||| How an entry-counter clause relates its number to the counters the
||| object would otherwise enter with: the whole count (`Fresh`), extra
||| ones beside it (`Additional`), or that many taken off it (`Fewer`).
||| [CR#614.1c] makes each of the three one replacement effect over the
||| same entry, so the mark is which arithmetic that effect writes.
||| `Fewer` is compleated's reminder -- "enters with two fewer loyalty
||| counters" -- where [CR#306.5b]'s printed loyalty is the count being
||| reduced.
public export
data EntryCounterMark = Fresh | Additional | Fewer

public export
Eq EntryCounterMark where
  (==) Fresh Fresh = True
  (==) Fresh _ = False
  (==) Additional Additional = True
  (==) Additional _ = False
  (==) Fewer Fewer = True
  (==) Fewer _ = False

public export
data PlayerGroupWord = AllPlayers | YourOpponents

public export
Eq PlayerGroupWord where
  (==) AllPlayers AllPlayers = True
  (==) AllPlayers _ = False
  (==) YourOpponents YourOpponents = True
  (==) YourOpponents _ = False

public export
data RoundMode = RoundUp | RoundDown

public export
Eq RoundMode where
  (==) RoundUp RoundUp = True
  (==) RoundUp _ = False
  (==) RoundDown RoundDown = True
  (==) RoundDown _ = False

public export
data ScaleFactor = Doubled | Tripled

public export
Eq ScaleFactor where
  (==) Doubled Doubled = True
  (==) Doubled _ = False
  (==) Tripled Tripled = True
  (==) Tripled _ = False

public export
data ShiftDir = ShiftUp | ShiftDown

public export
Eq ShiftDir where
  (==) ShiftUp ShiftUp = True
  (==) ShiftUp _ = False
  (==) ShiftDown ShiftDown = True
  (==) ShiftDown _ = False


namespace Lookback
  ||| The window a retrospective reader scopes its event to. Three of the
  ||| values name a stretch of time; `ThisWay` names a CAUSE instead -- the
  ||| actions this spell or ability itself took. [CR#608.2c] is the rule that
  ||| lets later text read the instruction it follows, one of its two worked
  ||| examples being a "...this way" back-reference, and it settles no more
  ||| than that the reading is available: WHICH earlier instruction the phrase
  ||| names is left to whoever reads the card. The value carries no discourse
  ||| gate, where `DealtThisWay` carries one, because the bindings record the
  ||| mentions an earlier clause left rather than the instructions it ran, and
  ||| a window is generic over every event name. A text writing it with no
  ||| earlier instruction to name is tolerated overgeneration, refused at the
  ||| spelling boundary.
  ||| -- spelling: "this way" in the window's place, "dealt damage this
  ||| way", "destroyed this way".
  public export
  data Lookback = ThisTurn | ThisCombat | LastTurn | ThisGame | ThisWay

public export
data OnStack : Maybe Zone -> Type where
  OnTheStack : OnStack (Just Stack)

public export
zoneFits : Maybe Zone -> Maybe Zone -> Bool
zoneFits Nothing _ = True
zoneFits (Just _) Nothing = True
zoneFits (Just a) (Just b) = a == b

public export
ZoneFits : Maybe Zone -> Maybe Zone -> Type
ZoneFits subj desc = So (zoneFits subj desc)

public export
data Targetable : Kind -> Type where
  ObjectTgt : Targetable Object
  PlayerTgt : Targetable Player
  ||| [CR#115.1] lets a spell or ability target objects and players alike,
  ||| so a phrase that may denote either is targetable exactly when both
  ||| halves are.
  JoinTgt : Targetable a -> Targetable b -> Targetable (a \/ b)

||| [CR#120.1a]: damage can be dealt to a battle, a creature or a
||| planeswalker and to nothing else. [CR#115.4] names the same three
||| object types beside players.
public export
data DamageableTy : Maybe CardType -> Type where
  DamCreature : DamageableTy (Just Creature)
  DamPlaneswalker : DamageableTy (Just Planeswalker)
  DamBattle : DamageableTy (Just Battle)

||| The same set read off ONE half of a joined phrase, where the half may
||| also name no card type at all. [CR#120.1] states the whole recipient
||| set, so a half that names a type must name one on it; a half that
||| names none — "a permanent or player", Furnace of Rath — names nothing
||| off the set, and [CR#120.1a] bounds the referent whichever object it
||| turns out to be.
public export
damageableHalfTy : Maybe CardType -> Bool
damageableHalfTy Nothing = True
damageableHalfTy (Just Creature) = True
damageableHalfTy (Just Planeswalker) = True
damageableHalfTy (Just Battle) = True
damageableHalfTy (Just _) = False

public export
data Phrasal : Kind -> Type where
  PhObject : Phrasal Object
  PhPlayer : Phrasal Player
  PhQuality : Phrasal (Quality q)
  PhAbility : Phrasal Ability
  ||| A determiner over a joined description determines both halves at once
  ||| ("a permanent or player", Furnace of Rath).
  PhJoin : Phrasal a -> Phrasal b -> Phrasal (a \/ b)

public export
targetablePhrasal : Targetable k -> Phrasal k
targetablePhrasal ObjectTgt = PhObject
targetablePhrasal PlayerTgt = PhPlayer
targetablePhrasal (JoinTgt l r) =
  PhJoin (targetablePhrasal l) (targetablePhrasal r)


||| The NAME a card prints where an ability's whole text would otherwise
||| go: "flying", "ward {2}", "dredge 3". [CR#702.1] states the NOTATION,
||| not a set -- an object "lists only the name of the ability as a
||| keyword", and the keyword's own rule states what the word means. So
||| the meaning lives in the rule the word names, exactly as a keyword
||| ACTION's meaning lives in the body its macro expands; the label says
||| only which one.
|||
||| That makes the vocabulary OPEN. [CR#702]'s numbered sections look like
||| a closed enumeration and are not one: a level symbol [CR#711.2], a
||| chapter symbol [CR#714.2] and a class level bar [CR#716.2] are each
||| "a keyword ability" stated outside [CR#702] altogether, and this
||| grammar already writes one of them -- `ChapterMark` [CR#107.15]. A
||| word needs no [CR#702] entry to be a keyword, so a new keyword is a
||| new row of data, never a core enum arm plus a coverage re-decide.
|||
||| The label is still typo-checked at every gate that writes one,
||| against `keywordFacts` -- one DATA row per word, carrying what the
||| word's own rule says and its spelling cannot.
public export
KeywordLabel : Type
KeywordLabel = String

public export
data KeywordParamShape = NoParam | CostParam | QualityParam | SubjectParam
                       | NumberParam

public export
Eq KeywordParamShape where
  (==) NoParam NoParam = True
  (==) NoParam _ = False
  (==) CostParam CostParam = True
  (==) CostParam _ = False
  (==) QualityParam QualityParam = True
  (==) QualityParam _ = False
  (==) SubjectParam SubjectParam = True
  (==) SubjectParam _ = False
  (==) NumberParam NumberParam = True
  (==) NumberParam _ = False

||| When a keyword's ability acts, for the keywords that act before the
||| object carrying them has resolved.
public export
data StackRegime = AtCasting | AtResolution

public export
Eq StackRegime where
  (==) AtCasting AtCasting = True
  (==) AtCasting _ = False
  (==) AtResolution AtResolution = True
  (==) AtResolution _ = False

||| What a keyword's own rule says that the printed word cannot. Every
||| field is that rule speaking, and adding a row obliges nothing
||| anywhere else.
public export
record KeywordFacts where
  constructor MkKeywordFacts
  ||| the word as a card prints it
  word : KeywordLabel
  ||| the parameter the keyword's own rule writes after the word --
  ||| "Ward [cost]" [CR#702.21a], "Dredge N" [CR#702.52a]. Flying's rule
  ||| writes no slot [CR#702.9a], so the word stands bare.
  paramShape : KeywordParamShape
  ||| whether [CR#122.1b]'s keyword-counter list names the word. That
  ||| rule closes ITS list by enumeration -- flying, first strike,
  ||| double strike, deathtouch, decayed, exalted, haste, hexproof,
  ||| indestructible, lifelink, menace, reach, shadow, trample and
  ||| vigilance, and variants of those -- so every `False` here is the
  ||| rule's and not a gap.
  counterEligible : Bool
  ||| when the ability acts relative to the stack, where it acts before
  ||| its object resolves; `Nothing` where the question does not arise
  regime : Maybe StackRegime
  ||| whether a permanent card may print the word
  onPermanentCard : Bool
  ||| whether an instant or sorcery card may print it. `Card.idr`'s
  ||| `keywordCardOk` reads these three and says why the question is not
  ||| `regime`'s re-asked.
  onSpellCard : Bool
  ||| whether a card that stays in the command zone may print it. Nearly
  ||| every word's rule speaks of a permanent, of casting, or of a zone
  ||| such a card can never reach, so nearly every row is False. Storied
  ||| is the exception: [CR#702.195a] names no bearer at all, and
  ||| [CR#313.4,314.4,315.5] give the card a static ability that
  ||| functions from the command zone. Ascend is the near miss --
  ||| [CR#702.131a,702.131b] give the word a reading on a spell and on a
  ||| permanent and nowhere else.
  onCommandZoneCard : Bool

||| The keyword vocabulary, open by construction: a row is a word and
||| what its rule says about it.
|||
||| The last group's abilities function away from the battlefield, each
||| rule naming its own zone [CR#113.6b]: a graveyard for unearth
||| [CR#702.84a], flashback [CR#702.34a], dredge [CR#702.52a] and retrace
||| [CR#702.81a]; a hand for cycling [CR#702.29a], ninjutsu [CR#702.49a]
||| and miracle [CR#702.94a]; the stack for warp [CR#702.185a].
public export
keywordFacts : List KeywordFacts
keywordFacts =
  --                                  param        ctr   regime              perm  spell cz
  [ MkKeywordFacts "Haste"            NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Flying"           NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Trample"          NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Vigilance"        NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Deathtouch"       NoParam      True  (Just AtResolution) True  False False
  , MkKeywordFacts "DoubleStrike"     NoParam      True  Nothing             True  False False
  , MkKeywordFacts "FirstStrike"      NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Reach"            NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Convoke"          NoParam      False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Improvise"        NoParam      False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Storm"            NoParam      False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Lifelink"         NoParam      True  (Just AtResolution) True  False False
  , MkKeywordFacts "Ward"             CostParam    False Nothing             True  False False
  , MkKeywordFacts "Protection"       QualityParam False Nothing             True  False False
  , MkKeywordFacts "Enchant"          SubjectParam False Nothing             True  False False
  , MkKeywordFacts "Equip"            CostParam    False Nothing             True  False False
  , MkKeywordFacts "Ascend"           NoParam      False Nothing             True  True  False
  , MkKeywordFacts "Storied"          NoParam      False Nothing             True  False True
  , MkKeywordFacts "Renown"           NumberParam  False Nothing             True  False False
  , MkKeywordFacts "Indestructible"   NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Flash"            NoParam      False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Kicker"           CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Multikicker"      CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "CumulativeUpkeep" CostParam    False Nothing             True  False False
  , MkKeywordFacts "Echo"             CostParam    False Nothing             True  False False
  , MkKeywordFacts "Hexproof"         NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Menace"           NoParam      True  Nothing             True  False False
  , MkKeywordFacts "Skulk"            NoParam      False Nothing             True  False False
  , MkKeywordFacts "Bushido"          NumberParam  False Nothing             True  False False
  , MkKeywordFacts "Unearth"          CostParam    False Nothing             True  False False
  , MkKeywordFacts "Flashback"        CostParam    False Nothing             False True  False
  , MkKeywordFacts "Dredge"           NumberParam  False Nothing             True  True  False
  , MkKeywordFacts "Retrace"          NoParam      False Nothing             True  True  False
  , MkKeywordFacts "Cycling"          CostParam    False Nothing             True  True  False
  , MkKeywordFacts "Ninjutsu"         CostParam    False Nothing             True  False False
  , MkKeywordFacts "Miracle"          CostParam    False Nothing             True  True  False
  , MkKeywordFacts "Warp"             CostParam    False (Just AtCasting)    True  True  False
  ]

public export
keywordFactsIn : KeywordLabel -> List KeywordFacts -> Maybe KeywordFacts
keywordFactsIn k [] = Nothing
keywordFactsIn k (f :: fs) = if word f == k then Just f else keywordFactsIn k fs

public export
keywordFactsFor : KeywordLabel -> Maybe KeywordFacts
keywordFactsFor k = keywordFactsIn k keywordFacts

||| The membership gate: typo-safety without a per-word type. Every
||| reader below is fail-closed on an unknown word, so a gate that
||| consumes one of them needs no separate knownness check.
public export
knownKeyword : KeywordLabel -> Bool
knownKeyword k = isJust (keywordFactsFor k)

public export
KnownKeyword : KeywordLabel -> Type
KnownKeyword k = So (knownKeyword k)

public export
keywordParamShape : KeywordLabel -> KeywordParamShape
keywordParamShape k = maybe NoParam paramShape (keywordFactsFor k)

||| Whether the word names a cost -- its rule writes a cost where another
||| word writes a number or nothing [CR#118.1]. Fail-closed through
||| `keywordParamShape`, so an unknown word names no cost.
public export
keywordCosts : KeywordLabel -> Bool
keywordCosts k = keywordParamShape k == CostParam

||| Known AND written bare: an unknown word is not a parameterless
||| keyword, it is no keyword.
public export
keywordParamless : KeywordLabel -> Bool
keywordParamless k =
  maybe False (\f => paramShape f == NoParam) (keywordFactsFor k)

public export
keywordCounterOk : KeywordLabel -> Bool
keywordCounterOk k = maybe False counterEligible (keywordFactsFor k)

public export
KeywordCounterEligible : KeywordLabel -> Type
KeywordCounterEligible k = So (keywordCounterOk k)

public export
keywordStackRegime : KeywordLabel -> Maybe StackRegime
keywordStackRegime k = keywordFactsFor k >>= regime

||| WHICH of an object's own optional costs a later clause reads back.
||| [CR#607.2i] links the ability that offers a cost to the ability that
||| asks whether it was paid, and closes the naming question in its own
||| last sentence -- "Each of those abilities will specify which cost it
||| refers to". So the read is SORTED, and sorted by the cost's PRINTED
||| name rather than by any identifier a clause mints: a keyword names it
||| ("was kicked", "its madness cost was paid"), and where one keyword
||| offers a card two costs [CR#702.33b] the ordinal picks which, exactly
||| as [CR#702.33f] defines "with its [A] kicker" by the order the costs
||| are listed on the card.
||| A cost the card writes out instead of naming -- "You may pay {1}{B}
||| rather than pay this spell's mana cost", read back as "If the {1}{B}
||| cost was paid" -- has no name to give, so the card's own alternative
||| cost [CR#118.9] is the third arm and the printed symbols are
||| spelling.
public export
data PaidCostName : Type where
  ByKeyword : (kw : KeywordLabel) -> PaidCostName
  ByNthKeyword : (ord : Ordinal) -> (kw : KeywordLabel) -> PaidCostName
  TheAlternative : PaidCostName

public export
Eq PaidCostName where
  (==) (ByKeyword a) (ByKeyword b) = a == b
  (==) (ByKeyword _) _ = False
  (==) (ByNthKeyword (Nth m) a) (ByNthKeyword (Nth n) b) = m == n && a == b
  (==) (ByNthKeyword _ _) _ = False
  (==) TheAlternative TheAlternative = True
  (==) TheAlternative _ = False

||| A named cost has to be one there is: a keyword arm needs a word whose
||| rule writes a cost, and the written alternative needs nothing.
public export
paidCostNamed : PaidCostName -> Bool
paidCostNamed (ByKeyword kw) = keywordCosts kw
paidCostNamed (ByNthKeyword _ kw) = keywordCosts kw
paidCostNamed TheAlternative = True

public export
PaidCostNamed : PaidCostName -> Type
PaidCostNamed n = So (paidCostNamed n)

public export
data AbilityClass : Type where
  AnyActivated : AbilityClass
  LoyaltyClass : AbilityClass
  KeywordClass : (k : KeywordLabel) -> {auto 0 kn : KnownKeyword k} ->
                 AbilityClass

public export
Eq AbilityClass where
  (==) AnyActivated AnyActivated = True
  (==) AnyActivated _ = False
  (==) LoyaltyClass LoyaltyClass = True
  (==) LoyaltyClass _ = False
  (==) (KeywordClass a) (KeywordClass b) = a == b
  (==) (KeywordClass _) _ = False


||| [CR#207.2c]: an ability word appears in italics at the beginning of some
||| abilities. The words tie together cards with similar functionality but
||| have no special rules meaning and no individual entries in the rules, so
||| this sum carries names only. The list is closed: these are exactly the
||| words [CR#207.2c] enumerates, in its order. A label Scryfall files as an
||| ability word but [CR#207.2c] omits is not one here — Corrupted and Will
||| of the Planeswalkers print in italics and are absent from the rule.
public export
data AbilityWordName = Adamant | Addendum | Alliance | Battalion | Bloodrush
                     | Celebration | Channel | Chroma | Cohort | Constellation
                     | Converge | CouncilsDilemma | Coven | Delirium
                     | Descend4 | Descend8 | Disappear | Domain | Eerie
                     | Eminence | Enrage | FatefulHour | FathomlessDescent
                     | Ferocious | Flurry | Formidable | Grandeur | Hellbent
                     | Heroic | Imprint | Infusion | Inspired | JoinForces
                     | Kinship | Landfall | Lieutenant | Magecraft
                     | Metalcraft | Morbid | Opus | PackTactics | Paradox
                     | Parley | Radiance | Raid | Rally | Renew | Repartee
                     | Revolt | SecretCouncil | SpellMastery | Strive
                     | Survival | Sweep | TemptingOffer | Threshold
                     | Undergrowth | Valiant | Vivid | Void | WillOfTheCouncil


namespace Chroma
  public export
  data Color = White | Blue | Black | Red | Green

  public export
  data ColorOrColorless = Colorless | OfColor Color

public export
Eq Color where
  (==) White White = True
  (==) White _ = False
  (==) Blue Blue = True
  (==) Blue _ = False
  (==) Black Black = True
  (==) Black _ = False
  (==) Red Red = True
  (==) Red _ = False
  (==) Green Green = True
  (==) Green _ = False

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

||| Whether a mana cost writes the variable symbol: the spell's or
||| ability's own announcement of X [CR#107.3a].
public export
manaHasX : ManaCost -> Bool
manaHasX [] = False
manaHasX (Variable :: _) = True
manaHasX (_ :: ms) = manaHasX ms

public export
ManaRun : ManaCost -> Type
ManaRun c = NonEmpty c


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
altRunWritten : Maybe ProducedRun -> Bool
altRunWritten Nothing = True
altRunWritten (Just _) = True

public export
AltRunWritten : Maybe ProducedRun -> Type
AltRunWritten alt = So (altRunWritten alt)

public export
data ColorFreedom = SameColor | EachColor

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

||| An open subtype: its printed name and the card type whose subtype set it
||| belongs to. [CR#205.1a] names those sets -- creature, land, artifact,
||| enchantment, planeswalker and spell types -- [CR#205.3q] adds the battle
||| type, and [CR#205.3c] correlates each subtype word to its own card type,
||| so the pair is the whole word. The sets are OPEN: [CR#205.3m] is the
||| creature list, and it is amended set by set. So a subtype is DATA and
||| never a constructor -- where a subtype carries rules meaning, that
||| meaning is what it confers on its bearer, and a gate reads the label or
||| the conferral, never a constructor of its own.
public export
data Subtype : Type where
  MkSubtype : (host : CardType) -> (label : String) -> Subtype

||| The card type whose set the subtype belongs to. A projection of the
||| word's own sort, not a table a new subtype has to extend.
public export
subtypeType : Subtype -> CardType
subtypeType (MkSubtype host _) = host

||| The name the type line prints [CR#205.3b].
public export
subtypeLabel : Subtype -> String
subtypeLabel (MkSubtype _ label) = label

||| Both fields. [CR#205.3c] correlates a subtype word to one card type, so
||| on a well-formed line the host is redundant -- reading it keeps a
||| mis-sorted label out of another set's gate.
public export
Eq Subtype where
  (==) (MkSubtype h1 l1) (MkSubtype h2 l2) = h1 == h2 && l1 == l2

||| A subtype's set, applied to the word: `creatureType "Zombie"`,
||| `landType "Forest"`. Naming a subtype is writing one of these, and that
||| is the whole cost of a new one.
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

||| [CR#205.3k] gives instants and sorceries one shared set, which
||| `subtypeType`'s single answer cannot say; a spell type files under
||| Instant and `subsFitLine` reads either card type back.
public export
spellType : String -> Subtype
spellType n = MkSubtype Instant n

||| [CR#305.6] names the five basic land types by enumeration, so this one
||| set is closed where the rest are open.
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
-- [CR#205.3m,308.2]: kindreds and creatures share one subtype list, so a
-- creature-type space is a kindred's as much as a creature's.
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

-- [CR#109.2] admits a description that includes "a card type or
-- subtype", and reads a bare one onto the battlefield, so any permanent
-- type [CR#110.4] is a word a card may name itself by; a subtype rides
-- its own parent type through `ascriptionOk`.
public export
ascribesAsType : CardType -> Bool
ascribesAsType Creature = True
ascribesAsType Artifact = True
ascribesAsType Land = True
ascribesAsType Enchantment = True
ascribesAsType Planeswalker = True
ascribesAsType Battle = True
-- [CR#308.1]: a kindred card always has another card type, so it is
-- never the word such a card names itself by.
ascribesAsType Kindred = False
ascribesAsType Instant = False
ascribesAsType Sorcery = False
-- [CR#110.4]'s six permanent types are the words a card reads onto the
-- battlefield by; the command-zone types are none of them.
ascribesAsType Conspiracy = False
ascribesAsType Dungeon = False
ascribesAsType Phenomenon = False
ascribesAsType Plane = False
ascribesAsType Scheme = False
ascribesAsType Vanguard = False

public export
ascriptionOk : CardType -> Maybe Subtype -> Bool
ascriptionOk t Nothing = ascribesAsType t
ascriptionOk t (Just s) = ascribesAsType t && subtypeType s == t

namespace Counter
  public export
  data Delta : Type where
    Up : Nat -> Delta
    Down : Nat -> Delta

  public export
  Eq Delta where
    (==) (Up a) (Up b) = a == b
    (==) (Up _) _ = False
    (==) (Down a) (Down b) = a == b
    (==) (Down _) _ = False

||| [CR#205.4a] closes the supertypes by enumeration at five, so this
||| catalog ports the rule's whole set — the same discipline `Color` and
||| `CardType` follow — rather than only the ones a card has been
||| observed to write.
||| World carries [CR#205.4f]'s state-based action and Ongoing
||| [CR#205.4h]'s scheme exemption; both are supertypes a card prints.
public export
data Supertype = Legendary | Basic | Snow | Ongoing | World

public export
Eq Supertype where
  (==) Legendary Legendary = True
  (==) Legendary _ = False
  (==) Basic Basic = True
  (==) Basic _ = False
  (==) Snow Snow = True
  (==) Snow _ = False
  (==) Ongoing Ongoing = True
  (==) Ongoing _ = False
  (==) World World = True
  (==) World _ = False

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
Eq Designation where
  (==) Monarch Monarch = True
  (==) Monarch _ = False
  (==) TheInitiative TheInitiative = True
  (==) TheInitiative _ = False
  (==) CitysBlessing CitysBlessing = True
  (==) CitysBlessing _ = False
  (==) EnduringStory EnduringStory = True
  (==) EnduringStory _ = False
  (==) Goaded Goaded = True
  (==) Goaded _ = False
  (==) RingBearer RingBearer = True
  (==) RingBearer _ = False
  (==) Monstrous Monstrous = True
  (==) Monstrous _ = False
  (==) Renowned Renowned = True
  (==) Renowned _ = False
  (==) Suspected Suspected = True
  (==) Suspected _ = False
  (==) Saddled Saddled = True
  (==) Saddled _ = False
  (==) Prepared Prepared = True
  (==) Prepared _ = False
  (==) CommanderD CommanderD = True
  (==) CommanderD _ = False
  (==) Day Day = True
  (==) Day _ = False
  (==) Night Night = True
  (==) Night _ = False

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
designationChecked Day = True
designationChecked Night = True

public export
designationGiven : Designation -> Bool
designationGiven Monarch = True
designationGiven TheInitiative = True
designationGiven CitysBlessing = True
designationGiven EnduringStory = True
designationGiven Goaded = True
designationGiven RingBearer = True
designationGiven Monstrous = True
designationGiven Renowned = True
designationGiven Suspected = True
designationGiven Saddled = True
designationGiven Prepared = True
designationGiven CommanderD = False
designationGiven Day = True
designationGiven Night = True

||| The keywords whose expansion body confers a designation and whose
||| expansion this grammar can write. One arm, bought by monstrosity's
||| expansion [CR#701.37a] and Chillerpillar.
public export
data ConferringWord = MonstrosityW | SaddleW | AscendW | StoriedW | RenownW

public export
conferredDesignation : ConferringWord -> Designation
conferredDesignation MonstrosityW = Monstrous
conferredDesignation SaddleW = Saddled
conferredDesignation AscendW = CitysBlessing
conferredDesignation StoriedW = EnduringStory
conferredDesignation RenownW = Renowned

||| Why a line may confer a designation. `Instructed` is the bare
||| sentence, which needs the designation's own giving cell.
||| `InExpansionOf` is the warrant a keyword's own macro supplies when it
||| spells that keyword's expansion body.
|||
||| Nothing in the types ties `InExpansionOf` to an expansion: a bare
||| ability can name the word and confer. The bare-sentence refusal
||| therefore rests on macro discipline plus the `Instructed` pins, not
||| on structure. A structural expansion-context index is a recorded gap.
public export
data GivingWarrant : Designation -> Type where
  Instructed : {auto 0 at : So (designationGiven d)} -> GivingWarrant d
  InExpansionOf : (w : ConferringWord) -> GivingWarrant (conferredDesignation w)

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
Eq AttachWord where
  (==) Enchanted Enchanted = True
  (==) Enchanted _ = False
  (==) Equipped Equipped = True
  (==) Equipped _ = False
  (==) Fortified Fortified = True
  (==) Fortified _ = False

||| Which host word an attachment word may call its host by.
||| [CR#301.5f] and [CR#303.4m] both let the word name whatever the
||| permanent is attached to, so a broader permanent word is the host's
||| own — Luxior, Giada's Gift writes "equipped permanent". An Aura
||| attaches to an object or player [CR#303.4], which every noun word
||| names, so "enchanted" takes them all. The zeros are the two artifact
||| attachment rules: an Equipment attaches to a creature [CR#301.5] and
||| a Fortification to a land [CR#301.6]. [CR#110.4]'s non-permanent
||| types are zeros in those two rows for the same reason: neither an
||| Equipment nor a Fortification can attach to a card that never reaches
||| the battlefield.
public export
attachHeadOk : AttachWord -> NounWord -> Bool
attachHeadOk Enchanted _ = True
attachHeadOk Equipped (TypeW Creature) = True
attachHeadOk Equipped (TypeW Artifact) = False
attachHeadOk Equipped (TypeW Land) = False
attachHeadOk Equipped (TypeW Enchantment) = False
attachHeadOk Equipped (TypeW Planeswalker) = False
attachHeadOk Equipped (TypeW Battle) = False
attachHeadOk Equipped (TypeW Kindred) = False
attachHeadOk Equipped (TypeW Instant) = False
attachHeadOk Equipped (TypeW Sorcery) = False
attachHeadOk Equipped (TypeW Conspiracy) = False
attachHeadOk Equipped (TypeW Dungeon) = False
attachHeadOk Equipped (TypeW Phenomenon) = False
attachHeadOk Equipped (TypeW Plane) = False
attachHeadOk Equipped (TypeW Scheme) = False
attachHeadOk Equipped (TypeW Vanguard) = False
attachHeadOk Equipped CardW = False
attachHeadOk Equipped SpellW = False
attachHeadOk Equipped PlayerW = False
attachHeadOk Equipped PermanentW = True
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
attachHeadOk Fortified (TypeW Conspiracy) = False
attachHeadOk Fortified (TypeW Dungeon) = False
attachHeadOk Fortified (TypeW Phenomenon) = False
attachHeadOk Fortified (TypeW Plane) = False
attachHeadOk Fortified (TypeW Scheme) = False
attachHeadOk Fortified (TypeW Vanguard) = False
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
attachedCheckOk Fortified = True

public export
AttachHeadOk : AttachWord -> NounWord -> Type
AttachHeadOk w h = So (attachHeadOk w h)

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
Eq OutcomeVerb where
  (==) WinGame WinGame = True
  (==) WinGame _ = False
  (==) LoseGame LoseGame = True
  (==) LoseGame _ = False

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
riderAct Cast = True
riderAct Played = True
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
  KeywordCounter : (k : KeywordLabel) ->
                   {auto 0 ok : KeywordCounterEligible k} -> CounterKind
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
  ||| [CR#122.1c]'s replacement-and-prevention pair is the engine's
  ||| machinery, exactly as stun's is; the grammar carries the kind word.
  Shield : CounterKind
  ||| [CR#122.1] makes every counter a marker on an object; [CR#122.1e]
  ||| only says what the loyalty count indicates, and [CR#606.4] moves
  ||| loyalty counters as a cost. The count-equals-loyalty link
  ||| [CR#306.5c] is lowering's, not this layer's.
  LoyaltyCounter : CounterKind
  ||| Sagas' plan counters and the hour counters of Midnight Clock: the
  ||| kinds the ordinal counter headers name.
  Plan : CounterKind
  Hour : CounterKind
  ||| Investigator's Journal's stored draws: an ordinary marker whose
  ||| whole meaning is the ability that reads it [CR#122.1].
  Suspect : CounterKind
  ||| Chance Encounter's tally of won flips: the same ordinary marker
  ||| [CR#122.1], counted by the ability that reads it.
  Luck : CounterKind
  ||| Font of Agonies' tally of life paid: the same ordinary marker
  ||| [CR#122.1].
  Blood : CounterKind

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
counterScope Shield = Object
counterScope LoyaltyCounter = Object
counterScope Plan = Object
counterScope Hour = Object
counterScope Suspect = Object
counterScope Luck = Object
counterScope Blood = Object

public export
Eq CounterKind where
  (==) (BoostCounter ap at) (BoostCounter bp bt) =
    ap == bp && at == bt
  (==) (BoostCounter _ _) _ = False
  (==) Stun Stun = True
  (==) Stun _ = False
  (==) Time Time = True
  (==) Time _ = False
  (==) (KeywordCounter a) (KeywordCounter b) = a == b
  (==) (KeywordCounter _) _ = False
  (==) Charge Charge = True
  (==) Charge _ = False
  (==) Omen Omen = True
  (==) Omen _ = False
  (==) Spite Spite = True
  (==) Spite _ = False
  (==) Rev Rev = True
  (==) Rev _ = False
  (==) Intervention Intervention = True
  (==) Intervention _ = False
  (==) Poison Poison = True
  (==) Poison _ = False
  (==) Rad Rad = True
  (==) Rad _ = False
  (==) Experience Experience = True
  (==) Experience _ = False
  (==) Lore Lore = True
  (==) Lore _ = False
  (==) Age Age = True
  (==) Age _ = False
  (==) Shield Shield = True
  (==) Shield _ = False
  (==) LoyaltyCounter LoyaltyCounter = True
  (==) LoyaltyCounter _ = False
  (==) Plan Plan = True
  (==) Plan _ = False
  (==) Hour Hour = True
  (==) Hour _ = False
  (==) Suspect Suspect = True
  (==) Suspect _ = False
  (==) Luck Luck = True
  (==) Luck _ = False
  (==) Blood Blood = True
  (==) Blood _ = False

public export
data CounterKindNamed : Kind -> Maybe CounterKind -> Type where
  KindUnnamed : CounterKindNamed k Nothing
  KindNamed : {0 c : CounterKind} ->
              {auto 0 sc : counterScope c = k} ->
              CounterKindNamed k (Just c)

||| How a giving clause says which kind of counter: the printed word, or
||| a printed menu one arm of which the clause's own "you" [CR#109.5]
||| picks. The menu is the kind slot's content, not another operation --
||| same verb, same amount, same recipient -- against
||| `PutCountersOfThoseKinds`, where no kind is given at all.
|||
||| The pick needs no chooser slot and announces no mention: [CR#109.5]
||| fixes who picks where the clause writes "your", [CR#608.2c] where it
||| writes no one, [CR#614.12a] fixes when for the entry side, and no
||| line in the family reads the picked kind back.
||| -- spelling: "your choice of a [k1], [k2], or [k3] counter" (Denry
||| Klin); with the determiner repeated per arm, "your choice of a [k1]
||| counter or a [k2] counter" (Helica Glider); with the list postposed,
||| "your choice of a counter from among [k1], [k2], and [k3]" (Aragorn);
||| and with the chooser unwritten, "a [k1] counter or a [k2] counter"
||| (Dwarven Armorer).
public export
data CounterKindSource : Type where
  PrintedKind : CounterKind -> CounterKindSource
  ||| The menu has to have an arm: [CR#122.1] places a counter of some
  ||| name, and an empty list names none. A ONE-arm menu and a repeated
  ||| arm are both tolerated and both unwritten -- each is performable,
  ||| and each says what `PrintedKind` says, since [CR#122.1] makes
  ||| counters with the same name interchangeable.
  ChosenKind : (menu : List CounterKind) ->
               {auto 0 ne : NonEmpty menu} -> CounterKindSource

||| Every arm has to name a counter the recipient can hold: [CR#122.1]
||| puts a counter on an object or a player, so an arm at the other
||| scope is an arm no one could pick.
public export
counterSourceScope : CounterKindSource -> Kind -> Bool
counterSourceScope (PrintedKind c) k = counterScope c == k
counterSourceScope (ChosenKind menu) k = all (\c => counterScope c == k) menu

public export
CounterSourceScope : CounterKindSource -> Kind -> Type
CounterSourceScope s k = So (counterSourceScope s k)

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
||| [CR#107.15b] makes "{rN1}, {rN2}—" shorthand for two independent
||| abilities, so the marks carry no order; one number twice would name
||| one ability twice.
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


||| The spell types [CR#205.3k]: one list that instants and sorceries share.
||| `subtypeType` answers with a single card type and so under-reports these;
||| a spell type fits either of the two.
public export
spellSubtype : Subtype -> Bool
spellSubtype s = subtypeType s == Instant

public export
subsFitLine : List Subtype -> List CardType -> Bool
subsFitLine [] tys = True
subsFitLine (s :: ss) tys =
  -- [CR#308.2]: kindred subtypes are the same set as creature subtypes,
  -- so a creature subtype fits a Kindred-typed line too.
  (elem (subtypeType s) tys
     || (subtypeType s == Creature && elem Kindred tys)
     -- [CR#205.3k]: instants and sorceries share their subtype list, so
     -- either card type carries a spell type.
     || (spellSubtype s && (elem Instant tys || elem Sorcery tys)))
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
-- [CR#110.4] names the six permanent types; the command-zone types are
-- not among them, and [CR#311.2,312.2,313.2,314.2,315.3,309.2c] say so
-- again.
permanentType Conspiracy = False
permanentType Dungeon = False
permanentType Phenomenon = False
permanentType Plane = False
permanentType Scheme = False
permanentType Vanguard = False

||| The card types a spell has. Named apart from `spellType`, which builds
||| [CR#205.3k]'s spell types -- those are subtypes.
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
-- A spell is a card on the stack [CR#112.1]; the command-zone types
-- never reach it [CR#311.2,312.2,313.2,314.2,315.3,309.2c].
spellCardType Conspiracy = False
spellCardType Dungeon = False
spellCardType Phenomenon = False
spellCardType Plane = False
spellCardType Scheme = False
spellCardType Vanguard = False

||| The third answer beside `permanentType` and `spellCardType`, not a gap
||| in either: the card types whose own rule keeps the card in the command
||| zone, where it is no permanent and is never cast -- [CR#315.3]
||| conspiracy, [CR#309.2c] dungeon, [CR#312.2] phenomenon, [CR#311.2]
||| plane, [CR#314.2] scheme, [CR#313.2] vanguard.
public export
commandZoneType : CardType -> Bool
commandZoneType Conspiracy = True
commandZoneType Dungeon = True
commandZoneType Phenomenon = True
commandZoneType Plane = True
commandZoneType Scheme = True
commandZoneType Vanguard = True
commandZoneType Creature = False
commandZoneType Artifact = False
commandZoneType Land = False
commandZoneType Enchantment = False
commandZoneType Instant = False
commandZoneType Sorcery = False
commandZoneType Planeswalker = False
commandZoneType Battle = False
commandZoneType Kindred = False

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
-- [CR#710.4]: flipping is a one-way process, so nothing ever becomes
-- unflipped and there is no transition to watch.
statusEventOk Unflipped = False
statusEventOk FaceUp = True
statusEventOk FaceDown = True
statusEventOk PhasedIn = True
statusEventOk PhasedOut = True

public export
StatusEventVal : StatusVal c -> Type
StatusEventVal v = So (statusEventOk v)

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
typesDistinct : List CardType -> Bool
typesDistinct [] = True
typesDistinct (t :: ts) = not (elem t ts) && typesDistinct ts


public export
addedFits : Maybe CardType -> TypeLine -> Bool
addedFits subj (MkTypeLine [] tys) = True
addedFits subj (MkTypeLine (s :: ss) tys) =
  (elem (subtypeType s) tys ||
   (case subj of
      Nothing => False
      Just t => subtypeType s == t)) &&
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
-- [CR#205.1a]: instant and sorcery retain their card type automatically,
-- so no retention rider is needed for either.
retainable Instant = False
retainable Sorcery = False
-- [CR#205.1a] exempts instant and sorcery and no other type, so every
-- remaining type is one a retention rider [CR#205.1b] has to name.
retainable Conspiracy = True
retainable Dungeon = True
retainable Phenomenon = True
retainable Plane = True
retainable Scheme = True
retainable Vanguard = True

public export
retentionOk : TypeLine -> Maybe CardType -> Bool
retentionOk tl Nothing = True
retentionOk (MkTypeLine _ []) (Just t) = False
retentionOk (MkTypeLine _ (u :: us)) (Just t) = retainable t

public export
RetentionOk : TypeLine -> Maybe CardType -> Type
RetentionOk tl ret = So (retentionOk tl ret)

public export
data TokenRider = EntersTapped | EntersAttacking

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
Eq TurnPart where
  (==) Turn Turn = True
  (==) Turn _ = False
  (==) Upkeep Upkeep = True
  (==) Upkeep _ = False
  (==) EndStep EndStep = True
  (==) EndStep _ = False
  (==) Combat Combat = True
  (==) Combat _ = False
  (==) UntapStep UntapStep = True
  (==) UntapStep _ = False
  (==) EndOfCombat EndOfCombat = True
  (==) EndOfCombat _ = False
  (==) FirstMain FirstMain = True
  (==) FirstMain _ = False
  (==) PostcombatMain PostcombatMain = True
  (==) PostcombatMain _ = False
  (==) DrawStep DrawStep = True
  (==) DrawStep _ = False
  (==) MainPhase MainPhase = True
  (==) MainPhase _ = False

||| A turn-based action a window is written relative to, where
||| `TurnPart` names a part to be inside [CR#508.1].
public export
data TurnPoint = AttackersDeclared

public export
data Whose = Yours | ThatPlayers

namespace Owner
  public export
  data Owner = Yours | ThatPlayers | EachPlayers | EachOpponents
             | EachYours | AnOpponents
             | ThatTurns

public export
Eq Owner where
  (==) Yours Yours = True
  (==) Yours _ = False
  (==) ThatPlayers ThatPlayers = True
  (==) ThatPlayers _ = False
  (==) EachPlayers EachPlayers = True
  (==) EachPlayers _ = False
  (==) EachOpponents EachOpponents = True
  (==) EachOpponents _ = False
  (==) EachYours EachYours = True
  (==) EachYours _ = False
  (==) AnOpponents AnOpponents = True
  (==) AnOpponents _ = False
  (==) ThatTurns ThatTurns = True
  (==) ThatTurns _ = False

||| The possessor a turn part's header announces, read back by the
||| effect as "that player". "Your" and "each of your" name the
||| controller [CR#109.5], "that player's" reads an antecedent already
||| made, and the deictic possessor names a turn rather than a player,
||| so those four announce nothing.
public export
possessorB : Maybe Owner -> List Binding
possessorB Nothing = []
possessorB (Just Yours) = []
possessorB (Just ThatPlayers) = []
possessorB (Just EachPlayers) = [MkBinding EachD Player OneOf PlayerP]
possessorB (Just EachOpponents) = [MkBinding EachD Player OneOf PlayerP]
possessorB (Just EachYours) = []
possessorB (Just AnOpponents) = [MkBinding AD Player OneOf PlayerP]
possessorB (Just ThatTurns) = []

public export
data DurationEnd = StartOf TurnPart (Maybe Whose)
                 | EndOf TurnPart (Maybe Whose)


