module Experimental.Cards.Faces

import Experimental
import Experimental.Macros

%default total


cyberConversion : Instruction []
cyberConversion =
  Sequentially
    [ SetStatus FaceDown (Macros.target Macros.creature)
    , Macros.becomesAs ((Macros.It OneOf))
        (MkToken (Just (Lit 2 ** Lit 2)) []
                 (MkTypeLine [creatureType "Cyberman"] [Artifact, Creature]) [] Nothing)
        Nothing ]

breakOpen : Instruction []
breakOpen = SetStatus FaceUp (Macros.target (And [Macros.creature, Macros.faceDown,
                                               HasPossessor ControllerAx Macros.anOpponent]))

jushiApprentice : Ability
jushiApprentice =
  Macros.activated (Compound [Mana [Macros.generic 2, Macros.pip Blue], TapSymbol])
                   (Sequentially
               [ (Draw You (Lit 1))
               , If (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                AtLeast (Lit 9))
                    (SetStatus Flipped Macros.thisCreature)
                    Nothing ])

invasionOfDominaria : Card
invasionOfDominaria =
  Transforming
    (Macros.frontFace "Invasion of Dominaria" (Just [Macros.generic 2, Macros.pip White]) []
            (MkTypeLine [battleType "Siege"] [Battle])
            [ Macros.triggered When (Enters Macros.thisSiege Nothing)
                               (Sequentially [ Macros.gainsLife You (Lit 4)
                                             , Draw You (Lit 1) ]) ]
            (Macros.defenseBox 5))
    (Macros.backFace "Serra Faithkeeper" [] (MkTypeLine [creatureType "Angel"] [Creature])
               [ Macros.keyword "Flying", Macros.keyword "Vigilance" ]
               (Macros.printedBox (Just (4, 4))))

||| Missy
public export
missyFaceDownReturn : Ability
missyFaceDownReturn =
  Macros.triggered Whenever
    (Dies (Macros.a (And [Macros.creature, Not Macros.artifact,
                          OtherThan Macros.thisCreature])))
    (Sequentially
       [ Move ((Macros.It OneOf)) Macros.battlefieldZ
              [EntersAs FaceDown, EntersTapped, Under You]
       , Continuously
           (Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                              (MkTypeLine [creatureType "Cyberman"] [Artifact, Creature])
                              [] Nothing) Nothing))
           Nothing ])

||| Yedora, Grave Gardener
public export
yedoraGraveGardener : Card
yedoraGraveGardener =
  Macros.card "Yedora, Grave Gardener"
       (Just [Macros.generic 4, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Treefolk", creatureType "Druid"] [Creature])
       [ Macros.triggered Whenever
           (Dies (Macros.a (And [Macros.nontoken, Macros.creatureYouControl,
                                 OtherThan Macros.thisCreature])))
           (Macros.may You
              (Sequentially
                 [ Move ((Macros.It OneOf)) Macros.battlefieldZ
                        [EntersAs FaceDown, Under (Macros.ownerOf ((Macros.It OneOf)))]
                 , Continuously
                     (Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken Nothing []
                                        (MkTypeLine [landType "Forest"] [Land])
                                        [] Nothing) Nothing))
                     Nothing ])) ]
       (Just (5, 5))

||| Ral Zarek, Guest Lecturer's ultimate
public export
ralZarekGuestLecturerUltimate : Instruction []
ralZarekGuestLecturerUltimate =
  Sequentially [ FlipCoins You (FlipCount (Lit 5))
               , SkipsNext (Macros.target Opponent) Turn (LetterVal X)
               , Define X (CoinsShowing Heads) ]

public export
faceDownFlyingCounter : StaticSpec []
faceDownFlyingCounter =
  Macros.entersWithCounters (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                                  Macros.faceDown]))
                            (Lit 1) (Macros.flyingCounter)

goblinArchaeologist : Ability
goblinArchaeologist =
  Macros.activated (Compound [Mana [Macros.pip Red], TapSymbol])
    (Sequentially
       [(Macros.flipCoins You 1),
        Macros.ifThen (FlipCalled You WinsFlip)
          (Sequentially [Macros.destroy (Macros.target Macros.artifact),
                         SetStatus Untapped Macros.thisCreature]),
        Macros.ifThen (FlipCalled You LosesFlip)
          (Macros.sacrifice You Macros.thisCreature)])

public export
chanceEncounter : Card
chanceEncounter =
  Macros.card "Chance Encounter"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered Whenever (FlipsCoin You (Just WinsFlip))
                          (PutCounters (Lit 1) (PrintedKind (NamedCounter "Luck")) Macros.thisEnchantment)
       , Macros.triggeredIf At
                            (BeginningOf ThePart Upkeep (ByPlayer You))
                            (CompareAmt (CountersOn (NamedCounter "Luck") Macros.thisEnchantment)
                                        AtLeast (Lit 10))
                            (Concludes WinGame You) ]
       Nothing

||| Karplusan Minotaur's win arm
public export
karplusanMinotaurWinFlip : Ability
karplusanMinotaurWinFlip =
  Macros.triggered Whenever (FlipsCoin You (Just WinsFlip))
                   (DealDamage Macros.thisCreature (Lit 1)
                               (Macros.target Macros.anyTarget))

||| Ral Zarek's ultimate
public export
ralZarekUltimate : Instruction []
ralZarekUltimate =
  Sequentially [(Macros.flipCoins You 5),
                ExtraTurn You (CoinsShowing Heads)]

||| Krark's Thumb
public export
krarksThumbExtraFlip : Instruction []
krarksThumbExtraFlip =
  Macros.ifWouldInstead (Macros.flipsCoin You)
    (Sequentially [ (Macros.flipCoins You 2)
                  , IgnoreOutcomes (IgnoreChosen Nothing (Lit 1)) ])
    Nothing

||| Goblin Assassin
public export
goblinAssassinCoinTails : Instruction []
goblinAssassinCoinTails =
  Sequentially [ FlipCoins (Macros.each AnyPlayer) (FlipCount (Lit 1))
               , Macros.sacrifice (Macros.each (And [AnyPlayer, CoinCameUp Tails]))
                                  (Macros.aTheirChoice Macros.creature) ]

||| Rakdos, the Showstopper
public export
rakdosShowstopperFlips : Instruction []
rakdosShowstopperFlips =
  Sequentially
    [ (FlipCoins You (FlipPer (Macros.each (And [ Macros.creature
                   , Not (Or [ HasSubtype (creatureType "Demon")
                             , HasSubtype (creatureType "Devil")
                             , HasSubtype (creatureType "Imp") ]) ]))))
    , Macros.destroy (Macros.each (And [Macros.creature, CoinCameUp Tails])) ]

merfolkSecretkeeper : Card
merfolkSecretkeeper =
  Adventurer
    (Macros.frontFace "Merfolk Secretkeeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature]) []
            (Macros.printedBox (Just (0, 4))))
    (Macros.alternative "Venture Deeper" (Just [Macros.pip Blue]) []
            (MkTypeLine [spellType "Adventure"] [Sorcery])
            [ Spell Nothing (Macros.mills (Macros.target AnyPlayer) (Lit 4) They) ]
            Nothing)

orochiEggwatcher : Card
orochiEggwatcher =
  FlipCard
    (Macros.frontFace "Orochi Eggwatcher" (Just [Macros.generic 2, Macros.pip Green]) []
            (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
            [ Macros.activated
                (Compound [Mana [Macros.generic 2, Macros.pip Green], TapSymbol])
                (Sequentially
                   [ Macros.create (Lit 1) (Macros.creatureTok 1 1 [Green] [creatureType "Snake"])
                   , Macros.ifThen (CompareAmt (Macros.countOf Macros.creatureYouControl)
                                               AtLeast (Lit 10))
                                   (SetStatus Flipped Macros.thisCreature) ]) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.alternative "Shidako, Broodmistress" Nothing [Legendary]
               (MkTypeLine [creatureType "Snake", creatureType "Shaman"] [Creature])
               [ Macros.activated
                   (Compound [ Mana [Macros.pip Green]
                             , Do (Macros.sacrifice You (Macros.a Macros.creature)) ])
                   (Macros.gets (Macros.target Macros.creature)
                                (PtUp (Lit 3)) (PtUp (Lit 3))
                                (Just Macros.untilEndOfTurn)) ]
               (Macros.printedBox (Just (3, 3))))

public export
planeswalkerBackWithoutLoyalty : CardFace
planeswalkerBackWithoutLoyalty =
  Macros.backFace "" [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker]) [] Nothing

public export
planeswalkerBackWithoutLoyaltyOk : FaceLaws Back Faces.planeswalkerBackWithoutLoyalty
planeswalkerBackWithoutLoyaltyOk = MkCharacteristicsLaws

||| Shapeshifter's printed box
public export
shapeshifterBox : PrintedBox
shapeshifterBox = PtBox PrintedStar (PrintedMinusStar 7)

||| Tarmogoyf's printed box
public export
tarmogoyfBox : PrintedBox
tarmogoyfBox = PtBox PrintedStar (PrintedStarPlus 1)

||| Garruk Relentless
public export
garrukRelentlessFlip : Ability
garrukRelentlessFlip =
  Macros.triggered When
    (StateHolds
       (Matches Macros.thisPlaneswalker
                (Compare [CounterAxis (NamedCounter "Loyalty")] AtMost (Lit 2))))
    (Macros.transform Macros.thisPlaneswalker)

||| Mana Clash
public export
manaClashFlip : Instruction []
manaClashFlip =
  FlipCoins (EachOf (Both You (Macros.target Opponent))) (FlipCount (Lit 1))

public export
akkiLavarunner : Card
akkiLavarunner =
  FlipCard
    (Macros.frontFace "Akki Lavarunner" (Just [Macros.generic 3, Macros.pip Red]) []
            (MkTypeLine [creatureType "Goblin", creatureType "Warrior"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.triggered Whenever
                (DealsDamage AnyDamage Macros.thisCreature (OnePatient Macros.anOpponent))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.alternative "Tok-Tok, Volcano Born" Nothing [Legendary]
               (MkTypeLine [creatureType "Goblin", creatureType "Shaman"] [Creature])
               [ Macros.keywordQuality "Protection" (ColorIs Red)
               , Static (DamageRule AnyDamage (DealtBy (Macros.a (And [Macros.source, ColorIs Red]))) (ToRecipient (Macros.a AnyPlayer)) (Scale (Shifted ShiftUp (Lit 1))) Repeatedly) ]
               (Macros.printedBox (Just (2, 2))))

public export
bushiTenderfoot : Card
bushiTenderfoot =
  FlipCard
    (Macros.frontFace "Bushi Tenderfoot" (Just [Macros.pip White]) []
            (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
            [ Macros.triggered When
                (Dies (Macros.a (And [ Macros.creature
                                     , HappenedTo (MkLookback DamageTaken ThisTurn (Just (Involving Macros.thisCreature))) ])))
                (SetStatus Flipped Macros.thisCreature) ]
            (Macros.printedBox (Just (1, 1))))
    (Macros.alternative "Kenzo the Hardhearted" Nothing [Legendary]
               (MkTypeLine [creatureType "Human", creatureType "Samurai"] [Creature])
               [ Macros.keyword "DoubleStrike"
               , Macros.keywordNumber "Bushido" (Lit 2) ]
               (Macros.printedBox (Just (3, 4))))

||| Kitsune Mystic
public export
kitsuneMysticFlip : Ability
kitsuneMysticFlip =
  Macros.triggeredIf At
    (BeginningOf ThePart EndStep NoPossessor)
    (Matches Macros.thisCreature
       (AttachedBy Enchanted
          (Macros.counted (Macros.atLeast 2)
             (HasSubtype (enchantmentType "Aura")))))
    (SetStatus Flipped Macros.thisCreature)

||| Keeper of the Lens
public export
keeperOfTheLens : Card
keeperOfTheLens =
  Macros.card "Keeper of the Lens" (Just [Macros.generic 1]) []
       (MkTypeLine [creatureType "Golem"] [Artifact, Creature])
       [ Static (Visibility LookAt You
                   (VisibleObjects
                      (Macros.allOf (And [Macros.creature, Macros.faceDown,
                                   Not (HasPossessor ControllerAx You)])))) ]
       (Just (1, 2))

||| Lens of Clarity
public export
lensOfClarity : Card
lensOfClarity =
  Macros.card "Lens of Clarity" (Just [Macros.generic 1]) []
       (MkTypeLine [] [Artifact])
       [ Static (AndAlso Nothing
           [ Visibility LookAt You TopOfLibrary
           , Visibility LookAt You
               (VisibleObjects
                  (Macros.allOf (And [Macros.creature, Macros.faceDown,
                               Not (HasPossessor ControllerAx You)]))) ]) ]
       Nothing

public export
kitsuneMystic : Card
kitsuneMystic =
  FlipCard
    (Macros.frontFace "Kitsune Mystic" (Just [Macros.generic 3, Macros.pip White]) []
            (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
            [ kitsuneMysticFlip ]
            (Macros.printedBox (Just (2, 3))))
    (Macros.alternative "Autumn-Tail, Kitsune Sage" Nothing [Legendary]
               (MkTypeLine [creatureType "Fox", creatureType "Wizard"] [Creature])
               [ Macros.activated (Mana [Macros.generic 1])
                   (AttachTo
                      (Macros.target
                         (And [ HasSubtype (enchantmentType "Aura")
                              , AttachedTo (Macros.a Macros.creature) ]))
                      (Macros.a (And [Macros.creature,
                                      OtherThan (Macros.That (TypeW Creature) OneOf)]))) ]
               (Macros.printedBox (Just (4, 5))))

||| Vesuvan Shapeshifter
public export
vesuvanShapeshifterCopySpan :
  Instruction [MkBinding AD Object OneOf
            (ObjectP (Just Creature) (Just Battlefield) Nothing Nothing Nothing)]
vesuvanShapeshifterCopySpan =
  Continuously
    (BecomesCopy Macros.thisCreature (Macros.That (TypeW Creature) OneOf)
       [ExceptAbility
          (Macros.triggered At (BeginningOf ThePart Upkeep (ByPlayer You))
             (Macros.may You (SetStatus FaceDown Macros.thisCreature)))])
    (Just (UntilEvent (StatusEvent Macros.thisCreature FaceDown)))

||| Arlinn Kord // Arlinn, Embraced by the Moon
public export
arlinnKord : Card
arlinnKord =
  Transforming
    (Macros.frontFace "Arlinn Kord" (Just [Macros.generic 2, Macros.pip Red, Macros.pip Green])
            [Legendary] (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
            [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                (Continuously
                   (AndAlso Nothing [ Gets Adds (Described (TargetDet (Macros.upTo 1)) Macros.creature)
                                   (PtUp (Lit 2)) (PtUp (Lit 2))
                            , Gains ((Macros.It OneOf)) (Macros.keyword "Vigilance")
                            , Gains ((Macros.It OneOf)) (Macros.keyword "Haste") ])
                   (Just Macros.untilEndOfTurn))
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Sequentially
                   [ Macros.create (Lit 1)
                       (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])
                   , Macros.transform Macros.thisPlaneswalker ]) ]
            (Macros.loyaltyBox 3))
    (Macros.backFace "Arlinn, Embraced by the Moon" [Legendary]
               (MkTypeLine [planeswalkerType "Arlinn"] [Planeswalker])
               [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                   (Continuously
                      (AndAlso Nothing [ Gets Adds (Macros.allOf Macros.creatureYouControl)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains ((Macros.It ManyOf)) (Macros.keyword "Trample") ])
                      (Just Macros.untilEndOfTurn))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                   (Sequentially
                      [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
                      , Macros.transform Macros.thisPlaneswalker ])
               , Macros.activated (LoyaltySymbol (LoyaltyDown 6))
                   (GetsEmblem You
                      [ Static (AndAlso Nothing
                          [ Gains (Macros.allOf Macros.creatureYouControl) (Macros.keyword "Haste")
                          , Gains ((Macros.It ManyOf)) (Macros.activated TapSymbol
                              (DealDamage Macros.thisCreature (StatOf Power Macros.thisCreature)
                                          (Macros.target Macros.anyTarget))) ]) ]) ]
               Nothing)

||| Neglected Heirloom // Ashmouth Blade
public export
neglectedHeirloom : Card
neglectedHeirloom =
  Transforming
    (Macros.frontFace "Neglected Heirloom" (Just [Macros.generic 1]) []
            (MkTypeLine [artifactType "Equipment"] [Artifact])
            [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                           (PtUp (Lit 1)) (PtUp (Lit 1)))
            , Macros.triggered When
                (VerbedEvent Nothing "Transform"
                  (Just (AttachHost Equipped (TypeW Creature))) Nothing)
                (Macros.transform Macros.thisEquipment)
            , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
            Nothing)
    (Macros.backFace "Ashmouth Blade" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (AndAlso Nothing [ Gets Adds (AttachHost Equipped (TypeW Creature))
                                        (PtUp (Lit 3)) (PtUp (Lit 3))
                                 , Gains ((Macros.It OneOf)) (Macros.keyword "FirstStrike") ])
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
               Nothing)

||| Harvest Hand // Scrounged Scythe
public export
harvestHand : Card
harvestHand =
  Transforming
    (Macros.frontFace "Harvest Hand" (Just [Macros.generic 3]) []
            (MkTypeLine [creatureType "Scarecrow"] [Artifact, Creature])
            [ Macros.triggered When (Dies Macros.thisCreature)
                (Macros.returnToBattlefieldTransformed ((Macros.It OneOf)) You) ]
            (Macros.printedBox (Just (2, 2))))
    (Macros.backFace "Scrounged Scythe" []
               (MkTypeLine [artifactType "Equipment"] [Artifact])
               [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                              (PtUp (Lit 1)) (PtUp (Lit 1)))
               , Static (Macros.onlyWhile
                           (Gains (AttachHost Equipped (TypeW Creature))
                                  (Macros.keyword "Menace"))
                           (Matches (AttachHost Equipped (TypeW Creature))
                                    (HasSubtype (creatureType "Human"))))
               , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
               Nothing)

||| Cult of the Waxing Moon
public export
cultOfTheWaxingMoon : Card
cultOfTheWaxingMoon =
  Macros.card "Cult of the Waxing Moon"
       (Just [Macros.generic 4, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Macros.triggered Whenever
           (VerbedEvent Nothing "Transform"
              (Just (Macros.a (And [Permanent, HasPossessor ControllerAx You])))
              (Just (And [Macros.creature,
                          Not (HasSubtype (creatureType "Human"))])))
           (Macros.create (Lit 1)
              (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
       (Just (5, 4))

public export
chitteringHostOnScavengers : CardFace
chitteringHostOnScavengers =
  Macros.backFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously
                   (AndAlso Nothing [ Gets Adds (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains ((Macros.It ManyOf)) (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))

||| Chittering Host as the GRAF RATS card carries it
public export
chitteringHostOnGrafRats : CardFace
chitteringHostOnGrafRats =
  Macros.backFace "Chittering Host" []
            (MkTypeLine [creatureType "Eldrazi", creatureType "Horror"] [Creature])
            [ Macros.keyword "Haste"
            , Macros.keyword "Menace"
            , Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Continuously
                   (AndAlso Nothing [ Gets Adds (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                                   (PtUp (Lit 1)) (PtUp (Lit 0))
                            , Gains ((Macros.It ManyOf)) (Macros.keyword "Menace") ])
                   (Just Macros.untilEndOfTurn)) ]
            (Macros.printedBox (Just (5, 6)))


||| Midnight Scavengers // Chittering Host
public export
midnightScavengers : Card
midnightScavengers =
  Transforming
    (Macros.frontFace "Midnight Scavengers" (Just [Macros.generic 4, Macros.pip Black]) []
            (MkTypeLine [creatureType "Human", creatureType "Rogue"] [Creature])
            [ Macros.triggered When (Enters Macros.thisCreature Nothing)
                (Macros.may You
                   (Macros.returnTo
                      (Macros.target (And [ Macros.creature
                                          , InZone (Macros.graveyardOf You)
                                          , Compare [StatAxis ManaValue] AtMost (Lit 3) ]))
                      Macros.handZ [])) ]
            (Macros.printedBox (Just (3, 3))))
    Faces.chitteringHostOnScavengers

public export
meldThemInto : Instruction []
meldThemInto =
  Sequentially
    [ Macros.exile
        (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You,
                            Or [Named (PrintedName "Graf Rats"),
                                Named (PrintedName "Midnight Scavengers")]]))
    , Macros.meldInto (Macros.ItVerbed "Exile" ManyOf) "Chittering Host" ]

||| Profit // Loss
public export
profitLoss : Card
profitLoss =
  SplitCard
    (Macros.frontFace "Profit" (Just [Macros.generic 1, Macros.pip White]) []
            (MkTypeLine [] [Instant])
            [ Spell Nothing (Macros.gets (Macros.allOf Macros.creatureYouControl)
                                 (PtUp (Lit 1)) (PtUp (Lit 1))
                                 (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)
    (Macros.frontFace "Loss" (Just [Macros.generic 2, Macros.pip Black]) []
            (MkTypeLine [] [Instant])
            [ Spell Nothing (Macros.gets
                       (Macros.allOf Macros.creatureYourOpponentsControl)
                       (PtDown (Lit 1)) (PtDown (Lit 1))
                       (Just Macros.untilEndOfTurn))
            , Macros.keyword "Fuse" ]
            Nothing)

||| Glassworks // Shattered Yard
public export
glassworksShatteredYard : Card
glassworksShatteredYard =
  SharedLineSplit (MkTypeLine [enchantmentType "Room"] [Enchantment]) [] Nothing
    (MkSharedHalf "Glassworks" (Just [Macros.generic 2, Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 4)
              (Macros.target (And [Macros.creature,
                                   HasPossessor ControllerAx Macros.anOpponent]))) ])
    (MkSharedHalf "Shattered Yard" (Just [Macros.generic 4, Macros.pip Red])
       [ Macros.triggered At (BeginningOf ThePart EndStep (ByPlayer You))
           (DealDamage Macros.thisRoom (Lit 1) (Macros.each Opponent)) ])

||| Balemurk Leech
public export
balemurkLeech : Card
balemurkLeech =
  Macros.card "Balemurk Leech" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [creatureType "Leech"] [Creature])
       [ Macros.abilityWord "eerie"
           (Macros.triggeredJoined Whenever
              (Enters (Macros.a (And [Macros.enchantment, HasPossessor ControllerAx You]))
                      Nothing)
              [ Macros.joinedHead Whenever
                  (VerbedEvent (Just You) "Fully Unlock"
                     (Just (Macros.a (HasSubtype (enchantmentType "Room"))))
                     Nothing) ]
              (Macros.losesLife (Macros.each Opponent) (Lit 1))) ]
       (Just (2, 2))

||| Ghostly Keybearer
public export
ghostlyKeybearer : Card
ghostlyKeybearer =
  Macros.card "Ghostly Keybearer" (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
           (Unlock (DoorOf (Just Locked)
                      (Described (TargetDet (Macros.upTo 1)) (And [HasSubtype (enchantmentType "Room"),
                               HasPossessor ControllerAx You])))) ]
       (Just (3, 3))

||| Riddles in the Dark
public export
riddlesInTheDark : Card
riddlesInTheDark =
  Macros.card "Riddles in the Dark"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.lookAt (Macros.topSlice (Lit 4))
                  , SeparateIntoPiles You ((Macros.It ManyOf)) 2 [FaceDownPile, FaceUpPile]
                  , Macros.chooses Macros.anOpponent Macros.onePile
                  , Macros.move (Macros.That PileW OneOf) Macros.handZ
                  , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       Nothing

||| Fortune's Favor
public export
fortunesFavor : Card
fortunesFavor =
  Macros.card "Fortune's Favor"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Expose LookAt (Macros.target Opponent)
                           (ExposedCards (Macros.topSlice (Lit 4)))
                  , SeparateIntoPiles They ((Macros.It ManyOf)) 2 [FaceDownPile, FaceUpPile]
                  , Macros.move Macros.onePile Macros.handZ
                  , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       Nothing

||| Curator of Destinies
public export
curatorOfDestinies : Card
curatorOfDestinies =
  Macros.card "Curator of Destinies"
       (Just [Macros.generic 4, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Static (Macros.objectCant "Counter" This)
       , Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.lookAt (Macros.topSlice (Lit 5))
              , SeparateIntoPiles You ((Macros.It ManyOf)) 2 [FaceDownPile, FaceUpPile]
              , Macros.chooses Macros.anOpponent Macros.onePile
              , Macros.move (Macros.That PileW OneOf) Macros.handZ
              , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       (Just (5, 5))

||| Atris, Oracle of Half-Truths
public export
atrisOracleOfHalfTruths : Card
atrisOracleOfHalfTruths =
  Macros.cardOf "Atris, Oracle of Half-Truths"
       (Just [Macros.generic 2, Macros.pip Blue, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Advisor"] [Creature])
       [ Macros.keyword "Menace"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Expose LookAt (Macros.target Opponent)
                       (ExposedCards (Macros.topSlice (Lit 3)))
              , SeparateIntoPiles They ((Macros.It ManyOf)) 2 [FaceDownPile, FaceUpPile]
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       (Macros.printedBox (Just (3, 2)))

||| Garruk Relentless // Garruk, the Veil-Cursed
public export
garrukRelentless : Card
garrukRelentless =
  Transforming
    (Macros.frontFace "Garruk Relentless" (Just [Macros.generic 3, Macros.pip Green])
            [Legendary] (MkTypeLine [planeswalkerType "Garruk"] [Planeswalker])
            [ Faces.garrukRelentlessFlip
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Sequentially
                   [ DealDamage Macros.thisPlaneswalker (Lit 3)
                                (Macros.target Macros.creature)
                   , DealDamage (Macros.That (TypeW Creature) OneOf) (StatOf Power (Macros.That (TypeW Creature) OneOf))
                                Macros.thisPlaneswalker ])
            , Macros.activated (LoyaltySymbol LoyaltyZero)
                (Macros.create (Lit 1)
                   (Macros.creatureTok 2 2 [Green] [creatureType "Wolf"])) ]
            (Macros.loyaltyBox 3))
    (Macros.backFace "Garruk, the Veil-Cursed" [Legendary]
               (MkTypeLine [planeswalkerType "Garruk"] [Planeswalker])
               [ Macros.activated (LoyaltySymbol (LoyaltyUp 1))
                   (Macros.create (Lit 1)
                      (MkToken (Just (Lit 1 ** Lit 1)) [Black]
                               (MkTypeLine [creatureType "Wolf"] [Creature])
                               [Macros.keyword "Deathtouch"] Nothing))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 1))
                   ((IfDone (Macros.sacrifice You (Macros.a Macros.creature)) (Just (Sequentially
                         [ Macros.searchLibraryFor (Macros.exactly 1) Macros.creature
                         , Macros.revealsIt
                         , Macros.move Macros.foundCard Macros.handZ
                         , Macros.shuffle ])) Nothing))
               , Macros.activated (LoyaltySymbol (LoyaltyDown 3))
                   (Sequentially
                      [ Continuously
                          (AndAlso Nothing
                             [ Gains (Macros.allOf Macros.creatureYouControl)
                                     (Macros.keyword "Trample")
                             , Gets Adds (Macros.It ManyOf) (PtUp (LetterVal X)) (PtUp (LetterVal X)) ])
                          (Just Macros.untilEndOfTurn)
                      , Define X (Macros.countOf (And [Macros.creature,
                                                InZone (Macros.graveyardOf You)])) ]) ]
               Nothing)

||| Brimstone Mage
public export
brimstoneMage : Card
brimstoneMage =
  Macros.leveler "Brimstone Mage" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Shaman"] [Creature])
       [ Macros.levelUp (Mana [Macros.generic 3, Macros.pip Red]) ]
       (Just (2, 2))
       [ Macros.levelBand (LevelBetween 1 2) 2 3
           [ Macros.activated TapSymbol
               (DealDamage Macros.thisCreature (Lit 1) (Macros.target Macros.anyTarget)) ]
       , Macros.levelBand (LevelAtLeast 3) 2 4
           [ Macros.activated TapSymbol
               (DealDamage Macros.thisCreature (Lit 3) (Macros.target Macros.anyTarget)) ] ]

||| Student of Warfare
public export
studentOfWarfare : Card
studentOfWarfare =
  Macros.leveler "Student of Warfare" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.levelUp (Mana [Macros.pip White]) ]
       (Just (1, 1))
       [ Macros.levelBand (LevelBetween 2 6) 3 3 [ Macros.keyword "FirstStrike" ]
       , Macros.levelBand (LevelAtLeast 7) 4 4 [ Macros.keyword "DoubleStrike" ] ]

||| Kargan Dragonlord
public export
karganDragonlord : Card
karganDragonlord =
  Macros.leveler "Kargan Dragonlord" (Just [Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Warrior"] [Creature])
       [ Macros.levelUp (Mana [Macros.pip Red]) ]
       (Just (2, 2))
       [ Macros.levelBand (LevelBetween 4 7) 4 4 [ Macros.keyword "Flying" ]
       , Macros.levelBand (LevelAtLeast 8) 8 8
           [ Macros.keyword "Flying"
           , Macros.keyword "Trample"
           , Macros.activated (Mana [Macros.pip Red])
               (Macros.gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 0))
                            (Just Macros.untilEndOfTurn)) ] ]

||| Arcane Proxy
public export
arcaneProxy : Card
arcaneProxy =
  Macros.prototype "Arcane Proxy" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Wizard"] [Artifact, Creature])
       [ Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Matches ((Macros.It OneOf)) (Macros.castBy You))
           (Sequentially
              [ Macros.exile
                  (Macros.target (And [ Macros.instantOrSorcery
                                      , IsCard
                                      , Compare [StatAxis ManaValue] AtMost
                                                (StatOf Power Macros.thisCreature)
                                      , InZone (Macros.graveyardOf You) ]))
              , Copy FromCardZone You (Macros.That CardW OneOf) (Lit 1) []
              , Continuously
                  (Macros.mayPlayDeed "Cast" You (Macros.That CopyW OneOf) Nothing
                     (PlayRider Nothing Nothing Nothing False WithoutPaying))
                  Nothing ]) ]
       (Just (4, 3))
       (Macros.prototypeAlt [Macros.generic 1, Macros.pip Blue, Macros.pip Blue] 2 1)

||| Blitz Automaton
public export
blitzAutomaton : Card
blitzAutomaton =
  Macros.prototype "Blitz Automaton" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.keyword "Haste" ]
       (Just (6, 4))
       (Macros.prototypeAlt [Macros.generic 2, Macros.pip Red] 3 2)

||| Goring Warplow
public export
goringWarplow : Card
goringWarplow =
  Macros.prototype "Goring Warplow" (Just [Macros.generic 6]) []
       (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
       [ Macros.keyword "Deathtouch" ]
       (Just (5, 4))
       (Macros.prototypeAlt [Macros.generic 1, Macros.pip Black] 1 1)
