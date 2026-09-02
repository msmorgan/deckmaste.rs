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

||| The type's place in the catalog above. A positional code and no
||| meaning of its own: `cardTypeAt` decodes it, `cardTypeAtIx` closes the
||| round trip, and that round trip is what `sameCardTypeEq` is proved
||| from -- fifteen rows where reflecting an enumerated `==` back to
||| identity would need two hundred and twenty-five.
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
cardTypeIx Conspiracy = 9
cardTypeIx Dungeon = 10
cardTypeIx Phenomenon = 11
cardTypeIx Plane = 12
cardTypeIx Scheme = 13
cardTypeIx Vanguard = 14

||| The code's inverse, `Nothing` off the fifteen.
public export
cardTypeAt : Nat -> Maybe CardType
cardTypeAt 0 = Just Creature
cardTypeAt 1 = Just Artifact
cardTypeAt 2 = Just Land
cardTypeAt 3 = Just Enchantment
cardTypeAt 4 = Just Instant
cardTypeAt 5 = Just Sorcery
cardTypeAt 6 = Just Planeswalker
cardTypeAt 7 = Just Battle
cardTypeAt 8 = Just Kindred
cardTypeAt 9 = Just Conspiracy
cardTypeAt 10 = Just Dungeon
cardTypeAt 11 = Just Phenomenon
cardTypeAt 12 = Just Plane
cardTypeAt 13 = Just Scheme
cardTypeAt 14 = Just Vanguard
cardTypeAt _ = Nothing

||| A card type is its own code's decoding.
public export
cardTypeAtIx : (t : CardType) -> cardTypeAt (cardTypeIx t) = Just t
cardTypeAtIx Creature = Refl
cardTypeAtIx Artifact = Refl
cardTypeAtIx Land = Refl
cardTypeAtIx Enchantment = Refl
cardTypeAtIx Instant = Refl
cardTypeAtIx Sorcery = Refl
cardTypeAtIx Planeswalker = Refl
cardTypeAtIx Battle = Refl
cardTypeAtIx Kindred = Refl
cardTypeAtIx Conspiracy = Refl
cardTypeAtIx Dungeon = Refl
cardTypeAtIx Phenomenon = Refl
cardTypeAtIx Plane = Refl
cardTypeAtIx Scheme = Refl
cardTypeAtIx Vanguard = Refl

||| Equality is code equality: [CR#205.2a] gives each type its own word,
||| so no two share a place in the catalog.
public export
Eq CardType where
  (==) a b = cardTypeIx a == cardTypeIx b

||| `So (m == n)` is `m = n` for naturals.
public export
natEqSo : (m, n : Nat) -> So (m == n) -> m = n
natEqSo Z Z Oh = Refl
natEqSo Z (S _) Oh impossible
natEqSo (S _) Z Oh impossible
natEqSo (S j) (S k) ok = cong S (natEqSo j k ok)

||| ...and a natural matches itself.
public export
natEqRefl : (n : Nat) -> So (n == n)
natEqRefl Z = Oh
natEqRefl (S k) = natEqRefl k

||| `==` decides equality: a match is the identity of the two card types.
public export
sameCardTypeEq : (a, b : CardType) -> So (a == b) -> a = b
sameCardTypeEq a b ok =
  sameJust (trans (sym (cardTypeAtIx a))
                  (trans (cong cardTypeAt (natEqSo _ _ ok))
                         (cardTypeAtIx b)))
  where
    sameJust : {0 x, y : CardType} -> Just x = Just y -> x = y
    sameJust Refl = Refl

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


||| The sorts a chosen or bound quality may take. A `Q` suffix marks the
||| three whose bare name is already the type of value they range over --
||| `Subtype`, `CardType` and `CounterKind` are all data here.
public export
data QualitySort : Type where
  ||| "choose a color" [CR#105.1]
  Color : QualitySort
  ||| "choose a creature type", "choose a land type": one card type's
  ||| subtypes [CR#205.3c], the host carried beside the sort exactly as
  ||| `SubtypeAxis` carries it. WHICH of the host's subtypes a choice
  ||| runs over is the domain's business [CR#305.6], not the sort's, so
  ||| "a land type" and "a basic land type" are one sort under two
  ||| domains.
  SubtypeQ : CardType -> QualitySort
  ||| "the chosen name" [CR#201.4a]
  CardName : QualitySort
  ||| "the chosen number"
  Number : QualitySort
  ||| "choose a card type" [CR#205.2a]
  CardTypeQ : QualitySort
  ||| "choose a kind of counter" [CR#122.1]
  CounterKindQ : QualitySort

public export
Eq QualitySort where
  (==) Color Color = True
  (==) Color _ = False
  (==) (SubtypeQ a) (SubtypeQ b) = a == b
  (==) (SubtypeQ _) _ = False
  (==) CardName CardName = True
  (==) CardName _ = False
  (==) Number Number = True
  (==) Number _ = False
  (==) CardTypeQ CardTypeQ = True
  (==) CardTypeQ _ = False
  (==) CounterKindQ CounterKindQ = True
  (==) CounterKindQ _ = False

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
chosenQualityReadOk (SubtypeQ _) = True
chosenQualityReadOk CardName = True
chosenQualityReadOk Number = False
chosenQualityReadOk CardTypeQ = True
chosenQualityReadOk CounterKindQ = False

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
  ||| `AtLeastTwoArms`, so a heterogeneous fold seeds with a non-empty list's
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
sameQRefl (SubtypeQ h) = natEqRefl (cardTypeIx h)
sameQRefl CardName = Oh
sameQRefl Number = Oh
sameQRefl CardTypeQ = Oh
sameQRefl CounterKindQ = Oh

||| `==` decides equality: a match is the identity of the two sorts, and
||| at the subtype sort the identity of their hosts.
public export
sameQEq : (a, b : QualitySort) -> So (a == b) -> a = b
sameQEq Color Color _ = Refl
sameQEq Color (SubtypeQ y) ok = absurd ok
sameQEq Color CardName ok = absurd ok
sameQEq Color Number ok = absurd ok
sameQEq Color CardTypeQ ok = absurd ok
sameQEq Color CounterKindQ ok = absurd ok

sameQEq (SubtypeQ x) Color ok = absurd ok
sameQEq (SubtypeQ x) (SubtypeQ y) ok = cong SubtypeQ (sameCardTypeEq x y ok)
sameQEq (SubtypeQ x) CardName ok = absurd ok
sameQEq (SubtypeQ x) Number ok = absurd ok
sameQEq (SubtypeQ x) CardTypeQ ok = absurd ok
sameQEq (SubtypeQ x) CounterKindQ ok = absurd ok

sameQEq CardName Color ok = absurd ok
sameQEq CardName (SubtypeQ y) ok = absurd ok
sameQEq CardName CardName _ = Refl
sameQEq CardName Number ok = absurd ok
sameQEq CardName CardTypeQ ok = absurd ok
sameQEq CardName CounterKindQ ok = absurd ok

sameQEq Number Color ok = absurd ok
sameQEq Number (SubtypeQ y) ok = absurd ok
sameQEq Number CardName ok = absurd ok
sameQEq Number Number _ = Refl
sameQEq Number CardTypeQ ok = absurd ok
sameQEq Number CounterKindQ ok = absurd ok

sameQEq CardTypeQ Color ok = absurd ok
sameQEq CardTypeQ (SubtypeQ y) ok = absurd ok
sameQEq CardTypeQ CardName ok = absurd ok
sameQEq CardTypeQ Number ok = absurd ok
sameQEq CardTypeQ CardTypeQ _ = Refl
sameQEq CardTypeQ CounterKindQ ok = absurd ok

sameQEq CounterKindQ Color ok = absurd ok
sameQEq CounterKindQ (SubtypeQ y) ok = absurd ok
sameQEq CounterKindQ CardName ok = absurd ok
sameQEq CounterKindQ Number ok = absurd ok
sameQEq CounterKindQ CardTypeQ ok = absurd ok
sameQEq CounterKindQ CounterKindQ _ = Refl

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


||| The card type an axis presupposes. A characteristic's own answer
||| [CR#208.1,306.5]; none for a player's number, since [CR#109.3] makes
||| a characteristic a property of an OBJECT and a player is not one.
public export
axisType : ProjAxis -> Maybe CardType
axisType (CharAxis c) = comparedType c
axisType (PlayerStatAxis _) = Nothing

||| The type a LIST of axes presupposes, read disjunctively.
||| A list holds a type only when every member presupposes that same one:
||| "power or toughness" is still a creature's [CR#208.1], while "mana
||| value, power, or toughness" may be asked of any object, since mana
||| value presupposes nothing. A `joinSeed`-style collapse would answer
||| the second wrongly, so the silent member is what empties the answer
||| rather than what defers to its neighbours.
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

||| The axes a comparison at kind `k` may read, and the ONLY gate the
||| row needs: every member is scoped to `k` [CR#109.3], and the list is
||| non-empty because there is no clause for the empty one. The scope is
||| written as `projScope a` rather than checked against a Bool, so the
||| kind is SOLVED from the axes at each call site exactly as
||| `Superlative`'s own `projScope ax = k` solves it.
public export
data AxesAt : Kind -> List ProjAxis -> Type where
  LastAxis : (0 a : ProjAxis) -> AxesAt (projScope a) [a]
  NextAxis : (0 a : ProjAxis) -> AxesAt (projScope a) (b :: as) ->
             AxesAt (projScope a) (a :: b :: as)

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
||| `Nothing` is a vocabulary gap, never a refusal. The two left are
||| [CR#110.4]'s permanent types -- a second sort over [CR#205.2a]'s own
||| words under a restriction, and the corpus writes one line of it --
||| and [CR#105.5]'s pairs, whose value is a PAIR and so no sort the
||| quality vocabulary carries; the latter is what blocks Niv-Mizzet
||| Reborn's domainless pass.
public export
kindAxisSort : KindAxis -> Maybe QualitySort
kindAxisSort CardTypeAxis = Just CardTypeQ
kindAxisSort PermanentTypeAxis = Nothing
kindAxisSort ColorAxis = Just Color
kindAxisSort (SubtypeAxis host _) = Just (SubtypeQ host)
kindAxisSort (ValueAxis _) = Just Number
kindAxisSort CounterKindAxis = Just CounterKindQ
kindAxisSort ColorPairAxis = Nothing

||| Whether the axis's values are a set the RULES close, so a pass may
||| run over them with no domain to draw them from ("For each color,
||| return up to one target card of that color from your graveyard").
||| [CR#105.1] closes the colours at five, [CR#205.2a] enumerates the
||| card types, [CR#110.4] the permanent types among them, [CR#105.5]
||| the ten pairs, and [CR#305.6] the five basic land types. No rule
||| closes the other subtype sets -- the creature types [CR#205.3m] are
||| a printed list amended set by set -- nor [CR#122.1]'s counter kinds,
||| which that rule leaves to any word a card writes; a characteristic's
||| values are unbounded. A domainless pass over an unclosed set names
||| no range.
||| Only the colour and card-type arms are WRITTEN domainless (5 lines
||| and 2); the basic land type, permanent type and pair arms stand open
||| at their zero.
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
                 -- what a PLANAR roll leaves: a face and never a number.
                 -- [CR#901.3a] gives the die one Planeswalker face, one
                 -- chaos face and four blanks, and [CR#706.7] has every
                 -- numerical read ignore the planar roll, so the mention
                 -- carries no value at all -- it is what "ignore one"
                 -- names and what no quantity read may take.
                 | PlanarRolled
                 | NamedNumber
                 -- `RepeatCount` is the number of iterations a repetition
                 -- WROTE, not the size of the batch it produced:
                 -- [CR#609.3] has an effect do only as much as possible,
                 -- so a player told to discard three cards holding two
                 -- discards two. The batch's own size is read off its
                 -- summary mention (`GroupSize`).
                 | RepeatCount
                 -- what a counter REMOVAL leaves: "for each charge counter
                 -- removed this way" (the five Mana Batteries and the
                 -- eleven storage lands). Its own sort beside
                 -- `CountersPut`, not a spelling of it: [CR#122.5] makes
                 -- a move a remove AND a put, so one clause can leave
                 -- both numbers and a single sort could not tell the
                 -- read apart. [CR#122.2] draws the same line from the
                 -- other side -- counters that cease to exist on a zone
                 -- change are "not 'removed'" -- so the word names an
                 -- action and not any loss of counters.
                 | CountersRemoved
                 -- the mana a clause PUT INTO A POOL: [CR#106.4] has an
                 -- add instruction send its mana to a player's mana pool,
                 -- where it stays as unspent mana, and that is what the
                 -- next sentence's "this mana" points at ("Until end of
                 -- turn, you don't lose this mana as steps and phases
                 -- end", Tundra Fumarole). An OUTCOME and not an object:
                 -- mana is a resource a pool holds, no `Kind` names it,
                 -- and `itReaches` demands an object, so "it" can never
                 -- reach this mention -- which is right, since no line
                 -- writes "it" for mana.
                 | ManaAdded
                 -- a MANA ABILITY resolved and produced mana here.
                 -- [CR#106.12a] fires the tapped-for-mana trigger
                 -- "whenever such a mana ability resolves and produces
                 -- mana", so the header's own event is a production and
                 -- not merely an act, and that is what "one mana of any
                 -- type that land produced" reads back off.
                 -- Its own sort beside `ManaAdded`: the two answer
                 -- different questions -- what went into a pool, against
                 -- what an ability in scope was able to make -- and one
                 -- sort would let "that land produced" be written after a
                 -- bare add, where no land was tapped at all.
                 | ManaProduced

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

||| Which possessor a relational noun derives from an object. The two
||| axes are the two the rules define over a card: [CR#108.3] gives every
||| card an owner from the start of the game and never takes it away,
||| and [CR#109.4] gives a controller only to objects on the stack or the
||| battlefield. `ControllerOf` and `OwnerOf` predate this and stay two
||| constructors; the axis is data because the MEMBER-WISE possessor
||| reads one relation at either axis and would otherwise pay a second
||| constructor's tables to say so.
public export
data PossessorAxis = OwnerAx | ControllerAx

||| [CR#400.1]'s seven zones. A sideboard is not among them and is not
||| added beside `Command`: [CR#400.11a] puts sideboard cards outside the
||| game, and [CR#400.11] states outright that outside the game is not a
||| zone.
public export
data Zone = Battlefield | Graveyard | Exile | Hand | Library | Stack
          | Command

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
  (==) Command Command = True
  (==) Command _ = False

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
||| two fields are SPELLING. The last three are the action's own rule
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
  ||| where the action's own rule FINDS that patient: [CR#701.9a]
  ||| discards from a hand, [CR#701.17a] mills from the top of a library,
  ||| [CR#701.8a] and [CR#701.21a] each take a permanent off the
  ||| battlefield. `Nothing` where the rule names no zone to take it from
  ||| ([CR#701.13a] exiles an object from wherever it is) or where the
  ||| clause that instructs the act names the zone itself ("Put"). It is
  ||| the verbed event's zone gate, which is why that reading needs no
  ||| zone table of its own.
  actZone : Maybe Zone
  ||| where the action's own rule leaves that patient: a graveyard for
  ||| [CR#701.9a] and [CR#701.17a], the exile zone for [CR#701.13a].
  ||| `Nothing` where the rule moves nothing ([CR#701.26a] turns a
  ||| permanent sideways) or states no destination of its own, in which
  ||| case the patient stays in the zone it was named in.
  actDest : Maybe Zone
  ||| whether the action's own rule performs it in ORDERED STEPS, so that
  ||| there is a moment INSIDE the act for a clause to name -- [CR#701.22a]
  ||| looks at the top N cards and THEN puts them, and [CR#701.25a] writes
  ||| the same two steps, which is what "while scrying" names. False where
  ||| the rule states the act as one change ([CR#701.8a], [CR#701.9a]),
  ||| leaving nothing between a beginning and an end to stand in. Read by
  ||| the header's concurrent clause through `eventUnderwayOk`, and by
  ||| nothing else: the act still HAPPENS at one moment for every
  ||| retrospective reader [CR#603.2].
  actStepwise : Bool
  ||| the zones the action's own rule performs it IN, where the rule
  ||| performs it on a ZONE rather than on an object patient:
  ||| [CR#701.23a] searches for a card in a zone, [CR#701.24a] shuffles a
  ||| library. `[]` everywhere else, which is every act whose rule names
  ||| a patient and every act whose rule takes a number. It is the
  ||| complement's locus gate -- what "you search your library this way"
  ||| writes -- and it is a LIST because one rule may name several zones
  ||| where another names one.
  actLoci : List Zone
  ||| whether the action's own rule spells the act with its PATIENT as the
  ||| surface subject in the ACTIVE voice -- "this creature transforms" --
  ||| rather than only as something an actor does to an object.
  ||| [CR#701.27e] states that spelling outright: "some triggered
  ||| abilities trigger when an object 'transforms into' an object with a
  ||| specified characteristic", and the act's patient is what that
  ||| sentence puts before the verb. The same rule licenses the "into
  ||| [what it became]" complement, which is why ONE field carries both:
  ||| no printed line writes either without the rule that states the pair.
  ||| False for every act whose rule names an actor performing it on
  ||| something ([CR#701.8a] destroys, [CR#701.9a] discards). Those spell
  ||| the actorless voice as a passive with the participle instead, which
  ||| is what `participle` is for -- and the two are exclusive in fact as
  ||| well as in spelling: `Transform` records no participle because
  ||| [CR#701.27g] gave those words to a state.
  actIntransitive : Bool

||| The label vocabulary, open by construction: a row is a name, its
||| participle and what its own rule says about the act, and adding one
||| adds no obligation anywhere else.
public export
verbFacts : List VerbFacts
verbFacts =
  --                            participle       patient
  --                            zone             destination
  [ MkVerbFacts "Destroy"     (Just "destroyed") (Just Object)
                              (Just Battlefield) (Just Graveyard) False [] False
  , MkVerbFacts "Sacrifice"   (Just "sacrificed") (Just Object)
                              (Just Battlefield) (Just Graveyard) False [] False
  -- [CR#701.13a] exiles an object from wherever it is, so the act names
  -- no zone to take it from.
  , MkVerbFacts "Exile"       (Just "exiled")    (Just Object)
                              Nothing            (Just Exile) False [] False
  , MkVerbFacts "Discard"     (Just "discarded") (Just Object)
                              (Just Hand)        (Just Graveyard) False [] False
  , MkVerbFacts "Mill"        (Just "milled")    (Just Object)
                              (Just Library)     (Just Graveyard) False [] False
  -- [CR#701.22b] and [CR#701.25c] both name a trigger on the act, and
  -- both rules write a NUMBER of cards looked at rather than a patient
  -- the act carries off, so the event announces no such thing.
  -- [CR#701.22a] looks at the top N cards, THEN puts any number of them
  -- on the bottom: two ordered steps, so the act has a moment inside it
  -- and "while scrying" names one.
  , MkVerbFacts "Scry"        Nothing            Nothing
                              Nothing            Nothing True [] False
  -- [CR#701.25a] writes the same two steps into the graveyard.
  , MkVerbFacts "Surveil"     Nothing            Nothing
                              Nothing            Nothing True [] False
  -- [CR#701.26a] turns a PERMANENT sideways and leaves it where it is:
  -- a zone to find the patient in, and no destination.
  , MkVerbFacts "Tap"         (Just "tapped")    (Just Object)
                              (Just Battlefield) Nothing False [] False
  -- [CR#701.26b] rotates a PERMANENT back upright and leaves it where
  -- it is: `Tap`'s row read the other way, and the participle is that
  -- row's too. It is what "Untap target creature. It gains haste until
  -- end of turn" reads back -- 99 supported faces, every one of them
  -- naming the permanent the untap acted on.
  , MkVerbFacts "Untap"       (Just "untapped")  (Just Object)
                              (Just Battlefield) Nothing False [] False
  -- "Return" is no [CR#701] keyword action, and stands here on "Put"'s
  -- ground: [CR#701.1] leaves an unkeyworded verb its standard English
  -- meaning and the body says the rest. Both ends are the clause's --
  -- [CR#400.1]'s zones are what a return line names on either side --
  -- so the row states neither, and the participle is absent for "Put"'s
  -- reason: "the returned card" would have to name the destination too.
  -- What the label buys is the stamp, which "Return target creature card
  -- from your graveyard to the battlefield. It gains haste" reads back
  -- (93 supported faces, all producer-bound).
  , MkVerbFacts "Return"      Nothing            (Just Object)
                              Nothing            Nothing False [] False
  -- "Gain control" is no keyword action either, and moves nothing:
  -- [CR#613.1b] applies a control change in layer 2 and [CR#110.2] makes
  -- the controller a property of the permanent, which stays on the
  -- battlefield throughout. So the patient is found there and left
  -- there, and no participle: "the controlled creature" is what no
  -- printed line spells. The stamp is read by "Gain control of target
  -- creature until end of turn. Untap it" (19 supported faces).
  , MkVerbFacts "GainControl" Nothing            (Just Object)
                              (Just Battlefield) Nothing False [] False
  -- "Put" is no [CR#701] keyword action, and under labels that is
  -- unremarkable: a label needs no rules entry of its own, because its
  -- body speaks for it [CR#701.1]. Both ends are the clause's, so the
  -- label states neither -- which is why the EVENT reading of this label
  -- is the poorer term beside `PutInto` and is left at its printed zero.
  , MkVerbFacts "Put"         Nothing            (Just Object)
                              Nothing            Nothing False [] False
  -- the stamp a search leaves is read by the shuffle gate, not by a
  -- participle anaphor: no printed line names a search's patient that
  -- way, and "the searched card" is not what English would spell. What a
  -- printed line writes after the verb is the ZONE [CR#701.23a] has the
  -- act look in; the card it finds is named by the instructing clause.
  -- That zone is the LOCUS, and [CR#701.23a] refuses none of them: it
  -- looks at all cards in the named zone whatever zone that is, hidden
  -- ones included. The three the corpus writes are the possessed three
  -- [CR#400.1] gives each player their own of; the other four
  -- overgenerate at a printed zero, on `destTypeOk`'s terms.
  , MkVerbFacts "Search"      Nothing            Nothing
                              Nothing            Nothing False
                              [Battlefield, Graveyard, Exile, Hand,
                               Library, Stack, Command] False
  -- [CR#701.24a] shuffles a LIBRARY, and what a printed line writes
  -- after the verb is that pile or the cards the instructing clause
  -- names -- no patient the act itself carries off. The participle is
  -- absent for "put onto the battlefield"'s reason: "cards shuffled into
  -- your library this way" would have to name the destination too.
  -- [CR#701.24a]'s act is performed on a LIBRARY (or on a face-down
  -- pile, which is no zone), so the library is its locus. Stated from
  -- the rule and read by nothing: 0 supported lines write "shuffled
  -- your library this way" (measured 2026-09-02).
  , MkVerbFacts "Shuffle"     Nothing            Nothing
                              Nothing            Nothing False [Library] False
  -- [CR#701.34a] has the ACT make its own choice -- "choose any number
  -- of permanents and/or players that have a counter" -- rather than
  -- take a patient from the instructing clause, so every printed line
  -- writes the verb with nothing after it and "whenever you
  -- proliferate" announces no participant. Nothing moves, so no
  -- destination either, and no printed line names a proliferated
  -- permanent by participle.
  , MkVerbFacts "Proliferate" Nothing            Nothing
                              Nothing            Nothing False [] False
  -- [CR#701.54] makes the Ring's temptation a keyword action, and
  -- [CR#701.54d] names the trigger on it outright -- "Some abilities
  -- trigger 'Whenever the Ring tempts you'" -- exactly as [CR#701.22b]
  -- does for the scry. [CR#701.54a]'s act chooses a Ring-bearer and
  -- [CR#701.54c] grants an emblem; nothing is carried anywhere, so the
  -- act names no patient, no zone to find one in and no destination,
  -- and no printed line names a participant of it by participle.
  -- The ACTOR is the tempted player: [CR#701.54d] has the Ring tempt a
  -- player whenever THEY complete [CR#701.54a]'s actions. That the
  -- printed clause writes the Ring as its surface subject and the player
  -- after the verb is the label's own spelling, which is why the label
  -- is the whole printed phrase.
  , MkVerbFacts "The Ring Tempts You" Nothing    Nothing
                              Nothing            Nothing False [] False
  -- [CR#701.27a] turns a PERMANENT over so that its other face is up,
  -- and admits only permanents represented by double-faced tokens and
  -- double-faced cards, so the act finds its patient on the battlefield
  -- and leaves it standing there: a zone, and no destination, exactly as
  -- the tap rows read.
  -- NO PARTICIPLE, and a rule says so rather than a printed zero.
  -- [CR#701.27g] has already given those words to a STATE -- a
  -- "transformed permanent" is "a double-faced permanent on the
  -- battlefield with its back face up", and "a permanent with its front
  -- face up is never considered a transformed permanent, even if it had
  -- its back face up previously" -- so a verbed anaphor spelling them
  -- would name the wrong permanents, the ones this act left front face
  -- up among them. 0 supported lines write "the transformed [noun]" as
  -- a lookback (measured 2026-08-28).
  , MkVerbFacts "Transform"   Nothing            (Just Object)
                              (Just Battlefield) Nothing False [] True
  -- [CR#701.28a] converts by turning a permanent so that its other face
  -- is up, and routes the whole of it back through the transform rules
  -- in as many words: one act under a second printed word, which is what
  -- two labels over one body are for. The row is `Transform`'s
  -- unchanged, for that reason.
  , MkVerbFacts "Convert"     Nothing            (Just Object)
                              (Just Battlefield) Nothing False [] True
  -- [CR#701.42a] melds the two cards of a meld pair by putting THEM
  -- onto the battlefield with their back faces up and combined: a
  -- patient the act carries and a destination it states. No zone of its
  -- own to find them in -- every printed line exiles them first ("exile
  -- them, then meld them into [Z]", 7 supported faces, measured
  -- 2026-08-28), which is the instructing clause's business and not the
  -- act's. No participle: what the act leaves is one permanent
  -- represented by two cards [CR#712.4a], so there is no melded object
  -- beside it for a later clause to name, and 0 supported lines write
  -- "melded" at all.
  , MkVerbFacts "Meld"        Nothing            (Just Object)
                              Nothing            (Just Battlefield) False [] False
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

||| Where the action's own rule FINDS its patient, when it names a zone.
public export
actZoneOf : VerbLabel -> Maybe Zone
actZoneOf v = verbFactsFor v >>= actZone

||| Where the action's own rule leaves its patient, when it moves it.
public export
actDestOf : VerbLabel -> Maybe Zone
actDestOf v = verbFactsFor v >>= actDest

||| The zones the action's own rule performs it IN. An unknown label
||| answers `[]`, which every seat that writes one already refuses
||| through `KnownVerb`.
public export
actLociOf : VerbLabel -> List Zone
actLociOf v = maybe [] actLoci (verbFactsFor v)

||| Whether the act is performed on a ZONE at all: what decides between
||| "you searched your library" and the acts whose clause names no place.
public export
actNamesLocus : VerbLabel -> Bool
actNamesLocus v = case actLociOf v of
                    [] => False
                    _ => True

||| Whether the action's own rule performs it in ordered steps. An
||| unknown label answers False, which is the same answer `KnownVerb`
||| already refuses at every seat that writes one.
public export
actStepwiseOf : VerbLabel -> Bool
actStepwiseOf v = maybe False actStepwise (verbFactsFor v)

||| Whether the action's own rule spells the act with its patient as the
||| surface subject, and with it admits the "into [what it became]"
||| complement [CR#701.27e]. An unknown label answers False.
public export
actIntransitiveOf : VerbLabel -> Bool
actIntransitiveOf v = maybe False actIntransitive (verbFactsFor v)

||| Whether the label records a participle at all -- what the ACTORLESS
||| passive needs in order to be spellable ("[what] is [participle]").
||| `Transform` answers False and `IntransitiveAct` is the voice it
||| writes instead.
public export
actNamesParticiple : VerbLabel -> Bool
actNamesParticiple v = isJust (participleOf v)

||| What a labeled action left on the mention it acted on: WHICH label
||| acted, whether it found the referent on the battlefield, and whether
||| it took the referent out of the zone it was in. The third fact is not
||| derivable from the first two -- a label may move its patient
||| ("exile it") or leave it where it stands ("tap it") -- and only a
||| MOVE takes the counters with it [CR#400.7].
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
data Payload : Kind -> Type where
  ||| The last field is the mention's own CARDINALITY, where the text
  ||| stated one: "the top two cards of your library" knows it is two,
  ||| "one of them" knows it is one, and a description knows nothing.
  ||| It is recorded here because a binding is what the anaphora reads,
  ||| and the subset complement -- "put one of them into your hand and
  ||| THE OTHER into your graveyard", 104 occurrences over 102 supported
  ||| cards (re-measured 2026-08-28) -- is a read no other fact answers:
  ||| what makes the remainder a singular "the other" rather than a
  ||| plural "the rest" is that the group was counted and all but one
  ||| member has been taken.
  ||| `Nothing` is the ordinary answer and means only that no count was
  ||| stated, never that the set is empty or unbounded.
  ObjectP : (ty : Maybe CardType) -> (zone : Maybe Zone) ->
            (prov : Maybe Stamp) -> (orig : Maybe Origin) ->
            (size : Maybe Nat) -> Payload Object
  PlayerP : Payload Player
  ||| A player a CHOICE bound, where `PlayerP` is any other player
  ||| mention. [CR#607.2d] links "choose a [value]" to the later "the
  ||| chosen [value]" and the counting that finds the link has to tell a
  ||| chosen player from "you" or "each opponent"; nothing else about the
  ||| mention differs, so it echoes and describes exactly as `PlayerP`
  ||| does. The quality sorts need no such mark -- their kind already
  ||| says a choice made them -- which is why this is the one payload
  ||| the choice vocabulary adds.
  ChosenPlayerP : Payload Player
  QualityP : Payload (Quality q)
  OutcomeP : (sort : OutcomeSort) -> Payload Outcome
  GapP : Payload Gap
  LetterP : Payload (LetterK l)
  TurnRefP : Payload TurnRef
  ||| An ability on the stack [CR#113.3b,113.3c], and its ORIGIN, for
  ||| `ObjectP`'s reason at the other kind the stack holds: [CR#707.10]
  ||| copies "a spell, activated ability, or triggered ability" alike, so
  ||| a copy clause can put an ability mention into the discourse beside
  ||| the ability it copied, and "the copy" has to be able to tell them
  ||| apart. `TokenOrigin` never appears here -- [CR#111.1] makes only
  ||| permanents -- so the field's live values are `Just CopyOrigin` and
  ||| `Nothing`; it is `Maybe Origin` and not a flag because the reading
  ||| words ask `isCopyOrigin`, one question for both kinds.
  AbilityP : (orig : Maybe Origin) -> Payload Ability
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
payloadZone (ObjectP _ zn _ _ _) = zn
payloadZone PlayerP = Nothing
payloadZone ChosenPlayerP = Nothing
payloadZone QualityP = Nothing
payloadZone (OutcomeP _) = Nothing
payloadZone GapP = Nothing
payloadZone LetterP = Nothing
payloadZone TurnRefP = Nothing
payloadZone (AbilityP _) = Nothing
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
payloadTy (ObjectP ty _ _ _ _) = ty
payloadTy PlayerP = Nothing
payloadTy ChosenPlayerP = Nothing
payloadTy QualityP = Nothing
payloadTy (OutcomeP _) = Nothing
payloadTy GapP = Nothing
payloadTy LetterP = Nothing
payloadTy TurnRefP = Nothing
payloadTy (AbilityP _) = Nothing
payloadTy (JoinP l r) = joinSeed (payloadTy l) (payloadTy r)

public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ pl) = payloadZone pl

public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ pl) = payloadTy pl

||| How many members a mention has, where the text counted them. Only an
||| object mention can carry a count -- the other payloads name one thing
||| apiece -- and a join reports its object half's, on `payloadZone`'s
||| model.
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
payloadSize (JoinP l r) = maybe (payloadSize r) Just (payloadSize l)

public export
bindingSize : Binding -> Maybe Nat
bindingSize (MkBinding _ _ _ pl) = payloadSize pl

||| Write a stated count onto a mention `bindFor` already built. The
||| determiner rows that carry a quantity mint their binding through
||| `bindFor`, which sees the description and not the count, so the count
||| is written here rather than threaded through a slot nothing else
||| wants.
public export
sized : Maybe Nat -> Binding -> Binding
sized sz (MkBinding det Object pl (ObjectP ty zn pv og _)) =
  MkBinding det Object pl (ObjectP ty zn pv og sz)
sized _ b = b


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
  (==) PlanarRolled PlanarRolled = True
  (==) PlanarRolled _ = False
  (==) NamedNumber NamedNumber = True
  (==) NamedNumber _ = False
  (==) RepeatCount RepeatCount = True
  (==) RepeatCount _ = False
  (==) CountersRemoved CountersRemoved = True
  (==) CountersRemoved _ = False
  (==) ManaAdded ManaAdded = True
  (==) ManaAdded _ = False
  (==) ManaProduced ManaProduced = True
  (==) ManaProduced _ = False

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

||| What a CHOICE may bind. Every quality sort, and the player.
||| A player is deliberately NOT a `QualitySort`: that catalog ranges
||| over [CR#109.3]'s characteristics, which is why `chosenQualityReadOk`
||| can ask whether an object matches a chosen value, and a player is no
||| characteristic of anything. What the two share is [CR#607.2d], which
||| links "choose a [value]" to "the chosen [value]" over a VALUE of any
||| kind, so the counting that finds the link is one function over both.
public export
data ChoiceSort : Type where
  QSort : QualitySort -> ChoiceSort
  ||| "choose a player", "choose an opponent" [CR#607.2d].
  PlayerC : ChoiceSort

public export
Eq ChoiceSort where
  (==) (QSort a) (QSort b) = a == b
  (==) (QSort _) PlayerC = False
  (==) PlayerC (QSort _) = False
  (==) PlayerC PlayerC = True

||| The mention a chooser leaves. Indefinite at both sorts: the choice
||| INTRODUCES its value, and the definite word waits for the read.
public export
choiceB : ChoiceSort -> Binding
choiceB (QSort q) = qualityB q
choiceB PlayerC = MkBinding AD Player OneOf ChosenPlayerP

||| Whether one binding is a mention of the sort a choice bound. At a
||| quality sort the KIND says it, under `kindLte` exactly as
||| `countOnes` reads it; at the player sort the kind is shared with
||| every other player mention, so the payload's mark says it instead.
public export
choiceBinds : ChoiceSort -> (k : Kind) -> Payload k -> Bool
choiceBinds (QSort q) k _ = kindLte (Quality q) k
choiceBinds PlayerC _ ChosenPlayerP = True
choiceBinds PlayerC _ _ = False

||| Which sort a chooser at kind `k` binds, where the kind is one a
||| choice can bind a VALUE at. [CR#607.2d]'s linkage runs over "a
||| [value]", and the two kinds that carry one are the qualities
||| [CR#109.3] and the player. Every other kind answers `Nothing`,
||| the `Object` one included: "choose a creature card exiled with
||| [this]" announces a MENTION, which the ordinary anaphora reads,
||| and no chosen VALUE for a linked ability to name.
public export
choiceSortAt : Kind -> Maybe ChoiceSort
choiceSortAt (Quality q) = Just (QSort q)
choiceSortAt Player = Just PlayerC
choiceSortAt _ = Nothing

||| The choice binding a chooser at kind `k` leaves, as a delta.
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

||| Whether the discourse carries a planar die some clause rolled. An
||| existence test on `coinFlipInScope`'s model and never a count: the
||| roll leaves no number [CR#706.7], so there is nothing here for a
||| quantity read to be confused by, and one clause may roll several
||| ("roll that many planar dice plus one").
public export
planarRollInScope : Bindings -> Bool
planarRollInScope [] = False
planarRollInScope (MkBinding _ Outcome OneOf (OutcomeP PlanarRolled) :: _) = True
planarRollInScope (_ :: bs) = planarRollInScope bs

||| Whether the discourse carries something an ignore instruction can set
||| aside: a roll the text made, a coin it flipped, or a planar die it
||| rolled. [CR#706.6] gives the word its meaning over a roll -- the
||| ignored one "is considered to have never happened" -- and the printed
||| lines write the same word over each of the three randomisers, so the
||| gate is their disjunction rather than the roll's test alone.
||| The roll's own arm stays a count of one, which is what every other
||| read of a roll asks for; the other two are existence tests, since
||| neither leaves a number that two mentions could confuse.
public export
ignorableInScope : Bindings -> Bool
ignorableInScope bs =
  countOutcomes RollResult bs == 1 || coinFlipInScope bs ||
  planarRollInScope bs

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
-- and a planar roll leaves none: [CR#706.7] has every effect that refers
-- to a numerical result of a die roll ignore the rolling of the planar
-- die, and [CR#901.3a]'s faces are not numbered.
outcomeIsQuantity PlanarRolled = False
-- a defining sentence names a number outright [CR#604.3,208.1], so the
-- anaphor that reads a quantity back ("that number") has one to name.
outcomeIsQuantity NamedNumber = True
-- so does a repetition: the text wrote how many times.
outcomeIsQuantity RepeatCount = True
-- and a removal leaves the number of counters taken off.
outcomeIsQuantity CountersRemoved = True
-- mana leaves no number for "that much" to name. [CR#106.1b] gives mana
-- six TYPES, and what a later sentence reads back is mana of those types
-- ("you don't lose THIS MANA", "add the mana lost this way") and never an
-- amount of it: 0 supported lines read a quantity off an add (measured
-- 2026-08-28). A production in scope leaves no number either --
-- what [CR#106.12a]'s trigger makes readable is the TYPE its source
-- produced, which `ProducedByEvent` names and no quantity read reaches.
outcomeIsQuantity ManaAdded = False
outcomeIsQuantity ManaProduced = False

||| What "that much" folds: the outcome mentions that carry a number.
public export
countQuantOutcomes : Bindings -> Nat
countQuantOutcomes [] = Z
countQuantOutcomes (MkBinding _ Outcome OneOf (OutcomeP s) :: bs) =
  if outcomeIsQuantity s then S (countQuantOutcomes bs) else countQuantOutcomes bs
countQuantOutcomes (_ :: bs) = countQuantOutcomes bs

||| The singular mentions a choice at sort `s` left. `countOnes`
||| generalized to the choice vocabulary rather than duplicated for it:
||| at a quality sort the two agree definitionally (`countQualityIsCountOnes`),
||| and the player sort is the arm `countOnes` cannot serve, because
||| "you" is a singular `Player` mention and no choice made it.
public export
countChoice : ChoiceSort -> Bindings -> Nat
countChoice s [] = Z
countChoice s (MkBinding _ k OneOf p :: bs) =
  if choiceBinds s k p then S (countChoice s bs) else countChoice s bs
countChoice s (_ :: bs) = countChoice s bs

||| The marked read demands that a choice STAND, which is existence and
||| not uniqueness [CR#607.2d]: "the last chosen color" names the latest
||| of however many the chooser made, and zero is not one or more.
||| RECORDED OVERGENERATION, not gated: nothing in this grammar
||| represents a chooser's REPEATABILITY, so the row admits a marked read
||| after a chooser that can fire only once -- a sentence no printed line
||| writes. Measured over the supported corpus: 12 occurrences over 12
||| cards, every one of them behind a chooser that can fire more than
||| once (a second printed chooser, a fused enters-and-upkeep trigger, a
||| combat/loyalty/activated chooser, or an attach rider that re-fires on
||| re-equip). Perfect covariance and still not a gate, in the same
||| posture the verb-provenance overgeneration takes: what would close it
||| is a fact about the CHOOSER, which is a different subsystem.
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

||| A context with every mention of one letter taken out of it.
||| [CR#107.3k] is what asks for it: "if an object's activated ability
||| has an {X}, [-X], or X in its activation cost, the value of X for
||| that ability is INDEPENDENT of any other values of X chosen for that
||| object or for other instances of abilities of that object", an
||| explicit exception to [CR#107.3i]. So an activated ability is
||| written in a context where the object's other X is not there to be
||| read, and the ability's own cost opens its own.
public export
dropLetter : Letter -> Bindings -> Bindings
dropLetter l [] = []
dropLetter l (MkBinding d k pl pd :: bs) =
  if kindLte (LetterK l) k then dropLetter l bs
                           else MkBinding d k pl pd :: dropLetter l bs

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

||| "the other [X]": whether the prefix leaves a SINGLETON remainder --
||| one counted group, and parts taken out of it totalling all but one of
||| its members. That is the fact `theRestOk` cannot state and the reason
||| the subset complement had no row: "the rest" needs only that
||| something was taken, where "the other" needs to know how much is
||| left, and nothing recorded the sizes until `ObjectP` did.
||| The group's own count is read off the mention the text assembled;
||| a group whose size was never stated answers False, and the phrase
||| stays "the rest".
public export
partsTaken : Bindings -> Nat
partsTaken [] = Z
partsTaken (b@(MkBinding PartD _ _ _) :: bs) =
  if kindLte Object b.kind
    then (case bindingSize b of
            Just n => n + partsTaken bs
            Nothing => S (partsTaken bs))
    else partsTaken bs
partsTaken (_ :: bs) = partsTaken bs

public export
countedGroupSize : Bindings -> Maybe Nat
countedGroupSize [] = Nothing
countedGroupSize (MkBinding PartD _ _ _ :: bs) = countedGroupSize bs
countedGroupSize (b :: bs) =
  if objGroup b then bindingSize b else countedGroupSize bs

public export
theOtherOk : Bindings -> Bool
theOtherOk bs = theRestOk bs &&
                (case countedGroupSize bs of
                   Nothing => False
                   Just n => n == S (partsTaken bs))

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

||| Heterogeneously indexed on purpose: the binding's `kind` field is
||| compared separately by `sameBinding`, so two payloads are compared by
||| constructor and field alone.
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

||| A field two alternatives BOTH state, kept only where they state the
||| same thing. Where they differ the union has forgotten it, which is
||| not a loss of information but the mention's own content: the sentence
||| cannot say which alternative happened, so it may not claim what only
||| one of them said.
public export
agreedField : (a -> a -> Bool) -> Maybe a -> Maybe a -> Maybe a
agreedField f (Just x) (Just y) = if f x y then Just x else Nothing
agreedField f _ _ = Nothing

||| The UNION OF ALTERNATIVES over two payloads: what a mention names
||| when either of two announcements may have been the one that happened.
|||
||| Two shapes, and they are one operation seen at two kinds. Payloads of
||| the SAME sort keep the fields they agree on and forget the rest --
||| "cast an instant spell" beside "cast a sorcery spell" leaves a spell
||| on the stack whose type the sentence cannot state, which is exactly
||| what its own text calls it afterwards ("that spell"). Payloads of
||| DIFFERENT sorts pair into `JoinP`, the union family's own payload:
||| "cast a spell" beside "activate an ability" leaves the pair
||| [CR#115.2] names, read back by `AbilityJoinW`.
|||
||| The object half is always written LEFT, whatever order the arms came
||| in. The union is commutative and English is not: every printed line
||| writes "that spell or ability" and none writes the halves the other
||| way round.
|||
||| `Nothing` where no union exists -- a sort with no union word behind
||| it, or a pair the corpus never writes. A union that cannot be named
||| is not one the discourse may carry.
public export
unionPayload : {j : Kind} -> {k : Kind} -> Payload j -> Payload k ->
               Maybe (m : Kind ** Payload m)
unionPayload (ObjectP t1 z1 v1 o1 s1) (ObjectP t2 z2 v2 o2 s2) =
  Just (Object ** ObjectP (agreedField (==) t1 t2) (agreedField (==) z1 z2)
                          (agreedField sameStamp v1 v2)
                          (agreedField sameOrigin o1 o2)
                          (agreedField (==) s1 s2))
unionPayload (AbilityP o1) (AbilityP o2) =
  Just (Ability ** AbilityP (agreedField sameOrigin o1 o2))
unionPayload PlayerP PlayerP = Just (Player ** PlayerP)
unionPayload ChosenPlayerP ChosenPlayerP = Just (Player ** ChosenPlayerP)
unionPayload p@(ObjectP _ _ _ _ _) q@(AbilityP _) =
  Just (Object \/ Ability ** JoinP p q)
unionPayload p@(AbilityP _) q@(ObjectP _ _ _ _ _) =
  Just (Object \/ Ability ** JoinP q p)
unionPayload p@(ObjectP _ _ _ _ _) PlayerP = Just (Object \/ Player ** JoinP p PlayerP)
unionPayload PlayerP q@(ObjectP _ _ _ _ _) = Just (Object \/ Player ** JoinP q PlayerP)
unionPayload _ _ = Nothing

||| Two announcements unioned. The DETERMINER and the PLURALITY must
||| agree outright: those are what the reading word asks before it looks
||| at the payload, and a union that had forgotten them would name
||| something no word could reach. Identical announcements union to
||| themselves, which is what carries a coordination's shared prefix
||| through unchanged.
public export
unionBinding : Binding -> Binding -> Maybe Binding
unionBinding b c =
  if sameBinding b c then Just b
  else case (b, c) of
         (MkBinding d1 _ p1 pl1, MkBinding d2 _ p2 pl2) =>
           if sameDet d1 d2 && samePlur p1 p2
             then map (\(m ** pl) => MkBinding d1 m p1 pl) (unionPayload pl1 pl2)
             else Nothing

||| The union pointwise. Lists of different lengths have none: a
||| coordination whose arms announce different NUMBERS of things says
||| nothing determinate at any position, which is the reading the whole
||| agreement rule already gave them.
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
-- [CR#400.2] lists the command zone among the public zones.
publicZone Command = True

||| A state-based CAUSE of losing the game -- what a partial-cause
||| immunity carves out. The frame is [CR#104.3]'s three state-based
||| causes and only those: [CR#104.3b] "if a player's life total is 0 or
||| less, that player loses the game"; [CR#104.3c] drawing from a library
||| with too few cards; [CR#104.3d] ten or more poison counters. The
||| other ways [CR#104.3] names are not causes an effect can carve out at
||| all -- [CR#104.3a]'s concession is the one thing a card may never
||| override [CR#101.1], and [CR#104.3e]'s effect-stated loss is what the
||| immunity deliberately LEAVES standing.
|||
||| ONE row, because one is what is printed: all 7 supported lines write
||| [CR#104.3b] (Phyrexian Unlife, Lich, Lich's Tomb, Soul Echo,
||| Transcendence, Pact Weapon, Marina Vendrell's Grimoire). The other
||| two cells of the frame are named here so a printing of either costs a
||| row and nothing else; neither is held out by a rule.
public export
data LoseCause = ZeroOrLessLife

public export
Eq LoseCause where
  (==) ZeroOrLessLife ZeroOrLessLife = True

public export
data ExposeVerb = LookAt | Reveal

-- [CR#400.2]: battlefield, graveyard, exile and command are already
-- public zones, so revealing them says nothing; the library is hidden but
-- too large.
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

-- `VisibleThing` and `visibilityOk` moved to `Experimental.Phrase`:
-- the complement gained an OBJECT arm, and a `Noun` is not in scope here.

public export
pubB : Binding -> Bool
pubB (MkBinding _ _ _ (ObjectP _ (Just z) _ _ _)) = publicZone z
pubB (MkBinding _ _ _ (ObjectP _ Nothing _ _ _)) = True
pubB (MkBinding _ _ _ PlayerP) = True
pubB (MkBinding _ _ _ ChosenPlayerP) = True
pubB (MkBinding _ _ _ QualityP) = True
pubB (MkBinding _ _ _ (OutcomeP _)) = True  -- what happened is a public fact
pubB (MkBinding _ _ _ GapP) = True          -- so is a comparison's margin
pubB (MkBinding _ _ _ LetterP) = True       -- and so is a value the text defines
pubB (MkBinding _ _ _ TurnRefP) = True       -- and so is a value the text defines
pubB (MkBinding _ _ _ (AbilityP _)) = True       -- an ability class is public too
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
||| earlier clause moved names none [CR#400.7]. Carrying a stamp is NOT
||| that evidence on its own -- a label that changed its patient's state
||| and left it where it stood stamps the same mention ("each creature
||| tapped this way") -- so the question is asked of the fact the stamp
||| records, and of no table over labels.
public export
stampMoves : Maybe Stamp -> Bool
stampMoves Nothing = False
stampMoves (Just (MkStamp _ _ mv)) = mv

||| Whether a labeled action found this referent on the battlefield.
||| [CR#110.1] stops it being a permanent as it leaves, so a later clause
||| naming it "that permanent" is reading last known information
||| [CR#608.2h] -- which is the reading 15 supported lines write ("Destroy
||| target artifact or enchantment. … deals 2 damage to THAT PERMANENT's
||| controller"). The participle read already asks this same fact
||| (`verbedWordOk PermanentW`); the demonstrative asks it here.
public export
stampWasField : Maybe Stamp -> Bool
stampWasField Nothing = False
stampWasField (Just (MkStamp _ wasF _)) = wasF

||| The stamp a payload was left with, read like `payloadZone`.
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
payloadProv (JoinP l r) = maybe (payloadProv r) Just (payloadProv l)

||| The ORIGIN a payload records -- what made the referent -- read like
||| `payloadProv`. Only an object mention carries one, and only the two
||| clauses that make an object write it: a create clause [CR#111.1] and
||| a copy clause. A join reports its object half's, on `payloadZone`'s
||| model.
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
payloadOrig (JoinP l r) = maybe (payloadOrig r) Just (payloadOrig l)

||| An `It`/`Them` anaphor is of kind `Object`, and `kindLte` is what
||| lets it land on a JOINED antecedent: "any target ... that permanent
||| or player" resolves `Object` against `Object \/ Player`.
public export
itReaches : Plurality -> Binding -> Bool
itReaches pl b = kindLte Object b.kind && isOne pl == isOne b.plur

||| `itReaches` at the ABILITY kind: the ability pronoun's candidates.
||| [CR#109.1] makes an ability on the stack an object, so the printed
||| "it" is one word; this grammar indexes the two kinds apart so that
||| the ability predicates can be typed, and the pronoun therefore counts
||| its own mentions at each.
public export
itAbilityReaches : Plurality -> Binding -> Bool
itAbilityReaches pl b = kindLte Ability b.kind && isOne pl == isOne b.plur

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

||| The head words a demonstrative carries. Three of them read a UNION
||| mention back WHOLE where the rest name one referent: `JoinW` where
||| the antecedent reached a player ("that creature or player"),
||| `AbilityJoinW` where it reached an ability ("that spell or ability",
||| [CR#115.2]'s own pair), and `CopyJoinW` where the antecedent is a
||| union the COPY clause made. Each asks the antecedent's own kind, so
||| no two of them read one another's mentions.
|||
||| The copy words come in a pair for the same reason the union words
||| do, and part on kind the same way. [CR#707.10] copies "a spell,
||| activated ability, or triggered ability" alike, so a copy clause can
||| put an ability on the stack beside the ability it copied, and both
||| are `AbilityP`: `CopyW` names an object copy and `AbilityCopyW` an
||| ability copy. Both spell the same English word, exactly as `JoinW`
||| and `AbilityJoinW` both spell "that [x] or [y]" -- the word is one
||| and the kind is what a card's own text has already fixed.
public export
data NounWord = TypeW CardType | CardW | SpellW | PlayerW
              | PermanentW | TokenW | CopyW | JoinW | AbilityJoinW
              | ||| "Whenever you activate an ability, ... copy THAT
                ||| ABILITY": the plain ability demonstrative, and
                ||| `SpellW`'s row at the other kind the stack holds --
                ||| it reaches an `AbilityP` the copy clause did NOT make,
                ||| exactly as `SpellW` reaches a stack object that is no
                ||| copy. 15 supported lines write "copy that ability"
                ||| (re-measured 2026-09-02: Rings of Brighthearth,
                ||| Illusionist's Bracers, Lithoform Engine's kin), each
                ||| reading a header that announced one.
                ||| No zone is asked, for `AbilityCopyW`'s reason: the
                ||| ability kind places nothing [CR#113.3b,113.3c] and it
                ||| is the head's class that says the ability reached the
                ||| stack.
                ||| -- spelling: "that ability", "those abilities".
                AbilityW
              | ||| "Copy target triggered ability you control. You may
                ||| choose new targets for THE COPY": the copy mention at
                ||| the other kind the stack holds. 29 supported lines
                ||| write a copy of an ability (re-measured 2026-09-02:
                ||| 14 at "copy target [class] ability", 15 at "copy that
                ||| ability"), and every one of them reads the copy back
                ||| in the next sentence.
                ||| It asks `isCopyOrigin` of an `AbilityP`, where
                ||| `CopyW` asks it of an `ObjectP` on the stack: the
                ||| ability kind places nothing [CR#113.3b,113.3c], so
                ||| the origin is the whole question here and no zone is
                ||| asked.
                ||| -- spelling: "the copy" / "the copies".
                AbilityCopyW
              | ||| "When you next cast an instant spell, cast a sorcery
                ||| spell, or activate a loyalty ability this turn, copy
                ||| that spell or ability twice. You may choose new
                ||| targets for THE COPIES" (Repeated Reverberation): the
                ||| copy mention over a union antecedent. What the copy
                ||| clause copied was itself a union of alternatives, so
                ||| what it made is one too -- copies of whichever arm
                ||| fired -- and the mention names the pair.
                ||| `AbilityJoinW`'s question with the origin asked, as
                ||| `CopyW` is `SpellW`'s.
                ||| -- spelling: "the copy" / "the copies".
                CopyJoinW
              | ||| "the exiled creature card", "that land card": the type
                ||| word and the card word written TOGETHER. [CR#109.2]
                ||| makes the two readings different questions -- a
                ||| description carrying a card type and NOTHING else
                ||| means a permanent on the battlefield, which is what
                ||| `TypeW` asks -- and [CR#109.2a] states this one: the
                ||| word "card" beside a description names a card
                ||| matching it, wherever that description's zone puts
                ||| it. So this is not `TypeW` with a looser gate; it is
                ||| the other rule's word, and it asks `isCardZone` where
                ||| `TypeW` asks the battlefield.
                ||| -- spelling: "[type] card".
                TypedCardW CardType

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
-- [CR#903.3d]: an effect naming a commander in a zone names a CARD in
-- that zone, so the command zone's objects take the card word.
isCardZone (Just Command) = True

public export
onFieldZone : Maybe Zone -> Bool
onFieldZone Nothing = False
onFieldZone (Just Battlefield) = True
onFieldZone (Just Graveyard) = False
onFieldZone (Just Exile) = False
onFieldZone (Just Hand) = False
onFieldZone (Just Library) = False
onFieldZone (Just Stack) = False
onFieldZone (Just Command) = False

public export
onStackZone : Maybe Zone -> Bool
onStackZone Nothing = False
onStackZone (Just Battlefield) = False
onStackZone (Just Graveyard) = False
onStackZone (Just Exile) = False
onStackZone (Just Hand) = False
onStackZone (Just Library) = False
onStackZone (Just Stack) = True
onStackZone (Just Command) = False

||| The carrier a verb slot's own rule demands of the object it acts on,
||| and so the candidates a bare "it" written in that slot may resolve
||| to. [CR#109.2] gives the three: a description naming neither a zone
||| nor a carrier word means a permanent on the battlefield, "card" names
||| a card in a stated zone [CR#109.2a], and "spell" one on the stack
||| [CR#109.2b]. The rows are the same three sorts `PermanentW`, `CardW`
||| and `SpellW` filter on, read here off the ZONE alone: this is what a
||| verb demands of its object, not a word the card prints.
||| Sacrifice is the worked example -- [CR#701.21a] lets a player
||| sacrifice a permanent and nothing else -- so "…, sacrifice it" after
||| a header that announced a spell and the permanent it targeted has one
||| candidate, not two.
public export
data SlotCarrier = PermanentSlot | CardSlot | SpellSlot

public export
slotZoneOk : SlotCarrier -> Maybe Zone -> Bool
slotZoneOk PermanentSlot zn = onFieldZone zn
slotZoneOk CardSlot zn = isCardZone zn
slotZoneOk SpellSlot zn = onStackZone zn

||| `itReaches` narrowed to one slot's carrier. A mention that places its
||| referent nowhere is admitted by no slot: a union half carries no zone
||| of its own, so "a spell or ability" stands outside every verb's
||| candidate set rather than inside all of them.
public export
itAtReaches : SlotCarrier -> Binding -> Bool
itAtReaches sl b = itReaches OneOf b && slotZoneOk sl (bindingZone b)

||| `countOnes Object` over the candidates one verb slot admits. Still
||| counted uniqueness and still a fold over the reading-order prefix;
||| the narrowing is of WHICH mentions are candidates, not of how the
||| winner is picked. Where two candidates share the slot's carrier the
||| count is 2 and the read is refused, as it is today.
public export
countOnesAt : SlotCarrier -> Bindings -> Nat
countOnesAt sl [] = Z
countOnesAt sl (b :: bs) =
  if itAtReaches sl b then S (countOnesAt sl bs) else countOnesAt sl bs

public export
provOfItAt : SlotCarrier -> Bindings -> Maybe Stamp
provOfItAt sl [] = Nothing
provOfItAt sl (b :: bs) =
  if itAtReaches sl b then payloadProv b.payload else provOfItAt sl bs

public export
zoneOfItAt : SlotCarrier -> Bindings -> Maybe Zone
zoneOfItAt sl [] = Nothing
zoneOfItAt sl (b :: bs) =
  if itAtReaches sl b then bindingZone b else zoneOfItAt sl bs

public export
tyOfItAt : SlotCarrier -> Bindings -> Maybe CardType
tyOfItAt sl [] = Nothing
tyOfItAt sl (b :: bs) =
  if itAtReaches sl b then bindingTy b else tyOfItAt sl bs

||| The provenance a LABELED action leaves on the mention it acted on,
||| whatever the action did to it: a move writes one ("the exiled card"),
||| and so does a status change ("each creature tapped this way"). What
||| the stamp records is that this label acted on this referent, whether
||| it found it on the battlefield, and whether it carried it out of the
||| zone it was in -- never which kind of body the label rode. Body shapes
||| past those get a row when a printed line needs one.
||| The caller says whether the referent moved, because the caller is the
||| one holding both zones: `setZone` compares them, and the two deictic
||| rows know their subject's zone without a binding to read it off.
public export
mkStamp : Maybe VerbLabel -> (oldZn : Maybe Zone) -> (moved : Bool) -> Maybe Stamp
mkStamp Nothing oldZn moved = Nothing
mkStamp (Just v) oldZn moved = Just (MkStamp v (onFieldZone oldZn) moved)

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
halfReaches (TypeW t) (ObjectP ty _ _ _ _) = tyIs t ty
halfReaches PermanentW (ObjectP ty _ _ _ _) = isNothing ty
halfReaches PlayerW PlayerP = True
halfReaches PlayerW ChosenPlayerP = True
halfReaches _ _ = False

||| Whether a mention is a UNION mention -- the pair a joined
||| demonstrative reads back whole. WHICH union it is the binding's kind
||| says; this says only that there is one.
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
joinedPayload (JoinP _ _) = True

public export
wordReaches : NounWord -> Binding -> Bool
wordReaches (TypeW t) (MkBinding _ _ _ (ObjectP ty zn _ _ _)) = onFieldZone zn && tyIs t ty
wordReaches (TypeW t) (MkBinding _ _ _ PlayerP) = False
wordReaches (TypeW t) (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches (TypeW t) (MkBinding _ _ _ QualityP) = False
wordReaches (TypeW t) (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches (TypeW t) (MkBinding _ _ _ GapP) = False
wordReaches (TypeW t) (MkBinding _ _ _ LetterP) = False
wordReaches (TypeW t) (MkBinding _ _ _ TurnRefP) = False
wordReaches (TypeW t) (MkBinding _ _ _ (AbilityP _)) = False
wordReaches (TypeW t) (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches (TypeW t) pl
wordReaches CardW (MkBinding _ _ _ (ObjectP _ zn _ _ _)) = isCardZone zn
wordReaches CardW (MkBinding _ _ _ PlayerP) = False
wordReaches CardW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches CardW (MkBinding _ _ _ QualityP) = False
wordReaches CardW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CardW (MkBinding _ _ _ GapP) = False
wordReaches CardW (MkBinding _ _ _ LetterP) = False
wordReaches CardW (MkBinding _ _ _ TurnRefP) = False
wordReaches CardW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches CardW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ (ObjectP ty zn _ _ _)) =
  isCardZone zn && tyIs t ty
wordReaches (TypedCardW t) (MkBinding _ _ _ PlayerP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ QualityP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ GapP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ LetterP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ TurnRefP) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ (AbilityP _)) = False
wordReaches (TypedCardW t) (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches SpellW (MkBinding _ _ _ (ObjectP _ zn _ og _)) =
  onStackZone zn && not (isCopyOrigin og)
wordReaches SpellW (MkBinding _ _ _ PlayerP) = False
wordReaches SpellW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches SpellW (MkBinding _ _ _ QualityP) = False
wordReaches SpellW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches SpellW (MkBinding _ _ _ GapP) = False
wordReaches SpellW (MkBinding _ _ _ LetterP) = False
wordReaches SpellW (MkBinding _ _ _ TurnRefP) = False
wordReaches SpellW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches SpellW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ (ObjectP _ _ _ _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ PlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ ChosenPlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ QualityP) = False
wordReaches PlayerW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PlayerW (MkBinding _ _ _ GapP) = False
wordReaches PlayerW (MkBinding _ _ _ LetterP) = False
wordReaches PlayerW (MkBinding _ _ _ TurnRefP) = False
wordReaches PlayerW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches PlayerW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PlayerW pl
-- on the battlefield now, or where a labeled action took it off the
-- battlefield: [CR#608.2h] reads a departed referent by its last known
-- information, and [CR#110.1] is why the read is that and not the
-- current state.
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn pv _ _)) =
  onFieldZone zn || stampWasField pv
wordReaches PermanentW (MkBinding _ _ _ PlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ QualityP) = False
wordReaches PermanentW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PermanentW (MkBinding _ _ _ GapP) = False
wordReaches PermanentW (MkBinding _ _ _ LetterP) = False
wordReaches PermanentW (MkBinding _ _ _ TurnRefP) = False
wordReaches PermanentW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches PermanentW (MkBinding _ _ _ pl@(JoinP _ _)) = halfReaches PermanentW pl
wordReaches TokenW (MkBinding _ _ _ (ObjectP _ zn _ og _)) =
  onFieldZone zn && isTokenOrigin og
wordReaches TokenW (MkBinding _ _ _ PlayerP) = False
wordReaches TokenW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches TokenW (MkBinding _ _ _ QualityP) = False
wordReaches TokenW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches TokenW (MkBinding _ _ _ GapP) = False
wordReaches TokenW (MkBinding _ _ _ LetterP) = False
wordReaches TokenW (MkBinding _ _ _ TurnRefP) = False
wordReaches TokenW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches TokenW (MkBinding _ _ _ (JoinP _ _)) = False
-- The ORIGIN alone, and no zone. A copy clause is the only thing that
-- stamps `CopyOrigin`, and [CR#707.12] creates its copy "in the same
-- zone the object is in" rather than on the stack, so re-asking the
-- stack here would hide the copy-a-card verb's own mention from the word
-- that names it. A token copy carries `TokenOrigin` and is unaffected.
wordReaches CopyW (MkBinding _ _ _ (ObjectP _ zn _ og _)) = isCopyOrigin og
wordReaches CopyW (MkBinding _ _ _ PlayerP) = False
wordReaches CopyW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches CopyW (MkBinding _ _ _ QualityP) = False
wordReaches CopyW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CopyW (MkBinding _ _ _ GapP) = False
wordReaches CopyW (MkBinding _ _ _ LetterP) = False
wordReaches CopyW (MkBinding _ _ _ TurnRefP) = False
wordReaches CopyW (MkBinding _ _ _ (AbilityP _)) = False
wordReaches CopyW (MkBinding _ _ _ (JoinP _ _)) = False
-- The two ability words ask the origin alone: `AbilityP` places nothing,
-- so there is no stack to re-ask, and the copy clause is the only thing
-- that writes an origin at this kind. They part on it as `SpellW` and
-- `CopyW` part at `Object`.
wordReaches AbilityW (MkBinding _ _ _ (AbilityP og)) = not (isCopyOrigin og)
wordReaches AbilityW (MkBinding _ _ _ (ObjectP _ _ _ _ _)) = False
wordReaches AbilityW (MkBinding _ _ _ PlayerP) = False
wordReaches AbilityW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches AbilityW (MkBinding _ _ _ QualityP) = False
wordReaches AbilityW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches AbilityW (MkBinding _ _ _ GapP) = False
wordReaches AbilityW (MkBinding _ _ _ LetterP) = False
wordReaches AbilityW (MkBinding _ _ _ TurnRefP) = False
wordReaches AbilityW (MkBinding _ _ _ (JoinP _ _)) = False
wordReaches AbilityCopyW (MkBinding _ _ _ (AbilityP og)) = isCopyOrigin og
wordReaches AbilityCopyW (MkBinding _ _ _ (ObjectP _ _ _ _ _)) = False
wordReaches AbilityCopyW (MkBinding _ _ _ PlayerP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ ChosenPlayerP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ QualityP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches AbilityCopyW (MkBinding _ _ _ GapP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ LetterP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ TurnRefP) = False
wordReaches AbilityCopyW (MkBinding _ _ _ (JoinP _ _)) = False
-- The three union words ask the same question of the payload -- was the
-- mention a pair? -- and part on the antecedent's KIND, which is the
-- only thing telling "that creature or player" from "that spell or
-- ability". A bare ability mention is no pair, so none of them reaches
-- it.
-- The ORIGIN parts the last two: `AbilityJoinW` reads the union a card
-- named and `CopyJoinW` the union a copy clause made, exactly as
-- `SpellW` and `CopyW` part at `Object`. `payloadOrig` reports a join by
-- its object half, which is the half a copy clause stamps.
wordReaches JoinW (MkBinding _ kd _ pl) = joinedPayload pl && kindLte Player kd
wordReaches AbilityJoinW (MkBinding _ kd _ pl) =
  joinedPayload pl && kindLte Ability kd && not (isCopyOrigin (payloadOrig pl))
wordReaches CopyJoinW (MkBinding _ kd _ pl) =
  joinedPayload pl && kindLte Ability kd && isCopyOrigin (payloadOrig pl)

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
kindOfW (TypedCardW _) = Object
kindOfW SpellW = Object
kindOfW PlayerW = Player
kindOfW PermanentW = Object
kindOfW TokenW = Object
kindOfW CopyW = Object
kindOfW JoinW = Object \/ Player
kindOfW AbilityJoinW = Object \/ Ability
kindOfW AbilityW = Ability
kindOfW AbilityCopyW = Ability
kindOfW CopyJoinW = Object \/ Ability

public export
stampedBy : VerbLabel -> Stamp -> Bool
stampedBy v (MkStamp v' _ _) = v == v'

public export
verbedWordOk : NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
verbedWordOk (TypeW t) (MkStamp _ wasF _) ty zn = wasF && tyIs t ty
verbedWordOk CardW st ty zn = isCardZone zn
-- [CR#109.2a]'s reading, so the zone is the card word's question and
-- the type is the mention's own; the battlefield stamp `TypeW` asks
-- for is exactly what this word does not ask.
verbedWordOk (TypedCardW t) st ty zn = isCardZone zn && tyIs t ty
verbedWordOk SpellW st ty zn = onStackZone zn
verbedWordOk PlayerW st ty zn = False
verbedWordOk PermanentW (MkStamp _ wasF _) ty zn = wasF
verbedWordOk TokenW st ty zn = False
verbedWordOk CopyW st ty zn = False
verbedWordOk JoinW st ty zn = False
verbedWordOk AbilityJoinW st ty zn = False
verbedWordOk AbilityW st ty zn = False
verbedWordOk AbilityCopyW st ty zn = False
verbedWordOk CopyJoinW st ty zn = False

||| Whether a mention carries the stamp ONE named label left. A mention
||| with no stamp is one no labeled action in this text acted on, which
||| is why the absent stamp answers `False` for every label rather than
||| standing for any: the question asked is which action wrote this
||| referent, and nothing having written it is an answer to it.
public export
stampIs : VerbLabel -> Maybe Stamp -> Bool
stampIs v Nothing = False
stampIs v (Just st) = stampedBy v st

||| `itReaches` narrowed to the mentions one label stamped: the bare
||| pronoun read at the verb that made its referent what a later clause
||| names. The participle read (`TheVerbed`) asks a stamped mention two
||| questions -- which label wrote it, and whether the head word reaches
||| what the label left -- and SPELLS the answer as a participle ("the
||| destroyed creature"). This asks the first question alone and spells
||| nothing: the pronoun is still "it", and the label only says which of
||| the prefix's mentions are candidates.
|||
||| It is the other side of the clause from `itAtReaches`, which narrows
||| by the CONSUMING verb's own rule [CR#109.2]. This narrows by the
||| PRODUCING one. Neither is a preference: both are counted uniqueness
||| over a smaller candidate set, so two mentions the same label stamped
||| refuse the read exactly as two battlefield mentions refuse the
||| carrier-scoped one.
public export
itVerbedReaches : VerbLabel -> Binding -> Bool
itVerbedReaches v b = itReaches OneOf b && stampIs v (payloadProv b.payload)

||| `countOnes Object` over the candidates one label stamped.
public export
countVerbedIt : VerbLabel -> Bindings -> Nat
countVerbedIt v [] = Z
countVerbedIt v (b :: bs) =
  if itVerbedReaches v b then S (countVerbedIt v bs) else countVerbedIt v bs

public export
provOfVerbedIt : VerbLabel -> Bindings -> Maybe Stamp
provOfVerbedIt v [] = Nothing
provOfVerbedIt v (b :: bs) =
  if itVerbedReaches v b then payloadProv b.payload else provOfVerbedIt v bs

public export
zoneOfVerbedIt : VerbLabel -> Bindings -> Maybe Zone
zoneOfVerbedIt v [] = Nothing
zoneOfVerbedIt v (b :: bs) =
  if itVerbedReaches v b then bindingZone b else zoneOfVerbedIt v bs

public export
tyOfVerbedIt : VerbLabel -> Bindings -> Maybe CardType
tyOfVerbedIt v [] = Nothing
tyOfVerbedIt v (b :: bs) =
  if itVerbedReaches v b then bindingTy b else tyOfVerbedIt v bs

||| `itVerbedReaches`' PLURAL twin: the group pronoun read at the label
||| that stamped its referent. `Them` asks the prefix for its one group
||| mention; this asks for the one that a named keyword action left its
||| mark on, so a sentence naming a batch its own clause made stands
||| where a second batch was announced earlier. The narrowing is the
||| singular row's exactly -- counted uniqueness over a smaller candidate
||| set, never a preference among a larger one -- and the only difference
||| is which plurality `itReaches` is asked about.
public export
themVerbedReaches : VerbLabel -> Binding -> Bool
themVerbedReaches v b = itReaches ManyOf b && stampIs v (payloadProv b.payload)

||| `countManys Object` over the candidates one label stamped.
public export
countVerbedThem : VerbLabel -> Bindings -> Nat
countVerbedThem v [] = Z
countVerbedThem v (b :: bs) =
  if themVerbedReaches v b then S (countVerbedThem v bs) else countVerbedThem v bs

public export
provOfVerbedThem : VerbLabel -> Bindings -> Maybe Stamp
provOfVerbedThem v [] = Nothing
provOfVerbedThem v (b :: bs) =
  if themVerbedReaches v b then payloadProv b.payload else provOfVerbedThem v bs

public export
zoneOfVerbedThem : VerbLabel -> Bindings -> Maybe Zone
zoneOfVerbedThem v [] = Nothing
zoneOfVerbedThem v (b :: bs) =
  if themVerbedReaches v b then bindingZone b else zoneOfVerbedThem v bs

public export
tyOfVerbedThem : VerbLabel -> Bindings -> Maybe CardType
tyOfVerbedThem v [] = Nothing
tyOfVerbedThem v (b :: bs) =
  if themVerbedReaches v b then bindingTy b else tyOfVerbedThem v bs

||| `itReaches` narrowed to the mentions a CREATE clause MADE. Where
||| `itVerbedReaches` narrows by the label a keyword action left, this
||| narrows by the origin the create clause already wrote onto its own
||| mention: [CR#111.1] has an effect put a token onto the battlefield
||| and [CR#111.2] makes the creating player its controller, so "create
||| a Clue token. It's an artifact with …" names the thing that clause
||| made and not the permanent the sentence before it named.
|||
||| It is the object-side twin of `countTokenSpecs`, and the pair is the
||| two readings of one clause: that counter runs over the DEFINITION of
||| characteristics [CR#111.3] a create clause wrote, for "those tokens"
||| to name, where this runs over the singular OBJECT it made. The
||| plurality is why they part -- a definition is one whether it made one
||| token or six -- so this asks `itReaches OneOf` as every singular
||| pronoun does.
public export
itTokenReaches : Binding -> Bool
itTokenReaches b = itReaches OneOf b && isTokenOrigin (payloadOrig b.payload)

||| `countOnes Object` over the mentions a create clause made.
public export
countItToken : Bindings -> Nat
countItToken [] = Z
countItToken (b :: bs) =
  if itTokenReaches b then S (countItToken bs) else countItToken bs

public export
provOfItToken : Bindings -> Maybe Stamp
provOfItToken [] = Nothing
provOfItToken (b :: bs) =
  if itTokenReaches b then payloadProv b.payload else provOfItToken bs

public export
zoneOfItToken : Bindings -> Maybe Zone
zoneOfItToken [] = Nothing
zoneOfItToken (b :: bs) =
  if itTokenReaches b then bindingZone b else zoneOfItToken bs

public export
tyOfItToken : Bindings -> Maybe CardType
tyOfItToken [] = Nothing
tyOfItToken (b :: bs) =
  if itTokenReaches b then bindingTy b else tyOfItToken bs

||| What a type-naming TEST leaves on the mention it tested. [CR#608.2c]
||| has an effect's instructions followed in order, so a test the text
||| already made is knowledge every clause after it reads: "exile the top
||| card of your library. If it's a creature card, …" has established, by
||| the time the consequent is read, that the exiled card is a creature
||| card, and the mention that recorded no type when it was announced
||| ("the top card of your library" names none) is the thing that fact is
||| about.
|||
||| It writes the type and NOTHING else: determiner, kind, plurality,
||| zone and the stamp a labeled action left all stand, which is what
||| makes this a re-mark and not an announcement. It writes only where
||| the mention recorded no type. A mention that already carries one has
||| the slot filled, and a test naming a second type is telling the
||| clause something the one slot cannot hold; over-writing it would lose
||| the announced word to a tested one, which no printed line asks for.
public export
markTy : Maybe CardType -> Binding -> Binding
markTy ty (MkBinding det Object plur (ObjectP Nothing zn st og sz)) =
  MkBinding det Object plur (ObjectP ty zn st og sz)
markTy ty b = b

||| The re-mark itself: the FIRST binding the read's own test admits is
||| re-marked where it stands, and the list is otherwise the list it was.
||| Same shape as `setZone`'s folds, which re-zone the referent a clause
||| moved -- and the same shape `settleTargets` and `defineLetter` have,
||| the licensed in-place form: nothing is inserted, nothing is dropped,
||| and no binding changes position.
public export
markFirst : (Binding -> Bool) -> Maybe CardType -> Bindings -> Bindings
markFirst q ty [] = []
markFirst q ty (b :: bs) =
  if q b then markTy ty b :: bs else b :: markFirst q ty bs

||| Whether a shuffle leaves a mention readable. [CR#701.24b] keeps the
||| cards a search FOUND out of the shuffle, so a mention of one survives
||| it. Every other card in the pile is randomized where no player knows
||| its order [CR#701.24a], and a revealed one stops being revealed and
||| becomes a new object outright [CR#701.20d], so a mention of one does
||| not survive. Only a library mention is at stake: a card an earlier
||| clause moved elsewhere is not in the pile being randomized. The gate
||| reads the label's own stamp, which is why "Search" is a catalogued
||| label.
|||
||| OWNER-BLIND, recorded rather than closed. [CR#701.24b] scopes to ONE
||| library -- "shuffle THAT library ... all the cards in that library
||| except those are shuffled" -- so a mention in a different library is
||| untouched whatever this clause randomizes. The payload records a
||| mention's zone and not its owner, so the gate has nothing to match a
||| shuffled library's possessor against; an owner field on `ObjectP` is
||| what the match would cost. Measured at ZERO: no supported line
||| shuffles a library other than the one its own paragraph named, so
||| the owner-scoped reading and this one agree on every written line.
public export
survivesShuffle : Binding -> Bool
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) (Just st) _ _)) =
  stampedBy "Search" st
survivesShuffle (MkBinding _ _ _ (ObjectP _ (Just Library) Nothing _ _)) = False
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
verbedMatch v w (MkBinding _ _ OneOf (ObjectP ty zn (Just st) _ _)) = stampWordOk v w st ty zn
verbedMatch v w (MkBinding _ _ OneOf (ObjectP _ _ Nothing _ _)) = False
verbedMatch v w (MkBinding _ _ ManyOf (ObjectP _ _ _ _ _)) = False
verbedMatch v w (MkBinding _ _ _ PlayerP) = False
verbedMatch v w (MkBinding _ _ _ ChosenPlayerP) = False
verbedMatch v w (MkBinding _ _ _ QualityP) = False
verbedMatch v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatch v w (MkBinding _ _ _ GapP) = False
verbedMatch v w (MkBinding _ _ _ LetterP) = False
verbedMatch v w (MkBinding _ _ _ TurnRefP) = False
verbedMatch v w (MkBinding _ _ _ (AbilityP _)) = False
verbedMatch v w (MkBinding _ _ _ (JoinP _ _)) = False

public export
verbedMatchMany : VerbLabel -> NounWord -> Binding -> Bool
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP ty zn (Just st) _ _)) = stampWordOk v w st ty zn
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP _ _ Nothing _ _)) = False
verbedMatchMany v w (MkBinding _ _ OneOf (ObjectP _ _ _ _ _)) = False
verbedMatchMany v w (MkBinding _ _ _ PlayerP) = False
verbedMatchMany v w (MkBinding _ _ _ ChosenPlayerP) = False
verbedMatchMany v w (MkBinding _ _ _ QualityP) = False
verbedMatchMany v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatchMany v w (MkBinding _ _ _ GapP) = False
verbedMatchMany v w (MkBinding _ _ _ LetterP) = False
verbedMatchMany v w (MkBinding _ _ _ TurnRefP) = False
verbedMatchMany v w (MkBinding _ _ _ (AbilityP _)) = False
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

||| How many UNION mentions the prefix holds a half of which this word
||| names. The split read of a union ("that player or that planeswalker's
||| controller") writes one arm per half [CR#115.1], and both arms name
||| halves of the SAME mention, so what makes the pair unambiguous is
||| that one union answers to them -- not that each arm's word is unique
||| in the whole prefix. A header that already announced a player leaves
||| `countWord PlayerW` at 2 and this at 1, which is the difference
||| between refusing Heart of Bogardan's body and writing it.
public export
countUnionHalf : NounWord -> Bindings -> Nat
countUnionHalf w [] = Z
countUnionHalf w (b :: bs) =
  if isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
    then S (countUnionHalf w bs)
    else countUnionHalf w bs

public export
zoneOfUnionHalf : NounWord -> Bindings -> Maybe Zone
zoneOfUnionHalf w [] = Nothing
zoneOfUnionHalf w (b :: bs) =
  if isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
    then bindingZone b
    else zoneOfUnionHalf w bs

public export
tyOfUnionHalf : NounWord -> Bindings -> Maybe CardType
tyOfUnionHalf w [] = Nothing
tyOfUnionHalf w (b :: bs) =
  if isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
    then bindingTy b
    else tyOfUnionHalf w bs

||| "those tokens" names the characteristics definition a create clause
||| wrote [CR#111.3], not the objects; [CR#111.7] ends the objects when
||| they leave the battlefield and leaves that definition standing.
||| PLURALITY-BLIND: one token defines characteristics exactly as a batch
||| does, so a create clause that made ONE leaves a definition to read --
||| Prismari Pianist writes "create a 1/1 blue and red Elemental creature
||| token. ... create three of THOSE TOKENS instead", and Mr. House writes
||| the singular anaphor itself ("instead create THAT TOKEN and a Treasure
||| token"), one spelling of this constructor rather than a second word.
public export
countTokenSpecs : Bindings -> Nat
countTokenSpecs [] = Z
countTokenSpecs (MkBinding SelfD _ _ _ :: bs) = countTokenSpecs bs
countTokenSpecs (MkBinding _ _ _ (ObjectP _ _ _ og _) :: bs) =
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

||| The stamp readers, `zoneOf…`'s twins at the provenance field. Only a
||| mention that READS BACK a binding can report one: a stamp is written
||| onto a binding by a labeled action, so a phrase describing its
||| referent afresh names nothing any label has acted on.
public export
provOfGroup : Bindings -> Maybe Stamp
provOfGroup bs = restSource bs >>= (\b => payloadProv b.payload)

public export
provOfThat : NounWord -> Bindings -> Maybe Stamp
provOfThat w [] = Nothing
provOfThat w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => payloadProv b.payload
    _ => provOfThat w bs

public export
provOfThose : NounWord -> Bindings -> Maybe Stamp
provOfThose w [] = Nothing
provOfThose w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => payloadProv b.payload
    _ => provOfThose w bs

public export
provOfVerbed : VerbLabel -> NounWord -> Bindings -> Maybe Stamp
provOfVerbed v w [] = Nothing
provOfVerbed v w (b :: bs) =
  if verbedMatch v w b then payloadProv b.payload else provOfVerbed v w bs

public export
provOfManyVerbed : VerbLabel -> NounWord -> Bindings -> Maybe Stamp
provOfManyVerbed v w [] = Nothing
provOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then payloadProv b.payload else provOfManyVerbed v w bs

public export
provOfUnionHalf : NounWord -> Bindings -> Maybe Stamp
provOfUnionHalf w [] = Nothing
provOfUnionHalf w (b :: bs) =
  if isOne b.plur && joinedPayload b.payload && halfReaches w b.payload
    then payloadProv b.payload
    else provOfUnionHalf w bs

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

||| `tyOfThose` with the plurality demand dropped, for the reads that take
||| a DEFINITION rather than a batch of objects: `countTokenSpecs` counts
||| a token-origin binding whatever its plurality, and the head type the
||| anaphor reads back has to be found in the same place the gate counted.
public export
tyOfThoseAny : NounWord -> Bindings -> Maybe CardType
tyOfThoseAny w [] = Nothing
tyOfThoseAny w (b :: bs) =
  if wordNow w b then bindingTy b else tyOfThoseAny w bs

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

||| The named player groups a phrase may head.
||| `YourTeam` is [CR#102.4]'s term, and the rule is what makes it a
||| group VALUE rather than a spelling of `You`: it is "shorthand for
||| 'you and/or your teammates'", which is more than one player whenever
||| the game has teams [CR#102.3], and collapses to "you" only in a game
||| that is not a multiplayer game between teams. A word whose denotation
||| the rules give as a set of players belongs beside `AllPlayers` and
||| `YourOpponents`; a spelling note on `You` would say the wrong thing
||| in exactly the games the word exists for. 15 supported lines write
||| "your team" (measured 2026-08-28).
public export
data PlayerGroupWord = AllPlayers | YourOpponents | YourTeam

public export
Eq PlayerGroupWord where
  (==) AllPlayers AllPlayers = True
  (==) AllPlayers _ = False
  (==) YourOpponents YourOpponents = True
  (==) YourOpponents _ = False
  (==) YourTeam YourTeam = True
  (==) YourTeam _ = False

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
  ||| `Triggering` is neither a stretch nor a cause: it names the very
  ||| OCCURRENCE the ability triggered on, which is what "if it entered
  ||| from your library" (Fblthp, the Lost) asks about. [CR#603.2]
  ||| triggers the ability on an event matching its condition and
  ||| [CR#603.2c] has it trigger "only once each time its trigger event
  ||| occurs", so one instance of the ability answers to exactly one
  ||| occurrence, and a clause in its text that names its own trigger
  ||| event names that one. Written in a text with no trigger event to
  ||| name, it is tolerated overgeneration on `ThisWay`'s own terms.
  ||| -- spelling: "this way" in the window's place, "dealt damage this
  ||| way", "destroyed this way"; `Triggering` writes nothing at all.
  public export
  data Lookback = ThisTurn | ThisCombat | LastTurn | ThisGame | ThisWay
                | Triggering

  ||| Structural agreement of two optional windows, for the reads that
  ||| compare descriptions. No `Eq Lookback`: nothing else asks.
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
  ||| "Counter target activated ability": [CR#115.2] admits as a target
  ||| an object that can't exist on the battlefield, "such as a spell or
  ||| ability", and [CR#109.1] makes an ability on the stack such an
  ||| object. The kind is what this row opens; WHICH abilities may be
  ||| targeted is the head's class, since [CR#115.1c,115.1d] give the
  ||| word to activated and triggered abilities alone. A head naming a
  ||| class outside that pair is tolerated overgeneration -- no printed
  ||| line writes one, so there is nothing here to refuse.
  AbilityTgt : Targetable Ability
  ||| [CR#115.1] lets a spell or ability target objects and players
  ||| alike, and [CR#115.2] adds the stack's own objects beside them, so
  ||| a phrase that may denote either half is targetable exactly when
  ||| both halves are.
  JoinTgt : Targetable a -> Targetable b -> Targetable (a \/ b)

||| The other side of the same relation: which kind of phrase HAS
||| targets. [CR#115.1a], [CR#115.1c] and [CR#115.1d] give the word
||| "target" to an instant or sorcery spell, to an activated ability and
||| to a triggered ability, and to nothing else; a player never targets,
||| since [CR#115.1] has a player CHOOSE the targets of the spell or
||| ability, which is what has them. The joined arm is [CR#115.9b]'s own
||| phrase, "[spell or ability] that targets [something]", and
||| [CR#702.21a] writes the same pair into ward.
public export
data Targeter : Kind -> Type where
  SpellTargets : Targeter Object
  AbilityTargets : Targeter Ability
  EitherTargets : Targeter a -> Targeter b -> Targeter (a \/ b)

||| How much of a targeter's targeting a description reads. [CR#115.9b]
||| checks the current state of the targets a phrase names; [CR#115.9c]
||| states the "only" reading as its own check -- how many DIFFERENT
||| things were chosen as targets, which must be one. Two readings of one
||| relation, so an extent slot and not a second row.
||| -- spelling: `SomeTarget`, "that targets [m]"; `SoleTarget`, "that
||| targets only [m]".
public export
data TargetExtent = SomeTarget | SoleTarget

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
targetablePhrasal AbilityTgt = PhAbility
targetablePhrasal (JoinTgt l r) =
  PhJoin (targetablePhrasal l) (targetablePhrasal r)

||| What a copy clause announces, shaped by the kind it copied.
||| [CR#707.10] puts the copy on the stack whatever it copied, so the
||| object arm records the stack; the ability arm records no zone because
||| the ability kind places nothing, and the ORIGIN is what both arms
||| carry and what the copy words read. A union is copied half by half,
||| since a copy of "that spell or ability" is a copy of whichever arm
||| fired and the mention has to keep saying so.
||| The type is the copied phrase's own -- [CR#707.2] gives the copy the
||| original's copiable values, so a copy of a creature spell is one too.
||| The `PhPlayer` and `PhQuality` rows are unreachable under `Copiable`
||| and are filled rather than left to a catch-all.
public export
copyPayload : {k : Kind} -> Phrasal k -> Maybe CardType -> Payload k
copyPayload PhObject ty = ObjectP ty (Just Stack) Nothing (Just CopyOrigin) Nothing
copyPayload PhAbility _ = AbilityP (Just CopyOrigin)
copyPayload (PhJoin l r) ty = JoinP (copyPayload l ty) (copyPayload r ty)
copyPayload PhPlayer _ = PlayerP
copyPayload {k = Quality q} PhQuality _ = QualityP


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

||| The shape of the parameter a keyword's own rule writes after the
||| word. `AbilityParam` is the one that names no value: [CR#702.142a],
||| [CR#702.177a] and [CR#702.193a] each read "a keyword that adds
||| additional rules to the activated ability that follows it", so what
||| follows the word is an ABILITY and not a cost, a number or a
||| quality. No `KeywordParam` produces that shape, which is what keeps
||| such a word from ever being written as a keyword LINE -- none of the
||| three ever stands alone -- while still letting `AbilityClass`'
||| `KeywordClass` name it.
public export
data KeywordParamShape = NoParam | CostParam | QualityParam | SubjectParam
                       | NumberParam | AbilityParam

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
  (==) AbilityParam AbilityParam = True
  (==) AbilityParam _ = False

||| Which KINDS a quality parameter may describe. [CR#702.16a] writes
||| the quality as "any characteristic value or information", which
||| describes an OBJECT, and that is what affinity's [CR#702.41a] "[text]"
||| and partner's [CR#702.124j] "[name]" are too. [CR#702.16k] is the
||| second kind: "Protection from [a player]" is a variant whose slot
||| names a PLAYER outright, and [CR#109.3] makes a player no
||| characteristic of anything, so the phrase names its referent rather
||| than matching one -- the same reason `ChosenPlayer` is a head noun
||| where `OfChosen` is a description.
|||
||| TWO kinds and not a join: [CR#702.16g] makes "protection from
||| [quality A] and from [quality B]" shorthand for two separate
||| abilities, so no printed slot names an object and a player at once
||| and there is no union site here to mark. Nothing writes a keyword
||| parameter describing a quality, an outcome, a turn or an ability.
|||
||| The two cards this does NOT unblock, so the next round does not
||| re-derive them. SERRA'S EMISSARY writes "protection from the chosen
||| card type", and the payload was never the blocker -- `CardTypeQ`
||| [CR#205.2a] landed with the card-type chooser and `OfChosen` reads
||| it -- its subject is "You and creatures you control", a mixed group
||| this grammar has no term for. RUNED HALO is the same blocker at the
||| other end: its parameter writes (the chosen card name, [CR#702.16a]'s
||| name-as-quality), and what refuses the line is that its SUBJECT is a
||| player where `Gains` takes an object. One line each.
public export
qualityParamKind : Kind -> Bool
qualityParamKind Object = True
qualityParamKind Player = True
qualityParamKind _ = False

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
||| WORDS DELIBERATELY ABSENT, and why -- so the next round does not
||| re-derive the reasons:
|||
||| * SIX words the layer below cannot resolve -- cascade, replicate,
|||   conspire, rebound, demonstrate and sticker kicker. No macro under
|||   `plugins/builtin/macros/keyword/` and no built-card use, so a row
|||   would name a word nothing beneath this grammar has anything for;
|||   The First Sliver, Cast Through Time and Flamekin Herald are
|||   unbuildable whatever a row here said. This was NINE: emerge left
|||   the list with a row above, bought by a describing read
|||   (`HasKeyword "Emerge"`, Foul Emissary) rather than by a keyword
|||   line, which is a use the count had not seen. PROWL and
|||   FREERUNNING left it the same way -- the READBACK channel is their
|||   consumer ("if its prowl cost was paid", 4 lines; "if its
|||   freerunning cost was paid", 1), and `PaidCost` reaches a word
|||   through `keywordCosts` and not through a macro below.
||| * SUSPEND, whose parameter is COMPOUND. [CR#702.62a] writes
|||   "Suspend N--[cost]" and expands it to an exile "with N time
|||   counters on it" after paying [cost]: a counter count beside a cost,
|||   and a counter count is no component of one. `KeywordParamShape` has
|||   no compound arm, and minting one is the same decision the
|||   restricted equip line's "[quality] [cost]" [CR#702.6c] asks -- one
|||   decision for both, taken with a witness or not at all.
||| * STATION. [CR#702.184a]'s "Station" takes no parameter and would be
|||   a bare row, but every printed station card also writes the station
|||   SYMBOLS [CR#702.184b] -- themselves keyword abilities, on a
|||   nonstandard layout -- so no card comes whole with the word alone,
|||   and a row with no consumer is what the open-catalog ruling refuses.
|||   The symbol ladder is this grammar's `ChapterMark` question asked
|||   again, and it is sub-machinery, not a catalog row.
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
  -- [CR#702.3a] makes defender a static ability with no slot after the
  -- word, and [CR#122.1b] does not name it among the keywords a keyword
  -- counter can be. Only a creature has it [CR#702.3b], so it prints on
  -- a permanent card alone.
  , MkKeywordFacts "Defender"         NoParam      False Nothing             True  False False
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
  -- [CR#702.24a] writes the word as "Cumulative upkeep [cost]", so the
  -- parameter is a cost and never absent. What the cost may BE is
  -- [CR#118.1]'s question and `costActionOk` answers it for every
  -- carrier at once: the four cards whose upkeep is an action rather
  -- than mana (Braid of Fire, Psychic Vortex, Varchild's War-Riders,
  -- Wall of Shards) bench with nothing minted, and the four that write
  -- "{a} or {b}" (Arctic Nishoba, Earthen Goo, Jotun Owl Keeper,
  -- Krovikan Whispers) spend `EitherCost`.
  -- 80 supported cards print the line and 5 grant it in a quotation
  -- (re-measured 2026-08-28). SIX CARRIERS STAY BLOCKED, each on its
  -- own gap and none of them this row's: Karplusan Minotaur (no
  -- coin-flip verb), Herald of Leshrac (a one-shot control change where
  -- `GainsControl` is a static), Jotun Grunt ("a single graveyard", a
  -- uniqueness phrase over a zone's possessor), Balduvian Shaman (a
  -- permanent described by NOT having a keyword, where `HasKeyword`
  -- demands `KeywordParamless`), Phyrexian Soulgorger (the `Phyrexian`
  -- name-straddle in its type line) and Cover of Winter (a prevention
  -- size read off the age tally).
  -- OUT OF SCOPE and recorded so it is not folded in: the longhand
  -- mirror without the keyword -- Cyclone and Phantasmal Sphere write
  -- the whole escalating procedure out, and Myr Prototype, Primordial
  -- Ooze and Rogue Skycaptain share the escalation while swapping the
  -- consequence.
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
  -- The three ability-marker words, each naming a CLASS of activated
  -- ability rather than an ability the object has: [CR#702.142a],
  -- [CR#702.177a] and [CR#702.193a] all read "a keyword that adds
  -- additional rules to the activated ability that follows it". They
  -- earn their rows from the class-subject lines -- "Boast abilities you
  -- activate cost {1} less to activate", "Exhaust abilities of other
  -- permanents you control cost {2} less to activate", "Power-up
  -- abilities of other creatures you control cost {3} less to activate"
  -- and Kang the Conqueror's "power-up abilities can't be activated" --
  -- which name the word where `KeywordClass` reads it. Cycling and
  -- ninjutsu are the same cell at words that already had rows.
  -- [CR#702.135a] makes afterlife a triggered ability with a number
  -- after the word: "Afterlife N" means "When this permanent is put
  -- into a graveyard from the battlefield, create N 1/1 white and black
  -- Spirit creature tokens with flying."
  , MkKeywordFacts "Afterlife"        NumberParam  False Nothing             True  False False
  , MkKeywordFacts "Boast"            AbilityParam False Nothing             True  False False
  , MkKeywordFacts "Exhaust"          AbilityParam False Nothing             True  False False
  , MkKeywordFacts "PowerUp"          AbilityParam False Nothing             True  False False
  -- The two parameterised words the slot was minted for and had no
  -- consumer at. [CR#702.41a] makes affinity "a static ability that
  -- functions while the spell with affinity is on the stack" and writes
  -- "Affinity for [text]", where [text] describes the permanents the
  -- reduction counts -- a described CLASS, which is the quality payload
  -- and not a number. The rule names no card type, so any castable card
  -- may print it, and a card that stays in the command zone is never a
  -- spell.
  -- [CR#702.86a] writes "Annihilator N" and expands it to "Whenever
  -- this creature attacks, defending player sacrifices N permanents", a
  -- triggered ability of a creature: a number after the word, no stack
  -- question, and a permanent card alone.
  , MkKeywordFacts "Affinity"         QualityParam False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Annihilator"      NumberParam  False Nothing             True  False False
  -- The two paramless evasion/protection words the keyword-CLASS
  -- carriers buy. [CR#702.36a,702.36b]: fear is an evasion ability and
  -- "a creature with fear can't be blocked except by artifact creatures
  -- and/or black creatures". [CR#702.18a]: "Shroud" means "This
  -- permanent or player can't be the target of spells or abilities".
  -- Neither is named by [CR#122.1b]'s keyword-counter list, and each
  -- rule speaks of a permanent.
  , MkKeywordFacts "Fear"             NoParam      False Nothing             True  False False
  , MkKeywordFacts "Shroud"           NoParam      False Nothing             True  False False
  -- [CR#702.14a] makes landwalk "a generic term that appears within an
  -- object's rules text as '[type]walk'", and [CR#702.14c] reads the
  -- ability off "the specified land type". So the LAND TYPE is the
  -- word's parameter and "islandwalk" is the spelling that writes the
  -- two as one word -- the same relationship every other row has to its
  -- printed form, and the reading core's own `Landwalk` macro takes
  -- (a quality parameter defaulting to Land). [CR#702.14b,702.14c] make
  -- it a blocking restriction on a creature.
  -- The row buys the CLASS term alone. An individual landwalk still does
  -- not write: the parameter is a land type, and the land-subtype
  -- quality sort is not minted. Magnigoth Treefolk stays doubly blocked
  -- past it -- "Domain — For each basic land type among lands you
  -- control, this creature has landwalk of that type" also wants the
  -- distributive over a counted type axis.
  , MkKeywordFacts "Landwalk"         QualityParam False Nothing             True  False False
  -- [CR#702.73a]: "Changeling" means "This object is every creature
  -- type", a characteristic-defining ability that "works everywhere,
  -- even outside the game" [CR#604.3]. The rule names no bearer and no
  -- zone, so all three card classes stand open -- the corpus writes it
  -- on permanents and on Kindred instants and sorceries alike, and no
  -- rule refuses the command zone the way [CR#702.34a] refuses a
  -- permanent card flashback.
  , MkKeywordFacts "Changeling"       NoParam      False Nothing             True  True  True
  -- The two tap-a-creature activated abilities, one number apiece:
  -- [CR#702.122a] "Crew N" and [CR#702.171a] "Saddle N" each mean "Tap
  -- any number of other untapped creatures you control with total power
  -- N or greater: ...". Both act on a permanent already on the
  -- battlefield, so no stack question arises.
  , MkKeywordFacts "Crew"             NumberParam  False Nothing             True  False False
  , MkKeywordFacts "Saddle"           NumberParam  False Nothing             True  False False
  -- [CR#702.124j]: "Partner with [name]" -- the slot is a card NAME.
  -- [CR#109.3] lists name among an object's characteristics and
  -- [CR#702.16a] takes a quality to be "any characteristic value or
  -- information", so a name is a quality and the payload is `Named`'s
  -- predicate. [CR#702.124a] has partner abilities "function
  -- before the game begins", which is no stack regime; the second half
  -- of [CR#702.124j] is a triggered ability of a permanent.
  , MkKeywordFacts "PartnerWith"      QualityParam False Nothing             True  False False
  -- Two words whose rule writes ONE cost after them, however many
  -- printed slots the notation splits it into.
  -- [CR#702.119a]: "Emerge [cost]" means "You may cast this spell by
  -- paying [cost] and sacrificing a creature rather than paying its mana
  -- cost" -- an alternative cost, so the ability acts while the spell is
  -- being cast, and the rule restricts the word to no card type.
  -- [CR#702.167a]: "Craft with [materials] [cost]" means "[Cost], Exile
  -- this permanent, Exile [materials] ...: Return this card to the
  -- battlefield transformed". The two printed slots are two COMPONENTS
  -- of the one activation cost the rule writes out [CR#118.1], not two
  -- parameters of different sorts, which is why craft needs no compound
  -- shape and suspend does: [CR#702.62a]'s "Suspend N--[cost]" writes a
  -- number of TIME COUNTERS beside its cost, and a counter count is no
  -- cost component. Suspend therefore has no row here; see the note on
  -- `keywordFacts` above.
  , MkKeywordFacts "Emerge"           CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Craft"            CostParam    False Nothing             True  False False
  -- THE ALTERNATIVE-COST WORDS, seven rows, each a cost the card offers
  -- INSTEAD of its mana cost [CR#118.9] and each bought by the READBACK
  -- channel: `PaidCost (ByKeyword w)` reaches a word through
  -- `keywordCosts`, so "if its [w] cost was paid" is a consumer no macro
  -- below has to supply. Re-measured 2026-08-28 over supported cards
  -- with reminder text stripped -- keyword lines / readback lines:
  -- Madness 61/4, Sneak 28/4, Mayhem 15/1, Freerunning 12/1, Prowl 10/4,
  -- Surge 11/3, Spectacle 11/2.
  --
  -- FOUR of them share ONE template, written here once and not four
  -- times: [CR#702.76a] (prowl), [CR#702.117a] (surge), [CR#702.137a]
  -- (spectacle) and [CR#702.173a] (freerunning) are each "a static
  -- ability that functions [while the spell is] on the stack" reading
  -- "You may pay [cost] rather than pay this spell's mana cost if
  -- [something happened this turn]". Only the condition differs --
  -- combat damage from a source sharing the spell's creature types
  -- (prowl), you or a teammate cast another spell (surge), an opponent
  -- lost life (spectacle), combat damage from an Assassin or a commander
  -- you control (freerunning) -- and all four route the cast through the
  -- alternative-cost rules [CR#601.2b,601.2f..601.2h]. So all four take
  -- `AtCasting`: the ability acts while the spell is being cast.
  --
  -- The other three each carry their own ZONE, which is why they are not
  -- the same template at a fifth condition. [CR#702.35a] makes madness a
  -- static ability in the HAND plus a triggered one, and the cast is from
  -- EXILE; [CR#702.187a,702.187b] make mayhem a static ability in the
  -- GRAVEYARD, casting from there "as long as you discarded this card
  -- this turn". Neither functions on the stack, so neither has a stack
  -- regime. [CR#702.190a] makes sneak a static ability on the stack like
  -- the first four, but its meaning is a TIMING permission plus a second
  -- cost component -- "Any time you could cast an instant during your
  -- declare blockers step, you may cast this spell by paying [cost] and
  -- returning an unblocked creature you control to its owner's hand" --
  -- and [CR#702.190b] adds an entry rider, so it templates alone.
  --
  -- No rule of the seven names a card type, and the corpus writes each on
  -- permanent and on spell cards alike; none is in [CR#122.1b]'s
  -- keyword-counter list; and none functions from the command zone.
  --
  -- WHAT A ROW DOES NOT BUY: the word's REMINDER TEXT. 1005 supported
  -- lines carry a parenthetical explaining an alternative-cost cast
  -- ("(You may cast this spell for its [w] cost ...)"), spread over 50
  -- leading words -- Flashback 202, Morph 150, Suspend 55, Bestow 41,
  -- Disguise 38, Cascade 35, Mutate 34 down to Impending 5 -- and ZERO
  -- of them survive reminder-stripping. Reminder text is not a card's
  -- own line, so none of that population is bench-payable and no row
  -- here is owed to it; what earns a row is the keyword LINE and the
  -- readback, both measured above. (Re-measured 2026-08-28; the
  -- umbrella's "956 lines / 25 keywords" was the same population under
  -- a narrower pattern.)
  --
  -- TWENTY more alternative-cost words print a keyword line and write
  -- NO readback anywhere in the corpus: escape, foretell, bestow,
  -- disguise, mutate, overload, disturb, dash, evoke, blitz, cleave,
  -- harmonize, impending, awaken, buyback, casualty, squad, offspring,
  -- gift and replicate. `PaidCost` names none of them, so their rows
  -- wait on a keyword-LINE consumer rather than on this channel. The
  -- umbrella listed them as measured readback payers; the re-measure
  -- says zero, and the correction is recorded here rather than
  -- re-derived.
  --
  -- TWO CARRIED CORRECTIONS, settled here so they are not re-bought.
  -- FALL OF THE TITANS was recorded as unreachable because its "Surge
  -- {X}{R}" line was taken for reminder text shared with ten siblings.
  -- The reminder was never the blocker: the keyword LINE is the card's
  -- own line, the parenthetical after it is the reminder, and with the
  -- Surge row the whole card writes. And the shape
  -- "If you've cast another spell this turn, you may pay {1}{U} rather
  -- than pay this spell's mana cost" is ZERO supported lines -- the
  -- condition is attested only inside surge's own rule [CR#702.117a] and
  -- in cost-reduction lines ("This spell costs {2} less to cast if
  -- you've cast another spell this turn", 4 of the 9 lines that write
  -- the condition at all). A RECORDED zero and not a pinned one:
  -- [CR#118.9] defines an alternative cost as one "listed in a spell's
  -- text ... that its controller may pay rather than paying the spell's
  -- mana cost" and puts no bound on what a card may condition one on,
  -- so nothing refuses the sentence and the corpus simply does not
  -- write it.
  , MkKeywordFacts "Madness"          CostParam    False Nothing             True  True  False
  , MkKeywordFacts "Prowl"            CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Surge"            CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Spectacle"        CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Freerunning"      CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Sneak"            CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Mayhem"           CostParam    False Nothing             True  True  False
  -- The two MODAL cost words, which are additional costs rather than
  -- alternative ones and earn their rows from the keyword LINE alone: 32
  -- supported entwine lines and 9 escalate lines, and ZERO readbacks
  -- anywhere in the corpus for either -- no card asks whether an entwine
  -- or escalate cost was paid, so `PaidCost` never names them and that
  -- zero is the measurement, not a gap.
  -- [CR#702.42a]: "Entwine [cost]" means "You may choose all modes of
  -- this spell instead of just the number specified. If you do, you pay
  -- an additional [cost]", a static ability of modal spells that
  -- functions while the spell is on the stack. [CR#702.120a]: "Escalate
  -- [cost]" means "For each mode you choose beyond the first as you cast
  -- this spell, you pay an additional [cost]", the same shape at a
  -- per-mode multiplier.
  -- WHAT THE ROWS DO NOT BUY: the linkage to the modal clause. Both
  -- rules speak of "modal spells" [CR#700.2] and nothing here checks
  -- that the card writes a `Modal`; a card carrying the word with no
  -- modes over-generates, recorded rather than gated, and the per-mode
  -- multiplier escalate writes has no term at all.
  , MkKeywordFacts "Entwine"          CostParam    False (Just AtCasting)    True  True  False
  , MkKeywordFacts "Escalate"         CostParam    False (Just AtCasting)    True  True  False
  -- [CR#702.102a]: "Fuse is a static ability found on some split cards
  -- ... that applies while the card with fuse is in a player's hand. If
  -- a player casts a split card with fuse from their hand, the player
  -- may choose to cast both halves of that split card rather than choose
  -- one half." The word takes no parameter -- the total cost is the two
  -- halves' own mana costs [CR#702.102c] and no slot is written after it
  -- -- so `NoParam`, and the ability acts as the spell is cast, which is
  -- `AtCasting`.
  -- 17 supported cards print the word, one keyword line on EACH half
  -- (measured 2026-08-28), and every one of the 34 halves is an instant
  -- or a sorcery: 18 sorcery halves and 16 instant halves, no permanent
  -- half among them. So the permanent cell is a measured zero and not a
  -- refusal -- [CR#709.5] admits permanent split cards and nothing in
  -- [CR#702.102] forbids one carrying fuse; the corpus has not printed
  -- it.
  -- WHAT THE ROW DOES NOT BUY: the fused spell itself. [CR#702.102b] and
  -- [CR#702.102d] give the resulting spell the combined characteristics
  -- of both halves and resolve the left half's instructions then the
  -- right's, which is a spell the grammar has no term for -- a card
  -- writes the WORD, and the two halves it fuses are `SplitCard`'s
  -- already.
  , MkKeywordFacts "Fuse"             NoParam      False (Just AtCasting)    False True  False
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

||| A CLASS of keyword abilities, named as ONE term where a
||| `KeywordLabel` names one ability: "protection", "landwalk",
||| "protection from any color". A quantifier over words, and the three
||| printed spellings are one shape.
|||
||| The quantification runs over the word's PARAMETER, and the rules are
||| what put it there. [CR#702.16a] writes every protection ability as
||| "Protection from [quality]", so a card that says "protection" with
||| nothing after it has named the word and left the quality open --
||| every protection ability, not one of them. [CR#702.14a] says the
||| same one level down: landwalk "is a generic term that appears within
||| an object's rules text as '[type]walk'", which is that word with its
||| land type left open. A word whose rule writes no parameter
||| quantifies over nothing and is no class; it is just the word.
|||
||| `familySort` NARROWS the open parameter to one sort instead of
||| leaving it open: "protection from any color" ranges over
||| [CR#105.1]'s colors and not over [CR#702.16a]'s whole quality range
||| ("any characteristic value or information"). Only a quality
||| parameter has sorts to narrow.
public export
record KeywordFamily where
  constructor MkKeywordFamily
  ||| the word whose parameter the term quantifies
  familyWord : KeywordLabel
  ||| the sort quantified over, or the word's whole parameter range
  familySort : Maybe QualitySort

public export
Eq KeywordFamily where
  (==) a b = familyWord a == familyWord b && familySort a == familySort b

||| Known word, parameterised word, and a sort named only where the
||| parameter is a quality. Fail-closed on an unknown word for
||| `knownKeyword`'s reason.
public export
keywordFamilyOk : KeywordFamily -> Bool
keywordFamilyOk c = case keywordFactsFor (familyWord c) of
  Nothing => False
  Just f => case familySort c of
    Nothing => not (paramShape f == NoParam)
    Just _ => paramShape f == QualityParam

public export
KeywordFamilyOk : KeywordFamily -> Type
KeywordFamilyOk c = So (keywordFamilyOk c)

||| What a position naming a keyword may name: one word, or a class of
||| them. ONE type for both seats that ask the question -- the read
||| ("a creature card with flying", "a creature card with protection")
||| and the keyword-list extension's element -- because the seven cards
||| that write a class write it in exactly those two places, and a term
||| admitted at one and refused at the other would say the trailer means
||| something the base sentence cannot.
public export
data KeywordTerm : Type where
  TheKeyword : (k : KeywordLabel) -> KeywordTerm
  AnyKeywordIn : (c : KeywordFamily) -> KeywordTerm

public export
Eq KeywordTerm where
  (==) (TheKeyword a) (TheKeyword b) = a == b
  (==) (TheKeyword _) _ = False
  (==) (AnyKeywordIn a) (AnyKeywordIn b) = a == b
  (==) (AnyKeywordIn _) _ = False

||| The membership gate at the term, fail-closed on both arms.
public export
knownKeywordTerm : KeywordTerm -> Bool
knownKeywordTerm (TheKeyword k) = knownKeyword k
knownKeywordTerm (AnyKeywordIn c) = keywordFamilyOk c

public export
KnownKeywordTerm : KeywordTerm -> Type
KnownKeywordTerm t = So (knownKeywordTerm t)

||| Whether a term may stand where each element is written BARE, which
||| is the keyword list's demand. The word arm's demand is unchanged --
||| `keywordParamless`, so "ward" is refused there exactly as it was --
||| and the class arm is admitted by its own rule rather than by
||| weakening that one: a class term is written bare because it names no
||| value, not because its word takes none.
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
||| The un-keyworded ADDITIONAL cost [CR#118.8] is the fourth arm for the
||| same reason at the other kind of cost: 308 supported lines write "As
||| an additional cost to cast this spell, ..." with no word to name it,
||| and 12 read it back as "if this spell's additional cost was paid".
||| It sorts beside `TheAlternative` and not under it -- [CR#118.8d]
||| makes an additional cost something the controller pays ON TOP of the
||| mana cost, where [CR#118.9] replaces it -- and a card writing both
||| gives the two reads two different answers.
public export
data PaidCostName : Type where
  ByKeyword : (kw : KeywordLabel) -> PaidCostName
  ByNthKeyword : (ord : Ordinal) -> (kw : KeywordLabel) -> PaidCostName
  TheAlternative : PaidCostName
  TheAdditional : PaidCostName

public export
Eq PaidCostName where
  (==) (ByKeyword a) (ByKeyword b) = a == b
  (==) (ByKeyword _) _ = False
  (==) (ByNthKeyword (Nth m) a) (ByNthKeyword (Nth n) b) = m == n && a == b
  (==) (ByNthKeyword _ _) _ = False
  (==) TheAlternative TheAlternative = True
  (==) TheAlternative _ = False
  (==) TheAdditional TheAdditional = True
  (==) TheAdditional _ = False

||| A named cost has to be one there is: a keyword arm needs a word whose
||| rule writes a cost, and the written alternative needs nothing.
public export
paidCostNamed : PaidCostName -> Bool
paidCostNamed (ByKeyword kw) = keywordCosts kw
paidCostNamed (ByNthKeyword _ kw) = keywordCosts kw
paidCostNamed TheAlternative = True
paidCostNamed TheAdditional = True

public export
PaidCostNamed : PaidCostName -> Type
PaidCostNamed n = So (paidCostNamed n)

public export
data AbilityClass : Type where
  ||| The bare word "ability", as "target spell or ability" and "counter
  ||| that spell or ability" write it. What a card names there is the
  ||| glossary's SECOND sense of the word -- an activated or triggered
  ||| ability on the stack, which is an object in its own right
  ||| [CR#109.1] -- and three rules close that set from three sides: only
  ||| activated and triggered abilities use the stack
  ||| [CR#113.3b,113.3c], only they can be countered [CR#113.9], and only
  ||| they are given the word "target" [CR#115.1c,115.1d]. A spell
  ||| ability is an instruction followed while its spell resolves
  ||| [CR#113.3a] and a static ability is simply true [CR#113.3d]; no
  ||| card can name either as a thing. So the arm is not "any ability
  ||| whatever" but the pair the stack holds, whose two halves are
  ||| `AnyActivated` and `AnyTriggered`.
  ||| -- spelling: "ability".
  AnyOnStack : AbilityClass
  AnyActivated : AbilityClass
  ||| "target triggered ability", "activated or triggered ability": the
  ||| other half of that pair, named on its own. [CR#113.3c] puts a
  ||| triggered ability on the stack, [CR#113.9] lets an effect that
  ||| counters abilities counter it there, and [CR#115.1d] gives it the
  ||| word "target" -- the same three rules that open `AnyActivated`,
  ||| said of the other kind. A row and not a narrowing written on
  ||| `AnyOnStack`, because the corpus writes the two halves side by side
  ||| inside one phrase ("activated or triggered ability") and a
  ||| disjunction needs a word for each disjunct.
  ||| -- spelling: "triggered ability".
  AnyTriggered : AbilityClass
  LoyaltyClass : AbilityClass
  KeywordClass : (k : KeywordLabel) -> {auto 0 kn : KnownKeyword k} ->
                 AbilityClass

public export
Eq AbilityClass where
  (==) AnyOnStack AnyOnStack = True
  (==) AnyOnStack _ = False
  (==) AnyActivated AnyActivated = True
  (==) AnyActivated _ = False
  (==) AnyTriggered AnyTriggered = True
  (==) AnyTriggered _ = False
  (==) LoyaltyClass LoyaltyClass = True
  (==) LoyaltyClass _ = False
  (==) (KeywordClass a) (KeywordClass b) = a == b
  (==) (KeywordClass _) _ = False

||| Whether a LIST of ability classes names each class once. A described
||| set of abilities is written as a coordination of class words ("all
||| activated and triggered abilities of the last chosen card", Koh, the
||| Face Stealer), and a word written twice describes the same abilities
||| twice.
public export
distinctClasses : List AbilityClass -> Bool
distinctClasses [] = True
distinctClasses (c :: cs) = not (elem c cs) && distinctClasses cs


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

||| [CR#207.2d]: a flavor word appears in italics at the beginning of some
||| abilities, exactly where an ability word does, and likewise has no
||| special rules meaning. The rule states the one difference that matters
||| to the vocabulary: "while an ability word ties together several
||| abilities with similar functionality, each flavor word is tailored to
||| the specific ability it appears with". So [CR#207.2c]'s list is closed
||| and enumerable and this one cannot be: 441 distinct words head 446
||| supported lines over 398 supported cards (measured 2026-09-02), one
||| word per line in all but a handful. It is an OPEN LABEL for the same
||| reason a subtype is.
public export
FlavorWordLabel : Type
FlavorWordLabel = String

||| The italicized word a printed ability may begin with [CR#207.2]. ONE
||| vocabulary and not two rows: [CR#207.2c] and [CR#207.2d] give the two
||| kinds the same position and the same (absent) rules meaning, and no
||| reader in this grammar asks which kind it got — the word is carried so
||| that the spelling layer can write it back, and read by nothing else.
||| The difference the two rules do state is which vocabulary the word
||| comes from, and that is exactly what these two arms carry.
public export
data ItalicWord = AnAbilityWord AbilityWordName
                | AFlavorWord FlavorWordLabel


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

||| The letters a printed mana cost brings into scope for the text that
||| shares the object with it. [CR#107.3a] has the caster announce the
||| value of X as the spell is cast and fixes any X in the spell's mana
||| cost at that value while it is on the stack; [CR#107.3i] then makes
||| every instance of X on the object share it. So a card whose cost
||| writes the variable symbol hands its text a letter already bound,
||| which is precisely what Prosperity's "X" reads.
|||
||| ONE letter, not a set: [CR#107.3p] gives Y the same rules as X, but no
||| printed mana cost writes it -- `ManaSymbol`'s variable arm is
||| unlettered, and a second variable in one cost has no symbol to be
||| spelled with.
|||
||| The scope is the object's, and its two [CR#107.3] exceptions are NOT
||| this function's to state. [CR#107.3k] makes an activated ability's own
||| activation-cost X independent of the object's, so a card whose text
||| activates for {X} must not read this letter for that ability's cost;
||| [CR#107.3j] does the same for a gained ability. Both are facts about
||| where an ability's cost gets its value, which is the ability's own
||| telescope and not the face's.
||| What a play permission says about paying for the cast it licenses.
||| [CR#118.9] lets an effect license a cast "without paying its mana
||| cost", which is an alternative cost of nothing, and 296 supported
||| lines write that of a card the clause has NAMED -- Omniscience,
||| Aetherworks Marvel, Memory Plunder -- as against the 16 that write it
||| of the spell being cast, which is `AltCost Nothing`'s row and not
||| this one. The two are different sentences: [CR#118.9]'s self line
||| modifies what THIS object costs, where the permission prices a
||| different or later card the licence has picked out.
|||
||| Two arms, and the third the corpus wants is recorded rather than
||| minted: 23 supported lines write a play permission with a WRITTEN
||| alternative cost after it ("by paying [c] ... rather than paying its
||| mana cost", Worldheart Phoenix). That is a `Cost`-carrying arm here,
||| and it waits on its own round.
||| -- spelling: nothing at `ItsOwnCost`; ", without paying its mana
||| cost" / "... their mana costs" after the permission otherwise.
public export
data PlayPayment = ItsOwnCost | WithoutPaying

public export
Eq PlayPayment where
  (==) ItsOwnCost ItsOwnCost = True
  (==) ItsOwnCost _ = False
  (==) WithoutPaying WithoutPaying = True
  (==) WithoutPaying _ = False

||| How many times ONE offer to pay a cost may be taken. [CR#702.56a]
||| writes the unbounded form in rules language -- replicate means "As an
||| additional cost to cast this spell, you may pay [cost] any number of
||| times" -- and the capped form is Tranquil Frillback's "up to three
||| times". 6 supported cards write the offer un-keyworded: the five
||| Adversaries at `AnyNumberOfTimes` and Tranquil Frillback at
||| `UpToTimes 3`.
|||
||| A slot on the payment and not a `Repeated` around it: `Repeated`
||| takes a definite `Amount` and iterates a clause, where this is ONE
||| payment offered a number of times as part of a single resolution,
||| which is what [CR#603.12a] rests on -- "if a resolving spell or
||| ability includes a choice to pay a cost multiple times and creates a
||| triggered ability that triggers when that payment is made, paying
||| that cost one or more times causes the reflexive triggered ability to
||| trigger only once".
||| -- spelling: nothing after the cost at `PaidOnce`; "any number of
||| times" and "up to [n] times" after it otherwise.
public export
data PayTimes = PaidOnce | AnyNumberOfTimes | UpToTimes Nat

public export
Eq PayTimes where
  (==) PaidOnce PaidOnce = True
  (==) PaidOnce _ = False
  (==) AnyNumberOfTimes AnyNumberOfTimes = True
  (==) AnyNumberOfTimes _ = False
  (==) (UpToTimes m) (UpToTimes n) = m == n
  (==) (UpToTimes _) _ = False

||| Whether the offer may be taken more than once -- the payment then
||| leaves a COUNT for the clause after it to read ("put that many
||| +1/+1 counters on this creature").
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

||| The mana a SCALED cost adds per unit of what it counts: generic
||| mana, whose size the amount's own numeral carries, or a printed RUN
||| of symbols.
|||
||| Three supported lines write the run: Cyclone's "pay {G} for each wind
||| counter on it", Thelon's Curse's "pay {U} for each creature chosen
||| this way", and Norn's Annex's "pays {W/P} for each of those
||| creatures" (measured 2026-08-28). What refuses them is not the count
||| -- that composes -- but the unit: a generic amount is a number, and
||| [CR#107.4a] makes coloured mana in a cost payable "only with the
||| appropriate color of mana", so a coloured per-unit says something a
||| number cannot. `ManaCost` is what already carries that, hybrid and
||| Phyrexian symbols included [CR#107.4e,107.4f], so the unit is a run
||| and no new symbol vocabulary is minted.
|||
||| ONE type for both scaled ledgers. The cost SHIFT carries the same
||| `Amount` and the same missing symbol ("costs {2}{R} more to cast"),
||| so when its coloured payload lands it takes this unit rather than a
||| second one of its own.
||| -- spelling: nothing at `GenericUnit` beyond the amount's own
||| numeral; the printed symbols at `RunUnit`.
public export
data ManaUnit : Type where
  GenericUnit : ManaUnit
  RunUnit : (run : ManaCost) -> {auto 0 wr : ManaRun run} -> ManaUnit

||| The special actions [CR#116.2] a mana payment can serve -- the ones
||| whose taking a player PAYS mana for. [CR#116.2] closes its own list
||| at twelve, and these are the four of the twelve that cost mana:
||| turning a face-down creature face up [CR#116.2b], which pays the
||| morph cost [CR#702.37e] or, for a manifested card, its mana cost;
||| putting a companion into hand for {3} [CR#116.2g]; foretelling a card
||| for {2} [CR#116.2h]; and unlocking a locked half for its unlock cost
||| [CR#116.2m]. The other eight take no cost at all, so no spend
||| restriction could name them, and the enumeration is the rule's rather
||| than the corpus's.
|||
||| Not a keyword-named cost: [CR#116.2b]'s action is taken by paying
||| whichever cost the permanent has, and Qarsi Deceiver prints both
||| readings side by side ("pay a mana cost to turn a manifested creature
||| face up, or pay a morph cost") precisely because they come apart.
||| [CR#116.2m]'s unlock action is [CR#709.5e] read from the special-action
||| side: a player controlling a permanent with one or more locked halves
||| "may pay the mana cost of a locked half of that permanent to give that
||| permanent the appropriate unlocked designation", any time they have
||| priority with an empty stack during a main phase of their turn. That
||| is what `UnlockDoor` names here, and the designations it gives are
||| `LeftHalfUnlocked` and `RightHalfUnlocked`. The action is named at
||| this seat and nowhere else -- it is not taken, because taking one is
||| a player's choice in play and no card face writes an instruction to
||| take it. The 2 supported lines that instruct an unlock ("unlock a
||| locked door of up to one target Room you control", "Lock or unlock a
||| door of target Room you control") are [CR#709.5f]/[CR#709.5g]
||| EFFECTS rather than this special action, and they wait on the door
||| noun [CR#709.5j] the Room scope fence left out.
||| -- spelling: "to turn permanents face up", "to foretell cards", "to
||| unlock doors".
public export
data SpecialAction = TurnFaceUp | PutCompanionIntoHand | Foretell | UnlockDoor

public export
Eq SpecialAction where
  (==) TurnFaceUp TurnFaceUp = True
  (==) TurnFaceUp _ = False
  (==) PutCompanionIntoHand PutCompanionIntoHand = True
  (==) PutCompanionIntoHand _ = False
  (==) Foretell Foretell = True
  (==) Foretell _ = False
  (==) UnlockDoor UnlockDoor = True
  (==) UnlockDoor _ = False

||| A COST, described so that a mana sentence can name it. [CR#106.1] has
||| players spend mana "to pay costs, usually when casting spells and
||| activating abilities": the cost is what a spend restriction restricts
||| to, and casting and activating are the two usual occasions, not the
||| whole of it. So a described cost is a third thing a purpose can name,
||| beside the object being cast and the source whose ability is
||| activated.
|||
||| Three arms, one per way a printed line picks a cost out. 15 supported
||| lines carry at least one of them, measured 2026-08-28 out of 204
||| spend-restriction lines; a line may write two arms, so the per-arm
||| counts below sum higher.
|||
||| * by a SYMBOL its text writes -- "on costs that contain {X}" (Rosheen
|||   Meanderer, Rosheen Roaring Prophet, Nexos, Elementalist's Palette)
|||   and "pay a cost that contains {C}" (Cultivator Drone), 5 lines.
||| * by the KEYWORD that declares it -- "to pay cumulative upkeep costs"
|||   (Adarkar Unicorn, Snowfall) [CR#702.24a], "pay a disturb cost"
|||   (Unblinking Observer), "pay a morph cost" (Qarsi Deceiver), 4
|||   lines. Named as `PaidCostName` names one, and gated by the same
|||   `keywordCosts`: a word that declares no cost names none here
|||   either.
||| * by the SPECIAL ACTION it pays for -- "to turn permanents face up"
|||   (Overgrown Zealot, Tin Street Gossip, Creeping Peeper, Qarsi
|||   Deceiver), "to foretell cards" (Niko Defies Destiny, Karfell
|||   Harbinger) and "unlock doors" (Smoky Lounge, Creeping Peeper), 7
|||   lines over the three actions.
|||
||| Two lines look like this arm and are `ToActivate`'s: Quinjet
||| Technician's "to activate power-up abilities" and Sorcerer Class'
||| "to gain a Class level" both name ACTIVATED abilities, which the
||| older cell already reaches -- [CR#716.2c] says so in as many words,
||| "to gain a Class level" meaning "to activate an ability indicated by
||| a class level bar".
|||
||| Jegantha, the Wellspring is measured and NOT this type: "This mana
||| can't be spent to pay generic mana costs" restricts which PART of a
||| cost the mana may pay, where every arm here picks out a whole cost.
||| [CR#107.4b] is what separates them -- numerical and variable symbols
||| "represent generic mana in costs", a component of a cost rather than
||| a cost -- and Jegantha's mana pays the {R} of {2}{R} perfectly well.
||| 1 line, its own gap.
|||
||| Two of the keyword-named lines have no word to name yet: `keywordFacts`
||| carries no Disturb and no Morph row, so Unblinking Observer and Qarsi
||| Deceiver wait on the keyword catalog rather than on this type. The
||| cost-arm/cast-arm disjunction both of them also write needs nothing:
||| `SpendOnly` already takes a LIST of purposes, and an arm of each kind
||| in one list is that sentence.
public export
data CostNamed : Type where
  Containing : (sym : ManaSymbol) -> CostNamed
  OfKeyword : (kw : KeywordLabel) -> CostNamed
  OfSpecialAction : (act : SpecialAction) -> CostNamed

||| A named cost has to be one there is: the keyword arm needs a word
||| whose own rule writes a cost, exactly as `paidCostNamed` demands at
||| the readback seat. A symbol and a special action name themselves.
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
altRunWritten : Maybe ProducedRun -> Bool
altRunWritten Nothing = True
altRunWritten (Just _) = True

public export
AltRunWritten : Maybe ProducedRun -> Type
AltRunWritten alt = So (altRunWritten alt)

||| How free the colours of a produced run are of one another.
||| [CR#106.1a] fixes the five colours, so the axis is over that set and
||| never over a written one -- a run whose colours the LINE writes is
||| `AmongWritten`, which states its own set.
|||
||| Three values, one per way a line leaves the choice: "add one mana of
||| any color" fixes one colour for every unit (`SameColor`), "add three
||| mana in any combination of colors" leaves each unit free of the rest
||| (`EachColor`, 34 supported lines, measured 2026-08-28), and "add two
||| mana of different colors" frees each unit but forbids a repeat
||| (`DistinctColors`, 4 lines).
|||
||| `DistinctColors` is a value on this axis and not a rider: what it
||| constrains is exactly what the other two constrain, the relation
||| between one unit's colour and the next, and a rider would let it be
||| written beside `SameColor` -- "two mana of the same color, of
||| different colors" -- which is a contradiction no rule could resolve.
public export
data ColorFreedom = SameColor | EachColor | DistinctColors

||| What [CR#609.4b]'s spend permission lets mana COUNT AS. The rule
||| writes the payload as "mana of any [type or color]", and
||| [CR#118.14]'s sibling phrasing spells the same widening as "mana of
||| any type can be spent" -- so the two words the rules use are the two
||| this type carries, plus the one colour a single line names outright.
|||
||| It is a MATCHER and not a `ProducedRun`: a run says what mana was
||| made [CR#106.1b], where this says what already-made mana may be
||| treated as while a cost is paid. [CR#609.4b] keeps them apart in as
||| many words -- the permission "doesn't change what mana was actually
||| spent to pay that cost" -- so a payload that spelled a production
||| would say the mana had changed type, which is what the rule denies.
|||
||| `AnyType` is not `AnyColor` plus colorless spelled twice:
||| [CR#106.1a] has five colours and [CR#106.1b] six types, so the two
||| words admit different sets and 1 supported line writes the wider one
||| where 49 write the narrower (measured 2026-08-28).
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

||| The word a MARKER OBJECT calls itself by, and the third ascription
||| axis beside a card type and a subtype. Neither word ascribes a type:
||| [CR#111.1] makes a token "a marker used to represent any permanent
||| that isn't represented by a card", which is not a type it has, and
||| [CR#114.3] says outright that an emblem "has no types". So neither
||| can ride `ascriptionOk`, whose two axes are both type-valued and
||| whose whole content is [CR#109.2]'s "card type or subtype".
||| Closed at two because the rules define exactly two marker objects
||| [CR#111.1,114.1]; a counter is not one, being a marker ON an object
||| rather than an object.
public export
data MarkerWord = TokenMarker | EmblemMarker

public export
Eq MarkerWord where
  (==) TokenMarker TokenMarker = True
  (==) TokenMarker _ = False
  (==) EmblemMarker EmblemMarker = True
  (==) EmblemMarker _ = False

||| Where the rules put each marker object: [CR#111.1] puts a token onto
||| the battlefield and [CR#114.2] puts an emblem into the command zone.
public export
markerZone : MarkerWord -> Zone
markerZone TokenMarker = Battlefield
markerZone EmblemMarker = Command

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

||| A designation a player, an object, a card or the game can hold.
|||
||| The UNLOCKED PAIR is the one entry no printed line names. [CR#709.5c]
||| states it outright -- "'Left half unlocked' and 'right half unlocked'
||| are designations that a permanent on the battlefield can have" -- and
||| [CR#709.5d] gives one as the permanent enters, [CR#709.5f] gives one
||| on an instruction to "unlock", [CR#709.5g] takes one away on an
||| instruction to "lock". So the rows are the rule's own vocabulary, and
||| what a card prints is always the DOOR: "unlock a locked door of up to
||| one target Room you control", "Lock or unlock a door of target Room
||| you control", "the number of unlocked doors among Rooms you control".
||| 0 supported lines write either designation by name, measured
||| 2026-08-28 -- so `HasDesignation` and `GainsDesignation` can reach
||| them and no printed sentence yet does, the door noun [CR#709.5j]
||| being the spelling that stands between.
public export
data Designation
  = -- PLAYER-held ([CR#725.1], [CR#726.1], [CR#702.131c], [CR#702.195b]).
    Monarch | TheInitiative | CitysBlessing | EnduringStory
  | -- OBJECT-held: the permanent markers ([CR#701.15b], [CR#701.54b]).
    Goaded | RingBearer | Monstrous | Renowned | Suspected | Saddled
  | Prepared
  | -- OBJECT-held, the unlocked pair ([CR#709.5c]).
    LeftHalfUnlocked | RightHalfUnlocked
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
designationScope LeftHalfUnlocked = HeldBy Object
designationScope RightHalfUnlocked = HeldBy Object
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
  (==) LeftHalfUnlocked LeftHalfUnlocked = True
  (==) LeftHalfUnlocked _ = False
  (==) RightHalfUnlocked RightHalfUnlocked = True
  (==) RightHalfUnlocked _ = False
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
designationChecked LeftHalfUnlocked = True
designationChecked RightHalfUnlocked = True
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
designationGiven LeftHalfUnlocked = True
designationGiven RightHalfUnlocked = True
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
designationSeedZone LeftHalfUnlocked = Just Battlefield
designationSeedZone RightHalfUnlocked = Just Battlefield
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
-- [CR#709.5c] gives the unlocked pair to "a permanent on the
-- battlefield" and names no card type, where every row above takes the
-- type its own rule names. Rooms are the only permanents that carry
-- them today; the rule is not written about Rooms, and neither is this.
designationSeedType LeftHalfUnlocked = Nothing
designationSeedType RightHalfUnlocked = Nothing
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
|||
||| The `Enchanted` catch-all STANDS, and no tightening is written here.
||| It admits "enchanted instant" and, since the catalog grew the six
||| command-zone types, "enchanted conspiracy" -- and the question that
||| decides both is where the restriction lives, not how many lines write
||| the phrase. [CR#702.5a] answers it outright: "the enchant ability
||| restricts what an Aura spell can target and what an Aura can
||| enchant", and [CR#303.4] says the same in the Aura's own rule. So the
||| constraint is the ENCHANT ability's subject, not this word's head, and
||| [CR#109.1] makes every one of these heads an object an Aura could in
||| principle be attached to -- a card is an object whatever zone it is
||| in, which is why "enchanted creature card in a graveyard" is ordinary
||| printed text. A rule refusing a head here would have to be a rule
||| about the PARTICIPLE, and the CR states none; a refusal by count is
||| what this workbench does not write.
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
attachHeadOk Equipped (TypedCardW _) = False
attachHeadOk Equipped SpellW = False
attachHeadOk Equipped PlayerW = False
attachHeadOk Equipped PermanentW = True
attachHeadOk Equipped TokenW = False
attachHeadOk Equipped CopyW = False
attachHeadOk Equipped JoinW = False
attachHeadOk Equipped AbilityJoinW = False
attachHeadOk Equipped AbilityW = False
attachHeadOk Equipped AbilityCopyW = False
attachHeadOk Equipped CopyJoinW = False
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
attachHeadOk Fortified (TypedCardW _) = False
attachHeadOk Fortified SpellW = False
attachHeadOk Fortified PlayerW = False
attachHeadOk Fortified PermanentW = False
attachHeadOk Fortified TokenW = False
attachHeadOk Fortified CopyW = False
attachHeadOk Fortified JoinW = False
attachHeadOk Fortified AbilityJoinW = False
attachHeadOk Fortified AbilityW = False
attachHeadOk Fortified AbilityCopyW = False
attachHeadOk Fortified CopyJoinW = False

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
attachHostZone (TypedCardW _) = Just Battlefield
attachHostZone SpellW = Just Battlefield
attachHostZone PermanentW = Just Battlefield
attachHostZone TokenW = Just Battlefield
attachHostZone CopyW = Just Stack
attachHostZone JoinW = Nothing
attachHostZone AbilityJoinW = Nothing
attachHostZone AbilityW = Nothing
attachHostZone AbilityCopyW = Nothing
attachHostZone CopyJoinW = Nothing

public export
attachHostTy : NounWord -> Maybe CardType
attachHostTy (TypeW t) = Just t
attachHostTy CardW = Nothing
attachHostTy (TypedCardW t) = Just t
attachHostTy SpellW = Nothing
attachHostTy PlayerW = Nothing
attachHostTy PermanentW = Nothing
attachHostTy TokenW = Nothing
attachHostTy CopyW = Nothing
attachHostTy JoinW = Nothing
attachHostTy AbilityJoinW = Nothing
attachHostTy AbilityW = Nothing
attachHostTy AbilityCopyW = Nothing
attachHostTy CopyJoinW = Nothing

public export
data OutcomeVerb = WinGame | LoseGame

public export
Eq OutcomeVerb where
  (==) WinGame WinGame = True
  (==) WinGame _ = False
  (==) LoseGame LoseGame = True
  (==) LoseGame _ = False

||| The prohibition ACTS -- `OutcomeGateKind`, `PlayerAct` and
||| `ObjectAct` -- retired into the deed labels of
||| `Experimental.Events.deedFacts`. They were three closed enums for
||| what is one open vocabulary: the same act is written in two voices
||| one card apart ("your opponents can't cast spells" against "spells
||| with the chosen name can't be cast"), and under labels that is one
||| deed at two roles rather than a row in each of two tables. What
||| `riderAct` decided is the `deedRides` field of the same rows.
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
  ||| Blood Spatter Analysis' tally of creatures that have died: the same
  ||| ordinary marker [CR#122.1], counted by the ability that reads it.
  Bloodstain : CounterKind
  ||| Cyclone's escalating tally: the same ordinary marker [CR#122.1],
  ||| counted by the payment that reads it.
  Wind : CounterKind

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
counterScope Bloodstain = Object
counterScope Wind = Object

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
  (==) Bloodstain Bloodstain = True
  (==) Bloodstain _ = False
  (==) Wind Wind = True
  (==) Wind _ = False

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
data CounterKindSource : Bindings -> Type where
  PrintedKind : CounterKind -> CounterKindSource bs
  ||| The menu has to have an arm: [CR#122.1] places a counter of some
  ||| name, and an empty list names none. A ONE-arm menu and a repeated
  ||| arm are both tolerated and both unwritten -- each is performable,
  ||| and each says what `PrintedKind` says, since [CR#122.1] makes
  ||| counters with the same name interchangeable.
  ChosenKind : (menu : List CounterKind) ->
               {auto 0 ne : NonEmpty menu} -> CounterKindSource bs
  ||| "your choice of two different counters ... from among [k1], [k2],
  ||| and [k3]" (Grimdancer): the same menu picked more than once, with
  ||| no kind picked twice. Its own arm beside `ChosenKind` because the
  ||| pick is a different SHAPE -- a subset of the menu where that one
  ||| takes a member -- and because the word has to be said at all for
  ||| the reason `ChosenKind` tolerates a repeated arm: [CR#122.1] makes
  ||| counters of one name interchangeable, so nothing but the printed
  ||| "different" stops one kind being picked twice.
  ||| The COUNT stays on the clause's own amount slot, where the menu
  ||| cannot see it; a pick wider than its menu is [CR#608.2d]'s
  ||| impossible option and is recorded overgeneration, since the amount
  ||| need not be literal.
  DistinctChosenKinds : (menu : List CounterKind) ->
                        {auto 0 ne : NonEmpty menu} -> CounterKindSource bs
  ||| "a counter of that kind": the kind an earlier clause BOUND, read
  ||| back [CR#607.2d]. Both binders write it -- the chooser ("choose a
  ||| kind of counter ... put a counter of that kind") and the
  ||| distributive pass ("for each kind of counter on [n], put another
  ||| counter of that kind on it") -- because both leave one
  ||| `Quality CounterKindQ` mention and the read asks only that there be
  ||| exactly one.
  ||| ONE benched carrier of the 13 supported lines, and it is a
  ||| paragraph rather than a whole card: Contractual Safeguard's second,
  ||| which the board-read chooser `CounterKindOn` unblocked. For the
  ||| other 12 the blocker is never this arm. 4 give the counter to "that
  ||| permanent or player" and `PutCounters` takes an object; 2 write "on
  ||| it" where two permanents are in scope and the pronoun refuses; 1
  ||| puts the pass inside a static; 1 mixes a printed kind and a bound
  ||| one in one menu; 1 needs a partitive holder. The last 3 are the
  ||| board-read chooser's own remainder: a trailing "if it doesn't have
  ||| a counter of that kind on it" (Aven Courier), a distributive
  ||| chooser under a Saga chapter (The Caves of Androzani), and an
  ||| at-random pick from a printed counter-kind menu with an exclusion
  ||| rider (Crystalline Giant). Each is somebody else's cell.
  ||| RECORDED OVERGENERATION: `counterSourceScope` answers True at every
  ||| kind here, where a printed word answers [CR#122.1]'s own scope. A
  ||| bound kind's scope is whatever the binder drew it from and no slot
  ||| carries that, so the gate has nothing to ask; every printed line
  ||| binds from a holder that could hold it.
  BoundKind : {auto 0 ok : countChoice (QSort CounterKindQ) bs = 1} ->
              CounterKindSource bs

||| Every arm has to name a counter the recipient can hold: [CR#122.1]
||| puts a counter on an object or a player, so an arm at the other
||| scope is an arm no one could pick.
public export
counterSourceScope : {0 bs : Bindings} -> CounterKindSource bs -> Kind -> Bool
counterSourceScope (PrintedKind c) k = counterScope c == k
counterSourceScope (ChosenKind menu) k = all (\c => counterScope c == k) menu
counterSourceScope (DistinctChosenKinds menu) k = all (\c => counterScope c == k) menu
counterSourceScope BoundKind k = True

public export
CounterSourceScope : {bs : Bindings} -> CounterKindSource bs -> Kind -> Type
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

||| [CR#110.4b]'s list, read off a mention's head type: a permanent spell
||| is "an artifact, battle, creature, enchantment, or planeswalker
||| spell". LAND is on `permanentType`'s list and not on this one --
||| [CR#305.1] makes a played land never a spell -- and a mention that
||| names no type answers no, since the transition is stated of a
||| permanent spell and not of every stack object.
public export
permanentSpellType : Maybe CardType -> Bool
permanentSpellType Nothing = False
permanentSpellType (Just Land) = False
permanentSpellType (Just t) = permanentType t

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
-- ungated like every zone but the battlefield. [CR#903.3] admits a
-- creature, Vehicle or Spacecraft card and [CR#903.3a] any card whose own
-- ability says it may be a commander, [CR#400.4b] keeps five further
-- types in the zone outright, and the legendary supertype the first of
-- those turns on is no card TYPE -- so the type slot alone refuses
-- nothing here. A described placement of an instant card into the command
-- zone overgenerates and is tolerated; no printed line writes one.
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

||| The TRIGGER HEADER's reader over the same transitions, and the ONE
||| cell where it disagrees with the table above.
||| Two tables and not one widened cell. `statusEventOk` asks whether a
||| transition exists at all to be named, which is a fact about the
||| status: [CR#710.4] makes flipping one-way, so nothing ever becomes
||| unflipped. This asks the different question a HEADER puts -- whether
||| the corpus writes a trigger on that transition -- and the two
||| questions have different answers at exactly one value.
||| FaceDown: 132 supported occurrences write "turned face up" and every
||| one of them heads a trigger or an as-clause; "turned face down"
||| occurs ONCE in the whole supported corpus (Vesuvan Shapeshifter,
||| "until this creature is turned face down") and that occurrence is a
||| DURATION ENDPOINT, not a header. Re-measured 2026-08-28. So the
||| header cell stays closed at zero and the endpoint reads the
||| transition through `statusEventOk`, which is what a split buys and a
||| widened cell would have thrown away.
||| Everything else is `statusEventOk`'s answer unchanged; there is no
||| second disagreement and none is claimed.
public export
statusHeaderOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusHeaderOk FaceDown = False
statusHeaderOk v = statusEventOk v

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

||| What a colour ascription writes: the colours it names, or the printed
||| quantifier "all colors" (8 supported lines). Two arms rather than a
||| five-way enumeration, for `AddsEveryType`'s own reason -- the printed
||| word is a quantifier and not a list, even though the space it ranges
||| over is closed where a subtype set is not ([CR#105.1] names the five).
||| An empty `SomeColors` is "colorless" [CR#105.2c].
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

||| An arrival rider: what a permanent's entry says about it beyond its
||| characteristics. `EntersAs` is indexed over `StatusVal` and carries
||| `SetStatus`' own gate, so the status vocabulary the flip verb writes
||| ("turn it face down", "tap it") is the vocabulary an arrival writes
||| too: [CR#708.3] turns an object face down BEFORE it enters, which is
||| what makes "return it to the battlefield face down" (Yedora, Grave
||| Gardener) a rider on the arrival and not a second instruction.
||| Attacking is not a status -- [CR#506.3a] and [CR#508.4d] speak of a
||| permanent that "enters the battlefield attacking", a combat position
||| no `StatusVal` denotes -- so it stays its own row.
||| -- spelling: "tapped", "face down", "attacking" after the destination.
public export
data TokenRider : Type where
  EntersAs : {0 c : StatusCat} -> (v : StatusVal c) ->
             {auto 0 at : StatusEffectVal v} -> TokenRider
  EntersAttacking : TokenRider
  ||| "Return this card to the battlefield transformed", "put it onto
  ||| the battlefield transformed under its owner's control": the back
  ||| face arrives face up. [CR#712.14a] makes it an arrival property
  ||| and not a second instruction -- "If a spell or ability puts a
  ||| double-faced card onto the battlefield 'transformed' or
  ||| 'converted,' it enters the battlefield with its back face up" --
  ||| which is the same sentence shape [CR#708.3] writes for the face-down
  ||| rider `EntersAs` already carries.
  ||| NOT an `EntersAs` value. [CR#110.5] closes a permanent's status at
  ||| four categories of two values each and back-face-up is none of them,
  ||| and [CR#701.27b] says in as many words that transforming a permanent
  ||| and turning one face up "are different game actions" even though
  ||| they share the physical
  ||| motion. So the vocabulary a `StatusVal` indexes cannot reach it and
  ||| a row of its own is what the rules leave.
  ||| 94 supported faces write it (measured 2026-08-28): 67 "to the
  ||| battlefield transformed", 92 occurrences of the "transformed under
  ||| [possessor]'s control" tail, and Corruption of Towashi's "a permanent
  ||| you control enters transformed".
  ||| -- spelling: "transformed" after the destination, before the
  ||| controller override.
  EntersTransformed : TokenRider
  ||| "exile them, then meld them into Brisela, Voice of Nightmares":
  ||| [CR#701.42a]'s own arrival, "put them onto the battlefield with
  ||| their back faces up and combined". Beside `EntersTransformed` and
  ||| not a use of it, because "combined" is the whole of what melding
  ||| adds -- [CR#712.4a] leaves "a single object represented by two
  ||| cards" where a transformed arrival leaves one card back face up.
  ||| The name is the clause's own slot: all 7 supported meld lines write
  ||| it, and it names the melded permanent rather than either component
  ||| [CR#712.4b].
  ||| It states no pairing gate. [CR#701.42b] admits only two cards of
  ||| the same meld pair and [CR#701.42c] leaves anything else in its
  ||| current zone, which is a fact about the CARDS named and not about
  ||| the clause naming them; nothing a card face writes could be
  ||| refused here for it.
  ||| -- spelling: "into [name]" after the verb.
  EntersMelded : (into : String) -> TokenRider

||| The commonest arrival rider, spelled as the status word it is.
public export
EntersTapped : TokenRider
EntersTapped = EntersAs Tapped

public export
lastType : List CardType -> Maybe CardType
lastType [] = Nothing
lastType [t] = Just t
lastType (_ :: ts) = lastType ts


public export
data TurnPart = Turn | Upkeep | EndStep | Combat | UntapStep | EndOfCombat
              | FirstMain | PostcombatMain | DrawStep
              | MainPhase
              -- the beginning phase [CR#500.1], the one phase the corpus
              -- ADDS that no other row names. Its three steps are the
              -- untap, upkeep and draw steps [CR#501.1], each of which
              -- already has a row, so this is the phase and never a
              -- spelling of one of them: [CR#500.8] adds a phase where
              -- [CR#500.9] adds a step. 3 supported lines write it --
              -- Cyclonus, Shadow of the Second Sun and Sphinx of the
              -- Second Sun, all "an additional beginning phase after
              -- this phase" (measured 2026-09-02).
              | BeginningPhase

||| Over what period a RANK counts. "The second spell you cast this
||| turn" ranks inside one stretch already named; "the second spell you
||| cast each turn" ranks inside every one of them, the count starting
||| again as the period does.
|||
||| The recurring value is NOT a `Lookback` arm and could not be one.
||| A `Lookback` names a stretch a retrospective reader scopes an event
||| to, and `sameWindow` compares two such stretches; "each turn" names
||| no stretch -- it says over what period a COUNT resets. That is the
||| distinction `NthOccurrence` already draws on the event side, where
||| the reset is its own slot beside the header's window for the same
||| reason.
||| -- spelling: "this turn" / "this game" at `RankWithin`, "each turn"
||| at `RankEach`.
public export
data RankPeriod : Type where
  RankWithin : Lookback -> RankPeriod
  RankEach : TurnPart -> RankPeriod

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
  (==) BeginningPhase BeginningPhase = True
  (==) BeginningPhase _ = False

||| A turn-based action a window is written relative to, where
||| `TurnPart` names a part to be inside [CR#508.1].
public export
data TurnPoint = AttackersDeclared

namespace Owner
  public export
  data Owner = Yours | ThatPlayers | EachPlayers | EachOpponents
             | EachYours | AnOpponents
             | ThatTurns
             -- "each OTHER player's untap step": the quantifier
             -- possessor with an other-marking over it. Beside
             -- `EachPlayers` and not a spelling of it -- [CR#102.1]
             -- makes the active player one of the players, so a
             -- quantifier that excludes the sentence's own subject
             -- ranges over a different set, and the 14 supported lines
             -- that write it (Seedborn Muse's family, measured
             -- 2026-09-02) all mean the step the subject's controller
             -- does not get. Not `EachOpponents` either: [CR#102.2]
             -- leaves a teammate a non-opponent [CR#102.3], and the
             -- word printed here is "other player", not "opponent".
             | EachOthers

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
  (==) EachOthers EachOthers = True
  (==) EachOthers _ = False

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
-- the other-marked quantifier announces a player exactly as the plain
-- one does: "each other player" ranges over players and a following
-- clause reads one of them back.
possessorB (Just EachOthers) = [MkBinding EachD Player OneOf PlayerP]

||| Which possessor may say WHOSE turn part a duration ends at. A
||| duration ends at ONE moment, and [CR#500.1] runs every phase and
||| step on every turn, so a possessor naming several players names
||| several such moments and no end at all: the four quantifier words
||| and the indefinite are refused here. `ThatTurns` is refused for the
||| other reason -- it names a TURN and not a player, so it answers a
||| different question than the one this slot asks.
|||
||| The two admitted words were a two-arm datatype of their own
||| (`Whose`), spelled row for row like `Owner`'s first two. One
||| vocabulary and an admission table is the house shape; the sub-enum
||| paid a second datatype to say what this one line says.
public export
durationPossessorOk : Maybe Owner -> Bool
durationPossessorOk Nothing = True
durationPossessorOk (Just Yours) = True
durationPossessorOk (Just ThatPlayers) = True
durationPossessorOk (Just EachPlayers) = False
durationPossessorOk (Just EachOpponents) = False
durationPossessorOk (Just EachYours) = False
durationPossessorOk (Just AnOpponents) = False
durationPossessorOk (Just ThatTurns) = False
durationPossessorOk (Just EachOthers) = False

public export
DurationPossessor : Maybe Owner -> Type
DurationPossessor w = So (durationPossessorOk w)

public export
data DurationEnd : Type where
  StartOf : TurnPart -> (w : Maybe Owner) ->
            {auto 0 dp : DurationPossessor w} -> DurationEnd
  EndOf : TurnPart -> (w : Maybe Owner) ->
          {auto 0 dp : DurationPossessor w} -> DurationEnd


