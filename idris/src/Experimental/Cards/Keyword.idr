module Experimental.Cards.Keyword

import Experimental
import Experimental.Macros
import Experimental.Cards.Trigger

%default total


jump : Instruction []
jump = Macros.gains (Macros.target Macros.creature) (Macros.keyword "Flying") (Just Macros.untilEndOfTurn)

gabrielAngelfire : Instruction []
gabrielAngelfire =
  Macros.gains Macros.thisCreature (Macros.keyword "Flying") (Just Macros.untilYourNextUpkeep)

builtToSmash : Instruction []
builtToSmash =
  Sequentially [Macros.gets (Macros.target (And [Macros.creature, Attacking])) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn),
                If (Macros.itsA (And [Macros.artifact, Macros.creature]))
                   (Macros.gains ((Macros.It OneOf)) (Macros.keyword "Trample") (Just Macros.untilEndOfTurn))
                   Nothing]

aviationPioneer : Instruction []
aviationPioneer =
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                          [Macros.keyword "Flying"] Nothing)

fireNavyTrebuchet : Instruction []
fireNavyTrebuchet =
  Macros.createTappedAttacking (Lit 1)
    (MkToken (Just (Lit 2 ** Lit 1)) [] (MkTypeLine [creatureType "Construct"] [Artifact, Creature])
             [Macros.keyword "Flying"] (Just "Ballistic Boulder"))

rorixBladewing : Card
rorixBladewing =
  Macros.card "Rorix Bladewing" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red, Macros.pip Red])
       [Legendary] (MkTypeLine [creatureType "Dragon"] [Creature])
       [Macros.keyword "Flying", Macros.keyword "Haste"] (Just (6, 5))

yotianSoldier : Card
yotianSoldier =
  Macros.card "Yotian Soldier" (Just [Macros.generic 3]) []
       (MkTypeLine [creatureType "Soldier"] [Artifact, Creature])
       [Macros.keyword "Vigilance"] (Just (1, 4))

||| Pym Particles
pymParticlesVigilanceGrant : Instruction []
pymParticlesVigilanceGrant =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Vigilance") (Just Macros.untilEndOfTurn)

bladebrand : Instruction []
bladebrand =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Deathtouch") (Just Macros.untilEndOfTurn)

criticalHit : Instruction []
criticalHit =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "DoubleStrike") (Just Macros.untilEndOfTurn)

lightningBlow : Instruction []
lightningBlow =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "FirstStrike") (Just Macros.untilEndOfTurn)

deathByDragons : Instruction []
deathByDragons =
  Create (Macros.each (And [AnyPlayer, OtherThan (Macros.target AnyPlayer)])) (Lit 1)
         (TokenWritten (MkToken (Just (Lit 5 ** Lit 5)) [Red] (MkTypeLine [creatureType "Dragon"] [Creature])
                                [Macros.keyword "Flying"] Nothing)) []

||| Concordant Crossroads
concordantCrossroads : Card
concordantCrossroads =
  Macros.card "Concordant Crossroads" (Just [Macros.pip Green]) [World]
       (MkTypeLine [] [Enchantment])
       [ Static (Gains (Macros.allOf Macros.creature) (Macros.keyword "Haste")) ] Nothing

||| Bristlepack Sentry
public export
bristlepackSentry : Card
bristlepackSentry =
  Macros.card "Bristlepack Sentry" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Plant", creatureType "Wolf"] [Creature])
       [ Macros.keyword "Defender"
       , Static (Macros.onlyWhile
                   (Macros.canDoAsThough Macros.thisCreature "Attack"
                                         (Not (HasKeyword (TheKeyword "Defender"))))
                   (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                 Compare [StatAxis Power] AtLeast (Lit 4)]))) ]
       (Just (3, 3))

platinumAngel : Card
platinumAngel =
  Macros.card "Platinum Angel" (Just [Macros.generic 7]) []
       (MkTypeLine [creatureType "Angel"] [Artifact, Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.playerCant "LoseGame" You)
       , Static (Macros.playerCant "WinGame" (PlayerGroup YourOpponents)) ]
       (Just (4, 4))

abyssalPersecutor : Card
abyssalPersecutor =
  Macros.card "Abyssal Persecutor"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Trample"
       , Static (Macros.playerCant "WinGame" You)
       , Static (Macros.playerCant "LoseGame" (PlayerGroup YourOpponents)) ]
       (Just (6, 6))

smogElemental : Card
smogElemental =
  Macros.card "Smog Elemental"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Gets Adds (Macros.allOf (And [Macros.creature, HasKeyword (TheKeyword "Flying"),
                                   HasPossessor ControllerAx (PlayerGroup YourOpponents)]))
                      (PtDown (Lit 1)) (PtDown (Lit 1))) ]
       (Just (3, 3))

exquisiteArchangel : Card
exquisiteArchangel =
  Macros.card "Exquisite Archangel"
       (Just [Macros.generic 5, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Intercepts (LosesGame You) [] Nothing
                            (Sequentially [Macros.exile You Macros.thisCreature,
                                           Macros.lifeTotalBecomes You (PlayerStatOf StartingLifeTotal You)])
                            Repeatedly Nothing) ]
       (Just (5, 5))

adelbertSteiner : Card
adelbertSteiner =
  Macros.card "Adelbert Steiner" (Just [Macros.generic 1, Macros.pip White])
       [Legendary] (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Lifelink"
       , Static (Gets Adds Macros.thisCreature
               (PtUp (Macros.forEach 1 (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))
               (PtUp (Macros.forEach 1 (And [HasSubtype (artifactType "Equipment"), HasPossessor ControllerAx You])))) ]
       (Just (2, 1))

inspiringStatuary : Card
inspiringStatuary =
  Macros.card "Inspiring Statuary" (Just [Macros.generic 3]) []
       (MkTypeLine [] [Artifact])
       [ Static (Gains (Macros.allOf (And [Not Macros.artifact, Macros.spell, Macros.castBy You]))
                       (Macros.keyword "Improvise")) ]
       Nothing

chiefEngineer : Card
chiefEngineer =
  Macros.card "Chief Engineer" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Artificer"] [Creature])
       [ Static (Gains (Macros.allOf (And [Macros.artifact, Macros.spell, Macros.castBy You]))
                       (Macros.keyword "Convoke")) ]
       (Just (1, 3))

firesongAndSunspeaker : Ability
firesongAndSunspeaker =
  Static (Gains (Macros.allOf (And [ColorIs Red, Macros.instantOrSorcery, Macros.spell,
                             HasPossessor ControllerAx You]))
                (Macros.keyword "Lifelink"))

briaRiptideRogue : Ability
briaRiptideRogue =
  Static (Gains (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                (Macros.keyword "Prowess"))

narsetEnlightenedExile : Ability
narsetEnlightenedExile =
  Static (Gains (Macros.allOf Macros.creatureYouControl) (Macros.keyword "Prowess"))

pontiffOfBlight : Ability
pontiffOfBlight =
  Static (Gains (Macros.allOf (Macros.otherCreatureYouControl Macros.thisCreature))
                (Macros.keyword "Extort"))

prismariTheInspiration : Card
prismariTheInspiration =
  Macros.card "Prismari, the Inspiration"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Elder", creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keywordCosting "Ward" (Macros.payLife You 5)
       , Static (Gains (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell, Macros.castBy You]))
                       Macros.storm) ]
       (Just (7, 7))

tomakulHonorGuard : Card
tomakulHonorGuard =
  Macros.card "Tomakul Honor Guard" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keywordCosting "Ward" (Mana [Macros.generic 2]) ]
       (Just (3, 1))

repentantBlacksmith : Card
repentantBlacksmith =
  Macros.card "Repentant Blacksmith" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.keywordQuality "Protection" (ColorIs Red) ]
       (Just (1, 2))

battleSquadron : Card
battleSquadron =
  Macros.cardOf "Battle Squadron" (Just [Macros.generic 3, Macros.pip Red, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.keyword "Flying"
       , Static (DefinesPt Macros.thisCreature BothEach
                           (Macros.countOf Macros.creatureYouControl)) ]
       (Just (PtBox PrintedStar PrintedStar))

eomerOfTheRiddermark : Card
eomerOfTheRiddermark =
  Macros.card "Éomer of the Riddermark" (Just [Macros.generic 4, Macros.pip Red])
       [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Knight"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggeredIf Whenever
                            (Macros.attacks Macros.thisCreature)
                            (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You,
                                          Superlative MaxOf (StatAxis Power)
                                            (And [Macros.creature,
                                                  InZone Macros.battlefieldZ])]))
                            (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Human", creatureType "Soldier"])) ]
       (Just (5, 4))

angelicGift : Card
angelicGift =
  Macros.card "Angelic Gift" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.keyword "Flying")) ]
       Nothing

consulsLieutenant : Card
consulsLieutenant =
  Macros.card "Consul's Lieutenant" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.renown 1
       , Macros.triggeredIf Whenever
                            (Macros.attacks Macros.thisCreature)
                            (Matches Macros.thisCreature
                                     (HasDesignation Renowned))
                            (Macros.gets (Macros.allOf (And [Macros.creature, Attacking,
                                                      HasPossessor ControllerAx You,
                                                      OtherThan Macros.thisCreature]))
                                         (PtUp (Lit 1)) (PtUp (Lit 1))
                                         (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

secretsOfTheGoldenCity : Card
secretsOfTheGoldenCity =
  Macros.card "Secrets of the Golden City"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.keyword "Ascend"
       , Spell Nothing (InsteadOf (Draw You (Lit 2))
                          (If (Matches You (HasDesignation CitysBlessing))
                              (Draw You (Lit 3))
                              Nothing)) ]
       Nothing

bombur : Card
bombur =
  Macros.card "Bombur, Gentle Dreamer"
       (Just [Macros.generic 2, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Dwarf", creatureType "Bard"] [Creature])
       [ Macros.keyword "Storied"
       , Static (Macros.onlyUnless (Macros.doesntUntap Macros.thisCreature (Just You))
                                 (Matches You (HasDesignation EnduringStory))) ]
       (Just (5, 3))

drachNyen : Card
drachNyen =
  Macros.card "Drach'Nyen"
       (Just [Macros.generic 4, Macros.pip Black, Macros.pip Red]) [Legendary]
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
                          (Macros.exile You (Described (TargetDet (Macros.upTo 1)) Macros.creature))
       , Static (AndAlso Nothing [ Gains (AttachHost Equipped (TypeW Creature))
                                 (Macros.keyword "Menace")
                         , Gets Adds (AttachHost Equipped (TypeW Creature))
                                (PtUp (LetterVal X)) (PtUp (Lit 0))
                         , DefinesLetter X
                             (StatOf Power
                                (Macros.a (ExiledWith Macros.thisEquipment))) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

colossalGraveReaver : Card
colossalGraveReaver =
  Macros.card "Colossal Grave-Reaver"
       (Just [Macros.generic 6, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOr Whenever
                            (Enters Macros.thisCreature Nothing)
                            [Macros.attacks Macros.thisCreature]
                            (Macros.mills You (Lit 3) You)
       , Macros.triggered Whenever
                          (Macros.putIntoFrom (Macros.counted (Macros.atLeast 1)
                                                     (And [Macros.creature,
                                                           InZone Macros.yourLibrary]))
                                       (Macros.graveyardOf You)
                                       (FromZone [Macros.yourLibrary]))
                          (Macros.putOntoBattlefield (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf)))) ]
       (Just (7, 6))

||| Grimdancer
public export
grimdancer : Card
grimdancer =
  Macros.card "Grimdancer"
       (Just [Macros.generic 1, Macros.pip Black, Macros.pip Black]) []
       (MkTypeLine [creatureType "Nightmare"] [Creature])
       [ Static (EntersRider Macros.thisCreature (WithCounters (Lit 2)
                   (DistinctChosenKinds [ KeywordCounter "Menace"
                                        , KeywordCounter "Deathtouch"
                                        , KeywordCounter "Lifelink" ])
                   Fresh)) ]
       (Just (3, 3))

divineVisitation : Card
divineVisitation =
  Macros.card "Divine Visitation"
       (Just [Macros.generic 3, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Static (Intercepts
                   (Macros.tokensCreatedUnder (Macros.counted (Macros.atLeast 1)
                                                            (And [Macros.creature, IsToken]))
                                              You) [] Nothing
                   (Macros.create GroupSize
                                  (MkToken (Just (Lit 4 ** Lit 4)) [White]
                                           (MkTypeLine [creatureType "Angel"] [Creature])
                                           [Macros.keyword "Flying", Macros.keyword "Vigilance"]
                                             Nothing))
                   Repeatedly Nothing) ]
       Nothing

adrixAndNev : Card
adrixAndNev =
  Macros.card "Adrix and Nev, Twincasters"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keywordCosting "Ward" (Mana [Macros.generic 2])
       , Static (Intercepts
                   (Macros.tokensCreatedUnder (Macros.counted (Macros.atLeast 1) IsToken) You) [] Nothing
                   (Create You (Macros.times 2 GroupSize) TokenAsThose [])
                   Repeatedly Nothing) ]
       (Just (2, 2))

windZendikon : Card
windZendikon =
  Macros.card "Wind Zendikon" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (Bundle (MkToken (Just (Lit 2 ** Lit 2)) [Blue]
                                   (MkTypeLine [creatureType "Elemental"] [Creature])
                                   [Macros.keyword "Flying"] Nothing) (Just Land)))
       , Macros.triggered When (Dies (AttachHost Enchanted (TypeW Land)))
                          (Macros.move (Macros.That CardW OneOf) Macros.handZ) ]
       Nothing

awakenTheBear : Card
awakenTheBear =
  Macros.card "Awaken the Bear" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Continuously
                  (AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                                  (PtUp (Lit 3)) (PtUp (Lit 3))
                           , Gains ((Macros.It OneOf)) (Macros.keyword "Trample") ])
                  (Just Macros.untilEndOfTurn)) ]
       Nothing

spidersilkArmor : Card
spidersilkArmor =
  Macros.card "Spidersilk Armor" (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [] [Enchantment])
       [ Static (AndAlso Nothing [ Gets Adds (Macros.allOf Macros.creatureYouControl)
                                (PtUp (Lit 0)) (PtUp (Lit 1))
                         , Gains ((Macros.It ManyOf)) (Macros.keyword "Reach") ]) ]
       Nothing

arcaneFlight : Card
arcaneFlight =
  Macros.card "Arcane Flight" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                         , Gains ((Macros.It OneOf)) (Macros.keyword "Flying") ]) ]
       Nothing

bootsOfSpeed : Card
bootsOfSpeed =
  Macros.card "Boots of Speed" (Just [Macros.pip Red]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso Nothing [ Gets Adds (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 1)) (PtUp (Lit 0))
                         , Gains ((Macros.It OneOf)) (Macros.keyword "Haste") ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing

frogify : Card
frogify =
  Macros.card "Frogify" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing) Nothing)
                         , Macros.hasBasePt ((Macros.It OneOf)) (Lit 1) (Lit 1) ]) ]
       Nothing

darksteelMutation : Card
darksteelMutation =
  Macros.card "Darksteel Mutation" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing []
                                             (MkTypeLine [creatureType "Insect"] [Artifact, Creature])
                                             [] Nothing) Nothing)
                         , Macros.hasBasePt ((Macros.It OneOf)) (Lit 0) (Lit 1)
                         , Gains ((Macros.It OneOf)) (Macros.keyword "Indestructible")
                         , LosesAllAbilities ((Macros.It OneOf)) Nothing ]) ]
       Nothing

kenrithsTransformation : Card
kenrithsTransformation =
  Macros.card "Kenrith's Transformation" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (AndAlso Nothing [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken Nothing [Green]
                                                (MkTypeLine [creatureType "Elk"] [Creature]) [] Nothing) Nothing)
                         , Macros.hasBasePt ((Macros.It OneOf)) (Lit 3) (Lit 3) ]) ]
       Nothing

amphibianDownpour : Card
amphibianDownpour =
  Macros.card "Amphibian Downpour" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.storm
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ LosesAllAbilities (AttachHost Enchanted (TypeW Creature)) Nothing
                         , Becomes ((Macros.It OneOf)) Sets (Bundle (MkToken Nothing [Blue]
                                                (MkTypeLine [creatureType "Frog"] [Creature]) [] Nothing) Nothing)
                         , Macros.hasBasePt ((Macros.It OneOf)) (Lit 1) (Lit 1) ]) ]
       Nothing

lignify : Card
lignify =
  Macros.card "Lignify" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Treefolk", enchantmentType "Aura"] [Kindred, Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Becomes (AttachHost Enchanted (TypeW Creature)) Sets (Bundle (MkToken Nothing [] (MkTypeLine [creatureType "Treefolk"] [])
                                             [] Nothing) Nothing)
                         , Macros.hasBasePt ((Macros.It OneOf)) (Lit 0) (Lit 4)
                         , LosesAllAbilities ((Macros.It OneOf)) Nothing ]) ]
       Nothing

nefariousImp : Card
nefariousImp =
  Macros.card "Nefarious Imp" (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [creatureType "Imp"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (Macros.leavesBattlefield
                             (Macros.counted (Macros.atLeast 1)
                                     (And [Permanent, HasPossessor ControllerAx You])))
                          (Macros.scry You (Lit 1)) ]
       (Just (2, 1))

||| Ainok Tracker
public export
ainokTracker : Card
ainokTracker =
  Macros.card "Ainok Tracker"
       (Just [Macros.generic 5, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dog", creatureType "Scout"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.keywordCosting "Morph"
                               (Mana [Macros.generic 4, Macros.pip Red]) ]
       (Just (3, 3))

irenicussVileDuplication : Card
irenicussVileDuplication =
  Macros.card "Irenicus's Vile Duplication"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [ Spell Nothing (Create You (Lit 1)
                  (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You]))
                               [ExceptAbility (Macros.keyword "Flying"),
                                ExceptNonlegendary])
                  []) ]
       Nothing

cacklingCounterpart : Card
cacklingCounterpart =
  Macros.card "Cackling Counterpart"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Create You (Lit 1)
                  (TokenCopyOf (Macros.target (And [Macros.creature, HasPossessor ControllerAx You])) [])
                  [])
       , Macros.keywordCosting "Flashback"
                               (Mana [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) ]
       Nothing

public export
chandrasSpitfire : Card
chandrasSpitfire =
  Macros.card "Chandra's Spitfire"
       (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Elemental"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
                          (IsDealtDamage NoncombatOnly (Macros.a Opponent))
                          (Macros.gets Macros.thisCreature (PtUp (Lit 3)) (PtUp (Lit 0))
                                       (Just Macros.untilEndOfTurn)) ]
       (Just (1, 3))

public export
livingHive : Card
livingHive =
  Macros.card "Living Hive"
       (Just [Macros.generic 6, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elemental", creatureType "Insect"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.triggered Whenever
                          (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Macros.create ThatMuch
                                  (Macros.creatureTok 1 1 [Green] [creatureType "Insect"])) ]
       (Just (6, 6))

public export
giantCindermaw : Card
giantCindermaw =
  Macros.card "Giant Cindermaw" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Dinosaur", creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.playerCant "GainLife" (PlayerGroup AllPlayers)) ]
       (Just (4, 3))

public export
lushGrowth : Card
lushGrowth =
  Macros.card "Lush Growth" (Just [Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Sets (Bundle (MkToken Nothing []
                                   (Macros.basicLandLine [landType "Mountain", landType "Forest", landType "Plains"])
                                   [] Nothing) Nothing)) ]
       Nothing

public export
voiceOfAll : Card
voiceOfAll =
  Macros.card "Voice of All"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.entersChoosing Macros.thisCreature Color)
       , Static (Gains Macros.thisCreature
                       (Macros.keywordQuality "Protection" (Macros.ofChosen Color))) ]
       (Just (2, 2))

public export
wardSliver : Card
wardSliver =
  Macros.card "Ward Sliver" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [creatureType "Sliver"] [Creature])
       [ Static (Macros.entersChoosing Macros.thisCreature Color)
       , Static (Gains (Macros.allOf (HasSubtype (creatureType "Sliver")))
                       (Macros.keywordQuality "Protection" (Macros.ofChosen Color))) ]
       (Just (2, 2))

public export
sanctuaryBlade : Card
sanctuaryBlade =
  Macros.card "Sanctuary Blade" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Macros.attachChoosing Macros.thisEquipment Color)
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Equipped (TypeW Creature))
                                (PtUp (Lit 2)) (PtUp (Lit 0))
                         , Gains (AttachHost Equipped (TypeW Creature))
                                 (Macros.keywordQuality "Protection"
                                    (Macros.ofTheLastChosen Color)) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Sinister Strength
public export
sinisterStrength : Card
sinisterStrength =
  Macros.card "Sinister Strength" (Just [Macros.generic 1, Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                                (PtUp (Lit 3)) (PtUp (Lit 1))
                         , Becomes ((Macros.It OneOf)) Sets (Colored (SomeColors [Black])) ]) ]
       Nothing

||| Crimson Wisps
public export
crimsonWisps : Card
crimsonWisps =
  Macros.card "Crimson Wisps" (Just [Macros.pip Red]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Continuously
                      (AndAlso Nothing [ Becomes (Macros.target Macros.creature) Sets (Colored (SomeColors [Red]))
                               , Gains ((Macros.It OneOf)) (Macros.keyword "Haste") ])
                      (Just Macros.untilEndOfTurn)
                  , (Draw You (Lit 1)) ]) ]
       Nothing

||| Ghoulflesh
public export
ghoulflesh : Card
ghoulflesh =
  Macros.card "Ghoulflesh" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                                (PtDown (Lit 1)) (PtDown (Lit 1))
                         , Becomes ((Macros.It OneOf)) Adds (Bundle (MkToken Nothing [Black]
                                                (MkTypeLine [creatureType "Zombie"] [])
                                                [] Nothing) Nothing) ]) ]
       Nothing

||| Blade of the Oni
public export
bladeOfTheOniStatic : StaticSpec []
bladeOfTheOniStatic =
  AndAlso Nothing [ Macros.hasBasePt (AttachHost Equipped (TypeW Creature)) (Lit 5) (Lit 5)
          , Gains ((Macros.It OneOf)) (Macros.keyword "Menace")
          , Becomes ((Macros.It OneOf)) Adds (Bundle (MkToken Nothing [Black]
                                 (MkTypeLine [creatureType "Demon"] []) [] Nothing) Nothing) ]

public export
historyOfBenalia : Card
historyOfBenalia =
  Macros.card "History of Benalia"
       (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI, ChapterII])
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 2 ** Lit 2)) [White]
                       (MkTypeLine [creatureType "Knight"] [Creature])
                       [Macros.keyword "Vigilance"] Nothing))
       , Macros.triggered When (ChapterMark [ChapterIII])
           (Macros.gets (Macros.allOf (And [HasSubtype (creatureType "Knight"), HasPossessor ControllerAx You]))
                        (PtUp (Lit 2)) (PtUp (Lit 1))
                        (Just Macros.untilEndOfTurn)) ]
       Nothing

||| Akroan Sergeant
public export
akroanSergeant : Card
akroanSergeant =
  Macros.card "Akroan Sergeant" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "FirstStrike"
       , Macros.renown 1 ]
       (Just (2, 2))

public export
castCreatureSpellsFromTop : StaticSpec []
castCreatureSpellsFromTop =
  (Macros.mayPlayDeed "Cast" You (Macros.allOf (And [Macros.spell, Macros.creature])) Nothing (PlayRider (Just Macros.onTopZ) Nothing Nothing False ItsOwnCost))

public export
garruksHorde : Card
garruksHorde =
  Macros.card "Garruk's Horde"
       (Just [Macros.generic 5, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Beast"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Visibility Reveal You TopOfLibrary)
       , Static castCreatureSpellsFromTop ]
       (Just (7, 7))

public export
wanderingEye : Card
wanderingEye =
  Macros.card "Wandering Eye"
       (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Visibility Reveal (PlayerGroup AllPlayers) WholeHand) ]
       (Just (1, 3))

public export
amorphousAxe : Card
amorphousAxe =
  Macros.card "Amorphous Axe" (Just [Macros.generic 2]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso Nothing
           [ Gets Adds (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 3))
                  (PtUp (Lit 0))
           , Becomes (AttachHost Equipped (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

public export
runedStalactite : Card
runedStalactite =
  Macros.card "Runed Stalactite" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (AndAlso Nothing
           [ Gets Adds (AttachHost Equipped (TypeW Creature)) (PtUp (Lit 1))
                  (PtUp (Lit 1))
           , Becomes (AttachHost Equipped (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

public export
arachnoform : Card
arachnoform =
  Macros.card "Arachnoform" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing
           [ Gets Adds (AttachHost Enchanted (TypeW Creature)) (PtUp (Lit 2))
                  (PtUp (Lit 2))
           , Gains (AttachHost Enchanted (TypeW Creature)) (Macros.keyword "Reach")
           , Becomes (AttachHost Enchanted (TypeW Creature)) Adds (EveryTypeOf CreatureSpace) ]) ]
       Nothing

public export
nyleasPresence : Card
nyleasPresence =
  Macros.card "Nylea's Presence"
       (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.land
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Becomes (AttachHost Enchanted (TypeW Land)) Adds (EveryTypeOf BasicLandSpace)) ]
       Nothing

||| Curse of Conformity
public export
curseOfConformity : Card
curseOfConformity =
  Macros.card "Curse of Conformity" (Just [Macros.generic 4, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura", enchantmentType "Curse"] [Enchantment])
       [ Macros.keywordSubject "Enchant" AnyPlayer
       , Static (AndAlso Nothing
           [ Macros.hasBasePt (Macros.allOf (And [Macros.creature, Not (HasSupertype Legendary),
                                    HasPossessor ControllerAx (AttachHost Enchanted PlayerW)]))
                       (Lit 3) (Lit 3)
           , Becomes ((Macros.It ManyOf)) Loses (EveryTypeOf CreatureSpace) ]) ]
       Nothing

||| Drumbellower
public export
drumbellower : Card
drumbellower =
  Macros.card "Drumbellower" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.untapsDuring (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You]))
                                     (Just (Macros.each Macros.otherPlayer))) ]
       (Just (2, 1))

public export
tayamLine : StaticSpec []
tayamLine =
  Macros.entersWithAdditionalCounters (Macros.each (And [Macros.creature, HasPossessor ControllerAx You,
                                                  OtherThan Macros.thisCreature]))
                                      (Lit 1)
                                      (KeywordCounter "Vigilance")

public export
clarionConqueror : Card
clarionConqueror =
  Macros.card "Clarion Conqueror" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (Or [Macros.artifact,
                                                       Macros.creature,
                                                       HasType Planeswalker])) ]))) ]
       (Just (3, 3))

||| Kang the Conqueror
public export
kangPowerUpLock : StaticSpec []
kangPowerUpLock =
  Macros.objectCant "Activate"
    (Macros.allOf (AbilityHead (KeywordClass "PowerUp")))

public export
vipersKiss : Card
vipersKiss =
  Macros.card "Viper's Kiss" (Just [Macros.pip Black]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (AndAlso Nothing
           [ Gets Adds (AttachHost Enchanted (TypeW Creature)) (PtDown (Lit 1)) (PtDown (Lit 1))
           , Macros.objectCant "Activate"
               (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf ((Macros.It OneOf))])) ]) ]
       Nothing

public export
stupefyingTouch : Card
stupefyingTouch =
  Macros.card "Stupefying Touch" (Just [Macros.generic 1, Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Macros.triggered When (Enters Macros.thisAura Nothing) (Draw You (Lit 1))
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (AttachHost Enchanted (TypeW Creature)) ]))) ]
       Nothing

public export
linvalaKeeperOfSilence : Card
linvalaKeeperOfSilence =
  Macros.card "Linvala, Keeper of Silence"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.objectCant "Activate"
                   (Macros.allOf (And [ AbilityHead AnyActivated
                               , AbilityOf (Macros.allOf (And [Macros.creature,
                                                        HasPossessor ControllerAx (PlayerGroup YourOpponents)])) ]))) ]
       (Just (3, 4))

public export
kyrenLegate : Card
kyrenLegate =
  Macros.card "Kyren Legate" (Just [Macros.generic 1, Macros.pip Red]) []
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Static (Macros.onlyWhile
                   (AltCost This Nothing)
                   (AndCond
                      [ Macros.exists (And [Macros.land, HasSubtype (landType "Plains"),
                                     HasPossessor ControllerAx Macros.anOpponent])
                      , Macros.exists (And [Macros.land, HasSubtype (landType "Mountain"),
                                     HasPossessor ControllerAx You]) ]))
       , Macros.keyword "Haste" ]
       (Just (1, 1))

public export
polarKraken : Card
polarKraken =
  Macros.card "Polar Kraken"
       (Just [Macros.generic 8, Macros.pip Blue, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Kraken"] [Creature])
       [ Macros.keyword "Trample"
       , Static (Macros.entersTapped Macros.thisCreature)
       , Macros.cumulativeUpkeep (Do (Macros.sacrifice You (Macros.a Macros.land))) ]
       (Just (11, 11))

public export
yavimayaAnts : Card
yavimayaAnts =
  Macros.card "Yavimaya Ants"
       (Just [Macros.generic 2, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Insect"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.keyword "Haste"
       , Macros.cumulativeUpkeep (Mana [Macros.pip Green, Macros.pip Green]) ]
       (Just (5, 1))

public export
illusionaryForces : Card
illusionaryForces =
  Macros.card "Illusionary Forces"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.cumulativeUpkeep (Mana [Macros.pip Blue]) ]
       (Just (4, 4))

public export
vexingSphinx : Card
vexingSphinx =
  Macros.card "Vexing Sphinx"
       (Just [Macros.generic 1, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.cumulativeUpkeep (Do ((Macros.discard You (Macros.a (InZone Macros.handZ)))))
       , Macros.triggered When (Dies Macros.thisCreature)
                          (Draw You (CountersOn (NamedCounter "Age") ((Macros.It OneOf)))) ]
       (Just (4, 4))

public export
manaChains : Card
manaChains =
  Macros.card "Mana Chains" (Just [Macros.pip Blue]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Gains (AttachHost Enchanted (TypeW Creature))
                       (Macros.cumulativeUpkeep (Mana [Macros.generic 1]))) ]
       Nothing

public export
dreamThief : Card
dreamThief =
  Macros.card "Dream Thief" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (OnlyIf (Draw You (Lit 1))
                          (Macros.happenedInvolving SpellCast You Lookback.ThisTurn
                                             (Macros.a (And [Macros.spell, ColorIs Blue,
                                                             OtherThan This])))
                          Nothing) ]
       (Just (2, 1))

public export
brightspearZealot : Card
brightspearZealot =
  Macros.card "Brightspear Zealot" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Soldier"] [Creature])
       [ Macros.keyword "Vigilance"
       , Static (Macros.onlyWhile
           (Gets Adds Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 0)))
           (CompareAmt (Macros.eventCountInvolving SpellCast
                                                   You
                                                   Lookback.ThisTurn
                                                   (Macros.a Macros.spell))
                       AtLeast (Lit 2))) ]
       (Just (2, 4))

public export
deepwayNavigator : Card
deepwayNavigator =
  Macros.card "Deepway Navigator" (Just [Macros.pip White, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flash"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (SetStatus Untapped
              (Macros.each (And [HasSubtype (creatureType "Merfolk"), HasPossessor ControllerAx You,
                          OtherThan Macros.thisCreature])))
       , Static (Macros.onlyWhile
           (Gets Adds (Macros.allOf (And [HasSubtype (creatureType "Merfolk"), HasPossessor ControllerAx You]))
                 (PtUp (Lit 1)) (PtUp (Lit 0)))
           (Macros.happenedInvolving AttackDeclaration
                                     You
                                     Lookback.ThisTurn
                                     (Macros.counted (Macros.atLeast 3)
                                        (HasSubtype (creatureType "Merfolk"))))) ]
       (Just (2, 2))

public export
bloodfireEnforcers : Card
bloodfireEnforcers =
  Macros.card "Bloodfire Enforcers" (Just [Macros.generic 3, Macros.pip Red]) []
       (MkTypeLine [creatureType "Human", creatureType "Monk"] [Creature])
       [ Static (Macros.onlyWhile
                   (AndAlso Nothing [ Gains Macros.thisCreature (Macros.keyword "FirstStrike")
                            , Gains ((Macros.It OneOf)) (Macros.keyword "Trample") ])
                   (AndCond [ Macros.exists (And [Macros.instant, InZone (Macros.graveyardOf You)])
                            , Macros.exists (And [Macros.sorcery, InZone (Macros.graveyardOf You)]) ])) ]
       (Just (5, 2))

public export
stormOfSouls : Instruction []
stormOfSouls =
  Sequentially [Macros.move (Macros.allOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                            Macros.battlefieldZ,
                Macros.becomesAs ((Macros.It ManyOf))
                                 (MkToken (Just (Lit 1 ** Lit 1)) [] (MkTypeLine [creatureType "Spirit"] [])
                                          [Macros.keyword "Flying"] Nothing)
                                 Nothing,
                Macros.exile You This]

public export
answeredPrayers : Card
answeredPrayers =
  Macros.card "Answered Prayers" (Just [Macros.generic 1, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Enchantment])
       [ Macros.triggered When (Enters (Macros.a Macros.creatureYouControl) Nothing)
           (Sequentially [Macros.gainsLife You (Lit 1),
                          If (NotCond (Matches Macros.thisEnchantment Macros.creature))
                             (Macros.becomesAs Macros.thisEnchantment
                                               (MkToken (Just (Lit 3 ** Lit 3)) []
                                                        (MkTypeLine [creatureType "Angel"] [Creature])
                                                        [Macros.keyword "Flying"] Nothing)
                                               (Just Macros.untilEndOfTurn))
                             Nothing]) ]
       Nothing

||| Hate Mirage's middle two sentences
public export
hateMirageTokens : Instruction []
hateMirageTokens =
  Sequentially
    [ ForEachOf (Described (TargetDet (Macros.upTo 2)) Macros.creatureYouDontControl)
                (Create You (Lit 1) (TokenCopyOf ((Macros.It OneOf)) []) [])
    , Macros.gains (Macros.That TokenW ManyOf) (Macros.keyword "Haste") Nothing ]

public export
descentOfTheDragons : Instruction []
descentOfTheDragons =
  Sequentially [Macros.destroy (Described (TargetDet Macros.anyNumber) Macros.creature),
                ForEachOf (Macros.TheVerbed "Destroy" (TypeW Creature) ThisWay ManyOf)
                          (Create (Macros.controllerOf ((Macros.It OneOf))) (Lit 1)
                                  (TokenWritten (MkToken (Just (Lit 4 ** Lit 4)) [Red]
                                                         (MkTypeLine [creatureType "Dragon"] [Creature])
                                                         [Macros.keyword "Flying"] Nothing))
                                  [])]

||| Brimaz, King of Oreskos
public export
brimazAttackToken : Ability
brimazAttackToken =
  Macros.triggered Whenever (Macros.attacks Macros.thisCreature)
    (Create You (Lit 1)
                   (TokenWritten (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                   (MkTypeLine [creatureType "Cat", creatureType "Soldier"] [Creature])
                                   [Macros.keyword "Vigilance"] Nothing))
                   [EntersAttacking NoDefender])

||| Sedris, the Traitor King
public export
sedrisTheTraitorKing : Ability
sedrisTheTraitorKing =
  Static (Gains (Macros.each (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth"
                   (Mana [Macros.generic 2, Macros.pip Black])))

||| Grixis
public export
grixis : Ability
grixis =
  Static (Gains (Macros.allOf (And [Or [ColorIs Blue, ColorIs Black, ColorIs Red],
                             Macros.creature,
                             InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth" ItsManaCost))

||| Dralnu, Lich Lord
public export
dralnuLichLord : Instruction []
dralnuLichLord =
  Macros.gains (Macros.target (And [Macros.instantOrSorcery,
                                    InZone (Macros.graveyardOf You)]))
               (Macros.keywordCosting "Flashback" ItsManaCost)
               (Just Macros.untilEndOfTurn)

||| Dregscape Zombie
public export
dregscapeZombie : Ability
dregscapeZombie = Macros.keywordCosting "Unearth" (Mana [Macros.pip Black])

||| Think Twice
public export
thinkTwice : Ability
thinkTwice =
  Macros.keywordCosting "Flashback" (Mana [Macros.generic 2, Macros.pip Blue])

||| Greater Mossdog
public export
greaterMossdog : Ability
greaterMossdog = Macros.keywordNumber "Dredge" (Lit 3)

||| Raven's Crime
public export
ravensCrime : Ability
ravensCrime = Macros.keyword "Retrace"

||| Barkhide Mauler
public export
barkhideMauler : Ability
barkhideMauler = Macros.keywordCosting "Cycling" (Mana [Macros.generic 2])

||| Ninja of the New Moon
public export
ninjaOfTheNewMoon : Ability
ninjaOfTheNewMoon =
  Macros.keywordCosting "Ninjutsu" (Mana [Macros.generic 3, Macros.pip Black])

||| Thunderous Wrath
public export
thunderousWrath : Ability
thunderousWrath = Macros.keywordCosting "Miracle" (Mana [Macros.pip Red])

||| Bygone Colossus
public export
bygoneColossus : Ability
bygoneColossus = Macros.keywordCosting "Warp" (Mana [Macros.generic 3])

||| Knight of Grace — "Hexproof from black" [CR#702.11d]
public export
knightOfGraceHexproofFromBlack : Ability
knightOfGraceHexproofFromBlack = Macros.keywordQuality "Hexproof" (ColorIs Black)

||| Eternal Dragon — "Plainscycling {2}" [CR#702.29e]
public export
eternalDragonPlainscycling : Ability
eternalDragonPlainscycling =
  Macros.keywordQualityCosting "Cycling" (HasSubtype (landType "Plains"))
                               (Mana [Macros.generic 2])

||| Nezumi Ronin
public export
nezumiRonin : Ability
nezumiRonin = Macros.keywordNumber "Bushido" (Lit 1)

||| Dragonlord Ojutai
public export
dragonlordOjutaiHexproof : Ability
dragonlordOjutaiHexproof =
  Static (Macros.onlyWhile (Gains Macros.thisCreature (Macros.keyword "Hexproof"))
                           (Matches (Macros.It OneOf) Macros.untapped))

||| Monoxa, Midway Manager
public export
monoxaRollTrigger : Ability
monoxaRollTrigger =
  Macros.triggered Whenever (Macros.youRollResultIn (Macros.atLeast 3))
    (Sequentially
      [ Macros.gains Macros.thisCreature (Macros.keyword "FirstStrike")
                     (Just Macros.untilEndOfTurn)
      , Macros.ifThen (CompareAmt (TheOutcome RollResult) AtLeast (Lit 4))
                      (Macros.gains Macros.thisCreature
                                    (Macros.keyword "Menace")
                                    (Just Macros.untilEndOfTurn))
      , Macros.ifThen (CompareAmt (TheOutcome RollResult) AtLeast (Lit 5))
                      (Macros.gains Macros.thisCreature
                                    (Macros.keyword "Lifelink")
                                    (Just Macros.untilEndOfTurn)) ])

||| Celebr-8000
public export
celebr8000Doubles : Instruction []
celebr8000Doubles =
  Sequentially [ (Macros.rollDice You 2 6)
               , Macros.ifThen RolledDoubles
                   (Macros.gains Macros.thisCreature
                                 (Macros.keyword "DoubleStrike")
                                 (Just Macros.untilEndOfTurn)) ]

||| Krosan Druid
public export
krosanDruid : Card
krosanDruid =
  Macros.card "Krosan Druid"
       (Just [Macros.generic 2, Macros.pip Green]) []
       (MkTypeLine [creatureType "Centaur", creatureType "Druid"] [Creature])
       [ Macros.keywordCosting "Kicker"
           (Mana [Macros.generic 4, Macros.pip Green])
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Macros.costWasPaid (ByKeyword "Kicker") Nothing Macros.thisCreature)
           (Macros.gainsLife You (Lit 10)) ]
       (Just (2, 3))

||| Lightkeeper of Emeria
public export
lightkeeperOfEmeria : Card
lightkeeperOfEmeria =
  Macros.card "Lightkeeper of Emeria"
       (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [creatureType "Angel"] [Creature])
       [ Macros.keywordCosting "Multikicker" (Mana [Macros.pip White])
       , Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Macros.gainsLife You
              (Macros.times 2 (Macros.timesPaid (ByKeyword "Kicker") Macros.thisCreature))) ]
       (Just (2, 4))

||| Merfolk Falconer
public export
merfolkFalconer : Card
merfolkFalconer =
  Macros.card "Merfolk Falconer"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Merfolk", creatureType "Wizard"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Casts You (Macros.a (CompareOver Macros.spell
                                    (Macros.paidCostRead (ByKeyword "Kicker") Nothing
                                                         (Macros.It OneOf))
                                    AtLeast (Lit 1))) Nothing)
           (Macros.scry You (Lit 2)) ]
       (Just (4, 4))

||| Borrowed Malevolence
public export
borrowedMalevolence : Card
borrowedMalevolence =
  Macros.card "Borrowed Malevolence" (Just [Macros.pip Black]) []
       (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])
       , Spell Nothing (Macros.chooseModes (Range (Just 1) (Just 2))
                  [ Macros.gets (Macros.target Macros.creature)
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                                (Just Macros.untilEndOfTurn)
                  , Macros.gets (Macros.target Macros.creature)
                                (PtDown (Lit 1)) (PtDown (Lit 1))
                                (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| New Perspectives
public export
newPerspectivesCyclingAltCost : Ability
newPerspectivesCyclingAltCost =
  Static (AltCost (Macros.allOf (AbilityHead (KeywordClass "Cycling")))
            (Just (Mana [Macros.generic 0])))

||| Thick-Skinned Goblin
public export
thickSkinnedGoblinEchoAltCost : Ability
thickSkinnedGoblinEchoAltCost =
  Static (AltCost (Macros.allOf (And [ AbilityHead (KeywordClass "Echo")
                              , AbilityOf (Macros.allOf (And [Permanent,
                                                       HasPossessor ControllerAx You])) ]))
            (Just (Mana [Macros.generic 0])))

||| Fumiko the Lowblood
public export
fumikoBushidoX : Ability
fumikoBushidoX =
  Static (AndAlso Nothing [ Gains Macros.thisCreature
                          (Macros.keywordNumber "Bushido" (LetterVal X))
                  , DefinesLetter X (Macros.countOf Attacking) ])

||| Rafter Demon
public export
rafterDemon : Card
rafterDemon =
  Macros.card "Rafter Demon"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [creatureType "Demon"] [Creature])
       [ Macros.keywordCosting "Spectacle"
           (Mana [Macros.generic 3, Macros.pip Black, Macros.pip Red])
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Macros.costWasPaid (ByKeyword "Spectacle") Nothing Macros.thisCreature)
           ((Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ)))) ]
       (Just (4, 2))

||| Tourach, Dread Cantor
public export
tourachDiscardTrigger : Ability
tourachDiscardTrigger =
  Macros.triggered Whenever
    (VerbedEvent (Just Macros.anOpponent) "Discard"
                 (Just (Macros.a (InZone Macros.handZ))) Nothing)
    (PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) Macros.thisCreature)

||| Shimmering Glasskite
public export
shimmeringGlasskite : Card
shimmeringGlasskite =
  Macros.card "Shimmering Glasskite"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Spirit"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggeredOnlyOnce Whenever
           (BecomesTarget Macros.thisCreature
              (Macros.a (Or [Macros.spell, AbilityHead AnyOnStack])))
           OncePerTurn
           (CounterSpell (Macros.That StackW OneOf)) ]
       (Just (2, 3))

||| Conqueror's Pledge
public export
conquerorsPledge : Card
conquerorsPledge =
  Macros.card "Conqueror's Pledge"
       (Just [Macros.generic 2, Macros.pip White, Macros.pip White, Macros.pip White]) []
       (MkTypeLine [] [Sorcery])
       [ Macros.keywordCosting "Kicker" (Mana [Macros.generic 6])
       , Spell Nothing (InsteadOf
                  (Macros.create (Lit 6)
                     (Macros.creatureTok 1 1 [White]
                        [creatureType "Kor", creatureType "Soldier"]))
                  (If (Macros.costWasPaid (ByKeyword "Kicker") Nothing This)
                      (Create You (Lit 12) TokenAsThose [])
                      Nothing)) ]
       Nothing

||| Soul of Emancipation
public export
soulOfEmancipation : Ability
soulOfEmancipation =
  Macros.triggered When (Enters Macros.thisCreature Nothing)
    (Sequentially
       [ Macros.destroy (Described (TargetDet (Macros.upTo 3)) (And [Permanent, Not Macros.land, OtherThan This]))
       , ForEachOf (Macros.TheVerbed "Destroy" PermanentW ThisWay ManyOf)
                   (Create (Macros.controllerOf (Macros.ItVerbed "Destroy" OneOf)) (Lit 1)
                           (TokenWritten
                              (MkToken (Just (Lit 3 ** Lit 3)) [White]
                                       (MkTypeLine [creatureType "Angel"] [Creature])
                                       [Macros.keyword "Flying"] Nothing))
                           []) ])

||| Tourach, Dread Cantor
public export
tourachDreadCantor : Card
tourachDreadCantor =
  Macros.card "Tourach, Dread Cantor"
       (Just [Macros.generic 1, Macros.pip Black]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keywordCosting "Kicker" (Mana [Macros.pip Black, Macros.pip Black])
       , Macros.keywordQuality "Protection" (ColorIs White)
       , Keyword.tourachDiscardTrigger
       , Macros.triggeredIf When (Enters Macros.thisCreature Nothing)
           (Macros.costWasPaid (ByKeyword "Kicker") Nothing Macros.thisCreature)
           (Macros.discard (Macros.target Opponent)
                            (Macros.countedAtRandom (Macros.exactly 2)
                                                    (InZone Macros.handZ))) ]
       (Just (2, 1))

||| Talrand's Invocation
public export
talrandsInvocation : Instruction []
talrandsInvocation =
  Macros.create (Lit 2)
    (MkToken (Just (Lit 2 ** Lit 2)) [Blue] (MkTypeLine [creatureType "Drake"] [Creature])
             [Macros.keyword "Flying"] Nothing)

||| Predatory Wurm
public export
predatoryWurm : Card
predatoryWurm =
  Macros.card "Predatory Wurm" (Just [Macros.generic 3, Macros.pip Green]) []
       (MkTypeLine [creatureType "Wurm"] [Creature])
       [ Macros.keyword "Vigilance"
       , Static (Macros.onlyWhile
                   (Gets Adds Macros.thisCreature (PtUp (Lit 2)) (PtUp (Lit 2)))
                   (Macros.exists (And [HasSubtype (planeswalkerType "Garruk"), HasPossessor ControllerAx You]))) ]
       (Just (4, 4))

||| Tower Winder
public export
towerWinder : Card
towerWinder =
  Macros.card "Tower Winder" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [creatureType "Snake"] [Creature])
       [ Macros.keyword "Reach"
       , Macros.keyword "Deathtouch"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.searchLibraryOrGraveyard
                  (Named (PrintedName "Command Tower"))
              , Macros.revealsIt
              , Macros.move Macros.foundCard Macros.handZ
              , If (Macros.happenedAt (VerbedAct "Search") You Lookback.ThisWay
                                      Macros.yourLibrary)
                   Macros.shuffle Nothing ]) ]
       (Just (1, 1))

||| Aim High
public export
aimHigh : Card
aimHigh =
  Macros.card "Aim High" (Just [Macros.generic 1, Macros.pip Green]) []
       (MkTypeLine [] [Instant])
       [ Spell Nothing (Sequentially
                  [ Macros.untap (Macros.target Macros.creature)
                  , Continuously
                      (AndAlso Nothing [ Gets Adds (Macros.ItVerbed "Untap" OneOf) (PtUp (Lit 2)) (PtUp (Lit 2))
                               , Gains (Macros.ItVerbed "Untap" OneOf) (Macros.keyword "Reach") ])
                      (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| Harried Dronesmith
public export
harriedDronesmithToken : Instruction []
harriedDronesmithToken =
  Sequentially [ Macros.create (Lit 1)
                   (MkToken (Just (Lit 1 ** Lit 1)) []
                            (MkTypeLine [creatureType "Thopter"] [Artifact, Creature])
                            [Macros.keyword "Flying"] Nothing)
               , Macros.gainsHaste Macros.itAsToken (Just Macros.untilEndOfTurn) ]

||| Archfiend's Vessel
public export
archfiendsVessel : Card
archfiendsVessel =
  Macros.card "Archfiend's Vessel" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Lifelink"
       , Macros.triggeredIf When
           (Enters This Nothing)
           (OrCond [ Happened ((Macros.It OneOf)) (MkLookback Entry Triggering (Just (FromZones (FromZone [Macros.graveyardOf You]) Nothing)))
                   , Matches ((Macros.It OneOf)) (And [Macros.castBy You,
                                      CastFrom (Macros.graveyardOf You)]) ])
           (Reflexively (Macros.exile You ((Macros.It OneOf)))
              (Macros.create (Lit 1)
                 (MkToken (Just (Lit 5 ** Lit 5)) [Black]
                          (MkTypeLine [creatureType "Demon"] [Creature])
                          [Macros.keyword "Flying"] Nothing))) ]
       (Just (1, 1))

||| Containment Priest
public export
containmentPriest : Card
containmentPriest =
  Macros.card "Containment Priest"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [creatureType "Human", creatureType "Cleric"] [Creature])
       [ Macros.keyword "Flash"
       , Static (Intercepts
                   (Enters (Macros.a (And [Macros.creature, Macros.nontoken,
                                           Not WasCast])) Nothing)
                   [] Nothing (Macros.exile You ((Macros.It OneOf))) Repeatedly Nothing) ]
       (Just (2, 2))

||| Veiling Oddity
public export
veilingOddityLine : Ability
veilingOddityLine =
  Macros.triggeredWhile When
    (Macros.lastCounterRemoved (NamedCounter "Time") This)
    (WhileTrue (Matches This (InZone Macros.exileZ)))
    (Continuously (Macros.deontic (Macros.allOf Macros.creature) Forbid ["Block"] Patient
                           NoDeonticPatient)
                  (Just ThisTurn))

public export
wizenedSnitches : Card
wizenedSnitches =
  Macros.card "Wizened Snitches"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Faerie", creatureType "Rogue"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Visibility Reveal (PlayerGroup AllPlayers) TopOfLibrary) ]
       (Just (1, 3))

||| Darkblade Agent
public export
darkbladeAgentDeathtouch : Ability
darkbladeAgentDeathtouch =
  Static (Conditionally (Gains Macros.thisCreature (Macros.keyword "Deathtouch"))
                        (Macros.happened (VerbedAct "Surveil") You ThisTurn)
                        AsLongAs)

||| Frenzied Gorespawn
public export
frenziedGorespawnMenaceTrigger : AbilityAt []
frenziedGorespawnMenaceTrigger =
  Triggered Whenever
    (Attacks (Macros.counted (Macros.atLeast 1) Macros.creature)
             (OneDefender Macros.anOpponent))
    [] Nothing [] Nothing Nothing Nothing
    (Macros.gains (Macros.That (TypeW Creature) ManyOf) (Macros.keyword "Menace")
                  (Just Macros.untilEndOfTurn))

||| Pir, Imaginative Rascal's replacement
public export
pirDistributive : Ability
pirDistributive =
  Static (Intercepts
            (Macros.manyBareCountersPutBy You
               (Macros.a (And [Permanent, HasPossessor ControllerAx (PlayerGroup YourTeam)])))
            [] Nothing
            (PutCounters (Plus ThatMuch (Lit 1)) ThoseKinds (Macros.That PermanentW OneOf))
            Repeatedly Nothing)

||| Frogmite
public export
frogmite : Card
frogmite =
  Macros.card "Frogmite" (Just [Macros.generic 4]) []
       (MkTypeLine [creatureType "Frog"] [Artifact, Creature])
       [ Macros.keywordQuality "Affinity" Macros.artifact ]
       (Just (2, 2))

||| Iymrith, Desert Doom
public export
iymrithDesertDoom : Card
iymrithDesertDoom =
  Macros.card "Iymrith, Desert Doom"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) [Legendary]
       (MkTypeLine [creatureType "Dragon"] [Creature])
       [ Macros.keyword "Flying"
       , Static (Macros.onlyWhile
                   (Gains Macros.thisCreature
                          (Macros.keywordCosting "Ward" (Mana [Macros.generic 4])))
                   (Matches (Macros.It OneOf) Macros.untapped))
       , Macros.triggered Whenever
                          (Macros.dealsCombatDamage Macros.thisCreature (Macros.a AnyPlayer))
                          (Sequentially
                             [ (Draw You (Lit 1))
                             , If (CompareAmt (Macros.countOf (InZone (Macros.handOf You)))
                                              Less (Lit 3))
                                  (Draw You TheDifference)
                                  Nothing ]) ]
       (Just (5, 5))

public export
protectionAbilities : KeywordTerm
protectionAbilities = AnyKeywordIn (MkKeywordFamily "Protection" Nothing)

public export
landwalkAbilities : KeywordTerm
landwalkAbilities = AnyKeywordIn (MkKeywordFamily "Landwalk" Nothing)

||| Shay Cormac
public export
wardAbilities : KeywordTerm
wardAbilities = AnyKeywordIn (MkKeywordFamily "Ward" Nothing)

||| Tolaria, Shelkin Brownie
public export
bandsWithOtherAbilities : KeywordTerm
bandsWithOtherAbilities = AnyKeywordIn (MkKeywordFamily "BandsWithOther" Nothing)

public export
protectionFromAnyColor : KeywordTerm
protectionFromAnyColor = AnyKeywordIn (MkKeywordFamily "Protection" (Just Color))

||| Cairn Wanderer
public export
cairnWanderer : Card
cairnWanderer =
  Macros.card "Cairn Wanderer" (Just [Macros.generic 4, Macros.pip Black]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.keyword "Changeling"
       , AlsoForKeywords
           (Static (Macros.onlyWhile
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))
                      (Macros.exists (And [Macros.creature, InZone Macros.graveyardZ,
                                    HasKeyword (TheKeyword "Flying")]))))
           [ TheKeyword "Fear", TheKeyword "FirstStrike"
           , TheKeyword "DoubleStrike", TheKeyword "Deathtouch"
           , TheKeyword "Haste", landwalkAbilities, TheKeyword "Lifelink"
           , protectionAbilities, TheKeyword "Reach", TheKeyword "Trample"
           , TheKeyword "Shroud", TheKeyword "Vigilance" ] ]
       (Just (4, 4))

||| Autarch Mammoth
public export
autarchMammoth : Card
autarchMammoth =
  Macros.card "Autarch Mammoth"
       (Just [Macros.generic 4, Macros.pip Green, Macros.pip Green]) []
       (MkTypeLine [creatureType "Elephant", creatureType "Mount"] [Creature])
       [ autarchMammothLine
       , Macros.keywordNumber "Saddle" (Lit 5) ]
       (Just (5, 5))

||| Debris Beetle
public export
debrisBeetle : Card
debrisBeetle =
  Macros.card "Debris Beetle"
       (Just [Macros.generic 2, Macros.pip Black, Macros.pip Green]) []
       (MkTypeLine [artifactType "Vehicle"] [Artifact])
       [ Macros.keyword "Trample"
       , Macros.triggered When (Enters Macros.thisVehicle Nothing)
           (Sequentially [ Macros.losesLife (Macros.each Opponent) (Lit 3)
                         , Macros.gainsLife You (Lit 3) ])
       , Macros.keywordNumber "Crew" (Lit 2) ]
       (Just (6, 6))

||| Pir, Imaginative Rascal
public export
pirImaginativeRascal : Card
pirImaginativeRascal =
  Macros.card "Pir, Imaginative Rascal"
       (Just [Macros.generic 2, Macros.pip Green]) [Legendary]
       (MkTypeLine [creatureType "Human"] [Creature])
       [ Macros.keywordQuality "PartnerWith"
                               (Named (PrintedName "Toothy, Imaginary Friend"))
       , pirDistributive ]
       (Just (1, 1))

||| Market Gnome
public export
marketGnome : Card
marketGnome =
  Macros.card "Market Gnome" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Gnome"] [Artifact, Creature])
       [ Macros.triggered When (Dies Macros.thisCreature)
           (Sequentially [ Macros.gainsLife You (Lit 1), (Draw You (Lit 1)) ])
       , Macros.triggeredWhile When
           (VerbedEvent Nothing "Exile" (Just Macros.thisCreature) Nothing)
           (WhileDoing (Activates You
                          (Macros.a (AbilityHead (KeywordClass "Craft")))))
           (Sequentially [ Macros.gainsLife You (Lit 1), (Draw You (Lit 1)) ]) ]
       (Just (0, 3))

||| Escaped Shapeshifter
public export
escapedShapeshifter : Card
escapedShapeshifter =
  Macros.card "Escaped Shapeshifter"
       (Just [Macros.generic 3, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ AlsoForKeywords
           (Static (Macros.onlyWhile
                      (Gains Macros.thisCreature (Macros.keyword "Flying"))
                      (Macros.exists (And [Macros.creature,
                                    HasPossessor ControllerAx Macros.anOpponent,
                                    HasKeyword (TheKeyword "Flying"),
                                    Not (Named (PrintedName "Escaped Shapeshifter"))]))))
           [ TheKeyword "FirstStrike", TheKeyword "Trample"
           , protectionFromAnyColor ] ]
       (Just (3, 4))

||| Ancestral Blade
public export
ancestralBlade : Card
ancestralBlade =
  Macros.card "Ancestral Blade" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Macros.triggered When (Enters Macros.thisEquipment Nothing)
           (Sequentially
              [ Macros.create (Lit 1)
                  (MkToken (Just (Lit 1 ** Lit 1)) [White]
                           (MkTypeLine [creatureType "Soldier"] [Creature])
                           [] Nothing)
              , AttachTo Macros.thisEquipment Macros.itAsToken ])
       , Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 1]) ]
       Nothing

||| Steelclaw Lance
public export
steelclawLance : Card
steelclawLance =
  Macros.card "Steelclaw Lance" (Just [Macros.pip Black, Macros.pip Red]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.keywordQualityCosting "Equip"
           (HasSubtype (creatureType "Knight")) (Mana [Macros.generic 1])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]
       Nothing

||| Commander's Plate's equip lines
public export
commandersPlateEquip : List Ability
commandersPlateEquip =
  [ Macros.keywordQualityCosting "Equip"
      (HasDesignation CommanderD) (Mana [Macros.generic 3])
  , Macros.keywordCosting "Equip" (Mana [Macros.generic 5]) ]

||| Luxior, Giada's Gift's equip lines
public export
luxiorEquipLines : List Ability
luxiorEquipLines =
  [ Macros.keywordQualityCosting "Equip"
      (HasType Planeswalker) (Mana [Macros.generic 1])
  , Macros.keywordCosting "Equip" (Mana [Macros.generic 3]) ]

||| Veiling Oddity
public export
veilingOddity : Card
veilingOddity =
  Macros.card "Veiling Oddity"
       (Just [Macros.generic 3, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Illusion"] [Creature])
       [ Macros.keywordNumberCosting "Suspend" (Lit 4)
           (Mana [Macros.generic 1, Macros.pip Blue])
       , veilingOddityLine ]
       (Just (2, 3))

||| Enormous Energy Blade
public export
enormousEnergyBlade : Card
enormousEnergyBlade =
  Macros.card "Enormous Energy Blade"
       (Just [Macros.generic 2, Macros.pip Black]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 4)) (PtUp (Lit 0)))
       , Macros.triggered Whenever
           (BecomesAttached Macros.thisEquipment (Macros.a Macros.creature))
           (Macros.tap (Macros.That (TypeW Creature) OneOf))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 2]) ]
       Nothing

||| Grafted Wargear
public export
graftedWargear : Card
graftedWargear =
  Macros.card "Grafted Wargear" (Just [Macros.generic 3]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 3)) (PtUp (Lit 2)))
       , Macros.triggered Whenever
           (BecomesUnattached Macros.thisEquipment (Macros.a Permanent))
           (Macros.sacrifice You (Macros.That PermanentW OneOf))
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 0]) ]
       Nothing

||| Black Ward
public export
blackWard : Card
blackWard =
  Macros.card "Black Ward" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (ColorIs Black)))
                   Macros.thisAura) ]
       Nothing

||| Cho-Manno's Blessing
public export
choMannosBlessing : Card
choMannosBlessing =
  Macros.card "Cho-Manno's Blessing" (Just [Macros.pip White, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (Macros.ofChosen Color)))
                   Macros.thisAura) ]
       Nothing

||| Pentarch Ward
public export
pentarchWard : Card
pentarchWard =
  Macros.card "Pentarch Ward" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Macros.triggered When (Enters Macros.thisAura Nothing) ((Draw You (Lit 1)))
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (Macros.ofChosen Color)))
                   Macros.thisAura) ]
       Nothing

||| Benevolent Blessing
public export
benevolentBlessing : Card
benevolentBlessing =
  Macros.card "Benevolent Blessing" (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keyword "Flash"
       , Macros.keywordSubject "Enchant" Macros.creature
       , Static (Macros.entersChoosing Macros.thisAura Color)
       , Static (DoesntRemove
                   (Gains (AttachHost Enchanted (TypeW Creature))
                          (Macros.keywordQuality "Protection" (Macros.ofChosen Color)))
                   (Macros.allOf (And [ Or [ HasSubtype (enchantmentType "Aura")
                                    , HasSubtype (artifactType "Equipment") ]
                               , HasPossessor ControllerAx You
                               , AttachedTo ((Macros.It OneOf)) ]))) ]
       Nothing

||| Wall of Shards
public export
wallOfShards : Card
wallOfShards =
  Macros.card "Wall of Shards" (Just [Macros.generic 1, Macros.pip White]) [Snow]
       (MkTypeLine [creatureType "Wall"] [Creature])
       [ Macros.keyword "Defender"
       , Macros.keyword "Flying"
       , Macros.cumulativeUpkeep (Do (Macros.gainsLife Macros.anOpponent (Lit 1))) ]
       (Just (1, 8))

||| Earthen Goo
public export
earthenGoo : Card
earthenGoo =
  Macros.card "Earthen Goo" (Just [Macros.generic 2, Macros.pip Red]) []
       (MkTypeLine [creatureType "Ooze"] [Creature])
       [ Macros.keyword "Trample"
       , Macros.cumulativeUpkeep
           (EitherCost (Mana [Macros.pip Red]) (Mana [Macros.pip Green]))
       , Static (Gets Adds Macros.thisCreature
                   (PtUp (Macros.times 1 (CountersOn (NamedCounter "Age") ((Macros.It OneOf)))))
                   (PtUp (Macros.times 1 (CountersOn (NamedCounter "Age") ((Macros.It OneOf)))))) ]
       (Just (2, 2))

||| Mutagen Connoisseur
public export
mutagenConnoisseur : Card
mutagenConnoisseur =
  Macros.card "Mutagen Connoisseur"
       (Just [Macros.generic 1, Macros.pip Green, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Vedalken", creatureType "Mutant"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.keyword "Vigilance"
       , Static (Gets Adds Macros.thisCreature
                      (PtUp (Macros.forEach 1
                               (And [IsTransformed, HasPossessor ControllerAx You])))
                      (PtUp (Lit 0))) ]
       (Just (0, 5))

||| Bedrock Tortoise
public export
bedrockTortoiseWindow : StaticSpec []
bedrockTortoiseWindow =
  OnlyDuring Turn (Just You)
    (Gains (Macros.allOf Macros.creatureYouControl)
           (Macros.keyword "Hexproof"))

||| Battlegate Mimic
public export
battlegateMimic : Card
battlegateMimic =
  Macros.card "Battlegate Mimic"
       (Just [Macros.generic 1, Macros.hybridPip Red White]) []
       (MkTypeLine [creatureType "Shapeshifter"] [Creature])
       [ Macros.triggered Whenever
           (Casts You (Macros.a (And [Macros.spell, ColorIs Red,
                                      ColorIs White])) Nothing)
           (Continuously
              (AndAlso Nothing [ Macros.hasBasePt Macros.thisCreature (Lit 4) (Lit 2)
                       , Gains Macros.thisCreature
                               (Macros.keyword "FirstStrike") ])
              (Just Macros.untilEndOfTurn)) ]
       (Just (2, 1))

||| Sphinx of Uthuun
public export
sphinxOfUthuun : Card
sphinxOfUthuun =
  Macros.card "Sphinx of Uthuun"
       (Just [Macros.generic 5, Macros.pip Blue, Macros.pip Blue]) []
       (MkTypeLine [creatureType "Sphinx"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered When (Enters Macros.thisCreature Nothing)
           (Sequentially
              [ Macros.revealCards (Macros.topSlice (Lit 5))
              , SeparateIntoPiles Macros.anOpponent ((Macros.It ManyOf)) 2 []
              , Macros.move Macros.onePile Macros.handZ
              , Macros.move (Macros.theOther Pile) Macros.graveyardZ ]) ]
       (Just (5, 6))

||| Marit Lage
public export
maritLage : TokenChars bs
maritLage =
  MkSupertypedToken (Just (Lit 20 ** Lit 20)) [Black] [Legendary]
    (MkTypeLine [creatureType "Avatar"] [Creature])
    [Macros.keyword "Flying", Macros.keyword "Indestructible"]
    (Just "Marit Lage")

||| Tuktuk the Explorer
public export
tuktukTheExplorer : Card
tuktukTheExplorer =
  Macros.card "Tuktuk the Explorer"
       (Just [Macros.generic 2, Macros.pip Red]) [Legendary]
       (MkTypeLine [creatureType "Goblin"] [Creature])
       [ Macros.keyword "Haste"
       , Macros.triggered When (Dies Macros.thisCreature)
           (Macros.create (Lit 1)
              (MkSupertypedToken (Just (Lit 5 ** Lit 5)) [] [Legendary]
                 (MkTypeLine [creatureType "Goblin", creatureType "Golem"]
                             [Artifact, Creature])
                 [] (Just "Tuktuk the Returned"))) ]
       (Just (1, 1))

||| Artificer's Assistant
public export
artificersAssistant : Card
artificersAssistant =
  Macros.card "Artificer's Assistant" (Just [Macros.pip Blue]) []
       (MkTypeLine [creatureType "Bird"] [Creature])
       [ Macros.keyword "Flying"
       , Macros.triggered Whenever
           (Casts You (Macros.a (And [IsHistoric, Macros.spell])) Nothing)
           (Macros.scry You (Lit 1)) ]
       (Just (1, 1))

||| Aya of Alexandria
public export
ayaOfAlexandria : Card
ayaOfAlexandria =
  Macros.card "Aya of Alexandria"
       (Just [Macros.generic 2, Macros.pip Red, Macros.pip White]) [Legendary]
       (MkTypeLine [creatureType "Human", creatureType "Assassin"] [Creature])
       [ Macros.keyword "Menace"
       , Macros.keyword "Lifelink"
       , Macros.triggered Whenever
           (Macros.dealsCombatDamage
              (Macros.a (And [IsHistoric, Macros.creature, HasPossessor ControllerAx You]))
              (Macros.a AnyPlayer))
           (Macros.create (Lit 1)
              (MkToken (Just (Lit 1 ** Lit 1)) [Black]
                       (MkTypeLine [creatureType "Assassin"] [Creature])
                       [Macros.keyword "Menace"] Nothing)) ]
       (Just (4, 3))

||| Indestructibility
public export
indestructibility : Card
indestructibility =
  Macros.card "Indestructibility" (Just [Macros.generic 3, Macros.pip White]) []
       (MkTypeLine [enchantmentType "Aura"] [Enchantment])
       [ Macros.keywordSubject "Enchant" Permanent
       , Static (Gains (AttachHost Enchanted PermanentW)
                       (Macros.keyword "Indestructible")) ]
       Nothing

||| Seraphic Greatsword
public export
seraphicGreatsword : Card
seraphicGreatsword =
  Macros.card "Seraphic Greatsword"
       (Just [Macros.generic 1, Macros.pip White]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [ Static (Gets Adds (AttachHost Equipped (TypeW Creature))
                      (PtUp (Lit 2)) (PtUp (Lit 2)))
       , Macros.triggered Whenever
           (Macros.attacksPlayer (AttachHost Equipped (TypeW Creature))
              (Macros.the (And [AnyPlayer,
                              Superlative MaxOf (PlayerStatAxis LifeTotal)
                                          AnyPlayer])))
           (Create You (Lit 1)
              (TokenWritten (MkToken (Just (Lit 4 ** Lit 4)) [White]
                               (MkTypeLine [creatureType "Angel"] [Creature])
                               [Macros.keyword "Flying"] Nothing))
              [EntersTapped, EntersAttacking (OneDefender (Macros.That PlayerW OneOf))])
       , Macros.keywordCosting "Equip" (Mana [Macros.generic 4]) ]
       Nothing

giantGrowth : Instruction []
giantGrowth = Macros.gets (Macros.target Macros.creature) (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn)

amassZombiesTwo : Instruction []
amassZombiesTwo = Macros.amass "Zombie" 2

||| Avarice Totem
avariceTotemExchange : Instruction []
avariceTotemExchange =
  Macros.exchangeControlOfThis Artifact (Macros.target (And [Permanent, Not Macros.land]))

||| Tovolar, Dire Overlord
tovolarNightfall : Instruction []
tovolarNightfall = GameBecomes Night

||| Spin into Myth
spinIntoMyth : Instruction []
spinIntoMyth =
  Sequentially [ Macros.move (Macros.target Macros.creature) Macros.onTopZ
               , Macros.fateseal Macros.anOpponent (Lit 2) ]

||| Pure // Simple
simpleHalf : Instruction []
simpleHalf = Macros.destroy (Macros.target (And [Permanent, Macros.multicolored]))

||| Korlash
public export
grandeurDiscardCost : Cost []
grandeurDiscardCost =
  Do (Macros.discard You
        (Macros.a (And [Named (PrintedName "Korlash, Heir to Blackblade"),
                        OtherThan This, InZone Macros.handZ])))

||| Gyruda, Doom of Depths
gyrudaCompanion : AbilityAt []
gyrudaCompanion =
  Macros.companion (EveryCardIs IsCard (ManaValueParity EvenValue))

||| Jegantha, the Wellspring
jeganthaCompanion : AbilityAt []
jeganthaCompanion =
  Macros.companion (NoCardIs IsCard RepeatedManaSymbol)

||| Kaheera, the Orphanguard
kaheeraCompanion : AbilityAt []
kaheeraCompanion =
  Macros.companion
    (EveryCardIs (And [Macros.creature, IsCard])
       (AnyTraitOf [ ACharacteristic (HasSubtype (creatureType "Cat"))
                   , ACharacteristic (HasSubtype (creatureType "Elemental"))
                   , ACharacteristic (HasSubtype (creatureType "Nightmare"))
                   , ACharacteristic (HasSubtype (creatureType "Dinosaur"))
                   , ACharacteristic (HasSubtype (creatureType "Beast")) ]))

||| Keruga, the Macrosage
kerugaCompanion : AbilityAt []
kerugaCompanion =
  Macros.companion
    (EveryCardIs IsCard
       (AnyTraitOf [ ACharacteristic (Compare [StatAxis ManaValue] AtLeast (Lit 3))
                   , ACharacteristic Macros.land ]))

||| Lurrus of the Dream-Den
lurrusCompanion : AbilityAt []
lurrusCompanion =
  Macros.companion
    (EveryCardIs (And [Permanent, IsCard])
       (ACharacteristic (Compare [StatAxis ManaValue] AtMost (Lit 2))))

||| Lutri, the Spellchaser
lutriCompanion : AbilityAt []
lutriCompanion =
  Macros.companion (CardsDiffer (And [Not Macros.land, IsCard]) CardName)

||| Obosh, the Preypiercer
oboshCompanion : AbilityAt []
oboshCompanion =
  Macros.companion
    (EveryCardIs IsCard
       (AnyTraitOf [ManaValueParity OddValue, ACharacteristic Macros.land]))

||| Umori, the Collector
umoriCompanion : AbilityAt []
umoriCompanion =
  Macros.companion (CardsShare (And [Not Macros.land, IsCard]) CardTypeQ)

||| Yorion, Sky Nomad
yorionCompanion : AbilityAt []
yorionCompanion = Macros.companion (DeckSizeOverMinimum 20)

||| Zirda, the Dawnwaker
zirdaCompanion : AbilityAt []
zirdaCompanion =
  Macros.companion
    (EveryCardIs (And [Permanent, IsCard]) (HasAbilityOf AnyActivated))
