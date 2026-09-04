module Experimental.ProofsCounters

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "of the chosen creature type"
public export
okChosenCreatureType :
  Predicate [MkBinding AD (Quality (SubtypeQ Creature)) OneOf QualityP] Object
okChosenCreatureType = Macros.ofChosen (SubtypeQ Creature)

||| "of the chosen creature type"
public export
badChosenWrongSort : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => Macros.ofChosen (SubtypeQ Creature) {ok})
badChosenWrongSort Refl impossible

||| "a counter of that kind"
public export
badChosenCounterKindRead : Unspellable
  (Predicate [MkBinding AD (Quality CounterKindQ) OneOf QualityP] Object)
  (\read => Macros.ofChosen CounterKindQ {read})
badChosenCounterKindRead Oh impossible

||| "this creature"
public export
okAscribedSelf : Noun [] Object
okAscribedSelf = AsType Creature This Nothing

||| "this creature"
public export
badAscribedTarget : Unspellable (Noun [] Object) (\ok =>
  AsType Creature (Macros.target Macros.creature) Nothing {asc = ok})
badAscribedTarget Oh impossible

||| "creature you control that isn't attacking"
public export
okConsistentConjunction : Predicate [] Object
okConsistentConjunction =
  And [Macros.creature, HasPossessor ControllerAx You, Not Attacking]

||| "creature you control that you don't control"
public export
okControlSelfNegation : Predicate [] Object
okControlSelfNegation =
  And [HasPossessor ControllerAx You, Not (HasPossessor ControllerAx You)]

||| "attacking noncreature"
public export
badAttackingNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Not Macros.creature] {cf = ok})
badAttackingNoncreature Oh impossible

||| "Whenever this creature blocks a creature or becomes blocked by a
||| creature, that creature gets -1/-1 until end of turn."
public export
okAltHeaderAgreeingReadback : Ability
okAltHeaderAgreeingReadback =
  Triggered Whenever (Blocks Macros.thisCreature (Just (Macros.a Macros.creature)))
            [BecomesBlocked Macros.thisCreature
                            (Just (Macros.a Macros.creature))]
            Nothing [] Nothing Nothing Nothing
            (Macros.gets (Macros.That (TypeW Creature) OneOf) (PtDown (Lit 1))
                         (PtDown (Lit 1)) (Just Macros.untilEndOfTurn))

public export
badAltHeaderMixedReadback : Unspellable Ability (\ok =>
  Triggered Whenever (Blocks Macros.thisCreature Nothing)
            [BecomesBlocked Macros.thisCreature
                            (Just (Macros.a Macros.creature))]
            Nothing [] Nothing Nothing Nothing
            (Macros.gets (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (PtDown (Lit 1))
                         (PtDown (Lit 1)) {ok = Builtin.snd ok} (Just Macros.untilEndOfTurn)))
badAltHeaderMixedReadback (Refl, _) impossible

public export
badThreeArmHeaderReadback : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature)
            [ Blocks Macros.thisCreature Nothing
            , BecomesTarget Macros.thisCreature (Macros.a Macros.spell) ]
            Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (StatOf Power ((Macros.It OneOf) {ok = ok}))
                        (Macros.each Opponent)))
badThreeArmHeaderReadback Refl impossible

public export
badJoinedHeaderReadback : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing
            [ Macros.joinedHead Whenever
                (Dies (Macros.a Macros.creatureYouControl)) ]
            Nothing Nothing Nothing
            (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
                         ((Macros.It OneOf) {ok = ok})))
badJoinedHeaderReadback Refl impossible

||| "Remove a +1/+1 counter from target creature."
public export
okRemoveCounterFromTarget : Instruction []
okRemoveCounterFromTarget =
  RemoveCounters (Just (Macros.exactly 1))
                 (Just (PrintedKind Macros.plusOnePlusOne))
                 (Macros.target Macros.creature)

||| "Destroy target creature. Remove a +1/+1 counter from it."
public export
badRemoveCountersDead : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                RemoveCounters (Just (Macros.exactly 1)) (Just (PrintedKind Macros.plusOnePlusOne)) ((Macros.It OneOf)) {cm = ok}])
badRemoveCountersDead Oh impossible

||| "Move a counter from target creature onto this creature."
public export
okMoveCounterOntoThis : Instruction []
okMoveCounterOntoThis =
  MoveCounters (Lit 1) Nothing (Macros.target Macros.creature)
               Macros.thisCreature

||| "Move a counter from target creature onto it."
public export
badMoveCountersSelf : Unspellable (Instruction []) (\ok =>
  MoveCounters (Lit 1) Nothing (Macros.target Macros.creature) ((Macros.It OneOf)) {md = ok})
badMoveCountersSelf Oh impossible

||| "Create a 1/1 creature creature token."
public export
badTokenDuplicateType : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [] [Creature, Creature])
                          [] Nothing) {tc = ok})
badTokenDuplicateType Oh impossible

||| "Create a 1/1 white white Soldier creature token."
public export
badTokenDuplicateColor : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (Macros.creatureTok 1 1 [White, White] [creatureType "Soldier"]) {tc = ok})
badTokenDuplicateColor Oh impossible

||| "Target opponent loses 2 life. You gain that much life."
public export
okThatMuchAfterOutcome : Instruction []
okThatMuchAfterOutcome =
  Sequentially [Macros.losesLife (Macros.target Opponent) (Lit 2),
                Macros.gainsLife You ThatMuch]

||| "This deals 2 damage to target creature and you gain that much life."
public export
badSimultaneousReadsOutcome : Unspellable (Instruction []) (\ok =>
  Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsOutcome Refl impossible

||| "You may create a token and put a +1/+1 counter on it."
public export
badSimultaneousReadsMayDeed : Unspellable (Instruction []) (\ok =>
  Simultaneously [Macros.may You (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])),
                  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badSimultaneousReadsMayDeed Refl impossible

||| "You may have this deal 2 damage and you gain that much life."
public export
badSimultaneousReadsMayOutcome : Unspellable (Instruction []) (\ok =>
  Simultaneously [Macros.may You (DealDamage This (Lit 2) (Macros.target Macros.creature)),
                  Macros.gainsLife You (ThatMuch {ok})])
badSimultaneousReadsMayOutcome Refl impossible

||| "Create a Plant token and a Soldier token. Put a +1/+1 counter on it."
public export
badBatchTwoCreatesThenIt : Unspellable (Instruction []) (\ok =>
  Sequentially [Simultaneously [Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Plant"]),
                               Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badBatchTwoCreatesThenIt Refl impossible

public export
badBatchTwoOutcomesThenThatMuch : Unspellable (Instruction []) (\ok =>
  Sequentially [Simultaneously [DealDamage This (Lit 2) (Macros.target Macros.creature),
                               Macros.losesLife (Macros.target Opponent) (Lit 3)],
                Macros.gainsLife You (ThatMuch {ok})])
badBatchTwoOutcomesThenThatMuch Refl impossible

||| "Create a 1/1 green Plant creature token. Put a +1/+1 counter on it."
public export
okCreatedThenCountered : Instruction []
okCreatedThenCountered =
  Sequentially [Create You (Lit 1)
                       (TokenWritten
                          (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) [],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Macros.It OneOf)]

public export
badDistributedCreationIt : Unspellable (Instruction []) (\ok =>
  Sequentially [Create (Macros.each AnyPlayer) (Lit 1)
                       (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) [],
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badDistributedCreationIt Refl impossible

||| "Put a +1/+1 counter on each of up to two target creatures."
public export
okDistributedCounterRecipient : Instruction []
okDistributedCounterRecipient =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature))

||| "Put a +1/+1 counter on up to two target creatures."
public export
badBarePluralCounterRecipient : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) (Described (TargetDet (Macros.upTo 2)) Macros.creature) {pm = ok})
badBarePluralCounterRecipient Oh impossible

||| "This deals 1 damage to each of up to two target creatures."
public export
okDistributedDamageRecipient : Instruction []
okDistributedDamageRecipient =
  DealDamage This (Lit 1)
             (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature))

||| "This deals 1 damage to up to two target creatures."
public export
badBarePluralDamageRecipient : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 1) (Described (TargetDet (Macros.upTo 2)) Macros.creature) {pm = ok})
badBarePluralDamageRecipient Oh impossible

||| "Choose any number of target creatures. Put a +1/+1 counter on them."
public export
badThemCounterRecipient : Unspellable (Instruction []) (\ok =>
  Sequentially [Choose Nothing Nothing (Described (TargetDet Macros.anyNumber) Macros.creature) Openly,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It ManyOf)) {pm = ok}])
badThemCounterRecipient Oh impossible

||| "Exile target creature with a +1/+1 counter on it."
public export
okExileWithCounterRider : Instruction []
okExileWithCounterRider =
  Enact Nothing "Exile"
        (Move (Macros.target Macros.creature) Macros.exileZ
              [WithCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Fresh])

||| "Exile target creature tapped."
public export
badExileTapped : Unspellable (Instruction []) (\ok =>
  Enact Nothing "Exile" (Move (Macros.target Macros.creature) Macros.exileZ
                      [EntersTapped] {rf = ok}))
badExileTapped Oh impossible

||| "Put a +1/+1 counter on target creature."
public export
okPutBoostCounterOnCreature : Instruction []
okPutBoostCounterOnCreature =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (Macros.target Macros.creature)

||| "Put a poison counter on target creature."
public export
badPutPoisonOnCreature : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Poison")) (Macros.target Macros.creature) {sc = ok})
badPutPoisonOnCreature Oh impossible

||| "You get a +1/+1 counter."
public export
badGetsBoostCounter : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) You {sc = ok})
badGetsBoostCounter Oh impossible

||| "Each opponent loses all poison counters."
public export
okLosesAllPoisonCounters : Instruction []
okLosesAllPoisonCounters =
  LosesCounters (Macros.each Opponent) (Just (PrintedKind (NamedCounter "Poison")))
                Nothing

||| "Each opponent loses all +1/+1 counters."
public export
badLosesAllBoostCounters : Unspellable (Instruction []) (\ok =>
  LosesCounters (Macros.each Opponent) (Just (PrintedKind Macros.plusOnePlusOne)) Nothing
                   {sc = ok})
badLosesAllBoostCounters Oh impossible

||| "the number of +1/+1 counters on this creature"
public export
okCountersHeldByObject : Amount []
okCountersHeldByObject = CountersOn Macros.plusOnePlusOne Macros.thisCreature

||| "the number of +1/+1 counters you have"
public export
badCountersHeldByPlayer : Unspellable (Amount []) (\ok =>
  CountersOn Macros.plusOnePlusOne You {sc = ok})
badCountersHeldByPlayer Refl impossible

||| "When the last +1/+1 counter is removed from this creature, draw a card."
public export
okLastBoostCounterRemoved : Ability
okLastBoostCounterRemoved =
  Triggered When
            (Macros.lastCounterRemoved Macros.plusOnePlusOne
                                       Macros.thisCreature)
            [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1))

||| "When the last poison counter is removed from this creature, draw a card."
public export
badLastPoisonCounterRemoved : Unspellable Ability (\ok =>
  Triggered When (Macros.lastCounterRemoved (NamedCounter "Poison") Macros.thisCreature {sc = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badLastPoisonCounterRemoved Refl impossible

||| "Put a charge counter on this artifact."
public export
okChargeCounterOnArtifact : Instruction []
okChargeCounterOnArtifact =
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) Macros.thisArtifact

||| "Each player gets a charge counter."
public export
badGetsChargeCounter : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Charge")) You {sc = ok})
badGetsChargeCounter Oh impossible

||| "… that many plus one +1/+1 counters are put on it instead"
public export
okManyCounterBatchSize : StaticSpec []
okManyCounterBatchSize =
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) ManyCounters
                           Nothing False) [] Nothing
             (PutCounters (Plus ThatMuch (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) (Macros.It OneOf))
             Repeatedly Nothing

public export
badSingularCounterBatchSize : Unspellable (StaticSpec []) (\ok =>
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) OneCounter Nothing False) [] Nothing
             (PutCounters (Plus (ThatMuch {ok}) (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf)))
             Repeatedly Nothing)
badSingularCounterBatchSize Refl impossible

||| "Whenever one or more +1/+1 counters are put on a creature you control, …"
public export
okUncausedCounterWithAgent : GameEvent []
okUncausedCounterWithAgent =
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You)
               False

public export
badCausedCounterWithAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You) True {cz = ok})
badCausedCounterWithAgent Oh impossible

||| "each creature with a +1/+1 counter on it"
public export
okPlusOneCounterDescription : Predicate [] Object
okPlusOneCounterDescription =
  HasCounters (Just Macros.plusOnePlusOne) {kn = Present}

||| "each creature with a poison counter on it"
public export
badPoisonCounterDescription : Unspellable (Predicate [] Object) (\ok =>
  HasCounters (Just (NamedCounter "Poison")) {kn = Present {ok}})
badPoisonCounterDescription Refl impossible

||| "a first strike counter"
public export
okFirstStrikeKeywordCounter : CounterKind
okFirstStrikeKeywordCounter = KeywordCounter "FirstStrike"

||| "a cumulative upkeep counter"
public export
badCumulativeUpkeepCounter : Unspellable CounterKind (\ok =>
  KeywordCounter "CumulativeUpkeep" {ok = ok})
badCumulativeUpkeepCounter Oh impossible

||| "your choice of a +1/+1 counter or a first strike counter on it."
public export
okCounterMenu : StaticSpec []
okCounterMenu =
  EntersRider Macros.thisCreature
    (WithCounters (Lit 1)
       (ChosenKind [Macros.plusOnePlusOne, KeywordCounter "FirstStrike"])
       Fresh)

||| "This creature enters with your choice of a counter on it."
public export
badEmptyCounterMenu : Unspellable (StaticSpec []) (\ok =>
  EntersRider Macros.thisCreature (WithCounters (Lit 1) (ChosenKind [] {ne = ok}) Fresh))
badEmptyCounterMenu IsNonEmpty impossible

||| "Put your choice of a +1/+1 counter or a first strike counter on target
||| creature."
public export
okSameScopeCounterMenu : Instruction []
okSameScopeCounterMenu =
  PutCounters (Lit 1)
              (ChosenKind [Macros.plusOnePlusOne, KeywordCounter "FirstStrike"])
              (Macros.target Macros.creature)

||| "Put your choice of a +1/+1 counter or a poison counter on target creature."
public export
badMixedScopeCounterMenu : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (ChosenKind [Macros.plusOnePlusOne, NamedCounter "Poison"])
              (Macros.target Macros.creature) {sc = ok})
badMixedScopeCounterMenu Oh impossible

||| "Put a poison counter on target opponent."
public export
okPoisonCounterLabel : Instruction []
okPoisonCounterLabel =
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Poison")) (Macros.target Opponent)

||| "Put a zorp counter on target creature."
public export
badUnknownCounterLabel : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Zorp" {ok}))
              (Macros.target Macros.creature))
badUnknownCounterLabel Oh impossible

||| "Put a flying counter on target creature." [CR#122.1b]
public export
badKeywordCounterNamedPlainly : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (NamedCounter "Flying" {ok}))
              (Macros.target Macros.creature))
badKeywordCounterNamedPlainly Oh impossible

public export
afterCountersPut : Bindings
afterCountersPut =
  eventAfter (the (GameEvent [])
    (CounterEvent CounterPut Nothing
                  (Macros.a (And [Macros.creature, OtherThan This]))
                  ManyCounters (Just You) False))

||| "Put that many counters of each of those kinds on this creature."
public export
okThoseKindsAfterCountersPut : Instruction ProofsCounters.afterCountersPut
okThoseKindsAfterCountersPut = PutCounters ThatMuch ThoseKinds This

||| "Put a counter of each of those kinds on target creature."
public export
badThoseKindsUnannounced : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (ThoseKinds {ok}) (Macros.target Macros.creature))
badThoseKindsUnannounced Refl impossible

||| "If ... +1/+1 counters ... that many plus one are put instead."
public export
okInterceptsCounterEvent : StaticSpec []
okInterceptsCounterEvent =
  Intercepts (Macros.counterEvent CounterPut Macros.plusOnePlusOne ManyCounters
                                  (Macros.a Macros.creatureYouControl))
             [] Nothing
             (PutCounters (Plus ThatMuch (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) (Macros.It OneOf))
             Repeatedly Nothing

public export
badTriggeringReplaced : Unspellable (StaticSpec []) (\ok =>
  Intercepts (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                                      , AbilityOf (Macros.a (And [Permanent, HasPossessor ControllerAx You])) ])))
             [] Nothing (Draw You (Lit 1)) Repeatedly Nothing {ok})
badTriggeringReplaced Oh impossible

||| "Target creature becomes an artifact in addition to its other types."
public export
okBecomesArtifact : Instruction []
okBecomesArtifact =
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Artifact])
                 Nothing

||| "Target land becomes a Zombie in addition to its other types."
public export
badBecomesZombieLand : Unspellable (Instruction []) (\ok =>
  Macros.becomes (Macros.target Macros.land) (Macros.subtypesOnly [creatureType "Zombie"]) Nothing {ok = ok})
badBecomesZombieLand Oh impossible

||| "Target creature becomes in addition to its other types."
public export
badBecomesNothing : Unspellable (Instruction []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (MkTypeLine [] []) Nothing {ok = ok})
badBecomesNothing Oh impossible

||| "Target creature becomes a creature in addition to its other types."
public export
badBecomesOwnType : Unspellable (Instruction []) (\ok =>
  Macros.becomes (Macros.target Macros.creature) (Macros.typesOnly [Creature]) Nothing {ok = ok})
badBecomesOwnType Oh impossible

public export
badOtherwiseReadsIfArm : Unspellable (Instruction []) (\ok =>
  OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Macros.exists Macros.creatureYouControl)
     (Just (SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsIfArm (Refl, _) impossible

||| "Create a 1/1 black Zombie creature token. Create two of those tokens."
public export
okAnaphoricTokenAfterToken : Instruction []
okAnaphoricTokenAfterToken =
  Sequentially [ Macros.create (Lit 1)
                   (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"])
               , Create You (Lit 2) TokenAsThose [] ]

||| "Destroy target creature. Create two of those tokens."
public export
badAnaphoricTokenAfterNonToken : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Create You (Lit 2) (TokenAsThose {ok}) [] ])
badAnaphoricTokenAfterNonToken Refl impossible

||| "Put a +1/+1 counter on target creature card in your graveyard."
||| No printed card on the bench.
public export
counterOnGraveyardCard : Instruction []
counterOnGraveyardCard =
  PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne)
              (Macros.target (And [Macros.creature,
                                   InZone (Macros.graveyardOf You)]))

||| "put a counter of each kind that's on this creature on target creature"
public export
okSameKindsOnCreature : Instruction []
okSameKindsOnCreature =
  PutCounters (Lit 1) (SameAs Macros.thisCreature) (Macros.target Macros.creature)

||| "put a counter of each kind that's on this creature on target player":
||| a +X/+Y counter modifies an object's power and toughness [CR#122.1a], so
||| the kinds read off a creature stay on objects.
public export
badSameKindsOnPlayer : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 1) (SameAs Macros.thisCreature) (Macros.target AnyPlayer) {sc = ok})
badSameKindsOnPlayer Oh impossible

||| "put seven counters of each kind that's on this creature on target
||| creature": the same-counters reading carries the number as well as the
||| kinds, so the amount slot holds no other literal.
public export
badSameKindsSevenEach : Unspellable (Instruction []) (\ok =>
  PutCounters (Lit 7) (SameAs Macros.thisCreature) (Macros.target Macros.creature) {am = ok})
badSameKindsSevenEach Oh impossible
