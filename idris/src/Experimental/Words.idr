||| The lexical/taxonomy substrate: card types, zones, keywords, subtypes,
||| counters, statuses, bindings, payloads, mana, and designations.
module Experimental.Words

import public Data.List
import public Data.Maybe
import public Data.Nat
import public Data.So

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
  Fighter : {auto 0 ok : So (combatant t)} -> FightParticipant (Just t)

public export
data Characteristic = Power | Toughness | ManaValue

public export
Eq Characteristic where
  (==) Power Power = True
  (==) Power _ = False
  (==) Toughness Toughness = True
  (==) Toughness _ = False
  (==) ManaValue ManaValue = True
  (==) ManaValue _ = False

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
ChosenQualityRead : QualitySort -> Type
ChosenQualityRead q = So (chosenQualityReadOk q)

public export
data LetterWord = LetterX | LetterY

public export
Eq LetterWord where
  (==) LetterX LetterX = True
  (==) LetterX LetterY = False
  (==) LetterY LetterX = False
  (==) LetterY LetterY = True

public export
data Kind = Object | Player | Quality QualitySort | Outcome | Gap
          | Letter LetterWord | TurnRef | Ability | ObjectOrPlayer

public export
Eq Kind where
  (==) Object Object = True
  (==) Object Player = False
  (==) Object (Quality _) = False
  (==) Object Outcome = False
  (==) Object Gap = False
  (==) Object (Letter _) = False
  (==) Object TurnRef = False
  (==) Object Ability = False
  (==) Object ObjectOrPlayer = False
  (==) Player Object = False
  (==) Player Player = True
  (==) Player (Quality _) = False
  (==) Player Outcome = False
  (==) Player Gap = False
  (==) Player (Letter _) = False
  (==) Player TurnRef = False
  (==) Player Ability = False
  (==) Player ObjectOrPlayer = False
  (==) (Quality _) Object = False
  (==) (Quality _) Player = False
  (==) (Quality a) (Quality b) = a == b
  (==) (Quality _) Outcome = False
  (==) (Quality _) Gap = False
  (==) (Quality _) (Letter _) = False
  (==) (Quality _) TurnRef = False
  (==) (Quality _) Ability = False
  (==) (Quality _) ObjectOrPlayer = False
  (==) Outcome Object = False
  (==) Outcome Player = False
  (==) Outcome (Quality _) = False
  (==) Outcome Outcome = True
  (==) Outcome Gap = False
  (==) Outcome (Letter _) = False
  (==) Outcome TurnRef = False
  (==) Outcome Ability = False
  (==) Outcome ObjectOrPlayer = False
  (==) Gap Object = False
  (==) Gap Player = False
  (==) Gap (Quality _) = False
  (==) Gap Outcome = False
  (==) Gap Gap = True
  (==) Gap (Letter _) = False
  (==) Gap TurnRef = False
  (==) Gap Ability = False
  (==) Gap ObjectOrPlayer = False
  (==) (Letter _) Object = False
  (==) (Letter _) Player = False
  (==) (Letter _) (Quality _) = False
  (==) (Letter _) Outcome = False
  (==) (Letter _) Gap = False
  (==) (Letter a) (Letter b) = a == b
  (==) (Letter _) TurnRef = False
  (==) (Letter _) Ability = False
  (==) (Letter _) ObjectOrPlayer = False
  (==) TurnRef Object = False
  (==) TurnRef Player = False
  (==) TurnRef (Quality _) = False
  (==) TurnRef Outcome = False
  (==) TurnRef Gap = False
  (==) TurnRef (Letter _) = False
  (==) TurnRef TurnRef = True
  (==) TurnRef Ability = False
  (==) TurnRef ObjectOrPlayer = False
  (==) Ability Object = False
  (==) Ability Player = False
  (==) Ability (Quality _) = False
  (==) Ability Outcome = False
  (==) Ability Gap = False
  (==) Ability (Letter _) = False
  (==) Ability TurnRef = False
  (==) Ability Ability = True
  (==) Ability ObjectOrPlayer = False
  (==) ObjectOrPlayer Object = False
  (==) ObjectOrPlayer Player = False
  (==) ObjectOrPlayer (Quality _) = False
  (==) ObjectOrPlayer Outcome = False
  (==) ObjectOrPlayer Gap = False
  (==) ObjectOrPlayer (Letter _) = False
  (==) ObjectOrPlayer TurnRef = False
  (==) ObjectOrPlayer Ability = False
  (==) ObjectOrPlayer ObjectOrPlayer = True

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

||| A `LetterWord` matches only itself.
public export
sameLetterWordRefl : (w : LetterWord) -> So (w == w)
sameLetterWordRefl LetterX = Oh
sameLetterWordRefl LetterY = Oh

||| `==` decides equality likewise.
public export
sameLetterWordEq : (a, b : LetterWord) -> So (a == b) -> a = b
sameLetterWordEq LetterX LetterX _ = Refl
sameLetterWordEq LetterX LetterY ok = absurd ok
sameLetterWordEq LetterY LetterX ok = absurd ok
sameLetterWordEq LetterY LetterY _ = Refl

||| Every kind matches itself.
public export
sameKindRefl : (k : Kind) -> So (k == k)
sameKindRefl Object = Oh
sameKindRefl Player = Oh
sameKindRefl (Quality q) = sameQRefl q
sameKindRefl Outcome = Oh
sameKindRefl Gap = Oh
sameKindRefl (Letter w) = sameLetterWordRefl w
sameKindRefl TurnRef = Oh
sameKindRefl Ability = Oh
sameKindRefl ObjectOrPlayer = Oh

||| Which kind pairs have a join. Like-with-like joins itself; `Object` and
||| `Player` join in either order; `ObjectOrPlayer` absorbs both and itself.
||| Nothing else joins: among the kinds a target slot admits (`Targetable`
||| has only `Object` and `Player`) the corpus attests this one cross-kind
||| join. Kinds outside that set are not surveyed here: "target spell or
||| ability" is attested and would be `Object \/ Ability`, so that cell is
||| `False` as unsurveyed, not refused — it is `workbench-unhomed-union-gates`'
||| to measure and admit, and no pin freezes it.
public export
joinable : Kind -> Kind -> Bool
joinable Object Player = True
joinable Object ObjectOrPlayer = True
joinable Player Object = True
joinable Player ObjectOrPlayer = True
joinable ObjectOrPlayer Object = True
joinable ObjectOrPlayer Player = True
joinable ObjectOrPlayer ObjectOrPlayer = True
joinable a b = a == b

export infixl 5 \/

||| The kind join, FLAT ON `Kind`: `Object \/ Player = ObjectOrPlayer`, one
||| joined kind and no pair in the index. WHICH union a term names is carried
||| by `Payload`, not by the kind — the ticket's flat/pair/powerset trichotomy
||| conflated the two levels, since its "pairs" ({Permanent,Player},
||| {Creature,Player}) are card-type and possessor content, which `Payload` and
||| the predicates already hold.
|||
||| The measurements: the demonstrative echo copies its antecedent's kind pair
||| 33 of 33 with no crossing in either direction; the 15 readbacks with no
||| pair to echo write the generic "permanent or player"; the 10 class-word
||| readbacks write that same generic pair, which a naive powerset kind
||| ({Creature,Player,Planeswalker,Battle}) would mis-spell as "that creature,
||| player, planeswalker, or battle"; and all 43 share one demonstrative.
||| So the 33 echoes derive from the Object-half type the joined payload will
||| carry, while the class word and the 15 no-pair readbacks have no type and
||| spell the generic pair — the kind index needs no collapse table.
|||
||| Site 2's 2x4 admissible-pair grid is payload admissibility, routed to
||| `workbench-unhomed-union-gates`; the joined payload and its Object-half
||| type are `workbench-joined-kind-binding`. Neither is decided here.
|||
||| The semilattice is UNBOUNDED by decision — no unit kind. `Or` carries
||| `TwoDisjuncts`, so a heterogeneous fold is over a non-empty list and seeds
||| with its head; the "an empty `Or` is vacuous" reading that justified old
||| semantics' `Empty` is refused upstream, so no `Top` and no `Bottom`.
|||
||| R5(c): the kinds are INPUTS. `ok` is erased and `Oh : So True` constrains
||| nothing, so no call may leave `a` or `b` to be inferred from the witness.
||| Off the attested pairs the operator returns its left kind; `ok` refuses
||| every such pair ("any target" is the one join English writes [CR#115.4]).
public export
(\/) : (a, b : Kind) -> {auto 0 ok : So (joinable a b)} -> Kind
(\/) Object Object = Object
(\/) Object Player = ObjectOrPlayer
(\/) Object ObjectOrPlayer = ObjectOrPlayer
(\/) Object _ = Object
(\/) Player Object = ObjectOrPlayer
(\/) Player Player = Player
(\/) Player ObjectOrPlayer = ObjectOrPlayer
(\/) Player _ = Player
(\/) ObjectOrPlayer _ = ObjectOrPlayer
(\/) (Quality q) _ = Quality q
(\/) Outcome _ = Outcome
(\/) Gap _ = Gap
(\/) (Letter w) _ = Letter w
(\/) TurnRef _ = TurnRef
(\/) Ability _ = Ability

||| Reflexivity of the gate: every kind joins itself.
public export
joinableRefl : (k : Kind) -> So (joinable k k)
joinableRefl Object = Oh
joinableRefl Player = Oh
joinableRefl (Quality q) = sameQRefl q
joinableRefl Outcome = Oh
joinableRefl Gap = Oh
joinableRefl (Letter w) = sameLetterWordRefl w
joinableRefl TurnRef = Oh
joinableRefl Ability = Oh
joinableRefl ObjectOrPlayer = Oh

||| Symmetry of the gate, as a LEMMA: a symmetric constructor on a witness
||| type would loop auto search, so the fact is a function instead.
public export
joinableSym : (a, b : Kind) -> So (joinable a b) -> So (joinable b a)
joinableSym Object Object _ = Oh
joinableSym Object Player _ = Oh
joinableSym Object (Quality _) ok = absurd ok
joinableSym Object Outcome ok = absurd ok
joinableSym Object Gap ok = absurd ok
joinableSym Object (Letter _) ok = absurd ok
joinableSym Object TurnRef ok = absurd ok
joinableSym Object Ability ok = absurd ok
joinableSym Object ObjectOrPlayer _ = Oh

joinableSym Player Object _ = Oh
joinableSym Player Player _ = Oh
joinableSym Player (Quality _) ok = absurd ok
joinableSym Player Outcome ok = absurd ok
joinableSym Player Gap ok = absurd ok
joinableSym Player (Letter _) ok = absurd ok
joinableSym Player TurnRef ok = absurd ok
joinableSym Player Ability ok = absurd ok
joinableSym Player ObjectOrPlayer _ = Oh

joinableSym (Quality _) Object ok = absurd ok
joinableSym (Quality _) Player ok = absurd ok
joinableSym (Quality x) (Quality y) ok = case sameQEq x y ok of Refl => ok
joinableSym (Quality _) Outcome ok = absurd ok
joinableSym (Quality _) Gap ok = absurd ok
joinableSym (Quality _) (Letter _) ok = absurd ok
joinableSym (Quality _) TurnRef ok = absurd ok
joinableSym (Quality _) Ability ok = absurd ok
joinableSym (Quality _) ObjectOrPlayer ok = absurd ok

joinableSym Outcome Object ok = absurd ok
joinableSym Outcome Player ok = absurd ok
joinableSym Outcome (Quality _) ok = absurd ok
joinableSym Outcome Outcome _ = Oh
joinableSym Outcome Gap ok = absurd ok
joinableSym Outcome (Letter _) ok = absurd ok
joinableSym Outcome TurnRef ok = absurd ok
joinableSym Outcome Ability ok = absurd ok
joinableSym Outcome ObjectOrPlayer ok = absurd ok

joinableSym Gap Object ok = absurd ok
joinableSym Gap Player ok = absurd ok
joinableSym Gap (Quality _) ok = absurd ok
joinableSym Gap Outcome ok = absurd ok
joinableSym Gap Gap _ = Oh
joinableSym Gap (Letter _) ok = absurd ok
joinableSym Gap TurnRef ok = absurd ok
joinableSym Gap Ability ok = absurd ok
joinableSym Gap ObjectOrPlayer ok = absurd ok

joinableSym (Letter _) Object ok = absurd ok
joinableSym (Letter _) Player ok = absurd ok
joinableSym (Letter _) (Quality _) ok = absurd ok
joinableSym (Letter _) Outcome ok = absurd ok
joinableSym (Letter _) Gap ok = absurd ok
joinableSym (Letter v) (Letter w) ok = case sameLetterWordEq v w ok of Refl => ok
joinableSym (Letter _) TurnRef ok = absurd ok
joinableSym (Letter _) Ability ok = absurd ok
joinableSym (Letter _) ObjectOrPlayer ok = absurd ok

joinableSym TurnRef Object ok = absurd ok
joinableSym TurnRef Player ok = absurd ok
joinableSym TurnRef (Quality _) ok = absurd ok
joinableSym TurnRef Outcome ok = absurd ok
joinableSym TurnRef Gap ok = absurd ok
joinableSym TurnRef (Letter _) ok = absurd ok
joinableSym TurnRef TurnRef _ = Oh
joinableSym TurnRef Ability ok = absurd ok
joinableSym TurnRef ObjectOrPlayer ok = absurd ok

joinableSym Ability Object ok = absurd ok
joinableSym Ability Player ok = absurd ok
joinableSym Ability (Quality _) ok = absurd ok
joinableSym Ability Outcome ok = absurd ok
joinableSym Ability Gap ok = absurd ok
joinableSym Ability (Letter _) ok = absurd ok
joinableSym Ability TurnRef ok = absurd ok
joinableSym Ability Ability _ = Oh
joinableSym Ability ObjectOrPlayer ok = absurd ok

joinableSym ObjectOrPlayer Object _ = Oh
joinableSym ObjectOrPlayer Player _ = Oh
joinableSym ObjectOrPlayer (Quality _) ok = absurd ok
joinableSym ObjectOrPlayer Outcome ok = absurd ok
joinableSym ObjectOrPlayer Gap ok = absurd ok
joinableSym ObjectOrPlayer (Letter _) ok = absurd ok
joinableSym ObjectOrPlayer TurnRef ok = absurd ok
joinableSym ObjectOrPlayer Ability ok = absurd ok
joinableSym ObjectOrPlayer ObjectOrPlayer _ = Oh

||| Idempotence.
public export
joinIdem : (k : Kind) -> (\/) k k {ok = joinableRefl k} = k
joinIdem Object = Refl
joinIdem Player = Refl
joinIdem (Quality q) = Refl
joinIdem Outcome = Refl
joinIdem Gap = Refl
joinIdem (Letter w) = Refl
joinIdem TurnRef = Refl
joinIdem Ability = Refl
joinIdem ObjectOrPlayer = Refl

||| Commutativity.
public export
joinComm : (a, b : Kind) -> (ok : So (joinable a b)) ->
           (\/) a b {ok} = (\/) b a {ok = joinableSym a b ok}
joinComm Object Object _ = Refl
joinComm Object Player _ = Refl
joinComm Object (Quality _) ok = absurd ok
joinComm Object Outcome ok = absurd ok
joinComm Object Gap ok = absurd ok
joinComm Object (Letter _) ok = absurd ok
joinComm Object TurnRef ok = absurd ok
joinComm Object Ability ok = absurd ok
joinComm Object ObjectOrPlayer _ = Refl

joinComm Player Object _ = Refl
joinComm Player Player _ = Refl
joinComm Player (Quality _) ok = absurd ok
joinComm Player Outcome ok = absurd ok
joinComm Player Gap ok = absurd ok
joinComm Player (Letter _) ok = absurd ok
joinComm Player TurnRef ok = absurd ok
joinComm Player Ability ok = absurd ok
joinComm Player ObjectOrPlayer _ = Refl

joinComm (Quality _) Object ok = absurd ok
joinComm (Quality _) Player ok = absurd ok
joinComm (Quality x) (Quality y) ok = case sameQEq x y ok of Refl => Refl
joinComm (Quality _) Outcome ok = absurd ok
joinComm (Quality _) Gap ok = absurd ok
joinComm (Quality _) (Letter _) ok = absurd ok
joinComm (Quality _) TurnRef ok = absurd ok
joinComm (Quality _) Ability ok = absurd ok
joinComm (Quality _) ObjectOrPlayer ok = absurd ok

joinComm Outcome Object ok = absurd ok
joinComm Outcome Player ok = absurd ok
joinComm Outcome (Quality _) ok = absurd ok
joinComm Outcome Outcome _ = Refl
joinComm Outcome Gap ok = absurd ok
joinComm Outcome (Letter _) ok = absurd ok
joinComm Outcome TurnRef ok = absurd ok
joinComm Outcome Ability ok = absurd ok
joinComm Outcome ObjectOrPlayer ok = absurd ok

joinComm Gap Object ok = absurd ok
joinComm Gap Player ok = absurd ok
joinComm Gap (Quality _) ok = absurd ok
joinComm Gap Outcome ok = absurd ok
joinComm Gap Gap _ = Refl
joinComm Gap (Letter _) ok = absurd ok
joinComm Gap TurnRef ok = absurd ok
joinComm Gap Ability ok = absurd ok
joinComm Gap ObjectOrPlayer ok = absurd ok

joinComm (Letter _) Object ok = absurd ok
joinComm (Letter _) Player ok = absurd ok
joinComm (Letter _) (Quality _) ok = absurd ok
joinComm (Letter _) Outcome ok = absurd ok
joinComm (Letter _) Gap ok = absurd ok
joinComm (Letter v) (Letter w) ok = case sameLetterWordEq v w ok of Refl => Refl
joinComm (Letter _) TurnRef ok = absurd ok
joinComm (Letter _) Ability ok = absurd ok
joinComm (Letter _) ObjectOrPlayer ok = absurd ok

joinComm TurnRef Object ok = absurd ok
joinComm TurnRef Player ok = absurd ok
joinComm TurnRef (Quality _) ok = absurd ok
joinComm TurnRef Outcome ok = absurd ok
joinComm TurnRef Gap ok = absurd ok
joinComm TurnRef (Letter _) ok = absurd ok
joinComm TurnRef TurnRef _ = Refl
joinComm TurnRef Ability ok = absurd ok
joinComm TurnRef ObjectOrPlayer ok = absurd ok

joinComm Ability Object ok = absurd ok
joinComm Ability Player ok = absurd ok
joinComm Ability (Quality _) ok = absurd ok
joinComm Ability Outcome ok = absurd ok
joinComm Ability Gap ok = absurd ok
joinComm Ability (Letter _) ok = absurd ok
joinComm Ability TurnRef ok = absurd ok
joinComm Ability Ability _ = Refl
joinComm Ability ObjectOrPlayer ok = absurd ok

joinComm ObjectOrPlayer Object _ = Refl
joinComm ObjectOrPlayer Player _ = Refl
joinComm ObjectOrPlayer (Quality _) ok = absurd ok
joinComm ObjectOrPlayer Outcome ok = absurd ok
joinComm ObjectOrPlayer Gap ok = absurd ok
joinComm ObjectOrPlayer (Letter _) ok = absurd ok
joinComm ObjectOrPlayer TurnRef ok = absurd ok
joinComm ObjectOrPlayer Ability ok = absurd ok
joinComm ObjectOrPlayer ObjectOrPlayer _ = Refl

||| Associativity over the attested sublattice: every regrouping the four
||| gates admit computes the same kind.
public export
joinAssoc : (a, b, c : Kind) ->
            (ab : So (joinable a b)) -> (bc : So (joinable b c)) ->
            (abc : So (joinable ((\/) a b {ok = ab}) c)) ->
            (a_bc : So (joinable a ((\/) b c {ok = bc}))) ->
            (\/) ((\/) a b {ok = ab}) c {ok = abc}
              = (\/) a ((\/) b c {ok = bc}) {ok = a_bc}
joinAssoc Object Object Object _ _ _ _ = Refl
joinAssoc Object Object Player _ _ _ _ = Refl
joinAssoc Object Object (Quality _) _ bc _ _ = absurd bc
joinAssoc Object Object Outcome _ bc _ _ = absurd bc
joinAssoc Object Object Gap _ bc _ _ = absurd bc
joinAssoc Object Object (Letter _) _ bc _ _ = absurd bc
joinAssoc Object Object TurnRef _ bc _ _ = absurd bc
joinAssoc Object Object Ability _ bc _ _ = absurd bc
joinAssoc Object Object ObjectOrPlayer _ _ _ _ = Refl

joinAssoc Object Player Object _ _ _ _ = Refl
joinAssoc Object Player Player _ _ _ _ = Refl
joinAssoc Object Player (Quality _) _ bc _ _ = absurd bc
joinAssoc Object Player Outcome _ bc _ _ = absurd bc
joinAssoc Object Player Gap _ bc _ _ = absurd bc
joinAssoc Object Player (Letter _) _ bc _ _ = absurd bc
joinAssoc Object Player TurnRef _ bc _ _ = absurd bc
joinAssoc Object Player Ability _ bc _ _ = absurd bc
joinAssoc Object Player ObjectOrPlayer _ _ _ _ = Refl

joinAssoc Object (Quality _) _ ab _ _ _ = absurd ab

joinAssoc Object Outcome _ ab _ _ _ = absurd ab

joinAssoc Object Gap _ ab _ _ _ = absurd ab

joinAssoc Object (Letter _) _ ab _ _ _ = absurd ab

joinAssoc Object TurnRef _ ab _ _ _ = absurd ab

joinAssoc Object Ability _ ab _ _ _ = absurd ab

joinAssoc Object ObjectOrPlayer _ _ _ _ _ = Refl

joinAssoc Player Object Object _ _ _ _ = Refl
joinAssoc Player Object Player _ _ _ _ = Refl
joinAssoc Player Object (Quality _) _ bc _ _ = absurd bc
joinAssoc Player Object Outcome _ bc _ _ = absurd bc
joinAssoc Player Object Gap _ bc _ _ = absurd bc
joinAssoc Player Object (Letter _) _ bc _ _ = absurd bc
joinAssoc Player Object TurnRef _ bc _ _ = absurd bc
joinAssoc Player Object Ability _ bc _ _ = absurd bc
joinAssoc Player Object ObjectOrPlayer _ _ _ _ = Refl

joinAssoc Player Player Object _ _ _ _ = Refl
joinAssoc Player Player Player _ _ _ _ = Refl
joinAssoc Player Player (Quality _) _ bc _ _ = absurd bc
joinAssoc Player Player Outcome _ bc _ _ = absurd bc
joinAssoc Player Player Gap _ bc _ _ = absurd bc
joinAssoc Player Player (Letter _) _ bc _ _ = absurd bc
joinAssoc Player Player TurnRef _ bc _ _ = absurd bc
joinAssoc Player Player Ability _ bc _ _ = absurd bc
joinAssoc Player Player ObjectOrPlayer _ _ _ _ = Refl

joinAssoc Player (Quality _) _ ab _ _ _ = absurd ab

joinAssoc Player Outcome _ ab _ _ _ = absurd ab

joinAssoc Player Gap _ ab _ _ _ = absurd ab

joinAssoc Player (Letter _) _ ab _ _ _ = absurd ab

joinAssoc Player TurnRef _ ab _ _ _ = absurd ab

joinAssoc Player Ability _ ab _ _ _ = absurd ab

joinAssoc Player ObjectOrPlayer _ _ _ _ _ = Refl

joinAssoc ObjectOrPlayer _ _ _ _ _ _ = Refl

joinAssoc (Quality q) _ _ _ _ _ _ = Refl
joinAssoc Outcome _ _ _ _ _ _ = Refl
joinAssoc Gap _ _ _ _ _ _ = Refl
joinAssoc (Letter w) _ _ _ _ _ _ = Refl
joinAssoc TurnRef _ _ _ _ _ _ = Refl
joinAssoc Ability _ _ _ _ _ _ = Refl

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
                 | DamagePrevented

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

public export
data Quantity : Type where
  Range : Maybe Nat -> Maybe Nat -> Quantity

public export
data NonZeroQ : Quantity -> Type where
  UnboundedAbove : NonZeroQ (Range lo Nothing)
  MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))

public export
quantWellFormed : Quantity -> Bool
quantWellFormed (Range Nothing _) = True
quantWellFormed (Range (Just Z) _) = False
quantWellFormed (Range (Just (S n)) Nothing) = True
quantWellFormed (Range (Just (S n)) (Just hi)) = lte (S n) hi

public export
WellFormedQ : Quantity -> Type
WellFormedQ q = So (quantWellFormed q)

public export
boundedIncrease : Quantity -> Bool
boundedIncrease (Range _ Nothing) = False
boundedIncrease (Range _ (Just _)) = True

public export
BoundedIncrease : Quantity -> Type
BoundedIncrease q = So (boundedIncrease q)

public export
quantPlur : Quantity -> Plurality
quantPlur (Range _ (Just (S Z))) = OneOf
quantPlur (Range _ _) = ManyOf

public export
modesFit : Quantity -> Nat -> Bool
modesFit (Range Nothing Nothing) n = True
modesFit (Range Nothing (Just hi)) n = lte hi n
modesFit (Range (Just lo) Nothing) n = lte lo n
-- [CR#700.2]: a modal spell offers a choice among its modes, so a
-- headcount that fixes the whole list instructs nothing.
modesFit (Range (Just lo) (Just hi)) n = lte hi n && not (lo == hi && hi == n)

public export
ModesFit : Quantity -> Nat -> Type
ModesFit q n = So (modesFit q n)

public export
modalHead : Quantity -> Nat -> Bool
modalHead (Range Nothing Nothing) n = True
modalHead (Range Nothing (Just (S Z))) n = True
modalHead (Range Nothing (Just _)) n = False
modalHead (Range (Just (S Z)) Nothing) n = True
modalHead (Range (Just _) Nothing) n = False
modalHead (Range (Just lo) (Just hi)) n =
  (lo == hi && lte lo 3) || (lo == 1 && hi == n)

public export
ModalHead : Quantity -> Nat -> Type
ModalHead q n = So (modalHead q n)

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
ArrangementFits : LibPos -> Maybe Arrangement -> Type
ArrangementFits p o = So (arrangementFits p o)

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
OrdinalFits : LibPos -> Maybe Arrangement -> Maybe LibOrdinal -> Type
OrdinalFits p o n = So (ordinalFits p o n)

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

public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes k (MkBinding _ k' OneOf _ :: bs) =
  if k == k' then S (countOnes k bs) else countOnes k bs
countOnes k (_ :: bs) = countOnes k bs

public export
countOutcomes : OutcomeSort -> Bindings -> Nat
countOutcomes s [] = Z
countOutcomes s (MkBinding _ Outcome OneOf (OutcomeP s') :: bs) =
  if s == s' then S (countOutcomes s bs) else countOutcomes s bs
countOutcomes s (_ :: bs) = countOutcomes s bs

public export
countQuality : QualitySort -> Bindings -> Nat
countQuality q [] = Z
countQuality q (MkBinding _ k OneOf _ :: bs) =
  if Quality q == k then S (countQuality q bs) else countQuality q bs
countQuality q (_ :: bs) = countQuality q bs

public export
data ChoiceStands : Nat -> Type where
  ChoiceMade : ChoiceStands (S n)

public export
countLetter : LetterWord -> Bindings -> Nat
countLetter w [] = Z
countLetter w (MkBinding _ k OneOf _ :: bs) =
  if Letter w == k then S (countLetter w bs) else countLetter w bs
countLetter w (_ :: bs) = countLetter w bs

public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ :: bs) =
  if k == k' then S (countManys k bs) else countManys k bs
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
theRestOk bs = countGroups bs == 1 && not (countParts bs == Z)

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
  if k == k' then True else anyTargeted k bs
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
anyTargetedTy : Kind -> CardType -> Bindings -> Bool
anyTargetedTy k t [] = False
anyTargetedTy k t (b@(MkBinding TargetD k' _ _) :: bs) =
  if k == k' && anchorTyOk t (bindingTy b) then True else anyTargetedTy k t bs
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
searchableZone : Zone -> Bool
searchableZone Library = True
searchableZone Graveyard = True
searchableZone Hand = True
searchableZone Battlefield = False
searchableZone Exile = False
searchableZone Stack = False

public export
SearchableZone : Zone -> Type
SearchableZone z = So (searchableZone z)

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
notInLibrary (MkBinding _ _ _ (ObjectP _ (Just z) _ _)) = not (z == Library)
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
data JoinedPlayer = JoinAnyPlayer | JoinOpponent

public export
Eq JoinedPlayer where
  (==) JoinAnyPlayer JoinAnyPlayer = True
  (==) JoinAnyPlayer _ = False
  (==) JoinOpponent JoinOpponent = True
  (==) JoinOpponent _ = False

public export
data JoinedClass = JoinPermanent | JoinCreature | JoinPlaneswalker | JoinBattle

public export
Eq JoinedClass where
  (==) JoinPermanent JoinPermanent = True
  (==) JoinPermanent _ = False
  (==) JoinCreature JoinCreature = True
  (==) JoinCreature _ = False
  (==) JoinPlaneswalker JoinPlaneswalker = True
  (==) JoinPlaneswalker _ = False
  (==) JoinBattle JoinBattle = True
  (==) JoinBattle _ = False

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
CounterHolder : Maybe Zone -> Type
CounterHolder z = So (counterZone z)

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
KeywordParamless : Keyword -> Type
KeywordParamless k = So (keywordParamless k)

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
altRunWritten (Just [_]) = True
altRunWritten (Just _) = False

public export
AltRunWritten : Maybe ProducedRun -> Type
AltRunWritten alt = So (altRunWritten alt)

public export
data ColorFreedom = SameColor | EachColor

public export
LoyaltyStep : Nat -> Type
LoyaltyStep n = IsSucc n

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
  (==) Town Town = True
  (==) Town _ = False
  (==) Desert Desert = True
  (==) Desert _ = False
  (==) Pegasus Pegasus = True
  (==) Pegasus _ = False
  (==) Faerie Faerie = True
  (==) Faerie _ = False
  (==) Kraken Kraken = True
  (==) Kraken _ = False
  (==) Sphinx Sphinx = True
  (==) Sphinx _ = False
  (==) Werewolf Werewolf = True
  (==) Werewolf _ = False
  (==) Eldrazi Eldrazi = True
  (==) Eldrazi _ = False
  (==) Goat Goat = True
  (==) Goat _ = False
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
SpaceHosted : TypeSpace -> Maybe CardType -> Type
SpaceHosted sp ty = So (spaceHosted sp ty)

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
ascriptionOk t (Just s) = ascribesAsSubtype s && subtypeType s == t

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

public export
data Supertype = Legendary | Basic | Snow

public export
Eq Supertype where
  (==) Legendary Legendary = True
  (==) Legendary _ = False
  (==) Basic Basic = True
  (==) Basic _ = False
  (==) Snow Snow = True
  (==) Snow _ = False

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
Eq AttachWord where
  (==) Enchanted Enchanted = True
  (==) Enchanted _ = False
  (==) Equipped Equipped = True
  (==) Equipped _ = False
  (==) Fortified Fortified = True
  (==) Fortified _ = False

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
  -- [CR#308.2]: kindred subtypes are the same set as creature subtypes,
  -- so a creature subtype fits a Kindred-typed line too.
  (elem (subtypeType s) tys
     || (subtypeType s == Creature && elem Kindred tys))
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
typesOrdered : List CardType -> Bool
typesOrdered [] = True
typesOrdered (t :: []) = True
typesOrdered (t :: u :: ts) = lt (typeRank t) (typeRank u) &&
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
statusEventOk Flipped = False
statusEventOk Unflipped = False
statusEventOk FaceUp = True
statusEventOk FaceDown = False
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
RetentionOk : TypeLine -> Maybe CardType -> Type
RetentionOk tl ret = So (retentionOk tl ret)

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
RidersOk : List TokenRider -> Type
RidersOk rs = So (ridersOk rs)

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

