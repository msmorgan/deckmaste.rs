module Experimental.Cards.Anaphora

import Experimental
import Experimental.Macros
import Experimental.Cards.Description

%default total


cloudshift : Instruction []
cloudshift = Sequentially [Macros.exile You (Macros.target Macros.creatureYouControl),
                           Macros.putOntoBattlefieldUnderYourControl (Macros.That CardW OneOf)]

bitterDownfall : Instruction []
bitterDownfall = Sequentially [Macros.destroy (Macros.target Macros.creature),
                               Macros.losesLife (Macros.controllerOf ((Macros.It OneOf))) (Lit 2)]

suspendedSentence : Instruction []
suspendedSentence = Sequentially [Macros.destroy (Macros.target (And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent])),
                                  Macros.losesLife (Macros.That PlayerW OneOf) (Lit 3)]

flickeringSpirit : Instruction []
flickeringSpirit = Sequentially [Macros.exile You Macros.thisCreature,
                                 Macros.move ((Macros.It OneOf)) Macros.battlefieldZ]

||| Bond of Revival
bondOfRevival : Instruction []
bondOfRevival = Sequentially [Macros.returnToBattlefield (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])),
                              Macros.gainsHaste (Macros.ItVerbed "Return" OneOf) (Just Macros.untilYourNextTurn)]

vraskasStoneglare : Instruction []
vraskasStoneglare = Sequentially [Macros.destroy (Macros.target Macros.creature),
                                  Macros.gainsLife You (StatOf Toughness ((Macros.It OneOf)))]

phthisis : Instruction []
phthisis = Sequentially [Macros.destroy (Macros.target Macros.creature),
                         Macros.losesLife (Macros.controllerOf ((Macros.It OneOf))) (Plus (StatOf Power ((Macros.It OneOf))) (StatOf Toughness ((Macros.It OneOf))))]

foulTongueShriek : Instruction []
foulTongueShriek = Sequentially [Macros.losesLife (Macros.target Opponent)
                                           (Macros.forEach 1 (And [Attacking, Macros.creature, HasPossessor ControllerAx You])),
                                 Macros.gainsLife You ThatMuch]

phyrexianInfiltrator : Instruction []
phyrexianInfiltrator =
  Simultaneously
    [Macros.gainControl (Macros.controllerOf (Macros.target Macros.creature)) Macros.thisCreature Nothing,
     Macros.gainControl (Macros.controllerOf Macros.thisCreature) (Macros.That (TypeW Creature) OneOf) Nothing]

impulse : Instruction []
impulse = Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
                       , Macros.move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ
                       , Macros.move (Macros.theRest Object) (Macros.onBottomIn AnyOrder)
                       ]

anticipate : Instruction []
anticipate = Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 3)))
                          , Macros.move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ
                          , Macros.move (Macros.theRest Object) (Macros.onBottomIn AnyOrder)
                          ]

revealFourPartition : Instruction []
revealFourPartition = Sequentially [ Macros.revealCards ((Macros.topSlice (Lit 4)))
                                   , Macros.move (Macros.someOf (Macros.exactly 1) (Macros.That CardW ManyOf)) Macros.handZ
                                   , Macros.move (Macros.theRest Object) Macros.graveyardZ
                                   ]

exileFourOfThem : Instruction []
exileFourOfThem = Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 8)))
                               , Macros.exile You (Macros.someOf (Macros.exactly 4) ((Macros.It ManyOf)))
                               , Macros.move (Macros.theRest Object) (Macros.onTopIn AnyOrder)
                               ]

sylvanScrying : Instruction []
sylvanScrying = Sequentially [ Macros.searchLibraryFor (Macros.exactly 1) Macros.land
                             , Macros.revealCards ((Macros.It OneOf))
                             , Macros.move ((Macros.It OneOf)) Macros.handZ
                             , Macros.shuffle
                             ]

searchToBattlefield : Instruction []
searchToBattlefield = Sequentially [ Macros.searchLibraryFor (Macros.exactly 1) Macros.creature
                                   , Macros.move ((Macros.It OneOf)) Macros.battlefieldZ
                                   , Macros.shuffle
                                   ]

glimpseTheUnthinkable : Instruction []
glimpseTheUnthinkable =
  Macros.mills (Macros.target AnyPlayer) (Lit 10) They

millThenReadGroup : Instruction []
millThenReadGroup =
  Sequentially [ Macros.mills You (Lit 3) You
               , Macros.exile You (Macros.That CardW ManyOf) ]

takeIntoCustody : Instruction []
takeIntoCustody = Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                                DoesntUntapNext ((Macros.It OneOf)) (Lit 1)]

||| Frenzied Gorespawn
public export
frenziedGorespawnGoad : Instruction []
frenziedGorespawnGoad =
  ForEachOf (Macros.each Opponent)
    (Macros.gainsDesignation
       (Macros.target (And [Macros.creature, HasPossessor ControllerAx (Macros.That PlayerW OneOf)]))
       Goaded Instructed)

spaceTimeAnomaly : Card
spaceTimeAnomaly =
  Macros.card "Space-Time Anomaly"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.mills (Macros.target AnyPlayer) (PlayerStatOf LifeTotal You) They) ]
       Nothing

||| Consecrate // Consume
consume : Ability
consume =
  Spell (Sequentially
           [ Macros.sacrifice (Macros.target AnyPlayer)
               (Macros.a (And [Macros.creature,
                               Superlative MaxOf (CharAxis Power)
                                 (And [Macros.creature, HasPossessor ControllerAx They])]))
           , Macros.gainsLife You (StatOf Power ((Macros.It OneOf))) ])

public export
bifurcate : Card
bifurcate =
  Macros.card "Bifurcate" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
              [ Macros.searchLibraryFor (Macros.exactly 1)
                  (And [Permanent, Named (SameNameAs
                          (Macros.target (And [Macros.creature, Macros.nontoken])))])
              , Macros.putOntoBattlefield (Macros.That CardW OneOf)
              , Macros.shuffle ]) ]
       Nothing

||| Mana Leak
public export
manaLeak : Card
manaLeak =
  Macros.card "Mana Leak" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Unless (CounterSpell (Macros.target Macros.spell))
                       (Macros.controllerOf ((Macros.It OneOf)))
                       (Mana [Macros.generic 3])) ]
       Nothing

public export
oust : Card
oust =
  Macros.card "Oust" (Just [Macros.pip White]) [] (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.move (Macros.target Macros.creature) (Macros.nthFromTop (Nth 2))
                  , Macros.gainsLife (Macros.controllerOf ((Macros.It OneOf))) (Lit 3) ]) ]
       Nothing

public export
riseFromTheGrave : Instruction []
riseFromTheGrave =
  Sequentially [Macros.putOntoBattlefieldUnderYourControl (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])),
                Macros.becomesAs (Macros.That (TypeW Creature) OneOf)
                                 (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                 Nothing]

public export
everAfter : Instruction []
everAfter =
  Sequentially [Macros.move (Described (TargetDet (Macros.upTo 2)) (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                            Macros.battlefieldZ,
                Macros.becomesAs (Macros.That (TypeW Creature) ManyOf)
                                 (MkToken Nothing [Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                                 Nothing,
                Macros.move This Macros.onBottomZ]

public export
martyrsCry : Instruction []
martyrsCry =
  Sequentially [Macros.exile You (Macros.allOf (And [Macros.creature, ColorIs White])),
                ForEachOf (Macros.TheVerbed "Exile" (TypeW Creature) Attributive ManyOf)
                          (Draw (Macros.controllerOf ((Macros.It OneOf))) (Lit 1))]

public export
anotherRound : Instruction []
anotherRound =
  Sequentially [ Macros.exile You (Macros.counted Macros.anyNumber
                                 Macros.creatureYouControl)
               , Macros.putOntoBattlefield ((Macros.It ManyOf))
               , Repeat (MoreTimes (LetterVal X)) ]

||| Eradicate
public export
eradicateSearch : Instruction []
eradicateSearch =
  Sequentially [ Macros.exile You (Macros.target (And [Macros.creature,
                                                   Not (ColorIs Black)]))
               , Macros.searchZonesOf (Macros.controllerOf (Macros.That CardW OneOf))
                                      (Named (SameNameAs (Macros.That CardW OneOf)))
               , Shuffle (Macros.That PlayerW OneOf) ]

||| Deem Inferior
public export
deemInferior : Instruction []
deemInferior =
  Macros.puts (Macros.ownerOf (Macros.target (And [Permanent, Not Macros.land])))
              ((Macros.It OneOf))
              (Macros.nthFromTopOrBottomZ (Nth 2))

||| Write into Being
public export
writeIntoBeingPlacement : Instruction []
writeIntoBeingPlacement =
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 2)))
               , Macros.exile You (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf)))
               , Macros.move (Macros.theRest Object) Macros.topOrBottomZ ]

||| Mystical Tutor
public export
mysticalTutor : Instruction []
mysticalTutor =
  Sequentially [ Macros.searchLibraryFor (Macros.exactly 1) Macros.instantOrSorcery
               , Macros.revealCards ((Macros.It OneOf))
               , Macros.shuffle
               , Macros.move ((Macros.It OneOf)) Macros.onTopZ ]

||| Demonic Tutor
public export
demonicTutor : Instruction []
demonicTutor =
  Sequentially [ Macros.searchLibraryFor (Macros.exactly 1) (And [])
               , Macros.move ((Macros.It OneOf)) Macros.handZ
               , Macros.shuffle ]

||| Thalia's Lancers
public export
thaliasLancersSearch : Instruction []
thaliasLancersSearch =
  Sequentially [ Macros.searchLibraryFor (Macros.exactly 1) (And [HasSupertype Legendary])
               , Macros.revealCards ((Macros.It OneOf))
               , Macros.move ((Macros.It OneOf)) Macros.handZ
               , Macros.shuffle ]

contrabandLivestock : Instruction []
contrabandLivestock =
  Sequentially
    [Macros.exile You (Macros.target Macros.creature),
     (Macros.rollDice You 1 20),
     Macros.resultsTable
       [Macros.rollRow (Macros.fromTo 1 9)
          (Create (Macros.controllerOf ((Macros.It OneOf))) (Lit 1)
                  (TokenWritten (Macros.creatureTok 4 4 [Green] [creatureType "Ox"])) []),
        Macros.rollRow (Macros.fromTo 10 19)
          (Create (Macros.controllerOf ((Macros.It OneOf))) (Lit 1)
                  (TokenWritten (Macros.creatureTok 2 2 [Green] [creatureType "Boar"])) []),
        Macros.rollRow (Macros.exactly 20)
          (Create (Macros.controllerOf ((Macros.It OneOf))) (Lit 1)
                  (TokenWritten (Macros.creatureTok 0 1 [White] [creatureType "Goat"])) [])]]

||| Hypnotic Specter
hypnoticSpecterDiscard : Instruction [MkBinding TheD Player OneOf PlayerP]
hypnoticSpecterDiscard = (Macros.discard They (Macros.aAtRandom (InZone Macros.handZ)))

||| Wyll, Blade of Frontiers
public export
wyllExtraDie : Instruction []
wyllExtraDie =
  Macros.ifWouldInstead (RollsDice You ManyDice AnyDie AnyResult)
    (Sequentially [ RollDice You (Plus ThatMuch (Lit 1)) ThoseDice
                  , IgnoreOutcomes (IgnoreExtreme LowestRoll) ])
    Nothing

||| Ichor Elixir
public export
ichorElixirPlanarDice : Instruction []
ichorElixirPlanarDice =
  Macros.ifWouldInstead Macros.youRollPlanarDice
    (Sequentially [ RollPlanarDie You (Plus ThatMuch (Lit 1))
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Vedalken Squirrel-Whacker
public export
vedalkenSquirrelWhackerReroll : Instruction []
vedalkenSquirrelWhackerReroll =
  Macros.ifWouldInstead (RollsDice You ManyDice (SidedDie 6) AnyResult)
    (RollDice You ThatMuch ThoseDice)
    Nothing

||| Investigator's Journal's count
public export
greatestCreaturesAPlayerControls : Amount []
greatestCreaturesAPlayerControls =
  AggregateOver MaxOf AnyPlayer
    (Macros.countOf (And [Macros.creature, HasPossessor ControllerAx They]))

||| Cavern-Hoard Dragon
public export
greatestArtifactsAnOpponentControls : Amount []
greatestArtifactsAnOpponentControls =
  AggregateOver MaxOf Opponent
    (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx They]))

public export
eachCreatureThatSplitControls :
  {bs : Bindings} ->
  {auto 0 ck : countReach (UnionHalf (TypeW Planeswalker)) OneOf bs = 1} ->
  {auto 0 pk : countReach (UnionHalf PlayerW) OneOf bs = 1} ->
  Noun bs Object
eachCreatureThatSplitControls =
  Macros.each (And [Macros.creature, HasPossessor ControllerAx (Macros.splitOverPlaneswalker {ck} {pk})])

public export
anyTargetAnnounced : Noun [] (Object \/ Player)
anyTargetAnnounced = Macros.target Macros.anyTarget

public export
playerOrPlaneswalkerAnnounced : Noun [] (Object \/ Player)
playerOrPlaneswalkerAnnounced = Description.targetPlayerOrPlaneswalker

||| Chain of Plasma
public export
chainOfPlasmaOfferee : Noun (nomIntro Anaphora.anyTargetAnnounced) Player
chainOfPlasmaOfferee = Macros.splitOverPermanent

||| Chain Lightning
public export
chainLightningPayer : Noun (nomIntro Anaphora.anyTargetAnnounced) Player
chainLightningPayer = Macros.splitOverPermanent

||| Flames of the Blood Hand
public export
flamesOfTheBloodHandSubject :
  Noun (nomIntro Anaphora.playerOrPlaneswalkerAnnounced) Player
flamesOfTheBloodHandSubject = Macros.splitOverPlaneswalker

||| Flaming Gambit
public export
flamingGambitOfferee :
  Noun (nomIntro Anaphora.playerOrPlaneswalkerAnnounced) Player
flamingGambitOfferee = Macros.splitOverPlaneswalker

||| Ertai's Trickery
public export
ertaisTrickery : Card
ertaisTrickery =
  Macros.card "Ertai's Trickery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (OnlyIf (CounterSpell (Macros.target Macros.spell))
                       (Macros.costWasPaid (ByKeyword "Kicker") Nothing (Macros.It OneOf)) Nothing) ]
       Nothing

||| Celebrate the Harvest
public export
celebrateTheHarvest : Card
celebrateTheHarvest =
  Macros.card "Celebrate the Harvest"
       (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.searchLibraryFor (UpToOf (LetterVal X))
                      (And [Macros.land, HasSupertype Basic])
                  , Define X (DistinctCount (ValueAxis Power)
                                (Macros.allOf Macros.creatureYouControl))
                  , Macros.putOntoBattlefieldTapped (Macros.That CardW ManyOf)
                  , Macros.shuffle ]) ]
       Nothing

||| Boreas Charger's description
public export
opponentWithMoreLands : Predicate [] Player
opponentWithMoreLands =
  CompareOver Opponent
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx They]))
    Greater
    (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))

public export
bioplasmCardTest : Condition Description.bioplasmAfterExile
bioplasmCardTest = Macros.itsACard Macros.creature

||| Blessed Respite
public export
blessedRespiteShuffle : Instruction []
blessedRespiteShuffle =
  Macros.shuffleInto (Macros.target AnyPlayer)
    (Macros.allOf (InZone (Macros.graveyardOf They)))

||| Nekrataal
public export
nekrataalEntry : GameEvent []
nekrataalEntry = Enters Macros.thisCreature Nothing

public export
nekrataalDestroy : Instruction (eventAfter Anaphora.nekrataalEntry)
nekrataalDestroy =
  Macros.destroy (Macros.target
     (And [Macros.creature, Not Macros.artifact, Not (ColorIs Black)]))

public export
nekrataalRider : Bindings
nekrataalRider = riderIntro Anaphora.nekrataalDestroy

public export
nekrataalOneCreatureWord : countReach (Word (TypeW Creature)) OneOf Anaphora.nekrataalRider = 1
nekrataalOneCreatureWord = Refl

public export
nekrataalOneDestroyed : countReach (Stamped "Destroy") OneOf Anaphora.nekrataalRider = 1
nekrataalOneDestroyed = Refl

public export
sequencedRiderBody : Instruction []
sequencedRiderBody =
  Sequentially [ Macros.exile You (Macros.target Macros.artifact)
               , Macros.destroy (Macros.target Macros.creature) ]

public export
sequencedRider : Bindings
sequencedRider = riderIntro Anaphora.sequencedRiderBody

public export
sequencedRiderOneDestroyed : countReach (Stamped "Destroy") OneOf Anaphora.sequencedRider = 1
sequencedRiderOneDestroyed = Refl

public export
sequencedRiderOneExiled : countReach (Stamped "Exile") OneOf Anaphora.sequencedRider = 1
sequencedRiderOneExiled = Refl

||| Engulfing Flames
public export
engulfingFlamesBody : Instruction []
engulfingFlamesBody = DealDamage This (Lit 1) (Macros.target Macros.creature)

public export
engulfingFlamesRider : Bindings
engulfingFlamesRider = riderIntro Anaphora.engulfingFlamesBody

public export
engulfingFlamesNoDestroyStamp :
  countReach (Stamped "Destroy") OneOf Anaphora.engulfingFlamesRider = 0
engulfingFlamesNoDestroyStamp = Refl

public export
engulfingFlamesBareReadStands :
  countReach Bare OneOf Anaphora.engulfingFlamesRider = 1
engulfingFlamesBareReadStands = Refl

public export
bioplasmExiledPronoun : Noun Description.bioplasmAfterExile Object
bioplasmExiledPronoun = Macros.ItVerbed "Exile" OneOf

public export
bioplasmNoTypedRead :
  countReach (Verbed "Exile" (TypeW Creature) Attributive) OneOf
    Description.bioplasmAfterExile = 0
bioplasmNoTypedRead = Refl

public export
bioplasmExiledCardHasNoType :
  tyOfReach (Stamped "Exile") OneOf Description.bioplasmAfterExile = Nothing
bioplasmExiledCardHasNoType = Refl

public export
bioplasmAfterTest : Bindings
bioplasmAfterTest = condIntro Anaphora.bioplasmCardTest

public export
bioplasmTestRemarksType :
  tyOfReach (Stamped "Exile") OneOf Anaphora.bioplasmAfterTest = Just Creature
bioplasmTestRemarksType = Refl

public export
bioplasmTestKeepsCardSlot :
  countReach (AtSlot CardSlot) OneOf Anaphora.bioplasmAfterTest = 1
bioplasmTestKeepsCardSlot = Refl

public export
bioplasmTypedReadStillRefused :
  countReach (Verbed "Exile" (TypeW Creature) Attributive) OneOf
    Anaphora.bioplasmAfterTest = 0
bioplasmTypedReadStillRefused = Refl

public export
bioplasmTypedCardReadWrites :
  countReach (Verbed "Exile" (TypedCardW Creature) Attributive) OneOf
    Anaphora.bioplasmAfterTest = 1
bioplasmTypedCardReadWrites = Refl

||| Scapeshift
public export
scapeshiftSacrifice : Instruction []
scapeshiftSacrifice =
  Macros.sacrifice You (Macros.counted Macros.anyNumber Macros.land)

public export
scapeshiftSacrificed : Bindings
scapeshiftSacrificed = instrIntro Anaphora.scapeshiftSacrifice

public export
scapeshiftSearch : Instruction Anaphora.scapeshiftSacrificed
scapeshiftSearch = Macros.searchLibraryFor (UpToOf GroupSize) Macros.land

public export
scapeshiftAfterSearch : Bindings
scapeshiftAfterSearch = instrIntro Anaphora.scapeshiftSearch

public export
scapeshiftTwoGroups : countReach Bare ManyOf Anaphora.scapeshiftAfterSearch = 2
scapeshiftTwoGroups = Refl

public export
scapeshiftOneSearchedGroup :
  countReach (Stamped "Search") ManyOf Anaphora.scapeshiftAfterSearch = 1
scapeshiftOneSearchedGroup = Refl

public export
scapeshiftOneSacrificedGroup :
  countReach (Stamped "Sacrifice") ManyOf Anaphora.scapeshiftAfterSearch = 1
scapeshiftOneSacrificedGroup = Refl

||| Scapeshift
public export
scapeshift : Card
scapeshift =
  Macros.card "Scapeshift"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.sacrifice You (Macros.counted Macros.anyNumber
                                                       Macros.land)
                  , Macros.searchLibraryFor (UpToOf GroupSize) Macros.land
                  , Macros.putOntoBattlefieldTapped (Macros.ItVerbed "Search" ManyOf)
                  , Macros.shuffle ]) ]
       Nothing

||| Bind to Life, Vastlands Scavenger's adventure
public export
bindToLife : Instruction []
bindToLife =
  Sequentially [ Macros.mills You (Lit 7) You
               , Macros.move (Macros.fromAmong (Macros.exactly 1) Macros.creature ((Macros.It ManyOf)))
                             Macros.battlefieldZ ]

||| Glamdring, Foe-hammer's Gleam of Death
public export
gleamOfDeath : Instruction []
gleamOfDeath =
  Sequentially [ Macros.mills You (Lit 6) You
               , Macros.move (Macros.allFromAmong
                                (Or [Macros.instant, Macros.sorcery]) ((Macros.It ManyOf)))
                             Macros.handZ ]

||| Tezzeret, Master of the Bridge
public export
tezzeretAllArtifacts : Instruction []
tezzeretAllArtifacts =
  Sequentially [ Macros.mills You (Lit 6) You
               , Macros.move (Macros.allFromAmong Macros.artifact ((Macros.It ManyOf)))
                             Macros.battlefieldZ ]

public export
companyContext : Bindings
companyContext = Macros.lookedTop [] (Lit 6)

public export
companyDescribedSlice : Noun Anaphora.companyContext Object
companyDescribedSlice =
  Macros.fromAmong (Macros.upTo 2)
                   (And [Macros.creature, Compare [CharAxis ManaValue] AtMost (Lit 3)])
                   ((Macros.It ManyOf))

public export
companyBareSlice : Noun Anaphora.companyContext Object
companyBareSlice = Macros.someOf (Macros.exactly 2) ((Macros.It ManyOf))

||| Lord of the Void
public export
exileTopThenPutFromAmong : Instruction []
exileTopThenPutFromAmong =
  Sequentially [ Macros.exile You ((Macros.topSlice (Lit 7)))
               , Macros.move (Macros.fromAmong (Macros.exactly 1) Macros.creature ((Macros.It ManyOf)))
                             Macros.battlefieldZ ]

||| Thought Sponge
public export
greatestCardsAnOpponentDrew : Amount []
greatestCardsAnOpponentDrew =
  AggregateOver MaxOf Opponent (Macros.eventCount CardDrawn They ThisTurn)

public export
greatestCardsAPlayerDiscardedThisWay : Amount []
greatestCardsAPlayerDiscardedThisWay =
  AggregateOver MaxOf AnyPlayer
    (Macros.eventCountInvolving (VerbedAct "Discard") They ThisWay
       Macros.everyObject)

public export
eachPlayerBase : Noun [] Player
eachPlayerBase = Macros.each AnyPlayer

public export
eachPlayerBindsNoSingular : countOnes Player (nomIntro Anaphora.eachPlayerBase) = 0
eachPlayerBindsNoSingular = Refl

public export
eachPlayerBindsAGroup : countManys Player (nomIntro Anaphora.eachPlayerBase) = 1
eachPlayerBindsAGroup = Refl

||| Soul Ransom
public export
soulRansomRansom : Instruction []
soulRansomRansom =
  Sequentially [ Macros.sacrificeIt (Macros.controllerOf Macros.thisAura)
               , Draw They (Lit 2) ]

public export
thisAurasController : Noun [] Player
thisAurasController = Macros.controllerOf Macros.thisAura

public export
targetCreaturesController : Noun [] Player
targetCreaturesController = Macros.controllerOf (Macros.target Macros.creature)

public export
possessiveDeicticIsReadableByIt :
  countReach (AtSlot PermanentSlot) OneOf
    (nomIntro Anaphora.thisAurasController) = 1
possessiveDeicticIsReadableByIt = Refl

public export
possessiveDeicticIsNotADemonstrative :
  countReach (Word (TypeW Enchantment)) OneOf
    (nomIntro Anaphora.thisAurasController) = 0
possessiveDeicticIsNotADemonstrative = Refl

public export
possessiveDescribedBaseUnchanged :
  countReach (AtSlot PermanentSlot) OneOf
    (nomIntro Anaphora.targetCreaturesController) = 1
possessiveDescribedBaseUnchanged = Refl

||| Bile Blight
public export
bileBlight : Card
bileBlight =
  Macros.card "Bile Blight" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.gets
                  (Both (Macros.target Macros.creature)
                          (Macros.allOf (And [ Macros.creature
                                      , Named (SameNameAs (Macros.That (TypeW Creature) OneOf))
                                      , OtherThan (Macros.That (TypeW Creature) OneOf) ])))
                  (PtDown (Lit 3)) (PtDown (Lit 3))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Echoing Ruin
public export
echoingRuin : Card
echoingRuin =
  Macros.card "Echoing Ruin" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy
                  (Both (Macros.target Macros.artifact)
                          (Macros.allOf (And [ Macros.artifact
                                      , Named (SameNameAs (Macros.That (TypeW Artifact) OneOf))
                                      , OtherThan (Macros.That (TypeW Artifact) OneOf) ])))) ]
       Nothing

||| Hijack
public export
hijack : Card
hijack =
  Macros.card "Hijack" (Just [Macros.generic 1, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Sequentially
                  [ Macros.gainControl You (Macros.target (Or [Macros.artifact, Macros.creature]))
                                       (Just Macros.untilEndOfTurn)
                  , Macros.untap (Macros.ItVerbed "GainControl" OneOf)
                  , Macros.gainsHaste (Macros.ItVerbed "Untap" OneOf) (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Open the Vaults
public export
openTheVaults : Instruction []
openTheVaults =
  Macros.returnTo (Macros.allOf (And [IsCard,
                       Or [Macros.artifact, Macros.enchantment],
                       InZone (Macros.graveyardOf (PlayerGroup AllPlayers))]))
                  Macros.battlefieldZ
                  [Under (PossessorsOf OwnerAx ((Macros.It ManyOf)))]

||| Codecracker Hound
public export
codecrackerHoundLook : Instruction []
codecrackerHoundLook =
  Sequentially
    [ Macros.lookAt (Macros.topSlice (Lit 2))
    , Macros.move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ
    , Macros.move (Macros.theOther Object) Macros.graveyardZ ]

public export
oneOfTheTopTwo : Noun [] Object
oneOfTheTopTwo = Macros.someOf (Macros.exactly 1) (LibrarySlice OnTop (Lit 2) You)

public export
oneOfAnUncountedTop : Noun [] Object
oneOfAnUncountedTop =
  Macros.someOf (Macros.exactly 1)
                (LibrarySlice OnTop (Macros.countOf Macros.creature) You)

public export
theArtifactsAmongTheTopFive : Noun [] Object
theArtifactsAmongTheTopFive =
  Macros.allAmong Macros.artifact (LibrarySlice OnTop (Lit 5) You)

public export
theOtherAfterATwoCardLook :
  theOtherOk Object (nomIntro Anaphora.oneOfTheTopTwo) = True
theOtherAfterATwoCardLook = Refl

public export
theOtherNeedsAStatedCount :
  theOtherOk Object (nomIntro Anaphora.oneOfAnUncountedTop) = False
theOtherNeedsAStatedCount = Refl

public export
theRestStandsWhereTheOtherRefuses :
  theRestOk Object (nomIntro Anaphora.oneOfAnUncountedTop) = True
theRestStandsWhereTheOtherRefuses = Refl

public export
theOtherRefusesTheUniversalSlice :
  (theOtherOk Object (nomIntro Anaphora.theArtifactsAmongTheTopFive),
   theRestOk Object (nomIntro Anaphora.theArtifactsAmongTheTopFive))
    = (False, True)
theOtherRefusesTheUniversalSlice = Refl

public export
eachPlayerShufflesTheirHandAndGraveyard : Instruction []
eachPlayerShufflesTheirHandAndGraveyard =
  Macros.shuffleInto (Macros.each AnyPlayer)
    (Both (Macros.allOf (InZone (Macros.handOf They)))
            (Macros.allOf (InZone (Macros.graveyardOf They))))

||| Fact or Fiction
public export
factOrFiction : Card
factOrFiction =
  Macros.card "Fact or Fiction" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent ((Macros.It ManyOf)) 2 []
                  , Macros.move Macros.onePile Macros.handZ
                  , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       Nothing

unsummon : Instruction []
unsummon = Macros.move (Macros.target Macros.creature) Macros.handZ

grismold : Instruction []
grismold = Create (Macros.each AnyPlayer) (Lit 1)
                  (TokenWritten (Macros.creatureTok 1 1 [Green] [creatureType "Plant"])) []

||| Grenzo, Dungeon Warden
grenzoBottomCard : Instruction []
grenzoBottomCard = Macros.move Macros.bottomCard Macros.graveyardZ

public export
chronostutter : Card
chronostutter =
  Macros.card "Chronostutter" (Just [Macros.generic 5, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell (Macros.move (Macros.target Macros.creature) (Macros.nthFromTop (Nth 2))) ]
       Nothing

public export
nekrataalTwoObjects : countOnes Object Anaphora.nekrataalRider = 2
nekrataalTwoObjects = Refl

public export
bioplasmTestMintsNothing : countOnes Object Anaphora.bioplasmAfterTest = 2
bioplasmTestMintsNothing = Refl

public export
wholeSliceAtBase : SliceCount []
wholeSliceAtBase = WholeSlice

public export
wholeSliceIsPluralAndUncounted :
  (nounPlur Anaphora.theArtifactsAmongTheTopFive,
   sliceExact Anaphora.wholeSliceAtBase) = (ManyOf, Nothing)
wholeSliceIsPluralAndUncounted = Refl

public export
describedSliceReadsAsCreature : nounTy Anaphora.companyDescribedSlice = Just Creature
describedSliceReadsAsCreature = Refl

public export
bareSliceReadsUntyped : nounTy Anaphora.companyBareSlice = Nothing
bareSliceReadsUntyped = Refl

public export
describedSliceKeepsGroupZone :
  nounZone Anaphora.companyDescribedSlice = nounZone Anaphora.companyBareSlice
describedSliceKeepsGroupZone = Refl

||| Stomp and Howl
public export
stompAndHowl : Card
stompAndHowl =
  Macros.card "Stomp and Howl" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.destroy (Both (Macros.target Macros.artifact)
                                       (Macros.target Macros.enchantment))) ]
       Nothing

||| Churning Eddy
public export
churningEddy : Card
churningEddy =
  Macros.card "Churning Eddy" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.returnTo (Both (Macros.target Macros.creature)
                                        (Macros.target Macros.land))
                                Macros.handZ []) ]
       Nothing

||| Secret Rendezvous
public export
secretRendezvous : Card
secretRendezvous =
  Macros.card "Secret Rendezvous"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Draw (EachOf (Both You (Macros.target Opponent))) (Lit 3)) ]
       Nothing

||| Aggressive Instinct
public export
aggressiveInstinct : Card
aggressiveInstinct =
  Macros.card "Aggressive Instinct" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell (Macros.dealsDamageOwnPower (Macros.target Macros.creatureYouControl)
                                           (Macros.target Macros.creatureYouDontControl)) ]
       Nothing

||| Arcum Dagsson
public export
arcumDagssonSacrifice : Instruction []
arcumDagssonSacrifice =
  ControllerSacrifices (Macros.target (And [Macros.artifact, Macros.creature]))
