module Experimental.Cards.Static

import Experimental
import Experimental.Macros
import Experimental.Cards.Choice

%default total


forkedBolt : Instruction []
forkedBolt = Macros.dealsDivided This (Lit 2) (Described (TargetDet (Macros.oneThrough 2)) Macros.anyTarget)

thoughtReflection : Ability
thoughtReflection =
  Static (Intercepts (Draws You) [] Nothing (Draw You (Lit 2)) Repeatedly Nothing)

jorKadeen : Ability
jorKadeen =
  Static (Macros.onlyWhile (Macros.getsPt (Macros.allOf Macros.creatureYouControl) (Up (Lit 3)) (Up (Lit 0)))
                   (CompareAmt (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                        AtLeast (Lit 3)))

abandonedOutpost : Ability
abandonedOutpost = Static (Macros.entersTapped Macros.thisLand)

||| Time Vault
timeVaultLock : Ability
timeVaultLock = Static (Macros.doesntUntap Macros.thisArtifact (Just You))

hymnOfRebirth : Instruction []
hymnOfRebirth =
  Macros.putOntoBattlefieldUnderYourControl
    (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))

counterspell : Instruction []
counterspell = CounterSpell (Macros.target Macros.spell)

anthemOfChampions : Card
anthemOfChampions =
  Macros.card "Anthem of Champions" (Just [Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [Static (Macros.getsPt (Macros.allOf Macros.creatureYouControl) (Up (Lit 1)) (Up (Lit 1)))] Nothing

adantoVanguard : Ability
adantoVanguard =
  Static (Macros.onlyWhile (Macros.getsPt Macros.thisCreature (Up (Lit 2)) (Up (Lit 0)))
                          (Matches Macros.thisCreature Attacking))

||| Nowhere to Run
public export
nowhereToRunTargetLine : StaticSpec []
nowhereToRunTargetLine =
  Macros.canBeTargetedAsThough (Macros.allOf Macros.creatureYourOpponentsControl)
    (Macros.allOf (Or [Macros.spell, AbilityHead AnyOnStack]))
    (Not (HasKeyword (TheKeyword "Hexproof")))

||| Hithlain Rope
public export
hithlainRopeSacrificeLock : StaticSpec []
hithlainRopeSacrificeLock = Macros.objectCant "Sacrifice" This


||| Lich's Mastery
lichsMasteryGate : Ability
lichsMasteryGate = Static (Macros.playerCant "LoseGame" You)

theGoldenThrone : Ability
theGoldenThrone =
  Static (Intercepts (LosesGame You) [] Nothing
                     (Sequentially [Macros.exile Macros.thisArtifact,
                                    Macros.lifeBecomes You (Lit 1)])
                     Repeatedly Nothing)

stunningReversal : Ability
stunningReversal =
  Spell Nothing (Continuously (Intercepts (LosesGame You) [] Nothing
                                  (Sequentially [Draw You (Lit 7),
                                                 Macros.lifeBecomes You (Lit 1)])
                                  NextTimeOnly Nothing)
                      (Just ThisTurn))

pathOfBravery : Ability
pathOfBravery =
  Static (Macros.onlyWhile (Macros.getsPt (Macros.allOf Macros.creatureYouControl) (Up (Lit 1)) (Up (Lit 1)))
                          (CompareAmt (PlayerStatOf LifeTotal You) AtLeast
                                      (PlayerStatOf StartingLifeTotal You)))

deathsShadow : Card
deathsShadow =
  Macros.card "Death's Shadow" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Avatar"] [Creature])
       [ Static (AndAlso Nothing [ Modify Macros.thisCreature Power (Down (LetterVal X))
                                 , Modify Macros.thisCreature Toughness (Down (LetterVal X))
                         , DefinesLetter X (PlayerStatOf LifeTotal You) ]) ]
       (Just (13, 13))

spontaneousMutation : Ability
spontaneousMutation =
  Static (AndAlso Nothing [ Modify (AttachHost Enchanted (TypeW Creature)) Power (Down (LetterVal X))
                          , Modify (AttachHost Enchanted (TypeW Creature)) Toughness (Down (Lit 0))
                  , DefinesLetter X (Macros.countOf (InZone (Macros.graveyardOf You))) ])

maro : Card
maro =
  Macros.cardOf "Maro" (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Static (DefinesPt Macros.thisCreature BothEach
                           (Macros.countOf (InZone (Macros.handOf You)))) ]
       (Just (PtBox PrintedStar PrintedStar))

peopleOfTheWoods : Card
peopleOfTheWoods =
  Macros.cardOf "People of the Woods" (Just [Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Static (DefinesPt Macros.thisCreature ToughnessAlone
                           (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You]))) ]
       (Just (PtBox (PrintedNum 1) PrintedStar))

scourgeOfTheSkyclaves : Ability
scourgeOfTheSkyclaves =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (Lit 20) (Macros.aggregate MaxOf (PlayerStatAxis LifeTotal) AnyPlayer)))

aettirAndPriwen : Ability
aettirAndPriwen =
  Static (AndAlso Nothing [ Modify (AttachHost Equipped (TypeW Creature)) Power (Set (LetterVal X))
                          , Modify (AttachHost Equipped (TypeW Creature)) Toughness (Set (LetterVal X))
                  , DefinesLetter X (PlayerStatOf LifeTotal You) ])

diminish : Card
diminish =
  Macros.card "Diminish" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously (Macros.getsBase (Macros.target Macros.creature) (Lit 1) (Lit 1))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

cycleOfLife : Instruction []
cycleOfLife =
  Continuously (Macros.getsBase (Macros.target (And [Macros.creature, Macros.castBy You])) (Lit 0) (Lit 1))
               (Just Macros.untilYourNextUpkeep)

aboutFace : Card
aboutFace =
  Macros.card "About Face" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously (SwitchesPt (Macros.target Macros.creature))
                             (Just Macros.untilEndOfTurn)) ]
       Nothing

roilingHorror : Ability
roilingHorror =
  Static (DefinesPt Macros.thisCreature BothEach
                    (Minus (PlayerStatOf LifeTotal You)
                           (PlayerStatOf LifeTotal
                              (Macros.a (And [Opponent,
                                              Superlative MaxOf
                                                (PlayerStatAxis LifeTotal)
                                                Opponent])))))

||| Katara, the Fearless
public export
kataraTheFearless : Card
kataraTheFearless =
  Macros.card "Katara, the Fearless"
       (Just [Macros.pip Green, Macros.pip White, Macros.pip Blue]) [Legendary]
       (MkTypeLine [ creatureType "Human", creatureType "Warrior"
                   , creatureType "Ally" ] [Creature])
       [ Static (TriggersAdditionally
                   (Triggers
                      (Macros.a (And [ AbilityHead AnyTriggered
                                     , AbilityOf (Macros.a
                                         (And [ HasSubtype (creatureType "Ally")
                                              , HasPossessor ControllerAx You ])) ])))
                   (Macros.exactly 1)) ]
       (Just (3, 3))

||| Rain of Gore
public export
rainOfGore : Card
rainOfGore =
  Macros.card "Rain of Gore" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Causes
                      (CausedBySource
                         (Macros.a (Or [Macros.spell, AbilityHead AnyOnStack])))
                      (LifeChanges (Macros.controllerOf ((Macros.It OneOf))) LifeGoesUp))
                   [] Nothing
                   (Macros.losesLife (Macros.That PlayerW OneOf) ThatMuch)
                   Repeatedly Nothing) ]
       Nothing

||| Master Chef
masterChefGrantedAbility : Ability
masterChefGrantedAbility =
  Static (Macros.entersWithAdditionalCounters Macros.thisCreature (Lit 1)
                                              Macros.plusOnePlusOne)


anointedProcession : Card
anointedProcession =
  Macros.card "Anointed Procession"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedByEffectUnder (Macros.counted (Macros.atLeast 1) IsToken)
                                                      You) [] Nothing
                   (Create You (Macros.times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing) ]
       Nothing

naturalAffinity : Card
naturalAffinity =
  Macros.card "Natural Affinity" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (Becomes (Macros.allOf Macros.land) Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                                     (MkTypeLine [] [Creature]) [] Nothing) (Just Land)))
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

turnToFrog : Card
turnToFrog =
  Macros.card "Turn to Frog" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (AndAlso Nothing [ LosesAllAbilities (Macros.target Macros.creature) Nothing
                           , Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken Nothing [Blue]
                                                  (MkTypeLine [creatureType "Frog"] []) [] Nothing) Nothing)
                           , Modify ((Macros.It OneOf)) Power (Set (Lit 1))
                           , Modify (Macros.itsOther ((Macros.It OneOf)) (Set (Lit 1))) Toughness (Set (Lit 1)) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

humility : Card
humility =
  Macros.card "Humility" (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso Nothing [ LosesAllAbilities (Macros.allOf Macros.creature) Nothing
                         , Modify ((Macros.It ManyOf)) Power (Set (Lit 1))
                         , Modify (Macros.itsOther ((Macros.It ManyOf)) (Set (Lit 1))) Toughness (Set (Lit 1)) ]) ]
       Nothing

||| Tahngarth, First Mate
public export
tahngarthAttacksThatJoin : Instruction (instrIntro Choice.tahngarthChoosesDefender)
tahngarthAttacksThatJoin =
  BecomesAttacking Macros.thisCreature (OneDefender Macros.thatJoin)

public export
mindlockOrb : Card
mindlockOrb =
  Macros.card "Mindlock Orb" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.playerCant "Search" (PlayerGroup AllPlayers)) ]
       Nothing

public export
silence : Card
silence =
  Macros.card "Silence" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (Macros.playerCant "Cast" (PlayerGroup YourOpponents))
                  (Just ThisTurn)) ]
       Nothing

public export
shadowOfDoubt : Card
shadowOfDoubt =
  Macros.card "Shadow of Doubt"
       (Just [Macros.hybridPip Blue Black, Macros.hybridPip Blue Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (Macros.playerCant "Search" (PlayerGroup AllPlayers))
                      (Just ThisTurn)
                  , (Draw You (Lit 1)) ]) ]
       Nothing

public export
omenMachineDraw : Ability
omenMachineDraw = Static (Macros.playerCant "Draw" (PlayerGroup AllPlayers))

public export
solfataraLandLock : Instruction []
solfataraLandLock =
  Continuously (Macros.playerCant "Play" (Macros.target AnyPlayer))
               (Just ThisTurn)

public export
bloodMoon : Card
bloodMoon =
  Macros.card "Blood Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing) Nothing)) ]
       Nothing

public export
magusOfTheMoon : Card
magusOfTheMoon =
  Macros.card "Magus of the Moon" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"])
                                   [] Nothing) Nothing)) ]
       (Just (2, 2))

public export
harbingerOfTheSeas : Card
harbingerOfTheSeas =
  Macros.card "Harbinger of the Seas"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Static (Becomes (Macros.allOf (And [Macros.land, Not (HasSupertype Basic)])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Island"])
                                   [] Nothing) Nothing)) ]
       (Just (2, 2))

public export
yavimayaCradleOfGrowth : Card
yavimayaCradleOfGrowth =
  Macros.card "Yavimaya, Cradle of Growth" Nothing [Legendary]
       (MkTypeLine [] [Land])
       [ Static (Becomes (Macros.allOf Macros.land) Adds (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Forest"]) [] Nothing) Nothing)) ]
       Nothing

public export
rallyTheRanks : Card
rallyTheRanks =
  Macros.card "Rally the Ranks" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   Macros.ofChosen (SubtypeQ Creature)]))
                      (Up (Lit 1)) (Up (Lit 1))) ]
       Nothing

public export
sharedTriumph : Card
sharedTriumph =
  Macros.card "Shared Triumph" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, Macros.ofChosen (SubtypeQ Creature)]))
                      (Up (Lit 1)) (Up (Lit 1))) ]
       Nothing

public export
hallOfTriumph : Card
hallOfTriumph =
  Macros.card "Hall of Triumph" (Just [Macros.generic 3]) [Legendary]
       (MkTypeLine [] [Artifact])
       [ Static (Macros.entersChoosing Macros.thisArtifact Color)
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                   Macros.ofChosen Color]))
                      (Up (Lit 1)) (Up (Lit 1))) ]
       Nothing

public export
engineeredPlague : Card
engineeredPlague =
  Macros.card "Engineered Plague"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.entersChoosing Macros.thisEnchantment (SubtypeQ Creature))
       , Static (Macros.getsPt (Macros.allOf (And [Macros.creature, Macros.ofChosen (SubtypeQ Creature)]))
                      (Down (Lit 1)) (Down (Lit 1))) ]
       Nothing

||| Volrath's Laboratory
public export
volrathsLaboratoryChoice : StaticSpec []
volrathsLaboratoryChoice =
  AndAlso Nothing [ Macros.entersChoosing Macros.thisArtifact Color
          , Macros.entersChoosing Macros.thisArtifact (SubtypeQ Creature) ]

||| Call to Arms
public export
callToArmsChoice : StaticSpec []
callToArmsChoice =
  AndAlso Nothing [ Macros.entersChoosing Macros.thisEnchantment Color
          , Macros.entersChoosingPlayer Macros.thisEnchantment
                                        (Just OpponentsOnly) ]

public export
encroachingMycosynth : Card
encroachingMycosynth =
  Macros.card "Encroaching Mycosynth" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [ Static (AlsoOffBattlefield
                   (Becomes (Macros.allOf (And [Permanent, Not Macros.land,
                                             HasPossessor ControllerAx You])) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) Nothing))) ]
       Nothing

||| Darkest Hour
public export
darkestHour : Card
darkestHour =
  Macros.card "Darkest Hour" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf Macros.creature) Sets (Colored (SomeColors [Black]))) ]
       Nothing

||| Thran Lens
public export
thranLens : Card
thranLens =
  Macros.card "Thran Lens" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Becomes (Macros.allOf Permanent) Sets (Colored (SomeColors []))) ]
       Nothing

||| Ghostflame Sliver
public export
ghostflameSliver : Card
ghostflameSliver =
  Macros.card "Ghostflame Sliver" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Static (Becomes (Macros.allOf (HasSubtype (creatureType "Sliver"))) Sets (Colored (SomeColors []))) ]
       (Just (2, 2))

||| Nightcreep
public export
nightcreep : Card
nightcreep =
  Macros.card "Nightcreep" (Just [Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (AndAlso Nothing [ Becomes (Macros.allOf Macros.creature) Sets (Colored (SomeColors [Black]))
                           , Becomes (Macros.allOf Macros.land) Sets (Bundle (MkToken Nothing []
                                               (Macros.basicLandLine [landType "Swamp"])
                                               [] Nothing) Nothing) ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Celestial Dawn
public export
celestialDawnAscriptions : AbilitySeq []
celestialDawnAscriptions =
  [ Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Plains"])
                              [] Nothing) Nothing))
  , Static (AlsoOffBattlefield
              (Becomes (Macros.allOf (And [Permanent, Not Macros.land, HasPossessor ControllerAx You])) Sets (Colored (SomeColors [White])))) ]

||| Transguild Courier
public export
transguildCourier : Card
transguildCourier =
  Macros.card "Transguild Courier" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (Becomes Macros.thisCreature Sets (Colored EveryColor)) ]
       (Just (3, 3))

||| Booby Trap
public export
boobyTrapNameChoice : StaticSpec []
boobyTrapNameChoice =
  Macros.entersChoosingFrom Macros.thisArtifact
                            CardName
                            (NameOfCard (Not (And [HasSupertype Basic,
                                                   Macros.land])))

public export
dovinsVeto : Card
dovinsVeto =
  Macros.card "Dovin's Veto" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.objectCant "Counter" This)
       , Spell Nothing (CounterSpell
                  (Macros.target (And [Macros.spell, Not Macros.creature]))) ]
       Nothing

public export
goblinSpy : Card
goblinSpy =
  Macros.card "Goblin Spy" (Just [Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin", creatureType "Rogue"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary) ]
       (Just (1, 1))

public export
futureSight : Card
futureSight =
  Macros.card "Future Sight"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static playLandsAndCastSpellsFromTop ]
       Nothing

public export
magusOfTheFuture : Card
magusOfTheFuture =
  Macros.card "Magus of the Future"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"] [Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static playLandsAndCastSpellsFromTop ]
       (Just (2, 3))

public export
telepathy : Card
telepathy =
  Macros.card "Telepathy" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup YourOpponents) WholeHand) ]
       Nothing

public export
assembleThePlayers : Card
assembleThePlayers =
  Macros.card "Assemble the Players"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility LookAt You TopOfLibrary)
       , Static castSmallCreatureFromTopOnceEachTurn ]
       Nothing

public export
prismaticOmen : Card
prismaticOmen =
  Macros.card "Prismatic Omen" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)) ]
       Nothing

public export
mistformUltimus : Card
mistformUltimus =
  Macros.card "Mistform Ultimus"
       (Just [Macros.generic 3, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Static (Becomes Macros.thisCreature Adds (EveryTypeOf CreatureSpace)) ]
       (Just (3, 3))

public export
volatileClaws : Card
volatileClaws =
  Macros.card "Volatile Claws"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
           (AndAlso Nothing
             [ Modify (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Power (Up (Lit 2))
             , Modify (Macros.itsOther (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) (Up (Lit 2))) Toughness (Up (Lit 0))
             , Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (EveryTypeOf CreatureSpace) ])
           (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Nameless Inversion
public export
namelessInversionBody : Instruction []
namelessInversionBody =
  Continuously (AndAlso Nothing [ Modify (Macros.target Macros.creature) Power (Up (Lit 3))
                                , Modify (Macros.itsOther (Macros.target Macros.creature) (Up (Lit 3))) Toughness (Down (Lit 3))
                        , Becomes ((Macros.It OneOf)) Loses (EveryTypeOf CreatureSpace) ])
               (Just Macros.untilEndOfTurn)

||| Ego Erasure
public export
egoErasureBody : Instruction []
egoErasureBody =
  Continuously (AndAlso Nothing [ Modify (Macros.allOf (And [Macros.creature,
                                            HasPossessor ControllerAx (Macros.target AnyPlayer)])) Power (Down (Lit 2))
                                , Modify (Macros.itsOther (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx (Macros.target AnyPlayer)])) (Down (Lit 2))) Toughness (Up (Lit 0))
                        , Becomes ((Macros.It ManyOf)) Loses (EveryTypeOf CreatureSpace) ])
               (Just Macros.untilEndOfTurn)

||| Lithoform Blight
public export
lithoformBlightLoss : StaticSpec []
lithoformBlightLoss =
  AndAlso Nothing [ Becomes (AttachHost Enchanted (TypeW Land)) Loses (EveryTypeOf LandSpace)
          , LosesAllAbilities ((Macros.It OneOf)) Nothing ]

||| Energybending
public export
energybending : Card
energybending =
  Macros.card "Energybending" (Just [Macros.generic 2]) []
       (MkTypeLine [spellType "Lesson"] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace))
                      (Just Macros.untilEndOfTurn)
                  , (Draw You (Lit 1)) ]) ]
       Nothing

||| Seedborn Muse
public export
seedbornMuse : Card
seedbornMuse =
  Macros.card "Seedborn Muse"
       (Just [Macros.generic 3, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Static (Macros.untapsDuring (Macros.allOf (HasPossessor ControllerAx You)) (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 4))

||| Unwinding Clock
public export
unwindingClock : Card
unwindingClock =
  Macros.card "Unwinding Clock" (Just [Macros.generic 4]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.untapsDuring (Macros.allOf (And [Macros.artifact, HasPossessor ControllerAx You]))
                                     (Just (Macros.each Macros.otherPlayer))) ]
       Nothing

||| Thousand Moons Infantry
public export
thousandMoonsInfantry : Card
thousandMoonsInfantry =
  Macros.card "Thousand Moons Infantry"
       (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Static (Macros.untapsDuring Macros.thisCreature (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 4))

||| Blatant Thievery
public export
blatantThievery : Card
blatantThievery =
  Macros.card "Blatant Thievery"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue,
              Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (ForEachOf (Macros.each Opponent)
                  (Continuously
                     (GainsControl You (Macros.target (HasPossessor ControllerAx (Macros.That PlayerW OneOf))))
                     Nothing)) ]
       Nothing

public export
cursedTotem : Card
cursedTotem =
  Macros.card "Cursed Totem" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.creature) ]))) ]
       Nothing

public export
nullRod : Card
nullRod =
  Macros.card "Null Rod" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.artifact) ]))) ]
       Nothing

public export
stonySilence : Card
stonySilence =
  Macros.card "Stony Silence" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf Macros.artifact) ]))) ]
       Nothing

public export
dampingMatrix : Card
dampingMatrix =
  Macros.card "Damping Matrix" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                       Macros.creature]))
                               , Not IsManaAbility ]))) ]
       Nothing

public export
crash : Card
crash =
  Macros.card "Crash" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.sacrifice You
                   (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")]))))))
       , Spell Nothing (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
moggSalvage : Card
moggSalvage =
  Macros.card "Mogg Salvage" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.onlyWhile
                   (AltCost This Nothing)
                   (AndCond
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Island"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Mountain"),
                                     HasPossessor ControllerAx You]) ]))
       , Spell Nothing (Macros.destroy (Macros.target Macros.artifact)) ]
       Nothing

public export
abolish : Card
abolish =
  Macros.card "Abolish" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.discard You
                   (Macros.a (And [HasSubtype (landType "Plains"), InZone Macros.handZ]))))))
       , Spell Nothing (Macros.destroy (Macros.target (Or [Macros.artifact,
                                                   Macros.enchantment]))) ]
       Nothing

public export
gush : Card
gush =
  Macros.card "Gush" (Just [Macros.generic 4, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.move
                   (Macros.counted (Macros.exactly 2)
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       HasPossessor ControllerAx You]))
                   Macros.handZ))))
       , Spell Nothing ((Draw You (Lit 2))) ]
       Nothing

public export
sunscour : Card
sunscour =
  Macros.card "Sunscour"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Static (AltCost This (Just (Do (Macros.exiles You
                   (Macros.counted (Macros.exactly 2)
                                 (And [ColorIs White,
                                       InZone (Macros.handOf You)]))))))
       , Spell Nothing (Macros.destroy (Macros.allOf Macros.creature)) ]
       Nothing

public export
massacre : Card
massacre =
  Macros.card "Massacre"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [] [Sorcery])
       [ Static (Macros.onlyWhile
                   (AltCost This Nothing)
                   (AndCond
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"),
                                     HasPossessor ControllerAx You]) ]))
       , Spell Nothing (Macros.gets (Macros.allOf Macros.creature)
                            (Down (Lit 2)) (Down (Lit 2))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
rouse : Card
rouse =
  Macros.card "Rouse" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.onlyWhile
                   (AltCost This (Just (Macros.payLife You 2)))
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Swamp"), HasPossessor ControllerAx You])))
       , Spell Nothing (Macros.gets (Macros.target Macros.creature)
                            (Up (Lit 2)) (Up (Lit 0))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

public export
thwart : Card
thwart =
  Macros.card "Thwart"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Static (AltCost This (Just (Do (Macros.move
                   (Macros.counted (Macros.exactly 3)
                                 (And [Macros.land, HasSubtype (landType "Island"),
                                       HasPossessor ControllerAx You]))
                   Macros.handZ))))
       , Spell Nothing (CounterSpell (Macros.target Macros.spell)) ]
       Nothing

public export
theLadyOfOtariaLine : StaticSpec []
theLadyOfOtariaLine =
  AltCost This (Just (Do (SetStatus Tapped
            (Macros.counted (Macros.exactly 3)
                          (And [Macros.creature, HasSubtype (creatureType "Dwarf"),
                                HasPossessor ControllerAx You, Macros.untapped])))))

||| Clergy of the Holy Nimbus
public export
clergyOfTheHolyNimbus : Ability
clergyOfTheHolyNimbus =
  Static (Intercepts (VerbedEvent Nothing "Destroy"
                                  (Just Macros.thisCreature) Nothing)
                     [] Nothing
                     (Regenerate ((Macros.It OneOf))) Repeatedly Nothing)

||| Rampant Frogantua
public export
rampantFrogantuaPump : Ability
rampantFrogantuaPump =
  Static (Macros.getsPt Macros.thisCreature
               (Up (Macros.forEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame])))
               (Up (Macros.forEach 10 (And [AnyPlayer, Macros.happenedTo GameLoss ThisGame]))))

||| Maskwood Nexus
public export
maskwoodNexusTypes : Ability
maskwoodNexusTypes =
  Static (AlsoOffBattlefield
            (Becomes (Macros.allOf Macros.creatureYouControl) Adds (EveryTypeOf CreatureSpace)))

||| Luxior, Giada's Gift
public export
luxiorEquippedPermanent : Ability
luxiorEquippedPermanent =
  Static (Becomes (AttachHost Equipped PermanentW) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Creature]) [] Nothing) Nothing))

||| Nahiri, the Unforgiving's compleated reminder
public export
nahiriCompleatedEntry : Ability
nahiriCompleatedEntry =
  Static (Macros.entersWithFewerCounters Macros.thisPlaneswalker (Lit 2)
                                         (NamedCounter "Loyalty"))

||| Nimble Mongoose
public export
nimbleMongoose : Ability
nimbleMongoose =
  Macros.abilityWord "threshold"
    (Static (Macros.onlyWhile (Macros.getsPt Macros.thisCreature (Up (Lit 2)) (Up (Lit 2)))
                              (CompareAmt (Macros.countOf (InZone (Macros.graveyardOf You)))
                                          AtLeast (Lit 7))))

||| Anax, Hardened in the Forge
public export
anaxPowerDefinition : Ability
anaxPowerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone (Devotion You (LitColor Red) Nothing))

||| Aspect of Wolf
public export
aspectOfWolf : Ability
aspectOfWolf =
  Static (AndAlso Nothing
    [ Modify (AttachHost Enchanted (TypeW Creature)) Power (Up (LetterVal X))
    , Modify (AttachHost Enchanted (TypeW Creature)) Toughness (Up (LetterVal Y))
    , DefinesLetter X (Half RoundDown
                 (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You])))
    , DefinesLetter Y (Half RoundUp
                 (Macros.countOf (And [HasSubtype (landType "Forest"), HasPossessor ControllerAx You]))) ])

||| Lhurgoyf
public export
lhurgoyfDefinition : Ability
lhurgoyfDefinition =
  Static (AndAlso Nothing
    [ DefinesPt Macros.thisCreature PowerAlone
        (Macros.countOf (And [Macros.creature, InZone Macros.graveyardZ]))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Invigorate
public export
invigorate : Card
invigorate =
  Macros.card "Invigorate"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Static (Macros.onlyWhile
                   (AltCost This (Just (Do (Macros.gainsLife Macros.anOpponent
                                                            (Lit 3)))))
                   (Macros.exists (And [Macros.land, HasSubtype (landType "Forest"),
                                 HasPossessor ControllerAx You])))
       , Spell Nothing (Macros.gets (Macros.target Macros.creature)
                            (Up (Lit 4)) (Up (Lit 4))
                            (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Deflecting Swat
public export
deflectingSwatCommanderAltCost : Ability
deflectingSwatCommanderAltCost =
  Static (Macros.onlyWhile
            (AltCost This Nothing)
            (Macros.exists (And [HasDesignation CommanderD Nothing, HasPossessor ControllerAx You])))

||| Fist of Suns
public export
fistOfSuns : Card
fistOfSuns =
  Macros.card "Fist of Suns" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (AltCost (Macros.allOf (And [Macros.spell, Macros.castBy You]))
                   (Just (Mana [Macros.pip White, Macros.pip Blue,
                                Macros.pip Black, Macros.pip Red,
                                Macros.pip Green]))) ]
       Nothing

||| Rooftop Storm
public export
rooftopStorm : Card
rooftopStorm =
  Macros.card "Rooftop Storm"
       (Just [Macros.generic 5, Macros.pip Blue]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AltCost (Macros.allOf (And [ HasSubtype (creatureType "Zombie")
                                     , Macros.creature, Macros.spell
                                     , Macros.castBy You ]))
                   (Just (Mana [Macros.generic 0]))) ]
       Nothing

||| Voltage Surge's declaration
public export
voltageSurgeAddedCost : Ability
voltageSurgeAddedCost =
  Static (AddedCost (Do (Macros.sacrifice You (Macros.a Macros.artifact))) True)

||| Tarmogoyf
public export
tarmogoyfDefinition : Ability
tarmogoyfDefinition =
  Static (AndAlso Nothing
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (Macros.allOf (InZone (Macros.graveyardOf (PlayerGroup AllPlayers)))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Consuming Blob's definition
public export
consumingBlobDefinition : Ability
consumingBlobDefinition =
  Static (AndAlso Nothing
    [ DefinesPt Macros.thisCreature PowerAlone
        (DistinctCount CardTypeAxis
           (Macros.allOf (InZone (Macros.graveyardOf You))))
    , DefinesPt Macros.thisCreature ToughnessAlone
        (Plus ThatMuch (Lit 1)) ])

||| Nighthawk Scavenger's definition
public export
nighthawkScavengerDefinition : Ability
nighthawkScavengerDefinition =
  Static (DefinesPt Macros.thisCreature PowerAlone
    (Plus (Lit 1)
          (DistinctCount CardTypeAxis
             (Macros.allOf (InZone (Macros.graveyardOf (PlayerGroup YourOpponents)))))))

||| Faeburrow Elder's pump
public export
faeburrowElderPump : Ability
faeburrowElderPump =
  Static (Macros.getsPt Macros.thisCreature
               (Up (Macros.times 1 (DistinctCount ColorAxis
                                 (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))))
               (Up (Macros.times 1 (DistinctCount ColorAxis
                                 (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))))))

||| Bonds of Faith
public export
bondsOfFaithPump : Ability
bondsOfFaithPump =
  Static (Macros.onlyWhile
            (Macros.getsPt (AttachHost Enchanted (TypeW Creature)) (Up (Lit 2)) (Up (Lit 2)))
            (Matches ((Macros.It OneOf)) (HasSubtype (creatureType "Human"))))

||| Field of Dreams
public export
fieldOfDreams : Card
fieldOfDreams =
  Macros.card "Field of Dreams" (Just [Macros.pip Blue]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary) ]
       Nothing

||| Lantern of Insight
public export
lanternOfInsightRider : Ability
lanternOfInsightRider =
  Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary)

||| Revelation
public export
revelation : Card
revelation =
  Macros.card "Revelation" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       Nothing

||| Phyrexian Unlife
public export
phyrexianUnlifeImmunity : Ability
phyrexianUnlifeImmunity = Static (NoLossFromZeroLife You)

||| Umbris, Fear Manifest
public export
umbrisPump : Ability
umbrisPump =
  Static (Macros.getsPt Macros.thisCreature
               (Up (Macros.forEach 1
                        (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                              InZone Macros.exileZ])))
               (Up (Macros.forEach 1
                        (And [IsCard, HasPossessor OwnerAx (PlayerGroup YourOpponents),
                              InZone Macros.exileZ]))))

||| Turbulent Fen
public export
turbulentFen : Ability
turbulentFen =
  Static (Macros.onlyUnless (Macros.entersTapped Macros.thisLand)
                    (CompareAmt
                       (Macros.countOf (And [Macros.land,
                                      HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                       AtLeast (Lit 8)))

||| Bastion Protector
public export
bastionProtectorPump : Ability
bastionProtectorPump =
  Static (Macros.getsPt (Macros.allOf (And [Macros.creature, HasDesignation CommanderD Nothing,
                            HasPossessor ControllerAx You]))
               (Up (Lit 2)) (Up (Lit 2)))

||| Luxior, Giada's Gift
public export
luxiorTypeSetting : Ability
luxiorTypeSetting =
  Static (AndAlso Nothing
            [ Becomes (AttachHost Equipped PermanentW) Loses
                      (Bundle (MkToken Nothing [] (Macros.typesOnly [Planeswalker]) [] Nothing)
                              Nothing)
            , Becomes ((Macros.It OneOf)) Adds
                      (Bundle (MkToken Nothing [] (MkTypeLine [] [Creature]) [] Nothing)
                              Nothing) ])

||| Kasmina, Enigma Sage
public export
kasminaLoyaltySharing : StaticSpec []
kasminaLoyaltySharing =
  GainsAbilitiesOf (Macros.allOf (And [HasType Planeswalker, HasPossessor ControllerAx You,
                                OtherThan This]))
                   [LoyaltyClass] This Nothing

||| Nicol Bolas, Dragon-God
public export
nicolBolasDragonGodSharing : StaticSpec []
nicolBolasDragonGodSharing =
  GainsAbilitiesOf This [LoyaltyClass]
                   (Macros.allOf (And [HasType Planeswalker, OtherThan This,
                                InZone Macros.battlefieldZ]))
                   Nothing

||| Myr Welder
public export
myrWelderBorrowedAbilities : StaticSpec []
myrWelderBorrowedAbilities =
  GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                   (Macros.allOf (ExiledWith This)) Nothing

||| Sharkey, Tyrant of the Shire
public export
sharkeyBorrowedLandAbilities : StaticSpec []
sharkeyBorrowedLandAbilities =
  GainsAbilitiesOf This [AnyActivated]
                   (Macros.allOf (And [Macros.land,
                                HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                   (Just IsManaAbility)

||| Skill Borrower
public export
skillBorrower : Card
skillBorrower =
  Macros.card "Skill Borrower"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Human", creatureType "Wizard"]
                   [Artifact, Creature])
       [ Static (Visibility Reveal You TopOfLibrary)
       , Static (Macros.onlyWhile
                   (GainsAbilitiesOf Macros.thisCreature [AnyActivated]
                                     (Macros.topSlice (Lit 1)) Nothing)
                   (Matches (Macros.topSlice (Lit 1)) (Or [Macros.artifact, Macros.creature]))) ]
       (Just (1, 3))

||| Emissary of Grudges
public export
emissaryOfGrudgesEntry : StaticSpec []
emissaryOfGrudgesEntry =
  Macros.entersChoosingPlayerSecretly Macros.thisCreature (Just OpponentsOnly)

aerialAssault : Instruction []
aerialAssault = Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped]))

asphyxiate : Instruction []
asphyxiate = Macros.destroy (Macros.target (And [Macros.creature, Macros.untapped]))

vindicate : Instruction []
vindicate = Macros.destroy (Macros.target Permanent)

counterspellCard : Card
counterspellCard =
  Macros.card "Counterspell" (Just [Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant]) [Spell Nothing counterspell] Nothing

hymnOfRebirthCard : Card
hymnOfRebirthCard =
  Macros.card "Hymn of Rebirth" (Just [Macros.generic 3, Macros.pip Green, Macros.pip White]) []
       (MkTypeLine [] [Sorcery]) [Spell Nothing hymnOfRebirth] Nothing

chandrasPyrohelixCard : Card
chandrasPyrohelixCard =
  Macros.card "Chandra's Pyrohelix" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [] [Instant]) [Spell Nothing forkedBolt] Nothing

||| Smite
public export
smite : Card
smite =
  Macros.card "Smite" (Just [Macros.pip White]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Macros.destroy (Macros.target (And [Macros.creature, Blocked]))) ]
       Nothing

public export
eerieUltimatum : Card
eerieUltimatum =
  Macros.card "Eerie Ultimatum"
       (Just [Macros.pip White, Macros.pip White, Macros.pip Black,
              Macros.pip Black, Macros.pip Black, Macros.pip Green,
              Macros.pip Green]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Macros.move
                  (Macros.withDifferentNames
                     (Macros.counted Macros.anyNumber
                                   (And [Permanent,
                                         InZone (Macros.graveyardOf You)])))
                  Macros.battlefieldZ) ]
       Nothing

||| Gray Merchant of Asphodel
public export
grayMerchantDrain : Instruction []
grayMerchantDrain =
  Sequentially [ Macros.losesLife (Macros.each Opponent) (LetterVal X)
               , Define X (Devotion You (LitColor Black) Nothing) ]
