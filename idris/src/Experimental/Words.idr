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

public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost | CountersPut
                 | DamagePrevented | RollResult | CoinFlipped
                 | NamedNumber

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

namespace Verb
  public export
  data VerbName = Destroy | Sacrifice | Exile | Discard | Mill | Scry | Surveil
                | Put

public export
verbAgentive : VerbName -> Bool
verbAgentive Destroy = False
verbAgentive Sacrifice = True
verbAgentive Exile = False
verbAgentive Discard = True
verbAgentive Mill = True
verbAgentive Scry = True
verbAgentive Surveil = True
verbAgentive Put = True

||| [CR#400.7]: which verbs move their patient to another zone, so the
||| object the next clause reads is a new one.
public export
verbMoves : VerbName -> Bool
verbMoves Destroy = True
verbMoves Sacrifice = True
verbMoves Exile = True
verbMoves Discard = True
verbMoves Mill = True
verbMoves Surveil = True
verbMoves Put = True
verbMoves Scry = False

public export
Eq VerbName where
  (==) Scry Scry = True
  (==) Scry _ = False
  (==) Surveil Surveil = True
  (==) Surveil _ = False
  (==) Destroy Destroy = True
  (==) Destroy _ = False
  (==) Sacrifice Sacrifice = True
  (==) Sacrifice _ = False
  (==) Exile Exile = True
  (==) Exile _ = False
  (==) Discard Discard = True
  (==) Discard _ = False
  (==) Mill Mill = True
  (==) Mill _ = False
  (==) Put Put = True
  (==) Put _ = False

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

||| The card type a payload names, read the same way as `payloadZone`.
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
payloadTy (JoinP l r) = maybe (payloadTy r) Just (payloadTy l)

public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ pl) = payloadZone pl

public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ pl) = payloadTy pl

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
  (==) NamedNumber NamedNumber = True
  (==) NamedNumber _ = False

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
-- a defining sentence names a number outright [CR#604.3,208.1], so the
-- anaphor that reads a quantity back ("that number") has one to name.
outcomeIsQuantity NamedNumber = True

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

public export
-- "the rest" presupposes one assembled group (nothing to be the rest OF
-- without it) and at least one part taken from it (nothing taken and the
-- text would have written "them").
theRestOk : Bindings -> Bool
theRestOk bs = countGroups bs == 1 && not (countParts bs == Z)

public export
groupSpent : Bindings -> Bindings
groupSpent [] = []
groupSpent (b@(MkBinding PartD _ _ _) :: bs) = b :: groupSpent bs
groupSpent (b :: bs) = if objGroup b then groupSpent bs else b :: groupSpent bs

public export
zoneOfGroup : Bindings -> Maybe Zone
zoneOfGroup [] = Nothing
zoneOfGroup (MkBinding PartD _ _ _ :: bs) = zoneOfGroup bs
zoneOfGroup (b :: bs) =
  if objGroup b then bindingZone b else zoneOfGroup bs

public export
tyOfGroup : Bindings -> Maybe CardType
tyOfGroup [] = Nothing
tyOfGroup (MkBinding PartD _ _ _ :: bs) = tyOfGroup bs
tyOfGroup (b :: bs) =
  if objGroup b then bindingTy b else tyOfGroup bs

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


||| [CR#122.2]: counters on an object cease to exist when it moves from
||| one zone to another, so a clause that reads counters off a referent an
||| earlier clause moved names none [CR#400.7].
public export
stampMoves : Maybe Stamp -> Bool
stampMoves Nothing = False
stampMoves (Just (MkStamp v _)) = verbMoves v

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

public export
verbedMarkingOk : VerbName -> VerbedMarking -> Bool
verbedMarkingOk Destroy Attributive = True
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
-- No line marks a placement patient by the bare participle; the marking
-- names the destination too ("put onto the battlefield this way").
verbedMarkingOk Put Attributive = False
verbedMarkingOk Put ThisWay = False

public export
VerbedMarkingOk : VerbName -> VerbedMarking -> Type
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
wordReaches (TypeW t) (MkBinding _ _ _ (JoinP _ _)) = False
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
wordReaches PlayerW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn _ _)) = onFieldZone zn
wordReaches PermanentW (MkBinding _ _ _ PlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ QualityP) = False
wordReaches PermanentW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PermanentW (MkBinding _ _ _ GapP) = False
wordReaches PermanentW (MkBinding _ _ _ LetterP) = False
wordReaches PermanentW (MkBinding _ _ _ TurnRefP) = False
wordReaches PermanentW (MkBinding _ _ _ AbilityP) = False
wordReaches PermanentW (MkBinding _ _ _ (JoinP _ _)) = False
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
stampedBy : VerbName -> Stamp -> Bool
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
verbedMatch v w (MkBinding _ _ _ (JoinP _ _)) = False

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
verbedMatchMany v w (MkBinding _ _ _ (JoinP _ _)) = False

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
data EntryCounterMark = Fresh | Additional

public export
Eq EntryCounterMark where
  (==) Fresh Fresh = True
  (==) Fresh Additional = False
  (==) Additional Fresh = False
  (==) Additional Additional = True

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
  public export
  data Lookback = ThisTurn | ThisCombat | LastTurn | ThisGame

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


||| The last group functions away from the battlefield, each rule naming
||| its own zone [CR#113.6b]: a graveyard for unearth [CR#702.84a],
||| flashback [CR#702.34a], dredge [CR#702.52a] and retrace [CR#702.81a];
||| a hand for cycling [CR#702.29a], ninjutsu [CR#702.49a] and miracle
||| [CR#702.94a]; the stack for warp [CR#702.185a].
public export
data Keyword = Haste | Flying | Trample | Vigilance | Deathtouch
             | DoubleStrike | FirstStrike | Reach
             | Convoke | Improvise | Storm | Lifelink
             | Ward | Protection
             | Enchant | Equip | Ascend | Storied | Renown
             | Indestructible | Flash
             | CumulativeUpkeep
             | Hexproof | Menace | Skulk
             | Unearth | Flashback | Dredge | Retrace
             | Cycling | Ninjutsu | Miracle | Warp

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
-- Each rule writes its own parameter: "Unearth [cost]" [CR#702.84a],
-- "Dredge N" [CR#702.52a], a bare "Retrace" [CR#702.81a].
keywordParamShape Unearth = CostParam
keywordParamShape Flashback = CostParam
keywordParamShape Dredge = NumberParam
keywordParamShape Retrace = NoParam
keywordParamShape Cycling = CostParam
keywordParamShape Ninjutsu = CostParam
keywordParamShape Miracle = CostParam
keywordParamShape Warp = CostParam

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

public export
keywordParamless : Keyword -> Bool
keywordParamless k = keywordParamShape k == NoParam

public export
Eq Keyword where
  (==) Haste Haste = True
  (==) Haste _ = False
  (==) Flying Flying = True
  (==) Flying _ = False
  (==) Trample Trample = True
  (==) Trample _ = False
  (==) Vigilance Vigilance = True
  (==) Vigilance _ = False
  (==) Deathtouch Deathtouch = True
  (==) Deathtouch _ = False
  (==) DoubleStrike DoubleStrike = True
  (==) DoubleStrike _ = False
  (==) FirstStrike FirstStrike = True
  (==) FirstStrike _ = False
  (==) Reach Reach = True
  (==) Reach _ = False
  (==) Convoke Convoke = True
  (==) Convoke _ = False
  (==) Improvise Improvise = True
  (==) Improvise _ = False
  (==) Storm Storm = True
  (==) Storm _ = False
  (==) Lifelink Lifelink = True
  (==) Lifelink _ = False
  (==) Ward Ward = True
  (==) Ward _ = False
  (==) Protection Protection = True
  (==) Protection _ = False
  (==) Enchant Enchant = True
  (==) Enchant _ = False
  (==) Equip Equip = True
  (==) Equip _ = False
  (==) Ascend Ascend = True
  (==) Ascend _ = False
  (==) Storied Storied = True
  (==) Storied _ = False
  (==) Renown Renown = True
  (==) Renown _ = False
  (==) Indestructible Indestructible = True
  (==) Indestructible _ = False
  (==) Flash Flash = True
  (==) Flash _ = False
  (==) CumulativeUpkeep CumulativeUpkeep = True
  (==) CumulativeUpkeep _ = False
  (==) Hexproof Hexproof = True
  (==) Hexproof _ = False
  (==) Menace Menace = True
  (==) Menace _ = False
  (==) Skulk Skulk = True
  (==) Skulk _ = False
  (==) Unearth Unearth = True
  (==) Unearth _ = False
  (==) Flashback Flashback = True
  (==) Flashback _ = False
  (==) Dredge Dredge = True
  (==) Dredge _ = False
  (==) Retrace Retrace = True
  (==) Retrace _ = False
  (==) Cycling Cycling = True
  (==) Cycling _ = False
  (==) Ninjutsu Ninjutsu = True
  (==) Ninjutsu _ = False
  (==) Miracle Miracle = True
  (==) Miracle _ = False
  (==) Warp Warp = True
  (==) Warp _ = False

public export
data AbilityClass = AnyActivated | LoyaltyClass | KeywordClass Keyword

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
             | Goat | Ox | Boar
             | Spirit
             | Shapeshifter
             | Saga
             | Centaur
             | Monk
             | Nymph | Dryad
             | Nightmare | Fish
             | Horror | Gargoyle | Assassin
             | Skeleton | Golem
             | Town | Desert
             | Pegasus
             | Faerie
             | Snake
             | Adventure
             | Arlinn
             | Kraken | Sphinx
             | Werewolf | Eldrazi
             | Fungus

public export
Eq Subtype where
  (==) Goblin Goblin = True
  (==) Goblin _ = False
  (==) Equipment Equipment = True
  (==) Equipment _ = False
  (==) Avatar Avatar = True
  (==) Avatar _ = False
  (==) Insect Insect = True
  (==) Insect _ = False
  (==) Elder Elder = True
  (==) Elder _ = False
  (==) Dinosaur Dinosaur = True
  (==) Dinosaur _ = False
  (==) Horror Horror = True
  (==) Horror _ = False
  (==) Gargoyle Gargoyle = True
  (==) Gargoyle _ = False
  (==) Assassin Assassin = True
  (==) Assassin _ = False
  (==) Skeleton Skeleton = True
  (==) Skeleton _ = False
  (==) Golem Golem = True
  (==) Golem _ = False
  (==) Town Town = True
  (==) Town _ = False
  (==) Desert Desert = True
  (==) Desert _ = False
  (==) Pegasus Pegasus = True
  (==) Pegasus _ = False
  (==) Faerie Faerie = True
  (==) Faerie _ = False
  (==) Snake Snake = True
  (==) Snake _ = False
  (==) Adventure Adventure = True
  (==) Adventure _ = False
  (==) Arlinn Arlinn = True
  (==) Arlinn _ = False
  (==) Kraken Kraken = True
  (==) Kraken _ = False
  (==) Sphinx Sphinx = True
  (==) Sphinx _ = False
  (==) Werewolf Werewolf = True
  (==) Werewolf _ = False
  (==) Eldrazi Eldrazi = True
  (==) Eldrazi _ = False
  (==) Fungus Fungus = True
  (==) Fungus _ = False
  (==) Goat Goat = True
  (==) Goat _ = False
  (==) Ox Ox = True
  (==) Ox _ = False
  (==) Boar Boar = True
  (==) Boar _ = False
  (==) Spirit Spirit = True
  (==) Spirit _ = False
  (==) Shapeshifter Shapeshifter = True
  (==) Shapeshifter _ = False
  (==) Centaur Centaur = True
  (==) Centaur _ = False
  (==) Monk Monk = True
  (==) Monk _ = False
  (==) Nymph Nymph = True
  (==) Nymph _ = False
  (==) Dryad Dryad = True
  (==) Dryad _ = False
  (==) Nightmare Nightmare = True
  (==) Nightmare _ = False
  (==) Fish Fish = True
  (==) Fish _ = False
  (==) Saga Saga = True
  (==) Saga _ = False
  (==) Human Human = True
  (==) Human _ = False
  (==) Advisor Advisor = True
  (==) Advisor _ = False
  (==) Wizard Wizard = True
  (==) Wizard _ = False
  (==) Merfolk Merfolk = True
  (==) Merfolk _ = False
  (==) Shaman Shaman = True
  (==) Shaman _ = False
  (==) Vedalken Vedalken = True
  (==) Vedalken _ = False
  (==) Artificer Artificer = True
  (==) Artificer _ = False
  (==) Zombie Zombie = True
  (==) Zombie _ = False
  (==) Army Army = True
  (==) Army _ = False
  (==) Soldier Soldier = True
  (==) Soldier _ = False
  (==) Knight Knight = True
  (==) Knight _ = False
  (==) Myr Myr = True
  (==) Myr _ = False
  (==) Elk Elk = True
  (==) Elk _ = False
  (==) Treefolk Treefolk = True
  (==) Treefolk _ = False
  (==) Siege Siege = True
  (==) Siege _ = False
  (==) Imp Imp = True
  (==) Imp _ = False
  (==) Saheeli Saheeli = True
  (==) Saheeli _ = False
  (==) Jace Jace = True
  (==) Jace _ = False
  (==) Elspeth Elspeth = True
  (==) Elspeth _ = False
  (==) Thopter Thopter = True
  (==) Thopter _ = False
  (==) Construct Construct = True
  (==) Construct _ = False
  (==) Fractal Fractal = True
  (==) Fractal _ = False
  (==) Coward Coward = True
  (==) Coward _ = False
  (==) Demon Demon = True
  (==) Demon _ = False
  (==) Illusion Illusion = True
  (==) Illusion _ = False
  (==) Sliver Sliver = True
  (==) Sliver _ = False
  (==) Wall Wall = True
  (==) Wall _ = False
  (==) Cleric Cleric = True
  (==) Cleric _ = False
  (==) Angel Angel = True
  (==) Angel _ = False
  (==) Elemental Elemental = True
  (==) Elemental _ = False
  (==) Plant Plant = True
  (==) Plant _ = False
  (==) Dragon Dragon = True
  (==) Dragon _ = False
  (==) Plains Plains = True
  (==) Plains _ = False
  (==) Island Island = True
  (==) Island _ = False
  (==) Swamp Swamp = True
  (==) Swamp _ = False
  (==) Mountain Mountain = True
  (==) Mountain _ = False
  (==) Forest Forest = True
  (==) Forest _ = False
  (==) Aura Aura = True
  (==) Aura _ = False
  (==) Curse Curse = True
  (==) Curse _ = False
  (==) Tiefling Tiefling = True
  (==) Tiefling _ = False
  (==) Warlock Warlock = True
  (==) Warlock _ = False
  (==) Pirate Pirate = True
  (==) Pirate _ = False
  (==) Cat Cat = True
  (==) Cat _ = False
  (==) Beast Beast = True
  (==) Beast _ = False
  (==) Dwarf Dwarf = True
  (==) Dwarf _ = False
  (==) Bard Bard = True
  (==) Bard _ = False
  (==) Hero Hero = True
  (==) Hero _ = False
  (==) Elf Elf = True
  (==) Elf _ = False
  (==) Scout Scout = True
  (==) Scout _ = False
  (==) Rogue Rogue = True
  (==) Rogue _ = False
  (==) Druid Druid = True
  (==) Druid _ = False
  (==) Alien Alien = True
  (==) Alien _ = False
  (==) Warrior Warrior = True
  (==) Warrior _ = False
  (==) Vampire Vampire = True
  (==) Vampire _ = False
  (==) Mutant Mutant = True
  (==) Mutant _ = False
  (==) AssemblyWorker AssemblyWorker = True
  (==) AssemblyWorker _ = False
  (==) Ooze Ooze = True
  (==) Ooze _ = False
  (==) Frog Frog = True
  (==) Frog _ = False
  (==) Horse Horse = True
  (==) Horse _ = False
  (==) Bird Bird = True
  (==) Bird _ = False
  (==) Ally Ally = True
  (==) Ally _ = False
  (==) Gideon Gideon = True
  (==) Gideon _ = False
  (==) Arcane Arcane = True
  (==) Arcane _ = False

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
subtypeType Golem = Creature
subtypeType Town = Land
subtypeType Desert = Land
subtypeType Pegasus = Creature
subtypeType Faerie = Creature
subtypeType Kraken = Creature
subtypeType Sphinx = Creature
subtypeType Werewolf = Creature
subtypeType Eldrazi = Creature
subtypeType Fungus = Creature
subtypeType Goat = Creature
subtypeType Ox = Creature
subtypeType Boar = Creature
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
subtypeType Snake = Creature
subtypeType Arcane = Instant
subtypeType Adventure = Instant
subtypeType Arlinn = Planeswalker

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

||| [CR#122.1b] closes the keyword-counter list by enumeration — flying,
||| first strike, double strike, deathtouch, decayed, exalted, haste,
||| hexproof, indestructible, lifelink, menace, reach, shadow, trample and
||| vigilance, and variants of those — so every zero here is that rule's.
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
keywordCounterOk Unearth = False
keywordCounterOk Flashback = False
keywordCounterOk Dredge = False
keywordCounterOk Retrace = False
keywordCounterOk Cycling = False
keywordCounterOk Ninjutsu = False
keywordCounterOk Miracle = False
keywordCounterOk Warp = False

public export
KeywordCounterEligible : Keyword -> Type
KeywordCounterEligible k = So (keywordCounterOk k)

public export
data StackRegime = AtCasting | AtResolution

public export
Eq StackRegime where
  (==) AtCasting AtCasting = True
  (==) AtCasting _ = False
  (==) AtResolution AtResolution = True
  (==) AtResolution _ = False

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
-- [CR#702.185a] puts warp's two statics on the stack; the graveyard and
-- hand keywords name a zone off it.
keywordStackRegime Unearth = Nothing
keywordStackRegime Flashback = Nothing
keywordStackRegime Dredge = Nothing
keywordStackRegime Retrace = Nothing
keywordStackRegime Cycling = Nothing
keywordStackRegime Ninjutsu = Nothing
keywordStackRegime Miracle = Nothing
keywordStackRegime Warp = Just AtCasting

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



||| Printed order of the type words on a type line. Spelling-only: no gate
||| consumes it, and a type line's meaning does not depend on it. Awaiting a
||| home in english_v2's type-line construction (ticket
||| workbench-type-line-order-is-spelling).
public export
typePrintOrder : CardType -> Nat
typePrintOrder Kindred = 0
typePrintOrder Enchantment = 1
typePrintOrder Artifact = 2
typePrintOrder Land = 3
typePrintOrder Creature = 4
typePrintOrder Planeswalker = 5
typePrintOrder Battle = 6
typePrintOrder Instant = 7
typePrintOrder Sorcery = 8
typePrintOrder Conspiracy = 9
typePrintOrder Dungeon = 10
typePrintOrder Phenomenon = 11
typePrintOrder Plane = 12
typePrintOrder Scheme = 13
typePrintOrder Vanguard = 14

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
-- A spell is a card on the stack [CR#112.1]; the command-zone types
-- never reach it [CR#311.2,312.2,313.2,314.2,315.3,309.2c].
spellType Conspiracy = False
spellType Dungeon = False
spellType Phenomenon = False
spellType Plane = False
spellType Scheme = False
spellType Vanguard = False

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


